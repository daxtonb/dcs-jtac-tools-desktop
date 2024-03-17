use std::{error::Error, sync::Arc};

use common::dcs_unit::DcsUnit;
use dcs_listener::dcs_listener::listen;
use hub::web_socket_hub::WebSocketHub;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

use crate::transmitter::cursor_on_target::XmlSerializer;

mod common;
mod dcs_listener;
mod hub;
mod transmitter;
mod user_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let hub = Arc::new(WebSocketHub::new(9345));
    let hub_clone = hub.clone();
    tokio::spawn(async move { hub.start().await });

    let unit_handler = move |unit: DcsUnit| {
        println!("Received unit: {:?}", unit);
        match XmlSerializer::serialize_dcs_unit(&unit) {
            Ok(xml) => {
                hub_clone.broadcast_message(xml);
            }
            Err(err) => eprintln!("Failed to serialize DCS unit: {:?}", err),
        }
    };

    if let Err(e) = listen(unit_handler).await {
        eprintln!("Listener error: {}", e);
    }

    Ok(())
}
