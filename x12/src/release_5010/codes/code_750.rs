use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**750

See docs at <https://www.stedi.com/edi/x12/element/750>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProductProcessCharacteristicCode {
    ///01 - Limiting Operation
    LimitingOperation,
    ///02 - General Product Form
    GeneralProductForm,
    ///03 - Preservative
    Preservative,
    ///04 - Parameter
    Parameter,
    ///05 - Category
    Category,
    ///06 - Sub-category
    SubCategory,
    ///08 - Product
    Product,
    ///09 - Sub-product
    SubProduct,
    ///9A - Manufacturing Method
    ManufacturingMethod,
    ///9B - Product Application
    ProductApplication,
    ///9C - Engine
    Engine,
    ///9D - Transmission
    Transmission,
    ///10 - Major Grade
    MajorGrade,
    ///11 - Society, Government, Customer Specifications
    Code11,
    ///12 - Type and/or Process
    TypeAndOrProcess,
    ///13 - Quality (Quality Level)
    Code13,
    ///14 - Finish or Surface Roughness
    FinishOrSurfaceRoughness,
    ///15 - Heat Treat/Anneal
    HeatTreatAnneal,
    ///16 - Temper
    Temper,
    ///17 - Coating
    Coating,
    ///18 - Surface Treatment, Chemical
    Code18,
    ///19 - Surface Treatment, Mechanical
    Code19,
    ///20 - Ends: Slitting, Splitting, Cutting
    Code20,
    ///21 - Forming
    Forming,
    ///22 - Edge Treatment
    EdgeTreatment,
    ///23 - Welds/Splices
    WeldsSplices,
    ///25 - End Treatment
    EndTreatment,
    ///28 - Test Sample Frequency
    TestSampleFrequency,
    ///29 - Test Sample Location
    TestSampleLocation,
    ///30 - Test Sample Direction
    TestSampleDirection,
    ///32 - Type of Test/Inspection
    TypeOfTestInspection,
    ///33 - Testing and Inspection Agencies
    TestingAndInspectionAgencies,
    ///34 - Filament
    Filament,
    ///35 - Color
    Color,
    ///36 - Denier
    Denier,
    ///37 - Fiber
    Fiber,
    ///38 - Grade
    Grade,
    ///39 - Luster
    Luster,
    ///40 - Shade
    Shade,
    ///41 - Tint
    Tint,
    ///42 - Tow
    Tow,
    ///43 - Twist
    Twist,
    ///44 - Bacteriology
    Bacteriology,
    ///45 - Whole Effluent Toxicity
    WholeEffluentToxicity,
    ///46 - Sediment Toxicity
    SedimentToxicity,
    ///54 - Section Profile
    SectionProfile,
    ///55 - Alloy
    Alloy,
    ///56 - Special Processing
    SpecialProcessing,
    ///58 - Winding Instructions
    WindingInstructions,
    ///59 - Surface Protection
    SurfaceProtection,
    ///60 - Machine Run
    MachineRun,
    ///61 - End Use Application
    EndUseApplication,
    ///62 - Corrosion Resistance
    CorrosionResistance,
    ///63 - Product Life Cycle
    ProductLifeCycle,
    ///64 - Package Integrity
    PackageIntegrity,
    ///65 - Visual
    Visual,
    ///66 - Electrical
    Electrical,
    ///67 - Functional Performance
    FunctionalPerformance,
    ///68 - Chemistry
    Chemistry,
    ///69 - Physical
    Physical,
    ///70 - Magnetic
    Magnetic,
    ///71 - Mechanical
    Mechanical,
    ///72 - Metallography
    Metallography,
    ///73 - Vendor color description
    VendorColorDescription,
    ///74 - Vendor size description
    VendorSizeDescription,
    ///75 - Buyer's Color Description
    BuyersColorDescription,
    ///76 - Dye Lot Description
    DyeLotDescription,
    ///77 - Finish Description
    FinishDescription,
    ///78 - Pattern Description
    PatternDescription,
    ///79 - Put-up Description
    PutUpDescription,
    ///80 - MILSPEC (Military Specification)
    Code80,
    ///81 - FEDSPEC (Federal Specification)
    Code81,
    ///82 - FED-STD (Federal Standard)
    Code82,
    ///83 - CID (Commercial Item Description)
    Code83,
    ///84 - Special Specification
    SpecialSpecification,
    ///85 - Appearance
    Appearance,
    ///86 - Dispersion
    Dispersion,
    ///87 - Fluid
    Fluid,
    ///88 - Flow
    Flow,
    ///89 - Moisture
    Moisture,
    ///90 - Density
    Density,
    ///91 - Buyer's Item Size Description
    BuyersItemSizeDescription,
    ///92 - Fabric Description
    FabricDescription,
    ///93 - Shipping Unit Component
    ShippingUnitComponent,
    ///94 - Type Spinning
    TypeSpinning,
    ///95 - Wax Code
    WaxCode,
    ///96 - Electronically Cleaned
    ElectronicallyCleaned,
    ///97 - Conditioned Code
    ConditionedCode,
    ///99 - Precautionary Instructions
    PrecautionaryInstructions,
    ///AA - Assembly Required
    AssemblyRequired,
    ///AB - Construction
    Construction,
    ///AC - Consumer Instructions
    ConsumerInstructions,
    ///AD - Shelf Tag
    ShelfTag,
    ///AE - Fragrance
    Fragrance,
    ///AF - Editor
    Editor,
    ///AG - Translator
    Translator,
    ///AGE - Age
    Age,
    ///AH - Material
    Material,
    ///AI - Nutrition
    Nutrition,
    ///AJ - Recycle
    Recycle,
    ///AK - Silhouette
    Silhouette,
    ///AL - Discharge
    Discharge,
    ///AT - Process Action Taken
    ProcessActionTaken,
    ///B8 - Bureau of Alcohol, Tobacco and Firearms Class Code
    CodeB8,
    ///BC - Behind-the-Counter Drugs
    BehindTheCounterDrugs,
    ///BCC - Beverage Contents Characteristics, (e.g., Kosher, No Sulfites, etc. (Industry List)
    CodeBCC,
    ///BES - Beverage Segment
    BeverageSegment,
    ///BEV - Beverage Category
    BeverageCategory,
    ///BLM - Bottomhole Location Method Code
    BottomholeLocationMethodCode,
    ///BND - Brand Group
    BrandGroup,
    ///BPI - Bottomhole Pressure Method Indicator Code
    BottomholePressureMethodIndicatorCode,
    ///BRG - Brand Group: A grouping of similar brands, (e.g., Johnnie Walker)
    CodeBRG,
    ///BW - Basis Weight Size
    BasisWeightSize,
    ///C2 - Controlled Substance - Class 2
    ControlledSubstanceClass2,
    ///C3 - Controlled Substance-Class 3 (Narcotic)
    CodeC3,
    ///C4 - Controlled Substance - Class 4
    ControlledSubstanceClass4,
    ///C5 - Controlled Substance - Class 5
    ControlledSubstanceClass5,
    ///C6 - Controlled Substance-Class 3N (Non-narcotic)
    CodeC6,
    ///CCN - Common Chemical Name
    CommonChemicalName,
    ///CD - Collateral Description
    CollateralDescription,
    ///CFC - Company Field Code
    CompanyFieldCode,
    ///CH - Chassis
    Chassis,
    ///CHF - Chemical Family
    ChemicalFamily,
    ///CL - Color - Lower Body
    ColorLowerBody,
    ///CLT - Casing/Liner/Tubing Type
    CasingLinerTubingType,
    ///CM - Compliance Method
    ComplianceMethod,
    ///CMS - Commercial Status
    CommercialStatus,
    ///CO - Collection Method Code
    CollectionMethodCode,
    ///CP - Coupling
    Coupling,
    ///CS - Coating or Paint System Code
    CoatingOrPaintSystemCode,
    ///CU - Color - Upper Body
    ColorUpperBody,
    ///CW - Coating or Paint System Name
    CoatingOrPaintSystemName,
    ///DAC - Damage Code
    DamageCode,
    ///DAF - Damage Fault
    DamageFault,
    ///DE - Drug Efficacy Study Implementation
    DrugEfficacyStudyImplementation,
    ///DF - Dosage Form
    DosageForm,
    ///DIR - Directional Indicator
    DirectionalIndicator,
    ///DM - Dimensional
    Dimensional,
    ///DS - Drug Schedule
    DrugSchedule,
    ///EC - Escrow Code
    EscrowCode,
    ///EN - Engine with Transmission
    EngineWithTransmission,
    ///EV - Environmental Requirement
    EnvironmentalRequirement,
    ///FA - Failure Analysis Process
    FailureAnalysisProcess,
    ///FC - Fold Configurations
    FoldConfigurations,
    ///FCD - Field Code (EIA/DOD)
    CodeFCD,
    ///FDD - Forecast Deviation
    ForecastDeviation,
    ///FL - Fuel
    Fuel,
    ///FLV - Flavor
    Flavor,
    ///FMR - Formula
    Formula,
    ///FQ - Quality
    Quality,
    ///GD - Grain Direction
    GrainDirection,
    ///GEN - General Description
    GeneralDescription,
    ///GM - General Merchandise
    GeneralMerchandise,
    ///GS - Goods
    Goods,
    ///HB - Health and Beauty Aids
    HealthAndBeautyAids,
    ///HY - Hydraulics
    Hydraulics,
    ///HZ - Hazardous Material
    HazardousMaterial,
    ///HZR - Hazard Rating System
    HazardRatingSystem,
    ///ING - Ingredient
    Ingredient,
    ///INJ - Injectables
    Injectables,
    ///KI - Kit
    Kit,
    ///LC - Lead/Copper Sample Type
    LeadCopperSampleType,
    ///LO - Coordinate Description Code
    CoordinateDescriptionCode,
    ///MA - Material Status, Outside Processor
    CodeMA,
    ///MAC - Material Classification
    MaterialClassification,
    ///MB - Marking
    Marking,
    ///MBU - Minerals Management Service/Bureau of Land Management (Indian Land) Property/Unit Number
    CodeMBU,
    ///MM - Multi-Media
    MultiMedia,
    ///MS - Medical Supplies
    MedicalSupplies,
    ///MSG - Market Segment
    MarketSegment,
    ///NH - Non-Hazardous Material
    NonHazardousMaterial,
    ///OC - Options
    Options,
    ///OD - Odorized
    Odorized,
    ///ODR - Odor
    Odor,
    ///OR - Orientation
    Orientation,
    ///OT - Over-the-Counter Drug
    OverTheCounterDrug,
    ///P6 - Percentage of Alcohol
    PercentageOfAlcohol,
    ///PD - Physical Form: As Diluted
    PhysicalFormAsDiluted,
    ///PF - Physical Form: Concentrate
    PhysicalFormConcentrate,
    ///PFA - Physical Form
    PhysicalForm,
    ///PFC - Perforation Continuity Indicator
    PerforationContinuityIndicator,
    ///PFG - Physical Form: Gas
    PhysicalFormGas,
    ///PFI - Perforation Interval(s)
    CodePFI,
    ///PFK - Physical Form: Aerosol
    PhysicalFormAerosol,
    ///PFL - Physical Form: Liquid
    PhysicalFormLiquid,
    ///PFM - Physical Form: Emulsion
    PhysicalFormEmulsion,
    ///PFN - Physical Form: Semisolid
    PhysicalFormSemisolid,
    ///PFP - Physical Form: Powder
    PhysicalFormPowder,
    ///PFS - Physical Form: Solid
    PhysicalFormSolid,
    ///PFT - Perforation Type
    PerforationType,
    ///PG - Program
    Program,
    ///PP - Process/Production Unit
    ProcessProductionUnit,
    ///PR - Manufacturing Process
    ManufacturingProcess,
    ///PRI - Grape Variety
    GrapeVariety,
    ///PRO - Proprietary
    Proprietary,
    ///PSC - Pipeline Stream
    PipelineStream,
    ///PUB - Public Information
    Public,
    ///PUR - Pure Form
    PureForm,
    ///QAS - Quality Assurance Status
    QualityAssuranceStatus,
    ///R3 - Proof
    Proof,
    ///RA - Route of Administration
    RouteOfAdministration,
    ///RCC - Reservoir Code (Company)
    CodeRCC,
    ///RM - Results Method Code
    ResultsMethodCode,
    ///RR - Rejection Reason
    RejectionReason,
    ///RSD - Regulatory (State) District
    CodeRSD,
    ///RSE - Regulatory (State) Entity Code
    CodeRSE,
    ///RX - Prescription Drug
    PrescriptionDrug,
    ///SC - Source
    Source,
    ///SE - Services
    Services,
    ///SEC - Secret or Confidential Information
    SecretOrConfidential,
    ///SF - Service Feature
    ServiceFeature,
    ///SIZ - Sizing
    Sizing,
    ///SLM - Surface Location Method Code
    SurfaceLocationMethodCode,
    ///SOL - Solubility
    Solubility,
    ///ST - Sample Type
    SampleType,
    ///STL - State Controlled
    StateControlled,
    ///SYN - Synonym
    Synonym,
    ///TC - Therapeutic Class
    TherapeuticClass,
    ///TE - Therapeutic Equivalency Evaluation
    TherapeuticEquivalencyEvaluation,
    ///TF - Filtering
    Filtering,
    ///THR - Threshold
    Threshold,
    ///TIF - Title Insurance Form
    TitleInsuranceForm,
    ///TIR - Tire
    Tire,
    ///TP - Typeface
    Typeface,
    ///TR - Trimming
    Trimming,
    ///TRM - Test Remarks
    TestRemarks,
    ///TRN - Trade Name
    TradeName,
    ///TWF - Theoretical Weight Formula
    TheoreticalWeightFormula,
    ///TZ - Bureau of Alcohol, Tobacco and Firearms Type Code
    CodeTZ,
    ///VA - Vehicle
    Vehicle,
    ///VC - Volatile Organic Compound Control
    VolatileOrganicCompoundControl,
    ///VI - Vintage
    Vintage,
    ///WD - Warranty Description
    WarrantyDescription,
    ///WF - Wine Fruit
    WineFruit,
    ///WLC - Well Classification
    WellClassification,
    ///WT - Waste
    Waste,
    ///WTT - Well Test Type
    WellTestType,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl ProductProcessCharacteristicCode {
    pub fn code(&self) -> &str {
        {
            use ProductProcessCharacteristicCode::*;
            match self {
                LimitingOperation => "01",
                GeneralProductForm => "02",
                Preservative => "03",
                Parameter => "04",
                Category => "05",
                SubCategory => "06",
                Product => "08",
                SubProduct => "09",
                ManufacturingMethod => "9A",
                ProductApplication => "9B",
                Engine => "9C",
                Transmission => "9D",
                MajorGrade => "10",
                Code11 => "11",
                TypeAndOrProcess => "12",
                Code13 => "13",
                FinishOrSurfaceRoughness => "14",
                HeatTreatAnneal => "15",
                Temper => "16",
                Coating => "17",
                Code18 => "18",
                Code19 => "19",
                Code20 => "20",
                Forming => "21",
                EdgeTreatment => "22",
                WeldsSplices => "23",
                EndTreatment => "25",
                TestSampleFrequency => "28",
                TestSampleLocation => "29",
                TestSampleDirection => "30",
                TypeOfTestInspection => "32",
                TestingAndInspectionAgencies => "33",
                Filament => "34",
                Color => "35",
                Denier => "36",
                Fiber => "37",
                Grade => "38",
                Luster => "39",
                Shade => "40",
                Tint => "41",
                Tow => "42",
                Twist => "43",
                Bacteriology => "44",
                WholeEffluentToxicity => "45",
                SedimentToxicity => "46",
                SectionProfile => "54",
                Alloy => "55",
                SpecialProcessing => "56",
                WindingInstructions => "58",
                SurfaceProtection => "59",
                MachineRun => "60",
                EndUseApplication => "61",
                CorrosionResistance => "62",
                ProductLifeCycle => "63",
                PackageIntegrity => "64",
                Visual => "65",
                Electrical => "66",
                FunctionalPerformance => "67",
                Chemistry => "68",
                Physical => "69",
                Magnetic => "70",
                Mechanical => "71",
                Metallography => "72",
                VendorColorDescription => "73",
                VendorSizeDescription => "74",
                BuyersColorDescription => "75",
                DyeLotDescription => "76",
                FinishDescription => "77",
                PatternDescription => "78",
                PutUpDescription => "79",
                Code80 => "80",
                Code81 => "81",
                Code82 => "82",
                Code83 => "83",
                SpecialSpecification => "84",
                Appearance => "85",
                Dispersion => "86",
                Fluid => "87",
                Flow => "88",
                Moisture => "89",
                Density => "90",
                BuyersItemSizeDescription => "91",
                FabricDescription => "92",
                ShippingUnitComponent => "93",
                TypeSpinning => "94",
                WaxCode => "95",
                ElectronicallyCleaned => "96",
                ConditionedCode => "97",
                PrecautionaryInstructions => "99",
                AssemblyRequired => "AA",
                Construction => "AB",
                ConsumerInstructions => "AC",
                ShelfTag => "AD",
                Fragrance => "AE",
                Editor => "AF",
                Translator => "AG",
                Age => "AGE",
                Material => "AH",
                Nutrition => "AI",
                Recycle => "AJ",
                Silhouette => "AK",
                Discharge => "AL",
                ProcessActionTaken => "AT",
                CodeB8 => "B8",
                BehindTheCounterDrugs => "BC",
                CodeBCC => "BCC",
                BeverageSegment => "BES",
                BeverageCategory => "BEV",
                BottomholeLocationMethodCode => "BLM",
                BrandGroup => "BND",
                BottomholePressureMethodIndicatorCode => "BPI",
                CodeBRG => "BRG",
                BasisWeightSize => "BW",
                ControlledSubstanceClass2 => "C2",
                CodeC3 => "C3",
                ControlledSubstanceClass4 => "C4",
                ControlledSubstanceClass5 => "C5",
                CodeC6 => "C6",
                CommonChemicalName => "CCN",
                CollateralDescription => "CD",
                CompanyFieldCode => "CFC",
                Chassis => "CH",
                ChemicalFamily => "CHF",
                ColorLowerBody => "CL",
                CasingLinerTubingType => "CLT",
                ComplianceMethod => "CM",
                CommercialStatus => "CMS",
                CollectionMethodCode => "CO",
                Coupling => "CP",
                CoatingOrPaintSystemCode => "CS",
                ColorUpperBody => "CU",
                CoatingOrPaintSystemName => "CW",
                DamageCode => "DAC",
                DamageFault => "DAF",
                DrugEfficacyStudyImplementation => "DE",
                DosageForm => "DF",
                DirectionalIndicator => "DIR",
                Dimensional => "DM",
                DrugSchedule => "DS",
                EscrowCode => "EC",
                EngineWithTransmission => "EN",
                EnvironmentalRequirement => "EV",
                FailureAnalysisProcess => "FA",
                FoldConfigurations => "FC",
                CodeFCD => "FCD",
                ForecastDeviation => "FDD",
                Fuel => "FL",
                Flavor => "FLV",
                Formula => "FMR",
                Quality => "FQ",
                GrainDirection => "GD",
                GeneralDescription => "GEN",
                GeneralMerchandise => "GM",
                Goods => "GS",
                HealthAndBeautyAids => "HB",
                Hydraulics => "HY",
                HazardousMaterial => "HZ",
                HazardRatingSystem => "HZR",
                Ingredient => "ING",
                Injectables => "INJ",
                Kit => "KI",
                LeadCopperSampleType => "LC",
                CoordinateDescriptionCode => "LO",
                CodeMA => "MA",
                MaterialClassification => "MAC",
                Marking => "MB",
                CodeMBU => "MBU",
                MultiMedia => "MM",
                MedicalSupplies => "MS",
                MarketSegment => "MSG",
                NonHazardousMaterial => "NH",
                Options => "OC",
                Odorized => "OD",
                Odor => "ODR",
                Orientation => "OR",
                OverTheCounterDrug => "OT",
                PercentageOfAlcohol => "P6",
                PhysicalFormAsDiluted => "PD",
                PhysicalFormConcentrate => "PF",
                PhysicalForm => "PFA",
                PerforationContinuityIndicator => "PFC",
                PhysicalFormGas => "PFG",
                CodePFI => "PFI",
                PhysicalFormAerosol => "PFK",
                PhysicalFormLiquid => "PFL",
                PhysicalFormEmulsion => "PFM",
                PhysicalFormSemisolid => "PFN",
                PhysicalFormPowder => "PFP",
                PhysicalFormSolid => "PFS",
                PerforationType => "PFT",
                Program => "PG",
                ProcessProductionUnit => "PP",
                ManufacturingProcess => "PR",
                GrapeVariety => "PRI",
                Proprietary => "PRO",
                PipelineStream => "PSC",
                Public => "PUB",
                PureForm => "PUR",
                QualityAssuranceStatus => "QAS",
                Proof => "R3",
                RouteOfAdministration => "RA",
                CodeRCC => "RCC",
                ResultsMethodCode => "RM",
                RejectionReason => "RR",
                CodeRSD => "RSD",
                CodeRSE => "RSE",
                PrescriptionDrug => "RX",
                Source => "SC",
                Services => "SE",
                SecretOrConfidential => "SEC",
                ServiceFeature => "SF",
                Sizing => "SIZ",
                SurfaceLocationMethodCode => "SLM",
                Solubility => "SOL",
                SampleType => "ST",
                StateControlled => "STL",
                Synonym => "SYN",
                TherapeuticClass => "TC",
                TherapeuticEquivalencyEvaluation => "TE",
                Filtering => "TF",
                Threshold => "THR",
                TitleInsuranceForm => "TIF",
                Tire => "TIR",
                Typeface => "TP",
                Trimming => "TR",
                TestRemarks => "TRM",
                TradeName => "TRN",
                TheoreticalWeightFormula => "TWF",
                CodeTZ => "TZ",
                Vehicle => "VA",
                VolatileOrganicCompoundControl => "VC",
                Vintage => "VI",
                WarrantyDescription => "WD",
                WineFruit => "WF",
                WellClassification => "WLC",
                Waste => "WT",
                WellTestType => "WTT",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ProductProcessCharacteristicCode> {
        use ProductProcessCharacteristicCode::*;
        match code {
            b"01" => Some(LimitingOperation),
            b"02" => Some(GeneralProductForm),
            b"03" => Some(Preservative),
            b"04" => Some(Parameter),
            b"05" => Some(Category),
            b"06" => Some(SubCategory),
            b"08" => Some(Product),
            b"09" => Some(SubProduct),
            b"9A" => Some(ManufacturingMethod),
            b"9B" => Some(ProductApplication),
            b"9C" => Some(Engine),
            b"9D" => Some(Transmission),
            b"10" => Some(MajorGrade),
            b"11" => Some(Code11),
            b"12" => Some(TypeAndOrProcess),
            b"13" => Some(Code13),
            b"14" => Some(FinishOrSurfaceRoughness),
            b"15" => Some(HeatTreatAnneal),
            b"16" => Some(Temper),
            b"17" => Some(Coating),
            b"18" => Some(Code18),
            b"19" => Some(Code19),
            b"20" => Some(Code20),
            b"21" => Some(Forming),
            b"22" => Some(EdgeTreatment),
            b"23" => Some(WeldsSplices),
            b"25" => Some(EndTreatment),
            b"28" => Some(TestSampleFrequency),
            b"29" => Some(TestSampleLocation),
            b"30" => Some(TestSampleDirection),
            b"32" => Some(TypeOfTestInspection),
            b"33" => Some(TestingAndInspectionAgencies),
            b"34" => Some(Filament),
            b"35" => Some(Color),
            b"36" => Some(Denier),
            b"37" => Some(Fiber),
            b"38" => Some(Grade),
            b"39" => Some(Luster),
            b"40" => Some(Shade),
            b"41" => Some(Tint),
            b"42" => Some(Tow),
            b"43" => Some(Twist),
            b"44" => Some(Bacteriology),
            b"45" => Some(WholeEffluentToxicity),
            b"46" => Some(SedimentToxicity),
            b"54" => Some(SectionProfile),
            b"55" => Some(Alloy),
            b"56" => Some(SpecialProcessing),
            b"58" => Some(WindingInstructions),
            b"59" => Some(SurfaceProtection),
            b"60" => Some(MachineRun),
            b"61" => Some(EndUseApplication),
            b"62" => Some(CorrosionResistance),
            b"63" => Some(ProductLifeCycle),
            b"64" => Some(PackageIntegrity),
            b"65" => Some(Visual),
            b"66" => Some(Electrical),
            b"67" => Some(FunctionalPerformance),
            b"68" => Some(Chemistry),
            b"69" => Some(Physical),
            b"70" => Some(Magnetic),
            b"71" => Some(Mechanical),
            b"72" => Some(Metallography),
            b"73" => Some(VendorColorDescription),
            b"74" => Some(VendorSizeDescription),
            b"75" => Some(BuyersColorDescription),
            b"76" => Some(DyeLotDescription),
            b"77" => Some(FinishDescription),
            b"78" => Some(PatternDescription),
            b"79" => Some(PutUpDescription),
            b"80" => Some(Code80),
            b"81" => Some(Code81),
            b"82" => Some(Code82),
            b"83" => Some(Code83),
            b"84" => Some(SpecialSpecification),
            b"85" => Some(Appearance),
            b"86" => Some(Dispersion),
            b"87" => Some(Fluid),
            b"88" => Some(Flow),
            b"89" => Some(Moisture),
            b"90" => Some(Density),
            b"91" => Some(BuyersItemSizeDescription),
            b"92" => Some(FabricDescription),
            b"93" => Some(ShippingUnitComponent),
            b"94" => Some(TypeSpinning),
            b"95" => Some(WaxCode),
            b"96" => Some(ElectronicallyCleaned),
            b"97" => Some(ConditionedCode),
            b"99" => Some(PrecautionaryInstructions),
            b"AA" => Some(AssemblyRequired),
            b"AB" => Some(Construction),
            b"AC" => Some(ConsumerInstructions),
            b"AD" => Some(ShelfTag),
            b"AE" => Some(Fragrance),
            b"AF" => Some(Editor),
            b"AG" => Some(Translator),
            b"AGE" => Some(Age),
            b"AH" => Some(Material),
            b"AI" => Some(Nutrition),
            b"AJ" => Some(Recycle),
            b"AK" => Some(Silhouette),
            b"AL" => Some(Discharge),
            b"AT" => Some(ProcessActionTaken),
            b"B8" => Some(CodeB8),
            b"BC" => Some(BehindTheCounterDrugs),
            b"BCC" => Some(CodeBCC),
            b"BES" => Some(BeverageSegment),
            b"BEV" => Some(BeverageCategory),
            b"BLM" => Some(BottomholeLocationMethodCode),
            b"BND" => Some(BrandGroup),
            b"BPI" => Some(BottomholePressureMethodIndicatorCode),
            b"BRG" => Some(CodeBRG),
            b"BW" => Some(BasisWeightSize),
            b"C2" => Some(ControlledSubstanceClass2),
            b"C3" => Some(CodeC3),
            b"C4" => Some(ControlledSubstanceClass4),
            b"C5" => Some(ControlledSubstanceClass5),
            b"C6" => Some(CodeC6),
            b"CCN" => Some(CommonChemicalName),
            b"CD" => Some(CollateralDescription),
            b"CFC" => Some(CompanyFieldCode),
            b"CH" => Some(Chassis),
            b"CHF" => Some(ChemicalFamily),
            b"CL" => Some(ColorLowerBody),
            b"CLT" => Some(CasingLinerTubingType),
            b"CM" => Some(ComplianceMethod),
            b"CMS" => Some(CommercialStatus),
            b"CO" => Some(CollectionMethodCode),
            b"CP" => Some(Coupling),
            b"CS" => Some(CoatingOrPaintSystemCode),
            b"CU" => Some(ColorUpperBody),
            b"CW" => Some(CoatingOrPaintSystemName),
            b"DAC" => Some(DamageCode),
            b"DAF" => Some(DamageFault),
            b"DE" => Some(DrugEfficacyStudyImplementation),
            b"DF" => Some(DosageForm),
            b"DIR" => Some(DirectionalIndicator),
            b"DM" => Some(Dimensional),
            b"DS" => Some(DrugSchedule),
            b"EC" => Some(EscrowCode),
            b"EN" => Some(EngineWithTransmission),
            b"EV" => Some(EnvironmentalRequirement),
            b"FA" => Some(FailureAnalysisProcess),
            b"FC" => Some(FoldConfigurations),
            b"FCD" => Some(CodeFCD),
            b"FDD" => Some(ForecastDeviation),
            b"FL" => Some(Fuel),
            b"FLV" => Some(Flavor),
            b"FMR" => Some(Formula),
            b"FQ" => Some(Quality),
            b"GD" => Some(GrainDirection),
            b"GEN" => Some(GeneralDescription),
            b"GM" => Some(GeneralMerchandise),
            b"GS" => Some(Goods),
            b"HB" => Some(HealthAndBeautyAids),
            b"HY" => Some(Hydraulics),
            b"HZ" => Some(HazardousMaterial),
            b"HZR" => Some(HazardRatingSystem),
            b"ING" => Some(Ingredient),
            b"INJ" => Some(Injectables),
            b"KI" => Some(Kit),
            b"LC" => Some(LeadCopperSampleType),
            b"LO" => Some(CoordinateDescriptionCode),
            b"MA" => Some(CodeMA),
            b"MAC" => Some(MaterialClassification),
            b"MB" => Some(Marking),
            b"MBU" => Some(CodeMBU),
            b"MM" => Some(MultiMedia),
            b"MS" => Some(MedicalSupplies),
            b"MSG" => Some(MarketSegment),
            b"NH" => Some(NonHazardousMaterial),
            b"OC" => Some(Options),
            b"OD" => Some(Odorized),
            b"ODR" => Some(Odor),
            b"OR" => Some(Orientation),
            b"OT" => Some(OverTheCounterDrug),
            b"P6" => Some(PercentageOfAlcohol),
            b"PD" => Some(PhysicalFormAsDiluted),
            b"PF" => Some(PhysicalFormConcentrate),
            b"PFA" => Some(PhysicalForm),
            b"PFC" => Some(PerforationContinuityIndicator),
            b"PFG" => Some(PhysicalFormGas),
            b"PFI" => Some(CodePFI),
            b"PFK" => Some(PhysicalFormAerosol),
            b"PFL" => Some(PhysicalFormLiquid),
            b"PFM" => Some(PhysicalFormEmulsion),
            b"PFN" => Some(PhysicalFormSemisolid),
            b"PFP" => Some(PhysicalFormPowder),
            b"PFS" => Some(PhysicalFormSolid),
            b"PFT" => Some(PerforationType),
            b"PG" => Some(Program),
            b"PP" => Some(ProcessProductionUnit),
            b"PR" => Some(ManufacturingProcess),
            b"PRI" => Some(GrapeVariety),
            b"PRO" => Some(Proprietary),
            b"PSC" => Some(PipelineStream),
            b"PUB" => Some(Public),
            b"PUR" => Some(PureForm),
            b"QAS" => Some(QualityAssuranceStatus),
            b"R3" => Some(Proof),
            b"RA" => Some(RouteOfAdministration),
            b"RCC" => Some(CodeRCC),
            b"RM" => Some(ResultsMethodCode),
            b"RR" => Some(RejectionReason),
            b"RSD" => Some(CodeRSD),
            b"RSE" => Some(CodeRSE),
            b"RX" => Some(PrescriptionDrug),
            b"SC" => Some(Source),
            b"SE" => Some(Services),
            b"SEC" => Some(SecretOrConfidential),
            b"SF" => Some(ServiceFeature),
            b"SIZ" => Some(Sizing),
            b"SLM" => Some(SurfaceLocationMethodCode),
            b"SOL" => Some(Solubility),
            b"ST" => Some(SampleType),
            b"STL" => Some(StateControlled),
            b"SYN" => Some(Synonym),
            b"TC" => Some(TherapeuticClass),
            b"TE" => Some(TherapeuticEquivalencyEvaluation),
            b"TF" => Some(Filtering),
            b"THR" => Some(Threshold),
            b"TIF" => Some(TitleInsuranceForm),
            b"TIR" => Some(Tire),
            b"TP" => Some(Typeface),
            b"TR" => Some(Trimming),
            b"TRM" => Some(TestRemarks),
            b"TRN" => Some(TradeName),
            b"TWF" => Some(TheoreticalWeightFormula),
            b"TZ" => Some(CodeTZ),
            b"VA" => Some(Vehicle),
            b"VC" => Some(VolatileOrganicCompoundControl),
            b"VI" => Some(Vintage),
            b"WD" => Some(WarrantyDescription),
            b"WF" => Some(WineFruit),
            b"WLC" => Some(WellClassification),
            b"WT" => Some(Waste),
            b"WTT" => Some(WellTestType),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ProductProcessCharacteristicCode::*;
        match self {
            LimitingOperation => "Limiting Operation",
            GeneralProductForm => "General Product Form",
            Preservative => "Preservative",
            Parameter => "Parameter",
            Category => "Category",
            SubCategory => "Sub-category",
            Product => "Product",
            SubProduct => "Sub-product",
            ManufacturingMethod => "Manufacturing Method",
            ProductApplication => "Product Application",
            Engine => "Engine",
            Transmission => "Transmission",
            MajorGrade => "Major Grade",
            Code11 => "Society, Government, Customer Specifications",
            TypeAndOrProcess => "Type and/or Process",
            Code13 => "Quality (Quality Level)",
            FinishOrSurfaceRoughness => "Finish or Surface Roughness",
            HeatTreatAnneal => "Heat Treat/Anneal",
            Temper => "Temper",
            Coating => "Coating",
            Code18 => "Surface Treatment, Chemical",
            Code19 => "Surface Treatment, Mechanical",
            Code20 => "Ends: Slitting, Splitting, Cutting",
            Forming => "Forming",
            EdgeTreatment => "Edge Treatment",
            WeldsSplices => "Welds/Splices",
            EndTreatment => "End Treatment",
            TestSampleFrequency => "Test Sample Frequency",
            TestSampleLocation => "Test Sample Location",
            TestSampleDirection => "Test Sample Direction",
            TypeOfTestInspection => "Type of Test/Inspection",
            TestingAndInspectionAgencies => "Testing and Inspection Agencies",
            Filament => "Filament",
            Color => "Color",
            Denier => "Denier",
            Fiber => "Fiber",
            Grade => "Grade",
            Luster => "Luster",
            Shade => "Shade",
            Tint => "Tint",
            Tow => "Tow",
            Twist => "Twist",
            Bacteriology => "Bacteriology",
            WholeEffluentToxicity => "Whole Effluent Toxicity",
            SedimentToxicity => "Sediment Toxicity",
            SectionProfile => "Section Profile",
            Alloy => "Alloy",
            SpecialProcessing => "Special Processing",
            WindingInstructions => "Winding Instructions",
            SurfaceProtection => "Surface Protection",
            MachineRun => "Machine Run",
            EndUseApplication => "End Use Application",
            CorrosionResistance => "Corrosion Resistance",
            ProductLifeCycle => "Product Life Cycle",
            PackageIntegrity => "Package Integrity",
            Visual => "Visual",
            Electrical => "Electrical",
            FunctionalPerformance => "Functional Performance",
            Chemistry => "Chemistry",
            Physical => "Physical",
            Magnetic => "Magnetic",
            Mechanical => "Mechanical",
            Metallography => "Metallography",
            VendorColorDescription => "Vendor color description",
            VendorSizeDescription => "Vendor size description",
            BuyersColorDescription => "Buyer's Color Description",
            DyeLotDescription => "Dye Lot Description",
            FinishDescription => "Finish Description",
            PatternDescription => "Pattern Description",
            PutUpDescription => "Put-up Description",
            Code80 => "MILSPEC (Military Specification)",
            Code81 => "FEDSPEC (Federal Specification)",
            Code82 => "FED-STD (Federal Standard)",
            Code83 => "CID (Commercial Item Description)",
            SpecialSpecification => "Special Specification",
            Appearance => "Appearance",
            Dispersion => "Dispersion",
            Fluid => "Fluid",
            Flow => "Flow",
            Moisture => "Moisture",
            Density => "Density",
            BuyersItemSizeDescription => "Buyer's Item Size Description",
            FabricDescription => "Fabric Description",
            ShippingUnitComponent => "Shipping Unit Component",
            TypeSpinning => "Type Spinning",
            WaxCode => "Wax Code",
            ElectronicallyCleaned => "Electronically Cleaned",
            ConditionedCode => "Conditioned Code",
            PrecautionaryInstructions => "Precautionary Instructions",
            AssemblyRequired => "Assembly Required",
            Construction => "Construction",
            ConsumerInstructions => "Consumer Instructions",
            ShelfTag => "Shelf Tag",
            Fragrance => "Fragrance",
            Editor => "Editor",
            Translator => "Translator",
            Age => "Age",
            Material => "Material",
            Nutrition => "Nutrition",
            Recycle => "Recycle",
            Silhouette => "Silhouette",
            Discharge => "Discharge",
            ProcessActionTaken => "Process Action Taken",
            CodeB8 => "Bureau of Alcohol, Tobacco and Firearms Class Code",
            BehindTheCounterDrugs => "Behind-the-Counter Drugs",
            CodeBCC => {
                "Beverage Contents Characteristics, (e.g., Kosher, No Sulfites, etc. (Industry List)"
            }
            BeverageSegment => "Beverage Segment",
            BeverageCategory => "Beverage Category",
            BottomholeLocationMethodCode => "Bottomhole Location Method Code",
            BrandGroup => "Brand Group",
            BottomholePressureMethodIndicatorCode => {
                "Bottomhole Pressure Method Indicator Code"
            }
            CodeBRG => {
                "Brand Group: A grouping of similar brands, (e.g., Johnnie Walker)"
            }
            BasisWeightSize => "Basis Weight Size",
            ControlledSubstanceClass2 => "Controlled Substance - Class 2",
            CodeC3 => "Controlled Substance-Class 3 (Narcotic)",
            ControlledSubstanceClass4 => "Controlled Substance - Class 4",
            ControlledSubstanceClass5 => "Controlled Substance - Class 5",
            CodeC6 => "Controlled Substance-Class 3N (Non-narcotic)",
            CommonChemicalName => "Common Chemical Name",
            CollateralDescription => "Collateral Description",
            CompanyFieldCode => "Company Field Code",
            Chassis => "Chassis",
            ChemicalFamily => "Chemical Family",
            ColorLowerBody => "Color - Lower Body",
            CasingLinerTubingType => "Casing/Liner/Tubing Type",
            ComplianceMethod => "Compliance Method",
            CommercialStatus => "Commercial Status",
            CollectionMethodCode => "Collection Method Code",
            Coupling => "Coupling",
            CoatingOrPaintSystemCode => "Coating or Paint System Code",
            ColorUpperBody => "Color - Upper Body",
            CoatingOrPaintSystemName => "Coating or Paint System Name",
            DamageCode => "Damage Code",
            DamageFault => "Damage Fault",
            DrugEfficacyStudyImplementation => "Drug Efficacy Study Implementation",
            DosageForm => "Dosage Form",
            DirectionalIndicator => "Directional Indicator",
            Dimensional => "Dimensional",
            DrugSchedule => "Drug Schedule",
            EscrowCode => "Escrow Code",
            EngineWithTransmission => "Engine with Transmission",
            EnvironmentalRequirement => "Environmental Requirement",
            FailureAnalysisProcess => "Failure Analysis Process",
            FoldConfigurations => "Fold Configurations",
            CodeFCD => "Field Code (EIA/DOD)",
            ForecastDeviation => "Forecast Deviation",
            Fuel => "Fuel",
            Flavor => "Flavor",
            Formula => "Formula",
            Quality => "Quality",
            GrainDirection => "Grain Direction",
            GeneralDescription => "General Description",
            GeneralMerchandise => "General Merchandise",
            Goods => "Goods",
            HealthAndBeautyAids => "Health and Beauty Aids",
            Hydraulics => "Hydraulics",
            HazardousMaterial => "Hazardous Material",
            HazardRatingSystem => "Hazard Rating System",
            Ingredient => "Ingredient",
            Injectables => "Injectables",
            Kit => "Kit",
            LeadCopperSampleType => "Lead/Copper Sample Type",
            CoordinateDescriptionCode => "Coordinate Description Code",
            CodeMA => "Material Status, Outside Processor",
            MaterialClassification => "Material Classification",
            Marking => "Marking",
            CodeMBU => {
                "Minerals Management Service/Bureau of Land Management (Indian Land) Property/Unit Number"
            }
            MultiMedia => "Multi-Media",
            MedicalSupplies => "Medical Supplies",
            MarketSegment => "Market Segment",
            NonHazardousMaterial => "Non-Hazardous Material",
            Options => "Options",
            Odorized => "Odorized",
            Odor => "Odor",
            Orientation => "Orientation",
            OverTheCounterDrug => "Over-the-Counter Drug",
            PercentageOfAlcohol => "Percentage of Alcohol",
            PhysicalFormAsDiluted => "Physical Form: As Diluted",
            PhysicalFormConcentrate => "Physical Form: Concentrate",
            PhysicalForm => "Physical Form",
            PerforationContinuityIndicator => "Perforation Continuity Indicator",
            PhysicalFormGas => "Physical Form: Gas",
            CodePFI => "Perforation Interval(s)",
            PhysicalFormAerosol => "Physical Form: Aerosol",
            PhysicalFormLiquid => "Physical Form: Liquid",
            PhysicalFormEmulsion => "Physical Form: Emulsion",
            PhysicalFormSemisolid => "Physical Form: Semisolid",
            PhysicalFormPowder => "Physical Form: Powder",
            PhysicalFormSolid => "Physical Form: Solid",
            PerforationType => "Perforation Type",
            Program => "Program",
            ProcessProductionUnit => "Process/Production Unit",
            ManufacturingProcess => "Manufacturing Process",
            GrapeVariety => "Grape Variety",
            Proprietary => "Proprietary",
            PipelineStream => "Pipeline Stream",
            Public => "Public Information",
            PureForm => "Pure Form",
            QualityAssuranceStatus => "Quality Assurance Status",
            Proof => "Proof",
            RouteOfAdministration => "Route of Administration",
            CodeRCC => "Reservoir Code (Company)",
            ResultsMethodCode => "Results Method Code",
            RejectionReason => "Rejection Reason",
            CodeRSD => "Regulatory (State) District",
            CodeRSE => "Regulatory (State) Entity Code",
            PrescriptionDrug => "Prescription Drug",
            Source => "Source",
            Services => "Services",
            SecretOrConfidential => "Secret or Confidential Information",
            ServiceFeature => "Service Feature",
            Sizing => "Sizing",
            SurfaceLocationMethodCode => "Surface Location Method Code",
            Solubility => "Solubility",
            SampleType => "Sample Type",
            StateControlled => "State Controlled",
            Synonym => "Synonym",
            TherapeuticClass => "Therapeutic Class",
            TherapeuticEquivalencyEvaluation => "Therapeutic Equivalency Evaluation",
            Filtering => "Filtering",
            Threshold => "Threshold",
            TitleInsuranceForm => "Title Insurance Form",
            Tire => "Tire",
            Typeface => "Typeface",
            Trimming => "Trimming",
            TestRemarks => "Test Remarks",
            TradeName => "Trade Name",
            TheoreticalWeightFormula => "Theoretical Weight Formula",
            CodeTZ => "Bureau of Alcohol, Tobacco and Firearms Type Code",
            Vehicle => "Vehicle",
            VolatileOrganicCompoundControl => "Volatile Organic Compound Control",
            Vintage => "Vintage",
            WarrantyDescription => "Warranty Description",
            WineFruit => "Wine Fruit",
            WellClassification => "Well Classification",
            Waste => "Waste",
            WellTestType => "Well Test Type",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<ProductProcessCharacteristicCode> {
        {
            use ProductProcessCharacteristicCode::*;
            match description {
                "Limiting Operation" => Some(LimitingOperation),
                "General Product Form" => Some(GeneralProductForm),
                "Preservative" => Some(Preservative),
                "Parameter" => Some(Parameter),
                "Category" => Some(Category),
                "Sub-category" => Some(SubCategory),
                "Product" => Some(Product),
                "Sub-product" => Some(SubProduct),
                "Manufacturing Method" => Some(ManufacturingMethod),
                "Product Application" => Some(ProductApplication),
                "Engine" => Some(Engine),
                "Transmission" => Some(Transmission),
                "Major Grade" => Some(MajorGrade),
                "Society, Government, Customer Specifications" => Some(Code11),
                "Type and/or Process" => Some(TypeAndOrProcess),
                "Quality (Quality Level)" => Some(Code13),
                "Finish or Surface Roughness" => Some(FinishOrSurfaceRoughness),
                "Heat Treat/Anneal" => Some(HeatTreatAnneal),
                "Temper" => Some(Temper),
                "Coating" => Some(Coating),
                "Surface Treatment, Chemical" => Some(Code18),
                "Surface Treatment, Mechanical" => Some(Code19),
                "Ends: Slitting, Splitting, Cutting" => Some(Code20),
                "Forming" => Some(Forming),
                "Edge Treatment" => Some(EdgeTreatment),
                "Welds/Splices" => Some(WeldsSplices),
                "End Treatment" => Some(EndTreatment),
                "Test Sample Frequency" => Some(TestSampleFrequency),
                "Test Sample Location" => Some(TestSampleLocation),
                "Test Sample Direction" => Some(TestSampleDirection),
                "Type of Test/Inspection" => Some(TypeOfTestInspection),
                "Testing and Inspection Agencies" => Some(TestingAndInspectionAgencies),
                "Filament" => Some(Filament),
                "Color" => Some(Color),
                "Denier" => Some(Denier),
                "Fiber" => Some(Fiber),
                "Grade" => Some(Grade),
                "Luster" => Some(Luster),
                "Shade" => Some(Shade),
                "Tint" => Some(Tint),
                "Tow" => Some(Tow),
                "Twist" => Some(Twist),
                "Bacteriology" => Some(Bacteriology),
                "Whole Effluent Toxicity" => Some(WholeEffluentToxicity),
                "Sediment Toxicity" => Some(SedimentToxicity),
                "Section Profile" => Some(SectionProfile),
                "Alloy" => Some(Alloy),
                "Special Processing" => Some(SpecialProcessing),
                "Winding Instructions" => Some(WindingInstructions),
                "Surface Protection" => Some(SurfaceProtection),
                "Machine Run" => Some(MachineRun),
                "End Use Application" => Some(EndUseApplication),
                "Corrosion Resistance" => Some(CorrosionResistance),
                "Product Life Cycle" => Some(ProductLifeCycle),
                "Package Integrity" => Some(PackageIntegrity),
                "Visual" => Some(Visual),
                "Electrical" => Some(Electrical),
                "Functional Performance" => Some(FunctionalPerformance),
                "Chemistry" => Some(Chemistry),
                "Physical" => Some(Physical),
                "Magnetic" => Some(Magnetic),
                "Mechanical" => Some(Mechanical),
                "Metallography" => Some(Metallography),
                "Vendor color description" => Some(VendorColorDescription),
                "Vendor size description" => Some(VendorSizeDescription),
                "Buyer's Color Description" => Some(BuyersColorDescription),
                "Dye Lot Description" => Some(DyeLotDescription),
                "Finish Description" => Some(FinishDescription),
                "Pattern Description" => Some(PatternDescription),
                "Put-up Description" => Some(PutUpDescription),
                "MILSPEC (Military Specification)" => Some(Code80),
                "FEDSPEC (Federal Specification)" => Some(Code81),
                "FED-STD (Federal Standard)" => Some(Code82),
                "CID (Commercial Item Description)" => Some(Code83),
                "Special Specification" => Some(SpecialSpecification),
                "Appearance" => Some(Appearance),
                "Dispersion" => Some(Dispersion),
                "Fluid" => Some(Fluid),
                "Flow" => Some(Flow),
                "Moisture" => Some(Moisture),
                "Density" => Some(Density),
                "Buyer's Item Size Description" => Some(BuyersItemSizeDescription),
                "Fabric Description" => Some(FabricDescription),
                "Shipping Unit Component" => Some(ShippingUnitComponent),
                "Type Spinning" => Some(TypeSpinning),
                "Wax Code" => Some(WaxCode),
                "Electronically Cleaned" => Some(ElectronicallyCleaned),
                "Conditioned Code" => Some(ConditionedCode),
                "Precautionary Instructions" => Some(PrecautionaryInstructions),
                "Assembly Required" => Some(AssemblyRequired),
                "Construction" => Some(Construction),
                "Consumer Instructions" => Some(ConsumerInstructions),
                "Shelf Tag" => Some(ShelfTag),
                "Fragrance" => Some(Fragrance),
                "Editor" => Some(Editor),
                "Translator" => Some(Translator),
                "Age" => Some(Age),
                "Material" => Some(Material),
                "Nutrition" => Some(Nutrition),
                "Recycle" => Some(Recycle),
                "Silhouette" => Some(Silhouette),
                "Discharge" => Some(Discharge),
                "Process Action Taken" => Some(ProcessActionTaken),
                "Bureau of Alcohol, Tobacco and Firearms Class Code" => Some(CodeB8),
                "Behind-the-Counter Drugs" => Some(BehindTheCounterDrugs),
                "Beverage Contents Characteristics, (e.g., Kosher, No Sulfites, etc. (Industry List)" => {
                    Some(CodeBCC)
                }
                "Beverage Segment" => Some(BeverageSegment),
                "Beverage Category" => Some(BeverageCategory),
                "Bottomhole Location Method Code" => Some(BottomholeLocationMethodCode),
                "Brand Group" => Some(BrandGroup),
                "Bottomhole Pressure Method Indicator Code" => {
                    Some(BottomholePressureMethodIndicatorCode)
                }
                "Brand Group: A grouping of similar brands, (e.g., Johnnie Walker)" => {
                    Some(CodeBRG)
                }
                "Basis Weight Size" => Some(BasisWeightSize),
                "Controlled Substance - Class 2" => Some(ControlledSubstanceClass2),
                "Controlled Substance-Class 3 (Narcotic)" => Some(CodeC3),
                "Controlled Substance - Class 4" => Some(ControlledSubstanceClass4),
                "Controlled Substance - Class 5" => Some(ControlledSubstanceClass5),
                "Controlled Substance-Class 3N (Non-narcotic)" => Some(CodeC6),
                "Common Chemical Name" => Some(CommonChemicalName),
                "Collateral Description" => Some(CollateralDescription),
                "Company Field Code" => Some(CompanyFieldCode),
                "Chassis" => Some(Chassis),
                "Chemical Family" => Some(ChemicalFamily),
                "Color - Lower Body" => Some(ColorLowerBody),
                "Casing/Liner/Tubing Type" => Some(CasingLinerTubingType),
                "Compliance Method" => Some(ComplianceMethod),
                "Commercial Status" => Some(CommercialStatus),
                "Collection Method Code" => Some(CollectionMethodCode),
                "Coupling" => Some(Coupling),
                "Coating or Paint System Code" => Some(CoatingOrPaintSystemCode),
                "Color - Upper Body" => Some(ColorUpperBody),
                "Coating or Paint System Name" => Some(CoatingOrPaintSystemName),
                "Damage Code" => Some(DamageCode),
                "Damage Fault" => Some(DamageFault),
                "Drug Efficacy Study Implementation" => {
                    Some(DrugEfficacyStudyImplementation)
                }
                "Dosage Form" => Some(DosageForm),
                "Directional Indicator" => Some(DirectionalIndicator),
                "Dimensional" => Some(Dimensional),
                "Drug Schedule" => Some(DrugSchedule),
                "Escrow Code" => Some(EscrowCode),
                "Engine with Transmission" => Some(EngineWithTransmission),
                "Environmental Requirement" => Some(EnvironmentalRequirement),
                "Failure Analysis Process" => Some(FailureAnalysisProcess),
                "Fold Configurations" => Some(FoldConfigurations),
                "Field Code (EIA/DOD)" => Some(CodeFCD),
                "Forecast Deviation" => Some(ForecastDeviation),
                "Fuel" => Some(Fuel),
                "Flavor" => Some(Flavor),
                "Formula" => Some(Formula),
                "Quality" => Some(Quality),
                "Grain Direction" => Some(GrainDirection),
                "General Description" => Some(GeneralDescription),
                "General Merchandise" => Some(GeneralMerchandise),
                "Goods" => Some(Goods),
                "Health and Beauty Aids" => Some(HealthAndBeautyAids),
                "Hydraulics" => Some(Hydraulics),
                "Hazardous Material" => Some(HazardousMaterial),
                "Hazard Rating System" => Some(HazardRatingSystem),
                "Ingredient" => Some(Ingredient),
                "Injectables" => Some(Injectables),
                "Kit" => Some(Kit),
                "Lead/Copper Sample Type" => Some(LeadCopperSampleType),
                "Coordinate Description Code" => Some(CoordinateDescriptionCode),
                "Material Status, Outside Processor" => Some(CodeMA),
                "Material Classification" => Some(MaterialClassification),
                "Marking" => Some(Marking),
                "Minerals Management Service/Bureau of Land Management (Indian Land) Property/Unit Number" => {
                    Some(CodeMBU)
                }
                "Multi-Media" => Some(MultiMedia),
                "Medical Supplies" => Some(MedicalSupplies),
                "Market Segment" => Some(MarketSegment),
                "Non-Hazardous Material" => Some(NonHazardousMaterial),
                "Options" => Some(Options),
                "Odorized" => Some(Odorized),
                "Odor" => Some(Odor),
                "Orientation" => Some(Orientation),
                "Over-the-Counter Drug" => Some(OverTheCounterDrug),
                "Percentage of Alcohol" => Some(PercentageOfAlcohol),
                "Physical Form: As Diluted" => Some(PhysicalFormAsDiluted),
                "Physical Form: Concentrate" => Some(PhysicalFormConcentrate),
                "Physical Form" => Some(PhysicalForm),
                "Perforation Continuity Indicator" => {
                    Some(PerforationContinuityIndicator)
                }
                "Physical Form: Gas" => Some(PhysicalFormGas),
                "Perforation Interval(s)" => Some(CodePFI),
                "Physical Form: Aerosol" => Some(PhysicalFormAerosol),
                "Physical Form: Liquid" => Some(PhysicalFormLiquid),
                "Physical Form: Emulsion" => Some(PhysicalFormEmulsion),
                "Physical Form: Semisolid" => Some(PhysicalFormSemisolid),
                "Physical Form: Powder" => Some(PhysicalFormPowder),
                "Physical Form: Solid" => Some(PhysicalFormSolid),
                "Perforation Type" => Some(PerforationType),
                "Program" => Some(Program),
                "Process/Production Unit" => Some(ProcessProductionUnit),
                "Manufacturing Process" => Some(ManufacturingProcess),
                "Grape Variety" => Some(GrapeVariety),
                "Proprietary" => Some(Proprietary),
                "Pipeline Stream" => Some(PipelineStream),
                "Public Information" => Some(Public),
                "Pure Form" => Some(PureForm),
                "Quality Assurance Status" => Some(QualityAssuranceStatus),
                "Proof" => Some(Proof),
                "Route of Administration" => Some(RouteOfAdministration),
                "Reservoir Code (Company)" => Some(CodeRCC),
                "Results Method Code" => Some(ResultsMethodCode),
                "Rejection Reason" => Some(RejectionReason),
                "Regulatory (State) District" => Some(CodeRSD),
                "Regulatory (State) Entity Code" => Some(CodeRSE),
                "Prescription Drug" => Some(PrescriptionDrug),
                "Source" => Some(Source),
                "Services" => Some(Services),
                "Secret or Confidential Information" => Some(SecretOrConfidential),
                "Service Feature" => Some(ServiceFeature),
                "Sizing" => Some(Sizing),
                "Surface Location Method Code" => Some(SurfaceLocationMethodCode),
                "Solubility" => Some(Solubility),
                "Sample Type" => Some(SampleType),
                "State Controlled" => Some(StateControlled),
                "Synonym" => Some(Synonym),
                "Therapeutic Class" => Some(TherapeuticClass),
                "Therapeutic Equivalency Evaluation" => {
                    Some(TherapeuticEquivalencyEvaluation)
                }
                "Filtering" => Some(Filtering),
                "Threshold" => Some(Threshold),
                "Title Insurance Form" => Some(TitleInsuranceForm),
                "Tire" => Some(Tire),
                "Typeface" => Some(Typeface),
                "Trimming" => Some(Trimming),
                "Test Remarks" => Some(TestRemarks),
                "Trade Name" => Some(TradeName),
                "Theoretical Weight Formula" => Some(TheoreticalWeightFormula),
                "Bureau of Alcohol, Tobacco and Firearms Type Code" => Some(CodeTZ),
                "Vehicle" => Some(Vehicle),
                "Volatile Organic Compound Control" => {
                    Some(VolatileOrganicCompoundControl)
                }
                "Vintage" => Some(Vintage),
                "Warranty Description" => Some(WarrantyDescription),
                "Wine Fruit" => Some(WineFruit),
                "Well Classification" => Some(WellClassification),
                "Waste" => Some(Waste),
                "Well Test Type" => Some(WellTestType),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for ProductProcessCharacteristicCode {
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
    type Value = ProductProcessCharacteristicCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Product/Process Characteristic Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProductProcessCharacteristicCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Product/Process Characteristic Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProductProcessCharacteristicCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Product/Process Characteristic Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ProductProcessCharacteristicCode {
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