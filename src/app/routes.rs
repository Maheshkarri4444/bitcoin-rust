use actix_web::{get,post,web,HttpResponse,Responder};
use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;
use crate::app::p2p_server::P2pServer;
use std::sync::{Arc,Mutex};
use tokio::sync::Mutex as TokioMutex;
use serde::Deserialize;
use std::cmp;

use crate::wallet::transaction_pool::TransactionPool;
use crate::wallet::wallet::Wallet;
use crate::app::miner::Miner;
use crate::wallet::transaction::ChainTransaction;


#[get("/blocks")]
async fn get_blocks(
    blockchain: web::Data<Arc<TokioMutex<Blockchain>>>,
)-> impl Responder{
    let chain = blockchain.lock().await;
    HttpResponse::Ok().json(&chain.chain)
}

#[derive(Deserialize)]
struct MineRequest {
    data:Vec<ChainTransaction>,
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
    blockchain:web::Data<Arc<TokioMutex<Blockchain>>>,
    p2p_server:web::Data<Arc<P2pServer>>,
) -> impl Responder {
    let recipient = payload.recipient.clone();
    let amount = payload.amount;

    let mut tp = transaction_pool.lock().await;

    match wallet.create_transaction(recipient,amount,&mut *tp,&blockchain).await{
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

#[get("/mine-transactions")]
async fn mine_transactions(
    miner: web::Data<Arc<TokioMutex<Miner>>>,
)->impl Responder{
    let mut miner = miner.lock().await;
    let block = miner.mine().await;
    println!("finished mine block");

    HttpResponse::Found()
        .append_header(("Location","/blocks"))
        .finish()
}

#[get("/publickey")]
async fn get_public_key(wallet: web::Data<Arc<Wallet>>)->impl Responder{
    HttpResponse::Ok().json(serde_json::json!({
        "public_key":wallet.public_key
    }))
}

#[get("/balance")]
async fn get_self_balance(
    wallet: web::Data<Arc<Wallet>>,
    blockchain: web::Data<Arc<TokioMutex<Blockchain>>>
)->impl Responder{
    let address = &wallet.public_key;
    let balance = Wallet::calculate_balance_for_address(&blockchain,address).await;
    HttpResponse::Ok().json(serde_json::json!({
        "address":address,
        "balance":balance
    }))
}


#[get("/balance/{pubkey}")]
async fn get_balance_by_pubkey(
    path: web::Path<String>,
    blockchain: web::Data<Arc<TokioMutex<Blockchain>>>
) -> impl Responder {
    let address = path.into_inner();
    let balance = Wallet::calculate_balance_for_address(&blockchain, &address).await;
    HttpResponse::Ok().json(serde_json::json!({
        "address": address,
        "balance": balance
    }))
}


pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_blocks);
    cfg.service(mine_block);
    cfg.service(get_transactions);
    cfg.service(post_transaction);
    cfg.service(mine_transactions);
    cfg.service(get_public_key);
    cfg.service(get_self_balance);
    cfg.service(get_balance_by_pubkey);
}