// use sha2::{Digest};
use std::fmt;
use serde::{Serialize, Deserialize};
use crate::config::{DIFFICULTY,MINE_RATE};
use crate::chain_util::ChainUtil;
use crate::wallet::transaction::ChainTransaction;

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct Block {
    pub block_number: u64, 
    pub timestamp:u128,
    pub last_hash: String,
    pub hash: String,
    pub data: Vec<ChainTransaction>,
    pub nonce: u64,
    pub difficulty: usize,
}

impl Block{
    pub fn new(block_number: u64, timestamp: u128, last_hash: String,hash: String,data:Vec<ChainTransaction>,nonce:u64,difficulty:usize)->Self{
        Self{
            block_number,
            timestamp,
            last_hash,
            hash,
            data,
            nonce,
            difficulty,
        }
    }

    pub fn genesis()->Block{
        Block::new(0,0,String::from("----"),String::from("f1r57-h45h"),vec![],0,DIFFICULTY)
    }

    pub fn mine_block(last_block: &Block,data:Vec<ChainTransaction>)->Block{
        let last_hash = last_block.hash.clone();
        let block_number = last_block.block_number +1;
        let mut nonce = 0u64;
        let mut _timestamp: u128;
        let  _difficulty = last_block.difficulty;
        loop {
            let timestamp = chrono::Utc::now().timestamp_millis() as u128;
            let difficulty = Block::adjust_difficulty(last_block,timestamp);
            let hash = Block::hash(block_number,timestamp,&last_hash , &data, &nonce,&difficulty);

            if hash.chars().take(difficulty).all(|c| c=='0'){
                return Block::new(
                    block_number,
                    timestamp,
                    last_hash,
                    hash,
                    data,
                    nonce,
                    difficulty,
                );
            }
            nonce+=1;
        }
    }

    pub fn hash(block_number:u64,timestamp:u128,last_hash:&str,data:&Vec<ChainTransaction>,nonce:&u64,difficulty:&usize)->String{
        let data_json = serde_json::to_string(data).expect("Failed to serialize for hashing");
        ChainUtil::hash(format!("{}{}{}{}{}{}",block_number,timestamp,last_hash,data_json,nonce,difficulty))
    }


    pub fn adjust_difficulty(last_block:&Block , current_time:u128)->usize{
        let difficulty =last_block.difficulty;
        if difficulty < 1{
            return 1;
        }
        if last_block.timestamp + MINE_RATE > current_time {
            difficulty + 1
        } else if difficulty > 1 {
            difficulty - 1
        } else {
            1
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result {
        write!{
            f,
            "Block -
            Number    : {}
            Timestamp : {}
            Last Hash : {}
            Hash      : {}
            Nonce     : {}  
            Difficulty: {}
            Data      : {:?}",
            self.block_number,
            self.timestamp,
            &self.last_hash[..10.min(self.last_hash.len())],
            &self.hash[..10.min(self.hash.len())],
            self.nonce,
            self.difficulty,
            self.data
        }
    }
}