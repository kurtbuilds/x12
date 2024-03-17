use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1349

See docs at <https://www.stedi.com/edi/x12/element/1349>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OxygenTestConditionCode {
    ///E - Exercising
    Exercising,
    ///N - No special conditions for test
    NoSpecialConditionsForTest,
    ///O - On oxygen
    OnOxygen,
    ///R - At rest on room air
    AtRestOnRoomAir,
    ///S - Sleeping
    Sleeping,
    ///W - Walking
    Walking,
    ///X - Other
    Other,
}
impl OxygenTestConditionCode {
    pub fn code(&self) -> &str {
        {
            use OxygenTestConditionCode::*;
            match self {
                Exercising => "E",
                NoSpecialConditionsForTest => "N",
                OnOxygen => "O",
                AtRestOnRoomAir => "R",
                Sleeping => "S",
                Walking => "W",
                Other => "X",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<OxygenTestConditionCode> {
        use OxygenTestConditionCode::*;
        match code {
            b"E" => Some(Exercising),
            b"N" => Some(NoSpecialConditionsForTest),
            b"O" => Some(OnOxygen),
            b"R" => Some(AtRestOnRoomAir),
            b"S" => Some(Sleeping),
            b"W" => Some(Walking),
            b"X" => Some(Other),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use OxygenTestConditionCode::*;
        match self {
            Exercising => "Exercising",
            NoSpecialConditionsForTest => "No special conditions for test",
            OnOxygen => "On oxygen",
            AtRestOnRoomAir => "At rest on room air",
            Sleeping => "Sleeping",
            Walking => "Walking",
            Other => "Other",
        }
    }
    fn from_description(description: &str) -> Option<OxygenTestConditionCode> {
        {
            use OxygenTestConditionCode::*;
            match description {
                "Exercising" => Some(Exercising),
                "No special conditions for test" => Some(NoSpecialConditionsForTest),
                "On oxygen" => Some(OnOxygen),
                "At rest on room air" => Some(AtRestOnRoomAir),
                "Sleeping" => Some(Sleeping),
                "Walking" => Some(Walking),
                "Other" => Some(Other),
                _ => None,
            }
        }
    }
}
impl Serialize for OxygenTestConditionCode {
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
    type Value = OxygenTestConditionCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Oxygen Test Condition Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        OxygenTestConditionCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Oxygen Test Condition Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        OxygenTestConditionCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Oxygen Test Condition Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for OxygenTestConditionCode {
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