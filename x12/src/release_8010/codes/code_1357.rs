use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1357

See docs at <https://www.stedi.com/edi/x12/element/1357>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PriorAuthorizationTypeCode {
    ///0 - Not Specified
    NotSpecified,
    ///1 - Prior Authorization
    PriorAuthorization,
    ///2 - Medical Certification
    MedicalCertification,
    ///3 - Early & Periodic Screening, Diagnosis, and Treatment (EPSDT)
    Code3,
    ///4 - Exempt from Copay
    ExemptFromCopay,
    ///5 - Exempt From Prescription Limits
    ExemptFromPrescriptionLimits,
    ///6 - Family Planning
    FamilyPlanning,
    ///7 - Aid to Families with Dependent Children (AFDC)
    Code7,
    ///8 - Payer Defined Exemption
    PayerDefinedExemption,
}
impl PriorAuthorizationTypeCode {
    pub fn code(&self) -> &str {
        {
            use PriorAuthorizationTypeCode::*;
            match self {
                NotSpecified => "0",
                PriorAuthorization => "1",
                MedicalCertification => "2",
                Code3 => "3",
                ExemptFromCopay => "4",
                ExemptFromPrescriptionLimits => "5",
                FamilyPlanning => "6",
                Code7 => "7",
                PayerDefinedExemption => "8",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<PriorAuthorizationTypeCode> {
        use PriorAuthorizationTypeCode::*;
        match code {
            b"0" => Some(NotSpecified),
            b"1" => Some(PriorAuthorization),
            b"2" => Some(MedicalCertification),
            b"3" => Some(Code3),
            b"4" => Some(ExemptFromCopay),
            b"5" => Some(ExemptFromPrescriptionLimits),
            b"6" => Some(FamilyPlanning),
            b"7" => Some(Code7),
            b"8" => Some(PayerDefinedExemption),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use PriorAuthorizationTypeCode::*;
        match self {
            NotSpecified => "Not Specified",
            PriorAuthorization => "Prior Authorization",
            MedicalCertification => "Medical Certification",
            Code3 => "Early & Periodic Screening, Diagnosis, and Treatment (EPSDT)",
            ExemptFromCopay => "Exempt from Copay",
            ExemptFromPrescriptionLimits => "Exempt From Prescription Limits",
            FamilyPlanning => "Family Planning",
            Code7 => "Aid to Families with Dependent Children (AFDC)",
            PayerDefinedExemption => "Payer Defined Exemption",
        }
    }
    fn from_description(description: &str) -> Option<PriorAuthorizationTypeCode> {
        {
            use PriorAuthorizationTypeCode::*;
            match description {
                "Not Specified" => Some(NotSpecified),
                "Prior Authorization" => Some(PriorAuthorization),
                "Medical Certification" => Some(MedicalCertification),
                "Early & Periodic Screening, Diagnosis, and Treatment (EPSDT)" => {
                    Some(Code3)
                }
                "Exempt from Copay" => Some(ExemptFromCopay),
                "Exempt From Prescription Limits" => Some(ExemptFromPrescriptionLimits),
                "Family Planning" => Some(FamilyPlanning),
                "Aid to Families with Dependent Children (AFDC)" => Some(Code7),
                "Payer Defined Exemption" => Some(PayerDefinedExemption),
                _ => None,
            }
        }
    }
}
impl Serialize for PriorAuthorizationTypeCode {
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
    type Value = PriorAuthorizationTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Prior Authorization Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PriorAuthorizationTypeCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Prior Authorization Type Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PriorAuthorizationTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Prior Authorization Type Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for PriorAuthorizationTypeCode {
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