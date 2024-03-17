use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1073

See docs at <https://www.stedi.com/edi/x12/element/1073>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YesNoConditionOrResponseCode {
    ///C - Contains
    Contains,
    ///E - Exempt
    Exempt,
    ///F - Free From
    FreeFrom,
    ///M - May Contain
    MayContain,
    ///N - No
    No,
    ///R - Under Review
    UnderReview,
    ///U - Unknown
    Unknown,
    ///W - Not Applicable
    NotApplicable,
    ///Y - Yes
    Yes,
}
impl YesNoConditionOrResponseCode {
    pub fn code(&self) -> &str {
        {
            use YesNoConditionOrResponseCode::*;
            match self {
                Contains => "C",
                Exempt => "E",
                FreeFrom => "F",
                MayContain => "M",
                No => "N",
                UnderReview => "R",
                Unknown => "U",
                NotApplicable => "W",
                Yes => "Y",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<YesNoConditionOrResponseCode> {
        use YesNoConditionOrResponseCode::*;
        match code {
            b"C" => Some(Contains),
            b"E" => Some(Exempt),
            b"F" => Some(FreeFrom),
            b"M" => Some(MayContain),
            b"N" => Some(No),
            b"R" => Some(UnderReview),
            b"U" => Some(Unknown),
            b"W" => Some(NotApplicable),
            b"Y" => Some(Yes),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use YesNoConditionOrResponseCode::*;
        match self {
            Contains => "Contains",
            Exempt => "Exempt",
            FreeFrom => "Free From",
            MayContain => "May Contain",
            No => "No",
            UnderReview => "Under Review",
            Unknown => "Unknown",
            NotApplicable => "Not Applicable",
            Yes => "Yes",
        }
    }
    fn from_description(description: &str) -> Option<YesNoConditionOrResponseCode> {
        {
            use YesNoConditionOrResponseCode::*;
            match description {
                "Contains" => Some(Contains),
                "Exempt" => Some(Exempt),
                "Free From" => Some(FreeFrom),
                "May Contain" => Some(MayContain),
                "No" => Some(No),
                "Under Review" => Some(UnderReview),
                "Unknown" => Some(Unknown),
                "Not Applicable" => Some(NotApplicable),
                "Yes" => Some(Yes),
                _ => None,
            }
        }
    }
}
impl Serialize for YesNoConditionOrResponseCode {
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
    type Value = YesNoConditionOrResponseCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Yes/No Condition or Response Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        YesNoConditionOrResponseCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Yes/No Condition or Response Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        YesNoConditionOrResponseCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Yes/No Condition or Response Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for YesNoConditionOrResponseCode {
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