use std::fmt;

use crate::common::{dcs_unit::{Coalition, DcsUnit}, unit_type::Level1UnitType};

/// Models a four-level hierarchy used for constructing atomic events. Consider making the number
/// of levels more dynamic (i.e. linked list).
#[derive(Debug, PartialEq)]
pub struct AtomicEvent {
    level_1: char,
    level_2: char,
    level_3: char,
}

impl fmt::Display for AtomicEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}",
            self.level_1, self.level_2, self.level_3
        )
    }
}

impl AtomicEvent {
    pub fn from(unit: &DcsUnit) -> AtomicEvent {
        AtomicEvent {
            level_1: 'a',
            level_2: coalition_to_atomic_event_char(&unit.coalition),
            level_3: level_1_unit_type_char(&unit.unit_type.level_1),
        }
    }
}

/// Handle conversion from DCS coalitions
fn coalition_to_atomic_event_char(coalition: &Coalition) -> char {
    match coalition {
        Coalition::NEUTRAL => 'n',
        Coalition::REDFOR => 'h',
        Coalition::BLUFOR => 'f',
    }
}

/// Handle conversion from DCS level 1 unit type
fn level_1_unit_type_char(unit_type: &Level1UnitType) -> char {
    match unit_type {
        Level1UnitType::AIR => 'A',
        Level1UnitType::GROUND => 'G',
        Level1UnitType::SEA => 'S',
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::{
        common::{dcs_unit::{Coalition, DcsUnit, Position3D, UnitType}, unit_type::Level1UnitType},
        cursor_on_target::atomic_event::{coalition_to_atomic_event_char, level_1_unit_type_char},
    };

    use super::AtomicEvent;

    #[test]
    fn given_dcs_unit_when_building_atomic_event_then_fields_are_mapped() {
        // Arrange
        let unit = DcsUnit {
            unit_name: "J-01334".to_string(),
            group_name: "J-01335".to_string(),
            coalition: Coalition::REDFOR,
            position: Position3D {
                latitude: 30.0090027,
                longitude: -85.9578735,
                altitude: -42.6,
                heading: 0.0568,
            },
            unit_type: UnitType {
                level_1: Level1UnitType::AIR,
                level_2: 1,
            },
            mission_date: "2005-04-05".to_string(),
            mission_start_time: 42_000,
            mission_time_elapsed: 218,
        };

        let expected = AtomicEvent {
            level_1: 'a',
            level_2: 'h',
            level_3: 'A',
        };

        // Act
        let result = AtomicEvent::from(&unit);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn given_atomic_event_when_serialized_to_string_then_formatted_as_atomic_event() {
        // Arrange
        let event = AtomicEvent {
            level_1: 'a',
            level_2: 'h',
            level_3: 'A',
        };

        // Act
        let result = event.to_string();

        // Assert
        assert_eq!("a-h-A", result);
    }

    #[test]
    fn given_neutral_unit_when_building_unit_type_then_n_returned() {
        // Arrange
        let coalition = Coalition::NEUTRAL;

        // Act
        let result = coalition_to_atomic_event_char(&coalition);

        // Assert
        assert_eq!('n', result);
    }

    #[test]
    fn given_redfor_unit_when_building_unit_type_then_h_returned() {
        // Arrange
        let coalition = Coalition::REDFOR;

        // Act
        let result = coalition_to_atomic_event_char(&coalition);

        // Assert
        assert_eq!('h', result);
    }

    #[test]
    fn given_blufor_unit_when_building_unit_type_then_f_returned() {
        // Arrange
        let coalition = Coalition::BLUFOR;

        // Act
        let result = coalition_to_atomic_event_char(&coalition);

        // Assert
        assert_eq!('f', result);
    }

    #[test]
    fn given_ground_unit_when_building_unit_type_then_g_returned() {
        // Arrange
        let unit_type = Level1UnitType::GROUND;

        // Act
        let result = level_1_unit_type_char(&unit_type);

        // Assert
        assert_eq!('G', result);
    }

    #[test]
    fn given_sea_unit_when_building_unit_type_then_s_returned() {
        // Arrange
        let unit_type = Level1UnitType::SEA;

        // Act
        let result = level_1_unit_type_char(&unit_type);

        // Assert
        assert_eq!('S', result);
    }

    #[test]
    fn given_air_unit_when_building_unit_type_then_a_returned() {
        // Arrange
        let unit_type = Level1UnitType::AIR;

        // Act
        let result = level_1_unit_type_char(&unit_type);

        // Assert
        assert_eq!('A', result);
    }
}
