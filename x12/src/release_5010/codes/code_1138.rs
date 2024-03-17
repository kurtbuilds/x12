use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1138

See docs at <https://www.stedi.com/edi/x12-005010/element/1138>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PayerResponsibilitySequenceNumberCode {
    ///A - Payer Responsibility Four
    PayerResponsibilityFour,
    ///B - Payer Responsibility Five
    PayerResponsibilityFive,
    ///C - Payer Responsibility Six
    PayerResponsibilitySix,
    ///D - Payer Responsibility Seven
    PayerResponsibilitySeven,
    ///E - Payer Responsibility Eight
    PayerResponsibilityEight,
    ///F - Payer Responsibility Nine
    PayerResponsibilityNine,
    ///G - Payer Responsibility Ten
    PayerResponsibilityTen,
    ///H - Payer Responsibility Eleven
    PayerResponsibilityEleven,
    ///N - Unconfirmed
    Unconfirmed,
    ///O - Noncapitated Agreement
    NoncapitatedAgreement,
    ///P - Primary
    Primary,
    ///S - Secondary
    Secondary,
    ///T - Tertiary
    Tertiary,
    ///U - Unknown
    Unknown,
}
impl PayerResponsibilitySequenceNumberCode {
    pub fn code(&self) -> &str {
        {
            use PayerResponsibilitySequenceNumberCode::*;
            match self {
                PayerResponsibilityFour => "A",
                PayerResponsibilityFive => "B",
                PayerResponsibilitySix => "C",
                PayerResponsibilitySeven => "D",
                PayerResponsibilityEight => "E",
                PayerResponsibilityNine => "F",
                PayerResponsibilityTen => "G",
                PayerResponsibilityEleven => "H",
                Unconfirmed => "N",
                NoncapitatedAgreement => "O",
                Primary => "P",
                Secondary => "S",
                Tertiary => "T",
                Unknown => "U",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<PayerResponsibilitySequenceNumberCode> {
        use PayerResponsibilitySequenceNumberCode::*;
        match code {
            b"A" => Some(PayerResponsibilityFour),
            b"B" => Some(PayerResponsibilityFive),
            b"C" => Some(PayerResponsibilitySix),
            b"D" => Some(PayerResponsibilitySeven),
            b"E" => Some(PayerResponsibilityEight),
            b"F" => Some(PayerResponsibilityNine),
            b"G" => Some(PayerResponsibilityTen),
            b"H" => Some(PayerResponsibilityEleven),
            b"N" => Some(Unconfirmed),
            b"O" => Some(NoncapitatedAgreement),
            b"P" => Some(Primary),
            b"S" => Some(Secondary),
            b"T" => Some(Tertiary),
            b"U" => Some(Unknown),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use PayerResponsibilitySequenceNumberCode::*;
        match self {
            PayerResponsibilityFour => "Payer Responsibility Four",
            PayerResponsibilityFive => "Payer Responsibility Five",
            PayerResponsibilitySix => "Payer Responsibility Six",
            PayerResponsibilitySeven => "Payer Responsibility Seven",
            PayerResponsibilityEight => "Payer Responsibility Eight",
            PayerResponsibilityNine => "Payer Responsibility Nine",
            PayerResponsibilityTen => "Payer Responsibility Ten",
            PayerResponsibilityEleven => "Payer Responsibility Eleven",
            Unconfirmed => "Unconfirmed",
            NoncapitatedAgreement => "Noncapitated Agreement",
            Primary => "Primary",
            Secondary => "Secondary",
            Tertiary => "Tertiary",
            Unknown => "Unknown",
        }
    }
    fn from_description(
        description: &str,
    ) -> Option<PayerResponsibilitySequenceNumberCode> {
        {
            use PayerResponsibilitySequenceNumberCode::*;
            match description {
                "Payer Responsibility Four" => Some(PayerResponsibilityFour),
                "Payer Responsibility Five" => Some(PayerResponsibilityFive),
                "Payer Responsibility Six" => Some(PayerResponsibilitySix),
                "Payer Responsibility Seven" => Some(PayerResponsibilitySeven),
                "Payer Responsibility Eight" => Some(PayerResponsibilityEight),
                "Payer Responsibility Nine" => Some(PayerResponsibilityNine),
                "Payer Responsibility Ten" => Some(PayerResponsibilityTen),
                "Payer Responsibility Eleven" => Some(PayerResponsibilityEleven),
                "Unconfirmed" => Some(Unconfirmed),
                "Noncapitated Agreement" => Some(NoncapitatedAgreement),
                "Primary" => Some(Primary),
                "Secondary" => Some(Secondary),
                "Tertiary" => Some(Tertiary),
                "Unknown" => Some(Unknown),
                _ => None,
            }
        }
    }
}
impl Serialize for PayerResponsibilitySequenceNumberCode {
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
    type Value = PayerResponsibilitySequenceNumberCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Payer Responsibility Sequence Number Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PayerResponsibilitySequenceNumberCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Payer Responsibility Sequence Number Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PayerResponsibilitySequenceNumberCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Payer Responsibility Sequence Number Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for PayerResponsibilitySequenceNumberCode {
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