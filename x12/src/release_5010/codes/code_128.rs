use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**128

See docs at <https://www.stedi.com/edi/x12/element/128>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReferenceIdentificationQualifier {
    ///00 - Contracting District Number
    ContractingDistrictNumber,
    ///0A - Supervisory Appraiser Certification Number
    SupervisoryAppraiserCertificationNumber,
    ///0B - State License Number
    StateLicenseNumber,
    ///0D - Subject Property Verification Source
    SubjectPropertyVerificationSource,
    ///0E - Subject Property Reference Number
    SubjectPropertyReferenceNumber,
    ///0F - Subscriber Number
    SubscriberNumber,
    ///0G - Reviewer File Number
    ReviewerFileNumber,
    ///0H - Comparable Property Pending Sale Reference Number
    ComparablePropertyPendingSaleReferenceNumber,
    ///0I - Comparable Property Sale Reference Number
    ComparablePropertySaleReferenceNumber,
    ///0J - Subject Property Non-Sale Reference Number
    SubjectPropertyNonSaleReferenceNumber,
    ///0K - Policy Form Identifying Number
    PolicyFormIdentifyingNumber,
    ///0L - Referenced By
    ReferencedBy,
    ///0M - Mortgage Identification Number
    MortgageIdentificationNumber,
    ///0N - Attached To
    AttachedTo,
    ///0P - Real Estate Owned Property Identifier
    RealEstateOwnedProperty,
    ///01 - American Bankers Assoc. (ABA) Transit/Routing Number (Including Check Digit, 9 Digits)
    Code01,
    ///1A - Blue Cross Provider Number
    BlueCrossProviderNumber,
    ///01A - Catalog of Federal Domestic Assistance
    CatalogOfFederalDomesticAssistance,
    ///1B - Blue Shield Provider Number
    BlueShieldProviderNumber,
    ///01B - Union Agreement
    UnionAgreement,
    ///1C - Medicare Provider Number
    MedicareProviderNumber,
    ///01C - Military Standard Requisitioning and Issue Procedures (MILSTRIP) Document Number
    Code01C,
    ///1D - Medicaid Provider Number
    MedicaidProviderNumber,
    ///01D - Federal Standard Requisitioning and Issue Procedures (FEDSTRIP) Document Number
    Code01D,
    ///1E - Dentist License Number
    DentistLicenseNumber,
    ///01E - Federal Supply Schedule Special (FSS) Item Number
    Code01E,
    ///1F - Anesthesia License Number
    AnesthesiaLicenseNumber,
    ///1G - Provider UPIN Number
    ProviderUpinNumber,
    ///01G - Payment Related Clause
    PaymentRelatedClause,
    ///1H - CHAMPUS Identification Number
    ChampusIdentificationNumber,
    ///01H - Special Price Authorization Number
    SpecialPriceAuthorizationNumber,
    ///1I - Department of Defense Identification Code (DoDIC)
    Code1I,
    ///1J - Facility ID Number
    FacilityIdNumber,
    ///1K - Payor's Claim Number
    PayorsClaimNumber,
    ///1L - Group or Policy Number
    GroupOrPolicyNumber,
    ///1M - Preferred Provider Organization Site Number
    PreferredProviderOrganizationSiteNumber,
    ///1N - Diagnosis Related Group (DRG) Number
    Code1N,
    ///1O - Consolidation Shipment Number
    ConsolidationShipmentNumber,
    ///1P - Accessorial Status Code
    AccessorialStatusCode,
    ///1Q - Error Identification Code
    ErrorIdentificationCode,
    ///1R - Storage Information Code
    StorageInformationCode,
    ///1S - Ambulatory Patient Group (APG) Number
    Code1S,
    ///1T - Resource Utilization Group (RUG) Number
    Code1T,
    ///1U - Pay Grade
    PayGrade,
    ///1V - Related Vendor Order Number
    RelatedVendorOrderNumber,
    ///1W - Member Identification Number
    MemberIdentificationNumber,
    ///1X - Credit or Debit Adjustment Number
    CreditOrDebitAdjustmentNumber,
    ///1Y - Repair Action Number
    RepairActionNumber,
    ///1Z - Financial Detail Code
    FinancialDetailCode,
    ///02 - Society for Worldwide Interbank Financial Telecommunication (S.W.I.F.T.) Identification (8 or 11 Characters)
    Code02,
    ///2A - Import License Number
    ImportLicenseNumber,
    ///2B - Terminal Release Order Number
    TerminalReleaseOrderNumber,
    ///2C - Long-term Disability Policy Number
    LongTermDisabilityPolicyNumber,
    ///2D - Aeronautical Equipment Reference Number (AERNO)
    Code2D,
    ///2E - Foreign Military Sales Case Number
    ForeignMilitarySalesCaseNumber,
    ///2F - Consolidated Invoice Number
    ConsolidatedInvoiceNumber,
    ///2G - Amendment
    Amendment,
    ///2H - Assigned by transaction set sender
    AssignedByTransactionSetSender,
    ///2I - Tracking Number
    TrackingNumber,
    ///2J - Floor Number
    FloorNumber,
    ///2K - Food and Drug Administration (FDA) Product Type
    Code2K,
    ///2L - Association of American Railroads (AAR) Railway Accounting Rules
    Code2L,
    ///2M - Federal Communications Commission (FCC) Identifier
    Code2M,
    ///2N - Federal Communications Commission (FCC) Trade/Brand Identifier
    Code2N,
    ///2O - Occupational Safety and Health Administration (OSHA) Claim Number
    Code2O,
    ///2P - Subdivision Identifier
    Subdivision,
    ///2Q - Food and Drug Administration (FDA) Accession Number
    Code2Q,
    ///2R - Coupon Redemption Number
    CouponRedemptionNumber,
    ///2S - Catalog
    Catalog,
    ///2T - Sub-subhouse Bill of Lading
    SubSubhouseBillOfLading,
    ///2U - Payer Identification Number
    PayerIdentificationNumber,
    ///2V - Special Government Accounting Classification Reference Number (ACRN)
    Code2V,
    ///2W - Change Order Authority
    ChangeOrderAuthority,
    ///2X - Supplemental Agreement Authority
    SupplementalAgreementAuthority,
    ///2Y - Wage Determination
    WageDetermination,
    ///2Z - U.S. Customs Service (USCS) Anti-dumping Duty Case Number
    Code2Z,
    ///03 - Clearing House Interbank Payment System (CHIPS) Participant Number (3 or 4 Digits)
    Code03,
    ///3A - Section of the National Housing Act Code
    SectionOfTheNationalHousingActCode,
    ///3B - Supplemental Claim Number
    SupplementalClaimNumber,
    ///3C - Payee Loan Number
    PayeeLoanNumber,
    ///3D - Servicer Loan Number
    ServicerLoanNumber,
    ///3E - Investor Loan Number
    InvestorLoanNumber,
    ///3F - Show Identification
    ShowIdentification,
    ///3G - Catastrophe Number
    CatastropheNumber,
    ///3H - Case Number
    CaseNumber,
    ///3I - Precinct Number
    PrecinctNumber,
    ///3J - Office Number
    OfficeNumber,
    ///3K - Petroleum Pool Code
    PetroleumPoolCode,
    ///3L - Branch Identifier
    Branch,
    ///3M - Federal Communications Commission (FCC) Condition Code
    Code3M,
    ///3N - Gas Custodian Identification
    GasCustodianIdentification,
    ///3O - U.S. Customs Service (USCS) Pre-approval Ruling Number
    Code3O,
    ///3P - Third Party Originator Number
    ThirdPartyOriginatorNumber,
    ///3Q - Food and Drug Administration (FDA) Product Code
    Code3Q,
    ///3R - U.S. Customs Service (USCS) Binding Ruling Number
    Code3R,
    ///3S - Provincial (Canadian) Sales Tax Exemption Number
    Code3S,
    ///3T - U.S. Customs Service (USCS) Pre-classification Ruling Number
    Code3T,
    ///3U - Protraction Number
    ProtractionNumber,
    ///3V - Formation Identifier
    FormationIdentifier,
    ///3W - U.S. Customs Service (USCS) Commercial Description
    Code3W,
    ///3X - Subcontract Number
    SubcontractNumber,
    ///3Y - Receiver Assigned Drop Zone
    ReceiverAssignedDropZone,
    ///3Z - Customs Broker Reference Number
    CustomsBrokerReferenceNumber,
    ///04 - Canadian Financial Institution Branch and Institution Number
    CanadianFinancialInstitutionBranchAndInstitutionNumber,
    ///4A - Personal Identification Number (PIN)
    Code4A,
    ///4B - Shipment Origin Code
    ShipmentOriginCode,
    ///4C - Shipment Destination Code
    ShipmentDestinationCode,
    ///4D - Shipping Zone
    ShippingZone,
    ///4E - Carrier-assigned Consignee Number
    CarrierAssignedConsigneeNumber,
    ///4F - Carrier-assigned Shipper Number
    CarrierAssignedShipperNumber,
    ///4G - Provincial Tax Identification
    ProvincialTaxIdentification,
    ///4H - Commercial Invoice Number
    CommercialInvoiceNumber,
    ///4I - Balance-due Reference Number
    BalanceDueReferenceNumber,
    ///4J - Vehicle-related Services Reference Number
    VehicleRelatedServicesReferenceNumber,
    ///4K - Accessorial Rail Diversion Reference Number
    AccessorialRailDiversionReferenceNumber,
    ///4L - Location-specific Services Reference Number
    LocationSpecificServicesReferenceNumber,
    ///4M - Special Move Reference Number
    SpecialMoveReferenceNumber,
    ///4N - Special Payment Reference Number
    SpecialPaymentReferenceNumber,
    ///4O - Canadian Goods & Services or Quebec Sales Tax Reference Number
    Code4O,
    ///4P - Affiliation Number
    AffiliationNumber,
    ///4Q - Call Sign
    CallSign,
    ///4R - Rule Section
    RuleSection,
    ///4S - Preferred Call Sign
    PreferredCallSign,
    ///4T - North American Datum Standard (NADS)
    Code4T,
    ///4U - Market Area
    MarketArea,
    ///4V - Emission Designator
    EmissionDesignator,
    ///4W - Study
    Study,
    ///4X - Log
    Log,
    ///4Y - Subhouse Bill of Lading
    SubhouseBillOfLading,
    ///4Z - U.S. Customs Service (USCS) Countervailing Duty Case Number
    Code4Z,
    ///05 - Clearing House Interbank Payment System (CHIPS) User Identification (6 digits)
    Code05,
    ///5A - Offense Tracking
    OffenseTracking,
    ///5B - Supplemental Account Number
    SupplementalAccountNumber,
    ///5C - Congressional District
    CongressionalDistrict,
    ///5D - Line of Credit Category
    LineOfCreditCategory,
    ///5E - Consumer Identifier
    Consumer,
    ///5F - Warrant
    Warrant,
    ///5G - Complaint
    Complaint,
    ///5H - Incident
    Incident,
    ///5I - Offender Tracking
    OffenderTracking,
    ///5J - Driver's License
    DriversLicense,
    ///5K - Commercial Driver's License
    CommercialDriversLicense,
    ///5L - Jurisdictional Community Number
    JurisdictionalCommunityNumber,
    ///5M - Previous Sequence
    PreviousSequence,
    ///5N - Citation of Statute
    CitationOfStatute,
    ///5O - Citation of Opinion
    CitationOfOpinion,
    ///5P - National Criminal Information Center Originating Agency Identification
    NationalCriminalInformationCenterOriginatingAgencyIdentification,
    ///5Q - State Criminal History Repository Individual Identification
    StateCriminalHistoryRepositoryIndividualIdentification,
    ///5R - Federal Bureau of Investigation Individual Identification
    FederalBureauOfInvestigationIndividualIdentification,
    ///5S - Processing Area
    ProcessingArea,
    ///5T - Payment Location
    PaymentLocation,
    ///5U - Flood Data Identifier
    FloodData,
    ///5V - Coupon Distribution Method
    CouponDistributionMethod,
    ///5W - Original Uniform Commercial Code Filing Number
    OriginalUniformCommercialCodeFilingNumber,
    ///5X - Amended Uniform Commercial Code Filing Number
    AmendedUniformCommercialCodeFilingNumber,
    ///5Y - Continuation Uniform Commercial Code Filing Number
    ContinuationUniformCommercialCodeFilingNumber,
    ///5Z - Uniform Commercial Code Filing Collateral Number
    UniformCommercialCodeFilingCollateralNumber,
    ///06 - System Number
    SystemNumber,
    ///6A - Consignee Reference Number
    ConsigneeReferenceNumber,
    ///6B - U.S. Customs Service (USCS) Entry Number
    Code6B,
    ///6C - U.S. Customs Service (USCS) Entry Type Code
    Code6C,
    ///6D - U.S. Customs Service (USCS) Statement Number
    Code6D,
    ///6E - Map Reference
    MapReference,
    ///6F - Appraiser License
    AppraiserLicense,
    ///6G - Map Number
    MapNumber,
    ///6H - Comparable Property Verification Source
    ComparablePropertyVerificationSource,
    ///6I - Comparable Property
    ComparableProperty,
    ///6J - Census Tract
    CensusTract,
    ///6K - Zone
    Zone,
    ///6L - Agent Contract Number
    AgentContractNumber,
    ///6M - Application Number
    ApplicationNumber,
    ///6N - Claimant Number
    ClaimantNumber,
    ///6O - Cross Reference Number
    CrossReferenceNumber,
    ///6P - Group Number
    GroupNumber,
    ///6Q - Insurance License Number
    InsuranceLicenseNumber,
    ///6R - Provider Control Number
    ProviderControlNumber,
    ///6S - Provider Order Ticket Number
    ProviderOrderTicketNumber,
    ///6T - Pilot License Number
    PilotLicenseNumber,
    ///6U - Question Number
    QuestionNumber,
    ///6V - Reissue Cession Number
    ReissueCessionNumber,
    ///6X - Specimen Identifier
    Specimen,
    ///6Y - Equipment Initial
    EquipmentInitial,
    ///6Z - Secretaria de Comercia y Famenta Industrial (SECOFI) Number
    Code6Z,
    ///07 - Add-On System Number
    AddOnSystemNumber,
    ///7A - Purchase Order Number Included in On-Order Position
    PurchaseOrderNumberIncludedInOnOrderPosition,
    ///7B - Purchase Order Number of Shipment Received since Last Reporting Date
    PurchaseOrderNumberOfShipmentReceivedSinceLastReportingDate,
    ///7C - Purchase Order Number of Order Received since Last Reporting Date
    PurchaseOrderNumberOfOrderReceivedSinceLastReportingDate,
    ///7D - Tester Identification
    TesterIdentification,
    ///7E - Collector Identification
    CollectorIdentification,
    ///7F - Repeat Location
    RepeatLocation,
    ///7G - Data Quality Reject Reason
    DataQualityRejectReason,
    ///7H - Environmental Protection Agency (EPA) Test Type Purpose Code
    Code7H,
    ///7I - Subscriber Authorization Number
    SubscriberAuthorizationNumber,
    ///7J - Toll Billing Telephone Reference Number
    TollBillingTelephoneReferenceNumber,
    ///7K - List of Materials
    ListOfMaterials,
    ///7L - Qualified Materials List
    QualifiedMaterialsList,
    ///7M - Frame
    Frame,
    ///7N - Piggyback
    Piggyback,
    ///7O - Tripleback
    Tripleback,
    ///7P - Sheet
    Sheet,
    ///7Q - Engineering Change Order
    EngineeringChangeOrder,
    ///7R - Representative Identification Number
    RepresentativeIdentificationNumber,
    ///7S - Drawing Type
    DrawingType,
    ///7T - Master Contract
    MasterContract,
    ///7U - Related Transaction Reference Number
    RelatedTransactionReferenceNumber,
    ///7W - Interchange Train Identification
    InterchangeTrainIdentification,
    ///7X - Home Mortgage Disclosure Act (HMDA) State Code
    Code7X,
    ///7Y - Home Mortgage Disclosure Act (HMDA) County Code
    Code7Y,
    ///7Z - Home Mortgage Disclosure Act (HMDA) Metropolitan Statistical Area (MSA)
    Code7Z,
    ///08 - Carrier Assigned Package Identification Number
    CarrierAssignedPackageIdentificationNumber,
    ///8A - Health Maintenance Organization (HMO) Authorization Number
    Code8A,
    ///8B - Preferred Provider Organization (PPO) Authorization Number
    Code8B,
    ///8C - Third-party Organization (TPO) Authorization Number
    Code8C,
    ///8D - Chemical Abstract Service Registry Number
    ChemicalAbstractServiceRegistryNumber,
    ///8E - Guarantor Loan Number
    GuarantorLoanNumber,
    ///8F - School Loan Number
    SchoolLoanNumber,
    ///8G - Automated Clearinghouse (ACH) Trace Number
    Code8G,
    ///8H - Check List Number
    CheckListNumber,
    ///8I - FEDWIRE Confirmation Number
    FedwireConfirmationNumber,
    ///8J - Society for Worldwide Interbank Financial Telecommunications (SWIFT) Confirmation Number
    Code8J,
    ///8K - Dominion of Canada Code
    DominionOfCanadaCode,
    ///8L - International Standard Industry Classification Code (ISIC)
    Code8L,
    ///8M - Originating Company Identifier
    OriginatingCompany,
    ///8N - Receiving Company Identifier
    ReceivingCompany,
    ///8O - Automated Clearing House (ACH) Entry Description
    Code8O,
    ///8P - Originating Depository Financial Institution Identifier
    OriginatingDepositoryFinancialInstitution,
    ///8Q - Receiving Depository Financial Institution Identifier
    ReceivingDepositoryFinancialInstitution,
    ///8R - Security Type
    SecurityType,
    ///8S - Broker Identification
    BrokerIdentification,
    ///8U - Bank Assigned Security Identifier
    BankAssignedSecurity,
    ///8V - Credit Reference
    CreditReference,
    ///8W - Bank to Bank Information
    BankToBank,
    ///8X - Transaction Category or Type
    TransactionCategoryOrType,
    ///8Y - Safekeeping Account Number
    SafekeepingAccountNumber,
    ///8Z - Alternate Clause Number
    AlternateClauseNumber,
    ///09 - Customs Bar Code Number
    CustomsBarCodeNumber,
    ///9A - Repriced Claim Reference Number
    RepricedClaimReferenceNumber,
    ///9B - Repriced Line Item Reference Number
    RepricedLineItemReferenceNumber,
    ///9C - Adjusted Repriced Claim Reference Number
    AdjustedRepricedClaimReferenceNumber,
    ///9D - Adjusted Repriced Line Item Reference Number
    AdjustedRepricedLineItemReferenceNumber,
    ///9E - Replacement Claim Number
    ReplacementClaimNumber,
    ///9F - Referral Number
    ReferralNumber,
    ///9G - Department of Defense Form 250 Requirement Code
    DepartmentOfDefenseForm250RequirementCode,
    ///9H - Packaging Group Number
    PackagingGroupNumber,
    ///9I - Automated Clearing House (ACH) Standard Entry Class
    Code9I,
    ///9J - Pension Contract
    PensionContract,
    ///9K - Servicer
    Servicer,
    ///9L - Service Bureau
    ServiceBureau,
    ///9M - Clearing House Interbank Payments System (CHIPS) Sequence Number
    Code9M,
    ///9N - Investor
    Investor,
    ///9P - Loan Type
    LoanType,
    ///9Q - Pool Suffix
    PoolSuffix,
    ///9R - Job Order Number
    JobOrderNumber,
    ///9S - Delivery Region
    DeliveryRegion,
    ///9T - Tenor
    Tenor,
    ///9U - Loan Feature Code
    LoanFeatureCode,
    ///9V - Payment Category
    PaymentCategory,
    ///9W - Payer Category
    PayerCategory,
    ///9X - Account Category
    AccountCategory,
    ///9Y - Bank Assigned Bankers Reference Number
    BankAssignedBankersReferenceNumber,
    ///9Z - Chamber of Commerce Number
    ChamberOfCommerceNumber,
    ///10 - Account Managers Code
    AccountManagersCode,
    ///11 - Account Number
    AccountNumber,
    ///12 - Billing Account
    BillingAccount,
    ///13 - Horizontal Coordinate
    HorizontalCoordinate,
    ///14 - Master Account Number
    MasterAccountNumber,
    ///15 - Vertical Coordinate
    VerticalCoordinate,
    ///16 - Military Interdepartmental Purchase Request (MIPR) Number
    Code16,
    ///17 - Client Reporting Category
    ClientReportingCategory,
    ///18 - Plan Number
    PlanNumber,
    ///19 - Division Identifier
    Division,
    ///20 - Repair Part Number
    RepairPartNumber,
    ///21 - American Gas Association Equation Number
    AmericanGasAssociationEquationNumber,
    ///22 - Special Charge or Allowance Code
    SpecialChargeOrAllowanceCode,
    ///23 - Client Number
    ClientNumber,
    ///24 - Short-term Disability Policy Number
    ShortTermDisabilityPolicyNumber,
    ///25 - Reason Not Lowest Cost Code
    ReasonNotLowestCostCode,
    ///26 - Union Number
    UnionNumber,
    ///27 - Insuror Pool Identification Number
    InsurorPoolIdentificationNumber,
    ///28 - Employee Identification Number
    EmployeeIdentificationNumber,
    ///29 - Foreclosure Account Number
    ForeclosureAccountNumber,
    ///30 - United States Government Visa Number
    UnitedStatesGovernmentVisaNumber,
    ///31 - Docket Number
    DocketNumber,
    ///32 - Credit Repository Code
    CreditRepositoryCode,
    ///33 - Lender Case Number
    LenderCaseNumber,
    ///34 - Loan Request Number
    LoanRequestNumber,
    ///35 - Multifamily Project Number
    MultifamilyProjectNumber,
    ///36 - Underwriter Identification Number
    UnderwriterIdentificationNumber,
    ///37 - Condominium Identification Number
    CondominiumIdentificationNumber,
    ///38 - Master Policy Number
    MasterPolicyNumber,
    ///39 - Proposal Number
    ProposalNumber,
    ///40 - Lease Schedule Number - Replacement
    LeaseScheduleNumberReplacement,
    ///41 - Lease Schedule Number - Prior
    LeaseScheduleNumberPrior,
    ///42 - Phone Calls
    PhoneCalls,
    ///43 - Supporting Document Number
    SupportingDocumentNumber,
    ///44 - End Use Number
    EndUseNumber,
    ///45 - Old Account Number
    OldAccountNumber,
    ///46 - Old Meter Number
    OldMeterNumber,
    ///47 - Plate Number
    PlateNumber,
    ///48 - Agency's Student Number
    AgencysStudentNumber,
    ///49 - Family Unit Number
    FamilyUnitNumber,
    ///50 - State Student Identification Number
    StateStudentIdentificationNumber,
    ///51 - Picture Number
    PictureNumber,
    ///52 - SWIFT (MT 100)
    Code52,
    ///53 - SWIFT (MT 202)
    Code53,
    ///54 - FEDWIRE (Federal Wire Transfer)
    Code54,
    ///55 - Sequence Number
    SequenceNumber,
    ///56 - Corrected Social Security Number
    CorrectedSocialSecurityNumber,
    ///57 - Prior Incorrect Social Security Number
    PriorIncorrectSocialSecurityNumber,
    ///58 - Corrected Batch Number
    CorrectedBatchNumber,
    ///59 - Prior Incorrect Batch Number
    PriorIncorrectBatchNumber,
    ///60 - Account Suffix Code
    AccountSuffixCode,
    ///61 - Taxing Authority Identification Number
    TaxingAuthorityIdentificationNumber,
    ///63 - Prior Loan Number
    PriorLoanNumber,
    ///64 - Jurisdictional Community Name Identifier
    JurisdictionalCommunityName,
    ///65 - Total Order Cycle Number
    TotalOrderCycleNumber,
    ///66 - Previous Policy Number
    PreviousPolicyNumber,
    ///67 - Previous Claim History Identifier
    PreviousClaimHistory,
    ///68 - Dental Insurance Account Number
    DentalInsuranceAccountNumber,
    ///69 - Dental Insurance Policy Number
    DentalInsurancePolicyNumber,
    ///70 - Calendar Number
    CalendarNumber,
    ///71 - (Working) Shift Number
    Code71,
    ///72 - Schedule Reference Number
    ScheduleReferenceNumber,
    ///73 - Statement of Work (SOW)
    Code73,
    ///74 - Work Breakdown Structure (WBS)
    Code74,
    ///75 - Organization Breakdown Structure
    OrganizationBreakdownStructure,
    ///76 - Milestone
    Milestone,
    ///77 - Work Package
    WorkPackage,
    ///78 - Planning Package
    PlanningPackage,
    ///79 - Cost Account
    CostAccount,
    ///80 - Charge Number
    ChargeNumber,
    ///81 - Symbol Number (for Milestone or LOB reports)
    Code81,
    ///82 - Data Item Description (DID) Reference
    Code82,
    ///83 - Extended (or Exhibit) Line Item Number (ELIN)
    Code83,
    ///84 - Contractor Data Requirements List (CDRL)
    Code84,
    ///85 - Subcontractor Data Requirements (SDRL)
    Code85,
    ///86 - Operation Number
    OperationNumber,
    ///87 - Functional Category
    FunctionalCategory,
    ///88 - Work Center
    WorkCenter,
    ///89 - Assembly Number
    AssemblyNumber,
    ///90 - Subassembly Number
    SubassemblyNumber,
    ///91 - Cost Element
    CostElement,
    ///92 - Change Document Number
    ChangeDocumentNumber,
    ///93 - Funds Authorization
    FundsAuthorization,
    ///94 - File Identification Number
    FileIdentificationNumber,
    ///95 - Committee on Uniform Security Identification Procedures (CUSIP) Number
    Code95,
    ///96 - Stock Certificate Number
    StockCertificateNumber,
    ///97 - Package Number
    PackageNumber,
    ///98 - Container/Packaging Specification Number
    ContainerPackagingSpecificationNumber,
    ///99 - Rate Conference ID Code
    RateConferenceIdCode,
    ///A0 - Advertiser Number
    AdvertiserNumber,
    ///A1 - Analysis number/Test number
    AnalysisNumberTestNumber,
    ///A2 - Disability Insurance Account Number
    DisabilityInsuranceAccountNumber,
    ///A3 - Assignment Number
    AssignmentNumber,
    ///A4 - Disability Insurance Policy Number
    DisabilityInsurancePolicyNumber,
    ///A5 - Educational Institution Identification Number
    EducationalInstitutionIdentificationNumber,
    ///A7 - Flexible Spending Account (FSA) Insurance Account Number
    CodeA7,
    ///A8 - Flexible Spending Account (FSA) Insurance Policy Number
    CodeA8,
    ///A9 - Health Insurance Account Number
    HealthInsuranceAccountNumber,
    ///AA - Accounts Receivable Statement Number
    AccountsReceivableStatementNumber,
    ///AAA - Distributor's Split Agent Number
    DistributorsSplitAgentNumber,
    ///AAB - Fund Manager's Reference Number
    FundManagersReferenceNumber,
    ///AAC - Agency Hierarchical Level
    AgencyHierarchicalLevel,
    ///AAD - Officer License Number
    OfficerLicenseNumber,
    ///AAE - Previous Distributor Number
    PreviousDistributorNumber,
    ///AAF - Interviewer ID
    InterviewerId,
    ///AAG - Military ID
    MilitaryId,
    ///AAH - Option Policy Number
    OptionPolicyNumber,
    ///AAI - Payroll Account Number
    PayrollAccountNumber,
    ///AAJ - Prior Contract Number
    PriorContractNumber,
    ///AAK - Worksite Number
    WorksiteNumber,
    ///AAL - Agent Number
    AgentNumber,
    ///AAM - Treaty Identifier
    Treaty,
    ///AAN - Associated Case Control Number
    AssociatedCaseControlNumber,
    ///AAO - Carrier Assigned Code
    CarrierAssignedCode,
    ///AAP - Dealer Number
    DealerNumber,
    ///AAQ - Directory Number
    DirectoryNumber,
    ///AAR - Distributor Assigned Transaction Number
    DistributorAssignedTransactionNumber,
    ///AAS - Distributor Assigned Order Number
    DistributorAssignedOrderNumber,
    ///AAT - Distributor's Account Number
    DistributorsAccountNumber,
    ///AAU - General Agency Number
    GeneralAgencyNumber,
    ///AAV - Laboratory Number
    LaboratoryNumber,
    ///AAW - Agency Assigned Number
    AgencyAssignedNumber,
    ///AAX - List Bill Number
    ListBillNumber,
    ///AAY - Accounting Period Reference
    AccountingPeriodReference,
    ///AAZ - Paramedical ID Number
    ParamedicalIdNumber,
    ///AB - Acceptable Source Purchaser ID
    AcceptableSourcePurchaserId,
    ///ABA - Payroll Number
    PayrollNumber,
    ///ABB - Personal ID Number
    PersonalIdNumber,
    ///ABC - Policy Link Number
    PolicyLinkNumber,
    ///ABD - Secondary Policy Number
    SecondaryPolicyNumber,
    ///ABE - Special Quote Number
    SpecialQuoteNumber,
    ///ABF - National Property Registry System Level 1
    NationalPropertyRegistrySystemLevel1,
    ///ABG - National Property Registry System Level 2
    NationalPropertyRegistrySystemLevel2,
    ///ABH - Investor Assigned Identification Number
    InvestorAssignedIdentificationNumber,
    ///ABI - Motor Fuel Certificate Number
    MotorFuelCertificateNumber,
    ///ABJ - Ginnie Mae (Government National Mortgage Association) Pool Package Number
    CodeABJ,
    ///ABK - Mortgage Electronic Registration System Organization Identifier
    MortgageElectronicRegistrationSystemOrganization,
    ///ABL - Seller Loan Number
    SellerLoanNumber,
    ///ABM - Sub-Servicer Loan Number
    SubServicerLoanNumber,
    ///ABN - National Property Registry System Level 3
    NationalPropertyRegistrySystemLevel3,
    ///ABO - State Hazardous Waste Entity Identifier
    StateHazardousWasteEntity,
    ///ABP - Bankruptcy Procedure Number
    BankruptcyProcedureNumber,
    ///ABQ - National Business Identification Number
    NationalBusinessIdentificationNumber,
    ///ABR - Prior Data Universal Number System (D-U-N-S) Number, Dun & Bradstreet
    CodeABR,
    ///ABS - Vessel Name
    VesselName,
    ///ABT - Security Instrument Number
    SecurityInstrumentNumber,
    ///ABU - Assignment Recording Number
    AssignmentRecordingNumber,
    ///ABV - Book Number
    BookNumber,
    ///ABW - Business Tax Number
    BusinessTaxNumber,
    ///ABX - North American Industrial Classification System Code-2
    NorthAmericanIndustrialClassificationSystemCode2,
    ///ABY - Centers for Medicare and Medicaid Services PlanID
    CentersForMedicareAndMedicaidServicesPlanId,
    ///ABZ - Employment Visa
    EmploymentVisa,
    ///AC - Air Cargo Transfer Manifest
    AirCargoTransferManifest,
    ///ACA - Growth Factor Reference
    GrowthFactorReference,
    ///ACB - Region
    Region,
    ///ACC - Status
    Status,
    ///ACD - Class Code
    ClassCode,
    ///ACE - Service Request Number
    ServiceRequestNumber,
    ///ACF - Supplement Number
    SupplementNumber,
    ///ACG - Previous Ticket Number
    PreviousTicketNumber,
    ///ACH - One Call Agency Ticket Number
    OneCallAgencyTicketNumber,
    ///ACI - Ticket Number
    TicketNumber,
    ///ACJ - Bill of Material Revision Number
    BillOfMaterialRevisionNumber,
    ///ACK - Drawing Revision Number
    DrawingRevisionNumber,
    ///ACL - Application Transaction Reference Number
    ApplicationTransactionReferenceNumber,
    ///ACM - Related Object Identification Number
    RelatedObjectIdentificationNumber,
    ///ACN - Common Access Reference Number
    CommonAccessReferenceNumber,
    ///ACO - First Transfer Number
    FirstTransferNumber,
    ///ACP - Continuous Transfer Number
    ContinuousTransferNumber,
    ///ACQ - Last Transfer Number
    LastTransferNumber,
    ///ACR - Automated Clearinghouse (ACH) Return/Notification of Change (NOC) Code
    CodeACR,
    ///ACS - Society of Property Information Compilers and Analysts
    SocietyOfPropertyInformationCompilersAndAnalysts,
    ///ACT - Accounting Code
    AccountingCode,
    ///ACU - Green Card
    GreenCard,
    ///ACV - Agency Assigned Employee ID
    AgencyAssignedEmployeeId,
    ///ACW - Passport
    Passport,
    ///ACX - Unemployment Insurance Number
    UnemploymentInsuranceNumber,
    ///ACY - North American Industrial Classification System Code-1
    NorthAmericanIndustrialClassificationSystemCode1,
    ///ACZ - Occupation Code
    OccupationCode,
    ///AD - Acceptable Source DUNS Number
    AcceptableSourceDunsNumber,
    ///ADA - Agency for International Development Acquisition Regulation (AIDAR)
    CodeADA,
    ///ADB - Master Property Number
    MasterPropertyNumber,
    ///ADC - Project Property Number
    ProjectPropertyNumber,
    ///ADD - Unit Property Number
    UnitPropertyNumber,
    ///ADE - Associated Property Number
    AssociatedPropertyNumber,
    ///ADF - Associated Number For Limited Common Element Parking
    AssociatedNumberForLimitedCommonElementParking,
    ///ADG - Associated Number For Unit Parking
    AssociatedNumberForUnitParking,
    ///ADH - Associated Number For Joined Unit not re-subdivided
    AssociatedNumberForJoinedUnitNotReSubdivided,
    ///ADI - Processor Identification Number
    ProcessorIdentificationNumber,
    ///ADJ - Occupation Classification Code
    OccupationClassificationCode,
    ///ADK - Employee Tax Filing Status Code
    EmployeeTaxFilingStatusCode,
    ///ADL - Insured Location Identifier
    InsuredLocation,
    ///ADM - Air Dimension Code
    AirDimensionCode,
    ///ADN - Self Insurance Identification Number
    SelfInsuranceIdentificationNumber,
    ///ADO - Self Insurer Organization Type
    SelfInsurerOrganizationType,
    ///ADP - Self Insurer Authorization Type Code
    SelfInsurerAuthorizationTypeCode,
    ///ADQ - County Business Registration Number
    CountyBusinessRegistrationNumber,
    ///ADR - Postal Template Identifier
    PostalTemplate,
    ///ADS - Reduced Earning Week Identifier
    ReducedEarningWeek,
    ///ADT - Full Denial Reason Identifier
    FullDenialReason,
    ///ADU - Federal Energy Regulatory Commission Certificate of Public Convenience
    FederalEnergyRegulatoryCommissionCertificateOfPublicConvenience,
    ///ADV - Suspension Identifier
    Suspension,
    ///ADW - Managed Care Organization Code
    ManagedCareOrganizationCode,
    ///ADX - Managed Care Organization Identification Number
    ManagedCareOrganizationIdentificationNumber,
    ///ADY - Public Utilities Commission Certificate of Public Convenience
    PublicUtilitiesCommissionCertificateOfPublicConvenience,
    ///ADZ - Retail Merchant's Certification Number
    RetailMerchantsCertificationNumber,
    ///AE - Authorization for Expense (AFE) Number
    CodeAE,
    ///AEA - Numero de Cedula de Identidad (CIN) Number
    CodeAEA,
    ///AEB - Company's Registry Office (CRO) Number
    CodeAEB,
    ///AEC - Government Registration Number
    GovernmentRegistrationNumber,
    ///AED - Judicial Number
    JudicialNumber,
    ///AEE - Numero de Identificacion Tributaria (NIT)
    CodeAEE,
    ///AEF - Passport Number
    PassportNumber,
    ///AEG - Patron Number
    PatronNumber,
    ///AEH - Registro Informacion Fiscal (RIF)
    CodeAEH,
    ///AEI - Registro Unico de Contribuyente (RUC)
    CodeAEI,
    ///AEJ - Superintendencia de Inversiones Extranjeras (SIEX) Number
    CodeAEJ,
    ///AEK - Tokyo Shoko Research Business Identifier
    TokyoShokoResearchBusiness,
    ///AEL - Registro Nacional de Contribuyente (RNC)
    CodeAEL,
    ///AEM - Distribution Center Number
    DistributionCenterNumber,
    ///AEN - Institute of Security and Future Market Development (ISFMD) Serial Number
    CodeAEN,
    ///AEO - Public Deed Number
    PublicDeedNumber,
    ///AEP - Stock Exchange Code
    StockExchangeCode,
    ///AEQ - Secretary of State Assigned Identification Number
    SecretaryOfStateAssignedIdentificationNumber,
    ///AER - Department Where Injury Occurred Identification
    DepartmentWhereInjuryOccurredIdentification,
    ///AES - Bureau of Labor and Statistics Schedule Identifier
    BureauOfLaborAndStatisticsSchedule,
    ///AET - State Charter Number
    StateCharterNumber,
    ///AEU - Employee/Non-Employee Classification Qualifier
    EmployeeNonEmployeeClassificationQualifier,
    ///AEV - Full Time/Part Time Employee Classification Qualifier
    FullTimePartTimeEmployeeClassificationQualifier,
    ///AEX - Premium Audit Priority Identifier
    PremiumAuditPriority,
    ///AEY - Premium Audit Purpose Identifier
    PremiumAuditPurpose,
    ///AEZ - Premium Audit Type Identifier
    PremiumAuditType,
    ///AF - Airlines Flight Identification Number
    AirlinesFlightIdentificationNumber,
    ///AFA - Split Premium Audit Change Identifier
    SplitPremiumAuditChange,
    ///AFB - Subline of Insurance
    SublineOfInsurance,
    ///AFC - Verification Source Code
    VerificationSourceCode,
    ///AFD - Underwriting Alert Reference Code
    UnderwritingAlertReferenceCode,
    ///AFE - Commercial/Private Passenger Vehicle Qualifier
    CommercialPrivatePassengerVehicleQualifier,
    ///AFF - Vehicle Business Use Qualifier
    VehicleBusinessUseQualifier,
    ///AFG - Vehicle Size Class Qualifier
    VehicleSizeClassQualifier,
    ///AFH - Vehicle Radius of Operation Qualifier
    VehicleRadiusOfOperationQualifier,
    ///AFI - Trailer Type Qualifier
    TrailerTypeQualifier,
    ///AFJ - State Sales Tax Identification Number
    StateSalesTaxIdentificationNumber,
    ///AFK - Card Issuer Transaction Code
    CardIssuerTransactionCode,
    ///AFL - Card Billing Type Code
    CardBillingTypeCode,
    ///AFM - Client Company Code
    ClientCompanyCode,
    ///AFN - Merchant Category Code (MCC)
    CodeAFN,
    ///AFO - Card Account Type Code
    CardAccountTypeCode,
    ///AFP - Card Account Status Code
    CardAccountStatusCode,
    ///AFQ - Card Account Reporting Level
    CardAccountReportingLevel,
    ///AFR - Card Account Reporting Identifier
    CardAccountReporting,
    ///AFS - American Osteopathic Association (AOA) Certification Number
    CodeAFS,
    ///AFT - Fee Schedule Identifier
    FeeSchedule,
    ///AFU - United States Standard Metropolitan Statistical Area (MSA) Code
    CodeAFU,
    ///AFV - State Controlled Substance License Number
    StateControlledSubstanceLicenseNumber,
    ///AFW - Point of Origination
    PointOfOrigination,
    ///AFX - Point of Destination
    PointOfDestination,
    ///AFY - Assessment Number
    AssessmentNumber,
    ///AFZ - Certificate Number
    CertificateNumber,
    ///AG - Agent's Shipment Number
    AgentsShipmentNumber,
    ///AGA - State or Province Assigned Business Registry Number
    StateOrProvinceAssignedBusinessRegistryNumber,
    ///AGB - Municipality Assigned Business Registry Number
    MunicipalityAssignedBusinessRegistryNumber,
    ///AGC - Clave Unica de Identificacion Tributaria (CUIT)
    CodeAGC,
    ///AGD - Registro Unico Tributario (RUT)
    CodeAGD,
    ///AGH - Lender Use
    LenderUse,
    ///AGI - Guarantor Use
    GuarantorUse,
    ///AGJ - School Use
    SchoolUse,
    ///AGK - Reservation System Code
    ReservationSystemCode,
    ///AGL - Order Origination Code
    OrderOriginationCode,
    ///AGM - Folio Number
    FolioNumber,
    ///AGN - Corporate Identification Code
    CorporateIdentificationCode,
    ///AGO - Cadastro Geral do Contribuinte (CGC)
    CodeAGO,
    ///AGP - Conjunction Travel Ticket
    ConjunctionTravelTicket,
    ///AGQ - List Tracking Identifier
    ListTracking,
    ///AH - Agreement Number
    AgreementNumber,
    ///AHC - Air Handling Code
    AirHandlingCode,
    ///AI - Associated Invoices
    AssociatedInvoices,
    ///AJ - Accounts Receivable Customer Account
    AccountsReceivableCustomerAccount,
    ///AK - Sending Company Audit Number (Automated Clearinghouse Transfers)
    CodeAK,
    ///AL - Accounting (Equipment) Location Number
    CodeAL,
    ///ALC - Agency Location Code
    AgencyLocationCode,
    ///ALG - Title Company Code Book Reference
    TitleCompanyCodeBookReference,
    ///ALH - Title Document Schedule
    TitleDocumentSchedule,
    ///ALI - Recording Number
    RecordingNumber,
    ///ALJ - Title Policy Number
    TitlePolicyNumber,
    ///ALR - Alien Registration Number
    AlienRegistrationNumber,
    ///ALS - Alternative List ID
    AlternativeListId,
    ///ALT - Alteration Number
    AlterationNumber,
    ///AM - Adjustment Memo (Charge Back)
    CodeAM,
    ///AN - Associated Purchase Orders
    AssociatedPurchaseOrders,
    ///AO - Appointment Number
    AppointmentNumber,
    ///AP - Accounts Receivable Number
    AccountsReceivableNumber,
    ///APC - Ambulatory Payment Classification
    AmbulatoryPaymentClassification,
    ///API - American Petroleum Institute (API) Deduction Code
    CodeAPI,
    ///AQ - Access Code
    AccessCode,
    ///AR - Arrival Code
    ArrivalCode,
    ///AS - Acceptable Source Supplier ID
    AcceptableSourceSupplierId,
    ///ASL - Atomic Safety and Licensing Board Panel (ASLBP) Number
    CodeASL,
    ///ASP - Animal Species
    AnimalSpecies,
    ///AST - Animal Strain
    AnimalStrain,
    ///AT - Appropriation Number
    AppropriationNumber,
    ///ATC - Maintenance Availability Type
    MaintenanceAvailabilityType,
    ///AU - Authorization to Meet Competition Number
    AuthorizationToMeetCompetitionNumber,
    ///AV - Health Insurance Rating Account Number
    HealthInsuranceRatingAccountNumber,
    ///AW - Air Waybill Number
    AirWaybillNumber,
    ///AX - Government Accounting Class Reference Number (ACRN)
    CodeAX,
    ///AY - Floor Plan Approval Number
    FloorPlanApprovalNumber,
    ///AZ - Health Insurance Policy Number
    HealthInsurancePolicyNumber,
    ///B1 - Lessee Bill Code Number
    LesseeBillCodeNumber,
    ///B2 - Axle Ratio
    AxleRatio,
    ///B3 - Preferred Provider Organization Number
    PreferredProviderOrganizationNumber,
    ///B4 - Bilateral Car Service Agreements
    BilateralCarServiceAgreements,
    ///B5 - Health Insurance Rating Suffix Code
    HealthInsuranceRatingSuffixCode,
    ///B6 - Life Insurance Billing Account Number
    LifeInsuranceBillingAccountNumber,
    ///B7 - Life Insurance Policy Number
    LifeInsurancePolicyNumber,
    ///B8 - Life Insurance Billing Suffix Code
    LifeInsuranceBillingSuffixCode,
    ///B9 - Retirement Plan Account Number
    RetirementPlanAccountNumber,
    ///BA - Retirement Plan Policy Number
    RetirementPlanPolicyNumber,
    ///BAA - Franchise Tax Account Number
    FranchiseTaxAccountNumber,
    ///BAB - Certificate of Incorporation Number
    CertificateOfIncorporationNumber,
    ///BAC - Beam Assembly Code
    BeamAssemblyCode,
    ///BAD - State Tax Identification Number
    StateTaxIdentificationNumber,
    ///BAE - Charter Number
    CharterNumber,
    ///BAF - Receipt Number
    ReceiptNumber,
    ///BAG - Withdrawal Account Number
    WithdrawalAccountNumber,
    ///BAH - Deposit Account Number
    DepositAccountNumber,
    ///BAI - Business Identification Number
    BusinessIdentificationNumber,
    ///BAJ - United States Postal Service (USPS) PLANET (PostaL AlphaNumEric coding Technique) Code
    CodeBAJ,
    ///BAK - Address Correction Service (ACS) Participation Code
    CodeBAK,
    ///BB - Authorization Number
    AuthorizationNumber,
    ///BC - Buyer's Contract Number
    BuyersContractNumber,
    ///BCI - Basic Contract Line Item Number
    BasicContractLineItemNumber,
    ///BCN - Birth Certificate Number
    BirthCertificateNumber,
    ///BCP - Border Crossing Permit Number
    BorderCrossingPermitNumber,
    ///BD - Bid Number
    BidNumber,
    ///BDG - Badge Number
    BadgeNumber,
    ///BDN - Build Directive Number
    BuildDirectiveNumber,
    ///BE - Business Activity
    BusinessActivity,
    ///BEN - Broker Entry Number
    BrokerEntryNumber,
    ///BF - Billing Center Identification
    BillingCenterIdentification,
    ///BG - Beginning Serial Number
    BeginningSerialNumber,
    ///BH - Lease Schedule Number - Blanket
    LeaseScheduleNumberBlanket,
    ///BI - Bonded Carrier Internal Revenue Service Identification Number
    BondedCarrierInternalRevenueServiceIdentificationNumber,
    ///BJ - Carrier's Customs Bond Number
    CarriersCustomsBondNumber,
    ///BK - Broker's Order Number
    BrokersOrderNumber,
    ///BKT - Bank Telegraphic Number
    BankTelegraphicNumber,
    ///BL - Government Bill of Lading
    GovernmentBillOfLading,
    ///BLT - Billing Type
    BillingType,
    ///BM - Bill of Lading Number
    BillOfLadingNumber,
    ///BMM - Begin Mile Marker
    BeginMileMarker,
    ///BN - Booking Number
    BookingNumber,
    ///BO - Bin Location Number
    BinLocationNumber,
    ///BOI - Binary Object Identifier
    BinaryObject,
    ///BP - Adjustment Control Number
    AdjustmentControlNumber,
    ///BQ - Health Maintenance Organization Code Number
    HealthMaintenanceOrganizationCodeNumber,
    ///BR - Broker or Sales Office Number
    BrokerOrSalesOfficeNumber,
    ///BS - Split Booking Number
    SplitBookingNumber,
    ///BT - Batch Number
    BatchNumber,
    ///BU - Buyer's Approval Mark
    BuyersApprovalMark,
    ///BV - Purchase Order Line Item Identifier (Buyer)
    CodeBV,
    ///BW - Blended With Batch Number
    BlendedWithBatchNumber,
    ///BX - Buyer's Shipment Mark Number
    BuyersShipmentMarkNumber,
    ///BY - Repair Category Number
    RepairCategoryNumber,
    ///BZ - Complaint Code
    ComplaintCode,
    ///C0 - Canadian Social Insurance Number
    CanadianSocialInsuranceNumber,
    ///C1 - Customer material specification number
    CustomerMaterialSpecificationNumber,
    ///C2 - Customer process specification number
    CustomerProcessSpecificationNumber,
    ///C3 - Customer specification number
    CustomerSpecificationNumber,
    ///C4 - Change Number
    ChangeNumber,
    ///C5 - Customer Tracking Number For Loaned Materials
    CustomerTrackingNumberForLoanedMaterials,
    ///C6 - Carnet Number
    CarnetNumber,
    ///C7 - Contract Line Item Number
    ContractLineItemNumber,
    ///C8 - Corrected Contract Number
    CorrectedContractNumber,
    ///C9 - Previous Credit/Debit Adjustment Number
    PreviousCreditDebitAdjustmentNumber,
    ///CA - Cost Allocation Reference
    CostAllocationReference,
    ///CAA - Accident History Identifier
    AccidentHistory,
    ///CAB - Chemical Identifier
    Chemical,
    ///CAC - Discharge Point Identification
    DischargePointIdentification,
    ///CAD - Emission Unit Identification Number
    EmissionUnitIdentificationNumber,
    ///CAE - Facility Federal Identification Number
    FacilityFederalIdentificationNumber,
    ///CAF - Latitude Expressed in Decimal Degrees
    LatitudeExpressedInDecimalDegrees,
    ///CAG - Longitude Expressed in Decimal Degrees
    LongitudeExpressedInDecimalDegrees,
    ///CAH - Office of Regulatory Information Systems (ORIS) Code
    CodeCAH,
    ///CAI - Process Identifier
    Process,
    ///CAJ - Stack Identification Number
    StackIdentificationNumber,
    ///CAK - Facility State Identification Number
    FacilityStateIdentificationNumber,
    ///CAL - U.S. Environmental Protection Agency (EPA) Hazardous Waste Code
    CodeCAL,
    /**CAM - U.S. Environmental Protection Agency (EPA)
Identification Number*/
    CodeCAM,
    ///CAT - Category Identifier
    Category,
    ///CB - Combined Shipment
    CombinedShipment,
    ///CBG - Census Block Group
    CensusBlockGroup,
    ///CC - Contract Co-op Number
    ContractCoOpNumber,
    ///CD - Credit Note Number
    CreditNoteNumber,
    ///CDN - Citizenship Document Number
    CitizenshipDocumentNumber,
    ///CDT - Contracting District Type Code
    ContractingDistrictTypeCode,
    ///CE - Class of Contract Code
    ClassOfContractCode,
    ///CF - Fleet Reference Number
    FleetReferenceNumber,
    ///CFR - Federal Regulation
    FederalRegulation,
    ///CG - Consignee's Order Number
    ConsigneesOrderNumber,
    ///CH - Customer Catalog Number
    CustomerCatalogNumber,
    ///CHR - Chromatograph Identifier
    Chromatograph,
    ///CI - Unique Consignment Identifier
    UniqueConsignment,
    ///CID - Campus Identification Number
    CampusIdentificationNumber,
    ///CIR - Circuit Number
    CircuitNumber,
    ///CIT - Citation
    Citation,
    ///CJ - Clause Number
    ClauseNumber,
    ///CK - Check Number
    CheckNumber,
    ///CL - Seller's Credit Memo
    SellersCreditMemo,
    ///CLI - Coverage List ID
    CoverageListId,
    ///CM - Buyer's Credit Memo
    BuyersCreditMemo,
    ///CMN - Continuous Move Number
    ContinuousMoveNumber,
    ///CMP - Customer Maintenance Period Sequence Number
    CustomerMaintenancePeriodSequenceNumber,
    ///CMT - Component
    Component,
    ///CN - Carrier's Reference Number (PRO/Invoice)
    CodeCN,
    ///CNA - Assembly Control Number
    AssemblyControlNumber,
    ///CNO - Commitment Number
    CommitmentNumber,
    ///CNS - Canadian National Student Number
    CanadianNationalStudentNumber,
    ///CO - Customer Order Number
    CustomerOrderNumber,
    ///COL - Collocation Indicator
    CollocationIndicator,
    ///COT - Certificate of Transportation
    CertificateOfTransportation,
    ///CP - Condition of Purchase Document Number
    ConditionOfPurchaseDocumentNumber,
    ///CPA - Canadian Province Operating Authority Number
    CanadianProvinceOperatingAuthorityNumber,
    ///CPD - Discrepant Container Packaging Number
    DiscrepantContainerPackagingNumber,
    ///CPR - Required Container Packaging Number
    RequiredContainerPackagingNumber,
    ///CPT - Current Procedural Terminology Code
    CurrentProceduralTerminologyCode,
    ///CQ - Customshouse Broker License Number
    CustomshouseBrokerLicenseNumber,
    ///CR - Customer Reference Number
    CustomerReferenceNumber,
    ///CRN - Casualty Report Number
    CasualtyReportNumber,
    ///CRS - Casualty Report Serial Number
    CasualtyReportSerialNumber,
    ///CS - Condition of Sale Document Number
    ConditionOfSaleDocumentNumber,
    ///CSC - CS54 Key Train Indicator Code
    Cs54KeyTrainIndicatorCode,
    ///CSG - CS54 Key Train Indicator Group Name
    Cs54KeyTrainIndicatorGroupName,
    ///CST - Census State Code
    CensusStateCode,
    ///CT - Contract Number
    ContractNumber,
    ///CTS - Census Tract Suffix
    CensusTractSuffix,
    ///CU - Clear Text Clause
    ClearTextClause,
    ///CUB - U.S. Customs Service (USCS) Bill of Lading Number
    CodeCUB,
    ///CV - Coil Number
    CoilNumber,
    ///CVS - Commercial Vehicle Safety Assurance Number
    CommercialVehicleSafetyAssuranceNumber,
    ///CW - Canadian Wheat Board Permit Number
    CanadianWheatBoardPermitNumber,
    ///CX - Consignment Classification ID
    ConsignmentClassificationId,
    ///CY - Commercial Registration Number
    CommercialRegistrationNumber,
    ///CYC - Periodicity Code
    PeriodicityCode,
    ///CZ - Contract Rider Number (Used in conjunction with contract number)
    CodeCZ,
    ///D0 - Data Reliability Code
    DataReliabilityCode,
    ///D1 - Drug Enforcement Administration Order Blank Number
    DrugEnforcementAdministrationOrderBlankNumber,
    ///D2 - Supplier Document Identification Number
    SupplierDocumentIdentificationNumber,
    ///D3 - National Council for Prescription Drug Programs Pharmacy Number
    NationalCouncilForPrescriptionDrugProgramsPharmacyNumber,
    ///D4 - Cut Number
    CutNumber,
    ///D5 - Dye Lot Number
    DyeLotNumber,
    ///D6 - Duplicate Bill Number
    DuplicateBillNumber,
    ///D7 - Coverage Code
    CoverageCode,
    ///D8 - Loss Report Number
    LossReportNumber,
    ///D9 - Claim Number
    ClaimNumber,
    ///DA - Domicile Branch Number
    DomicileBranchNumber,
    ///DAI - District Assigned ID
    DistrictAssignedId,
    ///DAN - Delivery Appointment Number
    DeliveryAppointmentNumber,
    ///DB - Buyer's Debit Memo
    BuyersDebitMemo,
    ///DC - Dealer purchase order number
    DealerPurchaseOrderNumber,
    ///DD - Document Identification Code
    DocumentIdentificationCode,
    ///DE - Depositor Number
    DepositorNumber,
    ///DF - Defense Federal Acquisition Regulations (DFAR)
    CodeDF,
    ///DG - Drawing Number
    DrawingNumber,
    ///DH - Drug Enforcement Administration Number
    DrugEnforcementAdministrationNumber,
    ///DHH - Department of Health and Human Services Acquisition Regulation (HHSAR)
    CodeDHH,
    ///DI - Distributor Invoice Number
    DistributorInvoiceNumber,
    ///DIS - District Number
    DistrictNumber,
    ///DJ - Delivery Ticket Number
    DeliveryTicketNumber,
    ///DK - Dock Number
    DockNumber,
    ///DL - Seller's Debit Memo
    SellersDebitMemo,
    ///DM - Associated Product Number
    AssociatedProductNumber,
    ///DN - Draft Number
    DraftNumber,
    ///DNR - Deposit Number
    DepositNumber,
    ///DNS - D-U-N-S+4, D-U-N-S Number with Four Character Suffix
    CodeDNS,
    ///DO - Delivery Order Number
    DeliveryOrderNumber,
    ///DOA - Department of Agriculture Acquisition Regulation (AGAR)
    CodeDOA,
    ///DOC - Department of Commerce Acquisition Regulation (CAR)
    CodeDOC,
    ///DOE - Department of Energy Acquisition Regulation (DEAR)
    CodeDOE,
    ///DOI - Department of Interior Acquisition Regulation (DIAR)
    CodeDOI,
    ///DOJ - Department of Justice Acquisition Regulation (JAR)
    CodeDOJ,
    ///DOL - Department of Labor Acquisition Regulation (DOLAR)
    CodeDOL,
    ///DON - Density Order Number
    DensityOrderNumber,
    ///DOS - Department of State Acquisition Regulation (DOSAR)
    CodeDOS,
    ///DOT - Department of Transportation Acquisition Regulation (TAR)
    CodeDOT,
    ///DP - Department Number
    DepartmentNumber,
    ///DQ - Delivery Quote Number
    DeliveryQuoteNumber,
    ///DR - Dock Receipt Number
    DockReceiptNumber,
    ///DRN - Drainhole Number
    DrainholeNumber,
    ///DS - Defense Priorities Allocation System (DPAS) Priority Rating
    CodeDS,
    ///DSC - Departure from Specification Class Code
    DepartureFromSpecificationClassCode,
    ///DSI - Departure from Specification Number
    DepartureFromSpecificationNumber,
    ///DST - Departure from Specification Type Code
    DepartureFromSpecificationTypeCode,
    ///DT - Downstream Shipper Contract Number
    DownstreamShipperContractNumber,
    ///DTS - Department of the Treasury Acquisition/Procurement Regulation (TAPR)
    CodeDTS,
    ///DU - Dependents Information
    Dependents,
    ///DUN - D-U-N-S Number Dun & Bradstreet
    CodeDUN,
    ///DV - Diversion Authority Number
    DiversionAuthorityNumber,
    ///DW - Deposit Sequence Number
    DepositSequenceNumber,
    ///DX - Department/Agency Number
    DepartmentAgencyNumber,
    ///DY - Department of Defense Transportation Service Code Number (Household Goods)
    CodeDY,
    ///DZ - Certified Registered Nurse Anesthetist (CRNA) Provider Identification Number
    CodeDZ,
    ///E00 - Course Section Number
    CourseSectionNumber,
    ///E1 - Emergency Order Number
    EmergencyOrderNumber,
    ///E01 - Non-Teaching Credential Field Codes
    NonTeachingCredentialFieldCodes,
    ///E2 - Part Causing Repair Number
    PartCausingRepairNumber,
    ///E02 - Classification of Instructional Programs (CIP) Codes
    CodeE02,
    ///E3 - Expansion on Effect of Change Number
    ExpansionOnEffectOfChangeNumber,
    ///E4 - Charge Card Number
    ChargeCardNumber,
    ///E5 - Claimant's Claim Number
    ClaimantsClaimNumber,
    ///E6 - Backout Procedure Code
    BackoutProcedureCode,
    ///E7 - Service Bulletin Number
    ServiceBulletinNumber,
    ///E8 - Service Contract (Coverage) Number
    CodeE8,
    ///E9 - Attachment Code
    AttachmentCode,
    ///EA - Medical Record Identification Number
    MedicalRecordIdentificationNumber,
    ///EB - Embargo Permit Number
    EmbargoPermitNumber,
    ///EC - Circular
    Circular,
    ///ECA - Fund Identifier
    Fund,
    ///ECB - Ballot Identifier
    Ballot,
    ///ECC - Legislative Identification Number
    LegislativeIdentificationNumber,
    ///ECD - Lobbied Activity Identifier
    LobbiedActivity,
    ///ECE - Petition Number
    PetitionNumber,
    ///ECF - Related Form Number
    RelatedFormNumber,
    ///ECJ - Carrier's Bond Number Covering Instruments of International Traffic (IIT)
    CodeECJ,
    ///ED - Export Declaration
    ExportDeclaration,
    ///EDA - Department of Education Acquisition Regulation (EDAR)
    CodeEDA,
    ///EE - Election District
    ElectionDistrict,
    ///EF - Electronic Funds Transfer ID Number
    ElectronicFundsTransferIdNumber,
    ///EG - Ending Serial Number
    EndingSerialNumber,
    ///EH - Financial Classification Code
    FinancialClassificationCode,
    ///EI - Employer's Identification Number
    EmployersIdentificationNumber,
    ///EII - Importer's Bond Number Covering Instruments of International Traffic (IIT)
    CodeEII,
    ///EJ - Patient Account Number
    PatientAccountNumber,
    ///EK - Healthcare Manpower Shortage Area (HMSA) Facility Identification Number
    CodeEK,
    ///EL - Electronic device pin number
    ElectronicDevicePinNumber,
    ///EM - Electronic Payment Reference Number
    ElectronicPaymentReferenceNumber,
    ///EMM - End Mile Marker
    EndMileMarker,
    ///EN - Embargo Number
    EmbargoNumber,
    ///END - Endorsement Number
    EndorsementNumber,
    ///EO - Submitter Identification Number
    SubmitterIdentificationNumber,
    ///EP - Export Permit Number
    ExportPermitNumber,
    ///EPA - Environmental Protection Agency Acquisition Regulation (EPAAR)
    CodeEPA,
    ///EPB - Environmental Protection Agency Transporter Identification Number
    EnvironmentalProtectionAgencyTransporterIdentificationNumber,
    ///EPC - Employer Payroll Code Lists
    EmployerPayrollCodeLists,
    ///EQ - Equipment Number
    EquipmentNumber,
    ///ER - Container or Equipment Receipt Number
    ContainerOrEquipmentReceiptNumber,
    ///ES - Employer's Social Security Number
    EmployersSocialSecurityNumber,
    ///ESN - Estimate Sequence Number
    EstimateSequenceNumber,
    ///ET - Excess Transportation
    ExcessTransportation,
    ///EU - End User's Purchase Order Number
    EndUsersPurchaseOrderNumber,
    ///EV - Receiver Identification Number
    ReceiverIdentificationNumber,
    ///EVI - Event Identification
    EventIdentification,
    ///EW - Mammography Certification Number
    MammographyCertificationNumber,
    ///EX - Estimate Number
    EstimateNumber,
    ///EXP - Exposure State Code
    ExposureStateCode,
    ///EY - Receiver Sub-identification Number
    ReceiverSubIdentificationNumber,
    ///EZ - Electronic Data Interchange Agreement Number
    ElectronicDataInterchangeAgreementNumber,
    ///F1 - Version Code - National
    VersionCodeNational,
    ///F2 - Version Code - Local
    VersionCodeLocal,
    ///F3 - Submission Number
    SubmissionNumber,
    ///F4 - Facility Certification Number
    FacilityCertificationNumber,
    ///F5 - Medicare Version Code
    MedicareVersionCode,
    ///F6 - Health Insurance Claim (HIC) Number
    CodeF6,
    ///F7 - New Health Insurance Claim (HIC) Number
    CodeF7,
    ///F8 - Original Reference Number
    OriginalReferenceNumber,
    ///F9 - Freight Payor Reference Number
    FreightPayorReferenceNumber,
    ///FA - Federal Acquisition Regulations (FAR)
    CodeFA,
    ///FAN - Fannie Mae Seller Servicer Number
    FannieMaeSellerServicerNumber,
    ///FB - File Transfer Form Number
    FileTransferFormNumber,
    ///FC - Filer Code Issued by Customs
    FilerCodeIssuedByCustoms,
    ///FCN - Assigned Contract Number
    AssignedContractNumber,
    ///FD - Filer Code Issued by Bureau of Census
    FilerCodeIssuedByBureauOfCensus,
    ///FE - Failure mechanism number
    FailureMechanismNumber,
    ///FEN - Foreign Entry Number
    ForeignEntryNumber,
    ///FF - Film Number
    FilmNumber,
    ///FG - Fund Identification Number
    FundIdentificationNumber,
    ///FH - Clinic Number
    ClinicNumber,
    ///FHC - Federal Housing Administration Computerized Homes Underwriting Management System (CHUMS) Identification Number
    CodeFHC,
    ///FHO - Federal Housing Administration Originator Identification
    FederalHousingAdministrationOriginatorIdentification,
    ///FI - File Identifier
    File,
    ///FJ - Line Item Control Number
    LineItemControlNumber,
    ///FK - Finish Lot Number
    FinishLotNumber,
    ///FL - Fine Line Classification
    FineLineClassification,
    ///FLZ - Flood Zone
    FloodZone,
    ///FM - Federal Maritime Commission (FMC) Forwarders Number
    CodeFM,
    ///FMG - Educational Commission for Foreign Medical Graduates (ECFMG) Certification Number
    CodeFMG,
    ///FMP - Facility Measurement Point Number
    FacilityMeasurementPointNumber,
    ///FN - Forwarder's/Agent's Reference Number
    ForwardersAgentsReferenceNumber,
    ///FND - Finder Number
    FinderNumber,
    ///FO - Drug Formulary Number
    DrugFormularyNumber,
    ///FP - Forestry Permit Number
    ForestryPermitNumber,
    ///FQ - Form Number
    FormNumber,
    ///FR - Freight Bill Number
    FreightBillNumber,
    ///FRN - Freddie Mac Seller Servicer Number
    FreddieMacSellerServicerNumber,
    ///FS - Final Sequence Number
    FinalSequenceNumber,
    ///FSC - Fund Source Code
    FundSourceCode,
    ///FSN - Assigned Sequence Number
    AssignedSequenceNumber,
    ///FT - Foreign Trade Zone
    ForeignTradeZone,
    ///FTN - Premarket Notification Number
    PremarketNotificationNumber,
    ///FTP - File Transfer Protocol (FTP) Locator
    CodeFTP,
    ///FTZ - Foreign Trade Zone (FTZ) Admission Number
    CodeFTZ,
    ///FU - Fund Code
    FundCode,
    ///FV - Health Maintenance Organization (HMO) Reference Number
    CodeFV,
    ///FW - State License Identification Number
    StateLicenseIdentificationNumber,
    ///FWC - Final Work Candidate Number
    FinalWorkCandidateNumber,
    ///FX - Failure Analysis Report Number
    FailureAnalysisReportNumber,
    ///FY - Claim Office Number
    ClaimOfficeNumber,
    ///FZ - Processor's Invoice Number
    ProcessorsInvoiceNumber,
    ///G1 - Prior Authorization Number
    PriorAuthorizationNumber,
    ///G2 - Provider Commercial Number
    ProviderCommercialNumber,
    ///G3 - Predetermination of Benefits Identification Number
    PredeterminationOfBenefitsIdentificationNumber,
    ///G4 - Peer Review Organization (PRO) Approval Number
    CodeG4,
    ///G5 - Provider Site Number
    ProviderSiteNumber,
    ///G6 - Payer Assigned Resubmission Reference Number
    PayerAssignedResubmissionReferenceNumber,
    ///G7 - Resubmission Reason Code
    ResubmissionReasonCode,
    ///G8 - Resubmission Number
    ResubmissionNumber,
    ///G9 - Secondary Employee Identification Number
    SecondaryEmployeeIdentificationNumber,
    ///GA - Government Advance Progress
    GovernmentAdvanceProgress,
    ///GB - Grain Block Number
    GrainBlockNumber,
    ///GC - Government Contract Number
    GovernmentContractNumber,
    ///GD - Return Goods Bill of Lading Number
    ReturnGoodsBillOfLadingNumber,
    ///GE - Geographic Number
    GeographicNumber,
    ///GF - Specialty License Number
    SpecialtyLicenseNumber,
    ///GG - Gauge Ticket Number
    GaugeTicketNumber,
    ///GH - Identification Card Serial Number
    IdentificationCardSerialNumber,
    ///GI - Secondary Provider Number
    SecondaryProviderNumber,
    ///GJ - Cornbore Certification Number
    CornboreCertificationNumber,
    ///GK - Third Party Reference Number
    ThirdPartyReferenceNumber,
    ///GL - Geographic Destination Zone Number
    GeographicDestinationZoneNumber,
    ///GM - Loan Acquisition Number
    LoanAcquisitionNumber,
    ///GN - Folder Number
    FolderNumber,
    ///GO - Exhibit Identifier
    Exhibit,
    ///GP - Government Priority Number
    GovernmentPriorityNumber,
    ///GQ - Internal Purchase Order Release Number
    InternalPurchaseOrderReleaseNumber,
    ///GR - Grain Order Reference Number
    GrainOrderReferenceNumber,
    ///GS - General Services Administration Regulations (GSAR)
    CodeGS,
    ///GT - Goods and Service Tax Registration Number
    GoodsAndServiceTaxRegistrationNumber,
    ///GU - Internal Purchase Order Item Number
    InternalPurchaseOrderItemNumber,
    ///GV - Third Party Purchase Order Number
    ThirdPartyPurchaseOrderNumber,
    ///GW - Third Party Purchase Order Release Number
    ThirdPartyPurchaseOrderReleaseNumber,
    ///GWS - Group Work Candidate Sequence Number
    GroupWorkCandidateSequenceNumber,
    ///GX - Third Party Purchase Order Item Number
    ThirdPartyPurchaseOrderItemNumber,
    ///GY - Empty Repositioning Number
    EmptyRepositioningNumber,
    ///GZ - General Ledger Account
    GeneralLedgerAccount,
    ///H1 - High Fabrication Authorization Number
    HighFabricationAuthorizationNumber,
    ///H2 - High Raw Material Authorization Number
    HighRawMaterialAuthorizationNumber,
    ///H3 - Gravity Source Meter Number
    GravitySourceMeterNumber,
    ///H5 - Special Clause
    SpecialClause,
    ///H6 - Quality Clause
    QualityClause,
    ///H7 - Standard Clause
    StandardClause,
    ///H8 - Home Mortgage Disclosure Act (HMDA) Census Tract
    CodeH8,
    ///H9 - Payment History Reference Number
    PaymentHistoryReferenceNumber,
    ///HA - Competent Authority
    CompetentAuthority,
    ///HB - Bill & Hold Invoice Number
    CodeHB,
    ///HC - Heat Code
    HeatCode,
    ///HD - Department of Transportation Hazardous Number
    DepartmentOfTransportationHazardousNumber,
    ///HE - Hazardous Exemption Number
    HazardousExemptionNumber,
    ///HF - Engineering Data List
    EngineeringDataList,
    ///HG - Civil Action Number
    CivilActionNumber,
    ///HH - Fiscal Code
    FiscalCode,
    ///HHT - Type of Household Goods Code
    TypeOfHouseholdGoodsCode,
    ///HI - Health Industry Number (HIN)
    CodeHI,
    ///HJ - Identity Card Number
    IdentityCardNumber,
    ///HK - Judgment Number
    JudgmentNumber,
    ///HL - SIREN Number
    SirenNumber,
    ///HM - SIRET Number
    SiretNumber,
    ///HMB - Home Mortgage Disclosure Act Block Number Area
    HomeMortgageDisclosureActBlockNumberArea,
    ///HN - Hazardous Certification Number
    HazardousCertificationNumber,
    ///HO - Shipper's Hazardous Number
    ShippersHazardousNumber,
    ///HP - Pack & Hold Invoice Number
    CodeHP,
    ///HPI - Centers for Medicare and Medicaid Services National Provider Identifier
    CentersForMedicareAndMedicaidServicesNationalProvider,
    ///HQ - Reinsurance Reference
    ReinsuranceReference,
    ///HR - Horsepower
    Horsepower,
    ///HS - Harmonized Code System (Canada)
    CodeHS,
    ///HT - Code of Federal Regulations
    CodeOfFederalRegulations,
    ///HU - Type of Escrow Number
    TypeOfEscrowNumber,
    ///HUD - Department of Housing and Urban Development Acquisition Regulation (HUDAR)
    CodeHUD,
    ///HV - Escrow File Number
    EscrowFileNumber,
    ///HW - High/Wide File Number
    HighWideFileNumber,
    ///HX - Auto Loss Item Number
    AutoLossItemNumber,
    ///HY - Property Loss Item Number
    PropertyLossItemNumber,
    ///HZ - Tax Agency Number (MERS [Mortgage Electronic Registration System] Federal Information Processing Standards [FIPS] Based Number)
    CodeHZ,
    ///I1 - Owning Bureau Identification Number
    OwningBureauIdentificationNumber,
    ///I2 - Interstate Commerce Commission (ICC) Account Number
    CodeI2,
    ///I3 - Non-American Identification Number
    NonAmericanIdentificationNumber,
    ///I4 - Credit Counseling Identification Number
    CreditCounselingIdentificationNumber,
    ///I5 - Invoice Identification
    InvoiceIdentification,
    ///I7 - Credit Report Number
    CreditReportNumber,
    ///I9 - Pollutant
    Pollutant,
    ///IA - Internal Vendor Number
    InternalVendorNumber,
    ///IB - In Bond Number
    InBondNumber,
    ///IC - Inbound-to Party
    InboundToParty,
    ///ICD - ICD-9-CM (International Classification of Diseases)
    CodeICD,
    ///ID - Insurance Certificate Number
    InsuranceCertificateNumber,
    ///IE - Interchange Agreement Number
    InterchangeAgreementNumber,
    ///IF - Issue Number
    IssueNumber,
    ///IFC - Initial Failure Claim
    InitialFailureClaim,
    ///IFT - International Fuel Tax Agreement Account Number
    InternationalFuelTaxAgreementAccountNumber,
    ///IG - Insurance Policy Number
    InsurancePolicyNumber,
    ///IH - Initial Dealer Claim Number
    InitialDealerClaimNumber,
    ///II - Initial Sample Inspection Report Number
    InitialSampleInspectionReportNumber,
    ///IID - Image Identifier
    Image,
    ///IJ - Standard Industry Classification (SIC) Code
    CodeIJ,
    ///IK - Invoice Number
    InvoiceNumber,
    ///IL - Internal Order Number
    InternalOrderNumber,
    ///IM - Intergovernmental Maritime Organization (IMO) Number
    CodeIM,
    ///IMP - Integrated Master Plan (IMP)
    CodeIMP,
    ///IMS - Integrated Master Schedule (IMS)
    CodeIMS,
    ///IN - Consignee's Invoice Number
    ConsigneesInvoiceNumber,
    ///IND - Investigatorial New Drug Number
    InvestigatorialNewDrugNumber,
    ///IO - Inbound-to or Outbound-from Party
    InboundToOrOutboundFromParty,
    ///IP - Inspection Report Number
    InspectionReportNumber,
    ///IQ - End Item
    EndItem,
    ///IR - Intra Plant Routing
    IntraPlantRouting,
    ///IRN - Importer's Reference Number to Letter of Credit
    ImportersReferenceNumberToLetterOfCredit,
    ///IRP - International Registration Plan Account Number
    InternationalRegistrationPlanAccountNumber,
    ///IS - Invoice Number Suffix
    InvoiceNumberSuffix,
    ///ISC - International Standard Industrial Classification (ISIC) Dominion of Canada Code (DCC)
    CodeISC,
    ///ISN - International Registration Plan Sticker Number
    InternationalRegistrationPlanStickerNumber,
    ///ISS - Inspection and Survey Sequence Number
    InspectionAndSurveySequenceNumber,
    ///IT - Internal Customer Number
    InternalCustomerNumber,
    ///ITI - Initial Trouble Indication
    InitialTroubleIndication,
    ///IU - Barge Permit Number
    BargePermitNumber,
    ///IV - Seller's Invoice Number
    SellersInvoiceNumber,
    ///IW - Part Interchangeability
    PartInterchangeability,
    ///IX - Item Number
    ItemNumber,
    ///IZ - Insured Parcel Post Number
    InsuredParcelPostNumber,
    ///J0 - Proceeding
    Proceeding,
    ///J1 - Creditor
    Creditor,
    ///J2 - Attorney
    Attorney,
    ///J3 - Judge
    Judge,
    ///J4 - Trustee
    Trustee,
    ///J5 - Originating Case
    OriginatingCase,
    ///J6 - Adversary Case
    AdversaryCase,
    ///J7 - Lead Case
    LeadCase,
    ///J8 - Jointly Administered Case
    JointlyAdministeredCase,
    ///J9 - Substantively Consolidated Case
    SubstantivelyConsolidatedCase,
    ///JA - Beginning Job Sequence Number
    BeginningJobSequenceNumber,
    ///JB - Job (Project) Number
    CodeJB,
    ///JC - Review
    Review,
    ///JCS - Joint Credit Specification Number
    JointCreditSpecificationNumber,
    ///JD - User Identification
    UserIdentification,
    ///JE - Ending Job Sequence Number
    EndingJobSequenceNumber,
    ///JF - Automated Underwriting Reference Number
    AutomatedUnderwritingReferenceNumber,
    ///JH - Tag
    Tag,
    ///JI - Multiple Listing Service Area
    MultipleListingServiceArea,
    ///JK - Multiple Listing Service Sub-area
    MultipleListingServiceSubArea,
    ///JL - Packet
    Packet,
    ///JM - Multiple Listing Service Map X Coordinate
    MultipleListingServiceMapXCoordinate,
    ///JN - Multiple Listing Service Map Y Coordinate
    MultipleListingServiceMapYCoordinate,
    ///JO - Multiple Listing Number
    MultipleListingNumber,
    ///JP - Multiple Listing Service Book Type
    MultipleListingServiceBookType,
    ///JQ - Elevation
    Elevation,
    ///JR - Property Component Location
    PropertyComponentLocation,
    ///JS - Job Sequence Number
    JobSequenceNumber,
    ///JT - Prior Tax Identification Number (TIN)
    CodeJT,
    ///JU - Prior Phone Number
    PriorPhoneNumber,
    ///JV - Prior Health Industry Number
    PriorHealthIndustryNumber,
    ///JW - Prior Universal Provider Identification Number (UPIN)
    CodeJW,
    ///JX - Prior Postal Zip Code
    PriorPostalZipCode,
    ///JY - Origin of Shipment Harmonized-Based Code
    OriginOfShipmentHarmonizedBasedCode,
    ///JZ - Governing Class Code
    GoverningClassCode,
    ///K0 - Approval Code
    ApprovalCode,
    ///K1 - Foreign Military Sales Notice Number
    ForeignMilitarySalesNoticeNumber,
    ///K2 - Certified Mail Number
    CertifiedMailNumber,
    ///K3 - Registered Mail Number
    RegisteredMailNumber,
    ///K4 - Criticality Designator
    CriticalityDesignator,
    ///K5 - Task Order
    TaskOrder,
    ///K6 - Purchase Description
    PurchaseDescription,
    ///K7 - Paragraph Number
    ParagraphNumber,
    ///K8 - Project Paragraph Number
    ProjectParagraphNumber,
    ///K9 - Inquiry Request Number
    InquiryRequestNumber,
    ///KA - Distribution List
    DistributionList,
    ///KAS - Associated Contract Identifier
    AssociatedContract,
    ///KB - Beginning Kanban Serial Number
    BeginningKanbanSerialNumber,
    ///KC - Exhibit Distribution List
    ExhibitDistributionList,
    ///KCS - Confirmation Service Contract Identifier
    ConfirmationServiceContract,
    ///KD - Special Instructions Number
    SpecialInstructionsNumber,
    ///KE - Ending Kanban Serial Number
    EndingKanbanSerialNumber,
    ///KG - Foreclosing Status
    ForeclosingStatus,
    ///KH - Type of Law Suit
    TypeOfLawSuit,
    ///KI - Type of Outstanding Judgment
    TypeOfOutstandingJudgment,
    ///KII - Confirmation Intraday Identifier
    ConfirmationIntraday,
    ///KJ - Tax Lien Jurisdiction
    TaxLienJurisdiction,
    ///KK - Delivery Reference
    DeliveryReference,
    ///KL - Contract Reference
    ContractReference,
    ///KM - Rental Account Number
    RentalAccountNumber,
    ///KN - Census Automated Files ID
    CensusAutomatedFilesId,
    ///KO - Customs Drawback Entry Number
    CustomsDrawbackEntryNumber,
    ///KP - Health Certificate Number
    HealthCertificateNumber,
    ///KQ - Procuring Agency
    ProcuringAgency,
    ///KR - Response to a Request for Quotation Reference
    ResponseToARequestForQuotationReference,
    ///KRL - Releaser Contract Identifier
    ReleaserContract,
    ///KRP - Replacement Shipper Contract Identifier
    ReplacementShipperContract,
    ///KS - Solicitation
    Solicitation,
    ///KSR - Service Requester Contract Identifier
    ServiceRequesterContract,
    ///KT - Request for Quotation Reference
    RequestForQuotationReference,
    ///KU - Office Symbol
    OfficeSymbol,
    ///KV - Distribution Statement Code
    DistributionStatementCode,
    ///KW - Certification
    Certification,
    ///KX - Representation
    Representation,
    ///KY - Site Specific Procedures, Terms, and Conditions
    CodeKY,
    ///KZ - Master Solicitation Procedures, Terms, and Conditions
    CodeKZ,
    ///L0 - Collision Industry Electronic Commerce Association (CIECA)
    CodeL0,
    ///L1 - Letters or Notes
    LettersOrNotes,
    ///L2 - Location on Product Code
    LocationOnProductCode,
    ///L3 - Labor Operation Number
    LaborOperationNumber,
    ///L4 - Proposal Paragraph Number
    ProposalParagraphNumber,
    ///L5 - Subexhibit Line Item Number
    SubexhibitLineItemNumber,
    ///L6 - Subcontract Line Item Number
    SubcontractLineItemNumber,
    ///L7 - Customer's Release Number
    CustomersReleaseNumber,
    ///L8 - Consignee's Release Number
    ConsigneesReleaseNumber,
    ///L9 - Customer's Part Number
    CustomersPartNumber,
    ///LA - Shipping Label Serial Number
    ShippingLabelSerialNumber,
    ///LAA - Lottery Authority Activation Number
    LotteryAuthorityActivationNumber,
    ///LAN - Lane Number
    LaneNumber,
    ///LB - Lockbox
    Lockbox,
    ///LC - Lease Number
    LeaseNumber,
    ///LD - Loan Number
    LoanNumber,
    ///LE - Lender Entity Number
    LenderEntityNumber,
    ///LEN - Location Exception Order Number
    LocationExceptionOrderNumber,
    ///LF - Assembly Line Feed Location
    AssemblyLineFeedLocation,
    ///LG - Lease Schedule Number
    LeaseScheduleNumber,
    ///LH - Longitude Expressed in Seconds
    LongitudeExpressedInSeconds,
    ///LI - Line Item Identifier (Seller's)
    CodeLI,
    ///LIC - Health Industry Business Communications Council (HIBCC) Labeler Identification Code (LIC)
    CodeLIC,
    ///LJ - Local Jurisdiction
    LocalJurisdiction,
    ///LK - Longitude expressed in Degrees, Minutes and Seconds
    CodeLK,
    ///LL - Latitude Expressed in Seconds
    LatitudeExpressedInSeconds,
    ///LM - Product Period for which Labor Costs are Firm
    ProductPeriodForWhichLaborCostsAreFirm,
    ///LMI - Local Media Identifier
    LocalMedia,
    ///LN - Non Pickup Limited Tariff Number
    NonPickupLimitedTariffNumber,
    ///LO - Load Planning Number
    LoadPlanningNumber,
    ///LOI - Logical Observation Identifier Names and Codes (LOINC)
    CodeLOI,
    ///LOS - Loss Conditions
    LossConditions,
    ///LP - For Pickup Limited Freight Tariff Number
    ForPickupLimitedFreightTariffNumber,
    ///LPK - Loan Prospector Key Number
    LoanProspectorKeyNumber,
    ///LQ - Latitude Expressed in Degrees, Minutes and Seconds
    CodeLQ,
    ///LR - Local Student Identification Number
    LocalStudentIdentificationNumber,
    ///LS - Bar-Coded Serial Number
    BarCodedSerialNumber,
    ///LSD - Logistics Support Documentation Type Code
    LogisticsSupportDocumentationTypeCode,
    ///LT - Lot Number
    LotNumber,
    ///LU - Location Number
    LocationNumber,
    ///LV - License Plate Number
    LicensePlateNumber,
    ///LVO - Levying Officer Identification
    LevyingOfficerIdentification,
    ///LW - Location Within Equipment
    LocationWithinEquipment,
    ///LX - Qualified Products List
    QualifiedProductsList,
    ///LY - Destination of Shipment Harmonized-Based Code
    DestinationOfShipmentHarmonizedBasedCode,
    ///LZ - Lender Account Number
    LenderAccountNumber,
    ///M0 - Mexican Pedimento Number
    MexicanPedimentoNumber,
    ///M1 - Material Storage Location
    MaterialStorageLocation,
    ///M2 - Major Force Program
    MajorForceProgram,
    ///M3 - Crop Year
    CropYear,
    ///M5 - Lease Agreement Amendment Number - Master
    LeaseAgreementAmendmentNumberMaster,
    ///M6 - Military Ordnance Security Risk Number
    MilitaryOrdnanceSecurityRiskNumber,
    ///M7 - Medical Assistance Category
    MedicalAssistanceCategory,
    ///M8 - Limited Partnership Identification Number
    LimitedPartnershipIdentificationNumber,
    ///M9 - Tax Shelter Number
    TaxShelterNumber,
    ///MA - Ship Notice/Manifest Number
    ShipNoticeManifestNumber,
    ///MB - Master Bill of Lading
    MasterBillOfLading,
    ///MBS - Mortgage Backed Security (MBS) Policy Number
    CodeMBS,
    ///MBX - Mailbox
    Mailbox,
    ///MC - Microfilm Number
    MicrofilmNumber,
    ///MCC - Carrier's Bond Number Covering Merchandise Shipment and Instruments of International Traffic (IIT)
    CodeMCC,
    ///MCI - Motor Carrier Identification Number
    MotorCarrierIdentificationNumber,
    ///MCN - MORNETPlus Case Number
    MornetPlusCaseNumber,
    ///MD - Magazine Code
    MagazineCode,
    ///MDN - Hazardous Waste Manifest Document Number
    HazardousWasteManifestDocumentNumber,
    ///ME - Message Address or ID
    MessageAddressOrId,
    ///MF - Manufacturers Part Number
    ManufacturersPartNumber,
    ///MG - Meter Number
    MeterNumber,
    ///MH - Manufacturing Order Number
    ManufacturingOrderNumber,
    ///MI - Mill Order Number
    MillOrderNumber,
    ///MII - Importer's Bond Number Covering Merchandise Shipment and Instruments of International Traffic (IIT)
    CodeMII,
    ///MIN - MORNETPlus Institution Number
    MornetPlusInstitutionNumber,
    ///MJ - Model Number
    ModelNumber,
    ///MK - Manifest Key Number
    ManifestKeyNumber,
    ///ML - Military Rank/Civilian Pay Grade Number
    MilitaryRankCivilianPayGradeNumber,
    ///MM - Master Lease Agreement Number
    MasterLeaseAgreementNumber,
    ///MN - MICR Number
    MicrNumber,
    ///MO - Manufacturing Operation Number
    ManufacturingOperationNumber,
    ///MP - Multiple P.O.s of an Invoice
    MultiplePOSOfAnInvoice,
    ///MPN - Marketing Plan Identification Number
    MarketingPlanIdentificationNumber,
    ///MQ - Meter Proving Report Number
    MeterProvingReportNumber,
    ///MR - Merchandise Type Code
    MerchandiseTypeCode,
    ///MRC - Eligibility Category
    EligibilityCategory,
    ///MRN - Mother's Medical Record Identification Number
    MothersMedicalRecordIdentificationNumber,
    ///MS - Manufacturer's Material Safety Data Sheet Number
    ManufacturersMaterialSafetyDataSheetNumber,
    ///MSL - Mail Slot
    MailSlot,
    ///MT - Meter Ticket Number
    MeterTicketNumber,
    ///MU - Military Specification (MILSPEC) Number
    CodeMU,
    ///MUI - MORNETPlus User Identification
    MornetPlusUserIdentification,
    ///MV - Migrant Number
    MigrantNumber,
    ///MW - Military Call Number
    MilitaryCallNumber,
    ///MX - Material Change Notice Number
    MaterialChangeNoticeNumber,
    ///MY - Model year number
    ModelYearNumber,
    ///MZ - Maintenance Request Number
    MaintenanceRequestNumber,
    ///MZO - Multiple Zone Order Number
    MultipleZoneOrderNumber,
    ///N0 - Nomination Number
    NominationNumber,
    ///N1 - Local School Course Number
    LocalSchoolCourseNumber,
    ///N2 - Local School District Course Number
    LocalSchoolDistrictCourseNumber,
    ///N3 - Statewide Course Number
    StatewideCourseNumber,
    ///N4 - United States Department of Education, National Center for Education Statistics (NCES) Course Number
    CodeN4,
    ///N5 - Provider Plan Network Identification Number
    ProviderPlanNetworkIdentificationNumber,
    ///N6 - Plan Network Identification Number
    PlanNetworkIdentificationNumber,
    ///N7 - Facility Network Identification Number
    FacilityNetworkIdentificationNumber,
    ///N8 - Secondary Health Insurance Identification Number
    SecondaryHealthInsuranceIdentificationNumber,
    ///N9 - Data Authentication Number
    DataAuthenticationNumber,
    ///NA - North American Hazardous Classification Number
    NorthAmericanHazardousClassificationNumber,
    ///NAS - National Aeronautics and Space Administration FAR Supplement (NFS)
    CodeNAS,
    ///NB - Letter of Credit Number
    LetterOfCreditNumber,
    ///NC - Secondary Coverage Company Number
    SecondaryCoverageCompanyNumber,
    ///ND - Letter of Credit Draft Number
    LetterOfCreditDraftNumber,
    ///NDA - Abbreviated New Drug Application Number
    AbbreviatedNewDrugApplicationNumber,
    ///NDB - New Drug Application Number
    NewDrugApplicationNumber,
    ///NE - Lease Rider Number
    LeaseRiderNumber,
    ///NF - National Association of Insurance Commissioners (NAIC) Code
    CodeNF,
    ///NFC - National Flood Insurance Program Community Name
    NationalFloodInsuranceProgramCommunityName,
    ///NFD - National Flood Insurance Program County
    NationalFloodInsuranceProgramCounty,
    ///NFM - National Flood Insurance Program Map Number
    NationalFloodInsuranceProgramMapNumber,
    ///NFN - National Flood Insurance Program Community Number
    NationalFloodInsuranceProgramCommunityNumber,
    ///NFS - National Flood Insurance Program State
    NationalFloodInsuranceProgramState,
    ///NG - Natural Gas Policy Act Category Code
    NaturalGasPolicyActCategoryCode,
    ///NH - Rate Card Number
    RateCardNumber,
    ///NI - Military Standard (MIL-STD) Number
    CodeNI,
    ///NJ - Technical Document Number
    TechnicalDocumentNumber,
    ///NK - Prior Case
    PriorCase,
    ///NL - Technical Order Number
    TechnicalOrderNumber,
    ///NM - Discounter Registration Number
    DiscounterRegistrationNumber,
    ///NMT - Nomination Model Type
    NominationModelType,
    ///NN - Nonconformance Report Number
    NonconformanceReportNumber,
    ///NO - No OT5 Authority-zero Mileage Rate
    NoOt5AuthorityZeroMileageRate,
    ///NP - Partial Payment Number
    PartialPaymentNumber,
    ///NQ - Medicaid Recipient Identification Number
    MedicaidRecipientIdentificationNumber,
    ///NR - Progress Payment Number
    ProgressPaymentNumber,
    ///NS - National Stock Number
    NationalStockNumber,
    ///NT - Administrator's Reference Number
    AdministratorsReferenceNumber,
    ///NTP - Non-originating Third Party Number
    NonOriginatingThirdPartyNumber,
    ///NU - Pending Case
    PendingCase,
    ///NW - Associated Policy Number
    AssociatedPolicyNumber,
    ///NX - Related Nonconformance Number
    RelatedNonconformanceNumber,
    ///NY - Agent Claim Number
    AgentClaimNumber,
    ///NZ - Critical Application
    CriticalApplication,
    ///O1 - Outer Continental Shelf Area Code
    OuterContinentalShelfAreaCode,
    ///O2 - Outer Continental Shelf Block Number
    OuterContinentalShelfBlockNumber,
    ///O5 - OT5 Authority-Condition or Restriction on Car Hire Rate
    Ot5AuthorityConditionOrRestrictionOnCarHireRate,
    ///O7 - On-line Procurement and Accounting Control (OPAC) Transaction
    CodeO7,
    ///O8 - Original Filing
    OriginalFiling,
    ///O9 - Continuation Filing
    ContinuationFiling,
    ///OA - Outlet Number
    OutletNumber,
    ///OB - Ocean Bill of Lading
    OceanBillOfLading,
    ///OC - Ocean Container Number
    OceanContainerNumber,
    ///OD - Original Return Request Reference Number
    OriginalReturnRequestReferenceNumber,
    ///OE - Open and Prepaid Station List Number
    OpenAndPrepaidStationListNumber,
    ///OF - Operator Identification Number
    OperatorIdentificationNumber,
    ///OFF - Offer Identifier
    Offer,
    ///OG - Termination Filing
    TerminationFiling,
    ///OH - Origin House
    OriginHouse,
    ///OI - Original Invoice Number
    OriginalInvoiceNumber,
    ///OIC - Object Identifier
    Object,
    ///OJ - Amendment Filing
    AmendmentFiling,
    ///OK - Offer Group
    OfferGroup,
    ///OL - Original Shipper's Bill of Lading Number
    OriginalShippersBillOfLadingNumber,
    ///OM - Ocean Manifest
    OceanManifest,
    ///ON - Dealer Order Number
    DealerOrderNumber,
    ///OOS - Out of Service Number
    OutOfServiceNumber,
    ///OP - Original Purchase Order
    OriginalPurchaseOrder,
    ///OPE - National Center for Education Statistics Office of Postsecondary Education (OPE) Code
    CodeOPE,
    ///OPF - National Center for Education Statistics Integrated Postsecondary Education Data System (IPEDS) Athletic Conference Code
    CodeOPF,
    ///OQ - Order Number
    OrderNumber,
    ///OR - Order/Paragraph Number
    OrderParagraphNumber,
    ///OS - Outbound-from Party
    OutboundFromParty,
    ///OT - Sales Allowance Number
    SalesAllowanceNumber,
    ///OU - Tariff Supplement Number
    TariffSupplementNumber,
    ///OV - Tariff Suffix Number
    TariffSuffixNumber,
    ///OW - Service Order Number
    ServiceOrderNumber,
    ///OX - Statement Number
    StatementNumber,
    ///OZ - Product Number
    ProductNumber,
    ///P1 - Previous Contract Number
    PreviousContractNumber,
    ///P2 - Previous Drug Enforcement Administration Number
    PreviousDrugEnforcementAdministrationNumber,
    ///P3 - Previous customer reference number
    PreviousCustomerReferenceNumber,
    ///P4 - Project Code
    ProjectCode,
    ///P5 - Position Code
    PositionCode,
    ///P6 - Pipeline Number
    PipelineNumber,
    ///P7 - Product Line Number
    ProductLineNumber,
    ///P8 - Pickup Reference Number
    PickupReferenceNumber,
    ///P9 - Page Number
    PageNumber,
    ///PA - Price Area Number
    PriceAreaNumber,
    ///PAC - Patent Cooperation Treaty Application Number
    PatentCooperationTreatyApplicationNumber,
    ///PAN - Nonprovisional Patent Application Number
    NonprovisionalPatentApplicationNumber,
    ///PAP - Provisional Patent Application Number
    ProvisionalPatentApplicationNumber,
    ///PB - Payer's Financial Institution Account Number for Check, Draft, or Wire Payments; Originating Company Account Number for ACH Transfers
    CodePB,
    ///PC - Production Code
    ProductionCode,
    ///PCC - Pool Contract Code
    PoolContractCode,
    ///PCN - Protocol Number
    ProtocolNumber,
    ///PD - Promotion/Deal Number
    PromotionDealNumber,
    ///PDI - Partial Denial Indicator
    PartialDenialIndicator,
    ///PDL - Previous Driver's License
    PreviousDriversLicense,
    ///PDR - Partial Denial Reason Identifier
    PartialDenialReason,
    ///PE - Plant Number
    PlantNumber,
    ///PF - Prime Contractor Contract Number
    PrimeContractorContractNumber,
    ///PG - Product Group
    ProductGroup,
    ///PGC - Packing Group Code
    PackingGroupCode,
    ///PGD - Downstream Package Identifier
    DownstreamPackage,
    ///PGN - Plug Number
    PlugNumber,
    ///PGS - Proposed Group Work Candidate Sequence Number
    ProposedGroupWorkCandidateSequenceNumber,
    ///PH - Priority Rating
    PriorityRating,
    ///PHC - Process Handling Code
    ProcessHandlingCode,
    ///PHY - Physician State License Number
    PhysicianStateLicenseNumber,
    ///PI - Price List Change or Issue Number
    PriceListChangeOrIssueNumber,
    ///PID - Program Identification Number
    ProgramIdentificationNumber,
    ///PIN - Platform Identification Number
    PlatformIdentificationNumber,
    ///PJ - Packer Number
    PackerNumber,
    ///PJC - Previous Report Number
    PreviousReportNumber,
    ///PK - Packing List Number
    PackingListNumber,
    ///PKG - Package Identifier
    Package,
    ///PKU - Upstream Package Identifier
    UpstreamPackage,
    ///PL - Price List Number
    PriceListNumber,
    ///PLA - Product Licensing Agreement Number
    ProductLicensingAgreementNumber,
    ///PLN - Proposed Contract Number
    ProposedContractNumber,
    ///PM - Part Number
    PartNumber,
    ///PMN - Premarket Application Number
    PremarketApplicationNumber,
    ///PN - Permit Number
    PermitNumber,
    ///PNN - Patent Number
    PatentNumber,
    ///PO - Purchase Order Number
    PurchaseOrderNumber,
    ///POL - Policy Number
    PolicyNumber,
    ///POS - Position Title Number
    PositionTitleNumber,
    ///PP - Purchase Order Revision Number
    PurchaseOrderRevisionNumber,
    ///PPJ - Certificate of Purchase Number
    CertificateOfPurchaseNumber,
    ///PPK - Tax Bill Identification Number
    TaxBillIdentificationNumber,
    ///PPL - Current Year Tax Bill Number
    CurrentYearTaxBillNumber,
    ///PPM - Past Year Tax Bill Number
    PastYearTaxBillNumber,
    ///PPN - Payment Plan Number
    PaymentPlanNumber,
    ///PQ - Payee Identification
    PayeeIdentification,
    ///PR - Price Quote Number
    PriceQuoteNumber,
    ///PRS - Previously Reported Social Security Number
    PreviouslyReportedSocialSecurityNumber,
    ///PRT - Product Type
    ProductType,
    ///PS - Purchase Order Number Suffix
    PurchaseOrderNumberSuffix,
    ///PSI - Previous Shipment Identification Number - Continuous Move
    PreviousShipmentIdentificationNumberContinuousMove,
    ///PSL - Next Shipment Identification Number - Continuous Move
    NextShipmentIdentificationNumberContinuousMove,
    ///PSM - Credit Card
    CreditCard,
    ///PSN - Proposed Sequence Number
    ProposedSequenceNumber,
    ///PT - Purchase Option Agreement
    PurchaseOptionAgreement,
    ///PTC - Patent Type
    PatentType,
    ///PU - Previous Bill of Lading Number
    PreviousBillOfLadingNumber,
    ///PUA - Pickup Appointment Number
    PickupAppointmentNumber,
    ///PV - Product change information number
    ProductChangeInformationNumber,
    ///PVC - Payment Validation Code
    PaymentValidationCode,
    ///PW - Prior purchase order number
    PriorPurchaseOrderNumber,
    ///PWC - Preliminary Work Candidate Number
    PreliminaryWorkCandidateNumber,
    ///PWS - Proposed Work Candidate Sequence Number
    ProposedWorkCandidateSequenceNumber,
    ///PX - Previous Invoice Number
    PreviousInvoiceNumber,
    ///PXC - Health Care Provider Taxonomy Code
    HealthCareProviderTaxonomyCode,
    ///PY - Payee's Financial Institution Account Number for Check, Draft or Wire Payments; Receiving Company Account Number for ACH Transfer
    CodePY,
    ///PYA - Payroll Activity Code
    PayrollActivityCode,
    ///PYR - Pay Range
    PayRange,
    ///PZ - Product Change Notice Number
    ProductChangeNoticeNumber,
    ///Q1 - Quote Number
    QuoteNumber,
    ///Q2 - Starting Package Number
    StartingPackageNumber,
    ///Q3 - Ending Package Number
    EndingPackageNumber,
    ///Q4 - Prior Identifier Number
    PriorIdentifierNumber,
    ///Q5 - Property Control Number
    PropertyControlNumber,
    ///Q6 - Recall Number
    RecallNumber,
    ///Q7 - Receiver Claim Number
    ReceiverClaimNumber,
    ///Q8 - Registration Number
    RegistrationNumber,
    ///Q9 - Repair Order Number
    RepairOrderNumber,
    ///QA - Press Identifier
    Press,
    ///QB - Press Form Identifier
    PressForm,
    ///QC - Product Specification Document Number
    ProductSpecificationDocumentNumber,
    ///QD - Replacement Drug Enforcement Administration Number
    ReplacementDrugEnforcementAdministrationNumber,
    ///QE - Replacement Customer Reference Number
    ReplacementCustomerReferenceNumber,
    ///QF - Quality Disposition Area Identifier
    QualityDispositionArea,
    ///QG - Replacement Assembly Model Number
    ReplacementAssemblyModelNumber,
    ///QH - Replacement Assembly Serial Number
    ReplacementAssemblySerialNumber,
    ///QI - Quality Inspection Area Identifier
    QualityInspectionArea,
    ///QJ - Return Material Authorization Number
    ReturnMaterialAuthorizationNumber,
    ///QK - Sales Program Number
    SalesProgramNumber,
    ///QL - Service Authorization Number
    ServiceAuthorizationNumber,
    ///QM - Quality Review Material Crib Identifier
    QualityReviewMaterialCrib,
    ///QN - Stop Sequence Number
    StopSequenceNumber,
    ///QO - Service Estimate Number
    ServiceEstimateNumber,
    ///QP - Substitute Part Number
    SubstitutePartNumber,
    ///QQ - Unit Number
    UnitNumber,
    ///QR - Quality Report Number
    QualityReportNumber,
    ///QS - Warranty Coverage Code
    WarrantyCoverageCode,
    ///QT - Warranty Registration Number
    WarrantyRegistrationNumber,
    ///QU - Change Verification Procedure Code
    ChangeVerificationProcedureCode,
    ///QV - Major System Affected Code
    MajorSystemAffectedCode,
    ///QW - New Part Number
    NewPartNumber,
    ///QX - Old Part Number
    OldPartNumber,
    ///QY - Service Performed Code
    ServicePerformedCode,
    ///QZ - Reference Drawing Number
    ReferenceDrawingNumber,
    ///R0 - Regiristo Federal de Contribuyentes (Mexican Federal Tax ID Number)
    CodeR0,
    ///R1 - Current Revision Number
    CurrentRevisionNumber,
    ///R2 - Canceled Revision Number
    CanceledRevisionNumber,
    ///R3 - Correction Number
    CorrectionNumber,
    ///R4 - Tariff Section Number
    TariffSectionNumber,
    ///R5 - Tariff Page Number
    TariffPageNumber,
    ///R6 - Tariff Rule Number
    TariffRuleNumber,
    ///R7 - Accounts Receivable Open Item
    AccountsReceivableOpenItem,
    ///R8 - Rental Agreement Number
    RentalAgreementNumber,
    ///R9 - Rejection Number
    RejectionNumber,
    ///RA - Repetitive Cargo Shipment Number
    RepetitiveCargoShipmentNumber,
    ///RAA - Restricted Availability Authorization
    RestrictedAvailabilityAuthorization,
    ///RAN - Restricted Availability Number
    RestrictedAvailabilityNumber,
    ///RB - Rate code number
    RateCodeNumber,
    ///RC - Rail Routing Code
    RailRoutingCode,
    ///RD - Reel Number
    ReelNumber,
    ///RE - Release Number
    ReleaseNumber,
    ///REC - Related Case
    RelatedCase,
    ///RF - Export Reference Number
    ExportReferenceNumber,
    ///RG - Route Order Number-Domestic
    RouteOrderNumberDomestic,
    ///RGI - Regulatory Guideline Identifier
    RegulatoryGuideline,
    ///RH - Route Order Number-Export
    RouteOrderNumberExport,
    ///RI - Release invoice number for prior bill and hold
    ReleaseInvoiceNumberForPriorBillAndHold,
    ///RIG - Rig Number
    RigNumber,
    ///RJ - Route Order Number-Emergency
    RouteOrderNumberEmergency,
    ///RK - Rack Type Number
    RackTypeNumber,
    ///RL - Reserve Assembly Line Feed Location
    ReserveAssemblyLineFeedLocation,
    ///RLI - Role Identification Number
    RoleIdentificationNumber,
    ///RM - Raw material supplier Dun & Bradstreet number
    CodeRM,
    ///RN - Run Number
    RunNumber,
    ///RO - Repetitive Booking Number
    RepetitiveBookingNumber,
    ///RP - Repetitive Pattern Code
    RepetitivePatternCode,
    ///RPP - Relative Priority
    RelativePriority,
    ///RPS - Regulation Primary Number
    RegulationPrimaryNumber,
    ///RPT - Report Number
    ReportNumber,
    ///RQ - Purchase Requisition Number
    PurchaseRequisitionNumber,
    ///RR - Payer's Financial Institution Transit Routing Number for Check, Draft or Wire Payments. Originating Depository Financial Institution Routing Number for ACH Transfers
    CodeRR,
    ///RRC - Routing Request Control Number
    RoutingRequestControlNumber,
    ///RRS - Reconciliation Report Section Identification Code
    ReconciliationReportSectionIdentificationCode,
    ///RS - Returnable Container Serial Number
    ReturnableContainerSerialNumber,
    ///RSN - Reservation Number
    ReservationNumber,
    ///RSS - Regulation Secondary Number
    RegulationSecondaryNumber,
    ///RT - Payee's Financial Institution Transit Routing Number for Check, Draft or Wire Payments. Receiving Depository Financial Institution Transit Routing Number for ACH Transfers
    CodeRT,
    ///RU - Route Number
    RouteNumber,
    ///RV - Receiving Number
    ReceivingNumber,
    ///RW - Repetitive Waybill Code (Origin Carrier, Standard Point Location Code, Repetitive Waybill Code Number)
    CodeRW,
    ///RWK - Reporting Week
    ReportingWeek,
    ///RX - Resubmit number
    ResubmitNumber,
    ///RY - Rebate Number
    RebateNumber,
    ///RZ - Returned Goods Authorization Number
    ReturnedGoodsAuthorizationNumber,
    ///S0 - Special Approval
    SpecialApproval,
    ///S1 - Engineering Specification Number
    EngineeringSpecificationNumber,
    ///S2 - Data Source
    DataSource,
    ///S3 - Specification Number
    SpecificationNumber,
    ///S4 - Shippers Bond Number
    ShippersBondNumber,
    ///S5 - Routing Instruction Number
    RoutingInstructionNumber,
    ///S6 - Stock Number
    StockNumber,
    ///S7 - Stack Train Identification
    StackTrainIdentification,
    ///S8 - Seal Off Number
    SealOffNumber,
    ///S9 - Seal On Number
    SealOnNumber,
    ///SA - Salesperson
    Salesperson,
    ///SAL - Salary Step
    SalaryStep,
    ///SB - Sales Region Number
    SalesRegionNumber,
    ///SBN - Surety Bond Number
    SuretyBondNumber,
    ///SC - Shipper Car Order Number
    ShipperCarOrderNumber,
    ///SCA - Standard Carrier Alpha Code (SCAC)
    CodeSCA,
    ///SCN - Scale Number
    ScaleNumber,
    ///SD - Subday Number
    SubdayNumber,
    ///SDT - School District Type Code
    SchoolDistrictTypeCode,
    ///SE - Serial Number
    SerialNumber,
    ///SEK - Search Key
    SearchKey,
    ///SES - Session
    Session,
    ///SF - Ship From
    ShipFrom,
    ///SG - Savings
    Savings,
    ///SH - Sender Defined Clause
    SenderDefinedClause,
    ///SHL - Shelf Life Indicator
    ShelfLifeIndicator,
    ///SI - Shipper's Identifying Number for Shipment (SID)
    CodeSI,
    ///SII - Salvage Instruction Identifier
    SalvageInstruction,
    ///SJ - Set Number
    SetNumber,
    ///SK - Service Change Number
    ServiceChangeNumber,
    ///SL - Sales/Territory Code
    SalesTerritoryCode,
    ///SM - Sales Office Number
    SalesOfficeNumber,
    ///SMC - Settlement Method Code
    SettlementMethodCode,
    ///SMT - State of Massachusetts Town Code
    StateOfMassachusettsTownCode,
    ///SN - Seal Number
    SealNumber,
    ///SNH - SNOMED, Systematized Nomenclature of Medicine
    CodeSNH,
    ///SNP - US Customs & Border Protection Second Notify Party
    CodeSNP,
    ///SNV - State Non-Resident Violator Compact
    StateNonResidentViolatorCompact,
    ///SO - Shipper's Order (Invoice Number)
    CodeSO,
    ///SP - Scan Line
    ScanLine,
    ///SPL - Standard Point Location Code (SPLC)
    CodeSPL,
    ///SPN - Theater Screen Number
    TheaterScreenNumber,
    ///SQ - Container Sequence Number
    ContainerSequenceNumber,
    ///SR - Sales Responsibility
    SalesResponsibility,
    ///SS - Split Shipment Number
    SplitShipmentNumber,
    ///SST - School System Type Code
    SchoolSystemTypeCode,
    ///ST - Store Number
    StoreNumber,
    ///STB - Standard Transportation Commodity Code (STCC) Bridge Number
    CodeSTB,
    ///STR - Standard Transportation Commodity Code (STCC) Replacement Code
    CodeSTR,
    ///STS - Serviceability Standard Testing Reference
    ServiceabilityStandardTestingReference,
    ///SU - Special Processing Code
    SpecialProcessingCode,
    ///SUB - Title Reference
    TitleReference,
    ///SUC - Supervisory Union Code
    SupervisoryUnionCode,
    ///SUO - Spacing Unit Order Number
    SpacingUnitOrderNumber,
    ///SV - Service Charge Number
    ServiceChargeNumber,
    ///SW - Seller's Sale Number
    SellersSaleNumber,
    ///SX - Service Interrupt Tracking Number
    ServiceInterruptTrackingNumber,
    ///SY - Social Security Number
    SocialSecurityNumber,
    ///SZ - Specification Revision
    SpecificationRevision,
    ///T0 - Dealer Type Identification
    DealerTypeIdentification,
    ///T1 - Tax Exchange Code
    TaxExchangeCode,
    ///T2 - Tax Form Code
    TaxFormCode,
    ///T3 - Tax Schedule Code
    TaxScheduleCode,
    ///T4 - Signal Code
    SignalCode,
    ///T5 - Trailer Use Agreements
    TrailerUseAgreements,
    ///T6 - Tax Filing
    TaxFiling,
    ///T7 - Affected Subsystem Code
    AffectedSubsystemCode,
    ///T8 - Description of Change Code
    DescriptionOfChangeCode,
    ///T9 - Documentation Affected Number
    DocumentationAffectedNumber,
    ///TA - Telecommunication Circuit Supplemental ID
    TelecommunicationCircuitSupplementalId,
    ///TB - Trucker's Bill of Lading
    TruckersBillOfLading,
    ///TC - Vendor Terms
    VendorTerms,
    ///TD - Reason for Change
    ReasonForChange,
    ///TDT - Technical Documentation Type
    TechnicalDocumentationType,
    ///TE - Federal Maritime Commission (FMC) Tariff Number
    CodeTE,
    ///TF - Transfer Number
    TransferNumber,
    ///TFC - Time Failure
    TimeFailure,
    ///TG - Transportation Control Number (TCN)
    CodeTG,
    ///TH - Transportation Account Code (TAC)
    CodeTH,
    ///TI - TIR Number
    TirNumber,
    ///TIP - Technical Information Package
    TechnicalInformationPackage,
    ///TJ - Federal Taxpayer's Identification Number
    FederalTaxpayersIdentificationNumber,
    ///TK - Tank Number
    TankNumber,
    ///TL - Tax License Exemption
    TaxLicenseExemption,
    ///TM - Travel Manifest (ACI or OTR)
    CodeTM,
    ///TN - Transaction Reference Number
    TransactionReferenceNumber,
    ///TO - Terminal Operator Number
    TerminalOperatorNumber,
    ///TOC - Type of Comment
    TypeOfComment,
    ///TP - Test Specification Number
    TestSpecificationNumber,
    ///TPN - Transponder Number
    TransponderNumber,
    ///TQ - Tracer Action Request Number
    TracerActionRequestNumber,
    ///TR - Government Transportation Request
    GovernmentTransportationRequest,
    ///TS - Tariff Number
    TariffNumber,
    ///TSN - Template Sequence Number
    TemplateSequenceNumber,
    ///TT - Terminal Code
    TerminalCode,
    ///TU - Trial Location Code
    TrialLocationCode,
    ///TV - Line of Business
    LineOfBusiness,
    ///TW - Tax Worksheet
    TaxWorksheet,
    ///TX - Tax Exempt Number
    TaxExemptNumber,
    ///TY - Policy Type
    PolicyType,
    ///TZ - Total Cycle Number
    TotalCycleNumber,
    ///U0 - Consolidator's Receipt Number
    ConsolidatorsReceiptNumber,
    ///U1 - Regional Account Number
    RegionalAccountNumber,
    ///U2 - Term
    Term,
    ///U3 - Unique Supplier Identification Number (USIN)
    CodeU3,
    ///U4 - Unpaid Installment Reference Number
    UnpaidInstallmentReferenceNumber,
    ///U5 - Successor Account
    SuccessorAccount,
    ///U6 - Predecessor Account
    PredecessorAccount,
    ///U8 - Mortgage Backed Security (MBS) Loan Number
    CodeU8,
    ///U9 - Mortgage Backed Security (MBS) Pool Number
    CodeU9,
    ///UA - Mortgage Number
    MortgageNumber,
    ///UB - Unacceptable Source Purchaser ID
    UnacceptableSourcePurchaserId,
    ///UC - Mortgage Insurance Indicator Number
    MortgageInsuranceIndicatorNumber,
    ///UCB - EAN.UCC Bill of Lading Number (17 Digits)
    CodeUCB,
    ///UCM - EAN.UCC Master Bill of Lading Number (17 Digits)
    CodeUCM,
    ///UD - Unacceptable Source DUNS Number
    UnacceptableSourceDunsNumber,
    ///UE - Secondary Coverage Certificate Number
    SecondaryCoverageCertificateNumber,
    ///UF - Mortgage Insurance Company Number
    MortgageInsuranceCompanyNumber,
    ///UG - U.S. Government Transportation Control Number
    USGovernmentTransportationControlNumber,
    ///UH - Removal Number
    RemovalNumber,
    ///UI - Previous Course Number
    PreviousCourseNumber,
    ///UIC - Unit Identification Code (UIC)
    CodeUIC,
    ///UJ - Current or Latest Course Number
    CurrentOrLatestCourseNumber,
    ///UK - Equivalent Course Number at Requesting Institution
    EquivalentCourseNumberAtRequestingInstitution,
    ///UL - Cross-listed Course Number
    CrossListedCourseNumber,
    ///UM - Quarter Quarter Section Number
    QuarterQuarterSectionNumber,
    ///UN - United Nations Hazardous Classification Number
    UnitedNationsHazardousClassificationNumber,
    ///UO - Quarter Quarter Spot Number
    QuarterQuarterSpotNumber,
    ///UP - Upstream Shipper Contract Number
    UpstreamShipperContractNumber,
    ///UQ - Section Number
    SectionNumber,
    ///UR - Unit Relief Number
    UnitReliefNumber,
    ///URL - Uniform Resource Locator
    UniformResourceLocator,
    ///URP - Unit Report Period
    UnitReportPeriod,
    ///URQ - Unit Report Period ID
    UnitReportPeriodId,
    ///US - Unacceptable Source Supplier ID
    UnacceptableSourceSupplierId,
    ///UT - Unit Train
    UnitTrain,
    ///UU - Township Number
    TownshipNumber,
    ///UV - Range Number
    RangeNumber,
    ///UW - State Senate District
    StateSenateDistrict,
    ///UX - State Assembly District
    StateAssemblyDistrict,
    ///UY - Federal National Mortgage Association (Fannie Mae) Loan Number
    CodeUY,
    ///UZ - State Legislative District
    StateLegislativeDistrict,
    ///V0 - Version
    Version,
    ///V1 - Volume Purchase Agreement Number
    VolumePurchaseAgreementNumber,
    ///V2 - Visa Type
    VisaType,
    ///V3 - Voyage Number
    VoyageNumber,
    ///V4 - State Department I-20 Form Number
    StateDepartmentI20FormNumber,
    ///V5 - State Department IAP-66 Form Number
    StateDepartmentIap66FormNumber,
    ///V6 - North American Free Trade Agreement (NAFTA) Compliance Number
    CodeV6,
    ///V7 - Judicial District
    JudicialDistrict,
    ///V8 - Institution Number
    InstitutionNumber,
    ///V9 - Subservicer
    Subservicer,
    ///VA - Vessel Agent Number
    VesselAgentNumber,
    ///VAO - Veterans Administration Originator Identification
    VeteransAdministrationOriginatorIdentification,
    ///VB - Department of Veterans Affairs Acquisition Regulations (VAAR)
    CodeVB,
    ///VC - Vendor Contract Number
    VendorContractNumber,
    ///VD - Volume Number
    VolumeNumber,
    ///VE - Vendor Abbreviation Code
    VendorAbbreviationCode,
    ///VF - Vendor Change Identification Code
    VendorChangeIdentificationCode,
    ///VG - Vendor Change Procedure Code
    VendorChangeProcedureCode,
    ///VGS - Vehicle Garaged State Code
    VehicleGaragedStateCode,
    ///VH - County Legislative District
    CountyLegislativeDistrict,
    ///VI - Pool Number
    PoolNumber,
    ///VJ - Investor Note Holder Identification
    InvestorNoteHolderIdentification,
    ///VK - Institution Note Holder Identification
    InstitutionNoteHolderIdentification,
    ///VL - Third Party Note Holder Identification
    ThirdPartyNoteHolderIdentification,
    ///VM - Ward
    Ward,
    ///VN - Vendor Order Number
    VendorOrderNumber,
    ///VO - Institution Loan Number
    InstitutionLoanNumber,
    ///VP - Vendor Product Number
    VendorProductNumber,
    ///VQ - Related Contract Line Item Number
    RelatedContractLineItemNumber,
    ///VR - Vendor ID Number
    VendorIdNumber,
    ///VS - Vendor Order Number Suffix
    VendorOrderNumberSuffix,
    ///VT - Motor Vehicle ID Number
    MotorVehicleIdNumber,
    ///VU - Preparer's Verification Number
    PreparersVerificationNumber,
    ///VV - Voucher
    Voucher,
    ///VW - Standard
    Standard,
    ///VX - Value-Added Tax Registration Number (Europe)
    CodeVX,
    ///VY - Link Sequence Number
    LinkSequenceNumber,
    ///VZ - Sponsor's Reference Number
    SponsorsReferenceNumber,
    ///W1 - Disposal Turn-In Document Number
    DisposalTurnInDocumentNumber,
    ///W2 - Weapon System Number
    WeaponSystemNumber,
    ///W3 - Manufacturing Directive Number
    ManufacturingDirectiveNumber,
    ///W4 - Procurement Request Number
    ProcurementRequestNumber,
    ///W5 - Inspector Identification Number
    InspectorIdentificationNumber,
    ///W6 - Federal Supply Schedule Number
    FederalSupplyScheduleNumber,
    ///W7 - Commercial and Government Entity (CAGE) Code
    CodeW7,
    ///W8 - Suffix
    Suffix,
    ///W9 - Special Packaging Instruction Number
    SpecialPackagingInstructionNumber,
    ///WA - Labor or Affiliation Identification
    LaborOrAffiliationIdentification,
    ///WB - American Petroleum Institute (API) Well
    CodeWB,
    ///WC - Contract Option Number
    ContractOptionNumber,
    ///WCS - Work Candidate Sequence Number
    WorkCandidateSequenceNumber,
    ///WD - Review Period Number
    ReviewPeriodNumber,
    ///WDR - Withdrawal Record
    WithdrawalRecord,
    ///WE - Well Classification Code
    WellClassificationCode,
    ///WF - Locally Assigned Control Number
    LocallyAssignedControlNumber,
    ///WG - Vendor's Previous Job Number
    VendorsPreviousJobNumber,
    ///WH - Master Reference (Link) Number
    CodeWH,
    ///WI - Waiver
    Waiver,
    ///WJ - Pre-Award Survey
    PreAwardSurvey,
    ///WK - Type of Science Code
    TypeOfScienceCode,
    ///WL - Federal Supply Classification Code
    FederalSupplyClassificationCode,
    ///WM - Weight Agreement Number
    WeightAgreementNumber,
    ///WN - Well Number
    WellNumber,
    ///WO - Work Order Number
    WorkOrderNumber,
    ///WP - Warehouse Pick Ticket Number
    WarehousePickTicketNumber,
    ///WQ - Interim Funding Organization Loan Number
    InterimFundingOrganizationLoanNumber,
    ///WR - Warehouse Receipt Number
    WarehouseReceiptNumber,
    ///WS - Warehouse storage location number
    WarehouseStorageLocationNumber,
    ///WT - Broker's Reference Number
    BrokersReferenceNumber,
    ///WU - Vessel
    Vessel,
    ///WV - Dealer Identification
    DealerIdentification,
    ///WW - Depository Trust Company Identification
    DepositoryTrustCompanyIdentification,
    ///WX - Distributor's Account Identification
    DistributorsAccountIdentification,
    ///WY - Waybill Number
    WaybillNumber,
    ///WZ - Distributor's Representative Identification
    DistributorsRepresentativeIdentification,
    ///X0 - Debtor's Account
    DebtorsAccount,
    ///X1 - Provider Claim Number
    ProviderClaimNumber,
    ///X2 - Specification Class Number
    SpecificationClassNumber,
    ///X3 - Defect Code Number
    DefectCodeNumber,
    ///X4 - Clinical Laboratory Improvement Amendment Number
    ClinicalLaboratoryImprovementAmendmentNumber,
    ///X5 - State Industrial Accident Provider Number
    StateIndustrialAccidentProviderNumber,
    ///X6 - Original Voucher Number
    OriginalVoucherNumber,
    ///X7 - Batch Sequence Number
    BatchSequenceNumber,
    ///X8 - Secondary Suffix Code Indicator
    SecondarySuffixCodeIndicator,
    ///X9 - Internal Control Number
    InternalControlNumber,
    ///XA - Substitute National Stock Number
    SubstituteNationalStockNumber,
    ///XB - Substitute Manufacturer's Part Number
    SubstituteManufacturersPartNumber,
    ///XC - Cargo Control Number
    CargoControlNumber,
    ///XD - Subsistence Identification Number
    SubsistenceIdentificationNumber,
    ///XE - Transportation Priority Number
    TransportationPriorityNumber,
    ///XF - Government Bill of Lading Office Code
    GovernmentBillOfLadingOfficeCode,
    ///XG - Airline Ticket Number
    AirlineTicketNumber,
    ///XH - Contract Auditor ID Number
    ContractAuditorIdNumber,
    ///XI - Federal Home Loan Mortgage Corporation Loan Number
    FederalHomeLoanMortgageCorporationLoanNumber,
    ///XJ - Federal Home Loan Mortgage Corporation Default/Foreclosure Specialist Number
    FederalHomeLoanMortgageCorporationDefaultForeclosureSpecialistNumber,
    ///XK - Mortgagee Loan Number
    MortgageeLoanNumber,
    ///XL - Insured's Loan Number
    InsuredsLoanNumber,
    ///XM - Issuer Number
    IssuerNumber,
    ///XN - Title XIX Identifier Number
    TitleXixIdentifierNumber,
    ///XO - Sample Number
    SampleNumber,
    ///XP - Previous Cargo Control Number
    PreviousCargoControlNumber,
    ///XQ - Pier Number
    PierNumber,
    ///XR - Railroad Commission Record Number
    RailroadCommissionRecordNumber,
    ///XS - Gas Analysis Source Meter Number
    GasAnalysisSourceMeterNumber,
    ///XT - Toxicology ID
    ToxicologyId,
    ///XU - Universal Transverse Mercator - North
    UniversalTransverseMercatorNorth,
    ///XV - Universal Transverse Mercator - East
    UniversalTransverseMercatorEast,
    ///XW - Universal Transverse Mercator - Zone
    UniversalTransverseMercatorZone,
    ///XX - Rating Period
    RatingPeriod,
    ///XX1 - Special Program Code
    SpecialProgramCode,
    ///XX2 - Service Area Code
    ServiceAreaCode,
    ///XX3 - Function Code
    FunctionCode,
    ///XX4 - Object Code
    ObjectCode,
    ///XX5 - Organization Code
    OrganizationCode,
    ///XX6 - Subject Area Code
    SubjectAreaCode,
    ///XX7 - Schedule Type Code
    ScheduleTypeCode,
    ///XX8 - Alternating Schedule Identifier Code
    AlternatingScheduleIdentifierCode,
    ///XY - Other Unlisted Type of Reference Number
    OtherUnlistedTypeOfReferenceNumber,
    ///XZ - Pharmacy Prescription Number
    PharmacyPrescriptionNumber,
    ///Y0 - Debtor
    Debtor,
    ///Y1 - Claim Administrator Claim Number
    ClaimAdministratorClaimNumber,
    ///Y2 - Third-Party Administrator Claim Number
    ThirdPartyAdministratorClaimNumber,
    ///Y3 - Contract Holder Claim Number
    ContractHolderClaimNumber,
    ///Y4 - Agency Claim Number
    AgencyClaimNumber,
    ///Y5 - Delivery Trailer Manifest
    DeliveryTrailerManifest,
    ///Y6 - Sort and Segregate
    SortAndSegregate,
    ///Y8 - User ID
    UserId,
    ///Y9 - Current Certificate Number
    CurrentCertificateNumber,
    ///YA - Prior Certificate Number
    PriorCertificateNumber,
    ///YB - Revision Number
    RevisionNumber,
    ///YC - Tract
    Tract,
    ///YD - Buyer Identification
    BuyerIdentification,
    ///YE - Railroad Commission Oil Number
    RailroadCommissionOilNumber,
    ///YF - Lessee Identification
    LesseeIdentification,
    ///YH - Operator Assigned Unit Number
    OperatorAssignedUnitNumber,
    ///YI - Refiner Identification
    RefinerIdentification,
    ///YJ - Revenue Source
    RevenueSource,
    ///YK - Rent Payor Identification
    RentPayorIdentification,
    ///YL - Allowance Recipient Identification
    AllowanceRecipientIdentification,
    ///YM - Resource Screening Reference
    ResourceScreeningReference,
    ///YN - Receiver ID Qualifier
    ReceiverIdQualifier,
    ///YO - Formation
    Formation,
    ///YP - Selling Arrangement
    SellingArrangement,
    ///YQ - Minimum Royalty Payor Identification
    MinimumRoyaltyPayorIdentification,
    ///YR - Operator Lease Number
    OperatorLeaseNumber,
    ///YS - Yard Position
    YardPosition,
    ///YT - Reporter Identification
    ReporterIdentification,
    ///YV - Participating Area
    ParticipatingArea,
    ///YW - Engineering Change Proposal
    EngineeringChangeProposal,
    ///YX - Geographic Score
    GeographicScore,
    ///YY - Geographic Key
    GeographicKey,
    ///YZ - Geographic Index
    GeographicIndex,
    ///Z1 - Safety of Ship Certificate
    SafetyOfShipCertificate,
    ///Z2 - Safety of Radio Certificate
    SafetyOfRadioCertificate,
    ///Z3 - Safety Equipment Certificate
    SafetyEquipmentCertificate,
    ///Z4 - Civil Liabilities of Oil Certificate
    CivilLiabilitiesOfOilCertificate,
    ///Z5 - Load Line Certificate
    LoadLineCertificate,
    ///Z6 - Derat Certificate
    DeratCertificate,
    ///Z7 - Maritime Declaration of Health
    MaritimeDeclarationOfHealth,
    ///Z8 - Federal Housing Administration Case Number
    FederalHousingAdministrationCaseNumber,
    ///Z9 - Veterans Affairs Case Number
    VeteransAffairsCaseNumber,
    ///ZA - Supplier
    Supplier,
    ///ZB - Ultimate Consignee
    UltimateConsignee,
    ///ZC - Connecting Carrier
    ConnectingCarrier,
    ///ZD - Family Member Identification
    FamilyMemberIdentification,
    ///ZE - Coal Authority Number
    CoalAuthorityNumber,
    ///ZG - Sales Representative Order Number
    SalesRepresentativeOrderNumber,
    ///ZH - Carrier Assigned Reference Number
    CarrierAssignedReferenceNumber,
    ///ZI - Reference Version Number
    ReferenceVersionNumber,
    ///ZJ - Universal Railroad Revenue Waybill Identified Number (URRWIN)
    CodeZJ,
    ///ZK - Duplicate Waybill in Route
    DuplicateWaybillInRoute,
    ///ZL - Duplicate Waybill Not in Route
    DuplicateWaybillNotInRoute,
    ///ZM - Manufacturer Number
    ManufacturerNumber,
    ///ZN - Agency Case Number
    AgencyCaseNumber,
    ///ZO - Makegood Commercial Line Number
    MakegoodCommercialLineNumber,
    ///ZP - Spouse Tie
    SpouseTie,
    ///ZQ - Non-Spouse Tie
    NonSpouseTie,
    ///ZR - Supplier (Replacement)
    CodeZR,
    ///ZS - Software Application Number
    SoftwareApplicationNumber,
    ///ZT - Milling in Transit
    MillingInTransit,
    ///ZTS - Zone, Track, Spot Number (ZTS)
    CodeZTS,
    ///ZU - Field
    Field,
    ///ZV - Block
    Block,
    ///ZW - Area
    Area,
    ///ZX - County Code
    CountyCode,
    ///ZY - Referenced Pattern Identification
    ReferencedPatternIdentification,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl ReferenceIdentificationQualifier {
    pub fn code(&self) -> &str {
        {
            use ReferenceIdentificationQualifier::*;
            match self {
                ContractingDistrictNumber => "00",
                SupervisoryAppraiserCertificationNumber => "0A",
                StateLicenseNumber => "0B",
                SubjectPropertyVerificationSource => "0D",
                SubjectPropertyReferenceNumber => "0E",
                SubscriberNumber => "0F",
                ReviewerFileNumber => "0G",
                ComparablePropertyPendingSaleReferenceNumber => "0H",
                ComparablePropertySaleReferenceNumber => "0I",
                SubjectPropertyNonSaleReferenceNumber => "0J",
                PolicyFormIdentifyingNumber => "0K",
                ReferencedBy => "0L",
                MortgageIdentificationNumber => "0M",
                AttachedTo => "0N",
                RealEstateOwnedProperty => "0P",
                Code01 => "01",
                BlueCrossProviderNumber => "1A",
                CatalogOfFederalDomesticAssistance => "01A",
                BlueShieldProviderNumber => "1B",
                UnionAgreement => "01B",
                MedicareProviderNumber => "1C",
                Code01C => "01C",
                MedicaidProviderNumber => "1D",
                Code01D => "01D",
                DentistLicenseNumber => "1E",
                Code01E => "01E",
                AnesthesiaLicenseNumber => "1F",
                ProviderUpinNumber => "1G",
                PaymentRelatedClause => "01G",
                ChampusIdentificationNumber => "1H",
                SpecialPriceAuthorizationNumber => "01H",
                Code1I => "1I",
                FacilityIdNumber => "1J",
                PayorsClaimNumber => "1K",
                GroupOrPolicyNumber => "1L",
                PreferredProviderOrganizationSiteNumber => "1M",
                Code1N => "1N",
                ConsolidationShipmentNumber => "1O",
                AccessorialStatusCode => "1P",
                ErrorIdentificationCode => "1Q",
                StorageInformationCode => "1R",
                Code1S => "1S",
                Code1T => "1T",
                PayGrade => "1U",
                RelatedVendorOrderNumber => "1V",
                MemberIdentificationNumber => "1W",
                CreditOrDebitAdjustmentNumber => "1X",
                RepairActionNumber => "1Y",
                FinancialDetailCode => "1Z",
                Code02 => "02",
                ImportLicenseNumber => "2A",
                TerminalReleaseOrderNumber => "2B",
                LongTermDisabilityPolicyNumber => "2C",
                Code2D => "2D",
                ForeignMilitarySalesCaseNumber => "2E",
                ConsolidatedInvoiceNumber => "2F",
                Amendment => "2G",
                AssignedByTransactionSetSender => "2H",
                TrackingNumber => "2I",
                FloorNumber => "2J",
                Code2K => "2K",
                Code2L => "2L",
                Code2M => "2M",
                Code2N => "2N",
                Code2O => "2O",
                Subdivision => "2P",
                Code2Q => "2Q",
                CouponRedemptionNumber => "2R",
                Catalog => "2S",
                SubSubhouseBillOfLading => "2T",
                PayerIdentificationNumber => "2U",
                Code2V => "2V",
                ChangeOrderAuthority => "2W",
                SupplementalAgreementAuthority => "2X",
                WageDetermination => "2Y",
                Code2Z => "2Z",
                Code03 => "03",
                SectionOfTheNationalHousingActCode => "3A",
                SupplementalClaimNumber => "3B",
                PayeeLoanNumber => "3C",
                ServicerLoanNumber => "3D",
                InvestorLoanNumber => "3E",
                ShowIdentification => "3F",
                CatastropheNumber => "3G",
                CaseNumber => "3H",
                PrecinctNumber => "3I",
                OfficeNumber => "3J",
                PetroleumPoolCode => "3K",
                Branch => "3L",
                Code3M => "3M",
                GasCustodianIdentification => "3N",
                Code3O => "3O",
                ThirdPartyOriginatorNumber => "3P",
                Code3Q => "3Q",
                Code3R => "3R",
                Code3S => "3S",
                Code3T => "3T",
                ProtractionNumber => "3U",
                FormationIdentifier => "3V",
                Code3W => "3W",
                SubcontractNumber => "3X",
                ReceiverAssignedDropZone => "3Y",
                CustomsBrokerReferenceNumber => "3Z",
                CanadianFinancialInstitutionBranchAndInstitutionNumber => "04",
                Code4A => "4A",
                ShipmentOriginCode => "4B",
                ShipmentDestinationCode => "4C",
                ShippingZone => "4D",
                CarrierAssignedConsigneeNumber => "4E",
                CarrierAssignedShipperNumber => "4F",
                ProvincialTaxIdentification => "4G",
                CommercialInvoiceNumber => "4H",
                BalanceDueReferenceNumber => "4I",
                VehicleRelatedServicesReferenceNumber => "4J",
                AccessorialRailDiversionReferenceNumber => "4K",
                LocationSpecificServicesReferenceNumber => "4L",
                SpecialMoveReferenceNumber => "4M",
                SpecialPaymentReferenceNumber => "4N",
                Code4O => "4O",
                AffiliationNumber => "4P",
                CallSign => "4Q",
                RuleSection => "4R",
                PreferredCallSign => "4S",
                Code4T => "4T",
                MarketArea => "4U",
                EmissionDesignator => "4V",
                Study => "4W",
                Log => "4X",
                SubhouseBillOfLading => "4Y",
                Code4Z => "4Z",
                Code05 => "05",
                OffenseTracking => "5A",
                SupplementalAccountNumber => "5B",
                CongressionalDistrict => "5C",
                LineOfCreditCategory => "5D",
                Consumer => "5E",
                Warrant => "5F",
                Complaint => "5G",
                Incident => "5H",
                OffenderTracking => "5I",
                DriversLicense => "5J",
                CommercialDriversLicense => "5K",
                JurisdictionalCommunityNumber => "5L",
                PreviousSequence => "5M",
                CitationOfStatute => "5N",
                CitationOfOpinion => "5O",
                NationalCriminalInformationCenterOriginatingAgencyIdentification => "5P",
                StateCriminalHistoryRepositoryIndividualIdentification => "5Q",
                FederalBureauOfInvestigationIndividualIdentification => "5R",
                ProcessingArea => "5S",
                PaymentLocation => "5T",
                FloodData => "5U",
                CouponDistributionMethod => "5V",
                OriginalUniformCommercialCodeFilingNumber => "5W",
                AmendedUniformCommercialCodeFilingNumber => "5X",
                ContinuationUniformCommercialCodeFilingNumber => "5Y",
                UniformCommercialCodeFilingCollateralNumber => "5Z",
                SystemNumber => "06",
                ConsigneeReferenceNumber => "6A",
                Code6B => "6B",
                Code6C => "6C",
                Code6D => "6D",
                MapReference => "6E",
                AppraiserLicense => "6F",
                MapNumber => "6G",
                ComparablePropertyVerificationSource => "6H",
                ComparableProperty => "6I",
                CensusTract => "6J",
                Zone => "6K",
                AgentContractNumber => "6L",
                ApplicationNumber => "6M",
                ClaimantNumber => "6N",
                CrossReferenceNumber => "6O",
                GroupNumber => "6P",
                InsuranceLicenseNumber => "6Q",
                ProviderControlNumber => "6R",
                ProviderOrderTicketNumber => "6S",
                PilotLicenseNumber => "6T",
                QuestionNumber => "6U",
                ReissueCessionNumber => "6V",
                Specimen => "6X",
                EquipmentInitial => "6Y",
                Code6Z => "6Z",
                AddOnSystemNumber => "07",
                PurchaseOrderNumberIncludedInOnOrderPosition => "7A",
                PurchaseOrderNumberOfShipmentReceivedSinceLastReportingDate => "7B",
                PurchaseOrderNumberOfOrderReceivedSinceLastReportingDate => "7C",
                TesterIdentification => "7D",
                CollectorIdentification => "7E",
                RepeatLocation => "7F",
                DataQualityRejectReason => "7G",
                Code7H => "7H",
                SubscriberAuthorizationNumber => "7I",
                TollBillingTelephoneReferenceNumber => "7J",
                ListOfMaterials => "7K",
                QualifiedMaterialsList => "7L",
                Frame => "7M",
                Piggyback => "7N",
                Tripleback => "7O",
                Sheet => "7P",
                EngineeringChangeOrder => "7Q",
                RepresentativeIdentificationNumber => "7R",
                DrawingType => "7S",
                MasterContract => "7T",
                RelatedTransactionReferenceNumber => "7U",
                InterchangeTrainIdentification => "7W",
                Code7X => "7X",
                Code7Y => "7Y",
                Code7Z => "7Z",
                CarrierAssignedPackageIdentificationNumber => "08",
                Code8A => "8A",
                Code8B => "8B",
                Code8C => "8C",
                ChemicalAbstractServiceRegistryNumber => "8D",
                GuarantorLoanNumber => "8E",
                SchoolLoanNumber => "8F",
                Code8G => "8G",
                CheckListNumber => "8H",
                FedwireConfirmationNumber => "8I",
                Code8J => "8J",
                DominionOfCanadaCode => "8K",
                Code8L => "8L",
                OriginatingCompany => "8M",
                ReceivingCompany => "8N",
                Code8O => "8O",
                OriginatingDepositoryFinancialInstitution => "8P",
                ReceivingDepositoryFinancialInstitution => "8Q",
                SecurityType => "8R",
                BrokerIdentification => "8S",
                BankAssignedSecurity => "8U",
                CreditReference => "8V",
                BankToBank => "8W",
                TransactionCategoryOrType => "8X",
                SafekeepingAccountNumber => "8Y",
                AlternateClauseNumber => "8Z",
                CustomsBarCodeNumber => "09",
                RepricedClaimReferenceNumber => "9A",
                RepricedLineItemReferenceNumber => "9B",
                AdjustedRepricedClaimReferenceNumber => "9C",
                AdjustedRepricedLineItemReferenceNumber => "9D",
                ReplacementClaimNumber => "9E",
                ReferralNumber => "9F",
                DepartmentOfDefenseForm250RequirementCode => "9G",
                PackagingGroupNumber => "9H",
                Code9I => "9I",
                PensionContract => "9J",
                Servicer => "9K",
                ServiceBureau => "9L",
                Code9M => "9M",
                Investor => "9N",
                LoanType => "9P",
                PoolSuffix => "9Q",
                JobOrderNumber => "9R",
                DeliveryRegion => "9S",
                Tenor => "9T",
                LoanFeatureCode => "9U",
                PaymentCategory => "9V",
                PayerCategory => "9W",
                AccountCategory => "9X",
                BankAssignedBankersReferenceNumber => "9Y",
                ChamberOfCommerceNumber => "9Z",
                AccountManagersCode => "10",
                AccountNumber => "11",
                BillingAccount => "12",
                HorizontalCoordinate => "13",
                MasterAccountNumber => "14",
                VerticalCoordinate => "15",
                Code16 => "16",
                ClientReportingCategory => "17",
                PlanNumber => "18",
                Division => "19",
                RepairPartNumber => "20",
                AmericanGasAssociationEquationNumber => "21",
                SpecialChargeOrAllowanceCode => "22",
                ClientNumber => "23",
                ShortTermDisabilityPolicyNumber => "24",
                ReasonNotLowestCostCode => "25",
                UnionNumber => "26",
                InsurorPoolIdentificationNumber => "27",
                EmployeeIdentificationNumber => "28",
                ForeclosureAccountNumber => "29",
                UnitedStatesGovernmentVisaNumber => "30",
                DocketNumber => "31",
                CreditRepositoryCode => "32",
                LenderCaseNumber => "33",
                LoanRequestNumber => "34",
                MultifamilyProjectNumber => "35",
                UnderwriterIdentificationNumber => "36",
                CondominiumIdentificationNumber => "37",
                MasterPolicyNumber => "38",
                ProposalNumber => "39",
                LeaseScheduleNumberReplacement => "40",
                LeaseScheduleNumberPrior => "41",
                PhoneCalls => "42",
                SupportingDocumentNumber => "43",
                EndUseNumber => "44",
                OldAccountNumber => "45",
                OldMeterNumber => "46",
                PlateNumber => "47",
                AgencysStudentNumber => "48",
                FamilyUnitNumber => "49",
                StateStudentIdentificationNumber => "50",
                PictureNumber => "51",
                Code52 => "52",
                Code53 => "53",
                Code54 => "54",
                SequenceNumber => "55",
                CorrectedSocialSecurityNumber => "56",
                PriorIncorrectSocialSecurityNumber => "57",
                CorrectedBatchNumber => "58",
                PriorIncorrectBatchNumber => "59",
                AccountSuffixCode => "60",
                TaxingAuthorityIdentificationNumber => "61",
                PriorLoanNumber => "63",
                JurisdictionalCommunityName => "64",
                TotalOrderCycleNumber => "65",
                PreviousPolicyNumber => "66",
                PreviousClaimHistory => "67",
                DentalInsuranceAccountNumber => "68",
                DentalInsurancePolicyNumber => "69",
                CalendarNumber => "70",
                Code71 => "71",
                ScheduleReferenceNumber => "72",
                Code73 => "73",
                Code74 => "74",
                OrganizationBreakdownStructure => "75",
                Milestone => "76",
                WorkPackage => "77",
                PlanningPackage => "78",
                CostAccount => "79",
                ChargeNumber => "80",
                Code81 => "81",
                Code82 => "82",
                Code83 => "83",
                Code84 => "84",
                Code85 => "85",
                OperationNumber => "86",
                FunctionalCategory => "87",
                WorkCenter => "88",
                AssemblyNumber => "89",
                SubassemblyNumber => "90",
                CostElement => "91",
                ChangeDocumentNumber => "92",
                FundsAuthorization => "93",
                FileIdentificationNumber => "94",
                Code95 => "95",
                StockCertificateNumber => "96",
                PackageNumber => "97",
                ContainerPackagingSpecificationNumber => "98",
                RateConferenceIdCode => "99",
                AdvertiserNumber => "A0",
                AnalysisNumberTestNumber => "A1",
                DisabilityInsuranceAccountNumber => "A2",
                AssignmentNumber => "A3",
                DisabilityInsurancePolicyNumber => "A4",
                EducationalInstitutionIdentificationNumber => "A5",
                CodeA7 => "A7",
                CodeA8 => "A8",
                HealthInsuranceAccountNumber => "A9",
                AccountsReceivableStatementNumber => "AA",
                DistributorsSplitAgentNumber => "AAA",
                FundManagersReferenceNumber => "AAB",
                AgencyHierarchicalLevel => "AAC",
                OfficerLicenseNumber => "AAD",
                PreviousDistributorNumber => "AAE",
                InterviewerId => "AAF",
                MilitaryId => "AAG",
                OptionPolicyNumber => "AAH",
                PayrollAccountNumber => "AAI",
                PriorContractNumber => "AAJ",
                WorksiteNumber => "AAK",
                AgentNumber => "AAL",
                Treaty => "AAM",
                AssociatedCaseControlNumber => "AAN",
                CarrierAssignedCode => "AAO",
                DealerNumber => "AAP",
                DirectoryNumber => "AAQ",
                DistributorAssignedTransactionNumber => "AAR",
                DistributorAssignedOrderNumber => "AAS",
                DistributorsAccountNumber => "AAT",
                GeneralAgencyNumber => "AAU",
                LaboratoryNumber => "AAV",
                AgencyAssignedNumber => "AAW",
                ListBillNumber => "AAX",
                AccountingPeriodReference => "AAY",
                ParamedicalIdNumber => "AAZ",
                AcceptableSourcePurchaserId => "AB",
                PayrollNumber => "ABA",
                PersonalIdNumber => "ABB",
                PolicyLinkNumber => "ABC",
                SecondaryPolicyNumber => "ABD",
                SpecialQuoteNumber => "ABE",
                NationalPropertyRegistrySystemLevel1 => "ABF",
                NationalPropertyRegistrySystemLevel2 => "ABG",
                InvestorAssignedIdentificationNumber => "ABH",
                MotorFuelCertificateNumber => "ABI",
                CodeABJ => "ABJ",
                MortgageElectronicRegistrationSystemOrganization => "ABK",
                SellerLoanNumber => "ABL",
                SubServicerLoanNumber => "ABM",
                NationalPropertyRegistrySystemLevel3 => "ABN",
                StateHazardousWasteEntity => "ABO",
                BankruptcyProcedureNumber => "ABP",
                NationalBusinessIdentificationNumber => "ABQ",
                CodeABR => "ABR",
                VesselName => "ABS",
                SecurityInstrumentNumber => "ABT",
                AssignmentRecordingNumber => "ABU",
                BookNumber => "ABV",
                BusinessTaxNumber => "ABW",
                NorthAmericanIndustrialClassificationSystemCode2 => "ABX",
                CentersForMedicareAndMedicaidServicesPlanId => "ABY",
                EmploymentVisa => "ABZ",
                AirCargoTransferManifest => "AC",
                GrowthFactorReference => "ACA",
                Region => "ACB",
                Status => "ACC",
                ClassCode => "ACD",
                ServiceRequestNumber => "ACE",
                SupplementNumber => "ACF",
                PreviousTicketNumber => "ACG",
                OneCallAgencyTicketNumber => "ACH",
                TicketNumber => "ACI",
                BillOfMaterialRevisionNumber => "ACJ",
                DrawingRevisionNumber => "ACK",
                ApplicationTransactionReferenceNumber => "ACL",
                RelatedObjectIdentificationNumber => "ACM",
                CommonAccessReferenceNumber => "ACN",
                FirstTransferNumber => "ACO",
                ContinuousTransferNumber => "ACP",
                LastTransferNumber => "ACQ",
                CodeACR => "ACR",
                SocietyOfPropertyInformationCompilersAndAnalysts => "ACS",
                AccountingCode => "ACT",
                GreenCard => "ACU",
                AgencyAssignedEmployeeId => "ACV",
                Passport => "ACW",
                UnemploymentInsuranceNumber => "ACX",
                NorthAmericanIndustrialClassificationSystemCode1 => "ACY",
                OccupationCode => "ACZ",
                AcceptableSourceDunsNumber => "AD",
                CodeADA => "ADA",
                MasterPropertyNumber => "ADB",
                ProjectPropertyNumber => "ADC",
                UnitPropertyNumber => "ADD",
                AssociatedPropertyNumber => "ADE",
                AssociatedNumberForLimitedCommonElementParking => "ADF",
                AssociatedNumberForUnitParking => "ADG",
                AssociatedNumberForJoinedUnitNotReSubdivided => "ADH",
                ProcessorIdentificationNumber => "ADI",
                OccupationClassificationCode => "ADJ",
                EmployeeTaxFilingStatusCode => "ADK",
                InsuredLocation => "ADL",
                AirDimensionCode => "ADM",
                SelfInsuranceIdentificationNumber => "ADN",
                SelfInsurerOrganizationType => "ADO",
                SelfInsurerAuthorizationTypeCode => "ADP",
                CountyBusinessRegistrationNumber => "ADQ",
                PostalTemplate => "ADR",
                ReducedEarningWeek => "ADS",
                FullDenialReason => "ADT",
                FederalEnergyRegulatoryCommissionCertificateOfPublicConvenience => "ADU",
                Suspension => "ADV",
                ManagedCareOrganizationCode => "ADW",
                ManagedCareOrganizationIdentificationNumber => "ADX",
                PublicUtilitiesCommissionCertificateOfPublicConvenience => "ADY",
                RetailMerchantsCertificationNumber => "ADZ",
                CodeAE => "AE",
                CodeAEA => "AEA",
                CodeAEB => "AEB",
                GovernmentRegistrationNumber => "AEC",
                JudicialNumber => "AED",
                CodeAEE => "AEE",
                PassportNumber => "AEF",
                PatronNumber => "AEG",
                CodeAEH => "AEH",
                CodeAEI => "AEI",
                CodeAEJ => "AEJ",
                TokyoShokoResearchBusiness => "AEK",
                CodeAEL => "AEL",
                DistributionCenterNumber => "AEM",
                CodeAEN => "AEN",
                PublicDeedNumber => "AEO",
                StockExchangeCode => "AEP",
                SecretaryOfStateAssignedIdentificationNumber => "AEQ",
                DepartmentWhereInjuryOccurredIdentification => "AER",
                BureauOfLaborAndStatisticsSchedule => "AES",
                StateCharterNumber => "AET",
                EmployeeNonEmployeeClassificationQualifier => "AEU",
                FullTimePartTimeEmployeeClassificationQualifier => "AEV",
                PremiumAuditPriority => "AEX",
                PremiumAuditPurpose => "AEY",
                PremiumAuditType => "AEZ",
                AirlinesFlightIdentificationNumber => "AF",
                SplitPremiumAuditChange => "AFA",
                SublineOfInsurance => "AFB",
                VerificationSourceCode => "AFC",
                UnderwritingAlertReferenceCode => "AFD",
                CommercialPrivatePassengerVehicleQualifier => "AFE",
                VehicleBusinessUseQualifier => "AFF",
                VehicleSizeClassQualifier => "AFG",
                VehicleRadiusOfOperationQualifier => "AFH",
                TrailerTypeQualifier => "AFI",
                StateSalesTaxIdentificationNumber => "AFJ",
                CardIssuerTransactionCode => "AFK",
                CardBillingTypeCode => "AFL",
                ClientCompanyCode => "AFM",
                CodeAFN => "AFN",
                CardAccountTypeCode => "AFO",
                CardAccountStatusCode => "AFP",
                CardAccountReportingLevel => "AFQ",
                CardAccountReporting => "AFR",
                CodeAFS => "AFS",
                FeeSchedule => "AFT",
                CodeAFU => "AFU",
                StateControlledSubstanceLicenseNumber => "AFV",
                PointOfOrigination => "AFW",
                PointOfDestination => "AFX",
                AssessmentNumber => "AFY",
                CertificateNumber => "AFZ",
                AgentsShipmentNumber => "AG",
                StateOrProvinceAssignedBusinessRegistryNumber => "AGA",
                MunicipalityAssignedBusinessRegistryNumber => "AGB",
                CodeAGC => "AGC",
                CodeAGD => "AGD",
                LenderUse => "AGH",
                GuarantorUse => "AGI",
                SchoolUse => "AGJ",
                ReservationSystemCode => "AGK",
                OrderOriginationCode => "AGL",
                FolioNumber => "AGM",
                CorporateIdentificationCode => "AGN",
                CodeAGO => "AGO",
                ConjunctionTravelTicket => "AGP",
                ListTracking => "AGQ",
                AgreementNumber => "AH",
                AirHandlingCode => "AHC",
                AssociatedInvoices => "AI",
                AccountsReceivableCustomerAccount => "AJ",
                CodeAK => "AK",
                CodeAL => "AL",
                AgencyLocationCode => "ALC",
                TitleCompanyCodeBookReference => "ALG",
                TitleDocumentSchedule => "ALH",
                RecordingNumber => "ALI",
                TitlePolicyNumber => "ALJ",
                AlienRegistrationNumber => "ALR",
                AlternativeListId => "ALS",
                AlterationNumber => "ALT",
                CodeAM => "AM",
                AssociatedPurchaseOrders => "AN",
                AppointmentNumber => "AO",
                AccountsReceivableNumber => "AP",
                AmbulatoryPaymentClassification => "APC",
                CodeAPI => "API",
                AccessCode => "AQ",
                ArrivalCode => "AR",
                AcceptableSourceSupplierId => "AS",
                CodeASL => "ASL",
                AnimalSpecies => "ASP",
                AnimalStrain => "AST",
                AppropriationNumber => "AT",
                MaintenanceAvailabilityType => "ATC",
                AuthorizationToMeetCompetitionNumber => "AU",
                HealthInsuranceRatingAccountNumber => "AV",
                AirWaybillNumber => "AW",
                CodeAX => "AX",
                FloorPlanApprovalNumber => "AY",
                HealthInsurancePolicyNumber => "AZ",
                LesseeBillCodeNumber => "B1",
                AxleRatio => "B2",
                PreferredProviderOrganizationNumber => "B3",
                BilateralCarServiceAgreements => "B4",
                HealthInsuranceRatingSuffixCode => "B5",
                LifeInsuranceBillingAccountNumber => "B6",
                LifeInsurancePolicyNumber => "B7",
                LifeInsuranceBillingSuffixCode => "B8",
                RetirementPlanAccountNumber => "B9",
                RetirementPlanPolicyNumber => "BA",
                FranchiseTaxAccountNumber => "BAA",
                CertificateOfIncorporationNumber => "BAB",
                BeamAssemblyCode => "BAC",
                StateTaxIdentificationNumber => "BAD",
                CharterNumber => "BAE",
                ReceiptNumber => "BAF",
                WithdrawalAccountNumber => "BAG",
                DepositAccountNumber => "BAH",
                BusinessIdentificationNumber => "BAI",
                CodeBAJ => "BAJ",
                CodeBAK => "BAK",
                AuthorizationNumber => "BB",
                BuyersContractNumber => "BC",
                BasicContractLineItemNumber => "BCI",
                BirthCertificateNumber => "BCN",
                BorderCrossingPermitNumber => "BCP",
                BidNumber => "BD",
                BadgeNumber => "BDG",
                BuildDirectiveNumber => "BDN",
                BusinessActivity => "BE",
                BrokerEntryNumber => "BEN",
                BillingCenterIdentification => "BF",
                BeginningSerialNumber => "BG",
                LeaseScheduleNumberBlanket => "BH",
                BondedCarrierInternalRevenueServiceIdentificationNumber => "BI",
                CarriersCustomsBondNumber => "BJ",
                BrokersOrderNumber => "BK",
                BankTelegraphicNumber => "BKT",
                GovernmentBillOfLading => "BL",
                BillingType => "BLT",
                BillOfLadingNumber => "BM",
                BeginMileMarker => "BMM",
                BookingNumber => "BN",
                BinLocationNumber => "BO",
                BinaryObject => "BOI",
                AdjustmentControlNumber => "BP",
                HealthMaintenanceOrganizationCodeNumber => "BQ",
                BrokerOrSalesOfficeNumber => "BR",
                SplitBookingNumber => "BS",
                BatchNumber => "BT",
                BuyersApprovalMark => "BU",
                CodeBV => "BV",
                BlendedWithBatchNumber => "BW",
                BuyersShipmentMarkNumber => "BX",
                RepairCategoryNumber => "BY",
                ComplaintCode => "BZ",
                CanadianSocialInsuranceNumber => "C0",
                CustomerMaterialSpecificationNumber => "C1",
                CustomerProcessSpecificationNumber => "C2",
                CustomerSpecificationNumber => "C3",
                ChangeNumber => "C4",
                CustomerTrackingNumberForLoanedMaterials => "C5",
                CarnetNumber => "C6",
                ContractLineItemNumber => "C7",
                CorrectedContractNumber => "C8",
                PreviousCreditDebitAdjustmentNumber => "C9",
                CostAllocationReference => "CA",
                AccidentHistory => "CAA",
                Chemical => "CAB",
                DischargePointIdentification => "CAC",
                EmissionUnitIdentificationNumber => "CAD",
                FacilityFederalIdentificationNumber => "CAE",
                LatitudeExpressedInDecimalDegrees => "CAF",
                LongitudeExpressedInDecimalDegrees => "CAG",
                CodeCAH => "CAH",
                Process => "CAI",
                StackIdentificationNumber => "CAJ",
                FacilityStateIdentificationNumber => "CAK",
                CodeCAL => "CAL",
                CodeCAM => "CAM",
                Category => "CAT",
                CombinedShipment => "CB",
                CensusBlockGroup => "CBG",
                ContractCoOpNumber => "CC",
                CreditNoteNumber => "CD",
                CitizenshipDocumentNumber => "CDN",
                ContractingDistrictTypeCode => "CDT",
                ClassOfContractCode => "CE",
                FleetReferenceNumber => "CF",
                FederalRegulation => "CFR",
                ConsigneesOrderNumber => "CG",
                CustomerCatalogNumber => "CH",
                Chromatograph => "CHR",
                UniqueConsignment => "CI",
                CampusIdentificationNumber => "CID",
                CircuitNumber => "CIR",
                Citation => "CIT",
                ClauseNumber => "CJ",
                CheckNumber => "CK",
                SellersCreditMemo => "CL",
                CoverageListId => "CLI",
                BuyersCreditMemo => "CM",
                ContinuousMoveNumber => "CMN",
                CustomerMaintenancePeriodSequenceNumber => "CMP",
                Component => "CMT",
                CodeCN => "CN",
                AssemblyControlNumber => "CNA",
                CommitmentNumber => "CNO",
                CanadianNationalStudentNumber => "CNS",
                CustomerOrderNumber => "CO",
                CollocationIndicator => "COL",
                CertificateOfTransportation => "COT",
                ConditionOfPurchaseDocumentNumber => "CP",
                CanadianProvinceOperatingAuthorityNumber => "CPA",
                DiscrepantContainerPackagingNumber => "CPD",
                RequiredContainerPackagingNumber => "CPR",
                CurrentProceduralTerminologyCode => "CPT",
                CustomshouseBrokerLicenseNumber => "CQ",
                CustomerReferenceNumber => "CR",
                CasualtyReportNumber => "CRN",
                CasualtyReportSerialNumber => "CRS",
                ConditionOfSaleDocumentNumber => "CS",
                Cs54KeyTrainIndicatorCode => "CSC",
                Cs54KeyTrainIndicatorGroupName => "CSG",
                CensusStateCode => "CST",
                ContractNumber => "CT",
                CensusTractSuffix => "CTS",
                ClearTextClause => "CU",
                CodeCUB => "CUB",
                CoilNumber => "CV",
                CommercialVehicleSafetyAssuranceNumber => "CVS",
                CanadianWheatBoardPermitNumber => "CW",
                ConsignmentClassificationId => "CX",
                CommercialRegistrationNumber => "CY",
                PeriodicityCode => "CYC",
                CodeCZ => "CZ",
                DataReliabilityCode => "D0",
                DrugEnforcementAdministrationOrderBlankNumber => "D1",
                SupplierDocumentIdentificationNumber => "D2",
                NationalCouncilForPrescriptionDrugProgramsPharmacyNumber => "D3",
                CutNumber => "D4",
                DyeLotNumber => "D5",
                DuplicateBillNumber => "D6",
                CoverageCode => "D7",
                LossReportNumber => "D8",
                ClaimNumber => "D9",
                DomicileBranchNumber => "DA",
                DistrictAssignedId => "DAI",
                DeliveryAppointmentNumber => "DAN",
                BuyersDebitMemo => "DB",
                DealerPurchaseOrderNumber => "DC",
                DocumentIdentificationCode => "DD",
                DepositorNumber => "DE",
                CodeDF => "DF",
                DrawingNumber => "DG",
                DrugEnforcementAdministrationNumber => "DH",
                CodeDHH => "DHH",
                DistributorInvoiceNumber => "DI",
                DistrictNumber => "DIS",
                DeliveryTicketNumber => "DJ",
                DockNumber => "DK",
                SellersDebitMemo => "DL",
                AssociatedProductNumber => "DM",
                DraftNumber => "DN",
                DepositNumber => "DNR",
                CodeDNS => "DNS",
                DeliveryOrderNumber => "DO",
                CodeDOA => "DOA",
                CodeDOC => "DOC",
                CodeDOE => "DOE",
                CodeDOI => "DOI",
                CodeDOJ => "DOJ",
                CodeDOL => "DOL",
                DensityOrderNumber => "DON",
                CodeDOS => "DOS",
                CodeDOT => "DOT",
                DepartmentNumber => "DP",
                DeliveryQuoteNumber => "DQ",
                DockReceiptNumber => "DR",
                DrainholeNumber => "DRN",
                CodeDS => "DS",
                DepartureFromSpecificationClassCode => "DSC",
                DepartureFromSpecificationNumber => "DSI",
                DepartureFromSpecificationTypeCode => "DST",
                DownstreamShipperContractNumber => "DT",
                CodeDTS => "DTS",
                Dependents => "DU",
                CodeDUN => "DUN",
                DiversionAuthorityNumber => "DV",
                DepositSequenceNumber => "DW",
                DepartmentAgencyNumber => "DX",
                CodeDY => "DY",
                CodeDZ => "DZ",
                CourseSectionNumber => "E00",
                EmergencyOrderNumber => "E1",
                NonTeachingCredentialFieldCodes => "E01",
                PartCausingRepairNumber => "E2",
                CodeE02 => "E02",
                ExpansionOnEffectOfChangeNumber => "E3",
                ChargeCardNumber => "E4",
                ClaimantsClaimNumber => "E5",
                BackoutProcedureCode => "E6",
                ServiceBulletinNumber => "E7",
                CodeE8 => "E8",
                AttachmentCode => "E9",
                MedicalRecordIdentificationNumber => "EA",
                EmbargoPermitNumber => "EB",
                Circular => "EC",
                Fund => "ECA",
                Ballot => "ECB",
                LegislativeIdentificationNumber => "ECC",
                LobbiedActivity => "ECD",
                PetitionNumber => "ECE",
                RelatedFormNumber => "ECF",
                CodeECJ => "ECJ",
                ExportDeclaration => "ED",
                CodeEDA => "EDA",
                ElectionDistrict => "EE",
                ElectronicFundsTransferIdNumber => "EF",
                EndingSerialNumber => "EG",
                FinancialClassificationCode => "EH",
                EmployersIdentificationNumber => "EI",
                CodeEII => "EII",
                PatientAccountNumber => "EJ",
                CodeEK => "EK",
                ElectronicDevicePinNumber => "EL",
                ElectronicPaymentReferenceNumber => "EM",
                EndMileMarker => "EMM",
                EmbargoNumber => "EN",
                EndorsementNumber => "END",
                SubmitterIdentificationNumber => "EO",
                ExportPermitNumber => "EP",
                CodeEPA => "EPA",
                EnvironmentalProtectionAgencyTransporterIdentificationNumber => "EPB",
                EmployerPayrollCodeLists => "EPC",
                EquipmentNumber => "EQ",
                ContainerOrEquipmentReceiptNumber => "ER",
                EmployersSocialSecurityNumber => "ES",
                EstimateSequenceNumber => "ESN",
                ExcessTransportation => "ET",
                EndUsersPurchaseOrderNumber => "EU",
                ReceiverIdentificationNumber => "EV",
                EventIdentification => "EVI",
                MammographyCertificationNumber => "EW",
                EstimateNumber => "EX",
                ExposureStateCode => "EXP",
                ReceiverSubIdentificationNumber => "EY",
                ElectronicDataInterchangeAgreementNumber => "EZ",
                VersionCodeNational => "F1",
                VersionCodeLocal => "F2",
                SubmissionNumber => "F3",
                FacilityCertificationNumber => "F4",
                MedicareVersionCode => "F5",
                CodeF6 => "F6",
                CodeF7 => "F7",
                OriginalReferenceNumber => "F8",
                FreightPayorReferenceNumber => "F9",
                CodeFA => "FA",
                FannieMaeSellerServicerNumber => "FAN",
                FileTransferFormNumber => "FB",
                FilerCodeIssuedByCustoms => "FC",
                AssignedContractNumber => "FCN",
                FilerCodeIssuedByBureauOfCensus => "FD",
                FailureMechanismNumber => "FE",
                ForeignEntryNumber => "FEN",
                FilmNumber => "FF",
                FundIdentificationNumber => "FG",
                ClinicNumber => "FH",
                CodeFHC => "FHC",
                FederalHousingAdministrationOriginatorIdentification => "FHO",
                File => "FI",
                LineItemControlNumber => "FJ",
                FinishLotNumber => "FK",
                FineLineClassification => "FL",
                FloodZone => "FLZ",
                CodeFM => "FM",
                CodeFMG => "FMG",
                FacilityMeasurementPointNumber => "FMP",
                ForwardersAgentsReferenceNumber => "FN",
                FinderNumber => "FND",
                DrugFormularyNumber => "FO",
                ForestryPermitNumber => "FP",
                FormNumber => "FQ",
                FreightBillNumber => "FR",
                FreddieMacSellerServicerNumber => "FRN",
                FinalSequenceNumber => "FS",
                FundSourceCode => "FSC",
                AssignedSequenceNumber => "FSN",
                ForeignTradeZone => "FT",
                PremarketNotificationNumber => "FTN",
                CodeFTP => "FTP",
                CodeFTZ => "FTZ",
                FundCode => "FU",
                CodeFV => "FV",
                StateLicenseIdentificationNumber => "FW",
                FinalWorkCandidateNumber => "FWC",
                FailureAnalysisReportNumber => "FX",
                ClaimOfficeNumber => "FY",
                ProcessorsInvoiceNumber => "FZ",
                PriorAuthorizationNumber => "G1",
                ProviderCommercialNumber => "G2",
                PredeterminationOfBenefitsIdentificationNumber => "G3",
                CodeG4 => "G4",
                ProviderSiteNumber => "G5",
                PayerAssignedResubmissionReferenceNumber => "G6",
                ResubmissionReasonCode => "G7",
                ResubmissionNumber => "G8",
                SecondaryEmployeeIdentificationNumber => "G9",
                GovernmentAdvanceProgress => "GA",
                GrainBlockNumber => "GB",
                GovernmentContractNumber => "GC",
                ReturnGoodsBillOfLadingNumber => "GD",
                GeographicNumber => "GE",
                SpecialtyLicenseNumber => "GF",
                GaugeTicketNumber => "GG",
                IdentificationCardSerialNumber => "GH",
                SecondaryProviderNumber => "GI",
                CornboreCertificationNumber => "GJ",
                ThirdPartyReferenceNumber => "GK",
                GeographicDestinationZoneNumber => "GL",
                LoanAcquisitionNumber => "GM",
                FolderNumber => "GN",
                Exhibit => "GO",
                GovernmentPriorityNumber => "GP",
                InternalPurchaseOrderReleaseNumber => "GQ",
                GrainOrderReferenceNumber => "GR",
                CodeGS => "GS",
                GoodsAndServiceTaxRegistrationNumber => "GT",
                InternalPurchaseOrderItemNumber => "GU",
                ThirdPartyPurchaseOrderNumber => "GV",
                ThirdPartyPurchaseOrderReleaseNumber => "GW",
                GroupWorkCandidateSequenceNumber => "GWS",
                ThirdPartyPurchaseOrderItemNumber => "GX",
                EmptyRepositioningNumber => "GY",
                GeneralLedgerAccount => "GZ",
                HighFabricationAuthorizationNumber => "H1",
                HighRawMaterialAuthorizationNumber => "H2",
                GravitySourceMeterNumber => "H3",
                SpecialClause => "H5",
                QualityClause => "H6",
                StandardClause => "H7",
                CodeH8 => "H8",
                PaymentHistoryReferenceNumber => "H9",
                CompetentAuthority => "HA",
                CodeHB => "HB",
                HeatCode => "HC",
                DepartmentOfTransportationHazardousNumber => "HD",
                HazardousExemptionNumber => "HE",
                EngineeringDataList => "HF",
                CivilActionNumber => "HG",
                FiscalCode => "HH",
                TypeOfHouseholdGoodsCode => "HHT",
                CodeHI => "HI",
                IdentityCardNumber => "HJ",
                JudgmentNumber => "HK",
                SirenNumber => "HL",
                SiretNumber => "HM",
                HomeMortgageDisclosureActBlockNumberArea => "HMB",
                HazardousCertificationNumber => "HN",
                ShippersHazardousNumber => "HO",
                CodeHP => "HP",
                CentersForMedicareAndMedicaidServicesNationalProvider => "HPI",
                ReinsuranceReference => "HQ",
                Horsepower => "HR",
                CodeHS => "HS",
                CodeOfFederalRegulations => "HT",
                TypeOfEscrowNumber => "HU",
                CodeHUD => "HUD",
                EscrowFileNumber => "HV",
                HighWideFileNumber => "HW",
                AutoLossItemNumber => "HX",
                PropertyLossItemNumber => "HY",
                CodeHZ => "HZ",
                OwningBureauIdentificationNumber => "I1",
                CodeI2 => "I2",
                NonAmericanIdentificationNumber => "I3",
                CreditCounselingIdentificationNumber => "I4",
                InvoiceIdentification => "I5",
                CreditReportNumber => "I7",
                Pollutant => "I9",
                InternalVendorNumber => "IA",
                InBondNumber => "IB",
                InboundToParty => "IC",
                CodeICD => "ICD",
                InsuranceCertificateNumber => "ID",
                InterchangeAgreementNumber => "IE",
                IssueNumber => "IF",
                InitialFailureClaim => "IFC",
                InternationalFuelTaxAgreementAccountNumber => "IFT",
                InsurancePolicyNumber => "IG",
                InitialDealerClaimNumber => "IH",
                InitialSampleInspectionReportNumber => "II",
                Image => "IID",
                CodeIJ => "IJ",
                InvoiceNumber => "IK",
                InternalOrderNumber => "IL",
                CodeIM => "IM",
                CodeIMP => "IMP",
                CodeIMS => "IMS",
                ConsigneesInvoiceNumber => "IN",
                InvestigatorialNewDrugNumber => "IND",
                InboundToOrOutboundFromParty => "IO",
                InspectionReportNumber => "IP",
                EndItem => "IQ",
                IntraPlantRouting => "IR",
                ImportersReferenceNumberToLetterOfCredit => "IRN",
                InternationalRegistrationPlanAccountNumber => "IRP",
                InvoiceNumberSuffix => "IS",
                CodeISC => "ISC",
                InternationalRegistrationPlanStickerNumber => "ISN",
                InspectionAndSurveySequenceNumber => "ISS",
                InternalCustomerNumber => "IT",
                InitialTroubleIndication => "ITI",
                BargePermitNumber => "IU",
                SellersInvoiceNumber => "IV",
                PartInterchangeability => "IW",
                ItemNumber => "IX",
                InsuredParcelPostNumber => "IZ",
                Proceeding => "J0",
                Creditor => "J1",
                Attorney => "J2",
                Judge => "J3",
                Trustee => "J4",
                OriginatingCase => "J5",
                AdversaryCase => "J6",
                LeadCase => "J7",
                JointlyAdministeredCase => "J8",
                SubstantivelyConsolidatedCase => "J9",
                BeginningJobSequenceNumber => "JA",
                CodeJB => "JB",
                Review => "JC",
                JointCreditSpecificationNumber => "JCS",
                UserIdentification => "JD",
                EndingJobSequenceNumber => "JE",
                AutomatedUnderwritingReferenceNumber => "JF",
                Tag => "JH",
                MultipleListingServiceArea => "JI",
                MultipleListingServiceSubArea => "JK",
                Packet => "JL",
                MultipleListingServiceMapXCoordinate => "JM",
                MultipleListingServiceMapYCoordinate => "JN",
                MultipleListingNumber => "JO",
                MultipleListingServiceBookType => "JP",
                Elevation => "JQ",
                PropertyComponentLocation => "JR",
                JobSequenceNumber => "JS",
                CodeJT => "JT",
                PriorPhoneNumber => "JU",
                PriorHealthIndustryNumber => "JV",
                CodeJW => "JW",
                PriorPostalZipCode => "JX",
                OriginOfShipmentHarmonizedBasedCode => "JY",
                GoverningClassCode => "JZ",
                ApprovalCode => "K0",
                ForeignMilitarySalesNoticeNumber => "K1",
                CertifiedMailNumber => "K2",
                RegisteredMailNumber => "K3",
                CriticalityDesignator => "K4",
                TaskOrder => "K5",
                PurchaseDescription => "K6",
                ParagraphNumber => "K7",
                ProjectParagraphNumber => "K8",
                InquiryRequestNumber => "K9",
                DistributionList => "KA",
                AssociatedContract => "KAS",
                BeginningKanbanSerialNumber => "KB",
                ExhibitDistributionList => "KC",
                ConfirmationServiceContract => "KCS",
                SpecialInstructionsNumber => "KD",
                EndingKanbanSerialNumber => "KE",
                ForeclosingStatus => "KG",
                TypeOfLawSuit => "KH",
                TypeOfOutstandingJudgment => "KI",
                ConfirmationIntraday => "KII",
                TaxLienJurisdiction => "KJ",
                DeliveryReference => "KK",
                ContractReference => "KL",
                RentalAccountNumber => "KM",
                CensusAutomatedFilesId => "KN",
                CustomsDrawbackEntryNumber => "KO",
                HealthCertificateNumber => "KP",
                ProcuringAgency => "KQ",
                ResponseToARequestForQuotationReference => "KR",
                ReleaserContract => "KRL",
                ReplacementShipperContract => "KRP",
                Solicitation => "KS",
                ServiceRequesterContract => "KSR",
                RequestForQuotationReference => "KT",
                OfficeSymbol => "KU",
                DistributionStatementCode => "KV",
                Certification => "KW",
                Representation => "KX",
                CodeKY => "KY",
                CodeKZ => "KZ",
                CodeL0 => "L0",
                LettersOrNotes => "L1",
                LocationOnProductCode => "L2",
                LaborOperationNumber => "L3",
                ProposalParagraphNumber => "L4",
                SubexhibitLineItemNumber => "L5",
                SubcontractLineItemNumber => "L6",
                CustomersReleaseNumber => "L7",
                ConsigneesReleaseNumber => "L8",
                CustomersPartNumber => "L9",
                ShippingLabelSerialNumber => "LA",
                LotteryAuthorityActivationNumber => "LAA",
                LaneNumber => "LAN",
                Lockbox => "LB",
                LeaseNumber => "LC",
                LoanNumber => "LD",
                LenderEntityNumber => "LE",
                LocationExceptionOrderNumber => "LEN",
                AssemblyLineFeedLocation => "LF",
                LeaseScheduleNumber => "LG",
                LongitudeExpressedInSeconds => "LH",
                CodeLI => "LI",
                CodeLIC => "LIC",
                LocalJurisdiction => "LJ",
                CodeLK => "LK",
                LatitudeExpressedInSeconds => "LL",
                ProductPeriodForWhichLaborCostsAreFirm => "LM",
                LocalMedia => "LMI",
                NonPickupLimitedTariffNumber => "LN",
                LoadPlanningNumber => "LO",
                CodeLOI => "LOI",
                LossConditions => "LOS",
                ForPickupLimitedFreightTariffNumber => "LP",
                LoanProspectorKeyNumber => "LPK",
                CodeLQ => "LQ",
                LocalStudentIdentificationNumber => "LR",
                BarCodedSerialNumber => "LS",
                LogisticsSupportDocumentationTypeCode => "LSD",
                LotNumber => "LT",
                LocationNumber => "LU",
                LicensePlateNumber => "LV",
                LevyingOfficerIdentification => "LVO",
                LocationWithinEquipment => "LW",
                QualifiedProductsList => "LX",
                DestinationOfShipmentHarmonizedBasedCode => "LY",
                LenderAccountNumber => "LZ",
                MexicanPedimentoNumber => "M0",
                MaterialStorageLocation => "M1",
                MajorForceProgram => "M2",
                CropYear => "M3",
                LeaseAgreementAmendmentNumberMaster => "M5",
                MilitaryOrdnanceSecurityRiskNumber => "M6",
                MedicalAssistanceCategory => "M7",
                LimitedPartnershipIdentificationNumber => "M8",
                TaxShelterNumber => "M9",
                ShipNoticeManifestNumber => "MA",
                MasterBillOfLading => "MB",
                CodeMBS => "MBS",
                Mailbox => "MBX",
                MicrofilmNumber => "MC",
                CodeMCC => "MCC",
                MotorCarrierIdentificationNumber => "MCI",
                MornetPlusCaseNumber => "MCN",
                MagazineCode => "MD",
                HazardousWasteManifestDocumentNumber => "MDN",
                MessageAddressOrId => "ME",
                ManufacturersPartNumber => "MF",
                MeterNumber => "MG",
                ManufacturingOrderNumber => "MH",
                MillOrderNumber => "MI",
                CodeMII => "MII",
                MornetPlusInstitutionNumber => "MIN",
                ModelNumber => "MJ",
                ManifestKeyNumber => "MK",
                MilitaryRankCivilianPayGradeNumber => "ML",
                MasterLeaseAgreementNumber => "MM",
                MicrNumber => "MN",
                ManufacturingOperationNumber => "MO",
                MultiplePOSOfAnInvoice => "MP",
                MarketingPlanIdentificationNumber => "MPN",
                MeterProvingReportNumber => "MQ",
                MerchandiseTypeCode => "MR",
                EligibilityCategory => "MRC",
                MothersMedicalRecordIdentificationNumber => "MRN",
                ManufacturersMaterialSafetyDataSheetNumber => "MS",
                MailSlot => "MSL",
                MeterTicketNumber => "MT",
                CodeMU => "MU",
                MornetPlusUserIdentification => "MUI",
                MigrantNumber => "MV",
                MilitaryCallNumber => "MW",
                MaterialChangeNoticeNumber => "MX",
                ModelYearNumber => "MY",
                MaintenanceRequestNumber => "MZ",
                MultipleZoneOrderNumber => "MZO",
                NominationNumber => "N0",
                LocalSchoolCourseNumber => "N1",
                LocalSchoolDistrictCourseNumber => "N2",
                StatewideCourseNumber => "N3",
                CodeN4 => "N4",
                ProviderPlanNetworkIdentificationNumber => "N5",
                PlanNetworkIdentificationNumber => "N6",
                FacilityNetworkIdentificationNumber => "N7",
                SecondaryHealthInsuranceIdentificationNumber => "N8",
                DataAuthenticationNumber => "N9",
                NorthAmericanHazardousClassificationNumber => "NA",
                CodeNAS => "NAS",
                LetterOfCreditNumber => "NB",
                SecondaryCoverageCompanyNumber => "NC",
                LetterOfCreditDraftNumber => "ND",
                AbbreviatedNewDrugApplicationNumber => "NDA",
                NewDrugApplicationNumber => "NDB",
                LeaseRiderNumber => "NE",
                CodeNF => "NF",
                NationalFloodInsuranceProgramCommunityName => "NFC",
                NationalFloodInsuranceProgramCounty => "NFD",
                NationalFloodInsuranceProgramMapNumber => "NFM",
                NationalFloodInsuranceProgramCommunityNumber => "NFN",
                NationalFloodInsuranceProgramState => "NFS",
                NaturalGasPolicyActCategoryCode => "NG",
                RateCardNumber => "NH",
                CodeNI => "NI",
                TechnicalDocumentNumber => "NJ",
                PriorCase => "NK",
                TechnicalOrderNumber => "NL",
                DiscounterRegistrationNumber => "NM",
                NominationModelType => "NMT",
                NonconformanceReportNumber => "NN",
                NoOt5AuthorityZeroMileageRate => "NO",
                PartialPaymentNumber => "NP",
                MedicaidRecipientIdentificationNumber => "NQ",
                ProgressPaymentNumber => "NR",
                NationalStockNumber => "NS",
                AdministratorsReferenceNumber => "NT",
                NonOriginatingThirdPartyNumber => "NTP",
                PendingCase => "NU",
                AssociatedPolicyNumber => "NW",
                RelatedNonconformanceNumber => "NX",
                AgentClaimNumber => "NY",
                CriticalApplication => "NZ",
                OuterContinentalShelfAreaCode => "O1",
                OuterContinentalShelfBlockNumber => "O2",
                Ot5AuthorityConditionOrRestrictionOnCarHireRate => "O5",
                CodeO7 => "O7",
                OriginalFiling => "O8",
                ContinuationFiling => "O9",
                OutletNumber => "OA",
                OceanBillOfLading => "OB",
                OceanContainerNumber => "OC",
                OriginalReturnRequestReferenceNumber => "OD",
                OpenAndPrepaidStationListNumber => "OE",
                OperatorIdentificationNumber => "OF",
                Offer => "OFF",
                TerminationFiling => "OG",
                OriginHouse => "OH",
                OriginalInvoiceNumber => "OI",
                Object => "OIC",
                AmendmentFiling => "OJ",
                OfferGroup => "OK",
                OriginalShippersBillOfLadingNumber => "OL",
                OceanManifest => "OM",
                DealerOrderNumber => "ON",
                OutOfServiceNumber => "OOS",
                OriginalPurchaseOrder => "OP",
                CodeOPE => "OPE",
                CodeOPF => "OPF",
                OrderNumber => "OQ",
                OrderParagraphNumber => "OR",
                OutboundFromParty => "OS",
                SalesAllowanceNumber => "OT",
                TariffSupplementNumber => "OU",
                TariffSuffixNumber => "OV",
                ServiceOrderNumber => "OW",
                StatementNumber => "OX",
                ProductNumber => "OZ",
                PreviousContractNumber => "P1",
                PreviousDrugEnforcementAdministrationNumber => "P2",
                PreviousCustomerReferenceNumber => "P3",
                ProjectCode => "P4",
                PositionCode => "P5",
                PipelineNumber => "P6",
                ProductLineNumber => "P7",
                PickupReferenceNumber => "P8",
                PageNumber => "P9",
                PriceAreaNumber => "PA",
                PatentCooperationTreatyApplicationNumber => "PAC",
                NonprovisionalPatentApplicationNumber => "PAN",
                ProvisionalPatentApplicationNumber => "PAP",
                CodePB => "PB",
                ProductionCode => "PC",
                PoolContractCode => "PCC",
                ProtocolNumber => "PCN",
                PromotionDealNumber => "PD",
                PartialDenialIndicator => "PDI",
                PreviousDriversLicense => "PDL",
                PartialDenialReason => "PDR",
                PlantNumber => "PE",
                PrimeContractorContractNumber => "PF",
                ProductGroup => "PG",
                PackingGroupCode => "PGC",
                DownstreamPackage => "PGD",
                PlugNumber => "PGN",
                ProposedGroupWorkCandidateSequenceNumber => "PGS",
                PriorityRating => "PH",
                ProcessHandlingCode => "PHC",
                PhysicianStateLicenseNumber => "PHY",
                PriceListChangeOrIssueNumber => "PI",
                ProgramIdentificationNumber => "PID",
                PlatformIdentificationNumber => "PIN",
                PackerNumber => "PJ",
                PreviousReportNumber => "PJC",
                PackingListNumber => "PK",
                Package => "PKG",
                UpstreamPackage => "PKU",
                PriceListNumber => "PL",
                ProductLicensingAgreementNumber => "PLA",
                ProposedContractNumber => "PLN",
                PartNumber => "PM",
                PremarketApplicationNumber => "PMN",
                PermitNumber => "PN",
                PatentNumber => "PNN",
                PurchaseOrderNumber => "PO",
                PolicyNumber => "POL",
                PositionTitleNumber => "POS",
                PurchaseOrderRevisionNumber => "PP",
                CertificateOfPurchaseNumber => "PPJ",
                TaxBillIdentificationNumber => "PPK",
                CurrentYearTaxBillNumber => "PPL",
                PastYearTaxBillNumber => "PPM",
                PaymentPlanNumber => "PPN",
                PayeeIdentification => "PQ",
                PriceQuoteNumber => "PR",
                PreviouslyReportedSocialSecurityNumber => "PRS",
                ProductType => "PRT",
                PurchaseOrderNumberSuffix => "PS",
                PreviousShipmentIdentificationNumberContinuousMove => "PSI",
                NextShipmentIdentificationNumberContinuousMove => "PSL",
                CreditCard => "PSM",
                ProposedSequenceNumber => "PSN",
                PurchaseOptionAgreement => "PT",
                PatentType => "PTC",
                PreviousBillOfLadingNumber => "PU",
                PickupAppointmentNumber => "PUA",
                ProductChangeInformationNumber => "PV",
                PaymentValidationCode => "PVC",
                PriorPurchaseOrderNumber => "PW",
                PreliminaryWorkCandidateNumber => "PWC",
                ProposedWorkCandidateSequenceNumber => "PWS",
                PreviousInvoiceNumber => "PX",
                HealthCareProviderTaxonomyCode => "PXC",
                CodePY => "PY",
                PayrollActivityCode => "PYA",
                PayRange => "PYR",
                ProductChangeNoticeNumber => "PZ",
                QuoteNumber => "Q1",
                StartingPackageNumber => "Q2",
                EndingPackageNumber => "Q3",
                PriorIdentifierNumber => "Q4",
                PropertyControlNumber => "Q5",
                RecallNumber => "Q6",
                ReceiverClaimNumber => "Q7",
                RegistrationNumber => "Q8",
                RepairOrderNumber => "Q9",
                Press => "QA",
                PressForm => "QB",
                ProductSpecificationDocumentNumber => "QC",
                ReplacementDrugEnforcementAdministrationNumber => "QD",
                ReplacementCustomerReferenceNumber => "QE",
                QualityDispositionArea => "QF",
                ReplacementAssemblyModelNumber => "QG",
                ReplacementAssemblySerialNumber => "QH",
                QualityInspectionArea => "QI",
                ReturnMaterialAuthorizationNumber => "QJ",
                SalesProgramNumber => "QK",
                ServiceAuthorizationNumber => "QL",
                QualityReviewMaterialCrib => "QM",
                StopSequenceNumber => "QN",
                ServiceEstimateNumber => "QO",
                SubstitutePartNumber => "QP",
                UnitNumber => "QQ",
                QualityReportNumber => "QR",
                WarrantyCoverageCode => "QS",
                WarrantyRegistrationNumber => "QT",
                ChangeVerificationProcedureCode => "QU",
                MajorSystemAffectedCode => "QV",
                NewPartNumber => "QW",
                OldPartNumber => "QX",
                ServicePerformedCode => "QY",
                ReferenceDrawingNumber => "QZ",
                CodeR0 => "R0",
                CurrentRevisionNumber => "R1",
                CanceledRevisionNumber => "R2",
                CorrectionNumber => "R3",
                TariffSectionNumber => "R4",
                TariffPageNumber => "R5",
                TariffRuleNumber => "R6",
                AccountsReceivableOpenItem => "R7",
                RentalAgreementNumber => "R8",
                RejectionNumber => "R9",
                RepetitiveCargoShipmentNumber => "RA",
                RestrictedAvailabilityAuthorization => "RAA",
                RestrictedAvailabilityNumber => "RAN",
                RateCodeNumber => "RB",
                RailRoutingCode => "RC",
                ReelNumber => "RD",
                ReleaseNumber => "RE",
                RelatedCase => "REC",
                ExportReferenceNumber => "RF",
                RouteOrderNumberDomestic => "RG",
                RegulatoryGuideline => "RGI",
                RouteOrderNumberExport => "RH",
                ReleaseInvoiceNumberForPriorBillAndHold => "RI",
                RigNumber => "RIG",
                RouteOrderNumberEmergency => "RJ",
                RackTypeNumber => "RK",
                ReserveAssemblyLineFeedLocation => "RL",
                RoleIdentificationNumber => "RLI",
                CodeRM => "RM",
                RunNumber => "RN",
                RepetitiveBookingNumber => "RO",
                RepetitivePatternCode => "RP",
                RelativePriority => "RPP",
                RegulationPrimaryNumber => "RPS",
                ReportNumber => "RPT",
                PurchaseRequisitionNumber => "RQ",
                CodeRR => "RR",
                RoutingRequestControlNumber => "RRC",
                ReconciliationReportSectionIdentificationCode => "RRS",
                ReturnableContainerSerialNumber => "RS",
                ReservationNumber => "RSN",
                RegulationSecondaryNumber => "RSS",
                CodeRT => "RT",
                RouteNumber => "RU",
                ReceivingNumber => "RV",
                CodeRW => "RW",
                ReportingWeek => "RWK",
                ResubmitNumber => "RX",
                RebateNumber => "RY",
                ReturnedGoodsAuthorizationNumber => "RZ",
                SpecialApproval => "S0",
                EngineeringSpecificationNumber => "S1",
                DataSource => "S2",
                SpecificationNumber => "S3",
                ShippersBondNumber => "S4",
                RoutingInstructionNumber => "S5",
                StockNumber => "S6",
                StackTrainIdentification => "S7",
                SealOffNumber => "S8",
                SealOnNumber => "S9",
                Salesperson => "SA",
                SalaryStep => "SAL",
                SalesRegionNumber => "SB",
                SuretyBondNumber => "SBN",
                ShipperCarOrderNumber => "SC",
                CodeSCA => "SCA",
                ScaleNumber => "SCN",
                SubdayNumber => "SD",
                SchoolDistrictTypeCode => "SDT",
                SerialNumber => "SE",
                SearchKey => "SEK",
                Session => "SES",
                ShipFrom => "SF",
                Savings => "SG",
                SenderDefinedClause => "SH",
                ShelfLifeIndicator => "SHL",
                CodeSI => "SI",
                SalvageInstruction => "SII",
                SetNumber => "SJ",
                ServiceChangeNumber => "SK",
                SalesTerritoryCode => "SL",
                SalesOfficeNumber => "SM",
                SettlementMethodCode => "SMC",
                StateOfMassachusettsTownCode => "SMT",
                SealNumber => "SN",
                CodeSNH => "SNH",
                CodeSNP => "SNP",
                StateNonResidentViolatorCompact => "SNV",
                CodeSO => "SO",
                ScanLine => "SP",
                CodeSPL => "SPL",
                TheaterScreenNumber => "SPN",
                ContainerSequenceNumber => "SQ",
                SalesResponsibility => "SR",
                SplitShipmentNumber => "SS",
                SchoolSystemTypeCode => "SST",
                StoreNumber => "ST",
                CodeSTB => "STB",
                CodeSTR => "STR",
                ServiceabilityStandardTestingReference => "STS",
                SpecialProcessingCode => "SU",
                TitleReference => "SUB",
                SupervisoryUnionCode => "SUC",
                SpacingUnitOrderNumber => "SUO",
                ServiceChargeNumber => "SV",
                SellersSaleNumber => "SW",
                ServiceInterruptTrackingNumber => "SX",
                SocialSecurityNumber => "SY",
                SpecificationRevision => "SZ",
                DealerTypeIdentification => "T0",
                TaxExchangeCode => "T1",
                TaxFormCode => "T2",
                TaxScheduleCode => "T3",
                SignalCode => "T4",
                TrailerUseAgreements => "T5",
                TaxFiling => "T6",
                AffectedSubsystemCode => "T7",
                DescriptionOfChangeCode => "T8",
                DocumentationAffectedNumber => "T9",
                TelecommunicationCircuitSupplementalId => "TA",
                TruckersBillOfLading => "TB",
                VendorTerms => "TC",
                ReasonForChange => "TD",
                TechnicalDocumentationType => "TDT",
                CodeTE => "TE",
                TransferNumber => "TF",
                TimeFailure => "TFC",
                CodeTG => "TG",
                CodeTH => "TH",
                TirNumber => "TI",
                TechnicalInformationPackage => "TIP",
                FederalTaxpayersIdentificationNumber => "TJ",
                TankNumber => "TK",
                TaxLicenseExemption => "TL",
                CodeTM => "TM",
                TransactionReferenceNumber => "TN",
                TerminalOperatorNumber => "TO",
                TypeOfComment => "TOC",
                TestSpecificationNumber => "TP",
                TransponderNumber => "TPN",
                TracerActionRequestNumber => "TQ",
                GovernmentTransportationRequest => "TR",
                TariffNumber => "TS",
                TemplateSequenceNumber => "TSN",
                TerminalCode => "TT",
                TrialLocationCode => "TU",
                LineOfBusiness => "TV",
                TaxWorksheet => "TW",
                TaxExemptNumber => "TX",
                PolicyType => "TY",
                TotalCycleNumber => "TZ",
                ConsolidatorsReceiptNumber => "U0",
                RegionalAccountNumber => "U1",
                Term => "U2",
                CodeU3 => "U3",
                UnpaidInstallmentReferenceNumber => "U4",
                SuccessorAccount => "U5",
                PredecessorAccount => "U6",
                CodeU8 => "U8",
                CodeU9 => "U9",
                MortgageNumber => "UA",
                UnacceptableSourcePurchaserId => "UB",
                MortgageInsuranceIndicatorNumber => "UC",
                CodeUCB => "UCB",
                CodeUCM => "UCM",
                UnacceptableSourceDunsNumber => "UD",
                SecondaryCoverageCertificateNumber => "UE",
                MortgageInsuranceCompanyNumber => "UF",
                USGovernmentTransportationControlNumber => "UG",
                RemovalNumber => "UH",
                PreviousCourseNumber => "UI",
                CodeUIC => "UIC",
                CurrentOrLatestCourseNumber => "UJ",
                EquivalentCourseNumberAtRequestingInstitution => "UK",
                CrossListedCourseNumber => "UL",
                QuarterQuarterSectionNumber => "UM",
                UnitedNationsHazardousClassificationNumber => "UN",
                QuarterQuarterSpotNumber => "UO",
                UpstreamShipperContractNumber => "UP",
                SectionNumber => "UQ",
                UnitReliefNumber => "UR",
                UniformResourceLocator => "URL",
                UnitReportPeriod => "URP",
                UnitReportPeriodId => "URQ",
                UnacceptableSourceSupplierId => "US",
                UnitTrain => "UT",
                TownshipNumber => "UU",
                RangeNumber => "UV",
                StateSenateDistrict => "UW",
                StateAssemblyDistrict => "UX",
                CodeUY => "UY",
                StateLegislativeDistrict => "UZ",
                Version => "V0",
                VolumePurchaseAgreementNumber => "V1",
                VisaType => "V2",
                VoyageNumber => "V3",
                StateDepartmentI20FormNumber => "V4",
                StateDepartmentIap66FormNumber => "V5",
                CodeV6 => "V6",
                JudicialDistrict => "V7",
                InstitutionNumber => "V8",
                Subservicer => "V9",
                VesselAgentNumber => "VA",
                VeteransAdministrationOriginatorIdentification => "VAO",
                CodeVB => "VB",
                VendorContractNumber => "VC",
                VolumeNumber => "VD",
                VendorAbbreviationCode => "VE",
                VendorChangeIdentificationCode => "VF",
                VendorChangeProcedureCode => "VG",
                VehicleGaragedStateCode => "VGS",
                CountyLegislativeDistrict => "VH",
                PoolNumber => "VI",
                InvestorNoteHolderIdentification => "VJ",
                InstitutionNoteHolderIdentification => "VK",
                ThirdPartyNoteHolderIdentification => "VL",
                Ward => "VM",
                VendorOrderNumber => "VN",
                InstitutionLoanNumber => "VO",
                VendorProductNumber => "VP",
                RelatedContractLineItemNumber => "VQ",
                VendorIdNumber => "VR",
                VendorOrderNumberSuffix => "VS",
                MotorVehicleIdNumber => "VT",
                PreparersVerificationNumber => "VU",
                Voucher => "VV",
                Standard => "VW",
                CodeVX => "VX",
                LinkSequenceNumber => "VY",
                SponsorsReferenceNumber => "VZ",
                DisposalTurnInDocumentNumber => "W1",
                WeaponSystemNumber => "W2",
                ManufacturingDirectiveNumber => "W3",
                ProcurementRequestNumber => "W4",
                InspectorIdentificationNumber => "W5",
                FederalSupplyScheduleNumber => "W6",
                CodeW7 => "W7",
                Suffix => "W8",
                SpecialPackagingInstructionNumber => "W9",
                LaborOrAffiliationIdentification => "WA",
                CodeWB => "WB",
                ContractOptionNumber => "WC",
                WorkCandidateSequenceNumber => "WCS",
                ReviewPeriodNumber => "WD",
                WithdrawalRecord => "WDR",
                WellClassificationCode => "WE",
                LocallyAssignedControlNumber => "WF",
                VendorsPreviousJobNumber => "WG",
                CodeWH => "WH",
                Waiver => "WI",
                PreAwardSurvey => "WJ",
                TypeOfScienceCode => "WK",
                FederalSupplyClassificationCode => "WL",
                WeightAgreementNumber => "WM",
                WellNumber => "WN",
                WorkOrderNumber => "WO",
                WarehousePickTicketNumber => "WP",
                InterimFundingOrganizationLoanNumber => "WQ",
                WarehouseReceiptNumber => "WR",
                WarehouseStorageLocationNumber => "WS",
                BrokersReferenceNumber => "WT",
                Vessel => "WU",
                DealerIdentification => "WV",
                DepositoryTrustCompanyIdentification => "WW",
                DistributorsAccountIdentification => "WX",
                WaybillNumber => "WY",
                DistributorsRepresentativeIdentification => "WZ",
                DebtorsAccount => "X0",
                ProviderClaimNumber => "X1",
                SpecificationClassNumber => "X2",
                DefectCodeNumber => "X3",
                ClinicalLaboratoryImprovementAmendmentNumber => "X4",
                StateIndustrialAccidentProviderNumber => "X5",
                OriginalVoucherNumber => "X6",
                BatchSequenceNumber => "X7",
                SecondarySuffixCodeIndicator => "X8",
                InternalControlNumber => "X9",
                SubstituteNationalStockNumber => "XA",
                SubstituteManufacturersPartNumber => "XB",
                CargoControlNumber => "XC",
                SubsistenceIdentificationNumber => "XD",
                TransportationPriorityNumber => "XE",
                GovernmentBillOfLadingOfficeCode => "XF",
                AirlineTicketNumber => "XG",
                ContractAuditorIdNumber => "XH",
                FederalHomeLoanMortgageCorporationLoanNumber => "XI",
                FederalHomeLoanMortgageCorporationDefaultForeclosureSpecialistNumber => {
                    "XJ"
                }
                MortgageeLoanNumber => "XK",
                InsuredsLoanNumber => "XL",
                IssuerNumber => "XM",
                TitleXixIdentifierNumber => "XN",
                SampleNumber => "XO",
                PreviousCargoControlNumber => "XP",
                PierNumber => "XQ",
                RailroadCommissionRecordNumber => "XR",
                GasAnalysisSourceMeterNumber => "XS",
                ToxicologyId => "XT",
                UniversalTransverseMercatorNorth => "XU",
                UniversalTransverseMercatorEast => "XV",
                UniversalTransverseMercatorZone => "XW",
                RatingPeriod => "XX",
                SpecialProgramCode => "XX1",
                ServiceAreaCode => "XX2",
                FunctionCode => "XX3",
                ObjectCode => "XX4",
                OrganizationCode => "XX5",
                SubjectAreaCode => "XX6",
                ScheduleTypeCode => "XX7",
                AlternatingScheduleIdentifierCode => "XX8",
                OtherUnlistedTypeOfReferenceNumber => "XY",
                PharmacyPrescriptionNumber => "XZ",
                Debtor => "Y0",
                ClaimAdministratorClaimNumber => "Y1",
                ThirdPartyAdministratorClaimNumber => "Y2",
                ContractHolderClaimNumber => "Y3",
                AgencyClaimNumber => "Y4",
                DeliveryTrailerManifest => "Y5",
                SortAndSegregate => "Y6",
                UserId => "Y8",
                CurrentCertificateNumber => "Y9",
                PriorCertificateNumber => "YA",
                RevisionNumber => "YB",
                Tract => "YC",
                BuyerIdentification => "YD",
                RailroadCommissionOilNumber => "YE",
                LesseeIdentification => "YF",
                OperatorAssignedUnitNumber => "YH",
                RefinerIdentification => "YI",
                RevenueSource => "YJ",
                RentPayorIdentification => "YK",
                AllowanceRecipientIdentification => "YL",
                ResourceScreeningReference => "YM",
                ReceiverIdQualifier => "YN",
                Formation => "YO",
                SellingArrangement => "YP",
                MinimumRoyaltyPayorIdentification => "YQ",
                OperatorLeaseNumber => "YR",
                YardPosition => "YS",
                ReporterIdentification => "YT",
                ParticipatingArea => "YV",
                EngineeringChangeProposal => "YW",
                GeographicScore => "YX",
                GeographicKey => "YY",
                GeographicIndex => "YZ",
                SafetyOfShipCertificate => "Z1",
                SafetyOfRadioCertificate => "Z2",
                SafetyEquipmentCertificate => "Z3",
                CivilLiabilitiesOfOilCertificate => "Z4",
                LoadLineCertificate => "Z5",
                DeratCertificate => "Z6",
                MaritimeDeclarationOfHealth => "Z7",
                FederalHousingAdministrationCaseNumber => "Z8",
                VeteransAffairsCaseNumber => "Z9",
                Supplier => "ZA",
                UltimateConsignee => "ZB",
                ConnectingCarrier => "ZC",
                FamilyMemberIdentification => "ZD",
                CoalAuthorityNumber => "ZE",
                SalesRepresentativeOrderNumber => "ZG",
                CarrierAssignedReferenceNumber => "ZH",
                ReferenceVersionNumber => "ZI",
                CodeZJ => "ZJ",
                DuplicateWaybillInRoute => "ZK",
                DuplicateWaybillNotInRoute => "ZL",
                ManufacturerNumber => "ZM",
                AgencyCaseNumber => "ZN",
                MakegoodCommercialLineNumber => "ZO",
                SpouseTie => "ZP",
                NonSpouseTie => "ZQ",
                CodeZR => "ZR",
                SoftwareApplicationNumber => "ZS",
                MillingInTransit => "ZT",
                CodeZTS => "ZTS",
                Field => "ZU",
                Block => "ZV",
                Area => "ZW",
                CountyCode => "ZX",
                ReferencedPatternIdentification => "ZY",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ReferenceIdentificationQualifier> {
        use ReferenceIdentificationQualifier::*;
        match code {
            b"00" => Some(ContractingDistrictNumber),
            b"0A" => Some(SupervisoryAppraiserCertificationNumber),
            b"0B" => Some(StateLicenseNumber),
            b"0D" => Some(SubjectPropertyVerificationSource),
            b"0E" => Some(SubjectPropertyReferenceNumber),
            b"0F" => Some(SubscriberNumber),
            b"0G" => Some(ReviewerFileNumber),
            b"0H" => Some(ComparablePropertyPendingSaleReferenceNumber),
            b"0I" => Some(ComparablePropertySaleReferenceNumber),
            b"0J" => Some(SubjectPropertyNonSaleReferenceNumber),
            b"0K" => Some(PolicyFormIdentifyingNumber),
            b"0L" => Some(ReferencedBy),
            b"0M" => Some(MortgageIdentificationNumber),
            b"0N" => Some(AttachedTo),
            b"0P" => Some(RealEstateOwnedProperty),
            b"01" => Some(Code01),
            b"1A" => Some(BlueCrossProviderNumber),
            b"01A" => Some(CatalogOfFederalDomesticAssistance),
            b"1B" => Some(BlueShieldProviderNumber),
            b"01B" => Some(UnionAgreement),
            b"1C" => Some(MedicareProviderNumber),
            b"01C" => Some(Code01C),
            b"1D" => Some(MedicaidProviderNumber),
            b"01D" => Some(Code01D),
            b"1E" => Some(DentistLicenseNumber),
            b"01E" => Some(Code01E),
            b"1F" => Some(AnesthesiaLicenseNumber),
            b"1G" => Some(ProviderUpinNumber),
            b"01G" => Some(PaymentRelatedClause),
            b"1H" => Some(ChampusIdentificationNumber),
            b"01H" => Some(SpecialPriceAuthorizationNumber),
            b"1I" => Some(Code1I),
            b"1J" => Some(FacilityIdNumber),
            b"1K" => Some(PayorsClaimNumber),
            b"1L" => Some(GroupOrPolicyNumber),
            b"1M" => Some(PreferredProviderOrganizationSiteNumber),
            b"1N" => Some(Code1N),
            b"1O" => Some(ConsolidationShipmentNumber),
            b"1P" => Some(AccessorialStatusCode),
            b"1Q" => Some(ErrorIdentificationCode),
            b"1R" => Some(StorageInformationCode),
            b"1S" => Some(Code1S),
            b"1T" => Some(Code1T),
            b"1U" => Some(PayGrade),
            b"1V" => Some(RelatedVendorOrderNumber),
            b"1W" => Some(MemberIdentificationNumber),
            b"1X" => Some(CreditOrDebitAdjustmentNumber),
            b"1Y" => Some(RepairActionNumber),
            b"1Z" => Some(FinancialDetailCode),
            b"02" => Some(Code02),
            b"2A" => Some(ImportLicenseNumber),
            b"2B" => Some(TerminalReleaseOrderNumber),
            b"2C" => Some(LongTermDisabilityPolicyNumber),
            b"2D" => Some(Code2D),
            b"2E" => Some(ForeignMilitarySalesCaseNumber),
            b"2F" => Some(ConsolidatedInvoiceNumber),
            b"2G" => Some(Amendment),
            b"2H" => Some(AssignedByTransactionSetSender),
            b"2I" => Some(TrackingNumber),
            b"2J" => Some(FloorNumber),
            b"2K" => Some(Code2K),
            b"2L" => Some(Code2L),
            b"2M" => Some(Code2M),
            b"2N" => Some(Code2N),
            b"2O" => Some(Code2O),
            b"2P" => Some(Subdivision),
            b"2Q" => Some(Code2Q),
            b"2R" => Some(CouponRedemptionNumber),
            b"2S" => Some(Catalog),
            b"2T" => Some(SubSubhouseBillOfLading),
            b"2U" => Some(PayerIdentificationNumber),
            b"2V" => Some(Code2V),
            b"2W" => Some(ChangeOrderAuthority),
            b"2X" => Some(SupplementalAgreementAuthority),
            b"2Y" => Some(WageDetermination),
            b"2Z" => Some(Code2Z),
            b"03" => Some(Code03),
            b"3A" => Some(SectionOfTheNationalHousingActCode),
            b"3B" => Some(SupplementalClaimNumber),
            b"3C" => Some(PayeeLoanNumber),
            b"3D" => Some(ServicerLoanNumber),
            b"3E" => Some(InvestorLoanNumber),
            b"3F" => Some(ShowIdentification),
            b"3G" => Some(CatastropheNumber),
            b"3H" => Some(CaseNumber),
            b"3I" => Some(PrecinctNumber),
            b"3J" => Some(OfficeNumber),
            b"3K" => Some(PetroleumPoolCode),
            b"3L" => Some(Branch),
            b"3M" => Some(Code3M),
            b"3N" => Some(GasCustodianIdentification),
            b"3O" => Some(Code3O),
            b"3P" => Some(ThirdPartyOriginatorNumber),
            b"3Q" => Some(Code3Q),
            b"3R" => Some(Code3R),
            b"3S" => Some(Code3S),
            b"3T" => Some(Code3T),
            b"3U" => Some(ProtractionNumber),
            b"3V" => Some(FormationIdentifier),
            b"3W" => Some(Code3W),
            b"3X" => Some(SubcontractNumber),
            b"3Y" => Some(ReceiverAssignedDropZone),
            b"3Z" => Some(CustomsBrokerReferenceNumber),
            b"04" => Some(CanadianFinancialInstitutionBranchAndInstitutionNumber),
            b"4A" => Some(Code4A),
            b"4B" => Some(ShipmentOriginCode),
            b"4C" => Some(ShipmentDestinationCode),
            b"4D" => Some(ShippingZone),
            b"4E" => Some(CarrierAssignedConsigneeNumber),
            b"4F" => Some(CarrierAssignedShipperNumber),
            b"4G" => Some(ProvincialTaxIdentification),
            b"4H" => Some(CommercialInvoiceNumber),
            b"4I" => Some(BalanceDueReferenceNumber),
            b"4J" => Some(VehicleRelatedServicesReferenceNumber),
            b"4K" => Some(AccessorialRailDiversionReferenceNumber),
            b"4L" => Some(LocationSpecificServicesReferenceNumber),
            b"4M" => Some(SpecialMoveReferenceNumber),
            b"4N" => Some(SpecialPaymentReferenceNumber),
            b"4O" => Some(Code4O),
            b"4P" => Some(AffiliationNumber),
            b"4Q" => Some(CallSign),
            b"4R" => Some(RuleSection),
            b"4S" => Some(PreferredCallSign),
            b"4T" => Some(Code4T),
            b"4U" => Some(MarketArea),
            b"4V" => Some(EmissionDesignator),
            b"4W" => Some(Study),
            b"4X" => Some(Log),
            b"4Y" => Some(SubhouseBillOfLading),
            b"4Z" => Some(Code4Z),
            b"05" => Some(Code05),
            b"5A" => Some(OffenseTracking),
            b"5B" => Some(SupplementalAccountNumber),
            b"5C" => Some(CongressionalDistrict),
            b"5D" => Some(LineOfCreditCategory),
            b"5E" => Some(Consumer),
            b"5F" => Some(Warrant),
            b"5G" => Some(Complaint),
            b"5H" => Some(Incident),
            b"5I" => Some(OffenderTracking),
            b"5J" => Some(DriversLicense),
            b"5K" => Some(CommercialDriversLicense),
            b"5L" => Some(JurisdictionalCommunityNumber),
            b"5M" => Some(PreviousSequence),
            b"5N" => Some(CitationOfStatute),
            b"5O" => Some(CitationOfOpinion),
            b"5P" => {
                Some(NationalCriminalInformationCenterOriginatingAgencyIdentification)
            }
            b"5Q" => Some(StateCriminalHistoryRepositoryIndividualIdentification),
            b"5R" => Some(FederalBureauOfInvestigationIndividualIdentification),
            b"5S" => Some(ProcessingArea),
            b"5T" => Some(PaymentLocation),
            b"5U" => Some(FloodData),
            b"5V" => Some(CouponDistributionMethod),
            b"5W" => Some(OriginalUniformCommercialCodeFilingNumber),
            b"5X" => Some(AmendedUniformCommercialCodeFilingNumber),
            b"5Y" => Some(ContinuationUniformCommercialCodeFilingNumber),
            b"5Z" => Some(UniformCommercialCodeFilingCollateralNumber),
            b"06" => Some(SystemNumber),
            b"6A" => Some(ConsigneeReferenceNumber),
            b"6B" => Some(Code6B),
            b"6C" => Some(Code6C),
            b"6D" => Some(Code6D),
            b"6E" => Some(MapReference),
            b"6F" => Some(AppraiserLicense),
            b"6G" => Some(MapNumber),
            b"6H" => Some(ComparablePropertyVerificationSource),
            b"6I" => Some(ComparableProperty),
            b"6J" => Some(CensusTract),
            b"6K" => Some(Zone),
            b"6L" => Some(AgentContractNumber),
            b"6M" => Some(ApplicationNumber),
            b"6N" => Some(ClaimantNumber),
            b"6O" => Some(CrossReferenceNumber),
            b"6P" => Some(GroupNumber),
            b"6Q" => Some(InsuranceLicenseNumber),
            b"6R" => Some(ProviderControlNumber),
            b"6S" => Some(ProviderOrderTicketNumber),
            b"6T" => Some(PilotLicenseNumber),
            b"6U" => Some(QuestionNumber),
            b"6V" => Some(ReissueCessionNumber),
            b"6X" => Some(Specimen),
            b"6Y" => Some(EquipmentInitial),
            b"6Z" => Some(Code6Z),
            b"07" => Some(AddOnSystemNumber),
            b"7A" => Some(PurchaseOrderNumberIncludedInOnOrderPosition),
            b"7B" => Some(PurchaseOrderNumberOfShipmentReceivedSinceLastReportingDate),
            b"7C" => Some(PurchaseOrderNumberOfOrderReceivedSinceLastReportingDate),
            b"7D" => Some(TesterIdentification),
            b"7E" => Some(CollectorIdentification),
            b"7F" => Some(RepeatLocation),
            b"7G" => Some(DataQualityRejectReason),
            b"7H" => Some(Code7H),
            b"7I" => Some(SubscriberAuthorizationNumber),
            b"7J" => Some(TollBillingTelephoneReferenceNumber),
            b"7K" => Some(ListOfMaterials),
            b"7L" => Some(QualifiedMaterialsList),
            b"7M" => Some(Frame),
            b"7N" => Some(Piggyback),
            b"7O" => Some(Tripleback),
            b"7P" => Some(Sheet),
            b"7Q" => Some(EngineeringChangeOrder),
            b"7R" => Some(RepresentativeIdentificationNumber),
            b"7S" => Some(DrawingType),
            b"7T" => Some(MasterContract),
            b"7U" => Some(RelatedTransactionReferenceNumber),
            b"7W" => Some(InterchangeTrainIdentification),
            b"7X" => Some(Code7X),
            b"7Y" => Some(Code7Y),
            b"7Z" => Some(Code7Z),
            b"08" => Some(CarrierAssignedPackageIdentificationNumber),
            b"8A" => Some(Code8A),
            b"8B" => Some(Code8B),
            b"8C" => Some(Code8C),
            b"8D" => Some(ChemicalAbstractServiceRegistryNumber),
            b"8E" => Some(GuarantorLoanNumber),
            b"8F" => Some(SchoolLoanNumber),
            b"8G" => Some(Code8G),
            b"8H" => Some(CheckListNumber),
            b"8I" => Some(FedwireConfirmationNumber),
            b"8J" => Some(Code8J),
            b"8K" => Some(DominionOfCanadaCode),
            b"8L" => Some(Code8L),
            b"8M" => Some(OriginatingCompany),
            b"8N" => Some(ReceivingCompany),
            b"8O" => Some(Code8O),
            b"8P" => Some(OriginatingDepositoryFinancialInstitution),
            b"8Q" => Some(ReceivingDepositoryFinancialInstitution),
            b"8R" => Some(SecurityType),
            b"8S" => Some(BrokerIdentification),
            b"8U" => Some(BankAssignedSecurity),
            b"8V" => Some(CreditReference),
            b"8W" => Some(BankToBank),
            b"8X" => Some(TransactionCategoryOrType),
            b"8Y" => Some(SafekeepingAccountNumber),
            b"8Z" => Some(AlternateClauseNumber),
            b"09" => Some(CustomsBarCodeNumber),
            b"9A" => Some(RepricedClaimReferenceNumber),
            b"9B" => Some(RepricedLineItemReferenceNumber),
            b"9C" => Some(AdjustedRepricedClaimReferenceNumber),
            b"9D" => Some(AdjustedRepricedLineItemReferenceNumber),
            b"9E" => Some(ReplacementClaimNumber),
            b"9F" => Some(ReferralNumber),
            b"9G" => Some(DepartmentOfDefenseForm250RequirementCode),
            b"9H" => Some(PackagingGroupNumber),
            b"9I" => Some(Code9I),
            b"9J" => Some(PensionContract),
            b"9K" => Some(Servicer),
            b"9L" => Some(ServiceBureau),
            b"9M" => Some(Code9M),
            b"9N" => Some(Investor),
            b"9P" => Some(LoanType),
            b"9Q" => Some(PoolSuffix),
            b"9R" => Some(JobOrderNumber),
            b"9S" => Some(DeliveryRegion),
            b"9T" => Some(Tenor),
            b"9U" => Some(LoanFeatureCode),
            b"9V" => Some(PaymentCategory),
            b"9W" => Some(PayerCategory),
            b"9X" => Some(AccountCategory),
            b"9Y" => Some(BankAssignedBankersReferenceNumber),
            b"9Z" => Some(ChamberOfCommerceNumber),
            b"10" => Some(AccountManagersCode),
            b"11" => Some(AccountNumber),
            b"12" => Some(BillingAccount),
            b"13" => Some(HorizontalCoordinate),
            b"14" => Some(MasterAccountNumber),
            b"15" => Some(VerticalCoordinate),
            b"16" => Some(Code16),
            b"17" => Some(ClientReportingCategory),
            b"18" => Some(PlanNumber),
            b"19" => Some(Division),
            b"20" => Some(RepairPartNumber),
            b"21" => Some(AmericanGasAssociationEquationNumber),
            b"22" => Some(SpecialChargeOrAllowanceCode),
            b"23" => Some(ClientNumber),
            b"24" => Some(ShortTermDisabilityPolicyNumber),
            b"25" => Some(ReasonNotLowestCostCode),
            b"26" => Some(UnionNumber),
            b"27" => Some(InsurorPoolIdentificationNumber),
            b"28" => Some(EmployeeIdentificationNumber),
            b"29" => Some(ForeclosureAccountNumber),
            b"30" => Some(UnitedStatesGovernmentVisaNumber),
            b"31" => Some(DocketNumber),
            b"32" => Some(CreditRepositoryCode),
            b"33" => Some(LenderCaseNumber),
            b"34" => Some(LoanRequestNumber),
            b"35" => Some(MultifamilyProjectNumber),
            b"36" => Some(UnderwriterIdentificationNumber),
            b"37" => Some(CondominiumIdentificationNumber),
            b"38" => Some(MasterPolicyNumber),
            b"39" => Some(ProposalNumber),
            b"40" => Some(LeaseScheduleNumberReplacement),
            b"41" => Some(LeaseScheduleNumberPrior),
            b"42" => Some(PhoneCalls),
            b"43" => Some(SupportingDocumentNumber),
            b"44" => Some(EndUseNumber),
            b"45" => Some(OldAccountNumber),
            b"46" => Some(OldMeterNumber),
            b"47" => Some(PlateNumber),
            b"48" => Some(AgencysStudentNumber),
            b"49" => Some(FamilyUnitNumber),
            b"50" => Some(StateStudentIdentificationNumber),
            b"51" => Some(PictureNumber),
            b"52" => Some(Code52),
            b"53" => Some(Code53),
            b"54" => Some(Code54),
            b"55" => Some(SequenceNumber),
            b"56" => Some(CorrectedSocialSecurityNumber),
            b"57" => Some(PriorIncorrectSocialSecurityNumber),
            b"58" => Some(CorrectedBatchNumber),
            b"59" => Some(PriorIncorrectBatchNumber),
            b"60" => Some(AccountSuffixCode),
            b"61" => Some(TaxingAuthorityIdentificationNumber),
            b"63" => Some(PriorLoanNumber),
            b"64" => Some(JurisdictionalCommunityName),
            b"65" => Some(TotalOrderCycleNumber),
            b"66" => Some(PreviousPolicyNumber),
            b"67" => Some(PreviousClaimHistory),
            b"68" => Some(DentalInsuranceAccountNumber),
            b"69" => Some(DentalInsurancePolicyNumber),
            b"70" => Some(CalendarNumber),
            b"71" => Some(Code71),
            b"72" => Some(ScheduleReferenceNumber),
            b"73" => Some(Code73),
            b"74" => Some(Code74),
            b"75" => Some(OrganizationBreakdownStructure),
            b"76" => Some(Milestone),
            b"77" => Some(WorkPackage),
            b"78" => Some(PlanningPackage),
            b"79" => Some(CostAccount),
            b"80" => Some(ChargeNumber),
            b"81" => Some(Code81),
            b"82" => Some(Code82),
            b"83" => Some(Code83),
            b"84" => Some(Code84),
            b"85" => Some(Code85),
            b"86" => Some(OperationNumber),
            b"87" => Some(FunctionalCategory),
            b"88" => Some(WorkCenter),
            b"89" => Some(AssemblyNumber),
            b"90" => Some(SubassemblyNumber),
            b"91" => Some(CostElement),
            b"92" => Some(ChangeDocumentNumber),
            b"93" => Some(FundsAuthorization),
            b"94" => Some(FileIdentificationNumber),
            b"95" => Some(Code95),
            b"96" => Some(StockCertificateNumber),
            b"97" => Some(PackageNumber),
            b"98" => Some(ContainerPackagingSpecificationNumber),
            b"99" => Some(RateConferenceIdCode),
            b"A0" => Some(AdvertiserNumber),
            b"A1" => Some(AnalysisNumberTestNumber),
            b"A2" => Some(DisabilityInsuranceAccountNumber),
            b"A3" => Some(AssignmentNumber),
            b"A4" => Some(DisabilityInsurancePolicyNumber),
            b"A5" => Some(EducationalInstitutionIdentificationNumber),
            b"A7" => Some(CodeA7),
            b"A8" => Some(CodeA8),
            b"A9" => Some(HealthInsuranceAccountNumber),
            b"AA" => Some(AccountsReceivableStatementNumber),
            b"AAA" => Some(DistributorsSplitAgentNumber),
            b"AAB" => Some(FundManagersReferenceNumber),
            b"AAC" => Some(AgencyHierarchicalLevel),
            b"AAD" => Some(OfficerLicenseNumber),
            b"AAE" => Some(PreviousDistributorNumber),
            b"AAF" => Some(InterviewerId),
            b"AAG" => Some(MilitaryId),
            b"AAH" => Some(OptionPolicyNumber),
            b"AAI" => Some(PayrollAccountNumber),
            b"AAJ" => Some(PriorContractNumber),
            b"AAK" => Some(WorksiteNumber),
            b"AAL" => Some(AgentNumber),
            b"AAM" => Some(Treaty),
            b"AAN" => Some(AssociatedCaseControlNumber),
            b"AAO" => Some(CarrierAssignedCode),
            b"AAP" => Some(DealerNumber),
            b"AAQ" => Some(DirectoryNumber),
            b"AAR" => Some(DistributorAssignedTransactionNumber),
            b"AAS" => Some(DistributorAssignedOrderNumber),
            b"AAT" => Some(DistributorsAccountNumber),
            b"AAU" => Some(GeneralAgencyNumber),
            b"AAV" => Some(LaboratoryNumber),
            b"AAW" => Some(AgencyAssignedNumber),
            b"AAX" => Some(ListBillNumber),
            b"AAY" => Some(AccountingPeriodReference),
            b"AAZ" => Some(ParamedicalIdNumber),
            b"AB" => Some(AcceptableSourcePurchaserId),
            b"ABA" => Some(PayrollNumber),
            b"ABB" => Some(PersonalIdNumber),
            b"ABC" => Some(PolicyLinkNumber),
            b"ABD" => Some(SecondaryPolicyNumber),
            b"ABE" => Some(SpecialQuoteNumber),
            b"ABF" => Some(NationalPropertyRegistrySystemLevel1),
            b"ABG" => Some(NationalPropertyRegistrySystemLevel2),
            b"ABH" => Some(InvestorAssignedIdentificationNumber),
            b"ABI" => Some(MotorFuelCertificateNumber),
            b"ABJ" => Some(CodeABJ),
            b"ABK" => Some(MortgageElectronicRegistrationSystemOrganization),
            b"ABL" => Some(SellerLoanNumber),
            b"ABM" => Some(SubServicerLoanNumber),
            b"ABN" => Some(NationalPropertyRegistrySystemLevel3),
            b"ABO" => Some(StateHazardousWasteEntity),
            b"ABP" => Some(BankruptcyProcedureNumber),
            b"ABQ" => Some(NationalBusinessIdentificationNumber),
            b"ABR" => Some(CodeABR),
            b"ABS" => Some(VesselName),
            b"ABT" => Some(SecurityInstrumentNumber),
            b"ABU" => Some(AssignmentRecordingNumber),
            b"ABV" => Some(BookNumber),
            b"ABW" => Some(BusinessTaxNumber),
            b"ABX" => Some(NorthAmericanIndustrialClassificationSystemCode2),
            b"ABY" => Some(CentersForMedicareAndMedicaidServicesPlanId),
            b"ABZ" => Some(EmploymentVisa),
            b"AC" => Some(AirCargoTransferManifest),
            b"ACA" => Some(GrowthFactorReference),
            b"ACB" => Some(Region),
            b"ACC" => Some(Status),
            b"ACD" => Some(ClassCode),
            b"ACE" => Some(ServiceRequestNumber),
            b"ACF" => Some(SupplementNumber),
            b"ACG" => Some(PreviousTicketNumber),
            b"ACH" => Some(OneCallAgencyTicketNumber),
            b"ACI" => Some(TicketNumber),
            b"ACJ" => Some(BillOfMaterialRevisionNumber),
            b"ACK" => Some(DrawingRevisionNumber),
            b"ACL" => Some(ApplicationTransactionReferenceNumber),
            b"ACM" => Some(RelatedObjectIdentificationNumber),
            b"ACN" => Some(CommonAccessReferenceNumber),
            b"ACO" => Some(FirstTransferNumber),
            b"ACP" => Some(ContinuousTransferNumber),
            b"ACQ" => Some(LastTransferNumber),
            b"ACR" => Some(CodeACR),
            b"ACS" => Some(SocietyOfPropertyInformationCompilersAndAnalysts),
            b"ACT" => Some(AccountingCode),
            b"ACU" => Some(GreenCard),
            b"ACV" => Some(AgencyAssignedEmployeeId),
            b"ACW" => Some(Passport),
            b"ACX" => Some(UnemploymentInsuranceNumber),
            b"ACY" => Some(NorthAmericanIndustrialClassificationSystemCode1),
            b"ACZ" => Some(OccupationCode),
            b"AD" => Some(AcceptableSourceDunsNumber),
            b"ADA" => Some(CodeADA),
            b"ADB" => Some(MasterPropertyNumber),
            b"ADC" => Some(ProjectPropertyNumber),
            b"ADD" => Some(UnitPropertyNumber),
            b"ADE" => Some(AssociatedPropertyNumber),
            b"ADF" => Some(AssociatedNumberForLimitedCommonElementParking),
            b"ADG" => Some(AssociatedNumberForUnitParking),
            b"ADH" => Some(AssociatedNumberForJoinedUnitNotReSubdivided),
            b"ADI" => Some(ProcessorIdentificationNumber),
            b"ADJ" => Some(OccupationClassificationCode),
            b"ADK" => Some(EmployeeTaxFilingStatusCode),
            b"ADL" => Some(InsuredLocation),
            b"ADM" => Some(AirDimensionCode),
            b"ADN" => Some(SelfInsuranceIdentificationNumber),
            b"ADO" => Some(SelfInsurerOrganizationType),
            b"ADP" => Some(SelfInsurerAuthorizationTypeCode),
            b"ADQ" => Some(CountyBusinessRegistrationNumber),
            b"ADR" => Some(PostalTemplate),
            b"ADS" => Some(ReducedEarningWeek),
            b"ADT" => Some(FullDenialReason),
            b"ADU" => {
                Some(FederalEnergyRegulatoryCommissionCertificateOfPublicConvenience)
            }
            b"ADV" => Some(Suspension),
            b"ADW" => Some(ManagedCareOrganizationCode),
            b"ADX" => Some(ManagedCareOrganizationIdentificationNumber),
            b"ADY" => Some(PublicUtilitiesCommissionCertificateOfPublicConvenience),
            b"ADZ" => Some(RetailMerchantsCertificationNumber),
            b"AE" => Some(CodeAE),
            b"AEA" => Some(CodeAEA),
            b"AEB" => Some(CodeAEB),
            b"AEC" => Some(GovernmentRegistrationNumber),
            b"AED" => Some(JudicialNumber),
            b"AEE" => Some(CodeAEE),
            b"AEF" => Some(PassportNumber),
            b"AEG" => Some(PatronNumber),
            b"AEH" => Some(CodeAEH),
            b"AEI" => Some(CodeAEI),
            b"AEJ" => Some(CodeAEJ),
            b"AEK" => Some(TokyoShokoResearchBusiness),
            b"AEL" => Some(CodeAEL),
            b"AEM" => Some(DistributionCenterNumber),
            b"AEN" => Some(CodeAEN),
            b"AEO" => Some(PublicDeedNumber),
            b"AEP" => Some(StockExchangeCode),
            b"AEQ" => Some(SecretaryOfStateAssignedIdentificationNumber),
            b"AER" => Some(DepartmentWhereInjuryOccurredIdentification),
            b"AES" => Some(BureauOfLaborAndStatisticsSchedule),
            b"AET" => Some(StateCharterNumber),
            b"AEU" => Some(EmployeeNonEmployeeClassificationQualifier),
            b"AEV" => Some(FullTimePartTimeEmployeeClassificationQualifier),
            b"AEX" => Some(PremiumAuditPriority),
            b"AEY" => Some(PremiumAuditPurpose),
            b"AEZ" => Some(PremiumAuditType),
            b"AF" => Some(AirlinesFlightIdentificationNumber),
            b"AFA" => Some(SplitPremiumAuditChange),
            b"AFB" => Some(SublineOfInsurance),
            b"AFC" => Some(VerificationSourceCode),
            b"AFD" => Some(UnderwritingAlertReferenceCode),
            b"AFE" => Some(CommercialPrivatePassengerVehicleQualifier),
            b"AFF" => Some(VehicleBusinessUseQualifier),
            b"AFG" => Some(VehicleSizeClassQualifier),
            b"AFH" => Some(VehicleRadiusOfOperationQualifier),
            b"AFI" => Some(TrailerTypeQualifier),
            b"AFJ" => Some(StateSalesTaxIdentificationNumber),
            b"AFK" => Some(CardIssuerTransactionCode),
            b"AFL" => Some(CardBillingTypeCode),
            b"AFM" => Some(ClientCompanyCode),
            b"AFN" => Some(CodeAFN),
            b"AFO" => Some(CardAccountTypeCode),
            b"AFP" => Some(CardAccountStatusCode),
            b"AFQ" => Some(CardAccountReportingLevel),
            b"AFR" => Some(CardAccountReporting),
            b"AFS" => Some(CodeAFS),
            b"AFT" => Some(FeeSchedule),
            b"AFU" => Some(CodeAFU),
            b"AFV" => Some(StateControlledSubstanceLicenseNumber),
            b"AFW" => Some(PointOfOrigination),
            b"AFX" => Some(PointOfDestination),
            b"AFY" => Some(AssessmentNumber),
            b"AFZ" => Some(CertificateNumber),
            b"AG" => Some(AgentsShipmentNumber),
            b"AGA" => Some(StateOrProvinceAssignedBusinessRegistryNumber),
            b"AGB" => Some(MunicipalityAssignedBusinessRegistryNumber),
            b"AGC" => Some(CodeAGC),
            b"AGD" => Some(CodeAGD),
            b"AGH" => Some(LenderUse),
            b"AGI" => Some(GuarantorUse),
            b"AGJ" => Some(SchoolUse),
            b"AGK" => Some(ReservationSystemCode),
            b"AGL" => Some(OrderOriginationCode),
            b"AGM" => Some(FolioNumber),
            b"AGN" => Some(CorporateIdentificationCode),
            b"AGO" => Some(CodeAGO),
            b"AGP" => Some(ConjunctionTravelTicket),
            b"AGQ" => Some(ListTracking),
            b"AH" => Some(AgreementNumber),
            b"AHC" => Some(AirHandlingCode),
            b"AI" => Some(AssociatedInvoices),
            b"AJ" => Some(AccountsReceivableCustomerAccount),
            b"AK" => Some(CodeAK),
            b"AL" => Some(CodeAL),
            b"ALC" => Some(AgencyLocationCode),
            b"ALG" => Some(TitleCompanyCodeBookReference),
            b"ALH" => Some(TitleDocumentSchedule),
            b"ALI" => Some(RecordingNumber),
            b"ALJ" => Some(TitlePolicyNumber),
            b"ALR" => Some(AlienRegistrationNumber),
            b"ALS" => Some(AlternativeListId),
            b"ALT" => Some(AlterationNumber),
            b"AM" => Some(CodeAM),
            b"AN" => Some(AssociatedPurchaseOrders),
            b"AO" => Some(AppointmentNumber),
            b"AP" => Some(AccountsReceivableNumber),
            b"APC" => Some(AmbulatoryPaymentClassification),
            b"API" => Some(CodeAPI),
            b"AQ" => Some(AccessCode),
            b"AR" => Some(ArrivalCode),
            b"AS" => Some(AcceptableSourceSupplierId),
            b"ASL" => Some(CodeASL),
            b"ASP" => Some(AnimalSpecies),
            b"AST" => Some(AnimalStrain),
            b"AT" => Some(AppropriationNumber),
            b"ATC" => Some(MaintenanceAvailabilityType),
            b"AU" => Some(AuthorizationToMeetCompetitionNumber),
            b"AV" => Some(HealthInsuranceRatingAccountNumber),
            b"AW" => Some(AirWaybillNumber),
            b"AX" => Some(CodeAX),
            b"AY" => Some(FloorPlanApprovalNumber),
            b"AZ" => Some(HealthInsurancePolicyNumber),
            b"B1" => Some(LesseeBillCodeNumber),
            b"B2" => Some(AxleRatio),
            b"B3" => Some(PreferredProviderOrganizationNumber),
            b"B4" => Some(BilateralCarServiceAgreements),
            b"B5" => Some(HealthInsuranceRatingSuffixCode),
            b"B6" => Some(LifeInsuranceBillingAccountNumber),
            b"B7" => Some(LifeInsurancePolicyNumber),
            b"B8" => Some(LifeInsuranceBillingSuffixCode),
            b"B9" => Some(RetirementPlanAccountNumber),
            b"BA" => Some(RetirementPlanPolicyNumber),
            b"BAA" => Some(FranchiseTaxAccountNumber),
            b"BAB" => Some(CertificateOfIncorporationNumber),
            b"BAC" => Some(BeamAssemblyCode),
            b"BAD" => Some(StateTaxIdentificationNumber),
            b"BAE" => Some(CharterNumber),
            b"BAF" => Some(ReceiptNumber),
            b"BAG" => Some(WithdrawalAccountNumber),
            b"BAH" => Some(DepositAccountNumber),
            b"BAI" => Some(BusinessIdentificationNumber),
            b"BAJ" => Some(CodeBAJ),
            b"BAK" => Some(CodeBAK),
            b"BB" => Some(AuthorizationNumber),
            b"BC" => Some(BuyersContractNumber),
            b"BCI" => Some(BasicContractLineItemNumber),
            b"BCN" => Some(BirthCertificateNumber),
            b"BCP" => Some(BorderCrossingPermitNumber),
            b"BD" => Some(BidNumber),
            b"BDG" => Some(BadgeNumber),
            b"BDN" => Some(BuildDirectiveNumber),
            b"BE" => Some(BusinessActivity),
            b"BEN" => Some(BrokerEntryNumber),
            b"BF" => Some(BillingCenterIdentification),
            b"BG" => Some(BeginningSerialNumber),
            b"BH" => Some(LeaseScheduleNumberBlanket),
            b"BI" => Some(BondedCarrierInternalRevenueServiceIdentificationNumber),
            b"BJ" => Some(CarriersCustomsBondNumber),
            b"BK" => Some(BrokersOrderNumber),
            b"BKT" => Some(BankTelegraphicNumber),
            b"BL" => Some(GovernmentBillOfLading),
            b"BLT" => Some(BillingType),
            b"BM" => Some(BillOfLadingNumber),
            b"BMM" => Some(BeginMileMarker),
            b"BN" => Some(BookingNumber),
            b"BO" => Some(BinLocationNumber),
            b"BOI" => Some(BinaryObject),
            b"BP" => Some(AdjustmentControlNumber),
            b"BQ" => Some(HealthMaintenanceOrganizationCodeNumber),
            b"BR" => Some(BrokerOrSalesOfficeNumber),
            b"BS" => Some(SplitBookingNumber),
            b"BT" => Some(BatchNumber),
            b"BU" => Some(BuyersApprovalMark),
            b"BV" => Some(CodeBV),
            b"BW" => Some(BlendedWithBatchNumber),
            b"BX" => Some(BuyersShipmentMarkNumber),
            b"BY" => Some(RepairCategoryNumber),
            b"BZ" => Some(ComplaintCode),
            b"C0" => Some(CanadianSocialInsuranceNumber),
            b"C1" => Some(CustomerMaterialSpecificationNumber),
            b"C2" => Some(CustomerProcessSpecificationNumber),
            b"C3" => Some(CustomerSpecificationNumber),
            b"C4" => Some(ChangeNumber),
            b"C5" => Some(CustomerTrackingNumberForLoanedMaterials),
            b"C6" => Some(CarnetNumber),
            b"C7" => Some(ContractLineItemNumber),
            b"C8" => Some(CorrectedContractNumber),
            b"C9" => Some(PreviousCreditDebitAdjustmentNumber),
            b"CA" => Some(CostAllocationReference),
            b"CAA" => Some(AccidentHistory),
            b"CAB" => Some(Chemical),
            b"CAC" => Some(DischargePointIdentification),
            b"CAD" => Some(EmissionUnitIdentificationNumber),
            b"CAE" => Some(FacilityFederalIdentificationNumber),
            b"CAF" => Some(LatitudeExpressedInDecimalDegrees),
            b"CAG" => Some(LongitudeExpressedInDecimalDegrees),
            b"CAH" => Some(CodeCAH),
            b"CAI" => Some(Process),
            b"CAJ" => Some(StackIdentificationNumber),
            b"CAK" => Some(FacilityStateIdentificationNumber),
            b"CAL" => Some(CodeCAL),
            b"CAM" => Some(CodeCAM),
            b"CAT" => Some(Category),
            b"CB" => Some(CombinedShipment),
            b"CBG" => Some(CensusBlockGroup),
            b"CC" => Some(ContractCoOpNumber),
            b"CD" => Some(CreditNoteNumber),
            b"CDN" => Some(CitizenshipDocumentNumber),
            b"CDT" => Some(ContractingDistrictTypeCode),
            b"CE" => Some(ClassOfContractCode),
            b"CF" => Some(FleetReferenceNumber),
            b"CFR" => Some(FederalRegulation),
            b"CG" => Some(ConsigneesOrderNumber),
            b"CH" => Some(CustomerCatalogNumber),
            b"CHR" => Some(Chromatograph),
            b"CI" => Some(UniqueConsignment),
            b"CID" => Some(CampusIdentificationNumber),
            b"CIR" => Some(CircuitNumber),
            b"CIT" => Some(Citation),
            b"CJ" => Some(ClauseNumber),
            b"CK" => Some(CheckNumber),
            b"CL" => Some(SellersCreditMemo),
            b"CLI" => Some(CoverageListId),
            b"CM" => Some(BuyersCreditMemo),
            b"CMN" => Some(ContinuousMoveNumber),
            b"CMP" => Some(CustomerMaintenancePeriodSequenceNumber),
            b"CMT" => Some(Component),
            b"CN" => Some(CodeCN),
            b"CNA" => Some(AssemblyControlNumber),
            b"CNO" => Some(CommitmentNumber),
            b"CNS" => Some(CanadianNationalStudentNumber),
            b"CO" => Some(CustomerOrderNumber),
            b"COL" => Some(CollocationIndicator),
            b"COT" => Some(CertificateOfTransportation),
            b"CP" => Some(ConditionOfPurchaseDocumentNumber),
            b"CPA" => Some(CanadianProvinceOperatingAuthorityNumber),
            b"CPD" => Some(DiscrepantContainerPackagingNumber),
            b"CPR" => Some(RequiredContainerPackagingNumber),
            b"CPT" => Some(CurrentProceduralTerminologyCode),
            b"CQ" => Some(CustomshouseBrokerLicenseNumber),
            b"CR" => Some(CustomerReferenceNumber),
            b"CRN" => Some(CasualtyReportNumber),
            b"CRS" => Some(CasualtyReportSerialNumber),
            b"CS" => Some(ConditionOfSaleDocumentNumber),
            b"CSC" => Some(Cs54KeyTrainIndicatorCode),
            b"CSG" => Some(Cs54KeyTrainIndicatorGroupName),
            b"CST" => Some(CensusStateCode),
            b"CT" => Some(ContractNumber),
            b"CTS" => Some(CensusTractSuffix),
            b"CU" => Some(ClearTextClause),
            b"CUB" => Some(CodeCUB),
            b"CV" => Some(CoilNumber),
            b"CVS" => Some(CommercialVehicleSafetyAssuranceNumber),
            b"CW" => Some(CanadianWheatBoardPermitNumber),
            b"CX" => Some(ConsignmentClassificationId),
            b"CY" => Some(CommercialRegistrationNumber),
            b"CYC" => Some(PeriodicityCode),
            b"CZ" => Some(CodeCZ),
            b"D0" => Some(DataReliabilityCode),
            b"D1" => Some(DrugEnforcementAdministrationOrderBlankNumber),
            b"D2" => Some(SupplierDocumentIdentificationNumber),
            b"D3" => Some(NationalCouncilForPrescriptionDrugProgramsPharmacyNumber),
            b"D4" => Some(CutNumber),
            b"D5" => Some(DyeLotNumber),
            b"D6" => Some(DuplicateBillNumber),
            b"D7" => Some(CoverageCode),
            b"D8" => Some(LossReportNumber),
            b"D9" => Some(ClaimNumber),
            b"DA" => Some(DomicileBranchNumber),
            b"DAI" => Some(DistrictAssignedId),
            b"DAN" => Some(DeliveryAppointmentNumber),
            b"DB" => Some(BuyersDebitMemo),
            b"DC" => Some(DealerPurchaseOrderNumber),
            b"DD" => Some(DocumentIdentificationCode),
            b"DE" => Some(DepositorNumber),
            b"DF" => Some(CodeDF),
            b"DG" => Some(DrawingNumber),
            b"DH" => Some(DrugEnforcementAdministrationNumber),
            b"DHH" => Some(CodeDHH),
            b"DI" => Some(DistributorInvoiceNumber),
            b"DIS" => Some(DistrictNumber),
            b"DJ" => Some(DeliveryTicketNumber),
            b"DK" => Some(DockNumber),
            b"DL" => Some(SellersDebitMemo),
            b"DM" => Some(AssociatedProductNumber),
            b"DN" => Some(DraftNumber),
            b"DNR" => Some(DepositNumber),
            b"DNS" => Some(CodeDNS),
            b"DO" => Some(DeliveryOrderNumber),
            b"DOA" => Some(CodeDOA),
            b"DOC" => Some(CodeDOC),
            b"DOE" => Some(CodeDOE),
            b"DOI" => Some(CodeDOI),
            b"DOJ" => Some(CodeDOJ),
            b"DOL" => Some(CodeDOL),
            b"DON" => Some(DensityOrderNumber),
            b"DOS" => Some(CodeDOS),
            b"DOT" => Some(CodeDOT),
            b"DP" => Some(DepartmentNumber),
            b"DQ" => Some(DeliveryQuoteNumber),
            b"DR" => Some(DockReceiptNumber),
            b"DRN" => Some(DrainholeNumber),
            b"DS" => Some(CodeDS),
            b"DSC" => Some(DepartureFromSpecificationClassCode),
            b"DSI" => Some(DepartureFromSpecificationNumber),
            b"DST" => Some(DepartureFromSpecificationTypeCode),
            b"DT" => Some(DownstreamShipperContractNumber),
            b"DTS" => Some(CodeDTS),
            b"DU" => Some(Dependents),
            b"DUN" => Some(CodeDUN),
            b"DV" => Some(DiversionAuthorityNumber),
            b"DW" => Some(DepositSequenceNumber),
            b"DX" => Some(DepartmentAgencyNumber),
            b"DY" => Some(CodeDY),
            b"DZ" => Some(CodeDZ),
            b"E00" => Some(CourseSectionNumber),
            b"E1" => Some(EmergencyOrderNumber),
            b"E01" => Some(NonTeachingCredentialFieldCodes),
            b"E2" => Some(PartCausingRepairNumber),
            b"E02" => Some(CodeE02),
            b"E3" => Some(ExpansionOnEffectOfChangeNumber),
            b"E4" => Some(ChargeCardNumber),
            b"E5" => Some(ClaimantsClaimNumber),
            b"E6" => Some(BackoutProcedureCode),
            b"E7" => Some(ServiceBulletinNumber),
            b"E8" => Some(CodeE8),
            b"E9" => Some(AttachmentCode),
            b"EA" => Some(MedicalRecordIdentificationNumber),
            b"EB" => Some(EmbargoPermitNumber),
            b"EC" => Some(Circular),
            b"ECA" => Some(Fund),
            b"ECB" => Some(Ballot),
            b"ECC" => Some(LegislativeIdentificationNumber),
            b"ECD" => Some(LobbiedActivity),
            b"ECE" => Some(PetitionNumber),
            b"ECF" => Some(RelatedFormNumber),
            b"ECJ" => Some(CodeECJ),
            b"ED" => Some(ExportDeclaration),
            b"EDA" => Some(CodeEDA),
            b"EE" => Some(ElectionDistrict),
            b"EF" => Some(ElectronicFundsTransferIdNumber),
            b"EG" => Some(EndingSerialNumber),
            b"EH" => Some(FinancialClassificationCode),
            b"EI" => Some(EmployersIdentificationNumber),
            b"EII" => Some(CodeEII),
            b"EJ" => Some(PatientAccountNumber),
            b"EK" => Some(CodeEK),
            b"EL" => Some(ElectronicDevicePinNumber),
            b"EM" => Some(ElectronicPaymentReferenceNumber),
            b"EMM" => Some(EndMileMarker),
            b"EN" => Some(EmbargoNumber),
            b"END" => Some(EndorsementNumber),
            b"EO" => Some(SubmitterIdentificationNumber),
            b"EP" => Some(ExportPermitNumber),
            b"EPA" => Some(CodeEPA),
            b"EPB" => Some(EnvironmentalProtectionAgencyTransporterIdentificationNumber),
            b"EPC" => Some(EmployerPayrollCodeLists),
            b"EQ" => Some(EquipmentNumber),
            b"ER" => Some(ContainerOrEquipmentReceiptNumber),
            b"ES" => Some(EmployersSocialSecurityNumber),
            b"ESN" => Some(EstimateSequenceNumber),
            b"ET" => Some(ExcessTransportation),
            b"EU" => Some(EndUsersPurchaseOrderNumber),
            b"EV" => Some(ReceiverIdentificationNumber),
            b"EVI" => Some(EventIdentification),
            b"EW" => Some(MammographyCertificationNumber),
            b"EX" => Some(EstimateNumber),
            b"EXP" => Some(ExposureStateCode),
            b"EY" => Some(ReceiverSubIdentificationNumber),
            b"EZ" => Some(ElectronicDataInterchangeAgreementNumber),
            b"F1" => Some(VersionCodeNational),
            b"F2" => Some(VersionCodeLocal),
            b"F3" => Some(SubmissionNumber),
            b"F4" => Some(FacilityCertificationNumber),
            b"F5" => Some(MedicareVersionCode),
            b"F6" => Some(CodeF6),
            b"F7" => Some(CodeF7),
            b"F8" => Some(OriginalReferenceNumber),
            b"F9" => Some(FreightPayorReferenceNumber),
            b"FA" => Some(CodeFA),
            b"FAN" => Some(FannieMaeSellerServicerNumber),
            b"FB" => Some(FileTransferFormNumber),
            b"FC" => Some(FilerCodeIssuedByCustoms),
            b"FCN" => Some(AssignedContractNumber),
            b"FD" => Some(FilerCodeIssuedByBureauOfCensus),
            b"FE" => Some(FailureMechanismNumber),
            b"FEN" => Some(ForeignEntryNumber),
            b"FF" => Some(FilmNumber),
            b"FG" => Some(FundIdentificationNumber),
            b"FH" => Some(ClinicNumber),
            b"FHC" => Some(CodeFHC),
            b"FHO" => Some(FederalHousingAdministrationOriginatorIdentification),
            b"FI" => Some(File),
            b"FJ" => Some(LineItemControlNumber),
            b"FK" => Some(FinishLotNumber),
            b"FL" => Some(FineLineClassification),
            b"FLZ" => Some(FloodZone),
            b"FM" => Some(CodeFM),
            b"FMG" => Some(CodeFMG),
            b"FMP" => Some(FacilityMeasurementPointNumber),
            b"FN" => Some(ForwardersAgentsReferenceNumber),
            b"FND" => Some(FinderNumber),
            b"FO" => Some(DrugFormularyNumber),
            b"FP" => Some(ForestryPermitNumber),
            b"FQ" => Some(FormNumber),
            b"FR" => Some(FreightBillNumber),
            b"FRN" => Some(FreddieMacSellerServicerNumber),
            b"FS" => Some(FinalSequenceNumber),
            b"FSC" => Some(FundSourceCode),
            b"FSN" => Some(AssignedSequenceNumber),
            b"FT" => Some(ForeignTradeZone),
            b"FTN" => Some(PremarketNotificationNumber),
            b"FTP" => Some(CodeFTP),
            b"FTZ" => Some(CodeFTZ),
            b"FU" => Some(FundCode),
            b"FV" => Some(CodeFV),
            b"FW" => Some(StateLicenseIdentificationNumber),
            b"FWC" => Some(FinalWorkCandidateNumber),
            b"FX" => Some(FailureAnalysisReportNumber),
            b"FY" => Some(ClaimOfficeNumber),
            b"FZ" => Some(ProcessorsInvoiceNumber),
            b"G1" => Some(PriorAuthorizationNumber),
            b"G2" => Some(ProviderCommercialNumber),
            b"G3" => Some(PredeterminationOfBenefitsIdentificationNumber),
            b"G4" => Some(CodeG4),
            b"G5" => Some(ProviderSiteNumber),
            b"G6" => Some(PayerAssignedResubmissionReferenceNumber),
            b"G7" => Some(ResubmissionReasonCode),
            b"G8" => Some(ResubmissionNumber),
            b"G9" => Some(SecondaryEmployeeIdentificationNumber),
            b"GA" => Some(GovernmentAdvanceProgress),
            b"GB" => Some(GrainBlockNumber),
            b"GC" => Some(GovernmentContractNumber),
            b"GD" => Some(ReturnGoodsBillOfLadingNumber),
            b"GE" => Some(GeographicNumber),
            b"GF" => Some(SpecialtyLicenseNumber),
            b"GG" => Some(GaugeTicketNumber),
            b"GH" => Some(IdentificationCardSerialNumber),
            b"GI" => Some(SecondaryProviderNumber),
            b"GJ" => Some(CornboreCertificationNumber),
            b"GK" => Some(ThirdPartyReferenceNumber),
            b"GL" => Some(GeographicDestinationZoneNumber),
            b"GM" => Some(LoanAcquisitionNumber),
            b"GN" => Some(FolderNumber),
            b"GO" => Some(Exhibit),
            b"GP" => Some(GovernmentPriorityNumber),
            b"GQ" => Some(InternalPurchaseOrderReleaseNumber),
            b"GR" => Some(GrainOrderReferenceNumber),
            b"GS" => Some(CodeGS),
            b"GT" => Some(GoodsAndServiceTaxRegistrationNumber),
            b"GU" => Some(InternalPurchaseOrderItemNumber),
            b"GV" => Some(ThirdPartyPurchaseOrderNumber),
            b"GW" => Some(ThirdPartyPurchaseOrderReleaseNumber),
            b"GWS" => Some(GroupWorkCandidateSequenceNumber),
            b"GX" => Some(ThirdPartyPurchaseOrderItemNumber),
            b"GY" => Some(EmptyRepositioningNumber),
            b"GZ" => Some(GeneralLedgerAccount),
            b"H1" => Some(HighFabricationAuthorizationNumber),
            b"H2" => Some(HighRawMaterialAuthorizationNumber),
            b"H3" => Some(GravitySourceMeterNumber),
            b"H5" => Some(SpecialClause),
            b"H6" => Some(QualityClause),
            b"H7" => Some(StandardClause),
            b"H8" => Some(CodeH8),
            b"H9" => Some(PaymentHistoryReferenceNumber),
            b"HA" => Some(CompetentAuthority),
            b"HB" => Some(CodeHB),
            b"HC" => Some(HeatCode),
            b"HD" => Some(DepartmentOfTransportationHazardousNumber),
            b"HE" => Some(HazardousExemptionNumber),
            b"HF" => Some(EngineeringDataList),
            b"HG" => Some(CivilActionNumber),
            b"HH" => Some(FiscalCode),
            b"HHT" => Some(TypeOfHouseholdGoodsCode),
            b"HI" => Some(CodeHI),
            b"HJ" => Some(IdentityCardNumber),
            b"HK" => Some(JudgmentNumber),
            b"HL" => Some(SirenNumber),
            b"HM" => Some(SiretNumber),
            b"HMB" => Some(HomeMortgageDisclosureActBlockNumberArea),
            b"HN" => Some(HazardousCertificationNumber),
            b"HO" => Some(ShippersHazardousNumber),
            b"HP" => Some(CodeHP),
            b"HPI" => Some(CentersForMedicareAndMedicaidServicesNationalProvider),
            b"HQ" => Some(ReinsuranceReference),
            b"HR" => Some(Horsepower),
            b"HS" => Some(CodeHS),
            b"HT" => Some(CodeOfFederalRegulations),
            b"HU" => Some(TypeOfEscrowNumber),
            b"HUD" => Some(CodeHUD),
            b"HV" => Some(EscrowFileNumber),
            b"HW" => Some(HighWideFileNumber),
            b"HX" => Some(AutoLossItemNumber),
            b"HY" => Some(PropertyLossItemNumber),
            b"HZ" => Some(CodeHZ),
            b"I1" => Some(OwningBureauIdentificationNumber),
            b"I2" => Some(CodeI2),
            b"I3" => Some(NonAmericanIdentificationNumber),
            b"I4" => Some(CreditCounselingIdentificationNumber),
            b"I5" => Some(InvoiceIdentification),
            b"I7" => Some(CreditReportNumber),
            b"I9" => Some(Pollutant),
            b"IA" => Some(InternalVendorNumber),
            b"IB" => Some(InBondNumber),
            b"IC" => Some(InboundToParty),
            b"ICD" => Some(CodeICD),
            b"ID" => Some(InsuranceCertificateNumber),
            b"IE" => Some(InterchangeAgreementNumber),
            b"IF" => Some(IssueNumber),
            b"IFC" => Some(InitialFailureClaim),
            b"IFT" => Some(InternationalFuelTaxAgreementAccountNumber),
            b"IG" => Some(InsurancePolicyNumber),
            b"IH" => Some(InitialDealerClaimNumber),
            b"II" => Some(InitialSampleInspectionReportNumber),
            b"IID" => Some(Image),
            b"IJ" => Some(CodeIJ),
            b"IK" => Some(InvoiceNumber),
            b"IL" => Some(InternalOrderNumber),
            b"IM" => Some(CodeIM),
            b"IMP" => Some(CodeIMP),
            b"IMS" => Some(CodeIMS),
            b"IN" => Some(ConsigneesInvoiceNumber),
            b"IND" => Some(InvestigatorialNewDrugNumber),
            b"IO" => Some(InboundToOrOutboundFromParty),
            b"IP" => Some(InspectionReportNumber),
            b"IQ" => Some(EndItem),
            b"IR" => Some(IntraPlantRouting),
            b"IRN" => Some(ImportersReferenceNumberToLetterOfCredit),
            b"IRP" => Some(InternationalRegistrationPlanAccountNumber),
            b"IS" => Some(InvoiceNumberSuffix),
            b"ISC" => Some(CodeISC),
            b"ISN" => Some(InternationalRegistrationPlanStickerNumber),
            b"ISS" => Some(InspectionAndSurveySequenceNumber),
            b"IT" => Some(InternalCustomerNumber),
            b"ITI" => Some(InitialTroubleIndication),
            b"IU" => Some(BargePermitNumber),
            b"IV" => Some(SellersInvoiceNumber),
            b"IW" => Some(PartInterchangeability),
            b"IX" => Some(ItemNumber),
            b"IZ" => Some(InsuredParcelPostNumber),
            b"J0" => Some(Proceeding),
            b"J1" => Some(Creditor),
            b"J2" => Some(Attorney),
            b"J3" => Some(Judge),
            b"J4" => Some(Trustee),
            b"J5" => Some(OriginatingCase),
            b"J6" => Some(AdversaryCase),
            b"J7" => Some(LeadCase),
            b"J8" => Some(JointlyAdministeredCase),
            b"J9" => Some(SubstantivelyConsolidatedCase),
            b"JA" => Some(BeginningJobSequenceNumber),
            b"JB" => Some(CodeJB),
            b"JC" => Some(Review),
            b"JCS" => Some(JointCreditSpecificationNumber),
            b"JD" => Some(UserIdentification),
            b"JE" => Some(EndingJobSequenceNumber),
            b"JF" => Some(AutomatedUnderwritingReferenceNumber),
            b"JH" => Some(Tag),
            b"JI" => Some(MultipleListingServiceArea),
            b"JK" => Some(MultipleListingServiceSubArea),
            b"JL" => Some(Packet),
            b"JM" => Some(MultipleListingServiceMapXCoordinate),
            b"JN" => Some(MultipleListingServiceMapYCoordinate),
            b"JO" => Some(MultipleListingNumber),
            b"JP" => Some(MultipleListingServiceBookType),
            b"JQ" => Some(Elevation),
            b"JR" => Some(PropertyComponentLocation),
            b"JS" => Some(JobSequenceNumber),
            b"JT" => Some(CodeJT),
            b"JU" => Some(PriorPhoneNumber),
            b"JV" => Some(PriorHealthIndustryNumber),
            b"JW" => Some(CodeJW),
            b"JX" => Some(PriorPostalZipCode),
            b"JY" => Some(OriginOfShipmentHarmonizedBasedCode),
            b"JZ" => Some(GoverningClassCode),
            b"K0" => Some(ApprovalCode),
            b"K1" => Some(ForeignMilitarySalesNoticeNumber),
            b"K2" => Some(CertifiedMailNumber),
            b"K3" => Some(RegisteredMailNumber),
            b"K4" => Some(CriticalityDesignator),
            b"K5" => Some(TaskOrder),
            b"K6" => Some(PurchaseDescription),
            b"K7" => Some(ParagraphNumber),
            b"K8" => Some(ProjectParagraphNumber),
            b"K9" => Some(InquiryRequestNumber),
            b"KA" => Some(DistributionList),
            b"KAS" => Some(AssociatedContract),
            b"KB" => Some(BeginningKanbanSerialNumber),
            b"KC" => Some(ExhibitDistributionList),
            b"KCS" => Some(ConfirmationServiceContract),
            b"KD" => Some(SpecialInstructionsNumber),
            b"KE" => Some(EndingKanbanSerialNumber),
            b"KG" => Some(ForeclosingStatus),
            b"KH" => Some(TypeOfLawSuit),
            b"KI" => Some(TypeOfOutstandingJudgment),
            b"KII" => Some(ConfirmationIntraday),
            b"KJ" => Some(TaxLienJurisdiction),
            b"KK" => Some(DeliveryReference),
            b"KL" => Some(ContractReference),
            b"KM" => Some(RentalAccountNumber),
            b"KN" => Some(CensusAutomatedFilesId),
            b"KO" => Some(CustomsDrawbackEntryNumber),
            b"KP" => Some(HealthCertificateNumber),
            b"KQ" => Some(ProcuringAgency),
            b"KR" => Some(ResponseToARequestForQuotationReference),
            b"KRL" => Some(ReleaserContract),
            b"KRP" => Some(ReplacementShipperContract),
            b"KS" => Some(Solicitation),
            b"KSR" => Some(ServiceRequesterContract),
            b"KT" => Some(RequestForQuotationReference),
            b"KU" => Some(OfficeSymbol),
            b"KV" => Some(DistributionStatementCode),
            b"KW" => Some(Certification),
            b"KX" => Some(Representation),
            b"KY" => Some(CodeKY),
            b"KZ" => Some(CodeKZ),
            b"L0" => Some(CodeL0),
            b"L1" => Some(LettersOrNotes),
            b"L2" => Some(LocationOnProductCode),
            b"L3" => Some(LaborOperationNumber),
            b"L4" => Some(ProposalParagraphNumber),
            b"L5" => Some(SubexhibitLineItemNumber),
            b"L6" => Some(SubcontractLineItemNumber),
            b"L7" => Some(CustomersReleaseNumber),
            b"L8" => Some(ConsigneesReleaseNumber),
            b"L9" => Some(CustomersPartNumber),
            b"LA" => Some(ShippingLabelSerialNumber),
            b"LAA" => Some(LotteryAuthorityActivationNumber),
            b"LAN" => Some(LaneNumber),
            b"LB" => Some(Lockbox),
            b"LC" => Some(LeaseNumber),
            b"LD" => Some(LoanNumber),
            b"LE" => Some(LenderEntityNumber),
            b"LEN" => Some(LocationExceptionOrderNumber),
            b"LF" => Some(AssemblyLineFeedLocation),
            b"LG" => Some(LeaseScheduleNumber),
            b"LH" => Some(LongitudeExpressedInSeconds),
            b"LI" => Some(CodeLI),
            b"LIC" => Some(CodeLIC),
            b"LJ" => Some(LocalJurisdiction),
            b"LK" => Some(CodeLK),
            b"LL" => Some(LatitudeExpressedInSeconds),
            b"LM" => Some(ProductPeriodForWhichLaborCostsAreFirm),
            b"LMI" => Some(LocalMedia),
            b"LN" => Some(NonPickupLimitedTariffNumber),
            b"LO" => Some(LoadPlanningNumber),
            b"LOI" => Some(CodeLOI),
            b"LOS" => Some(LossConditions),
            b"LP" => Some(ForPickupLimitedFreightTariffNumber),
            b"LPK" => Some(LoanProspectorKeyNumber),
            b"LQ" => Some(CodeLQ),
            b"LR" => Some(LocalStudentIdentificationNumber),
            b"LS" => Some(BarCodedSerialNumber),
            b"LSD" => Some(LogisticsSupportDocumentationTypeCode),
            b"LT" => Some(LotNumber),
            b"LU" => Some(LocationNumber),
            b"LV" => Some(LicensePlateNumber),
            b"LVO" => Some(LevyingOfficerIdentification),
            b"LW" => Some(LocationWithinEquipment),
            b"LX" => Some(QualifiedProductsList),
            b"LY" => Some(DestinationOfShipmentHarmonizedBasedCode),
            b"LZ" => Some(LenderAccountNumber),
            b"M0" => Some(MexicanPedimentoNumber),
            b"M1" => Some(MaterialStorageLocation),
            b"M2" => Some(MajorForceProgram),
            b"M3" => Some(CropYear),
            b"M5" => Some(LeaseAgreementAmendmentNumberMaster),
            b"M6" => Some(MilitaryOrdnanceSecurityRiskNumber),
            b"M7" => Some(MedicalAssistanceCategory),
            b"M8" => Some(LimitedPartnershipIdentificationNumber),
            b"M9" => Some(TaxShelterNumber),
            b"MA" => Some(ShipNoticeManifestNumber),
            b"MB" => Some(MasterBillOfLading),
            b"MBS" => Some(CodeMBS),
            b"MBX" => Some(Mailbox),
            b"MC" => Some(MicrofilmNumber),
            b"MCC" => Some(CodeMCC),
            b"MCI" => Some(MotorCarrierIdentificationNumber),
            b"MCN" => Some(MornetPlusCaseNumber),
            b"MD" => Some(MagazineCode),
            b"MDN" => Some(HazardousWasteManifestDocumentNumber),
            b"ME" => Some(MessageAddressOrId),
            b"MF" => Some(ManufacturersPartNumber),
            b"MG" => Some(MeterNumber),
            b"MH" => Some(ManufacturingOrderNumber),
            b"MI" => Some(MillOrderNumber),
            b"MII" => Some(CodeMII),
            b"MIN" => Some(MornetPlusInstitutionNumber),
            b"MJ" => Some(ModelNumber),
            b"MK" => Some(ManifestKeyNumber),
            b"ML" => Some(MilitaryRankCivilianPayGradeNumber),
            b"MM" => Some(MasterLeaseAgreementNumber),
            b"MN" => Some(MicrNumber),
            b"MO" => Some(ManufacturingOperationNumber),
            b"MP" => Some(MultiplePOSOfAnInvoice),
            b"MPN" => Some(MarketingPlanIdentificationNumber),
            b"MQ" => Some(MeterProvingReportNumber),
            b"MR" => Some(MerchandiseTypeCode),
            b"MRC" => Some(EligibilityCategory),
            b"MRN" => Some(MothersMedicalRecordIdentificationNumber),
            b"MS" => Some(ManufacturersMaterialSafetyDataSheetNumber),
            b"MSL" => Some(MailSlot),
            b"MT" => Some(MeterTicketNumber),
            b"MU" => Some(CodeMU),
            b"MUI" => Some(MornetPlusUserIdentification),
            b"MV" => Some(MigrantNumber),
            b"MW" => Some(MilitaryCallNumber),
            b"MX" => Some(MaterialChangeNoticeNumber),
            b"MY" => Some(ModelYearNumber),
            b"MZ" => Some(MaintenanceRequestNumber),
            b"MZO" => Some(MultipleZoneOrderNumber),
            b"N0" => Some(NominationNumber),
            b"N1" => Some(LocalSchoolCourseNumber),
            b"N2" => Some(LocalSchoolDistrictCourseNumber),
            b"N3" => Some(StatewideCourseNumber),
            b"N4" => Some(CodeN4),
            b"N5" => Some(ProviderPlanNetworkIdentificationNumber),
            b"N6" => Some(PlanNetworkIdentificationNumber),
            b"N7" => Some(FacilityNetworkIdentificationNumber),
            b"N8" => Some(SecondaryHealthInsuranceIdentificationNumber),
            b"N9" => Some(DataAuthenticationNumber),
            b"NA" => Some(NorthAmericanHazardousClassificationNumber),
            b"NAS" => Some(CodeNAS),
            b"NB" => Some(LetterOfCreditNumber),
            b"NC" => Some(SecondaryCoverageCompanyNumber),
            b"ND" => Some(LetterOfCreditDraftNumber),
            b"NDA" => Some(AbbreviatedNewDrugApplicationNumber),
            b"NDB" => Some(NewDrugApplicationNumber),
            b"NE" => Some(LeaseRiderNumber),
            b"NF" => Some(CodeNF),
            b"NFC" => Some(NationalFloodInsuranceProgramCommunityName),
            b"NFD" => Some(NationalFloodInsuranceProgramCounty),
            b"NFM" => Some(NationalFloodInsuranceProgramMapNumber),
            b"NFN" => Some(NationalFloodInsuranceProgramCommunityNumber),
            b"NFS" => Some(NationalFloodInsuranceProgramState),
            b"NG" => Some(NaturalGasPolicyActCategoryCode),
            b"NH" => Some(RateCardNumber),
            b"NI" => Some(CodeNI),
            b"NJ" => Some(TechnicalDocumentNumber),
            b"NK" => Some(PriorCase),
            b"NL" => Some(TechnicalOrderNumber),
            b"NM" => Some(DiscounterRegistrationNumber),
            b"NMT" => Some(NominationModelType),
            b"NN" => Some(NonconformanceReportNumber),
            b"NO" => Some(NoOt5AuthorityZeroMileageRate),
            b"NP" => Some(PartialPaymentNumber),
            b"NQ" => Some(MedicaidRecipientIdentificationNumber),
            b"NR" => Some(ProgressPaymentNumber),
            b"NS" => Some(NationalStockNumber),
            b"NT" => Some(AdministratorsReferenceNumber),
            b"NTP" => Some(NonOriginatingThirdPartyNumber),
            b"NU" => Some(PendingCase),
            b"NW" => Some(AssociatedPolicyNumber),
            b"NX" => Some(RelatedNonconformanceNumber),
            b"NY" => Some(AgentClaimNumber),
            b"NZ" => Some(CriticalApplication),
            b"O1" => Some(OuterContinentalShelfAreaCode),
            b"O2" => Some(OuterContinentalShelfBlockNumber),
            b"O5" => Some(Ot5AuthorityConditionOrRestrictionOnCarHireRate),
            b"O7" => Some(CodeO7),
            b"O8" => Some(OriginalFiling),
            b"O9" => Some(ContinuationFiling),
            b"OA" => Some(OutletNumber),
            b"OB" => Some(OceanBillOfLading),
            b"OC" => Some(OceanContainerNumber),
            b"OD" => Some(OriginalReturnRequestReferenceNumber),
            b"OE" => Some(OpenAndPrepaidStationListNumber),
            b"OF" => Some(OperatorIdentificationNumber),
            b"OFF" => Some(Offer),
            b"OG" => Some(TerminationFiling),
            b"OH" => Some(OriginHouse),
            b"OI" => Some(OriginalInvoiceNumber),
            b"OIC" => Some(Object),
            b"OJ" => Some(AmendmentFiling),
            b"OK" => Some(OfferGroup),
            b"OL" => Some(OriginalShippersBillOfLadingNumber),
            b"OM" => Some(OceanManifest),
            b"ON" => Some(DealerOrderNumber),
            b"OOS" => Some(OutOfServiceNumber),
            b"OP" => Some(OriginalPurchaseOrder),
            b"OPE" => Some(CodeOPE),
            b"OPF" => Some(CodeOPF),
            b"OQ" => Some(OrderNumber),
            b"OR" => Some(OrderParagraphNumber),
            b"OS" => Some(OutboundFromParty),
            b"OT" => Some(SalesAllowanceNumber),
            b"OU" => Some(TariffSupplementNumber),
            b"OV" => Some(TariffSuffixNumber),
            b"OW" => Some(ServiceOrderNumber),
            b"OX" => Some(StatementNumber),
            b"OZ" => Some(ProductNumber),
            b"P1" => Some(PreviousContractNumber),
            b"P2" => Some(PreviousDrugEnforcementAdministrationNumber),
            b"P3" => Some(PreviousCustomerReferenceNumber),
            b"P4" => Some(ProjectCode),
            b"P5" => Some(PositionCode),
            b"P6" => Some(PipelineNumber),
            b"P7" => Some(ProductLineNumber),
            b"P8" => Some(PickupReferenceNumber),
            b"P9" => Some(PageNumber),
            b"PA" => Some(PriceAreaNumber),
            b"PAC" => Some(PatentCooperationTreatyApplicationNumber),
            b"PAN" => Some(NonprovisionalPatentApplicationNumber),
            b"PAP" => Some(ProvisionalPatentApplicationNumber),
            b"PB" => Some(CodePB),
            b"PC" => Some(ProductionCode),
            b"PCC" => Some(PoolContractCode),
            b"PCN" => Some(ProtocolNumber),
            b"PD" => Some(PromotionDealNumber),
            b"PDI" => Some(PartialDenialIndicator),
            b"PDL" => Some(PreviousDriversLicense),
            b"PDR" => Some(PartialDenialReason),
            b"PE" => Some(PlantNumber),
            b"PF" => Some(PrimeContractorContractNumber),
            b"PG" => Some(ProductGroup),
            b"PGC" => Some(PackingGroupCode),
            b"PGD" => Some(DownstreamPackage),
            b"PGN" => Some(PlugNumber),
            b"PGS" => Some(ProposedGroupWorkCandidateSequenceNumber),
            b"PH" => Some(PriorityRating),
            b"PHC" => Some(ProcessHandlingCode),
            b"PHY" => Some(PhysicianStateLicenseNumber),
            b"PI" => Some(PriceListChangeOrIssueNumber),
            b"PID" => Some(ProgramIdentificationNumber),
            b"PIN" => Some(PlatformIdentificationNumber),
            b"PJ" => Some(PackerNumber),
            b"PJC" => Some(PreviousReportNumber),
            b"PK" => Some(PackingListNumber),
            b"PKG" => Some(Package),
            b"PKU" => Some(UpstreamPackage),
            b"PL" => Some(PriceListNumber),
            b"PLA" => Some(ProductLicensingAgreementNumber),
            b"PLN" => Some(ProposedContractNumber),
            b"PM" => Some(PartNumber),
            b"PMN" => Some(PremarketApplicationNumber),
            b"PN" => Some(PermitNumber),
            b"PNN" => Some(PatentNumber),
            b"PO" => Some(PurchaseOrderNumber),
            b"POL" => Some(PolicyNumber),
            b"POS" => Some(PositionTitleNumber),
            b"PP" => Some(PurchaseOrderRevisionNumber),
            b"PPJ" => Some(CertificateOfPurchaseNumber),
            b"PPK" => Some(TaxBillIdentificationNumber),
            b"PPL" => Some(CurrentYearTaxBillNumber),
            b"PPM" => Some(PastYearTaxBillNumber),
            b"PPN" => Some(PaymentPlanNumber),
            b"PQ" => Some(PayeeIdentification),
            b"PR" => Some(PriceQuoteNumber),
            b"PRS" => Some(PreviouslyReportedSocialSecurityNumber),
            b"PRT" => Some(ProductType),
            b"PS" => Some(PurchaseOrderNumberSuffix),
            b"PSI" => Some(PreviousShipmentIdentificationNumberContinuousMove),
            b"PSL" => Some(NextShipmentIdentificationNumberContinuousMove),
            b"PSM" => Some(CreditCard),
            b"PSN" => Some(ProposedSequenceNumber),
            b"PT" => Some(PurchaseOptionAgreement),
            b"PTC" => Some(PatentType),
            b"PU" => Some(PreviousBillOfLadingNumber),
            b"PUA" => Some(PickupAppointmentNumber),
            b"PV" => Some(ProductChangeInformationNumber),
            b"PVC" => Some(PaymentValidationCode),
            b"PW" => Some(PriorPurchaseOrderNumber),
            b"PWC" => Some(PreliminaryWorkCandidateNumber),
            b"PWS" => Some(ProposedWorkCandidateSequenceNumber),
            b"PX" => Some(PreviousInvoiceNumber),
            b"PXC" => Some(HealthCareProviderTaxonomyCode),
            b"PY" => Some(CodePY),
            b"PYA" => Some(PayrollActivityCode),
            b"PYR" => Some(PayRange),
            b"PZ" => Some(ProductChangeNoticeNumber),
            b"Q1" => Some(QuoteNumber),
            b"Q2" => Some(StartingPackageNumber),
            b"Q3" => Some(EndingPackageNumber),
            b"Q4" => Some(PriorIdentifierNumber),
            b"Q5" => Some(PropertyControlNumber),
            b"Q6" => Some(RecallNumber),
            b"Q7" => Some(ReceiverClaimNumber),
            b"Q8" => Some(RegistrationNumber),
            b"Q9" => Some(RepairOrderNumber),
            b"QA" => Some(Press),
            b"QB" => Some(PressForm),
            b"QC" => Some(ProductSpecificationDocumentNumber),
            b"QD" => Some(ReplacementDrugEnforcementAdministrationNumber),
            b"QE" => Some(ReplacementCustomerReferenceNumber),
            b"QF" => Some(QualityDispositionArea),
            b"QG" => Some(ReplacementAssemblyModelNumber),
            b"QH" => Some(ReplacementAssemblySerialNumber),
            b"QI" => Some(QualityInspectionArea),
            b"QJ" => Some(ReturnMaterialAuthorizationNumber),
            b"QK" => Some(SalesProgramNumber),
            b"QL" => Some(ServiceAuthorizationNumber),
            b"QM" => Some(QualityReviewMaterialCrib),
            b"QN" => Some(StopSequenceNumber),
            b"QO" => Some(ServiceEstimateNumber),
            b"QP" => Some(SubstitutePartNumber),
            b"QQ" => Some(UnitNumber),
            b"QR" => Some(QualityReportNumber),
            b"QS" => Some(WarrantyCoverageCode),
            b"QT" => Some(WarrantyRegistrationNumber),
            b"QU" => Some(ChangeVerificationProcedureCode),
            b"QV" => Some(MajorSystemAffectedCode),
            b"QW" => Some(NewPartNumber),
            b"QX" => Some(OldPartNumber),
            b"QY" => Some(ServicePerformedCode),
            b"QZ" => Some(ReferenceDrawingNumber),
            b"R0" => Some(CodeR0),
            b"R1" => Some(CurrentRevisionNumber),
            b"R2" => Some(CanceledRevisionNumber),
            b"R3" => Some(CorrectionNumber),
            b"R4" => Some(TariffSectionNumber),
            b"R5" => Some(TariffPageNumber),
            b"R6" => Some(TariffRuleNumber),
            b"R7" => Some(AccountsReceivableOpenItem),
            b"R8" => Some(RentalAgreementNumber),
            b"R9" => Some(RejectionNumber),
            b"RA" => Some(RepetitiveCargoShipmentNumber),
            b"RAA" => Some(RestrictedAvailabilityAuthorization),
            b"RAN" => Some(RestrictedAvailabilityNumber),
            b"RB" => Some(RateCodeNumber),
            b"RC" => Some(RailRoutingCode),
            b"RD" => Some(ReelNumber),
            b"RE" => Some(ReleaseNumber),
            b"REC" => Some(RelatedCase),
            b"RF" => Some(ExportReferenceNumber),
            b"RG" => Some(RouteOrderNumberDomestic),
            b"RGI" => Some(RegulatoryGuideline),
            b"RH" => Some(RouteOrderNumberExport),
            b"RI" => Some(ReleaseInvoiceNumberForPriorBillAndHold),
            b"RIG" => Some(RigNumber),
            b"RJ" => Some(RouteOrderNumberEmergency),
            b"RK" => Some(RackTypeNumber),
            b"RL" => Some(ReserveAssemblyLineFeedLocation),
            b"RLI" => Some(RoleIdentificationNumber),
            b"RM" => Some(CodeRM),
            b"RN" => Some(RunNumber),
            b"RO" => Some(RepetitiveBookingNumber),
            b"RP" => Some(RepetitivePatternCode),
            b"RPP" => Some(RelativePriority),
            b"RPS" => Some(RegulationPrimaryNumber),
            b"RPT" => Some(ReportNumber),
            b"RQ" => Some(PurchaseRequisitionNumber),
            b"RR" => Some(CodeRR),
            b"RRC" => Some(RoutingRequestControlNumber),
            b"RRS" => Some(ReconciliationReportSectionIdentificationCode),
            b"RS" => Some(ReturnableContainerSerialNumber),
            b"RSN" => Some(ReservationNumber),
            b"RSS" => Some(RegulationSecondaryNumber),
            b"RT" => Some(CodeRT),
            b"RU" => Some(RouteNumber),
            b"RV" => Some(ReceivingNumber),
            b"RW" => Some(CodeRW),
            b"RWK" => Some(ReportingWeek),
            b"RX" => Some(ResubmitNumber),
            b"RY" => Some(RebateNumber),
            b"RZ" => Some(ReturnedGoodsAuthorizationNumber),
            b"S0" => Some(SpecialApproval),
            b"S1" => Some(EngineeringSpecificationNumber),
            b"S2" => Some(DataSource),
            b"S3" => Some(SpecificationNumber),
            b"S4" => Some(ShippersBondNumber),
            b"S5" => Some(RoutingInstructionNumber),
            b"S6" => Some(StockNumber),
            b"S7" => Some(StackTrainIdentification),
            b"S8" => Some(SealOffNumber),
            b"S9" => Some(SealOnNumber),
            b"SA" => Some(Salesperson),
            b"SAL" => Some(SalaryStep),
            b"SB" => Some(SalesRegionNumber),
            b"SBN" => Some(SuretyBondNumber),
            b"SC" => Some(ShipperCarOrderNumber),
            b"SCA" => Some(CodeSCA),
            b"SCN" => Some(ScaleNumber),
            b"SD" => Some(SubdayNumber),
            b"SDT" => Some(SchoolDistrictTypeCode),
            b"SE" => Some(SerialNumber),
            b"SEK" => Some(SearchKey),
            b"SES" => Some(Session),
            b"SF" => Some(ShipFrom),
            b"SG" => Some(Savings),
            b"SH" => Some(SenderDefinedClause),
            b"SHL" => Some(ShelfLifeIndicator),
            b"SI" => Some(CodeSI),
            b"SII" => Some(SalvageInstruction),
            b"SJ" => Some(SetNumber),
            b"SK" => Some(ServiceChangeNumber),
            b"SL" => Some(SalesTerritoryCode),
            b"SM" => Some(SalesOfficeNumber),
            b"SMC" => Some(SettlementMethodCode),
            b"SMT" => Some(StateOfMassachusettsTownCode),
            b"SN" => Some(SealNumber),
            b"SNH" => Some(CodeSNH),
            b"SNP" => Some(CodeSNP),
            b"SNV" => Some(StateNonResidentViolatorCompact),
            b"SO" => Some(CodeSO),
            b"SP" => Some(ScanLine),
            b"SPL" => Some(CodeSPL),
            b"SPN" => Some(TheaterScreenNumber),
            b"SQ" => Some(ContainerSequenceNumber),
            b"SR" => Some(SalesResponsibility),
            b"SS" => Some(SplitShipmentNumber),
            b"SST" => Some(SchoolSystemTypeCode),
            b"ST" => Some(StoreNumber),
            b"STB" => Some(CodeSTB),
            b"STR" => Some(CodeSTR),
            b"STS" => Some(ServiceabilityStandardTestingReference),
            b"SU" => Some(SpecialProcessingCode),
            b"SUB" => Some(TitleReference),
            b"SUC" => Some(SupervisoryUnionCode),
            b"SUO" => Some(SpacingUnitOrderNumber),
            b"SV" => Some(ServiceChargeNumber),
            b"SW" => Some(SellersSaleNumber),
            b"SX" => Some(ServiceInterruptTrackingNumber),
            b"SY" => Some(SocialSecurityNumber),
            b"SZ" => Some(SpecificationRevision),
            b"T0" => Some(DealerTypeIdentification),
            b"T1" => Some(TaxExchangeCode),
            b"T2" => Some(TaxFormCode),
            b"T3" => Some(TaxScheduleCode),
            b"T4" => Some(SignalCode),
            b"T5" => Some(TrailerUseAgreements),
            b"T6" => Some(TaxFiling),
            b"T7" => Some(AffectedSubsystemCode),
            b"T8" => Some(DescriptionOfChangeCode),
            b"T9" => Some(DocumentationAffectedNumber),
            b"TA" => Some(TelecommunicationCircuitSupplementalId),
            b"TB" => Some(TruckersBillOfLading),
            b"TC" => Some(VendorTerms),
            b"TD" => Some(ReasonForChange),
            b"TDT" => Some(TechnicalDocumentationType),
            b"TE" => Some(CodeTE),
            b"TF" => Some(TransferNumber),
            b"TFC" => Some(TimeFailure),
            b"TG" => Some(CodeTG),
            b"TH" => Some(CodeTH),
            b"TI" => Some(TirNumber),
            b"TIP" => Some(TechnicalInformationPackage),
            b"TJ" => Some(FederalTaxpayersIdentificationNumber),
            b"TK" => Some(TankNumber),
            b"TL" => Some(TaxLicenseExemption),
            b"TM" => Some(CodeTM),
            b"TN" => Some(TransactionReferenceNumber),
            b"TO" => Some(TerminalOperatorNumber),
            b"TOC" => Some(TypeOfComment),
            b"TP" => Some(TestSpecificationNumber),
            b"TPN" => Some(TransponderNumber),
            b"TQ" => Some(TracerActionRequestNumber),
            b"TR" => Some(GovernmentTransportationRequest),
            b"TS" => Some(TariffNumber),
            b"TSN" => Some(TemplateSequenceNumber),
            b"TT" => Some(TerminalCode),
            b"TU" => Some(TrialLocationCode),
            b"TV" => Some(LineOfBusiness),
            b"TW" => Some(TaxWorksheet),
            b"TX" => Some(TaxExemptNumber),
            b"TY" => Some(PolicyType),
            b"TZ" => Some(TotalCycleNumber),
            b"U0" => Some(ConsolidatorsReceiptNumber),
            b"U1" => Some(RegionalAccountNumber),
            b"U2" => Some(Term),
            b"U3" => Some(CodeU3),
            b"U4" => Some(UnpaidInstallmentReferenceNumber),
            b"U5" => Some(SuccessorAccount),
            b"U6" => Some(PredecessorAccount),
            b"U8" => Some(CodeU8),
            b"U9" => Some(CodeU9),
            b"UA" => Some(MortgageNumber),
            b"UB" => Some(UnacceptableSourcePurchaserId),
            b"UC" => Some(MortgageInsuranceIndicatorNumber),
            b"UCB" => Some(CodeUCB),
            b"UCM" => Some(CodeUCM),
            b"UD" => Some(UnacceptableSourceDunsNumber),
            b"UE" => Some(SecondaryCoverageCertificateNumber),
            b"UF" => Some(MortgageInsuranceCompanyNumber),
            b"UG" => Some(USGovernmentTransportationControlNumber),
            b"UH" => Some(RemovalNumber),
            b"UI" => Some(PreviousCourseNumber),
            b"UIC" => Some(CodeUIC),
            b"UJ" => Some(CurrentOrLatestCourseNumber),
            b"UK" => Some(EquivalentCourseNumberAtRequestingInstitution),
            b"UL" => Some(CrossListedCourseNumber),
            b"UM" => Some(QuarterQuarterSectionNumber),
            b"UN" => Some(UnitedNationsHazardousClassificationNumber),
            b"UO" => Some(QuarterQuarterSpotNumber),
            b"UP" => Some(UpstreamShipperContractNumber),
            b"UQ" => Some(SectionNumber),
            b"UR" => Some(UnitReliefNumber),
            b"URL" => Some(UniformResourceLocator),
            b"URP" => Some(UnitReportPeriod),
            b"URQ" => Some(UnitReportPeriodId),
            b"US" => Some(UnacceptableSourceSupplierId),
            b"UT" => Some(UnitTrain),
            b"UU" => Some(TownshipNumber),
            b"UV" => Some(RangeNumber),
            b"UW" => Some(StateSenateDistrict),
            b"UX" => Some(StateAssemblyDistrict),
            b"UY" => Some(CodeUY),
            b"UZ" => Some(StateLegislativeDistrict),
            b"V0" => Some(Version),
            b"V1" => Some(VolumePurchaseAgreementNumber),
            b"V2" => Some(VisaType),
            b"V3" => Some(VoyageNumber),
            b"V4" => Some(StateDepartmentI20FormNumber),
            b"V5" => Some(StateDepartmentIap66FormNumber),
            b"V6" => Some(CodeV6),
            b"V7" => Some(JudicialDistrict),
            b"V8" => Some(InstitutionNumber),
            b"V9" => Some(Subservicer),
            b"VA" => Some(VesselAgentNumber),
            b"VAO" => Some(VeteransAdministrationOriginatorIdentification),
            b"VB" => Some(CodeVB),
            b"VC" => Some(VendorContractNumber),
            b"VD" => Some(VolumeNumber),
            b"VE" => Some(VendorAbbreviationCode),
            b"VF" => Some(VendorChangeIdentificationCode),
            b"VG" => Some(VendorChangeProcedureCode),
            b"VGS" => Some(VehicleGaragedStateCode),
            b"VH" => Some(CountyLegislativeDistrict),
            b"VI" => Some(PoolNumber),
            b"VJ" => Some(InvestorNoteHolderIdentification),
            b"VK" => Some(InstitutionNoteHolderIdentification),
            b"VL" => Some(ThirdPartyNoteHolderIdentification),
            b"VM" => Some(Ward),
            b"VN" => Some(VendorOrderNumber),
            b"VO" => Some(InstitutionLoanNumber),
            b"VP" => Some(VendorProductNumber),
            b"VQ" => Some(RelatedContractLineItemNumber),
            b"VR" => Some(VendorIdNumber),
            b"VS" => Some(VendorOrderNumberSuffix),
            b"VT" => Some(MotorVehicleIdNumber),
            b"VU" => Some(PreparersVerificationNumber),
            b"VV" => Some(Voucher),
            b"VW" => Some(Standard),
            b"VX" => Some(CodeVX),
            b"VY" => Some(LinkSequenceNumber),
            b"VZ" => Some(SponsorsReferenceNumber),
            b"W1" => Some(DisposalTurnInDocumentNumber),
            b"W2" => Some(WeaponSystemNumber),
            b"W3" => Some(ManufacturingDirectiveNumber),
            b"W4" => Some(ProcurementRequestNumber),
            b"W5" => Some(InspectorIdentificationNumber),
            b"W6" => Some(FederalSupplyScheduleNumber),
            b"W7" => Some(CodeW7),
            b"W8" => Some(Suffix),
            b"W9" => Some(SpecialPackagingInstructionNumber),
            b"WA" => Some(LaborOrAffiliationIdentification),
            b"WB" => Some(CodeWB),
            b"WC" => Some(ContractOptionNumber),
            b"WCS" => Some(WorkCandidateSequenceNumber),
            b"WD" => Some(ReviewPeriodNumber),
            b"WDR" => Some(WithdrawalRecord),
            b"WE" => Some(WellClassificationCode),
            b"WF" => Some(LocallyAssignedControlNumber),
            b"WG" => Some(VendorsPreviousJobNumber),
            b"WH" => Some(CodeWH),
            b"WI" => Some(Waiver),
            b"WJ" => Some(PreAwardSurvey),
            b"WK" => Some(TypeOfScienceCode),
            b"WL" => Some(FederalSupplyClassificationCode),
            b"WM" => Some(WeightAgreementNumber),
            b"WN" => Some(WellNumber),
            b"WO" => Some(WorkOrderNumber),
            b"WP" => Some(WarehousePickTicketNumber),
            b"WQ" => Some(InterimFundingOrganizationLoanNumber),
            b"WR" => Some(WarehouseReceiptNumber),
            b"WS" => Some(WarehouseStorageLocationNumber),
            b"WT" => Some(BrokersReferenceNumber),
            b"WU" => Some(Vessel),
            b"WV" => Some(DealerIdentification),
            b"WW" => Some(DepositoryTrustCompanyIdentification),
            b"WX" => Some(DistributorsAccountIdentification),
            b"WY" => Some(WaybillNumber),
            b"WZ" => Some(DistributorsRepresentativeIdentification),
            b"X0" => Some(DebtorsAccount),
            b"X1" => Some(ProviderClaimNumber),
            b"X2" => Some(SpecificationClassNumber),
            b"X3" => Some(DefectCodeNumber),
            b"X4" => Some(ClinicalLaboratoryImprovementAmendmentNumber),
            b"X5" => Some(StateIndustrialAccidentProviderNumber),
            b"X6" => Some(OriginalVoucherNumber),
            b"X7" => Some(BatchSequenceNumber),
            b"X8" => Some(SecondarySuffixCodeIndicator),
            b"X9" => Some(InternalControlNumber),
            b"XA" => Some(SubstituteNationalStockNumber),
            b"XB" => Some(SubstituteManufacturersPartNumber),
            b"XC" => Some(CargoControlNumber),
            b"XD" => Some(SubsistenceIdentificationNumber),
            b"XE" => Some(TransportationPriorityNumber),
            b"XF" => Some(GovernmentBillOfLadingOfficeCode),
            b"XG" => Some(AirlineTicketNumber),
            b"XH" => Some(ContractAuditorIdNumber),
            b"XI" => Some(FederalHomeLoanMortgageCorporationLoanNumber),
            b"XJ" => {
                Some(
                    FederalHomeLoanMortgageCorporationDefaultForeclosureSpecialistNumber,
                )
            }
            b"XK" => Some(MortgageeLoanNumber),
            b"XL" => Some(InsuredsLoanNumber),
            b"XM" => Some(IssuerNumber),
            b"XN" => Some(TitleXixIdentifierNumber),
            b"XO" => Some(SampleNumber),
            b"XP" => Some(PreviousCargoControlNumber),
            b"XQ" => Some(PierNumber),
            b"XR" => Some(RailroadCommissionRecordNumber),
            b"XS" => Some(GasAnalysisSourceMeterNumber),
            b"XT" => Some(ToxicologyId),
            b"XU" => Some(UniversalTransverseMercatorNorth),
            b"XV" => Some(UniversalTransverseMercatorEast),
            b"XW" => Some(UniversalTransverseMercatorZone),
            b"XX" => Some(RatingPeriod),
            b"XX1" => Some(SpecialProgramCode),
            b"XX2" => Some(ServiceAreaCode),
            b"XX3" => Some(FunctionCode),
            b"XX4" => Some(ObjectCode),
            b"XX5" => Some(OrganizationCode),
            b"XX6" => Some(SubjectAreaCode),
            b"XX7" => Some(ScheduleTypeCode),
            b"XX8" => Some(AlternatingScheduleIdentifierCode),
            b"XY" => Some(OtherUnlistedTypeOfReferenceNumber),
            b"XZ" => Some(PharmacyPrescriptionNumber),
            b"Y0" => Some(Debtor),
            b"Y1" => Some(ClaimAdministratorClaimNumber),
            b"Y2" => Some(ThirdPartyAdministratorClaimNumber),
            b"Y3" => Some(ContractHolderClaimNumber),
            b"Y4" => Some(AgencyClaimNumber),
            b"Y5" => Some(DeliveryTrailerManifest),
            b"Y6" => Some(SortAndSegregate),
            b"Y8" => Some(UserId),
            b"Y9" => Some(CurrentCertificateNumber),
            b"YA" => Some(PriorCertificateNumber),
            b"YB" => Some(RevisionNumber),
            b"YC" => Some(Tract),
            b"YD" => Some(BuyerIdentification),
            b"YE" => Some(RailroadCommissionOilNumber),
            b"YF" => Some(LesseeIdentification),
            b"YH" => Some(OperatorAssignedUnitNumber),
            b"YI" => Some(RefinerIdentification),
            b"YJ" => Some(RevenueSource),
            b"YK" => Some(RentPayorIdentification),
            b"YL" => Some(AllowanceRecipientIdentification),
            b"YM" => Some(ResourceScreeningReference),
            b"YN" => Some(ReceiverIdQualifier),
            b"YO" => Some(Formation),
            b"YP" => Some(SellingArrangement),
            b"YQ" => Some(MinimumRoyaltyPayorIdentification),
            b"YR" => Some(OperatorLeaseNumber),
            b"YS" => Some(YardPosition),
            b"YT" => Some(ReporterIdentification),
            b"YV" => Some(ParticipatingArea),
            b"YW" => Some(EngineeringChangeProposal),
            b"YX" => Some(GeographicScore),
            b"YY" => Some(GeographicKey),
            b"YZ" => Some(GeographicIndex),
            b"Z1" => Some(SafetyOfShipCertificate),
            b"Z2" => Some(SafetyOfRadioCertificate),
            b"Z3" => Some(SafetyEquipmentCertificate),
            b"Z4" => Some(CivilLiabilitiesOfOilCertificate),
            b"Z5" => Some(LoadLineCertificate),
            b"Z6" => Some(DeratCertificate),
            b"Z7" => Some(MaritimeDeclarationOfHealth),
            b"Z8" => Some(FederalHousingAdministrationCaseNumber),
            b"Z9" => Some(VeteransAffairsCaseNumber),
            b"ZA" => Some(Supplier),
            b"ZB" => Some(UltimateConsignee),
            b"ZC" => Some(ConnectingCarrier),
            b"ZD" => Some(FamilyMemberIdentification),
            b"ZE" => Some(CoalAuthorityNumber),
            b"ZG" => Some(SalesRepresentativeOrderNumber),
            b"ZH" => Some(CarrierAssignedReferenceNumber),
            b"ZI" => Some(ReferenceVersionNumber),
            b"ZJ" => Some(CodeZJ),
            b"ZK" => Some(DuplicateWaybillInRoute),
            b"ZL" => Some(DuplicateWaybillNotInRoute),
            b"ZM" => Some(ManufacturerNumber),
            b"ZN" => Some(AgencyCaseNumber),
            b"ZO" => Some(MakegoodCommercialLineNumber),
            b"ZP" => Some(SpouseTie),
            b"ZQ" => Some(NonSpouseTie),
            b"ZR" => Some(CodeZR),
            b"ZS" => Some(SoftwareApplicationNumber),
            b"ZT" => Some(MillingInTransit),
            b"ZTS" => Some(CodeZTS),
            b"ZU" => Some(Field),
            b"ZV" => Some(Block),
            b"ZW" => Some(Area),
            b"ZX" => Some(CountyCode),
            b"ZY" => Some(ReferencedPatternIdentification),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ReferenceIdentificationQualifier::*;
        match self {
            ContractingDistrictNumber => "Contracting District Number",
            SupervisoryAppraiserCertificationNumber => {
                "Supervisory Appraiser Certification Number"
            }
            StateLicenseNumber => "State License Number",
            SubjectPropertyVerificationSource => "Subject Property Verification Source",
            SubjectPropertyReferenceNumber => "Subject Property Reference Number",
            SubscriberNumber => "Subscriber Number",
            ReviewerFileNumber => "Reviewer File Number",
            ComparablePropertyPendingSaleReferenceNumber => {
                "Comparable Property Pending Sale Reference Number"
            }
            ComparablePropertySaleReferenceNumber => {
                "Comparable Property Sale Reference Number"
            }
            SubjectPropertyNonSaleReferenceNumber => {
                "Subject Property Non-Sale Reference Number"
            }
            PolicyFormIdentifyingNumber => "Policy Form Identifying Number",
            ReferencedBy => "Referenced By",
            MortgageIdentificationNumber => "Mortgage Identification Number",
            AttachedTo => "Attached To",
            RealEstateOwnedProperty => "Real Estate Owned Property Identifier",
            Code01 => {
                "American Bankers Assoc. (ABA) Transit/Routing Number (Including Check Digit, 9 Digits)"
            }
            BlueCrossProviderNumber => "Blue Cross Provider Number",
            CatalogOfFederalDomesticAssistance => {
                "Catalog of Federal Domestic Assistance"
            }
            BlueShieldProviderNumber => "Blue Shield Provider Number",
            UnionAgreement => "Union Agreement",
            MedicareProviderNumber => "Medicare Provider Number",
            Code01C => {
                "Military Standard Requisitioning and Issue Procedures (MILSTRIP) Document Number"
            }
            MedicaidProviderNumber => "Medicaid Provider Number",
            Code01D => {
                "Federal Standard Requisitioning and Issue Procedures (FEDSTRIP) Document Number"
            }
            DentistLicenseNumber => "Dentist License Number",
            Code01E => "Federal Supply Schedule Special (FSS) Item Number",
            AnesthesiaLicenseNumber => "Anesthesia License Number",
            ProviderUpinNumber => "Provider UPIN Number",
            PaymentRelatedClause => "Payment Related Clause",
            ChampusIdentificationNumber => "CHAMPUS Identification Number",
            SpecialPriceAuthorizationNumber => "Special Price Authorization Number",
            Code1I => "Department of Defense Identification Code (DoDIC)",
            FacilityIdNumber => "Facility ID Number",
            PayorsClaimNumber => "Payor's Claim Number",
            GroupOrPolicyNumber => "Group or Policy Number",
            PreferredProviderOrganizationSiteNumber => {
                "Preferred Provider Organization Site Number"
            }
            Code1N => "Diagnosis Related Group (DRG) Number",
            ConsolidationShipmentNumber => "Consolidation Shipment Number",
            AccessorialStatusCode => "Accessorial Status Code",
            ErrorIdentificationCode => "Error Identification Code",
            StorageInformationCode => "Storage Information Code",
            Code1S => "Ambulatory Patient Group (APG) Number",
            Code1T => "Resource Utilization Group (RUG) Number",
            PayGrade => "Pay Grade",
            RelatedVendorOrderNumber => "Related Vendor Order Number",
            MemberIdentificationNumber => "Member Identification Number",
            CreditOrDebitAdjustmentNumber => "Credit or Debit Adjustment Number",
            RepairActionNumber => "Repair Action Number",
            FinancialDetailCode => "Financial Detail Code",
            Code02 => {
                "Society for Worldwide Interbank Financial Telecommunication (S.W.I.F.T.) Identification (8 or 11 Characters)"
            }
            ImportLicenseNumber => "Import License Number",
            TerminalReleaseOrderNumber => "Terminal Release Order Number",
            LongTermDisabilityPolicyNumber => "Long-term Disability Policy Number",
            Code2D => "Aeronautical Equipment Reference Number (AERNO)",
            ForeignMilitarySalesCaseNumber => "Foreign Military Sales Case Number",
            ConsolidatedInvoiceNumber => "Consolidated Invoice Number",
            Amendment => "Amendment",
            AssignedByTransactionSetSender => "Assigned by transaction set sender",
            TrackingNumber => "Tracking Number",
            FloorNumber => "Floor Number",
            Code2K => "Food and Drug Administration (FDA) Product Type",
            Code2L => "Association of American Railroads (AAR) Railway Accounting Rules",
            Code2M => "Federal Communications Commission (FCC) Identifier",
            Code2N => "Federal Communications Commission (FCC) Trade/Brand Identifier",
            Code2O => "Occupational Safety and Health Administration (OSHA) Claim Number",
            Subdivision => "Subdivision Identifier",
            Code2Q => "Food and Drug Administration (FDA) Accession Number",
            CouponRedemptionNumber => "Coupon Redemption Number",
            Catalog => "Catalog",
            SubSubhouseBillOfLading => "Sub-subhouse Bill of Lading",
            PayerIdentificationNumber => "Payer Identification Number",
            Code2V => {
                "Special Government Accounting Classification Reference Number (ACRN)"
            }
            ChangeOrderAuthority => "Change Order Authority",
            SupplementalAgreementAuthority => "Supplemental Agreement Authority",
            WageDetermination => "Wage Determination",
            Code2Z => "U.S. Customs Service (USCS) Anti-dumping Duty Case Number",
            Code03 => {
                "Clearing House Interbank Payment System (CHIPS) Participant Number (3 or 4 Digits)"
            }
            SectionOfTheNationalHousingActCode => {
                "Section of the National Housing Act Code"
            }
            SupplementalClaimNumber => "Supplemental Claim Number",
            PayeeLoanNumber => "Payee Loan Number",
            ServicerLoanNumber => "Servicer Loan Number",
            InvestorLoanNumber => "Investor Loan Number",
            ShowIdentification => "Show Identification",
            CatastropheNumber => "Catastrophe Number",
            CaseNumber => "Case Number",
            PrecinctNumber => "Precinct Number",
            OfficeNumber => "Office Number",
            PetroleumPoolCode => "Petroleum Pool Code",
            Branch => "Branch Identifier",
            Code3M => "Federal Communications Commission (FCC) Condition Code",
            GasCustodianIdentification => "Gas Custodian Identification",
            Code3O => "U.S. Customs Service (USCS) Pre-approval Ruling Number",
            ThirdPartyOriginatorNumber => "Third Party Originator Number",
            Code3Q => "Food and Drug Administration (FDA) Product Code",
            Code3R => "U.S. Customs Service (USCS) Binding Ruling Number",
            Code3S => "Provincial (Canadian) Sales Tax Exemption Number",
            Code3T => "U.S. Customs Service (USCS) Pre-classification Ruling Number",
            ProtractionNumber => "Protraction Number",
            FormationIdentifier => "Formation Identifier",
            Code3W => "U.S. Customs Service (USCS) Commercial Description",
            SubcontractNumber => "Subcontract Number",
            ReceiverAssignedDropZone => "Receiver Assigned Drop Zone",
            CustomsBrokerReferenceNumber => "Customs Broker Reference Number",
            CanadianFinancialInstitutionBranchAndInstitutionNumber => {
                "Canadian Financial Institution Branch and Institution Number"
            }
            Code4A => "Personal Identification Number (PIN)",
            ShipmentOriginCode => "Shipment Origin Code",
            ShipmentDestinationCode => "Shipment Destination Code",
            ShippingZone => "Shipping Zone",
            CarrierAssignedConsigneeNumber => "Carrier-assigned Consignee Number",
            CarrierAssignedShipperNumber => "Carrier-assigned Shipper Number",
            ProvincialTaxIdentification => "Provincial Tax Identification",
            CommercialInvoiceNumber => "Commercial Invoice Number",
            BalanceDueReferenceNumber => "Balance-due Reference Number",
            VehicleRelatedServicesReferenceNumber => {
                "Vehicle-related Services Reference Number"
            }
            AccessorialRailDiversionReferenceNumber => {
                "Accessorial Rail Diversion Reference Number"
            }
            LocationSpecificServicesReferenceNumber => {
                "Location-specific Services Reference Number"
            }
            SpecialMoveReferenceNumber => "Special Move Reference Number",
            SpecialPaymentReferenceNumber => "Special Payment Reference Number",
            Code4O => "Canadian Goods & Services or Quebec Sales Tax Reference Number",
            AffiliationNumber => "Affiliation Number",
            CallSign => "Call Sign",
            RuleSection => "Rule Section",
            PreferredCallSign => "Preferred Call Sign",
            Code4T => "North American Datum Standard (NADS)",
            MarketArea => "Market Area",
            EmissionDesignator => "Emission Designator",
            Study => "Study",
            Log => "Log",
            SubhouseBillOfLading => "Subhouse Bill of Lading",
            Code4Z => "U.S. Customs Service (USCS) Countervailing Duty Case Number",
            Code05 => {
                "Clearing House Interbank Payment System (CHIPS) User Identification (6 digits)"
            }
            OffenseTracking => "Offense Tracking",
            SupplementalAccountNumber => "Supplemental Account Number",
            CongressionalDistrict => "Congressional District",
            LineOfCreditCategory => "Line of Credit Category",
            Consumer => "Consumer Identifier",
            Warrant => "Warrant",
            Complaint => "Complaint",
            Incident => "Incident",
            OffenderTracking => "Offender Tracking",
            DriversLicense => "Driver's License",
            CommercialDriversLicense => "Commercial Driver's License",
            JurisdictionalCommunityNumber => "Jurisdictional Community Number",
            PreviousSequence => "Previous Sequence",
            CitationOfStatute => "Citation of Statute",
            CitationOfOpinion => "Citation of Opinion",
            NationalCriminalInformationCenterOriginatingAgencyIdentification => {
                "National Criminal Information Center Originating Agency Identification"
            }
            StateCriminalHistoryRepositoryIndividualIdentification => {
                "State Criminal History Repository Individual Identification"
            }
            FederalBureauOfInvestigationIndividualIdentification => {
                "Federal Bureau of Investigation Individual Identification"
            }
            ProcessingArea => "Processing Area",
            PaymentLocation => "Payment Location",
            FloodData => "Flood Data Identifier",
            CouponDistributionMethod => "Coupon Distribution Method",
            OriginalUniformCommercialCodeFilingNumber => {
                "Original Uniform Commercial Code Filing Number"
            }
            AmendedUniformCommercialCodeFilingNumber => {
                "Amended Uniform Commercial Code Filing Number"
            }
            ContinuationUniformCommercialCodeFilingNumber => {
                "Continuation Uniform Commercial Code Filing Number"
            }
            UniformCommercialCodeFilingCollateralNumber => {
                "Uniform Commercial Code Filing Collateral Number"
            }
            SystemNumber => "System Number",
            ConsigneeReferenceNumber => "Consignee Reference Number",
            Code6B => "U.S. Customs Service (USCS) Entry Number",
            Code6C => "U.S. Customs Service (USCS) Entry Type Code",
            Code6D => "U.S. Customs Service (USCS) Statement Number",
            MapReference => "Map Reference",
            AppraiserLicense => "Appraiser License",
            MapNumber => "Map Number",
            ComparablePropertyVerificationSource => {
                "Comparable Property Verification Source"
            }
            ComparableProperty => "Comparable Property",
            CensusTract => "Census Tract",
            Zone => "Zone",
            AgentContractNumber => "Agent Contract Number",
            ApplicationNumber => "Application Number",
            ClaimantNumber => "Claimant Number",
            CrossReferenceNumber => "Cross Reference Number",
            GroupNumber => "Group Number",
            InsuranceLicenseNumber => "Insurance License Number",
            ProviderControlNumber => "Provider Control Number",
            ProviderOrderTicketNumber => "Provider Order Ticket Number",
            PilotLicenseNumber => "Pilot License Number",
            QuestionNumber => "Question Number",
            ReissueCessionNumber => "Reissue Cession Number",
            Specimen => "Specimen Identifier",
            EquipmentInitial => "Equipment Initial",
            Code6Z => "Secretaria de Comercia y Famenta Industrial (SECOFI) Number",
            AddOnSystemNumber => "Add-On System Number",
            PurchaseOrderNumberIncludedInOnOrderPosition => {
                "Purchase Order Number Included in On-Order Position"
            }
            PurchaseOrderNumberOfShipmentReceivedSinceLastReportingDate => {
                "Purchase Order Number of Shipment Received since Last Reporting Date"
            }
            PurchaseOrderNumberOfOrderReceivedSinceLastReportingDate => {
                "Purchase Order Number of Order Received since Last Reporting Date"
            }
            TesterIdentification => "Tester Identification",
            CollectorIdentification => "Collector Identification",
            RepeatLocation => "Repeat Location",
            DataQualityRejectReason => "Data Quality Reject Reason",
            Code7H => "Environmental Protection Agency (EPA) Test Type Purpose Code",
            SubscriberAuthorizationNumber => "Subscriber Authorization Number",
            TollBillingTelephoneReferenceNumber => {
                "Toll Billing Telephone Reference Number"
            }
            ListOfMaterials => "List of Materials",
            QualifiedMaterialsList => "Qualified Materials List",
            Frame => "Frame",
            Piggyback => "Piggyback",
            Tripleback => "Tripleback",
            Sheet => "Sheet",
            EngineeringChangeOrder => "Engineering Change Order",
            RepresentativeIdentificationNumber => "Representative Identification Number",
            DrawingType => "Drawing Type",
            MasterContract => "Master Contract",
            RelatedTransactionReferenceNumber => "Related Transaction Reference Number",
            InterchangeTrainIdentification => "Interchange Train Identification",
            Code7X => "Home Mortgage Disclosure Act (HMDA) State Code",
            Code7Y => "Home Mortgage Disclosure Act (HMDA) County Code",
            Code7Z => {
                "Home Mortgage Disclosure Act (HMDA) Metropolitan Statistical Area (MSA)"
            }
            CarrierAssignedPackageIdentificationNumber => {
                "Carrier Assigned Package Identification Number"
            }
            Code8A => "Health Maintenance Organization (HMO) Authorization Number",
            Code8B => "Preferred Provider Organization (PPO) Authorization Number",
            Code8C => "Third-party Organization (TPO) Authorization Number",
            ChemicalAbstractServiceRegistryNumber => {
                "Chemical Abstract Service Registry Number"
            }
            GuarantorLoanNumber => "Guarantor Loan Number",
            SchoolLoanNumber => "School Loan Number",
            Code8G => "Automated Clearinghouse (ACH) Trace Number",
            CheckListNumber => "Check List Number",
            FedwireConfirmationNumber => "FEDWIRE Confirmation Number",
            Code8J => {
                "Society for Worldwide Interbank Financial Telecommunications (SWIFT) Confirmation Number"
            }
            DominionOfCanadaCode => "Dominion of Canada Code",
            Code8L => "International Standard Industry Classification Code (ISIC)",
            OriginatingCompany => "Originating Company Identifier",
            ReceivingCompany => "Receiving Company Identifier",
            Code8O => "Automated Clearing House (ACH) Entry Description",
            OriginatingDepositoryFinancialInstitution => {
                "Originating Depository Financial Institution Identifier"
            }
            ReceivingDepositoryFinancialInstitution => {
                "Receiving Depository Financial Institution Identifier"
            }
            SecurityType => "Security Type",
            BrokerIdentification => "Broker Identification",
            BankAssignedSecurity => "Bank Assigned Security Identifier",
            CreditReference => "Credit Reference",
            BankToBank => "Bank to Bank Information",
            TransactionCategoryOrType => "Transaction Category or Type",
            SafekeepingAccountNumber => "Safekeeping Account Number",
            AlternateClauseNumber => "Alternate Clause Number",
            CustomsBarCodeNumber => "Customs Bar Code Number",
            RepricedClaimReferenceNumber => "Repriced Claim Reference Number",
            RepricedLineItemReferenceNumber => "Repriced Line Item Reference Number",
            AdjustedRepricedClaimReferenceNumber => {
                "Adjusted Repriced Claim Reference Number"
            }
            AdjustedRepricedLineItemReferenceNumber => {
                "Adjusted Repriced Line Item Reference Number"
            }
            ReplacementClaimNumber => "Replacement Claim Number",
            ReferralNumber => "Referral Number",
            DepartmentOfDefenseForm250RequirementCode => {
                "Department of Defense Form 250 Requirement Code"
            }
            PackagingGroupNumber => "Packaging Group Number",
            Code9I => "Automated Clearing House (ACH) Standard Entry Class",
            PensionContract => "Pension Contract",
            Servicer => "Servicer",
            ServiceBureau => "Service Bureau",
            Code9M => "Clearing House Interbank Payments System (CHIPS) Sequence Number",
            Investor => "Investor",
            LoanType => "Loan Type",
            PoolSuffix => "Pool Suffix",
            JobOrderNumber => "Job Order Number",
            DeliveryRegion => "Delivery Region",
            Tenor => "Tenor",
            LoanFeatureCode => "Loan Feature Code",
            PaymentCategory => "Payment Category",
            PayerCategory => "Payer Category",
            AccountCategory => "Account Category",
            BankAssignedBankersReferenceNumber => {
                "Bank Assigned Bankers Reference Number"
            }
            ChamberOfCommerceNumber => "Chamber of Commerce Number",
            AccountManagersCode => "Account Managers Code",
            AccountNumber => "Account Number",
            BillingAccount => "Billing Account",
            HorizontalCoordinate => "Horizontal Coordinate",
            MasterAccountNumber => "Master Account Number",
            VerticalCoordinate => "Vertical Coordinate",
            Code16 => "Military Interdepartmental Purchase Request (MIPR) Number",
            ClientReportingCategory => "Client Reporting Category",
            PlanNumber => "Plan Number",
            Division => "Division Identifier",
            RepairPartNumber => "Repair Part Number",
            AmericanGasAssociationEquationNumber => {
                "American Gas Association Equation Number"
            }
            SpecialChargeOrAllowanceCode => "Special Charge or Allowance Code",
            ClientNumber => "Client Number",
            ShortTermDisabilityPolicyNumber => "Short-term Disability Policy Number",
            ReasonNotLowestCostCode => "Reason Not Lowest Cost Code",
            UnionNumber => "Union Number",
            InsurorPoolIdentificationNumber => "Insuror Pool Identification Number",
            EmployeeIdentificationNumber => "Employee Identification Number",
            ForeclosureAccountNumber => "Foreclosure Account Number",
            UnitedStatesGovernmentVisaNumber => "United States Government Visa Number",
            DocketNumber => "Docket Number",
            CreditRepositoryCode => "Credit Repository Code",
            LenderCaseNumber => "Lender Case Number",
            LoanRequestNumber => "Loan Request Number",
            MultifamilyProjectNumber => "Multifamily Project Number",
            UnderwriterIdentificationNumber => "Underwriter Identification Number",
            CondominiumIdentificationNumber => "Condominium Identification Number",
            MasterPolicyNumber => "Master Policy Number",
            ProposalNumber => "Proposal Number",
            LeaseScheduleNumberReplacement => "Lease Schedule Number - Replacement",
            LeaseScheduleNumberPrior => "Lease Schedule Number - Prior",
            PhoneCalls => "Phone Calls",
            SupportingDocumentNumber => "Supporting Document Number",
            EndUseNumber => "End Use Number",
            OldAccountNumber => "Old Account Number",
            OldMeterNumber => "Old Meter Number",
            PlateNumber => "Plate Number",
            AgencysStudentNumber => "Agency's Student Number",
            FamilyUnitNumber => "Family Unit Number",
            StateStudentIdentificationNumber => "State Student Identification Number",
            PictureNumber => "Picture Number",
            Code52 => "SWIFT (MT 100)",
            Code53 => "SWIFT (MT 202)",
            Code54 => "FEDWIRE (Federal Wire Transfer)",
            SequenceNumber => "Sequence Number",
            CorrectedSocialSecurityNumber => "Corrected Social Security Number",
            PriorIncorrectSocialSecurityNumber => {
                "Prior Incorrect Social Security Number"
            }
            CorrectedBatchNumber => "Corrected Batch Number",
            PriorIncorrectBatchNumber => "Prior Incorrect Batch Number",
            AccountSuffixCode => "Account Suffix Code",
            TaxingAuthorityIdentificationNumber => {
                "Taxing Authority Identification Number"
            }
            PriorLoanNumber => "Prior Loan Number",
            JurisdictionalCommunityName => "Jurisdictional Community Name Identifier",
            TotalOrderCycleNumber => "Total Order Cycle Number",
            PreviousPolicyNumber => "Previous Policy Number",
            PreviousClaimHistory => "Previous Claim History Identifier",
            DentalInsuranceAccountNumber => "Dental Insurance Account Number",
            DentalInsurancePolicyNumber => "Dental Insurance Policy Number",
            CalendarNumber => "Calendar Number",
            Code71 => "(Working) Shift Number",
            ScheduleReferenceNumber => "Schedule Reference Number",
            Code73 => "Statement of Work (SOW)",
            Code74 => "Work Breakdown Structure (WBS)",
            OrganizationBreakdownStructure => "Organization Breakdown Structure",
            Milestone => "Milestone",
            WorkPackage => "Work Package",
            PlanningPackage => "Planning Package",
            CostAccount => "Cost Account",
            ChargeNumber => "Charge Number",
            Code81 => "Symbol Number (for Milestone or LOB reports)",
            Code82 => "Data Item Description (DID) Reference",
            Code83 => "Extended (or Exhibit) Line Item Number (ELIN)",
            Code84 => "Contractor Data Requirements List (CDRL)",
            Code85 => "Subcontractor Data Requirements (SDRL)",
            OperationNumber => "Operation Number",
            FunctionalCategory => "Functional Category",
            WorkCenter => "Work Center",
            AssemblyNumber => "Assembly Number",
            SubassemblyNumber => "Subassembly Number",
            CostElement => "Cost Element",
            ChangeDocumentNumber => "Change Document Number",
            FundsAuthorization => "Funds Authorization",
            FileIdentificationNumber => "File Identification Number",
            Code95 => {
                "Committee on Uniform Security Identification Procedures (CUSIP) Number"
            }
            StockCertificateNumber => "Stock Certificate Number",
            PackageNumber => "Package Number",
            ContainerPackagingSpecificationNumber => {
                "Container/Packaging Specification Number"
            }
            RateConferenceIdCode => "Rate Conference ID Code",
            AdvertiserNumber => "Advertiser Number",
            AnalysisNumberTestNumber => "Analysis number/Test number",
            DisabilityInsuranceAccountNumber => "Disability Insurance Account Number",
            AssignmentNumber => "Assignment Number",
            DisabilityInsurancePolicyNumber => "Disability Insurance Policy Number",
            EducationalInstitutionIdentificationNumber => {
                "Educational Institution Identification Number"
            }
            CodeA7 => "Flexible Spending Account (FSA) Insurance Account Number",
            CodeA8 => "Flexible Spending Account (FSA) Insurance Policy Number",
            HealthInsuranceAccountNumber => "Health Insurance Account Number",
            AccountsReceivableStatementNumber => "Accounts Receivable Statement Number",
            DistributorsSplitAgentNumber => "Distributor's Split Agent Number",
            FundManagersReferenceNumber => "Fund Manager's Reference Number",
            AgencyHierarchicalLevel => "Agency Hierarchical Level",
            OfficerLicenseNumber => "Officer License Number",
            PreviousDistributorNumber => "Previous Distributor Number",
            InterviewerId => "Interviewer ID",
            MilitaryId => "Military ID",
            OptionPolicyNumber => "Option Policy Number",
            PayrollAccountNumber => "Payroll Account Number",
            PriorContractNumber => "Prior Contract Number",
            WorksiteNumber => "Worksite Number",
            AgentNumber => "Agent Number",
            Treaty => "Treaty Identifier",
            AssociatedCaseControlNumber => "Associated Case Control Number",
            CarrierAssignedCode => "Carrier Assigned Code",
            DealerNumber => "Dealer Number",
            DirectoryNumber => "Directory Number",
            DistributorAssignedTransactionNumber => {
                "Distributor Assigned Transaction Number"
            }
            DistributorAssignedOrderNumber => "Distributor Assigned Order Number",
            DistributorsAccountNumber => "Distributor's Account Number",
            GeneralAgencyNumber => "General Agency Number",
            LaboratoryNumber => "Laboratory Number",
            AgencyAssignedNumber => "Agency Assigned Number",
            ListBillNumber => "List Bill Number",
            AccountingPeriodReference => "Accounting Period Reference",
            ParamedicalIdNumber => "Paramedical ID Number",
            AcceptableSourcePurchaserId => "Acceptable Source Purchaser ID",
            PayrollNumber => "Payroll Number",
            PersonalIdNumber => "Personal ID Number",
            PolicyLinkNumber => "Policy Link Number",
            SecondaryPolicyNumber => "Secondary Policy Number",
            SpecialQuoteNumber => "Special Quote Number",
            NationalPropertyRegistrySystemLevel1 => {
                "National Property Registry System Level 1"
            }
            NationalPropertyRegistrySystemLevel2 => {
                "National Property Registry System Level 2"
            }
            InvestorAssignedIdentificationNumber => {
                "Investor Assigned Identification Number"
            }
            MotorFuelCertificateNumber => "Motor Fuel Certificate Number",
            CodeABJ => {
                "Ginnie Mae (Government National Mortgage Association) Pool Package Number"
            }
            MortgageElectronicRegistrationSystemOrganization => {
                "Mortgage Electronic Registration System Organization Identifier"
            }
            SellerLoanNumber => "Seller Loan Number",
            SubServicerLoanNumber => "Sub-Servicer Loan Number",
            NationalPropertyRegistrySystemLevel3 => {
                "National Property Registry System Level 3"
            }
            StateHazardousWasteEntity => "State Hazardous Waste Entity Identifier",
            BankruptcyProcedureNumber => "Bankruptcy Procedure Number",
            NationalBusinessIdentificationNumber => {
                "National Business Identification Number"
            }
            CodeABR => {
                "Prior Data Universal Number System (D-U-N-S) Number, Dun & Bradstreet"
            }
            VesselName => "Vessel Name",
            SecurityInstrumentNumber => "Security Instrument Number",
            AssignmentRecordingNumber => "Assignment Recording Number",
            BookNumber => "Book Number",
            BusinessTaxNumber => "Business Tax Number",
            NorthAmericanIndustrialClassificationSystemCode2 => {
                "North American Industrial Classification System Code-2"
            }
            CentersForMedicareAndMedicaidServicesPlanId => {
                "Centers for Medicare and Medicaid Services PlanID"
            }
            EmploymentVisa => "Employment Visa",
            AirCargoTransferManifest => "Air Cargo Transfer Manifest",
            GrowthFactorReference => "Growth Factor Reference",
            Region => "Region",
            Status => "Status",
            ClassCode => "Class Code",
            ServiceRequestNumber => "Service Request Number",
            SupplementNumber => "Supplement Number",
            PreviousTicketNumber => "Previous Ticket Number",
            OneCallAgencyTicketNumber => "One Call Agency Ticket Number",
            TicketNumber => "Ticket Number",
            BillOfMaterialRevisionNumber => "Bill of Material Revision Number",
            DrawingRevisionNumber => "Drawing Revision Number",
            ApplicationTransactionReferenceNumber => {
                "Application Transaction Reference Number"
            }
            RelatedObjectIdentificationNumber => "Related Object Identification Number",
            CommonAccessReferenceNumber => "Common Access Reference Number",
            FirstTransferNumber => "First Transfer Number",
            ContinuousTransferNumber => "Continuous Transfer Number",
            LastTransferNumber => "Last Transfer Number",
            CodeACR => {
                "Automated Clearinghouse (ACH) Return/Notification of Change (NOC) Code"
            }
            SocietyOfPropertyInformationCompilersAndAnalysts => {
                "Society of Property Information Compilers and Analysts"
            }
            AccountingCode => "Accounting Code",
            GreenCard => "Green Card",
            AgencyAssignedEmployeeId => "Agency Assigned Employee ID",
            Passport => "Passport",
            UnemploymentInsuranceNumber => "Unemployment Insurance Number",
            NorthAmericanIndustrialClassificationSystemCode1 => {
                "North American Industrial Classification System Code-1"
            }
            OccupationCode => "Occupation Code",
            AcceptableSourceDunsNumber => "Acceptable Source DUNS Number",
            CodeADA => {
                "Agency for International Development Acquisition Regulation (AIDAR)"
            }
            MasterPropertyNumber => "Master Property Number",
            ProjectPropertyNumber => "Project Property Number",
            UnitPropertyNumber => "Unit Property Number",
            AssociatedPropertyNumber => "Associated Property Number",
            AssociatedNumberForLimitedCommonElementParking => {
                "Associated Number For Limited Common Element Parking"
            }
            AssociatedNumberForUnitParking => "Associated Number For Unit Parking",
            AssociatedNumberForJoinedUnitNotReSubdivided => {
                "Associated Number For Joined Unit not re-subdivided"
            }
            ProcessorIdentificationNumber => "Processor Identification Number",
            OccupationClassificationCode => "Occupation Classification Code",
            EmployeeTaxFilingStatusCode => "Employee Tax Filing Status Code",
            InsuredLocation => "Insured Location Identifier",
            AirDimensionCode => "Air Dimension Code",
            SelfInsuranceIdentificationNumber => "Self Insurance Identification Number",
            SelfInsurerOrganizationType => "Self Insurer Organization Type",
            SelfInsurerAuthorizationTypeCode => "Self Insurer Authorization Type Code",
            CountyBusinessRegistrationNumber => "County Business Registration Number",
            PostalTemplate => "Postal Template Identifier",
            ReducedEarningWeek => "Reduced Earning Week Identifier",
            FullDenialReason => "Full Denial Reason Identifier",
            FederalEnergyRegulatoryCommissionCertificateOfPublicConvenience => {
                "Federal Energy Regulatory Commission Certificate of Public Convenience"
            }
            Suspension => "Suspension Identifier",
            ManagedCareOrganizationCode => "Managed Care Organization Code",
            ManagedCareOrganizationIdentificationNumber => {
                "Managed Care Organization Identification Number"
            }
            PublicUtilitiesCommissionCertificateOfPublicConvenience => {
                "Public Utilities Commission Certificate of Public Convenience"
            }
            RetailMerchantsCertificationNumber => {
                "Retail Merchant's Certification Number"
            }
            CodeAE => "Authorization for Expense (AFE) Number",
            CodeAEA => "Numero de Cedula de Identidad (CIN) Number",
            CodeAEB => "Company's Registry Office (CRO) Number",
            GovernmentRegistrationNumber => "Government Registration Number",
            JudicialNumber => "Judicial Number",
            CodeAEE => "Numero de Identificacion Tributaria (NIT)",
            PassportNumber => "Passport Number",
            PatronNumber => "Patron Number",
            CodeAEH => "Registro Informacion Fiscal (RIF)",
            CodeAEI => "Registro Unico de Contribuyente (RUC)",
            CodeAEJ => "Superintendencia de Inversiones Extranjeras (SIEX) Number",
            TokyoShokoResearchBusiness => "Tokyo Shoko Research Business Identifier",
            CodeAEL => "Registro Nacional de Contribuyente (RNC)",
            DistributionCenterNumber => "Distribution Center Number",
            CodeAEN => {
                "Institute of Security and Future Market Development (ISFMD) Serial Number"
            }
            PublicDeedNumber => "Public Deed Number",
            StockExchangeCode => "Stock Exchange Code",
            SecretaryOfStateAssignedIdentificationNumber => {
                "Secretary of State Assigned Identification Number"
            }
            DepartmentWhereInjuryOccurredIdentification => {
                "Department Where Injury Occurred Identification"
            }
            BureauOfLaborAndStatisticsSchedule => {
                "Bureau of Labor and Statistics Schedule Identifier"
            }
            StateCharterNumber => "State Charter Number",
            EmployeeNonEmployeeClassificationQualifier => {
                "Employee/Non-Employee Classification Qualifier"
            }
            FullTimePartTimeEmployeeClassificationQualifier => {
                "Full Time/Part Time Employee Classification Qualifier"
            }
            PremiumAuditPriority => "Premium Audit Priority Identifier",
            PremiumAuditPurpose => "Premium Audit Purpose Identifier",
            PremiumAuditType => "Premium Audit Type Identifier",
            AirlinesFlightIdentificationNumber => "Airlines Flight Identification Number",
            SplitPremiumAuditChange => "Split Premium Audit Change Identifier",
            SublineOfInsurance => "Subline of Insurance",
            VerificationSourceCode => "Verification Source Code",
            UnderwritingAlertReferenceCode => "Underwriting Alert Reference Code",
            CommercialPrivatePassengerVehicleQualifier => {
                "Commercial/Private Passenger Vehicle Qualifier"
            }
            VehicleBusinessUseQualifier => "Vehicle Business Use Qualifier",
            VehicleSizeClassQualifier => "Vehicle Size Class Qualifier",
            VehicleRadiusOfOperationQualifier => "Vehicle Radius of Operation Qualifier",
            TrailerTypeQualifier => "Trailer Type Qualifier",
            StateSalesTaxIdentificationNumber => "State Sales Tax Identification Number",
            CardIssuerTransactionCode => "Card Issuer Transaction Code",
            CardBillingTypeCode => "Card Billing Type Code",
            ClientCompanyCode => "Client Company Code",
            CodeAFN => "Merchant Category Code (MCC)",
            CardAccountTypeCode => "Card Account Type Code",
            CardAccountStatusCode => "Card Account Status Code",
            CardAccountReportingLevel => "Card Account Reporting Level",
            CardAccountReporting => "Card Account Reporting Identifier",
            CodeAFS => "American Osteopathic Association (AOA) Certification Number",
            FeeSchedule => "Fee Schedule Identifier",
            CodeAFU => "United States Standard Metropolitan Statistical Area (MSA) Code",
            StateControlledSubstanceLicenseNumber => {
                "State Controlled Substance License Number"
            }
            PointOfOrigination => "Point of Origination",
            PointOfDestination => "Point of Destination",
            AssessmentNumber => "Assessment Number",
            CertificateNumber => "Certificate Number",
            AgentsShipmentNumber => "Agent's Shipment Number",
            StateOrProvinceAssignedBusinessRegistryNumber => {
                "State or Province Assigned Business Registry Number"
            }
            MunicipalityAssignedBusinessRegistryNumber => {
                "Municipality Assigned Business Registry Number"
            }
            CodeAGC => "Clave Unica de Identificacion Tributaria (CUIT)",
            CodeAGD => "Registro Unico Tributario (RUT)",
            LenderUse => "Lender Use",
            GuarantorUse => "Guarantor Use",
            SchoolUse => "School Use",
            ReservationSystemCode => "Reservation System Code",
            OrderOriginationCode => "Order Origination Code",
            FolioNumber => "Folio Number",
            CorporateIdentificationCode => "Corporate Identification Code",
            CodeAGO => "Cadastro Geral do Contribuinte (CGC)",
            ConjunctionTravelTicket => "Conjunction Travel Ticket",
            ListTracking => "List Tracking Identifier",
            AgreementNumber => "Agreement Number",
            AirHandlingCode => "Air Handling Code",
            AssociatedInvoices => "Associated Invoices",
            AccountsReceivableCustomerAccount => "Accounts Receivable Customer Account",
            CodeAK => "Sending Company Audit Number (Automated Clearinghouse Transfers)",
            CodeAL => "Accounting (Equipment) Location Number",
            AgencyLocationCode => "Agency Location Code",
            TitleCompanyCodeBookReference => "Title Company Code Book Reference",
            TitleDocumentSchedule => "Title Document Schedule",
            RecordingNumber => "Recording Number",
            TitlePolicyNumber => "Title Policy Number",
            AlienRegistrationNumber => "Alien Registration Number",
            AlternativeListId => "Alternative List ID",
            AlterationNumber => "Alteration Number",
            CodeAM => "Adjustment Memo (Charge Back)",
            AssociatedPurchaseOrders => "Associated Purchase Orders",
            AppointmentNumber => "Appointment Number",
            AccountsReceivableNumber => "Accounts Receivable Number",
            AmbulatoryPaymentClassification => "Ambulatory Payment Classification",
            CodeAPI => "American Petroleum Institute (API) Deduction Code",
            AccessCode => "Access Code",
            ArrivalCode => "Arrival Code",
            AcceptableSourceSupplierId => "Acceptable Source Supplier ID",
            CodeASL => "Atomic Safety and Licensing Board Panel (ASLBP) Number",
            AnimalSpecies => "Animal Species",
            AnimalStrain => "Animal Strain",
            AppropriationNumber => "Appropriation Number",
            MaintenanceAvailabilityType => "Maintenance Availability Type",
            AuthorizationToMeetCompetitionNumber => {
                "Authorization to Meet Competition Number"
            }
            HealthInsuranceRatingAccountNumber => {
                "Health Insurance Rating Account Number"
            }
            AirWaybillNumber => "Air Waybill Number",
            CodeAX => "Government Accounting Class Reference Number (ACRN)",
            FloorPlanApprovalNumber => "Floor Plan Approval Number",
            HealthInsurancePolicyNumber => "Health Insurance Policy Number",
            LesseeBillCodeNumber => "Lessee Bill Code Number",
            AxleRatio => "Axle Ratio",
            PreferredProviderOrganizationNumber => {
                "Preferred Provider Organization Number"
            }
            BilateralCarServiceAgreements => "Bilateral Car Service Agreements",
            HealthInsuranceRatingSuffixCode => "Health Insurance Rating Suffix Code",
            LifeInsuranceBillingAccountNumber => "Life Insurance Billing Account Number",
            LifeInsurancePolicyNumber => "Life Insurance Policy Number",
            LifeInsuranceBillingSuffixCode => "Life Insurance Billing Suffix Code",
            RetirementPlanAccountNumber => "Retirement Plan Account Number",
            RetirementPlanPolicyNumber => "Retirement Plan Policy Number",
            FranchiseTaxAccountNumber => "Franchise Tax Account Number",
            CertificateOfIncorporationNumber => "Certificate of Incorporation Number",
            BeamAssemblyCode => "Beam Assembly Code",
            StateTaxIdentificationNumber => "State Tax Identification Number",
            CharterNumber => "Charter Number",
            ReceiptNumber => "Receipt Number",
            WithdrawalAccountNumber => "Withdrawal Account Number",
            DepositAccountNumber => "Deposit Account Number",
            BusinessIdentificationNumber => "Business Identification Number",
            CodeBAJ => {
                "United States Postal Service (USPS) PLANET (PostaL AlphaNumEric coding Technique) Code"
            }
            CodeBAK => "Address Correction Service (ACS) Participation Code",
            AuthorizationNumber => "Authorization Number",
            BuyersContractNumber => "Buyer's Contract Number",
            BasicContractLineItemNumber => "Basic Contract Line Item Number",
            BirthCertificateNumber => "Birth Certificate Number",
            BorderCrossingPermitNumber => "Border Crossing Permit Number",
            BidNumber => "Bid Number",
            BadgeNumber => "Badge Number",
            BuildDirectiveNumber => "Build Directive Number",
            BusinessActivity => "Business Activity",
            BrokerEntryNumber => "Broker Entry Number",
            BillingCenterIdentification => "Billing Center Identification",
            BeginningSerialNumber => "Beginning Serial Number",
            LeaseScheduleNumberBlanket => "Lease Schedule Number - Blanket",
            BondedCarrierInternalRevenueServiceIdentificationNumber => {
                "Bonded Carrier Internal Revenue Service Identification Number"
            }
            CarriersCustomsBondNumber => "Carrier's Customs Bond Number",
            BrokersOrderNumber => "Broker's Order Number",
            BankTelegraphicNumber => "Bank Telegraphic Number",
            GovernmentBillOfLading => "Government Bill of Lading",
            BillingType => "Billing Type",
            BillOfLadingNumber => "Bill of Lading Number",
            BeginMileMarker => "Begin Mile Marker",
            BookingNumber => "Booking Number",
            BinLocationNumber => "Bin Location Number",
            BinaryObject => "Binary Object Identifier",
            AdjustmentControlNumber => "Adjustment Control Number",
            HealthMaintenanceOrganizationCodeNumber => {
                "Health Maintenance Organization Code Number"
            }
            BrokerOrSalesOfficeNumber => "Broker or Sales Office Number",
            SplitBookingNumber => "Split Booking Number",
            BatchNumber => "Batch Number",
            BuyersApprovalMark => "Buyer's Approval Mark",
            CodeBV => "Purchase Order Line Item Identifier (Buyer)",
            BlendedWithBatchNumber => "Blended With Batch Number",
            BuyersShipmentMarkNumber => "Buyer's Shipment Mark Number",
            RepairCategoryNumber => "Repair Category Number",
            ComplaintCode => "Complaint Code",
            CanadianSocialInsuranceNumber => "Canadian Social Insurance Number",
            CustomerMaterialSpecificationNumber => {
                "Customer material specification number"
            }
            CustomerProcessSpecificationNumber => "Customer process specification number",
            CustomerSpecificationNumber => "Customer specification number",
            ChangeNumber => "Change Number",
            CustomerTrackingNumberForLoanedMaterials => {
                "Customer Tracking Number For Loaned Materials"
            }
            CarnetNumber => "Carnet Number",
            ContractLineItemNumber => "Contract Line Item Number",
            CorrectedContractNumber => "Corrected Contract Number",
            PreviousCreditDebitAdjustmentNumber => {
                "Previous Credit/Debit Adjustment Number"
            }
            CostAllocationReference => "Cost Allocation Reference",
            AccidentHistory => "Accident History Identifier",
            Chemical => "Chemical Identifier",
            DischargePointIdentification => "Discharge Point Identification",
            EmissionUnitIdentificationNumber => "Emission Unit Identification Number",
            FacilityFederalIdentificationNumber => {
                "Facility Federal Identification Number"
            }
            LatitudeExpressedInDecimalDegrees => "Latitude Expressed in Decimal Degrees",
            LongitudeExpressedInDecimalDegrees => {
                "Longitude Expressed in Decimal Degrees"
            }
            CodeCAH => "Office of Regulatory Information Systems (ORIS) Code",
            Process => "Process Identifier",
            StackIdentificationNumber => "Stack Identification Number",
            FacilityStateIdentificationNumber => "Facility State Identification Number",
            CodeCAL => "U.S. Environmental Protection Agency (EPA) Hazardous Waste Code",
            CodeCAM => {
                "U.S. Environmental Protection Agency (EPA)\nIdentification Number"
            }
            Category => "Category Identifier",
            CombinedShipment => "Combined Shipment",
            CensusBlockGroup => "Census Block Group",
            ContractCoOpNumber => "Contract Co-op Number",
            CreditNoteNumber => "Credit Note Number",
            CitizenshipDocumentNumber => "Citizenship Document Number",
            ContractingDistrictTypeCode => "Contracting District Type Code",
            ClassOfContractCode => "Class of Contract Code",
            FleetReferenceNumber => "Fleet Reference Number",
            FederalRegulation => "Federal Regulation",
            ConsigneesOrderNumber => "Consignee's Order Number",
            CustomerCatalogNumber => "Customer Catalog Number",
            Chromatograph => "Chromatograph Identifier",
            UniqueConsignment => "Unique Consignment Identifier",
            CampusIdentificationNumber => "Campus Identification Number",
            CircuitNumber => "Circuit Number",
            Citation => "Citation",
            ClauseNumber => "Clause Number",
            CheckNumber => "Check Number",
            SellersCreditMemo => "Seller's Credit Memo",
            CoverageListId => "Coverage List ID",
            BuyersCreditMemo => "Buyer's Credit Memo",
            ContinuousMoveNumber => "Continuous Move Number",
            CustomerMaintenancePeriodSequenceNumber => {
                "Customer Maintenance Period Sequence Number"
            }
            Component => "Component",
            CodeCN => "Carrier's Reference Number (PRO/Invoice)",
            AssemblyControlNumber => "Assembly Control Number",
            CommitmentNumber => "Commitment Number",
            CanadianNationalStudentNumber => "Canadian National Student Number",
            CustomerOrderNumber => "Customer Order Number",
            CollocationIndicator => "Collocation Indicator",
            CertificateOfTransportation => "Certificate of Transportation",
            ConditionOfPurchaseDocumentNumber => "Condition of Purchase Document Number",
            CanadianProvinceOperatingAuthorityNumber => {
                "Canadian Province Operating Authority Number"
            }
            DiscrepantContainerPackagingNumber => "Discrepant Container Packaging Number",
            RequiredContainerPackagingNumber => "Required Container Packaging Number",
            CurrentProceduralTerminologyCode => "Current Procedural Terminology Code",
            CustomshouseBrokerLicenseNumber => "Customshouse Broker License Number",
            CustomerReferenceNumber => "Customer Reference Number",
            CasualtyReportNumber => "Casualty Report Number",
            CasualtyReportSerialNumber => "Casualty Report Serial Number",
            ConditionOfSaleDocumentNumber => "Condition of Sale Document Number",
            Cs54KeyTrainIndicatorCode => "CS54 Key Train Indicator Code",
            Cs54KeyTrainIndicatorGroupName => "CS54 Key Train Indicator Group Name",
            CensusStateCode => "Census State Code",
            ContractNumber => "Contract Number",
            CensusTractSuffix => "Census Tract Suffix",
            ClearTextClause => "Clear Text Clause",
            CodeCUB => "U.S. Customs Service (USCS) Bill of Lading Number",
            CoilNumber => "Coil Number",
            CommercialVehicleSafetyAssuranceNumber => {
                "Commercial Vehicle Safety Assurance Number"
            }
            CanadianWheatBoardPermitNumber => "Canadian Wheat Board Permit Number",
            ConsignmentClassificationId => "Consignment Classification ID",
            CommercialRegistrationNumber => "Commercial Registration Number",
            PeriodicityCode => "Periodicity Code",
            CodeCZ => "Contract Rider Number (Used in conjunction with contract number)",
            DataReliabilityCode => "Data Reliability Code",
            DrugEnforcementAdministrationOrderBlankNumber => {
                "Drug Enforcement Administration Order Blank Number"
            }
            SupplierDocumentIdentificationNumber => {
                "Supplier Document Identification Number"
            }
            NationalCouncilForPrescriptionDrugProgramsPharmacyNumber => {
                "National Council for Prescription Drug Programs Pharmacy Number"
            }
            CutNumber => "Cut Number",
            DyeLotNumber => "Dye Lot Number",
            DuplicateBillNumber => "Duplicate Bill Number",
            CoverageCode => "Coverage Code",
            LossReportNumber => "Loss Report Number",
            ClaimNumber => "Claim Number",
            DomicileBranchNumber => "Domicile Branch Number",
            DistrictAssignedId => "District Assigned ID",
            DeliveryAppointmentNumber => "Delivery Appointment Number",
            BuyersDebitMemo => "Buyer's Debit Memo",
            DealerPurchaseOrderNumber => "Dealer purchase order number",
            DocumentIdentificationCode => "Document Identification Code",
            DepositorNumber => "Depositor Number",
            CodeDF => "Defense Federal Acquisition Regulations (DFAR)",
            DrawingNumber => "Drawing Number",
            DrugEnforcementAdministrationNumber => {
                "Drug Enforcement Administration Number"
            }
            CodeDHH => {
                "Department of Health and Human Services Acquisition Regulation (HHSAR)"
            }
            DistributorInvoiceNumber => "Distributor Invoice Number",
            DistrictNumber => "District Number",
            DeliveryTicketNumber => "Delivery Ticket Number",
            DockNumber => "Dock Number",
            SellersDebitMemo => "Seller's Debit Memo",
            AssociatedProductNumber => "Associated Product Number",
            DraftNumber => "Draft Number",
            DepositNumber => "Deposit Number",
            CodeDNS => "D-U-N-S+4, D-U-N-S Number with Four Character Suffix",
            DeliveryOrderNumber => "Delivery Order Number",
            CodeDOA => "Department of Agriculture Acquisition Regulation (AGAR)",
            CodeDOC => "Department of Commerce Acquisition Regulation (CAR)",
            CodeDOE => "Department of Energy Acquisition Regulation (DEAR)",
            CodeDOI => "Department of Interior Acquisition Regulation (DIAR)",
            CodeDOJ => "Department of Justice Acquisition Regulation (JAR)",
            CodeDOL => "Department of Labor Acquisition Regulation (DOLAR)",
            DensityOrderNumber => "Density Order Number",
            CodeDOS => "Department of State Acquisition Regulation (DOSAR)",
            CodeDOT => "Department of Transportation Acquisition Regulation (TAR)",
            DepartmentNumber => "Department Number",
            DeliveryQuoteNumber => "Delivery Quote Number",
            DockReceiptNumber => "Dock Receipt Number",
            DrainholeNumber => "Drainhole Number",
            CodeDS => "Defense Priorities Allocation System (DPAS) Priority Rating",
            DepartureFromSpecificationClassCode => {
                "Departure from Specification Class Code"
            }
            DepartureFromSpecificationNumber => "Departure from Specification Number",
            DepartureFromSpecificationTypeCode => {
                "Departure from Specification Type Code"
            }
            DownstreamShipperContractNumber => "Downstream Shipper Contract Number",
            CodeDTS => {
                "Department of the Treasury Acquisition/Procurement Regulation (TAPR)"
            }
            Dependents => "Dependents Information",
            CodeDUN => "D-U-N-S Number Dun & Bradstreet",
            DiversionAuthorityNumber => "Diversion Authority Number",
            DepositSequenceNumber => "Deposit Sequence Number",
            DepartmentAgencyNumber => "Department/Agency Number",
            CodeDY => {
                "Department of Defense Transportation Service Code Number (Household Goods)"
            }
            CodeDZ => {
                "Certified Registered Nurse Anesthetist (CRNA) Provider Identification Number"
            }
            CourseSectionNumber => "Course Section Number",
            EmergencyOrderNumber => "Emergency Order Number",
            NonTeachingCredentialFieldCodes => "Non-Teaching Credential Field Codes",
            PartCausingRepairNumber => "Part Causing Repair Number",
            CodeE02 => "Classification of Instructional Programs (CIP) Codes",
            ExpansionOnEffectOfChangeNumber => "Expansion on Effect of Change Number",
            ChargeCardNumber => "Charge Card Number",
            ClaimantsClaimNumber => "Claimant's Claim Number",
            BackoutProcedureCode => "Backout Procedure Code",
            ServiceBulletinNumber => "Service Bulletin Number",
            CodeE8 => "Service Contract (Coverage) Number",
            AttachmentCode => "Attachment Code",
            MedicalRecordIdentificationNumber => "Medical Record Identification Number",
            EmbargoPermitNumber => "Embargo Permit Number",
            Circular => "Circular",
            Fund => "Fund Identifier",
            Ballot => "Ballot Identifier",
            LegislativeIdentificationNumber => "Legislative Identification Number",
            LobbiedActivity => "Lobbied Activity Identifier",
            PetitionNumber => "Petition Number",
            RelatedFormNumber => "Related Form Number",
            CodeECJ => {
                "Carrier's Bond Number Covering Instruments of International Traffic (IIT)"
            }
            ExportDeclaration => "Export Declaration",
            CodeEDA => "Department of Education Acquisition Regulation (EDAR)",
            ElectionDistrict => "Election District",
            ElectronicFundsTransferIdNumber => "Electronic Funds Transfer ID Number",
            EndingSerialNumber => "Ending Serial Number",
            FinancialClassificationCode => "Financial Classification Code",
            EmployersIdentificationNumber => "Employer's Identification Number",
            CodeEII => {
                "Importer's Bond Number Covering Instruments of International Traffic (IIT)"
            }
            PatientAccountNumber => "Patient Account Number",
            CodeEK => {
                "Healthcare Manpower Shortage Area (HMSA) Facility Identification Number"
            }
            ElectronicDevicePinNumber => "Electronic device pin number",
            ElectronicPaymentReferenceNumber => "Electronic Payment Reference Number",
            EndMileMarker => "End Mile Marker",
            EmbargoNumber => "Embargo Number",
            EndorsementNumber => "Endorsement Number",
            SubmitterIdentificationNumber => "Submitter Identification Number",
            ExportPermitNumber => "Export Permit Number",
            CodeEPA => "Environmental Protection Agency Acquisition Regulation (EPAAR)",
            EnvironmentalProtectionAgencyTransporterIdentificationNumber => {
                "Environmental Protection Agency Transporter Identification Number"
            }
            EmployerPayrollCodeLists => "Employer Payroll Code Lists",
            EquipmentNumber => "Equipment Number",
            ContainerOrEquipmentReceiptNumber => "Container or Equipment Receipt Number",
            EmployersSocialSecurityNumber => "Employer's Social Security Number",
            EstimateSequenceNumber => "Estimate Sequence Number",
            ExcessTransportation => "Excess Transportation",
            EndUsersPurchaseOrderNumber => "End User's Purchase Order Number",
            ReceiverIdentificationNumber => "Receiver Identification Number",
            EventIdentification => "Event Identification",
            MammographyCertificationNumber => "Mammography Certification Number",
            EstimateNumber => "Estimate Number",
            ExposureStateCode => "Exposure State Code",
            ReceiverSubIdentificationNumber => "Receiver Sub-identification Number",
            ElectronicDataInterchangeAgreementNumber => {
                "Electronic Data Interchange Agreement Number"
            }
            VersionCodeNational => "Version Code - National",
            VersionCodeLocal => "Version Code - Local",
            SubmissionNumber => "Submission Number",
            FacilityCertificationNumber => "Facility Certification Number",
            MedicareVersionCode => "Medicare Version Code",
            CodeF6 => "Health Insurance Claim (HIC) Number",
            CodeF7 => "New Health Insurance Claim (HIC) Number",
            OriginalReferenceNumber => "Original Reference Number",
            FreightPayorReferenceNumber => "Freight Payor Reference Number",
            CodeFA => "Federal Acquisition Regulations (FAR)",
            FannieMaeSellerServicerNumber => "Fannie Mae Seller Servicer Number",
            FileTransferFormNumber => "File Transfer Form Number",
            FilerCodeIssuedByCustoms => "Filer Code Issued by Customs",
            AssignedContractNumber => "Assigned Contract Number",
            FilerCodeIssuedByBureauOfCensus => "Filer Code Issued by Bureau of Census",
            FailureMechanismNumber => "Failure mechanism number",
            ForeignEntryNumber => "Foreign Entry Number",
            FilmNumber => "Film Number",
            FundIdentificationNumber => "Fund Identification Number",
            ClinicNumber => "Clinic Number",
            CodeFHC => {
                "Federal Housing Administration Computerized Homes Underwriting Management System (CHUMS) Identification Number"
            }
            FederalHousingAdministrationOriginatorIdentification => {
                "Federal Housing Administration Originator Identification"
            }
            File => "File Identifier",
            LineItemControlNumber => "Line Item Control Number",
            FinishLotNumber => "Finish Lot Number",
            FineLineClassification => "Fine Line Classification",
            FloodZone => "Flood Zone",
            CodeFM => "Federal Maritime Commission (FMC) Forwarders Number",
            CodeFMG => {
                "Educational Commission for Foreign Medical Graduates (ECFMG) Certification Number"
            }
            FacilityMeasurementPointNumber => "Facility Measurement Point Number",
            ForwardersAgentsReferenceNumber => "Forwarder's/Agent's Reference Number",
            FinderNumber => "Finder Number",
            DrugFormularyNumber => "Drug Formulary Number",
            ForestryPermitNumber => "Forestry Permit Number",
            FormNumber => "Form Number",
            FreightBillNumber => "Freight Bill Number",
            FreddieMacSellerServicerNumber => "Freddie Mac Seller Servicer Number",
            FinalSequenceNumber => "Final Sequence Number",
            FundSourceCode => "Fund Source Code",
            AssignedSequenceNumber => "Assigned Sequence Number",
            ForeignTradeZone => "Foreign Trade Zone",
            PremarketNotificationNumber => "Premarket Notification Number",
            CodeFTP => "File Transfer Protocol (FTP) Locator",
            CodeFTZ => "Foreign Trade Zone (FTZ) Admission Number",
            FundCode => "Fund Code",
            CodeFV => "Health Maintenance Organization (HMO) Reference Number",
            StateLicenseIdentificationNumber => "State License Identification Number",
            FinalWorkCandidateNumber => "Final Work Candidate Number",
            FailureAnalysisReportNumber => "Failure Analysis Report Number",
            ClaimOfficeNumber => "Claim Office Number",
            ProcessorsInvoiceNumber => "Processor's Invoice Number",
            PriorAuthorizationNumber => "Prior Authorization Number",
            ProviderCommercialNumber => "Provider Commercial Number",
            PredeterminationOfBenefitsIdentificationNumber => {
                "Predetermination of Benefits Identification Number"
            }
            CodeG4 => "Peer Review Organization (PRO) Approval Number",
            ProviderSiteNumber => "Provider Site Number",
            PayerAssignedResubmissionReferenceNumber => {
                "Payer Assigned Resubmission Reference Number"
            }
            ResubmissionReasonCode => "Resubmission Reason Code",
            ResubmissionNumber => "Resubmission Number",
            SecondaryEmployeeIdentificationNumber => {
                "Secondary Employee Identification Number"
            }
            GovernmentAdvanceProgress => "Government Advance Progress",
            GrainBlockNumber => "Grain Block Number",
            GovernmentContractNumber => "Government Contract Number",
            ReturnGoodsBillOfLadingNumber => "Return Goods Bill of Lading Number",
            GeographicNumber => "Geographic Number",
            SpecialtyLicenseNumber => "Specialty License Number",
            GaugeTicketNumber => "Gauge Ticket Number",
            IdentificationCardSerialNumber => "Identification Card Serial Number",
            SecondaryProviderNumber => "Secondary Provider Number",
            CornboreCertificationNumber => "Cornbore Certification Number",
            ThirdPartyReferenceNumber => "Third Party Reference Number",
            GeographicDestinationZoneNumber => "Geographic Destination Zone Number",
            LoanAcquisitionNumber => "Loan Acquisition Number",
            FolderNumber => "Folder Number",
            Exhibit => "Exhibit Identifier",
            GovernmentPriorityNumber => "Government Priority Number",
            InternalPurchaseOrderReleaseNumber => {
                "Internal Purchase Order Release Number"
            }
            GrainOrderReferenceNumber => "Grain Order Reference Number",
            CodeGS => "General Services Administration Regulations (GSAR)",
            GoodsAndServiceTaxRegistrationNumber => {
                "Goods and Service Tax Registration Number"
            }
            InternalPurchaseOrderItemNumber => "Internal Purchase Order Item Number",
            ThirdPartyPurchaseOrderNumber => "Third Party Purchase Order Number",
            ThirdPartyPurchaseOrderReleaseNumber => {
                "Third Party Purchase Order Release Number"
            }
            GroupWorkCandidateSequenceNumber => "Group Work Candidate Sequence Number",
            ThirdPartyPurchaseOrderItemNumber => "Third Party Purchase Order Item Number",
            EmptyRepositioningNumber => "Empty Repositioning Number",
            GeneralLedgerAccount => "General Ledger Account",
            HighFabricationAuthorizationNumber => "High Fabrication Authorization Number",
            HighRawMaterialAuthorizationNumber => {
                "High Raw Material Authorization Number"
            }
            GravitySourceMeterNumber => "Gravity Source Meter Number",
            SpecialClause => "Special Clause",
            QualityClause => "Quality Clause",
            StandardClause => "Standard Clause",
            CodeH8 => "Home Mortgage Disclosure Act (HMDA) Census Tract",
            PaymentHistoryReferenceNumber => "Payment History Reference Number",
            CompetentAuthority => "Competent Authority",
            CodeHB => "Bill & Hold Invoice Number",
            HeatCode => "Heat Code",
            DepartmentOfTransportationHazardousNumber => {
                "Department of Transportation Hazardous Number"
            }
            HazardousExemptionNumber => "Hazardous Exemption Number",
            EngineeringDataList => "Engineering Data List",
            CivilActionNumber => "Civil Action Number",
            FiscalCode => "Fiscal Code",
            TypeOfHouseholdGoodsCode => "Type of Household Goods Code",
            CodeHI => "Health Industry Number (HIN)",
            IdentityCardNumber => "Identity Card Number",
            JudgmentNumber => "Judgment Number",
            SirenNumber => "SIREN Number",
            SiretNumber => "SIRET Number",
            HomeMortgageDisclosureActBlockNumberArea => {
                "Home Mortgage Disclosure Act Block Number Area"
            }
            HazardousCertificationNumber => "Hazardous Certification Number",
            ShippersHazardousNumber => "Shipper's Hazardous Number",
            CodeHP => "Pack & Hold Invoice Number",
            CentersForMedicareAndMedicaidServicesNationalProvider => {
                "Centers for Medicare and Medicaid Services National Provider Identifier"
            }
            ReinsuranceReference => "Reinsurance Reference",
            Horsepower => "Horsepower",
            CodeHS => "Harmonized Code System (Canada)",
            CodeOfFederalRegulations => "Code of Federal Regulations",
            TypeOfEscrowNumber => "Type of Escrow Number",
            CodeHUD => {
                "Department of Housing and Urban Development Acquisition Regulation (HUDAR)"
            }
            EscrowFileNumber => "Escrow File Number",
            HighWideFileNumber => "High/Wide File Number",
            AutoLossItemNumber => "Auto Loss Item Number",
            PropertyLossItemNumber => "Property Loss Item Number",
            CodeHZ => {
                "Tax Agency Number (MERS [Mortgage Electronic Registration System] Federal Information Processing Standards [FIPS] Based Number)"
            }
            OwningBureauIdentificationNumber => "Owning Bureau Identification Number",
            CodeI2 => "Interstate Commerce Commission (ICC) Account Number",
            NonAmericanIdentificationNumber => "Non-American Identification Number",
            CreditCounselingIdentificationNumber => {
                "Credit Counseling Identification Number"
            }
            InvoiceIdentification => "Invoice Identification",
            CreditReportNumber => "Credit Report Number",
            Pollutant => "Pollutant",
            InternalVendorNumber => "Internal Vendor Number",
            InBondNumber => "In Bond Number",
            InboundToParty => "Inbound-to Party",
            CodeICD => "ICD-9-CM (International Classification of Diseases)",
            InsuranceCertificateNumber => "Insurance Certificate Number",
            InterchangeAgreementNumber => "Interchange Agreement Number",
            IssueNumber => "Issue Number",
            InitialFailureClaim => "Initial Failure Claim",
            InternationalFuelTaxAgreementAccountNumber => {
                "International Fuel Tax Agreement Account Number"
            }
            InsurancePolicyNumber => "Insurance Policy Number",
            InitialDealerClaimNumber => "Initial Dealer Claim Number",
            InitialSampleInspectionReportNumber => {
                "Initial Sample Inspection Report Number"
            }
            Image => "Image Identifier",
            CodeIJ => "Standard Industry Classification (SIC) Code",
            InvoiceNumber => "Invoice Number",
            InternalOrderNumber => "Internal Order Number",
            CodeIM => "Intergovernmental Maritime Organization (IMO) Number",
            CodeIMP => "Integrated Master Plan (IMP)",
            CodeIMS => "Integrated Master Schedule (IMS)",
            ConsigneesInvoiceNumber => "Consignee's Invoice Number",
            InvestigatorialNewDrugNumber => "Investigatorial New Drug Number",
            InboundToOrOutboundFromParty => "Inbound-to or Outbound-from Party",
            InspectionReportNumber => "Inspection Report Number",
            EndItem => "End Item",
            IntraPlantRouting => "Intra Plant Routing",
            ImportersReferenceNumberToLetterOfCredit => {
                "Importer's Reference Number to Letter of Credit"
            }
            InternationalRegistrationPlanAccountNumber => {
                "International Registration Plan Account Number"
            }
            InvoiceNumberSuffix => "Invoice Number Suffix",
            CodeISC => {
                "International Standard Industrial Classification (ISIC) Dominion of Canada Code (DCC)"
            }
            InternationalRegistrationPlanStickerNumber => {
                "International Registration Plan Sticker Number"
            }
            InspectionAndSurveySequenceNumber => "Inspection and Survey Sequence Number",
            InternalCustomerNumber => "Internal Customer Number",
            InitialTroubleIndication => "Initial Trouble Indication",
            BargePermitNumber => "Barge Permit Number",
            SellersInvoiceNumber => "Seller's Invoice Number",
            PartInterchangeability => "Part Interchangeability",
            ItemNumber => "Item Number",
            InsuredParcelPostNumber => "Insured Parcel Post Number",
            Proceeding => "Proceeding",
            Creditor => "Creditor",
            Attorney => "Attorney",
            Judge => "Judge",
            Trustee => "Trustee",
            OriginatingCase => "Originating Case",
            AdversaryCase => "Adversary Case",
            LeadCase => "Lead Case",
            JointlyAdministeredCase => "Jointly Administered Case",
            SubstantivelyConsolidatedCase => "Substantively Consolidated Case",
            BeginningJobSequenceNumber => "Beginning Job Sequence Number",
            CodeJB => "Job (Project) Number",
            Review => "Review",
            JointCreditSpecificationNumber => "Joint Credit Specification Number",
            UserIdentification => "User Identification",
            EndingJobSequenceNumber => "Ending Job Sequence Number",
            AutomatedUnderwritingReferenceNumber => {
                "Automated Underwriting Reference Number"
            }
            Tag => "Tag",
            MultipleListingServiceArea => "Multiple Listing Service Area",
            MultipleListingServiceSubArea => "Multiple Listing Service Sub-area",
            Packet => "Packet",
            MultipleListingServiceMapXCoordinate => {
                "Multiple Listing Service Map X Coordinate"
            }
            MultipleListingServiceMapYCoordinate => {
                "Multiple Listing Service Map Y Coordinate"
            }
            MultipleListingNumber => "Multiple Listing Number",
            MultipleListingServiceBookType => "Multiple Listing Service Book Type",
            Elevation => "Elevation",
            PropertyComponentLocation => "Property Component Location",
            JobSequenceNumber => "Job Sequence Number",
            CodeJT => "Prior Tax Identification Number (TIN)",
            PriorPhoneNumber => "Prior Phone Number",
            PriorHealthIndustryNumber => "Prior Health Industry Number",
            CodeJW => "Prior Universal Provider Identification Number (UPIN)",
            PriorPostalZipCode => "Prior Postal Zip Code",
            OriginOfShipmentHarmonizedBasedCode => {
                "Origin of Shipment Harmonized-Based Code"
            }
            GoverningClassCode => "Governing Class Code",
            ApprovalCode => "Approval Code",
            ForeignMilitarySalesNoticeNumber => "Foreign Military Sales Notice Number",
            CertifiedMailNumber => "Certified Mail Number",
            RegisteredMailNumber => "Registered Mail Number",
            CriticalityDesignator => "Criticality Designator",
            TaskOrder => "Task Order",
            PurchaseDescription => "Purchase Description",
            ParagraphNumber => "Paragraph Number",
            ProjectParagraphNumber => "Project Paragraph Number",
            InquiryRequestNumber => "Inquiry Request Number",
            DistributionList => "Distribution List",
            AssociatedContract => "Associated Contract Identifier",
            BeginningKanbanSerialNumber => "Beginning Kanban Serial Number",
            ExhibitDistributionList => "Exhibit Distribution List",
            ConfirmationServiceContract => "Confirmation Service Contract Identifier",
            SpecialInstructionsNumber => "Special Instructions Number",
            EndingKanbanSerialNumber => "Ending Kanban Serial Number",
            ForeclosingStatus => "Foreclosing Status",
            TypeOfLawSuit => "Type of Law Suit",
            TypeOfOutstandingJudgment => "Type of Outstanding Judgment",
            ConfirmationIntraday => "Confirmation Intraday Identifier",
            TaxLienJurisdiction => "Tax Lien Jurisdiction",
            DeliveryReference => "Delivery Reference",
            ContractReference => "Contract Reference",
            RentalAccountNumber => "Rental Account Number",
            CensusAutomatedFilesId => "Census Automated Files ID",
            CustomsDrawbackEntryNumber => "Customs Drawback Entry Number",
            HealthCertificateNumber => "Health Certificate Number",
            ProcuringAgency => "Procuring Agency",
            ResponseToARequestForQuotationReference => {
                "Response to a Request for Quotation Reference"
            }
            ReleaserContract => "Releaser Contract Identifier",
            ReplacementShipperContract => "Replacement Shipper Contract Identifier",
            Solicitation => "Solicitation",
            ServiceRequesterContract => "Service Requester Contract Identifier",
            RequestForQuotationReference => "Request for Quotation Reference",
            OfficeSymbol => "Office Symbol",
            DistributionStatementCode => "Distribution Statement Code",
            Certification => "Certification",
            Representation => "Representation",
            CodeKY => "Site Specific Procedures, Terms, and Conditions",
            CodeKZ => "Master Solicitation Procedures, Terms, and Conditions",
            CodeL0 => "Collision Industry Electronic Commerce Association (CIECA)",
            LettersOrNotes => "Letters or Notes",
            LocationOnProductCode => "Location on Product Code",
            LaborOperationNumber => "Labor Operation Number",
            ProposalParagraphNumber => "Proposal Paragraph Number",
            SubexhibitLineItemNumber => "Subexhibit Line Item Number",
            SubcontractLineItemNumber => "Subcontract Line Item Number",
            CustomersReleaseNumber => "Customer's Release Number",
            ConsigneesReleaseNumber => "Consignee's Release Number",
            CustomersPartNumber => "Customer's Part Number",
            ShippingLabelSerialNumber => "Shipping Label Serial Number",
            LotteryAuthorityActivationNumber => "Lottery Authority Activation Number",
            LaneNumber => "Lane Number",
            Lockbox => "Lockbox",
            LeaseNumber => "Lease Number",
            LoanNumber => "Loan Number",
            LenderEntityNumber => "Lender Entity Number",
            LocationExceptionOrderNumber => "Location Exception Order Number",
            AssemblyLineFeedLocation => "Assembly Line Feed Location",
            LeaseScheduleNumber => "Lease Schedule Number",
            LongitudeExpressedInSeconds => "Longitude Expressed in Seconds",
            CodeLI => "Line Item Identifier (Seller's)",
            CodeLIC => {
                "Health Industry Business Communications Council (HIBCC) Labeler Identification Code (LIC)"
            }
            LocalJurisdiction => "Local Jurisdiction",
            CodeLK => "Longitude expressed in Degrees, Minutes and Seconds",
            LatitudeExpressedInSeconds => "Latitude Expressed in Seconds",
            ProductPeriodForWhichLaborCostsAreFirm => {
                "Product Period for which Labor Costs are Firm"
            }
            LocalMedia => "Local Media Identifier",
            NonPickupLimitedTariffNumber => "Non Pickup Limited Tariff Number",
            LoadPlanningNumber => "Load Planning Number",
            CodeLOI => "Logical Observation Identifier Names and Codes (LOINC)",
            LossConditions => "Loss Conditions",
            ForPickupLimitedFreightTariffNumber => {
                "For Pickup Limited Freight Tariff Number"
            }
            LoanProspectorKeyNumber => "Loan Prospector Key Number",
            CodeLQ => "Latitude Expressed in Degrees, Minutes and Seconds",
            LocalStudentIdentificationNumber => "Local Student Identification Number",
            BarCodedSerialNumber => "Bar-Coded Serial Number",
            LogisticsSupportDocumentationTypeCode => {
                "Logistics Support Documentation Type Code"
            }
            LotNumber => "Lot Number",
            LocationNumber => "Location Number",
            LicensePlateNumber => "License Plate Number",
            LevyingOfficerIdentification => "Levying Officer Identification",
            LocationWithinEquipment => "Location Within Equipment",
            QualifiedProductsList => "Qualified Products List",
            DestinationOfShipmentHarmonizedBasedCode => {
                "Destination of Shipment Harmonized-Based Code"
            }
            LenderAccountNumber => "Lender Account Number",
            MexicanPedimentoNumber => "Mexican Pedimento Number",
            MaterialStorageLocation => "Material Storage Location",
            MajorForceProgram => "Major Force Program",
            CropYear => "Crop Year",
            LeaseAgreementAmendmentNumberMaster => {
                "Lease Agreement Amendment Number - Master"
            }
            MilitaryOrdnanceSecurityRiskNumber => {
                "Military Ordnance Security Risk Number"
            }
            MedicalAssistanceCategory => "Medical Assistance Category",
            LimitedPartnershipIdentificationNumber => {
                "Limited Partnership Identification Number"
            }
            TaxShelterNumber => "Tax Shelter Number",
            ShipNoticeManifestNumber => "Ship Notice/Manifest Number",
            MasterBillOfLading => "Master Bill of Lading",
            CodeMBS => "Mortgage Backed Security (MBS) Policy Number",
            Mailbox => "Mailbox",
            MicrofilmNumber => "Microfilm Number",
            CodeMCC => {
                "Carrier's Bond Number Covering Merchandise Shipment and Instruments of International Traffic (IIT)"
            }
            MotorCarrierIdentificationNumber => "Motor Carrier Identification Number",
            MornetPlusCaseNumber => "MORNETPlus Case Number",
            MagazineCode => "Magazine Code",
            HazardousWasteManifestDocumentNumber => {
                "Hazardous Waste Manifest Document Number"
            }
            MessageAddressOrId => "Message Address or ID",
            ManufacturersPartNumber => "Manufacturers Part Number",
            MeterNumber => "Meter Number",
            ManufacturingOrderNumber => "Manufacturing Order Number",
            MillOrderNumber => "Mill Order Number",
            CodeMII => {
                "Importer's Bond Number Covering Merchandise Shipment and Instruments of International Traffic (IIT)"
            }
            MornetPlusInstitutionNumber => "MORNETPlus Institution Number",
            ModelNumber => "Model Number",
            ManifestKeyNumber => "Manifest Key Number",
            MilitaryRankCivilianPayGradeNumber => {
                "Military Rank/Civilian Pay Grade Number"
            }
            MasterLeaseAgreementNumber => "Master Lease Agreement Number",
            MicrNumber => "MICR Number",
            ManufacturingOperationNumber => "Manufacturing Operation Number",
            MultiplePOSOfAnInvoice => "Multiple P.O.s of an Invoice",
            MarketingPlanIdentificationNumber => "Marketing Plan Identification Number",
            MeterProvingReportNumber => "Meter Proving Report Number",
            MerchandiseTypeCode => "Merchandise Type Code",
            EligibilityCategory => "Eligibility Category",
            MothersMedicalRecordIdentificationNumber => {
                "Mother's Medical Record Identification Number"
            }
            ManufacturersMaterialSafetyDataSheetNumber => {
                "Manufacturer's Material Safety Data Sheet Number"
            }
            MailSlot => "Mail Slot",
            MeterTicketNumber => "Meter Ticket Number",
            CodeMU => "Military Specification (MILSPEC) Number",
            MornetPlusUserIdentification => "MORNETPlus User Identification",
            MigrantNumber => "Migrant Number",
            MilitaryCallNumber => "Military Call Number",
            MaterialChangeNoticeNumber => "Material Change Notice Number",
            ModelYearNumber => "Model year number",
            MaintenanceRequestNumber => "Maintenance Request Number",
            MultipleZoneOrderNumber => "Multiple Zone Order Number",
            NominationNumber => "Nomination Number",
            LocalSchoolCourseNumber => "Local School Course Number",
            LocalSchoolDistrictCourseNumber => "Local School District Course Number",
            StatewideCourseNumber => "Statewide Course Number",
            CodeN4 => {
                "United States Department of Education, National Center for Education Statistics (NCES) Course Number"
            }
            ProviderPlanNetworkIdentificationNumber => {
                "Provider Plan Network Identification Number"
            }
            PlanNetworkIdentificationNumber => "Plan Network Identification Number",
            FacilityNetworkIdentificationNumber => {
                "Facility Network Identification Number"
            }
            SecondaryHealthInsuranceIdentificationNumber => {
                "Secondary Health Insurance Identification Number"
            }
            DataAuthenticationNumber => "Data Authentication Number",
            NorthAmericanHazardousClassificationNumber => {
                "North American Hazardous Classification Number"
            }
            CodeNAS => {
                "National Aeronautics and Space Administration FAR Supplement (NFS)"
            }
            LetterOfCreditNumber => "Letter of Credit Number",
            SecondaryCoverageCompanyNumber => "Secondary Coverage Company Number",
            LetterOfCreditDraftNumber => "Letter of Credit Draft Number",
            AbbreviatedNewDrugApplicationNumber => {
                "Abbreviated New Drug Application Number"
            }
            NewDrugApplicationNumber => "New Drug Application Number",
            LeaseRiderNumber => "Lease Rider Number",
            CodeNF => "National Association of Insurance Commissioners (NAIC) Code",
            NationalFloodInsuranceProgramCommunityName => {
                "National Flood Insurance Program Community Name"
            }
            NationalFloodInsuranceProgramCounty => {
                "National Flood Insurance Program County"
            }
            NationalFloodInsuranceProgramMapNumber => {
                "National Flood Insurance Program Map Number"
            }
            NationalFloodInsuranceProgramCommunityNumber => {
                "National Flood Insurance Program Community Number"
            }
            NationalFloodInsuranceProgramState => {
                "National Flood Insurance Program State"
            }
            NaturalGasPolicyActCategoryCode => "Natural Gas Policy Act Category Code",
            RateCardNumber => "Rate Card Number",
            CodeNI => "Military Standard (MIL-STD) Number",
            TechnicalDocumentNumber => "Technical Document Number",
            PriorCase => "Prior Case",
            TechnicalOrderNumber => "Technical Order Number",
            DiscounterRegistrationNumber => "Discounter Registration Number",
            NominationModelType => "Nomination Model Type",
            NonconformanceReportNumber => "Nonconformance Report Number",
            NoOt5AuthorityZeroMileageRate => "No OT5 Authority-zero Mileage Rate",
            PartialPaymentNumber => "Partial Payment Number",
            MedicaidRecipientIdentificationNumber => {
                "Medicaid Recipient Identification Number"
            }
            ProgressPaymentNumber => "Progress Payment Number",
            NationalStockNumber => "National Stock Number",
            AdministratorsReferenceNumber => "Administrator's Reference Number",
            NonOriginatingThirdPartyNumber => "Non-originating Third Party Number",
            PendingCase => "Pending Case",
            AssociatedPolicyNumber => "Associated Policy Number",
            RelatedNonconformanceNumber => "Related Nonconformance Number",
            AgentClaimNumber => "Agent Claim Number",
            CriticalApplication => "Critical Application",
            OuterContinentalShelfAreaCode => "Outer Continental Shelf Area Code",
            OuterContinentalShelfBlockNumber => "Outer Continental Shelf Block Number",
            Ot5AuthorityConditionOrRestrictionOnCarHireRate => {
                "OT5 Authority-Condition or Restriction on Car Hire Rate"
            }
            CodeO7 => "On-line Procurement and Accounting Control (OPAC) Transaction",
            OriginalFiling => "Original Filing",
            ContinuationFiling => "Continuation Filing",
            OutletNumber => "Outlet Number",
            OceanBillOfLading => "Ocean Bill of Lading",
            OceanContainerNumber => "Ocean Container Number",
            OriginalReturnRequestReferenceNumber => {
                "Original Return Request Reference Number"
            }
            OpenAndPrepaidStationListNumber => "Open and Prepaid Station List Number",
            OperatorIdentificationNumber => "Operator Identification Number",
            Offer => "Offer Identifier",
            TerminationFiling => "Termination Filing",
            OriginHouse => "Origin House",
            OriginalInvoiceNumber => "Original Invoice Number",
            Object => "Object Identifier",
            AmendmentFiling => "Amendment Filing",
            OfferGroup => "Offer Group",
            OriginalShippersBillOfLadingNumber => {
                "Original Shipper's Bill of Lading Number"
            }
            OceanManifest => "Ocean Manifest",
            DealerOrderNumber => "Dealer Order Number",
            OutOfServiceNumber => "Out of Service Number",
            OriginalPurchaseOrder => "Original Purchase Order",
            CodeOPE => {
                "National Center for Education Statistics Office of Postsecondary Education (OPE) Code"
            }
            CodeOPF => {
                "National Center for Education Statistics Integrated Postsecondary Education Data System (IPEDS) Athletic Conference Code"
            }
            OrderNumber => "Order Number",
            OrderParagraphNumber => "Order/Paragraph Number",
            OutboundFromParty => "Outbound-from Party",
            SalesAllowanceNumber => "Sales Allowance Number",
            TariffSupplementNumber => "Tariff Supplement Number",
            TariffSuffixNumber => "Tariff Suffix Number",
            ServiceOrderNumber => "Service Order Number",
            StatementNumber => "Statement Number",
            ProductNumber => "Product Number",
            PreviousContractNumber => "Previous Contract Number",
            PreviousDrugEnforcementAdministrationNumber => {
                "Previous Drug Enforcement Administration Number"
            }
            PreviousCustomerReferenceNumber => "Previous customer reference number",
            ProjectCode => "Project Code",
            PositionCode => "Position Code",
            PipelineNumber => "Pipeline Number",
            ProductLineNumber => "Product Line Number",
            PickupReferenceNumber => "Pickup Reference Number",
            PageNumber => "Page Number",
            PriceAreaNumber => "Price Area Number",
            PatentCooperationTreatyApplicationNumber => {
                "Patent Cooperation Treaty Application Number"
            }
            NonprovisionalPatentApplicationNumber => {
                "Nonprovisional Patent Application Number"
            }
            ProvisionalPatentApplicationNumber => "Provisional Patent Application Number",
            CodePB => {
                "Payer's Financial Institution Account Number for Check, Draft, or Wire Payments; Originating Company Account Number for ACH Transfers"
            }
            ProductionCode => "Production Code",
            PoolContractCode => "Pool Contract Code",
            ProtocolNumber => "Protocol Number",
            PromotionDealNumber => "Promotion/Deal Number",
            PartialDenialIndicator => "Partial Denial Indicator",
            PreviousDriversLicense => "Previous Driver's License",
            PartialDenialReason => "Partial Denial Reason Identifier",
            PlantNumber => "Plant Number",
            PrimeContractorContractNumber => "Prime Contractor Contract Number",
            ProductGroup => "Product Group",
            PackingGroupCode => "Packing Group Code",
            DownstreamPackage => "Downstream Package Identifier",
            PlugNumber => "Plug Number",
            ProposedGroupWorkCandidateSequenceNumber => {
                "Proposed Group Work Candidate Sequence Number"
            }
            PriorityRating => "Priority Rating",
            ProcessHandlingCode => "Process Handling Code",
            PhysicianStateLicenseNumber => "Physician State License Number",
            PriceListChangeOrIssueNumber => "Price List Change or Issue Number",
            ProgramIdentificationNumber => "Program Identification Number",
            PlatformIdentificationNumber => "Platform Identification Number",
            PackerNumber => "Packer Number",
            PreviousReportNumber => "Previous Report Number",
            PackingListNumber => "Packing List Number",
            Package => "Package Identifier",
            UpstreamPackage => "Upstream Package Identifier",
            PriceListNumber => "Price List Number",
            ProductLicensingAgreementNumber => "Product Licensing Agreement Number",
            ProposedContractNumber => "Proposed Contract Number",
            PartNumber => "Part Number",
            PremarketApplicationNumber => "Premarket Application Number",
            PermitNumber => "Permit Number",
            PatentNumber => "Patent Number",
            PurchaseOrderNumber => "Purchase Order Number",
            PolicyNumber => "Policy Number",
            PositionTitleNumber => "Position Title Number",
            PurchaseOrderRevisionNumber => "Purchase Order Revision Number",
            CertificateOfPurchaseNumber => "Certificate of Purchase Number",
            TaxBillIdentificationNumber => "Tax Bill Identification Number",
            CurrentYearTaxBillNumber => "Current Year Tax Bill Number",
            PastYearTaxBillNumber => "Past Year Tax Bill Number",
            PaymentPlanNumber => "Payment Plan Number",
            PayeeIdentification => "Payee Identification",
            PriceQuoteNumber => "Price Quote Number",
            PreviouslyReportedSocialSecurityNumber => {
                "Previously Reported Social Security Number"
            }
            ProductType => "Product Type",
            PurchaseOrderNumberSuffix => "Purchase Order Number Suffix",
            PreviousShipmentIdentificationNumberContinuousMove => {
                "Previous Shipment Identification Number - Continuous Move"
            }
            NextShipmentIdentificationNumberContinuousMove => {
                "Next Shipment Identification Number - Continuous Move"
            }
            CreditCard => "Credit Card",
            ProposedSequenceNumber => "Proposed Sequence Number",
            PurchaseOptionAgreement => "Purchase Option Agreement",
            PatentType => "Patent Type",
            PreviousBillOfLadingNumber => "Previous Bill of Lading Number",
            PickupAppointmentNumber => "Pickup Appointment Number",
            ProductChangeInformationNumber => "Product change information number",
            PaymentValidationCode => "Payment Validation Code",
            PriorPurchaseOrderNumber => "Prior purchase order number",
            PreliminaryWorkCandidateNumber => "Preliminary Work Candidate Number",
            ProposedWorkCandidateSequenceNumber => {
                "Proposed Work Candidate Sequence Number"
            }
            PreviousInvoiceNumber => "Previous Invoice Number",
            HealthCareProviderTaxonomyCode => "Health Care Provider Taxonomy Code",
            CodePY => {
                "Payee's Financial Institution Account Number for Check, Draft or Wire Payments; Receiving Company Account Number for ACH Transfer"
            }
            PayrollActivityCode => "Payroll Activity Code",
            PayRange => "Pay Range",
            ProductChangeNoticeNumber => "Product Change Notice Number",
            QuoteNumber => "Quote Number",
            StartingPackageNumber => "Starting Package Number",
            EndingPackageNumber => "Ending Package Number",
            PriorIdentifierNumber => "Prior Identifier Number",
            PropertyControlNumber => "Property Control Number",
            RecallNumber => "Recall Number",
            ReceiverClaimNumber => "Receiver Claim Number",
            RegistrationNumber => "Registration Number",
            RepairOrderNumber => "Repair Order Number",
            Press => "Press Identifier",
            PressForm => "Press Form Identifier",
            ProductSpecificationDocumentNumber => "Product Specification Document Number",
            ReplacementDrugEnforcementAdministrationNumber => {
                "Replacement Drug Enforcement Administration Number"
            }
            ReplacementCustomerReferenceNumber => "Replacement Customer Reference Number",
            QualityDispositionArea => "Quality Disposition Area Identifier",
            ReplacementAssemblyModelNumber => "Replacement Assembly Model Number",
            ReplacementAssemblySerialNumber => "Replacement Assembly Serial Number",
            QualityInspectionArea => "Quality Inspection Area Identifier",
            ReturnMaterialAuthorizationNumber => "Return Material Authorization Number",
            SalesProgramNumber => "Sales Program Number",
            ServiceAuthorizationNumber => "Service Authorization Number",
            QualityReviewMaterialCrib => "Quality Review Material Crib Identifier",
            StopSequenceNumber => "Stop Sequence Number",
            ServiceEstimateNumber => "Service Estimate Number",
            SubstitutePartNumber => "Substitute Part Number",
            UnitNumber => "Unit Number",
            QualityReportNumber => "Quality Report Number",
            WarrantyCoverageCode => "Warranty Coverage Code",
            WarrantyRegistrationNumber => "Warranty Registration Number",
            ChangeVerificationProcedureCode => "Change Verification Procedure Code",
            MajorSystemAffectedCode => "Major System Affected Code",
            NewPartNumber => "New Part Number",
            OldPartNumber => "Old Part Number",
            ServicePerformedCode => "Service Performed Code",
            ReferenceDrawingNumber => "Reference Drawing Number",
            CodeR0 => {
                "Regiristo Federal de Contribuyentes (Mexican Federal Tax ID Number)"
            }
            CurrentRevisionNumber => "Current Revision Number",
            CanceledRevisionNumber => "Canceled Revision Number",
            CorrectionNumber => "Correction Number",
            TariffSectionNumber => "Tariff Section Number",
            TariffPageNumber => "Tariff Page Number",
            TariffRuleNumber => "Tariff Rule Number",
            AccountsReceivableOpenItem => "Accounts Receivable Open Item",
            RentalAgreementNumber => "Rental Agreement Number",
            RejectionNumber => "Rejection Number",
            RepetitiveCargoShipmentNumber => "Repetitive Cargo Shipment Number",
            RestrictedAvailabilityAuthorization => {
                "Restricted Availability Authorization"
            }
            RestrictedAvailabilityNumber => "Restricted Availability Number",
            RateCodeNumber => "Rate code number",
            RailRoutingCode => "Rail Routing Code",
            ReelNumber => "Reel Number",
            ReleaseNumber => "Release Number",
            RelatedCase => "Related Case",
            ExportReferenceNumber => "Export Reference Number",
            RouteOrderNumberDomestic => "Route Order Number-Domestic",
            RegulatoryGuideline => "Regulatory Guideline Identifier",
            RouteOrderNumberExport => "Route Order Number-Export",
            ReleaseInvoiceNumberForPriorBillAndHold => {
                "Release invoice number for prior bill and hold"
            }
            RigNumber => "Rig Number",
            RouteOrderNumberEmergency => "Route Order Number-Emergency",
            RackTypeNumber => "Rack Type Number",
            ReserveAssemblyLineFeedLocation => "Reserve Assembly Line Feed Location",
            RoleIdentificationNumber => "Role Identification Number",
            CodeRM => "Raw material supplier Dun & Bradstreet number",
            RunNumber => "Run Number",
            RepetitiveBookingNumber => "Repetitive Booking Number",
            RepetitivePatternCode => "Repetitive Pattern Code",
            RelativePriority => "Relative Priority",
            RegulationPrimaryNumber => "Regulation Primary Number",
            ReportNumber => "Report Number",
            PurchaseRequisitionNumber => "Purchase Requisition Number",
            CodeRR => {
                "Payer's Financial Institution Transit Routing Number for Check, Draft or Wire Payments. Originating Depository Financial Institution Routing Number for ACH Transfers"
            }
            RoutingRequestControlNumber => "Routing Request Control Number",
            ReconciliationReportSectionIdentificationCode => {
                "Reconciliation Report Section Identification Code"
            }
            ReturnableContainerSerialNumber => "Returnable Container Serial Number",
            ReservationNumber => "Reservation Number",
            RegulationSecondaryNumber => "Regulation Secondary Number",
            CodeRT => {
                "Payee's Financial Institution Transit Routing Number for Check, Draft or Wire Payments. Receiving Depository Financial Institution Transit Routing Number for ACH Transfers"
            }
            RouteNumber => "Route Number",
            ReceivingNumber => "Receiving Number",
            CodeRW => {
                "Repetitive Waybill Code (Origin Carrier, Standard Point Location Code, Repetitive Waybill Code Number)"
            }
            ReportingWeek => "Reporting Week",
            ResubmitNumber => "Resubmit number",
            RebateNumber => "Rebate Number",
            ReturnedGoodsAuthorizationNumber => "Returned Goods Authorization Number",
            SpecialApproval => "Special Approval",
            EngineeringSpecificationNumber => "Engineering Specification Number",
            DataSource => "Data Source",
            SpecificationNumber => "Specification Number",
            ShippersBondNumber => "Shippers Bond Number",
            RoutingInstructionNumber => "Routing Instruction Number",
            StockNumber => "Stock Number",
            StackTrainIdentification => "Stack Train Identification",
            SealOffNumber => "Seal Off Number",
            SealOnNumber => "Seal On Number",
            Salesperson => "Salesperson",
            SalaryStep => "Salary Step",
            SalesRegionNumber => "Sales Region Number",
            SuretyBondNumber => "Surety Bond Number",
            ShipperCarOrderNumber => "Shipper Car Order Number",
            CodeSCA => "Standard Carrier Alpha Code (SCAC)",
            ScaleNumber => "Scale Number",
            SubdayNumber => "Subday Number",
            SchoolDistrictTypeCode => "School District Type Code",
            SerialNumber => "Serial Number",
            SearchKey => "Search Key",
            Session => "Session",
            ShipFrom => "Ship From",
            Savings => "Savings",
            SenderDefinedClause => "Sender Defined Clause",
            ShelfLifeIndicator => "Shelf Life Indicator",
            CodeSI => "Shipper's Identifying Number for Shipment (SID)",
            SalvageInstruction => "Salvage Instruction Identifier",
            SetNumber => "Set Number",
            ServiceChangeNumber => "Service Change Number",
            SalesTerritoryCode => "Sales/Territory Code",
            SalesOfficeNumber => "Sales Office Number",
            SettlementMethodCode => "Settlement Method Code",
            StateOfMassachusettsTownCode => "State of Massachusetts Town Code",
            SealNumber => "Seal Number",
            CodeSNH => "SNOMED, Systematized Nomenclature of Medicine",
            CodeSNP => "US Customs & Border Protection Second Notify Party",
            StateNonResidentViolatorCompact => "State Non-Resident Violator Compact",
            CodeSO => "Shipper's Order (Invoice Number)",
            ScanLine => "Scan Line",
            CodeSPL => "Standard Point Location Code (SPLC)",
            TheaterScreenNumber => "Theater Screen Number",
            ContainerSequenceNumber => "Container Sequence Number",
            SalesResponsibility => "Sales Responsibility",
            SplitShipmentNumber => "Split Shipment Number",
            SchoolSystemTypeCode => "School System Type Code",
            StoreNumber => "Store Number",
            CodeSTB => "Standard Transportation Commodity Code (STCC) Bridge Number",
            CodeSTR => "Standard Transportation Commodity Code (STCC) Replacement Code",
            ServiceabilityStandardTestingReference => {
                "Serviceability Standard Testing Reference"
            }
            SpecialProcessingCode => "Special Processing Code",
            TitleReference => "Title Reference",
            SupervisoryUnionCode => "Supervisory Union Code",
            SpacingUnitOrderNumber => "Spacing Unit Order Number",
            ServiceChargeNumber => "Service Charge Number",
            SellersSaleNumber => "Seller's Sale Number",
            ServiceInterruptTrackingNumber => "Service Interrupt Tracking Number",
            SocialSecurityNumber => "Social Security Number",
            SpecificationRevision => "Specification Revision",
            DealerTypeIdentification => "Dealer Type Identification",
            TaxExchangeCode => "Tax Exchange Code",
            TaxFormCode => "Tax Form Code",
            TaxScheduleCode => "Tax Schedule Code",
            SignalCode => "Signal Code",
            TrailerUseAgreements => "Trailer Use Agreements",
            TaxFiling => "Tax Filing",
            AffectedSubsystemCode => "Affected Subsystem Code",
            DescriptionOfChangeCode => "Description of Change Code",
            DocumentationAffectedNumber => "Documentation Affected Number",
            TelecommunicationCircuitSupplementalId => {
                "Telecommunication Circuit Supplemental ID"
            }
            TruckersBillOfLading => "Trucker's Bill of Lading",
            VendorTerms => "Vendor Terms",
            ReasonForChange => "Reason for Change",
            TechnicalDocumentationType => "Technical Documentation Type",
            CodeTE => "Federal Maritime Commission (FMC) Tariff Number",
            TransferNumber => "Transfer Number",
            TimeFailure => "Time Failure",
            CodeTG => "Transportation Control Number (TCN)",
            CodeTH => "Transportation Account Code (TAC)",
            TirNumber => "TIR Number",
            TechnicalInformationPackage => "Technical Information Package",
            FederalTaxpayersIdentificationNumber => {
                "Federal Taxpayer's Identification Number"
            }
            TankNumber => "Tank Number",
            TaxLicenseExemption => "Tax License Exemption",
            CodeTM => "Travel Manifest (ACI or OTR)",
            TransactionReferenceNumber => "Transaction Reference Number",
            TerminalOperatorNumber => "Terminal Operator Number",
            TypeOfComment => "Type of Comment",
            TestSpecificationNumber => "Test Specification Number",
            TransponderNumber => "Transponder Number",
            TracerActionRequestNumber => "Tracer Action Request Number",
            GovernmentTransportationRequest => "Government Transportation Request",
            TariffNumber => "Tariff Number",
            TemplateSequenceNumber => "Template Sequence Number",
            TerminalCode => "Terminal Code",
            TrialLocationCode => "Trial Location Code",
            LineOfBusiness => "Line of Business",
            TaxWorksheet => "Tax Worksheet",
            TaxExemptNumber => "Tax Exempt Number",
            PolicyType => "Policy Type",
            TotalCycleNumber => "Total Cycle Number",
            ConsolidatorsReceiptNumber => "Consolidator's Receipt Number",
            RegionalAccountNumber => "Regional Account Number",
            Term => "Term",
            CodeU3 => "Unique Supplier Identification Number (USIN)",
            UnpaidInstallmentReferenceNumber => "Unpaid Installment Reference Number",
            SuccessorAccount => "Successor Account",
            PredecessorAccount => "Predecessor Account",
            CodeU8 => "Mortgage Backed Security (MBS) Loan Number",
            CodeU9 => "Mortgage Backed Security (MBS) Pool Number",
            MortgageNumber => "Mortgage Number",
            UnacceptableSourcePurchaserId => "Unacceptable Source Purchaser ID",
            MortgageInsuranceIndicatorNumber => "Mortgage Insurance Indicator Number",
            CodeUCB => "EAN.UCC Bill of Lading Number (17 Digits)",
            CodeUCM => "EAN.UCC Master Bill of Lading Number (17 Digits)",
            UnacceptableSourceDunsNumber => "Unacceptable Source DUNS Number",
            SecondaryCoverageCertificateNumber => "Secondary Coverage Certificate Number",
            MortgageInsuranceCompanyNumber => "Mortgage Insurance Company Number",
            USGovernmentTransportationControlNumber => {
                "U.S. Government Transportation Control Number"
            }
            RemovalNumber => "Removal Number",
            PreviousCourseNumber => "Previous Course Number",
            CodeUIC => "Unit Identification Code (UIC)",
            CurrentOrLatestCourseNumber => "Current or Latest Course Number",
            EquivalentCourseNumberAtRequestingInstitution => {
                "Equivalent Course Number at Requesting Institution"
            }
            CrossListedCourseNumber => "Cross-listed Course Number",
            QuarterQuarterSectionNumber => "Quarter Quarter Section Number",
            UnitedNationsHazardousClassificationNumber => {
                "United Nations Hazardous Classification Number"
            }
            QuarterQuarterSpotNumber => "Quarter Quarter Spot Number",
            UpstreamShipperContractNumber => "Upstream Shipper Contract Number",
            SectionNumber => "Section Number",
            UnitReliefNumber => "Unit Relief Number",
            UniformResourceLocator => "Uniform Resource Locator",
            UnitReportPeriod => "Unit Report Period",
            UnitReportPeriodId => "Unit Report Period ID",
            UnacceptableSourceSupplierId => "Unacceptable Source Supplier ID",
            UnitTrain => "Unit Train",
            TownshipNumber => "Township Number",
            RangeNumber => "Range Number",
            StateSenateDistrict => "State Senate District",
            StateAssemblyDistrict => "State Assembly District",
            CodeUY => "Federal National Mortgage Association (Fannie Mae) Loan Number",
            StateLegislativeDistrict => "State Legislative District",
            Version => "Version",
            VolumePurchaseAgreementNumber => "Volume Purchase Agreement Number",
            VisaType => "Visa Type",
            VoyageNumber => "Voyage Number",
            StateDepartmentI20FormNumber => "State Department I-20 Form Number",
            StateDepartmentIap66FormNumber => "State Department IAP-66 Form Number",
            CodeV6 => "North American Free Trade Agreement (NAFTA) Compliance Number",
            JudicialDistrict => "Judicial District",
            InstitutionNumber => "Institution Number",
            Subservicer => "Subservicer",
            VesselAgentNumber => "Vessel Agent Number",
            VeteransAdministrationOriginatorIdentification => {
                "Veterans Administration Originator Identification"
            }
            CodeVB => "Department of Veterans Affairs Acquisition Regulations (VAAR)",
            VendorContractNumber => "Vendor Contract Number",
            VolumeNumber => "Volume Number",
            VendorAbbreviationCode => "Vendor Abbreviation Code",
            VendorChangeIdentificationCode => "Vendor Change Identification Code",
            VendorChangeProcedureCode => "Vendor Change Procedure Code",
            VehicleGaragedStateCode => "Vehicle Garaged State Code",
            CountyLegislativeDistrict => "County Legislative District",
            PoolNumber => "Pool Number",
            InvestorNoteHolderIdentification => "Investor Note Holder Identification",
            InstitutionNoteHolderIdentification => {
                "Institution Note Holder Identification"
            }
            ThirdPartyNoteHolderIdentification => {
                "Third Party Note Holder Identification"
            }
            Ward => "Ward",
            VendorOrderNumber => "Vendor Order Number",
            InstitutionLoanNumber => "Institution Loan Number",
            VendorProductNumber => "Vendor Product Number",
            RelatedContractLineItemNumber => "Related Contract Line Item Number",
            VendorIdNumber => "Vendor ID Number",
            VendorOrderNumberSuffix => "Vendor Order Number Suffix",
            MotorVehicleIdNumber => "Motor Vehicle ID Number",
            PreparersVerificationNumber => "Preparer's Verification Number",
            Voucher => "Voucher",
            Standard => "Standard",
            CodeVX => "Value-Added Tax Registration Number (Europe)",
            LinkSequenceNumber => "Link Sequence Number",
            SponsorsReferenceNumber => "Sponsor's Reference Number",
            DisposalTurnInDocumentNumber => "Disposal Turn-In Document Number",
            WeaponSystemNumber => "Weapon System Number",
            ManufacturingDirectiveNumber => "Manufacturing Directive Number",
            ProcurementRequestNumber => "Procurement Request Number",
            InspectorIdentificationNumber => "Inspector Identification Number",
            FederalSupplyScheduleNumber => "Federal Supply Schedule Number",
            CodeW7 => "Commercial and Government Entity (CAGE) Code",
            Suffix => "Suffix",
            SpecialPackagingInstructionNumber => "Special Packaging Instruction Number",
            LaborOrAffiliationIdentification => "Labor or Affiliation Identification",
            CodeWB => "American Petroleum Institute (API) Well",
            ContractOptionNumber => "Contract Option Number",
            WorkCandidateSequenceNumber => "Work Candidate Sequence Number",
            ReviewPeriodNumber => "Review Period Number",
            WithdrawalRecord => "Withdrawal Record",
            WellClassificationCode => "Well Classification Code",
            LocallyAssignedControlNumber => "Locally Assigned Control Number",
            VendorsPreviousJobNumber => "Vendor's Previous Job Number",
            CodeWH => "Master Reference (Link) Number",
            Waiver => "Waiver",
            PreAwardSurvey => "Pre-Award Survey",
            TypeOfScienceCode => "Type of Science Code",
            FederalSupplyClassificationCode => "Federal Supply Classification Code",
            WeightAgreementNumber => "Weight Agreement Number",
            WellNumber => "Well Number",
            WorkOrderNumber => "Work Order Number",
            WarehousePickTicketNumber => "Warehouse Pick Ticket Number",
            InterimFundingOrganizationLoanNumber => {
                "Interim Funding Organization Loan Number"
            }
            WarehouseReceiptNumber => "Warehouse Receipt Number",
            WarehouseStorageLocationNumber => "Warehouse storage location number",
            BrokersReferenceNumber => "Broker's Reference Number",
            Vessel => "Vessel",
            DealerIdentification => "Dealer Identification",
            DepositoryTrustCompanyIdentification => {
                "Depository Trust Company Identification"
            }
            DistributorsAccountIdentification => "Distributor's Account Identification",
            WaybillNumber => "Waybill Number",
            DistributorsRepresentativeIdentification => {
                "Distributor's Representative Identification"
            }
            DebtorsAccount => "Debtor's Account",
            ProviderClaimNumber => "Provider Claim Number",
            SpecificationClassNumber => "Specification Class Number",
            DefectCodeNumber => "Defect Code Number",
            ClinicalLaboratoryImprovementAmendmentNumber => {
                "Clinical Laboratory Improvement Amendment Number"
            }
            StateIndustrialAccidentProviderNumber => {
                "State Industrial Accident Provider Number"
            }
            OriginalVoucherNumber => "Original Voucher Number",
            BatchSequenceNumber => "Batch Sequence Number",
            SecondarySuffixCodeIndicator => "Secondary Suffix Code Indicator",
            InternalControlNumber => "Internal Control Number",
            SubstituteNationalStockNumber => "Substitute National Stock Number",
            SubstituteManufacturersPartNumber => "Substitute Manufacturer's Part Number",
            CargoControlNumber => "Cargo Control Number",
            SubsistenceIdentificationNumber => "Subsistence Identification Number",
            TransportationPriorityNumber => "Transportation Priority Number",
            GovernmentBillOfLadingOfficeCode => "Government Bill of Lading Office Code",
            AirlineTicketNumber => "Airline Ticket Number",
            ContractAuditorIdNumber => "Contract Auditor ID Number",
            FederalHomeLoanMortgageCorporationLoanNumber => {
                "Federal Home Loan Mortgage Corporation Loan Number"
            }
            FederalHomeLoanMortgageCorporationDefaultForeclosureSpecialistNumber => {
                "Federal Home Loan Mortgage Corporation Default/Foreclosure Specialist Number"
            }
            MortgageeLoanNumber => "Mortgagee Loan Number",
            InsuredsLoanNumber => "Insured's Loan Number",
            IssuerNumber => "Issuer Number",
            TitleXixIdentifierNumber => "Title XIX Identifier Number",
            SampleNumber => "Sample Number",
            PreviousCargoControlNumber => "Previous Cargo Control Number",
            PierNumber => "Pier Number",
            RailroadCommissionRecordNumber => "Railroad Commission Record Number",
            GasAnalysisSourceMeterNumber => "Gas Analysis Source Meter Number",
            ToxicologyId => "Toxicology ID",
            UniversalTransverseMercatorNorth => "Universal Transverse Mercator - North",
            UniversalTransverseMercatorEast => "Universal Transverse Mercator - East",
            UniversalTransverseMercatorZone => "Universal Transverse Mercator - Zone",
            RatingPeriod => "Rating Period",
            SpecialProgramCode => "Special Program Code",
            ServiceAreaCode => "Service Area Code",
            FunctionCode => "Function Code",
            ObjectCode => "Object Code",
            OrganizationCode => "Organization Code",
            SubjectAreaCode => "Subject Area Code",
            ScheduleTypeCode => "Schedule Type Code",
            AlternatingScheduleIdentifierCode => "Alternating Schedule Identifier Code",
            OtherUnlistedTypeOfReferenceNumber => {
                "Other Unlisted Type of Reference Number"
            }
            PharmacyPrescriptionNumber => "Pharmacy Prescription Number",
            Debtor => "Debtor",
            ClaimAdministratorClaimNumber => "Claim Administrator Claim Number",
            ThirdPartyAdministratorClaimNumber => {
                "Third-Party Administrator Claim Number"
            }
            ContractHolderClaimNumber => "Contract Holder Claim Number",
            AgencyClaimNumber => "Agency Claim Number",
            DeliveryTrailerManifest => "Delivery Trailer Manifest",
            SortAndSegregate => "Sort and Segregate",
            UserId => "User ID",
            CurrentCertificateNumber => "Current Certificate Number",
            PriorCertificateNumber => "Prior Certificate Number",
            RevisionNumber => "Revision Number",
            Tract => "Tract",
            BuyerIdentification => "Buyer Identification",
            RailroadCommissionOilNumber => "Railroad Commission Oil Number",
            LesseeIdentification => "Lessee Identification",
            OperatorAssignedUnitNumber => "Operator Assigned Unit Number",
            RefinerIdentification => "Refiner Identification",
            RevenueSource => "Revenue Source",
            RentPayorIdentification => "Rent Payor Identification",
            AllowanceRecipientIdentification => "Allowance Recipient Identification",
            ResourceScreeningReference => "Resource Screening Reference",
            ReceiverIdQualifier => "Receiver ID Qualifier",
            Formation => "Formation",
            SellingArrangement => "Selling Arrangement",
            MinimumRoyaltyPayorIdentification => "Minimum Royalty Payor Identification",
            OperatorLeaseNumber => "Operator Lease Number",
            YardPosition => "Yard Position",
            ReporterIdentification => "Reporter Identification",
            ParticipatingArea => "Participating Area",
            EngineeringChangeProposal => "Engineering Change Proposal",
            GeographicScore => "Geographic Score",
            GeographicKey => "Geographic Key",
            GeographicIndex => "Geographic Index",
            SafetyOfShipCertificate => "Safety of Ship Certificate",
            SafetyOfRadioCertificate => "Safety of Radio Certificate",
            SafetyEquipmentCertificate => "Safety Equipment Certificate",
            CivilLiabilitiesOfOilCertificate => "Civil Liabilities of Oil Certificate",
            LoadLineCertificate => "Load Line Certificate",
            DeratCertificate => "Derat Certificate",
            MaritimeDeclarationOfHealth => "Maritime Declaration of Health",
            FederalHousingAdministrationCaseNumber => {
                "Federal Housing Administration Case Number"
            }
            VeteransAffairsCaseNumber => "Veterans Affairs Case Number",
            Supplier => "Supplier",
            UltimateConsignee => "Ultimate Consignee",
            ConnectingCarrier => "Connecting Carrier",
            FamilyMemberIdentification => "Family Member Identification",
            CoalAuthorityNumber => "Coal Authority Number",
            SalesRepresentativeOrderNumber => "Sales Representative Order Number",
            CarrierAssignedReferenceNumber => "Carrier Assigned Reference Number",
            ReferenceVersionNumber => "Reference Version Number",
            CodeZJ => "Universal Railroad Revenue Waybill Identified Number (URRWIN)",
            DuplicateWaybillInRoute => "Duplicate Waybill in Route",
            DuplicateWaybillNotInRoute => "Duplicate Waybill Not in Route",
            ManufacturerNumber => "Manufacturer Number",
            AgencyCaseNumber => "Agency Case Number",
            MakegoodCommercialLineNumber => "Makegood Commercial Line Number",
            SpouseTie => "Spouse Tie",
            NonSpouseTie => "Non-Spouse Tie",
            CodeZR => "Supplier (Replacement)",
            SoftwareApplicationNumber => "Software Application Number",
            MillingInTransit => "Milling in Transit",
            CodeZTS => "Zone, Track, Spot Number (ZTS)",
            Field => "Field",
            Block => "Block",
            Area => "Area",
            CountyCode => "County Code",
            ReferencedPatternIdentification => "Referenced Pattern Identification",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<ReferenceIdentificationQualifier> {
        {
            use ReferenceIdentificationQualifier::*;
            match description {
                "Contracting District Number" => Some(ContractingDistrictNumber),
                "Supervisory Appraiser Certification Number" => {
                    Some(SupervisoryAppraiserCertificationNumber)
                }
                "State License Number" => Some(StateLicenseNumber),
                "Subject Property Verification Source" => {
                    Some(SubjectPropertyVerificationSource)
                }
                "Subject Property Reference Number" => {
                    Some(SubjectPropertyReferenceNumber)
                }
                "Subscriber Number" => Some(SubscriberNumber),
                "Reviewer File Number" => Some(ReviewerFileNumber),
                "Comparable Property Pending Sale Reference Number" => {
                    Some(ComparablePropertyPendingSaleReferenceNumber)
                }
                "Comparable Property Sale Reference Number" => {
                    Some(ComparablePropertySaleReferenceNumber)
                }
                "Subject Property Non-Sale Reference Number" => {
                    Some(SubjectPropertyNonSaleReferenceNumber)
                }
                "Policy Form Identifying Number" => Some(PolicyFormIdentifyingNumber),
                "Referenced By" => Some(ReferencedBy),
                "Mortgage Identification Number" => Some(MortgageIdentificationNumber),
                "Attached To" => Some(AttachedTo),
                "Real Estate Owned Property Identifier" => Some(RealEstateOwnedProperty),
                "American Bankers Assoc. (ABA) Transit/Routing Number (Including Check Digit, 9 Digits)" => {
                    Some(Code01)
                }
                "Blue Cross Provider Number" => Some(BlueCrossProviderNumber),
                "Catalog of Federal Domestic Assistance" => {
                    Some(CatalogOfFederalDomesticAssistance)
                }
                "Blue Shield Provider Number" => Some(BlueShieldProviderNumber),
                "Union Agreement" => Some(UnionAgreement),
                "Medicare Provider Number" => Some(MedicareProviderNumber),
                "Military Standard Requisitioning and Issue Procedures (MILSTRIP) Document Number" => {
                    Some(Code01C)
                }
                "Medicaid Provider Number" => Some(MedicaidProviderNumber),
                "Federal Standard Requisitioning and Issue Procedures (FEDSTRIP) Document Number" => {
                    Some(Code01D)
                }
                "Dentist License Number" => Some(DentistLicenseNumber),
                "Federal Supply Schedule Special (FSS) Item Number" => Some(Code01E),
                "Anesthesia License Number" => Some(AnesthesiaLicenseNumber),
                "Provider UPIN Number" => Some(ProviderUpinNumber),
                "Payment Related Clause" => Some(PaymentRelatedClause),
                "CHAMPUS Identification Number" => Some(ChampusIdentificationNumber),
                "Special Price Authorization Number" => {
                    Some(SpecialPriceAuthorizationNumber)
                }
                "Department of Defense Identification Code (DoDIC)" => Some(Code1I),
                "Facility ID Number" => Some(FacilityIdNumber),
                "Payor's Claim Number" => Some(PayorsClaimNumber),
                "Group or Policy Number" => Some(GroupOrPolicyNumber),
                "Preferred Provider Organization Site Number" => {
                    Some(PreferredProviderOrganizationSiteNumber)
                }
                "Diagnosis Related Group (DRG) Number" => Some(Code1N),
                "Consolidation Shipment Number" => Some(ConsolidationShipmentNumber),
                "Accessorial Status Code" => Some(AccessorialStatusCode),
                "Error Identification Code" => Some(ErrorIdentificationCode),
                "Storage Information Code" => Some(StorageInformationCode),
                "Ambulatory Patient Group (APG) Number" => Some(Code1S),
                "Resource Utilization Group (RUG) Number" => Some(Code1T),
                "Pay Grade" => Some(PayGrade),
                "Related Vendor Order Number" => Some(RelatedVendorOrderNumber),
                "Member Identification Number" => Some(MemberIdentificationNumber),
                "Credit or Debit Adjustment Number" => {
                    Some(CreditOrDebitAdjustmentNumber)
                }
                "Repair Action Number" => Some(RepairActionNumber),
                "Financial Detail Code" => Some(FinancialDetailCode),
                "Society for Worldwide Interbank Financial Telecommunication (S.W.I.F.T.) Identification (8 or 11 Characters)" => {
                    Some(Code02)
                }
                "Import License Number" => Some(ImportLicenseNumber),
                "Terminal Release Order Number" => Some(TerminalReleaseOrderNumber),
                "Long-term Disability Policy Number" => {
                    Some(LongTermDisabilityPolicyNumber)
                }
                "Aeronautical Equipment Reference Number (AERNO)" => Some(Code2D),
                "Foreign Military Sales Case Number" => {
                    Some(ForeignMilitarySalesCaseNumber)
                }
                "Consolidated Invoice Number" => Some(ConsolidatedInvoiceNumber),
                "Amendment" => Some(Amendment),
                "Assigned by transaction set sender" => {
                    Some(AssignedByTransactionSetSender)
                }
                "Tracking Number" => Some(TrackingNumber),
                "Floor Number" => Some(FloorNumber),
                "Food and Drug Administration (FDA) Product Type" => Some(Code2K),
                "Association of American Railroads (AAR) Railway Accounting Rules" => {
                    Some(Code2L)
                }
                "Federal Communications Commission (FCC) Identifier" => Some(Code2M),
                "Federal Communications Commission (FCC) Trade/Brand Identifier" => {
                    Some(Code2N)
                }
                "Occupational Safety and Health Administration (OSHA) Claim Number" => {
                    Some(Code2O)
                }
                "Subdivision Identifier" => Some(Subdivision),
                "Food and Drug Administration (FDA) Accession Number" => Some(Code2Q),
                "Coupon Redemption Number" => Some(CouponRedemptionNumber),
                "Catalog" => Some(Catalog),
                "Sub-subhouse Bill of Lading" => Some(SubSubhouseBillOfLading),
                "Payer Identification Number" => Some(PayerIdentificationNumber),
                "Special Government Accounting Classification Reference Number (ACRN)" => {
                    Some(Code2V)
                }
                "Change Order Authority" => Some(ChangeOrderAuthority),
                "Supplemental Agreement Authority" => {
                    Some(SupplementalAgreementAuthority)
                }
                "Wage Determination" => Some(WageDetermination),
                "U.S. Customs Service (USCS) Anti-dumping Duty Case Number" => {
                    Some(Code2Z)
                }
                "Clearing House Interbank Payment System (CHIPS) Participant Number (3 or 4 Digits)" => {
                    Some(Code03)
                }
                "Section of the National Housing Act Code" => {
                    Some(SectionOfTheNationalHousingActCode)
                }
                "Supplemental Claim Number" => Some(SupplementalClaimNumber),
                "Payee Loan Number" => Some(PayeeLoanNumber),
                "Servicer Loan Number" => Some(ServicerLoanNumber),
                "Investor Loan Number" => Some(InvestorLoanNumber),
                "Show Identification" => Some(ShowIdentification),
                "Catastrophe Number" => Some(CatastropheNumber),
                "Case Number" => Some(CaseNumber),
                "Precinct Number" => Some(PrecinctNumber),
                "Office Number" => Some(OfficeNumber),
                "Petroleum Pool Code" => Some(PetroleumPoolCode),
                "Branch Identifier" => Some(Branch),
                "Federal Communications Commission (FCC) Condition Code" => Some(Code3M),
                "Gas Custodian Identification" => Some(GasCustodianIdentification),
                "U.S. Customs Service (USCS) Pre-approval Ruling Number" => Some(Code3O),
                "Third Party Originator Number" => Some(ThirdPartyOriginatorNumber),
                "Food and Drug Administration (FDA) Product Code" => Some(Code3Q),
                "U.S. Customs Service (USCS) Binding Ruling Number" => Some(Code3R),
                "Provincial (Canadian) Sales Tax Exemption Number" => Some(Code3S),
                "U.S. Customs Service (USCS) Pre-classification Ruling Number" => {
                    Some(Code3T)
                }
                "Protraction Number" => Some(ProtractionNumber),
                "Formation Identifier" => Some(FormationIdentifier),
                "U.S. Customs Service (USCS) Commercial Description" => Some(Code3W),
                "Subcontract Number" => Some(SubcontractNumber),
                "Receiver Assigned Drop Zone" => Some(ReceiverAssignedDropZone),
                "Customs Broker Reference Number" => Some(CustomsBrokerReferenceNumber),
                "Canadian Financial Institution Branch and Institution Number" => {
                    Some(CanadianFinancialInstitutionBranchAndInstitutionNumber)
                }
                "Personal Identification Number (PIN)" => Some(Code4A),
                "Shipment Origin Code" => Some(ShipmentOriginCode),
                "Shipment Destination Code" => Some(ShipmentDestinationCode),
                "Shipping Zone" => Some(ShippingZone),
                "Carrier-assigned Consignee Number" => {
                    Some(CarrierAssignedConsigneeNumber)
                }
                "Carrier-assigned Shipper Number" => Some(CarrierAssignedShipperNumber),
                "Provincial Tax Identification" => Some(ProvincialTaxIdentification),
                "Commercial Invoice Number" => Some(CommercialInvoiceNumber),
                "Balance-due Reference Number" => Some(BalanceDueReferenceNumber),
                "Vehicle-related Services Reference Number" => {
                    Some(VehicleRelatedServicesReferenceNumber)
                }
                "Accessorial Rail Diversion Reference Number" => {
                    Some(AccessorialRailDiversionReferenceNumber)
                }
                "Location-specific Services Reference Number" => {
                    Some(LocationSpecificServicesReferenceNumber)
                }
                "Special Move Reference Number" => Some(SpecialMoveReferenceNumber),
                "Special Payment Reference Number" => Some(SpecialPaymentReferenceNumber),
                "Canadian Goods & Services or Quebec Sales Tax Reference Number" => {
                    Some(Code4O)
                }
                "Affiliation Number" => Some(AffiliationNumber),
                "Call Sign" => Some(CallSign),
                "Rule Section" => Some(RuleSection),
                "Preferred Call Sign" => Some(PreferredCallSign),
                "North American Datum Standard (NADS)" => Some(Code4T),
                "Market Area" => Some(MarketArea),
                "Emission Designator" => Some(EmissionDesignator),
                "Study" => Some(Study),
                "Log" => Some(Log),
                "Subhouse Bill of Lading" => Some(SubhouseBillOfLading),
                "U.S. Customs Service (USCS) Countervailing Duty Case Number" => {
                    Some(Code4Z)
                }
                "Clearing House Interbank Payment System (CHIPS) User Identification (6 digits)" => {
                    Some(Code05)
                }
                "Offense Tracking" => Some(OffenseTracking),
                "Supplemental Account Number" => Some(SupplementalAccountNumber),
                "Congressional District" => Some(CongressionalDistrict),
                "Line of Credit Category" => Some(LineOfCreditCategory),
                "Consumer Identifier" => Some(Consumer),
                "Warrant" => Some(Warrant),
                "Complaint" => Some(Complaint),
                "Incident" => Some(Incident),
                "Offender Tracking" => Some(OffenderTracking),
                "Driver's License" => Some(DriversLicense),
                "Commercial Driver's License" => Some(CommercialDriversLicense),
                "Jurisdictional Community Number" => Some(JurisdictionalCommunityNumber),
                "Previous Sequence" => Some(PreviousSequence),
                "Citation of Statute" => Some(CitationOfStatute),
                "Citation of Opinion" => Some(CitationOfOpinion),
                "National Criminal Information Center Originating Agency Identification" => {
                    Some(
                        NationalCriminalInformationCenterOriginatingAgencyIdentification,
                    )
                }
                "State Criminal History Repository Individual Identification" => {
                    Some(StateCriminalHistoryRepositoryIndividualIdentification)
                }
                "Federal Bureau of Investigation Individual Identification" => {
                    Some(FederalBureauOfInvestigationIndividualIdentification)
                }
                "Processing Area" => Some(ProcessingArea),
                "Payment Location" => Some(PaymentLocation),
                "Flood Data Identifier" => Some(FloodData),
                "Coupon Distribution Method" => Some(CouponDistributionMethod),
                "Original Uniform Commercial Code Filing Number" => {
                    Some(OriginalUniformCommercialCodeFilingNumber)
                }
                "Amended Uniform Commercial Code Filing Number" => {
                    Some(AmendedUniformCommercialCodeFilingNumber)
                }
                "Continuation Uniform Commercial Code Filing Number" => {
                    Some(ContinuationUniformCommercialCodeFilingNumber)
                }
                "Uniform Commercial Code Filing Collateral Number" => {
                    Some(UniformCommercialCodeFilingCollateralNumber)
                }
                "System Number" => Some(SystemNumber),
                "Consignee Reference Number" => Some(ConsigneeReferenceNumber),
                "U.S. Customs Service (USCS) Entry Number" => Some(Code6B),
                "U.S. Customs Service (USCS) Entry Type Code" => Some(Code6C),
                "U.S. Customs Service (USCS) Statement Number" => Some(Code6D),
                "Map Reference" => Some(MapReference),
                "Appraiser License" => Some(AppraiserLicense),
                "Map Number" => Some(MapNumber),
                "Comparable Property Verification Source" => {
                    Some(ComparablePropertyVerificationSource)
                }
                "Comparable Property" => Some(ComparableProperty),
                "Census Tract" => Some(CensusTract),
                "Zone" => Some(Zone),
                "Agent Contract Number" => Some(AgentContractNumber),
                "Application Number" => Some(ApplicationNumber),
                "Claimant Number" => Some(ClaimantNumber),
                "Cross Reference Number" => Some(CrossReferenceNumber),
                "Group Number" => Some(GroupNumber),
                "Insurance License Number" => Some(InsuranceLicenseNumber),
                "Provider Control Number" => Some(ProviderControlNumber),
                "Provider Order Ticket Number" => Some(ProviderOrderTicketNumber),
                "Pilot License Number" => Some(PilotLicenseNumber),
                "Question Number" => Some(QuestionNumber),
                "Reissue Cession Number" => Some(ReissueCessionNumber),
                "Specimen Identifier" => Some(Specimen),
                "Equipment Initial" => Some(EquipmentInitial),
                "Secretaria de Comercia y Famenta Industrial (SECOFI) Number" => {
                    Some(Code6Z)
                }
                "Add-On System Number" => Some(AddOnSystemNumber),
                "Purchase Order Number Included in On-Order Position" => {
                    Some(PurchaseOrderNumberIncludedInOnOrderPosition)
                }
                "Purchase Order Number of Shipment Received since Last Reporting Date" => {
                    Some(PurchaseOrderNumberOfShipmentReceivedSinceLastReportingDate)
                }
                "Purchase Order Number of Order Received since Last Reporting Date" => {
                    Some(PurchaseOrderNumberOfOrderReceivedSinceLastReportingDate)
                }
                "Tester Identification" => Some(TesterIdentification),
                "Collector Identification" => Some(CollectorIdentification),
                "Repeat Location" => Some(RepeatLocation),
                "Data Quality Reject Reason" => Some(DataQualityRejectReason),
                "Environmental Protection Agency (EPA) Test Type Purpose Code" => {
                    Some(Code7H)
                }
                "Subscriber Authorization Number" => Some(SubscriberAuthorizationNumber),
                "Toll Billing Telephone Reference Number" => {
                    Some(TollBillingTelephoneReferenceNumber)
                }
                "List of Materials" => Some(ListOfMaterials),
                "Qualified Materials List" => Some(QualifiedMaterialsList),
                "Frame" => Some(Frame),
                "Piggyback" => Some(Piggyback),
                "Tripleback" => Some(Tripleback),
                "Sheet" => Some(Sheet),
                "Engineering Change Order" => Some(EngineeringChangeOrder),
                "Representative Identification Number" => {
                    Some(RepresentativeIdentificationNumber)
                }
                "Drawing Type" => Some(DrawingType),
                "Master Contract" => Some(MasterContract),
                "Related Transaction Reference Number" => {
                    Some(RelatedTransactionReferenceNumber)
                }
                "Interchange Train Identification" => {
                    Some(InterchangeTrainIdentification)
                }
                "Home Mortgage Disclosure Act (HMDA) State Code" => Some(Code7X),
                "Home Mortgage Disclosure Act (HMDA) County Code" => Some(Code7Y),
                "Home Mortgage Disclosure Act (HMDA) Metropolitan Statistical Area (MSA)" => {
                    Some(Code7Z)
                }
                "Carrier Assigned Package Identification Number" => {
                    Some(CarrierAssignedPackageIdentificationNumber)
                }
                "Health Maintenance Organization (HMO) Authorization Number" => {
                    Some(Code8A)
                }
                "Preferred Provider Organization (PPO) Authorization Number" => {
                    Some(Code8B)
                }
                "Third-party Organization (TPO) Authorization Number" => Some(Code8C),
                "Chemical Abstract Service Registry Number" => {
                    Some(ChemicalAbstractServiceRegistryNumber)
                }
                "Guarantor Loan Number" => Some(GuarantorLoanNumber),
                "School Loan Number" => Some(SchoolLoanNumber),
                "Automated Clearinghouse (ACH) Trace Number" => Some(Code8G),
                "Check List Number" => Some(CheckListNumber),
                "FEDWIRE Confirmation Number" => Some(FedwireConfirmationNumber),
                "Society for Worldwide Interbank Financial Telecommunications (SWIFT) Confirmation Number" => {
                    Some(Code8J)
                }
                "Dominion of Canada Code" => Some(DominionOfCanadaCode),
                "International Standard Industry Classification Code (ISIC)" => {
                    Some(Code8L)
                }
                "Originating Company Identifier" => Some(OriginatingCompany),
                "Receiving Company Identifier" => Some(ReceivingCompany),
                "Automated Clearing House (ACH) Entry Description" => Some(Code8O),
                "Originating Depository Financial Institution Identifier" => {
                    Some(OriginatingDepositoryFinancialInstitution)
                }
                "Receiving Depository Financial Institution Identifier" => {
                    Some(ReceivingDepositoryFinancialInstitution)
                }
                "Security Type" => Some(SecurityType),
                "Broker Identification" => Some(BrokerIdentification),
                "Bank Assigned Security Identifier" => Some(BankAssignedSecurity),
                "Credit Reference" => Some(CreditReference),
                "Bank to Bank Information" => Some(BankToBank),
                "Transaction Category or Type" => Some(TransactionCategoryOrType),
                "Safekeeping Account Number" => Some(SafekeepingAccountNumber),
                "Alternate Clause Number" => Some(AlternateClauseNumber),
                "Customs Bar Code Number" => Some(CustomsBarCodeNumber),
                "Repriced Claim Reference Number" => Some(RepricedClaimReferenceNumber),
                "Repriced Line Item Reference Number" => {
                    Some(RepricedLineItemReferenceNumber)
                }
                "Adjusted Repriced Claim Reference Number" => {
                    Some(AdjustedRepricedClaimReferenceNumber)
                }
                "Adjusted Repriced Line Item Reference Number" => {
                    Some(AdjustedRepricedLineItemReferenceNumber)
                }
                "Replacement Claim Number" => Some(ReplacementClaimNumber),
                "Referral Number" => Some(ReferralNumber),
                "Department of Defense Form 250 Requirement Code" => {
                    Some(DepartmentOfDefenseForm250RequirementCode)
                }
                "Packaging Group Number" => Some(PackagingGroupNumber),
                "Automated Clearing House (ACH) Standard Entry Class" => Some(Code9I),
                "Pension Contract" => Some(PensionContract),
                "Servicer" => Some(Servicer),
                "Service Bureau" => Some(ServiceBureau),
                "Clearing House Interbank Payments System (CHIPS) Sequence Number" => {
                    Some(Code9M)
                }
                "Investor" => Some(Investor),
                "Loan Type" => Some(LoanType),
                "Pool Suffix" => Some(PoolSuffix),
                "Job Order Number" => Some(JobOrderNumber),
                "Delivery Region" => Some(DeliveryRegion),
                "Tenor" => Some(Tenor),
                "Loan Feature Code" => Some(LoanFeatureCode),
                "Payment Category" => Some(PaymentCategory),
                "Payer Category" => Some(PayerCategory),
                "Account Category" => Some(AccountCategory),
                "Bank Assigned Bankers Reference Number" => {
                    Some(BankAssignedBankersReferenceNumber)
                }
                "Chamber of Commerce Number" => Some(ChamberOfCommerceNumber),
                "Account Managers Code" => Some(AccountManagersCode),
                "Account Number" => Some(AccountNumber),
                "Billing Account" => Some(BillingAccount),
                "Horizontal Coordinate" => Some(HorizontalCoordinate),
                "Master Account Number" => Some(MasterAccountNumber),
                "Vertical Coordinate" => Some(VerticalCoordinate),
                "Military Interdepartmental Purchase Request (MIPR) Number" => {
                    Some(Code16)
                }
                "Client Reporting Category" => Some(ClientReportingCategory),
                "Plan Number" => Some(PlanNumber),
                "Division Identifier" => Some(Division),
                "Repair Part Number" => Some(RepairPartNumber),
                "American Gas Association Equation Number" => {
                    Some(AmericanGasAssociationEquationNumber)
                }
                "Special Charge or Allowance Code" => Some(SpecialChargeOrAllowanceCode),
                "Client Number" => Some(ClientNumber),
                "Short-term Disability Policy Number" => {
                    Some(ShortTermDisabilityPolicyNumber)
                }
                "Reason Not Lowest Cost Code" => Some(ReasonNotLowestCostCode),
                "Union Number" => Some(UnionNumber),
                "Insuror Pool Identification Number" => {
                    Some(InsurorPoolIdentificationNumber)
                }
                "Employee Identification Number" => Some(EmployeeIdentificationNumber),
                "Foreclosure Account Number" => Some(ForeclosureAccountNumber),
                "United States Government Visa Number" => {
                    Some(UnitedStatesGovernmentVisaNumber)
                }
                "Docket Number" => Some(DocketNumber),
                "Credit Repository Code" => Some(CreditRepositoryCode),
                "Lender Case Number" => Some(LenderCaseNumber),
                "Loan Request Number" => Some(LoanRequestNumber),
                "Multifamily Project Number" => Some(MultifamilyProjectNumber),
                "Underwriter Identification Number" => {
                    Some(UnderwriterIdentificationNumber)
                }
                "Condominium Identification Number" => {
                    Some(CondominiumIdentificationNumber)
                }
                "Master Policy Number" => Some(MasterPolicyNumber),
                "Proposal Number" => Some(ProposalNumber),
                "Lease Schedule Number - Replacement" => {
                    Some(LeaseScheduleNumberReplacement)
                }
                "Lease Schedule Number - Prior" => Some(LeaseScheduleNumberPrior),
                "Phone Calls" => Some(PhoneCalls),
                "Supporting Document Number" => Some(SupportingDocumentNumber),
                "End Use Number" => Some(EndUseNumber),
                "Old Account Number" => Some(OldAccountNumber),
                "Old Meter Number" => Some(OldMeterNumber),
                "Plate Number" => Some(PlateNumber),
                "Agency's Student Number" => Some(AgencysStudentNumber),
                "Family Unit Number" => Some(FamilyUnitNumber),
                "State Student Identification Number" => {
                    Some(StateStudentIdentificationNumber)
                }
                "Picture Number" => Some(PictureNumber),
                "SWIFT (MT 100)" => Some(Code52),
                "SWIFT (MT 202)" => Some(Code53),
                "FEDWIRE (Federal Wire Transfer)" => Some(Code54),
                "Sequence Number" => Some(SequenceNumber),
                "Corrected Social Security Number" => Some(CorrectedSocialSecurityNumber),
                "Prior Incorrect Social Security Number" => {
                    Some(PriorIncorrectSocialSecurityNumber)
                }
                "Corrected Batch Number" => Some(CorrectedBatchNumber),
                "Prior Incorrect Batch Number" => Some(PriorIncorrectBatchNumber),
                "Account Suffix Code" => Some(AccountSuffixCode),
                "Taxing Authority Identification Number" => {
                    Some(TaxingAuthorityIdentificationNumber)
                }
                "Prior Loan Number" => Some(PriorLoanNumber),
                "Jurisdictional Community Name Identifier" => {
                    Some(JurisdictionalCommunityName)
                }
                "Total Order Cycle Number" => Some(TotalOrderCycleNumber),
                "Previous Policy Number" => Some(PreviousPolicyNumber),
                "Previous Claim History Identifier" => Some(PreviousClaimHistory),
                "Dental Insurance Account Number" => Some(DentalInsuranceAccountNumber),
                "Dental Insurance Policy Number" => Some(DentalInsurancePolicyNumber),
                "Calendar Number" => Some(CalendarNumber),
                "(Working) Shift Number" => Some(Code71),
                "Schedule Reference Number" => Some(ScheduleReferenceNumber),
                "Statement of Work (SOW)" => Some(Code73),
                "Work Breakdown Structure (WBS)" => Some(Code74),
                "Organization Breakdown Structure" => {
                    Some(OrganizationBreakdownStructure)
                }
                "Milestone" => Some(Milestone),
                "Work Package" => Some(WorkPackage),
                "Planning Package" => Some(PlanningPackage),
                "Cost Account" => Some(CostAccount),
                "Charge Number" => Some(ChargeNumber),
                "Symbol Number (for Milestone or LOB reports)" => Some(Code81),
                "Data Item Description (DID) Reference" => Some(Code82),
                "Extended (or Exhibit) Line Item Number (ELIN)" => Some(Code83),
                "Contractor Data Requirements List (CDRL)" => Some(Code84),
                "Subcontractor Data Requirements (SDRL)" => Some(Code85),
                "Operation Number" => Some(OperationNumber),
                "Functional Category" => Some(FunctionalCategory),
                "Work Center" => Some(WorkCenter),
                "Assembly Number" => Some(AssemblyNumber),
                "Subassembly Number" => Some(SubassemblyNumber),
                "Cost Element" => Some(CostElement),
                "Change Document Number" => Some(ChangeDocumentNumber),
                "Funds Authorization" => Some(FundsAuthorization),
                "File Identification Number" => Some(FileIdentificationNumber),
                "Committee on Uniform Security Identification Procedures (CUSIP) Number" => {
                    Some(Code95)
                }
                "Stock Certificate Number" => Some(StockCertificateNumber),
                "Package Number" => Some(PackageNumber),
                "Container/Packaging Specification Number" => {
                    Some(ContainerPackagingSpecificationNumber)
                }
                "Rate Conference ID Code" => Some(RateConferenceIdCode),
                "Advertiser Number" => Some(AdvertiserNumber),
                "Analysis number/Test number" => Some(AnalysisNumberTestNumber),
                "Disability Insurance Account Number" => {
                    Some(DisabilityInsuranceAccountNumber)
                }
                "Assignment Number" => Some(AssignmentNumber),
                "Disability Insurance Policy Number" => {
                    Some(DisabilityInsurancePolicyNumber)
                }
                "Educational Institution Identification Number" => {
                    Some(EducationalInstitutionIdentificationNumber)
                }
                "Flexible Spending Account (FSA) Insurance Account Number" => {
                    Some(CodeA7)
                }
                "Flexible Spending Account (FSA) Insurance Policy Number" => Some(CodeA8),
                "Health Insurance Account Number" => Some(HealthInsuranceAccountNumber),
                "Accounts Receivable Statement Number" => {
                    Some(AccountsReceivableStatementNumber)
                }
                "Distributor's Split Agent Number" => Some(DistributorsSplitAgentNumber),
                "Fund Manager's Reference Number" => Some(FundManagersReferenceNumber),
                "Agency Hierarchical Level" => Some(AgencyHierarchicalLevel),
                "Officer License Number" => Some(OfficerLicenseNumber),
                "Previous Distributor Number" => Some(PreviousDistributorNumber),
                "Interviewer ID" => Some(InterviewerId),
                "Military ID" => Some(MilitaryId),
                "Option Policy Number" => Some(OptionPolicyNumber),
                "Payroll Account Number" => Some(PayrollAccountNumber),
                "Prior Contract Number" => Some(PriorContractNumber),
                "Worksite Number" => Some(WorksiteNumber),
                "Agent Number" => Some(AgentNumber),
                "Treaty Identifier" => Some(Treaty),
                "Associated Case Control Number" => Some(AssociatedCaseControlNumber),
                "Carrier Assigned Code" => Some(CarrierAssignedCode),
                "Dealer Number" => Some(DealerNumber),
                "Directory Number" => Some(DirectoryNumber),
                "Distributor Assigned Transaction Number" => {
                    Some(DistributorAssignedTransactionNumber)
                }
                "Distributor Assigned Order Number" => {
                    Some(DistributorAssignedOrderNumber)
                }
                "Distributor's Account Number" => Some(DistributorsAccountNumber),
                "General Agency Number" => Some(GeneralAgencyNumber),
                "Laboratory Number" => Some(LaboratoryNumber),
                "Agency Assigned Number" => Some(AgencyAssignedNumber),
                "List Bill Number" => Some(ListBillNumber),
                "Accounting Period Reference" => Some(AccountingPeriodReference),
                "Paramedical ID Number" => Some(ParamedicalIdNumber),
                "Acceptable Source Purchaser ID" => Some(AcceptableSourcePurchaserId),
                "Payroll Number" => Some(PayrollNumber),
                "Personal ID Number" => Some(PersonalIdNumber),
                "Policy Link Number" => Some(PolicyLinkNumber),
                "Secondary Policy Number" => Some(SecondaryPolicyNumber),
                "Special Quote Number" => Some(SpecialQuoteNumber),
                "National Property Registry System Level 1" => {
                    Some(NationalPropertyRegistrySystemLevel1)
                }
                "National Property Registry System Level 2" => {
                    Some(NationalPropertyRegistrySystemLevel2)
                }
                "Investor Assigned Identification Number" => {
                    Some(InvestorAssignedIdentificationNumber)
                }
                "Motor Fuel Certificate Number" => Some(MotorFuelCertificateNumber),
                "Ginnie Mae (Government National Mortgage Association) Pool Package Number" => {
                    Some(CodeABJ)
                }
                "Mortgage Electronic Registration System Organization Identifier" => {
                    Some(MortgageElectronicRegistrationSystemOrganization)
                }
                "Seller Loan Number" => Some(SellerLoanNumber),
                "Sub-Servicer Loan Number" => Some(SubServicerLoanNumber),
                "National Property Registry System Level 3" => {
                    Some(NationalPropertyRegistrySystemLevel3)
                }
                "State Hazardous Waste Entity Identifier" => {
                    Some(StateHazardousWasteEntity)
                }
                "Bankruptcy Procedure Number" => Some(BankruptcyProcedureNumber),
                "National Business Identification Number" => {
                    Some(NationalBusinessIdentificationNumber)
                }
                "Prior Data Universal Number System (D-U-N-S) Number, Dun & Bradstreet" => {
                    Some(CodeABR)
                }
                "Vessel Name" => Some(VesselName),
                "Security Instrument Number" => Some(SecurityInstrumentNumber),
                "Assignment Recording Number" => Some(AssignmentRecordingNumber),
                "Book Number" => Some(BookNumber),
                "Business Tax Number" => Some(BusinessTaxNumber),
                "North American Industrial Classification System Code-2" => {
                    Some(NorthAmericanIndustrialClassificationSystemCode2)
                }
                "Centers for Medicare and Medicaid Services PlanID" => {
                    Some(CentersForMedicareAndMedicaidServicesPlanId)
                }
                "Employment Visa" => Some(EmploymentVisa),
                "Air Cargo Transfer Manifest" => Some(AirCargoTransferManifest),
                "Growth Factor Reference" => Some(GrowthFactorReference),
                "Region" => Some(Region),
                "Status" => Some(Status),
                "Class Code" => Some(ClassCode),
                "Service Request Number" => Some(ServiceRequestNumber),
                "Supplement Number" => Some(SupplementNumber),
                "Previous Ticket Number" => Some(PreviousTicketNumber),
                "One Call Agency Ticket Number" => Some(OneCallAgencyTicketNumber),
                "Ticket Number" => Some(TicketNumber),
                "Bill of Material Revision Number" => Some(BillOfMaterialRevisionNumber),
                "Drawing Revision Number" => Some(DrawingRevisionNumber),
                "Application Transaction Reference Number" => {
                    Some(ApplicationTransactionReferenceNumber)
                }
                "Related Object Identification Number" => {
                    Some(RelatedObjectIdentificationNumber)
                }
                "Common Access Reference Number" => Some(CommonAccessReferenceNumber),
                "First Transfer Number" => Some(FirstTransferNumber),
                "Continuous Transfer Number" => Some(ContinuousTransferNumber),
                "Last Transfer Number" => Some(LastTransferNumber),
                "Automated Clearinghouse (ACH) Return/Notification of Change (NOC) Code" => {
                    Some(CodeACR)
                }
                "Society of Property Information Compilers and Analysts" => {
                    Some(SocietyOfPropertyInformationCompilersAndAnalysts)
                }
                "Accounting Code" => Some(AccountingCode),
                "Green Card" => Some(GreenCard),
                "Agency Assigned Employee ID" => Some(AgencyAssignedEmployeeId),
                "Passport" => Some(Passport),
                "Unemployment Insurance Number" => Some(UnemploymentInsuranceNumber),
                "North American Industrial Classification System Code-1" => {
                    Some(NorthAmericanIndustrialClassificationSystemCode1)
                }
                "Occupation Code" => Some(OccupationCode),
                "Acceptable Source DUNS Number" => Some(AcceptableSourceDunsNumber),
                "Agency for International Development Acquisition Regulation (AIDAR)" => {
                    Some(CodeADA)
                }
                "Master Property Number" => Some(MasterPropertyNumber),
                "Project Property Number" => Some(ProjectPropertyNumber),
                "Unit Property Number" => Some(UnitPropertyNumber),
                "Associated Property Number" => Some(AssociatedPropertyNumber),
                "Associated Number For Limited Common Element Parking" => {
                    Some(AssociatedNumberForLimitedCommonElementParking)
                }
                "Associated Number For Unit Parking" => {
                    Some(AssociatedNumberForUnitParking)
                }
                "Associated Number For Joined Unit not re-subdivided" => {
                    Some(AssociatedNumberForJoinedUnitNotReSubdivided)
                }
                "Processor Identification Number" => Some(ProcessorIdentificationNumber),
                "Occupation Classification Code" => Some(OccupationClassificationCode),
                "Employee Tax Filing Status Code" => Some(EmployeeTaxFilingStatusCode),
                "Insured Location Identifier" => Some(InsuredLocation),
                "Air Dimension Code" => Some(AirDimensionCode),
                "Self Insurance Identification Number" => {
                    Some(SelfInsuranceIdentificationNumber)
                }
                "Self Insurer Organization Type" => Some(SelfInsurerOrganizationType),
                "Self Insurer Authorization Type Code" => {
                    Some(SelfInsurerAuthorizationTypeCode)
                }
                "County Business Registration Number" => {
                    Some(CountyBusinessRegistrationNumber)
                }
                "Postal Template Identifier" => Some(PostalTemplate),
                "Reduced Earning Week Identifier" => Some(ReducedEarningWeek),
                "Full Denial Reason Identifier" => Some(FullDenialReason),
                "Federal Energy Regulatory Commission Certificate of Public Convenience" => {
                    Some(FederalEnergyRegulatoryCommissionCertificateOfPublicConvenience)
                }
                "Suspension Identifier" => Some(Suspension),
                "Managed Care Organization Code" => Some(ManagedCareOrganizationCode),
                "Managed Care Organization Identification Number" => {
                    Some(ManagedCareOrganizationIdentificationNumber)
                }
                "Public Utilities Commission Certificate of Public Convenience" => {
                    Some(PublicUtilitiesCommissionCertificateOfPublicConvenience)
                }
                "Retail Merchant's Certification Number" => {
                    Some(RetailMerchantsCertificationNumber)
                }
                "Authorization for Expense (AFE) Number" => Some(CodeAE),
                "Numero de Cedula de Identidad (CIN) Number" => Some(CodeAEA),
                "Company's Registry Office (CRO) Number" => Some(CodeAEB),
                "Government Registration Number" => Some(GovernmentRegistrationNumber),
                "Judicial Number" => Some(JudicialNumber),
                "Numero de Identificacion Tributaria (NIT)" => Some(CodeAEE),
                "Passport Number" => Some(PassportNumber),
                "Patron Number" => Some(PatronNumber),
                "Registro Informacion Fiscal (RIF)" => Some(CodeAEH),
                "Registro Unico de Contribuyente (RUC)" => Some(CodeAEI),
                "Superintendencia de Inversiones Extranjeras (SIEX) Number" => {
                    Some(CodeAEJ)
                }
                "Tokyo Shoko Research Business Identifier" => {
                    Some(TokyoShokoResearchBusiness)
                }
                "Registro Nacional de Contribuyente (RNC)" => Some(CodeAEL),
                "Distribution Center Number" => Some(DistributionCenterNumber),
                "Institute of Security and Future Market Development (ISFMD) Serial Number" => {
                    Some(CodeAEN)
                }
                "Public Deed Number" => Some(PublicDeedNumber),
                "Stock Exchange Code" => Some(StockExchangeCode),
                "Secretary of State Assigned Identification Number" => {
                    Some(SecretaryOfStateAssignedIdentificationNumber)
                }
                "Department Where Injury Occurred Identification" => {
                    Some(DepartmentWhereInjuryOccurredIdentification)
                }
                "Bureau of Labor and Statistics Schedule Identifier" => {
                    Some(BureauOfLaborAndStatisticsSchedule)
                }
                "State Charter Number" => Some(StateCharterNumber),
                "Employee/Non-Employee Classification Qualifier" => {
                    Some(EmployeeNonEmployeeClassificationQualifier)
                }
                "Full Time/Part Time Employee Classification Qualifier" => {
                    Some(FullTimePartTimeEmployeeClassificationQualifier)
                }
                "Premium Audit Priority Identifier" => Some(PremiumAuditPriority),
                "Premium Audit Purpose Identifier" => Some(PremiumAuditPurpose),
                "Premium Audit Type Identifier" => Some(PremiumAuditType),
                "Airlines Flight Identification Number" => {
                    Some(AirlinesFlightIdentificationNumber)
                }
                "Split Premium Audit Change Identifier" => Some(SplitPremiumAuditChange),
                "Subline of Insurance" => Some(SublineOfInsurance),
                "Verification Source Code" => Some(VerificationSourceCode),
                "Underwriting Alert Reference Code" => {
                    Some(UnderwritingAlertReferenceCode)
                }
                "Commercial/Private Passenger Vehicle Qualifier" => {
                    Some(CommercialPrivatePassengerVehicleQualifier)
                }
                "Vehicle Business Use Qualifier" => Some(VehicleBusinessUseQualifier),
                "Vehicle Size Class Qualifier" => Some(VehicleSizeClassQualifier),
                "Vehicle Radius of Operation Qualifier" => {
                    Some(VehicleRadiusOfOperationQualifier)
                }
                "Trailer Type Qualifier" => Some(TrailerTypeQualifier),
                "State Sales Tax Identification Number" => {
                    Some(StateSalesTaxIdentificationNumber)
                }
                "Card Issuer Transaction Code" => Some(CardIssuerTransactionCode),
                "Card Billing Type Code" => Some(CardBillingTypeCode),
                "Client Company Code" => Some(ClientCompanyCode),
                "Merchant Category Code (MCC)" => Some(CodeAFN),
                "Card Account Type Code" => Some(CardAccountTypeCode),
                "Card Account Status Code" => Some(CardAccountStatusCode),
                "Card Account Reporting Level" => Some(CardAccountReportingLevel),
                "Card Account Reporting Identifier" => Some(CardAccountReporting),
                "American Osteopathic Association (AOA) Certification Number" => {
                    Some(CodeAFS)
                }
                "Fee Schedule Identifier" => Some(FeeSchedule),
                "United States Standard Metropolitan Statistical Area (MSA) Code" => {
                    Some(CodeAFU)
                }
                "State Controlled Substance License Number" => {
                    Some(StateControlledSubstanceLicenseNumber)
                }
                "Point of Origination" => Some(PointOfOrigination),
                "Point of Destination" => Some(PointOfDestination),
                "Assessment Number" => Some(AssessmentNumber),
                "Certificate Number" => Some(CertificateNumber),
                "Agent's Shipment Number" => Some(AgentsShipmentNumber),
                "State or Province Assigned Business Registry Number" => {
                    Some(StateOrProvinceAssignedBusinessRegistryNumber)
                }
                "Municipality Assigned Business Registry Number" => {
                    Some(MunicipalityAssignedBusinessRegistryNumber)
                }
                "Clave Unica de Identificacion Tributaria (CUIT)" => Some(CodeAGC),
                "Registro Unico Tributario (RUT)" => Some(CodeAGD),
                "Lender Use" => Some(LenderUse),
                "Guarantor Use" => Some(GuarantorUse),
                "School Use" => Some(SchoolUse),
                "Reservation System Code" => Some(ReservationSystemCode),
                "Order Origination Code" => Some(OrderOriginationCode),
                "Folio Number" => Some(FolioNumber),
                "Corporate Identification Code" => Some(CorporateIdentificationCode),
                "Cadastro Geral do Contribuinte (CGC)" => Some(CodeAGO),
                "Conjunction Travel Ticket" => Some(ConjunctionTravelTicket),
                "List Tracking Identifier" => Some(ListTracking),
                "Agreement Number" => Some(AgreementNumber),
                "Air Handling Code" => Some(AirHandlingCode),
                "Associated Invoices" => Some(AssociatedInvoices),
                "Accounts Receivable Customer Account" => {
                    Some(AccountsReceivableCustomerAccount)
                }
                "Sending Company Audit Number (Automated Clearinghouse Transfers)" => {
                    Some(CodeAK)
                }
                "Accounting (Equipment) Location Number" => Some(CodeAL),
                "Agency Location Code" => Some(AgencyLocationCode),
                "Title Company Code Book Reference" => {
                    Some(TitleCompanyCodeBookReference)
                }
                "Title Document Schedule" => Some(TitleDocumentSchedule),
                "Recording Number" => Some(RecordingNumber),
                "Title Policy Number" => Some(TitlePolicyNumber),
                "Alien Registration Number" => Some(AlienRegistrationNumber),
                "Alternative List ID" => Some(AlternativeListId),
                "Alteration Number" => Some(AlterationNumber),
                "Adjustment Memo (Charge Back)" => Some(CodeAM),
                "Associated Purchase Orders" => Some(AssociatedPurchaseOrders),
                "Appointment Number" => Some(AppointmentNumber),
                "Accounts Receivable Number" => Some(AccountsReceivableNumber),
                "Ambulatory Payment Classification" => {
                    Some(AmbulatoryPaymentClassification)
                }
                "American Petroleum Institute (API) Deduction Code" => Some(CodeAPI),
                "Access Code" => Some(AccessCode),
                "Arrival Code" => Some(ArrivalCode),
                "Acceptable Source Supplier ID" => Some(AcceptableSourceSupplierId),
                "Atomic Safety and Licensing Board Panel (ASLBP) Number" => Some(CodeASL),
                "Animal Species" => Some(AnimalSpecies),
                "Animal Strain" => Some(AnimalStrain),
                "Appropriation Number" => Some(AppropriationNumber),
                "Maintenance Availability Type" => Some(MaintenanceAvailabilityType),
                "Authorization to Meet Competition Number" => {
                    Some(AuthorizationToMeetCompetitionNumber)
                }
                "Health Insurance Rating Account Number" => {
                    Some(HealthInsuranceRatingAccountNumber)
                }
                "Air Waybill Number" => Some(AirWaybillNumber),
                "Government Accounting Class Reference Number (ACRN)" => Some(CodeAX),
                "Floor Plan Approval Number" => Some(FloorPlanApprovalNumber),
                "Health Insurance Policy Number" => Some(HealthInsurancePolicyNumber),
                "Lessee Bill Code Number" => Some(LesseeBillCodeNumber),
                "Axle Ratio" => Some(AxleRatio),
                "Preferred Provider Organization Number" => {
                    Some(PreferredProviderOrganizationNumber)
                }
                "Bilateral Car Service Agreements" => Some(BilateralCarServiceAgreements),
                "Health Insurance Rating Suffix Code" => {
                    Some(HealthInsuranceRatingSuffixCode)
                }
                "Life Insurance Billing Account Number" => {
                    Some(LifeInsuranceBillingAccountNumber)
                }
                "Life Insurance Policy Number" => Some(LifeInsurancePolicyNumber),
                "Life Insurance Billing Suffix Code" => {
                    Some(LifeInsuranceBillingSuffixCode)
                }
                "Retirement Plan Account Number" => Some(RetirementPlanAccountNumber),
                "Retirement Plan Policy Number" => Some(RetirementPlanPolicyNumber),
                "Franchise Tax Account Number" => Some(FranchiseTaxAccountNumber),
                "Certificate of Incorporation Number" => {
                    Some(CertificateOfIncorporationNumber)
                }
                "Beam Assembly Code" => Some(BeamAssemblyCode),
                "State Tax Identification Number" => Some(StateTaxIdentificationNumber),
                "Charter Number" => Some(CharterNumber),
                "Receipt Number" => Some(ReceiptNumber),
                "Withdrawal Account Number" => Some(WithdrawalAccountNumber),
                "Deposit Account Number" => Some(DepositAccountNumber),
                "Business Identification Number" => Some(BusinessIdentificationNumber),
                "United States Postal Service (USPS) PLANET (PostaL AlphaNumEric coding Technique) Code" => {
                    Some(CodeBAJ)
                }
                "Address Correction Service (ACS) Participation Code" => Some(CodeBAK),
                "Authorization Number" => Some(AuthorizationNumber),
                "Buyer's Contract Number" => Some(BuyersContractNumber),
                "Basic Contract Line Item Number" => Some(BasicContractLineItemNumber),
                "Birth Certificate Number" => Some(BirthCertificateNumber),
                "Border Crossing Permit Number" => Some(BorderCrossingPermitNumber),
                "Bid Number" => Some(BidNumber),
                "Badge Number" => Some(BadgeNumber),
                "Build Directive Number" => Some(BuildDirectiveNumber),
                "Business Activity" => Some(BusinessActivity),
                "Broker Entry Number" => Some(BrokerEntryNumber),
                "Billing Center Identification" => Some(BillingCenterIdentification),
                "Beginning Serial Number" => Some(BeginningSerialNumber),
                "Lease Schedule Number - Blanket" => Some(LeaseScheduleNumberBlanket),
                "Bonded Carrier Internal Revenue Service Identification Number" => {
                    Some(BondedCarrierInternalRevenueServiceIdentificationNumber)
                }
                "Carrier's Customs Bond Number" => Some(CarriersCustomsBondNumber),
                "Broker's Order Number" => Some(BrokersOrderNumber),
                "Bank Telegraphic Number" => Some(BankTelegraphicNumber),
                "Government Bill of Lading" => Some(GovernmentBillOfLading),
                "Billing Type" => Some(BillingType),
                "Bill of Lading Number" => Some(BillOfLadingNumber),
                "Begin Mile Marker" => Some(BeginMileMarker),
                "Booking Number" => Some(BookingNumber),
                "Bin Location Number" => Some(BinLocationNumber),
                "Binary Object Identifier" => Some(BinaryObject),
                "Adjustment Control Number" => Some(AdjustmentControlNumber),
                "Health Maintenance Organization Code Number" => {
                    Some(HealthMaintenanceOrganizationCodeNumber)
                }
                "Broker or Sales Office Number" => Some(BrokerOrSalesOfficeNumber),
                "Split Booking Number" => Some(SplitBookingNumber),
                "Batch Number" => Some(BatchNumber),
                "Buyer's Approval Mark" => Some(BuyersApprovalMark),
                "Purchase Order Line Item Identifier (Buyer)" => Some(CodeBV),
                "Blended With Batch Number" => Some(BlendedWithBatchNumber),
                "Buyer's Shipment Mark Number" => Some(BuyersShipmentMarkNumber),
                "Repair Category Number" => Some(RepairCategoryNumber),
                "Complaint Code" => Some(ComplaintCode),
                "Canadian Social Insurance Number" => Some(CanadianSocialInsuranceNumber),
                "Customer material specification number" => {
                    Some(CustomerMaterialSpecificationNumber)
                }
                "Customer process specification number" => {
                    Some(CustomerProcessSpecificationNumber)
                }
                "Customer specification number" => Some(CustomerSpecificationNumber),
                "Change Number" => Some(ChangeNumber),
                "Customer Tracking Number For Loaned Materials" => {
                    Some(CustomerTrackingNumberForLoanedMaterials)
                }
                "Carnet Number" => Some(CarnetNumber),
                "Contract Line Item Number" => Some(ContractLineItemNumber),
                "Corrected Contract Number" => Some(CorrectedContractNumber),
                "Previous Credit/Debit Adjustment Number" => {
                    Some(PreviousCreditDebitAdjustmentNumber)
                }
                "Cost Allocation Reference" => Some(CostAllocationReference),
                "Accident History Identifier" => Some(AccidentHistory),
                "Chemical Identifier" => Some(Chemical),
                "Discharge Point Identification" => Some(DischargePointIdentification),
                "Emission Unit Identification Number" => {
                    Some(EmissionUnitIdentificationNumber)
                }
                "Facility Federal Identification Number" => {
                    Some(FacilityFederalIdentificationNumber)
                }
                "Latitude Expressed in Decimal Degrees" => {
                    Some(LatitudeExpressedInDecimalDegrees)
                }
                "Longitude Expressed in Decimal Degrees" => {
                    Some(LongitudeExpressedInDecimalDegrees)
                }
                "Office of Regulatory Information Systems (ORIS) Code" => Some(CodeCAH),
                "Process Identifier" => Some(Process),
                "Stack Identification Number" => Some(StackIdentificationNumber),
                "Facility State Identification Number" => {
                    Some(FacilityStateIdentificationNumber)
                }
                "U.S. Environmental Protection Agency (EPA) Hazardous Waste Code" => {
                    Some(CodeCAL)
                }
                "U.S. Environmental Protection Agency (EPA)\nIdentification Number" => {
                    Some(CodeCAM)
                }
                "Category Identifier" => Some(Category),
                "Combined Shipment" => Some(CombinedShipment),
                "Census Block Group" => Some(CensusBlockGroup),
                "Contract Co-op Number" => Some(ContractCoOpNumber),
                "Credit Note Number" => Some(CreditNoteNumber),
                "Citizenship Document Number" => Some(CitizenshipDocumentNumber),
                "Contracting District Type Code" => Some(ContractingDistrictTypeCode),
                "Class of Contract Code" => Some(ClassOfContractCode),
                "Fleet Reference Number" => Some(FleetReferenceNumber),
                "Federal Regulation" => Some(FederalRegulation),
                "Consignee's Order Number" => Some(ConsigneesOrderNumber),
                "Customer Catalog Number" => Some(CustomerCatalogNumber),
                "Chromatograph Identifier" => Some(Chromatograph),
                "Unique Consignment Identifier" => Some(UniqueConsignment),
                "Campus Identification Number" => Some(CampusIdentificationNumber),
                "Circuit Number" => Some(CircuitNumber),
                "Citation" => Some(Citation),
                "Clause Number" => Some(ClauseNumber),
                "Check Number" => Some(CheckNumber),
                "Seller's Credit Memo" => Some(SellersCreditMemo),
                "Coverage List ID" => Some(CoverageListId),
                "Buyer's Credit Memo" => Some(BuyersCreditMemo),
                "Continuous Move Number" => Some(ContinuousMoveNumber),
                "Customer Maintenance Period Sequence Number" => {
                    Some(CustomerMaintenancePeriodSequenceNumber)
                }
                "Component" => Some(Component),
                "Carrier's Reference Number (PRO/Invoice)" => Some(CodeCN),
                "Assembly Control Number" => Some(AssemblyControlNumber),
                "Commitment Number" => Some(CommitmentNumber),
                "Canadian National Student Number" => Some(CanadianNationalStudentNumber),
                "Customer Order Number" => Some(CustomerOrderNumber),
                "Collocation Indicator" => Some(CollocationIndicator),
                "Certificate of Transportation" => Some(CertificateOfTransportation),
                "Condition of Purchase Document Number" => {
                    Some(ConditionOfPurchaseDocumentNumber)
                }
                "Canadian Province Operating Authority Number" => {
                    Some(CanadianProvinceOperatingAuthorityNumber)
                }
                "Discrepant Container Packaging Number" => {
                    Some(DiscrepantContainerPackagingNumber)
                }
                "Required Container Packaging Number" => {
                    Some(RequiredContainerPackagingNumber)
                }
                "Current Procedural Terminology Code" => {
                    Some(CurrentProceduralTerminologyCode)
                }
                "Customshouse Broker License Number" => {
                    Some(CustomshouseBrokerLicenseNumber)
                }
                "Customer Reference Number" => Some(CustomerReferenceNumber),
                "Casualty Report Number" => Some(CasualtyReportNumber),
                "Casualty Report Serial Number" => Some(CasualtyReportSerialNumber),
                "Condition of Sale Document Number" => {
                    Some(ConditionOfSaleDocumentNumber)
                }
                "CS54 Key Train Indicator Code" => Some(Cs54KeyTrainIndicatorCode),
                "CS54 Key Train Indicator Group Name" => {
                    Some(Cs54KeyTrainIndicatorGroupName)
                }
                "Census State Code" => Some(CensusStateCode),
                "Contract Number" => Some(ContractNumber),
                "Census Tract Suffix" => Some(CensusTractSuffix),
                "Clear Text Clause" => Some(ClearTextClause),
                "U.S. Customs Service (USCS) Bill of Lading Number" => Some(CodeCUB),
                "Coil Number" => Some(CoilNumber),
                "Commercial Vehicle Safety Assurance Number" => {
                    Some(CommercialVehicleSafetyAssuranceNumber)
                }
                "Canadian Wheat Board Permit Number" => {
                    Some(CanadianWheatBoardPermitNumber)
                }
                "Consignment Classification ID" => Some(ConsignmentClassificationId),
                "Commercial Registration Number" => Some(CommercialRegistrationNumber),
                "Periodicity Code" => Some(PeriodicityCode),
                "Contract Rider Number (Used in conjunction with contract number)" => {
                    Some(CodeCZ)
                }
                "Data Reliability Code" => Some(DataReliabilityCode),
                "Drug Enforcement Administration Order Blank Number" => {
                    Some(DrugEnforcementAdministrationOrderBlankNumber)
                }
                "Supplier Document Identification Number" => {
                    Some(SupplierDocumentIdentificationNumber)
                }
                "National Council for Prescription Drug Programs Pharmacy Number" => {
                    Some(NationalCouncilForPrescriptionDrugProgramsPharmacyNumber)
                }
                "Cut Number" => Some(CutNumber),
                "Dye Lot Number" => Some(DyeLotNumber),
                "Duplicate Bill Number" => Some(DuplicateBillNumber),
                "Coverage Code" => Some(CoverageCode),
                "Loss Report Number" => Some(LossReportNumber),
                "Claim Number" => Some(ClaimNumber),
                "Domicile Branch Number" => Some(DomicileBranchNumber),
                "District Assigned ID" => Some(DistrictAssignedId),
                "Delivery Appointment Number" => Some(DeliveryAppointmentNumber),
                "Buyer's Debit Memo" => Some(BuyersDebitMemo),
                "Dealer purchase order number" => Some(DealerPurchaseOrderNumber),
                "Document Identification Code" => Some(DocumentIdentificationCode),
                "Depositor Number" => Some(DepositorNumber),
                "Defense Federal Acquisition Regulations (DFAR)" => Some(CodeDF),
                "Drawing Number" => Some(DrawingNumber),
                "Drug Enforcement Administration Number" => {
                    Some(DrugEnforcementAdministrationNumber)
                }
                "Department of Health and Human Services Acquisition Regulation (HHSAR)" => {
                    Some(CodeDHH)
                }
                "Distributor Invoice Number" => Some(DistributorInvoiceNumber),
                "District Number" => Some(DistrictNumber),
                "Delivery Ticket Number" => Some(DeliveryTicketNumber),
                "Dock Number" => Some(DockNumber),
                "Seller's Debit Memo" => Some(SellersDebitMemo),
                "Associated Product Number" => Some(AssociatedProductNumber),
                "Draft Number" => Some(DraftNumber),
                "Deposit Number" => Some(DepositNumber),
                "D-U-N-S+4, D-U-N-S Number with Four Character Suffix" => Some(CodeDNS),
                "Delivery Order Number" => Some(DeliveryOrderNumber),
                "Department of Agriculture Acquisition Regulation (AGAR)" => {
                    Some(CodeDOA)
                }
                "Department of Commerce Acquisition Regulation (CAR)" => Some(CodeDOC),
                "Department of Energy Acquisition Regulation (DEAR)" => Some(CodeDOE),
                "Department of Interior Acquisition Regulation (DIAR)" => Some(CodeDOI),
                "Department of Justice Acquisition Regulation (JAR)" => Some(CodeDOJ),
                "Department of Labor Acquisition Regulation (DOLAR)" => Some(CodeDOL),
                "Density Order Number" => Some(DensityOrderNumber),
                "Department of State Acquisition Regulation (DOSAR)" => Some(CodeDOS),
                "Department of Transportation Acquisition Regulation (TAR)" => {
                    Some(CodeDOT)
                }
                "Department Number" => Some(DepartmentNumber),
                "Delivery Quote Number" => Some(DeliveryQuoteNumber),
                "Dock Receipt Number" => Some(DockReceiptNumber),
                "Drainhole Number" => Some(DrainholeNumber),
                "Defense Priorities Allocation System (DPAS) Priority Rating" => {
                    Some(CodeDS)
                }
                "Departure from Specification Class Code" => {
                    Some(DepartureFromSpecificationClassCode)
                }
                "Departure from Specification Number" => {
                    Some(DepartureFromSpecificationNumber)
                }
                "Departure from Specification Type Code" => {
                    Some(DepartureFromSpecificationTypeCode)
                }
                "Downstream Shipper Contract Number" => {
                    Some(DownstreamShipperContractNumber)
                }
                "Department of the Treasury Acquisition/Procurement Regulation (TAPR)" => {
                    Some(CodeDTS)
                }
                "Dependents Information" => Some(Dependents),
                "D-U-N-S Number Dun & Bradstreet" => Some(CodeDUN),
                "Diversion Authority Number" => Some(DiversionAuthorityNumber),
                "Deposit Sequence Number" => Some(DepositSequenceNumber),
                "Department/Agency Number" => Some(DepartmentAgencyNumber),
                "Department of Defense Transportation Service Code Number (Household Goods)" => {
                    Some(CodeDY)
                }
                "Certified Registered Nurse Anesthetist (CRNA) Provider Identification Number" => {
                    Some(CodeDZ)
                }
                "Course Section Number" => Some(CourseSectionNumber),
                "Emergency Order Number" => Some(EmergencyOrderNumber),
                "Non-Teaching Credential Field Codes" => {
                    Some(NonTeachingCredentialFieldCodes)
                }
                "Part Causing Repair Number" => Some(PartCausingRepairNumber),
                "Classification of Instructional Programs (CIP) Codes" => Some(CodeE02),
                "Expansion on Effect of Change Number" => {
                    Some(ExpansionOnEffectOfChangeNumber)
                }
                "Charge Card Number" => Some(ChargeCardNumber),
                "Claimant's Claim Number" => Some(ClaimantsClaimNumber),
                "Backout Procedure Code" => Some(BackoutProcedureCode),
                "Service Bulletin Number" => Some(ServiceBulletinNumber),
                "Service Contract (Coverage) Number" => Some(CodeE8),
                "Attachment Code" => Some(AttachmentCode),
                "Medical Record Identification Number" => {
                    Some(MedicalRecordIdentificationNumber)
                }
                "Embargo Permit Number" => Some(EmbargoPermitNumber),
                "Circular" => Some(Circular),
                "Fund Identifier" => Some(Fund),
                "Ballot Identifier" => Some(Ballot),
                "Legislative Identification Number" => {
                    Some(LegislativeIdentificationNumber)
                }
                "Lobbied Activity Identifier" => Some(LobbiedActivity),
                "Petition Number" => Some(PetitionNumber),
                "Related Form Number" => Some(RelatedFormNumber),
                "Carrier's Bond Number Covering Instruments of International Traffic (IIT)" => {
                    Some(CodeECJ)
                }
                "Export Declaration" => Some(ExportDeclaration),
                "Department of Education Acquisition Regulation (EDAR)" => Some(CodeEDA),
                "Election District" => Some(ElectionDistrict),
                "Electronic Funds Transfer ID Number" => {
                    Some(ElectronicFundsTransferIdNumber)
                }
                "Ending Serial Number" => Some(EndingSerialNumber),
                "Financial Classification Code" => Some(FinancialClassificationCode),
                "Employer's Identification Number" => Some(EmployersIdentificationNumber),
                "Importer's Bond Number Covering Instruments of International Traffic (IIT)" => {
                    Some(CodeEII)
                }
                "Patient Account Number" => Some(PatientAccountNumber),
                "Healthcare Manpower Shortage Area (HMSA) Facility Identification Number" => {
                    Some(CodeEK)
                }
                "Electronic device pin number" => Some(ElectronicDevicePinNumber),
                "Electronic Payment Reference Number" => {
                    Some(ElectronicPaymentReferenceNumber)
                }
                "End Mile Marker" => Some(EndMileMarker),
                "Embargo Number" => Some(EmbargoNumber),
                "Endorsement Number" => Some(EndorsementNumber),
                "Submitter Identification Number" => Some(SubmitterIdentificationNumber),
                "Export Permit Number" => Some(ExportPermitNumber),
                "Environmental Protection Agency Acquisition Regulation (EPAAR)" => {
                    Some(CodeEPA)
                }
                "Environmental Protection Agency Transporter Identification Number" => {
                    Some(EnvironmentalProtectionAgencyTransporterIdentificationNumber)
                }
                "Employer Payroll Code Lists" => Some(EmployerPayrollCodeLists),
                "Equipment Number" => Some(EquipmentNumber),
                "Container or Equipment Receipt Number" => {
                    Some(ContainerOrEquipmentReceiptNumber)
                }
                "Employer's Social Security Number" => {
                    Some(EmployersSocialSecurityNumber)
                }
                "Estimate Sequence Number" => Some(EstimateSequenceNumber),
                "Excess Transportation" => Some(ExcessTransportation),
                "End User's Purchase Order Number" => Some(EndUsersPurchaseOrderNumber),
                "Receiver Identification Number" => Some(ReceiverIdentificationNumber),
                "Event Identification" => Some(EventIdentification),
                "Mammography Certification Number" => {
                    Some(MammographyCertificationNumber)
                }
                "Estimate Number" => Some(EstimateNumber),
                "Exposure State Code" => Some(ExposureStateCode),
                "Receiver Sub-identification Number" => {
                    Some(ReceiverSubIdentificationNumber)
                }
                "Electronic Data Interchange Agreement Number" => {
                    Some(ElectronicDataInterchangeAgreementNumber)
                }
                "Version Code - National" => Some(VersionCodeNational),
                "Version Code - Local" => Some(VersionCodeLocal),
                "Submission Number" => Some(SubmissionNumber),
                "Facility Certification Number" => Some(FacilityCertificationNumber),
                "Medicare Version Code" => Some(MedicareVersionCode),
                "Health Insurance Claim (HIC) Number" => Some(CodeF6),
                "New Health Insurance Claim (HIC) Number" => Some(CodeF7),
                "Original Reference Number" => Some(OriginalReferenceNumber),
                "Freight Payor Reference Number" => Some(FreightPayorReferenceNumber),
                "Federal Acquisition Regulations (FAR)" => Some(CodeFA),
                "Fannie Mae Seller Servicer Number" => {
                    Some(FannieMaeSellerServicerNumber)
                }
                "File Transfer Form Number" => Some(FileTransferFormNumber),
                "Filer Code Issued by Customs" => Some(FilerCodeIssuedByCustoms),
                "Assigned Contract Number" => Some(AssignedContractNumber),
                "Filer Code Issued by Bureau of Census" => {
                    Some(FilerCodeIssuedByBureauOfCensus)
                }
                "Failure mechanism number" => Some(FailureMechanismNumber),
                "Foreign Entry Number" => Some(ForeignEntryNumber),
                "Film Number" => Some(FilmNumber),
                "Fund Identification Number" => Some(FundIdentificationNumber),
                "Clinic Number" => Some(ClinicNumber),
                "Federal Housing Administration Computerized Homes Underwriting Management System (CHUMS) Identification Number" => {
                    Some(CodeFHC)
                }
                "Federal Housing Administration Originator Identification" => {
                    Some(FederalHousingAdministrationOriginatorIdentification)
                }
                "File Identifier" => Some(File),
                "Line Item Control Number" => Some(LineItemControlNumber),
                "Finish Lot Number" => Some(FinishLotNumber),
                "Fine Line Classification" => Some(FineLineClassification),
                "Flood Zone" => Some(FloodZone),
                "Federal Maritime Commission (FMC) Forwarders Number" => Some(CodeFM),
                "Educational Commission for Foreign Medical Graduates (ECFMG) Certification Number" => {
                    Some(CodeFMG)
                }
                "Facility Measurement Point Number" => {
                    Some(FacilityMeasurementPointNumber)
                }
                "Forwarder's/Agent's Reference Number" => {
                    Some(ForwardersAgentsReferenceNumber)
                }
                "Finder Number" => Some(FinderNumber),
                "Drug Formulary Number" => Some(DrugFormularyNumber),
                "Forestry Permit Number" => Some(ForestryPermitNumber),
                "Form Number" => Some(FormNumber),
                "Freight Bill Number" => Some(FreightBillNumber),
                "Freddie Mac Seller Servicer Number" => {
                    Some(FreddieMacSellerServicerNumber)
                }
                "Final Sequence Number" => Some(FinalSequenceNumber),
                "Fund Source Code" => Some(FundSourceCode),
                "Assigned Sequence Number" => Some(AssignedSequenceNumber),
                "Foreign Trade Zone" => Some(ForeignTradeZone),
                "Premarket Notification Number" => Some(PremarketNotificationNumber),
                "File Transfer Protocol (FTP) Locator" => Some(CodeFTP),
                "Foreign Trade Zone (FTZ) Admission Number" => Some(CodeFTZ),
                "Fund Code" => Some(FundCode),
                "Health Maintenance Organization (HMO) Reference Number" => Some(CodeFV),
                "State License Identification Number" => {
                    Some(StateLicenseIdentificationNumber)
                }
                "Final Work Candidate Number" => Some(FinalWorkCandidateNumber),
                "Failure Analysis Report Number" => Some(FailureAnalysisReportNumber),
                "Claim Office Number" => Some(ClaimOfficeNumber),
                "Processor's Invoice Number" => Some(ProcessorsInvoiceNumber),
                "Prior Authorization Number" => Some(PriorAuthorizationNumber),
                "Provider Commercial Number" => Some(ProviderCommercialNumber),
                "Predetermination of Benefits Identification Number" => {
                    Some(PredeterminationOfBenefitsIdentificationNumber)
                }
                "Peer Review Organization (PRO) Approval Number" => Some(CodeG4),
                "Provider Site Number" => Some(ProviderSiteNumber),
                "Payer Assigned Resubmission Reference Number" => {
                    Some(PayerAssignedResubmissionReferenceNumber)
                }
                "Resubmission Reason Code" => Some(ResubmissionReasonCode),
                "Resubmission Number" => Some(ResubmissionNumber),
                "Secondary Employee Identification Number" => {
                    Some(SecondaryEmployeeIdentificationNumber)
                }
                "Government Advance Progress" => Some(GovernmentAdvanceProgress),
                "Grain Block Number" => Some(GrainBlockNumber),
                "Government Contract Number" => Some(GovernmentContractNumber),
                "Return Goods Bill of Lading Number" => {
                    Some(ReturnGoodsBillOfLadingNumber)
                }
                "Geographic Number" => Some(GeographicNumber),
                "Specialty License Number" => Some(SpecialtyLicenseNumber),
                "Gauge Ticket Number" => Some(GaugeTicketNumber),
                "Identification Card Serial Number" => {
                    Some(IdentificationCardSerialNumber)
                }
                "Secondary Provider Number" => Some(SecondaryProviderNumber),
                "Cornbore Certification Number" => Some(CornboreCertificationNumber),
                "Third Party Reference Number" => Some(ThirdPartyReferenceNumber),
                "Geographic Destination Zone Number" => {
                    Some(GeographicDestinationZoneNumber)
                }
                "Loan Acquisition Number" => Some(LoanAcquisitionNumber),
                "Folder Number" => Some(FolderNumber),
                "Exhibit Identifier" => Some(Exhibit),
                "Government Priority Number" => Some(GovernmentPriorityNumber),
                "Internal Purchase Order Release Number" => {
                    Some(InternalPurchaseOrderReleaseNumber)
                }
                "Grain Order Reference Number" => Some(GrainOrderReferenceNumber),
                "General Services Administration Regulations (GSAR)" => Some(CodeGS),
                "Goods and Service Tax Registration Number" => {
                    Some(GoodsAndServiceTaxRegistrationNumber)
                }
                "Internal Purchase Order Item Number" => {
                    Some(InternalPurchaseOrderItemNumber)
                }
                "Third Party Purchase Order Number" => {
                    Some(ThirdPartyPurchaseOrderNumber)
                }
                "Third Party Purchase Order Release Number" => {
                    Some(ThirdPartyPurchaseOrderReleaseNumber)
                }
                "Group Work Candidate Sequence Number" => {
                    Some(GroupWorkCandidateSequenceNumber)
                }
                "Third Party Purchase Order Item Number" => {
                    Some(ThirdPartyPurchaseOrderItemNumber)
                }
                "Empty Repositioning Number" => Some(EmptyRepositioningNumber),
                "General Ledger Account" => Some(GeneralLedgerAccount),
                "High Fabrication Authorization Number" => {
                    Some(HighFabricationAuthorizationNumber)
                }
                "High Raw Material Authorization Number" => {
                    Some(HighRawMaterialAuthorizationNumber)
                }
                "Gravity Source Meter Number" => Some(GravitySourceMeterNumber),
                "Special Clause" => Some(SpecialClause),
                "Quality Clause" => Some(QualityClause),
                "Standard Clause" => Some(StandardClause),
                "Home Mortgage Disclosure Act (HMDA) Census Tract" => Some(CodeH8),
                "Payment History Reference Number" => Some(PaymentHistoryReferenceNumber),
                "Competent Authority" => Some(CompetentAuthority),
                "Bill & Hold Invoice Number" => Some(CodeHB),
                "Heat Code" => Some(HeatCode),
                "Department of Transportation Hazardous Number" => {
                    Some(DepartmentOfTransportationHazardousNumber)
                }
                "Hazardous Exemption Number" => Some(HazardousExemptionNumber),
                "Engineering Data List" => Some(EngineeringDataList),
                "Civil Action Number" => Some(CivilActionNumber),
                "Fiscal Code" => Some(FiscalCode),
                "Type of Household Goods Code" => Some(TypeOfHouseholdGoodsCode),
                "Health Industry Number (HIN)" => Some(CodeHI),
                "Identity Card Number" => Some(IdentityCardNumber),
                "Judgment Number" => Some(JudgmentNumber),
                "SIREN Number" => Some(SirenNumber),
                "SIRET Number" => Some(SiretNumber),
                "Home Mortgage Disclosure Act Block Number Area" => {
                    Some(HomeMortgageDisclosureActBlockNumberArea)
                }
                "Hazardous Certification Number" => Some(HazardousCertificationNumber),
                "Shipper's Hazardous Number" => Some(ShippersHazardousNumber),
                "Pack & Hold Invoice Number" => Some(CodeHP),
                "Centers for Medicare and Medicaid Services National Provider Identifier" => {
                    Some(CentersForMedicareAndMedicaidServicesNationalProvider)
                }
                "Reinsurance Reference" => Some(ReinsuranceReference),
                "Horsepower" => Some(Horsepower),
                "Harmonized Code System (Canada)" => Some(CodeHS),
                "Code of Federal Regulations" => Some(CodeOfFederalRegulations),
                "Type of Escrow Number" => Some(TypeOfEscrowNumber),
                "Department of Housing and Urban Development Acquisition Regulation (HUDAR)" => {
                    Some(CodeHUD)
                }
                "Escrow File Number" => Some(EscrowFileNumber),
                "High/Wide File Number" => Some(HighWideFileNumber),
                "Auto Loss Item Number" => Some(AutoLossItemNumber),
                "Property Loss Item Number" => Some(PropertyLossItemNumber),
                "Tax Agency Number (MERS [Mortgage Electronic Registration System] Federal Information Processing Standards [FIPS] Based Number)" => {
                    Some(CodeHZ)
                }
                "Owning Bureau Identification Number" => {
                    Some(OwningBureauIdentificationNumber)
                }
                "Interstate Commerce Commission (ICC) Account Number" => Some(CodeI2),
                "Non-American Identification Number" => {
                    Some(NonAmericanIdentificationNumber)
                }
                "Credit Counseling Identification Number" => {
                    Some(CreditCounselingIdentificationNumber)
                }
                "Invoice Identification" => Some(InvoiceIdentification),
                "Credit Report Number" => Some(CreditReportNumber),
                "Pollutant" => Some(Pollutant),
                "Internal Vendor Number" => Some(InternalVendorNumber),
                "In Bond Number" => Some(InBondNumber),
                "Inbound-to Party" => Some(InboundToParty),
                "ICD-9-CM (International Classification of Diseases)" => Some(CodeICD),
                "Insurance Certificate Number" => Some(InsuranceCertificateNumber),
                "Interchange Agreement Number" => Some(InterchangeAgreementNumber),
                "Issue Number" => Some(IssueNumber),
                "Initial Failure Claim" => Some(InitialFailureClaim),
                "International Fuel Tax Agreement Account Number" => {
                    Some(InternationalFuelTaxAgreementAccountNumber)
                }
                "Insurance Policy Number" => Some(InsurancePolicyNumber),
                "Initial Dealer Claim Number" => Some(InitialDealerClaimNumber),
                "Initial Sample Inspection Report Number" => {
                    Some(InitialSampleInspectionReportNumber)
                }
                "Image Identifier" => Some(Image),
                "Standard Industry Classification (SIC) Code" => Some(CodeIJ),
                "Invoice Number" => Some(InvoiceNumber),
                "Internal Order Number" => Some(InternalOrderNumber),
                "Intergovernmental Maritime Organization (IMO) Number" => Some(CodeIM),
                "Integrated Master Plan (IMP)" => Some(CodeIMP),
                "Integrated Master Schedule (IMS)" => Some(CodeIMS),
                "Consignee's Invoice Number" => Some(ConsigneesInvoiceNumber),
                "Investigatorial New Drug Number" => Some(InvestigatorialNewDrugNumber),
                "Inbound-to or Outbound-from Party" => Some(InboundToOrOutboundFromParty),
                "Inspection Report Number" => Some(InspectionReportNumber),
                "End Item" => Some(EndItem),
                "Intra Plant Routing" => Some(IntraPlantRouting),
                "Importer's Reference Number to Letter of Credit" => {
                    Some(ImportersReferenceNumberToLetterOfCredit)
                }
                "International Registration Plan Account Number" => {
                    Some(InternationalRegistrationPlanAccountNumber)
                }
                "Invoice Number Suffix" => Some(InvoiceNumberSuffix),
                "International Standard Industrial Classification (ISIC) Dominion of Canada Code (DCC)" => {
                    Some(CodeISC)
                }
                "International Registration Plan Sticker Number" => {
                    Some(InternationalRegistrationPlanStickerNumber)
                }
                "Inspection and Survey Sequence Number" => {
                    Some(InspectionAndSurveySequenceNumber)
                }
                "Internal Customer Number" => Some(InternalCustomerNumber),
                "Initial Trouble Indication" => Some(InitialTroubleIndication),
                "Barge Permit Number" => Some(BargePermitNumber),
                "Seller's Invoice Number" => Some(SellersInvoiceNumber),
                "Part Interchangeability" => Some(PartInterchangeability),
                "Item Number" => Some(ItemNumber),
                "Insured Parcel Post Number" => Some(InsuredParcelPostNumber),
                "Proceeding" => Some(Proceeding),
                "Creditor" => Some(Creditor),
                "Attorney" => Some(Attorney),
                "Judge" => Some(Judge),
                "Trustee" => Some(Trustee),
                "Originating Case" => Some(OriginatingCase),
                "Adversary Case" => Some(AdversaryCase),
                "Lead Case" => Some(LeadCase),
                "Jointly Administered Case" => Some(JointlyAdministeredCase),
                "Substantively Consolidated Case" => Some(SubstantivelyConsolidatedCase),
                "Beginning Job Sequence Number" => Some(BeginningJobSequenceNumber),
                "Job (Project) Number" => Some(CodeJB),
                "Review" => Some(Review),
                "Joint Credit Specification Number" => {
                    Some(JointCreditSpecificationNumber)
                }
                "User Identification" => Some(UserIdentification),
                "Ending Job Sequence Number" => Some(EndingJobSequenceNumber),
                "Automated Underwriting Reference Number" => {
                    Some(AutomatedUnderwritingReferenceNumber)
                }
                "Tag" => Some(Tag),
                "Multiple Listing Service Area" => Some(MultipleListingServiceArea),
                "Multiple Listing Service Sub-area" => {
                    Some(MultipleListingServiceSubArea)
                }
                "Packet" => Some(Packet),
                "Multiple Listing Service Map X Coordinate" => {
                    Some(MultipleListingServiceMapXCoordinate)
                }
                "Multiple Listing Service Map Y Coordinate" => {
                    Some(MultipleListingServiceMapYCoordinate)
                }
                "Multiple Listing Number" => Some(MultipleListingNumber),
                "Multiple Listing Service Book Type" => {
                    Some(MultipleListingServiceBookType)
                }
                "Elevation" => Some(Elevation),
                "Property Component Location" => Some(PropertyComponentLocation),
                "Job Sequence Number" => Some(JobSequenceNumber),
                "Prior Tax Identification Number (TIN)" => Some(CodeJT),
                "Prior Phone Number" => Some(PriorPhoneNumber),
                "Prior Health Industry Number" => Some(PriorHealthIndustryNumber),
                "Prior Universal Provider Identification Number (UPIN)" => Some(CodeJW),
                "Prior Postal Zip Code" => Some(PriorPostalZipCode),
                "Origin of Shipment Harmonized-Based Code" => {
                    Some(OriginOfShipmentHarmonizedBasedCode)
                }
                "Governing Class Code" => Some(GoverningClassCode),
                "Approval Code" => Some(ApprovalCode),
                "Foreign Military Sales Notice Number" => {
                    Some(ForeignMilitarySalesNoticeNumber)
                }
                "Certified Mail Number" => Some(CertifiedMailNumber),
                "Registered Mail Number" => Some(RegisteredMailNumber),
                "Criticality Designator" => Some(CriticalityDesignator),
                "Task Order" => Some(TaskOrder),
                "Purchase Description" => Some(PurchaseDescription),
                "Paragraph Number" => Some(ParagraphNumber),
                "Project Paragraph Number" => Some(ProjectParagraphNumber),
                "Inquiry Request Number" => Some(InquiryRequestNumber),
                "Distribution List" => Some(DistributionList),
                "Associated Contract Identifier" => Some(AssociatedContract),
                "Beginning Kanban Serial Number" => Some(BeginningKanbanSerialNumber),
                "Exhibit Distribution List" => Some(ExhibitDistributionList),
                "Confirmation Service Contract Identifier" => {
                    Some(ConfirmationServiceContract)
                }
                "Special Instructions Number" => Some(SpecialInstructionsNumber),
                "Ending Kanban Serial Number" => Some(EndingKanbanSerialNumber),
                "Foreclosing Status" => Some(ForeclosingStatus),
                "Type of Law Suit" => Some(TypeOfLawSuit),
                "Type of Outstanding Judgment" => Some(TypeOfOutstandingJudgment),
                "Confirmation Intraday Identifier" => Some(ConfirmationIntraday),
                "Tax Lien Jurisdiction" => Some(TaxLienJurisdiction),
                "Delivery Reference" => Some(DeliveryReference),
                "Contract Reference" => Some(ContractReference),
                "Rental Account Number" => Some(RentalAccountNumber),
                "Census Automated Files ID" => Some(CensusAutomatedFilesId),
                "Customs Drawback Entry Number" => Some(CustomsDrawbackEntryNumber),
                "Health Certificate Number" => Some(HealthCertificateNumber),
                "Procuring Agency" => Some(ProcuringAgency),
                "Response to a Request for Quotation Reference" => {
                    Some(ResponseToARequestForQuotationReference)
                }
                "Releaser Contract Identifier" => Some(ReleaserContract),
                "Replacement Shipper Contract Identifier" => {
                    Some(ReplacementShipperContract)
                }
                "Solicitation" => Some(Solicitation),
                "Service Requester Contract Identifier" => Some(ServiceRequesterContract),
                "Request for Quotation Reference" => Some(RequestForQuotationReference),
                "Office Symbol" => Some(OfficeSymbol),
                "Distribution Statement Code" => Some(DistributionStatementCode),
                "Certification" => Some(Certification),
                "Representation" => Some(Representation),
                "Site Specific Procedures, Terms, and Conditions" => Some(CodeKY),
                "Master Solicitation Procedures, Terms, and Conditions" => Some(CodeKZ),
                "Collision Industry Electronic Commerce Association (CIECA)" => {
                    Some(CodeL0)
                }
                "Letters or Notes" => Some(LettersOrNotes),
                "Location on Product Code" => Some(LocationOnProductCode),
                "Labor Operation Number" => Some(LaborOperationNumber),
                "Proposal Paragraph Number" => Some(ProposalParagraphNumber),
                "Subexhibit Line Item Number" => Some(SubexhibitLineItemNumber),
                "Subcontract Line Item Number" => Some(SubcontractLineItemNumber),
                "Customer's Release Number" => Some(CustomersReleaseNumber),
                "Consignee's Release Number" => Some(ConsigneesReleaseNumber),
                "Customer's Part Number" => Some(CustomersPartNumber),
                "Shipping Label Serial Number" => Some(ShippingLabelSerialNumber),
                "Lottery Authority Activation Number" => {
                    Some(LotteryAuthorityActivationNumber)
                }
                "Lane Number" => Some(LaneNumber),
                "Lockbox" => Some(Lockbox),
                "Lease Number" => Some(LeaseNumber),
                "Loan Number" => Some(LoanNumber),
                "Lender Entity Number" => Some(LenderEntityNumber),
                "Location Exception Order Number" => Some(LocationExceptionOrderNumber),
                "Assembly Line Feed Location" => Some(AssemblyLineFeedLocation),
                "Lease Schedule Number" => Some(LeaseScheduleNumber),
                "Longitude Expressed in Seconds" => Some(LongitudeExpressedInSeconds),
                "Line Item Identifier (Seller's)" => Some(CodeLI),
                "Health Industry Business Communications Council (HIBCC) Labeler Identification Code (LIC)" => {
                    Some(CodeLIC)
                }
                "Local Jurisdiction" => Some(LocalJurisdiction),
                "Longitude expressed in Degrees, Minutes and Seconds" => Some(CodeLK),
                "Latitude Expressed in Seconds" => Some(LatitudeExpressedInSeconds),
                "Product Period for which Labor Costs are Firm" => {
                    Some(ProductPeriodForWhichLaborCostsAreFirm)
                }
                "Local Media Identifier" => Some(LocalMedia),
                "Non Pickup Limited Tariff Number" => Some(NonPickupLimitedTariffNumber),
                "Load Planning Number" => Some(LoadPlanningNumber),
                "Logical Observation Identifier Names and Codes (LOINC)" => Some(CodeLOI),
                "Loss Conditions" => Some(LossConditions),
                "For Pickup Limited Freight Tariff Number" => {
                    Some(ForPickupLimitedFreightTariffNumber)
                }
                "Loan Prospector Key Number" => Some(LoanProspectorKeyNumber),
                "Latitude Expressed in Degrees, Minutes and Seconds" => Some(CodeLQ),
                "Local Student Identification Number" => {
                    Some(LocalStudentIdentificationNumber)
                }
                "Bar-Coded Serial Number" => Some(BarCodedSerialNumber),
                "Logistics Support Documentation Type Code" => {
                    Some(LogisticsSupportDocumentationTypeCode)
                }
                "Lot Number" => Some(LotNumber),
                "Location Number" => Some(LocationNumber),
                "License Plate Number" => Some(LicensePlateNumber),
                "Levying Officer Identification" => Some(LevyingOfficerIdentification),
                "Location Within Equipment" => Some(LocationWithinEquipment),
                "Qualified Products List" => Some(QualifiedProductsList),
                "Destination of Shipment Harmonized-Based Code" => {
                    Some(DestinationOfShipmentHarmonizedBasedCode)
                }
                "Lender Account Number" => Some(LenderAccountNumber),
                "Mexican Pedimento Number" => Some(MexicanPedimentoNumber),
                "Material Storage Location" => Some(MaterialStorageLocation),
                "Major Force Program" => Some(MajorForceProgram),
                "Crop Year" => Some(CropYear),
                "Lease Agreement Amendment Number - Master" => {
                    Some(LeaseAgreementAmendmentNumberMaster)
                }
                "Military Ordnance Security Risk Number" => {
                    Some(MilitaryOrdnanceSecurityRiskNumber)
                }
                "Medical Assistance Category" => Some(MedicalAssistanceCategory),
                "Limited Partnership Identification Number" => {
                    Some(LimitedPartnershipIdentificationNumber)
                }
                "Tax Shelter Number" => Some(TaxShelterNumber),
                "Ship Notice/Manifest Number" => Some(ShipNoticeManifestNumber),
                "Master Bill of Lading" => Some(MasterBillOfLading),
                "Mortgage Backed Security (MBS) Policy Number" => Some(CodeMBS),
                "Mailbox" => Some(Mailbox),
                "Microfilm Number" => Some(MicrofilmNumber),
                "Carrier's Bond Number Covering Merchandise Shipment and Instruments of International Traffic (IIT)" => {
                    Some(CodeMCC)
                }
                "Motor Carrier Identification Number" => {
                    Some(MotorCarrierIdentificationNumber)
                }
                "MORNETPlus Case Number" => Some(MornetPlusCaseNumber),
                "Magazine Code" => Some(MagazineCode),
                "Hazardous Waste Manifest Document Number" => {
                    Some(HazardousWasteManifestDocumentNumber)
                }
                "Message Address or ID" => Some(MessageAddressOrId),
                "Manufacturers Part Number" => Some(ManufacturersPartNumber),
                "Meter Number" => Some(MeterNumber),
                "Manufacturing Order Number" => Some(ManufacturingOrderNumber),
                "Mill Order Number" => Some(MillOrderNumber),
                "Importer's Bond Number Covering Merchandise Shipment and Instruments of International Traffic (IIT)" => {
                    Some(CodeMII)
                }
                "MORNETPlus Institution Number" => Some(MornetPlusInstitutionNumber),
                "Model Number" => Some(ModelNumber),
                "Manifest Key Number" => Some(ManifestKeyNumber),
                "Military Rank/Civilian Pay Grade Number" => {
                    Some(MilitaryRankCivilianPayGradeNumber)
                }
                "Master Lease Agreement Number" => Some(MasterLeaseAgreementNumber),
                "MICR Number" => Some(MicrNumber),
                "Manufacturing Operation Number" => Some(ManufacturingOperationNumber),
                "Multiple P.O.s of an Invoice" => Some(MultiplePOSOfAnInvoice),
                "Marketing Plan Identification Number" => {
                    Some(MarketingPlanIdentificationNumber)
                }
                "Meter Proving Report Number" => Some(MeterProvingReportNumber),
                "Merchandise Type Code" => Some(MerchandiseTypeCode),
                "Eligibility Category" => Some(EligibilityCategory),
                "Mother's Medical Record Identification Number" => {
                    Some(MothersMedicalRecordIdentificationNumber)
                }
                "Manufacturer's Material Safety Data Sheet Number" => {
                    Some(ManufacturersMaterialSafetyDataSheetNumber)
                }
                "Mail Slot" => Some(MailSlot),
                "Meter Ticket Number" => Some(MeterTicketNumber),
                "Military Specification (MILSPEC) Number" => Some(CodeMU),
                "MORNETPlus User Identification" => Some(MornetPlusUserIdentification),
                "Migrant Number" => Some(MigrantNumber),
                "Military Call Number" => Some(MilitaryCallNumber),
                "Material Change Notice Number" => Some(MaterialChangeNoticeNumber),
                "Model year number" => Some(ModelYearNumber),
                "Maintenance Request Number" => Some(MaintenanceRequestNumber),
                "Multiple Zone Order Number" => Some(MultipleZoneOrderNumber),
                "Nomination Number" => Some(NominationNumber),
                "Local School Course Number" => Some(LocalSchoolCourseNumber),
                "Local School District Course Number" => {
                    Some(LocalSchoolDistrictCourseNumber)
                }
                "Statewide Course Number" => Some(StatewideCourseNumber),
                "United States Department of Education, National Center for Education Statistics (NCES) Course Number" => {
                    Some(CodeN4)
                }
                "Provider Plan Network Identification Number" => {
                    Some(ProviderPlanNetworkIdentificationNumber)
                }
                "Plan Network Identification Number" => {
                    Some(PlanNetworkIdentificationNumber)
                }
                "Facility Network Identification Number" => {
                    Some(FacilityNetworkIdentificationNumber)
                }
                "Secondary Health Insurance Identification Number" => {
                    Some(SecondaryHealthInsuranceIdentificationNumber)
                }
                "Data Authentication Number" => Some(DataAuthenticationNumber),
                "North American Hazardous Classification Number" => {
                    Some(NorthAmericanHazardousClassificationNumber)
                }
                "National Aeronautics and Space Administration FAR Supplement (NFS)" => {
                    Some(CodeNAS)
                }
                "Letter of Credit Number" => Some(LetterOfCreditNumber),
                "Secondary Coverage Company Number" => {
                    Some(SecondaryCoverageCompanyNumber)
                }
                "Letter of Credit Draft Number" => Some(LetterOfCreditDraftNumber),
                "Abbreviated New Drug Application Number" => {
                    Some(AbbreviatedNewDrugApplicationNumber)
                }
                "New Drug Application Number" => Some(NewDrugApplicationNumber),
                "Lease Rider Number" => Some(LeaseRiderNumber),
                "National Association of Insurance Commissioners (NAIC) Code" => {
                    Some(CodeNF)
                }
                "National Flood Insurance Program Community Name" => {
                    Some(NationalFloodInsuranceProgramCommunityName)
                }
                "National Flood Insurance Program County" => {
                    Some(NationalFloodInsuranceProgramCounty)
                }
                "National Flood Insurance Program Map Number" => {
                    Some(NationalFloodInsuranceProgramMapNumber)
                }
                "National Flood Insurance Program Community Number" => {
                    Some(NationalFloodInsuranceProgramCommunityNumber)
                }
                "National Flood Insurance Program State" => {
                    Some(NationalFloodInsuranceProgramState)
                }
                "Natural Gas Policy Act Category Code" => {
                    Some(NaturalGasPolicyActCategoryCode)
                }
                "Rate Card Number" => Some(RateCardNumber),
                "Military Standard (MIL-STD) Number" => Some(CodeNI),
                "Technical Document Number" => Some(TechnicalDocumentNumber),
                "Prior Case" => Some(PriorCase),
                "Technical Order Number" => Some(TechnicalOrderNumber),
                "Discounter Registration Number" => Some(DiscounterRegistrationNumber),
                "Nomination Model Type" => Some(NominationModelType),
                "Nonconformance Report Number" => Some(NonconformanceReportNumber),
                "No OT5 Authority-zero Mileage Rate" => {
                    Some(NoOt5AuthorityZeroMileageRate)
                }
                "Partial Payment Number" => Some(PartialPaymentNumber),
                "Medicaid Recipient Identification Number" => {
                    Some(MedicaidRecipientIdentificationNumber)
                }
                "Progress Payment Number" => Some(ProgressPaymentNumber),
                "National Stock Number" => Some(NationalStockNumber),
                "Administrator's Reference Number" => Some(AdministratorsReferenceNumber),
                "Non-originating Third Party Number" => {
                    Some(NonOriginatingThirdPartyNumber)
                }
                "Pending Case" => Some(PendingCase),
                "Associated Policy Number" => Some(AssociatedPolicyNumber),
                "Related Nonconformance Number" => Some(RelatedNonconformanceNumber),
                "Agent Claim Number" => Some(AgentClaimNumber),
                "Critical Application" => Some(CriticalApplication),
                "Outer Continental Shelf Area Code" => {
                    Some(OuterContinentalShelfAreaCode)
                }
                "Outer Continental Shelf Block Number" => {
                    Some(OuterContinentalShelfBlockNumber)
                }
                "OT5 Authority-Condition or Restriction on Car Hire Rate" => {
                    Some(Ot5AuthorityConditionOrRestrictionOnCarHireRate)
                }
                "On-line Procurement and Accounting Control (OPAC) Transaction" => {
                    Some(CodeO7)
                }
                "Original Filing" => Some(OriginalFiling),
                "Continuation Filing" => Some(ContinuationFiling),
                "Outlet Number" => Some(OutletNumber),
                "Ocean Bill of Lading" => Some(OceanBillOfLading),
                "Ocean Container Number" => Some(OceanContainerNumber),
                "Original Return Request Reference Number" => {
                    Some(OriginalReturnRequestReferenceNumber)
                }
                "Open and Prepaid Station List Number" => {
                    Some(OpenAndPrepaidStationListNumber)
                }
                "Operator Identification Number" => Some(OperatorIdentificationNumber),
                "Offer Identifier" => Some(Offer),
                "Termination Filing" => Some(TerminationFiling),
                "Origin House" => Some(OriginHouse),
                "Original Invoice Number" => Some(OriginalInvoiceNumber),
                "Object Identifier" => Some(Object),
                "Amendment Filing" => Some(AmendmentFiling),
                "Offer Group" => Some(OfferGroup),
                "Original Shipper's Bill of Lading Number" => {
                    Some(OriginalShippersBillOfLadingNumber)
                }
                "Ocean Manifest" => Some(OceanManifest),
                "Dealer Order Number" => Some(DealerOrderNumber),
                "Out of Service Number" => Some(OutOfServiceNumber),
                "Original Purchase Order" => Some(OriginalPurchaseOrder),
                "National Center for Education Statistics Office of Postsecondary Education (OPE) Code" => {
                    Some(CodeOPE)
                }
                "National Center for Education Statistics Integrated Postsecondary Education Data System (IPEDS) Athletic Conference Code" => {
                    Some(CodeOPF)
                }
                "Order Number" => Some(OrderNumber),
                "Order/Paragraph Number" => Some(OrderParagraphNumber),
                "Outbound-from Party" => Some(OutboundFromParty),
                "Sales Allowance Number" => Some(SalesAllowanceNumber),
                "Tariff Supplement Number" => Some(TariffSupplementNumber),
                "Tariff Suffix Number" => Some(TariffSuffixNumber),
                "Service Order Number" => Some(ServiceOrderNumber),
                "Statement Number" => Some(StatementNumber),
                "Product Number" => Some(ProductNumber),
                "Previous Contract Number" => Some(PreviousContractNumber),
                "Previous Drug Enforcement Administration Number" => {
                    Some(PreviousDrugEnforcementAdministrationNumber)
                }
                "Previous customer reference number" => {
                    Some(PreviousCustomerReferenceNumber)
                }
                "Project Code" => Some(ProjectCode),
                "Position Code" => Some(PositionCode),
                "Pipeline Number" => Some(PipelineNumber),
                "Product Line Number" => Some(ProductLineNumber),
                "Pickup Reference Number" => Some(PickupReferenceNumber),
                "Page Number" => Some(PageNumber),
                "Price Area Number" => Some(PriceAreaNumber),
                "Patent Cooperation Treaty Application Number" => {
                    Some(PatentCooperationTreatyApplicationNumber)
                }
                "Nonprovisional Patent Application Number" => {
                    Some(NonprovisionalPatentApplicationNumber)
                }
                "Provisional Patent Application Number" => {
                    Some(ProvisionalPatentApplicationNumber)
                }
                "Payer's Financial Institution Account Number for Check, Draft, or Wire Payments; Originating Company Account Number for ACH Transfers" => {
                    Some(CodePB)
                }
                "Production Code" => Some(ProductionCode),
                "Pool Contract Code" => Some(PoolContractCode),
                "Protocol Number" => Some(ProtocolNumber),
                "Promotion/Deal Number" => Some(PromotionDealNumber),
                "Partial Denial Indicator" => Some(PartialDenialIndicator),
                "Previous Driver's License" => Some(PreviousDriversLicense),
                "Partial Denial Reason Identifier" => Some(PartialDenialReason),
                "Plant Number" => Some(PlantNumber),
                "Prime Contractor Contract Number" => Some(PrimeContractorContractNumber),
                "Product Group" => Some(ProductGroup),
                "Packing Group Code" => Some(PackingGroupCode),
                "Downstream Package Identifier" => Some(DownstreamPackage),
                "Plug Number" => Some(PlugNumber),
                "Proposed Group Work Candidate Sequence Number" => {
                    Some(ProposedGroupWorkCandidateSequenceNumber)
                }
                "Priority Rating" => Some(PriorityRating),
                "Process Handling Code" => Some(ProcessHandlingCode),
                "Physician State License Number" => Some(PhysicianStateLicenseNumber),
                "Price List Change or Issue Number" => Some(PriceListChangeOrIssueNumber),
                "Program Identification Number" => Some(ProgramIdentificationNumber),
                "Platform Identification Number" => Some(PlatformIdentificationNumber),
                "Packer Number" => Some(PackerNumber),
                "Previous Report Number" => Some(PreviousReportNumber),
                "Packing List Number" => Some(PackingListNumber),
                "Package Identifier" => Some(Package),
                "Upstream Package Identifier" => Some(UpstreamPackage),
                "Price List Number" => Some(PriceListNumber),
                "Product Licensing Agreement Number" => {
                    Some(ProductLicensingAgreementNumber)
                }
                "Proposed Contract Number" => Some(ProposedContractNumber),
                "Part Number" => Some(PartNumber),
                "Premarket Application Number" => Some(PremarketApplicationNumber),
                "Permit Number" => Some(PermitNumber),
                "Patent Number" => Some(PatentNumber),
                "Purchase Order Number" => Some(PurchaseOrderNumber),
                "Policy Number" => Some(PolicyNumber),
                "Position Title Number" => Some(PositionTitleNumber),
                "Purchase Order Revision Number" => Some(PurchaseOrderRevisionNumber),
                "Certificate of Purchase Number" => Some(CertificateOfPurchaseNumber),
                "Tax Bill Identification Number" => Some(TaxBillIdentificationNumber),
                "Current Year Tax Bill Number" => Some(CurrentYearTaxBillNumber),
                "Past Year Tax Bill Number" => Some(PastYearTaxBillNumber),
                "Payment Plan Number" => Some(PaymentPlanNumber),
                "Payee Identification" => Some(PayeeIdentification),
                "Price Quote Number" => Some(PriceQuoteNumber),
                "Previously Reported Social Security Number" => {
                    Some(PreviouslyReportedSocialSecurityNumber)
                }
                "Product Type" => Some(ProductType),
                "Purchase Order Number Suffix" => Some(PurchaseOrderNumberSuffix),
                "Previous Shipment Identification Number - Continuous Move" => {
                    Some(PreviousShipmentIdentificationNumberContinuousMove)
                }
                "Next Shipment Identification Number - Continuous Move" => {
                    Some(NextShipmentIdentificationNumberContinuousMove)
                }
                "Credit Card" => Some(CreditCard),
                "Proposed Sequence Number" => Some(ProposedSequenceNumber),
                "Purchase Option Agreement" => Some(PurchaseOptionAgreement),
                "Patent Type" => Some(PatentType),
                "Previous Bill of Lading Number" => Some(PreviousBillOfLadingNumber),
                "Pickup Appointment Number" => Some(PickupAppointmentNumber),
                "Product change information number" => {
                    Some(ProductChangeInformationNumber)
                }
                "Payment Validation Code" => Some(PaymentValidationCode),
                "Prior purchase order number" => Some(PriorPurchaseOrderNumber),
                "Preliminary Work Candidate Number" => {
                    Some(PreliminaryWorkCandidateNumber)
                }
                "Proposed Work Candidate Sequence Number" => {
                    Some(ProposedWorkCandidateSequenceNumber)
                }
                "Previous Invoice Number" => Some(PreviousInvoiceNumber),
                "Health Care Provider Taxonomy Code" => {
                    Some(HealthCareProviderTaxonomyCode)
                }
                "Payee's Financial Institution Account Number for Check, Draft or Wire Payments; Receiving Company Account Number for ACH Transfer" => {
                    Some(CodePY)
                }
                "Payroll Activity Code" => Some(PayrollActivityCode),
                "Pay Range" => Some(PayRange),
                "Product Change Notice Number" => Some(ProductChangeNoticeNumber),
                "Quote Number" => Some(QuoteNumber),
                "Starting Package Number" => Some(StartingPackageNumber),
                "Ending Package Number" => Some(EndingPackageNumber),
                "Prior Identifier Number" => Some(PriorIdentifierNumber),
                "Property Control Number" => Some(PropertyControlNumber),
                "Recall Number" => Some(RecallNumber),
                "Receiver Claim Number" => Some(ReceiverClaimNumber),
                "Registration Number" => Some(RegistrationNumber),
                "Repair Order Number" => Some(RepairOrderNumber),
                "Press Identifier" => Some(Press),
                "Press Form Identifier" => Some(PressForm),
                "Product Specification Document Number" => {
                    Some(ProductSpecificationDocumentNumber)
                }
                "Replacement Drug Enforcement Administration Number" => {
                    Some(ReplacementDrugEnforcementAdministrationNumber)
                }
                "Replacement Customer Reference Number" => {
                    Some(ReplacementCustomerReferenceNumber)
                }
                "Quality Disposition Area Identifier" => Some(QualityDispositionArea),
                "Replacement Assembly Model Number" => {
                    Some(ReplacementAssemblyModelNumber)
                }
                "Replacement Assembly Serial Number" => {
                    Some(ReplacementAssemblySerialNumber)
                }
                "Quality Inspection Area Identifier" => Some(QualityInspectionArea),
                "Return Material Authorization Number" => {
                    Some(ReturnMaterialAuthorizationNumber)
                }
                "Sales Program Number" => Some(SalesProgramNumber),
                "Service Authorization Number" => Some(ServiceAuthorizationNumber),
                "Quality Review Material Crib Identifier" => {
                    Some(QualityReviewMaterialCrib)
                }
                "Stop Sequence Number" => Some(StopSequenceNumber),
                "Service Estimate Number" => Some(ServiceEstimateNumber),
                "Substitute Part Number" => Some(SubstitutePartNumber),
                "Unit Number" => Some(UnitNumber),
                "Quality Report Number" => Some(QualityReportNumber),
                "Warranty Coverage Code" => Some(WarrantyCoverageCode),
                "Warranty Registration Number" => Some(WarrantyRegistrationNumber),
                "Change Verification Procedure Code" => {
                    Some(ChangeVerificationProcedureCode)
                }
                "Major System Affected Code" => Some(MajorSystemAffectedCode),
                "New Part Number" => Some(NewPartNumber),
                "Old Part Number" => Some(OldPartNumber),
                "Service Performed Code" => Some(ServicePerformedCode),
                "Reference Drawing Number" => Some(ReferenceDrawingNumber),
                "Regiristo Federal de Contribuyentes (Mexican Federal Tax ID Number)" => {
                    Some(CodeR0)
                }
                "Current Revision Number" => Some(CurrentRevisionNumber),
                "Canceled Revision Number" => Some(CanceledRevisionNumber),
                "Correction Number" => Some(CorrectionNumber),
                "Tariff Section Number" => Some(TariffSectionNumber),
                "Tariff Page Number" => Some(TariffPageNumber),
                "Tariff Rule Number" => Some(TariffRuleNumber),
                "Accounts Receivable Open Item" => Some(AccountsReceivableOpenItem),
                "Rental Agreement Number" => Some(RentalAgreementNumber),
                "Rejection Number" => Some(RejectionNumber),
                "Repetitive Cargo Shipment Number" => Some(RepetitiveCargoShipmentNumber),
                "Restricted Availability Authorization" => {
                    Some(RestrictedAvailabilityAuthorization)
                }
                "Restricted Availability Number" => Some(RestrictedAvailabilityNumber),
                "Rate code number" => Some(RateCodeNumber),
                "Rail Routing Code" => Some(RailRoutingCode),
                "Reel Number" => Some(ReelNumber),
                "Release Number" => Some(ReleaseNumber),
                "Related Case" => Some(RelatedCase),
                "Export Reference Number" => Some(ExportReferenceNumber),
                "Route Order Number-Domestic" => Some(RouteOrderNumberDomestic),
                "Regulatory Guideline Identifier" => Some(RegulatoryGuideline),
                "Route Order Number-Export" => Some(RouteOrderNumberExport),
                "Release invoice number for prior bill and hold" => {
                    Some(ReleaseInvoiceNumberForPriorBillAndHold)
                }
                "Rig Number" => Some(RigNumber),
                "Route Order Number-Emergency" => Some(RouteOrderNumberEmergency),
                "Rack Type Number" => Some(RackTypeNumber),
                "Reserve Assembly Line Feed Location" => {
                    Some(ReserveAssemblyLineFeedLocation)
                }
                "Role Identification Number" => Some(RoleIdentificationNumber),
                "Raw material supplier Dun & Bradstreet number" => Some(CodeRM),
                "Run Number" => Some(RunNumber),
                "Repetitive Booking Number" => Some(RepetitiveBookingNumber),
                "Repetitive Pattern Code" => Some(RepetitivePatternCode),
                "Relative Priority" => Some(RelativePriority),
                "Regulation Primary Number" => Some(RegulationPrimaryNumber),
                "Report Number" => Some(ReportNumber),
                "Purchase Requisition Number" => Some(PurchaseRequisitionNumber),
                "Payer's Financial Institution Transit Routing Number for Check, Draft or Wire Payments. Originating Depository Financial Institution Routing Number for ACH Transfers" => {
                    Some(CodeRR)
                }
                "Routing Request Control Number" => Some(RoutingRequestControlNumber),
                "Reconciliation Report Section Identification Code" => {
                    Some(ReconciliationReportSectionIdentificationCode)
                }
                "Returnable Container Serial Number" => {
                    Some(ReturnableContainerSerialNumber)
                }
                "Reservation Number" => Some(ReservationNumber),
                "Regulation Secondary Number" => Some(RegulationSecondaryNumber),
                "Payee's Financial Institution Transit Routing Number for Check, Draft or Wire Payments. Receiving Depository Financial Institution Transit Routing Number for ACH Transfers" => {
                    Some(CodeRT)
                }
                "Route Number" => Some(RouteNumber),
                "Receiving Number" => Some(ReceivingNumber),
                "Repetitive Waybill Code (Origin Carrier, Standard Point Location Code, Repetitive Waybill Code Number)" => {
                    Some(CodeRW)
                }
                "Reporting Week" => Some(ReportingWeek),
                "Resubmit number" => Some(ResubmitNumber),
                "Rebate Number" => Some(RebateNumber),
                "Returned Goods Authorization Number" => {
                    Some(ReturnedGoodsAuthorizationNumber)
                }
                "Special Approval" => Some(SpecialApproval),
                "Engineering Specification Number" => {
                    Some(EngineeringSpecificationNumber)
                }
                "Data Source" => Some(DataSource),
                "Specification Number" => Some(SpecificationNumber),
                "Shippers Bond Number" => Some(ShippersBondNumber),
                "Routing Instruction Number" => Some(RoutingInstructionNumber),
                "Stock Number" => Some(StockNumber),
                "Stack Train Identification" => Some(StackTrainIdentification),
                "Seal Off Number" => Some(SealOffNumber),
                "Seal On Number" => Some(SealOnNumber),
                "Salesperson" => Some(Salesperson),
                "Salary Step" => Some(SalaryStep),
                "Sales Region Number" => Some(SalesRegionNumber),
                "Surety Bond Number" => Some(SuretyBondNumber),
                "Shipper Car Order Number" => Some(ShipperCarOrderNumber),
                "Standard Carrier Alpha Code (SCAC)" => Some(CodeSCA),
                "Scale Number" => Some(ScaleNumber),
                "Subday Number" => Some(SubdayNumber),
                "School District Type Code" => Some(SchoolDistrictTypeCode),
                "Serial Number" => Some(SerialNumber),
                "Search Key" => Some(SearchKey),
                "Session" => Some(Session),
                "Ship From" => Some(ShipFrom),
                "Savings" => Some(Savings),
                "Sender Defined Clause" => Some(SenderDefinedClause),
                "Shelf Life Indicator" => Some(ShelfLifeIndicator),
                "Shipper's Identifying Number for Shipment (SID)" => Some(CodeSI),
                "Salvage Instruction Identifier" => Some(SalvageInstruction),
                "Set Number" => Some(SetNumber),
                "Service Change Number" => Some(ServiceChangeNumber),
                "Sales/Territory Code" => Some(SalesTerritoryCode),
                "Sales Office Number" => Some(SalesOfficeNumber),
                "Settlement Method Code" => Some(SettlementMethodCode),
                "State of Massachusetts Town Code" => Some(StateOfMassachusettsTownCode),
                "Seal Number" => Some(SealNumber),
                "SNOMED, Systematized Nomenclature of Medicine" => Some(CodeSNH),
                "US Customs & Border Protection Second Notify Party" => Some(CodeSNP),
                "State Non-Resident Violator Compact" => {
                    Some(StateNonResidentViolatorCompact)
                }
                "Shipper's Order (Invoice Number)" => Some(CodeSO),
                "Scan Line" => Some(ScanLine),
                "Standard Point Location Code (SPLC)" => Some(CodeSPL),
                "Theater Screen Number" => Some(TheaterScreenNumber),
                "Container Sequence Number" => Some(ContainerSequenceNumber),
                "Sales Responsibility" => Some(SalesResponsibility),
                "Split Shipment Number" => Some(SplitShipmentNumber),
                "School System Type Code" => Some(SchoolSystemTypeCode),
                "Store Number" => Some(StoreNumber),
                "Standard Transportation Commodity Code (STCC) Bridge Number" => {
                    Some(CodeSTB)
                }
                "Standard Transportation Commodity Code (STCC) Replacement Code" => {
                    Some(CodeSTR)
                }
                "Serviceability Standard Testing Reference" => {
                    Some(ServiceabilityStandardTestingReference)
                }
                "Special Processing Code" => Some(SpecialProcessingCode),
                "Title Reference" => Some(TitleReference),
                "Supervisory Union Code" => Some(SupervisoryUnionCode),
                "Spacing Unit Order Number" => Some(SpacingUnitOrderNumber),
                "Service Charge Number" => Some(ServiceChargeNumber),
                "Seller's Sale Number" => Some(SellersSaleNumber),
                "Service Interrupt Tracking Number" => {
                    Some(ServiceInterruptTrackingNumber)
                }
                "Social Security Number" => Some(SocialSecurityNumber),
                "Specification Revision" => Some(SpecificationRevision),
                "Dealer Type Identification" => Some(DealerTypeIdentification),
                "Tax Exchange Code" => Some(TaxExchangeCode),
                "Tax Form Code" => Some(TaxFormCode),
                "Tax Schedule Code" => Some(TaxScheduleCode),
                "Signal Code" => Some(SignalCode),
                "Trailer Use Agreements" => Some(TrailerUseAgreements),
                "Tax Filing" => Some(TaxFiling),
                "Affected Subsystem Code" => Some(AffectedSubsystemCode),
                "Description of Change Code" => Some(DescriptionOfChangeCode),
                "Documentation Affected Number" => Some(DocumentationAffectedNumber),
                "Telecommunication Circuit Supplemental ID" => {
                    Some(TelecommunicationCircuitSupplementalId)
                }
                "Trucker's Bill of Lading" => Some(TruckersBillOfLading),
                "Vendor Terms" => Some(VendorTerms),
                "Reason for Change" => Some(ReasonForChange),
                "Technical Documentation Type" => Some(TechnicalDocumentationType),
                "Federal Maritime Commission (FMC) Tariff Number" => Some(CodeTE),
                "Transfer Number" => Some(TransferNumber),
                "Time Failure" => Some(TimeFailure),
                "Transportation Control Number (TCN)" => Some(CodeTG),
                "Transportation Account Code (TAC)" => Some(CodeTH),
                "TIR Number" => Some(TirNumber),
                "Technical Information Package" => Some(TechnicalInformationPackage),
                "Federal Taxpayer's Identification Number" => {
                    Some(FederalTaxpayersIdentificationNumber)
                }
                "Tank Number" => Some(TankNumber),
                "Tax License Exemption" => Some(TaxLicenseExemption),
                "Travel Manifest (ACI or OTR)" => Some(CodeTM),
                "Transaction Reference Number" => Some(TransactionReferenceNumber),
                "Terminal Operator Number" => Some(TerminalOperatorNumber),
                "Type of Comment" => Some(TypeOfComment),
                "Test Specification Number" => Some(TestSpecificationNumber),
                "Transponder Number" => Some(TransponderNumber),
                "Tracer Action Request Number" => Some(TracerActionRequestNumber),
                "Government Transportation Request" => {
                    Some(GovernmentTransportationRequest)
                }
                "Tariff Number" => Some(TariffNumber),
                "Template Sequence Number" => Some(TemplateSequenceNumber),
                "Terminal Code" => Some(TerminalCode),
                "Trial Location Code" => Some(TrialLocationCode),
                "Line of Business" => Some(LineOfBusiness),
                "Tax Worksheet" => Some(TaxWorksheet),
                "Tax Exempt Number" => Some(TaxExemptNumber),
                "Policy Type" => Some(PolicyType),
                "Total Cycle Number" => Some(TotalCycleNumber),
                "Consolidator's Receipt Number" => Some(ConsolidatorsReceiptNumber),
                "Regional Account Number" => Some(RegionalAccountNumber),
                "Term" => Some(Term),
                "Unique Supplier Identification Number (USIN)" => Some(CodeU3),
                "Unpaid Installment Reference Number" => {
                    Some(UnpaidInstallmentReferenceNumber)
                }
                "Successor Account" => Some(SuccessorAccount),
                "Predecessor Account" => Some(PredecessorAccount),
                "Mortgage Backed Security (MBS) Loan Number" => Some(CodeU8),
                "Mortgage Backed Security (MBS) Pool Number" => Some(CodeU9),
                "Mortgage Number" => Some(MortgageNumber),
                "Unacceptable Source Purchaser ID" => Some(UnacceptableSourcePurchaserId),
                "Mortgage Insurance Indicator Number" => {
                    Some(MortgageInsuranceIndicatorNumber)
                }
                "EAN.UCC Bill of Lading Number (17 Digits)" => Some(CodeUCB),
                "EAN.UCC Master Bill of Lading Number (17 Digits)" => Some(CodeUCM),
                "Unacceptable Source DUNS Number" => Some(UnacceptableSourceDunsNumber),
                "Secondary Coverage Certificate Number" => {
                    Some(SecondaryCoverageCertificateNumber)
                }
                "Mortgage Insurance Company Number" => {
                    Some(MortgageInsuranceCompanyNumber)
                }
                "U.S. Government Transportation Control Number" => {
                    Some(USGovernmentTransportationControlNumber)
                }
                "Removal Number" => Some(RemovalNumber),
                "Previous Course Number" => Some(PreviousCourseNumber),
                "Unit Identification Code (UIC)" => Some(CodeUIC),
                "Current or Latest Course Number" => Some(CurrentOrLatestCourseNumber),
                "Equivalent Course Number at Requesting Institution" => {
                    Some(EquivalentCourseNumberAtRequestingInstitution)
                }
                "Cross-listed Course Number" => Some(CrossListedCourseNumber),
                "Quarter Quarter Section Number" => Some(QuarterQuarterSectionNumber),
                "United Nations Hazardous Classification Number" => {
                    Some(UnitedNationsHazardousClassificationNumber)
                }
                "Quarter Quarter Spot Number" => Some(QuarterQuarterSpotNumber),
                "Upstream Shipper Contract Number" => Some(UpstreamShipperContractNumber),
                "Section Number" => Some(SectionNumber),
                "Unit Relief Number" => Some(UnitReliefNumber),
                "Uniform Resource Locator" => Some(UniformResourceLocator),
                "Unit Report Period" => Some(UnitReportPeriod),
                "Unit Report Period ID" => Some(UnitReportPeriodId),
                "Unacceptable Source Supplier ID" => Some(UnacceptableSourceSupplierId),
                "Unit Train" => Some(UnitTrain),
                "Township Number" => Some(TownshipNumber),
                "Range Number" => Some(RangeNumber),
                "State Senate District" => Some(StateSenateDistrict),
                "State Assembly District" => Some(StateAssemblyDistrict),
                "Federal National Mortgage Association (Fannie Mae) Loan Number" => {
                    Some(CodeUY)
                }
                "State Legislative District" => Some(StateLegislativeDistrict),
                "Version" => Some(Version),
                "Volume Purchase Agreement Number" => Some(VolumePurchaseAgreementNumber),
                "Visa Type" => Some(VisaType),
                "Voyage Number" => Some(VoyageNumber),
                "State Department I-20 Form Number" => Some(StateDepartmentI20FormNumber),
                "State Department IAP-66 Form Number" => {
                    Some(StateDepartmentIap66FormNumber)
                }
                "North American Free Trade Agreement (NAFTA) Compliance Number" => {
                    Some(CodeV6)
                }
                "Judicial District" => Some(JudicialDistrict),
                "Institution Number" => Some(InstitutionNumber),
                "Subservicer" => Some(Subservicer),
                "Vessel Agent Number" => Some(VesselAgentNumber),
                "Veterans Administration Originator Identification" => {
                    Some(VeteransAdministrationOriginatorIdentification)
                }
                "Department of Veterans Affairs Acquisition Regulations (VAAR)" => {
                    Some(CodeVB)
                }
                "Vendor Contract Number" => Some(VendorContractNumber),
                "Volume Number" => Some(VolumeNumber),
                "Vendor Abbreviation Code" => Some(VendorAbbreviationCode),
                "Vendor Change Identification Code" => {
                    Some(VendorChangeIdentificationCode)
                }
                "Vendor Change Procedure Code" => Some(VendorChangeProcedureCode),
                "Vehicle Garaged State Code" => Some(VehicleGaragedStateCode),
                "County Legislative District" => Some(CountyLegislativeDistrict),
                "Pool Number" => Some(PoolNumber),
                "Investor Note Holder Identification" => {
                    Some(InvestorNoteHolderIdentification)
                }
                "Institution Note Holder Identification" => {
                    Some(InstitutionNoteHolderIdentification)
                }
                "Third Party Note Holder Identification" => {
                    Some(ThirdPartyNoteHolderIdentification)
                }
                "Ward" => Some(Ward),
                "Vendor Order Number" => Some(VendorOrderNumber),
                "Institution Loan Number" => Some(InstitutionLoanNumber),
                "Vendor Product Number" => Some(VendorProductNumber),
                "Related Contract Line Item Number" => {
                    Some(RelatedContractLineItemNumber)
                }
                "Vendor ID Number" => Some(VendorIdNumber),
                "Vendor Order Number Suffix" => Some(VendorOrderNumberSuffix),
                "Motor Vehicle ID Number" => Some(MotorVehicleIdNumber),
                "Preparer's Verification Number" => Some(PreparersVerificationNumber),
                "Voucher" => Some(Voucher),
                "Standard" => Some(Standard),
                "Value-Added Tax Registration Number (Europe)" => Some(CodeVX),
                "Link Sequence Number" => Some(LinkSequenceNumber),
                "Sponsor's Reference Number" => Some(SponsorsReferenceNumber),
                "Disposal Turn-In Document Number" => Some(DisposalTurnInDocumentNumber),
                "Weapon System Number" => Some(WeaponSystemNumber),
                "Manufacturing Directive Number" => Some(ManufacturingDirectiveNumber),
                "Procurement Request Number" => Some(ProcurementRequestNumber),
                "Inspector Identification Number" => Some(InspectorIdentificationNumber),
                "Federal Supply Schedule Number" => Some(FederalSupplyScheduleNumber),
                "Commercial and Government Entity (CAGE) Code" => Some(CodeW7),
                "Suffix" => Some(Suffix),
                "Special Packaging Instruction Number" => {
                    Some(SpecialPackagingInstructionNumber)
                }
                "Labor or Affiliation Identification" => {
                    Some(LaborOrAffiliationIdentification)
                }
                "American Petroleum Institute (API) Well" => Some(CodeWB),
                "Contract Option Number" => Some(ContractOptionNumber),
                "Work Candidate Sequence Number" => Some(WorkCandidateSequenceNumber),
                "Review Period Number" => Some(ReviewPeriodNumber),
                "Withdrawal Record" => Some(WithdrawalRecord),
                "Well Classification Code" => Some(WellClassificationCode),
                "Locally Assigned Control Number" => Some(LocallyAssignedControlNumber),
                "Vendor's Previous Job Number" => Some(VendorsPreviousJobNumber),
                "Master Reference (Link) Number" => Some(CodeWH),
                "Waiver" => Some(Waiver),
                "Pre-Award Survey" => Some(PreAwardSurvey),
                "Type of Science Code" => Some(TypeOfScienceCode),
                "Federal Supply Classification Code" => {
                    Some(FederalSupplyClassificationCode)
                }
                "Weight Agreement Number" => Some(WeightAgreementNumber),
                "Well Number" => Some(WellNumber),
                "Work Order Number" => Some(WorkOrderNumber),
                "Warehouse Pick Ticket Number" => Some(WarehousePickTicketNumber),
                "Interim Funding Organization Loan Number" => {
                    Some(InterimFundingOrganizationLoanNumber)
                }
                "Warehouse Receipt Number" => Some(WarehouseReceiptNumber),
                "Warehouse storage location number" => {
                    Some(WarehouseStorageLocationNumber)
                }
                "Broker's Reference Number" => Some(BrokersReferenceNumber),
                "Vessel" => Some(Vessel),
                "Dealer Identification" => Some(DealerIdentification),
                "Depository Trust Company Identification" => {
                    Some(DepositoryTrustCompanyIdentification)
                }
                "Distributor's Account Identification" => {
                    Some(DistributorsAccountIdentification)
                }
                "Waybill Number" => Some(WaybillNumber),
                "Distributor's Representative Identification" => {
                    Some(DistributorsRepresentativeIdentification)
                }
                "Debtor's Account" => Some(DebtorsAccount),
                "Provider Claim Number" => Some(ProviderClaimNumber),
                "Specification Class Number" => Some(SpecificationClassNumber),
                "Defect Code Number" => Some(DefectCodeNumber),
                "Clinical Laboratory Improvement Amendment Number" => {
                    Some(ClinicalLaboratoryImprovementAmendmentNumber)
                }
                "State Industrial Accident Provider Number" => {
                    Some(StateIndustrialAccidentProviderNumber)
                }
                "Original Voucher Number" => Some(OriginalVoucherNumber),
                "Batch Sequence Number" => Some(BatchSequenceNumber),
                "Secondary Suffix Code Indicator" => Some(SecondarySuffixCodeIndicator),
                "Internal Control Number" => Some(InternalControlNumber),
                "Substitute National Stock Number" => Some(SubstituteNationalStockNumber),
                "Substitute Manufacturer's Part Number" => {
                    Some(SubstituteManufacturersPartNumber)
                }
                "Cargo Control Number" => Some(CargoControlNumber),
                "Subsistence Identification Number" => {
                    Some(SubsistenceIdentificationNumber)
                }
                "Transportation Priority Number" => Some(TransportationPriorityNumber),
                "Government Bill of Lading Office Code" => {
                    Some(GovernmentBillOfLadingOfficeCode)
                }
                "Airline Ticket Number" => Some(AirlineTicketNumber),
                "Contract Auditor ID Number" => Some(ContractAuditorIdNumber),
                "Federal Home Loan Mortgage Corporation Loan Number" => {
                    Some(FederalHomeLoanMortgageCorporationLoanNumber)
                }
                "Federal Home Loan Mortgage Corporation Default/Foreclosure Specialist Number" => {
                    Some(
                        FederalHomeLoanMortgageCorporationDefaultForeclosureSpecialistNumber,
                    )
                }
                "Mortgagee Loan Number" => Some(MortgageeLoanNumber),
                "Insured's Loan Number" => Some(InsuredsLoanNumber),
                "Issuer Number" => Some(IssuerNumber),
                "Title XIX Identifier Number" => Some(TitleXixIdentifierNumber),
                "Sample Number" => Some(SampleNumber),
                "Previous Cargo Control Number" => Some(PreviousCargoControlNumber),
                "Pier Number" => Some(PierNumber),
                "Railroad Commission Record Number" => {
                    Some(RailroadCommissionRecordNumber)
                }
                "Gas Analysis Source Meter Number" => Some(GasAnalysisSourceMeterNumber),
                "Toxicology ID" => Some(ToxicologyId),
                "Universal Transverse Mercator - North" => {
                    Some(UniversalTransverseMercatorNorth)
                }
                "Universal Transverse Mercator - East" => {
                    Some(UniversalTransverseMercatorEast)
                }
                "Universal Transverse Mercator - Zone" => {
                    Some(UniversalTransverseMercatorZone)
                }
                "Rating Period" => Some(RatingPeriod),
                "Special Program Code" => Some(SpecialProgramCode),
                "Service Area Code" => Some(ServiceAreaCode),
                "Function Code" => Some(FunctionCode),
                "Object Code" => Some(ObjectCode),
                "Organization Code" => Some(OrganizationCode),
                "Subject Area Code" => Some(SubjectAreaCode),
                "Schedule Type Code" => Some(ScheduleTypeCode),
                "Alternating Schedule Identifier Code" => {
                    Some(AlternatingScheduleIdentifierCode)
                }
                "Other Unlisted Type of Reference Number" => {
                    Some(OtherUnlistedTypeOfReferenceNumber)
                }
                "Pharmacy Prescription Number" => Some(PharmacyPrescriptionNumber),
                "Debtor" => Some(Debtor),
                "Claim Administrator Claim Number" => Some(ClaimAdministratorClaimNumber),
                "Third-Party Administrator Claim Number" => {
                    Some(ThirdPartyAdministratorClaimNumber)
                }
                "Contract Holder Claim Number" => Some(ContractHolderClaimNumber),
                "Agency Claim Number" => Some(AgencyClaimNumber),
                "Delivery Trailer Manifest" => Some(DeliveryTrailerManifest),
                "Sort and Segregate" => Some(SortAndSegregate),
                "User ID" => Some(UserId),
                "Current Certificate Number" => Some(CurrentCertificateNumber),
                "Prior Certificate Number" => Some(PriorCertificateNumber),
                "Revision Number" => Some(RevisionNumber),
                "Tract" => Some(Tract),
                "Buyer Identification" => Some(BuyerIdentification),
                "Railroad Commission Oil Number" => Some(RailroadCommissionOilNumber),
                "Lessee Identification" => Some(LesseeIdentification),
                "Operator Assigned Unit Number" => Some(OperatorAssignedUnitNumber),
                "Refiner Identification" => Some(RefinerIdentification),
                "Revenue Source" => Some(RevenueSource),
                "Rent Payor Identification" => Some(RentPayorIdentification),
                "Allowance Recipient Identification" => {
                    Some(AllowanceRecipientIdentification)
                }
                "Resource Screening Reference" => Some(ResourceScreeningReference),
                "Receiver ID Qualifier" => Some(ReceiverIdQualifier),
                "Formation" => Some(Formation),
                "Selling Arrangement" => Some(SellingArrangement),
                "Minimum Royalty Payor Identification" => {
                    Some(MinimumRoyaltyPayorIdentification)
                }
                "Operator Lease Number" => Some(OperatorLeaseNumber),
                "Yard Position" => Some(YardPosition),
                "Reporter Identification" => Some(ReporterIdentification),
                "Participating Area" => Some(ParticipatingArea),
                "Engineering Change Proposal" => Some(EngineeringChangeProposal),
                "Geographic Score" => Some(GeographicScore),
                "Geographic Key" => Some(GeographicKey),
                "Geographic Index" => Some(GeographicIndex),
                "Safety of Ship Certificate" => Some(SafetyOfShipCertificate),
                "Safety of Radio Certificate" => Some(SafetyOfRadioCertificate),
                "Safety Equipment Certificate" => Some(SafetyEquipmentCertificate),
                "Civil Liabilities of Oil Certificate" => {
                    Some(CivilLiabilitiesOfOilCertificate)
                }
                "Load Line Certificate" => Some(LoadLineCertificate),
                "Derat Certificate" => Some(DeratCertificate),
                "Maritime Declaration of Health" => Some(MaritimeDeclarationOfHealth),
                "Federal Housing Administration Case Number" => {
                    Some(FederalHousingAdministrationCaseNumber)
                }
                "Veterans Affairs Case Number" => Some(VeteransAffairsCaseNumber),
                "Supplier" => Some(Supplier),
                "Ultimate Consignee" => Some(UltimateConsignee),
                "Connecting Carrier" => Some(ConnectingCarrier),
                "Family Member Identification" => Some(FamilyMemberIdentification),
                "Coal Authority Number" => Some(CoalAuthorityNumber),
                "Sales Representative Order Number" => {
                    Some(SalesRepresentativeOrderNumber)
                }
                "Carrier Assigned Reference Number" => {
                    Some(CarrierAssignedReferenceNumber)
                }
                "Reference Version Number" => Some(ReferenceVersionNumber),
                "Universal Railroad Revenue Waybill Identified Number (URRWIN)" => {
                    Some(CodeZJ)
                }
                "Duplicate Waybill in Route" => Some(DuplicateWaybillInRoute),
                "Duplicate Waybill Not in Route" => Some(DuplicateWaybillNotInRoute),
                "Manufacturer Number" => Some(ManufacturerNumber),
                "Agency Case Number" => Some(AgencyCaseNumber),
                "Makegood Commercial Line Number" => Some(MakegoodCommercialLineNumber),
                "Spouse Tie" => Some(SpouseTie),
                "Non-Spouse Tie" => Some(NonSpouseTie),
                "Supplier (Replacement)" => Some(CodeZR),
                "Software Application Number" => Some(SoftwareApplicationNumber),
                "Milling in Transit" => Some(MillingInTransit),
                "Zone, Track, Spot Number (ZTS)" => Some(CodeZTS),
                "Field" => Some(Field),
                "Block" => Some(Block),
                "Area" => Some(Area),
                "County Code" => Some(CountyCode),
                "Referenced Pattern Identification" => {
                    Some(ReferencedPatternIdentification)
                }
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for ReferenceIdentificationQualifier {
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
    type Value = ReferenceIdentificationQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Reference Identification Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ReferenceIdentificationQualifier::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Reference Identification Qualifier: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ReferenceIdentificationQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Reference Identification Qualifier: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ReferenceIdentificationQualifier {
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