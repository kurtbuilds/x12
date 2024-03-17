use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1476

See docs at <https://www.stedi.com/edi/x12/element/1476>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LanguageProficiencyIndicatorCode {
    ///1 - English Only
    EnglishOnly,
    ///2 - Fully English Proficient
    FullyEnglishProficient,
    ///3 - Limited English Proficient
    LimitedEnglishProficient,
    ///4 - Non-English Speaking
    NonEnglishSpeaking,
    ///5 - Status Unknown
    StatusUnknown,
    ///6 - Redesignated Fluent English Proficient
    RedesignatedFluentEnglishProficient,
    ///A - Excellent or Fluent
    ExcellentOrFluent,
    ///B - Good
    Good,
    ///C - Fair
    Fair,
    ///D - Poor
    Poor,
    ///E - Unacceptable
    Unacceptable,
}
impl LanguageProficiencyIndicatorCode {
    pub fn code(&self) -> &str {
        {
            use LanguageProficiencyIndicatorCode::*;
            match self {
                EnglishOnly => "1",
                FullyEnglishProficient => "2",
                LimitedEnglishProficient => "3",
                NonEnglishSpeaking => "4",
                StatusUnknown => "5",
                RedesignatedFluentEnglishProficient => "6",
                ExcellentOrFluent => "A",
                Good => "B",
                Fair => "C",
                Poor => "D",
                Unacceptable => "E",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<LanguageProficiencyIndicatorCode> {
        use LanguageProficiencyIndicatorCode::*;
        match code {
            b"1" => Some(EnglishOnly),
            b"2" => Some(FullyEnglishProficient),
            b"3" => Some(LimitedEnglishProficient),
            b"4" => Some(NonEnglishSpeaking),
            b"5" => Some(StatusUnknown),
            b"6" => Some(RedesignatedFluentEnglishProficient),
            b"A" => Some(ExcellentOrFluent),
            b"B" => Some(Good),
            b"C" => Some(Fair),
            b"D" => Some(Poor),
            b"E" => Some(Unacceptable),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use LanguageProficiencyIndicatorCode::*;
        match self {
            EnglishOnly => "English Only",
            FullyEnglishProficient => "Fully English Proficient",
            LimitedEnglishProficient => "Limited English Proficient",
            NonEnglishSpeaking => "Non-English Speaking",
            StatusUnknown => "Status Unknown",
            RedesignatedFluentEnglishProficient => {
                "Redesignated Fluent English Proficient"
            }
            ExcellentOrFluent => "Excellent or Fluent",
            Good => "Good",
            Fair => "Fair",
            Poor => "Poor",
            Unacceptable => "Unacceptable",
        }
    }
    fn from_description(description: &str) -> Option<LanguageProficiencyIndicatorCode> {
        {
            use LanguageProficiencyIndicatorCode::*;
            match description {
                "English Only" => Some(EnglishOnly),
                "Fully English Proficient" => Some(FullyEnglishProficient),
                "Limited English Proficient" => Some(LimitedEnglishProficient),
                "Non-English Speaking" => Some(NonEnglishSpeaking),
                "Status Unknown" => Some(StatusUnknown),
                "Redesignated Fluent English Proficient" => {
                    Some(RedesignatedFluentEnglishProficient)
                }
                "Excellent or Fluent" => Some(ExcellentOrFluent),
                "Good" => Some(Good),
                "Fair" => Some(Fair),
                "Poor" => Some(Poor),
                "Unacceptable" => Some(Unacceptable),
                _ => None,
            }
        }
    }
}
impl Serialize for LanguageProficiencyIndicatorCode {
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
    type Value = LanguageProficiencyIndicatorCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Language Proficiency Indicator Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        LanguageProficiencyIndicatorCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Language Proficiency Indicator Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        LanguageProficiencyIndicatorCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Language Proficiency Indicator Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for LanguageProficiencyIndicatorCode {
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