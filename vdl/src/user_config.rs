use serde::{Deserialize, Serialize};
use utils::data::file_store::FileStore;

use crate::multi_functional_color_display::MfcdPosition;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserConfig {
    pub dcs_install_directory: String,
    pub dcs_saved_games_directory: String,
    pub mfcd_position: MfcdPosition
}

impl FileStore<UserConfig> for UserConfig {}
