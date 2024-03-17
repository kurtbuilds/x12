use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**679

See docs at <https://www.stedi.com/edi/x12-005010/element/679>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShipDeliveryPatternTimeCode {
    ///A - 1st Shift (Normal Working Hours)
    CodeA,
    ///B - 2nd Shift
    CodeB,
    ///C - 3rd Shift
    CodeC,
    ///D - A.M.
    AM,
    ///E - P.M.
    PM,
    ///F - As Directed
    AsDirected,
    ///G - Any Shift
    AnyShift,
    ///Y - None (Also Used to Cancel or Override a Previous Pattern)
    CodeY,
    ///Z - Mutually Defined
    MutuallyDefined,
}
impl ShipDeliveryPatternTimeCode {
    pub fn code(&self) -> &str {
        {
            use ShipDeliveryPatternTimeCode::*;
            match self {
                CodeA => "A",
                CodeB => "B",
                CodeC => "C",
                AM => "D",
                PM => "E",
                AsDirected => "F",
                AnyShift => "G",
                CodeY => "Y",
                MutuallyDefined => "Z",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ShipDeliveryPatternTimeCode> {
        use ShipDeliveryPatternTimeCode::*;
        match code {
            b"A" => Some(CodeA),
            b"B" => Some(CodeB),
            b"C" => Some(CodeC),
            b"D" => Some(AM),
            b"E" => Some(PM),
            b"F" => Some(AsDirected),
            b"G" => Some(AnyShift),
            b"Y" => Some(CodeY),
            b"Z" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ShipDeliveryPatternTimeCode::*;
        match self {
            CodeA => "1st Shift (Normal Working Hours)",
            CodeB => "2nd Shift",
            CodeC => "3rd Shift",
            AM => "A.M.",
            PM => "P.M.",
            AsDirected => "As Directed",
            AnyShift => "Any Shift",
            CodeY => "None (Also Used to Cancel or Override a Previous Pattern)",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<ShipDeliveryPatternTimeCode> {
        {
            use ShipDeliveryPatternTimeCode::*;
            match description {
                "1st Shift (Normal Working Hours)" => Some(CodeA),
                "2nd Shift" => Some(CodeB),
                "3rd Shift" => Some(CodeC),
                "A.M." => Some(AM),
                "P.M." => Some(PM),
                "As Directed" => Some(AsDirected),
                "Any Shift" => Some(AnyShift),
                "None (Also Used to Cancel or Override a Previous Pattern)" => {
                    Some(CodeY)
                }
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for ShipDeliveryPatternTimeCode {
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
    type Value = ShipDeliveryPatternTimeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Ship/Delivery Pattern Time Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ShipDeliveryPatternTimeCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Ship/Delivery Pattern Time Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ShipDeliveryPatternTimeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Ship/Delivery Pattern Time Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ShipDeliveryPatternTimeCode {
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