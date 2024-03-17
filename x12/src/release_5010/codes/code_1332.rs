use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1332

See docs at <https://www.stedi.com/edi/x12-005010/element/1332>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FacilityCodeQualifier {
    ///A - Uniform Billing Claim Form Bill Type
    UniformBillingClaimFormBillType,
    ///B - Place of Service Codes for Professional or Dental Services
    PlaceOfServiceCodesForProfessionalOrDentalServices,
}
impl FacilityCodeQualifier {
    pub fn code(&self) -> &str {
        {
            use FacilityCodeQualifier::*;
            match self {
                UniformBillingClaimFormBillType => "A",
                PlaceOfServiceCodesForProfessionalOrDentalServices => "B",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<FacilityCodeQualifier> {
        use FacilityCodeQualifier::*;
        match code {
            b"A" => Some(UniformBillingClaimFormBillType),
            b"B" => Some(PlaceOfServiceCodesForProfessionalOrDentalServices),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use FacilityCodeQualifier::*;
        match self {
            UniformBillingClaimFormBillType => "Uniform Billing Claim Form Bill Type",
            PlaceOfServiceCodesForProfessionalOrDentalServices => {
                "Place of Service Codes for Professional or Dental Services"
            }
        }
    }
    fn from_description(description: &str) -> Option<FacilityCodeQualifier> {
        {
            use FacilityCodeQualifier::*;
            match description {
                "Uniform Billing Claim Form Bill Type" => {
                    Some(UniformBillingClaimFormBillType)
                }
                "Place of Service Codes for Professional or Dental Services" => {
                    Some(PlaceOfServiceCodesForProfessionalOrDentalServices)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for FacilityCodeQualifier {
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
    type Value = FacilityCodeQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Facility Code Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        FacilityCodeQualifier::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Facility Code Qualifier: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        FacilityCodeQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Facility Code Qualifier: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for FacilityCodeQualifier {
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