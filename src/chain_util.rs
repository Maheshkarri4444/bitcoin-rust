use k256::ecdsa::{SigningKey, VerifyingKey};
use k256::EncodedPoint;
use rand_core::OsRng;
use uuid::Uuid;

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
}