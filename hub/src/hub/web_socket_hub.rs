use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use futures_util::{future::join_all, lock::Mutex, SinkExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message},
    WebSocketStream,
};

type Clients = Arc<Mutex<HashMap<u32, Arc<Mutex<WebSocketStream<TcpStream>>>>>>;

/// Hub for managing web socket communication.
pub struct WebSocketHub {
    clients: Clients,
    port: u16,
    next_client_id: AtomicU32,
}

impl WebSocketHub {
    /// Instantiates a new `WebSocketHub` for a port.
    pub fn new(port: u16) -> WebSocketHub {
        WebSocketHub {
            clients: Clients::default(),
            port: port,
            next_client_id: AtomicU32::new(0),
        }
    }

    /// Initiates listening for subscribers.
    pub async fn start(&self) -> Result<(), Error> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;

        // Listen for new client connections
        while let Ok((stream, _)) = listener.accept().await {
            let ws_stream = accept_async(stream).await?;
            let clients_clone = self.clients.clone();
            self.handle_new_client(clients_clone, ws_stream).await;
        }

        Ok(())
    }

    /// Sends a message to all subscribers.
    pub async fn broadcast_message(&self, message: String) -> Result<(), Error> {
        let clients = self.clients.lock().await;

        // Send messages to each client in parallel
        let futures: Vec<_> = clients
            .iter()
            .map(|(_, client)| {
                let message = message.clone();
                let client = client.clone();
                async move {
                    let mut client = client.lock().await;
                    client.send(Message::text(message)).await
                }
            })
            .collect();

        join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    /// Track the client and increment the client ID counter.
    async fn handle_new_client(&self, clients: Clients, stream: WebSocketStream<TcpStream>) {
        // Atomically increment the client ID
        let client_id = self.next_client_id.fetch_add(1, Ordering::Relaxed) + 1;

        println!("New client: {}", client_id);
        let mut clients_lock = clients.lock().await;
        clients_lock.insert(client_id, Arc::new(Mutex::new(stream)));
    }
}
