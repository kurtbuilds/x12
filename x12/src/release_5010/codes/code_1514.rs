use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1514

See docs at <https://www.stedi.com/edi/x12/element/1514>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DelayReasonCode {
    ///1 - Proof of Eligibility Unknown or Unavailable
    ProofOfEligibilityUnknownOrUnavailable,
    ///2 - Litigation
    Litigation,
    ///3 - Authorization Delays
    AuthorizationDelays,
    ///4 - Delay in Certifying Provider
    DelayInCertifyingProvider,
    ///5 - Delay in Supplying Billing Forms
    DelayInSupplyingBillingForms,
    ///6 - Delay in Delivery of Custom-made Appliances
    DelayInDeliveryOfCustomMadeAppliances,
    ///7 - Third Party Processing Delay
    ThirdPartyProcessingDelay,
    ///8 - Delay in Eligibility Determination
    DelayInEligibilityDetermination,
    ///9 - Original Claim Rejected or Denied Due to a Reason Unrelated to the Billing Limitation Rules
    OriginalClaimRejectedOrDeniedDueToAReasonUnrelatedToTheBillingLimitationRules,
    ///10 - Administration Delay in the Prior Approval Process
    AdministrationDelayInThePriorApprovalProcess,
    ///11 - Other
    Other,
    ///15 - Natural Disaster
    NaturalDisaster,
    ///16 - Lack of Information
    LackOf,
    ///17 - No response to initial request
    NoResponseToInitialRequest,
}
impl DelayReasonCode {
    pub fn code(&self) -> &str {
        {
            use DelayReasonCode::*;
            match self {
                ProofOfEligibilityUnknownOrUnavailable => "1",
                Litigation => "2",
                AuthorizationDelays => "3",
                DelayInCertifyingProvider => "4",
                DelayInSupplyingBillingForms => "5",
                DelayInDeliveryOfCustomMadeAppliances => "6",
                ThirdPartyProcessingDelay => "7",
                DelayInEligibilityDetermination => "8",
                OriginalClaimRejectedOrDeniedDueToAReasonUnrelatedToTheBillingLimitationRules => {
                    "9"
                }
                AdministrationDelayInThePriorApprovalProcess => "10",
                Other => "11",
                NaturalDisaster => "15",
                LackOf => "16",
                NoResponseToInitialRequest => "17",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<DelayReasonCode> {
        use DelayReasonCode::*;
        match code {
            b"1" => Some(ProofOfEligibilityUnknownOrUnavailable),
            b"2" => Some(Litigation),
            b"3" => Some(AuthorizationDelays),
            b"4" => Some(DelayInCertifyingProvider),
            b"5" => Some(DelayInSupplyingBillingForms),
            b"6" => Some(DelayInDeliveryOfCustomMadeAppliances),
            b"7" => Some(ThirdPartyProcessingDelay),
            b"8" => Some(DelayInEligibilityDetermination),
            b"9" => {
                Some(
                    OriginalClaimRejectedOrDeniedDueToAReasonUnrelatedToTheBillingLimitationRules,
                )
            }
            b"10" => Some(AdministrationDelayInThePriorApprovalProcess),
            b"11" => Some(Other),
            b"15" => Some(NaturalDisaster),
            b"16" => Some(LackOf),
            b"17" => Some(NoResponseToInitialRequest),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use DelayReasonCode::*;
        match self {
            ProofOfEligibilityUnknownOrUnavailable => {
                "Proof of Eligibility Unknown or Unavailable"
            }
            Litigation => "Litigation",
            AuthorizationDelays => "Authorization Delays",
            DelayInCertifyingProvider => "Delay in Certifying Provider",
            DelayInSupplyingBillingForms => "Delay in Supplying Billing Forms",
            DelayInDeliveryOfCustomMadeAppliances => {
                "Delay in Delivery of Custom-made Appliances"
            }
            ThirdPartyProcessingDelay => "Third Party Processing Delay",
            DelayInEligibilityDetermination => "Delay in Eligibility Determination",
            OriginalClaimRejectedOrDeniedDueToAReasonUnrelatedToTheBillingLimitationRules => {
                "Original Claim Rejected or Denied Due to a Reason Unrelated to the Billing Limitation Rules"
            }
            AdministrationDelayInThePriorApprovalProcess => {
                "Administration Delay in the Prior Approval Process"
            }
            Other => "Other",
            NaturalDisaster => "Natural Disaster",
            LackOf => "Lack of Information",
            NoResponseToInitialRequest => "No response to initial request",
        }
    }
    fn from_description(description: &str) -> Option<DelayReasonCode> {
        {
            use DelayReasonCode::*;
            match description {
                "Proof of Eligibility Unknown or Unavailable" => {
                    Some(ProofOfEligibilityUnknownOrUnavailable)
                }
                "Litigation" => Some(Litigation),
                "Authorization Delays" => Some(AuthorizationDelays),
                "Delay in Certifying Provider" => Some(DelayInCertifyingProvider),
                "Delay in Supplying Billing Forms" => Some(DelayInSupplyingBillingForms),
                "Delay in Delivery of Custom-made Appliances" => {
                    Some(DelayInDeliveryOfCustomMadeAppliances)
                }
                "Third Party Processing Delay" => Some(ThirdPartyProcessingDelay),
                "Delay in Eligibility Determination" => {
                    Some(DelayInEligibilityDetermination)
                }
                "Original Claim Rejected or Denied Due to a Reason Unrelated to the Billing Limitation Rules" => {
                    Some(
                        OriginalClaimRejectedOrDeniedDueToAReasonUnrelatedToTheBillingLimitationRules,
                    )
                }
                "Administration Delay in the Prior Approval Process" => {
                    Some(AdministrationDelayInThePriorApprovalProcess)
                }
                "Other" => Some(Other),
                "Natural Disaster" => Some(NaturalDisaster),
                "Lack of Information" => Some(LackOf),
                "No response to initial request" => Some(NoResponseToInitialRequest),
                _ => None,
            }
        }
    }
}
impl Serialize for DelayReasonCode {
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
    type Value = DelayReasonCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Delay Reason Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DelayReasonCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Delay Reason Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DelayReasonCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Delay Reason Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for DelayReasonCode {
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