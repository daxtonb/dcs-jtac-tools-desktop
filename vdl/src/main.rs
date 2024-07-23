use std::{error::Error, process::exit};

use display_settings::DisplaySettings;
use multi_functional_color_display::{MfcdPosition, MultiFunctionalColorDisplay};

use std::path::Path;
use user_config::UserConfig;
use utils::data::file_store::FileStore;

pub mod display_settings;
pub mod multi_functional_color_display;
pub mod user_config;

fn main() {
    let user_config = load_config().unwrap();
    let saved_options_path = Path::new(&user_config.dcs_saved_games_directory)
        .join("Config")
        .join("Options.lua");
    let display_settings = DisplaySettings::from_file(&saved_options_path).unwrap();

    let display_settings_path = Path::new(&user_config.dcs_install_directory)
        .join("Config")
        .join("MonitorSetup")
        .join(format!("{}.lua", display_settings.profile_name));
    let mfcd_settings =
        MultiFunctionalColorDisplay::all_from_file(&display_settings_path, &display_settings)
            .unwrap();
    if !mfcd_settings.contains_key(&user_config.mfcd_position) {
        eprintln!("No MFCD settings found for {:?}", user_config.mfcd_position);
        exit(1);
    }
    println!("{:?}", display_settings);
    println!("Hello, world!");
}

fn load_config() -> Result<UserConfig, Box<dyn Error>> {
    const DEFAULT_DCS_INSTALL_DIR: &str = "C:\\Program Files\\Eagle Dynamics\\DCS World OpenBeta";
    const DEFAULT_DCS_SAVED_GAMES_DIR: &str = "C:\\Users\\iamda\\Saved Games\\DCS.openbeta"; // <-- TODO: use `dirs` crate to get user's home directory
    const DEFAULT_MFCD_POSITION: MfcdPosition = MfcdPosition::Right;
    const CONFIG_FILE_PATH: &str = "vdl.config";

    let user_config = match UserConfig::get(CONFIG_FILE_PATH) {
        Some(config_from_file) => config_from_file,
        None => {
            let new_config = UserConfig {
                dcs_install_directory: DEFAULT_DCS_INSTALL_DIR.to_string(),
                dcs_saved_games_directory: DEFAULT_DCS_SAVED_GAMES_DIR.to_string(),
                mfcd_position: DEFAULT_MFCD_POSITION,
            };
            UserConfig::set(CONFIG_FILE_PATH, &new_config)?;
            new_config
        }
    };

    Ok(user_config)
}
