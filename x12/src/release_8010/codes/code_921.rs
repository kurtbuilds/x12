use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**921

See docs at <https://www.stedi.com/edi/x12/element/921>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisciplineTypeCode {
    ///AI - Home Health Aide
    HomeHealthAide,
    ///MS - Medical Social Worker
    MedicalSocialWorker,
    ///OT - Occupational Therapy
    OccupationalTherapy,
    ///PT - Physical Therapy
    PhysicalTherapy,
    ///SN - Skilled Nursing
    SkilledNursing,
    ///ST - Speech Therapy
    SpeechTherapy,
}
impl DisciplineTypeCode {
    pub fn code(&self) -> &str {
        {
            use DisciplineTypeCode::*;
            match self {
                HomeHealthAide => "AI",
                MedicalSocialWorker => "MS",
                OccupationalTherapy => "OT",
                PhysicalTherapy => "PT",
                SkilledNursing => "SN",
                SpeechTherapy => "ST",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<DisciplineTypeCode> {
        use DisciplineTypeCode::*;
        match code {
            b"AI" => Some(HomeHealthAide),
            b"MS" => Some(MedicalSocialWorker),
            b"OT" => Some(OccupationalTherapy),
            b"PT" => Some(PhysicalTherapy),
            b"SN" => Some(SkilledNursing),
            b"ST" => Some(SpeechTherapy),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use DisciplineTypeCode::*;
        match self {
            HomeHealthAide => "Home Health Aide",
            MedicalSocialWorker => "Medical Social Worker",
            OccupationalTherapy => "Occupational Therapy",
            PhysicalTherapy => "Physical Therapy",
            SkilledNursing => "Skilled Nursing",
            SpeechTherapy => "Speech Therapy",
        }
    }
    fn from_description(description: &str) -> Option<DisciplineTypeCode> {
        {
            use DisciplineTypeCode::*;
            match description {
                "Home Health Aide" => Some(HomeHealthAide),
                "Medical Social Worker" => Some(MedicalSocialWorker),
                "Occupational Therapy" => Some(OccupationalTherapy),
                "Physical Therapy" => Some(PhysicalTherapy),
                "Skilled Nursing" => Some(SkilledNursing),
                "Speech Therapy" => Some(SpeechTherapy),
                _ => None,
            }
        }
    }
}
impl Serialize for DisciplineTypeCode {
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
    type Value = DisciplineTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Discipline Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DisciplineTypeCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Discipline Type Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DisciplineTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Discipline Type Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for DisciplineTypeCode {
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