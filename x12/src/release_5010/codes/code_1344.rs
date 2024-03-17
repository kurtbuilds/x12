use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1344

See docs at <https://www.stedi.com/edi/x12-005010/element/1344>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NonVisitCode {
    ///L - Lab or Clinical Reports
    LabOrClinicalReports,
    ///N - Visiting Nurse
    VisitingNurse,
    ///O - Other
    Other,
}
impl NonVisitCode {
    pub fn code(&self) -> &str {
        {
            use NonVisitCode::*;
            match self {
                LabOrClinicalReports => "L",
                VisitingNurse => "N",
                Other => "O",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<NonVisitCode> {
        use NonVisitCode::*;
        match code {
            b"L" => Some(LabOrClinicalReports),
            b"N" => Some(VisitingNurse),
            b"O" => Some(Other),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use NonVisitCode::*;
        match self {
            LabOrClinicalReports => "Lab or Clinical Reports",
            VisitingNurse => "Visiting Nurse",
            Other => "Other",
        }
    }
    fn from_description(description: &str) -> Option<NonVisitCode> {
        {
            use NonVisitCode::*;
            match description {
                "Lab or Clinical Reports" => Some(LabOrClinicalReports),
                "Visiting Nurse" => Some(VisitingNurse),
                "Other" => Some(Other),
                _ => None,
            }
        }
    }
}
impl Serialize for NonVisitCode {
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
    type Value = NonVisitCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Non-Visit Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NonVisitCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Non-Visit Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NonVisitCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Non-Visit Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for NonVisitCode {
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