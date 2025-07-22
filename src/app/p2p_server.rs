use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio_tungstenite::{connect_async,accept_async, tungstenite::protocol::Message,WebSocketStream};
use tokio_tungstenite::MaybeTlsStream;
use futures::{StreamExt,SinkExt,future};
use tokio::net::TcpStream;
use url::Url;
use tokio::time::{timeout,Duration};
use serde_json::Value;

use crate::blockchain::blockchain::Blockchain;

//1st terminal cargo run
//2nd terminal HTTP_PORT=3002 P2P_PORT=5002 PEERS=ws://127.0.0.1:5001 cargo run
//3rd terminal HTTP_PORT=3003 P2P_PORT=5003 PEERS=ws://127.0.0.1:5001,ws://127.0.0.1:5002 cargo run



pub struct P2pServer{
    pub blockchain:Arc<Mutex<Blockchain>>,
    pub peers:Vec<String>,
    pub sockets:  Vec<Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
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

        self.connect_to_peers().await;

        loop{
            let (stream,_)=listener.accept().await.unwrap();
            let stream = MaybeTlsStream::Plain(stream); 
            // let ws_stream = WebSocketStream::from_raw_socket(stream, tokio_tungstenite::tungstenite::protocol::Role::Server, None).await;
            let ws_stream = accept_async(stream).await.expect("Failed to accept WebSocket connection");
            self.connect_socket(ws_stream).await;
        }
    }

    async fn connect_to_peers(&mut self) {
        let peers = self.peers.clone();
        println!("peers in connect to pers: {:?}", peers);
        for peer in peers {
            println!("connect to peer: {}", peer);
            let result = timeout(Duration::from_secs(5), connect_async(Url::parse(&peer).unwrap())).await;
            match result {
                Ok(Ok((ws_stream, _))) => {
                    println!("Connected to peer: {}", peer);
                    self.connect_socket(ws_stream).await;
                }
                Ok(Err(e)) => {
                    eprintln!("Failed to connect to peer {}: {}", peer, e);
                }
                Err(_) => {
                    eprintln!("Timeout while connecting to peer {}", peer);
                }
            }
        }
    }

    async fn connect_socket(
        &mut self,
        socket: tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>,
    ){
        println!("Peer socket connected!");
        let socket = Arc::new(Mutex::new(socket));
        self.sockets.push(socket.clone());

        self.message_handler(socket.clone()).await;

        let blockchain = self.blockchain.lock().await;
        let chain_json = serde_json::to_string(&blockchain.chain).unwrap();
        let mut locked_socket = socket.lock().await;
        if let Err(e)=locked_socket.send(Message::Text(chain_json)).await{
            eprintln!("Failed to send Blockchain: {}",e);
        }
    }

    async fn message_handler(
        &self,
        socket: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    ){
        tokio::spawn( async move {
                        let mut socket = socket.lock().await;
                        while let Some(msg_result)=socket.next().await{
                            match msg_result {
                                Ok(msg) if msg.is_text()=>{
                                    let text = msg.into_text().unwrap();
                                    match serde_json::from_str::<Value>(&text){
                                        Ok(data)=> println!("data: {:?}",data),
                                        Err(e)=>eprintln!("Invalid Json: {}",e),
                                    }
                                }
                                Ok(_)=>{}
                                Err(e)=>{
                                    eprintln!("Websocket error: {}",e);
                                    break;
                                }
                            }
                        }
            }
        );
    }

}




pub fn start_p2p_server(port:String,peers:String)->std::thread::JoinHandle<()>{
    std::thread::spawn(move|| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        println!("peers: {}",peers);
        runtime.block_on(async move {
            let blockchain = Arc::new(Mutex::new(Blockchain::new()));
            let peer_list:Vec<String>=peers
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s|!s.is_empty())
                .collect();
            println!("peers list : {:?}",peer_list);
            let mut server = P2pServer::new(blockchain.clone(),peer_list);
            let port_num = port.parse::<u16>().expect("Invalid port number");

            server.listen(port_num).await;
        })
    })
}