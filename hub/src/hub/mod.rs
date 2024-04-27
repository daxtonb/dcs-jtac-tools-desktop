mod client_session;
pub mod web_socket_hub;

use std::sync::Arc;

use futures_util::{
    lock::Mutex,
    stream::{SplitSink, SplitStream},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use self::web_socket_hub::WebSocketHub;

pub type ReadHalf = SplitStream<WebSocketStream<TcpStream>>;
pub type WriteHalf = SplitSink<WebSocketStream<TcpStream>, Message>;
pub type ClientRead = Arc<Mutex<ReadHalf>>;
pub type ClientWrite = Arc<Mutex<WriteHalf>>;
pub type ClientMessageHandlerFn = Arc<dyn Fn(&str, &str) + Send + Sync + 'static>;
pub type HostClientMessageHandlerFn = Arc<dyn Fn(Arc<WebSocketHub>, &str, &str) + Send + Sync + 'static>;
