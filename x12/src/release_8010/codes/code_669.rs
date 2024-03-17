use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**669

See docs at <https://www.stedi.com/edi/x12/element/669>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurrencyMarketExchangeCode {
    ///IMF - International Monetary Fund
    InternationalMonetaryFund,
    ///LNF - London (England) Exchange - First Closing
    CodeLNF,
    ///LNS - London (England) Exchange - Second Closing
    CodeLNS,
    ///NYC - New York Foreign Exchange
    NewYorkForeignExchange,
    ///PHI - Philadelphia Foreign Exchange
    PhiladelphiaForeignExchange,
    ///ZUR - Zurich (Switzerland) Exchange
    CodeZUR,
}
impl CurrencyMarketExchangeCode {
    pub fn code(&self) -> &str {
        {
            use CurrencyMarketExchangeCode::*;
            match self {
                InternationalMonetaryFund => "IMF",
                CodeLNF => "LNF",
                CodeLNS => "LNS",
                NewYorkForeignExchange => "NYC",
                PhiladelphiaForeignExchange => "PHI",
                CodeZUR => "ZUR",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<CurrencyMarketExchangeCode> {
        use CurrencyMarketExchangeCode::*;
        match code {
            b"IMF" => Some(InternationalMonetaryFund),
            b"LNF" => Some(CodeLNF),
            b"LNS" => Some(CodeLNS),
            b"NYC" => Some(NewYorkForeignExchange),
            b"PHI" => Some(PhiladelphiaForeignExchange),
            b"ZUR" => Some(CodeZUR),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use CurrencyMarketExchangeCode::*;
        match self {
            InternationalMonetaryFund => "International Monetary Fund",
            CodeLNF => "London (England) Exchange - First Closing",
            CodeLNS => "London (England) Exchange - Second Closing",
            NewYorkForeignExchange => "New York Foreign Exchange",
            PhiladelphiaForeignExchange => "Philadelphia Foreign Exchange",
            CodeZUR => "Zurich (Switzerland) Exchange",
        }
    }
    fn from_description(description: &str) -> Option<CurrencyMarketExchangeCode> {
        {
            use CurrencyMarketExchangeCode::*;
            match description {
                "International Monetary Fund" => Some(InternationalMonetaryFund),
                "London (England) Exchange - First Closing" => Some(CodeLNF),
                "London (England) Exchange - Second Closing" => Some(CodeLNS),
                "New York Foreign Exchange" => Some(NewYorkForeignExchange),
                "Philadelphia Foreign Exchange" => Some(PhiladelphiaForeignExchange),
                "Zurich (Switzerland) Exchange" => Some(CodeZUR),
                _ => None,
            }
        }
    }
}
impl Serialize for CurrencyMarketExchangeCode {
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
    type Value = CurrencyMarketExchangeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Currency Market/Exchange Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CurrencyMarketExchangeCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Currency Market/Exchange Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CurrencyMarketExchangeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Currency Market/Exchange Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for CurrencyMarketExchangeCode {
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