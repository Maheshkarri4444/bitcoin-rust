use k256::ecdsa::{VerifyingKey};
use uuid::Uuid;
use std::collections::HashMap;
use crate::chain_util::ChainUtil;
use crate::wallet::wallet::Wallet;

pub struct Transaction {
    pub id:String,
    pub input: Option<String>,
    pub outputs: Vec<Output>,
}

#[derive(Clone)]
pub struct Output {
    pub amount: u64,
    pub address: String,
}


impl Transaction{
    pub fn new_transaction(sender_wallet: &Wallet,recipient:String,amount:u64)->Option<Self>{
        if amount > sender_wallet.balance {
            eprintln!("Amount: {} exceeds balance.",amount);
            return None;
        }
        let mut outputs = Vec::new();

        outputs.push(Output{
            amount:sender_wallet.balance - amount,
            address: sender_wallet.public_key.clone(),
        });

        outputs.push(Output{
            amount,
            address:recipient,
        });

        Some(Transaction{
            id:ChainUtil::id(),
            input:None,
            outputs,
        })
    }
}
