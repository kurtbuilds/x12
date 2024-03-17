use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1358

See docs at <https://www.stedi.com/edi/x12/element/1358>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Prosthesis {
    ///I - Initial Placement
    InitialPlacement,
    ///R - Replacement
    Replacement,
}
impl Prosthesis {
    pub fn code(&self) -> &str {
        {
            use Prosthesis::*;
            match self {
                InitialPlacement => "I",
                Replacement => "R",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<Prosthesis> {
        use Prosthesis::*;
        match code {
            b"I" => Some(InitialPlacement),
            b"R" => Some(Replacement),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use Prosthesis::*;
        match self {
            InitialPlacement => "Initial Placement",
            Replacement => "Replacement",
        }
    }
    fn from_description(description: &str) -> Option<Prosthesis> {
        {
            use Prosthesis::*;
            match description {
                "Initial Placement" => Some(InitialPlacement),
                "Replacement" => Some(Replacement),
                _ => None,
            }
        }
    }
}
impl Serialize for Prosthesis {
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
    type Value = Prosthesis;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Prosthesis, Crown or Inlay Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Prosthesis::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Prosthesis, Crown or Inlay Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Prosthesis::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Prosthesis, Crown or Inlay Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for Prosthesis {
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