use std::{error::Error, sync::Arc};

use common::dcs_unit::DcsUnit;
use hub::web_socket_hub::WebSocketHub;
use udp_listener::listen;
use user_config::{
    coalition_flag::CoalitionFlag, unit_type_flag::UnitTypeFlag, user_config::UserConfig,
};

use crate::cursor_on_target::xml_serializer::XmlSerializer;

mod common;
mod cursor_on_target;
mod hub;
mod udp_listener;
mod user_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let user_config = load_config().unwrap();
    let hub = Arc::new(WebSocketHub::new(9345));
    let hub_clone = hub.clone();
    tokio::spawn(async move { hub.start().await });

    let unit_handler = move |unit: DcsUnit| {
        if !user_config.is_unit_configured(&unit) {
            return;
        }

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

fn load_config() -> Result<UserConfig, Box<dyn Error>> {
    const CONFIG_FILE_PATH: &str = "hub.config";
    let user_config = match UserConfig::from_file(CONFIG_FILE_PATH) {
        Ok(config_from_file) => config_from_file,
        Err(_) => {
            let new_config = UserConfig {
                coalition_flag: CoalitionFlag::BLUFOR,
                unit_type_flag: UnitTypeFlag::GROUND | UnitTypeFlag::AIR | UnitTypeFlag::SEA,
                export_frequency_frames: 100,
            };
            new_config.to_file(CONFIG_FILE_PATH)?;
            new_config
        }
    };

    Ok(user_config)
}
