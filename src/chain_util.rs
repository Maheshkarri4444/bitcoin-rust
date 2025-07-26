use k256::ecdsa::{SigningKey, VerifyingKey,signature::Verifier,Signature};
use k256::EncodedPoint;
use rand_core::OsRng;
use uuid::Uuid;
use sha2::{Sha256,Digest};
use serde_json;

pub struct ChainUtil;

impl ChainUtil {
    pub fn gen_key_pair() -> (SigningKey,VerifyingKey ){
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        (signing_key,verifying_key)
    }

    pub fn id()->String {
        Uuid::new_v4().to_string()
    }

    pub fn hash<T: serde::Serialize>(data:T)->String{
        let json = serde_json::to_string(&data).expect("Failed to Serialize the data for hashing");
        let mut hasher = Sha256::new();
        hasher.update(json);
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn verify_signature(public_key_hex:&str, signature_bytes:&[u8],data_hash:&str)->bool {
        let public_key_bytes = match hex::decode(public_key_hex){
            Ok(bytes)=>bytes,
            Err(_)=>return false,
        };

        let encoded_point = match EncodedPoint::from_bytes(&public_key_bytes){
            Ok(point)=>point,
            Err(_)=>return false,
        };

        let verifying_key = match 
        VerifyingKey::from_encoded_point(&encoded_point){
            Ok(key)=>key,
            Err(_)=>return false,
        };

        let signature = match Signature::from_der(signature_bytes){
            Ok(sig)=>sig,
            Err(_)=>return false,
        };

        let msg_bytes = match hex::decode(data_hash){
            Ok(bytes)=>bytes,
            Err(_)=>return false,
        };

        verifying_key.verify(&msg_bytes,&signature).is_ok()


    }
}