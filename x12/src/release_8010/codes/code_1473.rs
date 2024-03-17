use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1473

See docs at <https://www.stedi.com/edi/x12/element/1473>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PricingMethodologyCode {
    ///00 - Zero Pricing (Not Covered Under Contract)
    Code00,
    ///01 - Priced as Billed at 100%
    Code01,
    ///02 - Priced at the Standard Fee Schedule
    PricedAtTheStandardFeeSchedule,
    ///03 - Priced at a Contractual Percentage
    PricedAtAContractualPercentage,
    ///04 - Bundled Pricing
    BundledPricing,
    ///05 - Peer Review Pricing
    PeerReviewPricing,
    ///06 - Per Diem Pricing
    PerDiemPricing,
    ///07 - Flat Rate Pricing
    FlatRatePricing,
    ///08 - Combination Pricing
    CombinationPricing,
    ///09 - Maternity Pricing
    MaternityPricing,
    ///10 - Other Pricing
    OtherPricing,
    ///11 - Lower of Cost
    LowerOfCost,
    ///12 - Ratio of Cost
    RatioOfCost,
    ///13 - Cost Reimbursed
    CostReimbursed,
    ///14 - Adjustment Pricing
    AdjustmentPricing,
}
impl PricingMethodologyCode {
    pub fn code(&self) -> &str {
        {
            use PricingMethodologyCode::*;
            match self {
                Code00 => "00",
                Code01 => "01",
                PricedAtTheStandardFeeSchedule => "02",
                PricedAtAContractualPercentage => "03",
                BundledPricing => "04",
                PeerReviewPricing => "05",
                PerDiemPricing => "06",
                FlatRatePricing => "07",
                CombinationPricing => "08",
                MaternityPricing => "09",
                OtherPricing => "10",
                LowerOfCost => "11",
                RatioOfCost => "12",
                CostReimbursed => "13",
                AdjustmentPricing => "14",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<PricingMethodologyCode> {
        use PricingMethodologyCode::*;
        match code {
            b"00" => Some(Code00),
            b"01" => Some(Code01),
            b"02" => Some(PricedAtTheStandardFeeSchedule),
            b"03" => Some(PricedAtAContractualPercentage),
            b"04" => Some(BundledPricing),
            b"05" => Some(PeerReviewPricing),
            b"06" => Some(PerDiemPricing),
            b"07" => Some(FlatRatePricing),
            b"08" => Some(CombinationPricing),
            b"09" => Some(MaternityPricing),
            b"10" => Some(OtherPricing),
            b"11" => Some(LowerOfCost),
            b"12" => Some(RatioOfCost),
            b"13" => Some(CostReimbursed),
            b"14" => Some(AdjustmentPricing),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use PricingMethodologyCode::*;
        match self {
            Code00 => "Zero Pricing (Not Covered Under Contract)",
            Code01 => "Priced as Billed at 100%",
            PricedAtTheStandardFeeSchedule => "Priced at the Standard Fee Schedule",
            PricedAtAContractualPercentage => "Priced at a Contractual Percentage",
            BundledPricing => "Bundled Pricing",
            PeerReviewPricing => "Peer Review Pricing",
            PerDiemPricing => "Per Diem Pricing",
            FlatRatePricing => "Flat Rate Pricing",
            CombinationPricing => "Combination Pricing",
            MaternityPricing => "Maternity Pricing",
            OtherPricing => "Other Pricing",
            LowerOfCost => "Lower of Cost",
            RatioOfCost => "Ratio of Cost",
            CostReimbursed => "Cost Reimbursed",
            AdjustmentPricing => "Adjustment Pricing",
        }
    }
    fn from_description(description: &str) -> Option<PricingMethodologyCode> {
        {
            use PricingMethodologyCode::*;
            match description {
                "Zero Pricing (Not Covered Under Contract)" => Some(Code00),
                "Priced as Billed at 100%" => Some(Code01),
                "Priced at the Standard Fee Schedule" => {
                    Some(PricedAtTheStandardFeeSchedule)
                }
                "Priced at a Contractual Percentage" => {
                    Some(PricedAtAContractualPercentage)
                }
                "Bundled Pricing" => Some(BundledPricing),
                "Peer Review Pricing" => Some(PeerReviewPricing),
                "Per Diem Pricing" => Some(PerDiemPricing),
                "Flat Rate Pricing" => Some(FlatRatePricing),
                "Combination Pricing" => Some(CombinationPricing),
                "Maternity Pricing" => Some(MaternityPricing),
                "Other Pricing" => Some(OtherPricing),
                "Lower of Cost" => Some(LowerOfCost),
                "Ratio of Cost" => Some(RatioOfCost),
                "Cost Reimbursed" => Some(CostReimbursed),
                "Adjustment Pricing" => Some(AdjustmentPricing),
                _ => None,
            }
        }
    }
}
impl Serialize for PricingMethodologyCode {
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
    type Value = PricingMethodologyCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Pricing Methodology Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PricingMethodologyCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Pricing Methodology Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PricingMethodologyCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Pricing Methodology Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for PricingMethodologyCode {
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