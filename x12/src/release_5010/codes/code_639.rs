use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**639

See docs at <https://www.stedi.com/edi/x12/element/639>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BasisOfUnitPriceCode {
    ///AA - Bill
    Bill,
    ///AB - Pay
    Pay,
    ///AP - Advise Price
    AdvisePrice,
    ///AW - Average Wholesale Price
    AverageWholesalePrice,
    ///BD - Before Discount
    BeforeDiscount,
    ///BR - Broker
    Broker,
    ///BW - Biweekly Price per Unit
    BiweeklyPricePerUnit,
    ///CA - Catalog
    Catalog,
    ///CP - Current Price (Subject to Change)
    CodeCP,
    ///CR - Carnet
    Carnet,
    ///CT - Contract
    Contract,
    ///DI - Distributor
    Distributor,
    ///DP - Daily Price per Unit
    DailyPricePerUnit,
    ///DR - Dealer
    Dealer,
    ///DS - Discount
    Discount,
    ///EC - Estimated Credit
    EstimatedCredit,
    ///EH - Shift Differential
    ShiftDifferential,
    ///ES - Estimated
    Estimated,
    ///FB - Fabrication Cost
    FabricationCost,
    ///FO - Formula
    Formula,
    ///FX - Fixed Price
    FixedPrice,
    ///HF - Per 100 Feet
    Per100Feet,
    ///HP - Price per Hundred
    PricePerHundred,
    ///HT - Price Per 100,000
    CodeHT,
    ///KA - Price with Government Furnished Property
    PriceWithGovernmentFurnishedProperty,
    ///KP - Escalated Price
    EscalatedPrice,
    ///KR - In Stock
    InStock,
    ///LC - Catalog Price per Hundred
    CatalogPricePerHundred,
    ///LD - Catalog Price per Dozen
    CatalogPricePerDozen,
    ///LE - Catalog Price per Each
    CatalogPricePerEach,
    ///LM - Catalog Price per Thousand
    CatalogPricePerThousand,
    ///LR - Previous Catalog Price
    PreviousCatalogPrice,
    ///ME - Midterm Endorsement Price per Unit
    MidtermEndorsementPricePerUnit,
    ///ML - Price per Milliliter
    PricePerMilliliter,
    ///NC - No Charge
    NoCharge,
    ///NE - Not to Exceed
    NotToExceed,
    ///NQ - No Quote
    NoQuote,
    ///NS - Not Separately Priced
    NotSeparatelyPriced,
    ///NT - Net
    Net,
    ///PA - Price per Troy Ounce
    PricePerTroyOunce,
    ///PB - Annual Price Per Unit
    AnnualPricePerUnit,
    ///PD - Price per Dozen
    PricePerDozen,
    ///PE - Price per Each
    PricePerEach,
    ///PF - Price Per Foot
    PricePerFoot,
    ///PG - Price per Gram
    PricePerGram,
    ///PK - Price per Kilogram
    PricePerKilogram,
    ///PL - Price per Liter
    PricePerLiter,
    ///PM - Monthly Price Per Unit
    MonthlyPricePerUnit,
    ///PN - Price per Ten
    PricePerTen,
    ///PO - Price per Ounce
    PricePerOunce,
    ///PP - Price per Pound
    PricePerPound,
    ///PQ - Posted
    Posted,
    ///PR - Promotion
    Promotion,
    ///PS - Price Per Thousand Square Foot
    PricePerThousandSquareFoot,
    ///PT - Price per Ton
    PricePerTon,
    ///PU - Quarterly Price per Unit
    QuarterlyPricePerUnit,
    ///PV - Provisional Price
    ProvisionalPrice,
    ///PY - Price per Yard
    PricePerYard,
    ///QE - Quoted Price per Each
    QuotedPricePerEach,
    ///QH - Quoted Price per Hundred
    QuotedPricePerHundred,
    ///QR - Previous Quoted Price
    PreviousQuotedPrice,
    ///QS - Quoted Price per Thousand
    QuotedPricePerThousand,
    ///QT - Quoted
    Quoted,
    ///RC - Retail Price per Hundred
    RetailPricePerHundred,
    ///RD - Retail Price per Dozen
    RetailPricePerDozen,
    ///RE - Retail Price per Each
    RetailPricePerEach,
    ///RM - Retail Price per Thousand
    RetailPricePerThousand,
    ///RS - Resale Price
    ResalePrice,
    ///RT - Retail
    Retail,
    ///SA - Semi Annual Price per Unit
    SemiAnnualPricePerUnit,
    ///SC - Submitted Contract
    SubmittedContract,
    ///SM - Semi Monthly Price per Unit
    SemiMonthlyPricePerUnit,
    ///SR - Suggested Retail
    SuggestedRetail,
    ///ST - Standard
    Standard,
    ///SW - Submitted Wholesale
    SubmittedWholesale,
    ///TB - To be negotiated.
    ToBeNegotiated,
    ///TC - Contract Price per Hundred
    ContractPricePerHundred,
    ///TD - Contract Price per Dozen
    ContractPricePerDozen,
    ///TE - Contract Price per Each
    ContractPricePerEach,
    ///TF - Per 1000 Feet
    Per1000Feet,
    ///TM - Contract Price per Thousand
    ContractPricePerThousand,
    ///TP - Price per Thousand
    PricePerThousand,
    ///TT - Price Per 10,000
    CodeTT,
    ///UM - Price per Unit of Measure
    PricePerUnitOfMeasure,
    ///VQ - Verbal Quote
    VerbalQuote,
    ///WC - Wholesale Price per Hundred
    WholesalePricePerHundred,
    ///WD - Wholesale Price per Dozen
    WholesalePricePerDozen,
    ///WE - Wholesale Price per Each
    WholesalePricePerEach,
    ///WH - Wholesale
    Wholesale,
    ///WI - Weekly Price per Unit
    WeeklyPricePerUnit,
    ///WM - Wholesale Price per Thousand
    WholesalePricePerThousand,
}
impl BasisOfUnitPriceCode {
    pub fn code(&self) -> &str {
        {
            use BasisOfUnitPriceCode::*;
            match self {
                Bill => "AA",
                Pay => "AB",
                AdvisePrice => "AP",
                AverageWholesalePrice => "AW",
                BeforeDiscount => "BD",
                Broker => "BR",
                BiweeklyPricePerUnit => "BW",
                Catalog => "CA",
                CodeCP => "CP",
                Carnet => "CR",
                Contract => "CT",
                Distributor => "DI",
                DailyPricePerUnit => "DP",
                Dealer => "DR",
                Discount => "DS",
                EstimatedCredit => "EC",
                ShiftDifferential => "EH",
                Estimated => "ES",
                FabricationCost => "FB",
                Formula => "FO",
                FixedPrice => "FX",
                Per100Feet => "HF",
                PricePerHundred => "HP",
                CodeHT => "HT",
                PriceWithGovernmentFurnishedProperty => "KA",
                EscalatedPrice => "KP",
                InStock => "KR",
                CatalogPricePerHundred => "LC",
                CatalogPricePerDozen => "LD",
                CatalogPricePerEach => "LE",
                CatalogPricePerThousand => "LM",
                PreviousCatalogPrice => "LR",
                MidtermEndorsementPricePerUnit => "ME",
                PricePerMilliliter => "ML",
                NoCharge => "NC",
                NotToExceed => "NE",
                NoQuote => "NQ",
                NotSeparatelyPriced => "NS",
                Net => "NT",
                PricePerTroyOunce => "PA",
                AnnualPricePerUnit => "PB",
                PricePerDozen => "PD",
                PricePerEach => "PE",
                PricePerFoot => "PF",
                PricePerGram => "PG",
                PricePerKilogram => "PK",
                PricePerLiter => "PL",
                MonthlyPricePerUnit => "PM",
                PricePerTen => "PN",
                PricePerOunce => "PO",
                PricePerPound => "PP",
                Posted => "PQ",
                Promotion => "PR",
                PricePerThousandSquareFoot => "PS",
                PricePerTon => "PT",
                QuarterlyPricePerUnit => "PU",
                ProvisionalPrice => "PV",
                PricePerYard => "PY",
                QuotedPricePerEach => "QE",
                QuotedPricePerHundred => "QH",
                PreviousQuotedPrice => "QR",
                QuotedPricePerThousand => "QS",
                Quoted => "QT",
                RetailPricePerHundred => "RC",
                RetailPricePerDozen => "RD",
                RetailPricePerEach => "RE",
                RetailPricePerThousand => "RM",
                ResalePrice => "RS",
                Retail => "RT",
                SemiAnnualPricePerUnit => "SA",
                SubmittedContract => "SC",
                SemiMonthlyPricePerUnit => "SM",
                SuggestedRetail => "SR",
                Standard => "ST",
                SubmittedWholesale => "SW",
                ToBeNegotiated => "TB",
                ContractPricePerHundred => "TC",
                ContractPricePerDozen => "TD",
                ContractPricePerEach => "TE",
                Per1000Feet => "TF",
                ContractPricePerThousand => "TM",
                PricePerThousand => "TP",
                CodeTT => "TT",
                PricePerUnitOfMeasure => "UM",
                VerbalQuote => "VQ",
                WholesalePricePerHundred => "WC",
                WholesalePricePerDozen => "WD",
                WholesalePricePerEach => "WE",
                Wholesale => "WH",
                WeeklyPricePerUnit => "WI",
                WholesalePricePerThousand => "WM",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<BasisOfUnitPriceCode> {
        use BasisOfUnitPriceCode::*;
        match code {
            b"AA" => Some(Bill),
            b"AB" => Some(Pay),
            b"AP" => Some(AdvisePrice),
            b"AW" => Some(AverageWholesalePrice),
            b"BD" => Some(BeforeDiscount),
            b"BR" => Some(Broker),
            b"BW" => Some(BiweeklyPricePerUnit),
            b"CA" => Some(Catalog),
            b"CP" => Some(CodeCP),
            b"CR" => Some(Carnet),
            b"CT" => Some(Contract),
            b"DI" => Some(Distributor),
            b"DP" => Some(DailyPricePerUnit),
            b"DR" => Some(Dealer),
            b"DS" => Some(Discount),
            b"EC" => Some(EstimatedCredit),
            b"EH" => Some(ShiftDifferential),
            b"ES" => Some(Estimated),
            b"FB" => Some(FabricationCost),
            b"FO" => Some(Formula),
            b"FX" => Some(FixedPrice),
            b"HF" => Some(Per100Feet),
            b"HP" => Some(PricePerHundred),
            b"HT" => Some(CodeHT),
            b"KA" => Some(PriceWithGovernmentFurnishedProperty),
            b"KP" => Some(EscalatedPrice),
            b"KR" => Some(InStock),
            b"LC" => Some(CatalogPricePerHundred),
            b"LD" => Some(CatalogPricePerDozen),
            b"LE" => Some(CatalogPricePerEach),
            b"LM" => Some(CatalogPricePerThousand),
            b"LR" => Some(PreviousCatalogPrice),
            b"ME" => Some(MidtermEndorsementPricePerUnit),
            b"ML" => Some(PricePerMilliliter),
            b"NC" => Some(NoCharge),
            b"NE" => Some(NotToExceed),
            b"NQ" => Some(NoQuote),
            b"NS" => Some(NotSeparatelyPriced),
            b"NT" => Some(Net),
            b"PA" => Some(PricePerTroyOunce),
            b"PB" => Some(AnnualPricePerUnit),
            b"PD" => Some(PricePerDozen),
            b"PE" => Some(PricePerEach),
            b"PF" => Some(PricePerFoot),
            b"PG" => Some(PricePerGram),
            b"PK" => Some(PricePerKilogram),
            b"PL" => Some(PricePerLiter),
            b"PM" => Some(MonthlyPricePerUnit),
            b"PN" => Some(PricePerTen),
            b"PO" => Some(PricePerOunce),
            b"PP" => Some(PricePerPound),
            b"PQ" => Some(Posted),
            b"PR" => Some(Promotion),
            b"PS" => Some(PricePerThousandSquareFoot),
            b"PT" => Some(PricePerTon),
            b"PU" => Some(QuarterlyPricePerUnit),
            b"PV" => Some(ProvisionalPrice),
            b"PY" => Some(PricePerYard),
            b"QE" => Some(QuotedPricePerEach),
            b"QH" => Some(QuotedPricePerHundred),
            b"QR" => Some(PreviousQuotedPrice),
            b"QS" => Some(QuotedPricePerThousand),
            b"QT" => Some(Quoted),
            b"RC" => Some(RetailPricePerHundred),
            b"RD" => Some(RetailPricePerDozen),
            b"RE" => Some(RetailPricePerEach),
            b"RM" => Some(RetailPricePerThousand),
            b"RS" => Some(ResalePrice),
            b"RT" => Some(Retail),
            b"SA" => Some(SemiAnnualPricePerUnit),
            b"SC" => Some(SubmittedContract),
            b"SM" => Some(SemiMonthlyPricePerUnit),
            b"SR" => Some(SuggestedRetail),
            b"ST" => Some(Standard),
            b"SW" => Some(SubmittedWholesale),
            b"TB" => Some(ToBeNegotiated),
            b"TC" => Some(ContractPricePerHundred),
            b"TD" => Some(ContractPricePerDozen),
            b"TE" => Some(ContractPricePerEach),
            b"TF" => Some(Per1000Feet),
            b"TM" => Some(ContractPricePerThousand),
            b"TP" => Some(PricePerThousand),
            b"TT" => Some(CodeTT),
            b"UM" => Some(PricePerUnitOfMeasure),
            b"VQ" => Some(VerbalQuote),
            b"WC" => Some(WholesalePricePerHundred),
            b"WD" => Some(WholesalePricePerDozen),
            b"WE" => Some(WholesalePricePerEach),
            b"WH" => Some(Wholesale),
            b"WI" => Some(WeeklyPricePerUnit),
            b"WM" => Some(WholesalePricePerThousand),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use BasisOfUnitPriceCode::*;
        match self {
            Bill => "Bill",
            Pay => "Pay",
            AdvisePrice => "Advise Price",
            AverageWholesalePrice => "Average Wholesale Price",
            BeforeDiscount => "Before Discount",
            Broker => "Broker",
            BiweeklyPricePerUnit => "Biweekly Price per Unit",
            Catalog => "Catalog",
            CodeCP => "Current Price (Subject to Change)",
            Carnet => "Carnet",
            Contract => "Contract",
            Distributor => "Distributor",
            DailyPricePerUnit => "Daily Price per Unit",
            Dealer => "Dealer",
            Discount => "Discount",
            EstimatedCredit => "Estimated Credit",
            ShiftDifferential => "Shift Differential",
            Estimated => "Estimated",
            FabricationCost => "Fabrication Cost",
            Formula => "Formula",
            FixedPrice => "Fixed Price",
            Per100Feet => "Per 100 Feet",
            PricePerHundred => "Price per Hundred",
            CodeHT => "Price Per 100,000",
            PriceWithGovernmentFurnishedProperty => {
                "Price with Government Furnished Property"
            }
            EscalatedPrice => "Escalated Price",
            InStock => "In Stock",
            CatalogPricePerHundred => "Catalog Price per Hundred",
            CatalogPricePerDozen => "Catalog Price per Dozen",
            CatalogPricePerEach => "Catalog Price per Each",
            CatalogPricePerThousand => "Catalog Price per Thousand",
            PreviousCatalogPrice => "Previous Catalog Price",
            MidtermEndorsementPricePerUnit => "Midterm Endorsement Price per Unit",
            PricePerMilliliter => "Price per Milliliter",
            NoCharge => "No Charge",
            NotToExceed => "Not to Exceed",
            NoQuote => "No Quote",
            NotSeparatelyPriced => "Not Separately Priced",
            Net => "Net",
            PricePerTroyOunce => "Price per Troy Ounce",
            AnnualPricePerUnit => "Annual Price Per Unit",
            PricePerDozen => "Price per Dozen",
            PricePerEach => "Price per Each",
            PricePerFoot => "Price Per Foot",
            PricePerGram => "Price per Gram",
            PricePerKilogram => "Price per Kilogram",
            PricePerLiter => "Price per Liter",
            MonthlyPricePerUnit => "Monthly Price Per Unit",
            PricePerTen => "Price per Ten",
            PricePerOunce => "Price per Ounce",
            PricePerPound => "Price per Pound",
            Posted => "Posted",
            Promotion => "Promotion",
            PricePerThousandSquareFoot => "Price Per Thousand Square Foot",
            PricePerTon => "Price per Ton",
            QuarterlyPricePerUnit => "Quarterly Price per Unit",
            ProvisionalPrice => "Provisional Price",
            PricePerYard => "Price per Yard",
            QuotedPricePerEach => "Quoted Price per Each",
            QuotedPricePerHundred => "Quoted Price per Hundred",
            PreviousQuotedPrice => "Previous Quoted Price",
            QuotedPricePerThousand => "Quoted Price per Thousand",
            Quoted => "Quoted",
            RetailPricePerHundred => "Retail Price per Hundred",
            RetailPricePerDozen => "Retail Price per Dozen",
            RetailPricePerEach => "Retail Price per Each",
            RetailPricePerThousand => "Retail Price per Thousand",
            ResalePrice => "Resale Price",
            Retail => "Retail",
            SemiAnnualPricePerUnit => "Semi Annual Price per Unit",
            SubmittedContract => "Submitted Contract",
            SemiMonthlyPricePerUnit => "Semi Monthly Price per Unit",
            SuggestedRetail => "Suggested Retail",
            Standard => "Standard",
            SubmittedWholesale => "Submitted Wholesale",
            ToBeNegotiated => "To be negotiated.",
            ContractPricePerHundred => "Contract Price per Hundred",
            ContractPricePerDozen => "Contract Price per Dozen",
            ContractPricePerEach => "Contract Price per Each",
            Per1000Feet => "Per 1000 Feet",
            ContractPricePerThousand => "Contract Price per Thousand",
            PricePerThousand => "Price per Thousand",
            CodeTT => "Price Per 10,000",
            PricePerUnitOfMeasure => "Price per Unit of Measure",
            VerbalQuote => "Verbal Quote",
            WholesalePricePerHundred => "Wholesale Price per Hundred",
            WholesalePricePerDozen => "Wholesale Price per Dozen",
            WholesalePricePerEach => "Wholesale Price per Each",
            Wholesale => "Wholesale",
            WeeklyPricePerUnit => "Weekly Price per Unit",
            WholesalePricePerThousand => "Wholesale Price per Thousand",
        }
    }
    fn from_description(description: &str) -> Option<BasisOfUnitPriceCode> {
        {
            use BasisOfUnitPriceCode::*;
            match description {
                "Bill" => Some(Bill),
                "Pay" => Some(Pay),
                "Advise Price" => Some(AdvisePrice),
                "Average Wholesale Price" => Some(AverageWholesalePrice),
                "Before Discount" => Some(BeforeDiscount),
                "Broker" => Some(Broker),
                "Biweekly Price per Unit" => Some(BiweeklyPricePerUnit),
                "Catalog" => Some(Catalog),
                "Current Price (Subject to Change)" => Some(CodeCP),
                "Carnet" => Some(Carnet),
                "Contract" => Some(Contract),
                "Distributor" => Some(Distributor),
                "Daily Price per Unit" => Some(DailyPricePerUnit),
                "Dealer" => Some(Dealer),
                "Discount" => Some(Discount),
                "Estimated Credit" => Some(EstimatedCredit),
                "Shift Differential" => Some(ShiftDifferential),
                "Estimated" => Some(Estimated),
                "Fabrication Cost" => Some(FabricationCost),
                "Formula" => Some(Formula),
                "Fixed Price" => Some(FixedPrice),
                "Per 100 Feet" => Some(Per100Feet),
                "Price per Hundred" => Some(PricePerHundred),
                "Price Per 100,000" => Some(CodeHT),
                "Price with Government Furnished Property" => {
                    Some(PriceWithGovernmentFurnishedProperty)
                }
                "Escalated Price" => Some(EscalatedPrice),
                "In Stock" => Some(InStock),
                "Catalog Price per Hundred" => Some(CatalogPricePerHundred),
                "Catalog Price per Dozen" => Some(CatalogPricePerDozen),
                "Catalog Price per Each" => Some(CatalogPricePerEach),
                "Catalog Price per Thousand" => Some(CatalogPricePerThousand),
                "Previous Catalog Price" => Some(PreviousCatalogPrice),
                "Midterm Endorsement Price per Unit" => {
                    Some(MidtermEndorsementPricePerUnit)
                }
                "Price per Milliliter" => Some(PricePerMilliliter),
                "No Charge" => Some(NoCharge),
                "Not to Exceed" => Some(NotToExceed),
                "No Quote" => Some(NoQuote),
                "Not Separately Priced" => Some(NotSeparatelyPriced),
                "Net" => Some(Net),
                "Price per Troy Ounce" => Some(PricePerTroyOunce),
                "Annual Price Per Unit" => Some(AnnualPricePerUnit),
                "Price per Dozen" => Some(PricePerDozen),
                "Price per Each" => Some(PricePerEach),
                "Price Per Foot" => Some(PricePerFoot),
                "Price per Gram" => Some(PricePerGram),
                "Price per Kilogram" => Some(PricePerKilogram),
                "Price per Liter" => Some(PricePerLiter),
                "Monthly Price Per Unit" => Some(MonthlyPricePerUnit),
                "Price per Ten" => Some(PricePerTen),
                "Price per Ounce" => Some(PricePerOunce),
                "Price per Pound" => Some(PricePerPound),
                "Posted" => Some(Posted),
                "Promotion" => Some(Promotion),
                "Price Per Thousand Square Foot" => Some(PricePerThousandSquareFoot),
                "Price per Ton" => Some(PricePerTon),
                "Quarterly Price per Unit" => Some(QuarterlyPricePerUnit),
                "Provisional Price" => Some(ProvisionalPrice),
                "Price per Yard" => Some(PricePerYard),
                "Quoted Price per Each" => Some(QuotedPricePerEach),
                "Quoted Price per Hundred" => Some(QuotedPricePerHundred),
                "Previous Quoted Price" => Some(PreviousQuotedPrice),
                "Quoted Price per Thousand" => Some(QuotedPricePerThousand),
                "Quoted" => Some(Quoted),
                "Retail Price per Hundred" => Some(RetailPricePerHundred),
                "Retail Price per Dozen" => Some(RetailPricePerDozen),
                "Retail Price per Each" => Some(RetailPricePerEach),
                "Retail Price per Thousand" => Some(RetailPricePerThousand),
                "Resale Price" => Some(ResalePrice),
                "Retail" => Some(Retail),
                "Semi Annual Price per Unit" => Some(SemiAnnualPricePerUnit),
                "Submitted Contract" => Some(SubmittedContract),
                "Semi Monthly Price per Unit" => Some(SemiMonthlyPricePerUnit),
                "Suggested Retail" => Some(SuggestedRetail),
                "Standard" => Some(Standard),
                "Submitted Wholesale" => Some(SubmittedWholesale),
                "To be negotiated." => Some(ToBeNegotiated),
                "Contract Price per Hundred" => Some(ContractPricePerHundred),
                "Contract Price per Dozen" => Some(ContractPricePerDozen),
                "Contract Price per Each" => Some(ContractPricePerEach),
                "Per 1000 Feet" => Some(Per1000Feet),
                "Contract Price per Thousand" => Some(ContractPricePerThousand),
                "Price per Thousand" => Some(PricePerThousand),
                "Price Per 10,000" => Some(CodeTT),
                "Price per Unit of Measure" => Some(PricePerUnitOfMeasure),
                "Verbal Quote" => Some(VerbalQuote),
                "Wholesale Price per Hundred" => Some(WholesalePricePerHundred),
                "Wholesale Price per Dozen" => Some(WholesalePricePerDozen),
                "Wholesale Price per Each" => Some(WholesalePricePerEach),
                "Wholesale" => Some(Wholesale),
                "Weekly Price per Unit" => Some(WeeklyPricePerUnit),
                "Wholesale Price per Thousand" => Some(WholesalePricePerThousand),
                _ => None,
            }
        }
    }
}
impl Serialize for BasisOfUnitPriceCode {
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
    type Value = BasisOfUnitPriceCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Basis of Unit Price Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        BasisOfUnitPriceCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Basis of Unit Price Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        BasisOfUnitPriceCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Basis of Unit Price Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for BasisOfUnitPriceCode {
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