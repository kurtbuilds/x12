use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**923

See docs at <https://www.stedi.com/edi/x12/element/923>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrognosisCode {
    ///1 - Poor
    Poor,
    ///2 - Guarded
    Guarded,
    ///3 - Fair
    Fair,
    ///4 - Good
    Good,
    ///5 - Very Good
    VeryGood,
    ///6 - Excellent
    Excellent,
    ///7 - Less than 6 Months to Live
    LessThan6MonthsToLive,
    ///8 - Terminal
    Terminal,
}
impl PrognosisCode {
    pub fn code(&self) -> &str {
        {
            use PrognosisCode::*;
            match self {
                Poor => "1",
                Guarded => "2",
                Fair => "3",
                Good => "4",
                VeryGood => "5",
                Excellent => "6",
                LessThan6MonthsToLive => "7",
                Terminal => "8",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<PrognosisCode> {
        use PrognosisCode::*;
        match code {
            b"1" => Some(Poor),
            b"2" => Some(Guarded),
            b"3" => Some(Fair),
            b"4" => Some(Good),
            b"5" => Some(VeryGood),
            b"6" => Some(Excellent),
            b"7" => Some(LessThan6MonthsToLive),
            b"8" => Some(Terminal),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use PrognosisCode::*;
        match self {
            Poor => "Poor",
            Guarded => "Guarded",
            Fair => "Fair",
            Good => "Good",
            VeryGood => "Very Good",
            Excellent => "Excellent",
            LessThan6MonthsToLive => "Less than 6 Months to Live",
            Terminal => "Terminal",
        }
    }
    fn from_description(description: &str) -> Option<PrognosisCode> {
        {
            use PrognosisCode::*;
            match description {
                "Poor" => Some(Poor),
                "Guarded" => Some(Guarded),
                "Fair" => Some(Fair),
                "Good" => Some(Good),
                "Very Good" => Some(VeryGood),
                "Excellent" => Some(Excellent),
                "Less than 6 Months to Live" => Some(LessThan6MonthsToLive),
                "Terminal" => Some(Terminal),
                _ => None,
            }
        }
    }
}
impl Serialize for PrognosisCode {
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
    type Value = PrognosisCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Prognosis Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PrognosisCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Prognosis Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PrognosisCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Prognosis Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for PrognosisCode {
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