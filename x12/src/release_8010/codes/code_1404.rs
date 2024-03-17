use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1404

See docs at <https://www.stedi.com/edi/x12/element/1404>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImplantStatusCode {
    ///1 - Implanted
    Implanted,
    ///2 - Explanted
    Explanted,
}
impl ImplantStatusCode {
    pub fn code(&self) -> &str {
        {
            use ImplantStatusCode::*;
            match self {
                Implanted => "1",
                Explanted => "2",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ImplantStatusCode> {
        use ImplantStatusCode::*;
        match code {
            b"1" => Some(Implanted),
            b"2" => Some(Explanted),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ImplantStatusCode::*;
        match self {
            Implanted => "Implanted",
            Explanted => "Explanted",
        }
    }
    fn from_description(description: &str) -> Option<ImplantStatusCode> {
        {
            use ImplantStatusCode::*;
            match description {
                "Implanted" => Some(Implanted),
                "Explanted" => Some(Explanted),
                _ => None,
            }
        }
    }
}
impl Serialize for ImplantStatusCode {
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
    type Value = ImplantStatusCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Implant Status Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ImplantStatusCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Implant Status Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ImplantStatusCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Implant Status Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ImplantStatusCode {
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