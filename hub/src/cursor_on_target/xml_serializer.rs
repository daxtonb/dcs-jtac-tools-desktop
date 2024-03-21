

use chrono::{Duration, ParseError};

use crate::common::dcs_unit::{DcsUnit, MissionTimeCalculator};

use super::{atomic_event::AtomicEvent, Detail, Event, Point};

/// Used to handle XML serialization
pub trait ToXml {
    /// Serialize to an XML string.
    fn to_xml(&self) -> String;
}

/// Handles serialization of DCS units into the cursor-on-target XML format
pub struct XmlSerializer;

impl XmlSerializer {
    pub fn serialize_dcs_unit(unit: &DcsUnit) -> Result<String, ParseError> {
        let mission_time = unit.calculate_mission_time()?;

        let event = Event {
            point: Point {
                lat: unit.position.latitude,
                lon: unit.position.longitude,
                hae: unit.position.altitude,
            },
            detail: Detail {
                call_sign: unit.unit_name.to_string(),
            },
            unit_type: AtomicEvent::from(&unit).to_string(),
            uid: unit.unit_name.clone(),
            time: mission_time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            stale: (mission_time + Duration::try_minutes(1).unwrap())
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        };

        Ok(event.to_xml())
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::common::{dcs_unit::{Coalition, Position3D, UnitType}, unit_type::Level1UnitType};

    use super::*;

    #[test]
    fn given_dcs_unit_when_serialized_then_xml_is_generated_as_cot() {
        // Arrange
        let unit = DcsUnit {
            unit_name: "J-01334".to_string(),
            group_name: "J-01335".to_string(),
            coalition: Coalition::REDFOR,
            position: Position3D {
                latitude: 30.0090027,
                longitude: -85.9578735,
                altitude: -42.6,
                heading: 0.0568 
            },
            unit_type: UnitType {
                level_1: Level1UnitType::AIR,
                level_2: 1,
            },
            mission_date: "2005-04-05".to_string(),
            mission_start_time: 42_000,
            mission_time_elapsed: 218,
        };
        let expected = r#"<?xml version="1.0" standalone="yes"?><event version="2.0" uid="J-01334" type="a-h-A" how="m-g" time="2005-04-05T11:43:38Z" start="2005-04-05T11:43:38Z" stale="2005-04-05T11:44:38Z"><point lat="30.0090027" lon="-85.9578735" ce="0.0" hae="-42.6" le="0.0"/><detail><contact callsign="J-01334"/></detail></event>"#;

        // Act
        let result =
            XmlSerializer::serialize_dcs_unit(&unit).expect("DCS unit XML serialization failed.");

        // Assert
        assert_eq!(result, expected);
    }
}