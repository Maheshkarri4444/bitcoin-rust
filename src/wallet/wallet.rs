use std::fmt;
use crate::config::INITIAL_BALANCE;
use k256::ecdsa::{SigningKey,VerifyingKey,Signature,signature::Signer};
use k256::EncodedPoint;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use crate::chain_util::ChainUtil;

pub struct Wallet {
    pub balance: u64,
    pub key_pair: SigningKey,
    pub public_key: String, 
}

impl Wallet {
    pub fn new() -> Self{
        let (key_pair,verifying_key) = ChainUtil::gen_key_pair();
        let public_key_point:EncodedPoint = verifying_key.to_encoded_point(false);
        let public_key_hex = hex::encode(public_key_point.as_bytes());

    
        Self {
            balance:INITIAL_BALANCE,
            key_pair,
            public_key:public_key_hex,
        }
    }
    
    pub fn sign(&self , data_hash: &str) ->Signature {
        let bytes = hex::decode(data_hash).expect("Invalid hex string for signing");
        self.key_pair.sign(&bytes)
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result {
        write!{
            f,
            "Wallet -\n publicKey: {}\n balance:{}",
            self.public_key,
            self.balance
        }
    }
}