use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**648

See docs at <https://www.stedi.com/edi/x12/element/648>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PriceMultiplierQualifier {
    ///APB - Allowed Multiplier - Billed
    AllowedMultiplierBilled,
    ///API - Allowed Multiplier - Invoiced
    AllowedMultiplierInvoiced,
    ///APP - Allowed Multiplier - Medicare
    AllowedMultiplierMedicare,
    ///CSD - Cost Markup Multiplier - Original Cost
    CostMarkupMultiplierOriginalCost,
    ///CSR - Cost Markup Multiplier - Retail Cost
    CostMarkupMultiplierRetailCost,
    ///DAB - Default Allowed Multiplier - Billed
    DefaultAllowedMultiplierBilled,
    ///DAR - Default Allowed Multiplier - Medicare
    DefaultAllowedMultiplierMedicare,
    ///DIS - Discount Multiplier
    DiscountMultiplier,
    ///ILP - Cost Markup Multiplier - Wholesale Cost
    CostMarkupMultiplierWholesaleCost,
    ///PSP - Percent Solution Multiplier
    PercentSolutionMultiplier,
    ///SEL - Selling Multiplier
    SellingMultiplier,
}
impl PriceMultiplierQualifier {
    pub fn code(&self) -> &str {
        {
            use PriceMultiplierQualifier::*;
            match self {
                AllowedMultiplierBilled => "APB",
                AllowedMultiplierInvoiced => "API",
                AllowedMultiplierMedicare => "APP",
                CostMarkupMultiplierOriginalCost => "CSD",
                CostMarkupMultiplierRetailCost => "CSR",
                DefaultAllowedMultiplierBilled => "DAB",
                DefaultAllowedMultiplierMedicare => "DAR",
                DiscountMultiplier => "DIS",
                CostMarkupMultiplierWholesaleCost => "ILP",
                PercentSolutionMultiplier => "PSP",
                SellingMultiplier => "SEL",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<PriceMultiplierQualifier> {
        use PriceMultiplierQualifier::*;
        match code {
            b"APB" => Some(AllowedMultiplierBilled),
            b"API" => Some(AllowedMultiplierInvoiced),
            b"APP" => Some(AllowedMultiplierMedicare),
            b"CSD" => Some(CostMarkupMultiplierOriginalCost),
            b"CSR" => Some(CostMarkupMultiplierRetailCost),
            b"DAB" => Some(DefaultAllowedMultiplierBilled),
            b"DAR" => Some(DefaultAllowedMultiplierMedicare),
            b"DIS" => Some(DiscountMultiplier),
            b"ILP" => Some(CostMarkupMultiplierWholesaleCost),
            b"PSP" => Some(PercentSolutionMultiplier),
            b"SEL" => Some(SellingMultiplier),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use PriceMultiplierQualifier::*;
        match self {
            AllowedMultiplierBilled => "Allowed Multiplier - Billed",
            AllowedMultiplierInvoiced => "Allowed Multiplier - Invoiced",
            AllowedMultiplierMedicare => "Allowed Multiplier - Medicare",
            CostMarkupMultiplierOriginalCost => "Cost Markup Multiplier - Original Cost",
            CostMarkupMultiplierRetailCost => "Cost Markup Multiplier - Retail Cost",
            DefaultAllowedMultiplierBilled => "Default Allowed Multiplier - Billed",
            DefaultAllowedMultiplierMedicare => "Default Allowed Multiplier - Medicare",
            DiscountMultiplier => "Discount Multiplier",
            CostMarkupMultiplierWholesaleCost => {
                "Cost Markup Multiplier - Wholesale Cost"
            }
            PercentSolutionMultiplier => "Percent Solution Multiplier",
            SellingMultiplier => "Selling Multiplier",
        }
    }
    fn from_description(description: &str) -> Option<PriceMultiplierQualifier> {
        {
            use PriceMultiplierQualifier::*;
            match description {
                "Allowed Multiplier - Billed" => Some(AllowedMultiplierBilled),
                "Allowed Multiplier - Invoiced" => Some(AllowedMultiplierInvoiced),
                "Allowed Multiplier - Medicare" => Some(AllowedMultiplierMedicare),
                "Cost Markup Multiplier - Original Cost" => {
                    Some(CostMarkupMultiplierOriginalCost)
                }
                "Cost Markup Multiplier - Retail Cost" => {
                    Some(CostMarkupMultiplierRetailCost)
                }
                "Default Allowed Multiplier - Billed" => {
                    Some(DefaultAllowedMultiplierBilled)
                }
                "Default Allowed Multiplier - Medicare" => {
                    Some(DefaultAllowedMultiplierMedicare)
                }
                "Discount Multiplier" => Some(DiscountMultiplier),
                "Cost Markup Multiplier - Wholesale Cost" => {
                    Some(CostMarkupMultiplierWholesaleCost)
                }
                "Percent Solution Multiplier" => Some(PercentSolutionMultiplier),
                "Selling Multiplier" => Some(SellingMultiplier),
                _ => None,
            }
        }
    }
}
impl Serialize for PriceMultiplierQualifier {
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
    type Value = PriceMultiplierQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Price Multiplier Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PriceMultiplierQualifier::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Price Multiplier Qualifier: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PriceMultiplierQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Price Multiplier Qualifier: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for PriceMultiplierQualifier {
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