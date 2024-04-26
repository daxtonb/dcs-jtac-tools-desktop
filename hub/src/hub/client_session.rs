use std::{collections::HashSet, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::sync::{mpsc::Receiver, Mutex};
use tokio_tungstenite::tungstenite::Message;

use crate::common::messaging::MESSAGE_TOPIC_DELIMITER;

use super::{ClientRead, ClientWrite};

/// Encapsulates client data needed for starting and ending sessions.
pub struct ClientSession {
    pub client_id: u32,
    pub client_read: ClientRead,
    pub client_write: ClientWrite,
    host_read: Arc<Mutex<Receiver<String>>>,
    subscribed_topics: Arc<Mutex<HashSet<String>>>,
}

impl ClientSession {
    /// Creates a new instance of `ClientSession` and adds it to the `ClientRead` and `ClientWrite` hash tables.
    pub fn new(
        client_id: u32,
        client_read: ClientRead,
        client_write: ClientWrite,
        host_read: Arc<Mutex<Receiver<String>>>,
    ) -> Self {
        println!("Client {} connected", client_id);

        let session = Self {
            client_id,
            client_read,
            client_write,
            host_read,
            subscribed_topics: Arc::new(Mutex::new(HashSet::new())),
        };

        session
    }

    pub async fn listen_to_client(&self) {
        while let Some(result) = self.client_read.lock().await.next().await {
            match result {
                Ok(message) => {
                    if let Message::Text(text) = message {
                        if let Some((topic, body)) = text.split_once(MESSAGE_TOPIC_DELIMITER) {
                            if topic.to_string() == "SUBSCRIBE" {
                                let body = body.to_string();
                                self.subscribe_topic(body).await;
                            }
                        }
                    }
                }
                Err(err) => eprintln!("Failed to receive message from client: {:?}", err),
            }
        }
    }

    pub async fn listen_to_host(&self) {
        while let Some(message) = self.host_read.lock().await.recv().await {
            println!("Sending message to client {}: {}", self.client_id, message);
            if let Some((topic, body)) = message.split_once(MESSAGE_TOPIC_DELIMITER) {
                if self.is_subscribed(&topic.to_string()).await {
                    if let Err(err) = self
                        .client_write
                        .lock()
                        .await
                        .send(Message::text(body))
                        .await
                    {
                        eprintln!(
                            "Client {} failed to send message from host: {:?}",
                            self.client_id, err
                        );
                    }
                }
            }
        }
    }

    async fn subscribe_topic(&self, topic: String) {
        println!("Client {} subscribed to {}", self.client_id, topic);
        self.subscribed_topics.lock().await.insert(topic);
    }

    async fn is_subscribed(&self, topic: &String) -> bool {
        self.subscribed_topics.lock().await.contains(topic)
    }
}

impl Drop for ClientSession {
    fn drop(&mut self) {
        println!("Client {} disconnected", self.client_id);
    }
}
