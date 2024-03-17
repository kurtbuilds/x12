use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1347

See docs at <https://www.stedi.com/edi/x12/element/1347>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NutrientAdministrationTechniqueCode {
    ///G - Gastrostomy
    Gastrostomy,
    ///J - Jejunostomy
    Jejunostomy,
    ///N - Nasogastric Tube
    NasogastricTube,
    ///O - Other
    Other,
}
impl NutrientAdministrationTechniqueCode {
    pub fn code(&self) -> &str {
        {
            use NutrientAdministrationTechniqueCode::*;
            match self {
                Gastrostomy => "G",
                Jejunostomy => "J",
                NasogastricTube => "N",
                Other => "O",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<NutrientAdministrationTechniqueCode> {
        use NutrientAdministrationTechniqueCode::*;
        match code {
            b"G" => Some(Gastrostomy),
            b"J" => Some(Jejunostomy),
            b"N" => Some(NasogastricTube),
            b"O" => Some(Other),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use NutrientAdministrationTechniqueCode::*;
        match self {
            Gastrostomy => "Gastrostomy",
            Jejunostomy => "Jejunostomy",
            NasogastricTube => "Nasogastric Tube",
            Other => "Other",
        }
    }
    fn from_description(
        description: &str,
    ) -> Option<NutrientAdministrationTechniqueCode> {
        {
            use NutrientAdministrationTechniqueCode::*;
            match description {
                "Gastrostomy" => Some(Gastrostomy),
                "Jejunostomy" => Some(Jejunostomy),
                "Nasogastric Tube" => Some(NasogastricTube),
                "Other" => Some(Other),
                _ => None,
            }
        }
    }
}
impl Serialize for NutrientAdministrationTechniqueCode {
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
    type Value = NutrientAdministrationTechniqueCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Nutrient Administration Technique Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NutrientAdministrationTechniqueCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Nutrient Administration Technique Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NutrientAdministrationTechniqueCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Nutrient Administration Technique Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for NutrientAdministrationTechniqueCode {
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