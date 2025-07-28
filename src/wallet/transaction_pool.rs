use crate::wallet::transaction::Transaction;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct TransactionPool{
    pub transactions:Vec<Transaction>,
}

impl TransactionPool {
    pub fn new() -> Self{
        Self{
            transactions:Vec::new(),
        }
    }

    pub fn update_or_add_transaction(&mut self,transaction:Transaction){
        if let Some(pos)=self.transactions.iter().position(|t| t.id == transaction.id){
            self.transactions[pos] = transaction;
        } else {
            self.transactions.push(transaction);
        }
    }

    pub fn existing_transaction(&self, address:&str)-> Option<&Transaction>{
        self.transactions.iter().find(|t|{
            if let Some(input) = &t.input{
                input.address == address
            } else {
                false
            }
        })
    }

    pub fn valid_transactions(&self)->Vec<&Transaction>{
        self.transactions.iter().filter_map(|transaction| {
            let output_total: u64 = transaction.outputs.iter().map(|output| output.amount).sum();
            if transaction.input.clone().unwrap().amount != output_total {
                println!("Invalid transaction from: {}",transaction.input.clone().unwrap().address);
                return None;
            }
            if !Transaction::verify_transaction(transaction){
                println!("invalid signature from: {}",transaction.input.clone().unwrap().address);
                return None;
            }

            Some(transaction)
        }).collect()
    }


}
