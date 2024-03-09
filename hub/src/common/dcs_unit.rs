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
}

/// Models the exported DCS unit
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DcsUnit {
    /// The unit's identifier
    pub unit_name: String,

    /// The unit's group identifier
    pub group_name: String,

    /// The unit'scoalition
    pub coalition: Coalition,

    /// The unit's three-dimensional position
    pub position: Position3D,

    /// The categorization of the unit
    pub unit_type: UnitType,

    /// The date of the mission
    pub date: String,

    /// The start time of the mission
    pub mission_start_time: i32,

    /// The time elapsed since the start of the mission
    pub mission_time_elapsed: i32,
}

#[cfg(test)]
mod unit_tests {
    use super::Coalition;
    use super::DcsUnit;
    use super::Position3D;
    use super::UnitType;

    #[test]
    fn dcs_unit_can_deserialize() {
        // Arrange
        let json = r#"{"unit_name":"UNIT-1","group_name":"GROUP-1","coalition":2,"position":{"latitude":30.0090027,"longitude":-85.9578735,"altitude":132.67},"unit_type":{"level_1":"A","level_2":"B","level_3":"C","level_4":null},"date":"2024-03-08","mission_start_time":28800,"mission_time_elapsed":3600}"#;
        let expected = DcsUnit {
            unit_name: "UNIT-1".to_string(),
            group_name: "GROUP-1".to_string(),
            coalition: Coalition::BLUFOR,
            position: Position3D {
                latitude: 30.0090027,
                longitude: -85.9578735,
                altitude: 132.67,
            },
            unit_type: UnitType {
                level_1: 'A',
                level_2: 'B',
            },
            date: "2024-03-08".to_string(),
            mission_start_time: 28800,
            mission_time_elapsed: 3600,
        };

        // Act
        let result: DcsUnit = serde_json::from_str(json).expect("Failed to deserialize DCS Unit");

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn dcs_unit_can_serialize() {
        // Arrange
        let expected = r#"{"unit_name":"UNIT-1","group_name":"GROUP-1","coalition":2,"position":{"latitude":30.0090027,"longitude":-85.9578735,"altitude":132.67},"unit_type":{"level_1":"A","level_2":"B"},"date":"2024-03-08","mission_start_time":28800,"mission_time_elapsed":3600}"#;
        let dcs_unit = DcsUnit {
            unit_name: "UNIT-1".to_string(),
            group_name: "GROUP-1".to_string(),
            coalition: Coalition::BLUFOR,
            position: Position3D {
                latitude: 30.0090027,
                longitude: -85.9578735,
                altitude: 132.67,
            },
            unit_type: UnitType {
                level_1: 'A',
                level_2: 'B',
            },
            date: "2024-03-08".to_string(),
            mission_start_time: 28800,
            mission_time_elapsed: 3600,
        };

        // Act
        let result = serde_json::to_string(&dcs_unit).expect("Failed to serialize DCS Unit");

        // Assert
        assert_eq!(result, expected);
    }
}
