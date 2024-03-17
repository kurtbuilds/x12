use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1525

See docs at <https://www.stedi.com/edi/x12-005010/element/1525>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RequestCategoryCode {
    ///AR - Admission Review
    AdmissionReview,
    ///BA - Batch
    Batch,
    ///HS - Health Services Review
    HealthServicesReview,
    ///IN - Individual
    Individual,
    ///PR - Program Referral
    ProgramReferral,
    ///RE - Recurring
    Recurring,
    ///SC - Specialty Care Review
    SpecialtyCareReview,
}
impl RequestCategoryCode {
    pub fn code(&self) -> &str {
        {
            use RequestCategoryCode::*;
            match self {
                AdmissionReview => "AR",
                Batch => "BA",
                HealthServicesReview => "HS",
                Individual => "IN",
                ProgramReferral => "PR",
                Recurring => "RE",
                SpecialtyCareReview => "SC",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<RequestCategoryCode> {
        use RequestCategoryCode::*;
        match code {
            b"AR" => Some(AdmissionReview),
            b"BA" => Some(Batch),
            b"HS" => Some(HealthServicesReview),
            b"IN" => Some(Individual),
            b"PR" => Some(ProgramReferral),
            b"RE" => Some(Recurring),
            b"SC" => Some(SpecialtyCareReview),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use RequestCategoryCode::*;
        match self {
            AdmissionReview => "Admission Review",
            Batch => "Batch",
            HealthServicesReview => "Health Services Review",
            Individual => "Individual",
            ProgramReferral => "Program Referral",
            Recurring => "Recurring",
            SpecialtyCareReview => "Specialty Care Review",
        }
    }
    fn from_description(description: &str) -> Option<RequestCategoryCode> {
        {
            use RequestCategoryCode::*;
            match description {
                "Admission Review" => Some(AdmissionReview),
                "Batch" => Some(Batch),
                "Health Services Review" => Some(HealthServicesReview),
                "Individual" => Some(Individual),
                "Program Referral" => Some(ProgramReferral),
                "Recurring" => Some(Recurring),
                "Specialty Care Review" => Some(SpecialtyCareReview),
                _ => None,
            }
        }
    }
}
impl Serialize for RequestCategoryCode {
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
    type Value = RequestCategoryCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Request Category Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        RequestCategoryCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Request Category Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        RequestCategoryCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Request Category Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for RequestCategoryCode {
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