use actix_web::{get,post,web,HttpResponse,Responder};
use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;
use std::sync::Mutex;
use serde::Deserialize;

lazy_static::lazy_static!{
    static ref BLOCKCHAIN: Mutex<Blockchain>=Mutex::new(Blockchain::new());
}

#[get("/blocks")]
async fn get_blocks()-> impl Responder{
    let chain = BLOCKCHAIN.lock().unwrap();
    HttpResponse::Ok().json(&chain.chain)
}

#[derive(Deserialize)]
struct MineRequest {
    data:String,
}

#[post("/mine")]
async fn mine_block(data: web::Json<MineRequest>)->impl Responder {
    let mut chain = BLOCKCHAIN.lock().unwrap();
    let new_block = chain.add_block(data.data.clone());
    println!("New block mined:\n{}",new_block);
    HttpResponse::Found()
        .append_header(("Location","/blocks"))
        .finish()
}

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_blocks);
    cfg.service(mine_block);
}