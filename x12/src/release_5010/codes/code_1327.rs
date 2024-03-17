use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1327

See docs at <https://www.stedi.com/edi/x12/element/1327>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CopayStatusCode {
    ///0 - Copay exempt
    CopayExempt,
    ///1 - Recipient did not pay when asked
    RecipientDidNotPayWhenAsked,
    ///2 - Recipient paid when asked
    RecipientPaidWhenAsked,
    ///3 - Payment was not requested
    PaymentWasNotRequested,
}
impl CopayStatusCode {
    pub fn code(&self) -> &str {
        {
            use CopayStatusCode::*;
            match self {
                CopayExempt => "0",
                RecipientDidNotPayWhenAsked => "1",
                RecipientPaidWhenAsked => "2",
                PaymentWasNotRequested => "3",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<CopayStatusCode> {
        use CopayStatusCode::*;
        match code {
            b"0" => Some(CopayExempt),
            b"1" => Some(RecipientDidNotPayWhenAsked),
            b"2" => Some(RecipientPaidWhenAsked),
            b"3" => Some(PaymentWasNotRequested),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use CopayStatusCode::*;
        match self {
            CopayExempt => "Copay exempt",
            RecipientDidNotPayWhenAsked => "Recipient did not pay when asked",
            RecipientPaidWhenAsked => "Recipient paid when asked",
            PaymentWasNotRequested => "Payment was not requested",
        }
    }
    fn from_description(description: &str) -> Option<CopayStatusCode> {
        {
            use CopayStatusCode::*;
            match description {
                "Copay exempt" => Some(CopayExempt),
                "Recipient did not pay when asked" => Some(RecipientDidNotPayWhenAsked),
                "Recipient paid when asked" => Some(RecipientPaidWhenAsked),
                "Payment was not requested" => Some(PaymentWasNotRequested),
                _ => None,
            }
        }
    }
}
impl Serialize for CopayStatusCode {
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
    type Value = CopayStatusCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Copay Status Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CopayStatusCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Copay Status Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CopayStatusCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Copay Status Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for CopayStatusCode {
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