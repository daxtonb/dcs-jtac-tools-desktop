use std::fmt;

use crate::common::dcs_unit::{Coalition, DcsUnit};

/// Models a four-level hierarchy used for constructing atomic events. Consider making the number
/// of levels more dynamic (i.e. linked list).
#[derive(Debug, PartialEq)]
pub struct AtomicEvent {
    level_1: char,
    level_2: char,
    level_3: char,
    level_4: char,
}

impl fmt::Display for AtomicEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}-{}",
            self.level_1, self.level_2, self.level_3, self.level_4
        )
    }
}

impl AtomicEvent {
    pub fn from(unit: &DcsUnit) -> AtomicEvent {
        AtomicEvent {
            level_1: 'a',
            level_2: coalition_to_atomic_event_char(&unit.coalition),
            level_3: unit.unit_type.level_1,
            level_4: unit.unit_type.level_2,
        }
    }
}

fn coalition_to_atomic_event_char(coalition: &Coalition) -> char {
    match coalition {
        Coalition::NEUTRAL => 'f',
        Coalition::REDFOR => 'h',
        Coalition::BLUFOR => 'n',
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::{
        common::dcs_unit::{Coalition, DcsUnit, Position3D, UnitType},
        transmitter::atomic_event::coalition_to_atomic_event_char,
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
            },
            unit_type: UnitType {
                level_1: 'A',
                level_2: 'M',
            },
            mission_date: "2005-04-05".to_string(),
            mission_start_time: 42_000,
            mission_time_elapsed: 218,
        };

        let expected = AtomicEvent {
            level_1: 'a',
            level_2: 'h',
            level_3: 'A',
            level_4: 'M',
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
            level_4: 'M',
        };

        // Act
        let result = event.to_string();

        // Assert
        assert_eq!("a-h-A-M", result);
    }

    #[test]
    fn given_neutral_unit_when_building_unit_type_then_atom_string_generated() {
        // Arrange
        let coalition = Coalition::NEUTRAL;

        // Act
        let result = coalition_to_atomic_event_char(&coalition);

        // Assert
        assert_eq!('f', result);
    }

    #[test]
    fn given_redfor_unit_when_building_unit_type_then_atom_string_generated() {
        // Arrange
        let coalition = Coalition::REDFOR;

        // Act
        let result = coalition_to_atomic_event_char(&coalition);

        // Assert
        assert_eq!('h', result);
    }

    #[test]
    fn given_blufor_unit_when_building_unit_type_then_atom_string_generated() {
        // Arrange
        let coalition = Coalition::BLUFOR;

        // Act
        let result = coalition_to_atomic_event_char(&coalition);

        // Assert
        assert_eq!('n', result);
    }
}
