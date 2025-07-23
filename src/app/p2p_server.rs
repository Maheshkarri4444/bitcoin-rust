// use std::sync::Arc;
// use tokio::sync::Mutex;
// use tokio::net::TcpListener;
// use tokio_tungstenite::{connect_async,accept_async, tungstenite::protocol::Message,WebSocketStream};
// use tokio_tungstenite::MaybeTlsStream;
// use futures::{StreamExt,SinkExt,future};
// use tokio::net::TcpStream;
// use url::Url;
// use tokio::time::{timeout,Duration};
// use serde_json::Value;
// use crate::blockchain::block::Block;

// use crate::blockchain::blockchain::Blockchain;

// //1st terminal cargo run
// //2nd terminal HTTP_PORT=3002 P2P_PORT=5002 PEERS=ws://127.0.0.1:5001 cargo run
// //3rd terminal HTTP_PORT=3003 P2P_PORT=5003 PEERS=ws://127.0.0.1:5001,ws://127.0.0.1:5002 cargo run



// pub struct P2pServer{
//     pub blockchain:Arc<Mutex<Blockchain>>,
//     pub peers:Vec<String>,
//     pub sockets: Arc<Mutex<Vec<Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>>>>,
// }

// impl P2pServer{
//     pub fn new(blockchain:Arc<Mutex<Blockchain>>,peers:Vec<String>)-> Self{
//         P2pServer{
//             blockchain,
//             peers,
//             sockets:Arc::new(Mutex::new(Vec::new())),
//         }
//     }

//     pub async fn listen(self: Arc<Self>,port:u16){
//         let addr = format!("127.0.0.1:{}",port);
//         let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

//         println!("Listening for peer to peer connections on: {}",addr);

//         let self_clone = Arc::clone(&self);
//         tokio::spawn(async move{
//             self_clone.connect_to_peers().await;
//         });

       

//         loop{
//             let (stream,_)=listener.accept().await.unwrap();
//             let stream = MaybeTlsStream::Plain(stream); 
//             // let ws_stream = WebSocketStream::from_raw_socket(stream, tokio_tungstenite::tungstenite::protocol::Role::Server, None).await;
//             let ws_stream = accept_async(stream).await.expect("Failed to accept WebSocket connection");
//             let server_clone = self.clone();
//             tokio::spawn(async move {
//                 server_clone.connect_socket(ws_stream).await;
//             });
        
//         }
//     }

//     async fn connect_to_peers(self:Arc<Self>) {
//         let peers = self.peers.clone();
//         println!("peers in connect to pers: {:?}", peers);
//         for peer in peers {
//             println!("connect to peer: {}", peer);
//             let result = timeout(Duration::from_secs(5), connect_async(Url::parse(&peer).unwrap())).await;
//             match result {
//                 Ok(Ok((ws_stream, _))) => {
//                     println!("Connected to peer: {}", peer);
//                     let server_clone = Arc::clone(&self);
//                     tokio::spawn(async move{
//                         server_clone.connect_socket(ws_stream).await;
//                     });
//                 }
//                 Ok(Err(e)) => {
//                     eprintln!("Failed to connect to peer {}: {}", peer, e);
//                 }
//                 Err(_) => {
//                     eprintln!("Timeout while connecting to peer {}", peer);
//                 }
//             }
//         }
//     }

//     async fn connect_socket(
//         self: Arc<Self>,
//         socket: tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>,
//     ){
//         println!("Peer socket connected!");
//         let socket = Arc::new(Mutex::new(socket));
//         self.sockets.lock().await.push(socket.clone());
//         // println!("sockets connected: {:?}",self.sockets);
       
//         self.send_chain(socket.clone()).await;
//         tokio::spawn(self.clone().message_handler(socket.clone()));
//     }

//     pub async fn send_chain(&self, socket:Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>){
//         let blockchain = self.blockchain.lock().await;
//         let chain_json = serde_json::to_string(&blockchain.chain).unwrap();
//         let mut locked_socket = socket.lock().await;
//         if let Err(e)=locked_socket.send(Message::Text(chain_json)).await{
//             eprintln!("Failed to send Blockchain: {}",e);
//         }
//     }

//     pub async fn sync_chains(&self){
//         println!("sync_chains called");
//         let sockets = self.sockets.lock().await;
//         let chain = {
//             let blockchain = self.blockchain.lock().await;
//             serde_json::to_string(&blockchain.chain).unwrap()
//         };

//         let futures = sockets.iter().map(|socket| {
//             let msg = chain.clone();
//             let socket = socket.clone();
//             async move {
//                 let mut locked_socket = socket.lock().await;
//                 if let Err(e) = locked_socket.send(Message::Text(msg)).await{
//                     eprintln!("Failed to send Blockchain: {}",e);
//                 }
//             }
//         });
//         futures::future::join_all(futures).await;
//     }

//     async fn message_handler(
//         self:Arc<Self>,
//         socket: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
//     ){      
//         println!("message_handler.called");
//         let socket_clone = socket.clone();
//         tokio::spawn( async move {
//                         let mut socket_locked = socket_clone.lock().await;
//                         while let Some(msg_result)=socket_locked.next().await{
//                             match msg_result {
//                                 Ok(msg) if msg.is_text()=>{
//                                     let text = msg.into_text().unwrap();
//                                     match serde_json::from_str::<Value>(&text){
//                                         Ok(data)=>{
//                                             let mut blockchain = self.blockchain.lock().await;
//                                             if let Ok(new_chain)=serde_json::from_value::<Vec<Block>>(data.clone()){
//                                                 println!("replacing_chain in message_hangler");
//                                                 blockchain.replace_chain(new_chain);
//                                             } else {
//                                                 println!("Failed to parse new chain from incoming message.");
//                                             }
//                                         }
//                                         Err(e)=>eprintln!("Invalid Json:{}",e),
                                
