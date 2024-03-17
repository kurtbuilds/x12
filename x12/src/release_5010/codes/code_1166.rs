use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1166

See docs at <https://www.stedi.com/edi/x12-005010/element/1166>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContractTypeCode {
    ///01 - Diagnosis Related Group (DRG)
    Code01,
    ///02 - Per Diem
    PerDiem,
    ///03 - Variable Per Diem
    VariablePerDiem,
    ///04 - Flat
    Flat,
    ///05 - Capitated
    Capitated,
    ///06 - Percent
    Percent,
    ///09 - Other
    Other,
    ///AB - Negotiated Growing Equity Mortgage (GEM)
    CodeAB,
    ///AC - Anticipated Contract
    AnticipatedContract,
    ///AD - Federal Housing Authority Adjustable Rate Mortgage
    FederalHousingAuthorityAdjustableRateMortgage,
    ///AE - Federal Housing Authority Veterans Affairs Fixed Rate Mortgage (including standard Growing Equity Mortgages)
    CodeAE,
    ///AF - Conventional Second Mortgages
    ConventionalSecondMortgages,
    ///AG - Conventional Fixed Rate Mortgages
    ConventionalFixedRateMortgages,
    ///AH - Federal Housing Authority Veterans Affairs Graduated Payment Mortgage
    FederalHousingAuthorityVeteransAffairsGraduatedPaymentMortgage,
    ///AI - Negotiated Conventional, Graduated Payment, or Step Rate Mortgage
    CodeAI,
    ///AJ - Conventional Adjustable Rate Mortgage
    ConventionalAdjustableRateMortgage,
    ///CA - Cost Plus Incentive Fee (With Performance Incentives)
    CodeCA,
    ///CB - Cost Plus Incentive Fee (Without Performance Incentives)
    CodeCB,
    ///CH - Cost Sharing
    CostSharing,
    ///CP - Cost Plus
    CostPlus,
    ///CS - Cost
    Cost,
    ///CW - Cost Plus Award Fee
    CostPlusAwardFee,
    ///CX - Cost Plus Fixed Fee
    CostPlusFixedFee,
    ///CY - Cost Plus Incentive Fee
    CostPlusIncentiveFee,
    ///DI - Distributor
    Distributor,
    ///EA - Exclusive Agency
    ExclusiveAgency,
    ///ER - Exclusive Right
    ExclusiveRight,
    ///FA - Firm or Actual Contract
    FirmOrActualContract,
    ///FB - Fixed Price Incentive Firm Target (With Performance Incentive)
    CodeFB,
    ///FC - Fixed Price Incentive Firm Target (Without Performance Incentive)
    CodeFC,
    ///FD - Fixed Price Redetermination
    FixedPriceRedetermination,
    ///FE - Fixed Price with Escalation
    FixedPriceWithEscalation,
    ///FF - Fixed Price Incentive Successive Target (With Performance Incentive)
    CodeFF,
    ///FG - Fixed Price Incentive Successive Target (Without Performance Incentive)
    CodeFG,
    ///FH - Fixed Price Award Fee
    FixedPriceAwardFee,
    ///FI - Fixed Price Incentive
    FixedPriceIncentive,
    ///FJ - Fixed Price Level of Effort
    FixedPriceLevelOfEffort,
    ///FK - No Cost
    NoCost,
    ///FL - Flat Amount
    FlatAmount,
    ///FM - Retroactive Fixed Price Redetermination
    RetroactiveFixedPriceRedetermination,
    ///FR - Firm Fixed Price
    FirmFixedPrice,
    ///FX - Fixed Price with Economic Price Adjustment
    FixedPriceWithEconomicPriceAdjustment,
    ///LA - Labor
    Labor,
    ///LE - Level of Effort
    LevelOfEffort,
    ///LH - Labor Hours
    LaborHours,
    ///OC - Other Contract Type
    OtherContractType,
    ///PR - Prospect Reservation
    ProspectReservation,
    ///SP - Same Percentage as Film Rental Earned (SPFRE)
    CodeSP,
    ///TM - Time and Materials
    TimeAndMaterials,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl ContractTypeCode {
    pub fn code(&self) -> &str {
        {
            use ContractTypeCode::*;
            match self {
                Code01 => "01",
                PerDiem => "02",
                VariablePerDiem => "03",
                Flat => "04",
                Capitated => "05",
                Percent => "06",
                Other => "09",
                CodeAB => "AB",
                AnticipatedContract => "AC",
                FederalHousingAuthorityAdjustableRateMortgage => "AD",
                CodeAE => "AE",
                ConventionalSecondMortgages => "AF",
                ConventionalFixedRateMortgages => "AG",
                FederalHousingAuthorityVeteransAffairsGraduatedPaymentMortgage => "AH",
                CodeAI => "AI",
                ConventionalAdjustableRateMortgage => "AJ",
                CodeCA => "CA",
                CodeCB => "CB",
                CostSharing => "CH",
                CostPlus => "CP",
                Cost => "CS",
                CostPlusAwardFee => "CW",
                CostPlusFixedFee => "CX",
                CostPlusIncentiveFee => "CY",
                Distributor => "DI",
                ExclusiveAgency => "EA",
                ExclusiveRight => "ER",
                FirmOrActualContract => "FA",
                CodeFB => "FB",
                CodeFC => "FC",
                FixedPriceRedetermination => "FD",
                FixedPriceWithEscalation => "FE",
                CodeFF => "FF",
                CodeFG => "FG",
                FixedPriceAwardFee => "FH",
                FixedPriceIncentive => "FI",
                FixedPriceLevelOfEffort => "FJ",
                NoCost => "FK",
                FlatAmount => "FL",
                RetroactiveFixedPriceRedetermination => "FM",
                FirmFixedPrice => "FR",
                FixedPriceWithEconomicPriceAdjustment => "FX",
                Labor => "LA",
                LevelOfEffort => "LE",
                LaborHours => "LH",
                OtherContractType => "OC",
                ProspectReservation => "PR",
                CodeSP => "SP",
                TimeAndMaterials => "TM",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ContractTypeCode> {
        use ContractTypeCode::*;
        match code {
            b"01" => Some(Code01),
            b"02" => Some(PerDiem),
            b"03" => Some(VariablePerDiem),
            b"04" => Some(Flat),
            b"05" => Some(Capitated),
            b"06" => Some(Percent),
            b"09" => Some(Other),
            b"AB" => Some(CodeAB),
            b"AC" => Some(AnticipatedContract),
            b"AD" => Some(FederalHousingAuthorityAdjustableRateMortgage),
            b"AE" => Some(CodeAE),
            b"AF" => Some(ConventionalSecondMortgages),
            b"AG" => Some(ConventionalFixedRateMortgages),
            b"AH" => Some(FederalHousingAuthorityVeteransAffairsGraduatedPaymentMortgage),
            b"AI" => Some(CodeAI),
            b"AJ" => Some(ConventionalAdjustableRateMortgage),
            b"CA" => Some(CodeCA),
            b"CB" => Some(CodeCB),
            b"CH" => Some(CostSharing),
            b"CP" => Some(CostPlus),
            b"CS" => Some(Cost),
            b"CW" => Some(CostPlusAwardFee),
            b"CX" => Some(CostPlusFixedFee),
            b"CY" => Some(CostPlusIncentiveFee),
            b"DI" => Some(Distributor),
            b"EA" => Some(ExclusiveAgency),
            b"ER" => Some(ExclusiveRight),
            b"FA" => Some(FirmOrActualContract),
            b"FB" => Some(CodeFB),
            b"FC" => Some(CodeFC),
            b"FD" => Some(FixedPriceRedetermination),
            b"FE" => Some(FixedPriceWithEscalation),
            b"FF" => Some(CodeFF),
            b"FG" => Some(CodeFG),
            b"FH" => Some(FixedPriceAwardFee),
            b"FI" => Some(FixedPriceIncentive),
            b"FJ" => Some(FixedPriceLevelOfEffort),
            b"FK" => Some(NoCost),
            b"FL" => Some(FlatAmount),
            b"FM" => Some(RetroactiveFixedPriceRedetermination),
            b"FR" => Some(FirmFixedPrice),
            b"FX" => Some(FixedPriceWithEconomicPriceAdjustment),
            b"LA" => Some(Labor),
            b"LE" => Some(LevelOfEffort),
            b"LH" => Some(LaborHours),
            b"OC" => Some(OtherContractType),
            b"PR" => Some(ProspectReservation),
            b"SP" => Some(CodeSP),
            b"TM" => Some(TimeAndMaterials),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ContractTypeCode::*;
        match self {
            Code01 => "Diagnosis Related Group (DRG)",
            PerDiem => "Per Diem",
            VariablePerDiem => "Variable Per Diem",
            Flat => "Flat",
            Capitated => "Capitated",
            Percent => "Percent",
            Other => "Other",
            CodeAB => "Negotiated Growing Equity Mortgage (GEM)",
            AnticipatedContract => "Anticipated Contract",
            FederalHousingAuthorityAdjustableRateMortgage => {
                "Federal Housing Authority Adjustable Rate Mortgage"
            }
            CodeAE => {
                "Federal Housing Authority Veterans Affairs Fixed Rate Mortgage (including standard Growing Equity Mortgages)"
            }
            ConventionalSecondMortgages => "Conventional Second Mortgages",
            ConventionalFixedRateMortgages => "Conventional Fixed Rate Mortgages",
            FederalHousingAuthorityVeteransAffairsGraduatedPaymentMortgage => {
                "Federal Housing Authority Veterans Affairs Graduated Payment Mortgage"
            }
            CodeAI => "Negotiated Conventional, Graduated Payment, or Step Rate Mortgage",
            ConventionalAdjustableRateMortgage => "Conventional Adjustable Rate Mortgage",
            CodeCA => "Cost Plus Incentive Fee (With Performance Incentives)",
            CodeCB => "Cost Plus Incentive Fee (Without Performance Incentives)",
            CostSharing => "Cost Sharing",
            CostPlus => "Cost Plus",
            Cost => "Cost",
            CostPlusAwardFee => "Cost Plus Award Fee",
            CostPlusFixedFee => "Cost Plus Fixed Fee",
            CostPlusIncentiveFee => "Cost Plus Incentive Fee",
            Distributor => "Distributor",
            ExclusiveAgency => "Exclusive Agency",
            ExclusiveRight => "Exclusive Right",
            FirmOrActualContract => "Firm or Actual Contract",
            CodeFB => "Fixed Price Incentive Firm Target (With Performance Incentive)",
            CodeFC => "Fixed Price Incentive Firm Target (Without Performance Incentive)",
            FixedPriceRedetermination => "Fixed Price Redetermination",
            FixedPriceWithEscalation => "Fixed Price with Escalation",
            CodeFF => {
                "Fixed Price Incentive Successive Target (With Performance Incentive)"
            }
            CodeFG => {
                "Fixed Price Incentive Successive Target (Without Performance Incentive)"
            }
            FixedPriceAwardFee => "Fixed Price Award Fee",
            FixedPriceIncentive => "Fixed Price Incentive",
            FixedPriceLevelOfEffort => "Fixed Price Level of Effort",
            NoCost => "No Cost",
            FlatAmount => "Flat Amount",
            RetroactiveFixedPriceRedetermination => {
                "Retroactive Fixed Price Redetermination"
            }
            FirmFixedPrice => "Firm Fixed Price",
            FixedPriceWithEconomicPriceAdjustment => {
                "Fixed Price with Economic Price Adjustment"
            }
            Labor => "Labor",
            LevelOfEffort => "Level of Effort",
            LaborHours => "Labor Hours",
            OtherContractType => "Other Contract Type",
            ProspectReservation => "Prospect Reservation",
            CodeSP => "Same Percentage as Film Rental Earned (SPFRE)",
            TimeAndMaterials => "Time and Materials",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<ContractTypeCode> {
        {
            use ContractTypeCode::*;
            match description {
                "Diagnosis Related Group (DRG)" => Some(Code01),
                "Per Diem" => Some(PerDiem),
                "Variable Per Diem" => Some(VariablePerDiem),
                "Flat" => Some(Flat),
                "Capitated" => Some(Capitated),
                "Percent" => Some(Percent),
                "Other" => Some(Other),
                "Negotiated Growing Equity Mortgage (GEM)" => Some(CodeAB),
                "Anticipated Contract" => Some(AnticipatedContract),
                "Federal Housing Authority Adjustable Rate Mortgage" => {
                    Some(FederalHousingAuthorityAdjustableRateMortgage)
                }
                "Federal Housing Authority Veterans Affairs Fixed Rate Mortgage (including standard Growing Equity Mortgages)" => {
                    Some(CodeAE)
                }
                "Conventional Second Mortgages" => Some(ConventionalSecondMortgages),
                "Conventional Fixed Rate Mortgages" => {
                    Some(ConventionalFixedRateMortgages)
                }
                "Federal Housing Authority Veterans Affairs Graduated Payment Mortgage" => {
                    Some(FederalHousingAuthorityVeteransAffairsGraduatedPaymentMortgage)
                }
                "Negotiated Conventional, Graduated Payment, or Step Rate Mortgage" => {
                    Some(CodeAI)
                }
                "Conventional Adjustable Rate Mortgage" => {
                    Some(ConventionalAdjustableRateMortgage)
                }
                "Cost Plus Incentive Fee (With Performance Incentives)" => Some(CodeCA),
                "Cost Plus Incentive Fee (Without Performance Incentives)" => {
                    Some(CodeCB)
                }
                "Cost Sharing" => Some(CostSharing),
                "Cost Plus" => Some(CostPlus),
                "Cost" => Some(Cost),
                "Cost Plus Award Fee" => Some(CostPlusAwardFee),
                "Cost Plus Fixed Fee" => Some(CostPlusFixedFee),
                "Cost Plus Incentive Fee" => Some(CostPlusIncentiveFee),
                "Distributor" => Some(Distributor),
                "Exclusive Agency" => Some(ExclusiveAgency),
                "Exclusive Right" => Some(ExclusiveRight),
                "Firm or Actual Contract" => Some(FirmOrActualContract),
                "Fixed Price Incentive Firm Target (With Performance Incentive)" => {
                    Some(CodeFB)
                }
                "Fixed Price Incentive Firm Target (Without Performance Incentive)" => {
                    Some(CodeFC)
                }
                "Fixed Price Redetermination" => Some(FixedPriceRedetermination),
                "Fixed Price with Escalation" => Some(FixedPriceWithEscalation),
                "Fixed Price Incentive Successive Target (With Performance Incentive)" => {
                    Some(CodeFF)
                }
                "Fixed Price Incentive Successive Target (Without Performance Incentive)" => {
                    Some(CodeFG)
                }
                "Fixed Price Award Fee" => Some(FixedPriceAwardFee),
                "Fixed Price Incentive" => Some(FixedPriceIncentive),
                "Fixed Price Level of Effort" => Some(FixedPriceLevelOfEffort),
                "No Cost" => Some(NoCost),
                "Flat Amount" => Some(FlatAmount),
                "Retroactive Fixed Price Redetermination" => {
                    Some(RetroactiveFixedPriceRedetermination)
                }
                "Firm Fixed Price" => Some(FirmFixedPrice),
                "Fixed Price with Economic Price Adjustment" => {
                    Some(FixedPriceWithEconomicPriceAdjustment)
                }
                "Labor" => Some(Labor),
                "Level of Effort" => Some(LevelOfEffort),
                "Labor Hours" => Some(LaborHours),
                "Other Contract Type" => Some(OtherContractType),
                "Prospect Reservation" => Some(ProspectReservation),
                "Same Percentage as Film Rental Earned (SPFRE)" => Some(CodeSP),
                "Time and Materials" => Some(TimeAndMaterials),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for ContractTypeCode {
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
    type Value = ContractTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Contract Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ContractTypeCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Contract Type Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ContractTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Contract Type Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ContractTypeCode {
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