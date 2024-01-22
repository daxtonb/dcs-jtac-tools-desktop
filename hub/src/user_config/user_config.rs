use std::{io::{self, BufReader, BufRead, BufWriter, Write}, fs::File};

use serde::{Serialize, Deserialize};

use super::{coalition_flag::CoalitionFlag, unit_type_flag::UnitTypeFlag};

/// Encapsulates the settings configurable by the user.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    /// The coalition(s) the user wants to interact with.
    coalition_flag: CoalitionFlag,

    /// The unit type(s) the user wants to interact with.
    unit_type_flag: UnitTypeFlag,

    /// The name of the user's controlled unit.
    user_unit_name: String,

    /// The frequency at which unit data should be exported from DCS.
    export_frequency_frames: i32
}

impl UserConfig {
    /// Loads the user configuration from the file system.
    pub fn from_file(file_path: &str) -> io::Result<UserConfig> {
        let mut string_contents = String::new();
        let file = File::open(file_path)?;
        let buffer = BufReader::new(file);

        for line in buffer.lines() {
            string_contents.push_str(line?.as_str());
        }

        Ok(serde_json::from_str(&string_contents)?)
    }

    pub fn to_file(&self, file_path: &str) -> io::Result<()> {
        let json = serde_json::to_string(self)?;
        let file = File::create(file_path)?;
        let mut buffer = BufWriter::new(file);

        for line in json.lines() {
            writeln!(buffer, "{}", line)?
        }

        Ok(())
    }
}