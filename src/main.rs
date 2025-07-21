mod app;
mod blockchain;

use actix_web::{App,HttpServer};
use app::routes::config;
use std::env;
use dotenv::dotenv;
use std::thread;

#[actix_web::main]
async fn main()->std::io::Result<()>{
    dotenv().ok();

    let http_port = env::var("HTTP_PORT").unwrap_or_else(|_|"3001".into());
    let p2p_port = env::var("P2P_PORT").unwrap_or_else(|_|"5001".into());
    let peers =env::var("PEERS").unwrap_or_else(|_|"".into());

    println!("Starting HTTP server on http://localhost:{}",http_port);
    println!("Starting P2P server on http://localhost:{}",p2p_port);

    let p2p_port_clone = p2p_port.clone();
    let peers_clone = peers.clone();
    thread::spawn(move||{
        app::p2p_server::start_p2p_server(p2p_port_clone,peers_clone);
    });


    HttpServer::new(||{
        App::new()
            .configure(config)
    })
    .bind(format!("127.0.0.1:{}",http_port)).expect("unable to start server")
    .run()
    .await

}