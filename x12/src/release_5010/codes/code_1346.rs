use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1346

See docs at <https://www.stedi.com/edi/x12-005010/element/1346>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NutrientAdministrationMethodCode {
    ///G - Gravity
    Gravity,
    ///P - Pump
    Pump,
    ///S - Syringe
    Syringe,
}
impl NutrientAdministrationMethodCode {
    pub fn code(&self) -> &str {
        {
            use NutrientAdministrationMethodCode::*;
            match self {
                Gravity => "G",
                Pump => "P",
                Syringe => "S",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<NutrientAdministrationMethodCode> {
        use NutrientAdministrationMethodCode::*;
        match code {
            b"G" => Some(Gravity),
            b"P" => Some(Pump),
            b"S" => Some(Syringe),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use NutrientAdministrationMethodCode::*;
        match self {
            Gravity => "Gravity",
            Pump => "Pump",
            Syringe => "Syringe",
        }
    }
    fn from_description(description: &str) -> Option<NutrientAdministrationMethodCode> {
        {
            use NutrientAdministrationMethodCode::*;
            match description {
                "Gravity" => Some(Gravity),
                "Pump" => Some(Pump),
                "Syringe" => Some(Syringe),
                _ => None,
            }
        }
    }
}
impl Serialize for NutrientAdministrationMethodCode {
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
    type Value = NutrientAdministrationMethodCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Nutrient Administration Method Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NutrientAdministrationMethodCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Nutrient Administration Method Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NutrientAdministrationMethodCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Nutrient Administration Method Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for NutrientAdministrationMethodCode {
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