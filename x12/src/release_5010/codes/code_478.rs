use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**478

See docs at <https://www.stedi.com/edi/x12/element/478>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CreditDebitFlagCode {
    ///C - Credit
    Credit,
    ///D - Debit
    Debit,
}
impl CreditDebitFlagCode {
    pub fn code(&self) -> &str {
        {
            use CreditDebitFlagCode::*;
            match self {
                Credit => "C",
                Debit => "D",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<CreditDebitFlagCode> {
        use CreditDebitFlagCode::*;
        match code {
            b"C" => Some(Credit),
            b"D" => Some(Debit),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use CreditDebitFlagCode::*;
        match self {
            Credit => "Credit",
            Debit => "Debit",
        }
    }
    fn from_description(description: &str) -> Option<CreditDebitFlagCode> {
        {
            use CreditDebitFlagCode::*;
            match description {
                "Credit" => Some(Credit),
                "Debit" => Some(Debit),
                _ => None,
            }
        }
    }
}
impl Serialize for CreditDebitFlagCode {
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
    type Value = CreditDebitFlagCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Credit/Debit Flag Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CreditDebitFlagCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Credit/Debit Flag Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CreditDebitFlagCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Credit/Debit Flag Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for CreditDebitFlagCode {
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