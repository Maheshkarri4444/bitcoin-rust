use crate::blockchain::blockchain::Blockchain;
use crate::wallet::transaction_pool::TransactionPool;
use crate::wallet::wallet::Wallet;
use crate::app::p2p_server::P2pServer;


pub struct Miner<'a>{
    pub blockchain:&'a Blockchain,
    pub transaction_pool: &'a TransactionPool,
    pub wallet:&'a Wallet,
    pub p2p_server:&'a P2pServer,
}

impl<'a>Miner<'a>{
    pub fn new(
        blockchain:&'a Blockchain,
        transaction_pool: &'a TransactionPool,
        wallet:&'a Wallet,
        p2p_server:&'a P2pServer,
    ) -> Self {
        Self {
            blockchain,
            transaction_pool,
            wallet,
            p2p_server,
        }
    }

    pub fn mine(&self){
        let valid_transactions = self.transaction_pool.valid_transactions();

        println!("Mining Block with {} valid transctions.",valid_transactions.len());
    }


}