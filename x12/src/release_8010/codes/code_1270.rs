use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1270

See docs at <https://www.stedi.com/edi/x12/element/1270>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodeListQualifierCode {
    ///0 - Document Identification Code
    DocumentIdentificationCode,
    ///1 - Free On Board Site Code
    FreeOnBoardSiteCode,
    ///2 - Channel of Distribution Code
    ChannelOfDistributionCode,
    ///3 - Kind of Contract Code
    KindOfContractCode,
    ///4 - Type of Contract Code
    TypeOfContractCode,
    ///5 - Criticality Designator Code
    CriticalityDesignatorCode,
    ///7 - Quality Assurance Site Code
    QualityAssuranceSiteCode,
    ///8 - Acceptance Site Code
    AcceptanceSiteCode,
    ///10 - Transaction Status Indicator Code
    TransactionStatusIndicatorCode,
    ///11 - Contract Delivery Date Revision Agent Code
    ContractDeliveryDateRevisionAgentCode,
    ///12 - Reason for Contract Delivery Date Revision Code
    ReasonForContractDeliveryDateRevisionCode,
    ///13 - Recommendations Regarding Delayed Deliveries Code
    RecommendationsRegardingDelayedDeliveriesCode,
    ///14 - Contract Shipment Advice Code
    ContractShipmentAdviceCode,
    ///15 - Individual Insurance Financial Detail
    IndividualInsuranceFinancialDetail,
    ///16 - Cash Discount Stipulation Code
    CashDiscountStipulationCode,
    ///17 - Shipment Acceptance Discrepancy Explanation Code
    ShipmentAcceptanceDiscrepancyExplanationCode,
    ///18 - Insurance Plan Description Characteristics
    InsurancePlanDescriptionCharacteristics,
    ///19 - Contract Close-out Group Code
    ContractCloseOutGroupCode,
    ///20 - Payment Type Code
    PaymentTypeCode,
    ///21 - Contract Fund Reporting Transaction Code
    ContractFundReportingTransactionCode,
    ///22 - Contract Payment Deduction or Collection Code
    ContractPaymentDeductionOrCollectionCode,
    ///23 - Obligation Variance Code
    ObligationVarianceCode,
    ///24 - Plus or Minus Indicator Code
    PlusOrMinusIndicatorCode,
    ///25 - Reason for Delayed Closing of Contract File Code
    ReasonForDelayedClosingOfContractFileCode,
    ///26 - Contract Payment Line Item Status Code
    ContractPaymentLineItemStatusCode,
    ///27 - Special Reimbursable Provisions Code
    SpecialReimbursableProvisionsCode,
    ///28 - Kind of Modification Code
    KindOfModificationCode,
    ///29 - Purchasing Contract Officer (PCO) Instructions Code
    Code29,
    ///30 - Type of Delay Code
    TypeOfDelayCode,
    ///31 - Healthcare Provider Characteristics and Resources
    HealthcareProviderCharacteristicsAndResources,
    ///32 - Container and Roll-on/Roll-off Number Code
    ContainerAndRollOnRollOffNumberCode,
    ///33 - Air Commodity and Special Handling Code
    AirCommodityAndSpecialHandlingCode,
    ///34 - Water Commodity and Special Handling Code
    WaterCommodityAndSpecialHandlingCode,
    ///35 - Air Dimension Code
    AirDimensionCode,
    ///36 - Air Terminal Identifier Code
    AirTerminalIdentifierCode,
    ///37 - Water Terminal Identifier Code
    WaterTerminalIdentifierCode,
    ///38 - Consolidation and Containerization Point Code
    ConsolidationAndContainerizationPointCode,
    ///39 - Transportation Mode or Method Code
    TransportationModeOrMethodCode,
    ///40 - Type Pack Code
    TypePackCode,
    ///41 - Date Shipped or Received Code
    DateShippedOrReceivedCode,
    ///42 - Estimated Time of Arrival Code
    EstimatedTimeOfArrivalCode,
    ///43 - Military and Civilian Grade Code
    MilitaryAndCivilianGradeCode,
    ///44 - Seavan Ownership Code
    SeavanOwnershipCode,
    ///45 - Ocean Carrier Code
    OceanCarrierCode,
    ///46 - Voyage Document Number Code
    VoyageDocumentNumberCode,
    ///47 - Voyage Manifest Reference Code
    VoyageManifestReferenceCode,
    ///48 - Vessel Status and Terms of Carriage Code
    VesselStatusAndTermsOfCarriageCode,
    ///49 - Vessel Sustaining Code
    VesselSustainingCode,
    ///50 - Subrogation Action Code
    SubrogationActionCode,
    ///52 - Billing Advice Code
    BillingAdviceCode,
    ///53 - Billing Status Code
    BillingStatusCode,
    ///54 - Type of Bill Code
    TypeOfBillCode,
    ///55 - Recipient of Billing Status Code
    RecipientOfBillingStatusCode,
    ///56 - Sales Price Condition Code
    SalesPriceConditionCode,
    ///57 - Delivery Source Code
    DeliverySourceCode,
    ///58 - Transportation Bill Code
    TransportationBillCode,
    ///59 - Stock Fund or Non-stock Fund Code
    StockFundOrNonStockFundCode,
    ///60 - General Services Administration (GSA) Customer Supply Center Number Code
    Code60,
    ///61 - Information Indicator Code
    InformationIndicatorCode,
    ///62 - Communications Routing Identifier Code
    CommunicationsRoutingIdentifierCode,
    ///63 - Content Indicator Code
    ContentIndicatorCode,
    ///65 - Health Care Claim Status Code
    HealthCareClaimStatusCode,
    ///66 - Suffix or Limit Code
    SuffixOrLimitCode,
    ///67 - Type of Assistance Code
    TypeOfAssistanceCode,
    ///68 - Healthcare Provider Taxonomy
    HealthcareProviderTaxonomy,
    ///69 - Foreign Military Sales Country Code
    ForeignMilitarySalesCountryCode,
    ///71 - Service and Agency Code
    ServiceAndAgencyCode,
    ///72 - Disbursement Status Code
    DisbursementStatusCode,
    ///73 - Aid Type Code
    AidTypeCode,
    ///74 - Demand Code
    DemandCode,
    ///75 - Suffix Code
    SuffixCode,
    ///78 - Project Code
    ProjectCode,
    ///79 - Priority Designator Code
    PriorityDesignatorCode,
    ///80 - Advice Code
    AdviceCode,
    ///81 - Status Code
    StatusCode,
    ///82 - Shipment Hold Code
    ShipmentHoldCode,
    ///83 - Supply Condition Code
    SupplyConditionCode,
    ///84 - Management Code
    ManagementCode,
    ///85 - Country and Activity Code
    CountryAndActivityCode,
    ///87 - Subsistence Type of Pack Code
    SubsistenceTypeOfPackCode,
    ///88 - Disposal Authority Code
    DisposalAuthorityCode,
    ///89 - Cooperative Logistics Program Support Code
    CooperativeLogisticsProgramSupportCode,
    ///90 - Precious Metals Indicator Code
    PreciousMetalsIndicatorCode,
    ///91 - Automated Data Processing Equipment Identification Code
    AutomatedDataProcessingEquipmentIdentificationCode,
    ///92 - Reason for Disposal Code
    ReasonForDisposalCode,
    ///93 - Type of Storage Code
    TypeOfStorageCode,
    ///94 - Identification Code
    IdentificationCode,
    ///95 - Offer and Release Option Code
    OfferAndReleaseOptionCode,
    ///96 - Shipment Release Code
    ShipmentReleaseCode,
    ///97 - Ultimate Recipient Code
    UltimateRecipientCode,
    ///98 - Reason for Requisitioning Code
    ReasonForRequisitioningCode,
    ///99 - Purpose Code
    PurposeCode,
    ///100 - Freddie Mac (Federal Home Loan Mortgage Corporation) Special Character Code
    Code100,
    ///101 - Fannie Mae (Federal National Mortgage Association) Special Feature Code
    Code101,
    ///102 - Mortgage Index Source Code
    MortgageIndexSourceCode,
    ///103 - Fannie Mae (Federal National Mortgage Association) Remittance Programs
    Code103,
    ///104 - Freddie Mac (Federal Home Loan Mortgage Corporation) Remittance Programs
    Code104,
    ///105 - Freddie Mac (Federal Home Loan Mortgage Corporation) Mortgage Insurance Code
    Code105,
    ///106 - Fannie Mae (Federal National Mortgage Association) Pool Feature Code
    Code106,
    ///107 - Fannie Mae (Federal National Mortgage Association) Mortgage Insurance Code
    Code107,
    ///108 - Testing Statistical Category Code List
    TestingStatisticalCategoryCodeList,
    ///109 - Testing Demographic Category Code List
    TestingDemographicCategoryCodeList,
    ///A - American Society for Testing and Materials (ASTM)
    CodeA,
    ///A1 - Ownership Code
    OwnershipCode,
    ///A2 - Customer Within Country Code
    CustomerWithinCountryCode,
    ///A3 - Delivery Term Code
    DeliveryTermCode,
    ///A4 - Case Designator Number
    CaseDesignatorNumber,
    ///A5 - Subcase Number
    SubcaseNumber,
    ///A6 - Freight Forwarder Number
    FreightForwarderNumber,
    ///A7 - Record Control Number
    RecordControlNumber,
    ///A8 - Program Year Code
    ProgramYearCode,
    ///A9 - Supplemental Data
    SupplementalData,
    ///AA - Country Code (Finance and Acquisition)
    CodeAA,
    ///AAA - SNOMED, Systematized Nomenclature of Medicine
    CodeAAA,
    ///AAD - Asset Type
    AssetType,
    ///AAE - Current Asset Type
    CurrentAssetType,
    ///AAF - Current Liability Type
    CurrentLiabilityType,
    ///AAG - Dun and Bradstreet Canada's 8 digit Standard Industrial Classification Code
    DunAndBradstreetCanadas8DigitStandardIndustrialClassificationCode,
    ///AAH - Financial Item Allocation Code
    FinancialItemAllocationCode,
    ///AAI - Financial Item Attributed Code
    FinancialItemAttributedCode,
    ///AAJ - Financial Item Reclassification Code
    FinancialItemReclassificationCode,
    ///AAK - Functional Area
    FunctionalArea,
    ///AAL - Hobby Code
    HobbyCode,
    ///AAM - Investment Type
    InvestmentType,
    ///AAN - Liability Type
    LiabilityType,
    ///AAO - Projection Type
    ProjectionType,
    ///AAP - Trend Reason
    TrendReason,
    ///AAQ - NACHA (National Automated Clearing House Association)
    CodeAAQ,
    ///AAR - CPA (Canadian Payments Association)
    CodeAAR,
    ///AAS - Proprietary
    Proprietary,
    ///AAT - Fannie Mae Adjustable Rate Mortgage Plan Codes
    FannieMaeAdjustableRateMortgagePlanCodes,
    ///AAU - International Classification of Diseases Clinical Modification (ICD-9-CM) Diagnosis Encountered During Examination and Investigation of Individuals and Populations Code
    CodeAAU,
    ///AAV - International Classification of Diseases Clinical Modification (ICD-9-CM) Vaccination, Innoculation or Isolation Code
    CodeAAV,
    ///AAW - Immunization Injection Code
    ImmunizationInjectionCode,
    ///AAX - International Classification of Diseases Clinical Modification (ICD-9-CM) Code
    CodeAAX,
    ///AAY - Current Dental Terminology (CDT) Code
    CodeAAY,
    ///AB - Defense Priorities and Allocations System Code
    DefensePrioritiesAndAllocationsSystemCode,
    ///ABF - International Classification of Diseases Clinical Modification (ICD-10-CM) Diagnosis
    CodeABF,
    ///ABJ - International Classification of Diseases Clinical Modification (ICD-10-CM) Admitting Diagnosis
    CodeABJ,
    ///ABK - International Classification of Diseases Clinical Modification (ICD-10-CM) Principal Diagnosis
    CodeABK,
    ///ABN - International Classification of Diseases Clinical Modification (ICD-10-CM) External Cause of Injury Code
    CodeABN,
    ///ABR - Assigned by Receiver
    AssignedByReceiver,
    ///ABS - Assigned by Sender
    AssignedBySender,
    ///ABU - International Classification of Diseases Clinical Modification (ICD-10-CM) Diagnosis Encountered During Examination and Investigation of Individuals and Populations Code
    CodeABU,
    ///ABV - International Classification of Diseases Clinical Modification (ICD-10-CM) Vaccination, Innoculation or Isolation Code
    CodeABV,
    ///AC - Account Characteristics Code
    AccountCharacteristicsCode,
    ///ACC - Accounting Error Classification Code
    AccountingErrorClassificationCode,
    ///ACR - Academic Rank
    AcademicRank,
    ///ACS - List of DoD Accounting Requirements Codes.
    ListOfDoDAccountingRequirementsCodes,
    ///ACT - List of DoD Asset Category Codes
    ListOfDoDAssetCategoryCodes,
    ///ACU - List of DoD Controlled Item Codes.
    ListOfDoDControlledItemCodes,
    ///ACV - List of DoD Expendability, Recoverability, Reparability Category (ERRC) Codes.
    CodeACV,
    ///ACW - List of DoD Fiduciary Depreciation Method Codes
    ListOfDoDFiduciaryDepreciationMethodCodes,
    ///ACX - List of DoD National Item Identification Number (NIIN) Status Codes
    CodeACX,
    ///ACY - Add code ACY, "List of DoD (Army) Recoverability Codes"
    CodeACY,
    ///ACZ - List of DoD Reportable Item Control Codes (RICC)
    CodeACZ,
    ///AD - Acquisition Advice Code
    AcquisitionAdviceCode,
    ///AD1 - List of DoD Storage Requirement Codes
    ListOfDoDStorageRequirementCodes,
    ///AD2 - List of DoD Temperature Controlled Codes
    ListOfDoDTemperatureControlledCodes,
    ///AD3 - List of DoD Asset Type Codes
    ListOfDoDAssetTypeCodes,
    ///AD4 - List of DoD Utilization Measure Codes
    ListOfDoDUtilizationMeasureCodes,
    ///ADD - International Classification of Diseases Clinical Modification (ICD-10-CM) Primary Diagnosis
    CodeADD,
    ///ADJ - Accounting Adjustment Method
    AccountingAdjustmentMethod,
    ///AE - Beneficiary Type
    BeneficiaryType,
    ///AEA - Army Edit Action Code
    ArmyEditActionCode,
    ///AF - Class of Pitch
    ClassOfPitch,
    ///AG - Grade of Difficulty
    GradeOfDifficulty,
    ///AH - Acquisition Method Suffix Code
    AcquisitionMethodSuffixCode,
    ///AI - Acquisition Method Code
    AcquisitionMethodCode,
    ///AJ - Utilization Code
    UtilizationCode,
    ///AJT - Adjustment Type
    AdjustmentType,
    ///AK - Distribution Code
    DistributionCode,
    ///AL - Special Requirements Code
    SpecialRequirementsCode,
    ///ALM - Allocation Method
    AllocationMethod,
    ///ALP - Alteration Lookup
    AlterationLookup,
    ///AM - Locale of Activity
    LocaleOfActivity,
    ///AN - Nature of Event Code
    NatureOfEventCode,
    ///AO - Settlement/Payout Options
    SettlementPayoutOptions,
    ///AOR - Authorized Overrun Indicator
    AuthorizedOverrunIndicator,
    ///APE - Activite Principale Exercee (APE) Code
    CodeAPE,
    ///APR - International Classification of Diseases Clinical Modification (ICD-10-CM) Patient's Reason for Visit
    CodeAPR,
    ///AQ - Application Question Identifier
    ApplicationQuestion,
    ///AQA - All Quantity Available Indicator
    AllQuantityAvailableIndicator,
    ///AR - Arrest Reason
    ArrestReason,
    ///ARD - Asset Reclassification Denial Code
    AssetReclassificationDenialCode,
    ///ARI - Allocation Rank Indicator
    AllocationRankIndicator,
    ///ARL - Allocation Rank Level
    AllocationRankLevel,
    ///AS - Form Type Code
    FormTypeCode,
    ///ASD - International Classification of Diseases Clinical Modification (ICD-10-CM) Secondary Diagnosis
    CodeASD,
    ///AT - Allegation Type Code
    AllegationTypeCode,
    ///ATD - International Classification of Diseases Clinical Modification (ICD-10-CM) Tertiary Diagnosis
    CodeATD,
    ///ATT - Allocation Transaction Type Code
    AllocationTransactionTypeCode,
    ///AU - All Patient Refined Diagnosis Related Groups (APR-DRG)
    CodeAU,
    ///AV - Subrogation Payment Options
    SubrogationPaymentOptions,
    ///AW - All Patient Diagnosis Related Groups (AP-DRG)
    CodeAW,
    ///AX - Ambulatory Patient Groups (APG)
    CodeAX,
    ///AY - Subrogation Response Codes
    SubrogationResponseCodes,
    ///AZ - Subrogation Request Codes
    SubrogationRequestCodes,
    ///B - Bank Administration Institute (BAI)
    CodeB,
    ///BA - Vessel Stowage Location Code
    VesselStowageLocationCode,
    ///BB - Business Type
    BusinessType,
    ///BBF - International Classification of Diseases Clinical Modification (ICD-11-CM) Diagnosis
    CodeBBF,
    ///BBJ - International Classification of Diseases Clinical Modification (ICD-11-CM) Admitting Diagnosis
    CodeBBJ,
    ///BBK - International Classification of Diseases Clinical Modification (ICD-11-CM) Principal Diagnosis
    CodeBBK,
    ///BBN - International Classification of Diseases Clinical Modification (ICD-11-CM) External Cause of Injury Code
    CodeBBN,
    ///BBQ - International Classification of Diseases Clinical Modification (ICD-10-PCS) Other Procedure Codes
    CodeBBQ,
    ///BBR - International Classification of Diseases Clinical Modification (ICD-10-PCS) Principal Procedure Codes
    CodeBBR,
    ///BBU - International Classification of Diseases Clinical Modification (ICD-11-CM) Diagnosis Encountered During Examination and Investigation of Individuals and Populations Code
    CodeBBU,
    ///BBV - International Classification of Diseases Clinical Modification (ICD-11-CM) Vaccination, Innoculation or Isolation Code
    CodeBBV,
    ///BC - Transportation Holding Delay Code
    TransportationHoldingDelayCode,
    ///BCC - Business Change Code
    BusinessChangeCode,
    ///BCR - Business Credit Rating
    BusinessCreditRating,
    ///BD - Transportation Priority Code
    TransportationPriorityCode,
    ///BDD - International Classification of Diseases Clinical Modification (ICD-11-CM) Primary Diagnosis
    CodeBDD,
    ///BE - Value
    Value,
    ///BF - International Classification of Diseases Clinical Modification (ICD-9-CM) Diagnosis
    CodeBF,
    ///BG - Condition
    Condition,
    ///BH - Occurrence
    Occurrence,
    ///BI - Occurrence Span
    OccurrenceSpan,
    ///BJ - International Classification of Diseases Clinical Modification (ICD-9-CM) Admitting Diagnosis
    CodeBJ,
    ///BK - International Classification of Diseases Clinical Modification (ICD-9-CM) Principal Diagnosis
    CodeBK,
    ///BL - Application Fee Status Codes
    ApplicationFeeStatusCodes,
    ///BN - International Classification of Diseases Clinical Modification (ICD-9-CM) External Cause of Injury Code (E-codes)
    CodeBN,
    ///BO - Healthcare Common Procedure Coding System
    HealthcareCommonProcedureCodingSystem,
    ///BP - Healthcare Common Procedure Coding System Principal Procedure
    HealthcareCommonProcedureCodingSystemPrincipalProcedure,
    ///BPL - Board of Inspection and Survey Part Lookup
    BoardOfInspectionAndSurveyPartLookup,
    ///BPR - International Classification of Diseases Clinical Modification (ICD-11-CM) Patient's Reason for Visit
    CodeBPR,
    ///BQ - International Classification of Diseases Clinical Modification (ICD-9-CM) Other Procedure Codes
    CodeBQ,
    ///BR - International Classification of Diseases Clinical Modification (ICD-9-CM) Principal Procedure Codes
    CodeBR,
    ///BRL - Board of Inspection and Survey Responsibility Lookup
    BoardOfInspectionAndSurveyResponsibilityLookup,
    ///BS - Current Procedural Terminology (CPT) Codes
    CodeBS,
    ///BSD - International Classification of Diseases Clinical Modification (ICD-11-CM) Secondary Diagnosis
    CodeBSD,
    ///BSL - Board of Inspection and Survey Ship Lookup
    BoardOfInspectionAndSurveyShipLookup,
    ///BSP - Business Period
    BusinessPeriod,
    ///BT - Accident Description
    AccidentDescription,
    ///BTC - Balance Type Code
    BalanceTypeCode,
    ///BTD - International Classification of Diseases Clinical Modification (ICD-11-CM) Tertiary Diagnosis
    CodeBTD,
    ///BU - Part of Body Affected
    PartOfBodyAffected,
    ///BUI - Bid Up Indicator
    BidUpIndicator,
    ///BUR - Bureau of Labor Statistics Standardized Occupational Codes
    BureauOfLaborStatisticsStandardizedOccupationalCodes,
    ///BV - Education Institution Type Code
    EducationInstitutionTypeCode,
    ///BW - Educational Areas Code
    EducationalAreasCode,
    ///BX - Profession Type Code
    ProfessionTypeCode,
    ///BY - Share Type Code
    ShareTypeCode,
    ///BZ - Business Size Code
    BusinessSizeCode,
    ///C - Canadian Inter*EDI
    CodeC,
    ///C1 - Eye Color Code
    EyeColorCode,
    ///C2 - Hair Color Code
    HairColorCode,
    ///C3 - Skin Tone Code
    SkinToneCode,
    ///CA - Type of Inquiry Code
    TypeOfInquiryCode,
    ///CAH - Advanced Billing Concepts (ABC) Codes
    CodeCAH,
    ///CB - Billed Office Indicator Code
    BilledOfficeIndicatorCode,
    ///CBQ - International Classification of Diseases Clinical Modification (ICD-11-CM) Other Procedure Codes
    CodeCBQ,
    ///CBR - International Classification of Diseases Clinical Modification (ICD-11-CM) Principal Procedure Codes
    CodeCBR,
    ///CC - Treasury Symbol Code
    TreasurySymbolCode,
    ///CCC - Correction To Cause Code
    CorrectionToCauseCode,
    ///CD - Supplementary Accounting Classification Code
    SupplementaryAccountingClassificationCode,
    ///CE - Reference and Station Code
    ReferenceAndStationCode,
    ///CF - Major Force Program Code
    MajorForceProgramCode,
    ///CFI - Contractual Flow Indicator
    ContractualFlowIndicator,
    ///CG - Aircraft Mission Design Series Code
    AircraftMissionDesignSeriesCode,
    ///CH - Type of Issue Code
    TypeOfIssueCode,
    ///CHG - Charge Indicator
    ChargeIndicator,
    ///CI - Criminal Charge
    CriminalCharge,
    ///CIE - Collision Industry Electronic Commerce Association (CIECA) - Assignment Type
    CodeCIE,
    ///CJ - Criminal Charge Grade
    CriminalChargeGrade,
    ///CK - Coupon Adjustment Reason Code
    CouponAdjustmentReasonCode,
    ///CL - County Designator Code
    CountyDesignatorCode,
    ///CLP - Cause Lookup
    CauseLookup,
    ///CM - Financial Management Service Cash-Link Code
    FinancialManagementServiceCashLinkCode,
    ///CML - Customer Maintenance Level Lookup
    CustomerMaintenanceLevelLookup,
    ///CN - Cause of Injury Code
    CauseOfInjuryCode,
    ///CNC - Change Notice Code
    ChangeNoticeCode,
    ///CO - Customized Notice Type Code
    CustomizedNoticeTypeCode,
    ///COG - Cognizance Symbol
    CognizanceSymbol,
    ///COR - Confirming Party Role
    ConfirmingPartyRole,
    ///CP - Salvage Disposition Code
    SalvageDispositionCode,
    ///CPS - Court Party Status
    CourtPartyStatus,
    ///CQ - Capacity Type Indicator
    CapacityTypeIndicator,
    ///CR - Federal Item Identification Guide Criticality (FIIG) Code
    CodeCR,
    ///CRC - Complaint Request Code
    ComplaintRequestCode,
    ///CRI - Causative Research Indicator Code
    CausativeResearchIndicatorCode,
    ///CS - Clause Status Type
    ClauseStatusType,
    ///CSD - Customer Service Designator
    CustomerServiceDesignator,
    ///CSF - Corporate Statement Filing Code
    CorporateStatementFilingCode,
    ///CT - Compensation Type Codes
    CompensationTypeCodes,
    ///CTC - Carcass Tracking Code
    CarcassTrackingCode,
    ///CU - Cuisine Type Code
    CuisineTypeCode,
    ///CV - Coverage Code List
    CoverageCodeList,
    ///CW - Controvert Code
    ControvertCode,
    ///CZ - Conviction Offense Type
    ConvictionOffenseType,
    ///D - Court Document Type Code
    CourtDocumentTypeCode,
    ///D1 - Driver's License Withdrawal Type
    DriversLicenseWithdrawalType,
    ///D2 - Driver's License Withdrawal Extent
    DriversLicenseWithdrawalExtent,
    ///D3 - Driver's License Withdrawal Basis
    DriversLicenseWithdrawalBasis,
    ///D4 - Driver's License Withdrawal Due Process Status
    DriversLicenseWithdrawalDueProcessStatus,
    ///D5 - Driver's License Withdrawal Reason
    DriversLicenseWithdrawalReason,
    ///DA - Device Availability Code
    DeviceAvailabilityCode,
    ///DAC - Document Availability Code
    DocumentAvailabilityCode,
    ///DAP - All Patient, Severity-Adjusted DRGs (APS-DRG)
    CodeDAP,
    ///DB - Debtor Business Type Code
    DebtorBusinessTypeCode,
    ///DBS - DUN's Standard Industrial Classification (SIC) 2+2, Dun and Bradstreet
    CodeDBS,
    ///DC - Report Distribution Code
    ReportDistributionCode,
    ///DCC - Cause Code
    CauseCode,
    ///DCM - Medicare DRG (CMS-DRG & MS-DRG)
    CodeDCM,
    ///DCR - Disposition Category Change Reject Reason Code
    DispositionCategoryChangeRejectReasonCode,
    ///DCS - Disposition Sub-Category Code
    DispositionSubCategoryCode,
    ///DCT - Disposition Category Code
    DispositionCategoryCode,
    ///DD - International Classification of Diseases Clinical Modification (ICD-9-CM) Primary Diagnosis
    CodeDD,
    ///DE - Signal Code
    SignalCode,
    ///DF - Media and Status Code
    MediaAndStatusCode,
    ///DG - Fund Code
    FundCode,
    ///DGO - Dynamic Generator Set Code
    DynamicGeneratorSetCode,
    ///DH - Drug Detail Code
    DrugDetailCode,
    ///DI - Single Use Label Code
    SingleUseLabelCode,
    ///DIR - International-Refined DRGs (IR-DRG)
    CodeDIR,
    ///DJ - Remedial Action Code
    RemedialActionCode,
    ///DK - Program Originator Code
    ProgramOriginatorCode,
    ///DL - Service Contract Act Operation Code
    ServiceContractActOperationCode,
    ///DLO - Dynamic Locomotive Tag Code
    DynamicLocomotiveTagCode,
    ///DLP - Deferral Lookup
    DeferralLookup,
    ///DLT - Long Term Care DRG - LTC-DRG
    LongTermCareDrgLtcDrg,
    ///DM - Agent Status Code
    AgentStatusCode,
    ///DMI - Demilitarization Integrity Code
    DemilitarizationIntegrityCode,
    ///DMP - Demilitarization Performed Code
    DemilitarizationPerformedCode,
    ///DN - Nature of Debt Code
    NatureOfDebtCode,
    ///DNT - Document Number Requirement Type
    DocumentNumberRequirementType,
    ///DO - Device Operator Type Code
    DeviceOperatorTypeCode,
    ///DOF - Direction of Flow
    DirectionOfFlow,
    ///DP - Producer Financial History Codes
    ProducerFinancialHistoryCodes,
    ///DPC - Delivery Priority Code
    DeliveryPriorityCode,
    ///DPE - Association of American Railroads Deprescription Exception List
    AssociationOfAmericanRailroadsDeprescriptionExceptionList,
    ///DPL - Association of American Railroads Deprescription Distribution List
    AssociationOfAmericanRailroadsDeprescriptionDistributionList,
    ///DQ - Device Status Code
    DeviceStatusCode,
    ///DR - Diagnosis Related Group (DRG)
    CodeDR,
    ///DRD - Refined DRGs (R-DRG)
    CodeDRD,
    ///DRL - Collision Industry Electronic Commerce Association (CIECA) - Detail Repair Lines Code List
    CodeDRL,
    ///DRS - Disposition Services Reimbursement Code
    DispositionServicesReimbursementCode,
    ///DS - Related Device Applicability Code
    RelatedDeviceApplicabilityCode,
    ///DSC - Disposition Services Customer Type Code
    DispositionServicesCustomerTypeCode,
    ///DSD - Severity DRGs (S-DRG)
    CodeDSD,
    ///DSI - Disposition Services Indicator Code
    DispositionServicesIndicatorCode,
    ///DSR - Data Sets Requested
    DataSetsRequested,
    ///DSS - Delivery Scheduling Status
    DeliverySchedulingStatus,
    ///DT - Debtor Type Code
    DebtorTypeCode,
    ///DTS - List of codes identifying DoD Distribution Services Terms of Sale.
    ListOfCodesIdentifyingDoDDistributionServicesTermsOfSale,
    ///DU - Device Usage Code
    DeviceUsageCode,
    ///DV - Demand Planning Status Code
    DemandPlanningStatusCode,
    ///DW - Estimating Method Status Code
    EstimatingMethodStatusCode,
    ///DX - Contact Status Code
    ContactStatusCode,
    ///DY - Type of Firm Code
    TypeOfFirmCode,
    ///DZ - Reportable Event Status Code
    ReportableEventStatusCode,
    ///E - Diagnostic Statistical Manual of Mental Disorders Code List (DSM)
    CodeE,
    ///EA - Asset Status or Transaction Reporting Code
    AssetStatusOrTransactionReportingCode,
    ///EAA - Alabama Campaign Disclosure Report Codes
    AlabamaCampaignDisclosureReportCodes,
    ///EAB - Alaska Campaign Disclosure Report Codes
    AlaskaCampaignDisclosureReportCodes,
    ///EAC - American Samoa Campaign Disclosure Report Codes
    AmericanSamoaCampaignDisclosureReportCodes,
    ///EAD - Arizona Campaign Disclosure Report Codes
    ArizonaCampaignDisclosureReportCodes,
    ///EAE - Arkansas Campaign Disclosure Report Codes
    ArkansasCampaignDisclosureReportCodes,
    ///EAF - California Campaign Disclosure Report Codes
    CaliforniaCampaignDisclosureReportCodes,
    ///EAG - Colorado Campaign Disclosure Report Codes
    ColoradoCampaignDisclosureReportCodes,
    ///EAH - Connecticut Campaign Disclosure Report Codes
    ConnecticutCampaignDisclosureReportCodes,
    ///EAI - Delaware Campaign Disclosure Report Codes
    DelawareCampaignDisclosureReportCodes,
    ///EAJ - District of Columbia Campaign Disclosure Report Codes
    DistrictOfColumbiaCampaignDisclosureReportCodes,
    ///EAK - Florida Campaign Disclosure Report Codes
    FloridaCampaignDisclosureReportCodes,
    ///EAL - Georgia Campaign Disclosure Report Codes
    GeorgiaCampaignDisclosureReportCodes,
    ///EAM - Guam Campaign Disclosure Report Codes
    GuamCampaignDisclosureReportCodes,
    ///EAN - Hawaii Campaign Disclosure Report Codes
    HawaiiCampaignDisclosureReportCodes,
    ///EAO - Idaho Campaign Disclosure Report Codes
    IdahoCampaignDisclosureReportCodes,
    ///EAP - Illinois Campaign Disclosure Report Codes
    IllinoisCampaignDisclosureReportCodes,
    ///EAQ - Indiana Campaign Disclosure Report Codes
    IndianaCampaignDisclosureReportCodes,
    ///EAR - Iowa Campaign Disclosure Report Codes
    IowaCampaignDisclosureReportCodes,
    ///EAS - Kansas Campaign Disclosure Report Codes
    KansasCampaignDisclosureReportCodes,
    ///EAT - Kentucky Campaign Disclosure Report Codes
    KentuckyCampaignDisclosureReportCodes,
    ///EAU - Louisiana Campaign Disclosure Report Codes
    LouisianaCampaignDisclosureReportCodes,
    ///EAV - Maine Campaign Disclosure Report Codes
    MaineCampaignDisclosureReportCodes,
    ///EAW - Maryland Campaign Disclosure Report Codes
    MarylandCampaignDisclosureReportCodes,
    ///EAX - Massachusetts Campaign Disclosure Report Codes
    MassachusettsCampaignDisclosureReportCodes,
    ///EAY - Michigan Campaign Disclosure Report Codes
    MichiganCampaignDisclosureReportCodes,
    ///EAZ - Minnesota Campaign Disclosure Report Codes
    MinnesotaCampaignDisclosureReportCodes,
    ///EB - Asset Transfer Status Code
    AssetTransferStatusCode,
    ///EBA - Mississippi Campaign Disclosure Report Codes
    MississippiCampaignDisclosureReportCodes,
    ///EBB - Missouri Campaign Disclosure Report Codes
    MissouriCampaignDisclosureReportCodes,
    ///EBC - Montana Campaign Disclosure Report Codes
    MontanaCampaignDisclosureReportCodes,
    ///EBD - Nebraska Campaign Disclosure Report Codes
    NebraskaCampaignDisclosureReportCodes,
    ///EBE - Nevada Campaign Disclosure Report Codes
    NevadaCampaignDisclosureReportCodes,
    ///EBF - New Hampshire Campaign Disclosure Report Codes
    NewHampshireCampaignDisclosureReportCodes,
    ///EBG - New Jersey Campaign Disclosure Report Codes
    NewJerseyCampaignDisclosureReportCodes,
    ///EBH - New Mexico Campaign Disclosure Report Codes
    NewMexicoCampaignDisclosureReportCodes,
    ///EBI - New York Campaign Disclosure Report Codes
    NewYorkCampaignDisclosureReportCodes,
    ///EBJ - North Carolina Campaign Disclosure Report Codes
    NorthCarolinaCampaignDisclosureReportCodes,
    ///EBK - North Dakota Campaign Disclosure Report Codes
    NorthDakotaCampaignDisclosureReportCodes,
    ///EBL - Ohio Campaign Disclosure Report Codes
    OhioCampaignDisclosureReportCodes,
    ///EBM - Oklahoma Campaign Disclosure Report Codes
    OklahomaCampaignDisclosureReportCodes,
    ///EBN - Oregon Campaign Disclosure Report Codes
    OregonCampaignDisclosureReportCodes,
    ///EBO - Pennsylvania Campaign Disclosure Report Codes
    PennsylvaniaCampaignDisclosureReportCodes,
    ///EBP - Puerto Rico Campaign Disclosure Report Codes
    PuertoRicoCampaignDisclosureReportCodes,
    ///EBQ - Rhode Island Campaign Disclosure Report Codes
    RhodeIslandCampaignDisclosureReportCodes,
    ///EBR - South Carolina Campaign Disclosure Report Codes
    SouthCarolinaCampaignDisclosureReportCodes,
    ///EBS - South Dakota Campaign Disclosure Report Codes
    SouthDakotaCampaignDisclosureReportCodes,
    ///EBT - Tennessee Campaign Disclosure Report Codes
    TennesseeCampaignDisclosureReportCodes,
    ///EBU - Texas Campaign Disclosure Report Codes
    TexasCampaignDisclosureReportCodes,
    ///EBV - Utah Campaign Disclosure Report Codes
    UtahCampaignDisclosureReportCodes,
    ///EBW - Vermont Campaign Disclosure Report Codes
    VermontCampaignDisclosureReportCodes,
    ///EBX - Virginia Campaign Disclosure Report Codes
    VirginiaCampaignDisclosureReportCodes,
    ///EBY - Virgin Islands Campaign Disclosure Report Codes
    VirginIslandsCampaignDisclosureReportCodes,
    ///EBZ - Washington Campaign Disclosure Report Codes
    WashingtonCampaignDisclosureReportCodes,
    ///EC - Certification Requirements Code
    CertificationRequirementsCode,
    ///ECA - West Virginia Campaign Disclosure Report Codes
    WestVirginiaCampaignDisclosureReportCodes,
    ///ECB - Wisconsin Campaign Disclosure Report Codes
    WisconsinCampaignDisclosureReportCodes,
    ///ECC - Wyoming Campaign Disclosure Report Codes
    WyomingCampaignDisclosureReportCodes,
    ///ECD - Alberta Campaign Disclosure Report Codes
    AlbertaCampaignDisclosureReportCodes,
    ///ECE - British Columbia Campaign Disclosure Report Codes
    BritishColumbiaCampaignDisclosureReportCodes,
    ///ECF - Manitoba Campaign Disclosure Report Codes
    ManitobaCampaignDisclosureReportCodes,
    ///ECG - New Brunswick Campaign Disclosure Report Codes
    NewBrunswickCampaignDisclosureReportCodes,
    ///ECH - Newfoundland Campaign Disclosure Report Codes
    NewfoundlandCampaignDisclosureReportCodes,
    ///ECI - Northwest Territories Campaign Disclosure Report Codes
    NorthwestTerritoriesCampaignDisclosureReportCodes,
    ///ECJ - Nova Scotia Campaign Disclosure Report Codes
    NovaScotiaCampaignDisclosureReportCodes,
    ///ECK - Ontario Campaign Disclosure Report Codes
    OntarioCampaignDisclosureReportCodes,
    ///ECL - Prince Edward Island Campaign Disclosure Report Codes
    PrinceEdwardIslandCampaignDisclosureReportCodes,
    ///ECM - Quebec Campaign Disclosure Report Codes
    QuebecCampaignDisclosureReportCodes,
    ///ECN - Saskatchewan Campaign Disclosure Report Codes
    SaskatchewanCampaignDisclosureReportCodes,
    ///ECO - Yukon Territory Campaign Disclosure Report Codes
    YukonTerritoryCampaignDisclosureReportCodes,
    ///ECP - Federal Campaign Disclosure Report Codes
    FederalCampaignDisclosureReportCodes,
    ///ECQ - Alabama Lobbyist Report Codes
    AlabamaLobbyistReportCodes,
    ///ECR - Alaska Lobbyist Report Codes
    AlaskaLobbyistReportCodes,
    ///ECS - Arizona Lobbyist Report Codes
    ArizonaLobbyistReportCodes,
    ///ECT - Arkansas Lobbyist Report Codes
    ArkansasLobbyistReportCodes,
    ///ECU - California Lobbyist Report Codes
    CaliforniaLobbyistReportCodes,
    ///ECV - Colorado Lobbyist Report Codes
    ColoradoLobbyistReportCodes,
    ///ECW - Connecticut Lobbyist Report Codes
    ConnecticutLobbyistReportCodes,
    ///ECX - Delaware Lobbyist Report Codes
    DelawareLobbyistReportCodes,
    ///ECY - District of Columbia Lobbyist Report Codes
    DistrictOfColumbiaLobbyistReportCodes,
    ///ECZ - Florida Lobbyist Report Codes
    FloridaLobbyistReportCodes,
    ///ED - Coast Designation Code
    CoastDesignationCode,
    ///EDA - Georgia Lobbyist Report Codes
    GeorgiaLobbyistReportCodes,
    ///EDB - Hawaii Lobbyist Report Codes
    HawaiiLobbyistReportCodes,
    ///EDC - Idaho Lobbyist Report Codes
    IdahoLobbyistReportCodes,
    ///EDD - Illinois Lobbyist Report Codes
    IllinoisLobbyistReportCodes,
    ///EDE - Indiana Lobbyist Report Codes
    IndianaLobbyistReportCodes,
    ///EDF - Iowa Lobbyist Report Codes
    IowaLobbyistReportCodes,
    ///EDG - Kansas Lobbyist Report Codes
    KansasLobbyistReportCodes,
    ///EDH - Kentucky Lobbyist Report Codes
    KentuckyLobbyistReportCodes,
    ///EDI - Louisiana Lobbyist Report Codes
    LouisianaLobbyistReportCodes,
    ///EDJ - Maine Lobbyist Report Codes
    MaineLobbyistReportCodes,
    ///EDK - Maryland Lobbyist Report Codes
    MarylandLobbyistReportCodes,
    ///EDL - Massachusetts Lobbyist Report Codes
    MassachusettsLobbyistReportCodes,
    ///EDM - Michigan Lobbyist Report Codes
    MichiganLobbyistReportCodes,
    ///EDN - Minnesota Lobbyist Report Codes
    MinnesotaLobbyistReportCodes,
    ///EDO - Mississippi Lobbyist Report Codes
    MississippiLobbyistReportCodes,
    ///EDP - Missouri Lobbyist Report Codes
    MissouriLobbyistReportCodes,
    ///EDQ - Montana Lobbyist Report Codes
    MontanaLobbyistReportCodes,
    ///EDR - Nebraska Lobbyist Report Codes
    NebraskaLobbyistReportCodes,
    ///EDS - Nevada Lobbyist Report Codes
    NevadaLobbyistReportCodes,
    ///EDT - New Hampshire Lobbyist Report Codes
    NewHampshireLobbyistReportCodes,
    ///EDU - New Jersey Lobbyist Report Codes
    NewJerseyLobbyistReportCodes,
    ///EDV - New Mexico Lobbyist Report Codes
    NewMexicoLobbyistReportCodes,
    ///EDW - New York Lobbyist Report Codes
    NewYorkLobbyistReportCodes,
    ///EDX - North Carolina Lobbyist Report Codes
    NorthCarolinaLobbyistReportCodes,
    ///EDY - North Dakota Lobbyist Report Codes
    NorthDakotaLobbyistReportCodes,
    ///EDZ - Ohio Lobbyist Report Codes
    OhioLobbyistReportCodes,
    ///EE - Competitive Characteristics Code
    CompetitiveCharacteristicsCode,
    ///EEA - Oklahoma Lobbyist Report Codes
    OklahomaLobbyistReportCodes,
    ///EEB - Oregon Lobbyist Report Codes
    OregonLobbyistReportCodes,
    ///EEC - Pennsylvania Lobbyist Report Codes
    PennsylvaniaLobbyistReportCodes,
    ///EED - Puerto Rico Lobbyist Report Codes
    PuertoRicoLobbyistReportCodes,
    ///EEE - Rhode Island Lobbyist Report Codes
    RhodeIslandLobbyistReportCodes,
    ///EEF - South Carolina Lobbyist Report Codes
    SouthCarolinaLobbyistReportCodes,
    ///EEG - South Dakota Lobbyist Report Codes
    SouthDakotaLobbyistReportCodes,
    ///EEH - Tennessee Lobbyist Report Codes
    TennesseeLobbyistReportCodes,
    ///EEI - Texas Lobbyist Report Codes
    TexasLobbyistReportCodes,
    ///EEJ - Utah Lobbyist Report Codes
    UtahLobbyistReportCodes,
    ///EEK - Vermont Lobbyist Report Codes
    VermontLobbyistReportCodes,
    ///EEL - Virginia Lobbyist Report Codes
    VirginiaLobbyistReportCodes,
    ///EEM - Washington Lobbyist Report Codes
    WashingtonLobbyistReportCodes,
    ///EEN - West Virginia Lobbyist Report Codes
    WestVirginiaLobbyistReportCodes,
    ///EEO - Wisconsin Lobbyist Report Codes
    WisconsinLobbyistReportCodes,
    ///EEP - Wyoming Lobbyist Report Codes
    WyomingLobbyistReportCodes,
    ///EEQ - New York City Campaign Disclosure Report Codes
    NewYorkCityCampaignDisclosureReportCodes,
    ///EER - Seattle Campaign Disclosure Report Codes
    SeattleCampaignDisclosureReportCodes,
    ///EES - New York City Lobbyist Report Codes
    NewYorkCityLobbyistReportCodes,
    ///EF - Correction or Change for Storage Item Records Code
    CorrectionOrChangeForStorageItemRecordsCode,
    ///EG - Excavation Information Code List
    ExcavationInformationCodeList,
    ///EH - Type Due-In Indicator
    TypeDueInIndicator,
    ///EI - Discrepancy Indicator Code
    DiscrepancyIndicatorCode,
    ///EJ - Disposal Condition Code
    DisposalConditionCode,
    ///EK - Event or Exposure Code
    EventOrExposureCode,
    ///EL - Error Classification Code
    ErrorClassificationCode,
    ///EM - Inventory Category Code
    InventoryCategoryCode,
    ///EMC - Automotive Aftermarket Industry Association (AAIA) Emission Code
    CodeEMC,
    ///EN - Local Source Code
    LocalSourceCode,
    ///EO - Adverse Event Outcome Code
    AdverseEventOutcomeCode,
    ///EP - Enhanced Ambulatory Patient Groups (EAPG)
    CodeEP,
    ///EPI - Exchange Price Indicator
    ExchangePriceIndicator,
    ///EQ - Controlled Inventory Item Code
    ControlledInventoryItemCode,
    ///EQR - Equipment Request Codes
    EquipmentRequestCodes,
    ///ER - Department of Defense Identification Code
    DepartmentOfDefenseIdentificationCode,
    ///ERC - Equipment Repair Condition Code
    EquipmentRepairConditionCode,
    ///ERJ - Equipment Repair Job Code
    EquipmentRepairJobCode,
    ///ERL - Equipment Repair Location Code
    EquipmentRepairLocationCode,
    ///ERR - Equipment Repair Responsibility Code
    EquipmentRepairResponsibilityCode,
    ///ES - Extension Reason
    ExtensionReason,
    ///ESC - Electrostatic Discharge Code
    ElectrostaticDischargeCode,
    ///ESL - Equipment Status Lookup
    EquipmentStatusLookup,
    ///ET - Reject Advice Code
    RejectAdviceCode,
    ///ETL - Estimate Type Lookup
    EstimateTypeLookup,
    ///EU - Request Code
    RequestCode,
    ///EV - Review Period Indicator Code
    ReviewPeriodIndicatorCode,
    ///EW - Small Arms Error Transaction Reject Code
    SmallArmsErrorTransactionRejectCode,
    ///EWC - Evaluate Work Candidate Lookup
    EvaluateWorkCandidateLookup,
    ///EWM - Equipment Why Made Code
    EquipmentWhyMadeCode,
    ///EWR - Evaluate Work Candidate Reason Lookup
    EvaluateWorkCandidateReasonLookup,
    ///EX - Small Arms Transaction Code
    SmallArmsTransactionCode,
    ///EXD, XD - Export Declaration
    ExportDeclaration,
    ///EXP - Export Control Classification Number (ECCN)
    CodeEXP,
    ///EY - Special Program Requirement Status Code
    SpecialProgramRequirementStatusCode,
    ///EZ - Type Inspection Code
    TypeInspectionCode,
    ///F - Financial Rating
    FinancialRating,
    ///FA - Type of Contractor Code
    TypeOfContractorCode,
    ///FAP - Fannie Mae Refinance Plan Code
    FannieMaeRefinancePlanCode,
    ///FB - Type of Media Code
    TypeOfMediaCode,
    ///FC - Type Physical Inventory or Transaction History Code
    TypePhysicalInventoryOrTransactionHistoryCode,
    ///FC1 - Federal Communication, Control and Security Code List 1
    CodeFC1,
    ///FD - Demilitarization Code
    DemilitarizationCode,
    ///FE - Shelf Life Code
    ShelfLifeCode,
    ///FF - Essentiality Code
    EssentialityCode,
    ///FF1 - Federal Finance Code List 1
    FederalFinanceCodeList1,
    ///FG - Source Maintenance and Recoverability Code
    SourceMaintenanceAndRecoverabilityCode,
    ///FH - Type of Location Reconciliation Request
    TypeOfLocationReconciliationRequest,
    ///FH1 - Federal Health Care Code List 1
    FederalHealthCareCodeList1,
    ///FI - Applicant Type
    ApplicantType,
    ///FIR - Financial Inventory Report Code
    FinancialInventoryReportCode,
    ///FJ - Antenna Structure Type
    AntennaStructureType,
    ///FK - Station Classification
    StationClassification,
    ///FL - Radio Frequency Type
    RadioFrequencyType,
    ///FL1 - Federal Logistics Code List 1
    FederalLogisticsCodeList1,
    ///FM - Station Classification Type
    StationClassificationType,
    ///FMO - Former Major Organizational Entity Rule Number
    FormerMajorOrganizationalEntityRuleNumber,
    ///FMS - Foreign Military Sales and Military Assistance Program Grant Aid Type of Assistance/Financing Code
    ForeignMilitarySalesAndMilitaryAssistanceProgramGrantAidTypeOfAssistanceFinancingCode,
    ///FN - Class of Operation
    ClassOfOperation,
    ///FO - Antenna Polarization
    AntennaPolarization,
    ///FP - Fund Purpose
    FundPurpose,
    ///FP1 - Federal Procurement Code List 1
    FederalProcurementCodeList1,
    ///FQ - Radio System Type
    RadioSystemType,
    ///FR - Frequency Band
    FrequencyBand,
    ///FRP - Freddie Mac Refinance Plan Code
    FreddieMacRefinancePlanCode,
    ///FRT - Freight Bill Application Error Edit Codes
    FreightBillApplicationErrorEditCodes,
    ///FS - Area of Operation
    AreaOfOperation,
    ///FT - Application Type
    ApplicationType,
    ///FT1 - Federal Transportation Code List 1
    FederalTransportationCodeList1,
    ///FU - Authorization Type
    AuthorizationType,
    ///FV - Radio Service Type
    RadioServiceType,
    ///FW - Applicant Classification Type
    ApplicantClassificationType,
    ///FX - Frequency
    Frequency,
    ///FZ - Edit Error Code
    EditErrorCode,
    ///G - Risk Class
    RiskClass,
    ///G1 - Uniform Residential Appraisal Attributes Code
    UniformResidentialAppraisalAttributesCode,
    ///GA - Action Code
    ActionCode,
    ///GB - Medium of Transmission Code
    MediumOfTransmissionCode,
    ///GC - Management Indicator Code (Petroleum)
    CodeGC,
    ///GD - Gain or Loss Indicator Code
    GainOrLossIndicatorCode,
    ///GE - Type Adjustment Code
    TypeAdjustmentCode,
    ///GF - Type Identity Change Code
    TypeIdentityChangeCode,
    ///GG - Transportation Mode Reason Code
    TransportationModeReasonCode,
    ///GI - Notification Indicator Code
    NotificationIndicatorCode,
    ///GJ - Reject Indicator Code
    RejectIndicatorCode,
    ///GK - Investigation Status Code
    InvestigationStatusCode,
    ///GQ - Group Qualifier Code
    GroupQualifierCode,
    ///GR - National Council on Compensation Insurance (NCCI) Nature of Injury Code
    CodeGR,
    ///GS - Occupational Safety and Health Administration (OSHA) Nature of Injury Code
    CodeGS,
    ///GT - National Council on Compensation Insurance (NCCI) Part of Body Code
    CodeGT,
    ///GU - Occupational Safety and Health Administration (OSHA) Part of Body Code
    CodeGU,
    ///GV - National Council on Compensation Insurance (NCCI) Source of Injury Code
    CodeGV,
    ///GW - Occupational Safety and Health Administration (OSHA) Source of Injury Code
    CodeGW,
    ///GX - Glass Action Code
    GlassActionCode,
    ///GY - Cause of Loss Code
    CauseOfLossCode,
    ///GZ - Loss Description Code
    LossDescriptionCode,
    ///H - Life/Annuity Status Codes
    LifeAnnuityStatusCodes,
    ///HA - Discrepancy Code
    DiscrepancyCode,
    ///HB - Discrepancy Advice Code
    DiscrepancyAdviceCode,
    ///HC - Institutional Sector or Level Classification Code
    InstitutionalSectorOrLevelClassificationCode,
    ///HD - Discrepancy Status or Disposition Code
    DiscrepancyStatusOrDispositionCode,
    ///HE - Remittance Advice Remark Code
    RemittanceAdviceRemarkCode,
    ///HF - Education Staff Type Code
    EducationStaffTypeCode,
    ///HG - Education Fee Type Code
    EducationFeeTypeCode,
    ///HI - Health Industry Number
    HealthIndustryNumber,
    ///HJ - Institutional Fee Basis Code
    InstitutionalFeeBasisCode,
    ///HK - National Center for Education Statistics Integrated Postsecondary Education Data System Institutional Characteristics Survey Code
    NationalCenterForEducationStatisticsIntegratedPostsecondaryEducationDataSystemInstitutionalCharacteristicsSurveyCode,
    ///HL - Accreditation, Affiliation, or Licensing Level Code
    CodeHL,
    ///HM - National Center for Education Statistics Accreditation or Licensing Type
    NationalCenterForEducationStatisticsAccreditationOrLicensingType,
    ///HMC - Hazardous Material Content Code
    HazardousMaterialContentCode,
    ///HRC - Hazardous Response Codes
    HazardousResponseCodes,
    ///HS - Service Contract Act Occupation Category Code
    ServiceContractActOccupationCategoryCode,
    ///HZR - Association of American Railroads Standard Transportation Commodity Code Description Qualifier
    AssociationOfAmericanRailroadsStandardTransportationCommodityCodeDescriptionQualifier,
    ///I - Identifying Characteristics
    IdentifyingCharacteristics,
    ///IBP - Insurance Business Process Application Error Code
    InsuranceBusinessProcessApplicationErrorCode,
    ///IC - Collision Industry Electronic Commerce Association (CIECA) - Inspection
    CodeIC,
    ///ICF - International Classification of Functioning Disability and Health (ICF)
    CodeICF,
    ///ID - Identity Disclosure Code
    IdentityDisclosureCode,
    ///IF - Investment Fund Type
    InvestmentFundType,
    ///IMC - Item Management Code
    ItemManagementCode,
    ///IMP - Impact Recorder Code
    ImpactRecorderCode,
    ///IND - Intra-Navy Disposal Release Order Reject Advice Code
    IntraNavyDisposalReleaseOrderRejectAdviceCode,
    ///IPA - Impact Axis or Analog Port Code
    ImpactAxisOrAnalogPortCode,
    ///IPG - Issue Priority Group
    IssuePriorityGroup,
    ///IQ - IRS Qualification Code
    IrsQualificationCode,
    ///IRR - Issue, Repair and Requisition Code
    CodeIRR,
    ///IRT - Imbalance Reporting Type
    ImbalanceReportingType,
    ///IT - Initial Treatment Code
    InitialTreatmentCode,
    ///ITI - Interruptible Transportation Indicator
    InterruptibleTransportationIndicator,
    ///J - Trade Code
    TradeCode,
    ///J0 - Summons Type Code
    SummonsTypeCode,
    ///J1 - Judicial Hearing Type Code
    JudicialHearingTypeCode,
    ///J2 - Judicial Order Type Code
    JudicialOrderTypeCode,
    ///J3 - Judicial Sentence Type Code
    JudicialSentenceTypeCode,
    ///J4 - Court Disposition Code
    CourtDispositionCode,
    ///J5 - Court Appearance Type Code
    CourtAppearanceTypeCode,
    ///J6 - Court Pleading Type Code
    CourtPleadingTypeCode,
    ///J7 - Defendant Plea Type Code
    DefendantPleaTypeCode,
    ///J8 - Trial Type Code
    TrialTypeCode,
    ///J9 - Court Case Status Code
    CourtCaseStatusCode,
    ///JA - Physical Characteristics Code
    PhysicalCharacteristicsCode,
    ///JB - Weight or Fragility Code
    WeightOrFragilityCode,
    ///JC - Preservation Material Code
    PreservationMaterialCode,
    ///JCL - Job Characteristics Lookup
    JobCharacteristicsLookup,
    ///JD - Quantity per Unit Pack Code
    QuantityPerUnitPackCode,
    ///JE - Preservation Data Code
    PreservationDataCode,
    ///JF - Packing Requirement Level A Code
    PackingRequirementLevelACode,
    ///JG - Packing Requirement Level B Code
    PackingRequirementLevelBCode,
    ///JH - Packing Requirement Level C Code
    PackingRequirementLevelCCode,
    ///JI - Intermediate Container Code
    IntermediateContainerCode,
    ///JK - Intermediate Container Quantity Code
    IntermediateContainerQuantityCode,
    ///JL - Special Marking Code
    SpecialMarkingCode,
    ///JM - Type and Cause Code
    TypeAndCauseCode,
    ///JN - Mission Impact Statement Code
    MissionImpactStatementCode,
    ///JO - International Standard Designation System for Teeth and Areas of the Oral Cavity
    InternationalStandardDesignationSystemForTeethAndAreasOfTheOralCavity,
    ///JOL - Job Originator Lookup
    JobOriginatorLookup,
    ///JP - Universal National Tooth Designation System
    UniversalNationalToothDesignationSystem,
    ///K - Property Underwriting Condition Code
    PropertyUnderwritingConditionCode,
    ///KA - Deficiency Cause
    DeficiencyCause,
    ///KB - Discrepancy
    Discrepancy,
    ///KC - Preventive Measure
    PreventiveMeasure,
    ///KD - Contractor Alert List Reason
    ContractorAlertListReason,
    ///KE - Quality Alert List Reason
    QualityAlertListReason,
    ///KF - Contractor Alert List Status
    ContractorAlertListStatus,
    ///KG - Nature of Buy
    NatureOfBuy,
    ///KH - Type of Procurement
    TypeOfProcurement,
    ///KI - Representative Buy Indicator
    RepresentativeBuyIndicator,
    ///KJ - Assured Delivery Indicator
    AssuredDeliveryIndicator,
    ///KK - Award Source
    AwardSource,
    ///KL - Termination
    Termination,
    ///KM - Patient Event Problem Code
    PatientEventProblemCode,
    ///KO - Method Evaluation Code
    MethodEvaluationCode,
    ///KP - Result Evaluation Code
    ResultEvaluationCode,
    ///KQ - Conclusion Evaluation Code
    ConclusionEvaluationCode,
    ///KS - Device Event Problem Code
    DeviceEventProblemCode,
    ///KT - Dose Form Code
    DoseFormCode,
    ///KU - Route Code
    RouteCode,
    ///KW - Report Source Code
    ReportSourceCode,
    ///KYL - Key Event Lookup
    KeyEventLookup,
    ///KZ - Adverse Event Code
    AdverseEventCode,
    ///L - Line Item Condition Code
    LineItemConditionCode,
    ///LA - Contract
    Contract,
    ///LB - Contractor Review List Status
    ContractorReviewListStatus,
    ///LC - Laboratory Test Condition Code
    LaboratoryTestConditionCode,
    ///LCF - Location Capacity Flow Indicator
    LocationCapacityFlowIndicator,
    ///LD - Collision Industry Electronic Commerce Association (CIECA) - Loss Category
    CodeLD,
    ///LE - Life/Annuity Service Features
    LifeAnnuityServiceFeatures,
    ///LF - Life/Annuity Product Code
    LifeAnnuityProductCode,
    ///LG - Location Code
    LocationCode,
    ///LH - Basis of Jurisdiction Code
    BasisOfJurisdictionCode,
    ///LIN - Line of Authority
    LineOfAuthority,
    ///LJ - Principal Party Citizenship Code
    PrincipalPartyCitizenshipCode,
    ///LK - Nature of Suit Code
    NatureOfSuitCode,
    ///LM - Case Origin Code
    CaseOriginCode,
    ///LMT - Limit Type
    LimitType,
    ///LN - Line of business code
    LineOfBusinessCode,
    ///LO - Letter of Recommendation Rating Category
    LetterOfRecommendationRatingCategory,
    ///LOC - Location Indicator
    LocationIndicator,
    ///LOI - Logical Observation Identifier Names and Codes (LOINC) Codes
    CodeLOI,
    ///LP - Deficiency Indicator
    DeficiencyIndicator,
    ///LPC - Location Purpose Code
    LocationPurposeCode,
    ///LQ - Delinquency Indicator
    DelinquencyIndicator,
    ///LQT - Location Quantity Type Indicator
    LocationQuantityTypeIndicator,
    ///LR - Test Results Code
    TestResultsCode,
    ///LS - Loss Severity Code
    LossSeverityCode,
    ///LSC - Legal Structure Code
    LegalStructureCode,
    ///LT - Laboratory Results Identification Code
    LaboratoryResultsIdentificationCode,
    ///LZ - War Reserve Material Requirement Code
    WarReserveMaterialRequirementCode,
    ///M - Policy Type Code
    PolicyTypeCode,
    ///MA - Multi-Media Object
    MultiMediaObject,
    ///MAC - Material Management Aggregation Code
    MaterialManagementAggregationCode,
    ///MB - Service Contract Act Occupation Classification Code
    ServiceContractActOccupationClassificationCode,
    ///MC - Manual Class Code
    ManualClassCode,
    ///MCC - Material Control Code
    MaterialControlCode,
    ///MCD - Generator Set Mounting Code
    GeneratorSetMountingCode,
    ///ME - Device Evaluation Code
    DeviceEvaluationCode,
    ///MEC - Method of Completion Code
    MethodOfCompletionCode,
    ///MFD - Manager Forced Directed Action
    ManagerForcedDirectedAction,
    ///MI - Minority Indicator
    MinorityIndicator,
    ///MJ - Drug Status Code
    DrugStatusCode,
    ///MK - Drug Status Adverse Event Code
    DrugStatusAdverseEventCode,
    ///ML - Lot Type Code
    LotTypeCode,
    ///MN - Post Market Study Status Code
    PostMarketStudyStatusCode,
    ///MOC - Mechanization of Contract Administration Services (MOCAS) System Error Code
    CodeMOC,
    ///MOE - Major Organizational Entity Rule Number
    MajorOrganizationalEntityRuleNumber,
    ///MPP - List of codes identifying DoD Mapping Product Procurement Types.
    ListOfCodesIdentifyingDoDMappingProductProcurementTypes,
    ///MPT - List of codes identifying DoD Mapping Product Types.
    ListOfCodesIdentifyingDoDMappingProductTypes,
    ///MRC - Reference Partial Descriptive Method Reason Code
    ReferencePartialDescriptiveMethodReasonCode,
    ///MRI - Maximum Rate Indicator
    MaximumRateIndicator,
    ///MS - Meter Status
    MeterStatus,
    ///MT - Meter Type
    MeterType,
    ///N - Valuation Type Code
    ValuationTypeCode,
    ///NA - Plant Clearance Office Code
    PlantClearanceOfficeCode,
    ///NAC - Nomenclature Activity Classification Economy (NACE) Code
    CodeNAC,
    ///NAF - Norme Activite Francaise (NAF) Code
    CodeNAF,
    ///NAN - Non-Approved Item Name
    NonApprovedItemName,
    ///NAS - Nature of Suit
    NatureOfSuit,
    ///NB - Inventory Type Code
    InventoryTypeCode,
    ///NBA - No Balance Affecting Transactions
    NoBalanceAffectingTransactions,
    ///NC - Property Record Status Code
    PropertyRecordStatusCode,
    ///NCD - Invoice Notes Code
    InvoiceNotesCode,
    ///NCE - Nomination Capacity Exceeded Indicator
    NominationCapacityExceededIndicator,
    ///ND - Control Unit Design Code
    ControlUnitDesignCode,
    ///NDC - National Drug Code (NDC)
    CodeNDC,
    ///NE - Direct Numerical Control System Code
    DirectNumericalControlSystemCode,
    ///NF - Type Numerical Control System Code
    TypeNumericalControlSystemCode,
    ///NGC - National Geospatial-Intelligence Agency (NGA) Product Code
    CodeNGC,
    ///NH - Property Source Code
    PropertySourceCode,
    ///NI - Nature of Injury Code
    NatureOfInjuryCode,
    ///NIR - Non-Induction Reason Code
    NonInductionReasonCode,
    ///NJ - Uniform Freight Classification (UFC) Code
    CodeNJ,
    ///NK - National Motor Freight Classification (NMFC) Code
    CodeNK,
    ///NMS - List of DoD Nonconsumable Item Material Support Codes (NIMSC)
    CodeNMS,
    ///NP - Special Category Code
    SpecialCategoryCode,
    ///NPC - Automotive Aftermarket Industry Association (AAIA) National Popularity Code
    CodeNPC,
    ///NR - Excess Material Disposition Code
    ExcessMaterialDispositionCode,
    ///NS - Hazardous Material Code
    HazardousMaterialCode,
    ///NT - Type of Cargo Code
    TypeOfCargoCode,
    ///NUB - National Uniform Billing Committee (NUBC) Revenue Codes
    CodeNUB,
    ///O - Source of Loss Code
    SourceOfLossCode,
    ///O1 - Office of Worker's Compensation Programs (OWCP) Source of Injury Code
    CodeO1,
    ///O2 - Office of Worker's Compensation Programs (OWCP) Nature of Injury Code
    CodeO2,
    ///O3 - Office of Worker's Compensation Programs (OWCP) Part of Body Code
    CodeO3,
    ///O4 - Office of Worker's Compensation Programs (OWCP) Occupation Code
    CodeO4,
    ///OC - Occupation Code
    OccupationCode,
    ///P - Ceiling Type Code
    CeilingTypeCode,
    ///PA - Prior Damage Location Code
    PriorDamageLocationCode,
    ///PB - Part of Body Code
    PartOfBodyCode,
    ///PC - Collision Industry Electronic Commerce Association (CIECA) - Profile
    CodePC,
    ///PCL - Planning Code Lookup
    PlanningCodeLookup,
    ///PCR - Pseudo Closure Reason Code
    PseudoClosureReasonCode,
    ///PD - Professional Designation
    ProfessionalDesignation,
    ///PDA - Pre-determined Allocation (PDA) Transaction Type Code
    CodePDA,
    ///PGS - Petroleum United States Geological Survey (USGS) Formation Code
    CodePGS,
    ///PHC - Phrase Code
    PhraseCode,
    ///PI - Collision Industry Electronic Commerce Association (CIECA) - Points of Impact
    CodePI,
    ///PIT - Petroleum Bill Type
    PetroleumBillType,
    ///PL - Priority Lookup
    PriorityLookup,
    ///PLC - Petroleum Land Category
    PetroleumLandCategory,
    ///PLS - Petroleum Lease Status
    PetroleumLeaseStatus,
    ///POB - Part of Body and Nature of Injury
    PartOfBodyAndNatureOfInjury,
    ///POS - Place of Service Code
    PlaceOfServiceCode,
    ///PPD - Petroleum Product Disposition
    PetroleumProductDisposition,
    ///PPP - Petroleum Product Point-of-Sale
    PetroleumProductPointOfSale,
    ///PPS - Petroleum Product Selling Arrangement
    PetroleumProductSellingArrangement,
    ///PPV - Petroleum Product Value Adjustment
    PetroleumProductValueAdjustment,
    ///PQA - Petroleum Quantity Allocations Code
    PetroleumQuantityAllocationsCode,
    ///PR - International Classification of Diseases Clinical Modification (ICD-9-CM) Patient's Reason for Visit
    CodePR,
    ///PRA - Petroleum Royalty Adjustment
    PetroleumRoyaltyAdjustment,
    ///PRC - Petroleum Royalty Calculation Method
    PetroleumRoyaltyCalculationMethod,
    ///PRI - Processing Rights Indicator
    ProcessingRightsIndicator,
    ///PRR - Petroleum Regulatory Report
    PetroleumRegulatoryReport,
    ///PRT - Petroleum Royalty Transaction
    PetroleumRoyaltyTransaction,
    ///PS - Professional Status Code
    ProfessionalStatusCode,
    ///PT - Price Tier
    PriceTier,
    ///PWA - Petroleum Well Action
    PetroleumWellAction,
    ///PWI - Petroleum Well Information
    PetroleumWell,
    ///PWR - Petroleum Well Shut-In Reason
    PetroleumWellShutInReason,
    ///PWS - Petroleum Well Classification Status
    PetroleumWellClassificationStatus,
    ///PWT - Petroleum Well Test Information
    PetroleumWellTest,
    ///Q - Surface Descriptor Code
    SurfaceDescriptorCode,
    ///QA - Response Status Code
    ResponseStatusCode,
    ///QB - Business Entity Filing Report Type Code
    BusinessEntityFilingReportTypeCode,
    ///QC - Business Entity Filing Detail Code
    BusinessEntityFilingDetailCode,
    ///QDR - Product Quality Deficiency Report Summary Code
    ProductQualityDeficiencyReportSummaryCode,
    ///QE - Domestic Line of Business Code
    DomesticLineOfBusinessCode,
    ///QF - Foreign Line of Business Code
    ForeignLineOfBusinessCode,
    ///QG - Business Entity Filing Status Code
    BusinessEntityFilingStatusCode,
    ///QH - Business Entity Filing Securities Information Code
    BusinessEntityFilingSecuritiesInformationCode,
    ///QI - Business Entity Financial Information Code
    BusinessEntityFinancialInformationCode,
    ///QJ - Business Entity Status Code
    BusinessEntityStatusCode,
    ///QK - Business Entity Filing Location Code
    BusinessEntityFilingLocationCode,
    ///QS - Query Status
    QueryStatus,
    ///QT - Quantity Type Indicator
    QuantityTypeIndicator,
    ///R - Coverage Modifier
    CoverageModifier,
    ///R1 - Upstream Rank (Priority)
    CodeR1,
    ///R2 - Receipt Rank (Priority)
    CodeR2,
    ///R3 - Delivery Rank (Priority)
    CodeR3,
    ///R4 - Downstream Rank (Priority)
    CodeR4,
    ///R5 - Threaded Rank
    ThreadedRank,
    ///RA - Religious Affiliation Code
    ReligiousAffiliationCode,
    ///RAS - Receipt Acceptance Site Code
    ReceiptAcceptanceSiteCode,
    ///RC - Requirement Code
    RequirementCode,
    ///RCA - Registered Contractor Activity Code
    RegisteredContractorActivityCode,
    ///RD - Property Ownership Type Code
    PropertyOwnershipTypeCode,
    ///RE - Property Type Code
    PropertyTypeCode,
    ///REC - Race or Ethnicity Collection Code
    RaceOrEthnicityCollectionCode,
    ///RED - Reduction Reason Code
    ReductionReasonCode,
    ///REN - Association of American Railroads Rate EDI Network Error Code
    AssociationOfAmericanRailroadsRateEdiNetworkErrorCode,
    ///RET - Classification of Race or Ethnicity
    ClassificationOfRaceOrEthnicity,
    ///RFC - Reference Number Format Code
    ReferenceNumberFormatCode,
    ///RFM - Reason for Movement Code
    ReasonForMovementCode,
    ///RI - Residency Indicator
    ResidencyIndicator,
    ///RJC - Reference Number Justification Code
    ReferenceNumberJustificationCode,
    ///RM - Insurance Industry Specific Remark Codes
    InsuranceIndustrySpecificRemarkCodes,
    ///RNC - Reference Number Category Code
    ReferenceNumberCategoryCode,
    ///RPQ - Replenishment Demand Information
    ReplenishmentDemand,
    ///RQ - Testing Service Question Code List
    TestingServiceQuestionCodeList,
    ///RQI - Retail Demand Information
    RetailDemand,
    ///RRC - Reason for Reversal Code
    ReasonForReversalCode,
    ///RSS - Receipt Scheduling Status
    ReceiptSchedulingStatus,
    ///RT - Request Type
    RequestType,
    ///RTC - Registration Type Code
    RegistrationTypeCode,
    ///RUM - Refrigeration Unit Operating Mode Code
    RefrigerationUnitOperatingModeCode,
    ///RVC - Reference Number Variation Code
    ReferenceNumberVariationCode,
    ///RX - National Council for Prescription Drug Programs Reject Codes
    NationalCouncilForPrescriptionDrugProgramsRejectCodes,
    ///S - Society for Worldwide Interbank Financial Telecommunications (SWIFT)
    CodeS,
    ///SA - Student Activity Type Code
    StudentActivityTypeCode,
    ///SAD - Security Assistance Document Number Requirement Type Code
    SecurityAssistanceDocumentNumberRequirementTypeCode,
    ///SAT - Stock Action/Technical Information Code
    StockActionTechnicalInformationCode,
    ///SB - Student Award Code
    StudentAwardCode,
    ///SBA - Statistic Bundes Amt (SBA) Code
    CodeSBA,
    ///SC - Source
    Source,
    ///SCI - Subsequent Cycle Indicator
    SubsequentCycleIndicator,
    ///SD - International Classification of Diseases Clinical Modification (ICD-9-CM) Secondary Diagnosis
    CodeSD,
    ///SE - Sound Code
    SoundCode,
    ///SEC - Stock Exchange Code
    StockExchangeCode,
    ///SET - Settlement Type
    SettlementType,
    ///SF - Run Type
    RunType,
    ///SFO - Swing Fuel Option Indicator
    SwingFuelOptionIndicator,
    ///SG - Source of Deposit Code
    SourceOfDepositCode,
    ///SH - Source of Lead Code
    SourceOfLeadCode,
    ///SHL - Safety Hazard Lookup
    SafetyHazardLookup,
    ///SHM - Accident Resulting Change Code
    AccidentResultingChangeCode,
    ///SHN - Active Mitigation Consideration Code
    ActiveMitigationConsiderationCode,
    ///SHO - Activity Methods Code
    ActivityMethodsCode,
    ///SHP - Analytical Method Code
    AnalyticalMethodCode,
    ///SHQ - Atmospheric Stability Class Code
    AtmosphericStabilityClassCode,
    ///SHR - Basis of Estimate Code
    BasisOfEstimateCode,
    ///SHS - Certification Code
    CertificationCode,
    ///SHT - Contributing Factor Code
    ContributingFactorCode,
    ///SHU - Control Device Type Code
    ControlDeviceTypeCode,
    ///SHV - Design/Standard Code
    DesignStandardCode,
    ///SHW - Device Classification Code
    DeviceClassificationCode,
    ///SHX - Discharge Indicator Code
    DischargeIndicatorCode,
    ///SHY - Discharge Quantity Range Code
    DischargeQuantityRangeCode,
    ///SHZ - Non-Reportable Discharge Indicator Code
    NonReportableDischargeIndicatorCode,
    ///SI - SIC (Standard Industrial Classification)
    CodeSI,
    ///SIA - Emergency Response Regulation/Statute Code
    EmergencyResponseRegulationStatuteCode,
    ///SIB - Emission Factor Type Code
    EmissionFactorTypeCode,
    ///SIC - Emission Release Point Type Code
    EmissionReleasePointTypeCode,
    ///SID - Emission Source Type Code
    EmissionSourceTypeCode,
    ///SIE - Emission Type Code
    EmissionTypeCode,
    ///SIF - Emission Unit Type Code
    EmissionUnitTypeCode,
    ///SIG - Endpoint Code
    EndpointCode,
    ///SIH - Environment Code
    EnvironmentCode,
    ///SII - Environmental Program Type Code
    EnvironmentalProgramTypeCode,
    ///SIJ - Environmental Receptor Code
    EnvironmentalReceptorCode,
    ///SIK - Facility Category Code
    FacilityCategoryCode,
    ///SIL - Facility Status Code
    FacilityStatusCode,
    ///SIM - Factor Calculation Method Code
    FactorCalculationMethodCode,
    ///SIN - Frequency of Analysis Code
    FrequencyOfAnalysisCode,
    ///SIO - Generator Status Code
    GeneratorStatusCode,
    ///SIP - Geometric Type Code
    GeometricTypeCode,
    ///SIQ - Hazardous Waste Form Code
    HazardousWasteFormCode,
    ///SIR - Horizontal Datum Code
    HorizontalDatumCode,
    ///SIS - Information System Code
    InformationSystemCode,
    ///SIT - Initiating Event Code
    InitiatingEventCode,
    ///SIU - Inventory Quantity Range Code
    InventoryQuantityRangeCode,
    ///SIV - Latitude/Longitude Source Code
    LatitudeLongitudeSourceCode,
    ///SIW - Latitude/Longitude Verification Code
    LatitudeLongitudeVerificationCode,
    ///SIX - Location Description Code
    LocationDescriptionCode,
    ///SIY - Major Hazard Code
    MajorHazardCode,
    ///SIZ - Manufacturing Code
    ManufacturingCode,
    ///SJ - Source of Injury Code
    SourceOfInjuryCode,
    ///SJA - Material Classification Code
    MaterialClassificationCode,
    ///SJB - Material Code
    MaterialCode,
    ///SJC - Maximum Achievable Control Technology Code
    MaximumAchievableControlTechnologyCode,
    ///SJD - Method of Collection Code
    MethodOfCollectionCode,
    ///SJE - Mitigation System Code
    MitigationSystemCode,
    ///SJF - Model Used Code
    ModelUsedCode,
    ///SJG - Monitoring/Detection System Code
    MonitoringDetectionSystemCode,
    ///SJH - Monitoring Location Code
    MonitoringLocationCode,
    ///SJI - Non-Generating Waste Code
    NonGeneratingWasteCode,
    ///SJJ - Off-Site Availability Code
    OffSiteAvailabilityCode,
    ///SJK - Off-Site Impact Code
    OffSiteImpactCode,
    ///SJL - On-Site Impact Code
    OnSiteImpactCode,
    ///SJM - On-Site Process System Type Code
    OnSiteProcessSystemTypeCode,
    ///SJN - Origin Code
    OriginCode,
    ///SJO - Parameter Code
    ParameterCode,
    ///SJP - Passive Mitigation Consideration Code
    PassiveMitigationConsiderationCode,
    ///SJQ - Permit Compliance Status Code
    PermitComplianceStatusCode,
    ///SJR - Physical State Code
    PhysicalStateCode,
    ///SJS - Point of Measurement Code
    PointOfMeasurementCode,
    ///SJT - Preservative Code
    PreservativeCode,
    ///SJU - Process Code
    ProcessCode,
    ///SJV - Process Control Code
    ProcessControlCode,
    ///SJW - Process Hazard Analysis Update Resulting Change Code
    ProcessHazardAnalysisUpdateResultingChangeCode,
    ///SJX - Process Hazards Analysis Technique Code
    ProcessHazardsAnalysisTechniqueCode,
    ///SJY - Public Receptor Code
    PublicReceptorCode,
    ///SJZ - Range of Concentration Code
    RangeOfConcentrationCode,
    ///SKA - Recovery Method Code
    RecoveryMethodCode,
    ///SKB - Recycling Method Code
    RecyclingMethodCode,
    ///SKC - Release Event Code
    ReleaseEventCode,
    ///SKD - Release Source Code
    ReleaseSourceCode,
    ///SKE - Reliability Indicator Code
    ReliabilityIndicatorCode,
    ///SKF - Rule Effectiveness Method Code
    RuleEffectivenessMethodCode,
    ///SKG - Sample Type Code
    SampleTypeCode,
    ///SKH - Scenario Code
    ScenarioCode,
    ///SKI - Site Location Code
    SiteLocationCode,
    ///SKJ - Source Category Code
    SourceCategoryCode,
    ///SKK - Source of Waste Generation Code
    SourceOfWasteGenerationCode,
    ///SKL - Source Reduction Activity Code
    SourceReductionActivityCode,
    ///SKM - System Type Code
    SystemTypeCode,
    ///SKN - Time Period Code
    TimePeriodCode,
    ///SKO - Topography Code
    TopographyCode,
    ///SKP - Transfer Quantity Range Code
    TransferQuantityRangeCode,
    ///SKQ - Type of Competency Testing Code
    TypeOfCompetencyTestingCode,
    ///SKR - Type of Training Code
    TypeOfTrainingCode,
    ///SKS - Type of Waste Management Code
    TypeOfWasteManagementCode,
    ///SKT - Use Code
    UseCode,
    ///SKU - Waste Emanation Code
    WasteEmanationCode,
    ///SKV - Waste Management Status Code
    WasteManagementStatusCode,
    ///SKW - Waste Stream Code
    WasteStreamCode,
    ///SKX - Waste Treatment Method Code
    WasteTreatmentMethodCode,
    ///SKY - Wind Direction Code
    WindDirectionCode,
    ///SKZ - Unit of Measure Code
    UnitOfMeasureCode,
    ///SL - Secondary Source of Injury
    SecondarySourceOfInjury,
    ///SLA - Shelf-Life Action Code
    ShelfLifeActionCode,
    ///SLC - Stockage List Code
    StockageListCode,
    ///SLS - Scheduling Status
    SchedulingStatus,
    ///SMB - Statement Basis
    StatementBasis,
    ///SMC - Special Material Content Code
    SpecialMaterialContentCode,
    ///SMD - Sample Device
    SampleDevice,
    ///SMI - Special Material Identification Code
    SpecialMaterialIdentificationCode,
    ///SMT - Sample Type
    SampleType,
    ///SO - Solicitation Cancellation Reason
    SolicitationCancellationReason,
    ///SP - Standard Occupation Classification Code
    StandardOccupationClassificationCode,
    ///SPD - Submitter's Priority Designator
    SubmittersPriorityDesignator,
    ///SPE - Special Dating
    SpecialDating,
    ///SR - Statistical Administrative Information Code
    StatisticalAdministrativeInformationCode,
    ///SRA - Service Requester Agent Indicator
    ServiceRequesterAgentIndicator,
    ///SRC - List of codes identifying DoD Serialized Report Type Codes.
    ListOfCodesIdentifyingDoDSerializedReportTypeCodes,
    ///SRL - Special Requirement Lookup
    SpecialRequirementLookup,
    ///SRR - Supplemental Reduction Reason
    SupplementalReductionReason,
    ///SRT - Storage Report Type
    StorageReportType,
    ///SS - System Status
    SystemStatus,
    ///SSC - Supply Status Code
    SupplyStatusCode,
    ///ST - Special Marketing Type Code
    SpecialMarketingTypeCode,
    ///STC - Association of American Railroads Standard Transportation Commodity Code Master Description Information
    AssociationOfAmericanRailroadsStandardTransportationCommodityCodeMasterDescription,
    ///STF - Forward and Store Application Error Edit Codes
    ForwardAndStoreApplicationErrorEditCodes,
    ///SUI - Solicited/Unsolicited Indicator
    SolicitedUnsolicitedIndicator,
    ///SVC - Service Code
    ServiceCode,
    ///SWR - Association of American Railroads Switch Release Codes
    AssociationOfAmericanRailroadsSwitchReleaseCodes,
    ///T - Personal Property and Contents Code
    PersonalPropertyAndContentsCode,
    ///T00 - Commercial Vehicle Operations Safety Code
    CommercialVehicleOperationsSafetyCode,
    ///T01 - Data Categories
    DataCategories,
    ///T02 - Event Codes
    EventCodes,
    ///T03 - Operation Type
    OperationType,
    ///T04 - Accident Parameters
    AccidentParameters,
    ///T05 - Inspection Parameters
    InspectionParameters,
    ///T06 - Driver Parameters
    DriverParameters,
    ///T07 - View Parameters
    ViewParameters,
    ///T08 - Vehicle Parameters
    VehicleParameters,
    ///T09 - Fleet Parameters
    FleetParameters,
    ///T10 - Query Options
    QueryOptions,
    ///T11 - Jurisdiction Type
    JurisdictionType,
    ///T12 - Single State Registration System and Operating Authority Credential
    SingleStateRegistrationSystemAndOperatingAuthorityCredential,
    ///T13 - Commercial Vehicle Operations Insurance
    CommercialVehicleOperationsInsurance,
    ///T14 - Commercial Vehicle Registration
    CommercialVehicleRegistration,
    ///T15 - Hazardous Materials Credential
    HazardousMaterialsCredential,
    ///T16 - Oversize/Overweight Credential
    OversizeOverweightCredential,
    ///T17 - Commercial Vehicle Tax
    CommercialVehicleTax,
    ///T18 - Commercial Vehicle Title
    CommercialVehicleTitle,
    ///T19 - Commercial Driver's License
    CommercialDriversLicense,
    ///T20 - Commercial Vehicle Type
    CommercialVehicleType,
    ///T21 - Commercial Vehicle Operations Status Code
    CommercialVehicleOperationsStatusCode,
    ///T22 - Safety and Fitness Electronic Record Systems Subscription Option
    SafetyAndFitnessElectronicRecordSystemsSubscriptionOption,
    ///T23 - Commercial Vehicle Operations Commodity Code
    CommercialVehicleOperationsCommodityCode,
    ///T24 - Commercial Vehicle Operations Hazardous Material Code
    CommercialVehicleOperationsHazardousMaterialCode,
    ///T25 - Safety and Fitness Electronic Record Systems Error Code
    SafetyAndFitnessElectronicRecordSystemsErrorCode,
    ///T26 - Commercial Vehicle Operations Jurisdiction Identifier Code
    CommercialVehicleOperationsJurisdictionIdentifierCode,
    ///T27 - Compliance Review Code
    ComplianceReviewCode,
    ///T28 - Incident Condition Code
    IncidentConditionCode,
    ///T29 - Incident Related Action Code
    IncidentRelatedActionCode,
    ///T30 - Incident Location Code
    IncidentLocationCode,
    ///T31 - Incident Consequence Code
    IncidentConsequenceCode,
    ///T32 - Road Characteristic Code
    RoadCharacteristicCode,
    ///T33 - Vehicle Occupant Code
    VehicleOccupantCode,
    ///T34 - Package Failure Code
    PackageFailureCode,
    ///T35 - Pedestrian Code
    PedestrianCode,
    ///TB - Association for Financial Professionals Service Code and Bank Service Code
    AssociationForFinancialProfessionalsServiceCodeAndBankServiceCode,
    ///TC - Treatment Codes
    TreatmentCodes,
    ///TC1 - Type of Change Code
    TypeOfChangeCode,
    ///TCD - Collision Industry Electronic Commerce Association (CIECA) - Totals Code List
    CodeTCD,
    ///TCL - Template Characteristic Lookup
    TemplateCharacteristicLookup,
    ///TD - International Classification of Diseases Clinical Modification (ICD-9-CM) Tertiary Diagnosis
    CodeTD,
    ///TDC - Discrepancy Report Type Code
    DiscrepancyReportTypeCode,
    ///TDJ - Technical Data Justification Code
    TechnicalDataJustificationCode,
    ///TE - Association for Financial Professionals Service Code
    AssociationForFinancialProfessionalsServiceCode,
    ///TF - Title Exception and Requirement Code List
    TitleExceptionAndRequirementCodeList,
    ///TFR - Tax or Fee Exemption Reason Code
    TaxOrFeeExemptionReasonCode,
    ///TG - Title Document Code List
    TitleDocumentCodeList,
    ///TL - Tap Location
    TapLocation,
    ///TOC - Weapon System Transaction Origination Code
    WeaponSystemTransactionOriginationCode,
    ///TOL - Template Owner Lookup
    TemplateOwnerLookup,
    ///TP - Tap Type
    TapType,
    ///TQ - Systemized Nomenclature of Dentistry (SNODENT)
    CodeTQ,
    ///TR - Report Code
    ReportCode,
    ///TT - Natural Gas Transaction Type
    NaturalGasTransactionType,
    ///TTD - Downstream Transaction Type
    DownstreamTransactionType,
    ///TTL - Template Type Lookup
    TemplateTypeLookup,
    ///TTU - Upstream Transaction Type
    UpstreamTransactionType,
    ///TX - Follow-up Code
    FollowUpCode,
    ///TY - Reportable Event Code
    ReportableEventCode,
    ///U - Residential and Commercial Room Code
    ResidentialAndCommercialRoomCode,
    ///UBT - UMLER Body Type
    UmlerBodyType,
    ///UDC - Unique Item Tracking Designator Code
    UniqueItemTrackingDesignatorCode,
    ///UER - Unique Item Tracking Error Reject Code
    UniqueItemTrackingErrorRejectCode,
    ///UFC - UMLER Fitting Code
    UmlerFittingCode,
    ///UJC - Urgency Justification Code
    UrgencyJustificationCode,
    ///UNP - (UN/SPSC) United Nations Products and Services Classification Code
    CodeUNP,
    ///UPC - Unclaimed Property Additions, Deletions, and Deductions Codes
    CodeUPC,
    ///UPT - Unclaimed Property Type Code
    UnclaimedPropertyTypeCode,
    ///UR - Event Reappearance Code
    EventReappearanceCode,
    ///US - Event Abatement Code
    EventAbatementCode,
    ///UT - Centers for Medicare and Medicaid Services (CMS) Certificate of Medical Necessity (CMN) forms
    CodeUT,
    ///UTC - Unique Item Tracking Transaction Code
    UniqueItemTrackingTransactionCode,
    ///UU - Unit Code
    UnitCode,
    ///V - Violation Type Code List
    ViolationTypeCodeList,
    ///VAL - Validation Code
    ValidationCode,
    ///VP - Collision Industry Electronic Commerce Association (CIECA) - Vehicle Line Item Category Code
    CodeVP,
    ///W - Court Issued Warrant Type Code
    CourtIssuedWarrantTypeCode,
    ///WAC - Weapon System Advice Code
    WeaponSystemAdviceCode,
    ///WDL - When Discovered Lookup
    WhenDiscoveredLookup,
    ///WEC - Weapon System Essentiality Code
    WeaponSystemEssentialityCode,
    ///WRC - Automotive Aftermarket Industry Association (AAIA) Warranty Code
    CodeWRC,
    ///WSC - Weapon System Status Code
    WeaponSystemStatusCode,
    ///WSD - Weapon System Designator Code
    WeaponSystemDesignatorCode,
    ///WSM - Weapon System Maintenance Code
    WeaponSystemMaintenanceCode,
    ///X - Vehicle Class
    VehicleClass,
    ///Y - Rental Charge
    RentalCharge,
    ///Z - Cancellation Reason
    CancellationReason,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl CodeListQualifierCode {
    pub fn code(&self) -> &str {
        {
            use CodeListQualifierCode::*;
            match self {
                DocumentIdentificationCode => "0",
                FreeOnBoardSiteCode => "1",
                ChannelOfDistributionCode => "2",
                KindOfContractCode => "3",
                TypeOfContractCode => "4",
                CriticalityDesignatorCode => "5",
                QualityAssuranceSiteCode => "7",
                AcceptanceSiteCode => "8",
                TransactionStatusIndicatorCode => "10",
                ContractDeliveryDateRevisionAgentCode => "11",
                ReasonForContractDeliveryDateRevisionCode => "12",
                RecommendationsRegardingDelayedDeliveriesCode => "13",
                ContractShipmentAdviceCode => "14",
                IndividualInsuranceFinancialDetail => "15",
                CashDiscountStipulationCode => "16",
                ShipmentAcceptanceDiscrepancyExplanationCode => "17",
                InsurancePlanDescriptionCharacteristics => "18",
                ContractCloseOutGroupCode => "19",
                PaymentTypeCode => "20",
                ContractFundReportingTransactionCode => "21",
                ContractPaymentDeductionOrCollectionCode => "22",
                ObligationVarianceCode => "23",
                PlusOrMinusIndicatorCode => "24",
                ReasonForDelayedClosingOfContractFileCode => "25",
                ContractPaymentLineItemStatusCode => "26",
                SpecialReimbursableProvisionsCode => "27",
                KindOfModificationCode => "28",
                Code29 => "29",
                TypeOfDelayCode => "30",
                HealthcareProviderCharacteristicsAndResources => "31",
                ContainerAndRollOnRollOffNumberCode => "32",
                AirCommodityAndSpecialHandlingCode => "33",
                WaterCommodityAndSpecialHandlingCode => "34",
                AirDimensionCode => "35",
                AirTerminalIdentifierCode => "36",
                WaterTerminalIdentifierCode => "37",
                ConsolidationAndContainerizationPointCode => "38",
                TransportationModeOrMethodCode => "39",
                TypePackCode => "40",
                DateShippedOrReceivedCode => "41",
                EstimatedTimeOfArrivalCode => "42",
                MilitaryAndCivilianGradeCode => "43",
                SeavanOwnershipCode => "44",
                OceanCarrierCode => "45",
                VoyageDocumentNumberCode => "46",
                VoyageManifestReferenceCode => "47",
                VesselStatusAndTermsOfCarriageCode => "48",
                VesselSustainingCode => "49",
                SubrogationActionCode => "50",
                BillingAdviceCode => "52",
                BillingStatusCode => "53",
                TypeOfBillCode => "54",
                RecipientOfBillingStatusCode => "55",
                SalesPriceConditionCode => "56",
                DeliverySourceCode => "57",
                TransportationBillCode => "58",
                StockFundOrNonStockFundCode => "59",
                Code60 => "60",
                InformationIndicatorCode => "61",
                CommunicationsRoutingIdentifierCode => "62",
                ContentIndicatorCode => "63",
                HealthCareClaimStatusCode => "65",
                SuffixOrLimitCode => "66",
                TypeOfAssistanceCode => "67",
                HealthcareProviderTaxonomy => "68",
                ForeignMilitarySalesCountryCode => "69",
                ServiceAndAgencyCode => "71",
                DisbursementStatusCode => "72",
                AidTypeCode => "73",
                DemandCode => "74",
                SuffixCode => "75",
                ProjectCode => "78",
                PriorityDesignatorCode => "79",
                AdviceCode => "80",
                StatusCode => "81",
                ShipmentHoldCode => "82",
                SupplyConditionCode => "83",
                ManagementCode => "84",
                CountryAndActivityCode => "85",
                SubsistenceTypeOfPackCode => "87",
                DisposalAuthorityCode => "88",
                CooperativeLogisticsProgramSupportCode => "89",
                PreciousMetalsIndicatorCode => "90",
                AutomatedDataProcessingEquipmentIdentificationCode => "91",
                ReasonForDisposalCode => "92",
                TypeOfStorageCode => "93",
                IdentificationCode => "94",
                OfferAndReleaseOptionCode => "95",
                ShipmentReleaseCode => "96",
                UltimateRecipientCode => "97",
                ReasonForRequisitioningCode => "98",
                PurposeCode => "99",
                Code100 => "100",
                Code101 => "101",
                MortgageIndexSourceCode => "102",
                Code103 => "103",
                Code104 => "104",
                Code105 => "105",
                Code106 => "106",
                Code107 => "107",
                TestingStatisticalCategoryCodeList => "108",
                TestingDemographicCategoryCodeList => "109",
                CodeA => "A",
                OwnershipCode => "A1",
                CustomerWithinCountryCode => "A2",
                DeliveryTermCode => "A3",
                CaseDesignatorNumber => "A4",
                SubcaseNumber => "A5",
                FreightForwarderNumber => "A6",
                RecordControlNumber => "A7",
                ProgramYearCode => "A8",
                SupplementalData => "A9",
                CodeAA => "AA",
                CodeAAA => "AAA",
                AssetType => "AAD",
                CurrentAssetType => "AAE",
                CurrentLiabilityType => "AAF",
                DunAndBradstreetCanadas8DigitStandardIndustrialClassificationCode => {
                    "AAG"
                }
                FinancialItemAllocationCode => "AAH",
                FinancialItemAttributedCode => "AAI",
                FinancialItemReclassificationCode => "AAJ",
                FunctionalArea => "AAK",
                HobbyCode => "AAL",
                InvestmentType => "AAM",
                LiabilityType => "AAN",
                ProjectionType => "AAO",
                TrendReason => "AAP",
                CodeAAQ => "AAQ",
                CodeAAR => "AAR",
                Proprietary => "AAS",
                FannieMaeAdjustableRateMortgagePlanCodes => "AAT",
                CodeAAU => "AAU",
                CodeAAV => "AAV",
                ImmunizationInjectionCode => "AAW",
                CodeAAX => "AAX",
                CodeAAY => "AAY",
                DefensePrioritiesAndAllocationsSystemCode => "AB",
                CodeABF => "ABF",
                CodeABJ => "ABJ",
                CodeABK => "ABK",
                CodeABN => "ABN",
                AssignedByReceiver => "ABR",
                AssignedBySender => "ABS",
                CodeABU => "ABU",
                CodeABV => "ABV",
                AccountCharacteristicsCode => "AC",
                AccountingErrorClassificationCode => "ACC",
                AcademicRank => "ACR",
                ListOfDoDAccountingRequirementsCodes => "ACS",
                ListOfDoDAssetCategoryCodes => "ACT",
                ListOfDoDControlledItemCodes => "ACU",
                CodeACV => "ACV",
                ListOfDoDFiduciaryDepreciationMethodCodes => "ACW",
                CodeACX => "ACX",
                CodeACY => "ACY",
                CodeACZ => "ACZ",
                AcquisitionAdviceCode => "AD",
                ListOfDoDStorageRequirementCodes => "AD1",
                ListOfDoDTemperatureControlledCodes => "AD2",
                ListOfDoDAssetTypeCodes => "AD3",
                ListOfDoDUtilizationMeasureCodes => "AD4",
                CodeADD => "ADD",
                AccountingAdjustmentMethod => "ADJ",
                BeneficiaryType => "AE",
                ArmyEditActionCode => "AEA",
                ClassOfPitch => "AF",
                GradeOfDifficulty => "AG",
                AcquisitionMethodSuffixCode => "AH",
                AcquisitionMethodCode => "AI",
                UtilizationCode => "AJ",
                AdjustmentType => "AJT",
                DistributionCode => "AK",
                SpecialRequirementsCode => "AL",
                AllocationMethod => "ALM",
                AlterationLookup => "ALP",
                LocaleOfActivity => "AM",
                NatureOfEventCode => "AN",
                SettlementPayoutOptions => "AO",
                AuthorizedOverrunIndicator => "AOR",
                CodeAPE => "APE",
                CodeAPR => "APR",
                ApplicationQuestion => "AQ",
                AllQuantityAvailableIndicator => "AQA",
                ArrestReason => "AR",
                AssetReclassificationDenialCode => "ARD",
                AllocationRankIndicator => "ARI",
                AllocationRankLevel => "ARL",
                FormTypeCode => "AS",
                CodeASD => "ASD",
                AllegationTypeCode => "AT",
                CodeATD => "ATD",
                AllocationTransactionTypeCode => "ATT",
                CodeAU => "AU",
                SubrogationPaymentOptions => "AV",
                CodeAW => "AW",
                CodeAX => "AX",
                SubrogationResponseCodes => "AY",
                SubrogationRequestCodes => "AZ",
                CodeB => "B",
                VesselStowageLocationCode => "BA",
                BusinessType => "BB",
                CodeBBF => "BBF",
                CodeBBJ => "BBJ",
                CodeBBK => "BBK",
                CodeBBN => "BBN",
                CodeBBQ => "BBQ",
                CodeBBR => "BBR",
                CodeBBU => "BBU",
                CodeBBV => "BBV",
                TransportationHoldingDelayCode => "BC",
                BusinessChangeCode => "BCC",
                BusinessCreditRating => "BCR",
                TransportationPriorityCode => "BD",
                CodeBDD => "BDD",
                Value => "BE",
                CodeBF => "BF",
                Condition => "BG",
                Occurrence => "BH",
                OccurrenceSpan => "BI",
                CodeBJ => "BJ",
                CodeBK => "BK",
                ApplicationFeeStatusCodes => "BL",
                CodeBN => "BN",
                HealthcareCommonProcedureCodingSystem => "BO",
                HealthcareCommonProcedureCodingSystemPrincipalProcedure => "BP",
                BoardOfInspectionAndSurveyPartLookup => "BPL",
                CodeBPR => "BPR",
                CodeBQ => "BQ",
                CodeBR => "BR",
                BoardOfInspectionAndSurveyResponsibilityLookup => "BRL",
                CodeBS => "BS",
                CodeBSD => "BSD",
                BoardOfInspectionAndSurveyShipLookup => "BSL",
                BusinessPeriod => "BSP",
                AccidentDescription => "BT",
                BalanceTypeCode => "BTC",
                CodeBTD => "BTD",
                PartOfBodyAffected => "BU",
                BidUpIndicator => "BUI",
                BureauOfLaborStatisticsStandardizedOccupationalCodes => "BUR",
                EducationInstitutionTypeCode => "BV",
                EducationalAreasCode => "BW",
                ProfessionTypeCode => "BX",
                ShareTypeCode => "BY",
                BusinessSizeCode => "BZ",
                CodeC => "C",
                EyeColorCode => "C1",
                HairColorCode => "C2",
                SkinToneCode => "C3",
                TypeOfInquiryCode => "CA",
                CodeCAH => "CAH",
                BilledOfficeIndicatorCode => "CB",
                CodeCBQ => "CBQ",
                CodeCBR => "CBR",
                TreasurySymbolCode => "CC",
                CorrectionToCauseCode => "CCC",
                SupplementaryAccountingClassificationCode => "CD",
                ReferenceAndStationCode => "CE",
                MajorForceProgramCode => "CF",
                ContractualFlowIndicator => "CFI",
                AircraftMissionDesignSeriesCode => "CG",
                TypeOfIssueCode => "CH",
                ChargeIndicator => "CHG",
                CriminalCharge => "CI",
                CodeCIE => "CIE",
                CriminalChargeGrade => "CJ",
                CouponAdjustmentReasonCode => "CK",
                CountyDesignatorCode => "CL",
                CauseLookup => "CLP",
                FinancialManagementServiceCashLinkCode => "CM",
                CustomerMaintenanceLevelLookup => "CML",
                CauseOfInjuryCode => "CN",
                ChangeNoticeCode => "CNC",
                CustomizedNoticeTypeCode => "CO",
                CognizanceSymbol => "COG",
                ConfirmingPartyRole => "COR",
                SalvageDispositionCode => "CP",
                CourtPartyStatus => "CPS",
                CapacityTypeIndicator => "CQ",
                CodeCR => "CR",
                ComplaintRequestCode => "CRC",
                CausativeResearchIndicatorCode => "CRI",
                ClauseStatusType => "CS",
                CustomerServiceDesignator => "CSD",
                CorporateStatementFilingCode => "CSF",
                CompensationTypeCodes => "CT",
                CarcassTrackingCode => "CTC",
                CuisineTypeCode => "CU",
                CoverageCodeList => "CV",
                ControvertCode => "CW",
                ConvictionOffenseType => "CZ",
                CourtDocumentTypeCode => "D",
                DriversLicenseWithdrawalType => "D1",
                DriversLicenseWithdrawalExtent => "D2",
                DriversLicenseWithdrawalBasis => "D3",
                DriversLicenseWithdrawalDueProcessStatus => "D4",
                DriversLicenseWithdrawalReason => "D5",
                DeviceAvailabilityCode => "DA",
                DocumentAvailabilityCode => "DAC",
                CodeDAP => "DAP",
                DebtorBusinessTypeCode => "DB",
                CodeDBS => "DBS",
                ReportDistributionCode => "DC",
                CauseCode => "DCC",
                CodeDCM => "DCM",
                DispositionCategoryChangeRejectReasonCode => "DCR",
                DispositionSubCategoryCode => "DCS",
                DispositionCategoryCode => "DCT",
                CodeDD => "DD",
                SignalCode => "DE",
                MediaAndStatusCode => "DF",
                FundCode => "DG",
                DynamicGeneratorSetCode => "DGO",
                DrugDetailCode => "DH",
                SingleUseLabelCode => "DI",
                CodeDIR => "DIR",
                RemedialActionCode => "DJ",
                ProgramOriginatorCode => "DK",
                ServiceContractActOperationCode => "DL",
                DynamicLocomotiveTagCode => "DLO",
                DeferralLookup => "DLP",
                LongTermCareDrgLtcDrg => "DLT",
                AgentStatusCode => "DM",
                DemilitarizationIntegrityCode => "DMI",
                DemilitarizationPerformedCode => "DMP",
                NatureOfDebtCode => "DN",
                DocumentNumberRequirementType => "DNT",
                DeviceOperatorTypeCode => "DO",
                DirectionOfFlow => "DOF",
                ProducerFinancialHistoryCodes => "DP",
                DeliveryPriorityCode => "DPC",
                AssociationOfAmericanRailroadsDeprescriptionExceptionList => "DPE",
                AssociationOfAmericanRailroadsDeprescriptionDistributionList => "DPL",
                DeviceStatusCode => "DQ",
                CodeDR => "DR",
                CodeDRD => "DRD",
                CodeDRL => "DRL",
                DispositionServicesReimbursementCode => "DRS",
                RelatedDeviceApplicabilityCode => "DS",
                DispositionServicesCustomerTypeCode => "DSC",
                CodeDSD => "DSD",
                DispositionServicesIndicatorCode => "DSI",
                DataSetsRequested => "DSR",
                DeliverySchedulingStatus => "DSS",
                DebtorTypeCode => "DT",
                ListOfCodesIdentifyingDoDDistributionServicesTermsOfSale => "DTS",
                DeviceUsageCode => "DU",
                DemandPlanningStatusCode => "DV",
                EstimatingMethodStatusCode => "DW",
                ContactStatusCode => "DX",
                TypeOfFirmCode => "DY",
                ReportableEventStatusCode => "DZ",
                CodeE => "E",
                AssetStatusOrTransactionReportingCode => "EA",
                AlabamaCampaignDisclosureReportCodes => "EAA",
                AlaskaCampaignDisclosureReportCodes => "EAB",
                AmericanSamoaCampaignDisclosureReportCodes => "EAC",
                ArizonaCampaignDisclosureReportCodes => "EAD",
                ArkansasCampaignDisclosureReportCodes => "EAE",
                CaliforniaCampaignDisclosureReportCodes => "EAF",
                ColoradoCampaignDisclosureReportCodes => "EAG",
                ConnecticutCampaignDisclosureReportCodes => "EAH",
                DelawareCampaignDisclosureReportCodes => "EAI",
                DistrictOfColumbiaCampaignDisclosureReportCodes => "EAJ",
                FloridaCampaignDisclosureReportCodes => "EAK",
                GeorgiaCampaignDisclosureReportCodes => "EAL",
                GuamCampaignDisclosureReportCodes => "EAM",
                HawaiiCampaignDisclosureReportCodes => "EAN",
                IdahoCampaignDisclosureReportCodes => "EAO",
                IllinoisCampaignDisclosureReportCodes => "EAP",
                IndianaCampaignDisclosureReportCodes => "EAQ",
                IowaCampaignDisclosureReportCodes => "EAR",
                KansasCampaignDisclosureReportCodes => "EAS",
                KentuckyCampaignDisclosureReportCodes => "EAT",
                LouisianaCampaignDisclosureReportCodes => "EAU",
                MaineCampaignDisclosureReportCodes => "EAV",
                MarylandCampaignDisclosureReportCodes => "EAW",
                MassachusettsCampaignDisclosureReportCodes => "EAX",
                MichiganCampaignDisclosureReportCodes => "EAY",
                MinnesotaCampaignDisclosureReportCodes => "EAZ",
                AssetTransferStatusCode => "EB",
                MississippiCampaignDisclosureReportCodes => "EBA",
                MissouriCampaignDisclosureReportCodes => "EBB",
                MontanaCampaignDisclosureReportCodes => "EBC",
                NebraskaCampaignDisclosureReportCodes => "EBD",
                NevadaCampaignDisclosureReportCodes => "EBE",
                NewHampshireCampaignDisclosureReportCodes => "EBF",
                NewJerseyCampaignDisclosureReportCodes => "EBG",
                NewMexicoCampaignDisclosureReportCodes => "EBH",
                NewYorkCampaignDisclosureReportCodes => "EBI",
                NorthCarolinaCampaignDisclosureReportCodes => "EBJ",
                NorthDakotaCampaignDisclosureReportCodes => "EBK",
                OhioCampaignDisclosureReportCodes => "EBL",
                OklahomaCampaignDisclosureReportCodes => "EBM",
                OregonCampaignDisclosureReportCodes => "EBN",
                PennsylvaniaCampaignDisclosureReportCodes => "EBO",
                PuertoRicoCampaignDisclosureReportCodes => "EBP",
                RhodeIslandCampaignDisclosureReportCodes => "EBQ",
                SouthCarolinaCampaignDisclosureReportCodes => "EBR",
                SouthDakotaCampaignDisclosureReportCodes => "EBS",
                TennesseeCampaignDisclosureReportCodes => "EBT",
                TexasCampaignDisclosureReportCodes => "EBU",
                UtahCampaignDisclosureReportCodes => "EBV",
                VermontCampaignDisclosureReportCodes => "EBW",
                VirginiaCampaignDisclosureReportCodes => "EBX",
                VirginIslandsCampaignDisclosureReportCodes => "EBY",
                WashingtonCampaignDisclosureReportCodes => "EBZ",
                CertificationRequirementsCode => "EC",
                WestVirginiaCampaignDisclosureReportCodes => "ECA",
                WisconsinCampaignDisclosureReportCodes => "ECB",
                WyomingCampaignDisclosureReportCodes => "ECC",
                AlbertaCampaignDisclosureReportCodes => "ECD",
                BritishColumbiaCampaignDisclosureReportCodes => "ECE",
                ManitobaCampaignDisclosureReportCodes => "ECF",
                NewBrunswickCampaignDisclosureReportCodes => "ECG",
                NewfoundlandCampaignDisclosureReportCodes => "ECH",
                NorthwestTerritoriesCampaignDisclosureReportCodes => "ECI",
                NovaScotiaCampaignDisclosureReportCodes => "ECJ",
                OntarioCampaignDisclosureReportCodes => "ECK",
                PrinceEdwardIslandCampaignDisclosureReportCodes => "ECL",
                QuebecCampaignDisclosureReportCodes => "ECM",
                SaskatchewanCampaignDisclosureReportCodes => "ECN",
                YukonTerritoryCampaignDisclosureReportCodes => "ECO",
                FederalCampaignDisclosureReportCodes => "ECP",
                AlabamaLobbyistReportCodes => "ECQ",
                AlaskaLobbyistReportCodes => "ECR",
                ArizonaLobbyistReportCodes => "ECS",
                ArkansasLobbyistReportCodes => "ECT",
                CaliforniaLobbyistReportCodes => "ECU",
                ColoradoLobbyistReportCodes => "ECV",
                ConnecticutLobbyistReportCodes => "ECW",
                DelawareLobbyistReportCodes => "ECX",
                DistrictOfColumbiaLobbyistReportCodes => "ECY",
                FloridaLobbyistReportCodes => "ECZ",
                CoastDesignationCode => "ED",
                GeorgiaLobbyistReportCodes => "EDA",
                HawaiiLobbyistReportCodes => "EDB",
                IdahoLobbyistReportCodes => "EDC",
                IllinoisLobbyistReportCodes => "EDD",
                IndianaLobbyistReportCodes => "EDE",
                IowaLobbyistReportCodes => "EDF",
                KansasLobbyistReportCodes => "EDG",
                KentuckyLobbyistReportCodes => "EDH",
                LouisianaLobbyistReportCodes => "EDI",
                MaineLobbyistReportCodes => "EDJ",
                MarylandLobbyistReportCodes => "EDK",
                MassachusettsLobbyistReportCodes => "EDL",
                MichiganLobbyistReportCodes => "EDM",
                MinnesotaLobbyistReportCodes => "EDN",
                MississippiLobbyistReportCodes => "EDO",
                MissouriLobbyistReportCodes => "EDP",
                MontanaLobbyistReportCodes => "EDQ",
                NebraskaLobbyistReportCodes => "EDR",
                NevadaLobbyistReportCodes => "EDS",
                NewHampshireLobbyistReportCodes => "EDT",
                NewJerseyLobbyistReportCodes => "EDU",
                NewMexicoLobbyistReportCodes => "EDV",
                NewYorkLobbyistReportCodes => "EDW",
                NorthCarolinaLobbyistReportCodes => "EDX",
                NorthDakotaLobbyistReportCodes => "EDY",
                OhioLobbyistReportCodes => "EDZ",
                CompetitiveCharacteristicsCode => "EE",
                OklahomaLobbyistReportCodes => "EEA",
                OregonLobbyistReportCodes => "EEB",
                PennsylvaniaLobbyistReportCodes => "EEC",
                PuertoRicoLobbyistReportCodes => "EED",
                RhodeIslandLobbyistReportCodes => "EEE",
                SouthCarolinaLobbyistReportCodes => "EEF",
                SouthDakotaLobbyistReportCodes => "EEG",
                TennesseeLobbyistReportCodes => "EEH",
                TexasLobbyistReportCodes => "EEI",
                UtahLobbyistReportCodes => "EEJ",
                VermontLobbyistReportCodes => "EEK",
                VirginiaLobbyistReportCodes => "EEL",
                WashingtonLobbyistReportCodes => "EEM",
                WestVirginiaLobbyistReportCodes => "EEN",
                WisconsinLobbyistReportCodes => "EEO",
                WyomingLobbyistReportCodes => "EEP",
                NewYorkCityCampaignDisclosureReportCodes => "EEQ",
                SeattleCampaignDisclosureReportCodes => "EER",
                NewYorkCityLobbyistReportCodes => "EES",
                CorrectionOrChangeForStorageItemRecordsCode => "EF",
                ExcavationInformationCodeList => "EG",
                TypeDueInIndicator => "EH",
                DiscrepancyIndicatorCode => "EI",
                DisposalConditionCode => "EJ",
                EventOrExposureCode => "EK",
                ErrorClassificationCode => "EL",
                InventoryCategoryCode => "EM",
                CodeEMC => "EMC",
                LocalSourceCode => "EN",
                AdverseEventOutcomeCode => "EO",
                CodeEP => "EP",
                ExchangePriceIndicator => "EPI",
                ControlledInventoryItemCode => "EQ",
                EquipmentRequestCodes => "EQR",
                DepartmentOfDefenseIdentificationCode => "ER",
                EquipmentRepairConditionCode => "ERC",
                EquipmentRepairJobCode => "ERJ",
                EquipmentRepairLocationCode => "ERL",
                EquipmentRepairResponsibilityCode => "ERR",
                ExtensionReason => "ES",
                ElectrostaticDischargeCode => "ESC",
                EquipmentStatusLookup => "ESL",
                RejectAdviceCode => "ET",
                EstimateTypeLookup => "ETL",
                RequestCode => "EU",
                ReviewPeriodIndicatorCode => "EV",
                SmallArmsErrorTransactionRejectCode => "EW",
                EvaluateWorkCandidateLookup => "EWC",
                EquipmentWhyMadeCode => "EWM",
                EvaluateWorkCandidateReasonLookup => "EWR",
                SmallArmsTransactionCode => "EX",
                ExportDeclaration => "EXD",
                CodeEXP => "EXP",
                SpecialProgramRequirementStatusCode => "EY",
                TypeInspectionCode => "EZ",
                FinancialRating => "F",
                TypeOfContractorCode => "FA",
                FannieMaeRefinancePlanCode => "FAP",
                TypeOfMediaCode => "FB",
                TypePhysicalInventoryOrTransactionHistoryCode => "FC",
                CodeFC1 => "FC1",
                DemilitarizationCode => "FD",
                ShelfLifeCode => "FE",
                EssentialityCode => "FF",
                FederalFinanceCodeList1 => "FF1",
                SourceMaintenanceAndRecoverabilityCode => "FG",
                TypeOfLocationReconciliationRequest => "FH",
                FederalHealthCareCodeList1 => "FH1",
                ApplicantType => "FI",
                FinancialInventoryReportCode => "FIR",
                AntennaStructureType => "FJ",
                StationClassification => "FK",
                RadioFrequencyType => "FL",
                FederalLogisticsCodeList1 => "FL1",
                StationClassificationType => "FM",
                FormerMajorOrganizationalEntityRuleNumber => "FMO",
                ForeignMilitarySalesAndMilitaryAssistanceProgramGrantAidTypeOfAssistanceFinancingCode => {
                    "FMS"
                }
                ClassOfOperation => "FN",
                AntennaPolarization => "FO",
                FundPurpose => "FP",
                FederalProcurementCodeList1 => "FP1",
                RadioSystemType => "FQ",
                FrequencyBand => "FR",
                FreddieMacRefinancePlanCode => "FRP",
                FreightBillApplicationErrorEditCodes => "FRT",
                AreaOfOperation => "FS",
                ApplicationType => "FT",
                FederalTransportationCodeList1 => "FT1",
                AuthorizationType => "FU",
                RadioServiceType => "FV",
                ApplicantClassificationType => "FW",
                Frequency => "FX",
                EditErrorCode => "FZ",
                RiskClass => "G",
                UniformResidentialAppraisalAttributesCode => "G1",
                ActionCode => "GA",
                MediumOfTransmissionCode => "GB",
                CodeGC => "GC",
                GainOrLossIndicatorCode => "GD",
                TypeAdjustmentCode => "GE",
                TypeIdentityChangeCode => "GF",
                TransportationModeReasonCode => "GG",
                NotificationIndicatorCode => "GI",
                RejectIndicatorCode => "GJ",
                InvestigationStatusCode => "GK",
                GroupQualifierCode => "GQ",
                CodeGR => "GR",
                CodeGS => "GS",
                CodeGT => "GT",
                CodeGU => "GU",
                CodeGV => "GV",
                CodeGW => "GW",
                GlassActionCode => "GX",
                CauseOfLossCode => "GY",
                LossDescriptionCode => "GZ",
                LifeAnnuityStatusCodes => "H",
                DiscrepancyCode => "HA",
                DiscrepancyAdviceCode => "HB",
                InstitutionalSectorOrLevelClassificationCode => "HC",
                DiscrepancyStatusOrDispositionCode => "HD",
                RemittanceAdviceRemarkCode => "HE",
                EducationStaffTypeCode => "HF",
                EducationFeeTypeCode => "HG",
                HealthIndustryNumber => "HI",
                InstitutionalFeeBasisCode => "HJ",
                NationalCenterForEducationStatisticsIntegratedPostsecondaryEducationDataSystemInstitutionalCharacteristicsSurveyCode => {
                    "HK"
                }
                CodeHL => "HL",
                NationalCenterForEducationStatisticsAccreditationOrLicensingType => "HM",
                HazardousMaterialContentCode => "HMC",
                HazardousResponseCodes => "HRC",
                ServiceContractActOccupationCategoryCode => "HS",
                AssociationOfAmericanRailroadsStandardTransportationCommodityCodeDescriptionQualifier => {
                    "HZR"
                }
                IdentifyingCharacteristics => "I",
                InsuranceBusinessProcessApplicationErrorCode => "IBP",
                CodeIC => "IC",
                CodeICF => "ICF",
                IdentityDisclosureCode => "ID",
                InvestmentFundType => "IF",
                ItemManagementCode => "IMC",
                ImpactRecorderCode => "IMP",
                IntraNavyDisposalReleaseOrderRejectAdviceCode => "IND",
                ImpactAxisOrAnalogPortCode => "IPA",
                IssuePriorityGroup => "IPG",
                IrsQualificationCode => "IQ",
                CodeIRR => "IRR",
                ImbalanceReportingType => "IRT",
                InitialTreatmentCode => "IT",
                InterruptibleTransportationIndicator => "ITI",
                TradeCode => "J",
                SummonsTypeCode => "J0",
                JudicialHearingTypeCode => "J1",
                JudicialOrderTypeCode => "J2",
                JudicialSentenceTypeCode => "J3",
                CourtDispositionCode => "J4",
                CourtAppearanceTypeCode => "J5",
                CourtPleadingTypeCode => "J6",
                DefendantPleaTypeCode => "J7",
                TrialTypeCode => "J8",
                CourtCaseStatusCode => "J9",
                PhysicalCharacteristicsCode => "JA",
                WeightOrFragilityCode => "JB",
                PreservationMaterialCode => "JC",
                JobCharacteristicsLookup => "JCL",
                QuantityPerUnitPackCode => "JD",
                PreservationDataCode => "JE",
                PackingRequirementLevelACode => "JF",
                PackingRequirementLevelBCode => "JG",
                PackingRequirementLevelCCode => "JH",
                IntermediateContainerCode => "JI",
                IntermediateContainerQuantityCode => "JK",
                SpecialMarkingCode => "JL",
                TypeAndCauseCode => "JM",
                MissionImpactStatementCode => "JN",
                InternationalStandardDesignationSystemForTeethAndAreasOfTheOralCavity => {
                    "JO"
                }
                JobOriginatorLookup => "JOL",
                UniversalNationalToothDesignationSystem => "JP",
                PropertyUnderwritingConditionCode => "K",
                DeficiencyCause => "KA",
                Discrepancy => "KB",
                PreventiveMeasure => "KC",
                ContractorAlertListReason => "KD",
                QualityAlertListReason => "KE",
                ContractorAlertListStatus => "KF",
                NatureOfBuy => "KG",
                TypeOfProcurement => "KH",
                RepresentativeBuyIndicator => "KI",
                AssuredDeliveryIndicator => "KJ",
                AwardSource => "KK",
                Termination => "KL",
                PatientEventProblemCode => "KM",
                MethodEvaluationCode => "KO",
                ResultEvaluationCode => "KP",
                ConclusionEvaluationCode => "KQ",
                DeviceEventProblemCode => "KS",
                DoseFormCode => "KT",
                RouteCode => "KU",
                ReportSourceCode => "KW",
                KeyEventLookup => "KYL",
                AdverseEventCode => "KZ",
                LineItemConditionCode => "L",
                Contract => "LA",
                ContractorReviewListStatus => "LB",
                LaboratoryTestConditionCode => "LC",
                LocationCapacityFlowIndicator => "LCF",
                CodeLD => "LD",
                LifeAnnuityServiceFeatures => "LE",
                LifeAnnuityProductCode => "LF",
                LocationCode => "LG",
                BasisOfJurisdictionCode => "LH",
                LineOfAuthority => "LIN",
                PrincipalPartyCitizenshipCode => "LJ",
                NatureOfSuitCode => "LK",
                CaseOriginCode => "LM",
                LimitType => "LMT",
                LineOfBusinessCode => "LN",
                LetterOfRecommendationRatingCategory => "LO",
                LocationIndicator => "LOC",
                CodeLOI => "LOI",
                DeficiencyIndicator => "LP",
                LocationPurposeCode => "LPC",
                DelinquencyIndicator => "LQ",
                LocationQuantityTypeIndicator => "LQT",
                TestResultsCode => "LR",
                LossSeverityCode => "LS",
                LegalStructureCode => "LSC",
                LaboratoryResultsIdentificationCode => "LT",
                WarReserveMaterialRequirementCode => "LZ",
                PolicyTypeCode => "M",
                MultiMediaObject => "MA",
                MaterialManagementAggregationCode => "MAC",
                ServiceContractActOccupationClassificationCode => "MB",
                ManualClassCode => "MC",
                MaterialControlCode => "MCC",
                GeneratorSetMountingCode => "MCD",
                DeviceEvaluationCode => "ME",
                MethodOfCompletionCode => "MEC",
                ManagerForcedDirectedAction => "MFD",
                MinorityIndicator => "MI",
                DrugStatusCode => "MJ",
                DrugStatusAdverseEventCode => "MK",
                LotTypeCode => "ML",
                PostMarketStudyStatusCode => "MN",
                CodeMOC => "MOC",
                MajorOrganizationalEntityRuleNumber => "MOE",
                ListOfCodesIdentifyingDoDMappingProductProcurementTypes => "MPP",
                ListOfCodesIdentifyingDoDMappingProductTypes => "MPT",
                ReferencePartialDescriptiveMethodReasonCode => "MRC",
                MaximumRateIndicator => "MRI",
                MeterStatus => "MS",
                MeterType => "MT",
                ValuationTypeCode => "N",
                PlantClearanceOfficeCode => "NA",
                CodeNAC => "NAC",
                CodeNAF => "NAF",
                NonApprovedItemName => "NAN",
                NatureOfSuit => "NAS",
                InventoryTypeCode => "NB",
                NoBalanceAffectingTransactions => "NBA",
                PropertyRecordStatusCode => "NC",
                InvoiceNotesCode => "NCD",
                NominationCapacityExceededIndicator => "NCE",
                ControlUnitDesignCode => "ND",
                CodeNDC => "NDC",
                DirectNumericalControlSystemCode => "NE",
                TypeNumericalControlSystemCode => "NF",
                CodeNGC => "NGC",
                PropertySourceCode => "NH",
                NatureOfInjuryCode => "NI",
                NonInductionReasonCode => "NIR",
                CodeNJ => "NJ",
                CodeNK => "NK",
                CodeNMS => "NMS",
                SpecialCategoryCode => "NP",
                CodeNPC => "NPC",
                ExcessMaterialDispositionCode => "NR",
                HazardousMaterialCode => "NS",
                TypeOfCargoCode => "NT",
                CodeNUB => "NUB",
                SourceOfLossCode => "O",
                CodeO1 => "O1",
                CodeO2 => "O2",
                CodeO3 => "O3",
                CodeO4 => "O4",
                OccupationCode => "OC",
                CeilingTypeCode => "P",
                PriorDamageLocationCode => "PA",
                PartOfBodyCode => "PB",
                CodePC => "PC",
                PlanningCodeLookup => "PCL",
                PseudoClosureReasonCode => "PCR",
                ProfessionalDesignation => "PD",
                CodePDA => "PDA",
                CodePGS => "PGS",
                PhraseCode => "PHC",
                CodePI => "PI",
                PetroleumBillType => "PIT",
                PriorityLookup => "PL",
                PetroleumLandCategory => "PLC",
                PetroleumLeaseStatus => "PLS",
                PartOfBodyAndNatureOfInjury => "POB",
                PlaceOfServiceCode => "POS",
                PetroleumProductDisposition => "PPD",
                PetroleumProductPointOfSale => "PPP",
                PetroleumProductSellingArrangement => "PPS",
                PetroleumProductValueAdjustment => "PPV",
                PetroleumQuantityAllocationsCode => "PQA",
                CodePR => "PR",
                PetroleumRoyaltyAdjustment => "PRA",
                PetroleumRoyaltyCalculationMethod => "PRC",
                ProcessingRightsIndicator => "PRI",
                PetroleumRegulatoryReport => "PRR",
                PetroleumRoyaltyTransaction => "PRT",
                ProfessionalStatusCode => "PS",
                PriceTier => "PT",
                PetroleumWellAction => "PWA",
                PetroleumWell => "PWI",
                PetroleumWellShutInReason => "PWR",
                PetroleumWellClassificationStatus => "PWS",
                PetroleumWellTest => "PWT",
                SurfaceDescriptorCode => "Q",
                ResponseStatusCode => "QA",
                BusinessEntityFilingReportTypeCode => "QB",
                BusinessEntityFilingDetailCode => "QC",
                ProductQualityDeficiencyReportSummaryCode => "QDR",
                DomesticLineOfBusinessCode => "QE",
                ForeignLineOfBusinessCode => "QF",
                BusinessEntityFilingStatusCode => "QG",
                BusinessEntityFilingSecuritiesInformationCode => "QH",
                BusinessEntityFinancialInformationCode => "QI",
                BusinessEntityStatusCode => "QJ",
                BusinessEntityFilingLocationCode => "QK",
                QueryStatus => "QS",
                QuantityTypeIndicator => "QT",
                CoverageModifier => "R",
                CodeR1 => "R1",
                CodeR2 => "R2",
                CodeR3 => "R3",
                CodeR4 => "R4",
                ThreadedRank => "R5",
                ReligiousAffiliationCode => "RA",
                ReceiptAcceptanceSiteCode => "RAS",
                RequirementCode => "RC",
                RegisteredContractorActivityCode => "RCA",
                PropertyOwnershipTypeCode => "RD",
                PropertyTypeCode => "RE",
                RaceOrEthnicityCollectionCode => "REC",
                ReductionReasonCode => "RED",
                AssociationOfAmericanRailroadsRateEdiNetworkErrorCode => "REN",
                ClassificationOfRaceOrEthnicity => "RET",
                ReferenceNumberFormatCode => "RFC",
                ReasonForMovementCode => "RFM",
                ResidencyIndicator => "RI",
                ReferenceNumberJustificationCode => "RJC",
                InsuranceIndustrySpecificRemarkCodes => "RM",
                ReferenceNumberCategoryCode => "RNC",
                ReplenishmentDemand => "RPQ",
                TestingServiceQuestionCodeList => "RQ",
                RetailDemand => "RQI",
                ReasonForReversalCode => "RRC",
                ReceiptSchedulingStatus => "RSS",
                RequestType => "RT",
                RegistrationTypeCode => "RTC",
                RefrigerationUnitOperatingModeCode => "RUM",
                ReferenceNumberVariationCode => "RVC",
                NationalCouncilForPrescriptionDrugProgramsRejectCodes => "RX",
                CodeS => "S",
                StudentActivityTypeCode => "SA",
                SecurityAssistanceDocumentNumberRequirementTypeCode => "SAD",
                StockActionTechnicalInformationCode => "SAT",
                StudentAwardCode => "SB",
                CodeSBA => "SBA",
                Source => "SC",
                SubsequentCycleIndicator => "SCI",
                CodeSD => "SD",
                SoundCode => "SE",
                StockExchangeCode => "SEC",
                SettlementType => "SET",
                RunType => "SF",
                SwingFuelOptionIndicator => "SFO",
                SourceOfDepositCode => "SG",
                SourceOfLeadCode => "SH",
                SafetyHazardLookup => "SHL",
                AccidentResultingChangeCode => "SHM",
                ActiveMitigationConsiderationCode => "SHN",
                ActivityMethodsCode => "SHO",
                AnalyticalMethodCode => "SHP",
                AtmosphericStabilityClassCode => "SHQ",
                BasisOfEstimateCode => "SHR",
                CertificationCode => "SHS",
                ContributingFactorCode => "SHT",
                ControlDeviceTypeCode => "SHU",
                DesignStandardCode => "SHV",
                DeviceClassificationCode => "SHW",
                DischargeIndicatorCode => "SHX",
                DischargeQuantityRangeCode => "SHY",
                NonReportableDischargeIndicatorCode => "SHZ",
                CodeSI => "SI",
                EmergencyResponseRegulationStatuteCode => "SIA",
                EmissionFactorTypeCode => "SIB",
                EmissionReleasePointTypeCode => "SIC",
                EmissionSourceTypeCode => "SID",
                EmissionTypeCode => "SIE",
                EmissionUnitTypeCode => "SIF",
                EndpointCode => "SIG",
                EnvironmentCode => "SIH",
                EnvironmentalProgramTypeCode => "SII",
                EnvironmentalReceptorCode => "SIJ",
                FacilityCategoryCode => "SIK",
                FacilityStatusCode => "SIL",
                FactorCalculationMethodCode => "SIM",
                FrequencyOfAnalysisCode => "SIN",
                GeneratorStatusCode => "SIO",
                GeometricTypeCode => "SIP",
                HazardousWasteFormCode => "SIQ",
                HorizontalDatumCode => "SIR",
                InformationSystemCode => "SIS",
                InitiatingEventCode => "SIT",
                InventoryQuantityRangeCode => "SIU",
                LatitudeLongitudeSourceCode => "SIV",
                LatitudeLongitudeVerificationCode => "SIW",
                LocationDescriptionCode => "SIX",
                MajorHazardCode => "SIY",
                ManufacturingCode => "SIZ",
                SourceOfInjuryCode => "SJ",
                MaterialClassificationCode => "SJA",
                MaterialCode => "SJB",
                MaximumAchievableControlTechnologyCode => "SJC",
                MethodOfCollectionCode => "SJD",
                MitigationSystemCode => "SJE",
                ModelUsedCode => "SJF",
                MonitoringDetectionSystemCode => "SJG",
                MonitoringLocationCode => "SJH",
                NonGeneratingWasteCode => "SJI",
                OffSiteAvailabilityCode => "SJJ",
                OffSiteImpactCode => "SJK",
                OnSiteImpactCode => "SJL",
                OnSiteProcessSystemTypeCode => "SJM",
                OriginCode => "SJN",
                ParameterCode => "SJO",
                PassiveMitigationConsiderationCode => "SJP",
                PermitComplianceStatusCode => "SJQ",
                PhysicalStateCode => "SJR",
                PointOfMeasurementCode => "SJS",
                PreservativeCode => "SJT",
                ProcessCode => "SJU",
                ProcessControlCode => "SJV",
                ProcessHazardAnalysisUpdateResultingChangeCode => "SJW",
                ProcessHazardsAnalysisTechniqueCode => "SJX",
                PublicReceptorCode => "SJY",
                RangeOfConcentrationCode => "SJZ",
                RecoveryMethodCode => "SKA",
                RecyclingMethodCode => "SKB",
                ReleaseEventCode => "SKC",
                ReleaseSourceCode => "SKD",
                ReliabilityIndicatorCode => "SKE",
                RuleEffectivenessMethodCode => "SKF",
                SampleTypeCode => "SKG",
                ScenarioCode => "SKH",
                SiteLocationCode => "SKI",
                SourceCategoryCode => "SKJ",
                SourceOfWasteGenerationCode => "SKK",
                SourceReductionActivityCode => "SKL",
                SystemTypeCode => "SKM",
                TimePeriodCode => "SKN",
                TopographyCode => "SKO",
                TransferQuantityRangeCode => "SKP",
                TypeOfCompetencyTestingCode => "SKQ",
                TypeOfTrainingCode => "SKR",
                TypeOfWasteManagementCode => "SKS",
                UseCode => "SKT",
                WasteEmanationCode => "SKU",
                WasteManagementStatusCode => "SKV",
                WasteStreamCode => "SKW",
                WasteTreatmentMethodCode => "SKX",
                WindDirectionCode => "SKY",
                UnitOfMeasureCode => "SKZ",
                SecondarySourceOfInjury => "SL",
                ShelfLifeActionCode => "SLA",
                StockageListCode => "SLC",
                SchedulingStatus => "SLS",
                StatementBasis => "SMB",
                SpecialMaterialContentCode => "SMC",
                SampleDevice => "SMD",
                SpecialMaterialIdentificationCode => "SMI",
                SampleType => "SMT",
                SolicitationCancellationReason => "SO",
                StandardOccupationClassificationCode => "SP",
                SubmittersPriorityDesignator => "SPD",
                SpecialDating => "SPE",
                StatisticalAdministrativeInformationCode => "SR",
                ServiceRequesterAgentIndicator => "SRA",
                ListOfCodesIdentifyingDoDSerializedReportTypeCodes => "SRC",
                SpecialRequirementLookup => "SRL",
                SupplementalReductionReason => "SRR",
                StorageReportType => "SRT",
                SystemStatus => "SS",
                SupplyStatusCode => "SSC",
                SpecialMarketingTypeCode => "ST",
                AssociationOfAmericanRailroadsStandardTransportationCommodityCodeMasterDescription => {
                    "STC"
                }
                ForwardAndStoreApplicationErrorEditCodes => "STF",
                SolicitedUnsolicitedIndicator => "SUI",
                ServiceCode => "SVC",
                AssociationOfAmericanRailroadsSwitchReleaseCodes => "SWR",
                PersonalPropertyAndContentsCode => "T",
                CommercialVehicleOperationsSafetyCode => "T00",
                DataCategories => "T01",
                EventCodes => "T02",
                OperationType => "T03",
                AccidentParameters => "T04",
                InspectionParameters => "T05",
                DriverParameters => "T06",
                ViewParameters => "T07",
                VehicleParameters => "T08",
                FleetParameters => "T09",
                QueryOptions => "T10",
                JurisdictionType => "T11",
                SingleStateRegistrationSystemAndOperatingAuthorityCredential => "T12",
                CommercialVehicleOperationsInsurance => "T13",
                CommercialVehicleRegistration => "T14",
                HazardousMaterialsCredential => "T15",
                OversizeOverweightCredential => "T16",
                CommercialVehicleTax => "T17",
                CommercialVehicleTitle => "T18",
                CommercialDriversLicense => "T19",
                CommercialVehicleType => "T20",
                CommercialVehicleOperationsStatusCode => "T21",
                SafetyAndFitnessElectronicRecordSystemsSubscriptionOption => "T22",
                CommercialVehicleOperationsCommodityCode => "T23",
                CommercialVehicleOperationsHazardousMaterialCode => "T24",
                SafetyAndFitnessElectronicRecordSystemsErrorCode => "T25",
                CommercialVehicleOperationsJurisdictionIdentifierCode => "T26",
                ComplianceReviewCode => "T27",
                IncidentConditionCode => "T28",
                IncidentRelatedActionCode => "T29",
                IncidentLocationCode => "T30",
                IncidentConsequenceCode => "T31",
                RoadCharacteristicCode => "T32",
                VehicleOccupantCode => "T33",
                PackageFailureCode => "T34",
                PedestrianCode => "T35",
                AssociationForFinancialProfessionalsServiceCodeAndBankServiceCode => "TB",
                TreatmentCodes => "TC",
                TypeOfChangeCode => "TC1",
                CodeTCD => "TCD",
                TemplateCharacteristicLookup => "TCL",
                CodeTD => "TD",
                DiscrepancyReportTypeCode => "TDC",
                TechnicalDataJustificationCode => "TDJ",
                AssociationForFinancialProfessionalsServiceCode => "TE",
                TitleExceptionAndRequirementCodeList => "TF",
                TaxOrFeeExemptionReasonCode => "TFR",
                TitleDocumentCodeList => "TG",
                TapLocation => "TL",
                WeaponSystemTransactionOriginationCode => "TOC",
                TemplateOwnerLookup => "TOL",
                TapType => "TP",
                CodeTQ => "TQ",
                ReportCode => "TR",
                NaturalGasTransactionType => "TT",
                DownstreamTransactionType => "TTD",
                TemplateTypeLookup => "TTL",
                UpstreamTransactionType => "TTU",
                FollowUpCode => "TX",
                ReportableEventCode => "TY",
                ResidentialAndCommercialRoomCode => "U",
                UmlerBodyType => "UBT",
                UniqueItemTrackingDesignatorCode => "UDC",
                UniqueItemTrackingErrorRejectCode => "UER",
                UmlerFittingCode => "UFC",
                UrgencyJustificationCode => "UJC",
                CodeUNP => "UNP",
                CodeUPC => "UPC",
                UnclaimedPropertyTypeCode => "UPT",
                EventReappearanceCode => "UR",
                EventAbatementCode => "US",
                CodeUT => "UT",
                UniqueItemTrackingTransactionCode => "UTC",
                UnitCode => "UU",
                ViolationTypeCodeList => "V",
                ValidationCode => "VAL",
                CodeVP => "VP",
                CourtIssuedWarrantTypeCode => "W",
                WeaponSystemAdviceCode => "WAC",
                WhenDiscoveredLookup => "WDL",
                WeaponSystemEssentialityCode => "WEC",
                CodeWRC => "WRC",
                WeaponSystemStatusCode => "WSC",
                WeaponSystemDesignatorCode => "WSD",
                WeaponSystemMaintenanceCode => "WSM",
                VehicleClass => "X",
                RentalCharge => "Y",
                CancellationReason => "Z",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<CodeListQualifierCode> {
        use CodeListQualifierCode::*;
        match code {
            b"0" => Some(DocumentIdentificationCode),
            b"1" => Some(FreeOnBoardSiteCode),
            b"2" => Some(ChannelOfDistributionCode),
            b"3" => Some(KindOfContractCode),
            b"4" => Some(TypeOfContractCode),
            b"5" => Some(CriticalityDesignatorCode),
            b"7" => Some(QualityAssuranceSiteCode),
            b"8" => Some(AcceptanceSiteCode),
            b"10" => Some(TransactionStatusIndicatorCode),
            b"11" => Some(ContractDeliveryDateRevisionAgentCode),
            b"12" => Some(ReasonForContractDeliveryDateRevisionCode),
            b"13" => Some(RecommendationsRegardingDelayedDeliveriesCode),
            b"14" => Some(ContractShipmentAdviceCode),
            b"15" => Some(IndividualInsuranceFinancialDetail),
            b"16" => Some(CashDiscountStipulationCode),
            b"17" => Some(ShipmentAcceptanceDiscrepancyExplanationCode),
            b"18" => Some(InsurancePlanDescriptionCharacteristics),
            b"19" => Some(ContractCloseOutGroupCode),
            b"20" => Some(PaymentTypeCode),
            b"21" => Some(ContractFundReportingTransactionCode),
            b"22" => Some(ContractPaymentDeductionOrCollectionCode),
            b"23" => Some(ObligationVarianceCode),
            b"24" => Some(PlusOrMinusIndicatorCode),
            b"25" => Some(ReasonForDelayedClosingOfContractFileCode),
            b"26" => Some(ContractPaymentLineItemStatusCode),
            b"27" => Some(SpecialReimbursableProvisionsCode),
            b"28" => Some(KindOfModificationCode),
            b"29" => Some(Code29),
            b"30" => Some(TypeOfDelayCode),
            b"31" => Some(HealthcareProviderCharacteristicsAndResources),
            b"32" => Some(ContainerAndRollOnRollOffNumberCode),
            b"33" => Some(AirCommodityAndSpecialHandlingCode),
            b"34" => Some(WaterCommodityAndSpecialHandlingCode),
            b"35" => Some(AirDimensionCode),
            b"36" => Some(AirTerminalIdentifierCode),
            b"37" => Some(WaterTerminalIdentifierCode),
            b"38" => Some(ConsolidationAndContainerizationPointCode),
            b"39" => Some(TransportationModeOrMethodCode),
            b"40" => Some(TypePackCode),
            b"41" => Some(DateShippedOrReceivedCode),
            b"42" => Some(EstimatedTimeOfArrivalCode),
            b"43" => Some(MilitaryAndCivilianGradeCode),
            b"44" => Some(SeavanOwnershipCode),
            b"45" => Some(OceanCarrierCode),
            b"46" => Some(VoyageDocumentNumberCode),
            b"47" => Some(VoyageManifestReferenceCode),
            b"48" => Some(VesselStatusAndTermsOfCarriageCode),
            b"49" => Some(VesselSustainingCode),
            b"50" => Some(SubrogationActionCode),
            b"52" => Some(BillingAdviceCode),
            b"53" => Some(BillingStatusCode),
            b"54" => Some(TypeOfBillCode),
            b"55" => Some(RecipientOfBillingStatusCode),
            b"56" => Some(SalesPriceConditionCode),
            b"57" => Some(DeliverySourceCode),
            b"58" => Some(TransportationBillCode),
            b"59" => Some(StockFundOrNonStockFundCode),
            b"60" => Some(Code60),
            b"61" => Some(InformationIndicatorCode),
            b"62" => Some(CommunicationsRoutingIdentifierCode),
            b"63" => Some(ContentIndicatorCode),
            b"65" => Some(HealthCareClaimStatusCode),
            b"66" => Some(SuffixOrLimitCode),
            b"67" => Some(TypeOfAssistanceCode),
            b"68" => Some(HealthcareProviderTaxonomy),
            b"69" => Some(ForeignMilitarySalesCountryCode),
            b"71" => Some(ServiceAndAgencyCode),
            b"72" => Some(DisbursementStatusCode),
            b"73" => Some(AidTypeCode),
            b"74" => Some(DemandCode),
            b"75" => Some(SuffixCode),
            b"78" => Some(ProjectCode),
            b"79" => Some(PriorityDesignatorCode),
            b"80" => Some(AdviceCode),
            b"81" => Some(StatusCode),
            b"82" => Some(ShipmentHoldCode),
            b"83" => Some(SupplyConditionCode),
            b"84" => Some(ManagementCode),
            b"85" => Some(CountryAndActivityCode),
            b"87" => Some(SubsistenceTypeOfPackCode),
            b"88" => Some(DisposalAuthorityCode),
            b"89" => Some(CooperativeLogisticsProgramSupportCode),
            b"90" => Some(PreciousMetalsIndicatorCode),
            b"91" => Some(AutomatedDataProcessingEquipmentIdentificationCode),
            b"92" => Some(ReasonForDisposalCode),
            b"93" => Some(TypeOfStorageCode),
            b"94" => Some(IdentificationCode),
            b"95" => Some(OfferAndReleaseOptionCode),
            b"96" => Some(ShipmentReleaseCode),
            b"97" => Some(UltimateRecipientCode),
            b"98" => Some(ReasonForRequisitioningCode),
            b"99" => Some(PurposeCode),
            b"100" => Some(Code100),
            b"101" => Some(Code101),
            b"102" => Some(MortgageIndexSourceCode),
            b"103" => Some(Code103),
            b"104" => Some(Code104),
            b"105" => Some(Code105),
            b"106" => Some(Code106),
            b"107" => Some(Code107),
            b"108" => Some(TestingStatisticalCategoryCodeList),
            b"109" => Some(TestingDemographicCategoryCodeList),
            b"A" => Some(CodeA),
            b"A1" => Some(OwnershipCode),
            b"A2" => Some(CustomerWithinCountryCode),
            b"A3" => Some(DeliveryTermCode),
            b"A4" => Some(CaseDesignatorNumber),
            b"A5" => Some(SubcaseNumber),
            b"A6" => Some(FreightForwarderNumber),
            b"A7" => Some(RecordControlNumber),
            b"A8" => Some(ProgramYearCode),
            b"A9" => Some(SupplementalData),
            b"AA" => Some(CodeAA),
            b"AAA" => Some(CodeAAA),
            b"AAD" => Some(AssetType),
            b"AAE" => Some(CurrentAssetType),
            b"AAF" => Some(CurrentLiabilityType),
            b"AAG" => {
                Some(DunAndBradstreetCanadas8DigitStandardIndustrialClassificationCode)
            }
            b"AAH" => Some(FinancialItemAllocationCode),
            b"AAI" => Some(FinancialItemAttributedCode),
            b"AAJ" => Some(FinancialItemReclassificationCode),
            b"AAK" => Some(FunctionalArea),
            b"AAL" => Some(HobbyCode),
            b"AAM" => Some(InvestmentType),
            b"AAN" => Some(LiabilityType),
            b"AAO" => Some(ProjectionType),
            b"AAP" => Some(TrendReason),
            b"AAQ" => Some(CodeAAQ),
            b"AAR" => Some(CodeAAR),
            b"AAS" => Some(Proprietary),
            b"AAT" => Some(FannieMaeAdjustableRateMortgagePlanCodes),
            b"AAU" => Some(CodeAAU),
            b"AAV" => Some(CodeAAV),
            b"AAW" => Some(ImmunizationInjectionCode),
            b"AAX" => Some(CodeAAX),
            b"AAY" => Some(CodeAAY),
            b"AB" => Some(DefensePrioritiesAndAllocationsSystemCode),
            b"ABF" => Some(CodeABF),
            b"ABJ" => Some(CodeABJ),
            b"ABK" => Some(CodeABK),
            b"ABN" => Some(CodeABN),
            b"ABR" => Some(AssignedByReceiver),
            b"ABS" => Some(AssignedBySender),
            b"ABU" => Some(CodeABU),
            b"ABV" => Some(CodeABV),
            b"AC" => Some(AccountCharacteristicsCode),
            b"ACC" => Some(AccountingErrorClassificationCode),
            b"ACR" => Some(AcademicRank),
            b"ACS" => Some(ListOfDoDAccountingRequirementsCodes),
            b"ACT" => Some(ListOfDoDAssetCategoryCodes),
            b"ACU" => Some(ListOfDoDControlledItemCodes),
            b"ACV" => Some(CodeACV),
            b"ACW" => Some(ListOfDoDFiduciaryDepreciationMethodCodes),
            b"ACX" => Some(CodeACX),
            b"ACY" => Some(CodeACY),
            b"ACZ" => Some(CodeACZ),
            b"AD" => Some(AcquisitionAdviceCode),
            b"AD1" => Some(ListOfDoDStorageRequirementCodes),
            b"AD2" => Some(ListOfDoDTemperatureControlledCodes),
            b"AD3" => Some(ListOfDoDAssetTypeCodes),
            b"AD4" => Some(ListOfDoDUtilizationMeasureCodes),
            b"ADD" => Some(CodeADD),
            b"ADJ" => Some(AccountingAdjustmentMethod),
            b"AE" => Some(BeneficiaryType),
            b"AEA" => Some(ArmyEditActionCode),
            b"AF" => Some(ClassOfPitch),
            b"AG" => Some(GradeOfDifficulty),
            b"AH" => Some(AcquisitionMethodSuffixCode),
            b"AI" => Some(AcquisitionMethodCode),
            b"AJ" => Some(UtilizationCode),
            b"AJT" => Some(AdjustmentType),
            b"AK" => Some(DistributionCode),
            b"AL" => Some(SpecialRequirementsCode),
            b"ALM" => Some(AllocationMethod),
            b"ALP" => Some(AlterationLookup),
            b"AM" => Some(LocaleOfActivity),
            b"AN" => Some(NatureOfEventCode),
            b"AO" => Some(SettlementPayoutOptions),
            b"AOR" => Some(AuthorizedOverrunIndicator),
            b"APE" => Some(CodeAPE),
            b"APR" => Some(CodeAPR),
            b"AQ" => Some(ApplicationQuestion),
            b"AQA" => Some(AllQuantityAvailableIndicator),
            b"AR" => Some(ArrestReason),
            b"ARD" => Some(AssetReclassificationDenialCode),
            b"ARI" => Some(AllocationRankIndicator),
            b"ARL" => Some(AllocationRankLevel),
            b"AS" => Some(FormTypeCode),
            b"ASD" => Some(CodeASD),
            b"AT" => Some(AllegationTypeCode),
            b"ATD" => Some(CodeATD),
            b"ATT" => Some(AllocationTransactionTypeCode),
            b"AU" => Some(CodeAU),
            b"AV" => Some(SubrogationPaymentOptions),
            b"AW" => Some(CodeAW),
            b"AX" => Some(CodeAX),
            b"AY" => Some(SubrogationResponseCodes),
            b"AZ" => Some(SubrogationRequestCodes),
            b"B" => Some(CodeB),
            b"BA" => Some(VesselStowageLocationCode),
            b"BB" => Some(BusinessType),
            b"BBF" => Some(CodeBBF),
            b"BBJ" => Some(CodeBBJ),
            b"BBK" => Some(CodeBBK),
            b"BBN" => Some(CodeBBN),
            b"BBQ" => Some(CodeBBQ),
            b"BBR" => Some(CodeBBR),
            b"BBU" => Some(CodeBBU),
            b"BBV" => Some(CodeBBV),
            b"BC" => Some(TransportationHoldingDelayCode),
            b"BCC" => Some(BusinessChangeCode),
            b"BCR" => Some(BusinessCreditRating),
            b"BD" => Some(TransportationPriorityCode),
            b"BDD" => Some(CodeBDD),
            b"BE" => Some(Value),
            b"BF" => Some(CodeBF),
            b"BG" => Some(Condition),
            b"BH" => Some(Occurrence),
            b"BI" => Some(OccurrenceSpan),
            b"BJ" => Some(CodeBJ),
            b"BK" => Some(CodeBK),
            b"BL" => Some(ApplicationFeeStatusCodes),
            b"BN" => Some(CodeBN),
            b"BO" => Some(HealthcareCommonProcedureCodingSystem),
            b"BP" => Some(HealthcareCommonProcedureCodingSystemPrincipalProcedure),
            b"BPL" => Some(BoardOfInspectionAndSurveyPartLookup),
            b"BPR" => Some(CodeBPR),
            b"BQ" => Some(CodeBQ),
            b"BR" => Some(CodeBR),
            b"BRL" => Some(BoardOfInspectionAndSurveyResponsibilityLookup),
            b"BS" => Some(CodeBS),
            b"BSD" => Some(CodeBSD),
            b"BSL" => Some(BoardOfInspectionAndSurveyShipLookup),
            b"BSP" => Some(BusinessPeriod),
            b"BT" => Some(AccidentDescription),
            b"BTC" => Some(BalanceTypeCode),
            b"BTD" => Some(CodeBTD),
            b"BU" => Some(PartOfBodyAffected),
            b"BUI" => Some(BidUpIndicator),
            b"BUR" => Some(BureauOfLaborStatisticsStandardizedOccupationalCodes),
            b"BV" => Some(EducationInstitutionTypeCode),
            b"BW" => Some(EducationalAreasCode),
            b"BX" => Some(ProfessionTypeCode),
            b"BY" => Some(ShareTypeCode),
            b"BZ" => Some(BusinessSizeCode),
            b"C" => Some(CodeC),
            b"C1" => Some(EyeColorCode),
            b"C2" => Some(HairColorCode),
            b"C3" => Some(SkinToneCode),
            b"CA" => Some(TypeOfInquiryCode),
            b"CAH" => Some(CodeCAH),
            b"CB" => Some(BilledOfficeIndicatorCode),
            b"CBQ" => Some(CodeCBQ),
            b"CBR" => Some(CodeCBR),
            b"CC" => Some(TreasurySymbolCode),
            b"CCC" => Some(CorrectionToCauseCode),
            b"CD" => Some(SupplementaryAccountingClassificationCode),
            b"CE" => Some(ReferenceAndStationCode),
            b"CF" => Some(MajorForceProgramCode),
            b"CFI" => Some(ContractualFlowIndicator),
            b"CG" => Some(AircraftMissionDesignSeriesCode),
            b"CH" => Some(TypeOfIssueCode),
            b"CHG" => Some(ChargeIndicator),
            b"CI" => Some(CriminalCharge),
            b"CIE" => Some(CodeCIE),
            b"CJ" => Some(CriminalChargeGrade),
            b"CK" => Some(CouponAdjustmentReasonCode),
            b"CL" => Some(CountyDesignatorCode),
            b"CLP" => Some(CauseLookup),
            b"CM" => Some(FinancialManagementServiceCashLinkCode),
            b"CML" => Some(CustomerMaintenanceLevelLookup),
            b"CN" => Some(CauseOfInjuryCode),
            b"CNC" => Some(ChangeNoticeCode),
            b"CO" => Some(CustomizedNoticeTypeCode),
            b"COG" => Some(CognizanceSymbol),
            b"COR" => Some(ConfirmingPartyRole),
            b"CP" => Some(SalvageDispositionCode),
            b"CPS" => Some(CourtPartyStatus),
            b"CQ" => Some(CapacityTypeIndicator),
            b"CR" => Some(CodeCR),
            b"CRC" => Some(ComplaintRequestCode),
            b"CRI" => Some(CausativeResearchIndicatorCode),
            b"CS" => Some(ClauseStatusType),
            b"CSD" => Some(CustomerServiceDesignator),
            b"CSF" => Some(CorporateStatementFilingCode),
            b"CT" => Some(CompensationTypeCodes),
            b"CTC" => Some(CarcassTrackingCode),
            b"CU" => Some(CuisineTypeCode),
            b"CV" => Some(CoverageCodeList),
            b"CW" => Some(ControvertCode),
            b"CZ" => Some(ConvictionOffenseType),
            b"D" => Some(CourtDocumentTypeCode),
            b"D1" => Some(DriversLicenseWithdrawalType),
            b"D2" => Some(DriversLicenseWithdrawalExtent),
            b"D3" => Some(DriversLicenseWithdrawalBasis),
            b"D4" => Some(DriversLicenseWithdrawalDueProcessStatus),
            b"D5" => Some(DriversLicenseWithdrawalReason),
            b"DA" => Some(DeviceAvailabilityCode),
            b"DAC" => Some(DocumentAvailabilityCode),
            b"DAP" => Some(CodeDAP),
            b"DB" => Some(DebtorBusinessTypeCode),
            b"DBS" => Some(CodeDBS),
            b"DC" => Some(ReportDistributionCode),
            b"DCC" => Some(CauseCode),
            b"DCM" => Some(CodeDCM),
            b"DCR" => Some(DispositionCategoryChangeRejectReasonCode),
            b"DCS" => Some(DispositionSubCategoryCode),
            b"DCT" => Some(DispositionCategoryCode),
            b"DD" => Some(CodeDD),
            b"DE" => Some(SignalCode),
            b"DF" => Some(MediaAndStatusCode),
            b"DG" => Some(FundCode),
            b"DGO" => Some(DynamicGeneratorSetCode),
            b"DH" => Some(DrugDetailCode),
            b"DI" => Some(SingleUseLabelCode),
            b"DIR" => Some(CodeDIR),
            b"DJ" => Some(RemedialActionCode),
            b"DK" => Some(ProgramOriginatorCode),
            b"DL" => Some(ServiceContractActOperationCode),
            b"DLO" => Some(DynamicLocomotiveTagCode),
            b"DLP" => Some(DeferralLookup),
            b"DLT" => Some(LongTermCareDrgLtcDrg),
            b"DM" => Some(AgentStatusCode),
            b"DMI" => Some(DemilitarizationIntegrityCode),
            b"DMP" => Some(DemilitarizationPerformedCode),
            b"DN" => Some(NatureOfDebtCode),
            b"DNT" => Some(DocumentNumberRequirementType),
            b"DO" => Some(DeviceOperatorTypeCode),
            b"DOF" => Some(DirectionOfFlow),
            b"DP" => Some(ProducerFinancialHistoryCodes),
            b"DPC" => Some(DeliveryPriorityCode),
            b"DPE" => Some(AssociationOfAmericanRailroadsDeprescriptionExceptionList),
            b"DPL" => Some(AssociationOfAmericanRailroadsDeprescriptionDistributionList),
            b"DQ" => Some(DeviceStatusCode),
            b"DR" => Some(CodeDR),
            b"DRD" => Some(CodeDRD),
            b"DRL" => Some(CodeDRL),
            b"DRS" => Some(DispositionServicesReimbursementCode),
            b"DS" => Some(RelatedDeviceApplicabilityCode),
            b"DSC" => Some(DispositionServicesCustomerTypeCode),
            b"DSD" => Some(CodeDSD),
            b"DSI" => Some(DispositionServicesIndicatorCode),
            b"DSR" => Some(DataSetsRequested),
            b"DSS" => Some(DeliverySchedulingStatus),
            b"DT" => Some(DebtorTypeCode),
            b"DTS" => Some(ListOfCodesIdentifyingDoDDistributionServicesTermsOfSale),
            b"DU" => Some(DeviceUsageCode),
            b"DV" => Some(DemandPlanningStatusCode),
            b"DW" => Some(EstimatingMethodStatusCode),
            b"DX" => Some(ContactStatusCode),
            b"DY" => Some(TypeOfFirmCode),
            b"DZ" => Some(ReportableEventStatusCode),
            b"E" => Some(CodeE),
            b"EA" => Some(AssetStatusOrTransactionReportingCode),
            b"EAA" => Some(AlabamaCampaignDisclosureReportCodes),
            b"EAB" => Some(AlaskaCampaignDisclosureReportCodes),
            b"EAC" => Some(AmericanSamoaCampaignDisclosureReportCodes),
            b"EAD" => Some(ArizonaCampaignDisclosureReportCodes),
            b"EAE" => Some(ArkansasCampaignDisclosureReportCodes),
            b"EAF" => Some(CaliforniaCampaignDisclosureReportCodes),
            b"EAG" => Some(ColoradoCampaignDisclosureReportCodes),
            b"EAH" => Some(ConnecticutCampaignDisclosureReportCodes),
            b"EAI" => Some(DelawareCampaignDisclosureReportCodes),
            b"EAJ" => Some(DistrictOfColumbiaCampaignDisclosureReportCodes),
            b"EAK" => Some(FloridaCampaignDisclosureReportCodes),
            b"EAL" => Some(GeorgiaCampaignDisclosureReportCodes),
            b"EAM" => Some(GuamCampaignDisclosureReportCodes),
            b"EAN" => Some(HawaiiCampaignDisclosureReportCodes),
            b"EAO" => Some(IdahoCampaignDisclosureReportCodes),
            b"EAP" => Some(IllinoisCampaignDisclosureReportCodes),
            b"EAQ" => Some(IndianaCampaignDisclosureReportCodes),
            b"EAR" => Some(IowaCampaignDisclosureReportCodes),
            b"EAS" => Some(KansasCampaignDisclosureReportCodes),
            b"EAT" => Some(KentuckyCampaignDisclosureReportCodes),
            b"EAU" => Some(LouisianaCampaignDisclosureReportCodes),
            b"EAV" => Some(MaineCampaignDisclosureReportCodes),
            b"EAW" => Some(MarylandCampaignDisclosureReportCodes),
            b"EAX" => Some(MassachusettsCampaignDisclosureReportCodes),
            b"EAY" => Some(MichiganCampaignDisclosureReportCodes),
            b"EAZ" => Some(MinnesotaCampaignDisclosureReportCodes),
            b"EB" => Some(AssetTransferStatusCode),
            b"EBA" => Some(MississippiCampaignDisclosureReportCodes),
            b"EBB" => Some(MissouriCampaignDisclosureReportCodes),
            b"EBC" => Some(MontanaCampaignDisclosureReportCodes),
            b"EBD" => Some(NebraskaCampaignDisclosureReportCodes),
            b"EBE" => Some(NevadaCampaignDisclosureReportCodes),
            b"EBF" => Some(NewHampshireCampaignDisclosureReportCodes),
            b"EBG" => Some(NewJerseyCampaignDisclosureReportCodes),
            b"EBH" => Some(NewMexicoCampaignDisclosureReportCodes),
            b"EBI" => Some(NewYorkCampaignDisclosureReportCodes),
            b"EBJ" => Some(NorthCarolinaCampaignDisclosureReportCodes),
            b"EBK" => Some(NorthDakotaCampaignDisclosureReportCodes),
            b"EBL" => Some(OhioCampaignDisclosureReportCodes),
            b"EBM" => Some(OklahomaCampaignDisclosureReportCodes),
            b"EBN" => Some(OregonCampaignDisclosureReportCodes),
            b"EBO" => Some(PennsylvaniaCampaignDisclosureReportCodes),
            b"EBP" => Some(PuertoRicoCampaignDisclosureReportCodes),
            b"EBQ" => Some(RhodeIslandCampaignDisclosureReportCodes),
            b"EBR" => Some(SouthCarolinaCampaignDisclosureReportCodes),
            b"EBS" => Some(SouthDakotaCampaignDisclosureReportCodes),
            b"EBT" => Some(TennesseeCampaignDisclosureReportCodes),
            b"EBU" => Some(TexasCampaignDisclosureReportCodes),
            b"EBV" => Some(UtahCampaignDisclosureReportCodes),
            b"EBW" => Some(VermontCampaignDisclosureReportCodes),
            b"EBX" => Some(VirginiaCampaignDisclosureReportCodes),
            b"EBY" => Some(VirginIslandsCampaignDisclosureReportCodes),
            b"EBZ" => Some(WashingtonCampaignDisclosureReportCodes),
            b"EC" => Some(CertificationRequirementsCode),
            b"ECA" => Some(WestVirginiaCampaignDisclosureReportCodes),
            b"ECB" => Some(WisconsinCampaignDisclosureReportCodes),
            b"ECC" => Some(WyomingCampaignDisclosureReportCodes),
            b"ECD" => Some(AlbertaCampaignDisclosureReportCodes),
            b"ECE" => Some(BritishColumbiaCampaignDisclosureReportCodes),
            b"ECF" => Some(ManitobaCampaignDisclosureReportCodes),
            b"ECG" => Some(NewBrunswickCampaignDisclosureReportCodes),
            b"ECH" => Some(NewfoundlandCampaignDisclosureReportCodes),
            b"ECI" => Some(NorthwestTerritoriesCampaignDisclosureReportCodes),
            b"ECJ" => Some(NovaScotiaCampaignDisclosureReportCodes),
            b"ECK" => Some(OntarioCampaignDisclosureReportCodes),
            b"ECL" => Some(PrinceEdwardIslandCampaignDisclosureReportCodes),
            b"ECM" => Some(QuebecCampaignDisclosureReportCodes),
            b"ECN" => Some(SaskatchewanCampaignDisclosureReportCodes),
            b"ECO" => Some(YukonTerritoryCampaignDisclosureReportCodes),
            b"ECP" => Some(FederalCampaignDisclosureReportCodes),
            b"ECQ" => Some(AlabamaLobbyistReportCodes),
            b"ECR" => Some(AlaskaLobbyistReportCodes),
            b"ECS" => Some(ArizonaLobbyistReportCodes),
            b"ECT" => Some(ArkansasLobbyistReportCodes),
            b"ECU" => Some(CaliforniaLobbyistReportCodes),
            b"ECV" => Some(ColoradoLobbyistReportCodes),
            b"ECW" => Some(ConnecticutLobbyistReportCodes),
            b"ECX" => Some(DelawareLobbyistReportCodes),
            b"ECY" => Some(DistrictOfColumbiaLobbyistReportCodes),
            b"ECZ" => Some(FloridaLobbyistReportCodes),
            b"ED" => Some(CoastDesignationCode),
            b"EDA" => Some(GeorgiaLobbyistReportCodes),
            b"EDB" => Some(HawaiiLobbyistReportCodes),
            b"EDC" => Some(IdahoLobbyistReportCodes),
            b"EDD" => Some(IllinoisLobbyistReportCodes),
            b"EDE" => Some(IndianaLobbyistReportCodes),
            b"EDF" => Some(IowaLobbyistReportCodes),
            b"EDG" => Some(KansasLobbyistReportCodes),
            b"EDH" => Some(KentuckyLobbyistReportCodes),
            b"EDI" => Some(LouisianaLobbyistReportCodes),
            b"EDJ" => Some(MaineLobbyistReportCodes),
            b"EDK" => Some(MarylandLobbyistReportCodes),
            b"EDL" => Some(MassachusettsLobbyistReportCodes),
            b"EDM" => Some(MichiganLobbyistReportCodes),
            b"EDN" => Some(MinnesotaLobbyistReportCodes),
            b"EDO" => Some(MississippiLobbyistReportCodes),
            b"EDP" => Some(MissouriLobbyistReportCodes),
            b"EDQ" => Some(MontanaLobbyistReportCodes),
            b"EDR" => Some(NebraskaLobbyistReportCodes),
            b"EDS" => Some(NevadaLobbyistReportCodes),
            b"EDT" => Some(NewHampshireLobbyistReportCodes),
            b"EDU" => Some(NewJerseyLobbyistReportCodes),
            b"EDV" => Some(NewMexicoLobbyistReportCodes),
            b"EDW" => Some(NewYorkLobbyistReportCodes),
            b"EDX" => Some(NorthCarolinaLobbyistReportCodes),
            b"EDY" => Some(NorthDakotaLobbyistReportCodes),
            b"EDZ" => Some(OhioLobbyistReportCodes),
            b"EE" => Some(CompetitiveCharacteristicsCode),
            b"EEA" => Some(OklahomaLobbyistReportCodes),
            b"EEB" => Some(OregonLobbyistReportCodes),
            b"EEC" => Some(PennsylvaniaLobbyistReportCodes),
            b"EED" => Some(PuertoRicoLobbyistReportCodes),
            b"EEE" => Some(RhodeIslandLobbyistReportCodes),
            b"EEF" => Some(SouthCarolinaLobbyistReportCodes),
            b"EEG" => Some(SouthDakotaLobbyistReportCodes),
            b"EEH" => Some(TennesseeLobbyistReportCodes),
            b"EEI" => Some(TexasLobbyistReportCodes),
            b"EEJ" => Some(UtahLobbyistReportCodes),
            b"EEK" => Some(VermontLobbyistReportCodes),
            b"EEL" => Some(VirginiaLobbyistReportCodes),
            b"EEM" => Some(WashingtonLobbyistReportCodes),
            b"EEN" => Some(WestVirginiaLobbyistReportCodes),
            b"EEO" => Some(WisconsinLobbyistReportCodes),
            b"EEP" => Some(WyomingLobbyistReportCodes),
            b"EEQ" => Some(NewYorkCityCampaignDisclosureReportCodes),
            b"EER" => Some(SeattleCampaignDisclosureReportCodes),
            b"EES" => Some(NewYorkCityLobbyistReportCodes),
            b"EF" => Some(CorrectionOrChangeForStorageItemRecordsCode),
            b"EG" => Some(ExcavationInformationCodeList),
            b"EH" => Some(TypeDueInIndicator),
            b"EI" => Some(DiscrepancyIndicatorCode),
            b"EJ" => Some(DisposalConditionCode),
            b"EK" => Some(EventOrExposureCode),
            b"EL" => Some(ErrorClassificationCode),
            b"EM" => Some(InventoryCategoryCode),
            b"EMC" => Some(CodeEMC),
            b"EN" => Some(LocalSourceCode),
            b"EO" => Some(AdverseEventOutcomeCode),
            b"EP" => Some(CodeEP),
            b"EPI" => Some(ExchangePriceIndicator),
            b"EQ" => Some(ControlledInventoryItemCode),
            b"EQR" => Some(EquipmentRequestCodes),
            b"ER" => Some(DepartmentOfDefenseIdentificationCode),
            b"ERC" => Some(EquipmentRepairConditionCode),
            b"ERJ" => Some(EquipmentRepairJobCode),
            b"ERL" => Some(EquipmentRepairLocationCode),
            b"ERR" => Some(EquipmentRepairResponsibilityCode),
            b"ES" => Some(ExtensionReason),
            b"ESC" => Some(ElectrostaticDischargeCode),
            b"ESL" => Some(EquipmentStatusLookup),
            b"ET" => Some(RejectAdviceCode),
            b"ETL" => Some(EstimateTypeLookup),
            b"EU" => Some(RequestCode),
            b"EV" => Some(ReviewPeriodIndicatorCode),
            b"EW" => Some(SmallArmsErrorTransactionRejectCode),
            b"EWC" => Some(EvaluateWorkCandidateLookup),
            b"EWM" => Some(EquipmentWhyMadeCode),
            b"EWR" => Some(EvaluateWorkCandidateReasonLookup),
            b"EX" => Some(SmallArmsTransactionCode),
            b"EXD" => Some(ExportDeclaration),
            b"XD" => Some(ExportDeclaration),
            b"EXP" => Some(CodeEXP),
            b"EY" => Some(SpecialProgramRequirementStatusCode),
            b"EZ" => Some(TypeInspectionCode),
            b"F" => Some(FinancialRating),
            b"FA" => Some(TypeOfContractorCode),
            b"FAP" => Some(FannieMaeRefinancePlanCode),
            b"FB" => Some(TypeOfMediaCode),
            b"FC" => Some(TypePhysicalInventoryOrTransactionHistoryCode),
            b"FC1" => Some(CodeFC1),
            b"FD" => Some(DemilitarizationCode),
            b"FE" => Some(ShelfLifeCode),
            b"FF" => Some(EssentialityCode),
            b"FF1" => Some(FederalFinanceCodeList1),
            b"FG" => Some(SourceMaintenanceAndRecoverabilityCode),
            b"FH" => Some(TypeOfLocationReconciliationRequest),
            b"FH1" => Some(FederalHealthCareCodeList1),
            b"FI" => Some(ApplicantType),
            b"FIR" => Some(FinancialInventoryReportCode),
            b"FJ" => Some(AntennaStructureType),
            b"FK" => Some(StationClassification),
            b"FL" => Some(RadioFrequencyType),
            b"FL1" => Some(FederalLogisticsCodeList1),
            b"FM" => Some(StationClassificationType),
            b"FMO" => Some(FormerMajorOrganizationalEntityRuleNumber),
            b"FMS" => {
                Some(
                    ForeignMilitarySalesAndMilitaryAssistanceProgramGrantAidTypeOfAssistanceFinancingCode,
                )
            }
            b"FN" => Some(ClassOfOperation),
            b"FO" => Some(AntennaPolarization),
            b"FP" => Some(FundPurpose),
            b"FP1" => Some(FederalProcurementCodeList1),
            b"FQ" => Some(RadioSystemType),
            b"FR" => Some(FrequencyBand),
            b"FRP" => Some(FreddieMacRefinancePlanCode),
            b"FRT" => Some(FreightBillApplicationErrorEditCodes),
            b"FS" => Some(AreaOfOperation),
            b"FT" => Some(ApplicationType),
            b"FT1" => Some(FederalTransportationCodeList1),
            b"FU" => Some(AuthorizationType),
            b"FV" => Some(RadioServiceType),
            b"FW" => Some(ApplicantClassificationType),
            b"FX" => Some(Frequency),
            b"FZ" => Some(EditErrorCode),
            b"G" => Some(RiskClass),
            b"G1" => Some(UniformResidentialAppraisalAttributesCode),
            b"GA" => Some(ActionCode),
            b"GB" => Some(MediumOfTransmissionCode),
            b"GC" => Some(CodeGC),
            b"GD" => Some(GainOrLossIndicatorCode),
            b"GE" => Some(TypeAdjustmentCode),
            b"GF" => Some(TypeIdentityChangeCode),
            b"GG" => Some(TransportationModeReasonCode),
            b"GI" => Some(NotificationIndicatorCode),
            b"GJ" => Some(RejectIndicatorCode),
            b"GK" => Some(InvestigationStatusCode),
            b"GQ" => Some(GroupQualifierCode),
            b"GR" => Some(CodeGR),
            b"GS" => Some(CodeGS),
            b"GT" => Some(CodeGT),
            b"GU" => Some(CodeGU),
            b"GV" => Some(CodeGV),
            b"GW" => Some(CodeGW),
            b"GX" => Some(GlassActionCode),
            b"GY" => Some(CauseOfLossCode),
            b"GZ" => Some(LossDescriptionCode),
            b"H" => Some(LifeAnnuityStatusCodes),
            b"HA" => Some(DiscrepancyCode),
            b"HB" => Some(DiscrepancyAdviceCode),
            b"HC" => Some(InstitutionalSectorOrLevelClassificationCode),
            b"HD" => Some(DiscrepancyStatusOrDispositionCode),
            b"HE" => Some(RemittanceAdviceRemarkCode),
            b"HF" => Some(EducationStaffTypeCode),
            b"HG" => Some(EducationFeeTypeCode),
            b"HI" => Some(HealthIndustryNumber),
            b"HJ" => Some(InstitutionalFeeBasisCode),
            b"HK" => {
                Some(
                    NationalCenterForEducationStatisticsIntegratedPostsecondaryEducationDataSystemInstitutionalCharacteristicsSurveyCode,
                )
            }
            b"HL" => Some(CodeHL),
            b"HM" => {
                Some(NationalCenterForEducationStatisticsAccreditationOrLicensingType)
            }
            b"HMC" => Some(HazardousMaterialContentCode),
            b"HRC" => Some(HazardousResponseCodes),
            b"HS" => Some(ServiceContractActOccupationCategoryCode),
            b"HZR" => {
                Some(
                    AssociationOfAmericanRailroadsStandardTransportationCommodityCodeDescriptionQualifier,
                )
            }
            b"I" => Some(IdentifyingCharacteristics),
            b"IBP" => Some(InsuranceBusinessProcessApplicationErrorCode),
            b"IC" => Some(CodeIC),
            b"ICF" => Some(CodeICF),
            b"ID" => Some(IdentityDisclosureCode),
            b"IF" => Some(InvestmentFundType),
            b"IMC" => Some(ItemManagementCode),
            b"IMP" => Some(ImpactRecorderCode),
            b"IND" => Some(IntraNavyDisposalReleaseOrderRejectAdviceCode),
            b"IPA" => Some(ImpactAxisOrAnalogPortCode),
            b"IPG" => Some(IssuePriorityGroup),
            b"IQ" => Some(IrsQualificationCode),
            b"IRR" => Some(CodeIRR),
            b"IRT" => Some(ImbalanceReportingType),
            b"IT" => Some(InitialTreatmentCode),
            b"ITI" => Some(InterruptibleTransportationIndicator),
            b"J" => Some(TradeCode),
            b"J0" => Some(SummonsTypeCode),
            b"J1" => Some(JudicialHearingTypeCode),
            b"J2" => Some(JudicialOrderTypeCode),
            b"J3" => Some(JudicialSentenceTypeCode),
            b"J4" => Some(CourtDispositionCode),
            b"J5" => Some(CourtAppearanceTypeCode),
            b"J6" => Some(CourtPleadingTypeCode),
            b"J7" => Some(DefendantPleaTypeCode),
            b"J8" => Some(TrialTypeCode),
            b"J9" => Some(CourtCaseStatusCode),
            b"JA" => Some(PhysicalCharacteristicsCode),
            b"JB" => Some(WeightOrFragilityCode),
            b"JC" => Some(PreservationMaterialCode),
            b"JCL" => Some(JobCharacteristicsLookup),
            b"JD" => Some(QuantityPerUnitPackCode),
            b"JE" => Some(PreservationDataCode),
            b"JF" => Some(PackingRequirementLevelACode),
            b"JG" => Some(PackingRequirementLevelBCode),
            b"JH" => Some(PackingRequirementLevelCCode),
            b"JI" => Some(IntermediateContainerCode),
            b"JK" => Some(IntermediateContainerQuantityCode),
            b"JL" => Some(SpecialMarkingCode),
            b"JM" => Some(TypeAndCauseCode),
            b"JN" => Some(MissionImpactStatementCode),
            b"JO" => {
                Some(
                    InternationalStandardDesignationSystemForTeethAndAreasOfTheOralCavity,
                )
            }
            b"JOL" => Some(JobOriginatorLookup),
            b"JP" => Some(UniversalNationalToothDesignationSystem),
            b"K" => Some(PropertyUnderwritingConditionCode),
            b"KA" => Some(DeficiencyCause),
            b"KB" => Some(Discrepancy),
            b"KC" => Some(PreventiveMeasure),
            b"KD" => Some(ContractorAlertListReason),
            b"KE" => Some(QualityAlertListReason),
            b"KF" => Some(ContractorAlertListStatus),
            b"KG" => Some(NatureOfBuy),
            b"KH" => Some(TypeOfProcurement),
            b"KI" => Some(RepresentativeBuyIndicator),
            b"KJ" => Some(AssuredDeliveryIndicator),
            b"KK" => Some(AwardSource),
            b"KL" => Some(Termination),
            b"KM" => Some(PatientEventProblemCode),
            b"KO" => Some(MethodEvaluationCode),
            b"KP" => Some(ResultEvaluationCode),
            b"KQ" => Some(ConclusionEvaluationCode),
            b"KS" => Some(DeviceEventProblemCode),
            b"KT" => Some(DoseFormCode),
            b"KU" => Some(RouteCode),
            b"KW" => Some(ReportSourceCode),
            b"KYL" => Some(KeyEventLookup),
            b"KZ" => Some(AdverseEventCode),
            b"L" => Some(LineItemConditionCode),
            b"LA" => Some(Contract),
            b"LB" => Some(ContractorReviewListStatus),
            b"LC" => Some(LaboratoryTestConditionCode),
            b"LCF" => Some(LocationCapacityFlowIndicator),
            b"LD" => Some(CodeLD),
            b"LE" => Some(LifeAnnuityServiceFeatures),
            b"LF" => Some(LifeAnnuityProductCode),
            b"LG" => Some(LocationCode),
            b"LH" => Some(BasisOfJurisdictionCode),
            b"LIN" => Some(LineOfAuthority),
            b"LJ" => Some(PrincipalPartyCitizenshipCode),
            b"LK" => Some(NatureOfSuitCode),
            b"LM" => Some(CaseOriginCode),
            b"LMT" => Some(LimitType),
            b"LN" => Some(LineOfBusinessCode),
            b"LO" => Some(LetterOfRecommendationRatingCategory),
            b"LOC" => Some(LocationIndicator),
            b"LOI" => Some(CodeLOI),
            b"LP" => Some(DeficiencyIndicator),
            b"LPC" => Some(LocationPurposeCode),
            b"LQ" => Some(DelinquencyIndicator),
            b"LQT" => Some(LocationQuantityTypeIndicator),
            b"LR" => Some(TestResultsCode),
            b"LS" => Some(LossSeverityCode),
            b"LSC" => Some(LegalStructureCode),
            b"LT" => Some(LaboratoryResultsIdentificationCode),
            b"LZ" => Some(WarReserveMaterialRequirementCode),
            b"M" => Some(PolicyTypeCode),
            b"MA" => Some(MultiMediaObject),
            b"MAC" => Some(MaterialManagementAggregationCode),
            b"MB" => Some(ServiceContractActOccupationClassificationCode),
            b"MC" => Some(ManualClassCode),
            b"MCC" => Some(MaterialControlCode),
            b"MCD" => Some(GeneratorSetMountingCode),
            b"ME" => Some(DeviceEvaluationCode),
            b"MEC" => Some(MethodOfCompletionCode),
            b"MFD" => Some(ManagerForcedDirectedAction),
            b"MI" => Some(MinorityIndicator),
            b"MJ" => Some(DrugStatusCode),
            b"MK" => Some(DrugStatusAdverseEventCode),
            b"ML" => Some(LotTypeCode),
            b"MN" => Some(PostMarketStudyStatusCode),
            b"MOC" => Some(CodeMOC),
            b"MOE" => Some(MajorOrganizationalEntityRuleNumber),
            b"MPP" => Some(ListOfCodesIdentifyingDoDMappingProductProcurementTypes),
            b"MPT" => Some(ListOfCodesIdentifyingDoDMappingProductTypes),
            b"MRC" => Some(ReferencePartialDescriptiveMethodReasonCode),
            b"MRI" => Some(MaximumRateIndicator),
            b"MS" => Some(MeterStatus),
            b"MT" => Some(MeterType),
            b"N" => Some(ValuationTypeCode),
            b"NA" => Some(PlantClearanceOfficeCode),
            b"NAC" => Some(CodeNAC),
            b"NAF" => Some(CodeNAF),
            b"NAN" => Some(NonApprovedItemName),
            b"NAS" => Some(NatureOfSuit),
            b"NB" => Some(InventoryTypeCode),
            b"NBA" => Some(NoBalanceAffectingTransactions),
            b"NC" => Some(PropertyRecordStatusCode),
            b"NCD" => Some(InvoiceNotesCode),
            b"NCE" => Some(NominationCapacityExceededIndicator),
            b"ND" => Some(ControlUnitDesignCode),
            b"NDC" => Some(CodeNDC),
            b"NE" => Some(DirectNumericalControlSystemCode),
            b"NF" => Some(TypeNumericalControlSystemCode),
            b"NGC" => Some(CodeNGC),
            b"NH" => Some(PropertySourceCode),
            b"NI" => Some(NatureOfInjuryCode),
            b"NIR" => Some(NonInductionReasonCode),
            b"NJ" => Some(CodeNJ),
            b"NK" => Some(CodeNK),
            b"NMS" => Some(CodeNMS),
            b"NP" => Some(SpecialCategoryCode),
            b"NPC" => Some(CodeNPC),
            b"NR" => Some(ExcessMaterialDispositionCode),
            b"NS" => Some(HazardousMaterialCode),
            b"NT" => Some(TypeOfCargoCode),
            b"NUB" => Some(CodeNUB),
            b"O" => Some(SourceOfLossCode),
            b"O1" => Some(CodeO1),
            b"O2" => Some(CodeO2),
            b"O3" => Some(CodeO3),
            b"O4" => Some(CodeO4),
            b"OC" => Some(OccupationCode),
            b"P" => Some(CeilingTypeCode),
            b"PA" => Some(PriorDamageLocationCode),
            b"PB" => Some(PartOfBodyCode),
            b"PC" => Some(CodePC),
            b"PCL" => Some(PlanningCodeLookup),
            b"PCR" => Some(PseudoClosureReasonCode),
            b"PD" => Some(ProfessionalDesignation),
            b"PDA" => Some(CodePDA),
            b"PGS" => Some(CodePGS),
            b"PHC" => Some(PhraseCode),
            b"PI" => Some(CodePI),
            b"PIT" => Some(PetroleumBillType),
            b"PL" => Some(PriorityLookup),
            b"PLC" => Some(PetroleumLandCategory),
            b"PLS" => Some(PetroleumLeaseStatus),
            b"POB" => Some(PartOfBodyAndNatureOfInjury),
            b"POS" => Some(PlaceOfServiceCode),
            b"PPD" => Some(PetroleumProductDisposition),
            b"PPP" => Some(PetroleumProductPointOfSale),
            b"PPS" => Some(PetroleumProductSellingArrangement),
            b"PPV" => Some(PetroleumProductValueAdjustment),
            b"PQA" => Some(PetroleumQuantityAllocationsCode),
            b"PR" => Some(CodePR),
            b"PRA" => Some(PetroleumRoyaltyAdjustment),
            b"PRC" => Some(PetroleumRoyaltyCalculationMethod),
            b"PRI" => Some(ProcessingRightsIndicator),
            b"PRR" => Some(PetroleumRegulatoryReport),
            b"PRT" => Some(PetroleumRoyaltyTransaction),
            b"PS" => Some(ProfessionalStatusCode),
            b"PT" => Some(PriceTier),
            b"PWA" => Some(PetroleumWellAction),
            b"PWI" => Some(PetroleumWell),
            b"PWR" => Some(PetroleumWellShutInReason),
            b"PWS" => Some(PetroleumWellClassificationStatus),
            b"PWT" => Some(PetroleumWellTest),
            b"Q" => Some(SurfaceDescriptorCode),
            b"QA" => Some(ResponseStatusCode),
            b"QB" => Some(BusinessEntityFilingReportTypeCode),
            b"QC" => Some(BusinessEntityFilingDetailCode),
            b"QDR" => Some(ProductQualityDeficiencyReportSummaryCode),
            b"QE" => Some(DomesticLineOfBusinessCode),
            b"QF" => Some(ForeignLineOfBusinessCode),
            b"QG" => Some(BusinessEntityFilingStatusCode),
            b"QH" => Some(BusinessEntityFilingSecuritiesInformationCode),
            b"QI" => Some(BusinessEntityFinancialInformationCode),
            b"QJ" => Some(BusinessEntityStatusCode),
            b"QK" => Some(BusinessEntityFilingLocationCode),
            b"QS" => Some(QueryStatus),
            b"QT" => Some(QuantityTypeIndicator),
            b"R" => Some(CoverageModifier),
            b"R1" => Some(CodeR1),
            b"R2" => Some(CodeR2),
            b"R3" => Some(CodeR3),
            b"R4" => Some(CodeR4),
            b"R5" => Some(ThreadedRank),
            b"RA" => Some(ReligiousAffiliationCode),
            b"RAS" => Some(ReceiptAcceptanceSiteCode),
            b"RC" => Some(RequirementCode),
            b"RCA" => Some(RegisteredContractorActivityCode),
            b"RD" => Some(PropertyOwnershipTypeCode),
            b"RE" => Some(PropertyTypeCode),
            b"REC" => Some(RaceOrEthnicityCollectionCode),
            b"RED" => Some(ReductionReasonCode),
            b"REN" => Some(AssociationOfAmericanRailroadsRateEdiNetworkErrorCode),
            b"RET" => Some(ClassificationOfRaceOrEthnicity),
            b"RFC" => Some(ReferenceNumberFormatCode),
            b"RFM" => Some(ReasonForMovementCode),
            b"RI" => Some(ResidencyIndicator),
            b"RJC" => Some(ReferenceNumberJustificationCode),
            b"RM" => Some(InsuranceIndustrySpecificRemarkCodes),
            b"RNC" => Some(ReferenceNumberCategoryCode),
            b"RPQ" => Some(ReplenishmentDemand),
            b"RQ" => Some(TestingServiceQuestionCodeList),
            b"RQI" => Some(RetailDemand),
            b"RRC" => Some(ReasonForReversalCode),
            b"RSS" => Some(ReceiptSchedulingStatus),
            b"RT" => Some(RequestType),
            b"RTC" => Some(RegistrationTypeCode),
            b"RUM" => Some(RefrigerationUnitOperatingModeCode),
            b"RVC" => Some(ReferenceNumberVariationCode),
            b"RX" => Some(NationalCouncilForPrescriptionDrugProgramsRejectCodes),
            b"S" => Some(CodeS),
            b"SA" => Some(StudentActivityTypeCode),
            b"SAD" => Some(SecurityAssistanceDocumentNumberRequirementTypeCode),
            b"SAT" => Some(StockActionTechnicalInformationCode),
            b"SB" => Some(StudentAwardCode),
            b"SBA" => Some(CodeSBA),
            b"SC" => Some(Source),
            b"SCI" => Some(SubsequentCycleIndicator),
            b"SD" => Some(CodeSD),
            b"SE" => Some(SoundCode),
            b"SEC" => Some(StockExchangeCode),
            b"SET" => Some(SettlementType),
            b"SF" => Some(RunType),
            b"SFO" => Some(SwingFuelOptionIndicator),
            b"SG" => Some(SourceOfDepositCode),
            b"SH" => Some(SourceOfLeadCode),
            b"SHL" => Some(SafetyHazardLookup),
            b"SHM" => Some(AccidentResultingChangeCode),
            b"SHN" => Some(ActiveMitigationConsiderationCode),
            b"SHO" => Some(ActivityMethodsCode),
            b"SHP" => Some(AnalyticalMethodCode),
            b"SHQ" => Some(AtmosphericStabilityClassCode),
            b"SHR" => Some(BasisOfEstimateCode),
            b"SHS" => Some(CertificationCode),
            b"SHT" => Some(ContributingFactorCode),
            b"SHU" => Some(ControlDeviceTypeCode),
            b"SHV" => Some(DesignStandardCode),
            b"SHW" => Some(DeviceClassificationCode),
            b"SHX" => Some(DischargeIndicatorCode),
            b"SHY" => Some(DischargeQuantityRangeCode),
            b"SHZ" => Some(NonReportableDischargeIndicatorCode),
            b"SI" => Some(CodeSI),
            b"SIA" => Some(EmergencyResponseRegulationStatuteCode),
            b"SIB" => Some(EmissionFactorTypeCode),
            b"SIC" => Some(EmissionReleasePointTypeCode),
            b"SID" => Some(EmissionSourceTypeCode),
            b"SIE" => Some(EmissionTypeCode),
            b"SIF" => Some(EmissionUnitTypeCode),
            b"SIG" => Some(EndpointCode),
            b"SIH" => Some(EnvironmentCode),
            b"SII" => Some(EnvironmentalProgramTypeCode),
            b"SIJ" => Some(EnvironmentalReceptorCode),
            b"SIK" => Some(FacilityCategoryCode),
            b"SIL" => Some(FacilityStatusCode),
            b"SIM" => Some(FactorCalculationMethodCode),
            b"SIN" => Some(FrequencyOfAnalysisCode),
            b"SIO" => Some(GeneratorStatusCode),
            b"SIP" => Some(GeometricTypeCode),
            b"SIQ" => Some(HazardousWasteFormCode),
            b"SIR" => Some(HorizontalDatumCode),
            b"SIS" => Some(InformationSystemCode),
            b"SIT" => Some(InitiatingEventCode),
            b"SIU" => Some(InventoryQuantityRangeCode),
            b"SIV" => Some(LatitudeLongitudeSourceCode),
            b"SIW" => Some(LatitudeLongitudeVerificationCode),
            b"SIX" => Some(LocationDescriptionCode),
            b"SIY" => Some(MajorHazardCode),
            b"SIZ" => Some(ManufacturingCode),
            b"SJ" => Some(SourceOfInjuryCode),
            b"SJA" => Some(MaterialClassificationCode),
            b"SJB" => Some(MaterialCode),
            b"SJC" => Some(MaximumAchievableControlTechnologyCode),
            b"SJD" => Some(MethodOfCollectionCode),
            b"SJE" => Some(MitigationSystemCode),
            b"SJF" => Some(ModelUsedCode),
            b"SJG" => Some(MonitoringDetectionSystemCode),
            b"SJH" => Some(MonitoringLocationCode),
            b"SJI" => Some(NonGeneratingWasteCode),
            b"SJJ" => Some(OffSiteAvailabilityCode),
            b"SJK" => Some(OffSiteImpactCode),
            b"SJL" => Some(OnSiteImpactCode),
            b"SJM" => Some(OnSiteProcessSystemTypeCode),
            b"SJN" => Some(OriginCode),
            b"SJO" => Some(ParameterCode),
            b"SJP" => Some(PassiveMitigationConsiderationCode),
            b"SJQ" => Some(PermitComplianceStatusCode),
            b"SJR" => Some(PhysicalStateCode),
            b"SJS" => Some(PointOfMeasurementCode),
            b"SJT" => Some(PreservativeCode),
            b"SJU" => Some(ProcessCode),
            b"SJV" => Some(ProcessControlCode),
            b"SJW" => Some(ProcessHazardAnalysisUpdateResultingChangeCode),
            b"SJX" => Some(ProcessHazardsAnalysisTechniqueCode),
            b"SJY" => Some(PublicReceptorCode),
            b"SJZ" => Some(RangeOfConcentrationCode),
            b"SKA" => Some(RecoveryMethodCode),
            b"SKB" => Some(RecyclingMethodCode),
            b"SKC" => Some(ReleaseEventCode),
            b"SKD" => Some(ReleaseSourceCode),
            b"SKE" => Some(ReliabilityIndicatorCode),
            b"SKF" => Some(RuleEffectivenessMethodCode),
            b"SKG" => Some(SampleTypeCode),
            b"SKH" => Some(ScenarioCode),
            b"SKI" => Some(SiteLocationCode),
            b"SKJ" => Some(SourceCategoryCode),
            b"SKK" => Some(SourceOfWasteGenerationCode),
            b"SKL" => Some(SourceReductionActivityCode),
            b"SKM" => Some(SystemTypeCode),
            b"SKN" => Some(TimePeriodCode),
            b"SKO" => Some(TopographyCode),
            b"SKP" => Some(TransferQuantityRangeCode),
            b"SKQ" => Some(TypeOfCompetencyTestingCode),
            b"SKR" => Some(TypeOfTrainingCode),
            b"SKS" => Some(TypeOfWasteManagementCode),
            b"SKT" => Some(UseCode),
            b"SKU" => Some(WasteEmanationCode),
            b"SKV" => Some(WasteManagementStatusCode),
            b"SKW" => Some(WasteStreamCode),
            b"SKX" => Some(WasteTreatmentMethodCode),
            b"SKY" => Some(WindDirectionCode),
            b"SKZ" => Some(UnitOfMeasureCode),
            b"SL" => Some(SecondarySourceOfInjury),
            b"SLA" => Some(ShelfLifeActionCode),
            b"SLC" => Some(StockageListCode),
            b"SLS" => Some(SchedulingStatus),
            b"SMB" => Some(StatementBasis),
            b"SMC" => Some(SpecialMaterialContentCode),
            b"SMD" => Some(SampleDevice),
            b"SMI" => Some(SpecialMaterialIdentificationCode),
            b"SMT" => Some(SampleType),
            b"SO" => Some(SolicitationCancellationReason),
            b"SP" => Some(StandardOccupationClassificationCode),
            b"SPD" => Some(SubmittersPriorityDesignator),
            b"SPE" => Some(SpecialDating),
            b"SR" => Some(StatisticalAdministrativeInformationCode),
            b"SRA" => Some(ServiceRequesterAgentIndicator),
            b"SRC" => Some(ListOfCodesIdentifyingDoDSerializedReportTypeCodes),
            b"SRL" => Some(SpecialRequirementLookup),
            b"SRR" => Some(SupplementalReductionReason),
            b"SRT" => Some(StorageReportType),
            b"SS" => Some(SystemStatus),
            b"SSC" => Some(SupplyStatusCode),
            b"ST" => Some(SpecialMarketingTypeCode),
            b"STC" => {
                Some(
                    AssociationOfAmericanRailroadsStandardTransportationCommodityCodeMasterDescription,
                )
            }
            b"STF" => Some(ForwardAndStoreApplicationErrorEditCodes),
            b"SUI" => Some(SolicitedUnsolicitedIndicator),
            b"SVC" => Some(ServiceCode),
            b"SWR" => Some(AssociationOfAmericanRailroadsSwitchReleaseCodes),
            b"T" => Some(PersonalPropertyAndContentsCode),
            b"T00" => Some(CommercialVehicleOperationsSafetyCode),
            b"T01" => Some(DataCategories),
            b"T02" => Some(EventCodes),
            b"T03" => Some(OperationType),
            b"T04" => Some(AccidentParameters),
            b"T05" => Some(InspectionParameters),
            b"T06" => Some(DriverParameters),
            b"T07" => Some(ViewParameters),
            b"T08" => Some(VehicleParameters),
            b"T09" => Some(FleetParameters),
            b"T10" => Some(QueryOptions),
            b"T11" => Some(JurisdictionType),
            b"T12" => Some(SingleStateRegistrationSystemAndOperatingAuthorityCredential),
            b"T13" => Some(CommercialVehicleOperationsInsurance),
            b"T14" => Some(CommercialVehicleRegistration),
            b"T15" => Some(HazardousMaterialsCredential),
            b"T16" => Some(OversizeOverweightCredential),
            b"T17" => Some(CommercialVehicleTax),
            b"T18" => Some(CommercialVehicleTitle),
            b"T19" => Some(CommercialDriversLicense),
            b"T20" => Some(CommercialVehicleType),
            b"T21" => Some(CommercialVehicleOperationsStatusCode),
            b"T22" => Some(SafetyAndFitnessElectronicRecordSystemsSubscriptionOption),
            b"T23" => Some(CommercialVehicleOperationsCommodityCode),
            b"T24" => Some(CommercialVehicleOperationsHazardousMaterialCode),
            b"T25" => Some(SafetyAndFitnessElectronicRecordSystemsErrorCode),
            b"T26" => Some(CommercialVehicleOperationsJurisdictionIdentifierCode),
            b"T27" => Some(ComplianceReviewCode),
            b"T28" => Some(IncidentConditionCode),
            b"T29" => Some(IncidentRelatedActionCode),
            b"T30" => Some(IncidentLocationCode),
            b"T31" => Some(IncidentConsequenceCode),
            b"T32" => Some(RoadCharacteristicCode),
            b"T33" => Some(VehicleOccupantCode),
            b"T34" => Some(PackageFailureCode),
            b"T35" => Some(PedestrianCode),
            b"TB" => {
                Some(AssociationForFinancialProfessionalsServiceCodeAndBankServiceCode)
            }
            b"TC" => Some(TreatmentCodes),
            b"TC1" => Some(TypeOfChangeCode),
            b"TCD" => Some(CodeTCD),
            b"TCL" => Some(TemplateCharacteristicLookup),
            b"TD" => Some(CodeTD),
            b"TDC" => Some(DiscrepancyReportTypeCode),
            b"TDJ" => Some(TechnicalDataJustificationCode),
            b"TE" => Some(AssociationForFinancialProfessionalsServiceCode),
            b"TF" => Some(TitleExceptionAndRequirementCodeList),
            b"TFR" => Some(TaxOrFeeExemptionReasonCode),
            b"TG" => Some(TitleDocumentCodeList),
            b"TL" => Some(TapLocation),
            b"TOC" => Some(WeaponSystemTransactionOriginationCode),
            b"TOL" => Some(TemplateOwnerLookup),
            b"TP" => Some(TapType),
            b"TQ" => Some(CodeTQ),
            b"TR" => Some(ReportCode),
            b"TT" => Some(NaturalGasTransactionType),
            b"TTD" => Some(DownstreamTransactionType),
            b"TTL" => Some(TemplateTypeLookup),
            b"TTU" => Some(UpstreamTransactionType),
            b"TX" => Some(FollowUpCode),
            b"TY" => Some(ReportableEventCode),
            b"U" => Some(ResidentialAndCommercialRoomCode),
            b"UBT" => Some(UmlerBodyType),
            b"UDC" => Some(UniqueItemTrackingDesignatorCode),
            b"UER" => Some(UniqueItemTrackingErrorRejectCode),
            b"UFC" => Some(UmlerFittingCode),
            b"UJC" => Some(UrgencyJustificationCode),
            b"UNP" => Some(CodeUNP),
            b"UPC" => Some(CodeUPC),
            b"UPT" => Some(UnclaimedPropertyTypeCode),
            b"UR" => Some(EventReappearanceCode),
            b"US" => Some(EventAbatementCode),
            b"UT" => Some(CodeUT),
            b"UTC" => Some(UniqueItemTrackingTransactionCode),
            b"UU" => Some(UnitCode),
            b"V" => Some(ViolationTypeCodeList),
            b"VAL" => Some(ValidationCode),
            b"VP" => Some(CodeVP),
            b"W" => Some(CourtIssuedWarrantTypeCode),
            b"WAC" => Some(WeaponSystemAdviceCode),
            b"WDL" => Some(WhenDiscoveredLookup),
            b"WEC" => Some(WeaponSystemEssentialityCode),
            b"WRC" => Some(CodeWRC),
            b"WSC" => Some(WeaponSystemStatusCode),
            b"WSD" => Some(WeaponSystemDesignatorCode),
            b"WSM" => Some(WeaponSystemMaintenanceCode),
            b"X" => Some(VehicleClass),
            b"Y" => Some(RentalCharge),
            b"Z" => Some(CancellationReason),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use CodeListQualifierCode::*;
        match self {
            DocumentIdentificationCode => "Document Identification Code",
            FreeOnBoardSiteCode => "Free On Board Site Code",
            ChannelOfDistributionCode => "Channel of Distribution Code",
            KindOfContractCode => "Kind of Contract Code",
            TypeOfContractCode => "Type of Contract Code",
            CriticalityDesignatorCode => "Criticality Designator Code",
            QualityAssuranceSiteCode => "Quality Assurance Site Code",
            AcceptanceSiteCode => "Acceptance Site Code",
            TransactionStatusIndicatorCode => "Transaction Status Indicator Code",
            ContractDeliveryDateRevisionAgentCode => {
                "Contract Delivery Date Revision Agent Code"
            }
            ReasonForContractDeliveryDateRevisionCode => {
                "Reason for Contract Delivery Date Revision Code"
            }
            RecommendationsRegardingDelayedDeliveriesCode => {
                "Recommendations Regarding Delayed Deliveries Code"
            }
            ContractShipmentAdviceCode => "Contract Shipment Advice Code",
            IndividualInsuranceFinancialDetail => "Individual Insurance Financial Detail",
            CashDiscountStipulationCode => "Cash Discount Stipulation Code",
            ShipmentAcceptanceDiscrepancyExplanationCode => {
                "Shipment Acceptance Discrepancy Explanation Code"
            }
            InsurancePlanDescriptionCharacteristics => {
                "Insurance Plan Description Characteristics"
            }
            ContractCloseOutGroupCode => "Contract Close-out Group Code",
            PaymentTypeCode => "Payment Type Code",
            ContractFundReportingTransactionCode => {
                "Contract Fund Reporting Transaction Code"
            }
            ContractPaymentDeductionOrCollectionCode => {
                "Contract Payment Deduction or Collection Code"
            }
            ObligationVarianceCode => "Obligation Variance Code",
            PlusOrMinusIndicatorCode => "Plus or Minus Indicator Code",
            ReasonForDelayedClosingOfContractFileCode => {
                "Reason for Delayed Closing of Contract File Code"
            }
            ContractPaymentLineItemStatusCode => "Contract Payment Line Item Status Code",
            SpecialReimbursableProvisionsCode => "Special Reimbursable Provisions Code",
            KindOfModificationCode => "Kind of Modification Code",
            Code29 => "Purchasing Contract Officer (PCO) Instructions Code",
            TypeOfDelayCode => "Type of Delay Code",
            HealthcareProviderCharacteristicsAndResources => {
                "Healthcare Provider Characteristics and Resources"
            }
            ContainerAndRollOnRollOffNumberCode => {
                "Container and Roll-on/Roll-off Number Code"
            }
            AirCommodityAndSpecialHandlingCode => {
                "Air Commodity and Special Handling Code"
            }
            WaterCommodityAndSpecialHandlingCode => {
                "Water Commodity and Special Handling Code"
            }
            AirDimensionCode => "Air Dimension Code",
            AirTerminalIdentifierCode => "Air Terminal Identifier Code",
            WaterTerminalIdentifierCode => "Water Terminal Identifier Code",
            ConsolidationAndContainerizationPointCode => {
                "Consolidation and Containerization Point Code"
            }
            TransportationModeOrMethodCode => "Transportation Mode or Method Code",
            TypePackCode => "Type Pack Code",
            DateShippedOrReceivedCode => "Date Shipped or Received Code",
            EstimatedTimeOfArrivalCode => "Estimated Time of Arrival Code",
            MilitaryAndCivilianGradeCode => "Military and Civilian Grade Code",
            SeavanOwnershipCode => "Seavan Ownership Code",
            OceanCarrierCode => "Ocean Carrier Code",
            VoyageDocumentNumberCode => "Voyage Document Number Code",
            VoyageManifestReferenceCode => "Voyage Manifest Reference Code",
            VesselStatusAndTermsOfCarriageCode => {
                "Vessel Status and Terms of Carriage Code"
            }
            VesselSustainingCode => "Vessel Sustaining Code",
            SubrogationActionCode => "Subrogation Action Code",
            BillingAdviceCode => "Billing Advice Code",
            BillingStatusCode => "Billing Status Code",
            TypeOfBillCode => "Type of Bill Code",
            RecipientOfBillingStatusCode => "Recipient of Billing Status Code",
            SalesPriceConditionCode => "Sales Price Condition Code",
            DeliverySourceCode => "Delivery Source Code",
            TransportationBillCode => "Transportation Bill Code",
            StockFundOrNonStockFundCode => "Stock Fund or Non-stock Fund Code",
            Code60 => {
                "General Services Administration (GSA) Customer Supply Center Number Code"
            }
            InformationIndicatorCode => "Information Indicator Code",
            CommunicationsRoutingIdentifierCode => {
                "Communications Routing Identifier Code"
            }
            ContentIndicatorCode => "Content Indicator Code",
            HealthCareClaimStatusCode => "Health Care Claim Status Code",
            SuffixOrLimitCode => "Suffix or Limit Code",
            TypeOfAssistanceCode => "Type of Assistance Code",
            HealthcareProviderTaxonomy => "Healthcare Provider Taxonomy",
            ForeignMilitarySalesCountryCode => "Foreign Military Sales Country Code",
            ServiceAndAgencyCode => "Service and Agency Code",
            DisbursementStatusCode => "Disbursement Status Code",
            AidTypeCode => "Aid Type Code",
            DemandCode => "Demand Code",
            SuffixCode => "Suffix Code",
            ProjectCode => "Project Code",
            PriorityDesignatorCode => "Priority Designator Code",
            AdviceCode => "Advice Code",
            StatusCode => "Status Code",
            ShipmentHoldCode => "Shipment Hold Code",
            SupplyConditionCode => "Supply Condition Code",
            ManagementCode => "Management Code",
            CountryAndActivityCode => "Country and Activity Code",
            SubsistenceTypeOfPackCode => "Subsistence Type of Pack Code",
            DisposalAuthorityCode => "Disposal Authority Code",
            CooperativeLogisticsProgramSupportCode => {
                "Cooperative Logistics Program Support Code"
            }
            PreciousMetalsIndicatorCode => "Precious Metals Indicator Code",
            AutomatedDataProcessingEquipmentIdentificationCode => {
                "Automated Data Processing Equipment Identification Code"
            }
            ReasonForDisposalCode => "Reason for Disposal Code",
            TypeOfStorageCode => "Type of Storage Code",
            IdentificationCode => "Identification Code",
            OfferAndReleaseOptionCode => "Offer and Release Option Code",
            ShipmentReleaseCode => "Shipment Release Code",
            UltimateRecipientCode => "Ultimate Recipient Code",
            ReasonForRequisitioningCode => "Reason for Requisitioning Code",
            PurposeCode => "Purpose Code",
            Code100 => {
                "Freddie Mac (Federal Home Loan Mortgage Corporation) Special Character Code"
            }
            Code101 => {
                "Fannie Mae (Federal National Mortgage Association) Special Feature Code"
            }
            MortgageIndexSourceCode => "Mortgage Index Source Code",
            Code103 => {
                "Fannie Mae (Federal National Mortgage Association) Remittance Programs"
            }
            Code104 => {
                "Freddie Mac (Federal Home Loan Mortgage Corporation) Remittance Programs"
            }
            Code105 => {
                "Freddie Mac (Federal Home Loan Mortgage Corporation) Mortgage Insurance Code"
            }
            Code106 => {
                "Fannie Mae (Federal National Mortgage Association) Pool Feature Code"
            }
            Code107 => {
                "Fannie Mae (Federal National Mortgage Association) Mortgage Insurance Code"
            }
            TestingStatisticalCategoryCodeList => {
                "Testing Statistical Category Code List"
            }
            TestingDemographicCategoryCodeList => {
                "Testing Demographic Category Code List"
            }
            CodeA => "American Society for Testing and Materials (ASTM)",
            OwnershipCode => "Ownership Code",
            CustomerWithinCountryCode => "Customer Within Country Code",
            DeliveryTermCode => "Delivery Term Code",
            CaseDesignatorNumber => "Case Designator Number",
            SubcaseNumber => "Subcase Number",
            FreightForwarderNumber => "Freight Forwarder Number",
            RecordControlNumber => "Record Control Number",
            ProgramYearCode => "Program Year Code",
            SupplementalData => "Supplemental Data",
            CodeAA => "Country Code (Finance and Acquisition)",
            CodeAAA => "SNOMED, Systematized Nomenclature of Medicine",
            AssetType => "Asset Type",
            CurrentAssetType => "Current Asset Type",
            CurrentLiabilityType => "Current Liability Type",
            DunAndBradstreetCanadas8DigitStandardIndustrialClassificationCode => {
                "Dun and Bradstreet Canada's 8 digit Standard Industrial Classification Code"
            }
            FinancialItemAllocationCode => "Financial Item Allocation Code",
            FinancialItemAttributedCode => "Financial Item Attributed Code",
            FinancialItemReclassificationCode => "Financial Item Reclassification Code",
            FunctionalArea => "Functional Area",
            HobbyCode => "Hobby Code",
            InvestmentType => "Investment Type",
            LiabilityType => "Liability Type",
            ProjectionType => "Projection Type",
            TrendReason => "Trend Reason",
            CodeAAQ => "NACHA (National Automated Clearing House Association)",
            CodeAAR => "CPA (Canadian Payments Association)",
            Proprietary => "Proprietary",
            FannieMaeAdjustableRateMortgagePlanCodes => {
                "Fannie Mae Adjustable Rate Mortgage Plan Codes"
            }
            CodeAAU => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Diagnosis Encountered During Examination and Investigation of Individuals and Populations Code"
            }
            CodeAAV => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Vaccination, Innoculation or Isolation Code"
            }
            ImmunizationInjectionCode => "Immunization Injection Code",
            CodeAAX => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Code"
            }
            CodeAAY => "Current Dental Terminology (CDT) Code",
            DefensePrioritiesAndAllocationsSystemCode => {
                "Defense Priorities and Allocations System Code"
            }
            CodeABF => {
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Diagnosis"
            }
            CodeABJ => {
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Admitting Diagnosis"
            }
            CodeABK => {
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Principal Diagnosis"
            }
            CodeABN => {
                "International Classification of Diseases Clinical Modification (ICD-10-CM) External Cause of Injury Code"
            }
            AssignedByReceiver => "Assigned by Receiver",
            AssignedBySender => "Assigned by Sender",
            CodeABU => {
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Diagnosis Encountered During Examination and Investigation of Individuals and Populations Code"
            }
            CodeABV => {
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Vaccination, Innoculation or Isolation Code"
            }
            AccountCharacteristicsCode => "Account Characteristics Code",
            AccountingErrorClassificationCode => "Accounting Error Classification Code",
            AcademicRank => "Academic Rank",
            ListOfDoDAccountingRequirementsCodes => {
                "List of DoD Accounting Requirements Codes."
            }
            ListOfDoDAssetCategoryCodes => "List of DoD Asset Category Codes",
            ListOfDoDControlledItemCodes => "List of DoD Controlled Item Codes.",
            CodeACV => {
                "List of DoD Expendability, Recoverability, Reparability Category (ERRC) Codes."
            }
            ListOfDoDFiduciaryDepreciationMethodCodes => {
                "List of DoD Fiduciary Depreciation Method Codes"
            }
            CodeACX => {
                "List of DoD National Item Identification Number (NIIN) Status Codes"
            }
            CodeACY => "Add code ACY, \"List of DoD (Army) Recoverability Codes\"",
            CodeACZ => "List of DoD Reportable Item Control Codes (RICC)",
            AcquisitionAdviceCode => "Acquisition Advice Code",
            ListOfDoDStorageRequirementCodes => "List of DoD Storage Requirement Codes",
            ListOfDoDTemperatureControlledCodes => {
                "List of DoD Temperature Controlled Codes"
            }
            ListOfDoDAssetTypeCodes => "List of DoD Asset Type Codes",
            ListOfDoDUtilizationMeasureCodes => "List of DoD Utilization Measure Codes",
            CodeADD => {
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Primary Diagnosis"
            }
            AccountingAdjustmentMethod => "Accounting Adjustment Method",
            BeneficiaryType => "Beneficiary Type",
            ArmyEditActionCode => "Army Edit Action Code",
            ClassOfPitch => "Class of Pitch",
            GradeOfDifficulty => "Grade of Difficulty",
            AcquisitionMethodSuffixCode => "Acquisition Method Suffix Code",
            AcquisitionMethodCode => "Acquisition Method Code",
            UtilizationCode => "Utilization Code",
            AdjustmentType => "Adjustment Type",
            DistributionCode => "Distribution Code",
            SpecialRequirementsCode => "Special Requirements Code",
            AllocationMethod => "Allocation Method",
            AlterationLookup => "Alteration Lookup",
            LocaleOfActivity => "Locale of Activity",
            NatureOfEventCode => "Nature of Event Code",
            SettlementPayoutOptions => "Settlement/Payout Options",
            AuthorizedOverrunIndicator => "Authorized Overrun Indicator",
            CodeAPE => "Activite Principale Exercee (APE) Code",
            CodeAPR => {
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Patient's Reason for Visit"
            }
            ApplicationQuestion => "Application Question Identifier",
            AllQuantityAvailableIndicator => "All Quantity Available Indicator",
            ArrestReason => "Arrest Reason",
            AssetReclassificationDenialCode => "Asset Reclassification Denial Code",
            AllocationRankIndicator => "Allocation Rank Indicator",
            AllocationRankLevel => "Allocation Rank Level",
            FormTypeCode => "Form Type Code",
            CodeASD => {
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Secondary Diagnosis"
            }
            AllegationTypeCode => "Allegation Type Code",
            CodeATD => {
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Tertiary Diagnosis"
            }
            AllocationTransactionTypeCode => "Allocation Transaction Type Code",
            CodeAU => "All Patient Refined Diagnosis Related Groups (APR-DRG)",
            SubrogationPaymentOptions => "Subrogation Payment Options",
            CodeAW => "All Patient Diagnosis Related Groups (AP-DRG)",
            CodeAX => "Ambulatory Patient Groups (APG)",
            SubrogationResponseCodes => "Subrogation Response Codes",
            SubrogationRequestCodes => "Subrogation Request Codes",
            CodeB => "Bank Administration Institute (BAI)",
            VesselStowageLocationCode => "Vessel Stowage Location Code",
            BusinessType => "Business Type",
            CodeBBF => {
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Diagnosis"
            }
            CodeBBJ => {
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Admitting Diagnosis"
            }
            CodeBBK => {
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Principal Diagnosis"
            }
            CodeBBN => {
                "International Classification of Diseases Clinical Modification (ICD-11-CM) External Cause of Injury Code"
            }
            CodeBBQ => {
                "International Classification of Diseases Clinical Modification (ICD-10-PCS) Other Procedure Codes"
            }
            CodeBBR => {
                "International Classification of Diseases Clinical Modification (ICD-10-PCS) Principal Procedure Codes"
            }
            CodeBBU => {
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Diagnosis Encountered During Examination and Investigation of Individuals and Populations Code"
            }
            CodeBBV => {
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Vaccination, Innoculation or Isolation Code"
            }
            TransportationHoldingDelayCode => "Transportation Holding Delay Code",
            BusinessChangeCode => "Business Change Code",
            BusinessCreditRating => "Business Credit Rating",
            TransportationPriorityCode => "Transportation Priority Code",
            CodeBDD => {
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Primary Diagnosis"
            }
            Value => "Value",
            CodeBF => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Diagnosis"
            }
            Condition => "Condition",
            Occurrence => "Occurrence",
            OccurrenceSpan => "Occurrence Span",
            CodeBJ => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Admitting Diagnosis"
            }
            CodeBK => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Principal Diagnosis"
            }
            ApplicationFeeStatusCodes => "Application Fee Status Codes",
            CodeBN => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) External Cause of Injury Code (E-codes)"
            }
            HealthcareCommonProcedureCodingSystem => {
                "Healthcare Common Procedure Coding System"
            }
            HealthcareCommonProcedureCodingSystemPrincipalProcedure => {
                "Healthcare Common Procedure Coding System Principal Procedure"
            }
            BoardOfInspectionAndSurveyPartLookup => {
                "Board of Inspection and Survey Part Lookup"
            }
            CodeBPR => {
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Patient's Reason for Visit"
            }
            CodeBQ => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Other Procedure Codes"
            }
            CodeBR => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Principal Procedure Codes"
            }
            BoardOfInspectionAndSurveyResponsibilityLookup => {
                "Board of Inspection and Survey Responsibility Lookup"
            }
            CodeBS => "Current Procedural Terminology (CPT) Codes",
            CodeBSD => {
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Secondary Diagnosis"
            }
            BoardOfInspectionAndSurveyShipLookup => {
                "Board of Inspection and Survey Ship Lookup"
            }
            BusinessPeriod => "Business Period",
            AccidentDescription => "Accident Description",
            BalanceTypeCode => "Balance Type Code",
            CodeBTD => {
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Tertiary Diagnosis"
            }
            PartOfBodyAffected => "Part of Body Affected",
            BidUpIndicator => "Bid Up Indicator",
            BureauOfLaborStatisticsStandardizedOccupationalCodes => {
                "Bureau of Labor Statistics Standardized Occupational Codes"
            }
            EducationInstitutionTypeCode => "Education Institution Type Code",
            EducationalAreasCode => "Educational Areas Code",
            ProfessionTypeCode => "Profession Type Code",
            ShareTypeCode => "Share Type Code",
            BusinessSizeCode => "Business Size Code",
            CodeC => "Canadian Inter*EDI",
            EyeColorCode => "Eye Color Code",
            HairColorCode => "Hair Color Code",
            SkinToneCode => "Skin Tone Code",
            TypeOfInquiryCode => "Type of Inquiry Code",
            CodeCAH => "Advanced Billing Concepts (ABC) Codes",
            BilledOfficeIndicatorCode => "Billed Office Indicator Code",
            CodeCBQ => {
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Other Procedure Codes"
            }
            CodeCBR => {
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Principal Procedure Codes"
            }
            TreasurySymbolCode => "Treasury Symbol Code",
            CorrectionToCauseCode => "Correction To Cause Code",
            SupplementaryAccountingClassificationCode => {
                "Supplementary Accounting Classification Code"
            }
            ReferenceAndStationCode => "Reference and Station Code",
            MajorForceProgramCode => "Major Force Program Code",
            ContractualFlowIndicator => "Contractual Flow Indicator",
            AircraftMissionDesignSeriesCode => "Aircraft Mission Design Series Code",
            TypeOfIssueCode => "Type of Issue Code",
            ChargeIndicator => "Charge Indicator",
            CriminalCharge => "Criminal Charge",
            CodeCIE => {
                "Collision Industry Electronic Commerce Association (CIECA) - Assignment Type"
            }
            CriminalChargeGrade => "Criminal Charge Grade",
            CouponAdjustmentReasonCode => "Coupon Adjustment Reason Code",
            CountyDesignatorCode => "County Designator Code",
            CauseLookup => "Cause Lookup",
            FinancialManagementServiceCashLinkCode => {
                "Financial Management Service Cash-Link Code"
            }
            CustomerMaintenanceLevelLookup => "Customer Maintenance Level Lookup",
            CauseOfInjuryCode => "Cause of Injury Code",
            ChangeNoticeCode => "Change Notice Code",
            CustomizedNoticeTypeCode => "Customized Notice Type Code",
            CognizanceSymbol => "Cognizance Symbol",
            ConfirmingPartyRole => "Confirming Party Role",
            SalvageDispositionCode => "Salvage Disposition Code",
            CourtPartyStatus => "Court Party Status",
            CapacityTypeIndicator => "Capacity Type Indicator",
            CodeCR => "Federal Item Identification Guide Criticality (FIIG) Code",
            ComplaintRequestCode => "Complaint Request Code",
            CausativeResearchIndicatorCode => "Causative Research Indicator Code",
            ClauseStatusType => "Clause Status Type",
            CustomerServiceDesignator => "Customer Service Designator",
            CorporateStatementFilingCode => "Corporate Statement Filing Code",
            CompensationTypeCodes => "Compensation Type Codes",
            CarcassTrackingCode => "Carcass Tracking Code",
            CuisineTypeCode => "Cuisine Type Code",
            CoverageCodeList => "Coverage Code List",
            ControvertCode => "Controvert Code",
            ConvictionOffenseType => "Conviction Offense Type",
            CourtDocumentTypeCode => "Court Document Type Code",
            DriversLicenseWithdrawalType => "Driver's License Withdrawal Type",
            DriversLicenseWithdrawalExtent => "Driver's License Withdrawal Extent",
            DriversLicenseWithdrawalBasis => "Driver's License Withdrawal Basis",
            DriversLicenseWithdrawalDueProcessStatus => {
                "Driver's License Withdrawal Due Process Status"
            }
            DriversLicenseWithdrawalReason => "Driver's License Withdrawal Reason",
            DeviceAvailabilityCode => "Device Availability Code",
            DocumentAvailabilityCode => "Document Availability Code",
            CodeDAP => "All Patient, Severity-Adjusted DRGs (APS-DRG)",
            DebtorBusinessTypeCode => "Debtor Business Type Code",
            CodeDBS => {
                "DUN's Standard Industrial Classification (SIC) 2+2, Dun and Bradstreet"
            }
            ReportDistributionCode => "Report Distribution Code",
            CauseCode => "Cause Code",
            CodeDCM => "Medicare DRG (CMS-DRG & MS-DRG)",
            DispositionCategoryChangeRejectReasonCode => {
                "Disposition Category Change Reject Reason Code"
            }
            DispositionSubCategoryCode => "Disposition Sub-Category Code",
            DispositionCategoryCode => "Disposition Category Code",
            CodeDD => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Primary Diagnosis"
            }
            SignalCode => "Signal Code",
            MediaAndStatusCode => "Media and Status Code",
            FundCode => "Fund Code",
            DynamicGeneratorSetCode => "Dynamic Generator Set Code",
            DrugDetailCode => "Drug Detail Code",
            SingleUseLabelCode => "Single Use Label Code",
            CodeDIR => "International-Refined DRGs (IR-DRG)",
            RemedialActionCode => "Remedial Action Code",
            ProgramOriginatorCode => "Program Originator Code",
            ServiceContractActOperationCode => "Service Contract Act Operation Code",
            DynamicLocomotiveTagCode => "Dynamic Locomotive Tag Code",
            DeferralLookup => "Deferral Lookup",
            LongTermCareDrgLtcDrg => "Long Term Care DRG - LTC-DRG",
            AgentStatusCode => "Agent Status Code",
            DemilitarizationIntegrityCode => "Demilitarization Integrity Code",
            DemilitarizationPerformedCode => "Demilitarization Performed Code",
            NatureOfDebtCode => "Nature of Debt Code",
            DocumentNumberRequirementType => "Document Number Requirement Type",
            DeviceOperatorTypeCode => "Device Operator Type Code",
            DirectionOfFlow => "Direction of Flow",
            ProducerFinancialHistoryCodes => "Producer Financial History Codes",
            DeliveryPriorityCode => "Delivery Priority Code",
            AssociationOfAmericanRailroadsDeprescriptionExceptionList => {
                "Association of American Railroads Deprescription Exception List"
            }
            AssociationOfAmericanRailroadsDeprescriptionDistributionList => {
                "Association of American Railroads Deprescription Distribution List"
            }
            DeviceStatusCode => "Device Status Code",
            CodeDR => "Diagnosis Related Group (DRG)",
            CodeDRD => "Refined DRGs (R-DRG)",
            CodeDRL => {
                "Collision Industry Electronic Commerce Association (CIECA) - Detail Repair Lines Code List"
            }
            DispositionServicesReimbursementCode => {
                "Disposition Services Reimbursement Code"
            }
            RelatedDeviceApplicabilityCode => "Related Device Applicability Code",
            DispositionServicesCustomerTypeCode => {
                "Disposition Services Customer Type Code"
            }
            CodeDSD => "Severity DRGs (S-DRG)",
            DispositionServicesIndicatorCode => "Disposition Services Indicator Code",
            DataSetsRequested => "Data Sets Requested",
            DeliverySchedulingStatus => "Delivery Scheduling Status",
            DebtorTypeCode => "Debtor Type Code",
            ListOfCodesIdentifyingDoDDistributionServicesTermsOfSale => {
                "List of codes identifying DoD Distribution Services Terms of Sale."
            }
            DeviceUsageCode => "Device Usage Code",
            DemandPlanningStatusCode => "Demand Planning Status Code",
            EstimatingMethodStatusCode => "Estimating Method Status Code",
            ContactStatusCode => "Contact Status Code",
            TypeOfFirmCode => "Type of Firm Code",
            ReportableEventStatusCode => "Reportable Event Status Code",
            CodeE => "Diagnostic Statistical Manual of Mental Disorders Code List (DSM)",
            AssetStatusOrTransactionReportingCode => {
                "Asset Status or Transaction Reporting Code"
            }
            AlabamaCampaignDisclosureReportCodes => {
                "Alabama Campaign Disclosure Report Codes"
            }
            AlaskaCampaignDisclosureReportCodes => {
                "Alaska Campaign Disclosure Report Codes"
            }
            AmericanSamoaCampaignDisclosureReportCodes => {
                "American Samoa Campaign Disclosure Report Codes"
            }
            ArizonaCampaignDisclosureReportCodes => {
                "Arizona Campaign Disclosure Report Codes"
            }
            ArkansasCampaignDisclosureReportCodes => {
                "Arkansas Campaign Disclosure Report Codes"
            }
            CaliforniaCampaignDisclosureReportCodes => {
                "California Campaign Disclosure Report Codes"
            }
            ColoradoCampaignDisclosureReportCodes => {
                "Colorado Campaign Disclosure Report Codes"
            }
            ConnecticutCampaignDisclosureReportCodes => {
                "Connecticut Campaign Disclosure Report Codes"
            }
            DelawareCampaignDisclosureReportCodes => {
                "Delaware Campaign Disclosure Report Codes"
            }
            DistrictOfColumbiaCampaignDisclosureReportCodes => {
                "District of Columbia Campaign Disclosure Report Codes"
            }
            FloridaCampaignDisclosureReportCodes => {
                "Florida Campaign Disclosure Report Codes"
            }
            GeorgiaCampaignDisclosureReportCodes => {
                "Georgia Campaign Disclosure Report Codes"
            }
            GuamCampaignDisclosureReportCodes => "Guam Campaign Disclosure Report Codes",
            HawaiiCampaignDisclosureReportCodes => {
                "Hawaii Campaign Disclosure Report Codes"
            }
            IdahoCampaignDisclosureReportCodes => {
                "Idaho Campaign Disclosure Report Codes"
            }
            IllinoisCampaignDisclosureReportCodes => {
                "Illinois Campaign Disclosure Report Codes"
            }
            IndianaCampaignDisclosureReportCodes => {
                "Indiana Campaign Disclosure Report Codes"
            }
            IowaCampaignDisclosureReportCodes => "Iowa Campaign Disclosure Report Codes",
            KansasCampaignDisclosureReportCodes => {
                "Kansas Campaign Disclosure Report Codes"
            }
            KentuckyCampaignDisclosureReportCodes => {
                "Kentucky Campaign Disclosure Report Codes"
            }
            LouisianaCampaignDisclosureReportCodes => {
                "Louisiana Campaign Disclosure Report Codes"
            }
            MaineCampaignDisclosureReportCodes => {
                "Maine Campaign Disclosure Report Codes"
            }
            MarylandCampaignDisclosureReportCodes => {
                "Maryland Campaign Disclosure Report Codes"
            }
            MassachusettsCampaignDisclosureReportCodes => {
                "Massachusetts Campaign Disclosure Report Codes"
            }
            MichiganCampaignDisclosureReportCodes => {
                "Michigan Campaign Disclosure Report Codes"
            }
            MinnesotaCampaignDisclosureReportCodes => {
                "Minnesota Campaign Disclosure Report Codes"
            }
            AssetTransferStatusCode => "Asset Transfer Status Code",
            MississippiCampaignDisclosureReportCodes => {
                "Mississippi Campaign Disclosure Report Codes"
            }
            MissouriCampaignDisclosureReportCodes => {
                "Missouri Campaign Disclosure Report Codes"
            }
            MontanaCampaignDisclosureReportCodes => {
                "Montana Campaign Disclosure Report Codes"
            }
            NebraskaCampaignDisclosureReportCodes => {
                "Nebraska Campaign Disclosure Report Codes"
            }
            NevadaCampaignDisclosureReportCodes => {
                "Nevada Campaign Disclosure Report Codes"
            }
            NewHampshireCampaignDisclosureReportCodes => {
                "New Hampshire Campaign Disclosure Report Codes"
            }
            NewJerseyCampaignDisclosureReportCodes => {
                "New Jersey Campaign Disclosure Report Codes"
            }
            NewMexicoCampaignDisclosureReportCodes => {
                "New Mexico Campaign Disclosure Report Codes"
            }
            NewYorkCampaignDisclosureReportCodes => {
                "New York Campaign Disclosure Report Codes"
            }
            NorthCarolinaCampaignDisclosureReportCodes => {
                "North Carolina Campaign Disclosure Report Codes"
            }
            NorthDakotaCampaignDisclosureReportCodes => {
                "North Dakota Campaign Disclosure Report Codes"
            }
            OhioCampaignDisclosureReportCodes => "Ohio Campaign Disclosure Report Codes",
            OklahomaCampaignDisclosureReportCodes => {
                "Oklahoma Campaign Disclosure Report Codes"
            }
            OregonCampaignDisclosureReportCodes => {
                "Oregon Campaign Disclosure Report Codes"
            }
            PennsylvaniaCampaignDisclosureReportCodes => {
                "Pennsylvania Campaign Disclosure Report Codes"
            }
            PuertoRicoCampaignDisclosureReportCodes => {
                "Puerto Rico Campaign Disclosure Report Codes"
            }
            RhodeIslandCampaignDisclosureReportCodes => {
                "Rhode Island Campaign Disclosure Report Codes"
            }
            SouthCarolinaCampaignDisclosureReportCodes => {
                "South Carolina Campaign Disclosure Report Codes"
            }
            SouthDakotaCampaignDisclosureReportCodes => {
                "South Dakota Campaign Disclosure Report Codes"
            }
            TennesseeCampaignDisclosureReportCodes => {
                "Tennessee Campaign Disclosure Report Codes"
            }
            TexasCampaignDisclosureReportCodes => {
                "Texas Campaign Disclosure Report Codes"
            }
            UtahCampaignDisclosureReportCodes => "Utah Campaign Disclosure Report Codes",
            VermontCampaignDisclosureReportCodes => {
                "Vermont Campaign Disclosure Report Codes"
            }
            VirginiaCampaignDisclosureReportCodes => {
                "Virginia Campaign Disclosure Report Codes"
            }
            VirginIslandsCampaignDisclosureReportCodes => {
                "Virgin Islands Campaign Disclosure Report Codes"
            }
            WashingtonCampaignDisclosureReportCodes => {
                "Washington Campaign Disclosure Report Codes"
            }
            CertificationRequirementsCode => "Certification Requirements Code",
            WestVirginiaCampaignDisclosureReportCodes => {
                "West Virginia Campaign Disclosure Report Codes"
            }
            WisconsinCampaignDisclosureReportCodes => {
                "Wisconsin Campaign Disclosure Report Codes"
            }
            WyomingCampaignDisclosureReportCodes => {
                "Wyoming Campaign Disclosure Report Codes"
            }
            AlbertaCampaignDisclosureReportCodes => {
                "Alberta Campaign Disclosure Report Codes"
            }
            BritishColumbiaCampaignDisclosureReportCodes => {
                "British Columbia Campaign Disclosure Report Codes"
            }
            ManitobaCampaignDisclosureReportCodes => {
                "Manitoba Campaign Disclosure Report Codes"
            }
            NewBrunswickCampaignDisclosureReportCodes => {
                "New Brunswick Campaign Disclosure Report Codes"
            }
            NewfoundlandCampaignDisclosureReportCodes => {
                "Newfoundland Campaign Disclosure Report Codes"
            }
            NorthwestTerritoriesCampaignDisclosureReportCodes => {
                "Northwest Territories Campaign Disclosure Report Codes"
            }
            NovaScotiaCampaignDisclosureReportCodes => {
                "Nova Scotia Campaign Disclosure Report Codes"
            }
            OntarioCampaignDisclosureReportCodes => {
                "Ontario Campaign Disclosure Report Codes"
            }
            PrinceEdwardIslandCampaignDisclosureReportCodes => {
                "Prince Edward Island Campaign Disclosure Report Codes"
            }
            QuebecCampaignDisclosureReportCodes => {
                "Quebec Campaign Disclosure Report Codes"
            }
            SaskatchewanCampaignDisclosureReportCodes => {
                "Saskatchewan Campaign Disclosure Report Codes"
            }
            YukonTerritoryCampaignDisclosureReportCodes => {
                "Yukon Territory Campaign Disclosure Report Codes"
            }
            FederalCampaignDisclosureReportCodes => {
                "Federal Campaign Disclosure Report Codes"
            }
            AlabamaLobbyistReportCodes => "Alabama Lobbyist Report Codes",
            AlaskaLobbyistReportCodes => "Alaska Lobbyist Report Codes",
            ArizonaLobbyistReportCodes => "Arizona Lobbyist Report Codes",
            ArkansasLobbyistReportCodes => "Arkansas Lobbyist Report Codes",
            CaliforniaLobbyistReportCodes => "California Lobbyist Report Codes",
            ColoradoLobbyistReportCodes => "Colorado Lobbyist Report Codes",
            ConnecticutLobbyistReportCodes => "Connecticut Lobbyist Report Codes",
            DelawareLobbyistReportCodes => "Delaware Lobbyist Report Codes",
            DistrictOfColumbiaLobbyistReportCodes => {
                "District of Columbia Lobbyist Report Codes"
            }
            FloridaLobbyistReportCodes => "Florida Lobbyist Report Codes",
            CoastDesignationCode => "Coast Designation Code",
            GeorgiaLobbyistReportCodes => "Georgia Lobbyist Report Codes",
            HawaiiLobbyistReportCodes => "Hawaii Lobbyist Report Codes",
            IdahoLobbyistReportCodes => "Idaho Lobbyist Report Codes",
            IllinoisLobbyistReportCodes => "Illinois Lobbyist Report Codes",
            IndianaLobbyistReportCodes => "Indiana Lobbyist Report Codes",
            IowaLobbyistReportCodes => "Iowa Lobbyist Report Codes",
            KansasLobbyistReportCodes => "Kansas Lobbyist Report Codes",
            KentuckyLobbyistReportCodes => "Kentucky Lobbyist Report Codes",
            LouisianaLobbyistReportCodes => "Louisiana Lobbyist Report Codes",
            MaineLobbyistReportCodes => "Maine Lobbyist Report Codes",
            MarylandLobbyistReportCodes => "Maryland Lobbyist Report Codes",
            MassachusettsLobbyistReportCodes => "Massachusetts Lobbyist Report Codes",
            MichiganLobbyistReportCodes => "Michigan Lobbyist Report Codes",
            MinnesotaLobbyistReportCodes => "Minnesota Lobbyist Report Codes",
            MississippiLobbyistReportCodes => "Mississippi Lobbyist Report Codes",
            MissouriLobbyistReportCodes => "Missouri Lobbyist Report Codes",
            MontanaLobbyistReportCodes => "Montana Lobbyist Report Codes",
            NebraskaLobbyistReportCodes => "Nebraska Lobbyist Report Codes",
            NevadaLobbyistReportCodes => "Nevada Lobbyist Report Codes",
            NewHampshireLobbyistReportCodes => "New Hampshire Lobbyist Report Codes",
            NewJerseyLobbyistReportCodes => "New Jersey Lobbyist Report Codes",
            NewMexicoLobbyistReportCodes => "New Mexico Lobbyist Report Codes",
            NewYorkLobbyistReportCodes => "New York Lobbyist Report Codes",
            NorthCarolinaLobbyistReportCodes => "North Carolina Lobbyist Report Codes",
            NorthDakotaLobbyistReportCodes => "North Dakota Lobbyist Report Codes",
            OhioLobbyistReportCodes => "Ohio Lobbyist Report Codes",
            CompetitiveCharacteristicsCode => "Competitive Characteristics Code",
            OklahomaLobbyistReportCodes => "Oklahoma Lobbyist Report Codes",
            OregonLobbyistReportCodes => "Oregon Lobbyist Report Codes",
            PennsylvaniaLobbyistReportCodes => "Pennsylvania Lobbyist Report Codes",
            PuertoRicoLobbyistReportCodes => "Puerto Rico Lobbyist Report Codes",
            RhodeIslandLobbyistReportCodes => "Rhode Island Lobbyist Report Codes",
            SouthCarolinaLobbyistReportCodes => "South Carolina Lobbyist Report Codes",
            SouthDakotaLobbyistReportCodes => "South Dakota Lobbyist Report Codes",
            TennesseeLobbyistReportCodes => "Tennessee Lobbyist Report Codes",
            TexasLobbyistReportCodes => "Texas Lobbyist Report Codes",
            UtahLobbyistReportCodes => "Utah Lobbyist Report Codes",
            VermontLobbyistReportCodes => "Vermont Lobbyist Report Codes",
            VirginiaLobbyistReportCodes => "Virginia Lobbyist Report Codes",
            WashingtonLobbyistReportCodes => "Washington Lobbyist Report Codes",
            WestVirginiaLobbyistReportCodes => "West Virginia Lobbyist Report Codes",
            WisconsinLobbyistReportCodes => "Wisconsin Lobbyist Report Codes",
            WyomingLobbyistReportCodes => "Wyoming Lobbyist Report Codes",
            NewYorkCityCampaignDisclosureReportCodes => {
                "New York City Campaign Disclosure Report Codes"
            }
            SeattleCampaignDisclosureReportCodes => {
                "Seattle Campaign Disclosure Report Codes"
            }
            NewYorkCityLobbyistReportCodes => "New York City Lobbyist Report Codes",
            CorrectionOrChangeForStorageItemRecordsCode => {
                "Correction or Change for Storage Item Records Code"
            }
            ExcavationInformationCodeList => "Excavation Information Code List",
            TypeDueInIndicator => "Type Due-In Indicator",
            DiscrepancyIndicatorCode => "Discrepancy Indicator Code",
            DisposalConditionCode => "Disposal Condition Code",
            EventOrExposureCode => "Event or Exposure Code",
            ErrorClassificationCode => "Error Classification Code",
            InventoryCategoryCode => "Inventory Category Code",
            CodeEMC => "Automotive Aftermarket Industry Association (AAIA) Emission Code",
            LocalSourceCode => "Local Source Code",
            AdverseEventOutcomeCode => "Adverse Event Outcome Code",
            CodeEP => "Enhanced Ambulatory Patient Groups (EAPG)",
            ExchangePriceIndicator => "Exchange Price Indicator",
            ControlledInventoryItemCode => "Controlled Inventory Item Code",
            EquipmentRequestCodes => "Equipment Request Codes",
            DepartmentOfDefenseIdentificationCode => {
                "Department of Defense Identification Code"
            }
            EquipmentRepairConditionCode => "Equipment Repair Condition Code",
            EquipmentRepairJobCode => "Equipment Repair Job Code",
            EquipmentRepairLocationCode => "Equipment Repair Location Code",
            EquipmentRepairResponsibilityCode => "Equipment Repair Responsibility Code",
            ExtensionReason => "Extension Reason",
            ElectrostaticDischargeCode => "Electrostatic Discharge Code",
            EquipmentStatusLookup => "Equipment Status Lookup",
            RejectAdviceCode => "Reject Advice Code",
            EstimateTypeLookup => "Estimate Type Lookup",
            RequestCode => "Request Code",
            ReviewPeriodIndicatorCode => "Review Period Indicator Code",
            SmallArmsErrorTransactionRejectCode => {
                "Small Arms Error Transaction Reject Code"
            }
            EvaluateWorkCandidateLookup => "Evaluate Work Candidate Lookup",
            EquipmentWhyMadeCode => "Equipment Why Made Code",
            EvaluateWorkCandidateReasonLookup => "Evaluate Work Candidate Reason Lookup",
            SmallArmsTransactionCode => "Small Arms Transaction Code",
            ExportDeclaration => "Export Declaration",
            CodeEXP => "Export Control Classification Number (ECCN)",
            SpecialProgramRequirementStatusCode => {
                "Special Program Requirement Status Code"
            }
            TypeInspectionCode => "Type Inspection Code",
            FinancialRating => "Financial Rating",
            TypeOfContractorCode => "Type of Contractor Code",
            FannieMaeRefinancePlanCode => "Fannie Mae Refinance Plan Code",
            TypeOfMediaCode => "Type of Media Code",
            TypePhysicalInventoryOrTransactionHistoryCode => {
                "Type Physical Inventory or Transaction History Code"
            }
            CodeFC1 => "Federal Communication, Control and Security Code List 1",
            DemilitarizationCode => "Demilitarization Code",
            ShelfLifeCode => "Shelf Life Code",
            EssentialityCode => "Essentiality Code",
            FederalFinanceCodeList1 => "Federal Finance Code List 1",
            SourceMaintenanceAndRecoverabilityCode => {
                "Source Maintenance and Recoverability Code"
            }
            TypeOfLocationReconciliationRequest => {
                "Type of Location Reconciliation Request"
            }
            FederalHealthCareCodeList1 => "Federal Health Care Code List 1",
            ApplicantType => "Applicant Type",
            FinancialInventoryReportCode => "Financial Inventory Report Code",
            AntennaStructureType => "Antenna Structure Type",
            StationClassification => "Station Classification",
            RadioFrequencyType => "Radio Frequency Type",
            FederalLogisticsCodeList1 => "Federal Logistics Code List 1",
            StationClassificationType => "Station Classification Type",
            FormerMajorOrganizationalEntityRuleNumber => {
                "Former Major Organizational Entity Rule Number"
            }
            ForeignMilitarySalesAndMilitaryAssistanceProgramGrantAidTypeOfAssistanceFinancingCode => {
                "Foreign Military Sales and Military Assistance Program Grant Aid Type of Assistance/Financing Code"
            }
            ClassOfOperation => "Class of Operation",
            AntennaPolarization => "Antenna Polarization",
            FundPurpose => "Fund Purpose",
            FederalProcurementCodeList1 => "Federal Procurement Code List 1",
            RadioSystemType => "Radio System Type",
            FrequencyBand => "Frequency Band",
            FreddieMacRefinancePlanCode => "Freddie Mac Refinance Plan Code",
            FreightBillApplicationErrorEditCodes => {
                "Freight Bill Application Error Edit Codes"
            }
            AreaOfOperation => "Area of Operation",
            ApplicationType => "Application Type",
            FederalTransportationCodeList1 => "Federal Transportation Code List 1",
            AuthorizationType => "Authorization Type",
            RadioServiceType => "Radio Service Type",
            ApplicantClassificationType => "Applicant Classification Type",
            Frequency => "Frequency",
            EditErrorCode => "Edit Error Code",
            RiskClass => "Risk Class",
            UniformResidentialAppraisalAttributesCode => {
                "Uniform Residential Appraisal Attributes Code"
            }
            ActionCode => "Action Code",
            MediumOfTransmissionCode => "Medium of Transmission Code",
            CodeGC => "Management Indicator Code (Petroleum)",
            GainOrLossIndicatorCode => "Gain or Loss Indicator Code",
            TypeAdjustmentCode => "Type Adjustment Code",
            TypeIdentityChangeCode => "Type Identity Change Code",
            TransportationModeReasonCode => "Transportation Mode Reason Code",
            NotificationIndicatorCode => "Notification Indicator Code",
            RejectIndicatorCode => "Reject Indicator Code",
            InvestigationStatusCode => "Investigation Status Code",
            GroupQualifierCode => "Group Qualifier Code",
            CodeGR => {
                "National Council on Compensation Insurance (NCCI) Nature of Injury Code"
            }
            CodeGS => {
                "Occupational Safety and Health Administration (OSHA) Nature of Injury Code"
            }
            CodeGT => {
                "National Council on Compensation Insurance (NCCI) Part of Body Code"
            }
            CodeGU => {
                "Occupational Safety and Health Administration (OSHA) Part of Body Code"
            }
            CodeGV => {
                "National Council on Compensation Insurance (NCCI) Source of Injury Code"
            }
            CodeGW => {
                "Occupational Safety and Health Administration (OSHA) Source of Injury Code"
            }
            GlassActionCode => "Glass Action Code",
            CauseOfLossCode => "Cause of Loss Code",
            LossDescriptionCode => "Loss Description Code",
            LifeAnnuityStatusCodes => "Life/Annuity Status Codes",
            DiscrepancyCode => "Discrepancy Code",
            DiscrepancyAdviceCode => "Discrepancy Advice Code",
            InstitutionalSectorOrLevelClassificationCode => {
                "Institutional Sector or Level Classification Code"
            }
            DiscrepancyStatusOrDispositionCode => {
                "Discrepancy Status or Disposition Code"
            }
            RemittanceAdviceRemarkCode => "Remittance Advice Remark Code",
            EducationStaffTypeCode => "Education Staff Type Code",
            EducationFeeTypeCode => "Education Fee Type Code",
            HealthIndustryNumber => "Health Industry Number",
            InstitutionalFeeBasisCode => "Institutional Fee Basis Code",
            NationalCenterForEducationStatisticsIntegratedPostsecondaryEducationDataSystemInstitutionalCharacteristicsSurveyCode => {
                "National Center for Education Statistics Integrated Postsecondary Education Data System Institutional Characteristics Survey Code"
            }
            CodeHL => "Accreditation, Affiliation, or Licensing Level Code",
            NationalCenterForEducationStatisticsAccreditationOrLicensingType => {
                "National Center for Education Statistics Accreditation or Licensing Type"
            }
            HazardousMaterialContentCode => "Hazardous Material Content Code",
            HazardousResponseCodes => "Hazardous Response Codes",
            ServiceContractActOccupationCategoryCode => {
                "Service Contract Act Occupation Category Code"
            }
            AssociationOfAmericanRailroadsStandardTransportationCommodityCodeDescriptionQualifier => {
                "Association of American Railroads Standard Transportation Commodity Code Description Qualifier"
            }
            IdentifyingCharacteristics => "Identifying Characteristics",
            InsuranceBusinessProcessApplicationErrorCode => {
                "Insurance Business Process Application Error Code"
            }
            CodeIC => {
                "Collision Industry Electronic Commerce Association (CIECA) - Inspection"
            }
            CodeICF => {
                "International Classification of Functioning Disability and Health (ICF)"
            }
            IdentityDisclosureCode => "Identity Disclosure Code",
            InvestmentFundType => "Investment Fund Type",
            ItemManagementCode => "Item Management Code",
            ImpactRecorderCode => "Impact Recorder Code",
            IntraNavyDisposalReleaseOrderRejectAdviceCode => {
                "Intra-Navy Disposal Release Order Reject Advice Code"
            }
            ImpactAxisOrAnalogPortCode => "Impact Axis or Analog Port Code",
            IssuePriorityGroup => "Issue Priority Group",
            IrsQualificationCode => "IRS Qualification Code",
            CodeIRR => "Issue, Repair and Requisition Code",
            ImbalanceReportingType => "Imbalance Reporting Type",
            InitialTreatmentCode => "Initial Treatment Code",
            InterruptibleTransportationIndicator => {
                "Interruptible Transportation Indicator"
            }
            TradeCode => "Trade Code",
            SummonsTypeCode => "Summons Type Code",
            JudicialHearingTypeCode => "Judicial Hearing Type Code",
            JudicialOrderTypeCode => "Judicial Order Type Code",
            JudicialSentenceTypeCode => "Judicial Sentence Type Code",
            CourtDispositionCode => "Court Disposition Code",
            CourtAppearanceTypeCode => "Court Appearance Type Code",
            CourtPleadingTypeCode => "Court Pleading Type Code",
            DefendantPleaTypeCode => "Defendant Plea Type Code",
            TrialTypeCode => "Trial Type Code",
            CourtCaseStatusCode => "Court Case Status Code",
            PhysicalCharacteristicsCode => "Physical Characteristics Code",
            WeightOrFragilityCode => "Weight or Fragility Code",
            PreservationMaterialCode => "Preservation Material Code",
            JobCharacteristicsLookup => "Job Characteristics Lookup",
            QuantityPerUnitPackCode => "Quantity per Unit Pack Code",
            PreservationDataCode => "Preservation Data Code",
            PackingRequirementLevelACode => "Packing Requirement Level A Code",
            PackingRequirementLevelBCode => "Packing Requirement Level B Code",
            PackingRequirementLevelCCode => "Packing Requirement Level C Code",
            IntermediateContainerCode => "Intermediate Container Code",
            IntermediateContainerQuantityCode => "Intermediate Container Quantity Code",
            SpecialMarkingCode => "Special Marking Code",
            TypeAndCauseCode => "Type and Cause Code",
            MissionImpactStatementCode => "Mission Impact Statement Code",
            InternationalStandardDesignationSystemForTeethAndAreasOfTheOralCavity => {
                "International Standard Designation System for Teeth and Areas of the Oral Cavity"
            }
            JobOriginatorLookup => "Job Originator Lookup",
            UniversalNationalToothDesignationSystem => {
                "Universal National Tooth Designation System"
            }
            PropertyUnderwritingConditionCode => "Property Underwriting Condition Code",
            DeficiencyCause => "Deficiency Cause",
            Discrepancy => "Discrepancy",
            PreventiveMeasure => "Preventive Measure",
            ContractorAlertListReason => "Contractor Alert List Reason",
            QualityAlertListReason => "Quality Alert List Reason",
            ContractorAlertListStatus => "Contractor Alert List Status",
            NatureOfBuy => "Nature of Buy",
            TypeOfProcurement => "Type of Procurement",
            RepresentativeBuyIndicator => "Representative Buy Indicator",
            AssuredDeliveryIndicator => "Assured Delivery Indicator",
            AwardSource => "Award Source",
            Termination => "Termination",
            PatientEventProblemCode => "Patient Event Problem Code",
            MethodEvaluationCode => "Method Evaluation Code",
            ResultEvaluationCode => "Result Evaluation Code",
            ConclusionEvaluationCode => "Conclusion Evaluation Code",
            DeviceEventProblemCode => "Device Event Problem Code",
            DoseFormCode => "Dose Form Code",
            RouteCode => "Route Code",
            ReportSourceCode => "Report Source Code",
            KeyEventLookup => "Key Event Lookup",
            AdverseEventCode => "Adverse Event Code",
            LineItemConditionCode => "Line Item Condition Code",
            Contract => "Contract",
            ContractorReviewListStatus => "Contractor Review List Status",
            LaboratoryTestConditionCode => "Laboratory Test Condition Code",
            LocationCapacityFlowIndicator => "Location Capacity Flow Indicator",
            CodeLD => {
                "Collision Industry Electronic Commerce Association (CIECA) - Loss Category"
            }
            LifeAnnuityServiceFeatures => "Life/Annuity Service Features",
            LifeAnnuityProductCode => "Life/Annuity Product Code",
            LocationCode => "Location Code",
            BasisOfJurisdictionCode => "Basis of Jurisdiction Code",
            LineOfAuthority => "Line of Authority",
            PrincipalPartyCitizenshipCode => "Principal Party Citizenship Code",
            NatureOfSuitCode => "Nature of Suit Code",
            CaseOriginCode => "Case Origin Code",
            LimitType => "Limit Type",
            LineOfBusinessCode => "Line of business code",
            LetterOfRecommendationRatingCategory => {
                "Letter of Recommendation Rating Category"
            }
            LocationIndicator => "Location Indicator",
            CodeLOI => "Logical Observation Identifier Names and Codes (LOINC) Codes",
            DeficiencyIndicator => "Deficiency Indicator",
            LocationPurposeCode => "Location Purpose Code",
            DelinquencyIndicator => "Delinquency Indicator",
            LocationQuantityTypeIndicator => "Location Quantity Type Indicator",
            TestResultsCode => "Test Results Code",
            LossSeverityCode => "Loss Severity Code",
            LegalStructureCode => "Legal Structure Code",
            LaboratoryResultsIdentificationCode => {
                "Laboratory Results Identification Code"
            }
            WarReserveMaterialRequirementCode => "War Reserve Material Requirement Code",
            PolicyTypeCode => "Policy Type Code",
            MultiMediaObject => "Multi-Media Object",
            MaterialManagementAggregationCode => "Material Management Aggregation Code",
            ServiceContractActOccupationClassificationCode => {
                "Service Contract Act Occupation Classification Code"
            }
            ManualClassCode => "Manual Class Code",
            MaterialControlCode => "Material Control Code",
            GeneratorSetMountingCode => "Generator Set Mounting Code",
            DeviceEvaluationCode => "Device Evaluation Code",
            MethodOfCompletionCode => "Method of Completion Code",
            ManagerForcedDirectedAction => "Manager Forced Directed Action",
            MinorityIndicator => "Minority Indicator",
            DrugStatusCode => "Drug Status Code",
            DrugStatusAdverseEventCode => "Drug Status Adverse Event Code",
            LotTypeCode => "Lot Type Code",
            PostMarketStudyStatusCode => "Post Market Study Status Code",
            CodeMOC => {
                "Mechanization of Contract Administration Services (MOCAS) System Error Code"
            }
            MajorOrganizationalEntityRuleNumber => {
                "Major Organizational Entity Rule Number"
            }
            ListOfCodesIdentifyingDoDMappingProductProcurementTypes => {
                "List of codes identifying DoD Mapping Product Procurement Types."
            }
            ListOfCodesIdentifyingDoDMappingProductTypes => {
                "List of codes identifying DoD Mapping Product Types."
            }
            ReferencePartialDescriptiveMethodReasonCode => {
                "Reference Partial Descriptive Method Reason Code"
            }
            MaximumRateIndicator => "Maximum Rate Indicator",
            MeterStatus => "Meter Status",
            MeterType => "Meter Type",
            ValuationTypeCode => "Valuation Type Code",
            PlantClearanceOfficeCode => "Plant Clearance Office Code",
            CodeNAC => "Nomenclature Activity Classification Economy (NACE) Code",
            CodeNAF => "Norme Activite Francaise (NAF) Code",
            NonApprovedItemName => "Non-Approved Item Name",
            NatureOfSuit => "Nature of Suit",
            InventoryTypeCode => "Inventory Type Code",
            NoBalanceAffectingTransactions => "No Balance Affecting Transactions",
            PropertyRecordStatusCode => "Property Record Status Code",
            InvoiceNotesCode => "Invoice Notes Code",
            NominationCapacityExceededIndicator => {
                "Nomination Capacity Exceeded Indicator"
            }
            ControlUnitDesignCode => "Control Unit Design Code",
            CodeNDC => "National Drug Code (NDC)",
            DirectNumericalControlSystemCode => "Direct Numerical Control System Code",
            TypeNumericalControlSystemCode => "Type Numerical Control System Code",
            CodeNGC => "National Geospatial-Intelligence Agency (NGA) Product Code",
            PropertySourceCode => "Property Source Code",
            NatureOfInjuryCode => "Nature of Injury Code",
            NonInductionReasonCode => "Non-Induction Reason Code",
            CodeNJ => "Uniform Freight Classification (UFC) Code",
            CodeNK => "National Motor Freight Classification (NMFC) Code",
            CodeNMS => "List of DoD Nonconsumable Item Material Support Codes (NIMSC)",
            SpecialCategoryCode => "Special Category Code",
            CodeNPC => {
                "Automotive Aftermarket Industry Association (AAIA) National Popularity Code"
            }
            ExcessMaterialDispositionCode => "Excess Material Disposition Code",
            HazardousMaterialCode => "Hazardous Material Code",
            TypeOfCargoCode => "Type of Cargo Code",
            CodeNUB => "National Uniform Billing Committee (NUBC) Revenue Codes",
            SourceOfLossCode => "Source of Loss Code",
            CodeO1 => {
                "Office of Worker's Compensation Programs (OWCP) Source of Injury Code"
            }
            CodeO2 => {
                "Office of Worker's Compensation Programs (OWCP) Nature of Injury Code"
            }
            CodeO3 => "Office of Worker's Compensation Programs (OWCP) Part of Body Code",
            CodeO4 => "Office of Worker's Compensation Programs (OWCP) Occupation Code",
            OccupationCode => "Occupation Code",
            CeilingTypeCode => "Ceiling Type Code",
            PriorDamageLocationCode => "Prior Damage Location Code",
            PartOfBodyCode => "Part of Body Code",
            CodePC => {
                "Collision Industry Electronic Commerce Association (CIECA) - Profile"
            }
            PlanningCodeLookup => "Planning Code Lookup",
            PseudoClosureReasonCode => "Pseudo Closure Reason Code",
            ProfessionalDesignation => "Professional Designation",
            CodePDA => "Pre-determined Allocation (PDA) Transaction Type Code",
            CodePGS => "Petroleum United States Geological Survey (USGS) Formation Code",
            PhraseCode => "Phrase Code",
            CodePI => {
                "Collision Industry Electronic Commerce Association (CIECA) - Points of Impact"
            }
            PetroleumBillType => "Petroleum Bill Type",
            PriorityLookup => "Priority Lookup",
            PetroleumLandCategory => "Petroleum Land Category",
            PetroleumLeaseStatus => "Petroleum Lease Status",
            PartOfBodyAndNatureOfInjury => "Part of Body and Nature of Injury",
            PlaceOfServiceCode => "Place of Service Code",
            PetroleumProductDisposition => "Petroleum Product Disposition",
            PetroleumProductPointOfSale => "Petroleum Product Point-of-Sale",
            PetroleumProductSellingArrangement => "Petroleum Product Selling Arrangement",
            PetroleumProductValueAdjustment => "Petroleum Product Value Adjustment",
            PetroleumQuantityAllocationsCode => "Petroleum Quantity Allocations Code",
            CodePR => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Patient's Reason for Visit"
            }
            PetroleumRoyaltyAdjustment => "Petroleum Royalty Adjustment",
            PetroleumRoyaltyCalculationMethod => "Petroleum Royalty Calculation Method",
            ProcessingRightsIndicator => "Processing Rights Indicator",
            PetroleumRegulatoryReport => "Petroleum Regulatory Report",
            PetroleumRoyaltyTransaction => "Petroleum Royalty Transaction",
            ProfessionalStatusCode => "Professional Status Code",
            PriceTier => "Price Tier",
            PetroleumWellAction => "Petroleum Well Action",
            PetroleumWell => "Petroleum Well Information",
            PetroleumWellShutInReason => "Petroleum Well Shut-In Reason",
            PetroleumWellClassificationStatus => "Petroleum Well Classification Status",
            PetroleumWellTest => "Petroleum Well Test Information",
            SurfaceDescriptorCode => "Surface Descriptor Code",
            ResponseStatusCode => "Response Status Code",
            BusinessEntityFilingReportTypeCode => {
                "Business Entity Filing Report Type Code"
            }
            BusinessEntityFilingDetailCode => "Business Entity Filing Detail Code",
            ProductQualityDeficiencyReportSummaryCode => {
                "Product Quality Deficiency Report Summary Code"
            }
            DomesticLineOfBusinessCode => "Domestic Line of Business Code",
            ForeignLineOfBusinessCode => "Foreign Line of Business Code",
            BusinessEntityFilingStatusCode => "Business Entity Filing Status Code",
            BusinessEntityFilingSecuritiesInformationCode => {
                "Business Entity Filing Securities Information Code"
            }
            BusinessEntityFinancialInformationCode => {
                "Business Entity Financial Information Code"
            }
            BusinessEntityStatusCode => "Business Entity Status Code",
            BusinessEntityFilingLocationCode => "Business Entity Filing Location Code",
            QueryStatus => "Query Status",
            QuantityTypeIndicator => "Quantity Type Indicator",
            CoverageModifier => "Coverage Modifier",
            CodeR1 => "Upstream Rank (Priority)",
            CodeR2 => "Receipt Rank (Priority)",
            CodeR3 => "Delivery Rank (Priority)",
            CodeR4 => "Downstream Rank (Priority)",
            ThreadedRank => "Threaded Rank",
            ReligiousAffiliationCode => "Religious Affiliation Code",
            ReceiptAcceptanceSiteCode => "Receipt Acceptance Site Code",
            RequirementCode => "Requirement Code",
            RegisteredContractorActivityCode => "Registered Contractor Activity Code",
            PropertyOwnershipTypeCode => "Property Ownership Type Code",
            PropertyTypeCode => "Property Type Code",
            RaceOrEthnicityCollectionCode => "Race or Ethnicity Collection Code",
            ReductionReasonCode => "Reduction Reason Code",
            AssociationOfAmericanRailroadsRateEdiNetworkErrorCode => {
                "Association of American Railroads Rate EDI Network Error Code"
            }
            ClassificationOfRaceOrEthnicity => "Classification of Race or Ethnicity",
            ReferenceNumberFormatCode => "Reference Number Format Code",
            ReasonForMovementCode => "Reason for Movement Code",
            ResidencyIndicator => "Residency Indicator",
            ReferenceNumberJustificationCode => "Reference Number Justification Code",
            InsuranceIndustrySpecificRemarkCodes => {
                "Insurance Industry Specific Remark Codes"
            }
            ReferenceNumberCategoryCode => "Reference Number Category Code",
            ReplenishmentDemand => "Replenishment Demand Information",
            TestingServiceQuestionCodeList => "Testing Service Question Code List",
            RetailDemand => "Retail Demand Information",
            ReasonForReversalCode => "Reason for Reversal Code",
            ReceiptSchedulingStatus => "Receipt Scheduling Status",
            RequestType => "Request Type",
            RegistrationTypeCode => "Registration Type Code",
            RefrigerationUnitOperatingModeCode => {
                "Refrigeration Unit Operating Mode Code"
            }
            ReferenceNumberVariationCode => "Reference Number Variation Code",
            NationalCouncilForPrescriptionDrugProgramsRejectCodes => {
                "National Council for Prescription Drug Programs Reject Codes"
            }
            CodeS => {
                "Society for Worldwide Interbank Financial Telecommunications (SWIFT)"
            }
            StudentActivityTypeCode => "Student Activity Type Code",
            SecurityAssistanceDocumentNumberRequirementTypeCode => {
                "Security Assistance Document Number Requirement Type Code"
            }
            StockActionTechnicalInformationCode => {
                "Stock Action/Technical Information Code"
            }
            StudentAwardCode => "Student Award Code",
            CodeSBA => "Statistic Bundes Amt (SBA) Code",
            Source => "Source",
            SubsequentCycleIndicator => "Subsequent Cycle Indicator",
            CodeSD => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Secondary Diagnosis"
            }
            SoundCode => "Sound Code",
            StockExchangeCode => "Stock Exchange Code",
            SettlementType => "Settlement Type",
            RunType => "Run Type",
            SwingFuelOptionIndicator => "Swing Fuel Option Indicator",
            SourceOfDepositCode => "Source of Deposit Code",
            SourceOfLeadCode => "Source of Lead Code",
            SafetyHazardLookup => "Safety Hazard Lookup",
            AccidentResultingChangeCode => "Accident Resulting Change Code",
            ActiveMitigationConsiderationCode => "Active Mitigation Consideration Code",
            ActivityMethodsCode => "Activity Methods Code",
            AnalyticalMethodCode => "Analytical Method Code",
            AtmosphericStabilityClassCode => "Atmospheric Stability Class Code",
            BasisOfEstimateCode => "Basis of Estimate Code",
            CertificationCode => "Certification Code",
            ContributingFactorCode => "Contributing Factor Code",
            ControlDeviceTypeCode => "Control Device Type Code",
            DesignStandardCode => "Design/Standard Code",
            DeviceClassificationCode => "Device Classification Code",
            DischargeIndicatorCode => "Discharge Indicator Code",
            DischargeQuantityRangeCode => "Discharge Quantity Range Code",
            NonReportableDischargeIndicatorCode => {
                "Non-Reportable Discharge Indicator Code"
            }
            CodeSI => "SIC (Standard Industrial Classification)",
            EmergencyResponseRegulationStatuteCode => {
                "Emergency Response Regulation/Statute Code"
            }
            EmissionFactorTypeCode => "Emission Factor Type Code",
            EmissionReleasePointTypeCode => "Emission Release Point Type Code",
            EmissionSourceTypeCode => "Emission Source Type Code",
            EmissionTypeCode => "Emission Type Code",
            EmissionUnitTypeCode => "Emission Unit Type Code",
            EndpointCode => "Endpoint Code",
            EnvironmentCode => "Environment Code",
            EnvironmentalProgramTypeCode => "Environmental Program Type Code",
            EnvironmentalReceptorCode => "Environmental Receptor Code",
            FacilityCategoryCode => "Facility Category Code",
            FacilityStatusCode => "Facility Status Code",
            FactorCalculationMethodCode => "Factor Calculation Method Code",
            FrequencyOfAnalysisCode => "Frequency of Analysis Code",
            GeneratorStatusCode => "Generator Status Code",
            GeometricTypeCode => "Geometric Type Code",
            HazardousWasteFormCode => "Hazardous Waste Form Code",
            HorizontalDatumCode => "Horizontal Datum Code",
            InformationSystemCode => "Information System Code",
            InitiatingEventCode => "Initiating Event Code",
            InventoryQuantityRangeCode => "Inventory Quantity Range Code",
            LatitudeLongitudeSourceCode => "Latitude/Longitude Source Code",
            LatitudeLongitudeVerificationCode => "Latitude/Longitude Verification Code",
            LocationDescriptionCode => "Location Description Code",
            MajorHazardCode => "Major Hazard Code",
            ManufacturingCode => "Manufacturing Code",
            SourceOfInjuryCode => "Source of Injury Code",
            MaterialClassificationCode => "Material Classification Code",
            MaterialCode => "Material Code",
            MaximumAchievableControlTechnologyCode => {
                "Maximum Achievable Control Technology Code"
            }
            MethodOfCollectionCode => "Method of Collection Code",
            MitigationSystemCode => "Mitigation System Code",
            ModelUsedCode => "Model Used Code",
            MonitoringDetectionSystemCode => "Monitoring/Detection System Code",
            MonitoringLocationCode => "Monitoring Location Code",
            NonGeneratingWasteCode => "Non-Generating Waste Code",
            OffSiteAvailabilityCode => "Off-Site Availability Code",
            OffSiteImpactCode => "Off-Site Impact Code",
            OnSiteImpactCode => "On-Site Impact Code",
            OnSiteProcessSystemTypeCode => "On-Site Process System Type Code",
            OriginCode => "Origin Code",
            ParameterCode => "Parameter Code",
            PassiveMitigationConsiderationCode => "Passive Mitigation Consideration Code",
            PermitComplianceStatusCode => "Permit Compliance Status Code",
            PhysicalStateCode => "Physical State Code",
            PointOfMeasurementCode => "Point of Measurement Code",
            PreservativeCode => "Preservative Code",
            ProcessCode => "Process Code",
            ProcessControlCode => "Process Control Code",
            ProcessHazardAnalysisUpdateResultingChangeCode => {
                "Process Hazard Analysis Update Resulting Change Code"
            }
            ProcessHazardsAnalysisTechniqueCode => {
                "Process Hazards Analysis Technique Code"
            }
            PublicReceptorCode => "Public Receptor Code",
            RangeOfConcentrationCode => "Range of Concentration Code",
            RecoveryMethodCode => "Recovery Method Code",
            RecyclingMethodCode => "Recycling Method Code",
            ReleaseEventCode => "Release Event Code",
            ReleaseSourceCode => "Release Source Code",
            ReliabilityIndicatorCode => "Reliability Indicator Code",
            RuleEffectivenessMethodCode => "Rule Effectiveness Method Code",
            SampleTypeCode => "Sample Type Code",
            ScenarioCode => "Scenario Code",
            SiteLocationCode => "Site Location Code",
            SourceCategoryCode => "Source Category Code",
            SourceOfWasteGenerationCode => "Source of Waste Generation Code",
            SourceReductionActivityCode => "Source Reduction Activity Code",
            SystemTypeCode => "System Type Code",
            TimePeriodCode => "Time Period Code",
            TopographyCode => "Topography Code",
            TransferQuantityRangeCode => "Transfer Quantity Range Code",
            TypeOfCompetencyTestingCode => "Type of Competency Testing Code",
            TypeOfTrainingCode => "Type of Training Code",
            TypeOfWasteManagementCode => "Type of Waste Management Code",
            UseCode => "Use Code",
            WasteEmanationCode => "Waste Emanation Code",
            WasteManagementStatusCode => "Waste Management Status Code",
            WasteStreamCode => "Waste Stream Code",
            WasteTreatmentMethodCode => "Waste Treatment Method Code",
            WindDirectionCode => "Wind Direction Code",
            UnitOfMeasureCode => "Unit of Measure Code",
            SecondarySourceOfInjury => "Secondary Source of Injury",
            ShelfLifeActionCode => "Shelf-Life Action Code",
            StockageListCode => "Stockage List Code",
            SchedulingStatus => "Scheduling Status",
            StatementBasis => "Statement Basis",
            SpecialMaterialContentCode => "Special Material Content Code",
            SampleDevice => "Sample Device",
            SpecialMaterialIdentificationCode => "Special Material Identification Code",
            SampleType => "Sample Type",
            SolicitationCancellationReason => "Solicitation Cancellation Reason",
            StandardOccupationClassificationCode => {
                "Standard Occupation Classification Code"
            }
            SubmittersPriorityDesignator => "Submitter's Priority Designator",
            SpecialDating => "Special Dating",
            StatisticalAdministrativeInformationCode => {
                "Statistical Administrative Information Code"
            }
            ServiceRequesterAgentIndicator => "Service Requester Agent Indicator",
            ListOfCodesIdentifyingDoDSerializedReportTypeCodes => {
                "List of codes identifying DoD Serialized Report Type Codes."
            }
            SpecialRequirementLookup => "Special Requirement Lookup",
            SupplementalReductionReason => "Supplemental Reduction Reason",
            StorageReportType => "Storage Report Type",
            SystemStatus => "System Status",
            SupplyStatusCode => "Supply Status Code",
            SpecialMarketingTypeCode => "Special Marketing Type Code",
            AssociationOfAmericanRailroadsStandardTransportationCommodityCodeMasterDescription => {
                "Association of American Railroads Standard Transportation Commodity Code Master Description Information"
            }
            ForwardAndStoreApplicationErrorEditCodes => {
                "Forward and Store Application Error Edit Codes"
            }
            SolicitedUnsolicitedIndicator => "Solicited/Unsolicited Indicator",
            ServiceCode => "Service Code",
            AssociationOfAmericanRailroadsSwitchReleaseCodes => {
                "Association of American Railroads Switch Release Codes"
            }
            PersonalPropertyAndContentsCode => "Personal Property and Contents Code",
            CommercialVehicleOperationsSafetyCode => {
                "Commercial Vehicle Operations Safety Code"
            }
            DataCategories => "Data Categories",
            EventCodes => "Event Codes",
            OperationType => "Operation Type",
            AccidentParameters => "Accident Parameters",
            InspectionParameters => "Inspection Parameters",
            DriverParameters => "Driver Parameters",
            ViewParameters => "View Parameters",
            VehicleParameters => "Vehicle Parameters",
            FleetParameters => "Fleet Parameters",
            QueryOptions => "Query Options",
            JurisdictionType => "Jurisdiction Type",
            SingleStateRegistrationSystemAndOperatingAuthorityCredential => {
                "Single State Registration System and Operating Authority Credential"
            }
            CommercialVehicleOperationsInsurance => {
                "Commercial Vehicle Operations Insurance"
            }
            CommercialVehicleRegistration => "Commercial Vehicle Registration",
            HazardousMaterialsCredential => "Hazardous Materials Credential",
            OversizeOverweightCredential => "Oversize/Overweight Credential",
            CommercialVehicleTax => "Commercial Vehicle Tax",
            CommercialVehicleTitle => "Commercial Vehicle Title",
            CommercialDriversLicense => "Commercial Driver's License",
            CommercialVehicleType => "Commercial Vehicle Type",
            CommercialVehicleOperationsStatusCode => {
                "Commercial Vehicle Operations Status Code"
            }
            SafetyAndFitnessElectronicRecordSystemsSubscriptionOption => {
                "Safety and Fitness Electronic Record Systems Subscription Option"
            }
            CommercialVehicleOperationsCommodityCode => {
                "Commercial Vehicle Operations Commodity Code"
            }
            CommercialVehicleOperationsHazardousMaterialCode => {
                "Commercial Vehicle Operations Hazardous Material Code"
            }
            SafetyAndFitnessElectronicRecordSystemsErrorCode => {
                "Safety and Fitness Electronic Record Systems Error Code"
            }
            CommercialVehicleOperationsJurisdictionIdentifierCode => {
                "Commercial Vehicle Operations Jurisdiction Identifier Code"
            }
            ComplianceReviewCode => "Compliance Review Code",
            IncidentConditionCode => "Incident Condition Code",
            IncidentRelatedActionCode => "Incident Related Action Code",
            IncidentLocationCode => "Incident Location Code",
            IncidentConsequenceCode => "Incident Consequence Code",
            RoadCharacteristicCode => "Road Characteristic Code",
            VehicleOccupantCode => "Vehicle Occupant Code",
            PackageFailureCode => "Package Failure Code",
            PedestrianCode => "Pedestrian Code",
            AssociationForFinancialProfessionalsServiceCodeAndBankServiceCode => {
                "Association for Financial Professionals Service Code and Bank Service Code"
            }
            TreatmentCodes => "Treatment Codes",
            TypeOfChangeCode => "Type of Change Code",
            CodeTCD => {
                "Collision Industry Electronic Commerce Association (CIECA) - Totals Code List"
            }
            TemplateCharacteristicLookup => "Template Characteristic Lookup",
            CodeTD => {
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Tertiary Diagnosis"
            }
            DiscrepancyReportTypeCode => "Discrepancy Report Type Code",
            TechnicalDataJustificationCode => "Technical Data Justification Code",
            AssociationForFinancialProfessionalsServiceCode => {
                "Association for Financial Professionals Service Code"
            }
            TitleExceptionAndRequirementCodeList => {
                "Title Exception and Requirement Code List"
            }
            TaxOrFeeExemptionReasonCode => "Tax or Fee Exemption Reason Code",
            TitleDocumentCodeList => "Title Document Code List",
            TapLocation => "Tap Location",
            WeaponSystemTransactionOriginationCode => {
                "Weapon System Transaction Origination Code"
            }
            TemplateOwnerLookup => "Template Owner Lookup",
            TapType => "Tap Type",
            CodeTQ => "Systemized Nomenclature of Dentistry (SNODENT)",
            ReportCode => "Report Code",
            NaturalGasTransactionType => "Natural Gas Transaction Type",
            DownstreamTransactionType => "Downstream Transaction Type",
            TemplateTypeLookup => "Template Type Lookup",
            UpstreamTransactionType => "Upstream Transaction Type",
            FollowUpCode => "Follow-up Code",
            ReportableEventCode => "Reportable Event Code",
            ResidentialAndCommercialRoomCode => "Residential and Commercial Room Code",
            UmlerBodyType => "UMLER Body Type",
            UniqueItemTrackingDesignatorCode => "Unique Item Tracking Designator Code",
            UniqueItemTrackingErrorRejectCode => "Unique Item Tracking Error Reject Code",
            UmlerFittingCode => "UMLER Fitting Code",
            UrgencyJustificationCode => "Urgency Justification Code",
            CodeUNP => {
                "(UN/SPSC) United Nations Products and Services Classification Code"
            }
            CodeUPC => "Unclaimed Property Additions, Deletions, and Deductions Codes",
            UnclaimedPropertyTypeCode => "Unclaimed Property Type Code",
            EventReappearanceCode => "Event Reappearance Code",
            EventAbatementCode => "Event Abatement Code",
            CodeUT => {
                "Centers for Medicare and Medicaid Services (CMS) Certificate of Medical Necessity (CMN) forms"
            }
            UniqueItemTrackingTransactionCode => "Unique Item Tracking Transaction Code",
            UnitCode => "Unit Code",
            ViolationTypeCodeList => "Violation Type Code List",
            ValidationCode => "Validation Code",
            CodeVP => {
                "Collision Industry Electronic Commerce Association (CIECA) - Vehicle Line Item Category Code"
            }
            CourtIssuedWarrantTypeCode => "Court Issued Warrant Type Code",
            WeaponSystemAdviceCode => "Weapon System Advice Code",
            WhenDiscoveredLookup => "When Discovered Lookup",
            WeaponSystemEssentialityCode => "Weapon System Essentiality Code",
            CodeWRC => "Automotive Aftermarket Industry Association (AAIA) Warranty Code",
            WeaponSystemStatusCode => "Weapon System Status Code",
            WeaponSystemDesignatorCode => "Weapon System Designator Code",
            WeaponSystemMaintenanceCode => "Weapon System Maintenance Code",
            VehicleClass => "Vehicle Class",
            RentalCharge => "Rental Charge",
            CancellationReason => "Cancellation Reason",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<CodeListQualifierCode> {
        {
            use CodeListQualifierCode::*;
            match description {
                "Document Identification Code" => Some(DocumentIdentificationCode),
                "Free On Board Site Code" => Some(FreeOnBoardSiteCode),
                "Channel of Distribution Code" => Some(ChannelOfDistributionCode),
                "Kind of Contract Code" => Some(KindOfContractCode),
                "Type of Contract Code" => Some(TypeOfContractCode),
                "Criticality Designator Code" => Some(CriticalityDesignatorCode),
                "Quality Assurance Site Code" => Some(QualityAssuranceSiteCode),
                "Acceptance Site Code" => Some(AcceptanceSiteCode),
                "Transaction Status Indicator Code" => {
                    Some(TransactionStatusIndicatorCode)
                }
                "Contract Delivery Date Revision Agent Code" => {
                    Some(ContractDeliveryDateRevisionAgentCode)
                }
                "Reason for Contract Delivery Date Revision Code" => {
                    Some(ReasonForContractDeliveryDateRevisionCode)
                }
                "Recommendations Regarding Delayed Deliveries Code" => {
                    Some(RecommendationsRegardingDelayedDeliveriesCode)
                }
                "Contract Shipment Advice Code" => Some(ContractShipmentAdviceCode),
                "Individual Insurance Financial Detail" => {
                    Some(IndividualInsuranceFinancialDetail)
                }
                "Cash Discount Stipulation Code" => Some(CashDiscountStipulationCode),
                "Shipment Acceptance Discrepancy Explanation Code" => {
                    Some(ShipmentAcceptanceDiscrepancyExplanationCode)
                }
                "Insurance Plan Description Characteristics" => {
                    Some(InsurancePlanDescriptionCharacteristics)
                }
                "Contract Close-out Group Code" => Some(ContractCloseOutGroupCode),
                "Payment Type Code" => Some(PaymentTypeCode),
                "Contract Fund Reporting Transaction Code" => {
                    Some(ContractFundReportingTransactionCode)
                }
                "Contract Payment Deduction or Collection Code" => {
                    Some(ContractPaymentDeductionOrCollectionCode)
                }
                "Obligation Variance Code" => Some(ObligationVarianceCode),
                "Plus or Minus Indicator Code" => Some(PlusOrMinusIndicatorCode),
                "Reason for Delayed Closing of Contract File Code" => {
                    Some(ReasonForDelayedClosingOfContractFileCode)
                }
                "Contract Payment Line Item Status Code" => {
                    Some(ContractPaymentLineItemStatusCode)
                }
                "Special Reimbursable Provisions Code" => {
                    Some(SpecialReimbursableProvisionsCode)
                }
                "Kind of Modification Code" => Some(KindOfModificationCode),
                "Purchasing Contract Officer (PCO) Instructions Code" => Some(Code29),
                "Type of Delay Code" => Some(TypeOfDelayCode),
                "Healthcare Provider Characteristics and Resources" => {
                    Some(HealthcareProviderCharacteristicsAndResources)
                }
                "Container and Roll-on/Roll-off Number Code" => {
                    Some(ContainerAndRollOnRollOffNumberCode)
                }
                "Air Commodity and Special Handling Code" => {
                    Some(AirCommodityAndSpecialHandlingCode)
                }
                "Water Commodity and Special Handling Code" => {
                    Some(WaterCommodityAndSpecialHandlingCode)
                }
                "Air Dimension Code" => Some(AirDimensionCode),
                "Air Terminal Identifier Code" => Some(AirTerminalIdentifierCode),
                "Water Terminal Identifier Code" => Some(WaterTerminalIdentifierCode),
                "Consolidation and Containerization Point Code" => {
                    Some(ConsolidationAndContainerizationPointCode)
                }
                "Transportation Mode or Method Code" => {
                    Some(TransportationModeOrMethodCode)
                }
                "Type Pack Code" => Some(TypePackCode),
                "Date Shipped or Received Code" => Some(DateShippedOrReceivedCode),
                "Estimated Time of Arrival Code" => Some(EstimatedTimeOfArrivalCode),
                "Military and Civilian Grade Code" => Some(MilitaryAndCivilianGradeCode),
                "Seavan Ownership Code" => Some(SeavanOwnershipCode),
                "Ocean Carrier Code" => Some(OceanCarrierCode),
                "Voyage Document Number Code" => Some(VoyageDocumentNumberCode),
                "Voyage Manifest Reference Code" => Some(VoyageManifestReferenceCode),
                "Vessel Status and Terms of Carriage Code" => {
                    Some(VesselStatusAndTermsOfCarriageCode)
                }
                "Vessel Sustaining Code" => Some(VesselSustainingCode),
                "Subrogation Action Code" => Some(SubrogationActionCode),
                "Billing Advice Code" => Some(BillingAdviceCode),
                "Billing Status Code" => Some(BillingStatusCode),
                "Type of Bill Code" => Some(TypeOfBillCode),
                "Recipient of Billing Status Code" => Some(RecipientOfBillingStatusCode),
                "Sales Price Condition Code" => Some(SalesPriceConditionCode),
                "Delivery Source Code" => Some(DeliverySourceCode),
                "Transportation Bill Code" => Some(TransportationBillCode),
                "Stock Fund or Non-stock Fund Code" => Some(StockFundOrNonStockFundCode),
                "General Services Administration (GSA) Customer Supply Center Number Code" => {
                    Some(Code60)
                }
                "Information Indicator Code" => Some(InformationIndicatorCode),
                "Communications Routing Identifier Code" => {
                    Some(CommunicationsRoutingIdentifierCode)
                }
                "Content Indicator Code" => Some(ContentIndicatorCode),
                "Health Care Claim Status Code" => Some(HealthCareClaimStatusCode),
                "Suffix or Limit Code" => Some(SuffixOrLimitCode),
                "Type of Assistance Code" => Some(TypeOfAssistanceCode),
                "Healthcare Provider Taxonomy" => Some(HealthcareProviderTaxonomy),
                "Foreign Military Sales Country Code" => {
                    Some(ForeignMilitarySalesCountryCode)
                }
                "Service and Agency Code" => Some(ServiceAndAgencyCode),
                "Disbursement Status Code" => Some(DisbursementStatusCode),
                "Aid Type Code" => Some(AidTypeCode),
                "Demand Code" => Some(DemandCode),
                "Suffix Code" => Some(SuffixCode),
                "Project Code" => Some(ProjectCode),
                "Priority Designator Code" => Some(PriorityDesignatorCode),
                "Advice Code" => Some(AdviceCode),
                "Status Code" => Some(StatusCode),
                "Shipment Hold Code" => Some(ShipmentHoldCode),
                "Supply Condition Code" => Some(SupplyConditionCode),
                "Management Code" => Some(ManagementCode),
                "Country and Activity Code" => Some(CountryAndActivityCode),
                "Subsistence Type of Pack Code" => Some(SubsistenceTypeOfPackCode),
                "Disposal Authority Code" => Some(DisposalAuthorityCode),
                "Cooperative Logistics Program Support Code" => {
                    Some(CooperativeLogisticsProgramSupportCode)
                }
                "Precious Metals Indicator Code" => Some(PreciousMetalsIndicatorCode),
                "Automated Data Processing Equipment Identification Code" => {
                    Some(AutomatedDataProcessingEquipmentIdentificationCode)
                }
                "Reason for Disposal Code" => Some(ReasonForDisposalCode),
                "Type of Storage Code" => Some(TypeOfStorageCode),
                "Identification Code" => Some(IdentificationCode),
                "Offer and Release Option Code" => Some(OfferAndReleaseOptionCode),
                "Shipment Release Code" => Some(ShipmentReleaseCode),
                "Ultimate Recipient Code" => Some(UltimateRecipientCode),
                "Reason for Requisitioning Code" => Some(ReasonForRequisitioningCode),
                "Purpose Code" => Some(PurposeCode),
                "Freddie Mac (Federal Home Loan Mortgage Corporation) Special Character Code" => {
                    Some(Code100)
                }
                "Fannie Mae (Federal National Mortgage Association) Special Feature Code" => {
                    Some(Code101)
                }
                "Mortgage Index Source Code" => Some(MortgageIndexSourceCode),
                "Fannie Mae (Federal National Mortgage Association) Remittance Programs" => {
                    Some(Code103)
                }
                "Freddie Mac (Federal Home Loan Mortgage Corporation) Remittance Programs" => {
                    Some(Code104)
                }
                "Freddie Mac (Federal Home Loan Mortgage Corporation) Mortgage Insurance Code" => {
                    Some(Code105)
                }
                "Fannie Mae (Federal National Mortgage Association) Pool Feature Code" => {
                    Some(Code106)
                }
                "Fannie Mae (Federal National Mortgage Association) Mortgage Insurance Code" => {
                    Some(Code107)
                }
                "Testing Statistical Category Code List" => {
                    Some(TestingStatisticalCategoryCodeList)
                }
                "Testing Demographic Category Code List" => {
                    Some(TestingDemographicCategoryCodeList)
                }
                "American Society for Testing and Materials (ASTM)" => Some(CodeA),
                "Ownership Code" => Some(OwnershipCode),
                "Customer Within Country Code" => Some(CustomerWithinCountryCode),
                "Delivery Term Code" => Some(DeliveryTermCode),
                "Case Designator Number" => Some(CaseDesignatorNumber),
                "Subcase Number" => Some(SubcaseNumber),
                "Freight Forwarder Number" => Some(FreightForwarderNumber),
                "Record Control Number" => Some(RecordControlNumber),
                "Program Year Code" => Some(ProgramYearCode),
                "Supplemental Data" => Some(SupplementalData),
                "Country Code (Finance and Acquisition)" => Some(CodeAA),
                "SNOMED, Systematized Nomenclature of Medicine" => Some(CodeAAA),
                "Asset Type" => Some(AssetType),
                "Current Asset Type" => Some(CurrentAssetType),
                "Current Liability Type" => Some(CurrentLiabilityType),
                "Dun and Bradstreet Canada's 8 digit Standard Industrial Classification Code" => {
                    Some(
                        DunAndBradstreetCanadas8DigitStandardIndustrialClassificationCode,
                    )
                }
                "Financial Item Allocation Code" => Some(FinancialItemAllocationCode),
                "Financial Item Attributed Code" => Some(FinancialItemAttributedCode),
                "Financial Item Reclassification Code" => {
                    Some(FinancialItemReclassificationCode)
                }
                "Functional Area" => Some(FunctionalArea),
                "Hobby Code" => Some(HobbyCode),
                "Investment Type" => Some(InvestmentType),
                "Liability Type" => Some(LiabilityType),
                "Projection Type" => Some(ProjectionType),
                "Trend Reason" => Some(TrendReason),
                "NACHA (National Automated Clearing House Association)" => Some(CodeAAQ),
                "CPA (Canadian Payments Association)" => Some(CodeAAR),
                "Proprietary" => Some(Proprietary),
                "Fannie Mae Adjustable Rate Mortgage Plan Codes" => {
                    Some(FannieMaeAdjustableRateMortgagePlanCodes)
                }
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Diagnosis Encountered During Examination and Investigation of Individuals and Populations Code" => {
                    Some(CodeAAU)
                }
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Vaccination, Innoculation or Isolation Code" => {
                    Some(CodeAAV)
                }
                "Immunization Injection Code" => Some(ImmunizationInjectionCode),
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Code" => {
                    Some(CodeAAX)
                }
                "Current Dental Terminology (CDT) Code" => Some(CodeAAY),
                "Defense Priorities and Allocations System Code" => {
                    Some(DefensePrioritiesAndAllocationsSystemCode)
                }
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Diagnosis" => {
                    Some(CodeABF)
                }
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Admitting Diagnosis" => {
                    Some(CodeABJ)
                }
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Principal Diagnosis" => {
                    Some(CodeABK)
                }
                "International Classification of Diseases Clinical Modification (ICD-10-CM) External Cause of Injury Code" => {
                    Some(CodeABN)
                }
                "Assigned by Receiver" => Some(AssignedByReceiver),
                "Assigned by Sender" => Some(AssignedBySender),
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Diagnosis Encountered During Examination and Investigation of Individuals and Populations Code" => {
                    Some(CodeABU)
                }
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Vaccination, Innoculation or Isolation Code" => {
                    Some(CodeABV)
                }
                "Account Characteristics Code" => Some(AccountCharacteristicsCode),
                "Accounting Error Classification Code" => {
                    Some(AccountingErrorClassificationCode)
                }
                "Academic Rank" => Some(AcademicRank),
                "List of DoD Accounting Requirements Codes." => {
                    Some(ListOfDoDAccountingRequirementsCodes)
                }
                "List of DoD Asset Category Codes" => Some(ListOfDoDAssetCategoryCodes),
                "List of DoD Controlled Item Codes." => {
                    Some(ListOfDoDControlledItemCodes)
                }
                "List of DoD Expendability, Recoverability, Reparability Category (ERRC) Codes." => {
                    Some(CodeACV)
                }
                "List of DoD Fiduciary Depreciation Method Codes" => {
                    Some(ListOfDoDFiduciaryDepreciationMethodCodes)
                }
                "List of DoD National Item Identification Number (NIIN) Status Codes" => {
                    Some(CodeACX)
                }
                "Add code ACY, \"List of DoD (Army) Recoverability Codes\"" => {
                    Some(CodeACY)
                }
                "List of DoD Reportable Item Control Codes (RICC)" => Some(CodeACZ),
                "Acquisition Advice Code" => Some(AcquisitionAdviceCode),
                "List of DoD Storage Requirement Codes" => {
                    Some(ListOfDoDStorageRequirementCodes)
                }
                "List of DoD Temperature Controlled Codes" => {
                    Some(ListOfDoDTemperatureControlledCodes)
                }
                "List of DoD Asset Type Codes" => Some(ListOfDoDAssetTypeCodes),
                "List of DoD Utilization Measure Codes" => {
                    Some(ListOfDoDUtilizationMeasureCodes)
                }
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Primary Diagnosis" => {
                    Some(CodeADD)
                }
                "Accounting Adjustment Method" => Some(AccountingAdjustmentMethod),
                "Beneficiary Type" => Some(BeneficiaryType),
                "Army Edit Action Code" => Some(ArmyEditActionCode),
                "Class of Pitch" => Some(ClassOfPitch),
                "Grade of Difficulty" => Some(GradeOfDifficulty),
                "Acquisition Method Suffix Code" => Some(AcquisitionMethodSuffixCode),
                "Acquisition Method Code" => Some(AcquisitionMethodCode),
                "Utilization Code" => Some(UtilizationCode),
                "Adjustment Type" => Some(AdjustmentType),
                "Distribution Code" => Some(DistributionCode),
                "Special Requirements Code" => Some(SpecialRequirementsCode),
                "Allocation Method" => Some(AllocationMethod),
                "Alteration Lookup" => Some(AlterationLookup),
                "Locale of Activity" => Some(LocaleOfActivity),
                "Nature of Event Code" => Some(NatureOfEventCode),
                "Settlement/Payout Options" => Some(SettlementPayoutOptions),
                "Authorized Overrun Indicator" => Some(AuthorizedOverrunIndicator),
                "Activite Principale Exercee (APE) Code" => Some(CodeAPE),
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Patient's Reason for Visit" => {
                    Some(CodeAPR)
                }
                "Application Question Identifier" => Some(ApplicationQuestion),
                "All Quantity Available Indicator" => Some(AllQuantityAvailableIndicator),
                "Arrest Reason" => Some(ArrestReason),
                "Asset Reclassification Denial Code" => {
                    Some(AssetReclassificationDenialCode)
                }
                "Allocation Rank Indicator" => Some(AllocationRankIndicator),
                "Allocation Rank Level" => Some(AllocationRankLevel),
                "Form Type Code" => Some(FormTypeCode),
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Secondary Diagnosis" => {
                    Some(CodeASD)
                }
                "Allegation Type Code" => Some(AllegationTypeCode),
                "International Classification of Diseases Clinical Modification (ICD-10-CM) Tertiary Diagnosis" => {
                    Some(CodeATD)
                }
                "Allocation Transaction Type Code" => Some(AllocationTransactionTypeCode),
                "All Patient Refined Diagnosis Related Groups (APR-DRG)" => Some(CodeAU),
                "Subrogation Payment Options" => Some(SubrogationPaymentOptions),
                "All Patient Diagnosis Related Groups (AP-DRG)" => Some(CodeAW),
                "Ambulatory Patient Groups (APG)" => Some(CodeAX),
                "Subrogation Response Codes" => Some(SubrogationResponseCodes),
                "Subrogation Request Codes" => Some(SubrogationRequestCodes),
                "Bank Administration Institute (BAI)" => Some(CodeB),
                "Vessel Stowage Location Code" => Some(VesselStowageLocationCode),
                "Business Type" => Some(BusinessType),
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Diagnosis" => {
                    Some(CodeBBF)
                }
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Admitting Diagnosis" => {
                    Some(CodeBBJ)
                }
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Principal Diagnosis" => {
                    Some(CodeBBK)
                }
                "International Classification of Diseases Clinical Modification (ICD-11-CM) External Cause of Injury Code" => {
                    Some(CodeBBN)
                }
                "International Classification of Diseases Clinical Modification (ICD-10-PCS) Other Procedure Codes" => {
                    Some(CodeBBQ)
                }
                "International Classification of Diseases Clinical Modification (ICD-10-PCS) Principal Procedure Codes" => {
                    Some(CodeBBR)
                }
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Diagnosis Encountered During Examination and Investigation of Individuals and Populations Code" => {
                    Some(CodeBBU)
                }
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Vaccination, Innoculation or Isolation Code" => {
                    Some(CodeBBV)
                }
                "Transportation Holding Delay Code" => {
                    Some(TransportationHoldingDelayCode)
                }
                "Business Change Code" => Some(BusinessChangeCode),
                "Business Credit Rating" => Some(BusinessCreditRating),
                "Transportation Priority Code" => Some(TransportationPriorityCode),
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Primary Diagnosis" => {
                    Some(CodeBDD)
                }
                "Value" => Some(Value),
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Diagnosis" => {
                    Some(CodeBF)
                }
                "Condition" => Some(Condition),
                "Occurrence" => Some(Occurrence),
                "Occurrence Span" => Some(OccurrenceSpan),
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Admitting Diagnosis" => {
                    Some(CodeBJ)
                }
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Principal Diagnosis" => {
                    Some(CodeBK)
                }
                "Application Fee Status Codes" => Some(ApplicationFeeStatusCodes),
                "International Classification of Diseases Clinical Modification (ICD-9-CM) External Cause of Injury Code (E-codes)" => {
                    Some(CodeBN)
                }
                "Healthcare Common Procedure Coding System" => {
                    Some(HealthcareCommonProcedureCodingSystem)
                }
                "Healthcare Common Procedure Coding System Principal Procedure" => {
                    Some(HealthcareCommonProcedureCodingSystemPrincipalProcedure)
                }
                "Board of Inspection and Survey Part Lookup" => {
                    Some(BoardOfInspectionAndSurveyPartLookup)
                }
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Patient's Reason for Visit" => {
                    Some(CodeBPR)
                }
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Other Procedure Codes" => {
                    Some(CodeBQ)
                }
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Principal Procedure Codes" => {
                    Some(CodeBR)
                }
                "Board of Inspection and Survey Responsibility Lookup" => {
                    Some(BoardOfInspectionAndSurveyResponsibilityLookup)
                }
                "Current Procedural Terminology (CPT) Codes" => Some(CodeBS),
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Secondary Diagnosis" => {
                    Some(CodeBSD)
                }
                "Board of Inspection and Survey Ship Lookup" => {
                    Some(BoardOfInspectionAndSurveyShipLookup)
                }
                "Business Period" => Some(BusinessPeriod),
                "Accident Description" => Some(AccidentDescription),
                "Balance Type Code" => Some(BalanceTypeCode),
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Tertiary Diagnosis" => {
                    Some(CodeBTD)
                }
                "Part of Body Affected" => Some(PartOfBodyAffected),
                "Bid Up Indicator" => Some(BidUpIndicator),
                "Bureau of Labor Statistics Standardized Occupational Codes" => {
                    Some(BureauOfLaborStatisticsStandardizedOccupationalCodes)
                }
                "Education Institution Type Code" => Some(EducationInstitutionTypeCode),
                "Educational Areas Code" => Some(EducationalAreasCode),
                "Profession Type Code" => Some(ProfessionTypeCode),
                "Share Type Code" => Some(ShareTypeCode),
                "Business Size Code" => Some(BusinessSizeCode),
                "Canadian Inter*EDI" => Some(CodeC),
                "Eye Color Code" => Some(EyeColorCode),
                "Hair Color Code" => Some(HairColorCode),
                "Skin Tone Code" => Some(SkinToneCode),
                "Type of Inquiry Code" => Some(TypeOfInquiryCode),
                "Advanced Billing Concepts (ABC) Codes" => Some(CodeCAH),
                "Billed Office Indicator Code" => Some(BilledOfficeIndicatorCode),
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Other Procedure Codes" => {
                    Some(CodeCBQ)
                }
                "International Classification of Diseases Clinical Modification (ICD-11-CM) Principal Procedure Codes" => {
                    Some(CodeCBR)
                }
                "Treasury Symbol Code" => Some(TreasurySymbolCode),
                "Correction To Cause Code" => Some(CorrectionToCauseCode),
                "Supplementary Accounting Classification Code" => {
                    Some(SupplementaryAccountingClassificationCode)
                }
                "Reference and Station Code" => Some(ReferenceAndStationCode),
                "Major Force Program Code" => Some(MajorForceProgramCode),
                "Contractual Flow Indicator" => Some(ContractualFlowIndicator),
                "Aircraft Mission Design Series Code" => {
                    Some(AircraftMissionDesignSeriesCode)
                }
                "Type of Issue Code" => Some(TypeOfIssueCode),
                "Charge Indicator" => Some(ChargeIndicator),
                "Criminal Charge" => Some(CriminalCharge),
                "Collision Industry Electronic Commerce Association (CIECA) - Assignment Type" => {
                    Some(CodeCIE)
                }
                "Criminal Charge Grade" => Some(CriminalChargeGrade),
                "Coupon Adjustment Reason Code" => Some(CouponAdjustmentReasonCode),
                "County Designator Code" => Some(CountyDesignatorCode),
                "Cause Lookup" => Some(CauseLookup),
                "Financial Management Service Cash-Link Code" => {
                    Some(FinancialManagementServiceCashLinkCode)
                }
                "Customer Maintenance Level Lookup" => {
                    Some(CustomerMaintenanceLevelLookup)
                }
                "Cause of Injury Code" => Some(CauseOfInjuryCode),
                "Change Notice Code" => Some(ChangeNoticeCode),
                "Customized Notice Type Code" => Some(CustomizedNoticeTypeCode),
                "Cognizance Symbol" => Some(CognizanceSymbol),
                "Confirming Party Role" => Some(ConfirmingPartyRole),
                "Salvage Disposition Code" => Some(SalvageDispositionCode),
                "Court Party Status" => Some(CourtPartyStatus),
                "Capacity Type Indicator" => Some(CapacityTypeIndicator),
                "Federal Item Identification Guide Criticality (FIIG) Code" => {
                    Some(CodeCR)
                }
                "Complaint Request Code" => Some(ComplaintRequestCode),
                "Causative Research Indicator Code" => {
                    Some(CausativeResearchIndicatorCode)
                }
                "Clause Status Type" => Some(ClauseStatusType),
                "Customer Service Designator" => Some(CustomerServiceDesignator),
                "Corporate Statement Filing Code" => Some(CorporateStatementFilingCode),
                "Compensation Type Codes" => Some(CompensationTypeCodes),
                "Carcass Tracking Code" => Some(CarcassTrackingCode),
                "Cuisine Type Code" => Some(CuisineTypeCode),
                "Coverage Code List" => Some(CoverageCodeList),
                "Controvert Code" => Some(ControvertCode),
                "Conviction Offense Type" => Some(ConvictionOffenseType),
                "Court Document Type Code" => Some(CourtDocumentTypeCode),
                "Driver's License Withdrawal Type" => Some(DriversLicenseWithdrawalType),
                "Driver's License Withdrawal Extent" => {
                    Some(DriversLicenseWithdrawalExtent)
                }
                "Driver's License Withdrawal Basis" => {
                    Some(DriversLicenseWithdrawalBasis)
                }
                "Driver's License Withdrawal Due Process Status" => {
                    Some(DriversLicenseWithdrawalDueProcessStatus)
                }
                "Driver's License Withdrawal Reason" => {
                    Some(DriversLicenseWithdrawalReason)
                }
                "Device Availability Code" => Some(DeviceAvailabilityCode),
                "Document Availability Code" => Some(DocumentAvailabilityCode),
                "All Patient, Severity-Adjusted DRGs (APS-DRG)" => Some(CodeDAP),
                "Debtor Business Type Code" => Some(DebtorBusinessTypeCode),
                "DUN's Standard Industrial Classification (SIC) 2+2, Dun and Bradstreet" => {
                    Some(CodeDBS)
                }
                "Report Distribution Code" => Some(ReportDistributionCode),
                "Cause Code" => Some(CauseCode),
                "Medicare DRG (CMS-DRG & MS-DRG)" => Some(CodeDCM),
                "Disposition Category Change Reject Reason Code" => {
                    Some(DispositionCategoryChangeRejectReasonCode)
                }
                "Disposition Sub-Category Code" => Some(DispositionSubCategoryCode),
                "Disposition Category Code" => Some(DispositionCategoryCode),
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Primary Diagnosis" => {
                    Some(CodeDD)
                }
                "Signal Code" => Some(SignalCode),
                "Media and Status Code" => Some(MediaAndStatusCode),
                "Fund Code" => Some(FundCode),
                "Dynamic Generator Set Code" => Some(DynamicGeneratorSetCode),
                "Drug Detail Code" => Some(DrugDetailCode),
                "Single Use Label Code" => Some(SingleUseLabelCode),
                "International-Refined DRGs (IR-DRG)" => Some(CodeDIR),
                "Remedial Action Code" => Some(RemedialActionCode),
                "Program Originator Code" => Some(ProgramOriginatorCode),
                "Service Contract Act Operation Code" => {
                    Some(ServiceContractActOperationCode)
                }
                "Dynamic Locomotive Tag Code" => Some(DynamicLocomotiveTagCode),
                "Deferral Lookup" => Some(DeferralLookup),
                "Long Term Care DRG - LTC-DRG" => Some(LongTermCareDrgLtcDrg),
                "Agent Status Code" => Some(AgentStatusCode),
                "Demilitarization Integrity Code" => Some(DemilitarizationIntegrityCode),
                "Demilitarization Performed Code" => Some(DemilitarizationPerformedCode),
                "Nature of Debt Code" => Some(NatureOfDebtCode),
                "Document Number Requirement Type" => Some(DocumentNumberRequirementType),
                "Device Operator Type Code" => Some(DeviceOperatorTypeCode),
                "Direction of Flow" => Some(DirectionOfFlow),
                "Producer Financial History Codes" => Some(ProducerFinancialHistoryCodes),
                "Delivery Priority Code" => Some(DeliveryPriorityCode),
                "Association of American Railroads Deprescription Exception List" => {
                    Some(AssociationOfAmericanRailroadsDeprescriptionExceptionList)
                }
                "Association of American Railroads Deprescription Distribution List" => {
                    Some(AssociationOfAmericanRailroadsDeprescriptionDistributionList)
                }
                "Device Status Code" => Some(DeviceStatusCode),
                "Diagnosis Related Group (DRG)" => Some(CodeDR),
                "Refined DRGs (R-DRG)" => Some(CodeDRD),
                "Collision Industry Electronic Commerce Association (CIECA) - Detail Repair Lines Code List" => {
                    Some(CodeDRL)
                }
                "Disposition Services Reimbursement Code" => {
                    Some(DispositionServicesReimbursementCode)
                }
                "Related Device Applicability Code" => {
                    Some(RelatedDeviceApplicabilityCode)
                }
                "Disposition Services Customer Type Code" => {
                    Some(DispositionServicesCustomerTypeCode)
                }
                "Severity DRGs (S-DRG)" => Some(CodeDSD),
                "Disposition Services Indicator Code" => {
                    Some(DispositionServicesIndicatorCode)
                }
                "Data Sets Requested" => Some(DataSetsRequested),
                "Delivery Scheduling Status" => Some(DeliverySchedulingStatus),
                "Debtor Type Code" => Some(DebtorTypeCode),
                "List of codes identifying DoD Distribution Services Terms of Sale." => {
                    Some(ListOfCodesIdentifyingDoDDistributionServicesTermsOfSale)
                }
                "Device Usage Code" => Some(DeviceUsageCode),
                "Demand Planning Status Code" => Some(DemandPlanningStatusCode),
                "Estimating Method Status Code" => Some(EstimatingMethodStatusCode),
                "Contact Status Code" => Some(ContactStatusCode),
                "Type of Firm Code" => Some(TypeOfFirmCode),
                "Reportable Event Status Code" => Some(ReportableEventStatusCode),
                "Diagnostic Statistical Manual of Mental Disorders Code List (DSM)" => {
                    Some(CodeE)
                }
                "Asset Status or Transaction Reporting Code" => {
                    Some(AssetStatusOrTransactionReportingCode)
                }
                "Alabama Campaign Disclosure Report Codes" => {
                    Some(AlabamaCampaignDisclosureReportCodes)
                }
                "Alaska Campaign Disclosure Report Codes" => {
                    Some(AlaskaCampaignDisclosureReportCodes)
                }
                "American Samoa Campaign Disclosure Report Codes" => {
                    Some(AmericanSamoaCampaignDisclosureReportCodes)
                }
                "Arizona Campaign Disclosure Report Codes" => {
                    Some(ArizonaCampaignDisclosureReportCodes)
                }
                "Arkansas Campaign Disclosure Report Codes" => {
                    Some(ArkansasCampaignDisclosureReportCodes)
                }
                "California Campaign Disclosure Report Codes" => {
                    Some(CaliforniaCampaignDisclosureReportCodes)
                }
                "Colorado Campaign Disclosure Report Codes" => {
                    Some(ColoradoCampaignDisclosureReportCodes)
                }
                "Connecticut Campaign Disclosure Report Codes" => {
                    Some(ConnecticutCampaignDisclosureReportCodes)
                }
                "Delaware Campaign Disclosure Report Codes" => {
                    Some(DelawareCampaignDisclosureReportCodes)
                }
                "District of Columbia Campaign Disclosure Report Codes" => {
                    Some(DistrictOfColumbiaCampaignDisclosureReportCodes)
                }
                "Florida Campaign Disclosure Report Codes" => {
                    Some(FloridaCampaignDisclosureReportCodes)
                }
                "Georgia Campaign Disclosure Report Codes" => {
                    Some(GeorgiaCampaignDisclosureReportCodes)
                }
                "Guam Campaign Disclosure Report Codes" => {
                    Some(GuamCampaignDisclosureReportCodes)
                }
                "Hawaii Campaign Disclosure Report Codes" => {
                    Some(HawaiiCampaignDisclosureReportCodes)
                }
                "Idaho Campaign Disclosure Report Codes" => {
                    Some(IdahoCampaignDisclosureReportCodes)
                }
                "Illinois Campaign Disclosure Report Codes" => {
                    Some(IllinoisCampaignDisclosureReportCodes)
                }
                "Indiana Campaign Disclosure Report Codes" => {
                    Some(IndianaCampaignDisclosureReportCodes)
                }
                "Iowa Campaign Disclosure Report Codes" => {
                    Some(IowaCampaignDisclosureReportCodes)
                }
                "Kansas Campaign Disclosure Report Codes" => {
                    Some(KansasCampaignDisclosureReportCodes)
                }
                "Kentucky Campaign Disclosure Report Codes" => {
                    Some(KentuckyCampaignDisclosureReportCodes)
                }
                "Louisiana Campaign Disclosure Report Codes" => {
                    Some(LouisianaCampaignDisclosureReportCodes)
                }
                "Maine Campaign Disclosure Report Codes" => {
                    Some(MaineCampaignDisclosureReportCodes)
                }
                "Maryland Campaign Disclosure Report Codes" => {
                    Some(MarylandCampaignDisclosureReportCodes)
                }
                "Massachusetts Campaign Disclosure Report Codes" => {
                    Some(MassachusettsCampaignDisclosureReportCodes)
                }
                "Michigan Campaign Disclosure Report Codes" => {
                    Some(MichiganCampaignDisclosureReportCodes)
                }
                "Minnesota Campaign Disclosure Report Codes" => {
                    Some(MinnesotaCampaignDisclosureReportCodes)
                }
                "Asset Transfer Status Code" => Some(AssetTransferStatusCode),
                "Mississippi Campaign Disclosure Report Codes" => {
                    Some(MississippiCampaignDisclosureReportCodes)
                }
                "Missouri Campaign Disclosure Report Codes" => {
                    Some(MissouriCampaignDisclosureReportCodes)
                }
                "Montana Campaign Disclosure Report Codes" => {
                    Some(MontanaCampaignDisclosureReportCodes)
                }
                "Nebraska Campaign Disclosure Report Codes" => {
                    Some(NebraskaCampaignDisclosureReportCodes)
                }
                "Nevada Campaign Disclosure Report Codes" => {
                    Some(NevadaCampaignDisclosureReportCodes)
                }
                "New Hampshire Campaign Disclosure Report Codes" => {
                    Some(NewHampshireCampaignDisclosureReportCodes)
                }
                "New Jersey Campaign Disclosure Report Codes" => {
                    Some(NewJerseyCampaignDisclosureReportCodes)
                }
                "New Mexico Campaign Disclosure Report Codes" => {
                    Some(NewMexicoCampaignDisclosureReportCodes)
                }
                "New York Campaign Disclosure Report Codes" => {
                    Some(NewYorkCampaignDisclosureReportCodes)
                }
                "North Carolina Campaign Disclosure Report Codes" => {
                    Some(NorthCarolinaCampaignDisclosureReportCodes)
                }
                "North Dakota Campaign Disclosure Report Codes" => {
                    Some(NorthDakotaCampaignDisclosureReportCodes)
                }
                "Ohio Campaign Disclosure Report Codes" => {
                    Some(OhioCampaignDisclosureReportCodes)
                }
                "Oklahoma Campaign Disclosure Report Codes" => {
                    Some(OklahomaCampaignDisclosureReportCodes)
                }
                "Oregon Campaign Disclosure Report Codes" => {
                    Some(OregonCampaignDisclosureReportCodes)
                }
                "Pennsylvania Campaign Disclosure Report Codes" => {
                    Some(PennsylvaniaCampaignDisclosureReportCodes)
                }
                "Puerto Rico Campaign Disclosure Report Codes" => {
                    Some(PuertoRicoCampaignDisclosureReportCodes)
                }
                "Rhode Island Campaign Disclosure Report Codes" => {
                    Some(RhodeIslandCampaignDisclosureReportCodes)
                }
                "South Carolina Campaign Disclosure Report Codes" => {
                    Some(SouthCarolinaCampaignDisclosureReportCodes)
                }
                "South Dakota Campaign Disclosure Report Codes" => {
                    Some(SouthDakotaCampaignDisclosureReportCodes)
                }
                "Tennessee Campaign Disclosure Report Codes" => {
                    Some(TennesseeCampaignDisclosureReportCodes)
                }
                "Texas Campaign Disclosure Report Codes" => {
                    Some(TexasCampaignDisclosureReportCodes)
                }
                "Utah Campaign Disclosure Report Codes" => {
                    Some(UtahCampaignDisclosureReportCodes)
                }
                "Vermont Campaign Disclosure Report Codes" => {
                    Some(VermontCampaignDisclosureReportCodes)
                }
                "Virginia Campaign Disclosure Report Codes" => {
                    Some(VirginiaCampaignDisclosureReportCodes)
                }
                "Virgin Islands Campaign Disclosure Report Codes" => {
                    Some(VirginIslandsCampaignDisclosureReportCodes)
                }
                "Washington Campaign Disclosure Report Codes" => {
                    Some(WashingtonCampaignDisclosureReportCodes)
                }
                "Certification Requirements Code" => Some(CertificationRequirementsCode),
                "West Virginia Campaign Disclosure Report Codes" => {
                    Some(WestVirginiaCampaignDisclosureReportCodes)
                }
                "Wisconsin Campaign Disclosure Report Codes" => {
                    Some(WisconsinCampaignDisclosureReportCodes)
                }
                "Wyoming Campaign Disclosure Report Codes" => {
                    Some(WyomingCampaignDisclosureReportCodes)
                }
                "Alberta Campaign Disclosure Report Codes" => {
                    Some(AlbertaCampaignDisclosureReportCodes)
                }
                "British Columbia Campaign Disclosure Report Codes" => {
                    Some(BritishColumbiaCampaignDisclosureReportCodes)
                }
                "Manitoba Campaign Disclosure Report Codes" => {
                    Some(ManitobaCampaignDisclosureReportCodes)
                }
                "New Brunswick Campaign Disclosure Report Codes" => {
                    Some(NewBrunswickCampaignDisclosureReportCodes)
                }
                "Newfoundland Campaign Disclosure Report Codes" => {
                    Some(NewfoundlandCampaignDisclosureReportCodes)
                }
                "Northwest Territories Campaign Disclosure Report Codes" => {
                    Some(NorthwestTerritoriesCampaignDisclosureReportCodes)
                }
                "Nova Scotia Campaign Disclosure Report Codes" => {
                    Some(NovaScotiaCampaignDisclosureReportCodes)
                }
                "Ontario Campaign Disclosure Report Codes" => {
                    Some(OntarioCampaignDisclosureReportCodes)
                }
                "Prince Edward Island Campaign Disclosure Report Codes" => {
                    Some(PrinceEdwardIslandCampaignDisclosureReportCodes)
                }
                "Quebec Campaign Disclosure Report Codes" => {
                    Some(QuebecCampaignDisclosureReportCodes)
                }
                "Saskatchewan Campaign Disclosure Report Codes" => {
                    Some(SaskatchewanCampaignDisclosureReportCodes)
                }
                "Yukon Territory Campaign Disclosure Report Codes" => {
                    Some(YukonTerritoryCampaignDisclosureReportCodes)
                }
                "Federal Campaign Disclosure Report Codes" => {
                    Some(FederalCampaignDisclosureReportCodes)
                }
                "Alabama Lobbyist Report Codes" => Some(AlabamaLobbyistReportCodes),
                "Alaska Lobbyist Report Codes" => Some(AlaskaLobbyistReportCodes),
                "Arizona Lobbyist Report Codes" => Some(ArizonaLobbyistReportCodes),
                "Arkansas Lobbyist Report Codes" => Some(ArkansasLobbyistReportCodes),
                "California Lobbyist Report Codes" => Some(CaliforniaLobbyistReportCodes),
                "Colorado Lobbyist Report Codes" => Some(ColoradoLobbyistReportCodes),
                "Connecticut Lobbyist Report Codes" => {
                    Some(ConnecticutLobbyistReportCodes)
                }
                "Delaware Lobbyist Report Codes" => Some(DelawareLobbyistReportCodes),
                "District of Columbia Lobbyist Report Codes" => {
                    Some(DistrictOfColumbiaLobbyistReportCodes)
                }
                "Florida Lobbyist Report Codes" => Some(FloridaLobbyistReportCodes),
                "Coast Designation Code" => Some(CoastDesignationCode),
                "Georgia Lobbyist Report Codes" => Some(GeorgiaLobbyistReportCodes),
                "Hawaii Lobbyist Report Codes" => Some(HawaiiLobbyistReportCodes),
                "Idaho Lobbyist Report Codes" => Some(IdahoLobbyistReportCodes),
                "Illinois Lobbyist Report Codes" => Some(IllinoisLobbyistReportCodes),
                "Indiana Lobbyist Report Codes" => Some(IndianaLobbyistReportCodes),
                "Iowa Lobbyist Report Codes" => Some(IowaLobbyistReportCodes),
                "Kansas Lobbyist Report Codes" => Some(KansasLobbyistReportCodes),
                "Kentucky Lobbyist Report Codes" => Some(KentuckyLobbyistReportCodes),
                "Louisiana Lobbyist Report Codes" => Some(LouisianaLobbyistReportCodes),
                "Maine Lobbyist Report Codes" => Some(MaineLobbyistReportCodes),
                "Maryland Lobbyist Report Codes" => Some(MarylandLobbyistReportCodes),
                "Massachusetts Lobbyist Report Codes" => {
                    Some(MassachusettsLobbyistReportCodes)
                }
                "Michigan Lobbyist Report Codes" => Some(MichiganLobbyistReportCodes),
                "Minnesota Lobbyist Report Codes" => Some(MinnesotaLobbyistReportCodes),
                "Mississippi Lobbyist Report Codes" => {
                    Some(MississippiLobbyistReportCodes)
                }
                "Missouri Lobbyist Report Codes" => Some(MissouriLobbyistReportCodes),
                "Montana Lobbyist Report Codes" => Some(MontanaLobbyistReportCodes),
                "Nebraska Lobbyist Report Codes" => Some(NebraskaLobbyistReportCodes),
                "Nevada Lobbyist Report Codes" => Some(NevadaLobbyistReportCodes),
                "New Hampshire Lobbyist Report Codes" => {
                    Some(NewHampshireLobbyistReportCodes)
                }
                "New Jersey Lobbyist Report Codes" => Some(NewJerseyLobbyistReportCodes),
                "New Mexico Lobbyist Report Codes" => Some(NewMexicoLobbyistReportCodes),
                "New York Lobbyist Report Codes" => Some(NewYorkLobbyistReportCodes),
                "North Carolina Lobbyist Report Codes" => {
                    Some(NorthCarolinaLobbyistReportCodes)
                }
                "North Dakota Lobbyist Report Codes" => {
                    Some(NorthDakotaLobbyistReportCodes)
                }
                "Ohio Lobbyist Report Codes" => Some(OhioLobbyistReportCodes),
                "Competitive Characteristics Code" => {
                    Some(CompetitiveCharacteristicsCode)
                }
                "Oklahoma Lobbyist Report Codes" => Some(OklahomaLobbyistReportCodes),
                "Oregon Lobbyist Report Codes" => Some(OregonLobbyistReportCodes),
                "Pennsylvania Lobbyist Report Codes" => {
                    Some(PennsylvaniaLobbyistReportCodes)
                }
                "Puerto Rico Lobbyist Report Codes" => {
                    Some(PuertoRicoLobbyistReportCodes)
                }
                "Rhode Island Lobbyist Report Codes" => {
                    Some(RhodeIslandLobbyistReportCodes)
                }
                "South Carolina Lobbyist Report Codes" => {
                    Some(SouthCarolinaLobbyistReportCodes)
                }
                "South Dakota Lobbyist Report Codes" => {
                    Some(SouthDakotaLobbyistReportCodes)
                }
                "Tennessee Lobbyist Report Codes" => Some(TennesseeLobbyistReportCodes),
                "Texas Lobbyist Report Codes" => Some(TexasLobbyistReportCodes),
                "Utah Lobbyist Report Codes" => Some(UtahLobbyistReportCodes),
                "Vermont Lobbyist Report Codes" => Some(VermontLobbyistReportCodes),
                "Virginia Lobbyist Report Codes" => Some(VirginiaLobbyistReportCodes),
                "Washington Lobbyist Report Codes" => Some(WashingtonLobbyistReportCodes),
                "West Virginia Lobbyist Report Codes" => {
                    Some(WestVirginiaLobbyistReportCodes)
                }
                "Wisconsin Lobbyist Report Codes" => Some(WisconsinLobbyistReportCodes),
                "Wyoming Lobbyist Report Codes" => Some(WyomingLobbyistReportCodes),
                "New York City Campaign Disclosure Report Codes" => {
                    Some(NewYorkCityCampaignDisclosureReportCodes)
                }
                "Seattle Campaign Disclosure Report Codes" => {
                    Some(SeattleCampaignDisclosureReportCodes)
                }
                "New York City Lobbyist Report Codes" => {
                    Some(NewYorkCityLobbyistReportCodes)
                }
                "Correction or Change for Storage Item Records Code" => {
                    Some(CorrectionOrChangeForStorageItemRecordsCode)
                }
                "Excavation Information Code List" => Some(ExcavationInformationCodeList),
                "Type Due-In Indicator" => Some(TypeDueInIndicator),
                "Discrepancy Indicator Code" => Some(DiscrepancyIndicatorCode),
                "Disposal Condition Code" => Some(DisposalConditionCode),
                "Event or Exposure Code" => Some(EventOrExposureCode),
                "Error Classification Code" => Some(ErrorClassificationCode),
                "Inventory Category Code" => Some(InventoryCategoryCode),
                "Automotive Aftermarket Industry Association (AAIA) Emission Code" => {
                    Some(CodeEMC)
                }
                "Local Source Code" => Some(LocalSourceCode),
                "Adverse Event Outcome Code" => Some(AdverseEventOutcomeCode),
                "Enhanced Ambulatory Patient Groups (EAPG)" => Some(CodeEP),
                "Exchange Price Indicator" => Some(ExchangePriceIndicator),
                "Controlled Inventory Item Code" => Some(ControlledInventoryItemCode),
                "Equipment Request Codes" => Some(EquipmentRequestCodes),
                "Department of Defense Identification Code" => {
                    Some(DepartmentOfDefenseIdentificationCode)
                }
                "Equipment Repair Condition Code" => Some(EquipmentRepairConditionCode),
                "Equipment Repair Job Code" => Some(EquipmentRepairJobCode),
                "Equipment Repair Location Code" => Some(EquipmentRepairLocationCode),
                "Equipment Repair Responsibility Code" => {
                    Some(EquipmentRepairResponsibilityCode)
                }
                "Extension Reason" => Some(ExtensionReason),
                "Electrostatic Discharge Code" => Some(ElectrostaticDischargeCode),
                "Equipment Status Lookup" => Some(EquipmentStatusLookup),
                "Reject Advice Code" => Some(RejectAdviceCode),
                "Estimate Type Lookup" => Some(EstimateTypeLookup),
                "Request Code" => Some(RequestCode),
                "Review Period Indicator Code" => Some(ReviewPeriodIndicatorCode),
                "Small Arms Error Transaction Reject Code" => {
                    Some(SmallArmsErrorTransactionRejectCode)
                }
                "Evaluate Work Candidate Lookup" => Some(EvaluateWorkCandidateLookup),
                "Equipment Why Made Code" => Some(EquipmentWhyMadeCode),
                "Evaluate Work Candidate Reason Lookup" => {
                    Some(EvaluateWorkCandidateReasonLookup)
                }
                "Small Arms Transaction Code" => Some(SmallArmsTransactionCode),
                "Export Declaration" => Some(ExportDeclaration),
                "Export Control Classification Number (ECCN)" => Some(CodeEXP),
                "Special Program Requirement Status Code" => {
                    Some(SpecialProgramRequirementStatusCode)
                }
                "Type Inspection Code" => Some(TypeInspectionCode),
                "Financial Rating" => Some(FinancialRating),
                "Type of Contractor Code" => Some(TypeOfContractorCode),
                "Fannie Mae Refinance Plan Code" => Some(FannieMaeRefinancePlanCode),
                "Type of Media Code" => Some(TypeOfMediaCode),
                "Type Physical Inventory or Transaction History Code" => {
                    Some(TypePhysicalInventoryOrTransactionHistoryCode)
                }
                "Federal Communication, Control and Security Code List 1" => {
                    Some(CodeFC1)
                }
                "Demilitarization Code" => Some(DemilitarizationCode),
                "Shelf Life Code" => Some(ShelfLifeCode),
                "Essentiality Code" => Some(EssentialityCode),
                "Federal Finance Code List 1" => Some(FederalFinanceCodeList1),
                "Source Maintenance and Recoverability Code" => {
                    Some(SourceMaintenanceAndRecoverabilityCode)
                }
                "Type of Location Reconciliation Request" => {
                    Some(TypeOfLocationReconciliationRequest)
                }
                "Federal Health Care Code List 1" => Some(FederalHealthCareCodeList1),
                "Applicant Type" => Some(ApplicantType),
                "Financial Inventory Report Code" => Some(FinancialInventoryReportCode),
                "Antenna Structure Type" => Some(AntennaStructureType),
                "Station Classification" => Some(StationClassification),
                "Radio Frequency Type" => Some(RadioFrequencyType),
                "Federal Logistics Code List 1" => Some(FederalLogisticsCodeList1),
                "Station Classification Type" => Some(StationClassificationType),
                "Former Major Organizational Entity Rule Number" => {
                    Some(FormerMajorOrganizationalEntityRuleNumber)
                }
                "Foreign Military Sales and Military Assistance Program Grant Aid Type of Assistance/Financing Code" => {
                    Some(
                        ForeignMilitarySalesAndMilitaryAssistanceProgramGrantAidTypeOfAssistanceFinancingCode,
                    )
                }
                "Class of Operation" => Some(ClassOfOperation),
                "Antenna Polarization" => Some(AntennaPolarization),
                "Fund Purpose" => Some(FundPurpose),
                "Federal Procurement Code List 1" => Some(FederalProcurementCodeList1),
                "Radio System Type" => Some(RadioSystemType),
                "Frequency Band" => Some(FrequencyBand),
                "Freddie Mac Refinance Plan Code" => Some(FreddieMacRefinancePlanCode),
                "Freight Bill Application Error Edit Codes" => {
                    Some(FreightBillApplicationErrorEditCodes)
                }
                "Area of Operation" => Some(AreaOfOperation),
                "Application Type" => Some(ApplicationType),
                "Federal Transportation Code List 1" => {
                    Some(FederalTransportationCodeList1)
                }
                "Authorization Type" => Some(AuthorizationType),
                "Radio Service Type" => Some(RadioServiceType),
                "Applicant Classification Type" => Some(ApplicantClassificationType),
                "Frequency" => Some(Frequency),
                "Edit Error Code" => Some(EditErrorCode),
                "Risk Class" => Some(RiskClass),
                "Uniform Residential Appraisal Attributes Code" => {
                    Some(UniformResidentialAppraisalAttributesCode)
                }
                "Action Code" => Some(ActionCode),
                "Medium of Transmission Code" => Some(MediumOfTransmissionCode),
                "Management Indicator Code (Petroleum)" => Some(CodeGC),
                "Gain or Loss Indicator Code" => Some(GainOrLossIndicatorCode),
                "Type Adjustment Code" => Some(TypeAdjustmentCode),
                "Type Identity Change Code" => Some(TypeIdentityChangeCode),
                "Transportation Mode Reason Code" => Some(TransportationModeReasonCode),
                "Notification Indicator Code" => Some(NotificationIndicatorCode),
                "Reject Indicator Code" => Some(RejectIndicatorCode),
                "Investigation Status Code" => Some(InvestigationStatusCode),
                "Group Qualifier Code" => Some(GroupQualifierCode),
                "National Council on Compensation Insurance (NCCI) Nature of Injury Code" => {
                    Some(CodeGR)
                }
                "Occupational Safety and Health Administration (OSHA) Nature of Injury Code" => {
                    Some(CodeGS)
                }
                "National Council on Compensation Insurance (NCCI) Part of Body Code" => {
                    Some(CodeGT)
                }
                "Occupational Safety and Health Administration (OSHA) Part of Body Code" => {
                    Some(CodeGU)
                }
                "National Council on Compensation Insurance (NCCI) Source of Injury Code" => {
                    Some(CodeGV)
                }
                "Occupational Safety and Health Administration (OSHA) Source of Injury Code" => {
                    Some(CodeGW)
                }
                "Glass Action Code" => Some(GlassActionCode),
                "Cause of Loss Code" => Some(CauseOfLossCode),
                "Loss Description Code" => Some(LossDescriptionCode),
                "Life/Annuity Status Codes" => Some(LifeAnnuityStatusCodes),
                "Discrepancy Code" => Some(DiscrepancyCode),
                "Discrepancy Advice Code" => Some(DiscrepancyAdviceCode),
                "Institutional Sector or Level Classification Code" => {
                    Some(InstitutionalSectorOrLevelClassificationCode)
                }
                "Discrepancy Status or Disposition Code" => {
                    Some(DiscrepancyStatusOrDispositionCode)
                }
                "Remittance Advice Remark Code" => Some(RemittanceAdviceRemarkCode),
                "Education Staff Type Code" => Some(EducationStaffTypeCode),
                "Education Fee Type Code" => Some(EducationFeeTypeCode),
                "Health Industry Number" => Some(HealthIndustryNumber),
                "Institutional Fee Basis Code" => Some(InstitutionalFeeBasisCode),
                "National Center for Education Statistics Integrated Postsecondary Education Data System Institutional Characteristics Survey Code" => {
                    Some(
                        NationalCenterForEducationStatisticsIntegratedPostsecondaryEducationDataSystemInstitutionalCharacteristicsSurveyCode,
                    )
                }
                "Accreditation, Affiliation, or Licensing Level Code" => Some(CodeHL),
                "National Center for Education Statistics Accreditation or Licensing Type" => {
                    Some(
                        NationalCenterForEducationStatisticsAccreditationOrLicensingType,
                    )
                }
                "Hazardous Material Content Code" => Some(HazardousMaterialContentCode),
                "Hazardous Response Codes" => Some(HazardousResponseCodes),
                "Service Contract Act Occupation Category Code" => {
                    Some(ServiceContractActOccupationCategoryCode)
                }
                "Association of American Railroads Standard Transportation Commodity Code Description Qualifier" => {
                    Some(
                        AssociationOfAmericanRailroadsStandardTransportationCommodityCodeDescriptionQualifier,
                    )
                }
                "Identifying Characteristics" => Some(IdentifyingCharacteristics),
                "Insurance Business Process Application Error Code" => {
                    Some(InsuranceBusinessProcessApplicationErrorCode)
                }
                "Collision Industry Electronic Commerce Association (CIECA) - Inspection" => {
                    Some(CodeIC)
                }
                "International Classification of Functioning Disability and Health (ICF)" => {
                    Some(CodeICF)
                }
                "Identity Disclosure Code" => Some(IdentityDisclosureCode),
                "Investment Fund Type" => Some(InvestmentFundType),
                "Item Management Code" => Some(ItemManagementCode),
                "Impact Recorder Code" => Some(ImpactRecorderCode),
                "Intra-Navy Disposal Release Order Reject Advice Code" => {
                    Some(IntraNavyDisposalReleaseOrderRejectAdviceCode)
                }
                "Impact Axis or Analog Port Code" => Some(ImpactAxisOrAnalogPortCode),
                "Issue Priority Group" => Some(IssuePriorityGroup),
                "IRS Qualification Code" => Some(IrsQualificationCode),
                "Issue, Repair and Requisition Code" => Some(CodeIRR),
                "Imbalance Reporting Type" => Some(ImbalanceReportingType),
                "Initial Treatment Code" => Some(InitialTreatmentCode),
                "Interruptible Transportation Indicator" => {
                    Some(InterruptibleTransportationIndicator)
                }
                "Trade Code" => Some(TradeCode),
                "Summons Type Code" => Some(SummonsTypeCode),
                "Judicial Hearing Type Code" => Some(JudicialHearingTypeCode),
                "Judicial Order Type Code" => Some(JudicialOrderTypeCode),
                "Judicial Sentence Type Code" => Some(JudicialSentenceTypeCode),
                "Court Disposition Code" => Some(CourtDispositionCode),
                "Court Appearance Type Code" => Some(CourtAppearanceTypeCode),
                "Court Pleading Type Code" => Some(CourtPleadingTypeCode),
                "Defendant Plea Type Code" => Some(DefendantPleaTypeCode),
                "Trial Type Code" => Some(TrialTypeCode),
                "Court Case Status Code" => Some(CourtCaseStatusCode),
                "Physical Characteristics Code" => Some(PhysicalCharacteristicsCode),
                "Weight or Fragility Code" => Some(WeightOrFragilityCode),
                "Preservation Material Code" => Some(PreservationMaterialCode),
                "Job Characteristics Lookup" => Some(JobCharacteristicsLookup),
                "Quantity per Unit Pack Code" => Some(QuantityPerUnitPackCode),
                "Preservation Data Code" => Some(PreservationDataCode),
                "Packing Requirement Level A Code" => Some(PackingRequirementLevelACode),
                "Packing Requirement Level B Code" => Some(PackingRequirementLevelBCode),
                "Packing Requirement Level C Code" => Some(PackingRequirementLevelCCode),
                "Intermediate Container Code" => Some(IntermediateContainerCode),
                "Intermediate Container Quantity Code" => {
                    Some(IntermediateContainerQuantityCode)
                }
                "Special Marking Code" => Some(SpecialMarkingCode),
                "Type and Cause Code" => Some(TypeAndCauseCode),
                "Mission Impact Statement Code" => Some(MissionImpactStatementCode),
                "International Standard Designation System for Teeth and Areas of the Oral Cavity" => {
                    Some(
                        InternationalStandardDesignationSystemForTeethAndAreasOfTheOralCavity,
                    )
                }
                "Job Originator Lookup" => Some(JobOriginatorLookup),
                "Universal National Tooth Designation System" => {
                    Some(UniversalNationalToothDesignationSystem)
                }
                "Property Underwriting Condition Code" => {
                    Some(PropertyUnderwritingConditionCode)
                }
                "Deficiency Cause" => Some(DeficiencyCause),
                "Discrepancy" => Some(Discrepancy),
                "Preventive Measure" => Some(PreventiveMeasure),
                "Contractor Alert List Reason" => Some(ContractorAlertListReason),
                "Quality Alert List Reason" => Some(QualityAlertListReason),
                "Contractor Alert List Status" => Some(ContractorAlertListStatus),
                "Nature of Buy" => Some(NatureOfBuy),
                "Type of Procurement" => Some(TypeOfProcurement),
                "Representative Buy Indicator" => Some(RepresentativeBuyIndicator),
                "Assured Delivery Indicator" => Some(AssuredDeliveryIndicator),
                "Award Source" => Some(AwardSource),
                "Termination" => Some(Termination),
                "Patient Event Problem Code" => Some(PatientEventProblemCode),
                "Method Evaluation Code" => Some(MethodEvaluationCode),
                "Result Evaluation Code" => Some(ResultEvaluationCode),
                "Conclusion Evaluation Code" => Some(ConclusionEvaluationCode),
                "Device Event Problem Code" => Some(DeviceEventProblemCode),
                "Dose Form Code" => Some(DoseFormCode),
                "Route Code" => Some(RouteCode),
                "Report Source Code" => Some(ReportSourceCode),
                "Key Event Lookup" => Some(KeyEventLookup),
                "Adverse Event Code" => Some(AdverseEventCode),
                "Line Item Condition Code" => Some(LineItemConditionCode),
                "Contract" => Some(Contract),
                "Contractor Review List Status" => Some(ContractorReviewListStatus),
                "Laboratory Test Condition Code" => Some(LaboratoryTestConditionCode),
                "Location Capacity Flow Indicator" => Some(LocationCapacityFlowIndicator),
                "Collision Industry Electronic Commerce Association (CIECA) - Loss Category" => {
                    Some(CodeLD)
                }
                "Life/Annuity Service Features" => Some(LifeAnnuityServiceFeatures),
                "Life/Annuity Product Code" => Some(LifeAnnuityProductCode),
                "Location Code" => Some(LocationCode),
                "Basis of Jurisdiction Code" => Some(BasisOfJurisdictionCode),
                "Line of Authority" => Some(LineOfAuthority),
                "Principal Party Citizenship Code" => Some(PrincipalPartyCitizenshipCode),
                "Nature of Suit Code" => Some(NatureOfSuitCode),
                "Case Origin Code" => Some(CaseOriginCode),
                "Limit Type" => Some(LimitType),
                "Line of business code" => Some(LineOfBusinessCode),
                "Letter of Recommendation Rating Category" => {
                    Some(LetterOfRecommendationRatingCategory)
                }
                "Location Indicator" => Some(LocationIndicator),
                "Logical Observation Identifier Names and Codes (LOINC) Codes" => {
                    Some(CodeLOI)
                }
                "Deficiency Indicator" => Some(DeficiencyIndicator),
                "Location Purpose Code" => Some(LocationPurposeCode),
                "Delinquency Indicator" => Some(DelinquencyIndicator),
                "Location Quantity Type Indicator" => Some(LocationQuantityTypeIndicator),
                "Test Results Code" => Some(TestResultsCode),
                "Loss Severity Code" => Some(LossSeverityCode),
                "Legal Structure Code" => Some(LegalStructureCode),
                "Laboratory Results Identification Code" => {
                    Some(LaboratoryResultsIdentificationCode)
                }
                "War Reserve Material Requirement Code" => {
                    Some(WarReserveMaterialRequirementCode)
                }
                "Policy Type Code" => Some(PolicyTypeCode),
                "Multi-Media Object" => Some(MultiMediaObject),
                "Material Management Aggregation Code" => {
                    Some(MaterialManagementAggregationCode)
                }
                "Service Contract Act Occupation Classification Code" => {
                    Some(ServiceContractActOccupationClassificationCode)
                }
                "Manual Class Code" => Some(ManualClassCode),
                "Material Control Code" => Some(MaterialControlCode),
                "Generator Set Mounting Code" => Some(GeneratorSetMountingCode),
                "Device Evaluation Code" => Some(DeviceEvaluationCode),
                "Method of Completion Code" => Some(MethodOfCompletionCode),
                "Manager Forced Directed Action" => Some(ManagerForcedDirectedAction),
                "Minority Indicator" => Some(MinorityIndicator),
                "Drug Status Code" => Some(DrugStatusCode),
                "Drug Status Adverse Event Code" => Some(DrugStatusAdverseEventCode),
                "Lot Type Code" => Some(LotTypeCode),
                "Post Market Study Status Code" => Some(PostMarketStudyStatusCode),
                "Mechanization of Contract Administration Services (MOCAS) System Error Code" => {
                    Some(CodeMOC)
                }
                "Major Organizational Entity Rule Number" => {
                    Some(MajorOrganizationalEntityRuleNumber)
                }
                "List of codes identifying DoD Mapping Product Procurement Types." => {
                    Some(ListOfCodesIdentifyingDoDMappingProductProcurementTypes)
                }
                "List of codes identifying DoD Mapping Product Types." => {
                    Some(ListOfCodesIdentifyingDoDMappingProductTypes)
                }
                "Reference Partial Descriptive Method Reason Code" => {
                    Some(ReferencePartialDescriptiveMethodReasonCode)
                }
                "Maximum Rate Indicator" => Some(MaximumRateIndicator),
                "Meter Status" => Some(MeterStatus),
                "Meter Type" => Some(MeterType),
                "Valuation Type Code" => Some(ValuationTypeCode),
                "Plant Clearance Office Code" => Some(PlantClearanceOfficeCode),
                "Nomenclature Activity Classification Economy (NACE) Code" => {
                    Some(CodeNAC)
                }
                "Norme Activite Francaise (NAF) Code" => Some(CodeNAF),
                "Non-Approved Item Name" => Some(NonApprovedItemName),
                "Nature of Suit" => Some(NatureOfSuit),
                "Inventory Type Code" => Some(InventoryTypeCode),
                "No Balance Affecting Transactions" => {
                    Some(NoBalanceAffectingTransactions)
                }
                "Property Record Status Code" => Some(PropertyRecordStatusCode),
                "Invoice Notes Code" => Some(InvoiceNotesCode),
                "Nomination Capacity Exceeded Indicator" => {
                    Some(NominationCapacityExceededIndicator)
                }
                "Control Unit Design Code" => Some(ControlUnitDesignCode),
                "National Drug Code (NDC)" => Some(CodeNDC),
                "Direct Numerical Control System Code" => {
                    Some(DirectNumericalControlSystemCode)
                }
                "Type Numerical Control System Code" => {
                    Some(TypeNumericalControlSystemCode)
                }
                "National Geospatial-Intelligence Agency (NGA) Product Code" => {
                    Some(CodeNGC)
                }
                "Property Source Code" => Some(PropertySourceCode),
                "Nature of Injury Code" => Some(NatureOfInjuryCode),
                "Non-Induction Reason Code" => Some(NonInductionReasonCode),
                "Uniform Freight Classification (UFC) Code" => Some(CodeNJ),
                "National Motor Freight Classification (NMFC) Code" => Some(CodeNK),
                "List of DoD Nonconsumable Item Material Support Codes (NIMSC)" => {
                    Some(CodeNMS)
                }
                "Special Category Code" => Some(SpecialCategoryCode),
                "Automotive Aftermarket Industry Association (AAIA) National Popularity Code" => {
                    Some(CodeNPC)
                }
                "Excess Material Disposition Code" => Some(ExcessMaterialDispositionCode),
                "Hazardous Material Code" => Some(HazardousMaterialCode),
                "Type of Cargo Code" => Some(TypeOfCargoCode),
                "National Uniform Billing Committee (NUBC) Revenue Codes" => {
                    Some(CodeNUB)
                }
                "Source of Loss Code" => Some(SourceOfLossCode),
                "Office of Worker's Compensation Programs (OWCP) Source of Injury Code" => {
                    Some(CodeO1)
                }
                "Office of Worker's Compensation Programs (OWCP) Nature of Injury Code" => {
                    Some(CodeO2)
                }
                "Office of Worker's Compensation Programs (OWCP) Part of Body Code" => {
                    Some(CodeO3)
                }
                "Office of Worker's Compensation Programs (OWCP) Occupation Code" => {
                    Some(CodeO4)
                }
                "Occupation Code" => Some(OccupationCode),
                "Ceiling Type Code" => Some(CeilingTypeCode),
                "Prior Damage Location Code" => Some(PriorDamageLocationCode),
                "Part of Body Code" => Some(PartOfBodyCode),
                "Collision Industry Electronic Commerce Association (CIECA) - Profile" => {
                    Some(CodePC)
                }
                "Planning Code Lookup" => Some(PlanningCodeLookup),
                "Pseudo Closure Reason Code" => Some(PseudoClosureReasonCode),
                "Professional Designation" => Some(ProfessionalDesignation),
                "Pre-determined Allocation (PDA) Transaction Type Code" => Some(CodePDA),
                "Petroleum United States Geological Survey (USGS) Formation Code" => {
                    Some(CodePGS)
                }
                "Phrase Code" => Some(PhraseCode),
                "Collision Industry Electronic Commerce Association (CIECA) - Points of Impact" => {
                    Some(CodePI)
                }
                "Petroleum Bill Type" => Some(PetroleumBillType),
                "Priority Lookup" => Some(PriorityLookup),
                "Petroleum Land Category" => Some(PetroleumLandCategory),
                "Petroleum Lease Status" => Some(PetroleumLeaseStatus),
                "Part of Body and Nature of Injury" => Some(PartOfBodyAndNatureOfInjury),
                "Place of Service Code" => Some(PlaceOfServiceCode),
                "Petroleum Product Disposition" => Some(PetroleumProductDisposition),
                "Petroleum Product Point-of-Sale" => Some(PetroleumProductPointOfSale),
                "Petroleum Product Selling Arrangement" => {
                    Some(PetroleumProductSellingArrangement)
                }
                "Petroleum Product Value Adjustment" => {
                    Some(PetroleumProductValueAdjustment)
                }
                "Petroleum Quantity Allocations Code" => {
                    Some(PetroleumQuantityAllocationsCode)
                }
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Patient's Reason for Visit" => {
                    Some(CodePR)
                }
                "Petroleum Royalty Adjustment" => Some(PetroleumRoyaltyAdjustment),
                "Petroleum Royalty Calculation Method" => {
                    Some(PetroleumRoyaltyCalculationMethod)
                }
                "Processing Rights Indicator" => Some(ProcessingRightsIndicator),
                "Petroleum Regulatory Report" => Some(PetroleumRegulatoryReport),
                "Petroleum Royalty Transaction" => Some(PetroleumRoyaltyTransaction),
                "Professional Status Code" => Some(ProfessionalStatusCode),
                "Price Tier" => Some(PriceTier),
                "Petroleum Well Action" => Some(PetroleumWellAction),
                "Petroleum Well Information" => Some(PetroleumWell),
                "Petroleum Well Shut-In Reason" => Some(PetroleumWellShutInReason),
                "Petroleum Well Classification Status" => {
                    Some(PetroleumWellClassificationStatus)
                }
                "Petroleum Well Test Information" => Some(PetroleumWellTest),
                "Surface Descriptor Code" => Some(SurfaceDescriptorCode),
                "Response Status Code" => Some(ResponseStatusCode),
                "Business Entity Filing Report Type Code" => {
                    Some(BusinessEntityFilingReportTypeCode)
                }
                "Business Entity Filing Detail Code" => {
                    Some(BusinessEntityFilingDetailCode)
                }
                "Product Quality Deficiency Report Summary Code" => {
                    Some(ProductQualityDeficiencyReportSummaryCode)
                }
                "Domestic Line of Business Code" => Some(DomesticLineOfBusinessCode),
                "Foreign Line of Business Code" => Some(ForeignLineOfBusinessCode),
                "Business Entity Filing Status Code" => {
                    Some(BusinessEntityFilingStatusCode)
                }
                "Business Entity Filing Securities Information Code" => {
                    Some(BusinessEntityFilingSecuritiesInformationCode)
                }
                "Business Entity Financial Information Code" => {
                    Some(BusinessEntityFinancialInformationCode)
                }
                "Business Entity Status Code" => Some(BusinessEntityStatusCode),
                "Business Entity Filing Location Code" => {
                    Some(BusinessEntityFilingLocationCode)
                }
                "Query Status" => Some(QueryStatus),
                "Quantity Type Indicator" => Some(QuantityTypeIndicator),
                "Coverage Modifier" => Some(CoverageModifier),
                "Upstream Rank (Priority)" => Some(CodeR1),
                "Receipt Rank (Priority)" => Some(CodeR2),
                "Delivery Rank (Priority)" => Some(CodeR3),
                "Downstream Rank (Priority)" => Some(CodeR4),
                "Threaded Rank" => Some(ThreadedRank),
                "Religious Affiliation Code" => Some(ReligiousAffiliationCode),
                "Receipt Acceptance Site Code" => Some(ReceiptAcceptanceSiteCode),
                "Requirement Code" => Some(RequirementCode),
                "Registered Contractor Activity Code" => {
                    Some(RegisteredContractorActivityCode)
                }
                "Property Ownership Type Code" => Some(PropertyOwnershipTypeCode),
                "Property Type Code" => Some(PropertyTypeCode),
                "Race or Ethnicity Collection Code" => {
                    Some(RaceOrEthnicityCollectionCode)
                }
                "Reduction Reason Code" => Some(ReductionReasonCode),
                "Association of American Railroads Rate EDI Network Error Code" => {
                    Some(AssociationOfAmericanRailroadsRateEdiNetworkErrorCode)
                }
                "Classification of Race or Ethnicity" => {
                    Some(ClassificationOfRaceOrEthnicity)
                }
                "Reference Number Format Code" => Some(ReferenceNumberFormatCode),
                "Reason for Movement Code" => Some(ReasonForMovementCode),
                "Residency Indicator" => Some(ResidencyIndicator),
                "Reference Number Justification Code" => {
                    Some(ReferenceNumberJustificationCode)
                }
                "Insurance Industry Specific Remark Codes" => {
                    Some(InsuranceIndustrySpecificRemarkCodes)
                }
                "Reference Number Category Code" => Some(ReferenceNumberCategoryCode),
                "Replenishment Demand Information" => Some(ReplenishmentDemand),
                "Testing Service Question Code List" => {
                    Some(TestingServiceQuestionCodeList)
                }
                "Retail Demand Information" => Some(RetailDemand),
                "Reason for Reversal Code" => Some(ReasonForReversalCode),
                "Receipt Scheduling Status" => Some(ReceiptSchedulingStatus),
                "Request Type" => Some(RequestType),
                "Registration Type Code" => Some(RegistrationTypeCode),
                "Refrigeration Unit Operating Mode Code" => {
                    Some(RefrigerationUnitOperatingModeCode)
                }
                "Reference Number Variation Code" => Some(ReferenceNumberVariationCode),
                "National Council for Prescription Drug Programs Reject Codes" => {
                    Some(NationalCouncilForPrescriptionDrugProgramsRejectCodes)
                }
                "Society for Worldwide Interbank Financial Telecommunications (SWIFT)" => {
                    Some(CodeS)
                }
                "Student Activity Type Code" => Some(StudentActivityTypeCode),
                "Security Assistance Document Number Requirement Type Code" => {
                    Some(SecurityAssistanceDocumentNumberRequirementTypeCode)
                }
                "Stock Action/Technical Information Code" => {
                    Some(StockActionTechnicalInformationCode)
                }
                "Student Award Code" => Some(StudentAwardCode),
                "Statistic Bundes Amt (SBA) Code" => Some(CodeSBA),
                "Source" => Some(Source),
                "Subsequent Cycle Indicator" => Some(SubsequentCycleIndicator),
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Secondary Diagnosis" => {
                    Some(CodeSD)
                }
                "Sound Code" => Some(SoundCode),
                "Stock Exchange Code" => Some(StockExchangeCode),
                "Settlement Type" => Some(SettlementType),
                "Run Type" => Some(RunType),
                "Swing Fuel Option Indicator" => Some(SwingFuelOptionIndicator),
                "Source of Deposit Code" => Some(SourceOfDepositCode),
                "Source of Lead Code" => Some(SourceOfLeadCode),
                "Safety Hazard Lookup" => Some(SafetyHazardLookup),
                "Accident Resulting Change Code" => Some(AccidentResultingChangeCode),
                "Active Mitigation Consideration Code" => {
                    Some(ActiveMitigationConsiderationCode)
                }
                "Activity Methods Code" => Some(ActivityMethodsCode),
                "Analytical Method Code" => Some(AnalyticalMethodCode),
                "Atmospheric Stability Class Code" => Some(AtmosphericStabilityClassCode),
                "Basis of Estimate Code" => Some(BasisOfEstimateCode),
                "Certification Code" => Some(CertificationCode),
                "Contributing Factor Code" => Some(ContributingFactorCode),
                "Control Device Type Code" => Some(ControlDeviceTypeCode),
                "Design/Standard Code" => Some(DesignStandardCode),
                "Device Classification Code" => Some(DeviceClassificationCode),
                "Discharge Indicator Code" => Some(DischargeIndicatorCode),
                "Discharge Quantity Range Code" => Some(DischargeQuantityRangeCode),
                "Non-Reportable Discharge Indicator Code" => {
                    Some(NonReportableDischargeIndicatorCode)
                }
                "SIC (Standard Industrial Classification)" => Some(CodeSI),
                "Emergency Response Regulation/Statute Code" => {
                    Some(EmergencyResponseRegulationStatuteCode)
                }
                "Emission Factor Type Code" => Some(EmissionFactorTypeCode),
                "Emission Release Point Type Code" => Some(EmissionReleasePointTypeCode),
                "Emission Source Type Code" => Some(EmissionSourceTypeCode),
                "Emission Type Code" => Some(EmissionTypeCode),
                "Emission Unit Type Code" => Some(EmissionUnitTypeCode),
                "Endpoint Code" => Some(EndpointCode),
                "Environment Code" => Some(EnvironmentCode),
                "Environmental Program Type Code" => Some(EnvironmentalProgramTypeCode),
                "Environmental Receptor Code" => Some(EnvironmentalReceptorCode),
                "Facility Category Code" => Some(FacilityCategoryCode),
                "Facility Status Code" => Some(FacilityStatusCode),
                "Factor Calculation Method Code" => Some(FactorCalculationMethodCode),
                "Frequency of Analysis Code" => Some(FrequencyOfAnalysisCode),
                "Generator Status Code" => Some(GeneratorStatusCode),
                "Geometric Type Code" => Some(GeometricTypeCode),
                "Hazardous Waste Form Code" => Some(HazardousWasteFormCode),
                "Horizontal Datum Code" => Some(HorizontalDatumCode),
                "Information System Code" => Some(InformationSystemCode),
                "Initiating Event Code" => Some(InitiatingEventCode),
                "Inventory Quantity Range Code" => Some(InventoryQuantityRangeCode),
                "Latitude/Longitude Source Code" => Some(LatitudeLongitudeSourceCode),
                "Latitude/Longitude Verification Code" => {
                    Some(LatitudeLongitudeVerificationCode)
                }
                "Location Description Code" => Some(LocationDescriptionCode),
                "Major Hazard Code" => Some(MajorHazardCode),
                "Manufacturing Code" => Some(ManufacturingCode),
                "Source of Injury Code" => Some(SourceOfInjuryCode),
                "Material Classification Code" => Some(MaterialClassificationCode),
                "Material Code" => Some(MaterialCode),
                "Maximum Achievable Control Technology Code" => {
                    Some(MaximumAchievableControlTechnologyCode)
                }
                "Method of Collection Code" => Some(MethodOfCollectionCode),
                "Mitigation System Code" => Some(MitigationSystemCode),
                "Model Used Code" => Some(ModelUsedCode),
                "Monitoring/Detection System Code" => Some(MonitoringDetectionSystemCode),
                "Monitoring Location Code" => Some(MonitoringLocationCode),
                "Non-Generating Waste Code" => Some(NonGeneratingWasteCode),
                "Off-Site Availability Code" => Some(OffSiteAvailabilityCode),
                "Off-Site Impact Code" => Some(OffSiteImpactCode),
                "On-Site Impact Code" => Some(OnSiteImpactCode),
                "On-Site Process System Type Code" => Some(OnSiteProcessSystemTypeCode),
                "Origin Code" => Some(OriginCode),
                "Parameter Code" => Some(ParameterCode),
                "Passive Mitigation Consideration Code" => {
                    Some(PassiveMitigationConsiderationCode)
                }
                "Permit Compliance Status Code" => Some(PermitComplianceStatusCode),
                "Physical State Code" => Some(PhysicalStateCode),
                "Point of Measurement Code" => Some(PointOfMeasurementCode),
                "Preservative Code" => Some(PreservativeCode),
                "Process Code" => Some(ProcessCode),
                "Process Control Code" => Some(ProcessControlCode),
                "Process Hazard Analysis Update Resulting Change Code" => {
                    Some(ProcessHazardAnalysisUpdateResultingChangeCode)
                }
                "Process Hazards Analysis Technique Code" => {
                    Some(ProcessHazardsAnalysisTechniqueCode)
                }
                "Public Receptor Code" => Some(PublicReceptorCode),
                "Range of Concentration Code" => Some(RangeOfConcentrationCode),
                "Recovery Method Code" => Some(RecoveryMethodCode),
                "Recycling Method Code" => Some(RecyclingMethodCode),
                "Release Event Code" => Some(ReleaseEventCode),
                "Release Source Code" => Some(ReleaseSourceCode),
                "Reliability Indicator Code" => Some(ReliabilityIndicatorCode),
                "Rule Effectiveness Method Code" => Some(RuleEffectivenessMethodCode),
                "Sample Type Code" => Some(SampleTypeCode),
                "Scenario Code" => Some(ScenarioCode),
                "Site Location Code" => Some(SiteLocationCode),
                "Source Category Code" => Some(SourceCategoryCode),
                "Source of Waste Generation Code" => Some(SourceOfWasteGenerationCode),
                "Source Reduction Activity Code" => Some(SourceReductionActivityCode),
                "System Type Code" => Some(SystemTypeCode),
                "Time Period Code" => Some(TimePeriodCode),
                "Topography Code" => Some(TopographyCode),
                "Transfer Quantity Range Code" => Some(TransferQuantityRangeCode),
                "Type of Competency Testing Code" => Some(TypeOfCompetencyTestingCode),
                "Type of Training Code" => Some(TypeOfTrainingCode),
                "Type of Waste Management Code" => Some(TypeOfWasteManagementCode),
                "Use Code" => Some(UseCode),
                "Waste Emanation Code" => Some(WasteEmanationCode),
                "Waste Management Status Code" => Some(WasteManagementStatusCode),
                "Waste Stream Code" => Some(WasteStreamCode),
                "Waste Treatment Method Code" => Some(WasteTreatmentMethodCode),
                "Wind Direction Code" => Some(WindDirectionCode),
                "Unit of Measure Code" => Some(UnitOfMeasureCode),
                "Secondary Source of Injury" => Some(SecondarySourceOfInjury),
                "Shelf-Life Action Code" => Some(ShelfLifeActionCode),
                "Stockage List Code" => Some(StockageListCode),
                "Scheduling Status" => Some(SchedulingStatus),
                "Statement Basis" => Some(StatementBasis),
                "Special Material Content Code" => Some(SpecialMaterialContentCode),
                "Sample Device" => Some(SampleDevice),
                "Special Material Identification Code" => {
                    Some(SpecialMaterialIdentificationCode)
                }
                "Sample Type" => Some(SampleType),
                "Solicitation Cancellation Reason" => {
                    Some(SolicitationCancellationReason)
                }
                "Standard Occupation Classification Code" => {
                    Some(StandardOccupationClassificationCode)
                }
                "Submitter's Priority Designator" => Some(SubmittersPriorityDesignator),
                "Special Dating" => Some(SpecialDating),
                "Statistical Administrative Information Code" => {
                    Some(StatisticalAdministrativeInformationCode)
                }
                "Service Requester Agent Indicator" => {
                    Some(ServiceRequesterAgentIndicator)
                }
                "List of codes identifying DoD Serialized Report Type Codes." => {
                    Some(ListOfCodesIdentifyingDoDSerializedReportTypeCodes)
                }
                "Special Requirement Lookup" => Some(SpecialRequirementLookup),
                "Supplemental Reduction Reason" => Some(SupplementalReductionReason),
                "Storage Report Type" => Some(StorageReportType),
                "System Status" => Some(SystemStatus),
                "Supply Status Code" => Some(SupplyStatusCode),
                "Special Marketing Type Code" => Some(SpecialMarketingTypeCode),
                "Association of American Railroads Standard Transportation Commodity Code Master Description Information" => {
                    Some(
                        AssociationOfAmericanRailroadsStandardTransportationCommodityCodeMasterDescription,
                    )
                }
                "Forward and Store Application Error Edit Codes" => {
                    Some(ForwardAndStoreApplicationErrorEditCodes)
                }
                "Solicited/Unsolicited Indicator" => Some(SolicitedUnsolicitedIndicator),
                "Service Code" => Some(ServiceCode),
                "Association of American Railroads Switch Release Codes" => {
                    Some(AssociationOfAmericanRailroadsSwitchReleaseCodes)
                }
                "Personal Property and Contents Code" => {
                    Some(PersonalPropertyAndContentsCode)
                }
                "Commercial Vehicle Operations Safety Code" => {
                    Some(CommercialVehicleOperationsSafetyCode)
                }
                "Data Categories" => Some(DataCategories),
                "Event Codes" => Some(EventCodes),
                "Operation Type" => Some(OperationType),
                "Accident Parameters" => Some(AccidentParameters),
                "Inspection Parameters" => Some(InspectionParameters),
                "Driver Parameters" => Some(DriverParameters),
                "View Parameters" => Some(ViewParameters),
                "Vehicle Parameters" => Some(VehicleParameters),
                "Fleet Parameters" => Some(FleetParameters),
                "Query Options" => Some(QueryOptions),
                "Jurisdiction Type" => Some(JurisdictionType),
                "Single State Registration System and Operating Authority Credential" => {
                    Some(SingleStateRegistrationSystemAndOperatingAuthorityCredential)
                }
                "Commercial Vehicle Operations Insurance" => {
                    Some(CommercialVehicleOperationsInsurance)
                }
                "Commercial Vehicle Registration" => Some(CommercialVehicleRegistration),
                "Hazardous Materials Credential" => Some(HazardousMaterialsCredential),
                "Oversize/Overweight Credential" => Some(OversizeOverweightCredential),
                "Commercial Vehicle Tax" => Some(CommercialVehicleTax),
                "Commercial Vehicle Title" => Some(CommercialVehicleTitle),
                "Commercial Driver's License" => Some(CommercialDriversLicense),
                "Commercial Vehicle Type" => Some(CommercialVehicleType),
                "Commercial Vehicle Operations Status Code" => {
                    Some(CommercialVehicleOperationsStatusCode)
                }
                "Safety and Fitness Electronic Record Systems Subscription Option" => {
                    Some(SafetyAndFitnessElectronicRecordSystemsSubscriptionOption)
                }
                "Commercial Vehicle Operations Commodity Code" => {
                    Some(CommercialVehicleOperationsCommodityCode)
                }
                "Commercial Vehicle Operations Hazardous Material Code" => {
                    Some(CommercialVehicleOperationsHazardousMaterialCode)
                }
                "Safety and Fitness Electronic Record Systems Error Code" => {
                    Some(SafetyAndFitnessElectronicRecordSystemsErrorCode)
                }
                "Commercial Vehicle Operations Jurisdiction Identifier Code" => {
                    Some(CommercialVehicleOperationsJurisdictionIdentifierCode)
                }
                "Compliance Review Code" => Some(ComplianceReviewCode),
                "Incident Condition Code" => Some(IncidentConditionCode),
                "Incident Related Action Code" => Some(IncidentRelatedActionCode),
                "Incident Location Code" => Some(IncidentLocationCode),
                "Incident Consequence Code" => Some(IncidentConsequenceCode),
                "Road Characteristic Code" => Some(RoadCharacteristicCode),
                "Vehicle Occupant Code" => Some(VehicleOccupantCode),
                "Package Failure Code" => Some(PackageFailureCode),
                "Pedestrian Code" => Some(PedestrianCode),
                "Association for Financial Professionals Service Code and Bank Service Code" => {
                    Some(
                        AssociationForFinancialProfessionalsServiceCodeAndBankServiceCode,
                    )
                }
                "Treatment Codes" => Some(TreatmentCodes),
                "Type of Change Code" => Some(TypeOfChangeCode),
                "Collision Industry Electronic Commerce Association (CIECA) - Totals Code List" => {
                    Some(CodeTCD)
                }
                "Template Characteristic Lookup" => Some(TemplateCharacteristicLookup),
                "International Classification of Diseases Clinical Modification (ICD-9-CM) Tertiary Diagnosis" => {
                    Some(CodeTD)
                }
                "Discrepancy Report Type Code" => Some(DiscrepancyReportTypeCode),
                "Technical Data Justification Code" => {
                    Some(TechnicalDataJustificationCode)
                }
                "Association for Financial Professionals Service Code" => {
                    Some(AssociationForFinancialProfessionalsServiceCode)
                }
                "Title Exception and Requirement Code List" => {
                    Some(TitleExceptionAndRequirementCodeList)
                }
                "Tax or Fee Exemption Reason Code" => Some(TaxOrFeeExemptionReasonCode),
                "Title Document Code List" => Some(TitleDocumentCodeList),
                "Tap Location" => Some(TapLocation),
                "Weapon System Transaction Origination Code" => {
                    Some(WeaponSystemTransactionOriginationCode)
                }
                "Template Owner Lookup" => Some(TemplateOwnerLookup),
                "Tap Type" => Some(TapType),
                "Systemized Nomenclature of Dentistry (SNODENT)" => Some(CodeTQ),
                "Report Code" => Some(ReportCode),
                "Natural Gas Transaction Type" => Some(NaturalGasTransactionType),
                "Downstream Transaction Type" => Some(DownstreamTransactionType),
                "Template Type Lookup" => Some(TemplateTypeLookup),
                "Upstream Transaction Type" => Some(UpstreamTransactionType),
                "Follow-up Code" => Some(FollowUpCode),
                "Reportable Event Code" => Some(ReportableEventCode),
                "Residential and Commercial Room Code" => {
                    Some(ResidentialAndCommercialRoomCode)
                }
                "UMLER Body Type" => Some(UmlerBodyType),
                "Unique Item Tracking Designator Code" => {
                    Some(UniqueItemTrackingDesignatorCode)
                }
                "Unique Item Tracking Error Reject Code" => {
                    Some(UniqueItemTrackingErrorRejectCode)
                }
                "UMLER Fitting Code" => Some(UmlerFittingCode),
                "Urgency Justification Code" => Some(UrgencyJustificationCode),
                "(UN/SPSC) United Nations Products and Services Classification Code" => {
                    Some(CodeUNP)
                }
                "Unclaimed Property Additions, Deletions, and Deductions Codes" => {
                    Some(CodeUPC)
                }
                "Unclaimed Property Type Code" => Some(UnclaimedPropertyTypeCode),
                "Event Reappearance Code" => Some(EventReappearanceCode),
                "Event Abatement Code" => Some(EventAbatementCode),
                "Centers for Medicare and Medicaid Services (CMS) Certificate of Medical Necessity (CMN) forms" => {
                    Some(CodeUT)
                }
                "Unique Item Tracking Transaction Code" => {
                    Some(UniqueItemTrackingTransactionCode)
                }
                "Unit Code" => Some(UnitCode),
                "Violation Type Code List" => Some(ViolationTypeCodeList),
                "Validation Code" => Some(ValidationCode),
                "Collision Industry Electronic Commerce Association (CIECA) - Vehicle Line Item Category Code" => {
                    Some(CodeVP)
                }
                "Court Issued Warrant Type Code" => Some(CourtIssuedWarrantTypeCode),
                "Weapon System Advice Code" => Some(WeaponSystemAdviceCode),
                "When Discovered Lookup" => Some(WhenDiscoveredLookup),
                "Weapon System Essentiality Code" => Some(WeaponSystemEssentialityCode),
                "Automotive Aftermarket Industry Association (AAIA) Warranty Code" => {
                    Some(CodeWRC)
                }
                "Weapon System Status Code" => Some(WeaponSystemStatusCode),
                "Weapon System Designator Code" => Some(WeaponSystemDesignatorCode),
                "Weapon System Maintenance Code" => Some(WeaponSystemMaintenanceCode),
                "Vehicle Class" => Some(VehicleClass),
                "Rental Charge" => Some(RentalCharge),
                "Cancellation Reason" => Some(CancellationReason),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for CodeListQualifierCode {
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
    type Value = CodeListQualifierCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Code List Qualifier Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CodeListQualifierCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Code List Qualifier Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CodeListQualifierCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Code List Qualifier Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for CodeListQualifierCode {
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