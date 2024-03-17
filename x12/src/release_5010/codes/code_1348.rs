use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1348

See docs at <https://www.stedi.com/edi/x12-005010/element/1348>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OxygenEquipmentTypeCode {
    ///A - Concentrator
    Concentrator,
    ///B - Liquid Stationary
    LiquidStationary,
    ///C - Gaseous Stationary
    GaseousStationary,
    ///D - Liquid Portable
    LiquidPortable,
    ///E - Gaseous Portable
    GaseousPortable,
    ///O - Other
    Other,
}
impl OxygenEquipmentTypeCode {
    pub fn code(&self) -> &str {
        {
            use OxygenEquipmentTypeCode::*;
            match self {
                Concentrator => "A",
                LiquidStationary => "B",
                GaseousStationary => "C",
                LiquidPortable => "D",
                GaseousPortable => "E",
                Other => "O",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<OxygenEquipmentTypeCode> {
        use OxygenEquipmentTypeCode::*;
        match code {
            b"A" => Some(Concentrator),
            b"B" => Some(LiquidStationary),
            b"C" => Some(GaseousStationary),
            b"D" => Some(LiquidPortable),
            b"E" => Some(GaseousPortable),
            b"O" => Some(Other),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use OxygenEquipmentTypeCode::*;
        match self {
            Concentrator => "Concentrator",
            LiquidStationary => "Liquid Stationary",
            GaseousStationary => "Gaseous Stationary",
            LiquidPortable => "Liquid Portable",
            GaseousPortable => "Gaseous Portable",
            Other => "Other",
        }
    }
    fn from_description(description: &str) -> Option<OxygenEquipmentTypeCode> {
        {
            use OxygenEquipmentTypeCode::*;
            match description {
                "Concentrator" => Some(Concentrator),
                "Liquid Stationary" => Some(LiquidStationary),
                "Gaseous Stationary" => Some(GaseousStationary),
                "Liquid Portable" => Some(LiquidPortable),
                "Gaseous Portable" => Some(GaseousPortable),
                "Other" => Some(Other),
                _ => None,
            }
        }
    }
}
impl Serialize for OxygenEquipmentTypeCode {
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
    type Value = OxygenEquipmentTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Oxygen Equipment Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        OxygenEquipmentTypeCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Oxygen Equipment Type Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        OxygenEquipmentTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Oxygen Equipment Type Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for OxygenEquipmentTypeCode {
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