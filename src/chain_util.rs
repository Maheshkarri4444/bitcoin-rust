use k256::ecdsa::{
    SigningKey, VerifyingKey,
    signature::{ Verifier, Signature as _}, 
    Signature as K256Signature,
};
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
        println!("verify sign called");
        let public_key_bytes = match hex::decode(public_key_hex){
            Ok(bytes)=>bytes,
            Err(_)=>{  println!("pub_key bytes false");
            return false},
        };

        let encoded_point = match EncodedPoint::from_bytes(&public_key_bytes){
            Ok(point)=>point,
             Err(_)=>{  println!("encoded_point bytes false");
            return false},
        };

        let verifying_key = match 
        VerifyingKey::from_encoded_point(&encoded_point){
            Ok(key)=>key,
             Err(_)=>{  println!("verifying key bytes false");
            return false},
        };

        if signature_bytes.len() != 64 {
            println!("signature length is not 64 bytes: {}", signature_bytes.len());
            return false;
        }
        let signature = match K256Signature::from_bytes(signature_bytes) {
            Ok(sig) => sig,
            Err(_) => {
                println!("signature bytes invalid format");
                return false;
            }
        };

        let msg_bytes = match hex::decode(data_hash){
            Ok(bytes)=>bytes,
             Err(_)=>{  println!("msg_bytes bytes false");
            return false},
        };

        verifying_key.verify(&msg_bytes,&signature).is_ok()


    }
}