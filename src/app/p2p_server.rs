use std::sync::{Arc,Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures::{StreamExt,SinkExt};
use tokio::net::TcpStream;

use crate::blockchain::blockchain::Blockchain;

pub struct P2pServer{
    pub blockchain:Arc<Mutex<Blockchain>>,
    pub peers:Vec<String>,
    pub sockets: Vec<tokio_tungstenite::WebSocketStream<TcpStream>>,
}

impl P2pServer{
    pub fn new(blockchain:Arc<Mutex<Blockchain>>,peers:Vec<String>)-> Self{
        P2pServer{
            blockchain,
            peers,
            sockets:vec![],
        }
    }

    pub async fn listen(&mut self,port:u16){
        let addr = format!("127.0.0.1:{}",port);
        let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

        println!("Listening for peer to peer connections on: {}",addr);
        loop{
            let (stream,_)=listener.accept().await.unwrap();
            let ws_stream = accept_async(stream).await.unwrap();

            self.connect_socket(ws_stream).await;
        }
    }

    async fn connect_socket(
        &mut self,
        socket: tokio_tungstenite::WebSocketStream<TcpStream>,
    ){
        println!("Peer socket connected!");
        self.sockets.push(socket);
    }
}

pub fn start_p2p_server(port:String,peers:String)->std::thread::JoinHandle<()>{
    std::thread::spawn(move|| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async move {
            let blockchain = Arc::new(Mutex::new(Blockchain::new()));
            let peer_list:Vec<String>=peers
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s|!s.is_empty())
                .collect();
            
            let mut server = P2pServer::new(blockchain.clone(),peer_list);
            let port_num = port.parse::<u16>().expect("Invalid port number");

            server.listen(port_num).await;
        })
    })
}