//                                     }
//                                 }
//                                 Ok(_)=>{}
//                                 Err(e)=>{
//                                     eprintln!("Websocket error: {}",e);
//                                     break;
//                                 }
//                             }
//                         }
//                         println!("its called");
//             }
//         );
//     }

// }




// pub fn start_p2p_server(port:String,peers:String)->Arc<P2pServer>{
//     let blockchain = Arc::new(Mutex::new(Blockchain::new()));
//     let peer_list:Vec<String>=peers
//         .split(',')
//         .map(|s| s.trim().to_string())
//         .filter(|s|!s.is_empty())
//         .collect();
//     println!("peers list : {:?}",peer_list);
//     let server = Arc::new(P2pServer::new(blockchain.clone(),peer_list));
//     let server_clone = Arc::clone(&server);
    
//     std::thread::spawn(move|| {
//         let runtime = tokio::runtime::Runtime::new().unwrap();
//         println!("peers: {}",peers);
//         runtime.block_on(async move {
//             let port_num = port.parse::<u16>().expect("Invalid port number");
//             server_clone.listen(port_num).await;
//         });
//     });
//     server
// }


use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio_tungstenite::{
    accept_async, connect_async,
    tungstenite::protocol::Message,
    WebSocketStream, MaybeTlsStream,
};
use futures::{StreamExt, SinkExt};
use tokio::net::TcpStream;
use url::Url;
use tokio::time::{timeout, Duration};
use serde_json::Value;

use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;

/// A peer write-half and its reading task handle
#[derive(Clone)]
struct PeerSocket {
    write: Arc<Mutex<futures::stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    // _read_handle: tokio::task::JoinHandle<()>, // Keep this if you want to manage read tasks, otherwise drop
}

pub struct P2pServer {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub peers: Vec<String>,
    pub sockets: Arc<Mutex<Vec<PeerSocket>>>,
}

impl P2pServer {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>, peers: Vec<String>) -> Self {
        Self {
            blockchain,
            peers,
            sockets: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn listen(self: Arc<Self>, port: u16) {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
        println!("Listening for peer to peer connections on: {}", addr);

        let self_clone = Arc::clone(&self);
        tokio::spawn(async move {
            self_clone.connect_to_peers().await;
        });

        while let Ok((stream, _)) = listener.accept().await {
            let stream = MaybeTlsStream::Plain(stream);
            let ws_stream = accept_async(stream)
                .await
                .expect("Failed to accept WebSocket connection");
            let self_clone = self.clone();
            tokio::spawn(async move {
                self_clone.connect_socket(ws_stream).await;
            });
        }
    }

    async fn connect_to_peers(self: Arc<Self>) {
        let peers = self.peers.clone();
        println!("peers in connect to peers: {:?}", peers);
        for peer in peers {
            println!("connect to peer: {}", peer);
            let result =
                timeout(Duration::from_secs(5), connect_async(Url::parse(&peer).unwrap())).await;
            match result {
                Ok(Ok((ws_stream, _))) => {
                    println!("Connected to peer: {}", peer);
                    let self_clone = Arc::clone(&self);
                    tokio::spawn(async move {
                        self_clone.connect_socket(ws_stream).await;
                    });
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

    async fn connect_socket(self: Arc<Self>, ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) {
        println!("Peer socket connected!");

        // Split websocket stream into write and read halves
        let (write, mut read) = ws_stream.split();

        let write = Arc::new(Mutex::new(write));

        // Spawn a task to handle incoming messages
        let self_clone = self.clone();
        tokio::spawn(async move {
            while let Some(msg_result) = read.next().await {
                match msg_result {
                    Ok(msg) if msg.is_text() => {
                        let text = msg.into_text().unwrap();
                        // println!("Received message: {}", text);

                        match serde_json::from_str::<Value>(&text) {
                            Ok(data) => {
                                let mut blockchain = self_clone.blockchain.lock().await;
                                if let Ok(new_chain) = serde_json::from_value::<Vec<Block>>(data.clone()) {
                                    println!("replacing_chain in message_handler");
                                    blockchain.replace_chain(new_chain);
                                } else {
                                    println!("Failed to parse new chain from incoming message.");
                                }
                            }
                            Err(e) => eprintln!("Invalid JSON: {}", e),
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                }
            }
            println!("Read loop ended for peer socket.");
        });

        // Save the write half handle so we can send messages to this socket later
        let peer_socket = PeerSocket { write };

        self.sockets.lock().await.push(peer_socket.clone());

        // Send current blockchain on new connection
        self.send_chain_to_socket(peer_socket.clone()).await;
    }

    async fn send_chain_to_socket(&self, peer: PeerSocket) {
        let blockchain = self.blockchain.lock().await;
        let chain_json = serde_json::to_string(&blockchain.chain).unwrap();
        drop(blockchain);

        let mut writer = peer.write.lock().await;
        if let Err(e) = writer.send(Message::Text(chain_json)).await {
            eprintln!("Failed to send blockchain to socket: {}", e);
        }
    }

    pub async fn sync_chains(&self) {
        println!("sync_chains called");
        let sockets = self.sockets.lock().await;
        let chain = {
            let blockchain = self.blockchain.lock().await;
            serde_json::to_string(&blockchain.chain).unwrap()
        };
        drop(chain.clone());
        let futures = sockets.iter().map(|peer| {
            let msg = chain.clone();
            let write = Arc::clone(&peer.write);
            async move {
                let mut locked_writer = write.lock().await;
                if let Err(e) = locked_writer.send(Message::Text(msg)).await {
                    eprintln!("Failed to send blockchain to peer socket: {}", e);
                }
            }
        });
        futures::future::join_all(futures).await;
    }
}
