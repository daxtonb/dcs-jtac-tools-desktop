use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

use futures_util::{future::join_all, lock::Mutex, SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message},
};

use super::{
    client_session::ClientSession, ClientsByIdRead, ClientsByIdWrite,
};

/// Hub for managing web socket communication.
pub struct WebSocketHub {
    clients_by_id_read: ClientsByIdRead,
    clients_by_id_write: ClientsByIdWrite,
    port: u16,
    next_client_id: Arc<AtomicU32>,
}

impl WebSocketHub {
    /// Instantiates a new `WebSocketHub` for a port.
    pub fn new(port: u16) -> WebSocketHub {
        WebSocketHub {
            clients_by_id_read: ClientsByIdRead::default(),
            clients_by_id_write: ClientsByIdWrite::default(),
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

            let clients_by_id_read = self.clients_by_id_read.clone();
            let clients_by_id_write = self.clients_by_id_write.clone();
            let client_id = self.next_client_id.fetch_add(1, Ordering::Relaxed);
            let (write_half, read_half) = ws_stream.split();
            let client_read = Arc::new(Mutex::new(read_half));
            let client_write = Arc::new(Mutex::new(write_half));

            // Spawn a task for handling and tracking the new client
            tokio::spawn(async move {
                let client_session = ClientSession::new(
                    client_id,
                    clients_by_id_read,
                    clients_by_id_write,
                    client_read,
                    client_write,
                )
                .await;
                Self::listen_to_client(client_session).await;
            });
        }

        Ok(())
    }

    /// Sends a message to all subscribers.
    pub async fn broadcast_message(&self, message: String) -> Result<(), Error> {
        // TODO: Explore the possibility of a more granular/semantic lock
        let clients = self.clients_by_id_write.lock().await;

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

   
    async fn listen_to_client(client_session: ClientSession) {
        // Here we're just looping to detect disconnection.
        while let Some(result) = client_session.client_read.lock().await.next().await {
            match result {
                Ok(_) => {
                    // TODO: handle incoming client messages
                }
                Err(e) => {
                    eprintln!(
                        "Error on WebSocket for client {:?}: {:?}",
                        client_session.client_id, e
                    );
                    break; // Exit gracefully
                }
            }
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use futures_util::sink::SinkExt;
    use futures_util::stream::StreamExt;
    use std::time::Duration;
    use tokio::time::timeout;
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

    #[tokio::test]
    async fn test_client_connect_broadcast_and_disconnect() {
        // Start WebSocketHub on an available port (e.g., 0 lets the OS choose the port).
        let hub = Arc::new(WebSocketHub::new(6655));
        let hub_clone = hub.clone();
        let port = hub.port;
        tokio::spawn(async move {
            hub.start().await.expect("Failed to start the WebSocketHub");
        });

        // Give the server a moment to start up.
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Connect a client to the WebSocketHub.
        let url = format!("ws://127.0.0.1:{}", port);
        let (mut ws_stream, _) = connect_async(url)
            .await
            .expect("Failed to connect to WebSocketHub");

        // Send a message from the client to test broadcasting.
        let test_message = "Hello, WebSocketHub!";
        ws_stream
            .send(Message::Text(test_message.to_string()))
            .await
            .expect("Failed to send message from client");

        // Broadcast a message to all clients, including the one we just connected.
        let broadcast_message = "Broadcast message from hub";
        hub_clone
            .broadcast_message(broadcast_message.to_string())
            .await
            .expect("Failed to broadcast message");

        // Try to receive the broadcast message on the client side.
        if let Ok(Some(message)) = timeout(Duration::from_secs(5), ws_stream.next()).await {
            match message {
                Ok(msg) => match msg {
                    Message::Text(text) => assert_eq!(text, broadcast_message),
                    _ => panic!("Received a non-text message."),
                },
                Err(e) => panic!("Error receiving message: {:?}", e),
            }
        } else {
            panic!("Did not receive broadcast message in time.");
        }

        // Disconnect the client.
        ws_stream
            .close(None)
            .await
            .expect("Failed to close the WebSocket stream");
    }
}
