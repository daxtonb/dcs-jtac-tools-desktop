use std::error::Error;

use common::dcs_unit::DcsUnit;
use dcs_listener::dcs_listener::listen;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

mod common;
mod user_config;
mod dcs_listener;
mod transmitter;
mod hub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Start the listener in the background
    let listener_handle = tokio::spawn(async {
        let unit_handler = |unit: DcsUnit| {
            // Handle the unit here
            println!("Received unit: {:?}", unit);
        };

        if let Err(e) = listen(unit_handler).await {
            println!("Listener error: {}", e);
        }
    });

    let user_input_handle = tokio::spawn(async {
        let mut reader = BufReader::new(stdin()).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            match line.as_str() {
                "stop" => {
                    // Implement the logic to stop the listener
                    println!("Stopping listener...");
                    break;
                },
                _ => println!("Unknown command"),
            }
        }
    });

    // Wait for both tasks to complete
    let _ = tokio::try_join!(listener_handle, user_input_handle);
    
    Ok(())
}
