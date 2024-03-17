use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1364

See docs at <https://www.stedi.com/edi/x12/element/1364>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReviewCode {
    ///A - Case Turned Over to a Consultant
    CaseTurnedOverToAConsultant,
    ///B - Pre-Admission Testing
    PreAdmissionTesting,
    ///C - X-ray or Lab Procedure Related to a Covered Surgery
    XRayOrLabProcedureRelatedToACoveredSurgery,
    ///D - Provider/Supplier determined the service is not covered, but the patient is requesting a formal review by the payer
    CodeD,
    ///E - Beneficiary was notified that the item might not be considered medically necessary and has agreed in writing to pay for the item; A signed waiver is on file with the provider
    CodeE,
    ///F - Beneficiary was notified that the item might not be considered medically necessary and has not agreed to pay for the item; No signed waiver is on file with the provider
    CodeF,
    ///L - Reserved for Local Assignment
    ReservedForLocalAssignment,
    ///N - Reserved for National Assignment
    ReservedForNationalAssignment,
}
impl ReviewCode {
    pub fn code(&self) -> &str {
        {
            use ReviewCode::*;
            match self {
                CaseTurnedOverToAConsultant => "A",
                PreAdmissionTesting => "B",
                XRayOrLabProcedureRelatedToACoveredSurgery => "C",
                CodeD => "D",
                CodeE => "E",
                CodeF => "F",
                ReservedForLocalAssignment => "L",
                ReservedForNationalAssignment => "N",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ReviewCode> {
        use ReviewCode::*;
        match code {
            b"A" => Some(CaseTurnedOverToAConsultant),
            b"B" => Some(PreAdmissionTesting),
            b"C" => Some(XRayOrLabProcedureRelatedToACoveredSurgery),
            b"D" => Some(CodeD),
            b"E" => Some(CodeE),
            b"F" => Some(CodeF),
            b"L" => Some(ReservedForLocalAssignment),
            b"N" => Some(ReservedForNationalAssignment),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ReviewCode::*;
        match self {
            CaseTurnedOverToAConsultant => "Case Turned Over to a Consultant",
            PreAdmissionTesting => "Pre-Admission Testing",
            XRayOrLabProcedureRelatedToACoveredSurgery => {
                "X-ray or Lab Procedure Related to a Covered Surgery"
            }
            CodeD => {
                "Provider/Supplier determined the service is not covered, but the patient is requesting a formal review by the payer"
            }
            CodeE => {
                "Beneficiary was notified that the item might not be considered medically necessary and has agreed in writing to pay for the item; A signed waiver is on file with the provider"
            }
            CodeF => {
                "Beneficiary was notified that the item might not be considered medically necessary and has not agreed to pay for the item; No signed waiver is on file with the provider"
            }
            ReservedForLocalAssignment => "Reserved for Local Assignment",
            ReservedForNationalAssignment => "Reserved for National Assignment",
        }
    }
    fn from_description(description: &str) -> Option<ReviewCode> {
        {
            use ReviewCode::*;
            match description {
                "Case Turned Over to a Consultant" => Some(CaseTurnedOverToAConsultant),
                "Pre-Admission Testing" => Some(PreAdmissionTesting),
                "X-ray or Lab Procedure Related to a Covered Surgery" => {
                    Some(XRayOrLabProcedureRelatedToACoveredSurgery)
                }
                "Provider/Supplier determined the service is not covered, but the patient is requesting a formal review by the payer" => {
                    Some(CodeD)
                }
                "Beneficiary was notified that the item might not be considered medically necessary and has agreed in writing to pay for the item; A signed waiver is on file with the provider" => {
                    Some(CodeE)
                }
                "Beneficiary was notified that the item might not be considered medically necessary and has not agreed to pay for the item; No signed waiver is on file with the provider" => {
                    Some(CodeF)
                }
                "Reserved for Local Assignment" => Some(ReservedForLocalAssignment),
                "Reserved for National Assignment" => Some(ReservedForNationalAssignment),
                _ => None,
            }
        }
    }
}
impl Serialize for ReviewCode {
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
    type Value = ReviewCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Review Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ReviewCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Review Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ReviewCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Review Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for ReviewCode {
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