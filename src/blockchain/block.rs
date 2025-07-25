use sha2::{Sha256,Digest};
use std::fmt;
use serde::{Serialize, Deserialize};
use crate::config::DIFFICULTY;

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct Block {
    pub block_number: u64, 
    pub timestamp:u128,
    pub last_hash: String,
    pub hash: String,
    pub data: String,
    pub nonce: u64,
}

impl Block{
    pub fn new(block_number: u64, timestamp: u128, last_hash: String,hash: String,data:String,nonce:u64)->Self{
        Self{
            block_number,
            timestamp,
            last_hash,
            hash,
            data,
            nonce,
        }
    }

    pub fn genesis()->Block{
        Block::new(0,0,String::from("----"),String::from("f1r57-h45h"),String::from(""),0)
    }

    pub fn mine_block(last_block: &Block,data:String)->Block{
        let last_hash = last_block.hash.clone();
        let block_number = last_block.block_number +1;
        let mut nonce = 0u64;
        loop {
            let timestamp = chrono::Utc::now().timestamp_millis() as u128;
            let hash = Block::hash(block_number,timestamp,&last_hash , &data, &nonce);

            if hash.chars().take(DIFFICULTY).all(|c| c=='0'){
                return Block::new(
                    block_number,
                    timestamp,
                    last_hash,
                    hash,
                    data,
                    nonce,
                );
            }
            nonce+=1;
        }
    }

    pub fn hash(block_number:u64,timestamp:u128,last_hash:&str,data:&str,nonce:&u64)->String{
        let input = format!("{}{}{}{}{}",block_number,timestamp,last_hash,data,nonce);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}",result)
    }

    pub fn block_hash(block:&Block)->String{
        Block::hash(block.block_number,block.timestamp,&block.last_hash,&block.data,&block.nonce)
    }
}

impl fmt::Display for Block {
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result {
        write!{
            f,
            "Block -
            Number:    {}
            Timestamp: {}
            Last Hash: {}
            Hash     : {}
            Nonce    : {}  
            Data     : {}",
            self.block_number,
            self.timestamp,
            &self.last_hash[..10.min(self.last_hash.len())],
            &self.hash[..10.min(self.hash.len())],
            self.nonce,
            self.data
        }
    }
}