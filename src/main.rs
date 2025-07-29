mod app;
mod blockchain;
mod config;
mod chain_util;
mod wallet;

use actix_web::{App,HttpServer,web};
use app::routes::config;
use std::env;
use dotenv::dotenv;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use blockchain::blockchain::Blockchain;
use crate::wallet::transaction_pool::TransactionPool;
use crate::wallet::wallet::Wallet;
use app::p2p_server::P2pServer;
use crate::app::miner::Miner;

#[actix_web::main]
async fn main()->std::io::Result<()>{
    dotenv().ok();

    let http_port = env::var("HTTP_PORT").unwrap_or_else(|_|"3001".into());
    let p2p_port = env::var("P2P_PORT").unwrap_or_else(|_|"5001".into());
    let peers =env::var("PEERS").unwrap_or_else(|_|"".into());

    println!("Starting HTTP server on http://localhost:{}",http_port);
    println!("Starting P2P server on http://localhost:{}",p2p_port);

    let peer_list: Vec<String>= peers
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let blockchain = Arc::new(TokioMutex::new(Blockchain::new()));
    let transaction_pool = Arc::new(TokioMutex::new(TransactionPool::new()));
    let wallet = Arc::new(Wallet::new());


    let p2p_server = Arc::new(P2pServer::new(
        blockchain.clone(),
        transaction_pool.clone(),
        peer_list,
    ));

    let miner = Arc::new(TokioMutex::new(Miner::new(
        blockchain.clone(),
        transaction_pool.clone(),
        wallet.clone(),
        p2p_server.clone(),
    )));
    
    {
        let p2p_server = p2p_server.clone();
        let p2p_port = p2p_port.clone();
        std::thread::spawn(move||{
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async move {
                let port_num = p2p_port.parse::<u16>().expect("Invalid port nubmer");
                p2p_server.listen(port_num).await;
            });
        });
    }

    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .app_data(web::Data::new(p2p_server.clone()))
            .app_data(web::Data::new(transaction_pool.clone()))
            .app_data(web::Data::new(wallet.clone()))
            .app_data(web::Data::new(miner.clone()))
            .configure(config)
    })
    .bind(format!("127.0.0.1:{}",http_port)).expect("unable to start server")
    .run()
    .await

}