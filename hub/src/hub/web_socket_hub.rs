use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use futures_util::{future::join_all, lock::Mutex, SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message},
    WebSocketStream,
};

type ClientsById = Arc<Mutex<HashMap<u32, Arc<Mutex<WebSocketStream<TcpStream>>>>>>;

/// Hub for managing web socket communication.
pub struct WebSocketHub {
    clients_by_id: ClientsById,
    port: u16,
    next_client_id: Arc<AtomicU32>,
}

impl WebSocketHub {
    /// Instantiates a new `WebSocketHub` for a port.
    pub fn new(port: u16) -> WebSocketHub {
        WebSocketHub {
            clients_by_id: ClientsById::default(),
            port: port,
            next_client_id: Arc::new(AtomicU32::new(0)),
        }
    }

    /// Initiates listening for subscribers.
    pub async fn start(&self) -> Result<(), Error> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;

        while let Ok((stream, _)) = listener.accept().await {
            let ws_stream = match accept_async(stream).await {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("Failed to establish a WebSocket connection: {:?}", e);
                    continue;
                }
            };

            let clients_by_id_clone = self.clients_by_id.clone();
            let next_client_id_clone = self.next_client_id.clone();
            // Spawn a task for handling the new client, passing only the necessary parts of `self`.
            tokio::spawn(async move {
                Self::handle_new_client(clients_by_id_clone, ws_stream, next_client_id_clone);
            });
        }

        Ok(())
    } 
    
    /// Sends a message to all subscribers.
    pub async fn broadcast_message(&self, message: String) -> Result<(), Error> {
        let clients = self.clients_by_id.lock().await;

        // Send messages to each client in parallel
        let futures: Vec<_> = clients
            .iter()
            .map(|(_, client)| {
                let message = message.clone();
                let client = client.clone();
                async move {
                    let mut client = client.lock().await;

                    // Messages are not anticipated to be large
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

    async fn handle_new_client(
        clients_by_id: ClientsById,
        ws_stream: WebSocketStream<TcpStream>,
        next_client_id: Arc<AtomicU32>,
    ) {
        let client_id = next_client_id.fetch_add(1, Ordering::Relaxed);
        let client_arc = Arc::new(Mutex::new(ws_stream));
        let client_arc_clone = client_arc.clone();

        {
            let mut clients = clients_by_id.lock().await;
            clients.insert(client_id, client_arc_clone);
        }

        println!("New client: {}", client_id);
        let ws_stream = client_arc.lock().await;

        Self::listen_to_client(ws_stream, client_id, clients_by_id).await;
    }

    async fn listen_to_client(
        mut ws_stream: futures_util::lock::MutexGuard<'_, WebSocketStream<TcpStream>>,
        client_id: u32,
        clients_by_id: Arc<Mutex<HashMap<u32, Arc<Mutex<WebSocketStream<TcpStream>>>>>>,
    ) {
        // Here we're just looping to detect disconnection.
        while let Some(result) = ws_stream.next().await {
            match result {
                Ok(_) => {
                    // TODO: handle incoming client messages
                }
                Err(e) => {
                    eprintln!("Error on WebSocket for client {}: {:?}", client_id, e);
                    break; // Exit gracefully
                }
            }
        }

        // If we get here, the client has disconnected
        Self::handle_client_disconnected(clients_by_id, client_id).await;
    }

    async fn handle_client_disconnected(
        clients_by_id: Arc<Mutex<HashMap<u32, Arc<Mutex<WebSocketStream<TcpStream>>>>>>,
        client_id: u32,
    ) {
        let mut clients_lock = clients_by_id.lock().await;
        clients_lock.remove(&client_id);
        println!("Client {} disconnected", client_id);
    }
}

#[cfg(test)]
mod integration_tests {}
