use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1362

See docs at <https://www.stedi.com/edi/x12/element/1362>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelatedCausesCode {
    ///AA - Auto Accident
    AutoAccident,
    ///AB - Abuse
    Abuse,
    ///AP - Another Party Responsible
    AnotherPartyResponsible,
    ///EM - Employment
    Employment,
    ///OA - Other Accident
    OtherAccident,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl RelatedCausesCode {
    pub fn code(&self) -> &str {
        {
            use RelatedCausesCode::*;
            match self {
                AutoAccident => "AA",
                Abuse => "AB",
                AnotherPartyResponsible => "AP",
                Employment => "EM",
                OtherAccident => "OA",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<RelatedCausesCode> {
        use RelatedCausesCode::*;
        match code {
            b"AA" => Some(AutoAccident),
            b"AB" => Some(Abuse),
            b"AP" => Some(AnotherPartyResponsible),
            b"EM" => Some(Employment),
            b"OA" => Some(OtherAccident),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use RelatedCausesCode::*;
        match self {
            AutoAccident => "Auto Accident",
            Abuse => "Abuse",
            AnotherPartyResponsible => "Another Party Responsible",
            Employment => "Employment",
            OtherAccident => "Other Accident",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<RelatedCausesCode> {
        {
            use RelatedCausesCode::*;
            match description {
                "Auto Accident" => Some(AutoAccident),
                "Abuse" => Some(Abuse),
                "Another Party Responsible" => Some(AnotherPartyResponsible),
                "Employment" => Some(Employment),
                "Other Accident" => Some(OtherAccident),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for RelatedCausesCode {
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
    type Value = RelatedCausesCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Related-Causes Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        RelatedCausesCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Related-Causes Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        RelatedCausesCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Related-Causes Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for RelatedCausesCode {
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