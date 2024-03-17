use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**235

See docs at <https://www.stedi.com/edi/x12/element/235>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProductServiceIdQualifier {
    ///00 - Credit Card
    CreditCard,
    ///A1 - Plant Equipment Number
    PlantEquipmentNumber,
    ///A2 - Department of Defense Identification Code (DoDIC)
    CodeA2,
    ///A3 - Locally Assigned Control Number
    LocallyAssignedControlNumber,
    ///A4 - Subsistence Identification Number
    SubsistenceIdentificationNumber,
    ///A5 - Application State or Province
    ApplicationStateOrProvince,
    ///A6 - Document Identification Code
    DocumentIdentificationCode,
    ///A7 - Subline Item Number
    SublineItemNumber,
    ///A8 - Exhibit Line Item Number
    ExhibitLineItemNumber,
    ///A9 - Activity
    Activity,
    ///AA - Assembly Level Code, Vehicle Maintenance Reporting Standards (VMRS) 2000 Code Key 32
    CodeAA,
    ///AB - Assembly
    Assembly,
    ///AC - Aggregation Code (Used to Consolidate Part Families)
    CodeAC,
    ///AD - American Dental Association Codes
    AmericanDentalAssociationCodes,
    ///AE - Serial Item and Contribution Identifier (Defined in ANSI 239.56)
    CodeAE,
    ///AF - Front Axle Serial Number
    FrontAxleSerialNumber,
    ///AG - Age
    Age,
    ///AH - Billboards Commercial
    BillboardsCommercial,
    ///AI - Alternate ISBN
    AlternateIsbn,
    ///AJ - Piggyback Commercial
    PiggybackCommercial,
    ///AK - Refined Product Code
    RefinedProductCode,
    ///AL - Axle Serial Number
    AxleSerialNumber,
    ///AM - Allocation Methodology Identification Code
    AllocationMethodologyIdentificationCode,
    ///AN - Asset Number
    AssetNumber,
    ///AO - Assemblage Identification Number
    AssemblageIdentificationNumber,
    ///AP - Component Level Code, Vehicle Maintenance Reporting Standards (VMRS) 2000 Code Key 33
    CodeAP,
    ///AQ - Appraisal Product Type
    AppraisalProductType,
    ///AR - ARINC Part Number
    ArincPartNumber,
    ///AS - Rear Axle Serial Number
    RearAxleSerialNumber,
    ///AT - Assortability Code
    AssortabilityCode,
    ///AU - Appraisal Service
    AppraisalService,
    ///AV - Additional Product Identification Assigned by the Manufacturer
    AdditionalProductIdentificationAssignedByTheManufacturer,
    ///AW - Well Number
    WellNumber,
    ///AX - Assembly Lot Number
    AssemblyLotNumber,
    ///AY - System Level Code, Vehicle Maintenance Reporting Standards (VMRS) 2000 Code Key 31
    CodeAY,
    ///AZ - Axle Ratio
    AxleRatio,
    ///B1 - Volume Set
    VolumeSet,
    ///B2 - Library of Congress Code
    LibraryOfCongressCode,
    ///B3 - Edition Code
    EditionCode,
    ///B4 - Binding Code
    BindingCode,
    ///B5 - Title Code
    TitleCode,
    ///B6 - Author Code
    AuthorCode,
    ///B7 - Publisher Code
    PublisherCode,
    ///B8 - Batch Number
    BatchNumber,
    ///B9 - Front Axle Driven Serial Number
    FrontAxleDrivenSerialNumber,
    ///BA - Bale Number
    BaleNumber,
    ///BB - Print Color Standard Reference
    PrintColorStandardReference,
    ///BC - Supplier Brand Code
    SupplierBrandCode,
    ///BD - Buyer Ground Shade Identifier
    BuyerGroundShade,
    ///BE - Buyer's Engineering Change Level Number
    BuyersEngineeringChangeLevelNumber,
    ///BF - Vendor Ground Shade Identifier
    VendorGroundShade,
    ///BG - Ground Shade Standard Reference
    GroundShadeStandardReference,
    ///BH - Finish/Hand Standard Reference
    FinishHandStandardReference,
    ///BI - Rear Axle Rear Non Driven Serial Number
    RearAxleRearNonDrivenSerialNumber,
    ///BJ - Application Completion Method
    ApplicationCompletionMethod,
    ///BK - Backing
    Backing,
    ///BL - Brand/Label
    BrandLabel,
    ///BM - Beam Number
    BeamNumber,
    ///BN - Bar-Coded Serial Number
    BarCodedSerialNumber,
    ///BO - Buyers Color
    BuyersColor,
    ///BP - Buyer's Part Number
    BuyersPartNumber,
    ///BQ - Benefit ID
    BenefitId,
    ///BR - Compensation Type Code
    CompensationTypeCode,
    ///BS - Bill Subgroup Code
    BillSubgroupCode,
    ///BT - Volume Type Identification Code
    VolumeTypeIdentificationCode,
    ///BU - Bus Body Serial Number
    BusBodySerialNumber,
    ///BV - Beverage Common Code
    BeverageCommonCode,
    ///BW - Compensation Allocation Code
    CompensationAllocationCode,
    ///BX - Bill Credit Code
    BillCreditCode,
    ///BY - Concept Code
    ConceptCode,
    ///BZ - Coverage Type
    CoverageType,
    ///C1 - Channel
    Channel,
    ///C2 - Connector Type
    ConnectorType,
    ///C3 - Classification
    Classification,
    ///C4 - Configuration Item Identifier
    ConfigurationItem,
    ///C5 - Insurance Plan Description Characteristics
    InsurancePlanDescriptionCharacteristics,
    ///C6 - Asset Type
    AssetType,
    ///C7 - Asset Category
    AssetCategory,
    ///C8 - Fund Sub-Advisor
    FundSubAdvisor,
    ///C9 - Dun & Bradstreet Standard Product and Service Code
    CodeC9,
    ///CA - Case
    Case,
    ///CB - Buyer's Catalog Number
    BuyersCatalogNumber,
    ///CC - Compatible Cut Number
    CompatibleCutNumber,
    ///CD - Motor Vehicle-Line Designator
    MotorVehicleLineDesignator,
    ///CE - Class of Contract Code
    ClassOfContractCode,
    ///CF - Chassis Serial Number
    ChassisSerialNumber,
    ///CG - Commodity Grouping
    CommodityGrouping,
    ///CH - Country of Origin Code
    CountryOfOriginCode,
    ///CI - Common Language Equipment Identifier (CLEI)
    CodeCI,
    ///CJ - Current Procedural Terminology (CPT) Codes
    CodeCJ,
    ///CK - Candidate Oil
    CandidateOil,
    ///CL - Color
    Color,
    ///CM - National Retail Federation Color Code
    NationalRetailFederationColorCode,
    ///CN - Commodity Name
    CommodityName,
    ///CO - Chemical Abstract Service (CAS) Registry Number
    CodeCO,
    ///CP - Carry-over Part Number
    CarryOverPartNumber,
    ///CQ - Equipment Code per COPAS standard
    EquipmentCodePerCopasStandard,
    ///CR - Contract Number
    ContractNumber,
    ///CS - Service code per COPAS standard
    ServiceCodePerCopasStandard,
    ///CT - Continuation (ID Number Spans Multiple Product ID Data Elements)
    CodeCT,
    ///CU - Cut Number
    CutNumber,
    ///CV - Customer Provided Equipment
    CustomerProvidedEquipment,
    ///CW - Contract Activity Code
    ContractActivityCode,
    ///CX - Completion Number
    CompletionNumber,
    ///CY - Customer Company Registry Number
    CustomerCompanyRegistryNumber,
    ///CZ - Country from which Procured
    CountryFromWhichProcured,
    ///D1 - Underwriting Method of Direct Writer
    UnderwritingMethodOfDirectWriter,
    ///D2 - Medical Information Bureau (MIB) Authorization
    CodeD2,
    ///D3 - Policy Form
    PolicyForm,
    ///D4 - Plan Code
    PlanCode,
    ///D5 - Coverage Risk Type
    CoverageRiskType,
    ///D6 - Medical Stop Loss Level Codes
    MedicalStopLossLevelCodes,
    ///D7 - Medical Stop Loss Coverage Codes
    MedicalStopLossCoverageCodes,
    ///D8 - Medical Stop Loss Product Line Codes
    MedicalStopLossProductLineCodes,
    ///D9 - RxNorm
    RxNorm,
    ///DA - Automobile Repair Product Code
    AutomobileRepairProductCode,
    ///DB - GS1 DataBar
    Gs1DataBar,
    ///DC - International Classification of Diseases, 10th Revision, Clinical Modification (ICD-10-CM)
    CodeDC,
    ///DD - Distributor
    Distributor,
    ///DE - Design Number
    DesignNumber,
    ///DF - Device Family
    DeviceFamily,
    ///DG - Discount Grouping
    DiscountGrouping,
    ///DH - Transport4 Commodity Code
    Transport4CommodityCode,
    ///DI - Deposit Item Number
    DepositItemNumber,
    ///DJ - Dependent Proprietary Product
    DependentProprietaryProduct,
    ///DK - Primary Alternate Product
    PrimaryAlternateProduct,
    ///DL - Dye Lot Number
    DyeLotNumber,
    ///DM - Committee for Uniform Security Identification Procedure Number (CUSIP) Number
    CodeDM,
    ///DN - Die Number
    DieNumber,
    ///DO - Dividend Use
    DividendUse,
    ///DP - Discontinued Part Number
    DiscontinuedPartNumber,
    ///DQ - Event Type
    EventType,
    ///DR - Drawing Revision Number
    DrawingRevisionNumber,
    ///DS - Group ID
    GroupId,
    ///DT - Device Type
    DeviceType,
    ///DU - Joint Life Type
    JointLifeType,
    ///DV - Location Code
    LocationCode,
    ///DW - Nonforfeiture Option
    NonforfeitureOption,
    ///DX - International Classification of Diseases, 9th Revision, Clinical Modification (ICD-9-CM) - Diagnosis
    CodeDX,
    ///DY - Premium Rate Type
    PremiumRateType,
    ///DZ - Diagnosis Code Pointer
    DiagnosisCodePointer,
    ///E1 - Contract Change Authorization Type
    ContractChangeAuthorizationType,
    ///E2 - Fund Abbreviation
    FundAbbreviation,
    ///E3 - Fund Type
    FundType,
    ///E4 - Related Policy Identification
    RelatedPolicyIdentification,
    ///E5 - Tax Code
    TaxCode,
    ///E6 - Contract Transfer Reason Code
    ContractTransferReasonCode,
    ///E7 - Berenson-Eggers Type of Service
    BerensonEggersTypeOfService,
    ///E8 - International Society of Blood Transfusion device identifier
    InternationalSocietyOfBloodTransfusionDeviceIdentifier,
    ///E9 - Human Cell, Tissue or Cellular or Tissue-Based Product code
    CodeE9,
    ///EA - EAN-99 In-store Coupon Code
    Ean99InStoreCouponCode,
    ///EB - Fuel Tank Serial Number
    FuelTankSerialNumber,
    ///EC - Engineering Change Level
    EngineeringChangeLevel,
    ///ED - Engine Displacement Identification
    EngineDisplacementIdentification,
    ///EE - Premium Use
    PremiumUse,
    ///EF - Exhibit Identifier
    Exhibit,
    ///EG - Purpose of Insurance
    PurposeOfInsurance,
    ///EH - Sales Presentation ID
    SalesPresentationId,
    ///EI - Expense Identifier
    Expense,
    ///EJ - Service Feature ID
    ServiceFeatureId,
    ///EK - Settlement/Payout Option
    SettlementPayoutOption,
    ///EL - Buyer's Subline Item Number
    BuyersSublineItemNumber,
    ///EM - Equipment Identification Number
    EquipmentIdentificationNumber,
    ///EN - GTIN-13
    Gtin13,
    ///EO - GTIN-8
    Gtin8,
    ///EP - Buyer's End Product Number
    BuyersEndProductNumber,
    ///EQ - Equipment Type
    EquipmentType,
    ///ER - Jurisdiction Specific Procedure and Supply Codes
    JurisdictionSpecificProcedureAndSupplyCodes,
    ///ES - Engine Serial Number
    EngineSerialNumber,
    ///ET - Department of Defense Enterprise Identifier
    DepartmentOfDefenseEnterprise,
    ///EU - Skill Code
    SkillCode,
    ///EV - Equivalent Product
    EquivalentProduct,
    ///EX - Exchanged Part, Assembly or Product
    CodeEX,
    ///EZ - Shift Worked
    ShiftWorked,
    ///F1 - Catalog Number
    CatalogNumber,
    ///F2 - Technical Order Number
    TechnicalOrderNumber,
    ///F3 - Technical Manual Number
    TechnicalManualNumber,
    ///F4 - Series Identifier
    Series,
    ///F5 - Obligation Authority Number
    ObligationAuthorityNumber,
    ///F6 - First Prior Identifier
    FirstPrior,
    ///F7 - End-Item Description
    EndItemDescription,
    ///F8 - Next Higher Used Assembly
    NextHigherUsedAssembly,
    ///F9 - Former Publisher
    FormerPublisher,
    ///FA - Failed Subassembly Serial Number
    FailedSubassemblySerialNumber,
    ///FB - Form Number
    FormNumber,
    ///FC - Coupon Family Code
    CouponFamilyCode,
    ///FD - Fund
    Fund,
    ///FE - Feature
    Feature,
    ///FF - Fifth Wheel Serial Number
    FifthWheelSerialNumber,
    ///FG - Fund Manager
    FundManager,
    ///FH - Freddie Mac Affordable Lending Product Code
    FreddieMacAffordableLendingProductCode,
    ///FI - Finish Number
    FinishNumber,
    ///FJ - Fannie Mae Affordable Lending Product Code
    FannieMaeAffordableLendingProductCode,
    ///FK - Freddie Mac Project Condominium Classification Code
    FreddieMacProjectCondominiumClassificationCode,
    ///FL - Finish Lot Number
    FinishLotNumber,
    ///FM - Failed Subassembly Model Number
    FailedSubassemblyModelNumber,
    ///FN - Final Test Lot Number
    FinalTestLotNumber,
    ///FP - Fabric Pieces Per Roll
    FabricPiecesPerRoll,
    ///FQ - Fannie Mae Project Condominium Classification Code
    FannieMaeProjectCondominiumClassificationCode,
    ///FR - Front Axle, Rear
    CodeFR,
    ///FS - National Stock Number
    NationalStockNumber,
    ///FT - Federal Supply Classification
    FederalSupplyClassification,
    ///FU - National Alcohol Beverage Control Association (NABCA) Product Code
    CodeFU,
    ///FV - Drug Identification Number (DIN)
    CodeFV,
    ///FW - New Microcode
    NewMicrocode,
    ///FX - Federal Supply Group
    FederalSupplyGroup,
    ///GA - Gathering
    Gathering,
    ///GC - Grade Code
    GradeCode,
    ///GD - Grain Direction
    GrainDirection,
    ///GE - Generic Name Description
    GenericNameDescription,
    ///GI - Graphics Industry Bar Code (GIBC)
    CodeGI,
    ///GK - Glider Kit
    GliderKit,
    ///GM - General Services Administration (GSA) Special Item Number
    CodeGM,
    ///GN - Grade Name
    GradeName,
    ///GQ - Group Qualifier Code
    GroupQualifierCode,
    ///GR - Gear Ratio
    GearRatio,
    ///GS - General Specification Number
    GeneralSpecificationNumber,
    ///GU - Volume Usage Identification Code
    VolumeUsageIdentificationCode,
    ///GV - Serialized Global Returnable Asset Identifier (GRAI)
    CodeGV,
    ///GW - Global Returnable Asset Identifier (GRAI)
    CodeGW,
    ///GX - Global Individual Asset Identifier (GIAI)
    CodeGX,
    ///HA - Country Subdivision Code
    CountrySubdivisionCode,
    ///HB - Country of Origin with Country Subdivision Code
    CountryOfOriginWithCountrySubdivisionCode,
    ///HC - Healthcare Common Procedure Coding System (HCPCS) Codes
    CodeHC,
    ///HD - International Harmonized Commodity Code
    InternationalHarmonizedCommodityCode,
    ///HF - Country Subdivision from where Procured
    CountrySubdivisionFromWhereProcured,
    ///HI - HIBC (Health Care Industry Bar Code) Supplier Labeling Standard Primary Data Message
    CodeHI,
    ///HJ - Province or State of Product Processing Code
    ProvinceOrStateOfProductProcessingCode,
    ///HK - Province or State of Product Packaged
    ProvinceOrStateOfProductPackaged,
    ///HN - Heat Number
    HeatNumber,
    ///HP - Health Insurance Prospective Payment System (HIPPS) Rate Code
    CodeHP,
    ///HQ - Alcoholic Beverage Subregion
    AlcoholicBeverageSubregion,
    ///IA - Information Media Type
    InformationMediaType,
    ///IB - International Standard Book Number (ISBN)
    CodeIB,
    ///IC - Interior Color Number
    InteriorColorNumber,
    ///ID - International Classification of Diseases, 9th Revision, Clinical Modification (ICD-9-CM) - Procedure
    CodeID,
    ///IE - Insurer's Fund Code
    InsurersFundCode,
    ///IF - Investment Fund Type
    InvestmentFundType,
    ///IG - Ignition Key Number
    IgnitionKeyNumber,
    ///IH - International Classification of Diseases, 11th Revision,Clinical Modification (ICD-11-CM)
    CodeIH,
    ///II - Commodity Item Identification
    CommodityItemIdentification,
    ///IJ - International Classification of Diseases, 11th Revision, Procedure Coding System (ICD-11-PCS)
    CodeIJ,
    ///IM - Imprint (Trademark Code of Subsidiary)
    CodeIM,
    ///IN - Buyer's Item Number
    BuyersItemNumber,
    ///IP - International Classification of Diseases, 10th Revision, Procedure Coding System (ICD-10-PCS)
    CodeIP,
    ///IQ - IRS Qualification Code
    IrsQualificationCode,
    ///IR - Ingredient
    Ingredient,
    ///IS - International Standard Serial Number (ISSN)
    CodeIS,
    ///IT - Buyer's Style Number
    BuyersStyleNumber,
    ///IU - Department of Defense Issuing Agency Code
    DepartmentOfDefenseIssuingAgencyCode,
    ///IW - Interchangeability Code
    InterchangeabilityCode,
    ///IZ - Buyer's Size Code
    BuyersSizeCode,
    ///JA - Anniversary
    Anniversary,
    ///JB - Commission Identifier
    Commission,
    ///JC - Commission Year
    CommissionYear,
    ///JD - Contribution Year
    ContributionYear,
    ///JN - Job Number
    JobNumber,
    ///JP - Package Type Code
    PackageTypeCode,
    ///JS - Job Sequence Number
    JobSequenceNumber,
    ///KA - Engineering Data List
    EngineeringDataList,
    ///KB - Data Category Code
    DataCategoryCode,
    ///KD - Replacement National Stock Number
    ReplacementNationalStockNumber,
    ///KE - Military Standard
    MilitaryStandard,
    ///KF - Item Type Number
    ItemTypeNumber,
    ///KG - Time Compliant Technical Order
    TimeCompliantTechnicalOrder,
    ///KI - Cognizance Symbol
    CognizanceSymbol,
    ///KJ - Material Control Code
    MaterialControlCode,
    ///KK - Special Material Identification Code
    SpecialMaterialIdentificationCode,
    ///KL - Item Management Code
    ItemManagementCode,
    ///KM - Shelf-Life Code
    ShelfLifeCode,
    ///KN - Shelf-Life Action Code
    ShelfLifeActionCode,
    ///KP - Kanban Plan Number
    KanbanPlanNumber,
    ///L1 - Program Level
    ProgramLevel,
    ///L2 - Topic Level
    TopicLevel,
    ///L3 - Subtopic Level
    SubtopicLevel,
    ///L4 - Life/Annuity Service Features
    LifeAnnuityServiceFeatures,
    ///L5 - Line of Authority
    LineOfAuthority,
    ///L6 - Lube, Synthetic
    CodeL6,
    ///LA - Labor Group
    LaborGroup,
    ///LB - Logical Observation Identifier Names and Codes (LOINC) Codes
    CodeLB,
    ///LC - Laboratory Test Condition Code
    LaboratoryTestConditionCode,
    ///LD - SNOMED, Systematized Nomenclature of Medicine
    CodeLD,
    ///LG - Lift Gate Serial Number
    LiftGateSerialNumber,
    ///LM - Lottery Game Number
    LotteryGameNumber,
    ///LN - Lottery Pack/Book Number
    LotteryPackBookNumber,
    ///LP - Life/Annuity Product Code
    LifeAnnuityProductCode,
    ///LR - Lease Number
    LeaseNumber,
    ///LS - Load Sequence
    LoadSequence,
    ///LT - Lot Number
    LotNumber,
    ///LU - Lot Pricing Unit Number
    LotPricingUnitNumber,
    ///MA - Machine Number
    MachineNumber,
    ///MB - Measurement Type Code
    MeasurementTypeCode,
    ///MC - Mortgage Credit Data Order Type
    MortgageCreditDataOrderType,
    ///MD - Method of Delivery Code
    MethodOfDeliveryCode,
    ///ME - Market Program Code
    MarketProgramCode,
    ///MF - Manufacturer
    Manufacturer,
    ///MG - Manufacturer's Part Number
    ManufacturersPartNumber,
    ///MH - Medication Code
    MedicationCode,
    ///MI - Mortgage Insurance Product Code or Number
    MortgageInsuranceProductCodeOrNumber,
    ///MJ - Manual Transmission Serial Number
    ManualTransmissionSerialNumber,
    ///MK - Front Axle Non Driven Serial Number
    FrontAxleNonDrivenSerialNumber,
    ///MM - Motor Equipment Manufacturing Association (MEMA) Product Type Code
    CodeMM,
    ///MN - Model Number
    ModelNumber,
    ///MO - Movement Type Code
    MovementTypeCode,
    ///MP - Mortgage Product Code
    MortgageProductCode,
    ///MQ - Mortgage Underwriting Type
    MortgageUnderwritingType,
    ///MR - Maintenance Index Page Reference Number
    MaintenanceIndexPageReferenceNumber,
    ///MS - Military Specification (MILSPEC) Number
    CodeMS,
    ///MT - Major Product/Material/Machine Type
    MajorProductMaterialMachineType,
    ///MU - Authorized Parts List Number
    AuthorizedPartsListNumber,
    ///MV - Equipment Location
    EquipmentLocation,
    ///MW - Equipment Hierarchical Sequence Identifier
    EquipmentHierarchicalSequence,
    ///MX - Repair Induction Identifier
    RepairInduction,
    ///N1 - National Drug Code in 4-4-2 Format
    NationalDrugCodeIn442Format,
    ///N2 - National Drug Code in 5-3-2 Format
    NationalDrugCodeIn532Format,
    ///N3 - National Drug Code in 5-4-1 Format
    NationalDrugCodeIn541Format,
    ///N4 - National Drug Code in 5-4-2 Format
    NationalDrugCodeIn542Format,
    ///N5 - National Health Related Item Code in 5-5 Format
    NationalHealthRelatedItemCodeIn55Format,
    ///N6 - National Health Related Item Code in 4-6 Format
    NationalHealthRelatedItemCodeIn46Format,
    ///ND - National Drug Code (NDC)
    CodeND,
    ///NE - Yarn Count - English
    YarnCountEnglish,
    ///NG - National Glass Association (NAGS) Number
    CodeNG,
    ///NH - National Health Related Item Code
    NationalHealthRelatedItemCode,
    ///NM - Yarn Count - Metric
    YarnCountMetric,
    ///NN - National Item Identification Number
    NationalItemIdentificationNumber,
    ///NP - Natural Health Product Number
    NaturalHealthProductNumber,
    ///NR - Non-resaleable item (excluding deposit) number
    CodeNR,
    ///NU - National Uniform Billing Committee (NUBC) UB92 Codes
    CodeNU,
    ///NW - New Replacement Part or Assembly Defective
    NewReplacementPartOrAssemblyDefective,
    ///NZ - Combined NCCMA/Bank Service Code
    CombinedNccmaBankServiceCode,
    ///O9 - Old Vendor's (Seller's) Item Number
    CodeO9,
    ///OE - Original Equipment Number
    OriginalEquipmentNumber,
    ///OF - Old Common Language Equipment Identifier (CLEI) Code
    CodeOF,
    ///OG - Old Microcode
    OldMicrocode,
    ///OH - Opposite-Hand Part Number
    OppositeHandPartNumber,
    ///OI - Optical Industry Product Code
    OpticalIndustryProductCode,
    ///OL - Optical Cable Code
    OpticalCableCode,
    ///OM - Original Part Number
    OriginalPartNumber,
    ///ON - Customer Order Number
    CustomerOrderNumber,
    ///OO - Outside Production Operation Sheet Number
    OutsideProductionOperationSheetNumber,
    ///OP - Obsolete Part Number
    ObsoletePartNumber,
    ///OR - Offer Number
    OfferNumber,
    ///OT - Internal Number
    InternalNumber,
    ///OU - Original Unit of Issue
    OriginalUnitOfIssue,
    ///P1 - Petroleum Accountants Society of Canada Operating and Maintenance Code - Goods and Services Tax Not Applicable
    PetroleumAccountantsSocietyOfCanadaOperatingAndMaintenanceCodeGoodsAndServicesTaxNotApplicable,
    ///P2 - Petroleum Accountants Society of Canada Capital Expenditure Code - Goods and Services Tax Not Applicable
    PetroleumAccountantsSocietyOfCanadaCapitalExpenditureCodeGoodsAndServicesTaxNotApplicable,
    ///P3 - Petroleum Accountants Society of Canada Tubular Code
    PetroleumAccountantsSocietyOfCanadaTubularCode,
    ///P4 - Petroleum Accountants Society of Canada Non-Tubular Code
    PetroleumAccountantsSocietyOfCanadaNonTubularCode,
    ///P5 - Material Discharge Number
    MaterialDischargeNumber,
    ///P6 - Pump, Fire
    CodeP6,
    ///P7 - Previous Carrier
    PreviousCarrier,
    ///P8 - Retail Price Look Up Number (PLU)
    CodeP8,
    ///P9 - Ply
    Ply,
    ///PA - Pattern Number
    PatternNumber,
    ///PB - Petroleum Accountants Society of Canada Operating and Maintenance Code - Goods and Services Tax Forwarded
    PetroleumAccountantsSocietyOfCanadaOperatingAndMaintenanceCodeGoodsAndServicesTaxForwarded,
    ///PC - Prime Contractor Part Number
    PrimeContractorPartNumber,
    ///PD - Part Number Description
    PartNumberDescription,
    ///PE - Pieces in Roll
    PiecesInRoll,
    ///PF - Petroleum Accountants Society of Canada Capital Expenditure Code - Goods and Services Tax Forwarded
    PetroleumAccountantsSocietyOfCanadaCapitalExpenditureCodeGoodsAndServicesTaxForwarded,
    ///PG - Packaging Specification Number
    PackagingSpecificationNumber,
    ///PH - Property and Casualty Service Code
    PropertyAndCasualtyServiceCode,
    ///PI - Purchaser's Item Code
    PurchasersItemCode,
    ///PJ - Product Date Code (A code indicating the period during which a product was manufactured.)
    CodePJ,
    ///PK - Packaging Drawing
    PackagingDrawing,
    ///PL - Purchaser's Order Line Number
    PurchasersOrderLineNumber,
    ///PM - Number of Positions on Machine
    NumberOfPositionsOnMachine,
    ///PN - Company Part Number
    CompanyPartNumber,
    ///PO - Purchase Order Number
    PurchaseOrderNumber,
    ///PP - Air Transportation Association Proprietary Rights Code
    AirTransportationAssociationProprietaryRightsCode,
    ///PQ - Product ID Attribute Code
    ProductIdAttributeCode,
    ///PR - Process Number
    ProcessNumber,
    ///PS - Position
    Position,
    ///PT - Print or Drawing
    PrintOrDrawing,
    ///PU - Part Reference Number
    PartReferenceNumber,
    ///PV - Advertising Package Identification Code
    AdvertisingPackageIdentificationCode,
    ///PW - Part Drawing
    PartDrawing,
    ///PX - Secondary Ply
    SecondaryPly,
    ///PY - Operator Assigned Property Identification
    OperatorAssignedPropertyIdentification,
    ///PZ - Product Change Notice Number
    ProductChangeNoticeNumber,
    ///R1 - Replacement Subassembly Model Number
    ReplacementSubassemblyModelNumber,
    ///R2 - Replacement Subassembly Serial Number
    ReplacementSubassemblySerialNumber,
    ///R3 - Rear Axle, Middle
    CodeR3,
    ///R4 - Rear Axle, Pusher
    CodeR4,
    ///R5 - Rear Axle, Tag
    CodeR5,
    ///R6 - Rear Axle, Extended Tag
    CodeR6,
    ///R9 - Replacement Vendor's (Seller's) Item Number
    CodeR9,
    ///RA - Return Code
    ReturnCode,
    ///RB - National Uniform Billing Committee (NUBC) UB82 Codes
    CodeRB,
    ///RC - Returnable Container Number
    ReturnableContainerNumber,
    ///RD - Reel Number
    ReelNumber,
    ///RE - Reefer Serial Number
    ReeferSerialNumber,
    ///RF - Repair From Product Code
    RepairFromProductCode,
    ///RG - Reference Oil
    ReferenceOil,
    ///RH - Radiator Serial Number
    RadiatorSerialNumber,
    ///RI - Rear Axle Front Rear Driven Serial Number
    RearAxleFrontRearDrivenSerialNumber,
    ///RJ - Rear Axle Rear Driven Serial Number
    RearAxleRearDrivenSerialNumber,
    ///RK - Rack Number
    RackNumber,
    ///RL - Rate Detail Card
    RateDetailCard,
    ///RM - Related Model Number
    RelatedModelNumber,
    ///RN - Release Number
    ReleaseNumber,
    ///RO - Roll Number
    RollNumber,
    ///RP - Replaced Part Number
    ReplacedPartNumber,
    ///RQ - Automobile Rental Charge Item Code
    AutomobileRentalChargeItemCode,
    ///RR - Replacement Product Number
    ReplacementProductNumber,
    ///RS - Set Number
    SetNumber,
    ///RT - Reel Type
    ReelType,
    ///RU - Run Number
    RunNumber,
    ///RV - Repair Tag Number
    RepairTagNumber,
    ///RW - Relative Value Units
    RelativeValueUnits,
    ///RX - Provisioning Reference Number
    ProvisioningReferenceNumber,
    ///RY - Record Keeping or Model Year
    RecordKeepingOrModelYear,
    ///RZ - Related Model Type
    RelatedModelType,
    ///S1 - Shipper's Item Number
    ShippersItemNumber,
    ///S2 - Second Prior Identifier
    SecondPrior,
    ///S3 - Phase
    Phase,
    ///S4 - Laboratory Sample Identification
    LaboratorySampleIdentification,
    ///S5 - State Sample Identification
    StateSampleIdentification,
    ///S6 - Previous Sample Identification
    PreviousSampleIdentification,
    ///S7 - Source of Deposit Code
    SourceOfDepositCode,
    ///S8 - Source of Lead Code
    SourceOfLeadCode,
    ///SA - Schematic Diagram Reference Number
    SchematicDiagramReferenceNumber,
    ///SB - Submission Number
    SubmissionNumber,
    ///SC - Seller's Date Code
    SellersDateCode,
    ///SD - Supplier Company Registry Number
    SupplierCompanyRegistryNumber,
    ///SE - Section Print Number
    SectionPrintNumber,
    ///SF - Surface Finish
    SurfaceFinish,
    ///SG - Seat Serial Number
    SeatSerialNumber,
    ///SH - Service Requested
    ServiceRequested,
    ///SI - Standard Industrial Classification Code
    StandardIndustrialClassificationCode,
    ///SJ - Religious Retail Non-book Item
    ReligiousRetailNonBookItem,
    ///SK - Stock Keeping Unit (SKU)
    CodeSK,
    ///SL - Seller's Lot Number
    SellersLotNumber,
    ///SM - National Retail Federation Size Code
    NationalRetailFederationSizeCode,
    ///SN - Serial Number
    SerialNumber,
    ///SO - System Identifier
    System,
    ///SP - Superseded Purchase Order Number
    SupersededPurchaseOrderNumber,
    ///SQ - Roll Sequence Number
    RollSequenceNumber,
    ///SR - Substitute Product Number
    SubstituteProductNumber,
    ///SS - Superseded Part Number
    SupersededPartNumber,
    ///ST - Style Number
    StyleNumber,
    ///SU - Side Up/Side Down
    SideUpSideDown,
    ///SV - Service Rendered
    ServiceRendered,
    ///SW - Stock Number
    StockNumber,
    ///SX - Sleeper Box Key Number
    SleeperBoxKeyNumber,
    ///SY - Sleeper Box Serial Number
    SleeperBoxSerialNumber,
    ///SZ - Vendor Alphanumeric Size Code
    VendorAlphanumericSizeCode,
    ///T2 - Tex
    Tex,
    ///T3 - Third Prior Identifier
    ThirdPrior,
    ///T5 - Substitute GTIN-8
    SubstituteGtin8,
    ///T6 - Substitute GTIN-12
    SubstituteGtin12,
    ///T7 - Substitute GTIN-13
    SubstituteGtin13,
    ///T8 - Substitute GTIN-14
    SubstituteGtin14,
    ///TA - Pipeline Transaction Code
    PipelineTransactionCode,
    ///TB - Association for Financial Professionals Service Code and Bank Service Code
    AssociationForFinancialProfessionalsServiceCodeAndBankServiceCode,
    ///TC - Telecommunications Circuit ID
    TelecommunicationsCircuitId,
    ///TD - Treatment Codes
    TreatmentCodes,
    ///TE - Association for Financial Professionals Service Code
    AssociationForFinancialProfessionalsServiceCode,
    ///TF - The Air Cargo Tariff (TACT) Commodity Code
    CodeTF,
    ///TG - Automatic Transmission Serial Number
    AutomaticTransmissionSerialNumber,
    ///TH - Transfer Case Serial Number
    TransferCaseSerialNumber,
    ///TI - Trade In Identifier
    TradeIn,
    ///TJ - Auxiliary Transmission Serial Number
    AuxiliaryTransmissionSerialNumber,
    ///TK - Transmission, Auxillary
    CodeTK,
    ///TM - Telephone Industry Manufacturer Code
    TelephoneIndustryManufacturerCode,
    ///TN - Railroad-Owned Unit Train Number
    RailroadOwnedUnitTrainNumber,
    ///TP - Product Type Code
    ProductTypeCode,
    ///TR - Truck Body Serial Number
    TruckBodySerialNumber,
    ///TS - Transmission Serial Number
    TransmissionSerialNumber,
    ///TT - Type Selvage
    TypeSelvage,
    ///TU - Tested Material Identification Number
    TestedMaterialIdentificationNumber,
    ///TV - Line of Business
    LineOfBusiness,
    ///TW - Program Code
    ProgramCode,
    ///TX - Federal Aviation Administration (FAA) Service Bulletin Number
    CodeTX,
    ///TY - Telecommunications Industry Service Code
    TelecommunicationsIndustryServiceCode,
    ///TZ - Program Description Identifier
    ProgramDescription,
    ///U3 - United Nations Common Coding System (UNCCS)
    CodeU3,
    ///U5 - Broker Price Opinion Service
    BrokerPriceOpinionService,
    ///U6 - Real Estate Property Information Service
    RealEstatePropertyInformationService,
    ///U7 - Department of Defense Unique Item Identifier
    DepartmentOfDefenseUniqueItem,
    ///UC - Product Variant
    ProductVariant,
    ///UF - User-Defined Shipping Container Identifier
    UserDefinedShippingContainer,
    ///UJ - U.P.C./EAN Coupon Code (2-5-5)
    CodeUJ,
    ///UK - GTIN-14
    Gtin14,
    ///UL - U.P.C. Coupon Code (1-5-5-1)
    CodeUL,
    ///UM - Universal Vendor Marking, Short Code (UVM; U Line)
    CodeUM,
    ///UO - EAN.UCC Serial Shipping Container Code (SSCC)
    CodeUO,
    ///UP - GTIN-12
    Gtin12,
    ///UQ - United Nations (UN) Number (Dangerous Goods)
    CodeUQ,
    ///UR - UCC/EAN-128 Coupon Extended Code
    UccEan128CouponExtendedCode,
    ///US - Uniform Stock Symbol System Code Number
    UniformStockSymbolSystemCodeNumber,
    ///UT - Company-Owned Unit Train Number
    CompanyOwnedUnitTrainNumber,
    ///UU - Device Identifier
    Device,
    ///UV - Universal Vendor Marking, Long Code (UVM; R,P,M Lines)
    CodeUV,
    ///UX - Universal Product Number
    UniversalProductNumber,
    ///UY - Department of Defense Unique Item Identifier Type Reference Identifier
    DepartmentOfDefenseUniqueItemTypeReference,
    ///UZ - GS1 US Coupon Code with GS1-128 Coupon Extended Code
    Gs1UsCouponCodeWithGs1128CouponExtendedCode,
    ///V1 - Ingredient Country of Origin Code
    IngredientCountryOfOriginCode,
    ///V2 - Tariff Country of Origin Code
    TariffCountryOfOriginCode,
    ///V3 - Country of Last Processing Code
    CountryOfLastProcessingCode,
    ///V4 - Country of Assembly Code
    CountryOfAssemblyCode,
    ///V5 - Logistics Country of Origin Code
    LogisticsCountryOfOriginCode,
    ///V6 - United States Department of Agriculture (USDA) Country of Origin Code
    CodeV6,
    ///V7 - Consumer Product Variant Identifier
    ConsumerProductVariant,
    ///VA - Vendor's Style Number
    VendorsStyleNumber,
    ///VB - Vendor's Engineering Change Level Number
    VendorsEngineeringChangeLevelNumber,
    ///VC - Vendor's (Seller's) Catalog Number
    CodeVC,
    ///VE - Vendor Color
    VendorColor,
    ///VI - Vary Item Product Number
    VaryItemProductNumber,
    ///VM - Vehicle Maintenance Reporting Standards
    VehicleMaintenanceReportingStandards,
    ///VN - Vendor's (Seller's) Item Number
    CodeVN,
    ///VO - Vendor's Order Number
    VendorsOrderNumber,
    ///VP - Vendor's (Seller's) Part Number
    CodeVP,
    ///VS - Vendor's Supplemental Item Number
    VendorsSupplementalItemNumber,
    ///VT - Vintage
    Vintage,
    ///VU - Vendor's Basic Unit Number
    VendorsBasicUnitNumber,
    ///VV - Motor Vehicle ID Number
    MotorVehicleIdNumber,
    ///VX - Vendor's Specification Number
    VendorsSpecificationNumber,
    ///W1 - End Item Serial Number
    EndItemSerialNumber,
    ///W2 - Work Unit Number
    WorkUnitNumber,
    ///W5 - Reclamation Process
    ReclamationProcess,
    ///W6 - Woolen Run
    WoolenRun,
    ///W7 - Woolen Cut
    WoolenCut,
    ///WA - Random Weight Aggregation Code
    RandomWeightAggregationCode,
    ///WB - Car Class Code
    CarClassCode,
    ///WC - World Code
    WorldCode,
    ///WD - Airline Flight Code
    AirlineFlightCode,
    ///WE - Fare Basis Code
    FareBasisCode,
    ///WF - Service Class Code
    ServiceClassCode,
    ///WG - Stop Over Code
    StopOverCode,
    ///WJ - Telecom Service Type
    TelecomServiceType,
    ///WK - Advanced Billing Concepts (ABC) Codes
    CodeWK,
    ///WL - Wafer Lot Identifier
    WaferLot,
    ///WR - Yarn Count Worsted
    YarnCountWorsted,
    ///WS - Wheel Chair Lift Serial Number
    WheelChairLiftSerialNumber,
    ///XA - Preferred Part Number
    PreferredPartNumber,
    ///XC - Expendable Container Identification
    ExpendableContainerIdentification,
    ///XN - Export Control Classification Number (ECCN)
    CodeXN,
    ///XP - Preferred National Stock Number
    PreferredNationalStockNumber,
    ///XQ - Preferred Manufacturer
    PreferredManufacturer,
    ///XZ - Contractor Establishment Code
    ContractorEstablishmentCode,
    ///Y1 - Child GTIN-13
    ChildGtin13,
    ///Y3 - Child GTIN-14
    ChildGtin14,
    ///Y4 - Child GTIN-12
    ChildGtin12,
    ///YM - Map Edition Number
    MapEditionNumber,
    ///YP - Publication Number
    PublicationNumber,
    ///ZB - Commercial and Government Entity (CAGE) Code
    CodeZB,
    ///ZR - Service Control Identification
    ServiceControlIdentification,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl ProductServiceIdQualifier {
    pub fn code(&self) -> &str {
        {
            use ProductServiceIdQualifier::*;
            match self {
                CreditCard => "00",
                PlantEquipmentNumber => "A1",
                CodeA2 => "A2",
                LocallyAssignedControlNumber => "A3",
                SubsistenceIdentificationNumber => "A4",
                ApplicationStateOrProvince => "A5",
                DocumentIdentificationCode => "A6",
                SublineItemNumber => "A7",
                ExhibitLineItemNumber => "A8",
                Activity => "A9",
                CodeAA => "AA",
                Assembly => "AB",
                CodeAC => "AC",
                AmericanDentalAssociationCodes => "AD",
                CodeAE => "AE",
                FrontAxleSerialNumber => "AF",
                Age => "AG",
                BillboardsCommercial => "AH",
                AlternateIsbn => "AI",
                PiggybackCommercial => "AJ",
                RefinedProductCode => "AK",
                AxleSerialNumber => "AL",
                AllocationMethodologyIdentificationCode => "AM",
                AssetNumber => "AN",
                AssemblageIdentificationNumber => "AO",
                CodeAP => "AP",
                AppraisalProductType => "AQ",
                ArincPartNumber => "AR",
                RearAxleSerialNumber => "AS",
                AssortabilityCode => "AT",
                AppraisalService => "AU",
                AdditionalProductIdentificationAssignedByTheManufacturer => "AV",
                WellNumber => "AW",
                AssemblyLotNumber => "AX",
                CodeAY => "AY",
                AxleRatio => "AZ",
                VolumeSet => "B1",
                LibraryOfCongressCode => "B2",
                EditionCode => "B3",
                BindingCode => "B4",
                TitleCode => "B5",
                AuthorCode => "B6",
                PublisherCode => "B7",
                BatchNumber => "B8",
                FrontAxleDrivenSerialNumber => "B9",
                BaleNumber => "BA",
                PrintColorStandardReference => "BB",
                SupplierBrandCode => "BC",
                BuyerGroundShade => "BD",
                BuyersEngineeringChangeLevelNumber => "BE",
                VendorGroundShade => "BF",
                GroundShadeStandardReference => "BG",
                FinishHandStandardReference => "BH",
                RearAxleRearNonDrivenSerialNumber => "BI",
                ApplicationCompletionMethod => "BJ",
                Backing => "BK",
                BrandLabel => "BL",
                BeamNumber => "BM",
                BarCodedSerialNumber => "BN",
                BuyersColor => "BO",
                BuyersPartNumber => "BP",
                BenefitId => "BQ",
                CompensationTypeCode => "BR",
                BillSubgroupCode => "BS",
                VolumeTypeIdentificationCode => "BT",
                BusBodySerialNumber => "BU",
                BeverageCommonCode => "BV",
                CompensationAllocationCode => "BW",
                BillCreditCode => "BX",
                ConceptCode => "BY",
                CoverageType => "BZ",
                Channel => "C1",
                ConnectorType => "C2",
                Classification => "C3",
                ConfigurationItem => "C4",
                InsurancePlanDescriptionCharacteristics => "C5",
                AssetType => "C6",
                AssetCategory => "C7",
                FundSubAdvisor => "C8",
                CodeC9 => "C9",
                Case => "CA",
                BuyersCatalogNumber => "CB",
                CompatibleCutNumber => "CC",
                MotorVehicleLineDesignator => "CD",
                ClassOfContractCode => "CE",
                ChassisSerialNumber => "CF",
                CommodityGrouping => "CG",
                CountryOfOriginCode => "CH",
                CodeCI => "CI",
                CodeCJ => "CJ",
                CandidateOil => "CK",
                Color => "CL",
                NationalRetailFederationColorCode => "CM",
                CommodityName => "CN",
                CodeCO => "CO",
                CarryOverPartNumber => "CP",
                EquipmentCodePerCopasStandard => "CQ",
                ContractNumber => "CR",
                ServiceCodePerCopasStandard => "CS",
                CodeCT => "CT",
                CutNumber => "CU",
                CustomerProvidedEquipment => "CV",
                ContractActivityCode => "CW",
                CompletionNumber => "CX",
                CustomerCompanyRegistryNumber => "CY",
                CountryFromWhichProcured => "CZ",
                UnderwritingMethodOfDirectWriter => "D1",
                CodeD2 => "D2",
                PolicyForm => "D3",
                PlanCode => "D4",
                CoverageRiskType => "D5",
                MedicalStopLossLevelCodes => "D6",
                MedicalStopLossCoverageCodes => "D7",
                MedicalStopLossProductLineCodes => "D8",
                RxNorm => "D9",
                AutomobileRepairProductCode => "DA",
                Gs1DataBar => "DB",
                CodeDC => "DC",
                Distributor => "DD",
                DesignNumber => "DE",
                DeviceFamily => "DF",
                DiscountGrouping => "DG",
                Transport4CommodityCode => "DH",
                DepositItemNumber => "DI",
                DependentProprietaryProduct => "DJ",
                PrimaryAlternateProduct => "DK",
                DyeLotNumber => "DL",
                CodeDM => "DM",
                DieNumber => "DN",
                DividendUse => "DO",
                DiscontinuedPartNumber => "DP",
                EventType => "DQ",
                DrawingRevisionNumber => "DR",
                GroupId => "DS",
                DeviceType => "DT",
                JointLifeType => "DU",
                LocationCode => "DV",
                NonforfeitureOption => "DW",
                CodeDX => "DX",
                PremiumRateType => "DY",
                DiagnosisCodePointer => "DZ",
                ContractChangeAuthorizationType => "E1",
                FundAbbreviation => "E2",
                FundType => "E3",
                RelatedPolicyIdentification => "E4",
                TaxCode => "E5",
                ContractTransferReasonCode => "E6",
                BerensonEggersTypeOfService => "E7",
                InternationalSocietyOfBloodTransfusionDeviceIdentifier => "E8",
                CodeE9 => "E9",
                Ean99InStoreCouponCode => "EA",
                FuelTankSerialNumber => "EB",
                EngineeringChangeLevel => "EC",
                EngineDisplacementIdentification => "ED",
                PremiumUse => "EE",
                Exhibit => "EF",
                PurposeOfInsurance => "EG",
                SalesPresentationId => "EH",
                Expense => "EI",
                ServiceFeatureId => "EJ",
                SettlementPayoutOption => "EK",
                BuyersSublineItemNumber => "EL",
                EquipmentIdentificationNumber => "EM",
                Gtin13 => "EN",
                Gtin8 => "EO",
                BuyersEndProductNumber => "EP",
                EquipmentType => "EQ",
                JurisdictionSpecificProcedureAndSupplyCodes => "ER",
                EngineSerialNumber => "ES",
                DepartmentOfDefenseEnterprise => "ET",
                SkillCode => "EU",
                EquivalentProduct => "EV",
                CodeEX => "EX",
                ShiftWorked => "EZ",
                CatalogNumber => "F1",
                TechnicalOrderNumber => "F2",
                TechnicalManualNumber => "F3",
                Series => "F4",
                ObligationAuthorityNumber => "F5",
                FirstPrior => "F6",
                EndItemDescription => "F7",
                NextHigherUsedAssembly => "F8",
                FormerPublisher => "F9",
                FailedSubassemblySerialNumber => "FA",
                FormNumber => "FB",
                CouponFamilyCode => "FC",
                Fund => "FD",
                Feature => "FE",
                FifthWheelSerialNumber => "FF",
                FundManager => "FG",
                FreddieMacAffordableLendingProductCode => "FH",
                FinishNumber => "FI",
                FannieMaeAffordableLendingProductCode => "FJ",
                FreddieMacProjectCondominiumClassificationCode => "FK",
                FinishLotNumber => "FL",
                FailedSubassemblyModelNumber => "FM",
                FinalTestLotNumber => "FN",
                FabricPiecesPerRoll => "FP",
                FannieMaeProjectCondominiumClassificationCode => "FQ",
                CodeFR => "FR",
                NationalStockNumber => "FS",
                FederalSupplyClassification => "FT",
                CodeFU => "FU",
                CodeFV => "FV",
                NewMicrocode => "FW",
                FederalSupplyGroup => "FX",
                Gathering => "GA",
                GradeCode => "GC",
                GrainDirection => "GD",
                GenericNameDescription => "GE",
                CodeGI => "GI",
                GliderKit => "GK",
                CodeGM => "GM",
                GradeName => "GN",
                GroupQualifierCode => "GQ",
                GearRatio => "GR",
                GeneralSpecificationNumber => "GS",
                VolumeUsageIdentificationCode => "GU",
                CodeGV => "GV",
                CodeGW => "GW",
                CodeGX => "GX",
                CountrySubdivisionCode => "HA",
                CountryOfOriginWithCountrySubdivisionCode => "HB",
                CodeHC => "HC",
                InternationalHarmonizedCommodityCode => "HD",
                CountrySubdivisionFromWhereProcured => "HF",
                CodeHI => "HI",
                ProvinceOrStateOfProductProcessingCode => "HJ",
                ProvinceOrStateOfProductPackaged => "HK",
                HeatNumber => "HN",
                CodeHP => "HP",
                AlcoholicBeverageSubregion => "HQ",
                InformationMediaType => "IA",
                CodeIB => "IB",
                InteriorColorNumber => "IC",
                CodeID => "ID",
                InsurersFundCode => "IE",
                InvestmentFundType => "IF",
                IgnitionKeyNumber => "IG",
                CodeIH => "IH",
                CommodityItemIdentification => "II",
                CodeIJ => "IJ",
                CodeIM => "IM",
                BuyersItemNumber => "IN",
                CodeIP => "IP",
                IrsQualificationCode => "IQ",
                Ingredient => "IR",
                CodeIS => "IS",
                BuyersStyleNumber => "IT",
                DepartmentOfDefenseIssuingAgencyCode => "IU",
                InterchangeabilityCode => "IW",
                BuyersSizeCode => "IZ",
                Anniversary => "JA",
                Commission => "JB",
                CommissionYear => "JC",
                ContributionYear => "JD",
                JobNumber => "JN",
                PackageTypeCode => "JP",
                JobSequenceNumber => "JS",
                EngineeringDataList => "KA",
                DataCategoryCode => "KB",
                ReplacementNationalStockNumber => "KD",
                MilitaryStandard => "KE",
                ItemTypeNumber => "KF",
                TimeCompliantTechnicalOrder => "KG",
                CognizanceSymbol => "KI",
                MaterialControlCode => "KJ",
                SpecialMaterialIdentificationCode => "KK",
                ItemManagementCode => "KL",
                ShelfLifeCode => "KM",
                ShelfLifeActionCode => "KN",
                KanbanPlanNumber => "KP",
                ProgramLevel => "L1",
                TopicLevel => "L2",
                SubtopicLevel => "L3",
                LifeAnnuityServiceFeatures => "L4",
                LineOfAuthority => "L5",
                CodeL6 => "L6",
                LaborGroup => "LA",
                CodeLB => "LB",
                LaboratoryTestConditionCode => "LC",
                CodeLD => "LD",
                LiftGateSerialNumber => "LG",
                LotteryGameNumber => "LM",
                LotteryPackBookNumber => "LN",
                LifeAnnuityProductCode => "LP",
                LeaseNumber => "LR",
                LoadSequence => "LS",
                LotNumber => "LT",
                LotPricingUnitNumber => "LU",
                MachineNumber => "MA",
                MeasurementTypeCode => "MB",
                MortgageCreditDataOrderType => "MC",
                MethodOfDeliveryCode => "MD",
                MarketProgramCode => "ME",
                Manufacturer => "MF",
                ManufacturersPartNumber => "MG",
                MedicationCode => "MH",
                MortgageInsuranceProductCodeOrNumber => "MI",
                ManualTransmissionSerialNumber => "MJ",
                FrontAxleNonDrivenSerialNumber => "MK",
                CodeMM => "MM",
                ModelNumber => "MN",
                MovementTypeCode => "MO",
                MortgageProductCode => "MP",
                MortgageUnderwritingType => "MQ",
                MaintenanceIndexPageReferenceNumber => "MR",
                CodeMS => "MS",
                MajorProductMaterialMachineType => "MT",
                AuthorizedPartsListNumber => "MU",
                EquipmentLocation => "MV",
                EquipmentHierarchicalSequence => "MW",
                RepairInduction => "MX",
                NationalDrugCodeIn442Format => "N1",
                NationalDrugCodeIn532Format => "N2",
                NationalDrugCodeIn541Format => "N3",
                NationalDrugCodeIn542Format => "N4",
                NationalHealthRelatedItemCodeIn55Format => "N5",
                NationalHealthRelatedItemCodeIn46Format => "N6",
                CodeND => "ND",
                YarnCountEnglish => "NE",
                CodeNG => "NG",
                NationalHealthRelatedItemCode => "NH",
                YarnCountMetric => "NM",
                NationalItemIdentificationNumber => "NN",
                NaturalHealthProductNumber => "NP",
                CodeNR => "NR",
                CodeNU => "NU",
                NewReplacementPartOrAssemblyDefective => "NW",
                CombinedNccmaBankServiceCode => "NZ",
                CodeO9 => "O9",
                OriginalEquipmentNumber => "OE",
                CodeOF => "OF",
                OldMicrocode => "OG",
                OppositeHandPartNumber => "OH",
                OpticalIndustryProductCode => "OI",
                OpticalCableCode => "OL",
                OriginalPartNumber => "OM",
                CustomerOrderNumber => "ON",
                OutsideProductionOperationSheetNumber => "OO",
                ObsoletePartNumber => "OP",
                OfferNumber => "OR",
                InternalNumber => "OT",
                OriginalUnitOfIssue => "OU",
                PetroleumAccountantsSocietyOfCanadaOperatingAndMaintenanceCodeGoodsAndServicesTaxNotApplicable => {
                    "P1"
                }
                PetroleumAccountantsSocietyOfCanadaCapitalExpenditureCodeGoodsAndServicesTaxNotApplicable => {
                    "P2"
                }
                PetroleumAccountantsSocietyOfCanadaTubularCode => "P3",
                PetroleumAccountantsSocietyOfCanadaNonTubularCode => "P4",
                MaterialDischargeNumber => "P5",
                CodeP6 => "P6",
                PreviousCarrier => "P7",
                CodeP8 => "P8",
                Ply => "P9",
                PatternNumber => "PA",
                PetroleumAccountantsSocietyOfCanadaOperatingAndMaintenanceCodeGoodsAndServicesTaxForwarded => {
                    "PB"
                }
                PrimeContractorPartNumber => "PC",
                PartNumberDescription => "PD",
                PiecesInRoll => "PE",
                PetroleumAccountantsSocietyOfCanadaCapitalExpenditureCodeGoodsAndServicesTaxForwarded => {
                    "PF"
                }
                PackagingSpecificationNumber => "PG",
                PropertyAndCasualtyServiceCode => "PH",
                PurchasersItemCode => "PI",
                CodePJ => "PJ",
                PackagingDrawing => "PK",
                PurchasersOrderLineNumber => "PL",
                NumberOfPositionsOnMachine => "PM",
                CompanyPartNumber => "PN",
                PurchaseOrderNumber => "PO",
                AirTransportationAssociationProprietaryRightsCode => "PP",
                ProductIdAttributeCode => "PQ",
                ProcessNumber => "PR",
                Position => "PS",
                PrintOrDrawing => "PT",
                PartReferenceNumber => "PU",
                AdvertisingPackageIdentificationCode => "PV",
                PartDrawing => "PW",
                SecondaryPly => "PX",
                OperatorAssignedPropertyIdentification => "PY",
                ProductChangeNoticeNumber => "PZ",
                ReplacementSubassemblyModelNumber => "R1",
                ReplacementSubassemblySerialNumber => "R2",
                CodeR3 => "R3",
                CodeR4 => "R4",
                CodeR5 => "R5",
                CodeR6 => "R6",
                CodeR9 => "R9",
                ReturnCode => "RA",
                CodeRB => "RB",
                ReturnableContainerNumber => "RC",
                ReelNumber => "RD",
                ReeferSerialNumber => "RE",
                RepairFromProductCode => "RF",
                ReferenceOil => "RG",
                RadiatorSerialNumber => "RH",
                RearAxleFrontRearDrivenSerialNumber => "RI",
                RearAxleRearDrivenSerialNumber => "RJ",
                RackNumber => "RK",
                RateDetailCard => "RL",
                RelatedModelNumber => "RM",
                ReleaseNumber => "RN",
                RollNumber => "RO",
                ReplacedPartNumber => "RP",
                AutomobileRentalChargeItemCode => "RQ",
                ReplacementProductNumber => "RR",
                SetNumber => "RS",
                ReelType => "RT",
                RunNumber => "RU",
                RepairTagNumber => "RV",
                RelativeValueUnits => "RW",
                ProvisioningReferenceNumber => "RX",
                RecordKeepingOrModelYear => "RY",
                RelatedModelType => "RZ",
                ShippersItemNumber => "S1",
                SecondPrior => "S2",
                Phase => "S3",
                LaboratorySampleIdentification => "S4",
                StateSampleIdentification => "S5",
                PreviousSampleIdentification => "S6",
                SourceOfDepositCode => "S7",
                SourceOfLeadCode => "S8",
                SchematicDiagramReferenceNumber => "SA",
                SubmissionNumber => "SB",
                SellersDateCode => "SC",
                SupplierCompanyRegistryNumber => "SD",
                SectionPrintNumber => "SE",
                SurfaceFinish => "SF",
                SeatSerialNumber => "SG",
                ServiceRequested => "SH",
                StandardIndustrialClassificationCode => "SI",
                ReligiousRetailNonBookItem => "SJ",
                CodeSK => "SK",
                SellersLotNumber => "SL",
                NationalRetailFederationSizeCode => "SM",
                SerialNumber => "SN",
                System => "SO",
                SupersededPurchaseOrderNumber => "SP",
                RollSequenceNumber => "SQ",
                SubstituteProductNumber => "SR",
                SupersededPartNumber => "SS",
                StyleNumber => "ST",
                SideUpSideDown => "SU",
                ServiceRendered => "SV",
                StockNumber => "SW",
                SleeperBoxKeyNumber => "SX",
                SleeperBoxSerialNumber => "SY",
                VendorAlphanumericSizeCode => "SZ",
                Tex => "T2",
                ThirdPrior => "T3",
                SubstituteGtin8 => "T5",
                SubstituteGtin12 => "T6",
                SubstituteGtin13 => "T7",
                SubstituteGtin14 => "T8",
                PipelineTransactionCode => "TA",
                AssociationForFinancialProfessionalsServiceCodeAndBankServiceCode => "TB",
                TelecommunicationsCircuitId => "TC",
                TreatmentCodes => "TD",
                AssociationForFinancialProfessionalsServiceCode => "TE",
                CodeTF => "TF",
                AutomaticTransmissionSerialNumber => "TG",
                TransferCaseSerialNumber => "TH",
                TradeIn => "TI",
                AuxiliaryTransmissionSerialNumber => "TJ",
                CodeTK => "TK",
                TelephoneIndustryManufacturerCode => "TM",
                RailroadOwnedUnitTrainNumber => "TN",
                ProductTypeCode => "TP",
                TruckBodySerialNumber => "TR",
                TransmissionSerialNumber => "TS",
                TypeSelvage => "TT",
                TestedMaterialIdentificationNumber => "TU",
                LineOfBusiness => "TV",
                ProgramCode => "TW",
                CodeTX => "TX",
                TelecommunicationsIndustryServiceCode => "TY",
                ProgramDescription => "TZ",
                CodeU3 => "U3",
                BrokerPriceOpinionService => "U5",
                RealEstatePropertyInformationService => "U6",
                DepartmentOfDefenseUniqueItem => "U7",
                ProductVariant => "UC",
                UserDefinedShippingContainer => "UF",
                CodeUJ => "UJ",
                Gtin14 => "UK",
                CodeUL => "UL",
                CodeUM => "UM",
                CodeUO => "UO",
                Gtin12 => "UP",
                CodeUQ => "UQ",
                UccEan128CouponExtendedCode => "UR",
                UniformStockSymbolSystemCodeNumber => "US",
                CompanyOwnedUnitTrainNumber => "UT",
                Device => "UU",
                CodeUV => "UV",
                UniversalProductNumber => "UX",
                DepartmentOfDefenseUniqueItemTypeReference => "UY",
                Gs1UsCouponCodeWithGs1128CouponExtendedCode => "UZ",
                IngredientCountryOfOriginCode => "V1",
                TariffCountryOfOriginCode => "V2",
                CountryOfLastProcessingCode => "V3",
                CountryOfAssemblyCode => "V4",
                LogisticsCountryOfOriginCode => "V5",
                CodeV6 => "V6",
                ConsumerProductVariant => "V7",
                VendorsStyleNumber => "VA",
                VendorsEngineeringChangeLevelNumber => "VB",
                CodeVC => "VC",
                VendorColor => "VE",
                VaryItemProductNumber => "VI",
                VehicleMaintenanceReportingStandards => "VM",
                CodeVN => "VN",
                VendorsOrderNumber => "VO",
                CodeVP => "VP",
                VendorsSupplementalItemNumber => "VS",
                Vintage => "VT",
                VendorsBasicUnitNumber => "VU",
                MotorVehicleIdNumber => "VV",
                VendorsSpecificationNumber => "VX",
                EndItemSerialNumber => "W1",
                WorkUnitNumber => "W2",
                ReclamationProcess => "W5",
                WoolenRun => "W6",
                WoolenCut => "W7",
                RandomWeightAggregationCode => "WA",
                CarClassCode => "WB",
                WorldCode => "WC",
                AirlineFlightCode => "WD",
                FareBasisCode => "WE",
                ServiceClassCode => "WF",
                StopOverCode => "WG",
                TelecomServiceType => "WJ",
                CodeWK => "WK",
                WaferLot => "WL",
                YarnCountWorsted => "WR",
                WheelChairLiftSerialNumber => "WS",
                PreferredPartNumber => "XA",
                ExpendableContainerIdentification => "XC",
                CodeXN => "XN",
                PreferredNationalStockNumber => "XP",
                PreferredManufacturer => "XQ",
                ContractorEstablishmentCode => "XZ",
                ChildGtin13 => "Y1",
                ChildGtin14 => "Y3",
                ChildGtin12 => "Y4",
                MapEditionNumber => "YM",
                PublicationNumber => "YP",
                CodeZB => "ZB",
                ServiceControlIdentification => "ZR",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ProductServiceIdQualifier> {
        use ProductServiceIdQualifier::*;
        match code {
            b"00" => Some(CreditCard),
            b"A1" => Some(PlantEquipmentNumber),
            b"A2" => Some(CodeA2),
            b"A3" => Some(LocallyAssignedControlNumber),
            b"A4" => Some(SubsistenceIdentificationNumber),
            b"A5" => Some(ApplicationStateOrProvince),
            b"A6" => Some(DocumentIdentificationCode),
            b"A7" => Some(SublineItemNumber),
            b"A8" => Some(ExhibitLineItemNumber),
            b"A9" => Some(Activity),
            b"AA" => Some(CodeAA),
            b"AB" => Some(Assembly),
            b"AC" => Some(CodeAC),
            b"AD" => Some(AmericanDentalAssociationCodes),
            b"AE" => Some(CodeAE),
            b"AF" => Some(FrontAxleSerialNumber),
            b"AG" => Some(Age),
            b"AH" => Some(BillboardsCommercial),
            b"AI" => Some(AlternateIsbn),
            b"AJ" => Some(PiggybackCommercial),
            b"AK" => Some(RefinedProductCode),
            b"AL" => Some(AxleSerialNumber),
            b"AM" => Some(AllocationMethodologyIdentificationCode),
            b"AN" => Some(AssetNumber),
            b"AO" => Some(AssemblageIdentificationNumber),
            b"AP" => Some(CodeAP),
            b"AQ" => Some(AppraisalProductType),
            b"AR" => Some(ArincPartNumber),
            b"AS" => Some(RearAxleSerialNumber),
            b"AT" => Some(AssortabilityCode),
            b"AU" => Some(AppraisalService),
            b"AV" => Some(AdditionalProductIdentificationAssignedByTheManufacturer),
            b"AW" => Some(WellNumber),
            b"AX" => Some(AssemblyLotNumber),
            b"AY" => Some(CodeAY),
            b"AZ" => Some(AxleRatio),
            b"B1" => Some(VolumeSet),
            b"B2" => Some(LibraryOfCongressCode),
            b"B3" => Some(EditionCode),
            b"B4" => Some(BindingCode),
            b"B5" => Some(TitleCode),
            b"B6" => Some(AuthorCode),
            b"B7" => Some(PublisherCode),
            b"B8" => Some(BatchNumber),
            b"B9" => Some(FrontAxleDrivenSerialNumber),
            b"BA" => Some(BaleNumber),
            b"BB" => Some(PrintColorStandardReference),
            b"BC" => Some(SupplierBrandCode),
            b"BD" => Some(BuyerGroundShade),
            b"BE" => Some(BuyersEngineeringChangeLevelNumber),
            b"BF" => Some(VendorGroundShade),
            b"BG" => Some(GroundShadeStandardReference),
            b"BH" => Some(FinishHandStandardReference),
            b"BI" => Some(RearAxleRearNonDrivenSerialNumber),
            b"BJ" => Some(ApplicationCompletionMethod),
            b"BK" => Some(Backing),
            b"BL" => Some(BrandLabel),
            b"BM" => Some(BeamNumber),
            b"BN" => Some(BarCodedSerialNumber),
            b"BO" => Some(BuyersColor),
            b"BP" => Some(BuyersPartNumber),
            b"BQ" => Some(BenefitId),
            b"BR" => Some(CompensationTypeCode),
            b"BS" => Some(BillSubgroupCode),
            b"BT" => Some(VolumeTypeIdentificationCode),
            b"BU" => Some(BusBodySerialNumber),
            b"BV" => Some(BeverageCommonCode),
            b"BW" => Some(CompensationAllocationCode),
            b"BX" => Some(BillCreditCode),
            b"BY" => Some(ConceptCode),
            b"BZ" => Some(CoverageType),
            b"C1" => Some(Channel),
            b"C2" => Some(ConnectorType),
            b"C3" => Some(Classification),
            b"C4" => Some(ConfigurationItem),
            b"C5" => Some(InsurancePlanDescriptionCharacteristics),
            b"C6" => Some(AssetType),
            b"C7" => Some(AssetCategory),
            b"C8" => Some(FundSubAdvisor),
            b"C9" => Some(CodeC9),
            b"CA" => Some(Case),
            b"CB" => Some(BuyersCatalogNumber),
            b"CC" => Some(CompatibleCutNumber),
            b"CD" => Some(MotorVehicleLineDesignator),
            b"CE" => Some(ClassOfContractCode),
            b"CF" => Some(ChassisSerialNumber),
            b"CG" => Some(CommodityGrouping),
            b"CH" => Some(CountryOfOriginCode),
            b"CI" => Some(CodeCI),
            b"CJ" => Some(CodeCJ),
            b"CK" => Some(CandidateOil),
            b"CL" => Some(Color),
            b"CM" => Some(NationalRetailFederationColorCode),
            b"CN" => Some(CommodityName),
            b"CO" => Some(CodeCO),
            b"CP" => Some(CarryOverPartNumber),
            b"CQ" => Some(EquipmentCodePerCopasStandard),
            b"CR" => Some(ContractNumber),
            b"CS" => Some(ServiceCodePerCopasStandard),
            b"CT" => Some(CodeCT),
            b"CU" => Some(CutNumber),
            b"CV" => Some(CustomerProvidedEquipment),
            b"CW" => Some(ContractActivityCode),
            b"CX" => Some(CompletionNumber),
            b"CY" => Some(CustomerCompanyRegistryNumber),
            b"CZ" => Some(CountryFromWhichProcured),
            b"D1" => Some(UnderwritingMethodOfDirectWriter),
            b"D2" => Some(CodeD2),
            b"D3" => Some(PolicyForm),
            b"D4" => Some(PlanCode),
            b"D5" => Some(CoverageRiskType),
            b"D6" => Some(MedicalStopLossLevelCodes),
            b"D7" => Some(MedicalStopLossCoverageCodes),
            b"D8" => Some(MedicalStopLossProductLineCodes),
            b"D9" => Some(RxNorm),
            b"DA" => Some(AutomobileRepairProductCode),
            b"DB" => Some(Gs1DataBar),
            b"DC" => Some(CodeDC),
            b"DD" => Some(Distributor),
            b"DE" => Some(DesignNumber),
            b"DF" => Some(DeviceFamily),
            b"DG" => Some(DiscountGrouping),
            b"DH" => Some(Transport4CommodityCode),
            b"DI" => Some(DepositItemNumber),
            b"DJ" => Some(DependentProprietaryProduct),
            b"DK" => Some(PrimaryAlternateProduct),
            b"DL" => Some(DyeLotNumber),
            b"DM" => Some(CodeDM),
            b"DN" => Some(DieNumber),
            b"DO" => Some(DividendUse),
            b"DP" => Some(DiscontinuedPartNumber),
            b"DQ" => Some(EventType),
            b"DR" => Some(DrawingRevisionNumber),
            b"DS" => Some(GroupId),
            b"DT" => Some(DeviceType),
            b"DU" => Some(JointLifeType),
            b"DV" => Some(LocationCode),
            b"DW" => Some(NonforfeitureOption),
            b"DX" => Some(CodeDX),
            b"DY" => Some(PremiumRateType),
            b"DZ" => Some(DiagnosisCodePointer),
            b"E1" => Some(ContractChangeAuthorizationType),
            b"E2" => Some(FundAbbreviation),
            b"E3" => Some(FundType),
            b"E4" => Some(RelatedPolicyIdentification),
            b"E5" => Some(TaxCode),
            b"E6" => Some(ContractTransferReasonCode),
            b"E7" => Some(BerensonEggersTypeOfService),
            b"E8" => Some(InternationalSocietyOfBloodTransfusionDeviceIdentifier),
            b"E9" => Some(CodeE9),
            b"EA" => Some(Ean99InStoreCouponCode),
            b"EB" => Some(FuelTankSerialNumber),
            b"EC" => Some(EngineeringChangeLevel),
            b"ED" => Some(EngineDisplacementIdentification),
            b"EE" => Some(PremiumUse),
            b"EF" => Some(Exhibit),
            b"EG" => Some(PurposeOfInsurance),
            b"EH" => Some(SalesPresentationId),
            b"EI" => Some(Expense),
            b"EJ" => Some(ServiceFeatureId),
            b"EK" => Some(SettlementPayoutOption),
            b"EL" => Some(BuyersSublineItemNumber),
            b"EM" => Some(EquipmentIdentificationNumber),
            b"EN" => Some(Gtin13),
            b"EO" => Some(Gtin8),
            b"EP" => Some(BuyersEndProductNumber),
            b"EQ" => Some(EquipmentType),
            b"ER" => Some(JurisdictionSpecificProcedureAndSupplyCodes),
            b"ES" => Some(EngineSerialNumber),
            b"ET" => Some(DepartmentOfDefenseEnterprise),
            b"EU" => Some(SkillCode),
            b"EV" => Some(EquivalentProduct),
            b"EX" => Some(CodeEX),
            b"EZ" => Some(ShiftWorked),
            b"F1" => Some(CatalogNumber),
            b"F2" => Some(TechnicalOrderNumber),
            b"F3" => Some(TechnicalManualNumber),
            b"F4" => Some(Series),
            b"F5" => Some(ObligationAuthorityNumber),
            b"F6" => Some(FirstPrior),
            b"F7" => Some(EndItemDescription),
            b"F8" => Some(NextHigherUsedAssembly),
            b"F9" => Some(FormerPublisher),
            b"FA" => Some(FailedSubassemblySerialNumber),
            b"FB" => Some(FormNumber),
            b"FC" => Some(CouponFamilyCode),
            b"FD" => Some(Fund),
            b"FE" => Some(Feature),
            b"FF" => Some(FifthWheelSerialNumber),
            b"FG" => Some(FundManager),
            b"FH" => Some(FreddieMacAffordableLendingProductCode),
            b"FI" => Some(FinishNumber),
            b"FJ" => Some(FannieMaeAffordableLendingProductCode),
            b"FK" => Some(FreddieMacProjectCondominiumClassificationCode),
            b"FL" => Some(FinishLotNumber),
            b"FM" => Some(FailedSubassemblyModelNumber),
            b"FN" => Some(FinalTestLotNumber),
            b"FP" => Some(FabricPiecesPerRoll),
            b"FQ" => Some(FannieMaeProjectCondominiumClassificationCode),
            b"FR" => Some(CodeFR),
            b"FS" => Some(NationalStockNumber),
            b"FT" => Some(FederalSupplyClassification),
            b"FU" => Some(CodeFU),
            b"FV" => Some(CodeFV),
            b"FW" => Some(NewMicrocode),
            b"FX" => Some(FederalSupplyGroup),
            b"GA" => Some(Gathering),
            b"GC" => Some(GradeCode),
            b"GD" => Some(GrainDirection),
            b"GE" => Some(GenericNameDescription),
            b"GI" => Some(CodeGI),
            b"GK" => Some(GliderKit),
            b"GM" => Some(CodeGM),
            b"GN" => Some(GradeName),
            b"GQ" => Some(GroupQualifierCode),
            b"GR" => Some(GearRatio),
            b"GS" => Some(GeneralSpecificationNumber),
            b"GU" => Some(VolumeUsageIdentificationCode),
            b"GV" => Some(CodeGV),
            b"GW" => Some(CodeGW),
            b"GX" => Some(CodeGX),
            b"HA" => Some(CountrySubdivisionCode),
            b"HB" => Some(CountryOfOriginWithCountrySubdivisionCode),
            b"HC" => Some(CodeHC),
            b"HD" => Some(InternationalHarmonizedCommodityCode),
            b"HF" => Some(CountrySubdivisionFromWhereProcured),
            b"HI" => Some(CodeHI),
            b"HJ" => Some(ProvinceOrStateOfProductProcessingCode),
            b"HK" => Some(ProvinceOrStateOfProductPackaged),
            b"HN" => Some(HeatNumber),
            b"HP" => Some(CodeHP),
            b"HQ" => Some(AlcoholicBeverageSubregion),
            b"IA" => Some(InformationMediaType),
            b"IB" => Some(CodeIB),
            b"IC" => Some(InteriorColorNumber),
            b"ID" => Some(CodeID),
            b"IE" => Some(InsurersFundCode),
            b"IF" => Some(InvestmentFundType),
            b"IG" => Some(IgnitionKeyNumber),
            b"IH" => Some(CodeIH),
            b"II" => Some(CommodityItemIdentification),
            b"IJ" => Some(CodeIJ),
            b"IM" => Some(CodeIM),
            b"IN" => Some(BuyersItemNumber),
            b"IP" => Some(CodeIP),
            b"IQ" => Some(IrsQualificationCode),
            b"IR" => Some(Ingredient),
            b"IS" => Some(CodeIS),
            b"IT" => Some(BuyersStyleNumber),
            b"IU" => Some(DepartmentOfDefenseIssuingAgencyCode),
            b"IW" => Some(InterchangeabilityCode),
            b"IZ" => Some(BuyersSizeCode),
            b"JA" => Some(Anniversary),
            b"JB" => Some(Commission),
            b"JC" => Some(CommissionYear),
            b"JD" => Some(ContributionYear),
            b"JN" => Some(JobNumber),
            b"JP" => Some(PackageTypeCode),
            b"JS" => Some(JobSequenceNumber),
            b"KA" => Some(EngineeringDataList),
            b"KB" => Some(DataCategoryCode),
            b"KD" => Some(ReplacementNationalStockNumber),
            b"KE" => Some(MilitaryStandard),
            b"KF" => Some(ItemTypeNumber),
            b"KG" => Some(TimeCompliantTechnicalOrder),
            b"KI" => Some(CognizanceSymbol),
            b"KJ" => Some(MaterialControlCode),
            b"KK" => Some(SpecialMaterialIdentificationCode),
            b"KL" => Some(ItemManagementCode),
            b"KM" => Some(ShelfLifeCode),
            b"KN" => Some(ShelfLifeActionCode),
            b"KP" => Some(KanbanPlanNumber),
            b"L1" => Some(ProgramLevel),
            b"L2" => Some(TopicLevel),
            b"L3" => Some(SubtopicLevel),
            b"L4" => Some(LifeAnnuityServiceFeatures),
            b"L5" => Some(LineOfAuthority),
            b"L6" => Some(CodeL6),
            b"LA" => Some(LaborGroup),
            b"LB" => Some(CodeLB),
            b"LC" => Some(LaboratoryTestConditionCode),
            b"LD" => Some(CodeLD),
            b"LG" => Some(LiftGateSerialNumber),
            b"LM" => Some(LotteryGameNumber),
            b"LN" => Some(LotteryPackBookNumber),
            b"LP" => Some(LifeAnnuityProductCode),
            b"LR" => Some(LeaseNumber),
            b"LS" => Some(LoadSequence),
            b"LT" => Some(LotNumber),
            b"LU" => Some(LotPricingUnitNumber),
            b"MA" => Some(MachineNumber),
            b"MB" => Some(MeasurementTypeCode),
            b"MC" => Some(MortgageCreditDataOrderType),
            b"MD" => Some(MethodOfDeliveryCode),
            b"ME" => Some(MarketProgramCode),
            b"MF" => Some(Manufacturer),
            b"MG" => Some(ManufacturersPartNumber),
            b"MH" => Some(MedicationCode),
            b"MI" => Some(MortgageInsuranceProductCodeOrNumber),
            b"MJ" => Some(ManualTransmissionSerialNumber),
            b"MK" => Some(FrontAxleNonDrivenSerialNumber),
            b"MM" => Some(CodeMM),
            b"MN" => Some(ModelNumber),
            b"MO" => Some(MovementTypeCode),
            b"MP" => Some(MortgageProductCode),
            b"MQ" => Some(MortgageUnderwritingType),
            b"MR" => Some(MaintenanceIndexPageReferenceNumber),
            b"MS" => Some(CodeMS),
            b"MT" => Some(MajorProductMaterialMachineType),
            b"MU" => Some(AuthorizedPartsListNumber),
            b"MV" => Some(EquipmentLocation),
            b"MW" => Some(EquipmentHierarchicalSequence),
            b"MX" => Some(RepairInduction),
            b"N1" => Some(NationalDrugCodeIn442Format),
            b"N2" => Some(NationalDrugCodeIn532Format),
            b"N3" => Some(NationalDrugCodeIn541Format),
            b"N4" => Some(NationalDrugCodeIn542Format),
            b"N5" => Some(NationalHealthRelatedItemCodeIn55Format),
            b"N6" => Some(NationalHealthRelatedItemCodeIn46Format),
            b"ND" => Some(CodeND),
            b"NE" => Some(YarnCountEnglish),
            b"NG" => Some(CodeNG),
            b"NH" => Some(NationalHealthRelatedItemCode),
            b"NM" => Some(YarnCountMetric),
            b"NN" => Some(NationalItemIdentificationNumber),
            b"NP" => Some(NaturalHealthProductNumber),
            b"NR" => Some(CodeNR),
            b"NU" => Some(CodeNU),
            b"NW" => Some(NewReplacementPartOrAssemblyDefective),
            b"NZ" => Some(CombinedNccmaBankServiceCode),
            b"O9" => Some(CodeO9),
            b"OE" => Some(OriginalEquipmentNumber),
            b"OF" => Some(CodeOF),
            b"OG" => Some(OldMicrocode),
            b"OH" => Some(OppositeHandPartNumber),
            b"OI" => Some(OpticalIndustryProductCode),
            b"OL" => Some(OpticalCableCode),
            b"OM" => Some(OriginalPartNumber),
            b"ON" => Some(CustomerOrderNumber),
            b"OO" => Some(OutsideProductionOperationSheetNumber),
            b"OP" => Some(ObsoletePartNumber),
            b"OR" => Some(OfferNumber),
            b"OT" => Some(InternalNumber),
            b"OU" => Some(OriginalUnitOfIssue),
            b"P1" => {
                Some(
                    PetroleumAccountantsSocietyOfCanadaOperatingAndMaintenanceCodeGoodsAndServicesTaxNotApplicable,
                )
            }
            b"P2" => {
                Some(
                    PetroleumAccountantsSocietyOfCanadaCapitalExpenditureCodeGoodsAndServicesTaxNotApplicable,
                )
            }
            b"P3" => Some(PetroleumAccountantsSocietyOfCanadaTubularCode),
            b"P4" => Some(PetroleumAccountantsSocietyOfCanadaNonTubularCode),
            b"P5" => Some(MaterialDischargeNumber),
            b"P6" => Some(CodeP6),
            b"P7" => Some(PreviousCarrier),
            b"P8" => Some(CodeP8),
            b"P9" => Some(Ply),
            b"PA" => Some(PatternNumber),
            b"PB" => {
                Some(
                    PetroleumAccountantsSocietyOfCanadaOperatingAndMaintenanceCodeGoodsAndServicesTaxForwarded,
                )
            }
            b"PC" => Some(PrimeContractorPartNumber),
            b"PD" => Some(PartNumberDescription),
            b"PE" => Some(PiecesInRoll),
            b"PF" => {
                Some(
                    PetroleumAccountantsSocietyOfCanadaCapitalExpenditureCodeGoodsAndServicesTaxForwarded,
                )
            }
            b"PG" => Some(PackagingSpecificationNumber),
            b"PH" => Some(PropertyAndCasualtyServiceCode),
            b"PI" => Some(PurchasersItemCode),
            b"PJ" => Some(CodePJ),
            b"PK" => Some(PackagingDrawing),
            b"PL" => Some(PurchasersOrderLineNumber),
            b"PM" => Some(NumberOfPositionsOnMachine),
            b"PN" => Some(CompanyPartNumber),
            b"PO" => Some(PurchaseOrderNumber),
            b"PP" => Some(AirTransportationAssociationProprietaryRightsCode),
            b"PQ" => Some(ProductIdAttributeCode),
            b"PR" => Some(ProcessNumber),
            b"PS" => Some(Position),
            b"PT" => Some(PrintOrDrawing),
            b"PU" => Some(PartReferenceNumber),
            b"PV" => Some(AdvertisingPackageIdentificationCode),
            b"PW" => Some(PartDrawing),
            b"PX" => Some(SecondaryPly),
            b"PY" => Some(OperatorAssignedPropertyIdentification),
            b"PZ" => Some(ProductChangeNoticeNumber),
            b"R1" => Some(ReplacementSubassemblyModelNumber),
            b"R2" => Some(ReplacementSubassemblySerialNumber),
            b"R3" => Some(CodeR3),
            b"R4" => Some(CodeR4),
            b"R5" => Some(CodeR5),
            b"R6" => Some(CodeR6),
            b"R9" => Some(CodeR9),
            b"RA" => Some(ReturnCode),
            b"RB" => Some(CodeRB),
            b"RC" => Some(ReturnableContainerNumber),
            b"RD" => Some(ReelNumber),
            b"RE" => Some(ReeferSerialNumber),
            b"RF" => Some(RepairFromProductCode),
            b"RG" => Some(ReferenceOil),
            b"RH" => Some(RadiatorSerialNumber),
            b"RI" => Some(RearAxleFrontRearDrivenSerialNumber),
            b"RJ" => Some(RearAxleRearDrivenSerialNumber),
            b"RK" => Some(RackNumber),
            b"RL" => Some(RateDetailCard),
            b"RM" => Some(RelatedModelNumber),
            b"RN" => Some(ReleaseNumber),
            b"RO" => Some(RollNumber),
            b"RP" => Some(ReplacedPartNumber),
            b"RQ" => Some(AutomobileRentalChargeItemCode),
            b"RR" => Some(ReplacementProductNumber),
            b"RS" => Some(SetNumber),
            b"RT" => Some(ReelType),
            b"RU" => Some(RunNumber),
            b"RV" => Some(RepairTagNumber),
            b"RW" => Some(RelativeValueUnits),
            b"RX" => Some(ProvisioningReferenceNumber),
            b"RY" => Some(RecordKeepingOrModelYear),
            b"RZ" => Some(RelatedModelType),
            b"S1" => Some(ShippersItemNumber),
            b"S2" => Some(SecondPrior),
            b"S3" => Some(Phase),
            b"S4" => Some(LaboratorySampleIdentification),
            b"S5" => Some(StateSampleIdentification),
            b"S6" => Some(PreviousSampleIdentification),
            b"S7" => Some(SourceOfDepositCode),
            b"S8" => Some(SourceOfLeadCode),
            b"SA" => Some(SchematicDiagramReferenceNumber),
            b"SB" => Some(SubmissionNumber),
            b"SC" => Some(SellersDateCode),
            b"SD" => Some(SupplierCompanyRegistryNumber),
            b"SE" => Some(SectionPrintNumber),
            b"SF" => Some(SurfaceFinish),
            b"SG" => Some(SeatSerialNumber),
            b"SH" => Some(ServiceRequested),
            b"SI" => Some(StandardIndustrialClassificationCode),
            b"SJ" => Some(ReligiousRetailNonBookItem),
            b"SK" => Some(CodeSK),
            b"SL" => Some(SellersLotNumber),
            b"SM" => Some(NationalRetailFederationSizeCode),
            b"SN" => Some(SerialNumber),
            b"SO" => Some(System),
            b"SP" => Some(SupersededPurchaseOrderNumber),
            b"SQ" => Some(RollSequenceNumber),
            b"SR" => Some(SubstituteProductNumber),
            b"SS" => Some(SupersededPartNumber),
            b"ST" => Some(StyleNumber),
            b"SU" => Some(SideUpSideDown),
            b"SV" => Some(ServiceRendered),
            b"SW" => Some(StockNumber),
            b"SX" => Some(SleeperBoxKeyNumber),
            b"SY" => Some(SleeperBoxSerialNumber),
            b"SZ" => Some(VendorAlphanumericSizeCode),
            b"T2" => Some(Tex),
            b"T3" => Some(ThirdPrior),
            b"T5" => Some(SubstituteGtin8),
            b"T6" => Some(SubstituteGtin12),
            b"T7" => Some(SubstituteGtin13),
            b"T8" => Some(SubstituteGtin14),
            b"TA" => Some(PipelineTransactionCode),
            b"TB" => {
                Some(AssociationForFinancialProfessionalsServiceCodeAndBankServiceCode)
            }
            b"TC" => Some(TelecommunicationsCircuitId),
            b"TD" => Some(TreatmentCodes),
            b"TE" => Some(AssociationForFinancialProfessionalsServiceCode),
            b"TF" => Some(CodeTF),
            b"TG" => Some(AutomaticTransmissionSerialNumber),
            b"TH" => Some(TransferCaseSerialNumber),
            b"TI" => Some(TradeIn),
            b"TJ" => Some(AuxiliaryTransmissionSerialNumber),
            b"TK" => Some(CodeTK),
            b"TM" => Some(TelephoneIndustryManufacturerCode),
            b"TN" => Some(RailroadOwnedUnitTrainNumber),
            b"TP" => Some(ProductTypeCode),
            b"TR" => Some(TruckBodySerialNumber),
            b"TS" => Some(TransmissionSerialNumber),
            b"TT" => Some(TypeSelvage),
            b"TU" => Some(TestedMaterialIdentificationNumber),
            b"TV" => Some(LineOfBusiness),
            b"TW" => Some(ProgramCode),
            b"TX" => Some(CodeTX),
            b"TY" => Some(TelecommunicationsIndustryServiceCode),
            b"TZ" => Some(ProgramDescription),
            b"U3" => Some(CodeU3),
            b"U5" => Some(BrokerPriceOpinionService),
            b"U6" => Some(RealEstatePropertyInformationService),
            b"U7" => Some(DepartmentOfDefenseUniqueItem),
            b"UC" => Some(ProductVariant),
            b"UF" => Some(UserDefinedShippingContainer),
            b"UJ" => Some(CodeUJ),
            b"UK" => Some(Gtin14),
            b"UL" => Some(CodeUL),
            b"UM" => Some(CodeUM),
            b"UO" => Some(CodeUO),
            b"UP" => Some(Gtin12),
            b"UQ" => Some(CodeUQ),
            b"UR" => Some(UccEan128CouponExtendedCode),
            b"US" => Some(UniformStockSymbolSystemCodeNumber),
            b"UT" => Some(CompanyOwnedUnitTrainNumber),
            b"UU" => Some(Device),
            b"UV" => Some(CodeUV),
            b"UX" => Some(UniversalProductNumber),
            b"UY" => Some(DepartmentOfDefenseUniqueItemTypeReference),
            b"UZ" => Some(Gs1UsCouponCodeWithGs1128CouponExtendedCode),
            b"V1" => Some(IngredientCountryOfOriginCode),
            b"V2" => Some(TariffCountryOfOriginCode),
            b"V3" => Some(CountryOfLastProcessingCode),
            b"V4" => Some(CountryOfAssemblyCode),
            b"V5" => Some(LogisticsCountryOfOriginCode),
            b"V6" => Some(CodeV6),
            b"V7" => Some(ConsumerProductVariant),
            b"VA" => Some(VendorsStyleNumber),
            b"VB" => Some(VendorsEngineeringChangeLevelNumber),
            b"VC" => Some(CodeVC),
            b"VE" => Some(VendorColor),
            b"VI" => Some(VaryItemProductNumber),
            b"VM" => Some(VehicleMaintenanceReportingStandards),
            b"VN" => Some(CodeVN),
            b"VO" => Some(VendorsOrderNumber),
            b"VP" => Some(CodeVP),
            b"VS" => Some(VendorsSupplementalItemNumber),
            b"VT" => Some(Vintage),
            b"VU" => Some(VendorsBasicUnitNumber),
            b"VV" => Some(MotorVehicleIdNumber),
            b"VX" => Some(VendorsSpecificationNumber),
            b"W1" => Some(EndItemSerialNumber),
            b"W2" => Some(WorkUnitNumber),
            b"W5" => Some(ReclamationProcess),
            b"W6" => Some(WoolenRun),
            b"W7" => Some(WoolenCut),
            b"WA" => Some(RandomWeightAggregationCode),
            b"WB" => Some(CarClassCode),
            b"WC" => Some(WorldCode),
            b"WD" => Some(AirlineFlightCode),
            b"WE" => Some(FareBasisCode),
            b"WF" => Some(ServiceClassCode),
            b"WG" => Some(StopOverCode),
            b"WJ" => Some(TelecomServiceType),
            b"WK" => Some(CodeWK),
            b"WL" => Some(WaferLot),
            b"WR" => Some(YarnCountWorsted),
            b"WS" => Some(WheelChairLiftSerialNumber),
            b"XA" => Some(PreferredPartNumber),
            b"XC" => Some(ExpendableContainerIdentification),
            b"XN" => Some(CodeXN),
            b"XP" => Some(PreferredNationalStockNumber),
            b"XQ" => Some(PreferredManufacturer),
            b"XZ" => Some(ContractorEstablishmentCode),
            b"Y1" => Some(ChildGtin13),
            b"Y3" => Some(ChildGtin14),
            b"Y4" => Some(ChildGtin12),
            b"YM" => Some(MapEditionNumber),
            b"YP" => Some(PublicationNumber),
            b"ZB" => Some(CodeZB),
            b"ZR" => Some(ServiceControlIdentification),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ProductServiceIdQualifier::*;
        match self {
            CreditCard => "Credit Card",
            PlantEquipmentNumber => "Plant Equipment Number",
            CodeA2 => "Department of Defense Identification Code (DoDIC)",
            LocallyAssignedControlNumber => "Locally Assigned Control Number",
            SubsistenceIdentificationNumber => "Subsistence Identification Number",
            ApplicationStateOrProvince => "Application State or Province",
            DocumentIdentificationCode => "Document Identification Code",
            SublineItemNumber => "Subline Item Number",
            ExhibitLineItemNumber => "Exhibit Line Item Number",
            Activity => "Activity",
            CodeAA => {
                "Assembly Level Code, Vehicle Maintenance Reporting Standards (VMRS) 2000 Code Key 32"
            }
            Assembly => "Assembly",
            CodeAC => "Aggregation Code (Used to Consolidate Part Families)",
            AmericanDentalAssociationCodes => "American Dental Association Codes",
            CodeAE => "Serial Item and Contribution Identifier (Defined in ANSI 239.56)",
            FrontAxleSerialNumber => "Front Axle Serial Number",
            Age => "Age",
            BillboardsCommercial => "Billboards Commercial",
            AlternateIsbn => "Alternate ISBN",
            PiggybackCommercial => "Piggyback Commercial",
            RefinedProductCode => "Refined Product Code",
            AxleSerialNumber => "Axle Serial Number",
            AllocationMethodologyIdentificationCode => {
                "Allocation Methodology Identification Code"
            }
            AssetNumber => "Asset Number",
            AssemblageIdentificationNumber => "Assemblage Identification Number",
            CodeAP => {
                "Component Level Code, Vehicle Maintenance Reporting Standards (VMRS) 2000 Code Key 33"
            }
            AppraisalProductType => "Appraisal Product Type",
            ArincPartNumber => "ARINC Part Number",
            RearAxleSerialNumber => "Rear Axle Serial Number",
            AssortabilityCode => "Assortability Code",
            AppraisalService => "Appraisal Service",
            AdditionalProductIdentificationAssignedByTheManufacturer => {
                "Additional Product Identification Assigned by the Manufacturer"
            }
            WellNumber => "Well Number",
            AssemblyLotNumber => "Assembly Lot Number",
            CodeAY => {
                "System Level Code, Vehicle Maintenance Reporting Standards (VMRS) 2000 Code Key 31"
            }
            AxleRatio => "Axle Ratio",
            VolumeSet => "Volume Set",
            LibraryOfCongressCode => "Library of Congress Code",
            EditionCode => "Edition Code",
            BindingCode => "Binding Code",
            TitleCode => "Title Code",
            AuthorCode => "Author Code",
            PublisherCode => "Publisher Code",
            BatchNumber => "Batch Number",
            FrontAxleDrivenSerialNumber => "Front Axle Driven Serial Number",
            BaleNumber => "Bale Number",
            PrintColorStandardReference => "Print Color Standard Reference",
            SupplierBrandCode => "Supplier Brand Code",
            BuyerGroundShade => "Buyer Ground Shade Identifier",
            BuyersEngineeringChangeLevelNumber => {
                "Buyer's Engineering Change Level Number"
            }
            VendorGroundShade => "Vendor Ground Shade Identifier",
            GroundShadeStandardReference => "Ground Shade Standard Reference",
            FinishHandStandardReference => "Finish/Hand Standard Reference",
            RearAxleRearNonDrivenSerialNumber => {
                "Rear Axle Rear Non Driven Serial Number"
            }
            ApplicationCompletionMethod => "Application Completion Method",
            Backing => "Backing",
            BrandLabel => "Brand/Label",
            BeamNumber => "Beam Number",
            BarCodedSerialNumber => "Bar-Coded Serial Number",
            BuyersColor => "Buyers Color",
            BuyersPartNumber => "Buyer's Part Number",
            BenefitId => "Benefit ID",
            CompensationTypeCode => "Compensation Type Code",
            BillSubgroupCode => "Bill Subgroup Code",
            VolumeTypeIdentificationCode => "Volume Type Identification Code",
            BusBodySerialNumber => "Bus Body Serial Number",
            BeverageCommonCode => "Beverage Common Code",
            CompensationAllocationCode => "Compensation Allocation Code",
            BillCreditCode => "Bill Credit Code",
            ConceptCode => "Concept Code",
            CoverageType => "Coverage Type",
            Channel => "Channel",
            ConnectorType => "Connector Type",
            Classification => "Classification",
            ConfigurationItem => "Configuration Item Identifier",
            InsurancePlanDescriptionCharacteristics => {
                "Insurance Plan Description Characteristics"
            }
            AssetType => "Asset Type",
            AssetCategory => "Asset Category",
            FundSubAdvisor => "Fund Sub-Advisor",
            CodeC9 => "Dun & Bradstreet Standard Product and Service Code",
            Case => "Case",
            BuyersCatalogNumber => "Buyer's Catalog Number",
            CompatibleCutNumber => "Compatible Cut Number",
            MotorVehicleLineDesignator => "Motor Vehicle-Line Designator",
            ClassOfContractCode => "Class of Contract Code",
            ChassisSerialNumber => "Chassis Serial Number",
            CommodityGrouping => "Commodity Grouping",
            CountryOfOriginCode => "Country of Origin Code",
            CodeCI => "Common Language Equipment Identifier (CLEI)",
            CodeCJ => "Current Procedural Terminology (CPT) Codes",
            CandidateOil => "Candidate Oil",
            Color => "Color",
            NationalRetailFederationColorCode => "National Retail Federation Color Code",
            CommodityName => "Commodity Name",
            CodeCO => "Chemical Abstract Service (CAS) Registry Number",
            CarryOverPartNumber => "Carry-over Part Number",
            EquipmentCodePerCopasStandard => "Equipment Code per COPAS standard",
            ContractNumber => "Contract Number",
            ServiceCodePerCopasStandard => "Service code per COPAS standard",
            CodeCT => "Continuation (ID Number Spans Multiple Product ID Data Elements)",
            CutNumber => "Cut Number",
            CustomerProvidedEquipment => "Customer Provided Equipment",
            ContractActivityCode => "Contract Activity Code",
            CompletionNumber => "Completion Number",
            CustomerCompanyRegistryNumber => "Customer Company Registry Number",
            CountryFromWhichProcured => "Country from which Procured",
            UnderwritingMethodOfDirectWriter => "Underwriting Method of Direct Writer",
            CodeD2 => "Medical Information Bureau (MIB) Authorization",
            PolicyForm => "Policy Form",
            PlanCode => "Plan Code",
            CoverageRiskType => "Coverage Risk Type",
            MedicalStopLossLevelCodes => "Medical Stop Loss Level Codes",
            MedicalStopLossCoverageCodes => "Medical Stop Loss Coverage Codes",
            MedicalStopLossProductLineCodes => "Medical Stop Loss Product Line Codes",
            RxNorm => "RxNorm",
            AutomobileRepairProductCode => "Automobile Repair Product Code",
            Gs1DataBar => "GS1 DataBar",
            CodeDC => {
                "International Classification of Diseases, 10th Revision, Clinical Modification (ICD-10-CM)"
            }
            Distributor => "Distributor",
            DesignNumber => "Design Number",
            DeviceFamily => "Device Family",
            DiscountGrouping => "Discount Grouping",
            Transport4CommodityCode => "Transport4 Commodity Code",
            DepositItemNumber => "Deposit Item Number",
            DependentProprietaryProduct => "Dependent Proprietary Product",
            PrimaryAlternateProduct => "Primary Alternate Product",
            DyeLotNumber => "Dye Lot Number",
            CodeDM => {
                "Committee for Uniform Security Identification Procedure Number (CUSIP) Number"
            }
            DieNumber => "Die Number",
            DividendUse => "Dividend Use",
            DiscontinuedPartNumber => "Discontinued Part Number",
            EventType => "Event Type",
            DrawingRevisionNumber => "Drawing Revision Number",
            GroupId => "Group ID",
            DeviceType => "Device Type",
            JointLifeType => "Joint Life Type",
            LocationCode => "Location Code",
            NonforfeitureOption => "Nonforfeiture Option",
            CodeDX => {
                "International Classification of Diseases, 9th Revision, Clinical Modification (ICD-9-CM) - Diagnosis"
            }
            PremiumRateType => "Premium Rate Type",
            DiagnosisCodePointer => "Diagnosis Code Pointer",
            ContractChangeAuthorizationType => "Contract Change Authorization Type",
            FundAbbreviation => "Fund Abbreviation",
            FundType => "Fund Type",
            RelatedPolicyIdentification => "Related Policy Identification",
            TaxCode => "Tax Code",
            ContractTransferReasonCode => "Contract Transfer Reason Code",
            BerensonEggersTypeOfService => "Berenson-Eggers Type of Service",
            InternationalSocietyOfBloodTransfusionDeviceIdentifier => {
                "International Society of Blood Transfusion device identifier"
            }
            CodeE9 => "Human Cell, Tissue or Cellular or Tissue-Based Product code",
            Ean99InStoreCouponCode => "EAN-99 In-store Coupon Code",
            FuelTankSerialNumber => "Fuel Tank Serial Number",
            EngineeringChangeLevel => "Engineering Change Level",
            EngineDisplacementIdentification => "Engine Displacement Identification",
            PremiumUse => "Premium Use",
            Exhibit => "Exhibit Identifier",
            PurposeOfInsurance => "Purpose of Insurance",
            SalesPresentationId => "Sales Presentation ID",
            Expense => "Expense Identifier",
            ServiceFeatureId => "Service Feature ID",
            SettlementPayoutOption => "Settlement/Payout Option",
            BuyersSublineItemNumber => "Buyer's Subline Item Number",
            EquipmentIdentificationNumber => "Equipment Identification Number",
            Gtin13 => "GTIN-13",
            Gtin8 => "GTIN-8",
            BuyersEndProductNumber => "Buyer's End Product Number",
            EquipmentType => "Equipment Type",
            JurisdictionSpecificProcedureAndSupplyCodes => {
                "Jurisdiction Specific Procedure and Supply Codes"
            }
            EngineSerialNumber => "Engine Serial Number",
            DepartmentOfDefenseEnterprise => {
                "Department of Defense Enterprise Identifier"
            }
            SkillCode => "Skill Code",
            EquivalentProduct => "Equivalent Product",
            CodeEX => "Exchanged Part, Assembly or Product",
            ShiftWorked => "Shift Worked",
            CatalogNumber => "Catalog Number",
            TechnicalOrderNumber => "Technical Order Number",
            TechnicalManualNumber => "Technical Manual Number",
            Series => "Series Identifier",
            ObligationAuthorityNumber => "Obligation Authority Number",
            FirstPrior => "First Prior Identifier",
            EndItemDescription => "End-Item Description",
            NextHigherUsedAssembly => "Next Higher Used Assembly",
            FormerPublisher => "Former Publisher",
            FailedSubassemblySerialNumber => "Failed Subassembly Serial Number",
            FormNumber => "Form Number",
            CouponFamilyCode => "Coupon Family Code",
            Fund => "Fund",
            Feature => "Feature",
            FifthWheelSerialNumber => "Fifth Wheel Serial Number",
            FundManager => "Fund Manager",
            FreddieMacAffordableLendingProductCode => {
                "Freddie Mac Affordable Lending Product Code"
            }
            FinishNumber => "Finish Number",
            FannieMaeAffordableLendingProductCode => {
                "Fannie Mae Affordable Lending Product Code"
            }
            FreddieMacProjectCondominiumClassificationCode => {
                "Freddie Mac Project Condominium Classification Code"
            }
            FinishLotNumber => "Finish Lot Number",
            FailedSubassemblyModelNumber => "Failed Subassembly Model Number",
            FinalTestLotNumber => "Final Test Lot Number",
            FabricPiecesPerRoll => "Fabric Pieces Per Roll",
            FannieMaeProjectCondominiumClassificationCode => {
                "Fannie Mae Project Condominium Classification Code"
            }
            CodeFR => "Front Axle, Rear",
            NationalStockNumber => "National Stock Number",
            FederalSupplyClassification => "Federal Supply Classification",
            CodeFU => {
                "National Alcohol Beverage Control Association (NABCA) Product Code"
            }
            CodeFV => "Drug Identification Number (DIN)",
            NewMicrocode => "New Microcode",
            FederalSupplyGroup => "Federal Supply Group",
            Gathering => "Gathering",
            GradeCode => "Grade Code",
            GrainDirection => "Grain Direction",
            GenericNameDescription => "Generic Name Description",
            CodeGI => "Graphics Industry Bar Code (GIBC)",
            GliderKit => "Glider Kit",
            CodeGM => "General Services Administration (GSA) Special Item Number",
            GradeName => "Grade Name",
            GroupQualifierCode => "Group Qualifier Code",
            GearRatio => "Gear Ratio",
            GeneralSpecificationNumber => "General Specification Number",
            VolumeUsageIdentificationCode => "Volume Usage Identification Code",
            CodeGV => "Serialized Global Returnable Asset Identifier (GRAI)",
            CodeGW => "Global Returnable Asset Identifier (GRAI)",
            CodeGX => "Global Individual Asset Identifier (GIAI)",
            CountrySubdivisionCode => "Country Subdivision Code",
            CountryOfOriginWithCountrySubdivisionCode => {
                "Country of Origin with Country Subdivision Code"
            }
            CodeHC => "Healthcare Common Procedure Coding System (HCPCS) Codes",
            InternationalHarmonizedCommodityCode => {
                "International Harmonized Commodity Code"
            }
            CountrySubdivisionFromWhereProcured => {
                "Country Subdivision from where Procured"
            }
            CodeHI => {
                "HIBC (Health Care Industry Bar Code) Supplier Labeling Standard Primary Data Message"
            }
            ProvinceOrStateOfProductProcessingCode => {
                "Province or State of Product Processing Code"
            }
            ProvinceOrStateOfProductPackaged => "Province or State of Product Packaged",
            HeatNumber => "Heat Number",
            CodeHP => "Health Insurance Prospective Payment System (HIPPS) Rate Code",
            AlcoholicBeverageSubregion => "Alcoholic Beverage Subregion",
            InformationMediaType => "Information Media Type",
            CodeIB => "International Standard Book Number (ISBN)",
            InteriorColorNumber => "Interior Color Number",
            CodeID => {
                "International Classification of Diseases, 9th Revision, Clinical Modification (ICD-9-CM) - Procedure"
            }
            InsurersFundCode => "Insurer's Fund Code",
            InvestmentFundType => "Investment Fund Type",
            IgnitionKeyNumber => "Ignition Key Number",
            CodeIH => {
                "International Classification of Diseases, 11th Revision,Clinical Modification (ICD-11-CM)"
            }
            CommodityItemIdentification => "Commodity Item Identification",
            CodeIJ => {
                "International Classification of Diseases, 11th Revision, Procedure Coding System (ICD-11-PCS)"
            }
            CodeIM => "Imprint (Trademark Code of Subsidiary)",
            BuyersItemNumber => "Buyer's Item Number",
            CodeIP => {
                "International Classification of Diseases, 10th Revision, Procedure Coding System (ICD-10-PCS)"
            }
            IrsQualificationCode => "IRS Qualification Code",
            Ingredient => "Ingredient",
            CodeIS => "International Standard Serial Number (ISSN)",
            BuyersStyleNumber => "Buyer's Style Number",
            DepartmentOfDefenseIssuingAgencyCode => {
                "Department of Defense Issuing Agency Code"
            }
            InterchangeabilityCode => "Interchangeability Code",
            BuyersSizeCode => "Buyer's Size Code",
            Anniversary => "Anniversary",
            Commission => "Commission Identifier",
            CommissionYear => "Commission Year",
            ContributionYear => "Contribution Year",
            JobNumber => "Job Number",
            PackageTypeCode => "Package Type Code",
            JobSequenceNumber => "Job Sequence Number",
            EngineeringDataList => "Engineering Data List",
            DataCategoryCode => "Data Category Code",
            ReplacementNationalStockNumber => "Replacement National Stock Number",
            MilitaryStandard => "Military Standard",
            ItemTypeNumber => "Item Type Number",
            TimeCompliantTechnicalOrder => "Time Compliant Technical Order",
            CognizanceSymbol => "Cognizance Symbol",
            MaterialControlCode => "Material Control Code",
            SpecialMaterialIdentificationCode => "Special Material Identification Code",
            ItemManagementCode => "Item Management Code",
            ShelfLifeCode => "Shelf-Life Code",
            ShelfLifeActionCode => "Shelf-Life Action Code",
            KanbanPlanNumber => "Kanban Plan Number",
            ProgramLevel => "Program Level",
            TopicLevel => "Topic Level",
            SubtopicLevel => "Subtopic Level",
            LifeAnnuityServiceFeatures => "Life/Annuity Service Features",
            LineOfAuthority => "Line of Authority",
            CodeL6 => "Lube, Synthetic",
            LaborGroup => "Labor Group",
            CodeLB => "Logical Observation Identifier Names and Codes (LOINC) Codes",
            LaboratoryTestConditionCode => "Laboratory Test Condition Code",
            CodeLD => "SNOMED, Systematized Nomenclature of Medicine",
            LiftGateSerialNumber => "Lift Gate Serial Number",
            LotteryGameNumber => "Lottery Game Number",
            LotteryPackBookNumber => "Lottery Pack/Book Number",
            LifeAnnuityProductCode => "Life/Annuity Product Code",
            LeaseNumber => "Lease Number",
            LoadSequence => "Load Sequence",
            LotNumber => "Lot Number",
            LotPricingUnitNumber => "Lot Pricing Unit Number",
            MachineNumber => "Machine Number",
            MeasurementTypeCode => "Measurement Type Code",
            MortgageCreditDataOrderType => "Mortgage Credit Data Order Type",
            MethodOfDeliveryCode => "Method of Delivery Code",
            MarketProgramCode => "Market Program Code",
            Manufacturer => "Manufacturer",
            ManufacturersPartNumber => "Manufacturer's Part Number",
            MedicationCode => "Medication Code",
            MortgageInsuranceProductCodeOrNumber => {
                "Mortgage Insurance Product Code or Number"
            }
            ManualTransmissionSerialNumber => "Manual Transmission Serial Number",
            FrontAxleNonDrivenSerialNumber => "Front Axle Non Driven Serial Number",
            CodeMM => {
                "Motor Equipment Manufacturing Association (MEMA) Product Type Code"
            }
            ModelNumber => "Model Number",
            MovementTypeCode => "Movement Type Code",
            MortgageProductCode => "Mortgage Product Code",
            MortgageUnderwritingType => "Mortgage Underwriting Type",
            MaintenanceIndexPageReferenceNumber => {
                "Maintenance Index Page Reference Number"
            }
            CodeMS => "Military Specification (MILSPEC) Number",
            MajorProductMaterialMachineType => "Major Product/Material/Machine Type",
            AuthorizedPartsListNumber => "Authorized Parts List Number",
            EquipmentLocation => "Equipment Location",
            EquipmentHierarchicalSequence => "Equipment Hierarchical Sequence Identifier",
            RepairInduction => "Repair Induction Identifier",
            NationalDrugCodeIn442Format => "National Drug Code in 4-4-2 Format",
            NationalDrugCodeIn532Format => "National Drug Code in 5-3-2 Format",
            NationalDrugCodeIn541Format => "National Drug Code in 5-4-1 Format",
            NationalDrugCodeIn542Format => "National Drug Code in 5-4-2 Format",
            NationalHealthRelatedItemCodeIn55Format => {
                "National Health Related Item Code in 5-5 Format"
            }
            NationalHealthRelatedItemCodeIn46Format => {
                "National Health Related Item Code in 4-6 Format"
            }
            CodeND => "National Drug Code (NDC)",
            YarnCountEnglish => "Yarn Count - English",
            CodeNG => "National Glass Association (NAGS) Number",
            NationalHealthRelatedItemCode => "National Health Related Item Code",
            YarnCountMetric => "Yarn Count - Metric",
            NationalItemIdentificationNumber => "National Item Identification Number",
            NaturalHealthProductNumber => "Natural Health Product Number",
            CodeNR => "Non-resaleable item (excluding deposit) number",
            CodeNU => "National Uniform Billing Committee (NUBC) UB92 Codes",
            NewReplacementPartOrAssemblyDefective => {
                "New Replacement Part or Assembly Defective"
            }
            CombinedNccmaBankServiceCode => "Combined NCCMA/Bank Service Code",
            CodeO9 => "Old Vendor's (Seller's) Item Number",
            OriginalEquipmentNumber => "Original Equipment Number",
            CodeOF => "Old Common Language Equipment Identifier (CLEI) Code",
            OldMicrocode => "Old Microcode",
            OppositeHandPartNumber => "Opposite-Hand Part Number",
            OpticalIndustryProductCode => "Optical Industry Product Code",
            OpticalCableCode => "Optical Cable Code",
            OriginalPartNumber => "Original Part Number",
            CustomerOrderNumber => "Customer Order Number",
            OutsideProductionOperationSheetNumber => {
                "Outside Production Operation Sheet Number"
            }
            ObsoletePartNumber => "Obsolete Part Number",
            OfferNumber => "Offer Number",
            InternalNumber => "Internal Number",
            OriginalUnitOfIssue => "Original Unit of Issue",
            PetroleumAccountantsSocietyOfCanadaOperatingAndMaintenanceCodeGoodsAndServicesTaxNotApplicable => {
                "Petroleum Accountants Society of Canada Operating and Maintenance Code - Goods and Services Tax Not Applicable"
            }
            PetroleumAccountantsSocietyOfCanadaCapitalExpenditureCodeGoodsAndServicesTaxNotApplicable => {
                "Petroleum Accountants Society of Canada Capital Expenditure Code - Goods and Services Tax Not Applicable"
            }
            PetroleumAccountantsSocietyOfCanadaTubularCode => {
                "Petroleum Accountants Society of Canada Tubular Code"
            }
            PetroleumAccountantsSocietyOfCanadaNonTubularCode => {
                "Petroleum Accountants Society of Canada Non-Tubular Code"
            }
            MaterialDischargeNumber => "Material Discharge Number",
            CodeP6 => "Pump, Fire",
            PreviousCarrier => "Previous Carrier",
            CodeP8 => "Retail Price Look Up Number (PLU)",
            Ply => "Ply",
            PatternNumber => "Pattern Number",
            PetroleumAccountantsSocietyOfCanadaOperatingAndMaintenanceCodeGoodsAndServicesTaxForwarded => {
                "Petroleum Accountants Society of Canada Operating and Maintenance Code - Goods and Services Tax Forwarded"
            }
            PrimeContractorPartNumber => "Prime Contractor Part Number",
            PartNumberDescription => "Part Number Description",
            PiecesInRoll => "Pieces in Roll",
            PetroleumAccountantsSocietyOfCanadaCapitalExpenditureCodeGoodsAndServicesTaxForwarded => {
                "Petroleum Accountants Society of Canada Capital Expenditure Code - Goods and Services Tax Forwarded"
            }
            PackagingSpecificationNumber => "Packaging Specification Number",
            PropertyAndCasualtyServiceCode => "Property and Casualty Service Code",
            PurchasersItemCode => "Purchaser's Item Code",
            CodePJ => {
                "Product Date Code (A code indicating the period during which a product was manufactured.)"
            }
            PackagingDrawing => "Packaging Drawing",
            PurchasersOrderLineNumber => "Purchaser's Order Line Number",
            NumberOfPositionsOnMachine => "Number of Positions on Machine",
            CompanyPartNumber => "Company Part Number",
            PurchaseOrderNumber => "Purchase Order Number",
            AirTransportationAssociationProprietaryRightsCode => {
                "Air Transportation Association Proprietary Rights Code"
            }
            ProductIdAttributeCode => "Product ID Attribute Code",
            ProcessNumber => "Process Number",
            Position => "Position",
            PrintOrDrawing => "Print or Drawing",
            PartReferenceNumber => "Part Reference Number",
            AdvertisingPackageIdentificationCode => {
                "Advertising Package Identification Code"
            }
            PartDrawing => "Part Drawing",
            SecondaryPly => "Secondary Ply",
            OperatorAssignedPropertyIdentification => {
                "Operator Assigned Property Identification"
            }
            ProductChangeNoticeNumber => "Product Change Notice Number",
            ReplacementSubassemblyModelNumber => "Replacement Subassembly Model Number",
            ReplacementSubassemblySerialNumber => "Replacement Subassembly Serial Number",
            CodeR3 => "Rear Axle, Middle",
            CodeR4 => "Rear Axle, Pusher",
            CodeR5 => "Rear Axle, Tag",
            CodeR6 => "Rear Axle, Extended Tag",
            CodeR9 => "Replacement Vendor's (Seller's) Item Number",
            ReturnCode => "Return Code",
            CodeRB => "National Uniform Billing Committee (NUBC) UB82 Codes",
            ReturnableContainerNumber => "Returnable Container Number",
            ReelNumber => "Reel Number",
            ReeferSerialNumber => "Reefer Serial Number",
            RepairFromProductCode => "Repair From Product Code",
            ReferenceOil => "Reference Oil",
            RadiatorSerialNumber => "Radiator Serial Number",
            RearAxleFrontRearDrivenSerialNumber => {
                "Rear Axle Front Rear Driven Serial Number"
            }
            RearAxleRearDrivenSerialNumber => "Rear Axle Rear Driven Serial Number",
            RackNumber => "Rack Number",
            RateDetailCard => "Rate Detail Card",
            RelatedModelNumber => "Related Model Number",
            ReleaseNumber => "Release Number",
            RollNumber => "Roll Number",
            ReplacedPartNumber => "Replaced Part Number",
            AutomobileRentalChargeItemCode => "Automobile Rental Charge Item Code",
            ReplacementProductNumber => "Replacement Product Number",
            SetNumber => "Set Number",
            ReelType => "Reel Type",
            RunNumber => "Run Number",
            RepairTagNumber => "Repair Tag Number",
            RelativeValueUnits => "Relative Value Units",
            ProvisioningReferenceNumber => "Provisioning Reference Number",
            RecordKeepingOrModelYear => "Record Keeping or Model Year",
            RelatedModelType => "Related Model Type",
            ShippersItemNumber => "Shipper's Item Number",
            SecondPrior => "Second Prior Identifier",
            Phase => "Phase",
            LaboratorySampleIdentification => "Laboratory Sample Identification",
            StateSampleIdentification => "State Sample Identification",
            PreviousSampleIdentification => "Previous Sample Identification",
            SourceOfDepositCode => "Source of Deposit Code",
            SourceOfLeadCode => "Source of Lead Code",
            SchematicDiagramReferenceNumber => "Schematic Diagram Reference Number",
            SubmissionNumber => "Submission Number",
            SellersDateCode => "Seller's Date Code",
            SupplierCompanyRegistryNumber => "Supplier Company Registry Number",
            SectionPrintNumber => "Section Print Number",
            SurfaceFinish => "Surface Finish",
            SeatSerialNumber => "Seat Serial Number",
            ServiceRequested => "Service Requested",
            StandardIndustrialClassificationCode => {
                "Standard Industrial Classification Code"
            }
            ReligiousRetailNonBookItem => "Religious Retail Non-book Item",
            CodeSK => "Stock Keeping Unit (SKU)",
            SellersLotNumber => "Seller's Lot Number",
            NationalRetailFederationSizeCode => "National Retail Federation Size Code",
            SerialNumber => "Serial Number",
            System => "System Identifier",
            SupersededPurchaseOrderNumber => "Superseded Purchase Order Number",
            RollSequenceNumber => "Roll Sequence Number",
            SubstituteProductNumber => "Substitute Product Number",
            SupersededPartNumber => "Superseded Part Number",
            StyleNumber => "Style Number",
            SideUpSideDown => "Side Up/Side Down",
            ServiceRendered => "Service Rendered",
            StockNumber => "Stock Number",
            SleeperBoxKeyNumber => "Sleeper Box Key Number",
            SleeperBoxSerialNumber => "Sleeper Box Serial Number",
            VendorAlphanumericSizeCode => "Vendor Alphanumeric Size Code",
            Tex => "Tex",
            ThirdPrior => "Third Prior Identifier",
            SubstituteGtin8 => "Substitute GTIN-8",
            SubstituteGtin12 => "Substitute GTIN-12",
            SubstituteGtin13 => "Substitute GTIN-13",
            SubstituteGtin14 => "Substitute GTIN-14",
            PipelineTransactionCode => "Pipeline Transaction Code",
            AssociationForFinancialProfessionalsServiceCodeAndBankServiceCode => {
                "Association for Financial Professionals Service Code and Bank Service Code"
            }
            TelecommunicationsCircuitId => "Telecommunications Circuit ID",
            TreatmentCodes => "Treatment Codes",
            AssociationForFinancialProfessionalsServiceCode => {
                "Association for Financial Professionals Service Code"
            }
            CodeTF => "The Air Cargo Tariff (TACT) Commodity Code",
            AutomaticTransmissionSerialNumber => "Automatic Transmission Serial Number",
            TransferCaseSerialNumber => "Transfer Case Serial Number",
            TradeIn => "Trade In Identifier",
            AuxiliaryTransmissionSerialNumber => "Auxiliary Transmission Serial Number",
            CodeTK => "Transmission, Auxillary",
            TelephoneIndustryManufacturerCode => "Telephone Industry Manufacturer Code",
            RailroadOwnedUnitTrainNumber => "Railroad-Owned Unit Train Number",
            ProductTypeCode => "Product Type Code",
            TruckBodySerialNumber => "Truck Body Serial Number",
            TransmissionSerialNumber => "Transmission Serial Number",
            TypeSelvage => "Type Selvage",
            TestedMaterialIdentificationNumber => "Tested Material Identification Number",
            LineOfBusiness => "Line of Business",
            ProgramCode => "Program Code",
            CodeTX => "Federal Aviation Administration (FAA) Service Bulletin Number",
            TelecommunicationsIndustryServiceCode => {
                "Telecommunications Industry Service Code"
            }
            ProgramDescription => "Program Description Identifier",
            CodeU3 => "United Nations Common Coding System (UNCCS)",
            BrokerPriceOpinionService => "Broker Price Opinion Service",
            RealEstatePropertyInformationService => {
                "Real Estate Property Information Service"
            }
            DepartmentOfDefenseUniqueItem => {
                "Department of Defense Unique Item Identifier"
            }
            ProductVariant => "Product Variant",
            UserDefinedShippingContainer => "User-Defined Shipping Container Identifier",
            CodeUJ => "U.P.C./EAN Coupon Code (2-5-5)",
            Gtin14 => "GTIN-14",
            CodeUL => "U.P.C. Coupon Code (1-5-5-1)",
            CodeUM => "Universal Vendor Marking, Short Code (UVM; U Line)",
            CodeUO => "EAN.UCC Serial Shipping Container Code (SSCC)",
            Gtin12 => "GTIN-12",
            CodeUQ => "United Nations (UN) Number (Dangerous Goods)",
            UccEan128CouponExtendedCode => "UCC/EAN-128 Coupon Extended Code",
            UniformStockSymbolSystemCodeNumber => {
                "Uniform Stock Symbol System Code Number"
            }
            CompanyOwnedUnitTrainNumber => "Company-Owned Unit Train Number",
            Device => "Device Identifier",
            CodeUV => "Universal Vendor Marking, Long Code (UVM; R,P,M Lines)",
            UniversalProductNumber => "Universal Product Number",
            DepartmentOfDefenseUniqueItemTypeReference => {
                "Department of Defense Unique Item Identifier Type Reference Identifier"
            }
            Gs1UsCouponCodeWithGs1128CouponExtendedCode => {
                "GS1 US Coupon Code with GS1-128 Coupon Extended Code"
            }
            IngredientCountryOfOriginCode => "Ingredient Country of Origin Code",
            TariffCountryOfOriginCode => "Tariff Country of Origin Code",
            CountryOfLastProcessingCode => "Country of Last Processing Code",
            CountryOfAssemblyCode => "Country of Assembly Code",
            LogisticsCountryOfOriginCode => "Logistics Country of Origin Code",
            CodeV6 => {
                "United States Department of Agriculture (USDA) Country of Origin Code"
            }
            ConsumerProductVariant => "Consumer Product Variant Identifier",
            VendorsStyleNumber => "Vendor's Style Number",
            VendorsEngineeringChangeLevelNumber => {
                "Vendor's Engineering Change Level Number"
            }
            CodeVC => "Vendor's (Seller's) Catalog Number",
            VendorColor => "Vendor Color",
            VaryItemProductNumber => "Vary Item Product Number",
            VehicleMaintenanceReportingStandards => {
                "Vehicle Maintenance Reporting Standards"
            }
            CodeVN => "Vendor's (Seller's) Item Number",
            VendorsOrderNumber => "Vendor's Order Number",
            CodeVP => "Vendor's (Seller's) Part Number",
            VendorsSupplementalItemNumber => "Vendor's Supplemental Item Number",
            Vintage => "Vintage",
            VendorsBasicUnitNumber => "Vendor's Basic Unit Number",
            MotorVehicleIdNumber => "Motor Vehicle ID Number",
            VendorsSpecificationNumber => "Vendor's Specification Number",
            EndItemSerialNumber => "End Item Serial Number",
            WorkUnitNumber => "Work Unit Number",
            ReclamationProcess => "Reclamation Process",
            WoolenRun => "Woolen Run",
            WoolenCut => "Woolen Cut",
            RandomWeightAggregationCode => "Random Weight Aggregation Code",
            CarClassCode => "Car Class Code",
            WorldCode => "World Code",
            AirlineFlightCode => "Airline Flight Code",
            FareBasisCode => "Fare Basis Code",
            ServiceClassCode => "Service Class Code",
            StopOverCode => "Stop Over Code",
            TelecomServiceType => "Telecom Service Type",
            CodeWK => "Advanced Billing Concepts (ABC) Codes",
            WaferLot => "Wafer Lot Identifier",
            YarnCountWorsted => "Yarn Count Worsted",
            WheelChairLiftSerialNumber => "Wheel Chair Lift Serial Number",
            PreferredPartNumber => "Preferred Part Number",
            ExpendableContainerIdentification => "Expendable Container Identification",
            CodeXN => "Export Control Classification Number (ECCN)",
            PreferredNationalStockNumber => "Preferred National Stock Number",
            PreferredManufacturer => "Preferred Manufacturer",
            ContractorEstablishmentCode => "Contractor Establishment Code",
            ChildGtin13 => "Child GTIN-13",
            ChildGtin14 => "Child GTIN-14",
            ChildGtin12 => "Child GTIN-12",
            MapEditionNumber => "Map Edition Number",
            PublicationNumber => "Publication Number",
            CodeZB => "Commercial and Government Entity (CAGE) Code",
            ServiceControlIdentification => "Service Control Identification",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<ProductServiceIdQualifier> {
        {
            use ProductServiceIdQualifier::*;
            match description {
                "Credit Card" => Some(CreditCard),
                "Plant Equipment Number" => Some(PlantEquipmentNumber),
                "Department of Defense Identification Code (DoDIC)" => Some(CodeA2),
                "Locally Assigned Control Number" => Some(LocallyAssignedControlNumber),
                "Subsistence Identification Number" => {
                    Some(SubsistenceIdentificationNumber)
                }
                "Application State or Province" => Some(ApplicationStateOrProvince),
                "Document Identification Code" => Some(DocumentIdentificationCode),
                "Subline Item Number" => Some(SublineItemNumber),
                "Exhibit Line Item Number" => Some(ExhibitLineItemNumber),
                "Activity" => Some(Activity),
                "Assembly Level Code, Vehicle Maintenance Reporting Standards (VMRS) 2000 Code Key 32" => {
                    Some(CodeAA)
                }
                "Assembly" => Some(Assembly),
                "Aggregation Code (Used to Consolidate Part Families)" => Some(CodeAC),
                "American Dental Association Codes" => {
                    Some(AmericanDentalAssociationCodes)
                }
                "Serial Item and Contribution Identifier (Defined in ANSI 239.56)" => {
                    Some(CodeAE)
                }
                "Front Axle Serial Number" => Some(FrontAxleSerialNumber),
                "Age" => Some(Age),
                "Billboards Commercial" => Some(BillboardsCommercial),
                "Alternate ISBN" => Some(AlternateIsbn),
                "Piggyback Commercial" => Some(PiggybackCommercial),
                "Refined Product Code" => Some(RefinedProductCode),
                "Axle Serial Number" => Some(AxleSerialNumber),
                "Allocation Methodology Identification Code" => {
                    Some(AllocationMethodologyIdentificationCode)
                }
                "Asset Number" => Some(AssetNumber),
                "Assemblage Identification Number" => {
                    Some(AssemblageIdentificationNumber)
                }
                "Component Level Code, Vehicle Maintenance Reporting Standards (VMRS) 2000 Code Key 33" => {
                    Some(CodeAP)
                }
                "Appraisal Product Type" => Some(AppraisalProductType),
                "ARINC Part Number" => Some(ArincPartNumber),
                "Rear Axle Serial Number" => Some(RearAxleSerialNumber),
                "Assortability Code" => Some(AssortabilityCode),
                "Appraisal Service" => Some(AppraisalService),
                "Additional Product Identification Assigned by the Manufacturer" => {
                    Some(AdditionalProductIdentificationAssignedByTheManufacturer)
                }
                "Well Number" => Some(WellNumber),
                "Assembly Lot Number" => Some(AssemblyLotNumber),
                "System Level Code, Vehicle Maintenance Reporting Standards (VMRS) 2000 Code Key 31" => {
                    Some(CodeAY)
                }
                "Axle Ratio" => Some(AxleRatio),
                "Volume Set" => Some(VolumeSet),
                "Library of Congress Code" => Some(LibraryOfCongressCode),
                "Edition Code" => Some(EditionCode),
                "Binding Code" => Some(BindingCode),
                "Title Code" => Some(TitleCode),
                "Author Code" => Some(AuthorCode),
                "Publisher Code" => Some(PublisherCode),
                "Batch Number" => Some(BatchNumber),
                "Front Axle Driven Serial Number" => Some(FrontAxleDrivenSerialNumber),
                "Bale Number" => Some(BaleNumber),
                "Print Color Standard Reference" => Some(PrintColorStandardReference),
                "Supplier Brand Code" => Some(SupplierBrandCode),
                "Buyer Ground Shade Identifier" => Some(BuyerGroundShade),
                "Buyer's Engineering Change Level Number" => {
                    Some(BuyersEngineeringChangeLevelNumber)
                }
                "Vendor Ground Shade Identifier" => Some(VendorGroundShade),
                "Ground Shade Standard Reference" => Some(GroundShadeStandardReference),
                "Finish/Hand Standard Reference" => Some(FinishHandStandardReference),
                "Rear Axle Rear Non Driven Serial Number" => {
                    Some(RearAxleRearNonDrivenSerialNumber)
                }
                "Application Completion Method" => Some(ApplicationCompletionMethod),
                "Backing" => Some(Backing),
                "Brand/Label" => Some(BrandLabel),
                "Beam Number" => Some(BeamNumber),
                "Bar-Coded Serial Number" => Some(BarCodedSerialNumber),
                "Buyers Color" => Some(BuyersColor),
                "Buyer's Part Number" => Some(BuyersPartNumber),
                "Benefit ID" => Some(BenefitId),
                "Compensation Type Code" => Some(CompensationTypeCode),
                "Bill Subgroup Code" => Some(BillSubgroupCode),
                "Volume Type Identification Code" => Some(VolumeTypeIdentificationCode),
                "Bus Body Serial Number" => Some(BusBodySerialNumber),
                "Beverage Common Code" => Some(BeverageCommonCode),
                "Compensation Allocation Code" => Some(CompensationAllocationCode),
                "Bill Credit Code" => Some(BillCreditCode),
                "Concept Code" => Some(ConceptCode),
                "Coverage Type" => Some(CoverageType),
                "Channel" => Some(Channel),
                "Connector Type" => Some(ConnectorType),
                "Classification" => Some(Classification),
                "Configuration Item Identifier" => Some(ConfigurationItem),
                "Insurance Plan Description Characteristics" => {
                    Some(InsurancePlanDescriptionCharacteristics)
                }
                "Asset Type" => Some(AssetType),
                "Asset Category" => Some(AssetCategory),
                "Fund Sub-Advisor" => Some(FundSubAdvisor),
                "Dun & Bradstreet Standard Product and Service Code" => Some(CodeC9),
                "Case" => Some(Case),
                "Buyer's Catalog Number" => Some(BuyersCatalogNumber),
                "Compatible Cut Number" => Some(CompatibleCutNumber),
                "Motor Vehicle-Line Designator" => Some(MotorVehicleLineDesignator),
                "Class of Contract Code" => Some(ClassOfContractCode),
                "Chassis Serial Number" => Some(ChassisSerialNumber),
                "Commodity Grouping" => Some(CommodityGrouping),
                "Country of Origin Code" => Some(CountryOfOriginCode),
                "Common Language Equipment Identifier (CLEI)" => Some(CodeCI),
                "Current Procedural Terminology (CPT) Codes" => Some(CodeCJ),
                "Candidate Oil" => Some(CandidateOil),
                "Color" => Some(Color),
                "National Retail Federation Color Code" => {
                    Some(NationalRetailFederationColorCode)
                }
                "Commodity Name" => Some(CommodityName),
                "Chemical Abstract Service (CAS) Registry Number" => Some(CodeCO),
                "Carry-over Part Number" => Some(CarryOverPartNumber),
                "Equipment Code per COPAS standard" => {
                    Some(EquipmentCodePerCopasStandard)
                }
                "Contract Number" => Some(ContractNumber),
                "Service code per COPAS standard" => Some(ServiceCodePerCopasStandard),
                "Continuation (ID Number Spans Multiple Product ID Data Elements)" => {
                    Some(CodeCT)
                }
                "Cut Number" => Some(CutNumber),
                "Customer Provided Equipment" => Some(CustomerProvidedEquipment),
                "Contract Activity Code" => Some(ContractActivityCode),
                "Completion Number" => Some(CompletionNumber),
                "Customer Company Registry Number" => Some(CustomerCompanyRegistryNumber),
                "Country from which Procured" => Some(CountryFromWhichProcured),
                "Underwriting Method of Direct Writer" => {
                    Some(UnderwritingMethodOfDirectWriter)
                }
                "Medical Information Bureau (MIB) Authorization" => Some(CodeD2),
                "Policy Form" => Some(PolicyForm),
                "Plan Code" => Some(PlanCode),
                "Coverage Risk Type" => Some(CoverageRiskType),
                "Medical Stop Loss Level Codes" => Some(MedicalStopLossLevelCodes),
                "Medical Stop Loss Coverage Codes" => Some(MedicalStopLossCoverageCodes),
                "Medical Stop Loss Product Line Codes" => {
                    Some(MedicalStopLossProductLineCodes)
                }
                "RxNorm" => Some(RxNorm),
                "Automobile Repair Product Code" => Some(AutomobileRepairProductCode),
                "GS1 DataBar" => Some(Gs1DataBar),
                "International Classification of Diseases, 10th Revision, Clinical Modification (ICD-10-CM)" => {
                    Some(CodeDC)
                }
                "Distributor" => Some(Distributor),
                "Design Number" => Some(DesignNumber),
                "Device Family" => Some(DeviceFamily),
                "Discount Grouping" => Some(DiscountGrouping),
                "Transport4 Commodity Code" => Some(Transport4CommodityCode),
                "Deposit Item Number" => Some(DepositItemNumber),
                "Dependent Proprietary Product" => Some(DependentProprietaryProduct),
                "Primary Alternate Product" => Some(PrimaryAlternateProduct),
                "Dye Lot Number" => Some(DyeLotNumber),
                "Committee for Uniform Security Identification Procedure Number (CUSIP) Number" => {
                    Some(CodeDM)
                }
                "Die Number" => Some(DieNumber),
                "Dividend Use" => Some(DividendUse),
                "Discontinued Part Number" => Some(DiscontinuedPartNumber),
                "Event Type" => Some(EventType),
                "Drawing Revision Number" => Some(DrawingRevisionNumber),
                "Group ID" => Some(GroupId),
                "Device Type" => Some(DeviceType),
                "Joint Life Type" => Some(JointLifeType),
                "Location Code" => Some(LocationCode),
                "Nonforfeiture Option" => Some(NonforfeitureOption),
                "International Classification of Diseases, 9th Revision, Clinical Modification (ICD-9-CM) - Diagnosis" => {
                    Some(CodeDX)
                }
                "Premium Rate Type" => Some(PremiumRateType),
                "Diagnosis Code Pointer" => Some(DiagnosisCodePointer),
                "Contract Change Authorization Type" => {
                    Some(ContractChangeAuthorizationType)
                }
                "Fund Abbreviation" => Some(FundAbbreviation),
                "Fund Type" => Some(FundType),
                "Related Policy Identification" => Some(RelatedPolicyIdentification),
                "Tax Code" => Some(TaxCode),
                "Contract Transfer Reason Code" => Some(ContractTransferReasonCode),
                "Berenson-Eggers Type of Service" => Some(BerensonEggersTypeOfService),
                "International Society of Blood Transfusion device identifier" => {
                    Some(InternationalSocietyOfBloodTransfusionDeviceIdentifier)
                }
                "Human Cell, Tissue or Cellular or Tissue-Based Product code" => {
                    Some(CodeE9)
                }
                "EAN-99 In-store Coupon Code" => Some(Ean99InStoreCouponCode),
                "Fuel Tank Serial Number" => Some(FuelTankSerialNumber),
                "Engineering Change Level" => Some(EngineeringChangeLevel),
                "Engine Displacement Identification" => {
                    Some(EngineDisplacementIdentification)
                }
                "Premium Use" => Some(PremiumUse),
                "Exhibit Identifier" => Some(Exhibit),
                "Purpose of Insurance" => Some(PurposeOfInsurance),
                "Sales Presentation ID" => Some(SalesPresentationId),
                "Expense Identifier" => Some(Expense),
                "Service Feature ID" => Some(ServiceFeatureId),
                "Settlement/Payout Option" => Some(SettlementPayoutOption),
                "Buyer's Subline Item Number" => Some(BuyersSublineItemNumber),
                "Equipment Identification Number" => Some(EquipmentIdentificationNumber),
                "GTIN-13" => Some(Gtin13),
                "GTIN-8" => Some(Gtin8),
                "Buyer's End Product Number" => Some(BuyersEndProductNumber),
                "Equipment Type" => Some(EquipmentType),
                "Jurisdiction Specific Procedure and Supply Codes" => {
                    Some(JurisdictionSpecificProcedureAndSupplyCodes)
                }
                "Engine Serial Number" => Some(EngineSerialNumber),
                "Department of Defense Enterprise Identifier" => {
                    Some(DepartmentOfDefenseEnterprise)
                }
                "Skill Code" => Some(SkillCode),
                "Equivalent Product" => Some(EquivalentProduct),
                "Exchanged Part, Assembly or Product" => Some(CodeEX),
                "Shift Worked" => Some(ShiftWorked),
                "Catalog Number" => Some(CatalogNumber),
                "Technical Order Number" => Some(TechnicalOrderNumber),
                "Technical Manual Number" => Some(TechnicalManualNumber),
                "Series Identifier" => Some(Series),
                "Obligation Authority Number" => Some(ObligationAuthorityNumber),
                "First Prior Identifier" => Some(FirstPrior),
                "End-Item Description" => Some(EndItemDescription),
                "Next Higher Used Assembly" => Some(NextHigherUsedAssembly),
                "Former Publisher" => Some(FormerPublisher),
                "Failed Subassembly Serial Number" => Some(FailedSubassemblySerialNumber),
                "Form Number" => Some(FormNumber),
                "Coupon Family Code" => Some(CouponFamilyCode),
                "Fund" => Some(Fund),
                "Feature" => Some(Feature),
                "Fifth Wheel Serial Number" => Some(FifthWheelSerialNumber),
                "Fund Manager" => Some(FundManager),
                "Freddie Mac Affordable Lending Product Code" => {
                    Some(FreddieMacAffordableLendingProductCode)
                }
                "Finish Number" => Some(FinishNumber),
                "Fannie Mae Affordable Lending Product Code" => {
                    Some(FannieMaeAffordableLendingProductCode)
                }
                "Freddie Mac Project Condominium Classification Code" => {
                    Some(FreddieMacProjectCondominiumClassificationCode)
                }
                "Finish Lot Number" => Some(FinishLotNumber),
                "Failed Subassembly Model Number" => Some(FailedSubassemblyModelNumber),
                "Final Test Lot Number" => Some(FinalTestLotNumber),
                "Fabric Pieces Per Roll" => Some(FabricPiecesPerRoll),
                "Fannie Mae Project Condominium Classification Code" => {
                    Some(FannieMaeProjectCondominiumClassificationCode)
                }
                "Front Axle, Rear" => Some(CodeFR),
                "National Stock Number" => Some(NationalStockNumber),
                "Federal Supply Classification" => Some(FederalSupplyClassification),
                "National Alcohol Beverage Control Association (NABCA) Product Code" => {
                    Some(CodeFU)
                }
                "Drug Identification Number (DIN)" => Some(CodeFV),
                "New Microcode" => Some(NewMicrocode),
                "Federal Supply Group" => Some(FederalSupplyGroup),
                "Gathering" => Some(Gathering),
                "Grade Code" => Some(GradeCode),
                "Grain Direction" => Some(GrainDirection),
                "Generic Name Description" => Some(GenericNameDescription),
                "Graphics Industry Bar Code (GIBC)" => Some(CodeGI),
                "Glider Kit" => Some(GliderKit),
                "General Services Administration (GSA) Special Item Number" => {
                    Some(CodeGM)
                }
                "Grade Name" => Some(GradeName),
                "Group Qualifier Code" => Some(GroupQualifierCode),
                "Gear Ratio" => Some(GearRatio),
                "General Specification Number" => Some(GeneralSpecificationNumber),
                "Volume Usage Identification Code" => Some(VolumeUsageIdentificationCode),
                "Serialized Global Returnable Asset Identifier (GRAI)" => Some(CodeGV),
                "Global Returnable Asset Identifier (GRAI)" => Some(CodeGW),
                "Global Individual Asset Identifier (GIAI)" => Some(CodeGX),
                "Country Subdivision Code" => Some(CountrySubdivisionCode),
                "Country of Origin with Country Subdivision Code" => {
                    Some(CountryOfOriginWithCountrySubdivisionCode)
                }
                "Healthcare Common Procedure Coding System (HCPCS) Codes" => Some(CodeHC),
                "International Harmonized Commodity Code" => {
                    Some(InternationalHarmonizedCommodityCode)
                }
                "Country Subdivision from where Procured" => {
                    Some(CountrySubdivisionFromWhereProcured)
                }
                "HIBC (Health Care Industry Bar Code) Supplier Labeling Standard Primary Data Message" => {
                    Some(CodeHI)
                }
                "Province or State of Product Processing Code" => {
                    Some(ProvinceOrStateOfProductProcessingCode)
                }
                "Province or State of Product Packaged" => {
                    Some(ProvinceOrStateOfProductPackaged)
                }
                "Heat Number" => Some(HeatNumber),
                "Health Insurance Prospective Payment System (HIPPS) Rate Code" => {
                    Some(CodeHP)
                }
                "Alcoholic Beverage Subregion" => Some(AlcoholicBeverageSubregion),
                "Information Media Type" => Some(InformationMediaType),
                "International Standard Book Number (ISBN)" => Some(CodeIB),
                "Interior Color Number" => Some(InteriorColorNumber),
                "International Classification of Diseases, 9th Revision, Clinical Modification (ICD-9-CM) - Procedure" => {
                    Some(CodeID)
                }
                "Insurer's Fund Code" => Some(InsurersFundCode),
                "Investment Fund Type" => Some(InvestmentFundType),
                "Ignition Key Number" => Some(IgnitionKeyNumber),
                "International Classification of Diseases, 11th Revision,Clinical Modification (ICD-11-CM)" => {
                    Some(CodeIH)
                }
                "Commodity Item Identification" => Some(CommodityItemIdentification),
                "International Classification of Diseases, 11th Revision, Procedure Coding System (ICD-11-PCS)" => {
                    Some(CodeIJ)
                }
                "Imprint (Trademark Code of Subsidiary)" => Some(CodeIM),
                "Buyer's Item Number" => Some(BuyersItemNumber),
                "International Classification of Diseases, 10th Revision, Procedure Coding System (ICD-10-PCS)" => {
                    Some(CodeIP)
                }
                "IRS Qualification Code" => Some(IrsQualificationCode),
                "Ingredient" => Some(Ingredient),
                "International Standard Serial Number (ISSN)" => Some(CodeIS),
                "Buyer's Style Number" => Some(BuyersStyleNumber),
                "Department of Defense Issuing Agency Code" => {
                    Some(DepartmentOfDefenseIssuingAgencyCode)
                }
                "Interchangeability Code" => Some(InterchangeabilityCode),
                "Buyer's Size Code" => Some(BuyersSizeCode),
                "Anniversary" => Some(Anniversary),
                "Commission Identifier" => Some(Commission),
                "Commission Year" => Some(CommissionYear),
                "Contribution Year" => Some(ContributionYear),
                "Job Number" => Some(JobNumber),
                "Package Type Code" => Some(PackageTypeCode),
                "Job Sequence Number" => Some(JobSequenceNumber),
                "Engineering Data List" => Some(EngineeringDataList),
                "Data Category Code" => Some(DataCategoryCode),
                "Replacement National Stock Number" => {
                    Some(ReplacementNationalStockNumber)
                }
                "Military Standard" => Some(MilitaryStandard),
                "Item Type Number" => Some(ItemTypeNumber),
                "Time Compliant Technical Order" => Some(TimeCompliantTechnicalOrder),
                "Cognizance Symbol" => Some(CognizanceSymbol),
                "Material Control Code" => Some(MaterialControlCode),
                "Special Material Identification Code" => {
                    Some(SpecialMaterialIdentificationCode)
                }
                "Item Management Code" => Some(ItemManagementCode),
                "Shelf-Life Code" => Some(ShelfLifeCode),
                "Shelf-Life Action Code" => Some(ShelfLifeActionCode),
                "Kanban Plan Number" => Some(KanbanPlanNumber),
                "Program Level" => Some(ProgramLevel),
                "Topic Level" => Some(TopicLevel),
                "Subtopic Level" => Some(SubtopicLevel),
                "Life/Annuity Service Features" => Some(LifeAnnuityServiceFeatures),
                "Line of Authority" => Some(LineOfAuthority),
                "Lube, Synthetic" => Some(CodeL6),
                "Labor Group" => Some(LaborGroup),
                "Logical Observation Identifier Names and Codes (LOINC) Codes" => {
                    Some(CodeLB)
                }
                "Laboratory Test Condition Code" => Some(LaboratoryTestConditionCode),
                "SNOMED, Systematized Nomenclature of Medicine" => Some(CodeLD),
                "Lift Gate Serial Number" => Some(LiftGateSerialNumber),
                "Lottery Game Number" => Some(LotteryGameNumber),
                "Lottery Pack/Book Number" => Some(LotteryPackBookNumber),
                "Life/Annuity Product Code" => Some(LifeAnnuityProductCode),
                "Lease Number" => Some(LeaseNumber),
                "Load Sequence" => Some(LoadSequence),
                "Lot Number" => Some(LotNumber),
                "Lot Pricing Unit Number" => Some(LotPricingUnitNumber),
                "Machine Number" => Some(MachineNumber),
                "Measurement Type Code" => Some(MeasurementTypeCode),
                "Mortgage Credit Data Order Type" => Some(MortgageCreditDataOrderType),
                "Method of Delivery Code" => Some(MethodOfDeliveryCode),
                "Market Program Code" => Some(MarketProgramCode),
                "Manufacturer" => Some(Manufacturer),
                "Manufacturer's Part Number" => Some(ManufacturersPartNumber),
                "Medication Code" => Some(MedicationCode),
                "Mortgage Insurance Product Code or Number" => {
                    Some(MortgageInsuranceProductCodeOrNumber)
                }
                "Manual Transmission Serial Number" => {
                    Some(ManualTransmissionSerialNumber)
                }
                "Front Axle Non Driven Serial Number" => {
                    Some(FrontAxleNonDrivenSerialNumber)
                }
                "Motor Equipment Manufacturing Association (MEMA) Product Type Code" => {
                    Some(CodeMM)
                }
                "Model Number" => Some(ModelNumber),
                "Movement Type Code" => Some(MovementTypeCode),
                "Mortgage Product Code" => Some(MortgageProductCode),
                "Mortgage Underwriting Type" => Some(MortgageUnderwritingType),
                "Maintenance Index Page Reference Number" => {
                    Some(MaintenanceIndexPageReferenceNumber)
                }
                "Military Specification (MILSPEC) Number" => Some(CodeMS),
                "Major Product/Material/Machine Type" => {
                    Some(MajorProductMaterialMachineType)
                }
                "Authorized Parts List Number" => Some(AuthorizedPartsListNumber),
                "Equipment Location" => Some(EquipmentLocation),
                "Equipment Hierarchical Sequence Identifier" => {
                    Some(EquipmentHierarchicalSequence)
                }
                "Repair Induction Identifier" => Some(RepairInduction),
                "National Drug Code in 4-4-2 Format" => Some(NationalDrugCodeIn442Format),
                "National Drug Code in 5-3-2 Format" => Some(NationalDrugCodeIn532Format),
                "National Drug Code in 5-4-1 Format" => Some(NationalDrugCodeIn541Format),
                "National Drug Code in 5-4-2 Format" => Some(NationalDrugCodeIn542Format),
                "National Health Related Item Code in 5-5 Format" => {
                    Some(NationalHealthRelatedItemCodeIn55Format)
                }
                "National Health Related Item Code in 4-6 Format" => {
                    Some(NationalHealthRelatedItemCodeIn46Format)
                }
                "National Drug Code (NDC)" => Some(CodeND),
                "Yarn Count - English" => Some(YarnCountEnglish),
                "National Glass Association (NAGS) Number" => Some(CodeNG),
                "National Health Related Item Code" => {
                    Some(NationalHealthRelatedItemCode)
                }
                "Yarn Count - Metric" => Some(YarnCountMetric),
                "National Item Identification Number" => {
                    Some(NationalItemIdentificationNumber)
                }
                "Natural Health Product Number" => Some(NaturalHealthProductNumber),
                "Non-resaleable item (excluding deposit) number" => Some(CodeNR),
                "National Uniform Billing Committee (NUBC) UB92 Codes" => Some(CodeNU),
                "New Replacement Part or Assembly Defective" => {
                    Some(NewReplacementPartOrAssemblyDefective)
                }
                "Combined NCCMA/Bank Service Code" => Some(CombinedNccmaBankServiceCode),
                "Old Vendor's (Seller's) Item Number" => Some(CodeO9),
                "Original Equipment Number" => Some(OriginalEquipmentNumber),
                "Old Common Language Equipment Identifier (CLEI) Code" => Some(CodeOF),
                "Old Microcode" => Some(OldMicrocode),
                "Opposite-Hand Part Number" => Some(OppositeHandPartNumber),
                "Optical Industry Product Code" => Some(OpticalIndustryProductCode),
                "Optical Cable Code" => Some(OpticalCableCode),
                "Original Part Number" => Some(OriginalPartNumber),
                "Customer Order Number" => Some(CustomerOrderNumber),
                "Outside Production Operation Sheet Number" => {
                    Some(OutsideProductionOperationSheetNumber)
                }
                "Obsolete Part Number" => Some(ObsoletePartNumber),
                "Offer Number" => Some(OfferNumber),
                "Internal Number" => Some(InternalNumber),
                "Original Unit of Issue" => Some(OriginalUnitOfIssue),
                "Petroleum Accountants Society of Canada Operating and Maintenance Code - Goods and Services Tax Not Applicable" => {
                    Some(
                        PetroleumAccountantsSocietyOfCanadaOperatingAndMaintenanceCodeGoodsAndServicesTaxNotApplicable,
                    )
                }
                "Petroleum Accountants Society of Canada Capital Expenditure Code - Goods and Services Tax Not Applicable" => {
                    Some(
                        PetroleumAccountantsSocietyOfCanadaCapitalExpenditureCodeGoodsAndServicesTaxNotApplicable,
                    )
                }
                "Petroleum Accountants Society of Canada Tubular Code" => {
                    Some(PetroleumAccountantsSocietyOfCanadaTubularCode)
                }
                "Petroleum Accountants Society of Canada Non-Tubular Code" => {
                    Some(PetroleumAccountantsSocietyOfCanadaNonTubularCode)
                }
                "Material Discharge Number" => Some(MaterialDischargeNumber),
                "Pump, Fire" => Some(CodeP6),
                "Previous Carrier" => Some(PreviousCarrier),
                "Retail Price Look Up Number (PLU)" => Some(CodeP8),
                "Ply" => Some(Ply),
                "Pattern Number" => Some(PatternNumber),
                "Petroleum Accountants Society of Canada Operating and Maintenance Code - Goods and Services Tax Forwarded" => {
                    Some(
                        PetroleumAccountantsSocietyOfCanadaOperatingAndMaintenanceCodeGoodsAndServicesTaxForwarded,
                    )
                }
                "Prime Contractor Part Number" => Some(PrimeContractorPartNumber),
                "Part Number Description" => Some(PartNumberDescription),
                "Pieces in Roll" => Some(PiecesInRoll),
                "Petroleum Accountants Society of Canada Capital Expenditure Code - Goods and Services Tax Forwarded" => {
                    Some(
                        PetroleumAccountantsSocietyOfCanadaCapitalExpenditureCodeGoodsAndServicesTaxForwarded,
                    )
                }
                "Packaging Specification Number" => Some(PackagingSpecificationNumber),
                "Property and Casualty Service Code" => {
                    Some(PropertyAndCasualtyServiceCode)
                }
                "Purchaser's Item Code" => Some(PurchasersItemCode),
                "Product Date Code (A code indicating the period during which a product was manufactured.)" => {
                    Some(CodePJ)
                }
                "Packaging Drawing" => Some(PackagingDrawing),
                "Purchaser's Order Line Number" => Some(PurchasersOrderLineNumber),
                "Number of Positions on Machine" => Some(NumberOfPositionsOnMachine),
                "Company Part Number" => Some(CompanyPartNumber),
                "Purchase Order Number" => Some(PurchaseOrderNumber),
                "Air Transportation Association Proprietary Rights Code" => {
                    Some(AirTransportationAssociationProprietaryRightsCode)
                }
                "Product ID Attribute Code" => Some(ProductIdAttributeCode),
                "Process Number" => Some(ProcessNumber),
                "Position" => Some(Position),
                "Print or Drawing" => Some(PrintOrDrawing),
                "Part Reference Number" => Some(PartReferenceNumber),
                "Advertising Package Identification Code" => {
                    Some(AdvertisingPackageIdentificationCode)
                }
                "Part Drawing" => Some(PartDrawing),
                "Secondary Ply" => Some(SecondaryPly),
                "Operator Assigned Property Identification" => {
                    Some(OperatorAssignedPropertyIdentification)
                }
                "Product Change Notice Number" => Some(ProductChangeNoticeNumber),
                "Replacement Subassembly Model Number" => {
                    Some(ReplacementSubassemblyModelNumber)
                }
                "Replacement Subassembly Serial Number" => {
                    Some(ReplacementSubassemblySerialNumber)
                }
                "Rear Axle, Middle" => Some(CodeR3),
                "Rear Axle, Pusher" => Some(CodeR4),
                "Rear Axle, Tag" => Some(CodeR5),
                "Rear Axle, Extended Tag" => Some(CodeR6),
                "Replacement Vendor's (Seller's) Item Number" => Some(CodeR9),
                "Return Code" => Some(ReturnCode),
                "National Uniform Billing Committee (NUBC) UB82 Codes" => Some(CodeRB),
                "Returnable Container Number" => Some(ReturnableContainerNumber),
                "Reel Number" => Some(ReelNumber),
                "Reefer Serial Number" => Some(ReeferSerialNumber),
                "Repair From Product Code" => Some(RepairFromProductCode),
                "Reference Oil" => Some(ReferenceOil),
                "Radiator Serial Number" => Some(RadiatorSerialNumber),
                "Rear Axle Front Rear Driven Serial Number" => {
                    Some(RearAxleFrontRearDrivenSerialNumber)
                }
                "Rear Axle Rear Driven Serial Number" => {
                    Some(RearAxleRearDrivenSerialNumber)
                }
                "Rack Number" => Some(RackNumber),
                "Rate Detail Card" => Some(RateDetailCard),
                "Related Model Number" => Some(RelatedModelNumber),
                "Release Number" => Some(ReleaseNumber),
                "Roll Number" => Some(RollNumber),
                "Replaced Part Number" => Some(ReplacedPartNumber),
                "Automobile Rental Charge Item Code" => {
                    Some(AutomobileRentalChargeItemCode)
                }
                "Replacement Product Number" => Some(ReplacementProductNumber),
                "Set Number" => Some(SetNumber),
                "Reel Type" => Some(ReelType),
                "Run Number" => Some(RunNumber),
                "Repair Tag Number" => Some(RepairTagNumber),
                "Relative Value Units" => Some(RelativeValueUnits),
                "Provisioning Reference Number" => Some(ProvisioningReferenceNumber),
                "Record Keeping or Model Year" => Some(RecordKeepingOrModelYear),
                "Related Model Type" => Some(RelatedModelType),
                "Shipper's Item Number" => Some(ShippersItemNumber),
                "Second Prior Identifier" => Some(SecondPrior),
                "Phase" => Some(Phase),
                "Laboratory Sample Identification" => {
                    Some(LaboratorySampleIdentification)
                }
                "State Sample Identification" => Some(StateSampleIdentification),
                "Previous Sample Identification" => Some(PreviousSampleIdentification),
                "Source of Deposit Code" => Some(SourceOfDepositCode),
                "Source of Lead Code" => Some(SourceOfLeadCode),
                "Schematic Diagram Reference Number" => {
                    Some(SchematicDiagramReferenceNumber)
                }
                "Submission Number" => Some(SubmissionNumber),
                "Seller's Date Code" => Some(SellersDateCode),
                "Supplier Company Registry Number" => Some(SupplierCompanyRegistryNumber),
                "Section Print Number" => Some(SectionPrintNumber),
                "Surface Finish" => Some(SurfaceFinish),
                "Seat Serial Number" => Some(SeatSerialNumber),
                "Service Requested" => Some(ServiceRequested),
                "Standard Industrial Classification Code" => {
                    Some(StandardIndustrialClassificationCode)
                }
                "Religious Retail Non-book Item" => Some(ReligiousRetailNonBookItem),
                "Stock Keeping Unit (SKU)" => Some(CodeSK),
                "Seller's Lot Number" => Some(SellersLotNumber),
                "National Retail Federation Size Code" => {
                    Some(NationalRetailFederationSizeCode)
                }
                "Serial Number" => Some(SerialNumber),
                "System Identifier" => Some(System),
                "Superseded Purchase Order Number" => Some(SupersededPurchaseOrderNumber),
                "Roll Sequence Number" => Some(RollSequenceNumber),
                "Substitute Product Number" => Some(SubstituteProductNumber),
                "Superseded Part Number" => Some(SupersededPartNumber),
                "Style Number" => Some(StyleNumber),
                "Side Up/Side Down" => Some(SideUpSideDown),
                "Service Rendered" => Some(ServiceRendered),
                "Stock Number" => Some(StockNumber),
                "Sleeper Box Key Number" => Some(SleeperBoxKeyNumber),
                "Sleeper Box Serial Number" => Some(SleeperBoxSerialNumber),
                "Vendor Alphanumeric Size Code" => Some(VendorAlphanumericSizeCode),
                "Tex" => Some(Tex),
                "Third Prior Identifier" => Some(ThirdPrior),
                "Substitute GTIN-8" => Some(SubstituteGtin8),
                "Substitute GTIN-12" => Some(SubstituteGtin12),
                "Substitute GTIN-13" => Some(SubstituteGtin13),
                "Substitute GTIN-14" => Some(SubstituteGtin14),
                "Pipeline Transaction Code" => Some(PipelineTransactionCode),
                "Association for Financial Professionals Service Code and Bank Service Code" => {
                    Some(
                        AssociationForFinancialProfessionalsServiceCodeAndBankServiceCode,
                    )
                }
                "Telecommunications Circuit ID" => Some(TelecommunicationsCircuitId),
                "Treatment Codes" => Some(TreatmentCodes),
                "Association for Financial Professionals Service Code" => {
                    Some(AssociationForFinancialProfessionalsServiceCode)
                }
                "The Air Cargo Tariff (TACT) Commodity Code" => Some(CodeTF),
                "Automatic Transmission Serial Number" => {
                    Some(AutomaticTransmissionSerialNumber)
                }
                "Transfer Case Serial Number" => Some(TransferCaseSerialNumber),
                "Trade In Identifier" => Some(TradeIn),
                "Auxiliary Transmission Serial Number" => {
                    Some(AuxiliaryTransmissionSerialNumber)
                }
                "Transmission, Auxillary" => Some(CodeTK),
                "Telephone Industry Manufacturer Code" => {
                    Some(TelephoneIndustryManufacturerCode)
                }
                "Railroad-Owned Unit Train Number" => Some(RailroadOwnedUnitTrainNumber),
                "Product Type Code" => Some(ProductTypeCode),
                "Truck Body Serial Number" => Some(TruckBodySerialNumber),
                "Transmission Serial Number" => Some(TransmissionSerialNumber),
                "Type Selvage" => Some(TypeSelvage),
                "Tested Material Identification Number" => {
                    Some(TestedMaterialIdentificationNumber)
                }
                "Line of Business" => Some(LineOfBusiness),
                "Program Code" => Some(ProgramCode),
                "Federal Aviation Administration (FAA) Service Bulletin Number" => {
                    Some(CodeTX)
                }
                "Telecommunications Industry Service Code" => {
                    Some(TelecommunicationsIndustryServiceCode)
                }
                "Program Description Identifier" => Some(ProgramDescription),
                "United Nations Common Coding System (UNCCS)" => Some(CodeU3),
                "Broker Price Opinion Service" => Some(BrokerPriceOpinionService),
                "Real Estate Property Information Service" => {
                    Some(RealEstatePropertyInformationService)
                }
                "Department of Defense Unique Item Identifier" => {
                    Some(DepartmentOfDefenseUniqueItem)
                }
                "Product Variant" => Some(ProductVariant),
                "User-Defined Shipping Container Identifier" => {
                    Some(UserDefinedShippingContainer)
                }
                "U.P.C./EAN Coupon Code (2-5-5)" => Some(CodeUJ),
                "GTIN-14" => Some(Gtin14),
                "U.P.C. Coupon Code (1-5-5-1)" => Some(CodeUL),
                "Universal Vendor Marking, Short Code (UVM; U Line)" => Some(CodeUM),
                "EAN.UCC Serial Shipping Container Code (SSCC)" => Some(CodeUO),
                "GTIN-12" => Some(Gtin12),
                "United Nations (UN) Number (Dangerous Goods)" => Some(CodeUQ),
                "UCC/EAN-128 Coupon Extended Code" => Some(UccEan128CouponExtendedCode),
                "Uniform Stock Symbol System Code Number" => {
                    Some(UniformStockSymbolSystemCodeNumber)
                }
                "Company-Owned Unit Train Number" => Some(CompanyOwnedUnitTrainNumber),
                "Device Identifier" => Some(Device),
                "Universal Vendor Marking, Long Code (UVM; R,P,M Lines)" => Some(CodeUV),
                "Universal Product Number" => Some(UniversalProductNumber),
                "Department of Defense Unique Item Identifier Type Reference Identifier" => {
                    Some(DepartmentOfDefenseUniqueItemTypeReference)
                }
                "GS1 US Coupon Code with GS1-128 Coupon Extended Code" => {
                    Some(Gs1UsCouponCodeWithGs1128CouponExtendedCode)
                }
                "Ingredient Country of Origin Code" => {
                    Some(IngredientCountryOfOriginCode)
                }
                "Tariff Country of Origin Code" => Some(TariffCountryOfOriginCode),
                "Country of Last Processing Code" => Some(CountryOfLastProcessingCode),
                "Country of Assembly Code" => Some(CountryOfAssemblyCode),
                "Logistics Country of Origin Code" => Some(LogisticsCountryOfOriginCode),
                "United States Department of Agriculture (USDA) Country of Origin Code" => {
                    Some(CodeV6)
                }
                "Consumer Product Variant Identifier" => Some(ConsumerProductVariant),
                "Vendor's Style Number" => Some(VendorsStyleNumber),
                "Vendor's Engineering Change Level Number" => {
                    Some(VendorsEngineeringChangeLevelNumber)
                }
                "Vendor's (Seller's) Catalog Number" => Some(CodeVC),
                "Vendor Color" => Some(VendorColor),
                "Vary Item Product Number" => Some(VaryItemProductNumber),
                "Vehicle Maintenance Reporting Standards" => {
                    Some(VehicleMaintenanceReportingStandards)
                }
                "Vendor's (Seller's) Item Number" => Some(CodeVN),
                "Vendor's Order Number" => Some(VendorsOrderNumber),
                "Vendor's (Seller's) Part Number" => Some(CodeVP),
                "Vendor's Supplemental Item Number" => {
                    Some(VendorsSupplementalItemNumber)
                }
                "Vintage" => Some(Vintage),
                "Vendor's Basic Unit Number" => Some(VendorsBasicUnitNumber),
                "Motor Vehicle ID Number" => Some(MotorVehicleIdNumber),
                "Vendor's Specification Number" => Some(VendorsSpecificationNumber),
                "End Item Serial Number" => Some(EndItemSerialNumber),
                "Work Unit Number" => Some(WorkUnitNumber),
                "Reclamation Process" => Some(ReclamationProcess),
                "Woolen Run" => Some(WoolenRun),
                "Woolen Cut" => Some(WoolenCut),
                "Random Weight Aggregation Code" => Some(RandomWeightAggregationCode),
                "Car Class Code" => Some(CarClassCode),
                "World Code" => Some(WorldCode),
                "Airline Flight Code" => Some(AirlineFlightCode),
                "Fare Basis Code" => Some(FareBasisCode),
                "Service Class Code" => Some(ServiceClassCode),
                "Stop Over Code" => Some(StopOverCode),
                "Telecom Service Type" => Some(TelecomServiceType),
                "Advanced Billing Concepts (ABC) Codes" => Some(CodeWK),
                "Wafer Lot Identifier" => Some(WaferLot),
                "Yarn Count Worsted" => Some(YarnCountWorsted),
                "Wheel Chair Lift Serial Number" => Some(WheelChairLiftSerialNumber),
                "Preferred Part Number" => Some(PreferredPartNumber),
                "Expendable Container Identification" => {
                    Some(ExpendableContainerIdentification)
                }
                "Export Control Classification Number (ECCN)" => Some(CodeXN),
                "Preferred National Stock Number" => Some(PreferredNationalStockNumber),
                "Preferred Manufacturer" => Some(PreferredManufacturer),
                "Contractor Establishment Code" => Some(ContractorEstablishmentCode),
                "Child GTIN-13" => Some(ChildGtin13),
                "Child GTIN-14" => Some(ChildGtin14),
                "Child GTIN-12" => Some(ChildGtin12),
                "Map Edition Number" => Some(MapEditionNumber),
                "Publication Number" => Some(PublicationNumber),
                "Commercial and Government Entity (CAGE) Code" => Some(CodeZB),
                "Service Control Identification" => Some(ServiceControlIdentification),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for ProductServiceIdQualifier {
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
    type Value = ProductServiceIdQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Product/Service ID Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProductServiceIdQualifier::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Product/Service ID Qualifier: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProductServiceIdQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Product/Service ID Qualifier: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ProductServiceIdQualifier {
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