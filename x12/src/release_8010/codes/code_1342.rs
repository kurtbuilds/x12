use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1342

See docs at <https://www.stedi.com/edi/x12/element/1342>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NatureOfConditionCode {
    ///A - Acute Condition
    AcuteCondition,
    ///C - Chronic Condition
    ChronicCondition,
    ///D - Non-acute
    NonAcute,
    ///E - Non-Life Threatening
    NonLifeThreatening,
    ///F - Routine
    Routine,
    ///G - Symptomatic
    Symptomatic,
    ///M - Acute Manifestation of a Chronic Condition
    AcuteManifestationOfAChronicCondition,
}
impl NatureOfConditionCode {
    pub fn code(&self) -> &str {
        {
            use NatureOfConditionCode::*;
            match self {
                AcuteCondition => "A",
                ChronicCondition => "C",
                NonAcute => "D",
                NonLifeThreatening => "E",
                Routine => "F",
                Symptomatic => "G",
                AcuteManifestationOfAChronicCondition => "M",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<NatureOfConditionCode> {
        use NatureOfConditionCode::*;
        match code {
            b"A" => Some(AcuteCondition),
            b"C" => Some(ChronicCondition),
            b"D" => Some(NonAcute),
            b"E" => Some(NonLifeThreatening),
            b"F" => Some(Routine),
            b"G" => Some(Symptomatic),
            b"M" => Some(AcuteManifestationOfAChronicCondition),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use NatureOfConditionCode::*;
        match self {
            AcuteCondition => "Acute Condition",
            ChronicCondition => "Chronic Condition",
            NonAcute => "Non-acute",
            NonLifeThreatening => "Non-Life Threatening",
            Routine => "Routine",
            Symptomatic => "Symptomatic",
            AcuteManifestationOfAChronicCondition => {
                "Acute Manifestation of a Chronic Condition"
            }
        }
    }
    fn from_description(description: &str) -> Option<NatureOfConditionCode> {
        {
            use NatureOfConditionCode::*;
            match description {
                "Acute Condition" => Some(AcuteCondition),
                "Chronic Condition" => Some(ChronicCondition),
                "Non-acute" => Some(NonAcute),
                "Non-Life Threatening" => Some(NonLifeThreatening),
                "Routine" => Some(Routine),
                "Symptomatic" => Some(Symptomatic),
                "Acute Manifestation of a Chronic Condition" => {
                    Some(AcuteManifestationOfAChronicCondition)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for NatureOfConditionCode {
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
    type Value = NatureOfConditionCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Nature of Condition Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NatureOfConditionCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Nature of Condition Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NatureOfConditionCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Nature of Condition Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for NatureOfConditionCode {
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