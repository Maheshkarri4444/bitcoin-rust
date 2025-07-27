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
}
