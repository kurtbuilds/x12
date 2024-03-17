use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1335

See docs at <https://www.stedi.com/edi/x12-005010/element/1335>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InsulinDependentCode {
    ///1 - Widely fluctuating blood sugars
    WidelyFluctuatingBloodSugars,
    ///2 - Frequent episodes of insulin reactions
    FrequentEpisodesOfInsulinReactions,
    ///3 - Evidence of frequent significant ketosis
    EvidenceOfFrequentSignificantKetosis,
    ///N - Patient is not insulin dependent
    PatientIsNotInsulinDependent,
}
impl InsulinDependentCode {
    pub fn code(&self) -> &str {
        {
            use InsulinDependentCode::*;
            match self {
                WidelyFluctuatingBloodSugars => "1",
                FrequentEpisodesOfInsulinReactions => "2",
                EvidenceOfFrequentSignificantKetosis => "3",
                PatientIsNotInsulinDependent => "N",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<InsulinDependentCode> {
        use InsulinDependentCode::*;
        match code {
            b"1" => Some(WidelyFluctuatingBloodSugars),
            b"2" => Some(FrequentEpisodesOfInsulinReactions),
            b"3" => Some(EvidenceOfFrequentSignificantKetosis),
            b"N" => Some(PatientIsNotInsulinDependent),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use InsulinDependentCode::*;
        match self {
            WidelyFluctuatingBloodSugars => "Widely fluctuating blood sugars",
            FrequentEpisodesOfInsulinReactions => {
                "Frequent episodes of insulin reactions"
            }
            EvidenceOfFrequentSignificantKetosis => {
                "Evidence of frequent significant ketosis"
            }
            PatientIsNotInsulinDependent => "Patient is not insulin dependent",
        }
    }
    fn from_description(description: &str) -> Option<InsulinDependentCode> {
        {
            use InsulinDependentCode::*;
            match description {
                "Widely fluctuating blood sugars" => Some(WidelyFluctuatingBloodSugars),
                "Frequent episodes of insulin reactions" => {
                    Some(FrequentEpisodesOfInsulinReactions)
                }
                "Evidence of frequent significant ketosis" => {
                    Some(EvidenceOfFrequentSignificantKetosis)
                }
                "Patient is not insulin dependent" => Some(PatientIsNotInsulinDependent),
                _ => None,
            }
        }
    }
}
impl Serialize for InsulinDependentCode {
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
    type Value = InsulinDependentCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Insulin Dependent Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        InsulinDependentCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Insulin Dependent Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        InsulinDependentCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Insulin Dependent Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for InsulinDependentCode {
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