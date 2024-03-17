use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1303

See docs at <https://www.stedi.com/edi/x12/element/1303>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UseOfLanguageIndicatorCode {
    ///1 - Language of Instruction
    LanguageOfInstruction,
    ///2 - Language of Examination
    LanguageOfExamination,
    ///3 - Language in which Examination is Written
    LanguageInWhichExaminationIsWritten,
    ///4 - Language Spoken in the Home
    LanguageSpokenInTheHome,
    ///5 - Language Reading
    LanguageReading,
    ///6 - Language Writing
    LanguageWriting,
    ///7 - Language Speaking
    LanguageSpeaking,
    ///8 - Native Language
    NativeLanguage,
    ///9 - Language Signed
    LanguageSigned,
}
impl UseOfLanguageIndicatorCode {
    pub fn code(&self) -> &str {
        {
            use UseOfLanguageIndicatorCode::*;
            match self {
                LanguageOfInstruction => "1",
                LanguageOfExamination => "2",
                LanguageInWhichExaminationIsWritten => "3",
                LanguageSpokenInTheHome => "4",
                LanguageReading => "5",
                LanguageWriting => "6",
                LanguageSpeaking => "7",
                NativeLanguage => "8",
                LanguageSigned => "9",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<UseOfLanguageIndicatorCode> {
        use UseOfLanguageIndicatorCode::*;
        match code {
            b"1" => Some(LanguageOfInstruction),
            b"2" => Some(LanguageOfExamination),
            b"3" => Some(LanguageInWhichExaminationIsWritten),
            b"4" => Some(LanguageSpokenInTheHome),
            b"5" => Some(LanguageReading),
            b"6" => Some(LanguageWriting),
            b"7" => Some(LanguageSpeaking),
            b"8" => Some(NativeLanguage),
            b"9" => Some(LanguageSigned),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use UseOfLanguageIndicatorCode::*;
        match self {
            LanguageOfInstruction => "Language of Instruction",
            LanguageOfExamination => "Language of Examination",
            LanguageInWhichExaminationIsWritten => {
                "Language in which Examination is Written"
            }
            LanguageSpokenInTheHome => "Language Spoken in the Home",
            LanguageReading => "Language Reading",
            LanguageWriting => "Language Writing",
            LanguageSpeaking => "Language Speaking",
            NativeLanguage => "Native Language",
            LanguageSigned => "Language Signed",
        }
    }
    fn from_description(description: &str) -> Option<UseOfLanguageIndicatorCode> {
        {
            use UseOfLanguageIndicatorCode::*;
            match description {
                "Language of Instruction" => Some(LanguageOfInstruction),
                "Language of Examination" => Some(LanguageOfExamination),
                "Language in which Examination is Written" => {
                    Some(LanguageInWhichExaminationIsWritten)
                }
                "Language Spoken in the Home" => Some(LanguageSpokenInTheHome),
                "Language Reading" => Some(LanguageReading),
                "Language Writing" => Some(LanguageWriting),
                "Language Speaking" => Some(LanguageSpeaking),
                "Native Language" => Some(NativeLanguage),
                "Language Signed" => Some(LanguageSigned),
                _ => None,
            }
        }
    }
}
impl Serialize for UseOfLanguageIndicatorCode {
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
    type Value = UseOfLanguageIndicatorCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Use of Language Indicator Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UseOfLanguageIndicatorCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Use of Language Indicator Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UseOfLanguageIndicatorCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Use of Language Indicator Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for UseOfLanguageIndicatorCode {
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