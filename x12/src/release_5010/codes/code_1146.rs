use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1146

See docs at <https://www.stedi.com/edi/x12-005010/element/1146>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisabilityTypeCode {
    ///1 - Short Term Disability
    ShortTermDisability,
    ///2 - Long Term Disability
    LongTermDisability,
    ///3 - Permanent or Total Disability
    PermanentOrTotalDisability,
    ///4 - No Disability
    NoDisability,
    ///5 - Partial Disability
    PartialDisability,
    ///Z - Mutually Defined
    MutuallyDefined,
}
impl DisabilityTypeCode {
    pub fn code(&self) -> &str {
        {
            use DisabilityTypeCode::*;
            match self {
                ShortTermDisability => "1",
                LongTermDisability => "2",
                PermanentOrTotalDisability => "3",
                NoDisability => "4",
                PartialDisability => "5",
                MutuallyDefined => "Z",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<DisabilityTypeCode> {
        use DisabilityTypeCode::*;
        match code {
            b"1" => Some(ShortTermDisability),
            b"2" => Some(LongTermDisability),
            b"3" => Some(PermanentOrTotalDisability),
            b"4" => Some(NoDisability),
            b"5" => Some(PartialDisability),
            b"Z" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use DisabilityTypeCode::*;
        match self {
            ShortTermDisability => "Short Term Disability",
            LongTermDisability => "Long Term Disability",
            PermanentOrTotalDisability => "Permanent or Total Disability",
            NoDisability => "No Disability",
            PartialDisability => "Partial Disability",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<DisabilityTypeCode> {
        {
            use DisabilityTypeCode::*;
            match description {
                "Short Term Disability" => Some(ShortTermDisability),
                "Long Term Disability" => Some(LongTermDisability),
                "Permanent or Total Disability" => Some(PermanentOrTotalDisability),
                "No Disability" => Some(NoDisability),
                "Partial Disability" => Some(PartialDisability),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for DisabilityTypeCode {
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
    type Value = DisabilityTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Disability Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DisabilityTypeCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Disability Type Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DisabilityTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Disability Type Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for DisabilityTypeCode {
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