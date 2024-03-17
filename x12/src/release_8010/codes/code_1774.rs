use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1774

See docs at <https://www.stedi.com/edi/x12/element/1774>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClaimAuthorizationExceptionCode {
    ///1 - Immediate/Urgent Care
    ImmediateUrgentCare,
    ///2 - Services Rendered in a Retroactive Period
    ServicesRenderedInARetroactivePeriod,
    ///3 - Emergency Care
    EmergencyCare,
    ///4 - Subscriber has Temporary Medicaid
    SubscriberHasTemporaryMedicaid,
    ///5 - Request from County for Second Opinion to Determine if Recipient Can Work
    RequestFromCountyForSecondOpinionToDetermineIfRecipientCanWork,
    ///6 - Request for Override Pending
    RequestForOverridePending,
    ///7 - Special Handling
    SpecialHandling,
    ///Z - Mutually Defined
    MutuallyDefined,
}
impl ClaimAuthorizationExceptionCode {
    pub fn code(&self) -> &str {
        {
            use ClaimAuthorizationExceptionCode::*;
            match self {
                ImmediateUrgentCare => "1",
                ServicesRenderedInARetroactivePeriod => "2",
                EmergencyCare => "3",
                SubscriberHasTemporaryMedicaid => "4",
                RequestFromCountyForSecondOpinionToDetermineIfRecipientCanWork => "5",
                RequestForOverridePending => "6",
                SpecialHandling => "7",
                MutuallyDefined => "Z",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ClaimAuthorizationExceptionCode> {
        use ClaimAuthorizationExceptionCode::*;
        match code {
            b"1" => Some(ImmediateUrgentCare),
            b"2" => Some(ServicesRenderedInARetroactivePeriod),
            b"3" => Some(EmergencyCare),
            b"4" => Some(SubscriberHasTemporaryMedicaid),
            b"5" => Some(RequestFromCountyForSecondOpinionToDetermineIfRecipientCanWork),
            b"6" => Some(RequestForOverridePending),
            b"7" => Some(SpecialHandling),
            b"Z" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ClaimAuthorizationExceptionCode::*;
        match self {
            ImmediateUrgentCare => "Immediate/Urgent Care",
            ServicesRenderedInARetroactivePeriod => {
                "Services Rendered in a Retroactive Period"
            }
            EmergencyCare => "Emergency Care",
            SubscriberHasTemporaryMedicaid => "Subscriber has Temporary Medicaid",
            RequestFromCountyForSecondOpinionToDetermineIfRecipientCanWork => {
                "Request from County for Second Opinion to Determine if Recipient Can Work"
            }
            RequestForOverridePending => "Request for Override Pending",
            SpecialHandling => "Special Handling",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<ClaimAuthorizationExceptionCode> {
        {
            use ClaimAuthorizationExceptionCode::*;
            match description {
                "Immediate/Urgent Care" => Some(ImmediateUrgentCare),
                "Services Rendered in a Retroactive Period" => {
                    Some(ServicesRenderedInARetroactivePeriod)
                }
                "Emergency Care" => Some(EmergencyCare),
                "Subscriber has Temporary Medicaid" => {
                    Some(SubscriberHasTemporaryMedicaid)
                }
                "Request from County for Second Opinion to Determine if Recipient Can Work" => {
                    Some(RequestFromCountyForSecondOpinionToDetermineIfRecipientCanWork)
                }
                "Request for Override Pending" => Some(RequestForOverridePending),
                "Special Handling" => Some(SpecialHandling),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for ClaimAuthorizationExceptionCode {
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
    type Value = ClaimAuthorizationExceptionCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Claim Authorization Exception Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ClaimAuthorizationExceptionCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Claim Authorization Exception Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ClaimAuthorizationExceptionCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Claim Authorization Exception Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ClaimAuthorizationExceptionCode {
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