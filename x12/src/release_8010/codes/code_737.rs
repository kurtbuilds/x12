use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**737

See docs at <https://www.stedi.com/edi/x12/element/737>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementReferenceIdCode {
    ///9L - Ninth Layer
    NinthLayer,
    ///AA - Meter reading-beginning actual/ending actual
    MeterReadingBeginningActualEndingActual,
    ///AB - Average Balance
    AverageBalance,
    ///AC - Maximum Balance
    MaximumBalance,
    ///AD - Serving Specifications
    ServingSpecifications,
    ///AE - Meter reading-beginning actual/ending estimated
    MeterReadingBeginningActualEndingEstimated,
    ///AF - Actual Total
    ActualTotal,
    ///AG - Compliance Total
    ComplianceTotal,
    ///AH - Gross Compliance Total
    GrossComplianceTotal,
    ///AI - Net Compliance Total
    NetComplianceTotal,
    ///AJ - Effectiveness
    Effectiveness,
    ///AK - Penetration
    Penetration,
    ///AL - Exposure
    Exposure,
    ///AM - Capacity
    Capacity,
    ///AN - Work
    Work,
    ///AO - Account Opened Period
    AccountOpenedPeriod,
    ///AP - Apartments
    Apartments,
    ///AQ - Amount Owes
    AmountOwes,
    ///AR - Annual Result
    AnnualResult,
    ///AS - Modified Exposure
    ModifiedExposure,
    ///AT - Pro Rata Factor
    ProRataFactor,
    ///AU - Sample
    Sample,
    ///AV - Average Reading
    AverageReading,
    ///AW - Relative Humidity
    RelativeHumidity,
    ///AX - Alcohol
    Alcohol,
    ///BA - Base Point
    BasePoint,
    ///BB - Bank Balance
    BankBalance,
    ///BC - Billed Actual
    BilledActual,
    ///BD - Billed Dimensions
    BilledDimensions,
    ///BL - Bundle Limitation
    BundleLimitation,
    ///BM - Base Metal
    BaseMetal,
    ///BN - Billed Minimum
    BilledMinimum,
    ///BO - Meter Reading as Billed
    MeterReadingAsBilled,
    ///BP - Bearing Piece Limitation
    BearingPieceLimitation,
    ///BR - Billed History
    BilledHistory,
    ///BT - Batch Limits
    BatchLimits,
    ///BZ - Batten Size
    BattenSize,
    ///C1 - Conformance Property Status
    ConformancePropertyStatus,
    ///CA - Coating
    Coating,
    ///CB - Condominium
    Condominium,
    ///CC - Cooperative
    Cooperative,
    ///CF - Conversion Factor
    ConversionFactor,
    ///CG - Average Owing
    AverageOwing,
    ///CH - Chemistry
    Chemistry,
    ///CJ - Current Balance
    CurrentBalance,
    ///CK - Discounted Checks
    DiscountedChecks,
    ///CL - Drafts
    Drafts,
    ///CM - Commercial
    Commercial,
    ///CN - Core Notch Dimensions
    CoreNotchDimensions,
    ///CO - Concentration
    Concentration,
    ///CP - Letter of Credit Size
    LetterOfCreditSize,
    ///CQ - Payment Orders
    PaymentOrders,
    ///CS - Core Size
    CoreSize,
    ///CT - Counts
    Counts,
    ///CU - Pledge Size
    PledgeSize,
    ///CV - Cumulative Test Period
    CumulativeTestPeriod,
    ///CW - Promissory Notes in Force
    PromissoryNotesInForce,
    ///CY - Secured Amount
    SecuredAmount,
    ///DE - Defects
    Defects,
    ///DN - Dunnage Dimension
    DunnageDimension,
    ///DR - Per Drop
    PerDrop,
    ///DS - Defect Size
    DefectSize,
    ///DT - Dimensional Tolerance
    DimensionalTolerance,
    ///EA - Meter reading-beginning estimated/ending actual
    MeterReadingBeginningEstimatedEndingActual,
    ///EE - Meter reading-beginning estimated/ending estimated
    MeterReadingBeginningEstimatedEndingEstimated,
    ///EF - Evaluation Factors
    EvaluationFactors,
    ///EL - Electrical Characteristics
    ElectricalCharacteristics,
    ///EN - Environmental Conditions
    EnvironmentalConditions,
    ///FC - First of Campaign Result
    FirstOfCampaignResult,
    ///FD - Finished Dimensions
    FinishedDimensions,
    ///FH - Radio Operations
    RadioOperations,
    ///FJ - Antenna Characteristics
    AntennaCharacteristics,
    ///FV - Firing Values
    FiringValues,
    ///FZ - File Size
    FileSize,
    ///GC - Contractor Delivery Limitations
    ContractorDeliveryLimitations,
    ///GL - Guidelines
    Guidelines,
    ///GO - Government Ordering Limitations
    GovernmentOrderingLimitations,
    ///GP - Ordering Period Limitations
    OrderingPeriodLimitations,
    ///HC - High Credit Average
    HighCreditAverage,
    ///HR - Historical Result
    HistoricalResult,
    ///ID - Industrial
    Industrial,
    ///IG - Ingredient
    Ingredient,
    ///IN - Incrustation
    Incrustation,
    ///IR - Interpolated Result
    InterpolatedResult,
    ///LC - Limited Weight/Size Coils
    LimitedWeightSizeCoils,
    ///LD - Load Planning Dimensions
    LoadPlanningDimensions,
    ///LG - Loans Granted
    LoansGranted,
    ///LL - Lift Limitation
    LiftLimitation,
    ///LM - Layer of Multiple Layered Product
    LayerOfMultipleLayeredProduct,
    ///LP - Last Sold Period
    LastSoldPeriod,
    ///LS - Lot Status
    LotStatus,
    ///LT - Lot Limits
    LotLimits,
    ///MA - Accuracy
    Accuracy,
    ///MB - Activity Period Result
    ActivityPeriodResult,
    ///MC - Average Daily Limit
    AverageDailyLimit,
    ///MD - Design Capacity
    DesignCapacity,
    ///ME - Map Scale
    MapScale,
    ///MF - Maximum Daily Limit
    MaximumDailyLimit,
    ///MI - Minerals
    Minerals,
    ///MP - Maturity Period
    MaturityPeriod,
    ///MR - Base Material Result
    BaseMaterialResult,
    ///NC - Net Change
    NetChange,
    ///NE - Neighborhood
    Neighborhood,
    ///NS - North American Industrial Classification System (NAICS) Size Standard
    CodeNS,
    ///NU - Nutritional
    Nutritional,
    ///NX - Net Explosive Weight
    NetExplosiveWeight,
    ///OD - Ordered Dimensions
    OrderedDimensions,
    ///OG - Original
    Original,
    ///OL - Order Limits
    OrderLimits,
    ///OP - Other Property
    OtherProperty,
    ///P1 - Platform Limitation
    PlatformLimitation,
    ///PA - Pallet Dimensions
    PalletDimensions,
    ///PB - Receivership Period
    ReceivershipPeriod,
    ///PC - Parting Cut (Sawcut)
    CodePC,
    ///PD - Physical Dimensions
    PhysicalDimensions,
    ///PI - Project Incomplete
    ProjectIncomplete,
    ///PJ - Project Complete
    ProjectComplete,
    ///PK - Package Dimensions
    PackageDimensions,
    ///PL - Package Limitations
    PackageLimitations,
    ///PM - Permitted
    Permitted,
    ///PO - Position
    Position,
    ///PR - Product Dimension Range Price Bracket
    ProductDimensionRangePriceBracket,
    ///PS - Product Characteristic Specification
    ProductCharacteristicSpecification,
    ///PT - Pretest Period
    PretestPeriod,
    ///PU - Planned Urban Development
    PlannedUrbanDevelopment,
    ///PY - Property
    Property,
    ///QR - Quarterly Result
    QuarterlyResult,
    ///QV - Quantity Variation
    QuantityVariation,
    ///R1 - Opening Reading
    OpeningReading,
    ///R2 - Closing Reading
    ClosingReading,
    ///RA - Reject Amount
    RejectAmount,
    ///RB - Repair Size
    RepairSize,
    ///RG - Regulatory Limit
    RegulatoryLimit,
    ///RL - Receiving Facility Limitations
    ReceivingFacilityLimitations,
    ///RN - Lengths Limitation
    LengthsLimitation,
    ///RO - Roll Limits
    RollLimits,
    ///RP - Relative Position
    RelativePosition,
    ///RQ - Requested
    Requested,
    ///RS - Response Time
    ResponseTime,
    ///RT - Replacement
    Replacement,
    ///SA - Spacing/Margin
    SpacingMargin,
    ///SB - Single Family
    SingleFamily,
    ///SC - Standard Industrial Classification (SIC) Code Size Standards
    CodeSC,
    ///SD - Shipped Dimensions
    ShippedDimensions,
    ///SE - Property Specifications
    PropertySpecifications,
    ///SF - Shelf Life
    ShelfLife,
    ///SG - Serged Physical Dimensions
    SergedPhysicalDimensions,
    ///SH - Shipping Tolerance
    ShippingTolerance,
    ///SI - Selling Dimensions
    SellingDimensions,
    ///SJ - Subject Phase
    SubjectPhase,
    ///SK - Skid Dimensions
    SkidDimensions,
    ///SL - Skid Limitations
    SkidLimitations,
    ///SM - Shade
    Shade,
    ///SP - Splices
    Splices,
    ///SR - Surface Roughness
    SurfaceRoughness,
    ///SS - Serving Size
    ServingSize,
    ///ST - Surface Treatment
    SurfaceTreatment,
    ///SU - Surface
    Surface,
    ///SZ - Subject Property
    SubjectProperty,
    ///TA - Two to Four Family
    TwoToFourFamily,
    ///TD - Splice Tape Dimensions
    SpliceTapeDimensions,
    ///TE - Temperature
    Temperature,
    ///TI - Time
    Time,
    ///TL - Transportation Equipment Limitations
    TransportationEquipmentLimitations,
    ///TO - Total Dimensions
    TotalDimensions,
    ///TP - Test Period
    TestPeriod,
    ///TR - Test Results
    TestResults,
    ///TS - Single Test Limits
    SingleTestLimits,
    ///TT - This Type Property
    ThisTypeProperty,
    ///VD - Variant Days
    VariantDays,
    ///VI - Vitamin
    Vitamin,
    ///VT - Vacant
    Vacant,
    ///WA - Waste Amount
    WasteAmount,
    ///WR - Warranty
    Warranty,
    ///WT - Weights
    Weights,
    ///ZA - Multi - Family
    MultiFamily,
    ///ZB - Allergen
    Allergen,
    ///ZP - Log Zero Point of Reference
    LogZeroPointOfReference,
}
impl MeasurementReferenceIdCode {
    pub fn code(&self) -> &str {
        {
            use MeasurementReferenceIdCode::*;
            match self {
                NinthLayer => "9L",
                MeterReadingBeginningActualEndingActual => "AA",
                AverageBalance => "AB",
                MaximumBalance => "AC",
                ServingSpecifications => "AD",
                MeterReadingBeginningActualEndingEstimated => "AE",
                ActualTotal => "AF",
                ComplianceTotal => "AG",
                GrossComplianceTotal => "AH",
                NetComplianceTotal => "AI",
                Effectiveness => "AJ",
                Penetration => "AK",
                Exposure => "AL",
                Capacity => "AM",
                Work => "AN",
                AccountOpenedPeriod => "AO",
                Apartments => "AP",
                AmountOwes => "AQ",
                AnnualResult => "AR",
                ModifiedExposure => "AS",
                ProRataFactor => "AT",
                Sample => "AU",
                AverageReading => "AV",
                RelativeHumidity => "AW",
                Alcohol => "AX",
                BasePoint => "BA",
                BankBalance => "BB",
                BilledActual => "BC",
                BilledDimensions => "BD",
                BundleLimitation => "BL",
                BaseMetal => "BM",
                BilledMinimum => "BN",
                MeterReadingAsBilled => "BO",
                BearingPieceLimitation => "BP",
                BilledHistory => "BR",
                BatchLimits => "BT",
                BattenSize => "BZ",
                ConformancePropertyStatus => "C1",
                Coating => "CA",
                Condominium => "CB",
                Cooperative => "CC",
                ConversionFactor => "CF",
                AverageOwing => "CG",
                Chemistry => "CH",
                CurrentBalance => "CJ",
                DiscountedChecks => "CK",
                Drafts => "CL",
                Commercial => "CM",
                CoreNotchDimensions => "CN",
                Concentration => "CO",
                LetterOfCreditSize => "CP",
                PaymentOrders => "CQ",
                CoreSize => "CS",
                Counts => "CT",
                PledgeSize => "CU",
                CumulativeTestPeriod => "CV",
                PromissoryNotesInForce => "CW",
                SecuredAmount => "CY",
                Defects => "DE",
                DunnageDimension => "DN",
                PerDrop => "DR",
                DefectSize => "DS",
                DimensionalTolerance => "DT",
                MeterReadingBeginningEstimatedEndingActual => "EA",
                MeterReadingBeginningEstimatedEndingEstimated => "EE",
                EvaluationFactors => "EF",
                ElectricalCharacteristics => "EL",
                EnvironmentalConditions => "EN",
                FirstOfCampaignResult => "FC",
                FinishedDimensions => "FD",
                RadioOperations => "FH",
                AntennaCharacteristics => "FJ",
                FiringValues => "FV",
                FileSize => "FZ",
                ContractorDeliveryLimitations => "GC",
                Guidelines => "GL",
                GovernmentOrderingLimitations => "GO",
                OrderingPeriodLimitations => "GP",
                HighCreditAverage => "HC",
                HistoricalResult => "HR",
                Industrial => "ID",
                Ingredient => "IG",
                Incrustation => "IN",
                InterpolatedResult => "IR",
                LimitedWeightSizeCoils => "LC",
                LoadPlanningDimensions => "LD",
                LoansGranted => "LG",
                LiftLimitation => "LL",
                LayerOfMultipleLayeredProduct => "LM",
                LastSoldPeriod => "LP",
                LotStatus => "LS",
                LotLimits => "LT",
                Accuracy => "MA",
                ActivityPeriodResult => "MB",
                AverageDailyLimit => "MC",
                DesignCapacity => "MD",
                MapScale => "ME",
                MaximumDailyLimit => "MF",
                Minerals => "MI",
                MaturityPeriod => "MP",
                BaseMaterialResult => "MR",
                NetChange => "NC",
                Neighborhood => "NE",
                CodeNS => "NS",
                Nutritional => "NU",
                NetExplosiveWeight => "NX",
                OrderedDimensions => "OD",
                Original => "OG",
                OrderLimits => "OL",
                OtherProperty => "OP",
                PlatformLimitation => "P1",
                PalletDimensions => "PA",
                ReceivershipPeriod => "PB",
                CodePC => "PC",
                PhysicalDimensions => "PD",
                ProjectIncomplete => "PI",
                ProjectComplete => "PJ",
                PackageDimensions => "PK",
                PackageLimitations => "PL",
                Permitted => "PM",
                Position => "PO",
                ProductDimensionRangePriceBracket => "PR",
                ProductCharacteristicSpecification => "PS",
                PretestPeriod => "PT",
                PlannedUrbanDevelopment => "PU",
                Property => "PY",
                QuarterlyResult => "QR",
                QuantityVariation => "QV",
                OpeningReading => "R1",
                ClosingReading => "R2",
                RejectAmount => "RA",
                RepairSize => "RB",
                RegulatoryLimit => "RG",
                ReceivingFacilityLimitations => "RL",
                LengthsLimitation => "RN",
                RollLimits => "RO",
                RelativePosition => "RP",
                Requested => "RQ",
                ResponseTime => "RS",
                Replacement => "RT",
                SpacingMargin => "SA",
                SingleFamily => "SB",
                CodeSC => "SC",
                ShippedDimensions => "SD",
                PropertySpecifications => "SE",
                ShelfLife => "SF",
                SergedPhysicalDimensions => "SG",
                ShippingTolerance => "SH",
                SellingDimensions => "SI",
                SubjectPhase => "SJ",
                SkidDimensions => "SK",
                SkidLimitations => "SL",
                Shade => "SM",
                Splices => "SP",
                SurfaceRoughness => "SR",
                ServingSize => "SS",
                SurfaceTreatment => "ST",
                Surface => "SU",
                SubjectProperty => "SZ",
                TwoToFourFamily => "TA",
                SpliceTapeDimensions => "TD",
                Temperature => "TE",
                Time => "TI",
                TransportationEquipmentLimitations => "TL",
                TotalDimensions => "TO",
                TestPeriod => "TP",
                TestResults => "TR",
                SingleTestLimits => "TS",
                ThisTypeProperty => "TT",
                VariantDays => "VD",
                Vitamin => "VI",
                Vacant => "VT",
                WasteAmount => "WA",
                Warranty => "WR",
                Weights => "WT",
                MultiFamily => "ZA",
                Allergen => "ZB",
                LogZeroPointOfReference => "ZP",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<MeasurementReferenceIdCode> {
        use MeasurementReferenceIdCode::*;
        match code {
            b"9L" => Some(NinthLayer),
            b"AA" => Some(MeterReadingBeginningActualEndingActual),
            b"AB" => Some(AverageBalance),
            b"AC" => Some(MaximumBalance),
            b"AD" => Some(ServingSpecifications),
            b"AE" => Some(MeterReadingBeginningActualEndingEstimated),
            b"AF" => Some(ActualTotal),
            b"AG" => Some(ComplianceTotal),
            b"AH" => Some(GrossComplianceTotal),
            b"AI" => Some(NetComplianceTotal),
            b"AJ" => Some(Effectiveness),
            b"AK" => Some(Penetration),
            b"AL" => Some(Exposure),
            b"AM" => Some(Capacity),
            b"AN" => Some(Work),
            b"AO" => Some(AccountOpenedPeriod),
            b"AP" => Some(Apartments),
            b"AQ" => Some(AmountOwes),
            b"AR" => Some(AnnualResult),
            b"AS" => Some(ModifiedExposure),
            b"AT" => Some(ProRataFactor),
            b"AU" => Some(Sample),
            b"AV" => Some(AverageReading),
            b"AW" => Some(RelativeHumidity),
            b"AX" => Some(Alcohol),
            b"BA" => Some(BasePoint),
            b"BB" => Some(BankBalance),
            b"BC" => Some(BilledActual),
            b"BD" => Some(BilledDimensions),
            b"BL" => Some(BundleLimitation),
            b"BM" => Some(BaseMetal),
            b"BN" => Some(BilledMinimum),
            b"BO" => Some(MeterReadingAsBilled),
            b"BP" => Some(BearingPieceLimitation),
            b"BR" => Some(BilledHistory),
            b"BT" => Some(BatchLimits),
            b"BZ" => Some(BattenSize),
            b"C1" => Some(ConformancePropertyStatus),
            b"CA" => Some(Coating),
            b"CB" => Some(Condominium),
            b"CC" => Some(Cooperative),
            b"CF" => Some(ConversionFactor),
            b"CG" => Some(AverageOwing),
            b"CH" => Some(Chemistry),
            b"CJ" => Some(CurrentBalance),
            b"CK" => Some(DiscountedChecks),
            b"CL" => Some(Drafts),
            b"CM" => Some(Commercial),
            b"CN" => Some(CoreNotchDimensions),
            b"CO" => Some(Concentration),
            b"CP" => Some(LetterOfCreditSize),
            b"CQ" => Some(PaymentOrders),
            b"CS" => Some(CoreSize),
            b"CT" => Some(Counts),
            b"CU" => Some(PledgeSize),
            b"CV" => Some(CumulativeTestPeriod),
            b"CW" => Some(PromissoryNotesInForce),
            b"CY" => Some(SecuredAmount),
            b"DE" => Some(Defects),
            b"DN" => Some(DunnageDimension),
            b"DR" => Some(PerDrop),
            b"DS" => Some(DefectSize),
            b"DT" => Some(DimensionalTolerance),
            b"EA" => Some(MeterReadingBeginningEstimatedEndingActual),
            b"EE" => Some(MeterReadingBeginningEstimatedEndingEstimated),
            b"EF" => Some(EvaluationFactors),
            b"EL" => Some(ElectricalCharacteristics),
            b"EN" => Some(EnvironmentalConditions),
            b"FC" => Some(FirstOfCampaignResult),
            b"FD" => Some(FinishedDimensions),
            b"FH" => Some(RadioOperations),
            b"FJ" => Some(AntennaCharacteristics),
            b"FV" => Some(FiringValues),
            b"FZ" => Some(FileSize),
            b"GC" => Some(ContractorDeliveryLimitations),
            b"GL" => Some(Guidelines),
            b"GO" => Some(GovernmentOrderingLimitations),
            b"GP" => Some(OrderingPeriodLimitations),
            b"HC" => Some(HighCreditAverage),
            b"HR" => Some(HistoricalResult),
            b"ID" => Some(Industrial),
            b"IG" => Some(Ingredient),
            b"IN" => Some(Incrustation),
            b"IR" => Some(InterpolatedResult),
            b"LC" => Some(LimitedWeightSizeCoils),
            b"LD" => Some(LoadPlanningDimensions),
            b"LG" => Some(LoansGranted),
            b"LL" => Some(LiftLimitation),
            b"LM" => Some(LayerOfMultipleLayeredProduct),
            b"LP" => Some(LastSoldPeriod),
            b"LS" => Some(LotStatus),
            b"LT" => Some(LotLimits),
            b"MA" => Some(Accuracy),
            b"MB" => Some(ActivityPeriodResult),
            b"MC" => Some(AverageDailyLimit),
            b"MD" => Some(DesignCapacity),
            b"ME" => Some(MapScale),
            b"MF" => Some(MaximumDailyLimit),
            b"MI" => Some(Minerals),
            b"MP" => Some(MaturityPeriod),
            b"MR" => Some(BaseMaterialResult),
            b"NC" => Some(NetChange),
            b"NE" => Some(Neighborhood),
            b"NS" => Some(CodeNS),
            b"NU" => Some(Nutritional),
            b"NX" => Some(NetExplosiveWeight),
            b"OD" => Some(OrderedDimensions),
            b"OG" => Some(Original),
            b"OL" => Some(OrderLimits),
            b"OP" => Some(OtherProperty),
            b"P1" => Some(PlatformLimitation),
            b"PA" => Some(PalletDimensions),
            b"PB" => Some(ReceivershipPeriod),
            b"PC" => Some(CodePC),
            b"PD" => Some(PhysicalDimensions),
            b"PI" => Some(ProjectIncomplete),
            b"PJ" => Some(ProjectComplete),
            b"PK" => Some(PackageDimensions),
            b"PL" => Some(PackageLimitations),
            b"PM" => Some(Permitted),
            b"PO" => Some(Position),
            b"PR" => Some(ProductDimensionRangePriceBracket),
            b"PS" => Some(ProductCharacteristicSpecification),
            b"PT" => Some(PretestPeriod),
            b"PU" => Some(PlannedUrbanDevelopment),
            b"PY" => Some(Property),
            b"QR" => Some(QuarterlyResult),
            b"QV" => Some(QuantityVariation),
            b"R1" => Some(OpeningReading),
            b"R2" => Some(ClosingReading),
            b"RA" => Some(RejectAmount),
            b"RB" => Some(RepairSize),
            b"RG" => Some(RegulatoryLimit),
            b"RL" => Some(ReceivingFacilityLimitations),
            b"RN" => Some(LengthsLimitation),
            b"RO" => Some(RollLimits),
            b"RP" => Some(RelativePosition),
            b"RQ" => Some(Requested),
            b"RS" => Some(ResponseTime),
            b"RT" => Some(Replacement),
            b"SA" => Some(SpacingMargin),
            b"SB" => Some(SingleFamily),
            b"SC" => Some(CodeSC),
            b"SD" => Some(ShippedDimensions),
            b"SE" => Some(PropertySpecifications),
            b"SF" => Some(ShelfLife),
            b"SG" => Some(SergedPhysicalDimensions),
            b"SH" => Some(ShippingTolerance),
            b"SI" => Some(SellingDimensions),
            b"SJ" => Some(SubjectPhase),
            b"SK" => Some(SkidDimensions),
            b"SL" => Some(SkidLimitations),
            b"SM" => Some(Shade),
            b"SP" => Some(Splices),
            b"SR" => Some(SurfaceRoughness),
            b"SS" => Some(ServingSize),
            b"ST" => Some(SurfaceTreatment),
            b"SU" => Some(Surface),
            b"SZ" => Some(SubjectProperty),
            b"TA" => Some(TwoToFourFamily),
            b"TD" => Some(SpliceTapeDimensions),
            b"TE" => Some(Temperature),
            b"TI" => Some(Time),
            b"TL" => Some(TransportationEquipmentLimitations),
            b"TO" => Some(TotalDimensions),
            b"TP" => Some(TestPeriod),
            b"TR" => Some(TestResults),
            b"TS" => Some(SingleTestLimits),
            b"TT" => Some(ThisTypeProperty),
            b"VD" => Some(VariantDays),
            b"VI" => Some(Vitamin),
            b"VT" => Some(Vacant),
            b"WA" => Some(WasteAmount),
            b"WR" => Some(Warranty),
            b"WT" => Some(Weights),
            b"ZA" => Some(MultiFamily),
            b"ZB" => Some(Allergen),
            b"ZP" => Some(LogZeroPointOfReference),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use MeasurementReferenceIdCode::*;
        match self {
            NinthLayer => "Ninth Layer",
            MeterReadingBeginningActualEndingActual => {
                "Meter reading-beginning actual/ending actual"
            }
            AverageBalance => "Average Balance",
            MaximumBalance => "Maximum Balance",
            ServingSpecifications => "Serving Specifications",
            MeterReadingBeginningActualEndingEstimated => {
                "Meter reading-beginning actual/ending estimated"
            }
            ActualTotal => "Actual Total",
            ComplianceTotal => "Compliance Total",
            GrossComplianceTotal => "Gross Compliance Total",
            NetComplianceTotal => "Net Compliance Total",
            Effectiveness => "Effectiveness",
            Penetration => "Penetration",
            Exposure => "Exposure",
            Capacity => "Capacity",
            Work => "Work",
            AccountOpenedPeriod => "Account Opened Period",
            Apartments => "Apartments",
            AmountOwes => "Amount Owes",
            AnnualResult => "Annual Result",
            ModifiedExposure => "Modified Exposure",
            ProRataFactor => "Pro Rata Factor",
            Sample => "Sample",
            AverageReading => "Average Reading",
            RelativeHumidity => "Relative Humidity",
            Alcohol => "Alcohol",
            BasePoint => "Base Point",
            BankBalance => "Bank Balance",
            BilledActual => "Billed Actual",
            BilledDimensions => "Billed Dimensions",
            BundleLimitation => "Bundle Limitation",
            BaseMetal => "Base Metal",
            BilledMinimum => "Billed Minimum",
            MeterReadingAsBilled => "Meter Reading as Billed",
            BearingPieceLimitation => "Bearing Piece Limitation",
            BilledHistory => "Billed History",
            BatchLimits => "Batch Limits",
            BattenSize => "Batten Size",
            ConformancePropertyStatus => "Conformance Property Status",
            Coating => "Coating",
            Condominium => "Condominium",
            Cooperative => "Cooperative",
            ConversionFactor => "Conversion Factor",
            AverageOwing => "Average Owing",
            Chemistry => "Chemistry",
            CurrentBalance => "Current Balance",
            DiscountedChecks => "Discounted Checks",
            Drafts => "Drafts",
            Commercial => "Commercial",
            CoreNotchDimensions => "Core Notch Dimensions",
            Concentration => "Concentration",
            LetterOfCreditSize => "Letter of Credit Size",
            PaymentOrders => "Payment Orders",
            CoreSize => "Core Size",
            Counts => "Counts",
            PledgeSize => "Pledge Size",
            CumulativeTestPeriod => "Cumulative Test Period",
            PromissoryNotesInForce => "Promissory Notes in Force",
            SecuredAmount => "Secured Amount",
            Defects => "Defects",
            DunnageDimension => "Dunnage Dimension",
            PerDrop => "Per Drop",
            DefectSize => "Defect Size",
            DimensionalTolerance => "Dimensional Tolerance",
            MeterReadingBeginningEstimatedEndingActual => {
                "Meter reading-beginning estimated/ending actual"
            }
            MeterReadingBeginningEstimatedEndingEstimated => {
                "Meter reading-beginning estimated/ending estimated"
            }
            EvaluationFactors => "Evaluation Factors",
            ElectricalCharacteristics => "Electrical Characteristics",
            EnvironmentalConditions => "Environmental Conditions",
            FirstOfCampaignResult => "First of Campaign Result",
            FinishedDimensions => "Finished Dimensions",
            RadioOperations => "Radio Operations",
            AntennaCharacteristics => "Antenna Characteristics",
            FiringValues => "Firing Values",
            FileSize => "File Size",
            ContractorDeliveryLimitations => "Contractor Delivery Limitations",
            Guidelines => "Guidelines",
            GovernmentOrderingLimitations => "Government Ordering Limitations",
            OrderingPeriodLimitations => "Ordering Period Limitations",
            HighCreditAverage => "High Credit Average",
            HistoricalResult => "Historical Result",
            Industrial => "Industrial",
            Ingredient => "Ingredient",
            Incrustation => "Incrustation",
            InterpolatedResult => "Interpolated Result",
            LimitedWeightSizeCoils => "Limited Weight/Size Coils",
            LoadPlanningDimensions => "Load Planning Dimensions",
            LoansGranted => "Loans Granted",
            LiftLimitation => "Lift Limitation",
            LayerOfMultipleLayeredProduct => "Layer of Multiple Layered Product",
            LastSoldPeriod => "Last Sold Period",
            LotStatus => "Lot Status",
            LotLimits => "Lot Limits",
            Accuracy => "Accuracy",
            ActivityPeriodResult => "Activity Period Result",
            AverageDailyLimit => "Average Daily Limit",
            DesignCapacity => "Design Capacity",
            MapScale => "Map Scale",
            MaximumDailyLimit => "Maximum Daily Limit",
            Minerals => "Minerals",
            MaturityPeriod => "Maturity Period",
            BaseMaterialResult => "Base Material Result",
            NetChange => "Net Change",
            Neighborhood => "Neighborhood",
            CodeNS => {
                "North American Industrial Classification System (NAICS) Size Standard"
            }
            Nutritional => "Nutritional",
            NetExplosiveWeight => "Net Explosive Weight",
            OrderedDimensions => "Ordered Dimensions",
            Original => "Original",
            OrderLimits => "Order Limits",
            OtherProperty => "Other Property",
            PlatformLimitation => "Platform Limitation",
            PalletDimensions => "Pallet Dimensions",
            ReceivershipPeriod => "Receivership Period",
            CodePC => "Parting Cut (Sawcut)",
            PhysicalDimensions => "Physical Dimensions",
            ProjectIncomplete => "Project Incomplete",
            ProjectComplete => "Project Complete",
            PackageDimensions => "Package Dimensions",
            PackageLimitations => "Package Limitations",
            Permitted => "Permitted",
            Position => "Position",
            ProductDimensionRangePriceBracket => "Product Dimension Range Price Bracket",
            ProductCharacteristicSpecification => "Product Characteristic Specification",
            PretestPeriod => "Pretest Period",
            PlannedUrbanDevelopment => "Planned Urban Development",
            Property => "Property",
            QuarterlyResult => "Quarterly Result",
            QuantityVariation => "Quantity Variation",
            OpeningReading => "Opening Reading",
            ClosingReading => "Closing Reading",
            RejectAmount => "Reject Amount",
            RepairSize => "Repair Size",
            RegulatoryLimit => "Regulatory Limit",
            ReceivingFacilityLimitations => "Receiving Facility Limitations",
            LengthsLimitation => "Lengths Limitation",
            RollLimits => "Roll Limits",
            RelativePosition => "Relative Position",
            Requested => "Requested",
            ResponseTime => "Response Time",
            Replacement => "Replacement",
            SpacingMargin => "Spacing/Margin",
            SingleFamily => "Single Family",
            CodeSC => "Standard Industrial Classification (SIC) Code Size Standards",
            ShippedDimensions => "Shipped Dimensions",
            PropertySpecifications => "Property Specifications",
            ShelfLife => "Shelf Life",
            SergedPhysicalDimensions => "Serged Physical Dimensions",
            ShippingTolerance => "Shipping Tolerance",
            SellingDimensions => "Selling Dimensions",
            SubjectPhase => "Subject Phase",
            SkidDimensions => "Skid Dimensions",
            SkidLimitations => "Skid Limitations",
            Shade => "Shade",
            Splices => "Splices",
            SurfaceRoughness => "Surface Roughness",
            ServingSize => "Serving Size",
            SurfaceTreatment => "Surface Treatment",
            Surface => "Surface",
            SubjectProperty => "Subject Property",
            TwoToFourFamily => "Two to Four Family",
            SpliceTapeDimensions => "Splice Tape Dimensions",
            Temperature => "Temperature",
            Time => "Time",
            TransportationEquipmentLimitations => "Transportation Equipment Limitations",
            TotalDimensions => "Total Dimensions",
            TestPeriod => "Test Period",
            TestResults => "Test Results",
            SingleTestLimits => "Single Test Limits",
            ThisTypeProperty => "This Type Property",
            VariantDays => "Variant Days",
            Vitamin => "Vitamin",
            Vacant => "Vacant",
            WasteAmount => "Waste Amount",
            Warranty => "Warranty",
            Weights => "Weights",
            MultiFamily => "Multi - Family",
            Allergen => "Allergen",
            LogZeroPointOfReference => "Log Zero Point of Reference",
        }
    }
    fn from_description(description: &str) -> Option<MeasurementReferenceIdCode> {
        {
            use MeasurementReferenceIdCode::*;
            match description {
                "Ninth Layer" => Some(NinthLayer),
                "Meter reading-beginning actual/ending actual" => {
                    Some(MeterReadingBeginningActualEndingActual)
                }
                "Average Balance" => Some(AverageBalance),
                "Maximum Balance" => Some(MaximumBalance),
                "Serving Specifications" => Some(ServingSpecifications),
                "Meter reading-beginning actual/ending estimated" => {
                    Some(MeterReadingBeginningActualEndingEstimated)
                }
                "Actual Total" => Some(ActualTotal),
                "Compliance Total" => Some(ComplianceTotal),
                "Gross Compliance Total" => Some(GrossComplianceTotal),
                "Net Compliance Total" => Some(NetComplianceTotal),
                "Effectiveness" => Some(Effectiveness),
                "Penetration" => Some(Penetration),
                "Exposure" => Some(Exposure),
                "Capacity" => Some(Capacity),
                "Work" => Some(Work),
                "Account Opened Period" => Some(AccountOpenedPeriod),
                "Apartments" => Some(Apartments),
                "Amount Owes" => Some(AmountOwes),
                "Annual Result" => Some(AnnualResult),
                "Modified Exposure" => Some(ModifiedExposure),
                "Pro Rata Factor" => Some(ProRataFactor),
                "Sample" => Some(Sample),
                "Average Reading" => Some(AverageReading),
                "Relative Humidity" => Some(RelativeHumidity),
                "Alcohol" => Some(Alcohol),
                "Base Point" => Some(BasePoint),
                "Bank Balance" => Some(BankBalance),
                "Billed Actual" => Some(BilledActual),
                "Billed Dimensions" => Some(BilledDimensions),
                "Bundle Limitation" => Some(BundleLimitation),
                "Base Metal" => Some(BaseMetal),
                "Billed Minimum" => Some(BilledMinimum),
                "Meter Reading as Billed" => Some(MeterReadingAsBilled),
                "Bearing Piece Limitation" => Some(BearingPieceLimitation),
                "Billed History" => Some(BilledHistory),
                "Batch Limits" => Some(BatchLimits),
                "Batten Size" => Some(BattenSize),
                "Conformance Property Status" => Some(ConformancePropertyStatus),
                "Coating" => Some(Coating),
                "Condominium" => Some(Condominium),
                "Cooperative" => Some(Cooperative),
                "Conversion Factor" => Some(ConversionFactor),
                "Average Owing" => Some(AverageOwing),
                "Chemistry" => Some(Chemistry),
                "Current Balance" => Some(CurrentBalance),
                "Discounted Checks" => Some(DiscountedChecks),
                "Drafts" => Some(Drafts),
                "Commercial" => Some(Commercial),
                "Core Notch Dimensions" => Some(CoreNotchDimensions),
                "Concentration" => Some(Concentration),
                "Letter of Credit Size" => Some(LetterOfCreditSize),
                "Payment Orders" => Some(PaymentOrders),
                "Core Size" => Some(CoreSize),
                "Counts" => Some(Counts),
                "Pledge Size" => Some(PledgeSize),
                "Cumulative Test Period" => Some(CumulativeTestPeriod),
                "Promissory Notes in Force" => Some(PromissoryNotesInForce),
                "Secured Amount" => Some(SecuredAmount),
                "Defects" => Some(Defects),
                "Dunnage Dimension" => Some(DunnageDimension),
                "Per Drop" => Some(PerDrop),
                "Defect Size" => Some(DefectSize),
                "Dimensional Tolerance" => Some(DimensionalTolerance),
                "Meter reading-beginning estimated/ending actual" => {
                    Some(MeterReadingBeginningEstimatedEndingActual)
                }
                "Meter reading-beginning estimated/ending estimated" => {
                    Some(MeterReadingBeginningEstimatedEndingEstimated)
                }
                "Evaluation Factors" => Some(EvaluationFactors),
                "Electrical Characteristics" => Some(ElectricalCharacteristics),
                "Environmental Conditions" => Some(EnvironmentalConditions),
                "First of Campaign Result" => Some(FirstOfCampaignResult),
                "Finished Dimensions" => Some(FinishedDimensions),
                "Radio Operations" => Some(RadioOperations),
                "Antenna Characteristics" => Some(AntennaCharacteristics),
                "Firing Values" => Some(FiringValues),
                "File Size" => Some(FileSize),
                "Contractor Delivery Limitations" => Some(ContractorDeliveryLimitations),
                "Guidelines" => Some(Guidelines),
                "Government Ordering Limitations" => Some(GovernmentOrderingLimitations),
                "Ordering Period Limitations" => Some(OrderingPeriodLimitations),
                "High Credit Average" => Some(HighCreditAverage),
                "Historical Result" => Some(HistoricalResult),
                "Industrial" => Some(Industrial),
                "Ingredient" => Some(Ingredient),
                "Incrustation" => Some(Incrustation),
                "Interpolated Result" => Some(InterpolatedResult),
                "Limited Weight/Size Coils" => Some(LimitedWeightSizeCoils),
                "Load Planning Dimensions" => Some(LoadPlanningDimensions),
                "Loans Granted" => Some(LoansGranted),
                "Lift Limitation" => Some(LiftLimitation),
                "Layer of Multiple Layered Product" => {
                    Some(LayerOfMultipleLayeredProduct)
                }
                "Last Sold Period" => Some(LastSoldPeriod),
                "Lot Status" => Some(LotStatus),
                "Lot Limits" => Some(LotLimits),
                "Accuracy" => Some(Accuracy),
                "Activity Period Result" => Some(ActivityPeriodResult),
                "Average Daily Limit" => Some(AverageDailyLimit),
                "Design Capacity" => Some(DesignCapacity),
                "Map Scale" => Some(MapScale),
                "Maximum Daily Limit" => Some(MaximumDailyLimit),
                "Minerals" => Some(Minerals),
                "Maturity Period" => Some(MaturityPeriod),
                "Base Material Result" => Some(BaseMaterialResult),
                "Net Change" => Some(NetChange),
                "Neighborhood" => Some(Neighborhood),
                "North American Industrial Classification System (NAICS) Size Standard" => {
                    Some(CodeNS)
                }
                "Nutritional" => Some(Nutritional),
                "Net Explosive Weight" => Some(NetExplosiveWeight),
                "Ordered Dimensions" => Some(OrderedDimensions),
                "Original" => Some(Original),
                "Order Limits" => Some(OrderLimits),
                "Other Property" => Some(OtherProperty),
                "Platform Limitation" => Some(PlatformLimitation),
                "Pallet Dimensions" => Some(PalletDimensions),
                "Receivership Period" => Some(ReceivershipPeriod),
                "Parting Cut (Sawcut)" => Some(CodePC),
                "Physical Dimensions" => Some(PhysicalDimensions),
                "Project Incomplete" => Some(ProjectIncomplete),
                "Project Complete" => Some(ProjectComplete),
                "Package Dimensions" => Some(PackageDimensions),
                "Package Limitations" => Some(PackageLimitations),
                "Permitted" => Some(Permitted),
                "Position" => Some(Position),
                "Product Dimension Range Price Bracket" => {
                    Some(ProductDimensionRangePriceBracket)
                }
                "Product Characteristic Specification" => {
                    Some(ProductCharacteristicSpecification)
                }
                "Pretest Period" => Some(PretestPeriod),
                "Planned Urban Development" => Some(PlannedUrbanDevelopment),
                "Property" => Some(Property),
                "Quarterly Result" => Some(QuarterlyResult),
                "Quantity Variation" => Some(QuantityVariation),
                "Opening Reading" => Some(OpeningReading),
                "Closing Reading" => Some(ClosingReading),
                "Reject Amount" => Some(RejectAmount),
                "Repair Size" => Some(RepairSize),
                "Regulatory Limit" => Some(RegulatoryLimit),
                "Receiving Facility Limitations" => Some(ReceivingFacilityLimitations),
                "Lengths Limitation" => Some(LengthsLimitation),
                "Roll Limits" => Some(RollLimits),
                "Relative Position" => Some(RelativePosition),
                "Requested" => Some(Requested),
                "Response Time" => Some(ResponseTime),
                "Replacement" => Some(Replacement),
                "Spacing/Margin" => Some(SpacingMargin),
                "Single Family" => Some(SingleFamily),
                "Standard Industrial Classification (SIC) Code Size Standards" => {
                    Some(CodeSC)
                }
                "Shipped Dimensions" => Some(ShippedDimensions),
                "Property Specifications" => Some(PropertySpecifications),
                "Shelf Life" => Some(ShelfLife),
                "Serged Physical Dimensions" => Some(SergedPhysicalDimensions),
                "Shipping Tolerance" => Some(ShippingTolerance),
                "Selling Dimensions" => Some(SellingDimensions),
                "Subject Phase" => Some(SubjectPhase),
                "Skid Dimensions" => Some(SkidDimensions),
                "Skid Limitations" => Some(SkidLimitations),
                "Shade" => Some(Shade),
                "Splices" => Some(Splices),
                "Surface Roughness" => Some(SurfaceRoughness),
                "Serving Size" => Some(ServingSize),
                "Surface Treatment" => Some(SurfaceTreatment),
                "Surface" => Some(Surface),
                "Subject Property" => Some(SubjectProperty),
                "Two to Four Family" => Some(TwoToFourFamily),
                "Splice Tape Dimensions" => Some(SpliceTapeDimensions),
                "Temperature" => Some(Temperature),
                "Time" => Some(Time),
                "Transportation Equipment Limitations" => {
                    Some(TransportationEquipmentLimitations)
                }
                "Total Dimensions" => Some(TotalDimensions),
                "Test Period" => Some(TestPeriod),
                "Test Results" => Some(TestResults),
                "Single Test Limits" => Some(SingleTestLimits),
                "This Type Property" => Some(ThisTypeProperty),
                "Variant Days" => Some(VariantDays),
                "Vitamin" => Some(Vitamin),
                "Vacant" => Some(Vacant),
                "Waste Amount" => Some(WasteAmount),
                "Warranty" => Some(Warranty),
                "Weights" => Some(Weights),
                "Multi - Family" => Some(MultiFamily),
                "Allergen" => Some(Allergen),
                "Log Zero Point of Reference" => Some(LogZeroPointOfReference),
                _ => None,
            }
        }
    }
}
impl Serialize for MeasurementReferenceIdCode {
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
    type Value = MeasurementReferenceIdCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Measurement Reference ID Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MeasurementReferenceIdCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Measurement Reference ID Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MeasurementReferenceIdCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Measurement Reference ID Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for MeasurementReferenceIdCode {
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