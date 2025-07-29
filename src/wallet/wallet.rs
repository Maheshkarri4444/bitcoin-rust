use std::fmt;
use crate::config::INITIAL_BALANCE;
use k256::ecdsa::{SigningKey,VerifyingKey,Signature,signature::Signer};
use k256::EncodedPoint;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use crate::chain_util::ChainUtil;
use crate::wallet::transaction_pool::TransactionPool;
use crate::wallet::transaction::Transaction;
use crate::wallet::transaction::RewardTransaction;
use crate::blockchain::blockchain::Blockchain;
use std::sync::Arc;
use serde::{Serialize,Deserialize};
use tokio::sync::Mutex as TokioMutex;
use crate::wallet::transaction::ChainTransaction;
use tokio::sync::RwLock;

pub struct Wallet {
    pub balance: RwLock<u64>,
    pub key_pair: SigningKey,
    pub public_key: String, 
}


impl Wallet {
    pub fn new() -> Self{
        let (key_pair,verifying_key) = ChainUtil::gen_key_pair();
        let public_key_point:EncodedPoint = verifying_key.to_encoded_point(false);
        let public_key_hex = hex::encode(public_key_point.as_bytes());

    
        Self {
            balance:RwLock::new(INITIAL_BALANCE),
            key_pair,
            public_key:public_key_hex,
        }
    }
    
    pub fn sign(&self , data_hash: &str) ->Signature {
        let bytes = hex::decode(data_hash).expect("Invalid hex string for signing");
        self.key_pair.sign(&bytes)
    }

    pub async fn create_transaction(&self,recipient:String,amount:u64,transaction_pool:&mut TransactionPool,blockchain:&Arc<TokioMutex<Blockchain>>,)-> Option<Transaction>{
        let balance = Self::calculate_balance_for_address(blockchain,&self.public_key).await;
        {
            let mut balance_lock = self.balance.write().await;
            *balance_lock = balance;
        }
        if amount > balance {
            println!(
                "Amount: {} exceeds current balance: {}",
                amount , balance
            );
            return None;
        }
        if let Some(existing_tx) = transaction_pool.existing_transaction(&self.public_key){
            let mut tx = existing_tx.clone();
            if tx.update(self,recipient,amount,balance).is_some(){
                transaction_pool.update_or_add_transaction(tx.clone());
                Some(tx)
            }else{
                None
            }
        }else {
            if let Some(mut tx) = Transaction::new_transaction(self,recipient,amount,balance).await{
                transaction_pool.update_or_add_transaction(tx.clone());
                Some(tx)
            }else{
                None
            }
        }
    }



    pub async fn calculate_balance_for_address(
        blockchain: &Arc<TokioMutex<Blockchain>>,
        address:&str,
    ) -> u64 {
        let chain = blockchain.lock().await;
        let mut balance:u64 = 0;
        let mut transactions:Vec<ChainTransaction> = Vec::new();

        for block in &chain.chain{
            for tx in &block.data{
                transactions.push(tx.clone());
            }
        }

        let normal_txns_sent_by_address:Vec<&Transaction> = 
        transactions.iter().filter_map(|ct| {
            if let ChainTransaction::Normal(tx) = ct {
                if let Some(input) = &tx.input {
                    if input.address == address{
                        return Some(tx);
                    }
                }
            }
            None
        }).collect();
        let mut start_time = 0u128;
        if !normal_txns_sent_by_address.is_empty(){
            let recent_tx = normal_txns_sent_by_address
                .iter()
                .max_by_key(|tx| tx.input.as_ref().unwrap().timestamp)
                .unwrap();
            
            balance = recent_tx
                .outputs
                .iter()
                .find(|output| output.address == address)
                .map(|output| output.amount)
                .unwrap_or(0);
            
            start_time = recent_tx.input.as_ref().unwrap().timestamp;
        }

        for ct in &transactions {
            match ct {
                ChainTransaction::Normal(tx)=>{
                    if let Some(input) = &tx.input {
                        if input.timestamp > start_time {
                            for output in &tx.outputs {
                                if output.address == address{
                                    balance = balance.saturating_add(output.amount);
                                }
                            }
                        }
                    }
                }
                ChainTransaction::Reward(reward_tx)=>{
                    if reward_tx.output.address == address{
                        balance = balance.saturating_add(reward_tx.output.amount);
                    }
                }
            }
        }
        balance
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result {
        write!{
            f,
            "Wallet -\n publicKey: {}\n balance:{:?}",
            self.public_key,
            self.balance
        }
    }
}