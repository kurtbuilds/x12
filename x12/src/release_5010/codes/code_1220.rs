use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1220

See docs at <https://www.stedi.com/edi/x12-005010/element/1220>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StudentStatusCode {
    ///F - Full-time
    FullTime,
    ///N - Not a Student
    NotAStudent,
    ///P - Part-time
    PartTime,
}
impl StudentStatusCode {
    pub fn code(&self) -> &str {
        {
            use StudentStatusCode::*;
            match self {
                FullTime => "F",
                NotAStudent => "N",
                PartTime => "P",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<StudentStatusCode> {
        use StudentStatusCode::*;
        match code {
            b"F" => Some(FullTime),
            b"N" => Some(NotAStudent),
            b"P" => Some(PartTime),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use StudentStatusCode::*;
        match self {
            FullTime => "Full-time",
            NotAStudent => "Not a Student",
            PartTime => "Part-time",
        }
    }
    fn from_description(description: &str) -> Option<StudentStatusCode> {
        {
            use StudentStatusCode::*;
            match description {
                "Full-time" => Some(FullTime),
                "Not a Student" => Some(NotAStudent),
                "Part-time" => Some(PartTime),
                _ => None,
            }
        }
    }
}
impl Serialize for StudentStatusCode {
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
    type Value = StudentStatusCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Student Status Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        StudentStatusCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Student Status Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        StudentStatusCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Student Status Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for StudentStatusCode {
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