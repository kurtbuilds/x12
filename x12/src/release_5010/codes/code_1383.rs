use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1383

See docs at <https://www.stedi.com/edi/x12-005010/element/1383>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClaimSubmissionReasonCode {
    ///00 - Original
    Original,
    ///01 - Cancellation
    Cancellation,
    ///02 - Corrected and Verified Original Claim
    CorrectedAndVerifiedOriginalClaim,
    ///03 - Corrected and Verified Final Claim
    CorrectedAndVerifiedFinalClaim,
    ///05 - Replace
    Replace,
    ///07 - Duplicate
    Duplicate,
    ///08 - Pre-Determination
    PreDetermination,
    ///09 - Encounter
    Encounter,
    ///15 - Resubmission
    Resubmission,
    ///16 - Proposed
    Proposed,
    ///17 - Cancel to be Reissued
    CancelToBeReissued,
    ///18 - Reissue
    Reissue,
    ///20 - Final Transmission
    FinalTransmission,
    ///22 - Information Copy
    InformationCopy,
    ///27 - Verify
    Verify,
    ///28 - Late Charges
    LateCharges,
    ///29 - Adjustment
    Adjustment,
    ///PB - Predetermination of Dental Benefits
    PredeterminationOfDentalBenefits,
}
impl ClaimSubmissionReasonCode {
    pub fn code(&self) -> &str {
        {
            use ClaimSubmissionReasonCode::*;
            match self {
                Original => "00",
                Cancellation => "01",
                CorrectedAndVerifiedOriginalClaim => "02",
                CorrectedAndVerifiedFinalClaim => "03",
                Replace => "05",
                Duplicate => "07",
                PreDetermination => "08",
                Encounter => "09",
                Resubmission => "15",
                Proposed => "16",
                CancelToBeReissued => "17",
                Reissue => "18",
                FinalTransmission => "20",
                InformationCopy => "22",
                Verify => "27",
                LateCharges => "28",
                Adjustment => "29",
                PredeterminationOfDentalBenefits => "PB",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ClaimSubmissionReasonCode> {
        use ClaimSubmissionReasonCode::*;
        match code {
            b"00" => Some(Original),
            b"01" => Some(Cancellation),
            b"02" => Some(CorrectedAndVerifiedOriginalClaim),
            b"03" => Some(CorrectedAndVerifiedFinalClaim),
            b"05" => Some(Replace),
            b"07" => Some(Duplicate),
            b"08" => Some(PreDetermination),
            b"09" => Some(Encounter),
            b"15" => Some(Resubmission),
            b"16" => Some(Proposed),
            b"17" => Some(CancelToBeReissued),
            b"18" => Some(Reissue),
            b"20" => Some(FinalTransmission),
            b"22" => Some(InformationCopy),
            b"27" => Some(Verify),
            b"28" => Some(LateCharges),
            b"29" => Some(Adjustment),
            b"PB" => Some(PredeterminationOfDentalBenefits),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ClaimSubmissionReasonCode::*;
        match self {
            Original => "Original",
            Cancellation => "Cancellation",
            CorrectedAndVerifiedOriginalClaim => "Corrected and Verified Original Claim",
            CorrectedAndVerifiedFinalClaim => "Corrected and Verified Final Claim",
            Replace => "Replace",
            Duplicate => "Duplicate",
            PreDetermination => "Pre-Determination",
            Encounter => "Encounter",
            Resubmission => "Resubmission",
            Proposed => "Proposed",
            CancelToBeReissued => "Cancel to be Reissued",
            Reissue => "Reissue",
            FinalTransmission => "Final Transmission",
            InformationCopy => "Information Copy",
            Verify => "Verify",
            LateCharges => "Late Charges",
            Adjustment => "Adjustment",
            PredeterminationOfDentalBenefits => "Predetermination of Dental Benefits",
        }
    }
    fn from_description(description: &str) -> Option<ClaimSubmissionReasonCode> {
        {
            use ClaimSubmissionReasonCode::*;
            match description {
                "Original" => Some(Original),
                "Cancellation" => Some(Cancellation),
                "Corrected and Verified Original Claim" => {
                    Some(CorrectedAndVerifiedOriginalClaim)
                }
                "Corrected and Verified Final Claim" => {
                    Some(CorrectedAndVerifiedFinalClaim)
                }
                "Replace" => Some(Replace),
                "Duplicate" => Some(Duplicate),
                "Pre-Determination" => Some(PreDetermination),
                "Encounter" => Some(Encounter),
                "Resubmission" => Some(Resubmission),
                "Proposed" => Some(Proposed),
                "Cancel to be Reissued" => Some(CancelToBeReissued),
                "Reissue" => Some(Reissue),
                "Final Transmission" => Some(FinalTransmission),
                "Information Copy" => Some(InformationCopy),
                "Verify" => Some(Verify),
                "Late Charges" => Some(LateCharges),
                "Adjustment" => Some(Adjustment),
                "Predetermination of Dental Benefits" => {
                    Some(PredeterminationOfDentalBenefits)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for ClaimSubmissionReasonCode {
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
    type Value = ClaimSubmissionReasonCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Claim Submission Reason Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ClaimSubmissionReasonCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Claim Submission Reason Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ClaimSubmissionReasonCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Claim Submission Reason Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ClaimSubmissionReasonCode {
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