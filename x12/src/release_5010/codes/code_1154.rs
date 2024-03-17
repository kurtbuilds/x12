use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1154

See docs at <https://www.stedi.com/edi/x12-005010/element/1154>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WorkIntensityCode {
    ///H - Heavy
    Heavy,
    ///L - Light
    Light,
    ///M - Medium
    Medium,
    ///R - Regular
    Regular,
}
impl WorkIntensityCode {
    pub fn code(&self) -> &str {
        {
            use WorkIntensityCode::*;
            match self {
                Heavy => "H",
                Light => "L",
                Medium => "M",
                Regular => "R",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<WorkIntensityCode> {
        use WorkIntensityCode::*;
        match code {
            b"H" => Some(Heavy),
            b"L" => Some(Light),
            b"M" => Some(Medium),
            b"R" => Some(Regular),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use WorkIntensityCode::*;
        match self {
            Heavy => "Heavy",
            Light => "Light",
            Medium => "Medium",
            Regular => "Regular",
        }
    }
    fn from_description(description: &str) -> Option<WorkIntensityCode> {
        {
            use WorkIntensityCode::*;
            match description {
                "Heavy" => Some(Heavy),
                "Light" => Some(Light),
                "Medium" => Some(Medium),
                "Regular" => Some(Regular),
                _ => None,
            }
        }
    }
}
impl Serialize for WorkIntensityCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let value = if serializer.is_human_readable() {
            self.description()
        } else {
            self.code()
        };
        serializer.serialize_str(value)
    }
}
struct Visitor;
impl<'de> de::Visitor<'de> for Visitor {
    type Value = WorkIntensityCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Work Intensity Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        WorkIntensityCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Work Intensity Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        WorkIntensityCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Work Intensity Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for WorkIntensityCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(Visitor)
        } else {
            deserializer.deserialize_bytes(Visitor)
        }
    }
}