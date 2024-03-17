use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1320

See docs at <https://www.stedi.com/edi/x12/element/1320>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BasisOfDaysSupplyDeterminationCode {
    ///0 - Not Specified
    NotSpecified,
    ///1 - Explicit Directions
    ExplicitDirections,
    ///2 - PRN Directions (Take as Needed; Pharmacist Estimate)
    Code2,
    ///3 - As Directed by Physician
    AsDirectedByPhysician,
}
impl BasisOfDaysSupplyDeterminationCode {
    pub fn code(&self) -> &str {
        {
            use BasisOfDaysSupplyDeterminationCode::*;
            match self {
                NotSpecified => "0",
                ExplicitDirections => "1",
                Code2 => "2",
                AsDirectedByPhysician => "3",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<BasisOfDaysSupplyDeterminationCode> {
        use BasisOfDaysSupplyDeterminationCode::*;
        match code {
            b"0" => Some(NotSpecified),
            b"1" => Some(ExplicitDirections),
            b"2" => Some(Code2),
            b"3" => Some(AsDirectedByPhysician),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use BasisOfDaysSupplyDeterminationCode::*;
        match self {
            NotSpecified => "Not Specified",
            ExplicitDirections => "Explicit Directions",
            Code2 => "PRN Directions (Take as Needed; Pharmacist Estimate)",
            AsDirectedByPhysician => "As Directed by Physician",
        }
    }
    fn from_description(
        description: &str,
    ) -> Option<BasisOfDaysSupplyDeterminationCode> {
        {
            use BasisOfDaysSupplyDeterminationCode::*;
            match description {
                "Not Specified" => Some(NotSpecified),
                "Explicit Directions" => Some(ExplicitDirections),
                "PRN Directions (Take as Needed; Pharmacist Estimate)" => Some(Code2),
                "As Directed by Physician" => Some(AsDirectedByPhysician),
                _ => None,
            }
        }
    }
}
impl Serialize for BasisOfDaysSupplyDeterminationCode {
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
    type Value = BasisOfDaysSupplyDeterminationCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Basis of Days Supply Determination Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        BasisOfDaysSupplyDeterminationCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Basis of Days Supply Determination Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        BasisOfDaysSupplyDeterminationCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Basis of Days Supply Determination Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for BasisOfDaysSupplyDeterminationCode {
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