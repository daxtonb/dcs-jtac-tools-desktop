use super::{ClientRead, ClientWrite, ClientsByIdRead, ClientsByIdWrite};

pub struct ClientSession {
    pub client_id: u32,
    clients_by_id_read: ClientsByIdRead,
    clients_by_id_write: ClientsByIdWrite,
    pub client_read: ClientRead,
    pub client_write: ClientWrite,
}

impl ClientSession {
    pub async fn new(
        client_id: u32,
        clients_by_id_read: ClientsByIdRead,
        clients_by_id_write: ClientsByIdWrite,
        client_read: ClientRead,
        client_write: ClientWrite,
    ) -> Self {
        {
            clients_by_id_read
                .lock()
                .await
                .insert(client_id, client_read.clone());
        }
        {
            clients_by_id_write
                .lock()
                .await
                .insert(client_id, client_write.clone());
        }

        Self {
            client_id,
            clients_by_id_read,
            clients_by_id_write,
            client_read,
            client_write,
        }
    }
}

impl Drop for ClientSession {
    fn drop(&mut self) {
        let client_id = self.client_id;
        let clients_by_id_read = self.clients_by_id_read.clone();
        let clients_by_id_write = self.clients_by_id_write.clone();

        tokio::spawn(async move {
            {
                let mut clients = clients_by_id_read.lock().await;
                clients.remove(&client_id);
            }
            {
                let mut clients = clients_by_id_write.lock().await;
                clients.remove(&client_id);
            }
            println!("Client {} disconnected", client_id);
        });
    }
}
