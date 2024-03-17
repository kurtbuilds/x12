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
    ///AA1 - Composition Claim Made With
    CompositionClaimMadeWith,
    ///AA2 - Composition Claim Natural Ingredients
    CompositionClaimNaturalIngredients,
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
    ///AN - Anaerobic Spores
    AnaerobicSpores,
    ///AR - Allergen
    Allergen,
    ///AS - Aerobic Spores
    AerobicSpores,
    ///AT - Process Action Taken
    ProcessActionTaken,
    ///B8 - Bureau of Alcohol, Tobacco and Firearms Class Code
    CodeB8,
    ///BC - Behind-the-Counter Drugs
    BehindTheCounterDrugs,
    ///BCC - Beverage Contents Characteristics, (e.g., Kosher, No Sulfites, etc. (Industry List)
    CodeBCC,
    ///BE - Benefits
    Benefits,
    ///BES - Beverage Segment
    BeverageSegment,
    ///BEV - Beverage Category
    BeverageCategory,
    ///BF - Backflush Item Process
    BackflushItemProcess,
    ///BHZ - Biomedical Hazard
    BiomedicalHazard,
    ///BL - Blocked Stock
    BlockedStock,
    ///BLM - Bottomhole Location Method Code
    BottomholeLocationMethodCode,
    ///BND - Brand Group
    BrandGroup,
    ///BPI - Bottomhole Pressure Method Indicator Code
    BottomholePressureMethodIndicatorCode,
    ///BRG - Brand Group: A grouping of similar brands, (e.g., Johnnie Walker)
    CodeBRG,
    ///BRN - Brand Name
    BrandName,
    ///BTT - Battery Type
    BatteryType,
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
    ///CG - Color Grouping
    ColorGrouping,
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
    ///CN - Collection Name
    CollectionName,
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
    ///DG - Dangerous Goods Packing Group
    DangerousGoodsPackingGroup,
    ///DI - Diet Type
    DietType,
    ///DIR - Directional Indicator
    DirectionalIndicator,
    ///DM - Dimensional
    Dimensional,
    ///DP - Drug Product Identification
    DrugProductIdentification,
    ///DR - Direct Item Process
    DirectItemProcess,
    ///DS - Drug Schedule
    DrugSchedule,
    ///DT - Dangerous Goods Technical Name
    DangerousGoodsTechnicalName,
    ///EBC - Description of Goods and Services
    DescriptionOfGoodsAndServices,
    ///EC - Escrow Code
    EscrowCode,
    ///EN - Engine with Transmission
    EngineWithTransmission,
    ///EQ - Equipment
    Equipment,
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
    ///FP - Foodservice Program
    FoodserviceProgram,
    ///FQ - Quality
    Quality,
    ///FUN - Functional Name
    FunctionalName,
    ///GD - Grain Direction
    GrainDirection,
    ///GE - Genetically Modified Ingredients
    GeneticallyModifiedIngredients,
    ///GEN - General Description
    GeneralDescription,
    ///GM - General Merchandise
    GeneralMerchandise,
    ///GRM - Growing Method
    GrowingMethod,
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
    ///IC - Individual Component
    IndividualComponent,
    ///IML - Image Link
    ImageLink,
    ///ING - Ingredient
    Ingredient,
    ///INJ - Injectables
    Injectables,
    ///KI - Kit
    Kit,
    ///LA - Label Name
    LabelName,
    ///LC - Lead/Copper Sample Type
    LeadCopperSampleType,
    ///LO - Coordinate Description Code
    CoordinateDescriptionCode,
    ///LS - Local Source
    LocalSource,
    ///LSC - Label Storage Conditions
    LabelStorageConditions,
    ///M1 - Method of Catch
    MethodOfCatch,
    ///MA - Material Status, Outside Processor
    CodeMA,
    ///MAC - Material Classification
    MaterialClassification,
    ///MB - Marking
    Marking,
    ///MBU - Minerals Management Service/Bureau of Land Management (Indian Land) Property/Unit Number
    CodeMBU,
    ///MC - Markings Front
    MarkingsFront,
    ///MCL - Material Color
    MaterialColor,
    ///MD - Markings Back
    MarkingsBack,
    ///ME - Markings Alternate
    MarkingsAlternate,
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
    ///PC - Product Classification
    ProductClassification,
    ///PD - Physical Form: As Diluted
    PhysicalFormAsDiluted,
    ///PER - Program Performance
    ProgramPerformance,
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
    ///PKG - Packaging Materials
    PackagingMaterials,
    ///PL - Product Type
    ProductType,
    ///PP - Process/Production Unit
    ProcessProductionUnit,
    ///PR - Manufacturing Process
    ManufacturingProcess,
    ///PRC - Primary Component
    PrimaryComponent,
    ///PRI - Grape Variety
    GrapeVariety,
    ///PRO - Proprietary
    Proprietary,
    ///PSC - Pipeline Stream
    PipelineStream,
    ///PTC - Preparation Type
    PreparationType,
    ///PUB - Public Information
    Public,
    ///PUR - Pure Form
    PureForm,
    ///QAS - Quality Assurance Status
    QualityAssuranceStatus,
    ///QI - Quality Inspection Stock
    QualityInspectionStock,
    ///R3 - Proof
    Proof,
    ///RA - Route of Administration
    RouteOfAdministration,
    ///RCC - Reservoir Code (Company)
    CodeRCC,
    ///RG - Return Goods
    ReturnGoods,
    ///RM - Results Method Code
    ResultsMethodCode,
    ///RR - Rejection Reason
    RejectionReason,
    ///RSD - Regulatory (State) District
    CodeRSD,
    ///RSE - Regulatory (State) Entity Code
    CodeRSE,
    ///RT - Returnable
    Returnable,
    ///RX - Prescription Drug
    PrescriptionDrug,
    ///SB - Sub-Brand
    SubBrand,
    ///SC - Source
    Source,
    ///SE - Services
    Services,
    ///SEC - Secret or Confidential Information
    SecretOrConfidential,
    ///SEM - Semi-Finished Goods
    SemiFinishedGoods,
    ///SF - Service Feature
    ServiceFeature,
    ///SH - Trade Item Shape Description
    TradeItemShapeDescription,
    ///SIZ - Sizing
    Sizing,
    ///SLM - Surface Location Method Code
    SurfaceLocationMethodCode,
    ///SOL - Solubility
    Solubility,
    ///SS - Serving Suggestions
    ServingSuggestions,
    ///ST - Sample Type
    SampleType,
    ///STL - State Controlled
    StateControlled,
    ///SUP - Packaging Support Materials
    PackagingSupportMaterials,
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
    ///TIU - Trade Item Unit Indicator
    TradeItemUnitIndicator,
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
    ///UN - Unscheduled Drug
    UnscheduledDrug,
    ///UR - Unrestricted Stock
    UnrestrictedStock,
    ///VA - Vehicle
    Vehicle,
    ///VC - Volatile Organic Compound Control
    VolatileOrganicCompoundControl,
    ///VI - Vintage
    Vintage,
    ///VR - Variant
    Variant,
    ///WD - Warranty Description
    WarrantyDescription,
    ///WF - Wine Fruit
    WineFruit,
    ///WLC - Well Classification
    WellClassification,
    ///WS1 - Variable Weight Scale Line 1
    VariableWeightScaleLine1,
    ///WS2 - Variable Weight Scale Line 2
    VariableWeightScaleLine2,
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
                CompositionClaimMadeWith => "AA1",
                CompositionClaimNaturalIngredients => "AA2",
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
                AnaerobicSpores => "AN",
                Allergen => "AR",
                AerobicSpores => "AS",
                ProcessActionTaken => "AT",
                CodeB8 => "B8",
                BehindTheCounterDrugs => "BC",
                CodeBCC => "BCC",
                Benefits => "BE",
                BeverageSegment => "BES",
                BeverageCategory => "BEV",
                BackflushItemProcess => "BF",
                BiomedicalHazard => "BHZ",
                BlockedStock => "BL",
                BottomholeLocationMethodCode => "BLM",
                BrandGroup => "BND",
                BottomholePressureMethodIndicatorCode => "BPI",
                CodeBRG => "BRG",
                BrandName => "BRN",
                BatteryType => "BTT",
                BasisWeightSize => "BW",
                ControlledSubstanceClass2 => "C2",
                CodeC3 => "C3",
                ControlledSubstanceClass4 => "C4",
                ControlledSubstanceClass5 => "C5",
                CodeC6 => "C6",
                CommonChemicalName => "CCN",
                CollateralDescription => "CD",
                CompanyFieldCode => "CFC",
                ColorGrouping => "CG",
                Chassis => "CH",
                ChemicalFamily => "CHF",
                ColorLowerBody => "CL",
                CasingLinerTubingType => "CLT",
                ComplianceMethod => "CM",
                CommercialStatus => "CMS",
                CollectionName => "CN",
                CollectionMethodCode => "CO",
                Coupling => "CP",
                CoatingOrPaintSystemCode => "CS",
                ColorUpperBody => "CU",
                CoatingOrPaintSystemName => "CW",
                DamageCode => "DAC",
                DamageFault => "DAF",
                DrugEfficacyStudyImplementation => "DE",
                DosageForm => "DF",
                DangerousGoodsPackingGroup => "DG",
                DietType => "DI",
                DirectionalIndicator => "DIR",
                Dimensional => "DM",
                DrugProductIdentification => "DP",
                DirectItemProcess => "DR",
                DrugSchedule => "DS",
                DangerousGoodsTechnicalName => "DT",
                DescriptionOfGoodsAndServices => "EBC",
                EscrowCode => "EC",
                EngineWithTransmission => "EN",
                Equipment => "EQ",
                EnvironmentalRequirement => "EV",
                FailureAnalysisProcess => "FA",
                FoldConfigurations => "FC",
                CodeFCD => "FCD",
                ForecastDeviation => "FDD",
                Fuel => "FL",
                Flavor => "FLV",
                Formula => "FMR",
                FoodserviceProgram => "FP",
                Quality => "FQ",
                FunctionalName => "FUN",
                GrainDirection => "GD",
                GeneticallyModifiedIngredients => "GE",
                GeneralDescription => "GEN",
                GeneralMerchandise => "GM",
                GrowingMethod => "GRM",
                Goods => "GS",
                HealthAndBeautyAids => "HB",
                Hydraulics => "HY",
                HazardousMaterial => "HZ",
                HazardRatingSystem => "HZR",
                IndividualComponent => "IC",
                ImageLink => "IML",
                Ingredient => "ING",
                Injectables => "INJ",
                Kit => "KI",
                LabelName => "LA",
                LeadCopperSampleType => "LC",
                CoordinateDescriptionCode => "LO",
                LocalSource => "LS",
                LabelStorageConditions => "LSC",
                MethodOfCatch => "M1",
                CodeMA => "MA",
                MaterialClassification => "MAC",
                Marking => "MB",
                CodeMBU => "MBU",
                MarkingsFront => "MC",
                MaterialColor => "MCL",
                MarkingsBack => "MD",
                MarkingsAlternate => "ME",
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
                ProductClassification => "PC",
                PhysicalFormAsDiluted => "PD",
                ProgramPerformance => "PER",
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
                PackagingMaterials => "PKG",
                ProductType => "PL",
                ProcessProductionUnit => "PP",
                ManufacturingProcess => "PR",
                PrimaryComponent => "PRC",
                GrapeVariety => "PRI",
                Proprietary => "PRO",
                PipelineStream => "PSC",
                PreparationType => "PTC",
                Public => "PUB",
                PureForm => "PUR",
                QualityAssuranceStatus => "QAS",
                QualityInspectionStock => "QI",
                Proof => "R3",
                RouteOfAdministration => "RA",
                CodeRCC => "RCC",
                ReturnGoods => "RG",
                ResultsMethodCode => "RM",
                RejectionReason => "RR",
                CodeRSD => "RSD",
                CodeRSE => "RSE",
                Returnable => "RT",
                PrescriptionDrug => "RX",
                SubBrand => "SB",
                Source => "SC",
                Services => "SE",
                SecretOrConfidential => "SEC",
                SemiFinishedGoods => "SEM",
                ServiceFeature => "SF",
                TradeItemShapeDescription => "SH",
                Sizing => "SIZ",
                SurfaceLocationMethodCode => "SLM",
                Solubility => "SOL",
                ServingSuggestions => "SS",
                SampleType => "ST",
                StateControlled => "STL",
                PackagingSupportMaterials => "SUP",
                Synonym => "SYN",
                TherapeuticClass => "TC",
                TherapeuticEquivalencyEvaluation => "TE",
                Filtering => "TF",
                Threshold => "THR",
                TitleInsuranceForm => "TIF",
                Tire => "TIR",
                TradeItemUnitIndicator => "TIU",
                Typeface => "TP",
                Trimming => "TR",
                TestRemarks => "TRM",
                TradeName => "TRN",
                TheoreticalWeightFormula => "TWF",
                CodeTZ => "TZ",
                UnscheduledDrug => "UN",
                UnrestrictedStock => "UR",
                Vehicle => "VA",
                VolatileOrganicCompoundControl => "VC",
                Vintage => "VI",
                Variant => "VR",
                WarrantyDescription => "WD",
                WineFruit => "WF",
                WellClassification => "WLC",
                VariableWeightScaleLine1 => "WS1",
                VariableWeightScaleLine2 => "WS2",
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
            b"AA1" => Some(CompositionClaimMadeWith),
            b"AA2" => Some(CompositionClaimNaturalIngredients),
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
            b"AN" => Some(AnaerobicSpores),
            b"AR" => Some(Allergen),
            b"AS" => Some(AerobicSpores),
            b"AT" => Some(ProcessActionTaken),
            b"B8" => Some(CodeB8),
            b"BC" => Some(BehindTheCounterDrugs),
            b"BCC" => Some(CodeBCC),
            b"BE" => Some(Benefits),
            b"BES" => Some(BeverageSegment),
            b"BEV" => Some(BeverageCategory),
            b"BF" => Some(BackflushItemProcess),
            b"BHZ" => Some(BiomedicalHazard),
            b"BL" => Some(BlockedStock),
            b"BLM" => Some(BottomholeLocationMethodCode),
            b"BND" => Some(BrandGroup),
            b"BPI" => Some(BottomholePressureMethodIndicatorCode),
            b"BRG" => Some(CodeBRG),
            b"BRN" => Some(BrandName),
            b"BTT" => Some(BatteryType),
            b"BW" => Some(BasisWeightSize),
            b"C2" => Some(ControlledSubstanceClass2),
            b"C3" => Some(CodeC3),
            b"C4" => Some(ControlledSubstanceClass4),
            b"C5" => Some(ControlledSubstanceClass5),
            b"C6" => Some(CodeC6),
            b"CCN" => Some(CommonChemicalName),
            b"CD" => Some(CollateralDescription),
            b"CFC" => Some(CompanyFieldCode),
            b"CG" => Some(ColorGrouping),
            b"CH" => Some(Chassis),
            b"CHF" => Some(ChemicalFamily),
            b"CL" => Some(ColorLowerBody),
            b"CLT" => Some(CasingLinerTubingType),
            b"CM" => Some(ComplianceMethod),
            b"CMS" => Some(CommercialStatus),
            b"CN" => Some(CollectionName),
            b"CO" => Some(CollectionMethodCode),
            b"CP" => Some(Coupling),
            b"CS" => Some(CoatingOrPaintSystemCode),
            b"CU" => Some(ColorUpperBody),
            b"CW" => Some(CoatingOrPaintSystemName),
            b"DAC" => Some(DamageCode),
            b"DAF" => Some(DamageFault),
            b"DE" => Some(DrugEfficacyStudyImplementation),
            b"DF" => Some(DosageForm),
            b"DG" => Some(DangerousGoodsPackingGroup),
            b"DI" => Some(DietType),
            b"DIR" => Some(DirectionalIndicator),
            b"DM" => Some(Dimensional),
            b"DP" => Some(DrugProductIdentification),
            b"DR" => Some(DirectItemProcess),
            b"DS" => Some(DrugSchedule),
            b"DT" => Some(DangerousGoodsTechnicalName),
            b"EBC" => Some(DescriptionOfGoodsAndServices),
            b"EC" => Some(EscrowCode),
            b"EN" => Some(EngineWithTransmission),
            b"EQ" => Some(Equipment),
            b"EV" => Some(EnvironmentalRequirement),
            b"FA" => Some(FailureAnalysisProcess),
            b"FC" => Some(FoldConfigurations),
            b"FCD" => Some(CodeFCD),
            b"FDD" => Some(ForecastDeviation),
            b"FL" => Some(Fuel),
            b"FLV" => Some(Flavor),
            b"FMR" => Some(Formula),
            b"FP" => Some(FoodserviceProgram),
            b"FQ" => Some(Quality),
            b"FUN" => Some(FunctionalName),
            b"GD" => Some(GrainDirection),
            b"GE" => Some(GeneticallyModifiedIngredients),
            b"GEN" => Some(GeneralDescription),
            b"GM" => Some(GeneralMerchandise),
            b"GRM" => Some(GrowingMethod),
            b"GS" => Some(Goods),
            b"HB" => Some(HealthAndBeautyAids),
            b"HY" => Some(Hydraulics),
            b"HZ" => Some(HazardousMaterial),
            b"HZR" => Some(HazardRatingSystem),
            b"IC" => Some(IndividualComponent),
            b"IML" => Some(ImageLink),
            b"ING" => Some(Ingredient),
            b"INJ" => Some(Injectables),
            b"KI" => Some(Kit),
            b"LA" => Some(LabelName),
            b"LC" => Some(LeadCopperSampleType),
            b"LO" => Some(CoordinateDescriptionCode),
            b"LS" => Some(LocalSource),
            b"LSC" => Some(LabelStorageConditions),
            b"M1" => Some(MethodOfCatch),
            b"MA" => Some(CodeMA),
            b"MAC" => Some(MaterialClassification),
            b"MB" => Some(Marking),
            b"MBU" => Some(CodeMBU),
            b"MC" => Some(MarkingsFront),
            b"MCL" => Some(MaterialColor),
            b"MD" => Some(MarkingsBack),
            b"ME" => Some(MarkingsAlternate),
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
            b"PC" => Some(ProductClassification),
            b"PD" => Some(PhysicalFormAsDiluted),
            b"PER" => Some(ProgramPerformance),
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
            b"PKG" => Some(PackagingMaterials),
            b"PL" => Some(ProductType),
            b"PP" => Some(ProcessProductionUnit),
            b"PR" => Some(ManufacturingProcess),
            b"PRC" => Some(PrimaryComponent),
            b"PRI" => Some(GrapeVariety),
            b"PRO" => Some(Proprietary),
            b"PSC" => Some(PipelineStream),
            b"PTC" => Some(PreparationType),
            b"PUB" => Some(Public),
            b"PUR" => Some(PureForm),
            b"QAS" => Some(QualityAssuranceStatus),
            b"QI" => Some(QualityInspectionStock),
            b"R3" => Some(Proof),
            b"RA" => Some(RouteOfAdministration),
            b"RCC" => Some(CodeRCC),
            b"RG" => Some(ReturnGoods),
            b"RM" => Some(ResultsMethodCode),
            b"RR" => Some(RejectionReason),
            b"RSD" => Some(CodeRSD),
            b"RSE" => Some(CodeRSE),
            b"RT" => Some(Returnable),
            b"RX" => Some(PrescriptionDrug),
            b"SB" => Some(SubBrand),
            b"SC" => Some(Source),
            b"SE" => Some(Services),
            b"SEC" => Some(SecretOrConfidential),
            b"SEM" => Some(SemiFinishedGoods),
            b"SF" => Some(ServiceFeature),
            b"SH" => Some(TradeItemShapeDescription),
            b"SIZ" => Some(Sizing),
            b"SLM" => Some(SurfaceLocationMethodCode),
            b"SOL" => Some(Solubility),
            b"SS" => Some(ServingSuggestions),
            b"ST" => Some(SampleType),
            b"STL" => Some(StateControlled),
            b"SUP" => Some(PackagingSupportMaterials),
            b"SYN" => Some(Synonym),
            b"TC" => Some(TherapeuticClass),
            b"TE" => Some(TherapeuticEquivalencyEvaluation),
            b"TF" => Some(Filtering),
            b"THR" => Some(Threshold),
            b"TIF" => Some(TitleInsuranceForm),
            b"TIR" => Some(Tire),
            b"TIU" => Some(TradeItemUnitIndicator),
            b"TP" => Some(Typeface),
            b"TR" => Some(Trimming),
            b"TRM" => Some(TestRemarks),
            b"TRN" => Some(TradeName),
            b"TWF" => Some(TheoreticalWeightFormula),
            b"TZ" => Some(CodeTZ),
            b"UN" => Some(UnscheduledDrug),
            b"UR" => Some(UnrestrictedStock),
            b"VA" => Some(Vehicle),
            b"VC" => Some(VolatileOrganicCompoundControl),
            b"VI" => Some(Vintage),
            b"VR" => Some(Variant),
            b"WD" => Some(WarrantyDescription),
            b"WF" => Some(WineFruit),
            b"WLC" => Some(WellClassification),
            b"WS1" => Some(VariableWeightScaleLine1),
            b"WS2" => Some(VariableWeightScaleLine2),
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
            CompositionClaimMadeWith => "Composition Claim Made With",
            CompositionClaimNaturalIngredients => "Composition Claim Natural Ingredients",
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
            AnaerobicSpores => "Anaerobic Spores",
            Allergen => "Allergen",
            AerobicSpores => "Aerobic Spores",
            ProcessActionTaken => "Process Action Taken",
            CodeB8 => "Bureau of Alcohol, Tobacco and Firearms Class Code",
            BehindTheCounterDrugs => "Behind-the-Counter Drugs",
            CodeBCC => {
                "Beverage Contents Characteristics, (e.g., Kosher, No Sulfites, etc. (Industry List)"
            }
            Benefits => "Benefits",
            BeverageSegment => "Beverage Segment",
            BeverageCategory => "Beverage Category",
            BackflushItemProcess => "Backflush Item Process",
            BiomedicalHazard => "Biomedical Hazard",
            BlockedStock => "Blocked Stock",
            BottomholeLocationMethodCode => "Bottomhole Location Method Code",
            BrandGroup => "Brand Group",
            BottomholePressureMethodIndicatorCode => {
                "Bottomhole Pressure Method Indicator Code"
            }
            CodeBRG => {
                "Brand Group: A grouping of similar brands, (e.g., Johnnie Walker)"
            }
            BrandName => "Brand Name",
            BatteryType => "Battery Type",
            BasisWeightSize => "Basis Weight Size",
            ControlledSubstanceClass2 => "Controlled Substance - Class 2",
            CodeC3 => "Controlled Substance-Class 3 (Narcotic)",
            ControlledSubstanceClass4 => "Controlled Substance - Class 4",
            ControlledSubstanceClass5 => "Controlled Substance - Class 5",
            CodeC6 => "Controlled Substance-Class 3N (Non-narcotic)",
            CommonChemicalName => "Common Chemical Name",
            CollateralDescription => "Collateral Description",
            CompanyFieldCode => "Company Field Code",
            ColorGrouping => "Color Grouping",
            Chassis => "Chassis",
            ChemicalFamily => "Chemical Family",
            ColorLowerBody => "Color - Lower Body",
            CasingLinerTubingType => "Casing/Liner/Tubing Type",
            ComplianceMethod => "Compliance Method",
            CommercialStatus => "Commercial Status",
            CollectionName => "Collection Name",
            CollectionMethodCode => "Collection Method Code",
            Coupling => "Coupling",
            CoatingOrPaintSystemCode => "Coating or Paint System Code",
            ColorUpperBody => "Color - Upper Body",
            CoatingOrPaintSystemName => "Coating or Paint System Name",
            DamageCode => "Damage Code",
            DamageFault => "Damage Fault",
            DrugEfficacyStudyImplementation => "Drug Efficacy Study Implementation",
            DosageForm => "Dosage Form",
            DangerousGoodsPackingGroup => "Dangerous Goods Packing Group",
            DietType => "Diet Type",
            DirectionalIndicator => "Directional Indicator",
            Dimensional => "Dimensional",
            DrugProductIdentification => "Drug Product Identification",
            DirectItemProcess => "Direct Item Process",
            DrugSchedule => "Drug Schedule",
            DangerousGoodsTechnicalName => "Dangerous Goods Technical Name",
            DescriptionOfGoodsAndServices => "Description of Goods and Services",
            EscrowCode => "Escrow Code",
            EngineWithTransmission => "Engine with Transmission",
            Equipment => "Equipment",
            EnvironmentalRequirement => "Environmental Requirement",
            FailureAnalysisProcess => "Failure Analysis Process",
            FoldConfigurations => "Fold Configurations",
            CodeFCD => "Field Code (EIA/DOD)",
            ForecastDeviation => "Forecast Deviation",
            Fuel => "Fuel",
            Flavor => "Flavor",
            Formula => "Formula",
            FoodserviceProgram => "Foodservice Program",
            Quality => "Quality",
            FunctionalName => "Functional Name",
            GrainDirection => "Grain Direction",
            GeneticallyModifiedIngredients => "Genetically Modified Ingredients",
            GeneralDescription => "General Description",
            GeneralMerchandise => "General Merchandise",
            GrowingMethod => "Growing Method",
            Goods => "Goods",
            HealthAndBeautyAids => "Health and Beauty Aids",
            Hydraulics => "Hydraulics",
            HazardousMaterial => "Hazardous Material",
            HazardRatingSystem => "Hazard Rating System",
            IndividualComponent => "Individual Component",
            ImageLink => "Image Link",
            Ingredient => "Ingredient",
            Injectables => "Injectables",
            Kit => "Kit",
            LabelName => "Label Name",
            LeadCopperSampleType => "Lead/Copper Sample Type",
            CoordinateDescriptionCode => "Coordinate Description Code",
            LocalSource => "Local Source",
            LabelStorageConditions => "Label Storage Conditions",
            MethodOfCatch => "Method of Catch",
            CodeMA => "Material Status, Outside Processor",
            MaterialClassification => "Material Classification",
            Marking => "Marking",
            CodeMBU => {
                "Minerals Management Service/Bureau of Land Management (Indian Land) Property/Unit Number"
            }
            MarkingsFront => "Markings Front",
            MaterialColor => "Material Color",
            MarkingsBack => "Markings Back",
            MarkingsAlternate => "Markings Alternate",
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
            ProductClassification => "Product Classification",
            PhysicalFormAsDiluted => "Physical Form: As Diluted",
            ProgramPerformance => "Program Performance",
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
            PackagingMaterials => "Packaging Materials",
            ProductType => "Product Type",
            ProcessProductionUnit => "Process/Production Unit",
            ManufacturingProcess => "Manufacturing Process",
            PrimaryComponent => "Primary Component",
            GrapeVariety => "Grape Variety",
            Proprietary => "Proprietary",
            PipelineStream => "Pipeline Stream",
            PreparationType => "Preparation Type",
            Public => "Public Information",
            PureForm => "Pure Form",
            QualityAssuranceStatus => "Quality Assurance Status",
            QualityInspectionStock => "Quality Inspection Stock",
            Proof => "Proof",
            RouteOfAdministration => "Route of Administration",
            CodeRCC => "Reservoir Code (Company)",
            ReturnGoods => "Return Goods",
            ResultsMethodCode => "Results Method Code",
            RejectionReason => "Rejection Reason",
            CodeRSD => "Regulatory (State) District",
            CodeRSE => "Regulatory (State) Entity Code",
            Returnable => "Returnable",
            PrescriptionDrug => "Prescription Drug",
            SubBrand => "Sub-Brand",
            Source => "Source",
            Services => "Services",
            SecretOrConfidential => "Secret or Confidential Information",
            SemiFinishedGoods => "Semi-Finished Goods",
            ServiceFeature => "Service Feature",
            TradeItemShapeDescription => "Trade Item Shape Description",
            Sizing => "Sizing",
            SurfaceLocationMethodCode => "Surface Location Method Code",
            Solubility => "Solubility",
            ServingSuggestions => "Serving Suggestions",
            SampleType => "Sample Type",
            StateControlled => "State Controlled",
            PackagingSupportMaterials => "Packaging Support Materials",
            Synonym => "Synonym",
            TherapeuticClass => "Therapeutic Class",
            TherapeuticEquivalencyEvaluation => "Therapeutic Equivalency Evaluation",
            Filtering => "Filtering",
            Threshold => "Threshold",
            TitleInsuranceForm => "Title Insurance Form",
            Tire => "Tire",
            TradeItemUnitIndicator => "Trade Item Unit Indicator",
            Typeface => "Typeface",
            Trimming => "Trimming",
            TestRemarks => "Test Remarks",
            TradeName => "Trade Name",
            TheoreticalWeightFormula => "Theoretical Weight Formula",
            CodeTZ => "Bureau of Alcohol, Tobacco and Firearms Type Code",
            UnscheduledDrug => "Unscheduled Drug",
            UnrestrictedStock => "Unrestricted Stock",
            Vehicle => "Vehicle",
            VolatileOrganicCompoundControl => "Volatile Organic Compound Control",
            Vintage => "Vintage",
            Variant => "Variant",
            WarrantyDescription => "Warranty Description",
            WineFruit => "Wine Fruit",
            WellClassification => "Well Classification",
            VariableWeightScaleLine1 => "Variable Weight Scale Line 1",
            VariableWeightScaleLine2 => "Variable Weight Scale Line 2",
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
                "Composition Claim Made With" => Some(CompositionClaimMadeWith),
                "Composition Claim Natural Ingredients" => {
                    Some(CompositionClaimNaturalIngredients)
                }
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
                "Anaerobic Spores" => Some(AnaerobicSpores),
                "Allergen" => Some(Allergen),
                "Aerobic Spores" => Some(AerobicSpores),
                "Process Action Taken" => Some(ProcessActionTaken),
                "Bureau of Alcohol, Tobacco and Firearms Class Code" => Some(CodeB8),
                "Behind-the-Counter Drugs" => Some(BehindTheCounterDrugs),
                "Beverage Contents Characteristics, (e.g., Kosher, No Sulfites, etc. (Industry List)" => {
                    Some(CodeBCC)
                }
                "Benefits" => Some(Benefits),
                "Beverage Segment" => Some(BeverageSegment),
                "Beverage Category" => Some(BeverageCategory),
                "Backflush Item Process" => Some(BackflushItemProcess),
                "Biomedical Hazard" => Some(BiomedicalHazard),
                "Blocked Stock" => Some(BlockedStock),
                "Bottomhole Location Method Code" => Some(BottomholeLocationMethodCode),
                "Brand Group" => Some(BrandGroup),
                "Bottomhole Pressure Method Indicator Code" => {
                    Some(BottomholePressureMethodIndicatorCode)
                }
                "Brand Group: A grouping of similar brands, (e.g., Johnnie Walker)" => {
                    Some(CodeBRG)
                }
                "Brand Name" => Some(BrandName),
                "Battery Type" => Some(BatteryType),
                "Basis Weight Size" => Some(BasisWeightSize),
                "Controlled Substance - Class 2" => Some(ControlledSubstanceClass2),
                "Controlled Substance-Class 3 (Narcotic)" => Some(CodeC3),
                "Controlled Substance - Class 4" => Some(ControlledSubstanceClass4),
                "Controlled Substance - Class 5" => Some(ControlledSubstanceClass5),
                "Controlled Substance-Class 3N (Non-narcotic)" => Some(CodeC6),
                "Common Chemical Name" => Some(CommonChemicalName),
                "Collateral Description" => Some(CollateralDescription),
                "Company Field Code" => Some(CompanyFieldCode),
                "Color Grouping" => Some(ColorGrouping),
                "Chassis" => Some(Chassis),
                "Chemical Family" => Some(ChemicalFamily),
                "Color - Lower Body" => Some(ColorLowerBody),
                "Casing/Liner/Tubing Type" => Some(CasingLinerTubingType),
                "Compliance Method" => Some(ComplianceMethod),
                "Commercial Status" => Some(CommercialStatus),
                "Collection Name" => Some(CollectionName),
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
                "Dangerous Goods Packing Group" => Some(DangerousGoodsPackingGroup),
                "Diet Type" => Some(DietType),
                "Directional Indicator" => Some(DirectionalIndicator),
                "Dimensional" => Some(Dimensional),
                "Drug Product Identification" => Some(DrugProductIdentification),
                "Direct Item Process" => Some(DirectItemProcess),
                "Drug Schedule" => Some(DrugSchedule),
                "Dangerous Goods Technical Name" => Some(DangerousGoodsTechnicalName),
                "Description of Goods and Services" => {
                    Some(DescriptionOfGoodsAndServices)
                }
                "Escrow Code" => Some(EscrowCode),
                "Engine with Transmission" => Some(EngineWithTransmission),
                "Equipment" => Some(Equipment),
                "Environmental Requirement" => Some(EnvironmentalRequirement),
                "Failure Analysis Process" => Some(FailureAnalysisProcess),
                "Fold Configurations" => Some(FoldConfigurations),
                "Field Code (EIA/DOD)" => Some(CodeFCD),
                "Forecast Deviation" => Some(ForecastDeviation),
                "Fuel" => Some(Fuel),
                "Flavor" => Some(Flavor),
                "Formula" => Some(Formula),
                "Foodservice Program" => Some(FoodserviceProgram),
                "Quality" => Some(Quality),
                "Functional Name" => Some(FunctionalName),
                "Grain Direction" => Some(GrainDirection),
                "Genetically Modified Ingredients" => {
                    Some(GeneticallyModifiedIngredients)
                }
                "General Description" => Some(GeneralDescription),
                "General Merchandise" => Some(GeneralMerchandise),
                "Growing Method" => Some(GrowingMethod),
                "Goods" => Some(Goods),
                "Health and Beauty Aids" => Some(HealthAndBeautyAids),
                "Hydraulics" => Some(Hydraulics),
                "Hazardous Material" => Some(HazardousMaterial),
                "Hazard Rating System" => Some(HazardRatingSystem),
                "Individual Component" => Some(IndividualComponent),
                "Image Link" => Some(ImageLink),
                "Ingredient" => Some(Ingredient),
                "Injectables" => Some(Injectables),
                "Kit" => Some(Kit),
                "Label Name" => Some(LabelName),
                "Lead/Copper Sample Type" => Some(LeadCopperSampleType),
                "Coordinate Description Code" => Some(CoordinateDescriptionCode),
                "Local Source" => Some(LocalSource),
                "Label Storage Conditions" => Some(LabelStorageConditions),
                "Method of Catch" => Some(MethodOfCatch),
                "Material Status, Outside Processor" => Some(CodeMA),
                "Material Classification" => Some(MaterialClassification),
                "Marking" => Some(Marking),
                "Minerals Management Service/Bureau of Land Management (Indian Land) Property/Unit Number" => {
                    Some(CodeMBU)
                }
                "Markings Front" => Some(MarkingsFront),
                "Material Color" => Some(MaterialColor),
                "Markings Back" => Some(MarkingsBack),
                "Markings Alternate" => Some(MarkingsAlternate),
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
                "Product Classification" => Some(ProductClassification),
                "Physical Form: As Diluted" => Some(PhysicalFormAsDiluted),
                "Program Performance" => Some(ProgramPerformance),
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
                "Packaging Materials" => Some(PackagingMaterials),
                "Product Type" => Some(ProductType),
                "Process/Production Unit" => Some(ProcessProductionUnit),
                "Manufacturing Process" => Some(ManufacturingProcess),
                "Primary Component" => Some(PrimaryComponent),
                "Grape Variety" => Some(GrapeVariety),
                "Proprietary" => Some(Proprietary),
                "Pipeline Stream" => Some(PipelineStream),
                "Preparation Type" => Some(PreparationType),
                "Public Information" => Some(Public),
                "Pure Form" => Some(PureForm),
                "Quality Assurance Status" => Some(QualityAssuranceStatus),
                "Quality Inspection Stock" => Some(QualityInspectionStock),
                "Proof" => Some(Proof),
                "Route of Administration" => Some(RouteOfAdministration),
                "Reservoir Code (Company)" => Some(CodeRCC),
                "Return Goods" => Some(ReturnGoods),
                "Results Method Code" => Some(ResultsMethodCode),
                "Rejection Reason" => Some(RejectionReason),
                "Regulatory (State) District" => Some(CodeRSD),
                "Regulatory (State) Entity Code" => Some(CodeRSE),
                "Returnable" => Some(Returnable),
                "Prescription Drug" => Some(PrescriptionDrug),
                "Sub-Brand" => Some(SubBrand),
                "Source" => Some(Source),
                "Services" => Some(Services),
                "Secret or Confidential Information" => Some(SecretOrConfidential),
                "Semi-Finished Goods" => Some(SemiFinishedGoods),
                "Service Feature" => Some(ServiceFeature),
                "Trade Item Shape Description" => Some(TradeItemShapeDescription),
                "Sizing" => Some(Sizing),
                "Surface Location Method Code" => Some(SurfaceLocationMethodCode),
                "Solubility" => Some(Solubility),
                "Serving Suggestions" => Some(ServingSuggestions),
                "Sample Type" => Some(SampleType),
                "State Controlled" => Some(StateControlled),
                "Packaging Support Materials" => Some(PackagingSupportMaterials),
                "Synonym" => Some(Synonym),
                "Therapeutic Class" => Some(TherapeuticClass),
                "Therapeutic Equivalency Evaluation" => {
                    Some(TherapeuticEquivalencyEvaluation)
                }
                "Filtering" => Some(Filtering),
                "Threshold" => Some(Threshold),
                "Title Insurance Form" => Some(TitleInsuranceForm),
                "Tire" => Some(Tire),
                "Trade Item Unit Indicator" => Some(TradeItemUnitIndicator),
                "Typeface" => Some(Typeface),
                "Trimming" => Some(Trimming),
                "Test Remarks" => Some(TestRemarks),
                "Trade Name" => Some(TradeName),
                "Theoretical Weight Formula" => Some(TheoreticalWeightFormula),
                "Bureau of Alcohol, Tobacco and Firearms Type Code" => Some(CodeTZ),
                "Unscheduled Drug" => Some(UnscheduledDrug),
                "Unrestricted Stock" => Some(UnrestrictedStock),
                "Vehicle" => Some(Vehicle),
                "Volatile Organic Compound Control" => {
                    Some(VolatileOrganicCompoundControl)
                }
                "Vintage" => Some(Vintage),
                "Variant" => Some(Variant),
                "Warranty Description" => Some(WarrantyDescription),
                "Wine Fruit" => Some(WineFruit),
                "Well Classification" => Some(WellClassification),
                "Variable Weight Scale Line 1" => Some(VariableWeightScaleLine1),
                "Variable Weight Scale Line 2" => Some(VariableWeightScaleLine2),
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