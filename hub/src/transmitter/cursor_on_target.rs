use serde::{Deserialize, Serialize};

// Used for building and serializing cursor-on-target data
// See https://www.mitre.org/sites/default/files/pdf/09_4937.pdf

/// An optional element used to hold CoT sub-schema.
#[derive(Debug, Deserialize, Serialize)]
pub struct Detail {
    /// The unit call sign of the CoT
    callSign: String,
}

impl XmlSerializer for Detail {
    fn to_xml(&self) -> String {
        format!("<detail><contact callsign=\"{}\"/></detail>", self.callSign)
    }
}

/// Geographical location of the CoT
#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    /// Latitude referred to the WGS 84 ellipsoid in degrees
    lat: f64,

    /// Longitude referred to the WGS 84 in degrees
    lon: f64,

    /// Height above the WGS ellipsoid in meters
    hae: f32,
}

impl XmlSerializer for Point {
    fn to_xml(&self) -> String {
        format!(
            "<point lat=\"{}\"lon=\"{}\"ce=\"0.0\"hae=\"{}\"le=\"0.0\"/></event>",
            self.lat, self.lon, self.hae
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
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

impl XmlSerializer for Event {
    fn to_xml(&self) -> String {
        format!(
            "<event version=\"2.0\" uid=\"{}\" type=\"{}\" time=\"{}\" start=\"{}\" stale=\"{}\">{}{}</event>", 
            self.uid, self.unit_type, self.time, self.time, self.stale, self.point.to_xml(), self.detail.to_xml())
    }
}

/// Encapsulates data needed to build XML for the CoT.
pub struct CursorOnTarget {
    pub event: Event,
}


impl XmlSerializer for CursorOnTarget {
    /// Builds an XML string.
    fn to_xml(&self) -> String {
        format!(
            "<?xml version=\"1.0\" standalone=\"yes\"?>{}",
            self.event.to_xml()
        )
    }
}

/// XML data serializer.
trait XmlSerializer {
    fn to_xml(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_on_target_to_xml() {
        // Arrange
        let cot = CursorOnTarget {
            event: Event {
                unit_type: "J-01334".to_string(),
                uid: "a-h-A-M-F-U-M".to_string(),
                time: "2005-04-05T11:43:38.07Z".to_string(),
                stale: "2005-04-05T11:45:38.07Z".to_string(),
                point: Point {
                    lat: 30.0090027,
                    lon: -85.9578735,
                    hae: -42.6,
                },
                detail: Detail {
                    callSign: "Alpha".to_string(),
                },
            },
        };

        let expected_xml = r#"<?xml version="1.0" standalone="yes"?><event version="2.0" uid="a-h-A-M-F-U-M" type="J-01334" time="2005-04-05T11:43:38.07Z" start="2005-04-05T11:43:38.07Z" stale="2005-04-05T11:45:38.07Z"><point lat="30.0090027"lon="-85.9578735"ce="0.0"hae="-42.6"le="0.0"/></event><detail><contact callsign="Alpha"/></detail></event>"#;

        // Act
        let generated_xml = cot.to_xml();

        // Assert
        assert_eq!(generated_xml, expected_xml);
    }
}
