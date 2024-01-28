use std::error::Error;
use tokio::{net::UdpSocket, task::JoinHandle};

use super::dcs_unit::DcsUnit;

pub const DCS_LISTENER_PORT: u16 = 34254;
pub const DCS_MSG_DELIMITER: u8 = b'\n';

/// Starts a thread that will continuously listen for the DCS units export.
/// 
/// # Arguments
/// * `units_handler` - Closure for handling any captured DCS units from the export.
pub async fn listen<F>(units_handler: F) -> Result<(), Box<dyn Error>>
where
    F: Fn(Vec<DcsUnit>) + Send + Sync + 'static,
{
    let socket = setup_socket().await?;
    start_receiving_loop(socket, units_handler).await;

    Ok(())
}

async fn start_receiving_loop<F>(socket: UdpSocket, units_handler: F) -> JoinHandle<()>
where
    F: Fn(Vec<DcsUnit>) + Send + Sync + 'static,
{
    let buffer = [0u8; 1024];

    tokio::spawn(async move {
        loop {
            match receive_next(&socket, buffer).await {
                Ok(units) => units_handler(units),
                Err(e) => {
                    println!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    })
}

async fn setup_socket() -> Result<UdpSocket, Box<dyn Error>> {
    UdpSocket::bind(("127.0.0.1", DCS_LISTENER_PORT))
        .await
        .map_err(|e| e.into())
}

async fn receive_next(
    socket: &UdpSocket,
    mut buffer: [u8; 1024],
) -> Result<Vec<DcsUnit>, Box<dyn Error>> {
    let mut current_msg = Vec::new();

    loop {
        let size = socket.recv(&mut buffer).await?;

        if let Some(position) = buffer[..size].iter().position(|&b| b == DCS_MSG_DELIMITER) {
            current_msg.extend_from_slice(&buffer[..position]);
            let msg = String::from_utf8_lossy(&current_msg);
            let units = serde_json::from_str::<Vec<DcsUnit>>(&msg)?;
            current_msg.clear();

            return Ok(units);
        } else {
            current_msg.extend_from_slice(&buffer[..size]);
        }
    }
}

#[cfg(test)]
mod integration_tests {

    use tokio::net::UdpSocket;

    use crate::dcs_listener::dcs_unit::DcsUnit;

    use super::{listen, DCS_LISTENER_PORT, DCS_MSG_DELIMITER};

    #[tokio::test]
    async fn test_listen() {
        let units = vec![DcsUnit {}, DcsUnit {}];
        let mut serialized_units = serde_json::to_string(&units).unwrap();
        serialized_units.push(DCS_MSG_DELIMITER as char);

        let units_handler = move |received_units: Vec<DcsUnit>| {
            assert_eq!(received_units.len(), units.len());

            for received_unit in received_units {
                assert!(&units.contains(&received_unit))
            }
        };

        // Start the listener
        let list_task = tokio::spawn(async move {
            listen(units_handler)
                .await
                .expect("Unable to start listener")
        });

        // Give the listener some time to set up
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Create a separate socket for sending messages
        let sender_socket = UdpSocket::bind("127.0.0.1:0") // Bind to an arbitrary available port
            .await
            .expect("Unable to create sender socket");
        sender_socket
            .connect(("127.0.0.1", DCS_LISTENER_PORT))
            .await
            .expect("Unable to connect to listener");

        // Send the message
        sender_socket
            .send(serialized_units.as_bytes())
            .await
            .expect("Unable to send message");

        // Wait for a bit to ensure the message is processed
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        list_task.await.unwrap();
    }
}
