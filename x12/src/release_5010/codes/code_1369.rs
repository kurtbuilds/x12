use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1369

See docs at <https://www.stedi.com/edi/x12-005010/element/1369>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToothSurfaceCode {
    ///B - Buccal
    Buccal,
    ///D - Distal
    Distal,
    ///F - Facial
    Facial,
    ///I - Incisal
    Incisal,
    ///L - Lingual
    Lingual,
    ///M - Mesial
    Mesial,
    ///O - Occlusal
    Occlusal,
}
impl ToothSurfaceCode {
    pub fn code(&self) -> &str {
        {
            use ToothSurfaceCode::*;
            match self {
                Buccal => "B",
                Distal => "D",
                Facial => "F",
                Incisal => "I",
                Lingual => "L",
                Mesial => "M",
                Occlusal => "O",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ToothSurfaceCode> {
        use ToothSurfaceCode::*;
        match code {
            b"B" => Some(Buccal),
            b"D" => Some(Distal),
            b"F" => Some(Facial),
            b"I" => Some(Incisal),
            b"L" => Some(Lingual),
            b"M" => Some(Mesial),
            b"O" => Some(Occlusal),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ToothSurfaceCode::*;
        match self {
            Buccal => "Buccal",
            Distal => "Distal",
            Facial => "Facial",
            Incisal => "Incisal",
            Lingual => "Lingual",
            Mesial => "Mesial",
            Occlusal => "Occlusal",
        }
    }
    fn from_description(description: &str) -> Option<ToothSurfaceCode> {
        {
            use ToothSurfaceCode::*;
            match description {
                "Buccal" => Some(Buccal),
                "Distal" => Some(Distal),
                "Facial" => Some(Facial),
                "Incisal" => Some(Incisal),
                "Lingual" => Some(Lingual),
                "Mesial" => Some(Mesial),
                "Occlusal" => Some(Occlusal),
                _ => None,
            }
        }
    }
}
impl Serialize for ToothSurfaceCode {
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
    type Value = ToothSurfaceCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Tooth Surface Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ToothSurfaceCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Tooth Surface Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ToothSurfaceCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Tooth Surface Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ToothSurfaceCode {
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