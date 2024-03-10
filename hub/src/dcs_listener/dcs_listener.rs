use std::error::Error;
use tokio::{net::UdpSocket, task::JoinHandle};

use crate::common::dcs_unit::DcsUnit;

pub const DCS_LISTENER_PORT: u16 = 34254;
pub const DCS_MSG_DELIMITER: u8 = b'\n';
pub const DCS_LISTENER_BUFFER_SIZE: usize = 1024;

/// Starts a thread that will continuously listen for the DCS units export.
///
/// # Arguments
/// * `unit_handler` - Closure for handling any captured DCS units from the export.
pub async fn listen<F>(unit_handler: F) -> Result<(), Box<dyn Error>>
where
    F: Fn(DcsUnit) + Send + Sync + 'static,
{
    let socket = setup_socket().await?;
    start_receiving_loop(socket, unit_handler).await;

    Ok(())
}

async fn start_receiving_loop<F>(socket: UdpSocket, unit_handler: F) -> JoinHandle<()>
where
    F: Fn(DcsUnit) + Send + Sync + 'static,
{
    let buffer = [0u8; DCS_LISTENER_BUFFER_SIZE];

    tokio::spawn(async move {
        loop {
            match receive_next(&socket, buffer).await {
                Ok(unit) => unit_handler(unit),
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
) -> Result<DcsUnit, Box<dyn Error>> {
    let size = socket.recv(&mut buffer).await?;

    if size == 0 {
        // No data received, possibly due to a closed connection or other issue
        return Err("No data received".into());
    }

    // Iterate over the buffer to find the delimiter and extract the message
    for (index, &byte) in buffer[..size].iter().enumerate() {
        if byte == DCS_MSG_DELIMITER {
            let msg = String::from_utf8_lossy(&buffer[..index]);
            let unit = serde_json::from_str::<DcsUnit>(&msg)?;
            return Ok(unit);
        }
    }

    // Delimiter not found in the current datagram
    Err("Message delimiter not found in the datagram".into())
}

#[cfg(test)]
mod integration_tests {

    use tokio::net::UdpSocket;

    use crate::common::dcs_unit::{Coalition, DcsUnit, Position3D, UnitType};

    use super::{listen, DCS_LISTENER_PORT, DCS_MSG_DELIMITER};

    #[tokio::test]
    async fn test_listen() {
        // Create a few units
        let mut units = Vec::new();
        for i in 0..3 {
            units.push(DcsUnit {
                unit_name: format!("UNIT-{}", i),
                group_name: format!("GROUP-{}", i),
                coalition: Coalition::BLUFOR,
                position: Position3D {
                    latitude: 30.0090027 + (i as f64),
                    longitude: -85.9578735 + (i as f64),
                    altitude: 132.67 + (i as f32),
                },
                unit_type: UnitType {
                    level_1: 'A',
                    level_2: 'B',
                },
                mission_date: "2024-03-08".to_string(),
                mission_start_time: 28800,
                mission_time_elapsed: 3600,
            });
        }

        // Create a channel to signal that units_handler was called
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let unit_handler = {
            let tx = tx.clone();
            move |received_unit: DcsUnit| {
                let tx = tx.clone();

                // Transmit the received unit for future assertion
                tokio::spawn(async move {
                    tx.send(received_unit).await.expect("Failed to send signal");
                });
            }
        };

        // Start the listener
        let list_task = tokio::spawn(async move {
            listen(unit_handler)
                .await
                .expect("Unable to start listener")
        });

        // Create a separate socket for sending messages
        let sender_socket = UdpSocket::bind("127.0.0.1:0") // Bind to an arbitrary available port
            .await
            .expect("Unable to create sender socket");
        sender_socket
            .connect(("127.0.0.1", DCS_LISTENER_PORT))
            .await
            .expect("Unable to connect to listener");

        // Send units to the listener via socket
        for unit in &units {
            let mut serialized_unit = serde_json::to_string(&unit).unwrap();
            serialized_unit.push(DCS_MSG_DELIMITER as char);

            sender_socket
                .send(serialized_unit.as_bytes())
                .await
                .expect("Unable to send message");
        }

        // Check if all units were received
        let mut units_count = 0;
        while let Some(unit) = rx.recv().await {
            assert!(units.contains(&unit));
            units_count += 1;

            if units_count == units.len() {
                break;
            }
        }

        assert_eq!(units_count, units.len());

        list_task.await.unwrap();
    }
}
