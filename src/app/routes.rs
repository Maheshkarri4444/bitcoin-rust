use actix_web::{get,post,web,HttpResponse,Responder};
use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;
use crate::app::p2p_server::P2pServer;
use std::sync::{Arc,Mutex};
use tokio::sync::Mutex as TokioMutex;
use serde::Deserialize;

use crate::wallet::transaction_pool::TransactionPool;
use crate::wallet::wallet::Wallet;



#[get("/blocks")]
async fn get_blocks(
    blockchain: web::Data<Arc<TokioMutex<Blockchain>>>,
)-> impl Responder{
    let chain = blockchain.lock().await;
    HttpResponse::Ok().json(&chain.chain)
}

#[derive(Deserialize)]
struct MineRequest {
    data:Vec<String>,
}

#[post("/mine")]
async fn mine_block(
    data: web::Json<MineRequest>,
    blockchain: web::Data<Arc<TokioMutex<Blockchain>>>,
    p2p_server: web::Data<Arc<P2pServer>>,

    )->impl Responder {

    let mut chain = blockchain.lock().await;
    let new_block = chain.add_block(data.data.clone());
    println!("New block mined:\n{}",new_block);

    let server = p2p_server.clone();
    tokio::spawn(async move {
        server.sync_chains().await;
    });
    println!("here 1");
    HttpResponse::Found()
        .append_header(("Location","/blocks"))
        .finish()
}

#[get("/transactions")]
async fn get_transactions(
    transaction_pool: web::Data<Arc<TokioMutex<TransactionPool>>>,
) -> impl Responder {
    let pool = transaction_pool.lock().await;
    HttpResponse::Ok().json(&pool.transactions)
}

#[derive(Deserialize)]
struct TransactRequest {
    recipient:String,
    amount: u64,
}

#[post("/transact")]
async fn post_transaction(
    payload: web::Json<TransactRequest>,
    wallet: web::Data<Arc<Wallet>>,
    transaction_pool: web::Data<Arc<TokioMutex<TransactionPool>>>,
    p2p_server:web::Data<Arc<P2pServer>>,
) -> impl Responder {
    let recipient = payload.recipient.clone();
    let amount = payload.amount;

    let mut tp = transaction_pool.lock().await;

    match wallet.create_transaction(recipient,amount,&mut *tp){
        Some(ref transaction)=>{
            let _ = p2p_server.broadcast_transaction(transaction).await;
            HttpResponse::Found()
                .append_header(("Location","/transactions"))
                .finish()
        }
        None => {
            HttpResponse::BadRequest().body("Transaction creation failed insufficient balance or error")
        }
    }

}

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_blocks);
    cfg.service(mine_block);
    cfg.service(get_transactions);
    cfg.service(post_transaction);
}