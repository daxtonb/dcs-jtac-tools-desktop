use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use serde::{Deserialize, Serialize};

use crate::common::{
    dcs_unit::{Coalition, DcsUnit},
    unit_type::Level1UnitType,
};

use super::{coalition_flag::CoalitionFlag, unit_type_flag::UnitTypeFlag};

/// Encapsulates the settings configurable by the user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserConfig {
    /// The coalition(s) the user wants to interact with.
    pub coalition_flag: CoalitionFlag,

    /// The unit type(s) the user wants to interact with.
    pub unit_type_flag: UnitTypeFlag,

    /// The frequency at which unit data should be exported from DCS.
    pub export_frequency_frames: i32,
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

    pub fn is_unit_configured(&self, unit: &DcsUnit) -> bool {
        self.is_coalition_configured(unit) && self.is_unit_type_configured(unit)
    }

    fn is_unit_type_configured(&self, unit: &DcsUnit) -> bool {
        match unit.unit_type.level_1 {
            Level1UnitType::AIR => {
                (self.unit_type_flag & UnitTypeFlag::AIR) != UnitTypeFlag::empty()
            }
            Level1UnitType::GROUND => {
                (self.unit_type_flag & UnitTypeFlag::GROUND) != UnitTypeFlag::empty()
            }
            Level1UnitType::SEA => {
                (self.unit_type_flag & UnitTypeFlag::SEA) != UnitTypeFlag::empty()
            }
        }
    }

    fn is_coalition_configured(&self, unit: &DcsUnit) -> bool {
        match unit.coalition {
            Coalition::NEUTRAL => {
                (self.coalition_flag & CoalitionFlag::NEUTRAL) != CoalitionFlag::empty()
            }
            Coalition::REDFOR => {
                (self.coalition_flag & CoalitionFlag::REDFOR) != CoalitionFlag::empty()
            }
            Coalition::BLUFOR => {
                (self.coalition_flag & CoalitionFlag::BLUFOR) != CoalitionFlag::empty()
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::{
        common::{
            dcs_unit::{Coalition, DcsUnit, Position3D, UnitType},
            unit_type::Level1UnitType,
        },
        user_config::{
            coalition_flag::CoalitionFlag,
            unit_type_flag::UnitTypeFlag,
            user_config::UserConfig,
        },
    };

    #[test]
    fn given_config_coalition_flag_when_unit_coalition_matches_then_return_true() {
        let pairs = vec![
            (CoalitionFlag::NEUTRAL, Coalition::NEUTRAL),
            (CoalitionFlag::REDFOR, Coalition::REDFOR),
            (CoalitionFlag::BLUFOR, Coalition::BLUFOR),
        ];

        for (coalition_flag, coalition) in pairs {
            let config = build_user_config(Some(coalition_flag), None);
            let unit = build_dcs_unit(Some(coalition), None);

            assert!(config.is_coalition_configured(&unit));
        }
    }

    #[test]
    fn given_config_coalition_flag_when_unit_coalition_does_not_match_then_return_false() {
        let pairs = vec![
            (CoalitionFlag::NEUTRAL, Coalition::REDFOR),
            (CoalitionFlag::REDFOR, Coalition::BLUFOR),
            (CoalitionFlag::BLUFOR, Coalition::NEUTRAL),
        ];

        for (coalition_flag, coalition) in pairs {
            let config = build_user_config(Some(coalition_flag), None);
            let unit = build_dcs_unit(Some(coalition), None);

            assert!(!config.is_coalition_configured(&unit));
        }
    }

    #[test]
    fn given_config_unit_type_flag_when_unit_type_matches_then_return_true() {
        let pairs = vec![
            (UnitTypeFlag::AIR, Level1UnitType::AIR),
            (UnitTypeFlag::GROUND, Level1UnitType::GROUND),
            (UnitTypeFlag::SEA, Level1UnitType::SEA),
        ];

        for (unit_type_flag, unit_type) in pairs {
            let config = build_user_config(None, Some(unit_type_flag));
            let unit = build_dcs_unit(None, Some(unit_type));

            assert!(config.is_unit_type_configured(&unit));
        }
    }

    #[test]
    fn given_config_unit_type_flag_when_unit_type_does_not_match_then_return_false() {
        let pairs = vec![
            (UnitTypeFlag::AIR, Level1UnitType::GROUND),
            (UnitTypeFlag::GROUND, Level1UnitType::SEA),
            (UnitTypeFlag::SEA, Level1UnitType::AIR),
        ];

        for (unit_type_flag, unit_type) in pairs {
            let config = build_user_config(None, Some(unit_type_flag));
            let unit = build_dcs_unit(None, Some(unit_type));

            assert!(!config.is_unit_type_configured(&unit));
        }
    }

    #[test]
    fn give_config_when_unit_is_configured_then_return_true() {
        let config = build_user_config(Some(CoalitionFlag::BLUFOR), Some(UnitTypeFlag::GROUND));
        let unit = build_dcs_unit(Some(Coalition::BLUFOR), Some(Level1UnitType::GROUND));

        assert!(config.is_unit_configured(&unit));
    }

    #[test]
    fn give_config_when_unit_is_not_configured_then_return_false() {
        let config = build_user_config(Some(CoalitionFlag::BLUFOR), Some(UnitTypeFlag::GROUND));
        let unit = build_dcs_unit(Some(Coalition::REDFOR), Some(Level1UnitType::AIR));

        assert!(!config.is_unit_configured(&unit));
        
        let config = build_user_config(Some(CoalitionFlag::BLUFOR), Some(UnitTypeFlag::GROUND));
        let unit = build_dcs_unit(Some(Coalition::BLUFOR), Some(Level1UnitType::AIR));

        assert!(!config.is_unit_configured(&unit));
    }

    fn build_dcs_unit(coalition: Option<Coalition>, unit_type: Option<Level1UnitType>) -> DcsUnit {
        DcsUnit {
            coalition: match coalition {
                Some(coalition) => coalition,
                None => Coalition::BLUFOR,
            },
            unit_name: String::new(),
            group_name: String::new(),
            position: Position3D {
                latitude: 0.0,
                longitude: 0.0,
                altitude: 0.0,
                heading: 0.0,
            },
            unit_type: UnitType {
                level_1: match unit_type {
                    Some(unit_type) => unit_type,
                    None => Level1UnitType::AIR,
                },
                level_2: 0,
            },
            mission_date: String::new(),
            mission_start_time: 0,
            mission_time_elapsed: 0,
        }
    }

    fn build_user_config(
        coalition_flag: Option<CoalitionFlag>,
        unit_type_flag: Option<UnitTypeFlag>,
    ) -> UserConfig {
        UserConfig {
            coalition_flag: match coalition_flag {
                Some(coalition_flag) => coalition_flag,
                None => CoalitionFlag::empty(),
            } | CoalitionFlag(8),
            unit_type_flag: match unit_type_flag {
                Some(unit_type_flag) => unit_type_flag,
                None => UnitTypeFlag::empty(),
            } | UnitTypeFlag(8),
            export_frequency_frames: 0,
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use std::fs;

    use super::UserConfig;
    use crate::user_config::{coalition_flag::CoalitionFlag, unit_type_flag::UnitTypeFlag};

    #[test]
    fn test_write_and_read() {
        // Write
        let file_path = "test.config";
        let config = UserConfig {
            coalition_flag: CoalitionFlag::BLUFOR | CoalitionFlag::NEUTRAL,
            unit_type_flag: UnitTypeFlag::GROUND,
            export_frequency_frames: 10,
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
