use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1316

See docs at <https://www.stedi.com/edi/x12-005010/element/1316>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AmbulanceTransportCode {
    ///I - Initial Trip
    InitialTrip,
    ///R - Return Trip
    ReturnTrip,
    ///T - Transfer Trip
    TransferTrip,
    ///X - Round Trip
    RoundTrip,
}
impl AmbulanceTransportCode {
    pub fn code(&self) -> &str {
        {
            use AmbulanceTransportCode::*;
            match self {
                InitialTrip => "I",
                ReturnTrip => "R",
                TransferTrip => "T",
                RoundTrip => "X",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<AmbulanceTransportCode> {
        use AmbulanceTransportCode::*;
        match code {
            b"I" => Some(InitialTrip),
            b"R" => Some(ReturnTrip),
            b"T" => Some(TransferTrip),
            b"X" => Some(RoundTrip),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use AmbulanceTransportCode::*;
        match self {
            InitialTrip => "Initial Trip",
            ReturnTrip => "Return Trip",
            TransferTrip => "Transfer Trip",
            RoundTrip => "Round Trip",
        }
    }
    fn from_description(description: &str) -> Option<AmbulanceTransportCode> {
        {
            use AmbulanceTransportCode::*;
            match description {
                "Initial Trip" => Some(InitialTrip),
                "Return Trip" => Some(ReturnTrip),
                "Transfer Trip" => Some(TransferTrip),
                "Round Trip" => Some(RoundTrip),
                _ => None,
            }
        }
    }
}
impl Serialize for AmbulanceTransportCode {
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
    type Value = AmbulanceTransportCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Ambulance Transport Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        AmbulanceTransportCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Ambulance Transport Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        AmbulanceTransportCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Ambulance Transport Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for AmbulanceTransportCode {
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