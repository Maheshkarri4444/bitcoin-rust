mod app;
mod blockchain;

use actix_web::{App,HttpServer};
use app::routes::config;

#[actix_web::main]
async fn main()->std::io::Result<()>{
    println!("Starting server on http://localhost3001");

    HttpServer::new(||{
        App::new()
            .configure(config)
    })
    .bind(("127.0.0.1",3001)).expect("error while running the server")
    .run()
    .await

}