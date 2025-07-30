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
    address: &str,
) -> u64 {
    let chain = blockchain.lock().await;

    // 1) Scan to find the MOST RECENT outgoing spend by `address`.
    //    We track both the tx.input timestamp and the block timestamp containing it.
    let mut last_spend_input_ts: u128 = 0;
    let mut last_spend_block_ts: u128 = 0;
    let mut balance_after_last_spend: u64 = 0;

    for block in &chain.chain {
        for ct in &block.data {
            if let ChainTransaction::Normal(tx) = ct {
                if let Some(input) = &tx.input {
                    if input.address == address {
                        // Is this spend newer than what we have recorded so far?
                        if input.timestamp > last_spend_input_ts
                            || (input.timestamp == last_spend_input_ts
                                && block.timestamp > last_spend_block_ts)
                        {
                            last_spend_input_ts = input.timestamp;
                            last_spend_block_ts = block.timestamp;

                            // The balance right after this spend is the "change" output (if any).
                            balance_after_last_spend = tx
                                .outputs
                                .iter()
                                .find(|o| o.address == address)
                                .map(|o| o.amount)
                                .unwrap_or(0);
                        }
                    }
                }
            }
        }
    }

    // 2) Starting from the block AFTER the last spend, add ALL credits to `address`:
    //    - any normal tx outputs to `address`
    //    - any reward outputs to `address`
    //
    //    NOTE: We filter by block.timestamp > last_spend_block_ts to avoid
    //    re-adding earlier rewards/credits that are already reflected in the change.
    let mut balance = balance_after_last_spend;

    for block in &chain.chain {
        if block.timestamp <= last_spend_block_ts {
            continue;
        }
        for ct in &block.data {
            match ct {
                ChainTransaction::Normal(tx) => {
                    for o in &tx.outputs {
                        if o.address == address {
                            balance = balance.saturating_add(o.amount);
                        }
                    }
                }
                ChainTransaction::Reward(r) => {
                    if r.output.address == address {
                        balance = balance.saturating_add(r.output.amount);
                    }
                }
            }
        }
    }

    // 3) If there was NO spend, last_spend_block_ts == 0 and balance_after_last_spend == 0,
    //    so the loop above adds all credits from the beginning (block timestamps are > 0
    //    except genesis), which is the intended behavior for a never-spent address.
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