mod client_session;
pub mod web_socket_hub;

use std::sync::Arc;

use futures_util::{
    lock::Mutex,
    stream::{SplitSink, SplitStream},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

pub type ReadHalf = SplitStream<WebSocketStream<TcpStream>>;
pub type WriteHalf = SplitSink<WebSocketStream<TcpStream>, Message>;
pub type ClientRead = Arc<Mutex<ReadHalf>>;
pub type ClientWrite = Arc<Mutex<WriteHalf>>;
