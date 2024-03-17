use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1067

See docs at <https://www.stedi.com/edi/x12/element/1067>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaritalStatusCode {
    ///A - Common Law
    CommonLaw,
    ///B - Registered Domestic Partner
    RegisteredDomesticPartner,
    ///C - Not Applicable
    NotApplicable,
    ///D - Divorced
    Divorced,
    ///I - Single
    Single,
    ///K - Unknown
    Unknown,
    ///M - Married
    Married,
    ///R - Unreported
    Unreported,
    ///S - Separated
    Separated,
    ///U - Unmarried (Single or Divorced or Widowed)
    CodeU,
    ///W - Widowed
    Widowed,
    ///X - Legally Separated
    LegallySeparated,
}
impl MaritalStatusCode {
    pub fn code(&self) -> &str {
        {
            use MaritalStatusCode::*;
            match self {
                CommonLaw => "A",
                RegisteredDomesticPartner => "B",
                NotApplicable => "C",
                Divorced => "D",
                Single => "I",
                Unknown => "K",
                Married => "M",
                Unreported => "R",
                Separated => "S",
                CodeU => "U",
                Widowed => "W",
                LegallySeparated => "X",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<MaritalStatusCode> {
        use MaritalStatusCode::*;
        match code {
            b"A" => Some(CommonLaw),
            b"B" => Some(RegisteredDomesticPartner),
            b"C" => Some(NotApplicable),
            b"D" => Some(Divorced),
            b"I" => Some(Single),
            b"K" => Some(Unknown),
            b"M" => Some(Married),
            b"R" => Some(Unreported),
            b"S" => Some(Separated),
            b"U" => Some(CodeU),
            b"W" => Some(Widowed),
            b"X" => Some(LegallySeparated),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use MaritalStatusCode::*;
        match self {
            CommonLaw => "Common Law",
            RegisteredDomesticPartner => "Registered Domestic Partner",
            NotApplicable => "Not Applicable",
            Divorced => "Divorced",
            Single => "Single",
            Unknown => "Unknown",
            Married => "Married",
            Unreported => "Unreported",
            Separated => "Separated",
            CodeU => "Unmarried (Single or Divorced or Widowed)",
            Widowed => "Widowed",
            LegallySeparated => "Legally Separated",
        }
    }
    fn from_description(description: &str) -> Option<MaritalStatusCode> {
        {
            use MaritalStatusCode::*;
            match description {
                "Common Law" => Some(CommonLaw),
                "Registered Domestic Partner" => Some(RegisteredDomesticPartner),
                "Not Applicable" => Some(NotApplicable),
                "Divorced" => Some(Divorced),
                "Single" => Some(Single),
                "Unknown" => Some(Unknown),
                "Married" => Some(Married),
                "Unreported" => Some(Unreported),
                "Separated" => Some(Separated),
                "Unmarried (Single or Divorced or Widowed)" => Some(CodeU),
                "Widowed" => Some(Widowed),
                "Legally Separated" => Some(LegallySeparated),
                _ => None,
            }
        }
    }
}
impl Serialize for MaritalStatusCode {
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
    type Value = MaritalStatusCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Marital Status Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MaritalStatusCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Marital Status Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MaritalStatusCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Marital Status Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for MaritalStatusCode {
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