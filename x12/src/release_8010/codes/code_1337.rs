use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1337

See docs at <https://www.stedi.com/edi/x12/element/1337>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LevelOfCareCode {
    ///1 - Skilled Nursing Facility (SNF)
    Code1,
    ///2 - Intermediate Care Facility (ICF)
    Code2,
    ///3 - Intermediate Care Facility - Mentally Retarded (ICF-MR)
    Code3,
    ///4 - Chronic Disease Hospital (CD)
    Code4,
    ///5 - Intermediate Care Facility (ICF) Level II
    Code5,
    ///6 - Special Skilled Nursing Facility (SNF)
    Code6,
    ///7 - Nursing Facility (NF)
    Code7,
    ///8 - Hospice
    Hospice,
}
impl LevelOfCareCode {
    pub fn code(&self) -> &str {
        {
            use LevelOfCareCode::*;
            match self {
                Code1 => "1",
                Code2 => "2",
                Code3 => "3",
                Code4 => "4",
                Code5 => "5",
                Code6 => "6",
                Code7 => "7",
                Hospice => "8",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<LevelOfCareCode> {
        use LevelOfCareCode::*;
        match code {
            b"1" => Some(Code1),
            b"2" => Some(Code2),
            b"3" => Some(Code3),
            b"4" => Some(Code4),
            b"5" => Some(Code5),
            b"6" => Some(Code6),
            b"7" => Some(Code7),
            b"8" => Some(Hospice),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use LevelOfCareCode::*;
        match self {
            Code1 => "Skilled Nursing Facility (SNF)",
            Code2 => "Intermediate Care Facility (ICF)",
            Code3 => "Intermediate Care Facility - Mentally Retarded (ICF-MR)",
            Code4 => "Chronic Disease Hospital (CD)",
            Code5 => "Intermediate Care Facility (ICF) Level II",
            Code6 => "Special Skilled Nursing Facility (SNF)",
            Code7 => "Nursing Facility (NF)",
            Hospice => "Hospice",
        }
    }
    fn from_description(description: &str) -> Option<LevelOfCareCode> {
        {
            use LevelOfCareCode::*;
            match description {
                "Skilled Nursing Facility (SNF)" => Some(Code1),
                "Intermediate Care Facility (ICF)" => Some(Code2),
                "Intermediate Care Facility - Mentally Retarded (ICF-MR)" => Some(Code3),
                "Chronic Disease Hospital (CD)" => Some(Code4),
                "Intermediate Care Facility (ICF) Level II" => Some(Code5),
                "Special Skilled Nursing Facility (SNF)" => Some(Code6),
                "Nursing Facility (NF)" => Some(Code7),
                "Hospice" => Some(Hospice),
                _ => None,
            }
        }
    }
}
impl Serialize for LevelOfCareCode {
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
    type Value = LevelOfCareCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Level of Care Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        LevelOfCareCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Level of Care Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        LevelOfCareCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Level of Care Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for LevelOfCareCode {
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