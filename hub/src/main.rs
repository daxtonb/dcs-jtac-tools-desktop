use std::{error::Error, sync::Arc};

use common::dcs_unit::DcsUnit;
use hub::web_socket_hub::WebSocketHub;
use udp_listener::listen;

use crate::cursor_on_target::xml_serializer::XmlSerializer;

mod common;
mod udp_listener;
mod hub;
mod cursor_on_target;
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
