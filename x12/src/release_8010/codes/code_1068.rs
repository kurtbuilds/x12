use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1068

See docs at <https://www.stedi.com/edi/x12/element/1068>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GenderCode {
    ///A - Not Provided
    NotProvided,
    ///B - Not Applicable
    NotApplicable,
    ///F - Female
    Female,
    ///I - Nonbinary
    Nonbinary,
    ///M - Male
    Male,
    ///N - Non-sexed
    NonSexed,
    ///T - Self-reported as Transgender
    SelfReportedAsTransgender,
    ///U - Unknown
    Unknown,
    ///X - Unsexable
    Unsexable,
}
impl GenderCode {
    pub fn code(&self) -> &str {
        {
            use GenderCode::*;
            match self {
                NotProvided => "A",
                NotApplicable => "B",
                Female => "F",
                Nonbinary => "I",
                Male => "M",
                NonSexed => "N",
                SelfReportedAsTransgender => "T",
                Unknown => "U",
                Unsexable => "X",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<GenderCode> {
        use GenderCode::*;
        match code {
            b"A" => Some(NotProvided),
            b"B" => Some(NotApplicable),
            b"F" => Some(Female),
            b"I" => Some(Nonbinary),
            b"M" => Some(Male),
            b"N" => Some(NonSexed),
            b"T" => Some(SelfReportedAsTransgender),
            b"U" => Some(Unknown),
            b"X" => Some(Unsexable),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use GenderCode::*;
        match self {
            NotProvided => "Not Provided",
            NotApplicable => "Not Applicable",
            Female => "Female",
            Nonbinary => "Nonbinary",
            Male => "Male",
            NonSexed => "Non-sexed",
            SelfReportedAsTransgender => "Self-reported as Transgender",
            Unknown => "Unknown",
            Unsexable => "Unsexable",
        }
    }
    fn from_description(description: &str) -> Option<GenderCode> {
        {
            use GenderCode::*;
            match description {
                "Not Provided" => Some(NotProvided),
                "Not Applicable" => Some(NotApplicable),
                "Female" => Some(Female),
                "Nonbinary" => Some(Nonbinary),
                "Male" => Some(Male),
                "Non-sexed" => Some(NonSexed),
                "Self-reported as Transgender" => Some(SelfReportedAsTransgender),
                "Unknown" => Some(Unknown),
                "Unsexable" => Some(Unsexable),
                _ => None,
            }
        }
    }
}
impl Serialize for GenderCode {
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
    type Value = GenderCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Gender Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        GenderCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Gender Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        GenderCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Gender Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for GenderCode {
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