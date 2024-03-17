use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1319

See docs at <https://www.stedi.com/edi/x12/element/1319>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BasisOfCostDeterminationCode {
    ///0 - Not specified
    NotSpecified,
    ///1 - Average Wholesale Price (AWP)
    Code1,
    ///2 - Local Wholesaler
    LocalWholesaler,
    ///3 - Direct
    Direct,
    ///4 - Estimated Acquisition Cost
    EstimatedAcquisitionCost,
    ///5 - Acquisition Cost
    AcquisitionCost,
    ///6 - Maximum Allowable Cost (MAC)
    Code6,
    ///7 - Usual, Customary, and Reasonable (UCR)
    Code7,
    ///8 - Unit Dose
    UnitDose,
    ///9 - Brand Medically Necessary
    BrandMedicallyNecessary,
    ///10 - Other
    Other,
}
impl BasisOfCostDeterminationCode {
    pub fn code(&self) -> &str {
        {
            use BasisOfCostDeterminationCode::*;
            match self {
                NotSpecified => "0",
                Code1 => "1",
                LocalWholesaler => "2",
                Direct => "3",
                EstimatedAcquisitionCost => "4",
                AcquisitionCost => "5",
                Code6 => "6",
                Code7 => "7",
                UnitDose => "8",
                BrandMedicallyNecessary => "9",
                Other => "10",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<BasisOfCostDeterminationCode> {
        use BasisOfCostDeterminationCode::*;
        match code {
            b"0" => Some(NotSpecified),
            b"1" => Some(Code1),
            b"2" => Some(LocalWholesaler),
            b"3" => Some(Direct),
            b"4" => Some(EstimatedAcquisitionCost),
            b"5" => Some(AcquisitionCost),
            b"6" => Some(Code6),
            b"7" => Some(Code7),
            b"8" => Some(UnitDose),
            b"9" => Some(BrandMedicallyNecessary),
            b"10" => Some(Other),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use BasisOfCostDeterminationCode::*;
        match self {
            NotSpecified => "Not specified",
            Code1 => "Average Wholesale Price (AWP)",
            LocalWholesaler => "Local Wholesaler",
            Direct => "Direct",
            EstimatedAcquisitionCost => "Estimated Acquisition Cost",
            AcquisitionCost => "Acquisition Cost",
            Code6 => "Maximum Allowable Cost (MAC)",
            Code7 => "Usual, Customary, and Reasonable (UCR)",
            UnitDose => "Unit Dose",
            BrandMedicallyNecessary => "Brand Medically Necessary",
            Other => "Other",
        }
    }
    fn from_description(description: &str) -> Option<BasisOfCostDeterminationCode> {
        {
            use BasisOfCostDeterminationCode::*;
            match description {
                "Not specified" => Some(NotSpecified),
                "Average Wholesale Price (AWP)" => Some(Code1),
                "Local Wholesaler" => Some(LocalWholesaler),
                "Direct" => Some(Direct),
                "Estimated Acquisition Cost" => Some(EstimatedAcquisitionCost),
                "Acquisition Cost" => Some(AcquisitionCost),
                "Maximum Allowable Cost (MAC)" => Some(Code6),
                "Usual, Customary, and Reasonable (UCR)" => Some(Code7),
                "Unit Dose" => Some(UnitDose),
                "Brand Medically Necessary" => Some(BrandMedicallyNecessary),
                "Other" => Some(Other),
                _ => None,
            }
        }
    }
}
impl Serialize for BasisOfCostDeterminationCode {
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
    type Value = BasisOfCostDeterminationCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Basis of Cost Determination Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        BasisOfCostDeterminationCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Basis of Cost Determination Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        BasisOfCostDeterminationCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Basis of Cost Determination Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for BasisOfCostDeterminationCode {
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