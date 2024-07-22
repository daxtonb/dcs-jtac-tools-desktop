use serde::{Deserialize, Serialize};
use utils::data::file_store::FileStore;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserConfig {
    pub dcs_install_directory: String,
    pub dcs_saved_games_directory: String,
}

impl FileStore<UserConfig> for UserConfig {}
