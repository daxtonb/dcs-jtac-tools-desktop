pub mod atomic_event;
pub mod xml_serializer;

use serde::{Deserialize, Serialize};

use self::xml_serializer::ToXml;

// Used for building and serializing cursor-on-target data
// See https://www.mitre.org/sites/default/files/pdf/09_4937.pdf

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
            r#"<?xml version="1.0" standalone="yes"?><event version="2.0" uid="{}" type="{}" how="m-g" time="{}" start="{}" stale="{}">{}{}</event>"#,
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