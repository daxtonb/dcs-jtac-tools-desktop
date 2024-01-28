use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    net::IpAddr,
};

use serde::{Deserialize, Serialize};

use super::{coalition_flag::CoalitionFlag, unit_type_flag::UnitTypeFlag};

/// Encapsulates the settings configurable by the user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserConfig {
    /// The coalition(s) the user wants to interact with.
    pub coalition_flag: CoalitionFlag,

    /// The unit type(s) the user wants to interact with.
    pub unit_type_flag: UnitTypeFlag,

    /// The name of the user's controlled unit.
    pub user_unit_name: String,

    /// The frequency at which unit data should be exported from DCS.
    pub export_frequency_frames: i32,

    /// The IP address of the device to transmit to.
    pub device_ip_address: IpAddr,
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

    /// Writes the user configuration to the file system.
    pub fn to_file(&self, file_path: &str) -> io::Result<()> {
        let json = serde_json::to_string(self)?;
        let file = File::create(file_path)?;
        let mut buffer = BufWriter::new(file);

        buffer.write_all(json.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod integration_tests {
    use std::{fs, net::Ipv4Addr};

    use super::UserConfig;
    use crate::user_config::{coalition_flag::CoalitionFlag, unit_type_flag::UnitTypeFlag};

    #[test]
    fn test_write_and_read() {
        // Write
        let file_path = "test.config";
        let config = UserConfig {
            coalition_flag: CoalitionFlag::BLUFOR,
            unit_type_flag: UnitTypeFlag::GROUND,
            user_unit_name: String::from("My Unit"),
            export_frequency_frames: 10,
            device_ip_address: std::net::IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)),
        };

        config
            .to_file(file_path)
            .expect("Failed to write UserConfig to file.");

        // Read
        let config_from_file =
            UserConfig::from_file(file_path).expect("Failed to read UserConfig from file.");

        assert_eq!(config_from_file, config);

        // Cleanup
        fs::remove_file(file_path).unwrap();
    }
}
