use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**478

See docs at <https://www.stedi.com/edi/x12/element/478>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CreditDebitFlagCode {
    ///A - Credit (Actual)
    CodeA,
    ///C - Credit
    Credit,
    ///D - Debit
    Debit,
    ///E - Credit (Estimated)
    CodeE,
}
impl CreditDebitFlagCode {
    pub fn code(&self) -> &str {
        {
            use CreditDebitFlagCode::*;
            match self {
                CodeA => "A",
                Credit => "C",
                Debit => "D",
                CodeE => "E",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<CreditDebitFlagCode> {
        use CreditDebitFlagCode::*;
        match code {
            b"A" => Some(CodeA),
            b"C" => Some(Credit),
            b"D" => Some(Debit),
            b"E" => Some(CodeE),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use CreditDebitFlagCode::*;
        match self {
            CodeA => "Credit (Actual)",
            Credit => "Credit",
            Debit => "Debit",
            CodeE => "Credit (Estimated)",
        }
    }
    fn from_description(description: &str) -> Option<CreditDebitFlagCode> {
        {
            use CreditDebitFlagCode::*;
            match description {
                "Credit (Actual)" => Some(CodeA),
                "Credit" => Some(Credit),
                "Debit" => Some(Debit),
                "Credit (Estimated)" => Some(CodeE),
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