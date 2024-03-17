use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**236

See docs at <https://www.stedi.com/edi/x12/element/236>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PriceIdentifierCode {
    ///ACT - Actual
    Actual,
    ///AGC - Average Generic Product Price
    AverageGenericProductPrice,
    ///ALT - Alternate Price
    AlternatePrice,
    ///AWP - Average Wholesale Price
    AverageWholesalePrice,
    ///BBP - Balance-Based Price
    BalanceBasedPrice,
    ///BCH - Base Charge
    BaseCharge,
    ///BID - Bid Price
    BidPrice,
    ///C01 - Contract Tier 1
    ContractTier1,
    ///C02 - Contract Tier 2
    ContractTier2,
    ///C03 - Contract Tier 3
    ContractTier3,
    ///C04 - Contract Tier 4
    ContractTier4,
    ///C05 - Contract Tier 5
    ContractTier5,
    ///C06 - Contract Tier 6
    ContractTier6,
    ///C07 - Contract Tier 7
    ContractTier7,
    ///C08 - Contract Tier 8
    ContractTier8,
    ///C09 - Contract Tier 9
    ContractTier9,
    ///C10 - Contract Tier 10
    ContractTier10,
    ///C11 - Contract Tier 11
    ContractTier11,
    ///C12 - Contract Tier 12
    ContractTier12,
    ///C13 - Contract Tier 13
    ContractTier13,
    ///C14 - Contract Tier 14
    ContractTier14,
    ///C15 - Contract Tier 15
    ContractTier15,
    ///C16 - Contract Tier 16
    ContractTier16,
    ///C17 - Contract Tier 17
    ContractTier17,
    ///C18 - Contract Tier 18
    ContractTier18,
    ///C19 - Contract Tier 19
    ContractTier19,
    ///C20 - Contract Tier 20
    ContractTier20,
    ///C21 - Contract Tier 21
    ContractTier21,
    ///C22 - Contract Tier 22
    ContractTier22,
    ///C23 - Contract Tier 23
    ContractTier23,
    ///C24 - Contract Tier 24
    ContractTier24,
    ///C25 - Contract Tier 25
    ContractTier25,
    ///C26 - Contract Tier 26
    ContractTier26,
    ///C27 - Contract Tier 27
    ContractTier27,
    ///C28 - Contract Tier 28
    ContractTier28,
    ///C29 - Contract Tier 29
    ContractTier29,
    ///C30 - Contract Tier 30
    ContractTier30,
    ///CAN - Cancellation Charge
    CancellationCharge,
    ///CAT - Catalog Price
    CatalogPrice,
    ///CDF - Central Distribution Facility (Warehouse)
    CodeCDF,
    ///CDV - Current Domestic Value
    CurrentDomesticValue,
    ///CHG - Changed Price
    ChangedPrice,
    ///CON - Contract Price
    ContractPrice,
    ///CUP - Confirmed Unit Price
    ConfirmedUnitPrice,
    ///CUS - Declared Customs Unit Value
    DeclaredCustomsUnitValue,
    ///D01 - Federal Supply Schedule (FSS) Price
    CodeD01,
    ///D02 - Depot Price
    DepotPrice,
    ///D03 - Distribution and Pricing Agreement (DAPA) Price
    CodeD03,
    ///DAP - Dealer Adjusted Price
    DealerAdjustedPrice,
    ///DIS - Distributor's Price
    DistributorsPrice,
    ///DPR - Discount Price
    DiscountPrice,
    ///DSC - Discount Amount Allowed
    DiscountAmountAllowed,
    ///DSD - Direct Store Delivery
    DirectStoreDelivery,
    ///DSP - Direct Ship Program Price
    DirectShipProgramPrice,
    ///EDM - Emergency Direct Ship Price (Original Equipment Manufacturer)
    CodeEDM,
    ///EDP - Emergency Direct Ship Price
    EmergencyDirectShipPrice,
    ///EDS - Emergency Direct Ship Price (Supplier)
    CodeEDS,
    ///EDW - Emergency Direct Ship Price (Warehouse)
    CodeEDW,
    ///ELC - Estimated Landed Cost
    EstimatedLandedCost,
    ///EST - Estimated Price
    EstimatedPrice,
    ///EUP - Expected Unit Price
    ExpectedUnitPrice,
    ///FCH - Flat Charge
    FlatCharge,
    ///FCP - First Cost Price
    FirstCostPrice,
    ///FDS - Frequent Delivery Service
    FrequentDeliveryService,
    ///FET - Federal Excise Tax
    FederalExciseTax,
    ///FGP - Free Goods Price
    FreeGoodsPrice,
    ///FOR - Formula Price
    FormulaPrice,
    ///FSP - Free Service Price
    FreeServicePrice,
    ///FUL - Federal Upper Limit Price (Maximum Allowable Cost Pricing for Drugs)
    CodeFUL,
    ///FUP - Firm Price - Do Not Advise
    FirmPriceDoNotAdvise,
    ///GAP - Advertising Price
    AdvertisingPrice,
    ///GDP - Display Price
    DisplayPrice,
    ///GOV - Government Price
    GovernmentPrice,
    ///GSP - Shelf Price
    ShelfPrice,
    ///GTP - Temporary Price Reduction Price
    TemporaryPriceReductionPrice,
    ///ICL - Unit Price Through Quantity
    UnitPriceThroughQuantity,
    ///IND - Industrial Price
    IndustrialPrice,
    ///INS - Institutional Price
    InstitutionalPrice,
    ///INV - Invoice Billing Price
    InvoiceBillingPrice,
    ///LAR - Labor Rate
    LaborRate,
    ///LCP - Last Cost Price
    LastCostPrice,
    ///LPP - Lease to Purchase Price
    LeaseToPurchasePrice,
    ///LPR - List Price
    ListPrice,
    ///MAP - Mandatory to Advise Unit Price
    MandatoryToAdviseUnitPrice,
    ///MAS - Minimum Activity Surcharge
    MinimumActivitySurcharge,
    ///MAX - Maximum Order Quantity Price
    MaximumOrderQuantityPrice,
    ///MIN - Minimum Order Quantity Price
    MinimumOrderQuantityPrice,
    ///MNC - Minimum Charge
    MinimumCharge,
    ///MNR - Minimum Release Quantity Price
    MinimumReleaseQuantityPrice,
    ///MOD - Modal Premium
    ModalPremium,
    ///MPR - Maximum Price Reduction
    MaximumPriceReduction,
    ///MSR - Manufacturer's Suggested Retail
    ManufacturersSuggestedRetail,
    ///MXR - Maximum Release Quantity Price
    MaximumReleaseQuantityPrice,
    ///N01 - Noncontract Tier 1
    NoncontractTier1,
    ///N02 - Noncontract Tier 2
    NoncontractTier2,
    ///N03 - Noncontract Tier 3
    NoncontractTier3,
    ///N04 - Noncontract Tier 4
    NoncontractTier4,
    ///N05 - Noncontract Tier 5
    NoncontractTier5,
    ///N06 - Noncontract Tier 6
    NoncontractTier6,
    ///N07 - Noncontract Tier 7
    NoncontractTier7,
    ///N08 - Noncontract Tier 8
    NoncontractTier8,
    ///N09 - Noncontract Tier 9
    NoncontractTier9,
    ///N10 - Noncontract Tier 10
    NoncontractTier10,
    ///N11 - Noncontract Tier 11
    NoncontractTier11,
    ///N12 - Noncontract Tier 12
    NoncontractTier12,
    ///N13 - Noncontract Tier 13
    NoncontractTier13,
    ///N14 - Noncontract Tier 14
    NoncontractTier14,
    ///N15 - Noncontract Tier 15
    NoncontractTier15,
    ///N16 - Noncontract Tier 16
    NoncontractTier16,
    ///N17 - Noncontract Tier 17
    NoncontractTier17,
    ///N18 - Noncontract Tier 18
    NoncontractTier18,
    ///N19 - Noncontract Tier 19
    NoncontractTier19,
    ///N20 - Noncontract Tier 20
    NoncontractTier20,
    ///N21 - Noncontract Tier 21
    NoncontractTier21,
    ///N22 - Noncontract Tier 22
    NoncontractTier22,
    ///N23 - Noncontract Tier 23
    NoncontractTier23,
    ///N24 - Noncontract Tier 24
    NoncontractTier24,
    ///N25 - Noncontract Tier 25
    NoncontractTier25,
    ///N26 - Noncontract Tier 26
    NoncontractTier26,
    ///N27 - Noncontract Tier 27
    NoncontractTier27,
    ///N28 - Noncontract Tier 28
    NoncontractTier28,
    ///N29 - Noncontract Tier 29
    NoncontractTier29,
    ///N30 - Noncontract Tier 30
    NoncontractTier30,
    ///N31 - No Charge
    NoCharge,
    ///NET - Net Item Price
    NetItemPrice,
    ///OAP - Optional to Advise Unit Price
    OptionalToAdviseUnitPrice,
    ///OPP - Original Purchase Order Price
    OriginalPurchaseOrderPrice,
    ///PAP - Protection Level Price
    ProtectionLevelPrice,
    ///PAQ - Price Break Quantity(s)
    CodePAQ,
    ///PBQ - Unit Price Beginning Quantity
    UnitPriceBeginningQuantity,
    ///PBR - Price Break Purchase Order Count
    PriceBreakPurchaseOrderCount,
    ///PHS - Public Health Service Price
    PublicHealthServicePrice,
    ///PIE - Price in Effect at Time of Shipment
    PriceInEffectAtTimeOfShipment,
    ///PLT - Producing Plant Price
    ProducingPlantPrice,
    ///PPA - Packing Level Price
    PackingLevelPrice,
    ///PPD - Prepaid Freight Charges
    PrepaidFreightCharges,
    ///PRF - Professional Price
    ProfessionalPrice,
    ///PRO - Producer's Price
    ProducersPrice,
    ///PRP - Promotional price
    PromotionalPrice,
    ///PUR - Purchase
    Purchase,
    ///QTE - Quote Price
    QuotePrice,
    ///REG - Regular Charge
    RegularCharge,
    ///RES - Resale
    Resale,
    ///RPA - Rental Price, Annual
    CodeRPA,
    ///RPM - Rental Price, Monthly
    CodeRPM,
    ///RPP - Replacement Price
    ReplacementPrice,
    ///RSH - Rush Charge
    RushCharge,
    ///RTL - Retail
    Retail,
    ///SAC - Service Attempted Charge
    ServiceAttemptedCharge,
    ///SDP - Suggested Dealer Net Price
    SuggestedDealerNetPrice,
    ///SFP - Suggested Fleet Price
    SuggestedFleetPrice,
    ///SHD - Ship and Debit
    ShipAndDebit,
    ///SLP - Suggested List Price
    SuggestedListPrice,
    ///SPC - Special Price
    SpecialPrice,
    ///SPE - Single Price (Factors Equalized)
    CodeSPE,
    ///SSP - Secondary Supply Plant
    SecondarySupplyPlant,
    ///STA - Standard Price
    StandardPrice,
    ///SUM - Sum of Line Items
    SumOfLineItems,
    ///SWP - Suggested Wholesale Price
    SuggestedWholesalePrice,
    ///THP - Threshold Price
    ThresholdPrice,
    ///TOT - Total Invoice Amount Due
    TotalInvoiceAmountDue,
    ///TRF - Transfer
    Transfer,
    ///UCP - Unit cost price
    UnitCostPrice,
    ///ULC - Unsalable Item List Cost
    UnsalableItemListCost,
    ///WAR - Public Warehouse Price
    PublicWarehousePrice,
    ///WHL - Wholesale
    Wholesale,
    ///WSP - Waived Service Price
    WaivedServicePrice,
    ///ZNP - Zone Price
    ZonePrice,
}
impl PriceIdentifierCode {
    pub fn code(&self) -> &str {
        {
            use PriceIdentifierCode::*;
            match self {
                Actual => "ACT",
                AverageGenericProductPrice => "AGC",
                AlternatePrice => "ALT",
                AverageWholesalePrice => "AWP",
                BalanceBasedPrice => "BBP",
                BaseCharge => "BCH",
                BidPrice => "BID",
                ContractTier1 => "C01",
                ContractTier2 => "C02",
                ContractTier3 => "C03",
                ContractTier4 => "C04",
                ContractTier5 => "C05",
                ContractTier6 => "C06",
                ContractTier7 => "C07",
                ContractTier8 => "C08",
                ContractTier9 => "C09",
                ContractTier10 => "C10",
                ContractTier11 => "C11",
                ContractTier12 => "C12",
                ContractTier13 => "C13",
                ContractTier14 => "C14",
                ContractTier15 => "C15",
                ContractTier16 => "C16",
                ContractTier17 => "C17",
                ContractTier18 => "C18",
                ContractTier19 => "C19",
                ContractTier20 => "C20",
                ContractTier21 => "C21",
                ContractTier22 => "C22",
                ContractTier23 => "C23",
                ContractTier24 => "C24",
                ContractTier25 => "C25",
                ContractTier26 => "C26",
                ContractTier27 => "C27",
                ContractTier28 => "C28",
                ContractTier29 => "C29",
                ContractTier30 => "C30",
                CancellationCharge => "CAN",
                CatalogPrice => "CAT",
                CodeCDF => "CDF",
                CurrentDomesticValue => "CDV",
                ChangedPrice => "CHG",
                ContractPrice => "CON",
                ConfirmedUnitPrice => "CUP",
                DeclaredCustomsUnitValue => "CUS",
                CodeD01 => "D01",
                DepotPrice => "D02",
                CodeD03 => "D03",
                DealerAdjustedPrice => "DAP",
                DistributorsPrice => "DIS",
                DiscountPrice => "DPR",
                DiscountAmountAllowed => "DSC",
                DirectStoreDelivery => "DSD",
                DirectShipProgramPrice => "DSP",
                CodeEDM => "EDM",
                EmergencyDirectShipPrice => "EDP",
                CodeEDS => "EDS",
                CodeEDW => "EDW",
                EstimatedLandedCost => "ELC",
                EstimatedPrice => "EST",
                ExpectedUnitPrice => "EUP",
                FlatCharge => "FCH",
                FirstCostPrice => "FCP",
                FrequentDeliveryService => "FDS",
                FederalExciseTax => "FET",
                FreeGoodsPrice => "FGP",
                FormulaPrice => "FOR",
                FreeServicePrice => "FSP",
                CodeFUL => "FUL",
                FirmPriceDoNotAdvise => "FUP",
                AdvertisingPrice => "GAP",
                DisplayPrice => "GDP",
                GovernmentPrice => "GOV",
                ShelfPrice => "GSP",
                TemporaryPriceReductionPrice => "GTP",
                UnitPriceThroughQuantity => "ICL",
                IndustrialPrice => "IND",
                InstitutionalPrice => "INS",
                InvoiceBillingPrice => "INV",
                LaborRate => "LAR",
                LastCostPrice => "LCP",
                LeaseToPurchasePrice => "LPP",
                ListPrice => "LPR",
                MandatoryToAdviseUnitPrice => "MAP",
                MinimumActivitySurcharge => "MAS",
                MaximumOrderQuantityPrice => "MAX",
                MinimumOrderQuantityPrice => "MIN",
                MinimumCharge => "MNC",
                MinimumReleaseQuantityPrice => "MNR",
                ModalPremium => "MOD",
                MaximumPriceReduction => "MPR",
                ManufacturersSuggestedRetail => "MSR",
                MaximumReleaseQuantityPrice => "MXR",
                NoncontractTier1 => "N01",
                NoncontractTier2 => "N02",
                NoncontractTier3 => "N03",
                NoncontractTier4 => "N04",
                NoncontractTier5 => "N05",
                NoncontractTier6 => "N06",
                NoncontractTier7 => "N07",
                NoncontractTier8 => "N08",
                NoncontractTier9 => "N09",
                NoncontractTier10 => "N10",
                NoncontractTier11 => "N11",
                NoncontractTier12 => "N12",
                NoncontractTier13 => "N13",
                NoncontractTier14 => "N14",
                NoncontractTier15 => "N15",
                NoncontractTier16 => "N16",
                NoncontractTier17 => "N17",
                NoncontractTier18 => "N18",
                NoncontractTier19 => "N19",
                NoncontractTier20 => "N20",
                NoncontractTier21 => "N21",
                NoncontractTier22 => "N22",
                NoncontractTier23 => "N23",
                NoncontractTier24 => "N24",
                NoncontractTier25 => "N25",
                NoncontractTier26 => "N26",
                NoncontractTier27 => "N27",
                NoncontractTier28 => "N28",
                NoncontractTier29 => "N29",
                NoncontractTier30 => "N30",
                NoCharge => "N31",
                NetItemPrice => "NET",
                OptionalToAdviseUnitPrice => "OAP",
                OriginalPurchaseOrderPrice => "OPP",
                ProtectionLevelPrice => "PAP",
                CodePAQ => "PAQ",
                UnitPriceBeginningQuantity => "PBQ",
                PriceBreakPurchaseOrderCount => "PBR",
                PublicHealthServicePrice => "PHS",
                PriceInEffectAtTimeOfShipment => "PIE",
                ProducingPlantPrice => "PLT",
                PackingLevelPrice => "PPA",
                PrepaidFreightCharges => "PPD",
                ProfessionalPrice => "PRF",
                ProducersPrice => "PRO",
                PromotionalPrice => "PRP",
                Purchase => "PUR",
                QuotePrice => "QTE",
                RegularCharge => "REG",
                Resale => "RES",
                CodeRPA => "RPA",
                CodeRPM => "RPM",
                ReplacementPrice => "RPP",
                RushCharge => "RSH",
                Retail => "RTL",
                ServiceAttemptedCharge => "SAC",
                SuggestedDealerNetPrice => "SDP",
                SuggestedFleetPrice => "SFP",
                ShipAndDebit => "SHD",
                SuggestedListPrice => "SLP",
                SpecialPrice => "SPC",
                CodeSPE => "SPE",
                SecondarySupplyPlant => "SSP",
                StandardPrice => "STA",
                SumOfLineItems => "SUM",
                SuggestedWholesalePrice => "SWP",
                ThresholdPrice => "THP",
                TotalInvoiceAmountDue => "TOT",
                Transfer => "TRF",
                UnitCostPrice => "UCP",
                UnsalableItemListCost => "ULC",
                PublicWarehousePrice => "WAR",
                Wholesale => "WHL",
                WaivedServicePrice => "WSP",
                ZonePrice => "ZNP",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<PriceIdentifierCode> {
        use PriceIdentifierCode::*;
        match code {
            b"ACT" => Some(Actual),
            b"AGC" => Some(AverageGenericProductPrice),
            b"ALT" => Some(AlternatePrice),
            b"AWP" => Some(AverageWholesalePrice),
            b"BBP" => Some(BalanceBasedPrice),
            b"BCH" => Some(BaseCharge),
            b"BID" => Some(BidPrice),
            b"C01" => Some(ContractTier1),
            b"C02" => Some(ContractTier2),
            b"C03" => Some(ContractTier3),
            b"C04" => Some(ContractTier4),
            b"C05" => Some(ContractTier5),
            b"C06" => Some(ContractTier6),
            b"C07" => Some(ContractTier7),
            b"C08" => Some(ContractTier8),
            b"C09" => Some(ContractTier9),
            b"C10" => Some(ContractTier10),
            b"C11" => Some(ContractTier11),
            b"C12" => Some(ContractTier12),
            b"C13" => Some(ContractTier13),
            b"C14" => Some(ContractTier14),
            b"C15" => Some(ContractTier15),
            b"C16" => Some(ContractTier16),
            b"C17" => Some(ContractTier17),
            b"C18" => Some(ContractTier18),
            b"C19" => Some(ContractTier19),
            b"C20" => Some(ContractTier20),
            b"C21" => Some(ContractTier21),
            b"C22" => Some(ContractTier22),
            b"C23" => Some(ContractTier23),
            b"C24" => Some(ContractTier24),
            b"C25" => Some(ContractTier25),
            b"C26" => Some(ContractTier26),
            b"C27" => Some(ContractTier27),
            b"C28" => Some(ContractTier28),
            b"C29" => Some(ContractTier29),
            b"C30" => Some(ContractTier30),
            b"CAN" => Some(CancellationCharge),
            b"CAT" => Some(CatalogPrice),
            b"CDF" => Some(CodeCDF),
            b"CDV" => Some(CurrentDomesticValue),
            b"CHG" => Some(ChangedPrice),
            b"CON" => Some(ContractPrice),
            b"CUP" => Some(ConfirmedUnitPrice),
            b"CUS" => Some(DeclaredCustomsUnitValue),
            b"D01" => Some(CodeD01),
            b"D02" => Some(DepotPrice),
            b"D03" => Some(CodeD03),
            b"DAP" => Some(DealerAdjustedPrice),
            b"DIS" => Some(DistributorsPrice),
            b"DPR" => Some(DiscountPrice),
            b"DSC" => Some(DiscountAmountAllowed),
            b"DSD" => Some(DirectStoreDelivery),
            b"DSP" => Some(DirectShipProgramPrice),
            b"EDM" => Some(CodeEDM),
            b"EDP" => Some(EmergencyDirectShipPrice),
            b"EDS" => Some(CodeEDS),
            b"EDW" => Some(CodeEDW),
            b"ELC" => Some(EstimatedLandedCost),
            b"EST" => Some(EstimatedPrice),
            b"EUP" => Some(ExpectedUnitPrice),
            b"FCH" => Some(FlatCharge),
            b"FCP" => Some(FirstCostPrice),
            b"FDS" => Some(FrequentDeliveryService),
            b"FET" => Some(FederalExciseTax),
            b"FGP" => Some(FreeGoodsPrice),
            b"FOR" => Some(FormulaPrice),
            b"FSP" => Some(FreeServicePrice),
            b"FUL" => Some(CodeFUL),
            b"FUP" => Some(FirmPriceDoNotAdvise),
            b"GAP" => Some(AdvertisingPrice),
            b"GDP" => Some(DisplayPrice),
            b"GOV" => Some(GovernmentPrice),
            b"GSP" => Some(ShelfPrice),
            b"GTP" => Some(TemporaryPriceReductionPrice),
            b"ICL" => Some(UnitPriceThroughQuantity),
            b"IND" => Some(IndustrialPrice),
            b"INS" => Some(InstitutionalPrice),
            b"INV" => Some(InvoiceBillingPrice),
            b"LAR" => Some(LaborRate),
            b"LCP" => Some(LastCostPrice),
            b"LPP" => Some(LeaseToPurchasePrice),
            b"LPR" => Some(ListPrice),
            b"MAP" => Some(MandatoryToAdviseUnitPrice),
            b"MAS" => Some(MinimumActivitySurcharge),
            b"MAX" => Some(MaximumOrderQuantityPrice),
            b"MIN" => Some(MinimumOrderQuantityPrice),
            b"MNC" => Some(MinimumCharge),
            b"MNR" => Some(MinimumReleaseQuantityPrice),
            b"MOD" => Some(ModalPremium),
            b"MPR" => Some(MaximumPriceReduction),
            b"MSR" => Some(ManufacturersSuggestedRetail),
            b"MXR" => Some(MaximumReleaseQuantityPrice),
            b"N01" => Some(NoncontractTier1),
            b"N02" => Some(NoncontractTier2),
            b"N03" => Some(NoncontractTier3),
            b"N04" => Some(NoncontractTier4),
            b"N05" => Some(NoncontractTier5),
            b"N06" => Some(NoncontractTier6),
            b"N07" => Some(NoncontractTier7),
            b"N08" => Some(NoncontractTier8),
            b"N09" => Some(NoncontractTier9),
            b"N10" => Some(NoncontractTier10),
            b"N11" => Some(NoncontractTier11),
            b"N12" => Some(NoncontractTier12),
            b"N13" => Some(NoncontractTier13),
            b"N14" => Some(NoncontractTier14),
            b"N15" => Some(NoncontractTier15),
            b"N16" => Some(NoncontractTier16),
            b"N17" => Some(NoncontractTier17),
            b"N18" => Some(NoncontractTier18),
            b"N19" => Some(NoncontractTier19),
            b"N20" => Some(NoncontractTier20),
            b"N21" => Some(NoncontractTier21),
            b"N22" => Some(NoncontractTier22),
            b"N23" => Some(NoncontractTier23),
            b"N24" => Some(NoncontractTier24),
            b"N25" => Some(NoncontractTier25),
            b"N26" => Some(NoncontractTier26),
            b"N27" => Some(NoncontractTier27),
            b"N28" => Some(NoncontractTier28),
            b"N29" => Some(NoncontractTier29),
            b"N30" => Some(NoncontractTier30),
            b"N31" => Some(NoCharge),
            b"NET" => Some(NetItemPrice),
            b"OAP" => Some(OptionalToAdviseUnitPrice),
            b"OPP" => Some(OriginalPurchaseOrderPrice),
            b"PAP" => Some(ProtectionLevelPrice),
            b"PAQ" => Some(CodePAQ),
            b"PBQ" => Some(UnitPriceBeginningQuantity),
            b"PBR" => Some(PriceBreakPurchaseOrderCount),
            b"PHS" => Some(PublicHealthServicePrice),
            b"PIE" => Some(PriceInEffectAtTimeOfShipment),
            b"PLT" => Some(ProducingPlantPrice),
            b"PPA" => Some(PackingLevelPrice),
            b"PPD" => Some(PrepaidFreightCharges),
            b"PRF" => Some(ProfessionalPrice),
            b"PRO" => Some(ProducersPrice),
            b"PRP" => Some(PromotionalPrice),
            b"PUR" => Some(Purchase),
            b"QTE" => Some(QuotePrice),
            b"REG" => Some(RegularCharge),
            b"RES" => Some(Resale),
            b"RPA" => Some(CodeRPA),
            b"RPM" => Some(CodeRPM),
            b"RPP" => Some(ReplacementPrice),
            b"RSH" => Some(RushCharge),
            b"RTL" => Some(Retail),
            b"SAC" => Some(ServiceAttemptedCharge),
            b"SDP" => Some(SuggestedDealerNetPrice),
            b"SFP" => Some(SuggestedFleetPrice),
            b"SHD" => Some(ShipAndDebit),
            b"SLP" => Some(SuggestedListPrice),
            b"SPC" => Some(SpecialPrice),
            b"SPE" => Some(CodeSPE),
            b"SSP" => Some(SecondarySupplyPlant),
            b"STA" => Some(StandardPrice),
            b"SUM" => Some(SumOfLineItems),
            b"SWP" => Some(SuggestedWholesalePrice),
            b"THP" => Some(ThresholdPrice),
            b"TOT" => Some(TotalInvoiceAmountDue),
            b"TRF" => Some(Transfer),
            b"UCP" => Some(UnitCostPrice),
            b"ULC" => Some(UnsalableItemListCost),
            b"WAR" => Some(PublicWarehousePrice),
            b"WHL" => Some(Wholesale),
            b"WSP" => Some(WaivedServicePrice),
            b"ZNP" => Some(ZonePrice),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use PriceIdentifierCode::*;
        match self {
            Actual => "Actual",
            AverageGenericProductPrice => "Average Generic Product Price",
            AlternatePrice => "Alternate Price",
            AverageWholesalePrice => "Average Wholesale Price",
            BalanceBasedPrice => "Balance-Based Price",
            BaseCharge => "Base Charge",
            BidPrice => "Bid Price",
            ContractTier1 => "Contract Tier 1",
            ContractTier2 => "Contract Tier 2",
            ContractTier3 => "Contract Tier 3",
            ContractTier4 => "Contract Tier 4",
            ContractTier5 => "Contract Tier 5",
            ContractTier6 => "Contract Tier 6",
            ContractTier7 => "Contract Tier 7",
            ContractTier8 => "Contract Tier 8",
            ContractTier9 => "Contract Tier 9",
            ContractTier10 => "Contract Tier 10",
            ContractTier11 => "Contract Tier 11",
            ContractTier12 => "Contract Tier 12",
            ContractTier13 => "Contract Tier 13",
            ContractTier14 => "Contract Tier 14",
            ContractTier15 => "Contract Tier 15",
            ContractTier16 => "Contract Tier 16",
            ContractTier17 => "Contract Tier 17",
            ContractTier18 => "Contract Tier 18",
            ContractTier19 => "Contract Tier 19",
            ContractTier20 => "Contract Tier 20",
            ContractTier21 => "Contract Tier 21",
            ContractTier22 => "Contract Tier 22",
            ContractTier23 => "Contract Tier 23",
            ContractTier24 => "Contract Tier 24",
            ContractTier25 => "Contract Tier 25",
            ContractTier26 => "Contract Tier 26",
            ContractTier27 => "Contract Tier 27",
            ContractTier28 => "Contract Tier 28",
            ContractTier29 => "Contract Tier 29",
            ContractTier30 => "Contract Tier 30",
            CancellationCharge => "Cancellation Charge",
            CatalogPrice => "Catalog Price",
            CodeCDF => "Central Distribution Facility (Warehouse)",
            CurrentDomesticValue => "Current Domestic Value",
            ChangedPrice => "Changed Price",
            ContractPrice => "Contract Price",
            ConfirmedUnitPrice => "Confirmed Unit Price",
            DeclaredCustomsUnitValue => "Declared Customs Unit Value",
            CodeD01 => "Federal Supply Schedule (FSS) Price",
            DepotPrice => "Depot Price",
            CodeD03 => "Distribution and Pricing Agreement (DAPA) Price",
            DealerAdjustedPrice => "Dealer Adjusted Price",
            DistributorsPrice => "Distributor's Price",
            DiscountPrice => "Discount Price",
            DiscountAmountAllowed => "Discount Amount Allowed",
            DirectStoreDelivery => "Direct Store Delivery",
            DirectShipProgramPrice => "Direct Ship Program Price",
            CodeEDM => "Emergency Direct Ship Price (Original Equipment Manufacturer)",
            EmergencyDirectShipPrice => "Emergency Direct Ship Price",
            CodeEDS => "Emergency Direct Ship Price (Supplier)",
            CodeEDW => "Emergency Direct Ship Price (Warehouse)",
            EstimatedLandedCost => "Estimated Landed Cost",
            EstimatedPrice => "Estimated Price",
            ExpectedUnitPrice => "Expected Unit Price",
            FlatCharge => "Flat Charge",
            FirstCostPrice => "First Cost Price",
            FrequentDeliveryService => "Frequent Delivery Service",
            FederalExciseTax => "Federal Excise Tax",
            FreeGoodsPrice => "Free Goods Price",
            FormulaPrice => "Formula Price",
            FreeServicePrice => "Free Service Price",
            CodeFUL => {
                "Federal Upper Limit Price (Maximum Allowable Cost Pricing for Drugs)"
            }
            FirmPriceDoNotAdvise => "Firm Price - Do Not Advise",
            AdvertisingPrice => "Advertising Price",
            DisplayPrice => "Display Price",
            GovernmentPrice => "Government Price",
            ShelfPrice => "Shelf Price",
            TemporaryPriceReductionPrice => "Temporary Price Reduction Price",
            UnitPriceThroughQuantity => "Unit Price Through Quantity",
            IndustrialPrice => "Industrial Price",
            InstitutionalPrice => "Institutional Price",
            InvoiceBillingPrice => "Invoice Billing Price",
            LaborRate => "Labor Rate",
            LastCostPrice => "Last Cost Price",
            LeaseToPurchasePrice => "Lease to Purchase Price",
            ListPrice => "List Price",
            MandatoryToAdviseUnitPrice => "Mandatory to Advise Unit Price",
            MinimumActivitySurcharge => "Minimum Activity Surcharge",
            MaximumOrderQuantityPrice => "Maximum Order Quantity Price",
            MinimumOrderQuantityPrice => "Minimum Order Quantity Price",
            MinimumCharge => "Minimum Charge",
            MinimumReleaseQuantityPrice => "Minimum Release Quantity Price",
            ModalPremium => "Modal Premium",
            MaximumPriceReduction => "Maximum Price Reduction",
            ManufacturersSuggestedRetail => "Manufacturer's Suggested Retail",
            MaximumReleaseQuantityPrice => "Maximum Release Quantity Price",
            NoncontractTier1 => "Noncontract Tier 1",
            NoncontractTier2 => "Noncontract Tier 2",
            NoncontractTier3 => "Noncontract Tier 3",
            NoncontractTier4 => "Noncontract Tier 4",
            NoncontractTier5 => "Noncontract Tier 5",
            NoncontractTier6 => "Noncontract Tier 6",
            NoncontractTier7 => "Noncontract Tier 7",
            NoncontractTier8 => "Noncontract Tier 8",
            NoncontractTier9 => "Noncontract Tier 9",
            NoncontractTier10 => "Noncontract Tier 10",
            NoncontractTier11 => "Noncontract Tier 11",
            NoncontractTier12 => "Noncontract Tier 12",
            NoncontractTier13 => "Noncontract Tier 13",
            NoncontractTier14 => "Noncontract Tier 14",
            NoncontractTier15 => "Noncontract Tier 15",
            NoncontractTier16 => "Noncontract Tier 16",
            NoncontractTier17 => "Noncontract Tier 17",
            NoncontractTier18 => "Noncontract Tier 18",
            NoncontractTier19 => "Noncontract Tier 19",
            NoncontractTier20 => "Noncontract Tier 20",
            NoncontractTier21 => "Noncontract Tier 21",
            NoncontractTier22 => "Noncontract Tier 22",
            NoncontractTier23 => "Noncontract Tier 23",
            NoncontractTier24 => "Noncontract Tier 24",
            NoncontractTier25 => "Noncontract Tier 25",
            NoncontractTier26 => "Noncontract Tier 26",
            NoncontractTier27 => "Noncontract Tier 27",
            NoncontractTier28 => "Noncontract Tier 28",
            NoncontractTier29 => "Noncontract Tier 29",
            NoncontractTier30 => "Noncontract Tier 30",
            NoCharge => "No Charge",
            NetItemPrice => "Net Item Price",
            OptionalToAdviseUnitPrice => "Optional to Advise Unit Price",
            OriginalPurchaseOrderPrice => "Original Purchase Order Price",
            ProtectionLevelPrice => "Protection Level Price",
            CodePAQ => "Price Break Quantity(s)",
            UnitPriceBeginningQuantity => "Unit Price Beginning Quantity",
            PriceBreakPurchaseOrderCount => "Price Break Purchase Order Count",
            PublicHealthServicePrice => "Public Health Service Price",
            PriceInEffectAtTimeOfShipment => "Price in Effect at Time of Shipment",
            ProducingPlantPrice => "Producing Plant Price",
            PackingLevelPrice => "Packing Level Price",
            PrepaidFreightCharges => "Prepaid Freight Charges",
            ProfessionalPrice => "Professional Price",
            ProducersPrice => "Producer's Price",
            PromotionalPrice => "Promotional price",
            Purchase => "Purchase",
            QuotePrice => "Quote Price",
            RegularCharge => "Regular Charge",
            Resale => "Resale",
            CodeRPA => "Rental Price, Annual",
            CodeRPM => "Rental Price, Monthly",
            ReplacementPrice => "Replacement Price",
            RushCharge => "Rush Charge",
            Retail => "Retail",
            ServiceAttemptedCharge => "Service Attempted Charge",
            SuggestedDealerNetPrice => "Suggested Dealer Net Price",
            SuggestedFleetPrice => "Suggested Fleet Price",
            ShipAndDebit => "Ship and Debit",
            SuggestedListPrice => "Suggested List Price",
            SpecialPrice => "Special Price",
            CodeSPE => "Single Price (Factors Equalized)",
            SecondarySupplyPlant => "Secondary Supply Plant",
            StandardPrice => "Standard Price",
            SumOfLineItems => "Sum of Line Items",
            SuggestedWholesalePrice => "Suggested Wholesale Price",
            ThresholdPrice => "Threshold Price",
            TotalInvoiceAmountDue => "Total Invoice Amount Due",
            Transfer => "Transfer",
            UnitCostPrice => "Unit cost price",
            UnsalableItemListCost => "Unsalable Item List Cost",
            PublicWarehousePrice => "Public Warehouse Price",
            Wholesale => "Wholesale",
            WaivedServicePrice => "Waived Service Price",
            ZonePrice => "Zone Price",
        }
    }
    fn from_description(description: &str) -> Option<PriceIdentifierCode> {
        {
            use PriceIdentifierCode::*;
            match description {
                "Actual" => Some(Actual),
                "Average Generic Product Price" => Some(AverageGenericProductPrice),
                "Alternate Price" => Some(AlternatePrice),
                "Average Wholesale Price" => Some(AverageWholesalePrice),
                "Balance-Based Price" => Some(BalanceBasedPrice),
                "Base Charge" => Some(BaseCharge),
                "Bid Price" => Some(BidPrice),
                "Contract Tier 1" => Some(ContractTier1),
                "Contract Tier 2" => Some(ContractTier2),
                "Contract Tier 3" => Some(ContractTier3),
                "Contract Tier 4" => Some(ContractTier4),
                "Contract Tier 5" => Some(ContractTier5),
                "Contract Tier 6" => Some(ContractTier6),
                "Contract Tier 7" => Some(ContractTier7),
                "Contract Tier 8" => Some(ContractTier8),
                "Contract Tier 9" => Some(ContractTier9),
                "Contract Tier 10" => Some(ContractTier10),
                "Contract Tier 11" => Some(ContractTier11),
                "Contract Tier 12" => Some(ContractTier12),
                "Contract Tier 13" => Some(ContractTier13),
                "Contract Tier 14" => Some(ContractTier14),
                "Contract Tier 15" => Some(ContractTier15),
                "Contract Tier 16" => Some(ContractTier16),
                "Contract Tier 17" => Some(ContractTier17),
                "Contract Tier 18" => Some(ContractTier18),
                "Contract Tier 19" => Some(ContractTier19),
                "Contract Tier 20" => Some(ContractTier20),
                "Contract Tier 21" => Some(ContractTier21),
                "Contract Tier 22" => Some(ContractTier22),
                "Contract Tier 23" => Some(ContractTier23),
                "Contract Tier 24" => Some(ContractTier24),
                "Contract Tier 25" => Some(ContractTier25),
                "Contract Tier 26" => Some(ContractTier26),
                "Contract Tier 27" => Some(ContractTier27),
                "Contract Tier 28" => Some(ContractTier28),
                "Contract Tier 29" => Some(ContractTier29),
                "Contract Tier 30" => Some(ContractTier30),
                "Cancellation Charge" => Some(CancellationCharge),
                "Catalog Price" => Some(CatalogPrice),
                "Central Distribution Facility (Warehouse)" => Some(CodeCDF),
                "Current Domestic Value" => Some(CurrentDomesticValue),
                "Changed Price" => Some(ChangedPrice),
                "Contract Price" => Some(ContractPrice),
                "Confirmed Unit Price" => Some(ConfirmedUnitPrice),
                "Declared Customs Unit Value" => Some(DeclaredCustomsUnitValue),
                "Federal Supply Schedule (FSS) Price" => Some(CodeD01),
                "Depot Price" => Some(DepotPrice),
                "Distribution and Pricing Agreement (DAPA) Price" => Some(CodeD03),
                "Dealer Adjusted Price" => Some(DealerAdjustedPrice),
                "Distributor's Price" => Some(DistributorsPrice),
                "Discount Price" => Some(DiscountPrice),
                "Discount Amount Allowed" => Some(DiscountAmountAllowed),
                "Direct Store Delivery" => Some(DirectStoreDelivery),
                "Direct Ship Program Price" => Some(DirectShipProgramPrice),
                "Emergency Direct Ship Price (Original Equipment Manufacturer)" => {
                    Some(CodeEDM)
                }
                "Emergency Direct Ship Price" => Some(EmergencyDirectShipPrice),
                "Emergency Direct Ship Price (Supplier)" => Some(CodeEDS),
                "Emergency Direct Ship Price (Warehouse)" => Some(CodeEDW),
                "Estimated Landed Cost" => Some(EstimatedLandedCost),
                "Estimated Price" => Some(EstimatedPrice),
                "Expected Unit Price" => Some(ExpectedUnitPrice),
                "Flat Charge" => Some(FlatCharge),
                "First Cost Price" => Some(FirstCostPrice),
                "Frequent Delivery Service" => Some(FrequentDeliveryService),
                "Federal Excise Tax" => Some(FederalExciseTax),
                "Free Goods Price" => Some(FreeGoodsPrice),
                "Formula Price" => Some(FormulaPrice),
                "Free Service Price" => Some(FreeServicePrice),
                "Federal Upper Limit Price (Maximum Allowable Cost Pricing for Drugs)" => {
                    Some(CodeFUL)
                }
                "Firm Price - Do Not Advise" => Some(FirmPriceDoNotAdvise),
                "Advertising Price" => Some(AdvertisingPrice),
                "Display Price" => Some(DisplayPrice),
                "Government Price" => Some(GovernmentPrice),
                "Shelf Price" => Some(ShelfPrice),
                "Temporary Price Reduction Price" => Some(TemporaryPriceReductionPrice),
                "Unit Price Through Quantity" => Some(UnitPriceThroughQuantity),
                "Industrial Price" => Some(IndustrialPrice),
                "Institutional Price" => Some(InstitutionalPrice),
                "Invoice Billing Price" => Some(InvoiceBillingPrice),
                "Labor Rate" => Some(LaborRate),
                "Last Cost Price" => Some(LastCostPrice),
                "Lease to Purchase Price" => Some(LeaseToPurchasePrice),
                "List Price" => Some(ListPrice),
                "Mandatory to Advise Unit Price" => Some(MandatoryToAdviseUnitPrice),
                "Minimum Activity Surcharge" => Some(MinimumActivitySurcharge),
                "Maximum Order Quantity Price" => Some(MaximumOrderQuantityPrice),
                "Minimum Order Quantity Price" => Some(MinimumOrderQuantityPrice),
                "Minimum Charge" => Some(MinimumCharge),
                "Minimum Release Quantity Price" => Some(MinimumReleaseQuantityPrice),
                "Modal Premium" => Some(ModalPremium),
                "Maximum Price Reduction" => Some(MaximumPriceReduction),
                "Manufacturer's Suggested Retail" => Some(ManufacturersSuggestedRetail),
                "Maximum Release Quantity Price" => Some(MaximumReleaseQuantityPrice),
                "Noncontract Tier 1" => Some(NoncontractTier1),
                "Noncontract Tier 2" => Some(NoncontractTier2),
                "Noncontract Tier 3" => Some(NoncontractTier3),
                "Noncontract Tier 4" => Some(NoncontractTier4),
                "Noncontract Tier 5" => Some(NoncontractTier5),
                "Noncontract Tier 6" => Some(NoncontractTier6),
                "Noncontract Tier 7" => Some(NoncontractTier7),
                "Noncontract Tier 8" => Some(NoncontractTier8),
                "Noncontract Tier 9" => Some(NoncontractTier9),
                "Noncontract Tier 10" => Some(NoncontractTier10),
                "Noncontract Tier 11" => Some(NoncontractTier11),
                "Noncontract Tier 12" => Some(NoncontractTier12),
                "Noncontract Tier 13" => Some(NoncontractTier13),
                "Noncontract Tier 14" => Some(NoncontractTier14),
                "Noncontract Tier 15" => Some(NoncontractTier15),
                "Noncontract Tier 16" => Some(NoncontractTier16),
                "Noncontract Tier 17" => Some(NoncontractTier17),
                "Noncontract Tier 18" => Some(NoncontractTier18),
                "Noncontract Tier 19" => Some(NoncontractTier19),
                "Noncontract Tier 20" => Some(NoncontractTier20),
                "Noncontract Tier 21" => Some(NoncontractTier21),
                "Noncontract Tier 22" => Some(NoncontractTier22),
                "Noncontract Tier 23" => Some(NoncontractTier23),
                "Noncontract Tier 24" => Some(NoncontractTier24),
                "Noncontract Tier 25" => Some(NoncontractTier25),
                "Noncontract Tier 26" => Some(NoncontractTier26),
                "Noncontract Tier 27" => Some(NoncontractTier27),
                "Noncontract Tier 28" => Some(NoncontractTier28),
                "Noncontract Tier 29" => Some(NoncontractTier29),
                "Noncontract Tier 30" => Some(NoncontractTier30),
                "No Charge" => Some(NoCharge),
                "Net Item Price" => Some(NetItemPrice),
                "Optional to Advise Unit Price" => Some(OptionalToAdviseUnitPrice),
                "Original Purchase Order Price" => Some(OriginalPurchaseOrderPrice),
                "Protection Level Price" => Some(ProtectionLevelPrice),
                "Price Break Quantity(s)" => Some(CodePAQ),
                "Unit Price Beginning Quantity" => Some(UnitPriceBeginningQuantity),
                "Price Break Purchase Order Count" => Some(PriceBreakPurchaseOrderCount),
                "Public Health Service Price" => Some(PublicHealthServicePrice),
                "Price in Effect at Time of Shipment" => {
                    Some(PriceInEffectAtTimeOfShipment)
                }
                "Producing Plant Price" => Some(ProducingPlantPrice),
                "Packing Level Price" => Some(PackingLevelPrice),
                "Prepaid Freight Charges" => Some(PrepaidFreightCharges),
                "Professional Price" => Some(ProfessionalPrice),
                "Producer's Price" => Some(ProducersPrice),
                "Promotional price" => Some(PromotionalPrice),
                "Purchase" => Some(Purchase),
                "Quote Price" => Some(QuotePrice),
                "Regular Charge" => Some(RegularCharge),
                "Resale" => Some(Resale),
                "Rental Price, Annual" => Some(CodeRPA),
                "Rental Price, Monthly" => Some(CodeRPM),
                "Replacement Price" => Some(ReplacementPrice),
                "Rush Charge" => Some(RushCharge),
                "Retail" => Some(Retail),
                "Service Attempted Charge" => Some(ServiceAttemptedCharge),
                "Suggested Dealer Net Price" => Some(SuggestedDealerNetPrice),
                "Suggested Fleet Price" => Some(SuggestedFleetPrice),
                "Ship and Debit" => Some(ShipAndDebit),
                "Suggested List Price" => Some(SuggestedListPrice),
                "Special Price" => Some(SpecialPrice),
                "Single Price (Factors Equalized)" => Some(CodeSPE),
                "Secondary Supply Plant" => Some(SecondarySupplyPlant),
                "Standard Price" => Some(StandardPrice),
                "Sum of Line Items" => Some(SumOfLineItems),
                "Suggested Wholesale Price" => Some(SuggestedWholesalePrice),
                "Threshold Price" => Some(ThresholdPrice),
                "Total Invoice Amount Due" => Some(TotalInvoiceAmountDue),
                "Transfer" => Some(Transfer),
                "Unit cost price" => Some(UnitCostPrice),
                "Unsalable Item List Cost" => Some(UnsalableItemListCost),
                "Public Warehouse Price" => Some(PublicWarehousePrice),
                "Wholesale" => Some(Wholesale),
                "Waived Service Price" => Some(WaivedServicePrice),
                "Zone Price" => Some(ZonePrice),
                _ => None,
            }
        }
    }
}
impl Serialize for PriceIdentifierCode {
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
    type Value = PriceIdentifierCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Price Identifier Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PriceIdentifierCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Price Identifier Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PriceIdentifierCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Price Identifier Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for PriceIdentifierCode {
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