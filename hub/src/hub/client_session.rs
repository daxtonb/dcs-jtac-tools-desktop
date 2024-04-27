use std::{collections::HashSet, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::sync::{mpsc::Receiver, Mutex};
use tokio_tungstenite::tungstenite::Message;

use crate::common::messaging::MESSAGE_TOPIC_DELIMITER;

use super::{ClientMessageHandlerFn, ClientRead, ClientWrite};

/// Encapsulates client data needed for starting and ending sessions.
pub struct ClientSession {
    pub client_id: u32,
    pub client_read: ClientRead,
    pub client_write: ClientWrite,
    host_read: Arc<Mutex<Receiver<String>>>,
    subscribed_topics: Arc<Mutex<HashSet<String>>>,
    client_message_handler: Option<ClientMessageHandlerFn>,
}

impl ClientSession {
    /// Creates a new instance of `ClientSession` and adds it to the `ClientRead` and `ClientWrite` hash tables.
    pub fn new(
        client_id: u32,
        client_read: ClientRead,
        client_write: ClientWrite,
        host_read: Arc<Mutex<Receiver<String>>>,
        client_message_handler: Option<ClientMessageHandlerFn>,
    ) -> Self {
        println!("Client {} connected", client_id);

        let session = Self {
            client_id,
            client_read,
            client_write,
            host_read,
            subscribed_topics: Arc::new(Mutex::new(HashSet::new())),
            client_message_handler,
        };

        session
    }

    /// Initiates listening for messages from the client
    pub async fn listen_to_client(&self) {
        while let Some(result) = self.client_read.lock().await.next().await {
            match result {
                Ok(message) => self.handle_message_from_client(message).await,
                Err(err) => eprintln!("Failed to receive message from client: {:?}", err),
            }
        }
    }

    /// Initiates listening for messages from the host
    pub async fn listen_to_host(&self) {
        while let Some(message) = self.host_read.lock().await.recv().await {
            self.handle_message_from_host(message).await;
        }
    }

    async fn handle_message_from_client(
        &self,
        message: Message
    ) {
        match message {
            Message::Text(text) => match text.split_once(MESSAGE_TOPIC_DELIMITER) {
                Some((topic, body)) => {
                    self.manage_subscription(topic, body.to_string()).await;
                    if let Some(handler) = &self.client_message_handler {
                        handler(topic, body);
                    }
                }
                None => eprintln!(
                    "Message from client {} does not have the expected delimiter: {}",
                    self.client_id, MESSAGE_TOPIC_DELIMITER
                ),
            },
            _ => eprintln!(
                "Message from client {} was not in the expected text format",
                self.client_id
            ),
        }
    }

    async fn handle_message_from_host(&self, message: String) {
        println!("Sending message to client {}: {}", self.client_id, message);
        match message.split_once(MESSAGE_TOPIC_DELIMITER) {
            Some((topic, body)) => {
                if self.is_subscribed(topic.to_string()).await {
                    self.send_host_message_to_client(body).await;
                }
            }
            None => eprintln!(
                "Message from host to client {} does not have the expected delimiter: {}",
                self.client_id, MESSAGE_TOPIC_DELIMITER
            ),
        }
    }

    async fn manage_subscription(&self, topic: &str, body: String) {
        if topic == "SUBSCRIBE" {
            self.subscribe_topic(body).await;
        } else if topic == "UNSUBSCRIBE" {
            self.unsubscribe_topic(body).await
        }
    }

    async fn subscribe_topic(&self, topic: String) {
        println!("Client {} subscribed to {}", self.client_id, topic);
        self.subscribed_topics.lock().await.insert(topic);
    }

    async fn unsubscribe_topic(&self, topic: String) {
        println!("Client {} unsubscribed from {}", self.client_id, topic);
        self.subscribed_topics.lock().await.remove(&topic);
    }

    async fn is_subscribed(&self, topic: String) -> bool {
        self.subscribed_topics.lock().await.contains(&topic)
    }

    async fn send_host_message_to_client(&self, message: &str) {
        if let Err(err) = self
            .client_write
            .lock()
            .await
            .send(Message::text(message))
            .await
        {
            eprintln!(
                "Client {} failed to send message from host: {:?}",
                self.client_id, err
            );
        }
    }
}

impl Drop for ClientSession {
    fn drop(&mut self) {
        println!("Client {} disconnected", self.client_id);
    }
}
