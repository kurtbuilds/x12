use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1033

See docs at <https://www.stedi.com/edi/x12-005010/element/1033>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClaimAdjustmentGroupCode {
    ///CO - Contractual Obligations
    ContractualObligations,
    ///CR - Correction and Reversals
    CorrectionAndReversals,
    ///DE - Denials
    Denials,
    ///MA - Medicare Adjustments
    MedicareAdjustments,
    ///NC - Disallowed charges and noncovered services
    DisallowedChargesAndNoncoveredServices,
    ///OA - Other adjustments
    OtherAdjustments,
    ///PI - Payor Initiated Reductions
    PayorInitiatedReductions,
    ///PR - Patient Responsibility
    PatientResponsibility,
    ///RR - Regulatory Requirement
    RegulatoryRequirement,
}
impl ClaimAdjustmentGroupCode {
    pub fn code(&self) -> &str {
        {
            use ClaimAdjustmentGroupCode::*;
            match self {
                ContractualObligations => "CO",
                CorrectionAndReversals => "CR",
                Denials => "DE",
                MedicareAdjustments => "MA",
                DisallowedChargesAndNoncoveredServices => "NC",
                OtherAdjustments => "OA",
                PayorInitiatedReductions => "PI",
                PatientResponsibility => "PR",
                RegulatoryRequirement => "RR",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ClaimAdjustmentGroupCode> {
        use ClaimAdjustmentGroupCode::*;
        match code {
            b"CO" => Some(ContractualObligations),
            b"CR" => Some(CorrectionAndReversals),
            b"DE" => Some(Denials),
            b"MA" => Some(MedicareAdjustments),
            b"NC" => Some(DisallowedChargesAndNoncoveredServices),
            b"OA" => Some(OtherAdjustments),
            b"PI" => Some(PayorInitiatedReductions),
            b"PR" => Some(PatientResponsibility),
            b"RR" => Some(RegulatoryRequirement),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ClaimAdjustmentGroupCode::*;
        match self {
            ContractualObligations => "Contractual Obligations",
            CorrectionAndReversals => "Correction and Reversals",
            Denials => "Denials",
            MedicareAdjustments => "Medicare Adjustments",
            DisallowedChargesAndNoncoveredServices => {
                "Disallowed charges and noncovered services"
            }
            OtherAdjustments => "Other adjustments",
            PayorInitiatedReductions => "Payor Initiated Reductions",
            PatientResponsibility => "Patient Responsibility",
            RegulatoryRequirement => "Regulatory Requirement",
        }
    }
    fn from_description(description: &str) -> Option<ClaimAdjustmentGroupCode> {
        {
            use ClaimAdjustmentGroupCode::*;
            match description {
                "Contractual Obligations" => Some(ContractualObligations),
                "Correction and Reversals" => Some(CorrectionAndReversals),
                "Denials" => Some(Denials),
                "Medicare Adjustments" => Some(MedicareAdjustments),
                "Disallowed charges and noncovered services" => {
                    Some(DisallowedChargesAndNoncoveredServices)
                }
                "Other adjustments" => Some(OtherAdjustments),
                "Payor Initiated Reductions" => Some(PayorInitiatedReductions),
                "Patient Responsibility" => Some(PatientResponsibility),
                "Regulatory Requirement" => Some(RegulatoryRequirement),
                _ => None,
            }
        }
    }
}
impl Serialize for ClaimAdjustmentGroupCode {
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
    type Value = ClaimAdjustmentGroupCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Claim Adjustment Group Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ClaimAdjustmentGroupCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Claim Adjustment Group Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ClaimAdjustmentGroupCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Claim Adjustment Group Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ClaimAdjustmentGroupCode {
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