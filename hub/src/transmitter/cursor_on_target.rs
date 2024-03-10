use chrono::{Duration, ParseError};
use serde::{Deserialize, Serialize};

use crate::common::dcs_unit::{DcsUnit, MissionTimeCalculator};

use super::atomic_event::AtomicEvent;

// Used for building and serializing cursor-on-target data
// See https://www.mitre.org/sites/default/files/pdf/09_4937.pdf

/// Used to handle XML serialization
trait ToXml {
    /// Serialize to an XML string.
    fn to_xml(&self) -> String;
}

/// An optional element used to hold CoT sub-schema.
#[derive(Debug, Deserialize, Serialize)]
struct Detail {
    /// The unit call sign of the CoT
    call_sign: String,
}

impl ToXml for Detail {
    fn to_xml(&self) -> String {
        format!(
            r#"<detail><contact callsign="{}"/></detail>"#,
            self.call_sign
        )
    }
}

/// Geographical location of the CoT
#[derive(Debug, Deserialize, Serialize)]
struct Point {
    /// Latitude referred to the WGS 84 ellipsoid in degrees
    lat: f64,

    /// Longitude referred to the WGS 84 in degrees
    lon: f64,

    /// Height above the WGS ellipsoid in meters
    hae: f32,
}

impl ToXml for Point {
    fn to_xml(&self) -> String {
        format!(
            r#"<point lat="{}" lon="{}" ce="0.0" hae="{}" le="0.0"/>"#,
            self.lat, self.lon, self.hae
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Event {
    /// Hierarchically organized hint about event type.
    unit_type: String,

    /// Globally unique name for this information on this event.
    uid: String,

    /// Time stamp: when the event was generated.
    time: String,

    /// Ending time when an event should no longer be considered valid.
    stale: String,

    /// Geographical location of the CoT.
    point: Point,

    /// An optional element used to hold CoT sub-schema.
    detail: Detail,
}

impl ToXml for Event {
    fn to_xml(&self) -> String {
        format!(
            r#"<?xml version="1.0" standalone="yes"?><event version="2.0" uid="{}" type="{}" time="{}" start="{}" stale="{}">{}{}</event>"#,
            self.uid,
            self.unit_type,
            self.time,
            self.time,
            self.stale,
            self.point.to_xml(),
            self.detail.to_xml()
        )
    }
}

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
    use crate::common::dcs_unit::{Coalition, Position3D, UnitType};

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
            },
            unit_type: UnitType {
                level_1: 'A',
                level_2: 'M',
            },
            date: "2005-04-05".to_string(),
            mission_start_time: 42_000,
            mission_time_elapsed: 218,
        };
        let expected = r#"<?xml version="1.0" standalone="yes"?><event version="2.0" uid="J-01334" type="a-h-A-M" time="2005-04-05T11:43:38Z" start="2005-04-05T11:43:38Z" stale="2005-04-05T11:44:38Z"><point lat="30.0090027" lon="-85.9578735" ce="0.0" hae="-42.6" le="0.0"/><detail><contact callsign="J-01334"/></detail></event>"#;

        // Act
        let result =
            XmlSerializer::serialize_dcs_unit(&unit).expect("DCS unit XML serialization failed.");

        // Assert
        assert_eq!(result, expected);
    }
}
