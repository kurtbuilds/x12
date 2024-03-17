use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1329

See docs at <https://www.stedi.com/edi/x12-005010/element/1329>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DispenseAsWrittenCode {
    ///0 - Not Dispense As Written (DAW)
    Code0,
    ///1 - Physician Dispense As Written (DAW)
    Code1,
    ///2 - Patient Dispense As Written (DAW)
    Code2,
    ///3 - Pharmacy Dispense As Written (DAW)
    Code3,
    ///4 - No generic available
    NoGenericAvailable,
    ///5 - Brand Dispensed as Generic
    BrandDispensedAsGeneric,
    ///6 - Override
    Override,
    ///7 - Substitution Not Allowed-Brand Drug Mandated by Law
    SubstitutionNotAllowedBrandDrugMandatedByLaw,
    ///8 - Substitution Allowed-Generic Drug Not Available in Marketplace
    SubstitutionAllowedGenericDrugNotAvailableInMarketplace,
    ///9 - Other
    Other,
}
impl DispenseAsWrittenCode {
    pub fn code(&self) -> &str {
        {
            use DispenseAsWrittenCode::*;
            match self {
                Code0 => "0",
                Code1 => "1",
                Code2 => "2",
                Code3 => "3",
                NoGenericAvailable => "4",
                BrandDispensedAsGeneric => "5",
                Override => "6",
                SubstitutionNotAllowedBrandDrugMandatedByLaw => "7",
                SubstitutionAllowedGenericDrugNotAvailableInMarketplace => "8",
                Other => "9",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<DispenseAsWrittenCode> {
        use DispenseAsWrittenCode::*;
        match code {
            b"0" => Some(Code0),
            b"1" => Some(Code1),
            b"2" => Some(Code2),
            b"3" => Some(Code3),
            b"4" => Some(NoGenericAvailable),
            b"5" => Some(BrandDispensedAsGeneric),
            b"6" => Some(Override),
            b"7" => Some(SubstitutionNotAllowedBrandDrugMandatedByLaw),
            b"8" => Some(SubstitutionAllowedGenericDrugNotAvailableInMarketplace),
            b"9" => Some(Other),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use DispenseAsWrittenCode::*;
        match self {
            Code0 => "Not Dispense As Written (DAW)",
            Code1 => "Physician Dispense As Written (DAW)",
            Code2 => "Patient Dispense As Written (DAW)",
            Code3 => "Pharmacy Dispense As Written (DAW)",
            NoGenericAvailable => "No generic available",
            BrandDispensedAsGeneric => "Brand Dispensed as Generic",
            Override => "Override",
            SubstitutionNotAllowedBrandDrugMandatedByLaw => {
                "Substitution Not Allowed-Brand Drug Mandated by Law"
            }
            SubstitutionAllowedGenericDrugNotAvailableInMarketplace => {
                "Substitution Allowed-Generic Drug Not Available in Marketplace"
            }
            Other => "Other",
        }
    }
    fn from_description(description: &str) -> Option<DispenseAsWrittenCode> {
        {
            use DispenseAsWrittenCode::*;
            match description {
                "Not Dispense As Written (DAW)" => Some(Code0),
                "Physician Dispense As Written (DAW)" => Some(Code1),
                "Patient Dispense As Written (DAW)" => Some(Code2),
                "Pharmacy Dispense As Written (DAW)" => Some(Code3),
                "No generic available" => Some(NoGenericAvailable),
                "Brand Dispensed as Generic" => Some(BrandDispensedAsGeneric),
                "Override" => Some(Override),
                "Substitution Not Allowed-Brand Drug Mandated by Law" => {
                    Some(SubstitutionNotAllowedBrandDrugMandatedByLaw)
                }
                "Substitution Allowed-Generic Drug Not Available in Marketplace" => {
                    Some(SubstitutionAllowedGenericDrugNotAvailableInMarketplace)
                }
                "Other" => Some(Other),
                _ => None,
            }
        }
    }
}
impl Serialize for DispenseAsWrittenCode {
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
    type Value = DispenseAsWrittenCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Dispense as Written Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DispenseAsWrittenCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Dispense as Written Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DispenseAsWrittenCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Dispense as Written Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for DispenseAsWrittenCode {
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