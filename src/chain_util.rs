use k256::ecdsa::{SigningKey, VerifyingKey};
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
}