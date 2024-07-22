use std::error::Error;

use display_settings::DisplaySettings;

use std::path::Path;
use user_config::UserConfig;
use utils::data::file_store::FileStore;

pub mod display_settings;
pub mod user_config;

fn main() {
    let user_config = load_config().unwrap();
    let saved_options_path = Path::new(&user_config.dcs_saved_games_directory)
        .join("Config")
        .join("Options.lua");
    let display_settings = DisplaySettings::from_file(&saved_options_path).unwrap();
    println!("Options: {:?}", display_settings);
    println!("Hello, world!");
}

fn load_config() -> Result<UserConfig, Box<dyn Error>> {
    const DEFAULT_DCS_INSTALL_DIR: &str = "C:\\Program Files\\Eagle Dynamics\\DCS World OpenBeta";
    const DEFAULT_DCS_SAVED_GAMES_DIR: &str = "C:\\Users\\iamda\\Saved Games\\DCS.openbeta"; // <-- TODO: use `dirs` crate to get user's home directory
    const CONFIG_FILE_PATH: &str = "vdl.config";

    let user_config = match UserConfig::get(CONFIG_FILE_PATH) {
        Some(config_from_file) => config_from_file,
        None => {
            let new_config = UserConfig {
                dcs_install_directory: DEFAULT_DCS_INSTALL_DIR.to_string(),
                dcs_saved_games_directory: DEFAULT_DCS_SAVED_GAMES_DIR.to_string(),
            };
            UserConfig::set(CONFIG_FILE_PATH, &new_config)?;
            new_config
        }
    };

    Ok(user_config)
}
