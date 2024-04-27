use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

use futures_util::{lock::Mutex, stream, StreamExt};
use tokio::{
    net::TcpListener,
    sync,
    sync::mpsc::{self, channel, Receiver, Sender},
};
use tokio_tungstenite::{accept_async, tungstenite::Error};

use crate::{common::messaging::MESSAGE_TOPIC_DELIMITER, hub::ClientMessageHandlerFn};

use super::{client_session::ClientSession, HostClientMessageHandlerFn};

/// Hub for managing web socket communication.
pub struct WebSocketHub {
    port: u16,
    next_client_id: Arc<AtomicU32>,
    message_sender: mpsc::Sender<String>,
    client_senders: Arc<Mutex<Vec<Sender<String>>>>,
    client_message_handler: Option<HostClientMessageHandlerFn>,
}

impl WebSocketHub {
    /// Instantiates a new `WebSocketHub` for a port.
    pub fn new(
        port: u16,
        client_message_handler: Option<HostClientMessageHandlerFn>,
    ) -> WebSocketHub {
        let (message_sender, message_receiver) = channel(1024);

        let hub = WebSocketHub {
            port: port,
            next_client_id: Arc::new(AtomicU32::new(0)),
            message_sender: message_sender,
            client_senders: Arc::new(Mutex::new(Vec::new())),
            client_message_handler,
        };

        hub.start_broadcast_task(message_receiver);
        hub
    }

    /// Initiates listening for subscribers.
    pub async fn start(self: Arc<Self>) -> Result<(), Error> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;

        while let Ok((stream, _)) = listener.accept().await {
            println!("Attempting to connect client...");
            let ws_stream = match accept_async(stream).await {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("Failed to establish a WebSocket connection: {:?}", e);
                    continue;
                }
            };

            let (tx, rx) = mpsc::channel(1024);
            self.client_senders.lock().await.push(tx);

            let client_id = self.next_client_id.fetch_add(1, Ordering::Relaxed);
            let (write_half, read_half) = ws_stream.split();
            let client_read = Arc::new(Mutex::new(read_half));
            let client_write = Arc::new(Mutex::new(write_half));

            let self_clone = self.clone();
            let client_message_handler: Option<ClientMessageHandlerFn> = match self.client_message_handler.clone() {
                Some(handler) => Some(Arc::new(move |topic: &str, body: &str| {
                    (handler)(self_clone.clone(), topic, body)
                })),
                None => None,
            };
            

            let client_session = Arc::new(ClientSession::new(
                client_id,
                client_read,
                client_write,
                Arc::new(sync::Mutex::new(rx)),
                client_message_handler,
            ));

            Self::start_client_listen_task(client_session.clone()).await;
            Self::start_host_listen_task(client_session).await;
        }

        Ok(())
    }

    /// Sends a message to all subscribers.
    pub fn broadcast_message(&self, topic: String, message: String) {
        let message_sender = self.message_sender.clone();
        tokio::spawn(async move {
            println!(
                "Received topic/message from host: {}{}{}",
                topic, MESSAGE_TOPIC_DELIMITER, message
            );
            match message_sender
                .send(format!("{}{}{}", topic, MESSAGE_TOPIC_DELIMITER, message))
                .await
            {
                Ok(_) => println!("Message sent to clients: {}", message),
                Err(err) => eprintln!("Failed to send message to clients: {}", err),
            }
        });
    }

    async fn start_client_listen_task(client_session: Arc<ClientSession>) {
        tokio::spawn(async move {
            client_session.listen_to_client().await;
        });
    }

    async fn start_host_listen_task(client_session: Arc<ClientSession>) {
        tokio::spawn(async move {
            client_session.listen_to_host().await;
        });
    }

    fn start_broadcast_task(&self, mut message_receiver: Receiver<String>) {
        let clients = self.client_senders.clone();
        tokio::spawn(async move {
            while let Some(message) = message_receiver.recv().await {
                let clients = clients.lock().await;
                stream::iter(clients.iter())
                    .for_each(|client| {
                        let message = message.clone();
                        async move {
                            if let Err(err) = client.send(message).await {
                                eprint!("Failed to send message to client: {:?}", err);
                            };
                        }
                    })
                    .await;
            }
        });
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::common::messaging::MESSAGE_TOPIC_DELIMITER;

    use super::*;
    use futures_util::sink::SinkExt;
    use futures_util::stream::StreamExt;
    use std::time::Duration;
    use tokio::time::timeout;
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

    #[tokio::test]
    async fn test_client_connect_broadcast_and_disconnect() {
        // Start WebSocketHub on an available port
        let hub = Arc::new(WebSocketHub::new(6655, None));
        let hub_clone = hub.clone();
        let port = { hub.port };
        let topic = "UNITS".to_string();
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

        // Subscribe the client to a topic
        let test_message = format!("SUBSCRIBE{}{}", MESSAGE_TOPIC_DELIMITER, topic);
        ws_stream
            .send(Message::Text(test_message.to_string()))
            .await
            .expect("Failed to send message from client");

        // Allow the server a moment to subscribe the user
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Broadcast a message to all clients, including the one we just connected.
        let broadcast_message = "Broadcast message from hub";
        hub_clone.broadcast_message(topic, broadcast_message.to_string());

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

    #[tokio::test]
    async fn test_client_ignores_unsubscribed_message() {
        // Start WebSocketHub on an available port
        let hub = Arc::new(WebSocketHub::new(6654, None));
        let hub_clone = hub.clone();
        let port = { hub.port };
        let subscribed_topic = "TOPIC1".to_string();
        let unsubscribed_topic = "TOPIC2".to_string();
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

        // Subscribe the client to a topic
        let test_message = format!("SUBSCRIBE{}{}", MESSAGE_TOPIC_DELIMITER, subscribed_topic);
        ws_stream
            .send(Message::Text(test_message.to_string()))
            .await
            .expect("Failed to send message from client");

        // Allow the server a moment to subscribe the user
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Broadcast a message to all clients for the unsubscribed topic
        let broadcast_message = "Broadcast message from hub";
        hub_clone.broadcast_message(unsubscribed_topic, broadcast_message.to_string());

        // Verify client did not receive message for unsubscribed topic
        if let Ok(Some(message)) = timeout(Duration::from_millis(100), ws_stream.next()).await {
            match message {
                Ok(_) => panic!("Received message from unsubscribed topic!"),
                Err(e) => panic!("Error receiving message: {:?}", e),
            }
        }
    }

    #[tokio::test]
    async fn test_client_message_handling() {
        let client_message_handler = |_: Arc<WebSocketHub>, topic: &str, body: &str| {
            assert_eq!("SOME_TOPIC", topic);
            assert_eq!("Hello, host!", body);
        };

        // Start WebSocketHub on an available port
        let hub = Arc::new(WebSocketHub::new(
            6653,
            Some(Arc::new(client_message_handler)),
        ));
        let port = { hub.port };
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

        if let Err(err) = ws_stream
            .send(Message::Text(format!(
                "SOME_TOPIC{}Hello, host!",
                MESSAGE_TOPIC_DELIMITER
            )))
            .await
        {
            panic!("Error sending message: {:?}", err);
        }

        // Allow the server a moment to process the message
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
