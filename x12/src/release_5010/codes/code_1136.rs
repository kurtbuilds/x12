use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1136

See docs at <https://www.stedi.com/edi/x12-005010/element/1136>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodeCategory {
    ///00 - Employee Mobility
    EmployeeMobility,
    ///01 - Pre-existing Conditions
    PreExistingConditions,
    ///02 - Diagnosis
    Diagnosis,
    ///03 - Condition
    Condition,
    ///04 - Occurrence
    Occurrence,
    ///05 - Occurrence Span
    OccurrenceSpan,
    ///06 - Value
    Value,
    ///07 - Ambulance Certification
    AmbulanceCertification,
    ///08 - Chiropractic Certification
    ChiropracticCertification,
    ///09 - Durable Medical Equipment Certification
    DurableMedicalEquipmentCertification,
    ///10 - Enteral or Parenteral Therapy Certification
    EnteralOrParenteralTherapyCertification,
    ///11 - Oxygen Therapy Certification
    OxygenTherapyCertification,
    ///12 - Admitting Diagnosis
    AdmittingDiagnosis,
    ///13 - Principal Diagnosis
    PrincipalDiagnosis,
    ///14 - Pre-Existing Physical Condition
    PreExistingPhysicalCondition,
    ///15 - Pre-Existing Mental Condition
    PreExistingMentalCondition,
    ///16 - Routine Foot Care Class Finding
    RoutineFootCareClassFinding,
    ///17 - Systemic Condition for Routine Foot Care
    SystemicConditionForRoutineFootCare,
    ///18 - Co-op Advertising
    CoOpAdvertising,
    ///19 - Commercial Advertising
    CommercialAdvertising,
    ///20 - Specimen Kit Type Code
    SpecimenKitTypeCode,
    ///21 - Laboratory Test Condition Code
    LaboratoryTestConditionCode,
    ///22 - Automobile Loss
    AutomobileLoss,
    ///23 - Laboratory Results Identification Code
    LaboratoryResultsIdentificationCode,
    ///24 - Line of Business Code
    LineOfBusinessCode,
    ///25 - United States Department of Vital Statistics E-Code
    UnitedStatesDepartmentOfVitalStatisticsECode,
    ///26 - Employment Status Information
    EmploymentStatus,
    ///27 - Income
    Income,
    ///28 - Loan Information
    Loan,
    ///29 - Injury or Illness
    InjuryOrIllness,
    ///30 - Benefit Adjustment
    BenefitAdjustment,
    ///31 - Claimant
    Claimant,
    ///32 - Contractholder Branch
    ContractholderBranch,
    ///33 - Contractholder
    Contractholder,
    ///34 - Secondary Claim Administrator
    SecondaryClaimAdministrator,
    ///35 - Primary Claim Administrator
    PrimaryClaimAdministrator,
    ///36 - Reporting Agency
    ReportingAgency,
    ///37 - Process
    Process,
    ///38 - Hazardous Material
    HazardousMaterial,
    ///39 - Activity
    Activity,
    ///40 - Accident
    Accident,
    ///41 - Initial Treatment
    InitialTreatment,
    ///42 - Cause of Injury
    CauseOfInjury,
    ///43 - Part of Body
    PartOfBody,
    ///44 - Nature of Injury
    NatureOfInjury,
    ///45 - Source of Injury
    SourceOfInjury,
    ///46 - Job
    Job,
    ///47 - Loss Prevention
    LossPrevention,
    ///48 - Managed Care
    ManagedCare,
    ///49 - Risk Management
    RiskManagement,
    ///50 - Claim Handling
    ClaimHandling,
    ///51 - Event or Exposure
    EventOrExposure,
    ///52 - Equipment or Materials or Chemicals
    EquipmentOrMaterialsOrChemicals,
    ///53 - Coverage
    Coverage,
    ///54 - Overbite
    Overbite,
    ///55 - Overjet
    Overjet,
    ///56 - Profile
    Profile,
    ///57 - Crossbite
    Crossbite,
    ///58 - Arch Asymmetry
    ArchAsymmetry,
    ///59 - Dentition Midline
    DentitionMidline,
    ///60 - Crowding
    Crowding,
    ///61 - Molars
    Molars,
    ///62 - Cuspids
    Cuspids,
    ///63 - Interviewee
    Interviewee,
    ///64 - Verification of Deposit
    VerificationOfDeposit,
    ///65 - Verification of Mortgage
    VerificationOfMortgage,
    ///66 - Verification of Income or Employment or Both
    VerificationOfIncomeOrEmploymentOrBoth,
    ///67 - Verification of Rent
    VerificationOfRent,
    ///68 - Verification of Loan or Installment Debt or Both
    VerificationOfLoanOrInstallmentDebtOrBoth,
    ///69 - Anti-fungal Therapy
    AntiFungalTherapy,
    ///70 - Hospice
    Hospice,
    ///71 - Primary Diagnosis
    PrimaryDiagnosis,
    ///72 - Secondary Diagnosis
    SecondaryDiagnosis,
    ///73 - Tertiary Diagnosis
    TertiaryDiagnosis,
    ///74 - Procedure Code
    ProcedureCode,
    ///75 - Functional Limitations
    FunctionalLimitations,
    ///76 - Activities Permitted
    ActivitiesPermitted,
    ///77 - Mental Status
    MentalStatus,
    ///78 - Manner Property Title Held
    MannerPropertyTitleHeld,
    ///79 - Property Improvements
    PropertyImprovements,
    ///80 - Complete Appraisal
    CompleteAppraisal,
    ///81 - Limited Appraisal
    LimitedAppraisal,
    ///82 - Restricted Appraisal Report Limiting Conditions
    RestrictedAppraisalReportLimitingConditions,
    ///83 - Route of Administration
    RouteOfAdministration,
    ///84 - Borrower Information
    Borrower,
    ///85 - Contract Information
    Contract,
    ///86 - Fannie Mae (Federal National Mortgage Association)
    Code86,
    ///87 - Freddie Mac (Federal Home Loan Mortgage Corporation)
    Code87,
    ///88 - Deductible
    Deductible,
    ///89 - Advertising Copy
    AdvertisingCopy,
    ///90 - Private Remarks
    PrivateRemarks,
    ///91 - Compensation Notes
    CompensationNotes,
    ///92 - Open House Notes
    OpenHouseNotes,
    ///93 - Tour Notes
    TourNotes,
    ///94 - Terms of Sale
    TermsOfSale,
    ///95 - Restrictions
    Restrictions,
    ///96 - Disclosures
    Disclosures,
    ///97 - Exceptions
    Exceptions,
    ///98 - Inclusions
    Inclusions,
    ///99 - Lease Type
    LeaseType,
    ///A0 - Contracting District Type
    ContractingDistrictType,
    ///A1 - Mortgage Record Change
    MortgageRecordChange,
    ///A2 - Mortgage Insurance Termination
    MortgageInsuranceTermination,
    ///A3 - Mortgage Insurance Cancellation
    MortgageInsuranceCancellation,
    ///A4 - Mortgage Servicing Transfer
    MortgageServicingTransfer,
    ///A5 - Appraisal
    Appraisal,
    ///A6 - State License Disciplinary Action
    StateLicenseDisciplinaryAction,
    ///A7 - Source of Data
    SourceOfData,
    ///A8 - Endorsement
    Endorsement,
    ///A9 - Notification
    Notification,
    ///AA - All
    All,
    ///AB - Agent's Questions
    AgentsQuestions,
    ///AC - Agent's Share
    AgentsShare,
    ///AD - Benefits
    Benefits,
    ///AE - Contact or Reference Information
    ContactOrReference,
    ///AF - Cost Basis
    CostBasis,
    ///AG - Driving Infractions
    DrivingInfractions,
    ///AH - Excess Dividend Use
    ExcessDividendUse,
    ///AI - Home Health Aide
    HomeHealthAide,
    ///AJ - Existing Coverage Information
    ExistingCoverage,
    ///AK - Hospitalization
    Hospitalization,
    ///AL - Activity Limitations
    ActivityLimitations,
    ///AM - Juvenile Information
    Juvenile,
    ///AN - Occupation Information
    Occupation,
    ///AO - Personal Finance and Business Information
    PersonalFinanceAndBusiness,
    ///AP - Appearance
    Appearance,
    ///AQ - Rating Information
    Rating,
    ///AR - Arrest
    Arrest,
    ///AS - Replaced Amount
    ReplacedAmount,
    ///AT - Authority
    Authority,
    ///AU - Automated Underwriting Information
    AutomatedUnderwriting,
    ///AV - Aviation
    Aviation,
    ///AW - Surgery
    Surgery,
    ///AX - Travel Information
    Travel,
    ///AY - Age Remark
    AgeRemark,
    ///AZ - Property Remark
    PropertyRemark,
    ///B1 - Audit Data
    AuditData,
    ///B2 - Declaration Sheet Indicator
    DeclarationSheetIndicator,
    ///B3 - Servicing Data
    ServicingData,
    ///B4 - Single Family
    SingleFamily,
    ///B5 - Multifamily
    Multifamily,
    ///B6 - Payment Handling
    PaymentHandling,
    ///B7 - Ginnie Mae 1
    GinnieMae1,
    ///B8 - Ginnie Mae 2
    GinnieMae2,
    ///B9 - Ginnie Mae 2 Custom
    GinnieMae2Custom,
    ///BA - Bankruptcy
    Bankruptcy,
    ///BB - Business Beneficiary
    BusinessBeneficiary,
    ///BC - Building Condition
    BuildingCondition,
    ///BD - Buydown
    Buydown,
    ///BE - Beneficiary
    Beneficiary,
    ///BF - Tax Agency Parcel Identifier
    TaxAgencyParcel,
    ///BG - Historical Performance
    HistoricalPerformance,
    ///BH - Product Rules
    ProductRules,
    ///BI - Commercial Property
    CommercialProperty,
    ///BJ - Unimproved Land
    UnimprovedLand,
    ///BK - Banking
    Banking,
    ///BL - New Contract
    NewContract,
    ///BM - Original Contract
    OriginalContract,
    ///BN - Access
    Access,
    ///BO - Bond
    Bond,
    ///BP - Bankruptcy Petition
    BankruptcyPetition,
    ///BQ - Agent Sales Trend
    AgentSalesTrend,
    ///BR - Broker's Price Opinion
    BrokersPriceOpinion,
    ///BS - Bankruptcy Statement of Financial Affairs
    BankruptcyStatementOfFinancialAffairs,
    ///BT - Billings Trend
    BillingsTrend,
    ///BU - Assets
    Assets,
    ///BV - Cash Flow
    CashFlow,
    ///BW - Competition
    Competition,
    ///BX - Credit Line
    CreditLine,
    ///BY - Creditors Arrangement
    CreditorsArrangement,
    ///BZ - Creditors Meeting
    CreditorsMeeting,
    ///C1 - Depreciation Conditions
    DepreciationConditions,
    ///C2 - Adverse Environment Conditions
    AdverseEnvironmentConditions,
    ///C3 - Miscellaneous Adverse Conditions
    MiscellaneousAdverseConditions,
    ///C4 - Site Conditions
    SiteConditions,
    ///C5 - Subject Property Conditions
    SubjectPropertyConditions,
    ///C6 - Board of Directors
    BoardOfDirectors,
    ///C7 - Reserve
    Reserve,
    ///C8 - Payment
    Payment,
    ///C9 - Comorbidity
    Comorbidity,
    ///CA - Citizenship
    Citizenship,
    ///CB - Continuing Education
    ContinuingEducation,
    ///CC - Compensation Calculation
    CompensationCalculation,
    ///CD - Cause of Death
    CauseOfDeath,
    ///CE - Condominium
    Condominium,
    ///CF - Cooperative
    Cooperative,
    ///CG - Conviction
    Conviction,
    ///CH - Direct Sales Trend
    DirectSalesTrend,
    ///CI - Export Trend
    ExportTrend,
    ///CJ - Financial Embarrassment
    FinancialEmbarrassment,
    ///CK - Indebtedness
    Indebtedness,
    ///CL - Cancellation
    Cancellation,
    ///CM - Claim Amounts
    ClaimAmounts,
    ///CN - Comparison
    Comparison,
    ///CO - County
    County,
    ///CP - Complications
    Complications,
    ///CQ - Initial Capital
    InitialCapital,
    ///CR - Current Ratio
    CurrentRatio,
    ///CS - Common Stock
    CommonStock,
    ///CT - Commission Trend
    CommissionTrend,
    ///CU - Stockholders
    Stockholders,
    ///CV - Damage
    Damage,
    ///CW - Working Capital
    WorkingCapital,
    ///CX - Compensation Allocation
    CompensationAllocation,
    ///CY - Dividend Use
    DividendUse,
    ///CZ - Excess Premium Use
    ExcessPremiumUse,
    ///D1 - Unpaid Invoices
    UnpaidInvoices,
    ///D2 - Withdrawals
    Withdrawals,
    ///D3 - Imports
    Imports,
    ///D4 - Placed for Collection
    PlacedForCollection,
    ///DA - Drug Adjudication Information
    DrugAdjudication,
    ///DB - Liquidation Proceedings
    LiquidationProceedings,
    ///DC - Location
    Location,
    ///DD - Discharge Diagnosis
    DischargeDiagnosis,
    ///DE - Departmental
    Departmental,
    ///DF - Profit Margin
    ProfitMargin,
    ///DG - Proposal
    Proposal,
    ///DH - Receivership
    Receivership,
    ///DI - Driver Identification Information
    DriverIdentification,
    ///DJ - Provider Characteristics and Resources
    ProviderCharacteristicsAndResources,
    ///DK - Secondary Source of Injury
    SecondarySourceOfInjury,
    ///DL - Petitiions
    Petitiions,
    ///DM - Registered Charges
    RegisteredCharges,
    ///DN - Criminal Proceedings
    CriminalProceedings,
    ///DO - Historical Criminal Proceedings
    HistoricalCriminalProceedings,
    ///DP - Directions to Property
    DirectionsToProperty,
    ///DR - Driving
    Driving,
    ///DV - Driver Record Information
    DriverRecord,
    ///E1 - Spectacle Lenses
    SpectacleLenses,
    ///E2 - Contact Lenses
    ContactLenses,
    ///E3 - Spectacle Frames
    SpectacleFrames,
    ///E4 - Employment
    Employment,
    ///E5 - Examiner's Comments
    ExaminersComments,
    ///EB - Intercompany Relations
    IntercompanyRelations,
    ///EC - Judgments
    Judgments,
    ///ED - Liens
    Liens,
    ///EE - Operating Surplus Trend
    OperatingSurplusTrend,
    ///EF - Participating Interest
    ParticipatingInterest,
    ///EG - Protested Bills
    ProtestedBills,
    ///EH - Subcontracting Details
    SubcontractingDetails,
    ///EI - Suits
    Suits,
    ///EJ - Uniform Commercial Code (UCC) Filings
    CodeEJ,
    ///EK - Detrimental Legal Filings
    DetrimentalLegalFilings,
    ///EL - Customer Details
    CustomerDetails,
    ///EM - Supplier Detail
    SupplierDetail,
    ///ER - Employee Relocation
    EmployeeRelocation,
    ///ET - Education or Training
    EducationOrTraining,
    ///FA - Financial
    Financial,
    ///FC - Family Coverage
    FamilyCoverage,
    ///FH - Family History
    FamilyHistory,
    ///FI - Financing
    Financing,
    ///FL - Flood Determination
    FloodDetermination,
    ///FP - Franchise Tax Payments
    FranchiseTaxPayments,
    ///FR - Financial Remarks
    FinancialRemarks,
    ///FT - Foreign Travel
    ForeignTravel,
    ///GD - Demonstrations
    Demonstrations,
    ///GS - Shelf Format
    ShelfFormat,
    ///GU - Guarantees
    Guarantees,
    ///HA - Fixed
    Fixed,
    ///HB - Adjustable
    Adjustable,
    ///HC - Rate Adjustment
    RateAdjustment,
    ///HD - Payment Adjustment
    PaymentAdjustment,
    ///HE - Life of Loan
    LifeOfLoan,
    ///HF - Periodic Interest Rate
    PeriodicInterestRate,
    ///HG - Principal and Interest
    PrincipalAndInterest,
    ///HH - Health or Medical
    HealthOrMedical,
    ///HI - Late Charge
    LateCharge,
    ///HJ - Default Note Holder's Cost
    DefaultNoteHoldersCost,
    ///HK - Prepayment
    Prepayment,
    ///HL - Limited Payment
    LimitedPayment,
    ///HM - Rate Lookback
    RateLookback,
    ///HN - Payment Lookback
    PaymentLookback,
    ///HO - Index
    Index,
    ///HP - Mortgage Margin
    MortgageMargin,
    ///HQ - Single Family 2-4 Units
    SingleFamily24Units,
    ///HR - Amortization
    Amortization,
    ///HS - Rate Conversion
    RateConversion,
    ///HT - Interest Only
    InterestOnly,
    ///HU - Premium Audit Key Question
    PremiumAuditKeyQuestion,
    ///HY - History
    History,
    ///HZ - Hazardous Sports
    HazardousSports,
    ///IC - Issued Capital
    IssuedCapital,
    ///ID - Identification
    Identification,
    ///IH - Insurance History or Other Coverage
    InsuranceHistoryOrOtherCoverage,
    ///IM - Impairment
    Impairment,
    ///IN - Insurance
    Insurance,
    ///LA - License Revocation
    LicenseRevocation,
    ///LC - Location Status
    LocationStatus,
    ///LE - Level Remarks
    LevelRemarks,
    ///LI - Liability Status
    LiabilityStatus,
    ///LL - Local Language Description
    LocalLanguageDescription,
    ///LR - Listing Remarks
    ListingRemarks,
    ///LS - Life Style
    LifeStyle,
    ///LT - Legal Type
    LegalType,
    ///LZ - Loss Trend
    LossTrend,
    ///MA - Marital Status
    MaritalStatus,
    ///MI - Miscellaneous
    Miscellaneous,
    ///ML - Multiple Listing Service
    MultipleListingService,
    ///MN - Management
    Management,
    ///MO - Modification
    Modification,
    ///MP - Medication or Prescription
    MedicationOrPrescription,
    ///MS - Medical Social Worker
    MedicalSocialWorker,
    ///MT - Military Status
    MilitaryStatus,
    ///NC - Nominal Capital
    NominalCapital,
    ///NL - New Licensed Staff
    NewLicensedStaff,
    ///NS - Nature of Suit
    NatureOfSuit,
    ///NW - Not Work Related
    NotWorkRelated,
    ///OA - Owner Pays Notes
    OwnerPaysNotes,
    ///OC - Occupation Class
    OccupationClass,
    ///OF - Outside Financing
    OutsideFinancing,
    ///OI - Other Investor
    OtherInvestor,
    ///ON - Operations Trend
    OperationsTrend,
    ///OP - Operations
    Operations,
    ///OT - Occupational Therapy
    OccupationalTherapy,
    ///OU - Operations Outlook
    OperationsOutlook,
    ///PA - Performance
    Performance,
    ///PB - Profitability
    Profitability,
    ///PC - Paid in Capital
    PaidInCapital,
    ///PD - Public Records
    PublicRecords,
    ///PE - Penalty
    Penalty,
    ///PF - Profit Trend
    ProfitTrend,
    ///PG - Possession Notes
    PossessionNotes,
    ///PI - Photo Instructions
    PhotoInstructions,
    ///PJ - Patient Subjective Complaints
    PatientSubjectiveComplaints,
    ///PN - Parking Notes
    ParkingNotes,
    ///PO - Profit Outlook
    ProfitOutlook,
    ///PR - Property
    Property,
    ///PS - Preferred Stock
    PreferredStock,
    ///PT - Physical Therapy
    PhysicalTherapy,
    ///PX - Physician Examination Results
    PhysicianExaminationResults,
    ///R1 - Reason for Weight Loss
    ReasonForWeightLoss,
    ///R2 - Association of American Railroads Special Proper Shipping Name Flag
    AssociationOfAmericanRailroadsSpecialProperShippingNameFlag,
    ///R3 - Association of American Railroads Intermodal Indicator
    AssociationOfAmericanRailroadsIntermodalIndicator,
    ///R4 - Association of American Railroads U.S. to Canada Flag
    AssociationOfAmericanRailroadsUSToCanadaFlag,
    ///R5 - Residential Status
    ResidentialStatus,
    ///RC - Revocation
    Revocation,
    ///RE - Recovery
    Recovery,
    ///RI - Real Estate Property Information
    RealEstateProperty,
    ///RL - Radio License Application
    RadioLicenseApplication,
    ///RM - Remedy
    Remedy,
    ///RN - Related Entities
    RelatedEntities,
    ///RP - Retirement Plan Type
    RetirementPlanType,
    ///RR - Reinstatement
    Reinstatement,
    ///RS - Reason Last Seen
    ReasonLastSeen,
    ///RT - Registration Type
    RegistrationType,
    ///RU - Results
    Results,
    ///RV - Revenue Trend
    RevenueTrend,
    ///S1 - Investment Trend
    InvestmentTrend,
    ///S2 - Royalty Trend
    RoyaltyTrend,
    ///S3 - Purchases Trend
    PurchasesTrend,
    ///S4 - Labor Infraction
    LaborInfraction,
    ///S5 - Debentures
    Debentures,
    ///SA - Source Fund
    SourceFund,
    ///SD - Starting Details
    StartingDetails,
    ///SE - Summary and Evaluation
    SummaryAndEvaluation,
    ///SI - Showing Instructions
    ShowingInstructions,
    ///SJ - Suits, Judgments & Liens
    CodeSJ,
    ///SL - Supplement Note or Line
    SupplementNoteOrLine,
    ///SN - Skilled Nursing
    SkilledNursing,
    ///SP - Statement Preparation
    StatementPreparation,
    ///SR - Sales Trend
    SalesTrend,
    ///SS - Suspension
    Suspension,
    ///ST - Speech Therapy
    SpeechTherapy,
    ///SU - Substance Use
    SubstanceUse,
    ///SW - Reported Statement of Witness
    ReportedStatementOfWitness,
    ///SZ - Size
    Size,
    ///TB - Tobacco
    Tobacco,
    ///TE - Tests
    Tests,
    ///TF - Target Fund
    TargetFund,
    ///TH - Therapy
    Therapy,
    ///TI - Action
    Action,
    ///TM - Terms
    Terms,
    ///TN - Trend
    Trend,
    ///TP - Tenant Pays Notes
    TenantPaysNotes,
    ///TR - Treatment
    Treatment,
    ///TW - Two to Four Units
    TwoToFourUnits,
    ///TX - Tax Service
    TaxService,
    ///UA - Production Capacity
    ProductionCapacity,
    ///UB - Actual Production
    ActualProduction,
    ///UC - Branch Trend
    BranchTrend,
    ///UD - Retail Locations
    RetailLocations,
    ///UE - Net Profit
    NetProfit,
    ///UF - Ordinary Profit
    OrdinaryProfit,
    ///UG - Declared Profit to Local Tax Office
    DeclaredProfitToLocalTaxOffice,
    ///UH - Market Trend
    MarketTrend,
    ///UI - Pre Tax Profit
    PreTaxProfit,
    ///UJ - Net Worth
    NetWorth,
    ///UK - Debt to Equity
    DebtToEquity,
    ///UL - Equity Return
    EquityReturn,
    ///UM - Stability
    Stability,
    ///UN - Efficiency
    Efficiency,
    ///UO - Outlook
    Outlook,
    ///UP - Update
    Update,
    ///UQ - Corporate Registration
    CorporateRegistration,
    ///VA - Voter Registration Application
    VoterRegistrationApplication,
    ///VD - Voter Registration Application Disposition
    VoterRegistrationApplicationDisposition,
    ///VO - Violation
    Violation,
    ///WA - Warning
    Warning,
    ///WB - Prognosis
    Prognosis,
    ///WD - Treatment Plan
    TreatmentPlan,
    ///WE - Work Restrictions
    WorkRestrictions,
    ///WF - Witness Statement
    WitnessStatement,
    ///WG - Conditions Affecting Total Employees and Hours
    ConditionsAffectingTotalEmployeesAndHours,
    ///WH - Injury Work Related
    InjuryWorkRelated,
    ///WI - Illness Work Related
    IllnessWorkRelated,
    ///WK - Controvert Reason
    ControvertReason,
    ///WL - Supervisor's Comments
    SupervisorsComments,
    ///WM - Willful Misconduct
    WillfulMisconduct,
    ///WN - Supervisor's Exception
    SupervisorsException,
    ///WO - Claim Related Work Assignment Changes
    ClaimRelatedWorkAssignmentChanges,
    ///WP - 30 Day Delay Reason
    CodeWP,
    ///WQ - Employee Comment
    EmployeeComment,
    ///WR - Employee Comment Not Provided Reason
    EmployeeCommentNotProvidedReason,
    ///WS - Medical Records Not Attached Reason
    MedicalRecordsNotAttachedReason,
    ///WT - Work Exposures and Duration
    WorkExposuresAndDuration,
    ///X1 - Letter of Credit Overdrawn
    LetterOfCreditOverdrawn,
    ///X2 - Cargo Receipt Not Signed
    CargoReceiptNotSigned,
    ///X3 - Customs Statement Missing from Invoice
    CustomsStatementMissingFromInvoice,
    ///X4 - Purchase Order Not on Letter of Credit (Except Masters)
    CodeX4,
    ///X5 - Reduced Draft
    ReducedDraft,
    ///X6 - Time Drafts
    TimeDrafts,
    ///X7 - Demand for Payment
    DemandForPayment,
    ///X8 - Early Presentation of Documents
    EarlyPresentationOfDocuments,
    ///YR - Physician - Patient Report Inconsistency
    PhysicianPatientReportInconsistency,
    ///YT - Physician Test Results
    PhysicianTestResults,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl CodeCategory {
    pub fn code(&self) -> &str {
        {
            use CodeCategory::*;
            match self {
                EmployeeMobility => "00",
                PreExistingConditions => "01",
                Diagnosis => "02",
                Condition => "03",
                Occurrence => "04",
                OccurrenceSpan => "05",
                Value => "06",
                AmbulanceCertification => "07",
                ChiropracticCertification => "08",
                DurableMedicalEquipmentCertification => "09",
                EnteralOrParenteralTherapyCertification => "10",
                OxygenTherapyCertification => "11",
                AdmittingDiagnosis => "12",
                PrincipalDiagnosis => "13",
                PreExistingPhysicalCondition => "14",
                PreExistingMentalCondition => "15",
                RoutineFootCareClassFinding => "16",
                SystemicConditionForRoutineFootCare => "17",
                CoOpAdvertising => "18",
                CommercialAdvertising => "19",
                SpecimenKitTypeCode => "20",
                LaboratoryTestConditionCode => "21",
                AutomobileLoss => "22",
                LaboratoryResultsIdentificationCode => "23",
                LineOfBusinessCode => "24",
                UnitedStatesDepartmentOfVitalStatisticsECode => "25",
                EmploymentStatus => "26",
                Income => "27",
                Loan => "28",
                InjuryOrIllness => "29",
                BenefitAdjustment => "30",
                Claimant => "31",
                ContractholderBranch => "32",
                Contractholder => "33",
                SecondaryClaimAdministrator => "34",
                PrimaryClaimAdministrator => "35",
                ReportingAgency => "36",
                Process => "37",
                HazardousMaterial => "38",
                Activity => "39",
                Accident => "40",
                InitialTreatment => "41",
                CauseOfInjury => "42",
                PartOfBody => "43",
                NatureOfInjury => "44",
                SourceOfInjury => "45",
                Job => "46",
                LossPrevention => "47",
                ManagedCare => "48",
                RiskManagement => "49",
                ClaimHandling => "50",
                EventOrExposure => "51",
                EquipmentOrMaterialsOrChemicals => "52",
                Coverage => "53",
                Overbite => "54",
                Overjet => "55",
                Profile => "56",
                Crossbite => "57",
                ArchAsymmetry => "58",
                DentitionMidline => "59",
                Crowding => "60",
                Molars => "61",
                Cuspids => "62",
                Interviewee => "63",
                VerificationOfDeposit => "64",
                VerificationOfMortgage => "65",
                VerificationOfIncomeOrEmploymentOrBoth => "66",
                VerificationOfRent => "67",
                VerificationOfLoanOrInstallmentDebtOrBoth => "68",
                AntiFungalTherapy => "69",
                Hospice => "70",
                PrimaryDiagnosis => "71",
                SecondaryDiagnosis => "72",
                TertiaryDiagnosis => "73",
                ProcedureCode => "74",
                FunctionalLimitations => "75",
                ActivitiesPermitted => "76",
                MentalStatus => "77",
                MannerPropertyTitleHeld => "78",
                PropertyImprovements => "79",
                CompleteAppraisal => "80",
                LimitedAppraisal => "81",
                RestrictedAppraisalReportLimitingConditions => "82",
                RouteOfAdministration => "83",
                Borrower => "84",
                Contract => "85",
                Code86 => "86",
                Code87 => "87",
                Deductible => "88",
                AdvertisingCopy => "89",
                PrivateRemarks => "90",
                CompensationNotes => "91",
                OpenHouseNotes => "92",
                TourNotes => "93",
                TermsOfSale => "94",
                Restrictions => "95",
                Disclosures => "96",
                Exceptions => "97",
                Inclusions => "98",
                LeaseType => "99",
                ContractingDistrictType => "A0",
                MortgageRecordChange => "A1",
                MortgageInsuranceTermination => "A2",
                MortgageInsuranceCancellation => "A3",
                MortgageServicingTransfer => "A4",
                Appraisal => "A5",
                StateLicenseDisciplinaryAction => "A6",
                SourceOfData => "A7",
                Endorsement => "A8",
                Notification => "A9",
                All => "AA",
                AgentsQuestions => "AB",
                AgentsShare => "AC",
                Benefits => "AD",
                ContactOrReference => "AE",
                CostBasis => "AF",
                DrivingInfractions => "AG",
                ExcessDividendUse => "AH",
                HomeHealthAide => "AI",
                ExistingCoverage => "AJ",
                Hospitalization => "AK",
                ActivityLimitations => "AL",
                Juvenile => "AM",
                Occupation => "AN",
                PersonalFinanceAndBusiness => "AO",
                Appearance => "AP",
                Rating => "AQ",
                Arrest => "AR",
                ReplacedAmount => "AS",
                Authority => "AT",
                AutomatedUnderwriting => "AU",
                Aviation => "AV",
                Surgery => "AW",
                Travel => "AX",
                AgeRemark => "AY",
                PropertyRemark => "AZ",
                AuditData => "B1",
                DeclarationSheetIndicator => "B2",
                ServicingData => "B3",
                SingleFamily => "B4",
                Multifamily => "B5",
                PaymentHandling => "B6",
                GinnieMae1 => "B7",
                GinnieMae2 => "B8",
                GinnieMae2Custom => "B9",
                Bankruptcy => "BA",
                BusinessBeneficiary => "BB",
                BuildingCondition => "BC",
                Buydown => "BD",
                Beneficiary => "BE",
                TaxAgencyParcel => "BF",
                HistoricalPerformance => "BG",
                ProductRules => "BH",
                CommercialProperty => "BI",
                UnimprovedLand => "BJ",
                Banking => "BK",
                NewContract => "BL",
                OriginalContract => "BM",
                Access => "BN",
                Bond => "BO",
                BankruptcyPetition => "BP",
                AgentSalesTrend => "BQ",
                BrokersPriceOpinion => "BR",
                BankruptcyStatementOfFinancialAffairs => "BS",
                BillingsTrend => "BT",
                Assets => "BU",
                CashFlow => "BV",
                Competition => "BW",
                CreditLine => "BX",
                CreditorsArrangement => "BY",
                CreditorsMeeting => "BZ",
                DepreciationConditions => "C1",
                AdverseEnvironmentConditions => "C2",
                MiscellaneousAdverseConditions => "C3",
                SiteConditions => "C4",
                SubjectPropertyConditions => "C5",
                BoardOfDirectors => "C6",
                Reserve => "C7",
                Payment => "C8",
                Comorbidity => "C9",
                Citizenship => "CA",
                ContinuingEducation => "CB",
                CompensationCalculation => "CC",
                CauseOfDeath => "CD",
                Condominium => "CE",
                Cooperative => "CF",
                Conviction => "CG",
                DirectSalesTrend => "CH",
                ExportTrend => "CI",
                FinancialEmbarrassment => "CJ",
                Indebtedness => "CK",
                Cancellation => "CL",
                ClaimAmounts => "CM",
                Comparison => "CN",
                County => "CO",
                Complications => "CP",
                InitialCapital => "CQ",
                CurrentRatio => "CR",
                CommonStock => "CS",
                CommissionTrend => "CT",
                Stockholders => "CU",
                Damage => "CV",
                WorkingCapital => "CW",
                CompensationAllocation => "CX",
                DividendUse => "CY",
                ExcessPremiumUse => "CZ",
                UnpaidInvoices => "D1",
                Withdrawals => "D2",
                Imports => "D3",
                PlacedForCollection => "D4",
                DrugAdjudication => "DA",
                LiquidationProceedings => "DB",
                Location => "DC",
                DischargeDiagnosis => "DD",
                Departmental => "DE",
                ProfitMargin => "DF",
                Proposal => "DG",
                Receivership => "DH",
                DriverIdentification => "DI",
                ProviderCharacteristicsAndResources => "DJ",
                SecondarySourceOfInjury => "DK",
                Petitiions => "DL",
                RegisteredCharges => "DM",
                CriminalProceedings => "DN",
                HistoricalCriminalProceedings => "DO",
                DirectionsToProperty => "DP",
                Driving => "DR",
                DriverRecord => "DV",
                SpectacleLenses => "E1",
                ContactLenses => "E2",
                SpectacleFrames => "E3",
                Employment => "E4",
                ExaminersComments => "E5",
                IntercompanyRelations => "EB",
                Judgments => "EC",
                Liens => "ED",
                OperatingSurplusTrend => "EE",
                ParticipatingInterest => "EF",
                ProtestedBills => "EG",
                SubcontractingDetails => "EH",
                Suits => "EI",
                CodeEJ => "EJ",
                DetrimentalLegalFilings => "EK",
                CustomerDetails => "EL",
                SupplierDetail => "EM",
                EmployeeRelocation => "ER",
                EducationOrTraining => "ET",
                Financial => "FA",
                FamilyCoverage => "FC",
                FamilyHistory => "FH",
                Financing => "FI",
                FloodDetermination => "FL",
                FranchiseTaxPayments => "FP",
                FinancialRemarks => "FR",
                ForeignTravel => "FT",
                Demonstrations => "GD",
                ShelfFormat => "GS",
                Guarantees => "GU",
                Fixed => "HA",
                Adjustable => "HB",
                RateAdjustment => "HC",
                PaymentAdjustment => "HD",
                LifeOfLoan => "HE",
                PeriodicInterestRate => "HF",
                PrincipalAndInterest => "HG",
                HealthOrMedical => "HH",
                LateCharge => "HI",
                DefaultNoteHoldersCost => "HJ",
                Prepayment => "HK",
                LimitedPayment => "HL",
                RateLookback => "HM",
                PaymentLookback => "HN",
                Index => "HO",
                MortgageMargin => "HP",
                SingleFamily24Units => "HQ",
                Amortization => "HR",
                RateConversion => "HS",
                InterestOnly => "HT",
                PremiumAuditKeyQuestion => "HU",
                History => "HY",
                HazardousSports => "HZ",
                IssuedCapital => "IC",
                Identification => "ID",
                InsuranceHistoryOrOtherCoverage => "IH",
                Impairment => "IM",
                Insurance => "IN",
                LicenseRevocation => "LA",
                LocationStatus => "LC",
                LevelRemarks => "LE",
                LiabilityStatus => "LI",
                LocalLanguageDescription => "LL",
                ListingRemarks => "LR",
                LifeStyle => "LS",
                LegalType => "LT",
                LossTrend => "LZ",
                MaritalStatus => "MA",
                Miscellaneous => "MI",
                MultipleListingService => "ML",
                Management => "MN",
                Modification => "MO",
                MedicationOrPrescription => "MP",
                MedicalSocialWorker => "MS",
                MilitaryStatus => "MT",
                NominalCapital => "NC",
                NewLicensedStaff => "NL",
                NatureOfSuit => "NS",
                NotWorkRelated => "NW",
                OwnerPaysNotes => "OA",
                OccupationClass => "OC",
                OutsideFinancing => "OF",
                OtherInvestor => "OI",
                OperationsTrend => "ON",
                Operations => "OP",
                OccupationalTherapy => "OT",
                OperationsOutlook => "OU",
                Performance => "PA",
                Profitability => "PB",
                PaidInCapital => "PC",
                PublicRecords => "PD",
                Penalty => "PE",
                ProfitTrend => "PF",
                PossessionNotes => "PG",
                PhotoInstructions => "PI",
                PatientSubjectiveComplaints => "PJ",
                ParkingNotes => "PN",
                ProfitOutlook => "PO",
                Property => "PR",
                PreferredStock => "PS",
                PhysicalTherapy => "PT",
                PhysicianExaminationResults => "PX",
                ReasonForWeightLoss => "R1",
                AssociationOfAmericanRailroadsSpecialProperShippingNameFlag => "R2",
                AssociationOfAmericanRailroadsIntermodalIndicator => "R3",
                AssociationOfAmericanRailroadsUSToCanadaFlag => "R4",
                ResidentialStatus => "R5",
                Revocation => "RC",
                Recovery => "RE",
                RealEstateProperty => "RI",
                RadioLicenseApplication => "RL",
                Remedy => "RM",
                RelatedEntities => "RN",
                RetirementPlanType => "RP",
                Reinstatement => "RR",
                ReasonLastSeen => "RS",
                RegistrationType => "RT",
                Results => "RU",
                RevenueTrend => "RV",
                InvestmentTrend => "S1",
                RoyaltyTrend => "S2",
                PurchasesTrend => "S3",
                LaborInfraction => "S4",
                Debentures => "S5",
                SourceFund => "SA",
                StartingDetails => "SD",
                SummaryAndEvaluation => "SE",
                ShowingInstructions => "SI",
                CodeSJ => "SJ",
                SupplementNoteOrLine => "SL",
                SkilledNursing => "SN",
                StatementPreparation => "SP",
                SalesTrend => "SR",
                Suspension => "SS",
                SpeechTherapy => "ST",
                SubstanceUse => "SU",
                ReportedStatementOfWitness => "SW",
                Size => "SZ",
                Tobacco => "TB",
                Tests => "TE",
                TargetFund => "TF",
                Therapy => "TH",
                Action => "TI",
                Terms => "TM",
                Trend => "TN",
                TenantPaysNotes => "TP",
                Treatment => "TR",
                TwoToFourUnits => "TW",
                TaxService => "TX",
                ProductionCapacity => "UA",
                ActualProduction => "UB",
                BranchTrend => "UC",
                RetailLocations => "UD",
                NetProfit => "UE",
                OrdinaryProfit => "UF",
                DeclaredProfitToLocalTaxOffice => "UG",
                MarketTrend => "UH",
                PreTaxProfit => "UI",
                NetWorth => "UJ",
                DebtToEquity => "UK",
                EquityReturn => "UL",
                Stability => "UM",
                Efficiency => "UN",
                Outlook => "UO",
                Update => "UP",
                CorporateRegistration => "UQ",
                VoterRegistrationApplication => "VA",
                VoterRegistrationApplicationDisposition => "VD",
                Violation => "VO",
                Warning => "WA",
                Prognosis => "WB",
                TreatmentPlan => "WD",
                WorkRestrictions => "WE",
                WitnessStatement => "WF",
                ConditionsAffectingTotalEmployeesAndHours => "WG",
                InjuryWorkRelated => "WH",
                IllnessWorkRelated => "WI",
                ControvertReason => "WK",
                SupervisorsComments => "WL",
                WillfulMisconduct => "WM",
                SupervisorsException => "WN",
                ClaimRelatedWorkAssignmentChanges => "WO",
                CodeWP => "WP",
                EmployeeComment => "WQ",
                EmployeeCommentNotProvidedReason => "WR",
                MedicalRecordsNotAttachedReason => "WS",
                WorkExposuresAndDuration => "WT",
                LetterOfCreditOverdrawn => "X1",
                CargoReceiptNotSigned => "X2",
                CustomsStatementMissingFromInvoice => "X3",
                CodeX4 => "X4",
                ReducedDraft => "X5",
                TimeDrafts => "X6",
                DemandForPayment => "X7",
                EarlyPresentationOfDocuments => "X8",
                PhysicianPatientReportInconsistency => "YR",
                PhysicianTestResults => "YT",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<CodeCategory> {
        use CodeCategory::*;
        match code {
            b"00" => Some(EmployeeMobility),
            b"01" => Some(PreExistingConditions),
            b"02" => Some(Diagnosis),
            b"03" => Some(Condition),
            b"04" => Some(Occurrence),
            b"05" => Some(OccurrenceSpan),
            b"06" => Some(Value),
            b"07" => Some(AmbulanceCertification),
            b"08" => Some(ChiropracticCertification),
            b"09" => Some(DurableMedicalEquipmentCertification),
            b"10" => Some(EnteralOrParenteralTherapyCertification),
            b"11" => Some(OxygenTherapyCertification),
            b"12" => Some(AdmittingDiagnosis),
            b"13" => Some(PrincipalDiagnosis),
            b"14" => Some(PreExistingPhysicalCondition),
            b"15" => Some(PreExistingMentalCondition),
            b"16" => Some(RoutineFootCareClassFinding),
            b"17" => Some(SystemicConditionForRoutineFootCare),
            b"18" => Some(CoOpAdvertising),
            b"19" => Some(CommercialAdvertising),
            b"20" => Some(SpecimenKitTypeCode),
            b"21" => Some(LaboratoryTestConditionCode),
            b"22" => Some(AutomobileLoss),
            b"23" => Some(LaboratoryResultsIdentificationCode),
            b"24" => Some(LineOfBusinessCode),
            b"25" => Some(UnitedStatesDepartmentOfVitalStatisticsECode),
            b"26" => Some(EmploymentStatus),
            b"27" => Some(Income),
            b"28" => Some(Loan),
            b"29" => Some(InjuryOrIllness),
            b"30" => Some(BenefitAdjustment),
            b"31" => Some(Claimant),
            b"32" => Some(ContractholderBranch),
            b"33" => Some(Contractholder),
            b"34" => Some(SecondaryClaimAdministrator),
            b"35" => Some(PrimaryClaimAdministrator),
            b"36" => Some(ReportingAgency),
            b"37" => Some(Process),
            b"38" => Some(HazardousMaterial),
            b"39" => Some(Activity),
            b"40" => Some(Accident),
            b"41" => Some(InitialTreatment),
            b"42" => Some(CauseOfInjury),
            b"43" => Some(PartOfBody),
            b"44" => Some(NatureOfInjury),
            b"45" => Some(SourceOfInjury),
            b"46" => Some(Job),
            b"47" => Some(LossPrevention),
            b"48" => Some(ManagedCare),
            b"49" => Some(RiskManagement),
            b"50" => Some(ClaimHandling),
            b"51" => Some(EventOrExposure),
            b"52" => Some(EquipmentOrMaterialsOrChemicals),
            b"53" => Some(Coverage),
            b"54" => Some(Overbite),
            b"55" => Some(Overjet),
            b"56" => Some(Profile),
            b"57" => Some(Crossbite),
            b"58" => Some(ArchAsymmetry),
            b"59" => Some(DentitionMidline),
            b"60" => Some(Crowding),
            b"61" => Some(Molars),
            b"62" => Some(Cuspids),
            b"63" => Some(Interviewee),
            b"64" => Some(VerificationOfDeposit),
            b"65" => Some(VerificationOfMortgage),
            b"66" => Some(VerificationOfIncomeOrEmploymentOrBoth),
            b"67" => Some(VerificationOfRent),
            b"68" => Some(VerificationOfLoanOrInstallmentDebtOrBoth),
            b"69" => Some(AntiFungalTherapy),
            b"70" => Some(Hospice),
            b"71" => Some(PrimaryDiagnosis),
            b"72" => Some(SecondaryDiagnosis),
            b"73" => Some(TertiaryDiagnosis),
            b"74" => Some(ProcedureCode),
            b"75" => Some(FunctionalLimitations),
            b"76" => Some(ActivitiesPermitted),
            b"77" => Some(MentalStatus),
            b"78" => Some(MannerPropertyTitleHeld),
            b"79" => Some(PropertyImprovements),
            b"80" => Some(CompleteAppraisal),
            b"81" => Some(LimitedAppraisal),
            b"82" => Some(RestrictedAppraisalReportLimitingConditions),
            b"83" => Some(RouteOfAdministration),
            b"84" => Some(Borrower),
            b"85" => Some(Contract),
            b"86" => Some(Code86),
            b"87" => Some(Code87),
            b"88" => Some(Deductible),
            b"89" => Some(AdvertisingCopy),
            b"90" => Some(PrivateRemarks),
            b"91" => Some(CompensationNotes),
            b"92" => Some(OpenHouseNotes),
            b"93" => Some(TourNotes),
            b"94" => Some(TermsOfSale),
            b"95" => Some(Restrictions),
            b"96" => Some(Disclosures),
            b"97" => Some(Exceptions),
            b"98" => Some(Inclusions),
            b"99" => Some(LeaseType),
            b"A0" => Some(ContractingDistrictType),
            b"A1" => Some(MortgageRecordChange),
            b"A2" => Some(MortgageInsuranceTermination),
            b"A3" => Some(MortgageInsuranceCancellation),
            b"A4" => Some(MortgageServicingTransfer),
            b"A5" => Some(Appraisal),
            b"A6" => Some(StateLicenseDisciplinaryAction),
            b"A7" => Some(SourceOfData),
            b"A8" => Some(Endorsement),
            b"A9" => Some(Notification),
            b"AA" => Some(All),
            b"AB" => Some(AgentsQuestions),
            b"AC" => Some(AgentsShare),
            b"AD" => Some(Benefits),
            b"AE" => Some(ContactOrReference),
            b"AF" => Some(CostBasis),
            b"AG" => Some(DrivingInfractions),
            b"AH" => Some(ExcessDividendUse),
            b"AI" => Some(HomeHealthAide),
            b"AJ" => Some(ExistingCoverage),
            b"AK" => Some(Hospitalization),
            b"AL" => Some(ActivityLimitations),
            b"AM" => Some(Juvenile),
            b"AN" => Some(Occupation),
            b"AO" => Some(PersonalFinanceAndBusiness),
            b"AP" => Some(Appearance),
            b"AQ" => Some(Rating),
            b"AR" => Some(Arrest),
            b"AS" => Some(ReplacedAmount),
            b"AT" => Some(Authority),
            b"AU" => Some(AutomatedUnderwriting),
            b"AV" => Some(Aviation),
            b"AW" => Some(Surgery),
            b"AX" => Some(Travel),
            b"AY" => Some(AgeRemark),
            b"AZ" => Some(PropertyRemark),
            b"B1" => Some(AuditData),
            b"B2" => Some(DeclarationSheetIndicator),
            b"B3" => Some(ServicingData),
            b"B4" => Some(SingleFamily),
            b"B5" => Some(Multifamily),
            b"B6" => Some(PaymentHandling),
            b"B7" => Some(GinnieMae1),
            b"B8" => Some(GinnieMae2),
            b"B9" => Some(GinnieMae2Custom),
            b"BA" => Some(Bankruptcy),
            b"BB" => Some(BusinessBeneficiary),
            b"BC" => Some(BuildingCondition),
            b"BD" => Some(Buydown),
            b"BE" => Some(Beneficiary),
            b"BF" => Some(TaxAgencyParcel),
            b"BG" => Some(HistoricalPerformance),
            b"BH" => Some(ProductRules),
            b"BI" => Some(CommercialProperty),
            b"BJ" => Some(UnimprovedLand),
            b"BK" => Some(Banking),
            b"BL" => Some(NewContract),
            b"BM" => Some(OriginalContract),
            b"BN" => Some(Access),
            b"BO" => Some(Bond),
            b"BP" => Some(BankruptcyPetition),
            b"BQ" => Some(AgentSalesTrend),
            b"BR" => Some(BrokersPriceOpinion),
            b"BS" => Some(BankruptcyStatementOfFinancialAffairs),
            b"BT" => Some(BillingsTrend),
            b"BU" => Some(Assets),
            b"BV" => Some(CashFlow),
            b"BW" => Some(Competition),
            b"BX" => Some(CreditLine),
            b"BY" => Some(CreditorsArrangement),
            b"BZ" => Some(CreditorsMeeting),
            b"C1" => Some(DepreciationConditions),
            b"C2" => Some(AdverseEnvironmentConditions),
            b"C3" => Some(MiscellaneousAdverseConditions),
            b"C4" => Some(SiteConditions),
            b"C5" => Some(SubjectPropertyConditions),
            b"C6" => Some(BoardOfDirectors),
            b"C7" => Some(Reserve),
            b"C8" => Some(Payment),
            b"C9" => Some(Comorbidity),
            b"CA" => Some(Citizenship),
            b"CB" => Some(ContinuingEducation),
            b"CC" => Some(CompensationCalculation),
            b"CD" => Some(CauseOfDeath),
            b"CE" => Some(Condominium),
            b"CF" => Some(Cooperative),
            b"CG" => Some(Conviction),
            b"CH" => Some(DirectSalesTrend),
            b"CI" => Some(ExportTrend),
            b"CJ" => Some(FinancialEmbarrassment),
            b"CK" => Some(Indebtedness),
            b"CL" => Some(Cancellation),
            b"CM" => Some(ClaimAmounts),
            b"CN" => Some(Comparison),
            b"CO" => Some(County),
            b"CP" => Some(Complications),
            b"CQ" => Some(InitialCapital),
            b"CR" => Some(CurrentRatio),
            b"CS" => Some(CommonStock),
            b"CT" => Some(CommissionTrend),
            b"CU" => Some(Stockholders),
            b"CV" => Some(Damage),
            b"CW" => Some(WorkingCapital),
            b"CX" => Some(CompensationAllocation),
            b"CY" => Some(DividendUse),
            b"CZ" => Some(ExcessPremiumUse),
            b"D1" => Some(UnpaidInvoices),
            b"D2" => Some(Withdrawals),
            b"D3" => Some(Imports),
            b"D4" => Some(PlacedForCollection),
            b"DA" => Some(DrugAdjudication),
            b"DB" => Some(LiquidationProceedings),
            b"DC" => Some(Location),
            b"DD" => Some(DischargeDiagnosis),
            b"DE" => Some(Departmental),
            b"DF" => Some(ProfitMargin),
            b"DG" => Some(Proposal),
            b"DH" => Some(Receivership),
            b"DI" => Some(DriverIdentification),
            b"DJ" => Some(ProviderCharacteristicsAndResources),
            b"DK" => Some(SecondarySourceOfInjury),
            b"DL" => Some(Petitiions),
            b"DM" => Some(RegisteredCharges),
            b"DN" => Some(CriminalProceedings),
            b"DO" => Some(HistoricalCriminalProceedings),
            b"DP" => Some(DirectionsToProperty),
            b"DR" => Some(Driving),
            b"DV" => Some(DriverRecord),
            b"E1" => Some(SpectacleLenses),
            b"E2" => Some(ContactLenses),
            b"E3" => Some(SpectacleFrames),
            b"E4" => Some(Employment),
            b"E5" => Some(ExaminersComments),
            b"EB" => Some(IntercompanyRelations),
            b"EC" => Some(Judgments),
            b"ED" => Some(Liens),
            b"EE" => Some(OperatingSurplusTrend),
            b"EF" => Some(ParticipatingInterest),
            b"EG" => Some(ProtestedBills),
            b"EH" => Some(SubcontractingDetails),
            b"EI" => Some(Suits),
            b"EJ" => Some(CodeEJ),
            b"EK" => Some(DetrimentalLegalFilings),
            b"EL" => Some(CustomerDetails),
            b"EM" => Some(SupplierDetail),
            b"ER" => Some(EmployeeRelocation),
            b"ET" => Some(EducationOrTraining),
            b"FA" => Some(Financial),
            b"FC" => Some(FamilyCoverage),
            b"FH" => Some(FamilyHistory),
            b"FI" => Some(Financing),
            b"FL" => Some(FloodDetermination),
            b"FP" => Some(FranchiseTaxPayments),
            b"FR" => Some(FinancialRemarks),
            b"FT" => Some(ForeignTravel),
            b"GD" => Some(Demonstrations),
            b"GS" => Some(ShelfFormat),
            b"GU" => Some(Guarantees),
            b"HA" => Some(Fixed),
            b"HB" => Some(Adjustable),
            b"HC" => Some(RateAdjustment),
            b"HD" => Some(PaymentAdjustment),
            b"HE" => Some(LifeOfLoan),
            b"HF" => Some(PeriodicInterestRate),
            b"HG" => Some(PrincipalAndInterest),
            b"HH" => Some(HealthOrMedical),
            b"HI" => Some(LateCharge),
            b"HJ" => Some(DefaultNoteHoldersCost),
            b"HK" => Some(Prepayment),
            b"HL" => Some(LimitedPayment),
            b"HM" => Some(RateLookback),
            b"HN" => Some(PaymentLookback),
            b"HO" => Some(Index),
            b"HP" => Some(MortgageMargin),
            b"HQ" => Some(SingleFamily24Units),
            b"HR" => Some(Amortization),
            b"HS" => Some(RateConversion),
            b"HT" => Some(InterestOnly),
            b"HU" => Some(PremiumAuditKeyQuestion),
            b"HY" => Some(History),
            b"HZ" => Some(HazardousSports),
            b"IC" => Some(IssuedCapital),
            b"ID" => Some(Identification),
            b"IH" => Some(InsuranceHistoryOrOtherCoverage),
            b"IM" => Some(Impairment),
            b"IN" => Some(Insurance),
            b"LA" => Some(LicenseRevocation),
            b"LC" => Some(LocationStatus),
            b"LE" => Some(LevelRemarks),
            b"LI" => Some(LiabilityStatus),
            b"LL" => Some(LocalLanguageDescription),
            b"LR" => Some(ListingRemarks),
            b"LS" => Some(LifeStyle),
            b"LT" => Some(LegalType),
            b"LZ" => Some(LossTrend),
            b"MA" => Some(MaritalStatus),
            b"MI" => Some(Miscellaneous),
            b"ML" => Some(MultipleListingService),
            b"MN" => Some(Management),
            b"MO" => Some(Modification),
            b"MP" => Some(MedicationOrPrescription),
            b"MS" => Some(MedicalSocialWorker),
            b"MT" => Some(MilitaryStatus),
            b"NC" => Some(NominalCapital),
            b"NL" => Some(NewLicensedStaff),
            b"NS" => Some(NatureOfSuit),
            b"NW" => Some(NotWorkRelated),
            b"OA" => Some(OwnerPaysNotes),
            b"OC" => Some(OccupationClass),
            b"OF" => Some(OutsideFinancing),
            b"OI" => Some(OtherInvestor),
            b"ON" => Some(OperationsTrend),
            b"OP" => Some(Operations),
            b"OT" => Some(OccupationalTherapy),
            b"OU" => Some(OperationsOutlook),
            b"PA" => Some(Performance),
            b"PB" => Some(Profitability),
            b"PC" => Some(PaidInCapital),
            b"PD" => Some(PublicRecords),
            b"PE" => Some(Penalty),
            b"PF" => Some(ProfitTrend),
            b"PG" => Some(PossessionNotes),
            b"PI" => Some(PhotoInstructions),
            b"PJ" => Some(PatientSubjectiveComplaints),
            b"PN" => Some(ParkingNotes),
            b"PO" => Some(ProfitOutlook),
            b"PR" => Some(Property),
            b"PS" => Some(PreferredStock),
            b"PT" => Some(PhysicalTherapy),
            b"PX" => Some(PhysicianExaminationResults),
            b"R1" => Some(ReasonForWeightLoss),
            b"R2" => Some(AssociationOfAmericanRailroadsSpecialProperShippingNameFlag),
            b"R3" => Some(AssociationOfAmericanRailroadsIntermodalIndicator),
            b"R4" => Some(AssociationOfAmericanRailroadsUSToCanadaFlag),
            b"R5" => Some(ResidentialStatus),
            b"RC" => Some(Revocation),
            b"RE" => Some(Recovery),
            b"RI" => Some(RealEstateProperty),
            b"RL" => Some(RadioLicenseApplication),
            b"RM" => Some(Remedy),
            b"RN" => Some(RelatedEntities),
            b"RP" => Some(RetirementPlanType),
            b"RR" => Some(Reinstatement),
            b"RS" => Some(ReasonLastSeen),
            b"RT" => Some(RegistrationType),
            b"RU" => Some(Results),
            b"RV" => Some(RevenueTrend),
            b"S1" => Some(InvestmentTrend),
            b"S2" => Some(RoyaltyTrend),
            b"S3" => Some(PurchasesTrend),
            b"S4" => Some(LaborInfraction),
            b"S5" => Some(Debentures),
            b"SA" => Some(SourceFund),
            b"SD" => Some(StartingDetails),
            b"SE" => Some(SummaryAndEvaluation),
            b"SI" => Some(ShowingInstructions),
            b"SJ" => Some(CodeSJ),
            b"SL" => Some(SupplementNoteOrLine),
            b"SN" => Some(SkilledNursing),
            b"SP" => Some(StatementPreparation),
            b"SR" => Some(SalesTrend),
            b"SS" => Some(Suspension),
            b"ST" => Some(SpeechTherapy),
            b"SU" => Some(SubstanceUse),
            b"SW" => Some(ReportedStatementOfWitness),
            b"SZ" => Some(Size),
            b"TB" => Some(Tobacco),
            b"TE" => Some(Tests),
            b"TF" => Some(TargetFund),
            b"TH" => Some(Therapy),
            b"TI" => Some(Action),
            b"TM" => Some(Terms),
            b"TN" => Some(Trend),
            b"TP" => Some(TenantPaysNotes),
            b"TR" => Some(Treatment),
            b"TW" => Some(TwoToFourUnits),
            b"TX" => Some(TaxService),
            b"UA" => Some(ProductionCapacity),
            b"UB" => Some(ActualProduction),
            b"UC" => Some(BranchTrend),
            b"UD" => Some(RetailLocations),
            b"UE" => Some(NetProfit),
            b"UF" => Some(OrdinaryProfit),
            b"UG" => Some(DeclaredProfitToLocalTaxOffice),
            b"UH" => Some(MarketTrend),
            b"UI" => Some(PreTaxProfit),
            b"UJ" => Some(NetWorth),
            b"UK" => Some(DebtToEquity),
            b"UL" => Some(EquityReturn),
            b"UM" => Some(Stability),
            b"UN" => Some(Efficiency),
            b"UO" => Some(Outlook),
            b"UP" => Some(Update),
            b"UQ" => Some(CorporateRegistration),
            b"VA" => Some(VoterRegistrationApplication),
            b"VD" => Some(VoterRegistrationApplicationDisposition),
            b"VO" => Some(Violation),
            b"WA" => Some(Warning),
            b"WB" => Some(Prognosis),
            b"WD" => Some(TreatmentPlan),
            b"WE" => Some(WorkRestrictions),
            b"WF" => Some(WitnessStatement),
            b"WG" => Some(ConditionsAffectingTotalEmployeesAndHours),
            b"WH" => Some(InjuryWorkRelated),
            b"WI" => Some(IllnessWorkRelated),
            b"WK" => Some(ControvertReason),
            b"WL" => Some(SupervisorsComments),
            b"WM" => Some(WillfulMisconduct),
            b"WN" => Some(SupervisorsException),
            b"WO" => Some(ClaimRelatedWorkAssignmentChanges),
            b"WP" => Some(CodeWP),
            b"WQ" => Some(EmployeeComment),
            b"WR" => Some(EmployeeCommentNotProvidedReason),
            b"WS" => Some(MedicalRecordsNotAttachedReason),
            b"WT" => Some(WorkExposuresAndDuration),
            b"X1" => Some(LetterOfCreditOverdrawn),
            b"X2" => Some(CargoReceiptNotSigned),
            b"X3" => Some(CustomsStatementMissingFromInvoice),
            b"X4" => Some(CodeX4),
            b"X5" => Some(ReducedDraft),
            b"X6" => Some(TimeDrafts),
            b"X7" => Some(DemandForPayment),
            b"X8" => Some(EarlyPresentationOfDocuments),
            b"YR" => Some(PhysicianPatientReportInconsistency),
            b"YT" => Some(PhysicianTestResults),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use CodeCategory::*;
        match self {
            EmployeeMobility => "Employee Mobility",
            PreExistingConditions => "Pre-existing Conditions",
            Diagnosis => "Diagnosis",
            Condition => "Condition",
            Occurrence => "Occurrence",
            OccurrenceSpan => "Occurrence Span",
            Value => "Value",
            AmbulanceCertification => "Ambulance Certification",
            ChiropracticCertification => "Chiropractic Certification",
            DurableMedicalEquipmentCertification => {
                "Durable Medical Equipment Certification"
            }
            EnteralOrParenteralTherapyCertification => {
                "Enteral or Parenteral Therapy Certification"
            }
            OxygenTherapyCertification => "Oxygen Therapy Certification",
            AdmittingDiagnosis => "Admitting Diagnosis",
            PrincipalDiagnosis => "Principal Diagnosis",
            PreExistingPhysicalCondition => "Pre-Existing Physical Condition",
            PreExistingMentalCondition => "Pre-Existing Mental Condition",
            RoutineFootCareClassFinding => "Routine Foot Care Class Finding",
            SystemicConditionForRoutineFootCare => {
                "Systemic Condition for Routine Foot Care"
            }
            CoOpAdvertising => "Co-op Advertising",
            CommercialAdvertising => "Commercial Advertising",
            SpecimenKitTypeCode => "Specimen Kit Type Code",
            LaboratoryTestConditionCode => "Laboratory Test Condition Code",
            AutomobileLoss => "Automobile Loss",
            LaboratoryResultsIdentificationCode => {
                "Laboratory Results Identification Code"
            }
            LineOfBusinessCode => "Line of Business Code",
            UnitedStatesDepartmentOfVitalStatisticsECode => {
                "United States Department of Vital Statistics E-Code"
            }
            EmploymentStatus => "Employment Status Information",
            Income => "Income",
            Loan => "Loan Information",
            InjuryOrIllness => "Injury or Illness",
            BenefitAdjustment => "Benefit Adjustment",
            Claimant => "Claimant",
            ContractholderBranch => "Contractholder Branch",
            Contractholder => "Contractholder",
            SecondaryClaimAdministrator => "Secondary Claim Administrator",
            PrimaryClaimAdministrator => "Primary Claim Administrator",
            ReportingAgency => "Reporting Agency",
            Process => "Process",
            HazardousMaterial => "Hazardous Material",
            Activity => "Activity",
            Accident => "Accident",
            InitialTreatment => "Initial Treatment",
            CauseOfInjury => "Cause of Injury",
            PartOfBody => "Part of Body",
            NatureOfInjury => "Nature of Injury",
            SourceOfInjury => "Source of Injury",
            Job => "Job",
            LossPrevention => "Loss Prevention",
            ManagedCare => "Managed Care",
            RiskManagement => "Risk Management",
            ClaimHandling => "Claim Handling",
            EventOrExposure => "Event or Exposure",
            EquipmentOrMaterialsOrChemicals => "Equipment or Materials or Chemicals",
            Coverage => "Coverage",
            Overbite => "Overbite",
            Overjet => "Overjet",
            Profile => "Profile",
            Crossbite => "Crossbite",
            ArchAsymmetry => "Arch Asymmetry",
            DentitionMidline => "Dentition Midline",
            Crowding => "Crowding",
            Molars => "Molars",
            Cuspids => "Cuspids",
            Interviewee => "Interviewee",
            VerificationOfDeposit => "Verification of Deposit",
            VerificationOfMortgage => "Verification of Mortgage",
            VerificationOfIncomeOrEmploymentOrBoth => {
                "Verification of Income or Employment or Both"
            }
            VerificationOfRent => "Verification of Rent",
            VerificationOfLoanOrInstallmentDebtOrBoth => {
                "Verification of Loan or Installment Debt or Both"
            }
            AntiFungalTherapy => "Anti-fungal Therapy",
            Hospice => "Hospice",
            PrimaryDiagnosis => "Primary Diagnosis",
            SecondaryDiagnosis => "Secondary Diagnosis",
            TertiaryDiagnosis => "Tertiary Diagnosis",
            ProcedureCode => "Procedure Code",
            FunctionalLimitations => "Functional Limitations",
            ActivitiesPermitted => "Activities Permitted",
            MentalStatus => "Mental Status",
            MannerPropertyTitleHeld => "Manner Property Title Held",
            PropertyImprovements => "Property Improvements",
            CompleteAppraisal => "Complete Appraisal",
            LimitedAppraisal => "Limited Appraisal",
            RestrictedAppraisalReportLimitingConditions => {
                "Restricted Appraisal Report Limiting Conditions"
            }
            RouteOfAdministration => "Route of Administration",
            Borrower => "Borrower Information",
            Contract => "Contract Information",
            Code86 => "Fannie Mae (Federal National Mortgage Association)",
            Code87 => "Freddie Mac (Federal Home Loan Mortgage Corporation)",
            Deductible => "Deductible",
            AdvertisingCopy => "Advertising Copy",
            PrivateRemarks => "Private Remarks",
            CompensationNotes => "Compensation Notes",
            OpenHouseNotes => "Open House Notes",
            TourNotes => "Tour Notes",
            TermsOfSale => "Terms of Sale",
            Restrictions => "Restrictions",
            Disclosures => "Disclosures",
            Exceptions => "Exceptions",
            Inclusions => "Inclusions",
            LeaseType => "Lease Type",
            ContractingDistrictType => "Contracting District Type",
            MortgageRecordChange => "Mortgage Record Change",
            MortgageInsuranceTermination => "Mortgage Insurance Termination",
            MortgageInsuranceCancellation => "Mortgage Insurance Cancellation",
            MortgageServicingTransfer => "Mortgage Servicing Transfer",
            Appraisal => "Appraisal",
            StateLicenseDisciplinaryAction => "State License Disciplinary Action",
            SourceOfData => "Source of Data",
            Endorsement => "Endorsement",
            Notification => "Notification",
            All => "All",
            AgentsQuestions => "Agent's Questions",
            AgentsShare => "Agent's Share",
            Benefits => "Benefits",
            ContactOrReference => "Contact or Reference Information",
            CostBasis => "Cost Basis",
            DrivingInfractions => "Driving Infractions",
            ExcessDividendUse => "Excess Dividend Use",
            HomeHealthAide => "Home Health Aide",
            ExistingCoverage => "Existing Coverage Information",
            Hospitalization => "Hospitalization",
            ActivityLimitations => "Activity Limitations",
            Juvenile => "Juvenile Information",
            Occupation => "Occupation Information",
            PersonalFinanceAndBusiness => "Personal Finance and Business Information",
            Appearance => "Appearance",
            Rating => "Rating Information",
            Arrest => "Arrest",
            ReplacedAmount => "Replaced Amount",
            Authority => "Authority",
            AutomatedUnderwriting => "Automated Underwriting Information",
            Aviation => "Aviation",
            Surgery => "Surgery",
            Travel => "Travel Information",
            AgeRemark => "Age Remark",
            PropertyRemark => "Property Remark",
            AuditData => "Audit Data",
            DeclarationSheetIndicator => "Declaration Sheet Indicator",
            ServicingData => "Servicing Data",
            SingleFamily => "Single Family",
            Multifamily => "Multifamily",
            PaymentHandling => "Payment Handling",
            GinnieMae1 => "Ginnie Mae 1",
            GinnieMae2 => "Ginnie Mae 2",
            GinnieMae2Custom => "Ginnie Mae 2 Custom",
            Bankruptcy => "Bankruptcy",
            BusinessBeneficiary => "Business Beneficiary",
            BuildingCondition => "Building Condition",
            Buydown => "Buydown",
            Beneficiary => "Beneficiary",
            TaxAgencyParcel => "Tax Agency Parcel Identifier",
            HistoricalPerformance => "Historical Performance",
            ProductRules => "Product Rules",
            CommercialProperty => "Commercial Property",
            UnimprovedLand => "Unimproved Land",
            Banking => "Banking",
            NewContract => "New Contract",
            OriginalContract => "Original Contract",
            Access => "Access",
            Bond => "Bond",
            BankruptcyPetition => "Bankruptcy Petition",
            AgentSalesTrend => "Agent Sales Trend",
            BrokersPriceOpinion => "Broker's Price Opinion",
            BankruptcyStatementOfFinancialAffairs => {
                "Bankruptcy Statement of Financial Affairs"
            }
            BillingsTrend => "Billings Trend",
            Assets => "Assets",
            CashFlow => "Cash Flow",
            Competition => "Competition",
            CreditLine => "Credit Line",
            CreditorsArrangement => "Creditors Arrangement",
            CreditorsMeeting => "Creditors Meeting",
            DepreciationConditions => "Depreciation Conditions",
            AdverseEnvironmentConditions => "Adverse Environment Conditions",
            MiscellaneousAdverseConditions => "Miscellaneous Adverse Conditions",
            SiteConditions => "Site Conditions",
            SubjectPropertyConditions => "Subject Property Conditions",
            BoardOfDirectors => "Board of Directors",
            Reserve => "Reserve",
            Payment => "Payment",
            Comorbidity => "Comorbidity",
            Citizenship => "Citizenship",
            ContinuingEducation => "Continuing Education",
            CompensationCalculation => "Compensation Calculation",
            CauseOfDeath => "Cause of Death",
            Condominium => "Condominium",
            Cooperative => "Cooperative",
            Conviction => "Conviction",
            DirectSalesTrend => "Direct Sales Trend",
            ExportTrend => "Export Trend",
            FinancialEmbarrassment => "Financial Embarrassment",
            Indebtedness => "Indebtedness",
            Cancellation => "Cancellation",
            ClaimAmounts => "Claim Amounts",
            Comparison => "Comparison",
            County => "County",
            Complications => "Complications",
            InitialCapital => "Initial Capital",
            CurrentRatio => "Current Ratio",
            CommonStock => "Common Stock",
            CommissionTrend => "Commission Trend",
            Stockholders => "Stockholders",
            Damage => "Damage",
            WorkingCapital => "Working Capital",
            CompensationAllocation => "Compensation Allocation",
            DividendUse => "Dividend Use",
            ExcessPremiumUse => "Excess Premium Use",
            UnpaidInvoices => "Unpaid Invoices",
            Withdrawals => "Withdrawals",
            Imports => "Imports",
            PlacedForCollection => "Placed for Collection",
            DrugAdjudication => "Drug Adjudication Information",
            LiquidationProceedings => "Liquidation Proceedings",
            Location => "Location",
            DischargeDiagnosis => "Discharge Diagnosis",
            Departmental => "Departmental",
            ProfitMargin => "Profit Margin",
            Proposal => "Proposal",
            Receivership => "Receivership",
            DriverIdentification => "Driver Identification Information",
            ProviderCharacteristicsAndResources => {
                "Provider Characteristics and Resources"
            }
            SecondarySourceOfInjury => "Secondary Source of Injury",
            Petitiions => "Petitiions",
            RegisteredCharges => "Registered Charges",
            CriminalProceedings => "Criminal Proceedings",
            HistoricalCriminalProceedings => "Historical Criminal Proceedings",
            DirectionsToProperty => "Directions to Property",
            Driving => "Driving",
            DriverRecord => "Driver Record Information",
            SpectacleLenses => "Spectacle Lenses",
            ContactLenses => "Contact Lenses",
            SpectacleFrames => "Spectacle Frames",
            Employment => "Employment",
            ExaminersComments => "Examiner's Comments",
            IntercompanyRelations => "Intercompany Relations",
            Judgments => "Judgments",
            Liens => "Liens",
            OperatingSurplusTrend => "Operating Surplus Trend",
            ParticipatingInterest => "Participating Interest",
            ProtestedBills => "Protested Bills",
            SubcontractingDetails => "Subcontracting Details",
            Suits => "Suits",
            CodeEJ => "Uniform Commercial Code (UCC) Filings",
            DetrimentalLegalFilings => "Detrimental Legal Filings",
            CustomerDetails => "Customer Details",
            SupplierDetail => "Supplier Detail",
            EmployeeRelocation => "Employee Relocation",
            EducationOrTraining => "Education or Training",
            Financial => "Financial",
            FamilyCoverage => "Family Coverage",
            FamilyHistory => "Family History",
            Financing => "Financing",
            FloodDetermination => "Flood Determination",
            FranchiseTaxPayments => "Franchise Tax Payments",
            FinancialRemarks => "Financial Remarks",
            ForeignTravel => "Foreign Travel",
            Demonstrations => "Demonstrations",
            ShelfFormat => "Shelf Format",
            Guarantees => "Guarantees",
            Fixed => "Fixed",
            Adjustable => "Adjustable",
            RateAdjustment => "Rate Adjustment",
            PaymentAdjustment => "Payment Adjustment",
            LifeOfLoan => "Life of Loan",
            PeriodicInterestRate => "Periodic Interest Rate",
            PrincipalAndInterest => "Principal and Interest",
            HealthOrMedical => "Health or Medical",
            LateCharge => "Late Charge",
            DefaultNoteHoldersCost => "Default Note Holder's Cost",
            Prepayment => "Prepayment",
            LimitedPayment => "Limited Payment",
            RateLookback => "Rate Lookback",
            PaymentLookback => "Payment Lookback",
            Index => "Index",
            MortgageMargin => "Mortgage Margin",
            SingleFamily24Units => "Single Family 2-4 Units",
            Amortization => "Amortization",
            RateConversion => "Rate Conversion",
            InterestOnly => "Interest Only",
            PremiumAuditKeyQuestion => "Premium Audit Key Question",
            History => "History",
            HazardousSports => "Hazardous Sports",
            IssuedCapital => "Issued Capital",
            Identification => "Identification",
            InsuranceHistoryOrOtherCoverage => "Insurance History or Other Coverage",
            Impairment => "Impairment",
            Insurance => "Insurance",
            LicenseRevocation => "License Revocation",
            LocationStatus => "Location Status",
            LevelRemarks => "Level Remarks",
            LiabilityStatus => "Liability Status",
            LocalLanguageDescription => "Local Language Description",
            ListingRemarks => "Listing Remarks",
            LifeStyle => "Life Style",
            LegalType => "Legal Type",
            LossTrend => "Loss Trend",
            MaritalStatus => "Marital Status",
            Miscellaneous => "Miscellaneous",
            MultipleListingService => "Multiple Listing Service",
            Management => "Management",
            Modification => "Modification",
            MedicationOrPrescription => "Medication or Prescription",
            MedicalSocialWorker => "Medical Social Worker",
            MilitaryStatus => "Military Status",
            NominalCapital => "Nominal Capital",
            NewLicensedStaff => "New Licensed Staff",
            NatureOfSuit => "Nature of Suit",
            NotWorkRelated => "Not Work Related",
            OwnerPaysNotes => "Owner Pays Notes",
            OccupationClass => "Occupation Class",
            OutsideFinancing => "Outside Financing",
            OtherInvestor => "Other Investor",
            OperationsTrend => "Operations Trend",
            Operations => "Operations",
            OccupationalTherapy => "Occupational Therapy",
            OperationsOutlook => "Operations Outlook",
            Performance => "Performance",
            Profitability => "Profitability",
            PaidInCapital => "Paid in Capital",
            PublicRecords => "Public Records",
            Penalty => "Penalty",
            ProfitTrend => "Profit Trend",
            PossessionNotes => "Possession Notes",
            PhotoInstructions => "Photo Instructions",
            PatientSubjectiveComplaints => "Patient Subjective Complaints",
            ParkingNotes => "Parking Notes",
            ProfitOutlook => "Profit Outlook",
            Property => "Property",
            PreferredStock => "Preferred Stock",
            PhysicalTherapy => "Physical Therapy",
            PhysicianExaminationResults => "Physician Examination Results",
            ReasonForWeightLoss => "Reason for Weight Loss",
            AssociationOfAmericanRailroadsSpecialProperShippingNameFlag => {
                "Association of American Railroads Special Proper Shipping Name Flag"
            }
            AssociationOfAmericanRailroadsIntermodalIndicator => {
                "Association of American Railroads Intermodal Indicator"
            }
            AssociationOfAmericanRailroadsUSToCanadaFlag => {
                "Association of American Railroads U.S. to Canada Flag"
            }
            ResidentialStatus => "Residential Status",
            Revocation => "Revocation",
            Recovery => "Recovery",
            RealEstateProperty => "Real Estate Property Information",
            RadioLicenseApplication => "Radio License Application",
            Remedy => "Remedy",
            RelatedEntities => "Related Entities",
            RetirementPlanType => "Retirement Plan Type",
            Reinstatement => "Reinstatement",
            ReasonLastSeen => "Reason Last Seen",
            RegistrationType => "Registration Type",
            Results => "Results",
            RevenueTrend => "Revenue Trend",
            InvestmentTrend => "Investment Trend",
            RoyaltyTrend => "Royalty Trend",
            PurchasesTrend => "Purchases Trend",
            LaborInfraction => "Labor Infraction",
            Debentures => "Debentures",
            SourceFund => "Source Fund",
            StartingDetails => "Starting Details",
            SummaryAndEvaluation => "Summary and Evaluation",
            ShowingInstructions => "Showing Instructions",
            CodeSJ => "Suits, Judgments & Liens",
            SupplementNoteOrLine => "Supplement Note or Line",
            SkilledNursing => "Skilled Nursing",
            StatementPreparation => "Statement Preparation",
            SalesTrend => "Sales Trend",
            Suspension => "Suspension",
            SpeechTherapy => "Speech Therapy",
            SubstanceUse => "Substance Use",
            ReportedStatementOfWitness => "Reported Statement of Witness",
            Size => "Size",
            Tobacco => "Tobacco",
            Tests => "Tests",
            TargetFund => "Target Fund",
            Therapy => "Therapy",
            Action => "Action",
            Terms => "Terms",
            Trend => "Trend",
            TenantPaysNotes => "Tenant Pays Notes",
            Treatment => "Treatment",
            TwoToFourUnits => "Two to Four Units",
            TaxService => "Tax Service",
            ProductionCapacity => "Production Capacity",
            ActualProduction => "Actual Production",
            BranchTrend => "Branch Trend",
            RetailLocations => "Retail Locations",
            NetProfit => "Net Profit",
            OrdinaryProfit => "Ordinary Profit",
            DeclaredProfitToLocalTaxOffice => "Declared Profit to Local Tax Office",
            MarketTrend => "Market Trend",
            PreTaxProfit => "Pre Tax Profit",
            NetWorth => "Net Worth",
            DebtToEquity => "Debt to Equity",
            EquityReturn => "Equity Return",
            Stability => "Stability",
            Efficiency => "Efficiency",
            Outlook => "Outlook",
            Update => "Update",
            CorporateRegistration => "Corporate Registration",
            VoterRegistrationApplication => "Voter Registration Application",
            VoterRegistrationApplicationDisposition => {
                "Voter Registration Application Disposition"
            }
            Violation => "Violation",
            Warning => "Warning",
            Prognosis => "Prognosis",
            TreatmentPlan => "Treatment Plan",
            WorkRestrictions => "Work Restrictions",
            WitnessStatement => "Witness Statement",
            ConditionsAffectingTotalEmployeesAndHours => {
                "Conditions Affecting Total Employees and Hours"
            }
            InjuryWorkRelated => "Injury Work Related",
            IllnessWorkRelated => "Illness Work Related",
            ControvertReason => "Controvert Reason",
            SupervisorsComments => "Supervisor's Comments",
            WillfulMisconduct => "Willful Misconduct",
            SupervisorsException => "Supervisor's Exception",
            ClaimRelatedWorkAssignmentChanges => "Claim Related Work Assignment Changes",
            CodeWP => "30 Day Delay Reason",
            EmployeeComment => "Employee Comment",
            EmployeeCommentNotProvidedReason => "Employee Comment Not Provided Reason",
            MedicalRecordsNotAttachedReason => "Medical Records Not Attached Reason",
            WorkExposuresAndDuration => "Work Exposures and Duration",
            LetterOfCreditOverdrawn => "Letter of Credit Overdrawn",
            CargoReceiptNotSigned => "Cargo Receipt Not Signed",
            CustomsStatementMissingFromInvoice => {
                "Customs Statement Missing from Invoice"
            }
            CodeX4 => "Purchase Order Not on Letter of Credit (Except Masters)",
            ReducedDraft => "Reduced Draft",
            TimeDrafts => "Time Drafts",
            DemandForPayment => "Demand for Payment",
            EarlyPresentationOfDocuments => "Early Presentation of Documents",
            PhysicianPatientReportInconsistency => {
                "Physician - Patient Report Inconsistency"
            }
            PhysicianTestResults => "Physician Test Results",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<CodeCategory> {
        {
            use CodeCategory::*;
            match description {
                "Employee Mobility" => Some(EmployeeMobility),
                "Pre-existing Conditions" => Some(PreExistingConditions),
                "Diagnosis" => Some(Diagnosis),
                "Condition" => Some(Condition),
                "Occurrence" => Some(Occurrence),
                "Occurrence Span" => Some(OccurrenceSpan),
                "Value" => Some(Value),
                "Ambulance Certification" => Some(AmbulanceCertification),
                "Chiropractic Certification" => Some(ChiropracticCertification),
                "Durable Medical Equipment Certification" => {
                    Some(DurableMedicalEquipmentCertification)
                }
                "Enteral or Parenteral Therapy Certification" => {
                    Some(EnteralOrParenteralTherapyCertification)
                }
                "Oxygen Therapy Certification" => Some(OxygenTherapyCertification),
                "Admitting Diagnosis" => Some(AdmittingDiagnosis),
                "Principal Diagnosis" => Some(PrincipalDiagnosis),
                "Pre-Existing Physical Condition" => Some(PreExistingPhysicalCondition),
                "Pre-Existing Mental Condition" => Some(PreExistingMentalCondition),
                "Routine Foot Care Class Finding" => Some(RoutineFootCareClassFinding),
                "Systemic Condition for Routine Foot Care" => {
                    Some(SystemicConditionForRoutineFootCare)
                }
                "Co-op Advertising" => Some(CoOpAdvertising),
                "Commercial Advertising" => Some(CommercialAdvertising),
                "Specimen Kit Type Code" => Some(SpecimenKitTypeCode),
                "Laboratory Test Condition Code" => Some(LaboratoryTestConditionCode),
                "Automobile Loss" => Some(AutomobileLoss),
                "Laboratory Results Identification Code" => {
                    Some(LaboratoryResultsIdentificationCode)
                }
                "Line of Business Code" => Some(LineOfBusinessCode),
                "United States Department of Vital Statistics E-Code" => {
                    Some(UnitedStatesDepartmentOfVitalStatisticsECode)
                }
                "Employment Status Information" => Some(EmploymentStatus),
                "Income" => Some(Income),
                "Loan Information" => Some(Loan),
                "Injury or Illness" => Some(InjuryOrIllness),
                "Benefit Adjustment" => Some(BenefitAdjustment),
                "Claimant" => Some(Claimant),
                "Contractholder Branch" => Some(ContractholderBranch),
                "Contractholder" => Some(Contractholder),
                "Secondary Claim Administrator" => Some(SecondaryClaimAdministrator),
                "Primary Claim Administrator" => Some(PrimaryClaimAdministrator),
                "Reporting Agency" => Some(ReportingAgency),
                "Process" => Some(Process),
                "Hazardous Material" => Some(HazardousMaterial),
                "Activity" => Some(Activity),
                "Accident" => Some(Accident),
                "Initial Treatment" => Some(InitialTreatment),
                "Cause of Injury" => Some(CauseOfInjury),
                "Part of Body" => Some(PartOfBody),
                "Nature of Injury" => Some(NatureOfInjury),
                "Source of Injury" => Some(SourceOfInjury),
                "Job" => Some(Job),
                "Loss Prevention" => Some(LossPrevention),
                "Managed Care" => Some(ManagedCare),
                "Risk Management" => Some(RiskManagement),
                "Claim Handling" => Some(ClaimHandling),
                "Event or Exposure" => Some(EventOrExposure),
                "Equipment or Materials or Chemicals" => {
                    Some(EquipmentOrMaterialsOrChemicals)
                }
                "Coverage" => Some(Coverage),
                "Overbite" => Some(Overbite),
                "Overjet" => Some(Overjet),
                "Profile" => Some(Profile),
                "Crossbite" => Some(Crossbite),
                "Arch Asymmetry" => Some(ArchAsymmetry),
                "Dentition Midline" => Some(DentitionMidline),
                "Crowding" => Some(Crowding),
                "Molars" => Some(Molars),
                "Cuspids" => Some(Cuspids),
                "Interviewee" => Some(Interviewee),
                "Verification of Deposit" => Some(VerificationOfDeposit),
                "Verification of Mortgage" => Some(VerificationOfMortgage),
                "Verification of Income or Employment or Both" => {
                    Some(VerificationOfIncomeOrEmploymentOrBoth)
                }
                "Verification of Rent" => Some(VerificationOfRent),
                "Verification of Loan or Installment Debt or Both" => {
                    Some(VerificationOfLoanOrInstallmentDebtOrBoth)
                }
                "Anti-fungal Therapy" => Some(AntiFungalTherapy),
                "Hospice" => Some(Hospice),
                "Primary Diagnosis" => Some(PrimaryDiagnosis),
                "Secondary Diagnosis" => Some(SecondaryDiagnosis),
                "Tertiary Diagnosis" => Some(TertiaryDiagnosis),
                "Procedure Code" => Some(ProcedureCode),
                "Functional Limitations" => Some(FunctionalLimitations),
                "Activities Permitted" => Some(ActivitiesPermitted),
                "Mental Status" => Some(MentalStatus),
                "Manner Property Title Held" => Some(MannerPropertyTitleHeld),
                "Property Improvements" => Some(PropertyImprovements),
                "Complete Appraisal" => Some(CompleteAppraisal),
                "Limited Appraisal" => Some(LimitedAppraisal),
                "Restricted Appraisal Report Limiting Conditions" => {
                    Some(RestrictedAppraisalReportLimitingConditions)
                }
                "Route of Administration" => Some(RouteOfAdministration),
                "Borrower Information" => Some(Borrower),
                "Contract Information" => Some(Contract),
                "Fannie Mae (Federal National Mortgage Association)" => Some(Code86),
                "Freddie Mac (Federal Home Loan Mortgage Corporation)" => Some(Code87),
                "Deductible" => Some(Deductible),
                "Advertising Copy" => Some(AdvertisingCopy),
                "Private Remarks" => Some(PrivateRemarks),
                "Compensation Notes" => Some(CompensationNotes),
                "Open House Notes" => Some(OpenHouseNotes),
                "Tour Notes" => Some(TourNotes),
                "Terms of Sale" => Some(TermsOfSale),
                "Restrictions" => Some(Restrictions),
                "Disclosures" => Some(Disclosures),
                "Exceptions" => Some(Exceptions),
                "Inclusions" => Some(Inclusions),
                "Lease Type" => Some(LeaseType),
                "Contracting District Type" => Some(ContractingDistrictType),
                "Mortgage Record Change" => Some(MortgageRecordChange),
                "Mortgage Insurance Termination" => Some(MortgageInsuranceTermination),
                "Mortgage Insurance Cancellation" => Some(MortgageInsuranceCancellation),
                "Mortgage Servicing Transfer" => Some(MortgageServicingTransfer),
                "Appraisal" => Some(Appraisal),
                "State License Disciplinary Action" => {
                    Some(StateLicenseDisciplinaryAction)
                }
                "Source of Data" => Some(SourceOfData),
                "Endorsement" => Some(Endorsement),
                "Notification" => Some(Notification),
                "All" => Some(All),
                "Agent's Questions" => Some(AgentsQuestions),
                "Agent's Share" => Some(AgentsShare),
                "Benefits" => Some(Benefits),
                "Contact or Reference Information" => Some(ContactOrReference),
                "Cost Basis" => Some(CostBasis),
                "Driving Infractions" => Some(DrivingInfractions),
                "Excess Dividend Use" => Some(ExcessDividendUse),
                "Home Health Aide" => Some(HomeHealthAide),
                "Existing Coverage Information" => Some(ExistingCoverage),
                "Hospitalization" => Some(Hospitalization),
                "Activity Limitations" => Some(ActivityLimitations),
                "Juvenile Information" => Some(Juvenile),
                "Occupation Information" => Some(Occupation),
                "Personal Finance and Business Information" => {
                    Some(PersonalFinanceAndBusiness)
                }
                "Appearance" => Some(Appearance),
                "Rating Information" => Some(Rating),
                "Arrest" => Some(Arrest),
                "Replaced Amount" => Some(ReplacedAmount),
                "Authority" => Some(Authority),
                "Automated Underwriting Information" => Some(AutomatedUnderwriting),
                "Aviation" => Some(Aviation),
                "Surgery" => Some(Surgery),
                "Travel Information" => Some(Travel),
                "Age Remark" => Some(AgeRemark),
                "Property Remark" => Some(PropertyRemark),
                "Audit Data" => Some(AuditData),
                "Declaration Sheet Indicator" => Some(DeclarationSheetIndicator),
                "Servicing Data" => Some(ServicingData),
                "Single Family" => Some(SingleFamily),
                "Multifamily" => Some(Multifamily),
                "Payment Handling" => Some(PaymentHandling),
                "Ginnie Mae 1" => Some(GinnieMae1),
                "Ginnie Mae 2" => Some(GinnieMae2),
                "Ginnie Mae 2 Custom" => Some(GinnieMae2Custom),
                "Bankruptcy" => Some(Bankruptcy),
                "Business Beneficiary" => Some(BusinessBeneficiary),
                "Building Condition" => Some(BuildingCondition),
                "Buydown" => Some(Buydown),
                "Beneficiary" => Some(Beneficiary),
                "Tax Agency Parcel Identifier" => Some(TaxAgencyParcel),
                "Historical Performance" => Some(HistoricalPerformance),
                "Product Rules" => Some(ProductRules),
                "Commercial Property" => Some(CommercialProperty),
                "Unimproved Land" => Some(UnimprovedLand),
                "Banking" => Some(Banking),
                "New Contract" => Some(NewContract),
                "Original Contract" => Some(OriginalContract),
                "Access" => Some(Access),
                "Bond" => Some(Bond),
                "Bankruptcy Petition" => Some(BankruptcyPetition),
                "Agent Sales Trend" => Some(AgentSalesTrend),
                "Broker's Price Opinion" => Some(BrokersPriceOpinion),
                "Bankruptcy Statement of Financial Affairs" => {
                    Some(BankruptcyStatementOfFinancialAffairs)
                }
                "Billings Trend" => Some(BillingsTrend),
                "Assets" => Some(Assets),
                "Cash Flow" => Some(CashFlow),
                "Competition" => Some(Competition),
                "Credit Line" => Some(CreditLine),
                "Creditors Arrangement" => Some(CreditorsArrangement),
                "Creditors Meeting" => Some(CreditorsMeeting),
                "Depreciation Conditions" => Some(DepreciationConditions),
                "Adverse Environment Conditions" => Some(AdverseEnvironmentConditions),
                "Miscellaneous Adverse Conditions" => {
                    Some(MiscellaneousAdverseConditions)
                }
                "Site Conditions" => Some(SiteConditions),
                "Subject Property Conditions" => Some(SubjectPropertyConditions),
                "Board of Directors" => Some(BoardOfDirectors),
                "Reserve" => Some(Reserve),
                "Payment" => Some(Payment),
                "Comorbidity" => Some(Comorbidity),
                "Citizenship" => Some(Citizenship),
                "Continuing Education" => Some(ContinuingEducation),
                "Compensation Calculation" => Some(CompensationCalculation),
                "Cause of Death" => Some(CauseOfDeath),
                "Condominium" => Some(Condominium),
                "Cooperative" => Some(Cooperative),
                "Conviction" => Some(Conviction),
                "Direct Sales Trend" => Some(DirectSalesTrend),
                "Export Trend" => Some(ExportTrend),
                "Financial Embarrassment" => Some(FinancialEmbarrassment),
                "Indebtedness" => Some(Indebtedness),
                "Cancellation" => Some(Cancellation),
                "Claim Amounts" => Some(ClaimAmounts),
                "Comparison" => Some(Comparison),
                "County" => Some(County),
                "Complications" => Some(Complications),
                "Initial Capital" => Some(InitialCapital),
                "Current Ratio" => Some(CurrentRatio),
                "Common Stock" => Some(CommonStock),
                "Commission Trend" => Some(CommissionTrend),
                "Stockholders" => Some(Stockholders),
                "Damage" => Some(Damage),
                "Working Capital" => Some(WorkingCapital),
                "Compensation Allocation" => Some(CompensationAllocation),
                "Dividend Use" => Some(DividendUse),
                "Excess Premium Use" => Some(ExcessPremiumUse),
                "Unpaid Invoices" => Some(UnpaidInvoices),
                "Withdrawals" => Some(Withdrawals),
                "Imports" => Some(Imports),
                "Placed for Collection" => Some(PlacedForCollection),
                "Drug Adjudication Information" => Some(DrugAdjudication),
                "Liquidation Proceedings" => Some(LiquidationProceedings),
                "Location" => Some(Location),
                "Discharge Diagnosis" => Some(DischargeDiagnosis),
                "Departmental" => Some(Departmental),
                "Profit Margin" => Some(ProfitMargin),
                "Proposal" => Some(Proposal),
                "Receivership" => Some(Receivership),
                "Driver Identification Information" => Some(DriverIdentification),
                "Provider Characteristics and Resources" => {
                    Some(ProviderCharacteristicsAndResources)
                }
                "Secondary Source of Injury" => Some(SecondarySourceOfInjury),
                "Petitiions" => Some(Petitiions),
                "Registered Charges" => Some(RegisteredCharges),
                "Criminal Proceedings" => Some(CriminalProceedings),
                "Historical Criminal Proceedings" => Some(HistoricalCriminalProceedings),
                "Directions to Property" => Some(DirectionsToProperty),
                "Driving" => Some(Driving),
                "Driver Record Information" => Some(DriverRecord),
                "Spectacle Lenses" => Some(SpectacleLenses),
                "Contact Lenses" => Some(ContactLenses),
                "Spectacle Frames" => Some(SpectacleFrames),
                "Employment" => Some(Employment),
                "Examiner's Comments" => Some(ExaminersComments),
                "Intercompany Relations" => Some(IntercompanyRelations),
                "Judgments" => Some(Judgments),
                "Liens" => Some(Liens),
                "Operating Surplus Trend" => Some(OperatingSurplusTrend),
                "Participating Interest" => Some(ParticipatingInterest),
                "Protested Bills" => Some(ProtestedBills),
                "Subcontracting Details" => Some(SubcontractingDetails),
                "Suits" => Some(Suits),
                "Uniform Commercial Code (UCC) Filings" => Some(CodeEJ),
                "Detrimental Legal Filings" => Some(DetrimentalLegalFilings),
                "Customer Details" => Some(CustomerDetails),
                "Supplier Detail" => Some(SupplierDetail),
                "Employee Relocation" => Some(EmployeeRelocation),
                "Education or Training" => Some(EducationOrTraining),
                "Financial" => Some(Financial),
                "Family Coverage" => Some(FamilyCoverage),
                "Family History" => Some(FamilyHistory),
                "Financing" => Some(Financing),
                "Flood Determination" => Some(FloodDetermination),
                "Franchise Tax Payments" => Some(FranchiseTaxPayments),
                "Financial Remarks" => Some(FinancialRemarks),
                "Foreign Travel" => Some(ForeignTravel),
                "Demonstrations" => Some(Demonstrations),
                "Shelf Format" => Some(ShelfFormat),
                "Guarantees" => Some(Guarantees),
                "Fixed" => Some(Fixed),
                "Adjustable" => Some(Adjustable),
                "Rate Adjustment" => Some(RateAdjustment),
                "Payment Adjustment" => Some(PaymentAdjustment),
                "Life of Loan" => Some(LifeOfLoan),
                "Periodic Interest Rate" => Some(PeriodicInterestRate),
                "Principal and Interest" => Some(PrincipalAndInterest),
                "Health or Medical" => Some(HealthOrMedical),
                "Late Charge" => Some(LateCharge),
                "Default Note Holder's Cost" => Some(DefaultNoteHoldersCost),
                "Prepayment" => Some(Prepayment),
                "Limited Payment" => Some(LimitedPayment),
                "Rate Lookback" => Some(RateLookback),
                "Payment Lookback" => Some(PaymentLookback),
                "Index" => Some(Index),
                "Mortgage Margin" => Some(MortgageMargin),
                "Single Family 2-4 Units" => Some(SingleFamily24Units),
                "Amortization" => Some(Amortization),
                "Rate Conversion" => Some(RateConversion),
                "Interest Only" => Some(InterestOnly),
                "Premium Audit Key Question" => Some(PremiumAuditKeyQuestion),
                "History" => Some(History),
                "Hazardous Sports" => Some(HazardousSports),
                "Issued Capital" => Some(IssuedCapital),
                "Identification" => Some(Identification),
                "Insurance History or Other Coverage" => {
                    Some(InsuranceHistoryOrOtherCoverage)
                }
                "Impairment" => Some(Impairment),
                "Insurance" => Some(Insurance),
                "License Revocation" => Some(LicenseRevocation),
                "Location Status" => Some(LocationStatus),
                "Level Remarks" => Some(LevelRemarks),
                "Liability Status" => Some(LiabilityStatus),
                "Local Language Description" => Some(LocalLanguageDescription),
                "Listing Remarks" => Some(ListingRemarks),
                "Life Style" => Some(LifeStyle),
                "Legal Type" => Some(LegalType),
                "Loss Trend" => Some(LossTrend),
                "Marital Status" => Some(MaritalStatus),
                "Miscellaneous" => Some(Miscellaneous),
                "Multiple Listing Service" => Some(MultipleListingService),
                "Management" => Some(Management),
                "Modification" => Some(Modification),
                "Medication or Prescription" => Some(MedicationOrPrescription),
                "Medical Social Worker" => Some(MedicalSocialWorker),
                "Military Status" => Some(MilitaryStatus),
                "Nominal Capital" => Some(NominalCapital),
                "New Licensed Staff" => Some(NewLicensedStaff),
                "Nature of Suit" => Some(NatureOfSuit),
                "Not Work Related" => Some(NotWorkRelated),
                "Owner Pays Notes" => Some(OwnerPaysNotes),
                "Occupation Class" => Some(OccupationClass),
                "Outside Financing" => Some(OutsideFinancing),
                "Other Investor" => Some(OtherInvestor),
                "Operations Trend" => Some(OperationsTrend),
                "Operations" => Some(Operations),
                "Occupational Therapy" => Some(OccupationalTherapy),
                "Operations Outlook" => Some(OperationsOutlook),
                "Performance" => Some(Performance),
                "Profitability" => Some(Profitability),
                "Paid in Capital" => Some(PaidInCapital),
                "Public Records" => Some(PublicRecords),
                "Penalty" => Some(Penalty),
                "Profit Trend" => Some(ProfitTrend),
                "Possession Notes" => Some(PossessionNotes),
                "Photo Instructions" => Some(PhotoInstructions),
                "Patient Subjective Complaints" => Some(PatientSubjectiveComplaints),
                "Parking Notes" => Some(ParkingNotes),
                "Profit Outlook" => Some(ProfitOutlook),
                "Property" => Some(Property),
                "Preferred Stock" => Some(PreferredStock),
                "Physical Therapy" => Some(PhysicalTherapy),
                "Physician Examination Results" => Some(PhysicianExaminationResults),
                "Reason for Weight Loss" => Some(ReasonForWeightLoss),
                "Association of American Railroads Special Proper Shipping Name Flag" => {
                    Some(AssociationOfAmericanRailroadsSpecialProperShippingNameFlag)
                }
                "Association of American Railroads Intermodal Indicator" => {
                    Some(AssociationOfAmericanRailroadsIntermodalIndicator)
                }
                "Association of American Railroads U.S. to Canada Flag" => {
                    Some(AssociationOfAmericanRailroadsUSToCanadaFlag)
                }
                "Residential Status" => Some(ResidentialStatus),
                "Revocation" => Some(Revocation),
                "Recovery" => Some(Recovery),
                "Real Estate Property Information" => Some(RealEstateProperty),
                "Radio License Application" => Some(RadioLicenseApplication),
                "Remedy" => Some(Remedy),
                "Related Entities" => Some(RelatedEntities),
                "Retirement Plan Type" => Some(RetirementPlanType),
                "Reinstatement" => Some(Reinstatement),
                "Reason Last Seen" => Some(ReasonLastSeen),
                "Registration Type" => Some(RegistrationType),
                "Results" => Some(Results),
                "Revenue Trend" => Some(RevenueTrend),
                "Investment Trend" => Some(InvestmentTrend),
                "Royalty Trend" => Some(RoyaltyTrend),
                "Purchases Trend" => Some(PurchasesTrend),
                "Labor Infraction" => Some(LaborInfraction),
                "Debentures" => Some(Debentures),
                "Source Fund" => Some(SourceFund),
                "Starting Details" => Some(StartingDetails),
                "Summary and Evaluation" => Some(SummaryAndEvaluation),
                "Showing Instructions" => Some(ShowingInstructions),
                "Suits, Judgments & Liens" => Some(CodeSJ),
                "Supplement Note or Line" => Some(SupplementNoteOrLine),
                "Skilled Nursing" => Some(SkilledNursing),
                "Statement Preparation" => Some(StatementPreparation),
                "Sales Trend" => Some(SalesTrend),
                "Suspension" => Some(Suspension),
                "Speech Therapy" => Some(SpeechTherapy),
                "Substance Use" => Some(SubstanceUse),
                "Reported Statement of Witness" => Some(ReportedStatementOfWitness),
                "Size" => Some(Size),
                "Tobacco" => Some(Tobacco),
                "Tests" => Some(Tests),
                "Target Fund" => Some(TargetFund),
                "Therapy" => Some(Therapy),
                "Action" => Some(Action),
                "Terms" => Some(Terms),
                "Trend" => Some(Trend),
                "Tenant Pays Notes" => Some(TenantPaysNotes),
                "Treatment" => Some(Treatment),
                "Two to Four Units" => Some(TwoToFourUnits),
                "Tax Service" => Some(TaxService),
                "Production Capacity" => Some(ProductionCapacity),
                "Actual Production" => Some(ActualProduction),
                "Branch Trend" => Some(BranchTrend),
                "Retail Locations" => Some(RetailLocations),
                "Net Profit" => Some(NetProfit),
                "Ordinary Profit" => Some(OrdinaryProfit),
                "Declared Profit to Local Tax Office" => {
                    Some(DeclaredProfitToLocalTaxOffice)
                }
                "Market Trend" => Some(MarketTrend),
                "Pre Tax Profit" => Some(PreTaxProfit),
                "Net Worth" => Some(NetWorth),
                "Debt to Equity" => Some(DebtToEquity),
                "Equity Return" => Some(EquityReturn),
                "Stability" => Some(Stability),
                "Efficiency" => Some(Efficiency),
                "Outlook" => Some(Outlook),
                "Update" => Some(Update),
                "Corporate Registration" => Some(CorporateRegistration),
                "Voter Registration Application" => Some(VoterRegistrationApplication),
                "Voter Registration Application Disposition" => {
                    Some(VoterRegistrationApplicationDisposition)
                }
                "Violation" => Some(Violation),
                "Warning" => Some(Warning),
                "Prognosis" => Some(Prognosis),
                "Treatment Plan" => Some(TreatmentPlan),
                "Work Restrictions" => Some(WorkRestrictions),
                "Witness Statement" => Some(WitnessStatement),
                "Conditions Affecting Total Employees and Hours" => {
                    Some(ConditionsAffectingTotalEmployeesAndHours)
                }
                "Injury Work Related" => Some(InjuryWorkRelated),
                "Illness Work Related" => Some(IllnessWorkRelated),
                "Controvert Reason" => Some(ControvertReason),
                "Supervisor's Comments" => Some(SupervisorsComments),
                "Willful Misconduct" => Some(WillfulMisconduct),
                "Supervisor's Exception" => Some(SupervisorsException),
                "Claim Related Work Assignment Changes" => {
                    Some(ClaimRelatedWorkAssignmentChanges)
                }
                "30 Day Delay Reason" => Some(CodeWP),
                "Employee Comment" => Some(EmployeeComment),
                "Employee Comment Not Provided Reason" => {
                    Some(EmployeeCommentNotProvidedReason)
                }
                "Medical Records Not Attached Reason" => {
                    Some(MedicalRecordsNotAttachedReason)
                }
                "Work Exposures and Duration" => Some(WorkExposuresAndDuration),
                "Letter of Credit Overdrawn" => Some(LetterOfCreditOverdrawn),
                "Cargo Receipt Not Signed" => Some(CargoReceiptNotSigned),
                "Customs Statement Missing from Invoice" => {
                    Some(CustomsStatementMissingFromInvoice)
                }
                "Purchase Order Not on Letter of Credit (Except Masters)" => Some(CodeX4),
                "Reduced Draft" => Some(ReducedDraft),
                "Time Drafts" => Some(TimeDrafts),
                "Demand for Payment" => Some(DemandForPayment),
                "Early Presentation of Documents" => Some(EarlyPresentationOfDocuments),
                "Physician - Patient Report Inconsistency" => {
                    Some(PhysicianPatientReportInconsistency)
                }
                "Physician Test Results" => Some(PhysicianTestResults),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for CodeCategory {
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
    type Value = CodeCategory;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Code Category")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CodeCategory::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Code Category: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CodeCategory::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Code Category: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for CodeCategory {
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