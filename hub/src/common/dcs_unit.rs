use chrono::{DateTime, ParseError, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// The top-level DCS unit types
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Level1UnitType {
    AIR = 'A' as isize,
    GROUND = 'G' as isize,
    SEA = 'S' as isize,
}

/// The detailed aircraft unit type
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum AirLevel2UnitType {
    FIXED_WING = 'F' as isize,
    ROTARY_WING = 'H' as isize,
}

/// The DCS coalition
#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum Coalition {
    /// Neutral coalition
    NEUTRAL = 0,

    /// REDFOR coalition
    REDFOR = 1,

    /// BLUFOR coalition
    BLUFOR = 2,
}

/// The unit categorization
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UnitType {
    /// Top-level categorization of unit
    pub level_1: char,

    /// The sub-categorization of unit
    pub level_2: char,
}

/// 3-dimensional position of the unit
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Position3D {
    /// Latitudinal position of the unit
    pub latitude: f64,

    /// Longitudinal position of the unit
    pub longitude: f64,

    /// Elevation of the unit
    pub altitude: f32,

    /// The heading of the unit (in radians)
    pub heading: f64,
}

/// Models the exported DCS unit
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DcsUnit {
    /// The unit's identifier
    pub unit_name: String,

    /// The unit's group identifier
    pub group_name: String,

    /// The unit's coalition
    pub coalition: Coalition,

    /// The unit's three-dimensional position
    pub position: Position3D,

    /// The categorization of the unit
    pub unit_type: UnitType,

    /// The date of the mission
    pub mission_date: String,

    /// The start time of the mission
    pub mission_start_time: i32,

    /// The time elapsed since the start of the mission
    pub mission_time_elapsed: i32,
}

/// DCS World mission time calculator
pub trait MissionTimeCalculator {
    /// Uses the data available from DCS units to construct a `DateTime<Utc>` instance.
    fn calculate_mission_time(&self) -> Result<DateTime<Utc>, ParseError>;
}

impl MissionTimeCalculator for DcsUnit {
    fn calculate_mission_time(&self) -> Result<DateTime<Utc>, ParseError> {
        let current_time_sec = self.mission_start_time + self.mission_time_elapsed;
        let hour = current_time_sec / 3600;
        let minute = (current_time_sec % 3600) / 60;
        let second = current_time_sec % 60;

        format!(
            "{}T{:02}:{:02}:{:02}Z",
            self.mission_date, hour, minute, second
        )
        .parse::<DateTime<Utc>>()
    }
}

#[cfg(test)]
mod unit_tests {
    use std::mem::discriminant;

    use chrono::DateTime;
    use chrono::Utc;

    use crate::common::dcs_unit::{DcsUnit, MissionTimeCalculator};

    use super::{Coalition, Position3D, UnitType};

    #[test]
    fn given_json_string_when_deserialized_then_deserialization_succeeds() {
        // Arrange
        let json = r#"{"unit_name":"UNIT-1","group_name":"GROUP-1","coalition":2,"position":{"latitude":30.0090027,"longitude":-85.9578735,"altitude":132.67,"heading":2.0034},"unit_type":{"level_1":"A","level_2":"B","level_3":"C","level_4":null},"mission_date":"2024-03-08","mission_start_time":28800,"mission_time_elapsed":3600}"#;
        let expected = build_dcs_unit();

        // Act
        let result: DcsUnit = serde_json::from_str(json).expect("Failed to deserialize DCS Unit");

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn given_json_string_when_serialized_then_json_string_serialization_succeeds() {
        // Arrange
        let expected = r#"{"unit_name":"UNIT-1","group_name":"GROUP-1","coalition":2,"position":{"latitude":30.0090027,"longitude":-85.9578735,"altitude":132.67,"heading":2.0034},"unit_type":{"level_1":"A","level_2":"B"},"mission_date":"2024-03-08","mission_start_time":28800,"mission_time_elapsed":3600}"#;
        let dcs_unit = build_dcs_unit();

        // Act
        let result = serde_json::to_string(&dcs_unit).expect("Failed to serialize DCS Unit");

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn given_properly_formatted_date_info_when_deserialized_then_succeeds_to_build_date_time() {
        // Arrange
        let dcs_unit = build_dcs_unit();
        let expected: DateTime<Utc> = "2024-03-08T09:00:00Z".to_string().parse().unwrap();

        // Act
        let result = dcs_unit
            .calculate_mission_time()
            .expect("Failed to calculate mission time.");

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn given_malformed_date_info_when_deserialized_then_returns_error() {
        // Arrange
        let mut dcs_unit = build_dcs_unit();
        dcs_unit.mission_date = "2023-13-08".to_string();

        // Act
        let result = dcs_unit.calculate_mission_time();

        // Assert
        match result {
            Ok(_) => assert!(false, "Calculation succeeded unexpected"),
            Err(_) => assert!(true),
        }
    }

    fn build_dcs_unit() -> DcsUnit {
        DcsUnit {
            unit_name: "UNIT-1".to_string(),
            group_name: "GROUP-1".to_string(),
            coalition: Coalition::BLUFOR,
            position: Position3D {
                latitude: 30.0090027,
                longitude: -85.9578735,
                altitude: 132.67,
                heading: 2.0034,
            },
            unit_type: UnitType {
                level_1: 'A',
                level_2: 'B',
            },
            mission_date: "2024-03-08".to_string(),
            mission_start_time: 28800,
            mission_time_elapsed: 3600,
        }
    }
}
