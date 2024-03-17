use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**755

See docs at <https://www.stedi.com/edi/x12/element/755>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReportTypeCode {
    ///01 - Product Transfer
    ProductTransfer,
    ///02 - Resale
    Resale,
    ///03 - Report Justifying Treatment Beyond Utilization Guidelines
    ReportJustifyingTreatmentBeyondUtilizationGuidelines,
    ///04 - Drugs Administered
    DrugsAdministered,
    ///05 - Treatment Diagnosis
    TreatmentDiagnosis,
    ///06 - Initial Assessment
    InitialAssessment,
    ///07 - Functional Goals
    FunctionalGoals,
    ///08 - Plan of Treatment
    PlanOfTreatment,
    ///09 - Progress Report
    ProgressReport,
    ///10 - Continued Treatment
    ContinuedTreatment,
    ///11 - Chemical Analysis
    ChemicalAnalysis,
    ///12 - Mechanical Properties
    MechanicalProperties,
    ///13 - Certified Test Report
    CertifiedTestReport,
    ///14 - Core Loss Test Report
    CoreLossTestReport,
    ///15 - Justification for Admission
    JustificationForAdmission,
    ///16 - Western Region 01 (Internal Revenue Service Summary of line 31 on 1040)
    Code16,
    ///17 - Return From Tax Payer Report (Internal Revenue Service Summary)
    Code17,
    ///18 - Note Receivable
    NoteReceivable,
    ///19 - Credit Report
    CreditReport,
    ///20 - Process Plan
    ProcessPlan,
    ///21 - Recovery Plan
    RecoveryPlan,
    ///22 - Functional Plan
    FunctionalPlan,
    ///23 - Contractual Plan
    ContractualPlan,
    ///24 - Non-Contractual Plan
    NonContractualPlan,
    ///25 - Purchase Plan
    PurchasePlan,
    ///26 - Production Plan
    ProductionPlan,
    ///27 - Contract Delivery Schedule
    ContractDeliverySchedule,
    ///28 - Master Delivery Schedule
    MasterDeliverySchedule,
    ///29 - Assembly Plan
    AssemblyPlan,
    ///30 - Lead Time Chart
    LeadTimeChart,
    ///31 - Master Schedule
    MasterSchedule,
    ///32 - Master Production Schedule
    MasterProductionSchedule,
    ///33 - Manufacturing Flow Diagram
    ManufacturingFlowDiagram,
    ///34 - Lot Release Plan
    LotReleasePlan,
    ///35 - Subcontractor Plan
    SubcontractorPlan,
    ///36 - Development Plan
    DevelopmentPlan,
    ///37 - Lease Agreement
    LeaseAgreement,
    ///38 - Court Judgment
    CourtJudgment,
    ///39 - Lottery Winning Letter
    LotteryWinningLetter,
    ///40 - Curb Side Inspection
    CurbSideInspection,
    ///41 - Statistical Model
    StatisticalModel,
    ///42 - Interior and Exterior Inspection
    InteriorAndExteriorInspection,
    ///43 - Non-Inspection Report
    NonInspectionReport,
    ///44 - Review Appraisal
    ReviewAppraisal,
    ///45 - Investor Guidelines
    InvestorGuidelines,
    ///46 - Sole Discretion Inspection
    SoleDiscretionInspection,
    ///47 - Brokers Price Opinion
    BrokersPriceOpinion,
    ///48 - Social Security Benefit Letter
    SocialSecurityBenefitLetter,
    ///49 - Divorce Decree
    DivorceDecree,
    ///50 - Contract
    Contract,
    ///51 - Gift Letter
    GiftLetter,
    ///52 - Will
    Will,
    ///53 - Trust Agreement
    TrustAgreement,
    ///54 - Award Letter
    AwardLetter,
    ///55 - Rental Agreement
    RentalAgreement,
    ///56 - Preliminary Real Estate Settlement Statement
    PreliminaryRealEstateSettlementStatement,
    ///57 - Income Statement (Internal Revenue Service Form 1099)
    Code57,
    ///58 - Utility Bill
    UtilityBill,
    ///59 - Benefit Letter
    BenefitLetter,
    ///60 - Canceled Check
    CanceledCheck,
    ///61 - Individual Tax Return (Internal Revenue Service Form 1040)
    Code61,
    ///62 - Asset Account Statement
    AssetAccountStatement,
    ///63 - Statement of Profit and Loss
    StatementOfProfitAndLoss,
    ///64 - Partner Share of Income, Credit, Deductions, (Internal Revenue Service Form K1)
    Code64,
    ///65 - Partnership Tax Return (Internal Revenue Service Form 1120)
    Code65,
    ///66 - Pay Stub
    PayStub,
    ///67 - Wage and Tax Statement (Internal Revenue Service Form W2)
    Code67,
    ///68 - Year End Statement
    YearEndStatement,
    ///69 - Bank Statement
    BankStatement,
    ///70 - Verification of Loan/Installment Debt Form
    VerificationOfLoanInstallmentDebtForm,
    ///71 - Verification of Deposit Form
    VerificationOfDepositForm,
    ///72 - Verification of Mortgage/Rent Form
    VerificationOfMortgageRentForm,
    ///73 - Verification of Employment Form
    VerificationOfEmploymentForm,
    ///74 - Corporation Tax Return (Internal Revenue Service Form 1065)
    Code74,
    ///75 - Title Certificates
    TitleCertificates,
    ///76 - Real Estate Settlement Statement (Housing and Urban Development Form - 1 "HUD1")
    Code76,
    ///77 - Support Data for Verification
    SupportDataForVerification,
    ///78 - Visa/Export License
    VisaExportLicense,
    ///79 - Multi-Country Textile Declaration
    MultiCountryTextileDeclaration,
    ///80 - Single Country Textile Declaration
    SingleCountryTextileDeclaration,
    ///81 - Negative Textile Declaration
    NegativeTextileDeclaration,
    ///82 - Endangered Species Convention on International Trade in Endangered Species (CITES)
    Code82,
    ///83 - Trademark Release
    TrademarkRelease,
    ///84 - Water Resistance Statement
    WaterResistanceStatement,
    ///85 - Certificate of Ceramicware Ceramics Commodities Inspection Bureau (CCIB)
    Code85,
    ///86 - Wearing Apparel Detail Sheet (WADS)
    Code86,
    ///87 - Interim Footwear Invoice
    InterimFootwearInvoice,
    ///88 - Impact Resistance Statement
    ImpactResistanceStatement,
    ///89 - Toxic Substance Compliance Statement
    ToxicSubstanceComplianceStatement,
    ///90 - Foreign Shippers Declaration
    ForeignShippersDeclaration,
    ///91 - Veterinarian Certificate
    VeterinarianCertificate,
    ///92 - Child Labor Certificate
    ChildLaborCertificate,
    ///93 - Prison Labor Certificate
    PrisonLaborCertificate,
    ///94 - Purchase Order Copy
    PurchaseOrderCopy,
    ///95 - Product Analysis
    ProductAnalysis,
    ///96 - American Automotive Labeling Act Certificate
    AmericanAutomotiveLabelingActCertificate,
    ///97 - Broker Market Analysis
    BrokerMarketAnalysis,
    ///A1 - Air Emissions Statements
    AirEmissionsStatements,
    ///A2 - Anti-Dumping Gasoline Program Report
    AntiDumpingGasolineProgramReport,
    ///A3 - Allergies/Sensitivities Document
    AllergiesSensitivitiesDocument,
    ///A4 - Autopsy Report
    AutopsyReport,
    ///AA - Agent Inventory Report
    AgentInventoryReport,
    ///AB - Assembly Drawing
    AssemblyDrawing,
    ///AC - Assay Certificate
    AssayCertificate,
    ///AD - Agent/Distributor Inventory Report
    AgentDistributorInventoryReport,
    ///AE - Attachment
    Attachment,
    ///AF - Aid Form
    AidForm,
    ///AG - Actual
    Actual,
    ///AH - Easement Report
    EasementReport,
    ///AI - Conditions, Covenant, and Restrictions Report
    CodeAI,
    ///AJ - Impound Account Escrow Analysis Report
    ImpoundAccountEscrowAnalysisReport,
    ///AK - Closing Escrow Analysis Report
    ClosingEscrowAnalysisReport,
    ///AL - Reserved Escrow Analysis Report
    ReservedEscrowAnalysisReport,
    ///AM - Ambulance Certification
    AmbulanceCertification,
    ///AN - Title Policy
    TitlePolicy,
    ///AO - Average Outgoing Quality Report
    AverageOutgoingQualityReport,
    ///AP - Advanced Problem Notification
    AdvancedProblemNotification,
    ///AQ - Housing and Urban Development (HUD) 1 Report
    CodeAQ,
    ///AR - Asset Reclassification Extension Request
    AssetReclassificationExtensionRequest,
    ///AS - Admission Summary
    AdmissionSummary,
    ///AT - Purchase Order Attachment
    PurchaseOrderAttachment,
    ///AU - Automobile Claim Report
    AutomobileClaimReport,
    ///AV - Averaging Areas Report
    AveragingAreasReport,
    ///AW - Air Waybill
    AirWaybill,
    ///AX - Asset Reclassification Extension Response
    AssetReclassificationExtensionResponse,
    ///AY - Tax Certificate
    TaxCertificate,
    ///AZ - Home Owner Authorization
    HomeOwnerAuthorization,
    ///B1 - Batch Report
    BatchReport,
    ///B2 - Prescription
    Prescription,
    ///B3 - Physician Order
    PhysicianOrder,
    ///B4 - Referral Form
    ReferralForm,
    ///BA - Budget
    Budget,
    ///BB - Buy or Sell Exchange Contract Status Statement
    BuyOrSellExchangeContractStatusStatement,
    ///BC - Bill of Lading Copy
    BillOfLadingCopy,
    ///BE - Benzene Content Averaging Report
    BenzeneContentAveragingReport,
    ///BF - Bailment Warehouse Withdrawal Request
    BailmentWarehouseWithdrawalRequest,
    ///BL - Bill of Lading
    BillOfLading,
    ///BM - Bill of Material
    BillOfMaterial,
    ///BN - Beneficiary Certificate
    BeneficiaryCertificate,
    ///BO - Bill of Lading Original
    BillOfLadingOriginal,
    ///BR - Benchmark Testing Results
    BenchmarkTestingResults,
    ///BS - Baseline
    Baseline,
    ///BT - Blanket Test Results
    BlanketTestResults,
    ///BW - Bill of Sale
    BillOfSale,
    ///BY - Biennial Report
    BiennialReport,
    ///C1 - Cost Data Summary
    CostDataSummary,
    ///C2 - Functional Cost and Hour
    FunctionalCostAndHour,
    ///C3 - Progress Curve
    ProgressCurve,
    ///C4 - Plant-Wide Data
    PlantWideData,
    ///C5 - Certified Cost and Price Data
    CertifiedCostAndPriceData,
    ///C6 - Wage Determination
    WageDetermination,
    ///C7 - Credit Transfer Summary Report
    CreditTransferSummaryReport,
    ///C8 - Chemical/Radiological Report
    ChemicalRadiologicalReport,
    ///C9 - Certification/Authorization Document
    CertificationAuthorizationDocument,
    ///CA - Certificate of Analysis
    CertificateOfAnalysis,
    ///CB - Chiropractic Justification
    ChiropracticJustification,
    ///CC - C.A.A. Certificate of Conformance (British CAA)
    CodeCC,
    ///CD - Customer/Distributor Inventory Report
    CustomerDistributorInventoryReport,
    ///CE - Constable Report
    ConstableReport,
    ///CF - Capability
    Capability,
    ///CG - Certificate of Origin
    CertificateOfOrigin,
    ///CH - Certificate of Weight
    CertificateOfWeight,
    ///CI - Certificate of Inspection Report
    CertificateOfInspectionReport,
    ///CJ - Complications Document
    ComplicationsDocument,
    ///CK - Consent Form(s)
    CodeCK,
    ///CL - Cable
    Cable,
    ///CM - Customer/Manufacturer Inventory Report
    CustomerManufacturerInventoryReport,
    ///CN - Customer's Report of Nonconformance
    CustomersReportOfNonconformance,
    ///CO - Consignment Order
    ConsignmentOrder,
    ///CP - Certificate of Compliance (Material Certification)
    CodeCP,
    ///CQ - County Record
    CountyRecord,
    ///CR - Letter of Credit
    LetterOfCredit,
    ///CS - Consigned Inventory Sales Report
    ConsignedInventorySalesReport,
    ///CT - Certification
    Certification,
    ///CU - Customer Notification Letter
    CustomerNotificationLetter,
    ///CV - Change of Hospice Benefit
    ChangeOfHospiceBenefit,
    ///CW - Corrective Work Order
    CorrectiveWorkOrder,
    ///CX - Cost/Schedule Status Report (C/SSR)
    CodeCX,
    ///CY - Contract Funds Status Report (CFSR)
    CodeCY,
    ///CZ - Campus Police Report
    CampusPoliceReport,
    ///D2 - Drug Profile Document
    DrugProfileDocument,
    ///DA - Dental Models
    DentalModels,
    ///DB - Durable Medical Equipment Prescription
    DurableMedicalEquipmentPrescription,
    ///DC - Distributor/Customer Inventory Report
    DistributorCustomerInventoryReport,
    ///DD - Distributor Inventory Report
    DistributorInventoryReport,
    ///DE - Certificate of Quality
    CertificateOfQuality,
    ///DF - DA59 Special Customs Invoice for South Africa
    Da59SpecialCustomsInvoiceForSouthAfrica,
    ///DG - Diagnostic Report
    DiagnosticReport,
    ///DH - Nitrogen Certificate
    NitrogenCertificate,
    ///DI - Directory
    Directory,
    ///DJ - Discharge Monitoring Report
    DischargeMonitoringReport,
    ///DK - Drawback Affidavit
    DrawbackAffidavit,
    ///DL - Draft and Transmittal Letter
    DraftAndTransmittalLetter,
    ///DM - Distributor/Manufacturer Inventory Report
    DistributorManufacturerInventoryReport,
    ///DN - Deviation/Nonconformance Test Results and Request for Action
    DeviationNonconformanceTestResultsAndRequestForAction,
    ///DQ - Delinquency
    Delinquency,
    ///DR - Datalog Report
    DatalogReport,
    ///DS - Discharge Summary
    DischargeSummary,
    ///DT - Department of Transportation
    DepartmentOfTransportation,
    ///DU - Commercial
    Commercial,
    ///DV - Condominium
    Condominium,
    ///DW - Drawing(s)
    CodeDW,
    ///E1 - Exporter's Certificate and Agreement
    ExportersCertificateAndAgreement,
    ///EA - Electrical Average Outgoing Quality Report
    ElectricalAverageOutgoingQualityReport,
    ///EB - Explanation of Benefits (Coordination of Benefits or Medicare Secondary Payor)
    CodeEB,
    ///EC - Engineering Change Order
    EngineeringChangeOrder,
    ///ED - Environmental Exposure Document
    EnvironmentalExposureDocument,
    ///EH - Election of Hospice Benefit
    ElectionOfHospiceBenefit,
    ///EL - Eligibility
    Eligibility,
    ///EP - Experimental Material Purchase Order
    ExperimentalMaterialPurchaseOrder,
    ///ER - Engineering Change Request
    EngineeringChangeRequest,
    ///ES - Source Selection Plan
    SourceSelectionPlan,
    ///EX - Shippers Export Declaration
    ShippersExportDeclaration,
    ///EY - Barrel for Barrel Exchange Contract Status Statement
    BarrelForBarrelExchangeContractStatusStatement,
    ///F1 - Cost Performance Report (CPR) Format 1
    CodeF1,
    ///F2 - Cost Performance Report (CPR) Format 2
    CodeF2,
    ///F3 - Cost Performance Report (CPR) Format 3
    CodeF3,
    ///F4 - Cost Performance Report (CPR) Format 4
    CodeF4,
    ///F5 - Cost Performance Report (CPR) Format 5
    CodeF5,
    ///F6 - Transportation Carrier Inspection Report
    TransportationCarrierInspectionReport,
    ///F7 - Government Inspection Report
    GovernmentInspectionReport,
    ///F8 - Inspection Waiver (Written)
    CodeF8,
    ///F9 - Inspection Waiver (Oral)
    CodeF9,
    ///FB - Federal Bureau of Investigation
    FederalBureauOfInvestigation,
    ///FC - Fumigation Certificate
    FumigationCertificate,
    ///FD - Federal Specification Compliance
    FederalSpecificationCompliance,
    ///FE - Federal Emergency Management Agency
    FederalEmergencyManagementAgency,
    ///FH - Limitation of Heavy Elements
    LimitationOfHeavyElements,
    ///FI - Fire Report
    FireReport,
    ///FM - Family Medical History Document
    FamilyMedicalHistoryDocument,
    ///FO - Post-Operative Radiology Films
    PostOperativeRadiologyFilms,
    ///FR - Pre-Operative Radiology Films
    PreOperativeRadiologyFilms,
    ///FS - Certificate of Free Sale
    CertificateOfFreeSale,
    ///G1 - State Form
    StateForm,
    ///G2 - Clearance Letter
    ClearanceLetter,
    ///G3 - Background Release
    BackgroundRelease,
    ///G4 - Exam Results
    ExamResults,
    ///G5 - Prelicense Certificate
    PrelicenseCertificate,
    ///G6 - National Association of Securities Dealers Certification
    NationalAssociationOfSecuritiesDealersCertification,
    ///G7 - License Copy
    LicenseCopy,
    ///GP - Gas Processor's Report
    GasProcessorsReport,
    ///GT - Gas Transporter's Report
    GasTransportersReport,
    ///HC - Health Certificate
    HealthCertificate,
    ///HI - Hazardous Material Incident
    HazardousMaterialIncident,
    ///HP - History and Physical
    HistoryAndPhysical,
    ///HR - Health Clinic Records
    HealthClinicRecords,
    ///HW - Hazardous Waste Manifest
    HazardousWasteManifest,
    ///I2 - Consular Invoice
    ConsularInvoice,
    ///I3 - Customs Invoice
    CustomsInvoice,
    ///I4 - Forwarder's Invoice
    ForwardersInvoice,
    ///I5 - Immunization Record
    ImmunizationRecord,
    ///I6 - Carrier's Invoice
    CarriersInvoice,
    ///IA - Insurance Attachment
    InsuranceAttachment,
    ///IC - Insurance Certificate
    InsuranceCertificate,
    ///IM - Import License
    ImportLicense,
    ///IN - Inspection Request
    InspectionRequest,
    ///IP - Inventory Parameter Report
    InventoryParameterReport,
    ///IR - State School Immunization Records
    StateSchoolImmunizationRecords,
    ///IS - Index System
    IndexSystem,
    ///IT - Certified Inspection and Test Results
    CertifiedInspectionAndTestResults,
    ///IU - Inspection Result
    InspectionResult,
    ///IV - Invoice
    Invoice,
    ///JA - Certificate of Good Standing
    CertificateOfGoodStanding,
    ///JB - Tax Status Clearance
    TaxStatusClearance,
    ///JC - Consent to Use Name
    ConsentToUseName,
    ///JD - Certificate of Registration
    CertificateOfRegistration,
    ///JE - Certificate of Existence
    CertificateOfExistence,
    ///JF - Certificate of Status
    CertificateOfStatus,
    ///JG - Certificate of Name Change
    CertificateOfNameChange,
    ///JH - Certificate of Merger
    CertificateOfMerger,
    ///JI - Certificate of Significant Change
    CertificateOfSignificantChange,
    ///JK - Balance Sheet
    BalanceSheet,
    ///JL - Application of Name Reservation
    ApplicationOfNameReservation,
    ///JM - Schedule of Capital
    ScheduleOfCapital,
    ///JN - Foreign Tax Return
    ForeignTaxReturn,
    ///JO - Permit Application
    PermitApplication,
    ///JP - Admission Tax Return
    AdmissionTaxReturn,
    ///JQ - Addendum to Articles
    AddendumToArticles,
    ///JR - Articles and Amendments
    ArticlesAndAmendments,
    ///JS - Appointment of Commissioner as Registered Agent
    AppointmentOfCommissionerAsRegisteredAgent,
    ///JT - Certificate of Disclosure
    CertificateOfDisclosure,
    ///JV - Notice of Registered Office
    NoticeOfRegisteredOffice,
    ///JW - Notice of Directors
    NoticeOfDirectors,
    ///JX - Organization and First Biennial Report
    OrganizationAndFirstBiennialReport,
    ///JY - Agreement of Statutory Agent
    AgreementOfStatutoryAgent,
    ///JZ - Consent to Act
    ConsentToAct,
    ///KA - Contract Data Requirements List (CDRL)
    CodeKA,
    ///KC - Kosher Certificate
    KosherCertificate,
    ///KD - Engineering Drawing List
    EngineeringDrawingList,
    ///KE - Purchased Engineering Data List
    PurchasedEngineeringDataList,
    ///KF - Support Documents
    SupportDocuments,
    ///KG - Purchased Documents
    PurchasedDocuments,
    ///KH - Proposal Support Data
    ProposalSupportData,
    ///KI - Purchased Drawings
    PurchasedDrawings,
    ///KJ - Change Proposal Data
    ChangeProposalData,
    ///KY - Report of Assignment or Modification of Key Events
    ReportOfAssignmentOrModificationOfKeyEvents,
    ///KZ - Request for Assignment or Modification of Key Events
    RequestForAssignmentOrModificationOfKeyEvents,
    ///LA - Laboratory Results
    LaboratoryResults,
    ///LB - Legalized Bill of Lading
    LegalizedBillOfLading,
    ///LC - Location Inventory Report
    LocationInventoryReport,
    ///LD - Laboratory Quality Review Variation, Deviation
    CodeLD,
    ///LE - Latest Revised Estimate
    LatestRevisedEstimate,
    ///LG - Legalized Certificate of Origin
    LegalizedCertificateOfOrigin,
    ///LI - Legalized Invoice
    LegalizedInvoice,
    ///LO - Laboratory Quality Review Order, Waiver
    CodeLO,
    ///LP - Labor Plan
    LaborPlan,
    ///LR - Laboratory Quality Review Order, Deviation
    CodeLR,
    ///LS - Lease Settlement Statement
    LeaseSettlementStatement,
    ///LT - License Application Attachment
    LicenseApplicationAttachment,
    ///LW - Laboratory Quality Review Variation, Waiver
    CodeLW,
    ///M1 - Medical Record Attachment
    MedicalRecordAttachment,
    ///MA - Manufacturer/Agent Inventory Report
    ManufacturerAgentInventoryReport,
    ///MB - Manufacturer/Distributor Inventory Report
    ManufacturerDistributorInventoryReport,
    ///MC - Manufacturer/Customer Inventory Report
    ManufacturerCustomerInventoryReport,
    ///MD - Material Data Sheets
    MaterialDataSheets,
    ///ME - Major Deviation Request
    MajorDeviationRequest,
    ///MF - Manufacturing Specification
    ManufacturingSpecification,
    ///MG - Migrant Student Records Transfer System (MSRTS) Record
    CodeMG,
    ///MH - Report of Full Maintenance Period Detail
    ReportOfFullMaintenancePeriodDetail,
    ///MI - Mortgage Insurance Certification
    MortgageInsuranceCertification,
    ///MJ - Request for Maintenance Period Status
    RequestForMaintenancePeriodStatus,
    ///MK - Report of Maintenance Period Status
    ReportOfMaintenancePeriodStatus,
    ///ML - Request for Full Maintenance Period Detail
    RequestForFullMaintenancePeriodDetail,
    ///MM - Manufacturer Inventory Report
    ManufacturerInventoryReport,
    ///MN - Minor Deviation Request
    MinorDeviationRequest,
    ///MO - Manufacturer's Statement of Origin
    ManufacturersStatementOfOrigin,
    ///MP - Request for Establishment, Modification, or Cancellation of Maintenance Period
    CodeMP,
    ///MQ - Report of Establishment, Modification, or Cancellation of Maintenance Period
    CodeMQ,
    ///MR - Material Inspection and Receiving Report
    MaterialInspectionAndReceivingReport,
    ///MS - Material Safety Data Sheet
    MaterialSafetyDataSheet,
    ///MT - Models
    Models,
    ///MV - Metered Volumes
    MeteredVolumes,
    ///MZ - Motor Vehicle Report
    MotorVehicleReport,
    ///NA - National Insurance Crime Bureau Assignment
    NationalInsuranceCrimeBureauAssignment,
    ///NC - Certificate of Quantity
    CertificateOfQuantity,
    ///ND - Commercial Invoice
    CommercialInvoice,
    ///NI - National Insurance Crime Bureau
    NationalInsuranceCrimeBureau,
    ///NL - National Insurance Crime Bureau Total Loss
    NationalInsuranceCrimeBureauTotalLoss,
    ///NM - Monthly Contractor Financial Management Report
    MonthlyContractorFinancialManagementReport,
    ///NN - Nursing Notes
    NursingNotes,
    ///NO - National Insurance Crime Bureau Other than Theft
    NationalInsuranceCrimeBureauOtherThanTheft,
    ///NQ - Quarterly Contractor Financial Management Report
    QuarterlyContractorFinancialManagementReport,
    ///NR - NOx Emissions Averaging Report
    NOxEmissionsAveragingReport,
    ///NT - National Insurance Crime Bureau Total Theft
    NationalInsuranceCrimeBureauTotalTheft,
    ///OB - Operative Note
    OperativeNote,
    ///OC - Oxygen Content Averaging Report
    OxygenContentAveragingReport,
    ///OD - Orders and Treatments Document
    OrdersAndTreatmentsDocument,
    ///OE - Objective Physical Examination (including vital signs) Document
    CodeOE,
    ///OL - Ocean Bill of Lading
    OceanBillOfLading,
    ///OP - Outside Production Operation Sheet
    OutsideProductionOperationSheet,
    ///OR - Oil Storer's Report
    OilStorersReport,
    ///OS - Organization Breakdown Structure
    OrganizationBreakdownStructure,
    ///OT - Oil Transporter's Report
    OilTransportersReport,
    ///OX - Oxygen Therapy Certification
    OxygenTherapyCertification,
    ///OZ - Support Data for Claim
    SupportDataForClaim,
    ///P1 - Packing List
    PackingList,
    ///P2 - Protest
    Protest,
    ///P3 - Receipt
    Receipt,
    ///P4 - Pathology Report
    PathologyReport,
    ///P5 - Patient Medical History Document
    PatientMedicalHistoryDocument,
    ///P6 - Periodontal Charts
    PeriodontalCharts,
    ///P7 - Periodontal Reports
    PeriodontalReports,
    ///P8 - Property Claim Report
    PropertyClaimReport,
    ///PA - Part Drawing
    PartDrawing,
    ///PB - Product Catalog
    ProductCatalog,
    ///PC - Process Change Notice
    ProcessChangeNotice,
    ///PD - Proof of Delivery
    ProofOfDelivery,
    ///PE - Parenteral or Enteral Certification
    ParenteralOrEnteralCertification,
    ///PF - Product Specification
    ProductSpecification,
    ///PG - Packaging Specification
    PackagingSpecification,
    ///PH - Production History - Property Level
    ProductionHistoryPropertyLevel,
    ///PI - Product Availability Inquiry
    ProductAvailabilityInquiry,
    ///PJ - Purchasing Specification
    PurchasingSpecification,
    ///PK - Storage Information Inquiry
    StorageInformationInquiry,
    ///PL - Property Insurance Loss Register
    PropertyInsuranceLossRegister,
    ///PM - Proof of Insurance
    ProofOfInsurance,
    ///PN - Physical Therapy Notes
    PhysicalTherapyNotes,
    ///PO - Prosthetics or Orthotic Certification
    ProstheticsOrOrthoticCertification,
    ///PP - Proposal
    Proposal,
    ///PQ - Paramedical Results
    ParamedicalResults,
    ///PR - Purchase Report
    PurchaseReport,
    ///PS - Pipeline/Shipper Inventory Report
    PipelineShipperInventoryReport,
    ///PT - Inter-Plant Inventory Report
    InterPlantInventoryReport,
    ///PV - Police Report
    PoliceReport,
    ///PW - Production History - Well Level
    ProductionHistoryWellLevel,
    ///PX - Production, Injection and Disposition Report
    CodePX,
    ///PY - Physician's Report
    PhysiciansReport,
    ///PZ - Physical Therapy Certification
    PhysicalTherapyCertification,
    ///QC - Cause and Corrective Action Report
    CauseAndCorrectiveActionReport,
    ///QD - Quality Review Order, Purchasing
    CodeQD,
    ///QE - Quality Detail
    QualityDetail,
    ///QM - Quality Review Order, Manufacturing
    CodeQM,
    ///QR - Quality Report
    QualityReport,
    ///QS - Quality Review Order Supplement
    QualityReviewOrderSupplement,
    ///QT - Quality Summary
    QualitySummary,
    ///R1 - Reformulated Gasoline/Anti-Dumping Company Registration
    ReformulatedGasolineAntiDumpingCompanyRegistration,
    ///R2 - Reformulated Gasoline/Anti-Dumping Facility Registration
    ReformulatedGasolineAntiDumpingFacilityRegistration,
    ///R3 - Technical Information Package
    TechnicalInformationPackage,
    ///R4 - Purchased Technical Information Package
    PurchasedTechnicalInformationPackage,
    ///R5 - Technical Information
    Technical,
    ///R6 - Miscellaneous Information
    Miscellaneous,
    ///R7 - Compliance Review
    ComplianceReview,
    ///R9 - Accident
    Accident,
    ///RA - Revision Announcement
    RevisionAnnouncement,
    ///RB - Radiology Films
    RadiologyFilms,
    ///RC - Request for Cause and Corrective Action Report
    RequestForCauseAndCorrectiveActionReport,
    ///RD - Payment Bond
    PaymentBond,
    ///RE - Performance Bond
    PerformanceBond,
    ///RF - Reliability Fail Rate Report
    ReliabilityFailRateReport,
    ///RG - Residential
    Residential,
    ///RH - Bid Bond
    BidBond,
    ///RM - Request for Manufacturing Engineer Appraisal
    RequestForManufacturingEngineerAppraisal,
    ///RN - Supplier's Report of Nonconformance
    SuppliersReportOfNonconformance,
    ///RO - Regular Order
    RegularOrder,
    ///RR - Radiology Reports
    RadiologyReports,
    ///RT - Report of Tests and Analysis Report
    ReportOfTestsAndAnalysisReport,
    ///RV - Reid Vapor Pressure (RVP) Averaging Report
    CodeRV,
    ///RX - Renewable Oxygen Content Averaging Report
    RenewableOxygenContentAveragingReport,
    ///S1 - Supply and Shipment Status Report
    SupplyAndShipmentStatusReport,
    ///S2 - Supply Status Report
    SupplyStatusReport,
    ///S3 - Exception Supply Status Report
    ExceptionSupplyStatusReport,
    ///S4 - Exception Supply and Shipment Status Report
    ExceptionSupplyAndShipmentStatusReport,
    ///S5 - Product Quality Deficiency Report Category I
    ProductQualityDeficiencyReportCategoryI,
    ///S6 - Product Quality Deficiency Report Category II
    ProductQualityDeficiencyReportCategoryIi,
    ///S7 - "Walsh-Healey Act" Manufacturer or Regular Dealer
    CodeS7,
    ///S8 - Report of Findings
    ReportOfFindings,
    ///S9 - Representation
    Representation,
    ///SA - State Police Report
    StatePoliceReport,
    ///SB - Sample Approval and Rejection List
    SampleApprovalAndRejectionList,
    ///SC - Sanitary Certificate
    SanitaryCertificate,
    ///SD - Support Data for a Request for Quote
    SupportDataForARequestForQuote,
    ///SE - Security Police Report
    SecurityPoliceReport,
    ///SF - Contract Security Classification Specification
    ContractSecurityClassificationSpecification,
    ///SG - Symptoms Document
    SymptomsDocument,
    ///SH - Sheriff Report
    SheriffReport,
    ///SI - Seller Inventory Report
    SellerInventoryReport,
    ///SJ - Statement of Work
    StatementOfWork,
    ///SL - Sample Bale List
    SampleBaleList,
    ///SM - Shipping Manifests
    ShippingManifests,
    ///SN - Shipping Notice
    ShippingNotice,
    ///SO - Secretary Certificate
    SecretaryCertificate,
    ///SP - Specification
    Specification,
    ///SQ - Statistical Quality Documents
    StatisticalQualityDocuments,
    ///SR - Statistical Report
    StatisticalReport,
    ///SS - Seller Sales Report
    SellerSalesReport,
    ///ST - Student Educational Record (Transcript)
    CodeST,
    ///SU - Supplier's Certificate
    SuppliersCertificate,
    ///SV - Survey
    Survey,
    ///SW - Sea Waybill
    SeaWaybill,
    ///SX - Steamship Due Bill
    SteamshipDueBill,
    ///SY - Train Sheet
    TrainSheet,
    ///T1 - Title Bill
    TitleBill,
    ///T2 - Preliminary Title Work
    PreliminaryTitleWork,
    ///T3 - Loan Documents
    LoanDocuments,
    ///T4 - Tax Information
    Tax,
    ///T5 - Toxics Emissions Performance Averaging Report
    ToxicsEmissionsPerformanceAveragingReport,
    ///T6 - Toxics Release Inventory
    ToxicsReleaseInventory,
    ///T7 - Therapy Notes
    TherapyNotes,
    ///TA - Asset Support Inquiry
    AssetSupportInquiry,
    ///TB - Asset Support Advice
    AssetSupportAdvice,
    ///TC - Physical Inventory Request
    PhysicalInventoryRequest,
    ///TD - Asset Reclassification Response
    AssetReclassificationResponse,
    ///TE - Asset Reclassification Request
    AssetReclassificationRequest,
    ///TF - Transaction History Request
    TransactionHistoryRequest,
    ///TG - Two to Four Family
    TwoToFourFamily,
    ///TH - Total Theft Claim Report
    TotalTheftClaimReport,
    ///TI - Asset Status Inquiry
    AssetStatusInquiry,
    ///TJ - Asset Status Advice
    AssetStatusAdvice,
    ///TK - Logistics Transfer Inquiry
    LogisticsTransferInquiry,
    ///TL - Logistics Transfer Advice
    LogisticsTransferAdvice,
    ///TM - Stock Sale Report
    StockSaleReport,
    ///TN - Delayed Sale Report
    DelayedSaleReport,
    ///TO - Demand Report
    DemandReport,
    ///TP - Treatments Certificate
    TreatmentsCertificate,
    ///TQ - Storage Information Advice
    StorageInformationAdvice,
    ///TR - Transmittal Letter
    TransmittalLetter,
    ///TS - Sulfur, Olefins, and T90 Averaging Report
    CodeTS,
    ///TT - Title Transfer
    TitleTransfer,
    ///TX - Tax-exempt Certificate
    TaxExemptCertificate,
    ///U1 - Survey Report
    SurveyReport,
    ///UA - Union Agreement
    UnionAgreement,
    ///UB - Certificate of Designation of Registered Agent
    CertificateOfDesignationOfRegisteredAgent,
    ///UD - List of Officers and Directors
    ListOfOfficersAndDirectors,
    ///UE - Resolution and Consent Form
    ResolutionAndConsentForm,
    ///UF - Domestic Business Corporation Initial Report
    DomesticBusinessCorporationInitialReport,
    ///UG - Registered Agent Application
    RegisteredAgentApplication,
    ///UH - Articles of Incorporation
    ArticlesOfIncorporation,
    ///UI - Certificate of Compliance
    CertificateOfCompliance,
    ///UJ - Certificate of Authorization
    CertificateOfAuthorization,
    ///UK - Charter
    Charter,
    ///UL - Other Type of Report
    OtherTypeOfReport,
    ///UM - Affidavit of Acceptance
    AffidavitOfAcceptance,
    ///UN - Resolution Adopting Fictitious Name
    ResolutionAdoptingFictitiousName,
    ///UO - Trade Name Application
    TradeNameApplication,
    ///UP - Declaration of Solicitor
    DeclarationOfSolicitor,
    ///UQ - Memorandum of Association
    MemorandumOfAssociation,
    ///UR - Notice of Registered Agent
    NoticeOfRegisteredAgent,
    ///US - "BUY AMERICA" Certification of Compliance
    CodeUS,
    ///UU - Dissolution of Existing Registration
    DissolutionOfExistingRegistration,
    ///UV - Appointment of Statutory Agent
    AppointmentOfStatutoryAgent,
    ///UX - Regulatory Approval for Professional Association
    RegulatoryApprovalForProfessionalAssociation,
    ///UY - Initial Annual Report
    InitialAnnualReport,
    ///UZ - Certificate of Fact
    CertificateOfFact,
    ///V1 - Voter Registration Application
    VoterRegistrationApplication,
    ///V2 - Voter Registration Application Disposition
    VoterRegistrationApplicationDisposition,
    ///V3 - Voter Information Record
    VoterInformationRecord,
    ///V4 - Change of Name and/or Address
    ChangeOfNameAndOrAddress,
    ///V5 - Death Notification
    DeathNotification,
    ///V6 - Felony Conviction Notification
    FelonyConvictionNotification,
    ///V7 - Incompetency Notification
    IncompetencyNotification,
    ///VA - Variance Analysis
    VarianceAnalysis,
    ///VC - Volatile Organic Compounds (VOC) Emissions Averaging Report
    CodeVC,
    ///VD - Data Request for Vendor's Specifications or Drawings.
    DataRequestForVendorsSpecificationsOrDrawings,
    ///VM - Visual/Mechanical Average Outgoing Quality Report
    VisualMechanicalAverageOutgoingQualityReport,
    ///W1 - Safe Drinking Water Bacteriological Report
    SafeDrinkingWaterBacteriologicalReport,
    ///W2 - Safe Drinking Water Report
    SafeDrinkingWaterReport,
    ///WA - Fictitious Name Statement
    FictitiousNameStatement,
    ///WB - Work Breakdown Structure
    WorkBreakdownStructure,
    ///WC - Request for Assignment or Deletion of Work Candidate
    RequestForAssignmentOrDeletionOfWorkCandidate,
    ///WD - Report of Assignment or Deletion of Work Candidate
    ReportOfAssignmentOrDeletionOfWorkCandidate,
    ///WE - Business Conducted Prior to Qualification Form
    BusinessConductedPriorToQualificationForm,
    ///WF - By-Laws
    ByLaws,
    ///WG - Appointment of Agent for Service and Consent to Act
    AppointmentOfAgentForServiceAndConsentToAct,
    ///WH - Certificate of Name Clearance
    CertificateOfNameClearance,
    ///WI - Well Information
    Well,
    ///WP - Work Progress
    WorkProgress,
    ///WT - Well Test Information
    WellTest,
    ///X1 - Complete Appraisal
    CompleteAppraisal,
    ///X2 - Limited Appraisal
    LimitedAppraisal,
    ///X3 - Self-contained Report
    SelfContainedReport,
    ///X4 - Summary Report
    SummaryReport,
    ///X5 - Restricted Report
    RestrictedReport,
    ///XE - Equipment Test Results
    EquipmentTestResults,
    ///XP - Photographs
    Photographs,
    ///Y1 - Appraisal
    Appraisal,
    ///Y2 - Broker Price Opinion
    BrokerPriceOpinion,
    ///Y3 - Real Estate Property Information
    RealEstateProperty,
    ///ZA - Flood Determination Report
    FloodDeterminationReport,
    ///ZB - Conventional Ammunition Suspension Report
    ConventionalAmmunitionSuspensionReport,
    ///ZC - Self Monitoring Report
    SelfMonitoringReport,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl ReportTypeCode {
    pub fn code(&self) -> &str {
        {
            use ReportTypeCode::*;
            match self {
                ProductTransfer => "01",
                Resale => "02",
                ReportJustifyingTreatmentBeyondUtilizationGuidelines => "03",
                DrugsAdministered => "04",
                TreatmentDiagnosis => "05",
                InitialAssessment => "06",
                FunctionalGoals => "07",
                PlanOfTreatment => "08",
                ProgressReport => "09",
                ContinuedTreatment => "10",
                ChemicalAnalysis => "11",
                MechanicalProperties => "12",
                CertifiedTestReport => "13",
                CoreLossTestReport => "14",
                JustificationForAdmission => "15",
                Code16 => "16",
                Code17 => "17",
                NoteReceivable => "18",
                CreditReport => "19",
                ProcessPlan => "20",
                RecoveryPlan => "21",
                FunctionalPlan => "22",
                ContractualPlan => "23",
                NonContractualPlan => "24",
                PurchasePlan => "25",
                ProductionPlan => "26",
                ContractDeliverySchedule => "27",
                MasterDeliverySchedule => "28",
                AssemblyPlan => "29",
                LeadTimeChart => "30",
                MasterSchedule => "31",
                MasterProductionSchedule => "32",
                ManufacturingFlowDiagram => "33",
                LotReleasePlan => "34",
                SubcontractorPlan => "35",
                DevelopmentPlan => "36",
                LeaseAgreement => "37",
                CourtJudgment => "38",
                LotteryWinningLetter => "39",
                CurbSideInspection => "40",
                StatisticalModel => "41",
                InteriorAndExteriorInspection => "42",
                NonInspectionReport => "43",
                ReviewAppraisal => "44",
                InvestorGuidelines => "45",
                SoleDiscretionInspection => "46",
                BrokersPriceOpinion => "47",
                SocialSecurityBenefitLetter => "48",
                DivorceDecree => "49",
                Contract => "50",
                GiftLetter => "51",
                Will => "52",
                TrustAgreement => "53",
                AwardLetter => "54",
                RentalAgreement => "55",
                PreliminaryRealEstateSettlementStatement => "56",
                Code57 => "57",
                UtilityBill => "58",
                BenefitLetter => "59",
                CanceledCheck => "60",
                Code61 => "61",
                AssetAccountStatement => "62",
                StatementOfProfitAndLoss => "63",
                Code64 => "64",
                Code65 => "65",
                PayStub => "66",
                Code67 => "67",
                YearEndStatement => "68",
                BankStatement => "69",
                VerificationOfLoanInstallmentDebtForm => "70",
                VerificationOfDepositForm => "71",
                VerificationOfMortgageRentForm => "72",
                VerificationOfEmploymentForm => "73",
                Code74 => "74",
                TitleCertificates => "75",
                Code76 => "76",
                SupportDataForVerification => "77",
                VisaExportLicense => "78",
                MultiCountryTextileDeclaration => "79",
                SingleCountryTextileDeclaration => "80",
                NegativeTextileDeclaration => "81",
                Code82 => "82",
                TrademarkRelease => "83",
                WaterResistanceStatement => "84",
                Code85 => "85",
                Code86 => "86",
                InterimFootwearInvoice => "87",
                ImpactResistanceStatement => "88",
                ToxicSubstanceComplianceStatement => "89",
                ForeignShippersDeclaration => "90",
                VeterinarianCertificate => "91",
                ChildLaborCertificate => "92",
                PrisonLaborCertificate => "93",
                PurchaseOrderCopy => "94",
                ProductAnalysis => "95",
                AmericanAutomotiveLabelingActCertificate => "96",
                BrokerMarketAnalysis => "97",
                AirEmissionsStatements => "A1",
                AntiDumpingGasolineProgramReport => "A2",
                AllergiesSensitivitiesDocument => "A3",
                AutopsyReport => "A4",
                AgentInventoryReport => "AA",
                AssemblyDrawing => "AB",
                AssayCertificate => "AC",
                AgentDistributorInventoryReport => "AD",
                Attachment => "AE",
                AidForm => "AF",
                Actual => "AG",
                EasementReport => "AH",
                CodeAI => "AI",
                ImpoundAccountEscrowAnalysisReport => "AJ",
                ClosingEscrowAnalysisReport => "AK",
                ReservedEscrowAnalysisReport => "AL",
                AmbulanceCertification => "AM",
                TitlePolicy => "AN",
                AverageOutgoingQualityReport => "AO",
                AdvancedProblemNotification => "AP",
                CodeAQ => "AQ",
                AssetReclassificationExtensionRequest => "AR",
                AdmissionSummary => "AS",
                PurchaseOrderAttachment => "AT",
                AutomobileClaimReport => "AU",
                AveragingAreasReport => "AV",
                AirWaybill => "AW",
                AssetReclassificationExtensionResponse => "AX",
                TaxCertificate => "AY",
                HomeOwnerAuthorization => "AZ",
                BatchReport => "B1",
                Prescription => "B2",
                PhysicianOrder => "B3",
                ReferralForm => "B4",
                Budget => "BA",
                BuyOrSellExchangeContractStatusStatement => "BB",
                BillOfLadingCopy => "BC",
                BenzeneContentAveragingReport => "BE",
                BailmentWarehouseWithdrawalRequest => "BF",
                BillOfLading => "BL",
                BillOfMaterial => "BM",
                BeneficiaryCertificate => "BN",
                BillOfLadingOriginal => "BO",
                BenchmarkTestingResults => "BR",
                Baseline => "BS",
                BlanketTestResults => "BT",
                BillOfSale => "BW",
                BiennialReport => "BY",
                CostDataSummary => "C1",
                FunctionalCostAndHour => "C2",
                ProgressCurve => "C3",
                PlantWideData => "C4",
                CertifiedCostAndPriceData => "C5",
                WageDetermination => "C6",
                CreditTransferSummaryReport => "C7",
                ChemicalRadiologicalReport => "C8",
                CertificationAuthorizationDocument => "C9",
                CertificateOfAnalysis => "CA",
                ChiropracticJustification => "CB",
                CodeCC => "CC",
                CustomerDistributorInventoryReport => "CD",
                ConstableReport => "CE",
                Capability => "CF",
                CertificateOfOrigin => "CG",
                CertificateOfWeight => "CH",
                CertificateOfInspectionReport => "CI",
                ComplicationsDocument => "CJ",
                CodeCK => "CK",
                Cable => "CL",
                CustomerManufacturerInventoryReport => "CM",
                CustomersReportOfNonconformance => "CN",
                ConsignmentOrder => "CO",
                CodeCP => "CP",
                CountyRecord => "CQ",
                LetterOfCredit => "CR",
                ConsignedInventorySalesReport => "CS",
                Certification => "CT",
                CustomerNotificationLetter => "CU",
                ChangeOfHospiceBenefit => "CV",
                CorrectiveWorkOrder => "CW",
                CodeCX => "CX",
                CodeCY => "CY",
                CampusPoliceReport => "CZ",
                DrugProfileDocument => "D2",
                DentalModels => "DA",
                DurableMedicalEquipmentPrescription => "DB",
                DistributorCustomerInventoryReport => "DC",
                DistributorInventoryReport => "DD",
                CertificateOfQuality => "DE",
                Da59SpecialCustomsInvoiceForSouthAfrica => "DF",
                DiagnosticReport => "DG",
                NitrogenCertificate => "DH",
                Directory => "DI",
                DischargeMonitoringReport => "DJ",
                DrawbackAffidavit => "DK",
                DraftAndTransmittalLetter => "DL",
                DistributorManufacturerInventoryReport => "DM",
                DeviationNonconformanceTestResultsAndRequestForAction => "DN",
                Delinquency => "DQ",
                DatalogReport => "DR",
                DischargeSummary => "DS",
                DepartmentOfTransportation => "DT",
                Commercial => "DU",
                Condominium => "DV",
                CodeDW => "DW",
                ExportersCertificateAndAgreement => "E1",
                ElectricalAverageOutgoingQualityReport => "EA",
                CodeEB => "EB",
                EngineeringChangeOrder => "EC",
                EnvironmentalExposureDocument => "ED",
                ElectionOfHospiceBenefit => "EH",
                Eligibility => "EL",
                ExperimentalMaterialPurchaseOrder => "EP",
                EngineeringChangeRequest => "ER",
                SourceSelectionPlan => "ES",
                ShippersExportDeclaration => "EX",
                BarrelForBarrelExchangeContractStatusStatement => "EY",
                CodeF1 => "F1",
                CodeF2 => "F2",
                CodeF3 => "F3",
                CodeF4 => "F4",
                CodeF5 => "F5",
                TransportationCarrierInspectionReport => "F6",
                GovernmentInspectionReport => "F7",
                CodeF8 => "F8",
                CodeF9 => "F9",
                FederalBureauOfInvestigation => "FB",
                FumigationCertificate => "FC",
                FederalSpecificationCompliance => "FD",
                FederalEmergencyManagementAgency => "FE",
                LimitationOfHeavyElements => "FH",
                FireReport => "FI",
                FamilyMedicalHistoryDocument => "FM",
                PostOperativeRadiologyFilms => "FO",
                PreOperativeRadiologyFilms => "FR",
                CertificateOfFreeSale => "FS",
                StateForm => "G1",
                ClearanceLetter => "G2",
                BackgroundRelease => "G3",
                ExamResults => "G4",
                PrelicenseCertificate => "G5",
                NationalAssociationOfSecuritiesDealersCertification => "G6",
                LicenseCopy => "G7",
                GasProcessorsReport => "GP",
                GasTransportersReport => "GT",
                HealthCertificate => "HC",
                HazardousMaterialIncident => "HI",
                HistoryAndPhysical => "HP",
                HealthClinicRecords => "HR",
                HazardousWasteManifest => "HW",
                ConsularInvoice => "I2",
                CustomsInvoice => "I3",
                ForwardersInvoice => "I4",
                ImmunizationRecord => "I5",
                CarriersInvoice => "I6",
                InsuranceAttachment => "IA",
                InsuranceCertificate => "IC",
                ImportLicense => "IM",
                InspectionRequest => "IN",
                InventoryParameterReport => "IP",
                StateSchoolImmunizationRecords => "IR",
                IndexSystem => "IS",
                CertifiedInspectionAndTestResults => "IT",
                InspectionResult => "IU",
                Invoice => "IV",
                CertificateOfGoodStanding => "JA",
                TaxStatusClearance => "JB",
                ConsentToUseName => "JC",
                CertificateOfRegistration => "JD",
                CertificateOfExistence => "JE",
                CertificateOfStatus => "JF",
                CertificateOfNameChange => "JG",
                CertificateOfMerger => "JH",
                CertificateOfSignificantChange => "JI",
                BalanceSheet => "JK",
                ApplicationOfNameReservation => "JL",
                ScheduleOfCapital => "JM",
                ForeignTaxReturn => "JN",
                PermitApplication => "JO",
                AdmissionTaxReturn => "JP",
                AddendumToArticles => "JQ",
                ArticlesAndAmendments => "JR",
                AppointmentOfCommissionerAsRegisteredAgent => "JS",
                CertificateOfDisclosure => "JT",
                NoticeOfRegisteredOffice => "JV",
                NoticeOfDirectors => "JW",
                OrganizationAndFirstBiennialReport => "JX",
                AgreementOfStatutoryAgent => "JY",
                ConsentToAct => "JZ",
                CodeKA => "KA",
                KosherCertificate => "KC",
                EngineeringDrawingList => "KD",
                PurchasedEngineeringDataList => "KE",
                SupportDocuments => "KF",
                PurchasedDocuments => "KG",
                ProposalSupportData => "KH",
                PurchasedDrawings => "KI",
                ChangeProposalData => "KJ",
                ReportOfAssignmentOrModificationOfKeyEvents => "KY",
                RequestForAssignmentOrModificationOfKeyEvents => "KZ",
                LaboratoryResults => "LA",
                LegalizedBillOfLading => "LB",
                LocationInventoryReport => "LC",
                CodeLD => "LD",
                LatestRevisedEstimate => "LE",
                LegalizedCertificateOfOrigin => "LG",
                LegalizedInvoice => "LI",
                CodeLO => "LO",
                LaborPlan => "LP",
                CodeLR => "LR",
                LeaseSettlementStatement => "LS",
                LicenseApplicationAttachment => "LT",
                CodeLW => "LW",
                MedicalRecordAttachment => "M1",
                ManufacturerAgentInventoryReport => "MA",
                ManufacturerDistributorInventoryReport => "MB",
                ManufacturerCustomerInventoryReport => "MC",
                MaterialDataSheets => "MD",
                MajorDeviationRequest => "ME",
                ManufacturingSpecification => "MF",
                CodeMG => "MG",
                ReportOfFullMaintenancePeriodDetail => "MH",
                MortgageInsuranceCertification => "MI",
                RequestForMaintenancePeriodStatus => "MJ",
                ReportOfMaintenancePeriodStatus => "MK",
                RequestForFullMaintenancePeriodDetail => "ML",
                ManufacturerInventoryReport => "MM",
                MinorDeviationRequest => "MN",
                ManufacturersStatementOfOrigin => "MO",
                CodeMP => "MP",
                CodeMQ => "MQ",
                MaterialInspectionAndReceivingReport => "MR",
                MaterialSafetyDataSheet => "MS",
                Models => "MT",
                MeteredVolumes => "MV",
                MotorVehicleReport => "MZ",
                NationalInsuranceCrimeBureauAssignment => "NA",
                CertificateOfQuantity => "NC",
                CommercialInvoice => "ND",
                NationalInsuranceCrimeBureau => "NI",
                NationalInsuranceCrimeBureauTotalLoss => "NL",
                MonthlyContractorFinancialManagementReport => "NM",
                NursingNotes => "NN",
                NationalInsuranceCrimeBureauOtherThanTheft => "NO",
                QuarterlyContractorFinancialManagementReport => "NQ",
                NOxEmissionsAveragingReport => "NR",
                NationalInsuranceCrimeBureauTotalTheft => "NT",
                OperativeNote => "OB",
                OxygenContentAveragingReport => "OC",
                OrdersAndTreatmentsDocument => "OD",
                CodeOE => "OE",
                OceanBillOfLading => "OL",
                OutsideProductionOperationSheet => "OP",
                OilStorersReport => "OR",
                OrganizationBreakdownStructure => "OS",
                OilTransportersReport => "OT",
                OxygenTherapyCertification => "OX",
                SupportDataForClaim => "OZ",
                PackingList => "P1",
                Protest => "P2",
                Receipt => "P3",
                PathologyReport => "P4",
                PatientMedicalHistoryDocument => "P5",
                PeriodontalCharts => "P6",
                PeriodontalReports => "P7",
                PropertyClaimReport => "P8",
                PartDrawing => "PA",
                ProductCatalog => "PB",
                ProcessChangeNotice => "PC",
                ProofOfDelivery => "PD",
                ParenteralOrEnteralCertification => "PE",
                ProductSpecification => "PF",
                PackagingSpecification => "PG",
                ProductionHistoryPropertyLevel => "PH",
                ProductAvailabilityInquiry => "PI",
                PurchasingSpecification => "PJ",
                StorageInformationInquiry => "PK",
                PropertyInsuranceLossRegister => "PL",
                ProofOfInsurance => "PM",
                PhysicalTherapyNotes => "PN",
                ProstheticsOrOrthoticCertification => "PO",
                Proposal => "PP",
                ParamedicalResults => "PQ",
                PurchaseReport => "PR",
                PipelineShipperInventoryReport => "PS",
                InterPlantInventoryReport => "PT",
                PoliceReport => "PV",
                ProductionHistoryWellLevel => "PW",
                CodePX => "PX",
                PhysiciansReport => "PY",
                PhysicalTherapyCertification => "PZ",
                CauseAndCorrectiveActionReport => "QC",
                CodeQD => "QD",
                QualityDetail => "QE",
                CodeQM => "QM",
                QualityReport => "QR",
                QualityReviewOrderSupplement => "QS",
                QualitySummary => "QT",
                ReformulatedGasolineAntiDumpingCompanyRegistration => "R1",
                ReformulatedGasolineAntiDumpingFacilityRegistration => "R2",
                TechnicalInformationPackage => "R3",
                PurchasedTechnicalInformationPackage => "R4",
                Technical => "R5",
                Miscellaneous => "R6",
                ComplianceReview => "R7",
                Accident => "R9",
                RevisionAnnouncement => "RA",
                RadiologyFilms => "RB",
                RequestForCauseAndCorrectiveActionReport => "RC",
                PaymentBond => "RD",
                PerformanceBond => "RE",
                ReliabilityFailRateReport => "RF",
                Residential => "RG",
                BidBond => "RH",
                RequestForManufacturingEngineerAppraisal => "RM",
                SuppliersReportOfNonconformance => "RN",
                RegularOrder => "RO",
                RadiologyReports => "RR",
                ReportOfTestsAndAnalysisReport => "RT",
                CodeRV => "RV",
                RenewableOxygenContentAveragingReport => "RX",
                SupplyAndShipmentStatusReport => "S1",
                SupplyStatusReport => "S2",
                ExceptionSupplyStatusReport => "S3",
                ExceptionSupplyAndShipmentStatusReport => "S4",
                ProductQualityDeficiencyReportCategoryI => "S5",
                ProductQualityDeficiencyReportCategoryIi => "S6",
                CodeS7 => "S7",
                ReportOfFindings => "S8",
                Representation => "S9",
                StatePoliceReport => "SA",
                SampleApprovalAndRejectionList => "SB",
                SanitaryCertificate => "SC",
                SupportDataForARequestForQuote => "SD",
                SecurityPoliceReport => "SE",
                ContractSecurityClassificationSpecification => "SF",
                SymptomsDocument => "SG",
                SheriffReport => "SH",
                SellerInventoryReport => "SI",
                StatementOfWork => "SJ",
                SampleBaleList => "SL",
                ShippingManifests => "SM",
                ShippingNotice => "SN",
                SecretaryCertificate => "SO",
                Specification => "SP",
                StatisticalQualityDocuments => "SQ",
                StatisticalReport => "SR",
                SellerSalesReport => "SS",
                CodeST => "ST",
                SuppliersCertificate => "SU",
                Survey => "SV",
                SeaWaybill => "SW",
                SteamshipDueBill => "SX",
                TrainSheet => "SY",
                TitleBill => "T1",
                PreliminaryTitleWork => "T2",
                LoanDocuments => "T3",
                Tax => "T4",
                ToxicsEmissionsPerformanceAveragingReport => "T5",
                ToxicsReleaseInventory => "T6",
                TherapyNotes => "T7",
                AssetSupportInquiry => "TA",
                AssetSupportAdvice => "TB",
                PhysicalInventoryRequest => "TC",
                AssetReclassificationResponse => "TD",
                AssetReclassificationRequest => "TE",
                TransactionHistoryRequest => "TF",
                TwoToFourFamily => "TG",
                TotalTheftClaimReport => "TH",
                AssetStatusInquiry => "TI",
                AssetStatusAdvice => "TJ",
                LogisticsTransferInquiry => "TK",
                LogisticsTransferAdvice => "TL",
                StockSaleReport => "TM",
                DelayedSaleReport => "TN",
                DemandReport => "TO",
                TreatmentsCertificate => "TP",
                StorageInformationAdvice => "TQ",
                TransmittalLetter => "TR",
                CodeTS => "TS",
                TitleTransfer => "TT",
                TaxExemptCertificate => "TX",
                SurveyReport => "U1",
                UnionAgreement => "UA",
                CertificateOfDesignationOfRegisteredAgent => "UB",
                ListOfOfficersAndDirectors => "UD",
                ResolutionAndConsentForm => "UE",
                DomesticBusinessCorporationInitialReport => "UF",
                RegisteredAgentApplication => "UG",
                ArticlesOfIncorporation => "UH",
                CertificateOfCompliance => "UI",
                CertificateOfAuthorization => "UJ",
                Charter => "UK",
                OtherTypeOfReport => "UL",
                AffidavitOfAcceptance => "UM",
                ResolutionAdoptingFictitiousName => "UN",
                TradeNameApplication => "UO",
                DeclarationOfSolicitor => "UP",
                MemorandumOfAssociation => "UQ",
                NoticeOfRegisteredAgent => "UR",
                CodeUS => "US",
                DissolutionOfExistingRegistration => "UU",
                AppointmentOfStatutoryAgent => "UV",
                RegulatoryApprovalForProfessionalAssociation => "UX",
                InitialAnnualReport => "UY",
                CertificateOfFact => "UZ",
                VoterRegistrationApplication => "V1",
                VoterRegistrationApplicationDisposition => "V2",
                VoterInformationRecord => "V3",
                ChangeOfNameAndOrAddress => "V4",
                DeathNotification => "V5",
                FelonyConvictionNotification => "V6",
                IncompetencyNotification => "V7",
                VarianceAnalysis => "VA",
                CodeVC => "VC",
                DataRequestForVendorsSpecificationsOrDrawings => "VD",
                VisualMechanicalAverageOutgoingQualityReport => "VM",
                SafeDrinkingWaterBacteriologicalReport => "W1",
                SafeDrinkingWaterReport => "W2",
                FictitiousNameStatement => "WA",
                WorkBreakdownStructure => "WB",
                RequestForAssignmentOrDeletionOfWorkCandidate => "WC",
                ReportOfAssignmentOrDeletionOfWorkCandidate => "WD",
                BusinessConductedPriorToQualificationForm => "WE",
                ByLaws => "WF",
                AppointmentOfAgentForServiceAndConsentToAct => "WG",
                CertificateOfNameClearance => "WH",
                Well => "WI",
                WorkProgress => "WP",
                WellTest => "WT",
                CompleteAppraisal => "X1",
                LimitedAppraisal => "X2",
                SelfContainedReport => "X3",
                SummaryReport => "X4",
                RestrictedReport => "X5",
                EquipmentTestResults => "XE",
                Photographs => "XP",
                Appraisal => "Y1",
                BrokerPriceOpinion => "Y2",
                RealEstateProperty => "Y3",
                FloodDeterminationReport => "ZA",
                ConventionalAmmunitionSuspensionReport => "ZB",
                SelfMonitoringReport => "ZC",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ReportTypeCode> {
        use ReportTypeCode::*;
        match code {
            b"01" => Some(ProductTransfer),
            b"02" => Some(Resale),
            b"03" => Some(ReportJustifyingTreatmentBeyondUtilizationGuidelines),
            b"04" => Some(DrugsAdministered),
            b"05" => Some(TreatmentDiagnosis),
            b"06" => Some(InitialAssessment),
            b"07" => Some(FunctionalGoals),
            b"08" => Some(PlanOfTreatment),
            b"09" => Some(ProgressReport),
            b"10" => Some(ContinuedTreatment),
            b"11" => Some(ChemicalAnalysis),
            b"12" => Some(MechanicalProperties),
            b"13" => Some(CertifiedTestReport),
            b"14" => Some(CoreLossTestReport),
            b"15" => Some(JustificationForAdmission),
            b"16" => Some(Code16),
            b"17" => Some(Code17),
            b"18" => Some(NoteReceivable),
            b"19" => Some(CreditReport),
            b"20" => Some(ProcessPlan),
            b"21" => Some(RecoveryPlan),
            b"22" => Some(FunctionalPlan),
            b"23" => Some(ContractualPlan),
            b"24" => Some(NonContractualPlan),
            b"25" => Some(PurchasePlan),
            b"26" => Some(ProductionPlan),
            b"27" => Some(ContractDeliverySchedule),
            b"28" => Some(MasterDeliverySchedule),
            b"29" => Some(AssemblyPlan),
            b"30" => Some(LeadTimeChart),
            b"31" => Some(MasterSchedule),
            b"32" => Some(MasterProductionSchedule),
            b"33" => Some(ManufacturingFlowDiagram),
            b"34" => Some(LotReleasePlan),
            b"35" => Some(SubcontractorPlan),
            b"36" => Some(DevelopmentPlan),
            b"37" => Some(LeaseAgreement),
            b"38" => Some(CourtJudgment),
            b"39" => Some(LotteryWinningLetter),
            b"40" => Some(CurbSideInspection),
            b"41" => Some(StatisticalModel),
            b"42" => Some(InteriorAndExteriorInspection),
            b"43" => Some(NonInspectionReport),
            b"44" => Some(ReviewAppraisal),
            b"45" => Some(InvestorGuidelines),
            b"46" => Some(SoleDiscretionInspection),
            b"47" => Some(BrokersPriceOpinion),
            b"48" => Some(SocialSecurityBenefitLetter),
            b"49" => Some(DivorceDecree),
            b"50" => Some(Contract),
            b"51" => Some(GiftLetter),
            b"52" => Some(Will),
            b"53" => Some(TrustAgreement),
            b"54" => Some(AwardLetter),
            b"55" => Some(RentalAgreement),
            b"56" => Some(PreliminaryRealEstateSettlementStatement),
            b"57" => Some(Code57),
            b"58" => Some(UtilityBill),
            b"59" => Some(BenefitLetter),
            b"60" => Some(CanceledCheck),
            b"61" => Some(Code61),
            b"62" => Some(AssetAccountStatement),
            b"63" => Some(StatementOfProfitAndLoss),
            b"64" => Some(Code64),
            b"65" => Some(Code65),
            b"66" => Some(PayStub),
            b"67" => Some(Code67),
            b"68" => Some(YearEndStatement),
            b"69" => Some(BankStatement),
            b"70" => Some(VerificationOfLoanInstallmentDebtForm),
            b"71" => Some(VerificationOfDepositForm),
            b"72" => Some(VerificationOfMortgageRentForm),
            b"73" => Some(VerificationOfEmploymentForm),
            b"74" => Some(Code74),
            b"75" => Some(TitleCertificates),
            b"76" => Some(Code76),
            b"77" => Some(SupportDataForVerification),
            b"78" => Some(VisaExportLicense),
            b"79" => Some(MultiCountryTextileDeclaration),
            b"80" => Some(SingleCountryTextileDeclaration),
            b"81" => Some(NegativeTextileDeclaration),
            b"82" => Some(Code82),
            b"83" => Some(TrademarkRelease),
            b"84" => Some(WaterResistanceStatement),
            b"85" => Some(Code85),
            b"86" => Some(Code86),
            b"87" => Some(InterimFootwearInvoice),
            b"88" => Some(ImpactResistanceStatement),
            b"89" => Some(ToxicSubstanceComplianceStatement),
            b"90" => Some(ForeignShippersDeclaration),
            b"91" => Some(VeterinarianCertificate),
            b"92" => Some(ChildLaborCertificate),
            b"93" => Some(PrisonLaborCertificate),
            b"94" => Some(PurchaseOrderCopy),
            b"95" => Some(ProductAnalysis),
            b"96" => Some(AmericanAutomotiveLabelingActCertificate),
            b"97" => Some(BrokerMarketAnalysis),
            b"A1" => Some(AirEmissionsStatements),
            b"A2" => Some(AntiDumpingGasolineProgramReport),
            b"A3" => Some(AllergiesSensitivitiesDocument),
            b"A4" => Some(AutopsyReport),
            b"AA" => Some(AgentInventoryReport),
            b"AB" => Some(AssemblyDrawing),
            b"AC" => Some(AssayCertificate),
            b"AD" => Some(AgentDistributorInventoryReport),
            b"AE" => Some(Attachment),
            b"AF" => Some(AidForm),
            b"AG" => Some(Actual),
            b"AH" => Some(EasementReport),
            b"AI" => Some(CodeAI),
            b"AJ" => Some(ImpoundAccountEscrowAnalysisReport),
            b"AK" => Some(ClosingEscrowAnalysisReport),
            b"AL" => Some(ReservedEscrowAnalysisReport),
            b"AM" => Some(AmbulanceCertification),
            b"AN" => Some(TitlePolicy),
            b"AO" => Some(AverageOutgoingQualityReport),
            b"AP" => Some(AdvancedProblemNotification),
            b"AQ" => Some(CodeAQ),
            b"AR" => Some(AssetReclassificationExtensionRequest),
            b"AS" => Some(AdmissionSummary),
            b"AT" => Some(PurchaseOrderAttachment),
            b"AU" => Some(AutomobileClaimReport),
            b"AV" => Some(AveragingAreasReport),
            b"AW" => Some(AirWaybill),
            b"AX" => Some(AssetReclassificationExtensionResponse),
            b"AY" => Some(TaxCertificate),
            b"AZ" => Some(HomeOwnerAuthorization),
            b"B1" => Some(BatchReport),
            b"B2" => Some(Prescription),
            b"B3" => Some(PhysicianOrder),
            b"B4" => Some(ReferralForm),
            b"BA" => Some(Budget),
            b"BB" => Some(BuyOrSellExchangeContractStatusStatement),
            b"BC" => Some(BillOfLadingCopy),
            b"BE" => Some(BenzeneContentAveragingReport),
            b"BF" => Some(BailmentWarehouseWithdrawalRequest),
            b"BL" => Some(BillOfLading),
            b"BM" => Some(BillOfMaterial),
            b"BN" => Some(BeneficiaryCertificate),
            b"BO" => Some(BillOfLadingOriginal),
            b"BR" => Some(BenchmarkTestingResults),
            b"BS" => Some(Baseline),
            b"BT" => Some(BlanketTestResults),
            b"BW" => Some(BillOfSale),
            b"BY" => Some(BiennialReport),
            b"C1" => Some(CostDataSummary),
            b"C2" => Some(FunctionalCostAndHour),
            b"C3" => Some(ProgressCurve),
            b"C4" => Some(PlantWideData),
            b"C5" => Some(CertifiedCostAndPriceData),
            b"C6" => Some(WageDetermination),
            b"C7" => Some(CreditTransferSummaryReport),
            b"C8" => Some(ChemicalRadiologicalReport),
            b"C9" => Some(CertificationAuthorizationDocument),
            b"CA" => Some(CertificateOfAnalysis),
            b"CB" => Some(ChiropracticJustification),
            b"CC" => Some(CodeCC),
            b"CD" => Some(CustomerDistributorInventoryReport),
            b"CE" => Some(ConstableReport),
            b"CF" => Some(Capability),
            b"CG" => Some(CertificateOfOrigin),
            b"CH" => Some(CertificateOfWeight),
            b"CI" => Some(CertificateOfInspectionReport),
            b"CJ" => Some(ComplicationsDocument),
            b"CK" => Some(CodeCK),
            b"CL" => Some(Cable),
            b"CM" => Some(CustomerManufacturerInventoryReport),
            b"CN" => Some(CustomersReportOfNonconformance),
            b"CO" => Some(ConsignmentOrder),
            b"CP" => Some(CodeCP),
            b"CQ" => Some(CountyRecord),
            b"CR" => Some(LetterOfCredit),
            b"CS" => Some(ConsignedInventorySalesReport),
            b"CT" => Some(Certification),
            b"CU" => Some(CustomerNotificationLetter),
            b"CV" => Some(ChangeOfHospiceBenefit),
            b"CW" => Some(CorrectiveWorkOrder),
            b"CX" => Some(CodeCX),
            b"CY" => Some(CodeCY),
            b"CZ" => Some(CampusPoliceReport),
            b"D2" => Some(DrugProfileDocument),
            b"DA" => Some(DentalModels),
            b"DB" => Some(DurableMedicalEquipmentPrescription),
            b"DC" => Some(DistributorCustomerInventoryReport),
            b"DD" => Some(DistributorInventoryReport),
            b"DE" => Some(CertificateOfQuality),
            b"DF" => Some(Da59SpecialCustomsInvoiceForSouthAfrica),
            b"DG" => Some(DiagnosticReport),
            b"DH" => Some(NitrogenCertificate),
            b"DI" => Some(Directory),
            b"DJ" => Some(DischargeMonitoringReport),
            b"DK" => Some(DrawbackAffidavit),
            b"DL" => Some(DraftAndTransmittalLetter),
            b"DM" => Some(DistributorManufacturerInventoryReport),
            b"DN" => Some(DeviationNonconformanceTestResultsAndRequestForAction),
            b"DQ" => Some(Delinquency),
            b"DR" => Some(DatalogReport),
            b"DS" => Some(DischargeSummary),
            b"DT" => Some(DepartmentOfTransportation),
            b"DU" => Some(Commercial),
            b"DV" => Some(Condominium),
            b"DW" => Some(CodeDW),
            b"E1" => Some(ExportersCertificateAndAgreement),
            b"EA" => Some(ElectricalAverageOutgoingQualityReport),
            b"EB" => Some(CodeEB),
            b"EC" => Some(EngineeringChangeOrder),
            b"ED" => Some(EnvironmentalExposureDocument),
            b"EH" => Some(ElectionOfHospiceBenefit),
            b"EL" => Some(Eligibility),
            b"EP" => Some(ExperimentalMaterialPurchaseOrder),
            b"ER" => Some(EngineeringChangeRequest),
            b"ES" => Some(SourceSelectionPlan),
            b"EX" => Some(ShippersExportDeclaration),
            b"EY" => Some(BarrelForBarrelExchangeContractStatusStatement),
            b"F1" => Some(CodeF1),
            b"F2" => Some(CodeF2),
            b"F3" => Some(CodeF3),
            b"F4" => Some(CodeF4),
            b"F5" => Some(CodeF5),
            b"F6" => Some(TransportationCarrierInspectionReport),
            b"F7" => Some(GovernmentInspectionReport),
            b"F8" => Some(CodeF8),
            b"F9" => Some(CodeF9),
            b"FB" => Some(FederalBureauOfInvestigation),
            b"FC" => Some(FumigationCertificate),
            b"FD" => Some(FederalSpecificationCompliance),
            b"FE" => Some(FederalEmergencyManagementAgency),
            b"FH" => Some(LimitationOfHeavyElements),
            b"FI" => Some(FireReport),
            b"FM" => Some(FamilyMedicalHistoryDocument),
            b"FO" => Some(PostOperativeRadiologyFilms),
            b"FR" => Some(PreOperativeRadiologyFilms),
            b"FS" => Some(CertificateOfFreeSale),
            b"G1" => Some(StateForm),
            b"G2" => Some(ClearanceLetter),
            b"G3" => Some(BackgroundRelease),
            b"G4" => Some(ExamResults),
            b"G5" => Some(PrelicenseCertificate),
            b"G6" => Some(NationalAssociationOfSecuritiesDealersCertification),
            b"G7" => Some(LicenseCopy),
            b"GP" => Some(GasProcessorsReport),
            b"GT" => Some(GasTransportersReport),
            b"HC" => Some(HealthCertificate),
            b"HI" => Some(HazardousMaterialIncident),
            b"HP" => Some(HistoryAndPhysical),
            b"HR" => Some(HealthClinicRecords),
            b"HW" => Some(HazardousWasteManifest),
            b"I2" => Some(ConsularInvoice),
            b"I3" => Some(CustomsInvoice),
            b"I4" => Some(ForwardersInvoice),
            b"I5" => Some(ImmunizationRecord),
            b"I6" => Some(CarriersInvoice),
            b"IA" => Some(InsuranceAttachment),
            b"IC" => Some(InsuranceCertificate),
            b"IM" => Some(ImportLicense),
            b"IN" => Some(InspectionRequest),
            b"IP" => Some(InventoryParameterReport),
            b"IR" => Some(StateSchoolImmunizationRecords),
            b"IS" => Some(IndexSystem),
            b"IT" => Some(CertifiedInspectionAndTestResults),
            b"IU" => Some(InspectionResult),
            b"IV" => Some(Invoice),
            b"JA" => Some(CertificateOfGoodStanding),
            b"JB" => Some(TaxStatusClearance),
            b"JC" => Some(ConsentToUseName),
            b"JD" => Some(CertificateOfRegistration),
            b"JE" => Some(CertificateOfExistence),
            b"JF" => Some(CertificateOfStatus),
            b"JG" => Some(CertificateOfNameChange),
            b"JH" => Some(CertificateOfMerger),
            b"JI" => Some(CertificateOfSignificantChange),
            b"JK" => Some(BalanceSheet),
            b"JL" => Some(ApplicationOfNameReservation),
            b"JM" => Some(ScheduleOfCapital),
            b"JN" => Some(ForeignTaxReturn),
            b"JO" => Some(PermitApplication),
            b"JP" => Some(AdmissionTaxReturn),
            b"JQ" => Some(AddendumToArticles),
            b"JR" => Some(ArticlesAndAmendments),
            b"JS" => Some(AppointmentOfCommissionerAsRegisteredAgent),
            b"JT" => Some(CertificateOfDisclosure),
            b"JV" => Some(NoticeOfRegisteredOffice),
            b"JW" => Some(NoticeOfDirectors),
            b"JX" => Some(OrganizationAndFirstBiennialReport),
            b"JY" => Some(AgreementOfStatutoryAgent),
            b"JZ" => Some(ConsentToAct),
            b"KA" => Some(CodeKA),
            b"KC" => Some(KosherCertificate),
            b"KD" => Some(EngineeringDrawingList),
            b"KE" => Some(PurchasedEngineeringDataList),
            b"KF" => Some(SupportDocuments),
            b"KG" => Some(PurchasedDocuments),
            b"KH" => Some(ProposalSupportData),
            b"KI" => Some(PurchasedDrawings),
            b"KJ" => Some(ChangeProposalData),
            b"KY" => Some(ReportOfAssignmentOrModificationOfKeyEvents),
            b"KZ" => Some(RequestForAssignmentOrModificationOfKeyEvents),
            b"LA" => Some(LaboratoryResults),
            b"LB" => Some(LegalizedBillOfLading),
            b"LC" => Some(LocationInventoryReport),
            b"LD" => Some(CodeLD),
            b"LE" => Some(LatestRevisedEstimate),
            b"LG" => Some(LegalizedCertificateOfOrigin),
            b"LI" => Some(LegalizedInvoice),
            b"LO" => Some(CodeLO),
            b"LP" => Some(LaborPlan),
            b"LR" => Some(CodeLR),
            b"LS" => Some(LeaseSettlementStatement),
            b"LT" => Some(LicenseApplicationAttachment),
            b"LW" => Some(CodeLW),
            b"M1" => Some(MedicalRecordAttachment),
            b"MA" => Some(ManufacturerAgentInventoryReport),
            b"MB" => Some(ManufacturerDistributorInventoryReport),
            b"MC" => Some(ManufacturerCustomerInventoryReport),
            b"MD" => Some(MaterialDataSheets),
            b"ME" => Some(MajorDeviationRequest),
            b"MF" => Some(ManufacturingSpecification),
            b"MG" => Some(CodeMG),
            b"MH" => Some(ReportOfFullMaintenancePeriodDetail),
            b"MI" => Some(MortgageInsuranceCertification),
            b"MJ" => Some(RequestForMaintenancePeriodStatus),
            b"MK" => Some(ReportOfMaintenancePeriodStatus),
            b"ML" => Some(RequestForFullMaintenancePeriodDetail),
            b"MM" => Some(ManufacturerInventoryReport),
            b"MN" => Some(MinorDeviationRequest),
            b"MO" => Some(ManufacturersStatementOfOrigin),
            b"MP" => Some(CodeMP),
            b"MQ" => Some(CodeMQ),
            b"MR" => Some(MaterialInspectionAndReceivingReport),
            b"MS" => Some(MaterialSafetyDataSheet),
            b"MT" => Some(Models),
            b"MV" => Some(MeteredVolumes),
            b"MZ" => Some(MotorVehicleReport),
            b"NA" => Some(NationalInsuranceCrimeBureauAssignment),
            b"NC" => Some(CertificateOfQuantity),
            b"ND" => Some(CommercialInvoice),
            b"NI" => Some(NationalInsuranceCrimeBureau),
            b"NL" => Some(NationalInsuranceCrimeBureauTotalLoss),
            b"NM" => Some(MonthlyContractorFinancialManagementReport),
            b"NN" => Some(NursingNotes),
            b"NO" => Some(NationalInsuranceCrimeBureauOtherThanTheft),
            b"NQ" => Some(QuarterlyContractorFinancialManagementReport),
            b"NR" => Some(NOxEmissionsAveragingReport),
            b"NT" => Some(NationalInsuranceCrimeBureauTotalTheft),
            b"OB" => Some(OperativeNote),
            b"OC" => Some(OxygenContentAveragingReport),
            b"OD" => Some(OrdersAndTreatmentsDocument),
            b"OE" => Some(CodeOE),
            b"OL" => Some(OceanBillOfLading),
            b"OP" => Some(OutsideProductionOperationSheet),
            b"OR" => Some(OilStorersReport),
            b"OS" => Some(OrganizationBreakdownStructure),
            b"OT" => Some(OilTransportersReport),
            b"OX" => Some(OxygenTherapyCertification),
            b"OZ" => Some(SupportDataForClaim),
            b"P1" => Some(PackingList),
            b"P2" => Some(Protest),
            b"P3" => Some(Receipt),
            b"P4" => Some(PathologyReport),
            b"P5" => Some(PatientMedicalHistoryDocument),
            b"P6" => Some(PeriodontalCharts),
            b"P7" => Some(PeriodontalReports),
            b"P8" => Some(PropertyClaimReport),
            b"PA" => Some(PartDrawing),
            b"PB" => Some(ProductCatalog),
            b"PC" => Some(ProcessChangeNotice),
            b"PD" => Some(ProofOfDelivery),
            b"PE" => Some(ParenteralOrEnteralCertification),
            b"PF" => Some(ProductSpecification),
            b"PG" => Some(PackagingSpecification),
            b"PH" => Some(ProductionHistoryPropertyLevel),
            b"PI" => Some(ProductAvailabilityInquiry),
            b"PJ" => Some(PurchasingSpecification),
            b"PK" => Some(StorageInformationInquiry),
            b"PL" => Some(PropertyInsuranceLossRegister),
            b"PM" => Some(ProofOfInsurance),
            b"PN" => Some(PhysicalTherapyNotes),
            b"PO" => Some(ProstheticsOrOrthoticCertification),
            b"PP" => Some(Proposal),
            b"PQ" => Some(ParamedicalResults),
            b"PR" => Some(PurchaseReport),
            b"PS" => Some(PipelineShipperInventoryReport),
            b"PT" => Some(InterPlantInventoryReport),
            b"PV" => Some(PoliceReport),
            b"PW" => Some(ProductionHistoryWellLevel),
            b"PX" => Some(CodePX),
            b"PY" => Some(PhysiciansReport),
            b"PZ" => Some(PhysicalTherapyCertification),
            b"QC" => Some(CauseAndCorrectiveActionReport),
            b"QD" => Some(CodeQD),
            b"QE" => Some(QualityDetail),
            b"QM" => Some(CodeQM),
            b"QR" => Some(QualityReport),
            b"QS" => Some(QualityReviewOrderSupplement),
            b"QT" => Some(QualitySummary),
            b"R1" => Some(ReformulatedGasolineAntiDumpingCompanyRegistration),
            b"R2" => Some(ReformulatedGasolineAntiDumpingFacilityRegistration),
            b"R3" => Some(TechnicalInformationPackage),
            b"R4" => Some(PurchasedTechnicalInformationPackage),
            b"R5" => Some(Technical),
            b"R6" => Some(Miscellaneous),
            b"R7" => Some(ComplianceReview),
            b"R9" => Some(Accident),
            b"RA" => Some(RevisionAnnouncement),
            b"RB" => Some(RadiologyFilms),
            b"RC" => Some(RequestForCauseAndCorrectiveActionReport),
            b"RD" => Some(PaymentBond),
            b"RE" => Some(PerformanceBond),
            b"RF" => Some(ReliabilityFailRateReport),
            b"RG" => Some(Residential),
            b"RH" => Some(BidBond),
            b"RM" => Some(RequestForManufacturingEngineerAppraisal),
            b"RN" => Some(SuppliersReportOfNonconformance),
            b"RO" => Some(RegularOrder),
            b"RR" => Some(RadiologyReports),
            b"RT" => Some(ReportOfTestsAndAnalysisReport),
            b"RV" => Some(CodeRV),
            b"RX" => Some(RenewableOxygenContentAveragingReport),
            b"S1" => Some(SupplyAndShipmentStatusReport),
            b"S2" => Some(SupplyStatusReport),
            b"S3" => Some(ExceptionSupplyStatusReport),
            b"S4" => Some(ExceptionSupplyAndShipmentStatusReport),
            b"S5" => Some(ProductQualityDeficiencyReportCategoryI),
            b"S6" => Some(ProductQualityDeficiencyReportCategoryIi),
            b"S7" => Some(CodeS7),
            b"S8" => Some(ReportOfFindings),
            b"S9" => Some(Representation),
            b"SA" => Some(StatePoliceReport),
            b"SB" => Some(SampleApprovalAndRejectionList),
            b"SC" => Some(SanitaryCertificate),
            b"SD" => Some(SupportDataForARequestForQuote),
            b"SE" => Some(SecurityPoliceReport),
            b"SF" => Some(ContractSecurityClassificationSpecification),
            b"SG" => Some(SymptomsDocument),
            b"SH" => Some(SheriffReport),
            b"SI" => Some(SellerInventoryReport),
            b"SJ" => Some(StatementOfWork),
            b"SL" => Some(SampleBaleList),
            b"SM" => Some(ShippingManifests),
            b"SN" => Some(ShippingNotice),
            b"SO" => Some(SecretaryCertificate),
            b"SP" => Some(Specification),
            b"SQ" => Some(StatisticalQualityDocuments),
            b"SR" => Some(StatisticalReport),
            b"SS" => Some(SellerSalesReport),
            b"ST" => Some(CodeST),
            b"SU" => Some(SuppliersCertificate),
            b"SV" => Some(Survey),
            b"SW" => Some(SeaWaybill),
            b"SX" => Some(SteamshipDueBill),
            b"SY" => Some(TrainSheet),
            b"T1" => Some(TitleBill),
            b"T2" => Some(PreliminaryTitleWork),
            b"T3" => Some(LoanDocuments),
            b"T4" => Some(Tax),
            b"T5" => Some(ToxicsEmissionsPerformanceAveragingReport),
            b"T6" => Some(ToxicsReleaseInventory),
            b"T7" => Some(TherapyNotes),
            b"TA" => Some(AssetSupportInquiry),
            b"TB" => Some(AssetSupportAdvice),
            b"TC" => Some(PhysicalInventoryRequest),
            b"TD" => Some(AssetReclassificationResponse),
            b"TE" => Some(AssetReclassificationRequest),
            b"TF" => Some(TransactionHistoryRequest),
            b"TG" => Some(TwoToFourFamily),
            b"TH" => Some(TotalTheftClaimReport),
            b"TI" => Some(AssetStatusInquiry),
            b"TJ" => Some(AssetStatusAdvice),
            b"TK" => Some(LogisticsTransferInquiry),
            b"TL" => Some(LogisticsTransferAdvice),
            b"TM" => Some(StockSaleReport),
            b"TN" => Some(DelayedSaleReport),
            b"TO" => Some(DemandReport),
            b"TP" => Some(TreatmentsCertificate),
            b"TQ" => Some(StorageInformationAdvice),
            b"TR" => Some(TransmittalLetter),
            b"TS" => Some(CodeTS),
            b"TT" => Some(TitleTransfer),
            b"TX" => Some(TaxExemptCertificate),
            b"U1" => Some(SurveyReport),
            b"UA" => Some(UnionAgreement),
            b"UB" => Some(CertificateOfDesignationOfRegisteredAgent),
            b"UD" => Some(ListOfOfficersAndDirectors),
            b"UE" => Some(ResolutionAndConsentForm),
            b"UF" => Some(DomesticBusinessCorporationInitialReport),
            b"UG" => Some(RegisteredAgentApplication),
            b"UH" => Some(ArticlesOfIncorporation),
            b"UI" => Some(CertificateOfCompliance),
            b"UJ" => Some(CertificateOfAuthorization),
            b"UK" => Some(Charter),
            b"UL" => Some(OtherTypeOfReport),
            b"UM" => Some(AffidavitOfAcceptance),
            b"UN" => Some(ResolutionAdoptingFictitiousName),
            b"UO" => Some(TradeNameApplication),
            b"UP" => Some(DeclarationOfSolicitor),
            b"UQ" => Some(MemorandumOfAssociation),
            b"UR" => Some(NoticeOfRegisteredAgent),
            b"US" => Some(CodeUS),
            b"UU" => Some(DissolutionOfExistingRegistration),
            b"UV" => Some(AppointmentOfStatutoryAgent),
            b"UX" => Some(RegulatoryApprovalForProfessionalAssociation),
            b"UY" => Some(InitialAnnualReport),
            b"UZ" => Some(CertificateOfFact),
            b"V1" => Some(VoterRegistrationApplication),
            b"V2" => Some(VoterRegistrationApplicationDisposition),
            b"V3" => Some(VoterInformationRecord),
            b"V4" => Some(ChangeOfNameAndOrAddress),
            b"V5" => Some(DeathNotification),
            b"V6" => Some(FelonyConvictionNotification),
            b"V7" => Some(IncompetencyNotification),
            b"VA" => Some(VarianceAnalysis),
            b"VC" => Some(CodeVC),
            b"VD" => Some(DataRequestForVendorsSpecificationsOrDrawings),
            b"VM" => Some(VisualMechanicalAverageOutgoingQualityReport),
            b"W1" => Some(SafeDrinkingWaterBacteriologicalReport),
            b"W2" => Some(SafeDrinkingWaterReport),
            b"WA" => Some(FictitiousNameStatement),
            b"WB" => Some(WorkBreakdownStructure),
            b"WC" => Some(RequestForAssignmentOrDeletionOfWorkCandidate),
            b"WD" => Some(ReportOfAssignmentOrDeletionOfWorkCandidate),
            b"WE" => Some(BusinessConductedPriorToQualificationForm),
            b"WF" => Some(ByLaws),
            b"WG" => Some(AppointmentOfAgentForServiceAndConsentToAct),
            b"WH" => Some(CertificateOfNameClearance),
            b"WI" => Some(Well),
            b"WP" => Some(WorkProgress),
            b"WT" => Some(WellTest),
            b"X1" => Some(CompleteAppraisal),
            b"X2" => Some(LimitedAppraisal),
            b"X3" => Some(SelfContainedReport),
            b"X4" => Some(SummaryReport),
            b"X5" => Some(RestrictedReport),
            b"XE" => Some(EquipmentTestResults),
            b"XP" => Some(Photographs),
            b"Y1" => Some(Appraisal),
            b"Y2" => Some(BrokerPriceOpinion),
            b"Y3" => Some(RealEstateProperty),
            b"ZA" => Some(FloodDeterminationReport),
            b"ZB" => Some(ConventionalAmmunitionSuspensionReport),
            b"ZC" => Some(SelfMonitoringReport),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ReportTypeCode::*;
        match self {
            ProductTransfer => "Product Transfer",
            Resale => "Resale",
            ReportJustifyingTreatmentBeyondUtilizationGuidelines => {
                "Report Justifying Treatment Beyond Utilization Guidelines"
            }
            DrugsAdministered => "Drugs Administered",
            TreatmentDiagnosis => "Treatment Diagnosis",
            InitialAssessment => "Initial Assessment",
            FunctionalGoals => "Functional Goals",
            PlanOfTreatment => "Plan of Treatment",
            ProgressReport => "Progress Report",
            ContinuedTreatment => "Continued Treatment",
            ChemicalAnalysis => "Chemical Analysis",
            MechanicalProperties => "Mechanical Properties",
            CertifiedTestReport => "Certified Test Report",
            CoreLossTestReport => "Core Loss Test Report",
            JustificationForAdmission => "Justification for Admission",
            Code16 => {
                "Western Region 01 (Internal Revenue Service Summary of line 31 on 1040)"
            }
            Code17 => "Return From Tax Payer Report (Internal Revenue Service Summary)",
            NoteReceivable => "Note Receivable",
            CreditReport => "Credit Report",
            ProcessPlan => "Process Plan",
            RecoveryPlan => "Recovery Plan",
            FunctionalPlan => "Functional Plan",
            ContractualPlan => "Contractual Plan",
            NonContractualPlan => "Non-Contractual Plan",
            PurchasePlan => "Purchase Plan",
            ProductionPlan => "Production Plan",
            ContractDeliverySchedule => "Contract Delivery Schedule",
            MasterDeliverySchedule => "Master Delivery Schedule",
            AssemblyPlan => "Assembly Plan",
            LeadTimeChart => "Lead Time Chart",
            MasterSchedule => "Master Schedule",
            MasterProductionSchedule => "Master Production Schedule",
            ManufacturingFlowDiagram => "Manufacturing Flow Diagram",
            LotReleasePlan => "Lot Release Plan",
            SubcontractorPlan => "Subcontractor Plan",
            DevelopmentPlan => "Development Plan",
            LeaseAgreement => "Lease Agreement",
            CourtJudgment => "Court Judgment",
            LotteryWinningLetter => "Lottery Winning Letter",
            CurbSideInspection => "Curb Side Inspection",
            StatisticalModel => "Statistical Model",
            InteriorAndExteriorInspection => "Interior and Exterior Inspection",
            NonInspectionReport => "Non-Inspection Report",
            ReviewAppraisal => "Review Appraisal",
            InvestorGuidelines => "Investor Guidelines",
            SoleDiscretionInspection => "Sole Discretion Inspection",
            BrokersPriceOpinion => "Brokers Price Opinion",
            SocialSecurityBenefitLetter => "Social Security Benefit Letter",
            DivorceDecree => "Divorce Decree",
            Contract => "Contract",
            GiftLetter => "Gift Letter",
            Will => "Will",
            TrustAgreement => "Trust Agreement",
            AwardLetter => "Award Letter",
            RentalAgreement => "Rental Agreement",
            PreliminaryRealEstateSettlementStatement => {
                "Preliminary Real Estate Settlement Statement"
            }
            Code57 => "Income Statement (Internal Revenue Service Form 1099)",
            UtilityBill => "Utility Bill",
            BenefitLetter => "Benefit Letter",
            CanceledCheck => "Canceled Check",
            Code61 => "Individual Tax Return (Internal Revenue Service Form 1040)",
            AssetAccountStatement => "Asset Account Statement",
            StatementOfProfitAndLoss => "Statement of Profit and Loss",
            Code64 => {
                "Partner Share of Income, Credit, Deductions, (Internal Revenue Service Form K1)"
            }
            Code65 => "Partnership Tax Return (Internal Revenue Service Form 1120)",
            PayStub => "Pay Stub",
            Code67 => "Wage and Tax Statement (Internal Revenue Service Form W2)",
            YearEndStatement => "Year End Statement",
            BankStatement => "Bank Statement",
            VerificationOfLoanInstallmentDebtForm => {
                "Verification of Loan/Installment Debt Form"
            }
            VerificationOfDepositForm => "Verification of Deposit Form",
            VerificationOfMortgageRentForm => "Verification of Mortgage/Rent Form",
            VerificationOfEmploymentForm => "Verification of Employment Form",
            Code74 => "Corporation Tax Return (Internal Revenue Service Form 1065)",
            TitleCertificates => "Title Certificates",
            Code76 => {
                "Real Estate Settlement Statement (Housing and Urban Development Form - 1 \"HUD1\")"
            }
            SupportDataForVerification => "Support Data for Verification",
            VisaExportLicense => "Visa/Export License",
            MultiCountryTextileDeclaration => "Multi-Country Textile Declaration",
            SingleCountryTextileDeclaration => "Single Country Textile Declaration",
            NegativeTextileDeclaration => "Negative Textile Declaration",
            Code82 => {
                "Endangered Species Convention on International Trade in Endangered Species (CITES)"
            }
            TrademarkRelease => "Trademark Release",
            WaterResistanceStatement => "Water Resistance Statement",
            Code85 => {
                "Certificate of Ceramicware Ceramics Commodities Inspection Bureau (CCIB)"
            }
            Code86 => "Wearing Apparel Detail Sheet (WADS)",
            InterimFootwearInvoice => "Interim Footwear Invoice",
            ImpactResistanceStatement => "Impact Resistance Statement",
            ToxicSubstanceComplianceStatement => "Toxic Substance Compliance Statement",
            ForeignShippersDeclaration => "Foreign Shippers Declaration",
            VeterinarianCertificate => "Veterinarian Certificate",
            ChildLaborCertificate => "Child Labor Certificate",
            PrisonLaborCertificate => "Prison Labor Certificate",
            PurchaseOrderCopy => "Purchase Order Copy",
            ProductAnalysis => "Product Analysis",
            AmericanAutomotiveLabelingActCertificate => {
                "American Automotive Labeling Act Certificate"
            }
            BrokerMarketAnalysis => "Broker Market Analysis",
            AirEmissionsStatements => "Air Emissions Statements",
            AntiDumpingGasolineProgramReport => "Anti-Dumping Gasoline Program Report",
            AllergiesSensitivitiesDocument => "Allergies/Sensitivities Document",
            AutopsyReport => "Autopsy Report",
            AgentInventoryReport => "Agent Inventory Report",
            AssemblyDrawing => "Assembly Drawing",
            AssayCertificate => "Assay Certificate",
            AgentDistributorInventoryReport => "Agent/Distributor Inventory Report",
            Attachment => "Attachment",
            AidForm => "Aid Form",
            Actual => "Actual",
            EasementReport => "Easement Report",
            CodeAI => "Conditions, Covenant, and Restrictions Report",
            ImpoundAccountEscrowAnalysisReport => {
                "Impound Account Escrow Analysis Report"
            }
            ClosingEscrowAnalysisReport => "Closing Escrow Analysis Report",
            ReservedEscrowAnalysisReport => "Reserved Escrow Analysis Report",
            AmbulanceCertification => "Ambulance Certification",
            TitlePolicy => "Title Policy",
            AverageOutgoingQualityReport => "Average Outgoing Quality Report",
            AdvancedProblemNotification => "Advanced Problem Notification",
            CodeAQ => "Housing and Urban Development (HUD) 1 Report",
            AssetReclassificationExtensionRequest => {
                "Asset Reclassification Extension Request"
            }
            AdmissionSummary => "Admission Summary",
            PurchaseOrderAttachment => "Purchase Order Attachment",
            AutomobileClaimReport => "Automobile Claim Report",
            AveragingAreasReport => "Averaging Areas Report",
            AirWaybill => "Air Waybill",
            AssetReclassificationExtensionResponse => {
                "Asset Reclassification Extension Response"
            }
            TaxCertificate => "Tax Certificate",
            HomeOwnerAuthorization => "Home Owner Authorization",
            BatchReport => "Batch Report",
            Prescription => "Prescription",
            PhysicianOrder => "Physician Order",
            ReferralForm => "Referral Form",
            Budget => "Budget",
            BuyOrSellExchangeContractStatusStatement => {
                "Buy or Sell Exchange Contract Status Statement"
            }
            BillOfLadingCopy => "Bill of Lading Copy",
            BenzeneContentAveragingReport => "Benzene Content Averaging Report",
            BailmentWarehouseWithdrawalRequest => "Bailment Warehouse Withdrawal Request",
            BillOfLading => "Bill of Lading",
            BillOfMaterial => "Bill of Material",
            BeneficiaryCertificate => "Beneficiary Certificate",
            BillOfLadingOriginal => "Bill of Lading Original",
            BenchmarkTestingResults => "Benchmark Testing Results",
            Baseline => "Baseline",
            BlanketTestResults => "Blanket Test Results",
            BillOfSale => "Bill of Sale",
            BiennialReport => "Biennial Report",
            CostDataSummary => "Cost Data Summary",
            FunctionalCostAndHour => "Functional Cost and Hour",
            ProgressCurve => "Progress Curve",
            PlantWideData => "Plant-Wide Data",
            CertifiedCostAndPriceData => "Certified Cost and Price Data",
            WageDetermination => "Wage Determination",
            CreditTransferSummaryReport => "Credit Transfer Summary Report",
            ChemicalRadiologicalReport => "Chemical/Radiological Report",
            CertificationAuthorizationDocument => "Certification/Authorization Document",
            CertificateOfAnalysis => "Certificate of Analysis",
            ChiropracticJustification => "Chiropractic Justification",
            CodeCC => "C.A.A. Certificate of Conformance (British CAA)",
            CustomerDistributorInventoryReport => "Customer/Distributor Inventory Report",
            ConstableReport => "Constable Report",
            Capability => "Capability",
            CertificateOfOrigin => "Certificate of Origin",
            CertificateOfWeight => "Certificate of Weight",
            CertificateOfInspectionReport => "Certificate of Inspection Report",
            ComplicationsDocument => "Complications Document",
            CodeCK => "Consent Form(s)",
            Cable => "Cable",
            CustomerManufacturerInventoryReport => {
                "Customer/Manufacturer Inventory Report"
            }
            CustomersReportOfNonconformance => "Customer's Report of Nonconformance",
            ConsignmentOrder => "Consignment Order",
            CodeCP => "Certificate of Compliance (Material Certification)",
            CountyRecord => "County Record",
            LetterOfCredit => "Letter of Credit",
            ConsignedInventorySalesReport => "Consigned Inventory Sales Report",
            Certification => "Certification",
            CustomerNotificationLetter => "Customer Notification Letter",
            ChangeOfHospiceBenefit => "Change of Hospice Benefit",
            CorrectiveWorkOrder => "Corrective Work Order",
            CodeCX => "Cost/Schedule Status Report (C/SSR)",
            CodeCY => "Contract Funds Status Report (CFSR)",
            CampusPoliceReport => "Campus Police Report",
            DrugProfileDocument => "Drug Profile Document",
            DentalModels => "Dental Models",
            DurableMedicalEquipmentPrescription => {
                "Durable Medical Equipment Prescription"
            }
            DistributorCustomerInventoryReport => "Distributor/Customer Inventory Report",
            DistributorInventoryReport => "Distributor Inventory Report",
            CertificateOfQuality => "Certificate of Quality",
            Da59SpecialCustomsInvoiceForSouthAfrica => {
                "DA59 Special Customs Invoice for South Africa"
            }
            DiagnosticReport => "Diagnostic Report",
            NitrogenCertificate => "Nitrogen Certificate",
            Directory => "Directory",
            DischargeMonitoringReport => "Discharge Monitoring Report",
            DrawbackAffidavit => "Drawback Affidavit",
            DraftAndTransmittalLetter => "Draft and Transmittal Letter",
            DistributorManufacturerInventoryReport => {
                "Distributor/Manufacturer Inventory Report"
            }
            DeviationNonconformanceTestResultsAndRequestForAction => {
                "Deviation/Nonconformance Test Results and Request for Action"
            }
            Delinquency => "Delinquency",
            DatalogReport => "Datalog Report",
            DischargeSummary => "Discharge Summary",
            DepartmentOfTransportation => "Department of Transportation",
            Commercial => "Commercial",
            Condominium => "Condominium",
            CodeDW => "Drawing(s)",
            ExportersCertificateAndAgreement => "Exporter's Certificate and Agreement",
            ElectricalAverageOutgoingQualityReport => {
                "Electrical Average Outgoing Quality Report"
            }
            CodeEB => {
                "Explanation of Benefits (Coordination of Benefits or Medicare Secondary Payor)"
            }
            EngineeringChangeOrder => "Engineering Change Order",
            EnvironmentalExposureDocument => "Environmental Exposure Document",
            ElectionOfHospiceBenefit => "Election of Hospice Benefit",
            Eligibility => "Eligibility",
            ExperimentalMaterialPurchaseOrder => "Experimental Material Purchase Order",
            EngineeringChangeRequest => "Engineering Change Request",
            SourceSelectionPlan => "Source Selection Plan",
            ShippersExportDeclaration => "Shippers Export Declaration",
            BarrelForBarrelExchangeContractStatusStatement => {
                "Barrel for Barrel Exchange Contract Status Statement"
            }
            CodeF1 => "Cost Performance Report (CPR) Format 1",
            CodeF2 => "Cost Performance Report (CPR) Format 2",
            CodeF3 => "Cost Performance Report (CPR) Format 3",
            CodeF4 => "Cost Performance Report (CPR) Format 4",
            CodeF5 => "Cost Performance Report (CPR) Format 5",
            TransportationCarrierInspectionReport => {
                "Transportation Carrier Inspection Report"
            }
            GovernmentInspectionReport => "Government Inspection Report",
            CodeF8 => "Inspection Waiver (Written)",
            CodeF9 => "Inspection Waiver (Oral)",
            FederalBureauOfInvestigation => "Federal Bureau of Investigation",
            FumigationCertificate => "Fumigation Certificate",
            FederalSpecificationCompliance => "Federal Specification Compliance",
            FederalEmergencyManagementAgency => "Federal Emergency Management Agency",
            LimitationOfHeavyElements => "Limitation of Heavy Elements",
            FireReport => "Fire Report",
            FamilyMedicalHistoryDocument => "Family Medical History Document",
            PostOperativeRadiologyFilms => "Post-Operative Radiology Films",
            PreOperativeRadiologyFilms => "Pre-Operative Radiology Films",
            CertificateOfFreeSale => "Certificate of Free Sale",
            StateForm => "State Form",
            ClearanceLetter => "Clearance Letter",
            BackgroundRelease => "Background Release",
            ExamResults => "Exam Results",
            PrelicenseCertificate => "Prelicense Certificate",
            NationalAssociationOfSecuritiesDealersCertification => {
                "National Association of Securities Dealers Certification"
            }
            LicenseCopy => "License Copy",
            GasProcessorsReport => "Gas Processor's Report",
            GasTransportersReport => "Gas Transporter's Report",
            HealthCertificate => "Health Certificate",
            HazardousMaterialIncident => "Hazardous Material Incident",
            HistoryAndPhysical => "History and Physical",
            HealthClinicRecords => "Health Clinic Records",
            HazardousWasteManifest => "Hazardous Waste Manifest",
            ConsularInvoice => "Consular Invoice",
            CustomsInvoice => "Customs Invoice",
            ForwardersInvoice => "Forwarder's Invoice",
            ImmunizationRecord => "Immunization Record",
            CarriersInvoice => "Carrier's Invoice",
            InsuranceAttachment => "Insurance Attachment",
            InsuranceCertificate => "Insurance Certificate",
            ImportLicense => "Import License",
            InspectionRequest => "Inspection Request",
            InventoryParameterReport => "Inventory Parameter Report",
            StateSchoolImmunizationRecords => "State School Immunization Records",
            IndexSystem => "Index System",
            CertifiedInspectionAndTestResults => "Certified Inspection and Test Results",
            InspectionResult => "Inspection Result",
            Invoice => "Invoice",
            CertificateOfGoodStanding => "Certificate of Good Standing",
            TaxStatusClearance => "Tax Status Clearance",
            ConsentToUseName => "Consent to Use Name",
            CertificateOfRegistration => "Certificate of Registration",
            CertificateOfExistence => "Certificate of Existence",
            CertificateOfStatus => "Certificate of Status",
            CertificateOfNameChange => "Certificate of Name Change",
            CertificateOfMerger => "Certificate of Merger",
            CertificateOfSignificantChange => "Certificate of Significant Change",
            BalanceSheet => "Balance Sheet",
            ApplicationOfNameReservation => "Application of Name Reservation",
            ScheduleOfCapital => "Schedule of Capital",
            ForeignTaxReturn => "Foreign Tax Return",
            PermitApplication => "Permit Application",
            AdmissionTaxReturn => "Admission Tax Return",
            AddendumToArticles => "Addendum to Articles",
            ArticlesAndAmendments => "Articles and Amendments",
            AppointmentOfCommissionerAsRegisteredAgent => {
                "Appointment of Commissioner as Registered Agent"
            }
            CertificateOfDisclosure => "Certificate of Disclosure",
            NoticeOfRegisteredOffice => "Notice of Registered Office",
            NoticeOfDirectors => "Notice of Directors",
            OrganizationAndFirstBiennialReport => {
                "Organization and First Biennial Report"
            }
            AgreementOfStatutoryAgent => "Agreement of Statutory Agent",
            ConsentToAct => "Consent to Act",
            CodeKA => "Contract Data Requirements List (CDRL)",
            KosherCertificate => "Kosher Certificate",
            EngineeringDrawingList => "Engineering Drawing List",
            PurchasedEngineeringDataList => "Purchased Engineering Data List",
            SupportDocuments => "Support Documents",
            PurchasedDocuments => "Purchased Documents",
            ProposalSupportData => "Proposal Support Data",
            PurchasedDrawings => "Purchased Drawings",
            ChangeProposalData => "Change Proposal Data",
            ReportOfAssignmentOrModificationOfKeyEvents => {
                "Report of Assignment or Modification of Key Events"
            }
            RequestForAssignmentOrModificationOfKeyEvents => {
                "Request for Assignment or Modification of Key Events"
            }
            LaboratoryResults => "Laboratory Results",
            LegalizedBillOfLading => "Legalized Bill of Lading",
            LocationInventoryReport => "Location Inventory Report",
            CodeLD => "Laboratory Quality Review Variation, Deviation",
            LatestRevisedEstimate => "Latest Revised Estimate",
            LegalizedCertificateOfOrigin => "Legalized Certificate of Origin",
            LegalizedInvoice => "Legalized Invoice",
            CodeLO => "Laboratory Quality Review Order, Waiver",
            LaborPlan => "Labor Plan",
            CodeLR => "Laboratory Quality Review Order, Deviation",
            LeaseSettlementStatement => "Lease Settlement Statement",
            LicenseApplicationAttachment => "License Application Attachment",
            CodeLW => "Laboratory Quality Review Variation, Waiver",
            MedicalRecordAttachment => "Medical Record Attachment",
            ManufacturerAgentInventoryReport => "Manufacturer/Agent Inventory Report",
            ManufacturerDistributorInventoryReport => {
                "Manufacturer/Distributor Inventory Report"
            }
            ManufacturerCustomerInventoryReport => {
                "Manufacturer/Customer Inventory Report"
            }
            MaterialDataSheets => "Material Data Sheets",
            MajorDeviationRequest => "Major Deviation Request",
            ManufacturingSpecification => "Manufacturing Specification",
            CodeMG => "Migrant Student Records Transfer System (MSRTS) Record",
            ReportOfFullMaintenancePeriodDetail => {
                "Report of Full Maintenance Period Detail"
            }
            MortgageInsuranceCertification => "Mortgage Insurance Certification",
            RequestForMaintenancePeriodStatus => "Request for Maintenance Period Status",
            ReportOfMaintenancePeriodStatus => "Report of Maintenance Period Status",
            RequestForFullMaintenancePeriodDetail => {
                "Request for Full Maintenance Period Detail"
            }
            ManufacturerInventoryReport => "Manufacturer Inventory Report",
            MinorDeviationRequest => "Minor Deviation Request",
            ManufacturersStatementOfOrigin => "Manufacturer's Statement of Origin",
            CodeMP => {
                "Request for Establishment, Modification, or Cancellation of Maintenance Period"
            }
            CodeMQ => {
                "Report of Establishment, Modification, or Cancellation of Maintenance Period"
            }
            MaterialInspectionAndReceivingReport => {
                "Material Inspection and Receiving Report"
            }
            MaterialSafetyDataSheet => "Material Safety Data Sheet",
            Models => "Models",
            MeteredVolumes => "Metered Volumes",
            MotorVehicleReport => "Motor Vehicle Report",
            NationalInsuranceCrimeBureauAssignment => {
                "National Insurance Crime Bureau Assignment"
            }
            CertificateOfQuantity => "Certificate of Quantity",
            CommercialInvoice => "Commercial Invoice",
            NationalInsuranceCrimeBureau => "National Insurance Crime Bureau",
            NationalInsuranceCrimeBureauTotalLoss => {
                "National Insurance Crime Bureau Total Loss"
            }
            MonthlyContractorFinancialManagementReport => {
                "Monthly Contractor Financial Management Report"
            }
            NursingNotes => "Nursing Notes",
            NationalInsuranceCrimeBureauOtherThanTheft => {
                "National Insurance Crime Bureau Other than Theft"
            }
            QuarterlyContractorFinancialManagementReport => {
                "Quarterly Contractor Financial Management Report"
            }
            NOxEmissionsAveragingReport => "NOx Emissions Averaging Report",
            NationalInsuranceCrimeBureauTotalTheft => {
                "National Insurance Crime Bureau Total Theft"
            }
            OperativeNote => "Operative Note",
            OxygenContentAveragingReport => "Oxygen Content Averaging Report",
            OrdersAndTreatmentsDocument => "Orders and Treatments Document",
            CodeOE => "Objective Physical Examination (including vital signs) Document",
            OceanBillOfLading => "Ocean Bill of Lading",
            OutsideProductionOperationSheet => "Outside Production Operation Sheet",
            OilStorersReport => "Oil Storer's Report",
            OrganizationBreakdownStructure => "Organization Breakdown Structure",
            OilTransportersReport => "Oil Transporter's Report",
            OxygenTherapyCertification => "Oxygen Therapy Certification",
            SupportDataForClaim => "Support Data for Claim",
            PackingList => "Packing List",
            Protest => "Protest",
            Receipt => "Receipt",
            PathologyReport => "Pathology Report",
            PatientMedicalHistoryDocument => "Patient Medical History Document",
            PeriodontalCharts => "Periodontal Charts",
            PeriodontalReports => "Periodontal Reports",
            PropertyClaimReport => "Property Claim Report",
            PartDrawing => "Part Drawing",
            ProductCatalog => "Product Catalog",
            ProcessChangeNotice => "Process Change Notice",
            ProofOfDelivery => "Proof of Delivery",
            ParenteralOrEnteralCertification => "Parenteral or Enteral Certification",
            ProductSpecification => "Product Specification",
            PackagingSpecification => "Packaging Specification",
            ProductionHistoryPropertyLevel => "Production History - Property Level",
            ProductAvailabilityInquiry => "Product Availability Inquiry",
            PurchasingSpecification => "Purchasing Specification",
            StorageInformationInquiry => "Storage Information Inquiry",
            PropertyInsuranceLossRegister => "Property Insurance Loss Register",
            ProofOfInsurance => "Proof of Insurance",
            PhysicalTherapyNotes => "Physical Therapy Notes",
            ProstheticsOrOrthoticCertification => "Prosthetics or Orthotic Certification",
            Proposal => "Proposal",
            ParamedicalResults => "Paramedical Results",
            PurchaseReport => "Purchase Report",
            PipelineShipperInventoryReport => "Pipeline/Shipper Inventory Report",
            InterPlantInventoryReport => "Inter-Plant Inventory Report",
            PoliceReport => "Police Report",
            ProductionHistoryWellLevel => "Production History - Well Level",
            CodePX => "Production, Injection and Disposition Report",
            PhysiciansReport => "Physician's Report",
            PhysicalTherapyCertification => "Physical Therapy Certification",
            CauseAndCorrectiveActionReport => "Cause and Corrective Action Report",
            CodeQD => "Quality Review Order, Purchasing",
            QualityDetail => "Quality Detail",
            CodeQM => "Quality Review Order, Manufacturing",
            QualityReport => "Quality Report",
            QualityReviewOrderSupplement => "Quality Review Order Supplement",
            QualitySummary => "Quality Summary",
            ReformulatedGasolineAntiDumpingCompanyRegistration => {
                "Reformulated Gasoline/Anti-Dumping Company Registration"
            }
            ReformulatedGasolineAntiDumpingFacilityRegistration => {
                "Reformulated Gasoline/Anti-Dumping Facility Registration"
            }
            TechnicalInformationPackage => "Technical Information Package",
            PurchasedTechnicalInformationPackage => {
                "Purchased Technical Information Package"
            }
            Technical => "Technical Information",
            Miscellaneous => "Miscellaneous Information",
            ComplianceReview => "Compliance Review",
            Accident => "Accident",
            RevisionAnnouncement => "Revision Announcement",
            RadiologyFilms => "Radiology Films",
            RequestForCauseAndCorrectiveActionReport => {
                "Request for Cause and Corrective Action Report"
            }
            PaymentBond => "Payment Bond",
            PerformanceBond => "Performance Bond",
            ReliabilityFailRateReport => "Reliability Fail Rate Report",
            Residential => "Residential",
            BidBond => "Bid Bond",
            RequestForManufacturingEngineerAppraisal => {
                "Request for Manufacturing Engineer Appraisal"
            }
            SuppliersReportOfNonconformance => "Supplier's Report of Nonconformance",
            RegularOrder => "Regular Order",
            RadiologyReports => "Radiology Reports",
            ReportOfTestsAndAnalysisReport => "Report of Tests and Analysis Report",
            CodeRV => "Reid Vapor Pressure (RVP) Averaging Report",
            RenewableOxygenContentAveragingReport => {
                "Renewable Oxygen Content Averaging Report"
            }
            SupplyAndShipmentStatusReport => "Supply and Shipment Status Report",
            SupplyStatusReport => "Supply Status Report",
            ExceptionSupplyStatusReport => "Exception Supply Status Report",
            ExceptionSupplyAndShipmentStatusReport => {
                "Exception Supply and Shipment Status Report"
            }
            ProductQualityDeficiencyReportCategoryI => {
                "Product Quality Deficiency Report Category I"
            }
            ProductQualityDeficiencyReportCategoryIi => {
                "Product Quality Deficiency Report Category II"
            }
            CodeS7 => "\"Walsh-Healey Act\" Manufacturer or Regular Dealer",
            ReportOfFindings => "Report of Findings",
            Representation => "Representation",
            StatePoliceReport => "State Police Report",
            SampleApprovalAndRejectionList => "Sample Approval and Rejection List",
            SanitaryCertificate => "Sanitary Certificate",
            SupportDataForARequestForQuote => "Support Data for a Request for Quote",
            SecurityPoliceReport => "Security Police Report",
            ContractSecurityClassificationSpecification => {
                "Contract Security Classification Specification"
            }
            SymptomsDocument => "Symptoms Document",
            SheriffReport => "Sheriff Report",
            SellerInventoryReport => "Seller Inventory Report",
            StatementOfWork => "Statement of Work",
            SampleBaleList => "Sample Bale List",
            ShippingManifests => "Shipping Manifests",
            ShippingNotice => "Shipping Notice",
            SecretaryCertificate => "Secretary Certificate",
            Specification => "Specification",
            StatisticalQualityDocuments => "Statistical Quality Documents",
            StatisticalReport => "Statistical Report",
            SellerSalesReport => "Seller Sales Report",
            CodeST => "Student Educational Record (Transcript)",
            SuppliersCertificate => "Supplier's Certificate",
            Survey => "Survey",
            SeaWaybill => "Sea Waybill",
            SteamshipDueBill => "Steamship Due Bill",
            TrainSheet => "Train Sheet",
            TitleBill => "Title Bill",
            PreliminaryTitleWork => "Preliminary Title Work",
            LoanDocuments => "Loan Documents",
            Tax => "Tax Information",
            ToxicsEmissionsPerformanceAveragingReport => {
                "Toxics Emissions Performance Averaging Report"
            }
            ToxicsReleaseInventory => "Toxics Release Inventory",
            TherapyNotes => "Therapy Notes",
            AssetSupportInquiry => "Asset Support Inquiry",
            AssetSupportAdvice => "Asset Support Advice",
            PhysicalInventoryRequest => "Physical Inventory Request",
            AssetReclassificationResponse => "Asset Reclassification Response",
            AssetReclassificationRequest => "Asset Reclassification Request",
            TransactionHistoryRequest => "Transaction History Request",
            TwoToFourFamily => "Two to Four Family",
            TotalTheftClaimReport => "Total Theft Claim Report",
            AssetStatusInquiry => "Asset Status Inquiry",
            AssetStatusAdvice => "Asset Status Advice",
            LogisticsTransferInquiry => "Logistics Transfer Inquiry",
            LogisticsTransferAdvice => "Logistics Transfer Advice",
            StockSaleReport => "Stock Sale Report",
            DelayedSaleReport => "Delayed Sale Report",
            DemandReport => "Demand Report",
            TreatmentsCertificate => "Treatments Certificate",
            StorageInformationAdvice => "Storage Information Advice",
            TransmittalLetter => "Transmittal Letter",
            CodeTS => "Sulfur, Olefins, and T90 Averaging Report",
            TitleTransfer => "Title Transfer",
            TaxExemptCertificate => "Tax-exempt Certificate",
            SurveyReport => "Survey Report",
            UnionAgreement => "Union Agreement",
            CertificateOfDesignationOfRegisteredAgent => {
                "Certificate of Designation of Registered Agent"
            }
            ListOfOfficersAndDirectors => "List of Officers and Directors",
            ResolutionAndConsentForm => "Resolution and Consent Form",
            DomesticBusinessCorporationInitialReport => {
                "Domestic Business Corporation Initial Report"
            }
            RegisteredAgentApplication => "Registered Agent Application",
            ArticlesOfIncorporation => "Articles of Incorporation",
            CertificateOfCompliance => "Certificate of Compliance",
            CertificateOfAuthorization => "Certificate of Authorization",
            Charter => "Charter",
            OtherTypeOfReport => "Other Type of Report",
            AffidavitOfAcceptance => "Affidavit of Acceptance",
            ResolutionAdoptingFictitiousName => "Resolution Adopting Fictitious Name",
            TradeNameApplication => "Trade Name Application",
            DeclarationOfSolicitor => "Declaration of Solicitor",
            MemorandumOfAssociation => "Memorandum of Association",
            NoticeOfRegisteredAgent => "Notice of Registered Agent",
            CodeUS => "\"BUY AMERICA\" Certification of Compliance",
            DissolutionOfExistingRegistration => "Dissolution of Existing Registration",
            AppointmentOfStatutoryAgent => "Appointment of Statutory Agent",
            RegulatoryApprovalForProfessionalAssociation => {
                "Regulatory Approval for Professional Association"
            }
            InitialAnnualReport => "Initial Annual Report",
            CertificateOfFact => "Certificate of Fact",
            VoterRegistrationApplication => "Voter Registration Application",
            VoterRegistrationApplicationDisposition => {
                "Voter Registration Application Disposition"
            }
            VoterInformationRecord => "Voter Information Record",
            ChangeOfNameAndOrAddress => "Change of Name and/or Address",
            DeathNotification => "Death Notification",
            FelonyConvictionNotification => "Felony Conviction Notification",
            IncompetencyNotification => "Incompetency Notification",
            VarianceAnalysis => "Variance Analysis",
            CodeVC => "Volatile Organic Compounds (VOC) Emissions Averaging Report",
            DataRequestForVendorsSpecificationsOrDrawings => {
                "Data Request for Vendor's Specifications or Drawings."
            }
            VisualMechanicalAverageOutgoingQualityReport => {
                "Visual/Mechanical Average Outgoing Quality Report"
            }
            SafeDrinkingWaterBacteriologicalReport => {
                "Safe Drinking Water Bacteriological Report"
            }
            SafeDrinkingWaterReport => "Safe Drinking Water Report",
            FictitiousNameStatement => "Fictitious Name Statement",
            WorkBreakdownStructure => "Work Breakdown Structure",
            RequestForAssignmentOrDeletionOfWorkCandidate => {
                "Request for Assignment or Deletion of Work Candidate"
            }
            ReportOfAssignmentOrDeletionOfWorkCandidate => {
                "Report of Assignment or Deletion of Work Candidate"
            }
            BusinessConductedPriorToQualificationForm => {
                "Business Conducted Prior to Qualification Form"
            }
            ByLaws => "By-Laws",
            AppointmentOfAgentForServiceAndConsentToAct => {
                "Appointment of Agent for Service and Consent to Act"
            }
            CertificateOfNameClearance => "Certificate of Name Clearance",
            Well => "Well Information",
            WorkProgress => "Work Progress",
            WellTest => "Well Test Information",
            CompleteAppraisal => "Complete Appraisal",
            LimitedAppraisal => "Limited Appraisal",
            SelfContainedReport => "Self-contained Report",
            SummaryReport => "Summary Report",
            RestrictedReport => "Restricted Report",
            EquipmentTestResults => "Equipment Test Results",
            Photographs => "Photographs",
            Appraisal => "Appraisal",
            BrokerPriceOpinion => "Broker Price Opinion",
            RealEstateProperty => "Real Estate Property Information",
            FloodDeterminationReport => "Flood Determination Report",
            ConventionalAmmunitionSuspensionReport => {
                "Conventional Ammunition Suspension Report"
            }
            SelfMonitoringReport => "Self Monitoring Report",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<ReportTypeCode> {
        {
            use ReportTypeCode::*;
            match description {
                "Product Transfer" => Some(ProductTransfer),
                "Resale" => Some(Resale),
                "Report Justifying Treatment Beyond Utilization Guidelines" => {
                    Some(ReportJustifyingTreatmentBeyondUtilizationGuidelines)
                }
                "Drugs Administered" => Some(DrugsAdministered),
                "Treatment Diagnosis" => Some(TreatmentDiagnosis),
                "Initial Assessment" => Some(InitialAssessment),
                "Functional Goals" => Some(FunctionalGoals),
                "Plan of Treatment" => Some(PlanOfTreatment),
                "Progress Report" => Some(ProgressReport),
                "Continued Treatment" => Some(ContinuedTreatment),
                "Chemical Analysis" => Some(ChemicalAnalysis),
                "Mechanical Properties" => Some(MechanicalProperties),
                "Certified Test Report" => Some(CertifiedTestReport),
                "Core Loss Test Report" => Some(CoreLossTestReport),
                "Justification for Admission" => Some(JustificationForAdmission),
                "Western Region 01 (Internal Revenue Service Summary of line 31 on 1040)" => {
                    Some(Code16)
                }
                "Return From Tax Payer Report (Internal Revenue Service Summary)" => {
                    Some(Code17)
                }
                "Note Receivable" => Some(NoteReceivable),
                "Credit Report" => Some(CreditReport),
                "Process Plan" => Some(ProcessPlan),
                "Recovery Plan" => Some(RecoveryPlan),
                "Functional Plan" => Some(FunctionalPlan),
                "Contractual Plan" => Some(ContractualPlan),
                "Non-Contractual Plan" => Some(NonContractualPlan),
                "Purchase Plan" => Some(PurchasePlan),
                "Production Plan" => Some(ProductionPlan),
                "Contract Delivery Schedule" => Some(ContractDeliverySchedule),
                "Master Delivery Schedule" => Some(MasterDeliverySchedule),
                "Assembly Plan" => Some(AssemblyPlan),
                "Lead Time Chart" => Some(LeadTimeChart),
                "Master Schedule" => Some(MasterSchedule),
                "Master Production Schedule" => Some(MasterProductionSchedule),
                "Manufacturing Flow Diagram" => Some(ManufacturingFlowDiagram),
                "Lot Release Plan" => Some(LotReleasePlan),
                "Subcontractor Plan" => Some(SubcontractorPlan),
                "Development Plan" => Some(DevelopmentPlan),
                "Lease Agreement" => Some(LeaseAgreement),
                "Court Judgment" => Some(CourtJudgment),
                "Lottery Winning Letter" => Some(LotteryWinningLetter),
                "Curb Side Inspection" => Some(CurbSideInspection),
                "Statistical Model" => Some(StatisticalModel),
                "Interior and Exterior Inspection" => Some(InteriorAndExteriorInspection),
                "Non-Inspection Report" => Some(NonInspectionReport),
                "Review Appraisal" => Some(ReviewAppraisal),
                "Investor Guidelines" => Some(InvestorGuidelines),
                "Sole Discretion Inspection" => Some(SoleDiscretionInspection),
                "Brokers Price Opinion" => Some(BrokersPriceOpinion),
                "Social Security Benefit Letter" => Some(SocialSecurityBenefitLetter),
                "Divorce Decree" => Some(DivorceDecree),
                "Contract" => Some(Contract),
                "Gift Letter" => Some(GiftLetter),
                "Will" => Some(Will),
                "Trust Agreement" => Some(TrustAgreement),
                "Award Letter" => Some(AwardLetter),
                "Rental Agreement" => Some(RentalAgreement),
                "Preliminary Real Estate Settlement Statement" => {
                    Some(PreliminaryRealEstateSettlementStatement)
                }
                "Income Statement (Internal Revenue Service Form 1099)" => Some(Code57),
                "Utility Bill" => Some(UtilityBill),
                "Benefit Letter" => Some(BenefitLetter),
                "Canceled Check" => Some(CanceledCheck),
                "Individual Tax Return (Internal Revenue Service Form 1040)" => {
                    Some(Code61)
                }
                "Asset Account Statement" => Some(AssetAccountStatement),
                "Statement of Profit and Loss" => Some(StatementOfProfitAndLoss),
                "Partner Share of Income, Credit, Deductions, (Internal Revenue Service Form K1)" => {
                    Some(Code64)
                }
                "Partnership Tax Return (Internal Revenue Service Form 1120)" => {
                    Some(Code65)
                }
                "Pay Stub" => Some(PayStub),
                "Wage and Tax Statement (Internal Revenue Service Form W2)" => {
                    Some(Code67)
                }
                "Year End Statement" => Some(YearEndStatement),
                "Bank Statement" => Some(BankStatement),
                "Verification of Loan/Installment Debt Form" => {
                    Some(VerificationOfLoanInstallmentDebtForm)
                }
                "Verification of Deposit Form" => Some(VerificationOfDepositForm),
                "Verification of Mortgage/Rent Form" => {
                    Some(VerificationOfMortgageRentForm)
                }
                "Verification of Employment Form" => Some(VerificationOfEmploymentForm),
                "Corporation Tax Return (Internal Revenue Service Form 1065)" => {
                    Some(Code74)
                }
                "Title Certificates" => Some(TitleCertificates),
                "Real Estate Settlement Statement (Housing and Urban Development Form - 1 \"HUD1\")" => {
                    Some(Code76)
                }
                "Support Data for Verification" => Some(SupportDataForVerification),
                "Visa/Export License" => Some(VisaExportLicense),
                "Multi-Country Textile Declaration" => {
                    Some(MultiCountryTextileDeclaration)
                }
                "Single Country Textile Declaration" => {
                    Some(SingleCountryTextileDeclaration)
                }
                "Negative Textile Declaration" => Some(NegativeTextileDeclaration),
                "Endangered Species Convention on International Trade in Endangered Species (CITES)" => {
                    Some(Code82)
                }
                "Trademark Release" => Some(TrademarkRelease),
                "Water Resistance Statement" => Some(WaterResistanceStatement),
                "Certificate of Ceramicware Ceramics Commodities Inspection Bureau (CCIB)" => {
                    Some(Code85)
                }
                "Wearing Apparel Detail Sheet (WADS)" => Some(Code86),
                "Interim Footwear Invoice" => Some(InterimFootwearInvoice),
                "Impact Resistance Statement" => Some(ImpactResistanceStatement),
                "Toxic Substance Compliance Statement" => {
                    Some(ToxicSubstanceComplianceStatement)
                }
                "Foreign Shippers Declaration" => Some(ForeignShippersDeclaration),
                "Veterinarian Certificate" => Some(VeterinarianCertificate),
                "Child Labor Certificate" => Some(ChildLaborCertificate),
                "Prison Labor Certificate" => Some(PrisonLaborCertificate),
                "Purchase Order Copy" => Some(PurchaseOrderCopy),
                "Product Analysis" => Some(ProductAnalysis),
                "American Automotive Labeling Act Certificate" => {
                    Some(AmericanAutomotiveLabelingActCertificate)
                }
                "Broker Market Analysis" => Some(BrokerMarketAnalysis),
                "Air Emissions Statements" => Some(AirEmissionsStatements),
                "Anti-Dumping Gasoline Program Report" => {
                    Some(AntiDumpingGasolineProgramReport)
                }
                "Allergies/Sensitivities Document" => {
                    Some(AllergiesSensitivitiesDocument)
                }
                "Autopsy Report" => Some(AutopsyReport),
                "Agent Inventory Report" => Some(AgentInventoryReport),
                "Assembly Drawing" => Some(AssemblyDrawing),
                "Assay Certificate" => Some(AssayCertificate),
                "Agent/Distributor Inventory Report" => {
                    Some(AgentDistributorInventoryReport)
                }
                "Attachment" => Some(Attachment),
                "Aid Form" => Some(AidForm),
                "Actual" => Some(Actual),
                "Easement Report" => Some(EasementReport),
                "Conditions, Covenant, and Restrictions Report" => Some(CodeAI),
                "Impound Account Escrow Analysis Report" => {
                    Some(ImpoundAccountEscrowAnalysisReport)
                }
                "Closing Escrow Analysis Report" => Some(ClosingEscrowAnalysisReport),
                "Reserved Escrow Analysis Report" => Some(ReservedEscrowAnalysisReport),
                "Ambulance Certification" => Some(AmbulanceCertification),
                "Title Policy" => Some(TitlePolicy),
                "Average Outgoing Quality Report" => Some(AverageOutgoingQualityReport),
                "Advanced Problem Notification" => Some(AdvancedProblemNotification),
                "Housing and Urban Development (HUD) 1 Report" => Some(CodeAQ),
                "Asset Reclassification Extension Request" => {
                    Some(AssetReclassificationExtensionRequest)
                }
                "Admission Summary" => Some(AdmissionSummary),
                "Purchase Order Attachment" => Some(PurchaseOrderAttachment),
                "Automobile Claim Report" => Some(AutomobileClaimReport),
                "Averaging Areas Report" => Some(AveragingAreasReport),
                "Air Waybill" => Some(AirWaybill),
                "Asset Reclassification Extension Response" => {
                    Some(AssetReclassificationExtensionResponse)
                }
                "Tax Certificate" => Some(TaxCertificate),
                "Home Owner Authorization" => Some(HomeOwnerAuthorization),
                "Batch Report" => Some(BatchReport),
                "Prescription" => Some(Prescription),
                "Physician Order" => Some(PhysicianOrder),
                "Referral Form" => Some(ReferralForm),
                "Budget" => Some(Budget),
                "Buy or Sell Exchange Contract Status Statement" => {
                    Some(BuyOrSellExchangeContractStatusStatement)
                }
                "Bill of Lading Copy" => Some(BillOfLadingCopy),
                "Benzene Content Averaging Report" => Some(BenzeneContentAveragingReport),
                "Bailment Warehouse Withdrawal Request" => {
                    Some(BailmentWarehouseWithdrawalRequest)
                }
                "Bill of Lading" => Some(BillOfLading),
                "Bill of Material" => Some(BillOfMaterial),
                "Beneficiary Certificate" => Some(BeneficiaryCertificate),
                "Bill of Lading Original" => Some(BillOfLadingOriginal),
                "Benchmark Testing Results" => Some(BenchmarkTestingResults),
                "Baseline" => Some(Baseline),
                "Blanket Test Results" => Some(BlanketTestResults),
                "Bill of Sale" => Some(BillOfSale),
                "Biennial Report" => Some(BiennialReport),
                "Cost Data Summary" => Some(CostDataSummary),
                "Functional Cost and Hour" => Some(FunctionalCostAndHour),
                "Progress Curve" => Some(ProgressCurve),
                "Plant-Wide Data" => Some(PlantWideData),
                "Certified Cost and Price Data" => Some(CertifiedCostAndPriceData),
                "Wage Determination" => Some(WageDetermination),
                "Credit Transfer Summary Report" => Some(CreditTransferSummaryReport),
                "Chemical/Radiological Report" => Some(ChemicalRadiologicalReport),
                "Certification/Authorization Document" => {
                    Some(CertificationAuthorizationDocument)
                }
                "Certificate of Analysis" => Some(CertificateOfAnalysis),
                "Chiropractic Justification" => Some(ChiropracticJustification),
                "C.A.A. Certificate of Conformance (British CAA)" => Some(CodeCC),
                "Customer/Distributor Inventory Report" => {
                    Some(CustomerDistributorInventoryReport)
                }
                "Constable Report" => Some(ConstableReport),
                "Capability" => Some(Capability),
                "Certificate of Origin" => Some(CertificateOfOrigin),
                "Certificate of Weight" => Some(CertificateOfWeight),
                "Certificate of Inspection Report" => Some(CertificateOfInspectionReport),
                "Complications Document" => Some(ComplicationsDocument),
                "Consent Form(s)" => Some(CodeCK),
                "Cable" => Some(Cable),
                "Customer/Manufacturer Inventory Report" => {
                    Some(CustomerManufacturerInventoryReport)
                }
                "Customer's Report of Nonconformance" => {
                    Some(CustomersReportOfNonconformance)
                }
                "Consignment Order" => Some(ConsignmentOrder),
                "Certificate of Compliance (Material Certification)" => Some(CodeCP),
                "County Record" => Some(CountyRecord),
                "Letter of Credit" => Some(LetterOfCredit),
                "Consigned Inventory Sales Report" => Some(ConsignedInventorySalesReport),
                "Certification" => Some(Certification),
                "Customer Notification Letter" => Some(CustomerNotificationLetter),
                "Change of Hospice Benefit" => Some(ChangeOfHospiceBenefit),
                "Corrective Work Order" => Some(CorrectiveWorkOrder),
                "Cost/Schedule Status Report (C/SSR)" => Some(CodeCX),
                "Contract Funds Status Report (CFSR)" => Some(CodeCY),
                "Campus Police Report" => Some(CampusPoliceReport),
                "Drug Profile Document" => Some(DrugProfileDocument),
                "Dental Models" => Some(DentalModels),
                "Durable Medical Equipment Prescription" => {
                    Some(DurableMedicalEquipmentPrescription)
                }
                "Distributor/Customer Inventory Report" => {
                    Some(DistributorCustomerInventoryReport)
                }
                "Distributor Inventory Report" => Some(DistributorInventoryReport),
                "Certificate of Quality" => Some(CertificateOfQuality),
                "DA59 Special Customs Invoice for South Africa" => {
                    Some(Da59SpecialCustomsInvoiceForSouthAfrica)
                }
                "Diagnostic Report" => Some(DiagnosticReport),
                "Nitrogen Certificate" => Some(NitrogenCertificate),
                "Directory" => Some(Directory),
                "Discharge Monitoring Report" => Some(DischargeMonitoringReport),
                "Drawback Affidavit" => Some(DrawbackAffidavit),
                "Draft and Transmittal Letter" => Some(DraftAndTransmittalLetter),
                "Distributor/Manufacturer Inventory Report" => {
                    Some(DistributorManufacturerInventoryReport)
                }
                "Deviation/Nonconformance Test Results and Request for Action" => {
                    Some(DeviationNonconformanceTestResultsAndRequestForAction)
                }
                "Delinquency" => Some(Delinquency),
                "Datalog Report" => Some(DatalogReport),
                "Discharge Summary" => Some(DischargeSummary),
                "Department of Transportation" => Some(DepartmentOfTransportation),
                "Commercial" => Some(Commercial),
                "Condominium" => Some(Condominium),
                "Drawing(s)" => Some(CodeDW),
                "Exporter's Certificate and Agreement" => {
                    Some(ExportersCertificateAndAgreement)
                }
                "Electrical Average Outgoing Quality Report" => {
                    Some(ElectricalAverageOutgoingQualityReport)
                }
                "Explanation of Benefits (Coordination of Benefits or Medicare Secondary Payor)" => {
                    Some(CodeEB)
                }
                "Engineering Change Order" => Some(EngineeringChangeOrder),
                "Environmental Exposure Document" => Some(EnvironmentalExposureDocument),
                "Election of Hospice Benefit" => Some(ElectionOfHospiceBenefit),
                "Eligibility" => Some(Eligibility),
                "Experimental Material Purchase Order" => {
                    Some(ExperimentalMaterialPurchaseOrder)
                }
                "Engineering Change Request" => Some(EngineeringChangeRequest),
                "Source Selection Plan" => Some(SourceSelectionPlan),
                "Shippers Export Declaration" => Some(ShippersExportDeclaration),
                "Barrel for Barrel Exchange Contract Status Statement" => {
                    Some(BarrelForBarrelExchangeContractStatusStatement)
                }
                "Cost Performance Report (CPR) Format 1" => Some(CodeF1),
                "Cost Performance Report (CPR) Format 2" => Some(CodeF2),
                "Cost Performance Report (CPR) Format 3" => Some(CodeF3),
                "Cost Performance Report (CPR) Format 4" => Some(CodeF4),
                "Cost Performance Report (CPR) Format 5" => Some(CodeF5),
                "Transportation Carrier Inspection Report" => {
                    Some(TransportationCarrierInspectionReport)
                }
                "Government Inspection Report" => Some(GovernmentInspectionReport),
                "Inspection Waiver (Written)" => Some(CodeF8),
                "Inspection Waiver (Oral)" => Some(CodeF9),
                "Federal Bureau of Investigation" => Some(FederalBureauOfInvestigation),
                "Fumigation Certificate" => Some(FumigationCertificate),
                "Federal Specification Compliance" => {
                    Some(FederalSpecificationCompliance)
                }
                "Federal Emergency Management Agency" => {
                    Some(FederalEmergencyManagementAgency)
                }
                "Limitation of Heavy Elements" => Some(LimitationOfHeavyElements),
                "Fire Report" => Some(FireReport),
                "Family Medical History Document" => Some(FamilyMedicalHistoryDocument),
                "Post-Operative Radiology Films" => Some(PostOperativeRadiologyFilms),
                "Pre-Operative Radiology Films" => Some(PreOperativeRadiologyFilms),
                "Certificate of Free Sale" => Some(CertificateOfFreeSale),
                "State Form" => Some(StateForm),
                "Clearance Letter" => Some(ClearanceLetter),
                "Background Release" => Some(BackgroundRelease),
                "Exam Results" => Some(ExamResults),
                "Prelicense Certificate" => Some(PrelicenseCertificate),
                "National Association of Securities Dealers Certification" => {
                    Some(NationalAssociationOfSecuritiesDealersCertification)
                }
                "License Copy" => Some(LicenseCopy),
                "Gas Processor's Report" => Some(GasProcessorsReport),
                "Gas Transporter's Report" => Some(GasTransportersReport),
                "Health Certificate" => Some(HealthCertificate),
                "Hazardous Material Incident" => Some(HazardousMaterialIncident),
                "History and Physical" => Some(HistoryAndPhysical),
                "Health Clinic Records" => Some(HealthClinicRecords),
                "Hazardous Waste Manifest" => Some(HazardousWasteManifest),
                "Consular Invoice" => Some(ConsularInvoice),
                "Customs Invoice" => Some(CustomsInvoice),
                "Forwarder's Invoice" => Some(ForwardersInvoice),
                "Immunization Record" => Some(ImmunizationRecord),
                "Carrier's Invoice" => Some(CarriersInvoice),
                "Insurance Attachment" => Some(InsuranceAttachment),
                "Insurance Certificate" => Some(InsuranceCertificate),
                "Import License" => Some(ImportLicense),
                "Inspection Request" => Some(InspectionRequest),
                "Inventory Parameter Report" => Some(InventoryParameterReport),
                "State School Immunization Records" => {
                    Some(StateSchoolImmunizationRecords)
                }
                "Index System" => Some(IndexSystem),
                "Certified Inspection and Test Results" => {
                    Some(CertifiedInspectionAndTestResults)
                }
                "Inspection Result" => Some(InspectionResult),
                "Invoice" => Some(Invoice),
                "Certificate of Good Standing" => Some(CertificateOfGoodStanding),
                "Tax Status Clearance" => Some(TaxStatusClearance),
                "Consent to Use Name" => Some(ConsentToUseName),
                "Certificate of Registration" => Some(CertificateOfRegistration),
                "Certificate of Existence" => Some(CertificateOfExistence),
                "Certificate of Status" => Some(CertificateOfStatus),
                "Certificate of Name Change" => Some(CertificateOfNameChange),
                "Certificate of Merger" => Some(CertificateOfMerger),
                "Certificate of Significant Change" => {
                    Some(CertificateOfSignificantChange)
                }
                "Balance Sheet" => Some(BalanceSheet),
                "Application of Name Reservation" => Some(ApplicationOfNameReservation),
                "Schedule of Capital" => Some(ScheduleOfCapital),
                "Foreign Tax Return" => Some(ForeignTaxReturn),
                "Permit Application" => Some(PermitApplication),
                "Admission Tax Return" => Some(AdmissionTaxReturn),
                "Addendum to Articles" => Some(AddendumToArticles),
                "Articles and Amendments" => Some(ArticlesAndAmendments),
                "Appointment of Commissioner as Registered Agent" => {
                    Some(AppointmentOfCommissionerAsRegisteredAgent)
                }
                "Certificate of Disclosure" => Some(CertificateOfDisclosure),
                "Notice of Registered Office" => Some(NoticeOfRegisteredOffice),
                "Notice of Directors" => Some(NoticeOfDirectors),
                "Organization and First Biennial Report" => {
                    Some(OrganizationAndFirstBiennialReport)
                }
                "Agreement of Statutory Agent" => Some(AgreementOfStatutoryAgent),
                "Consent to Act" => Some(ConsentToAct),
                "Contract Data Requirements List (CDRL)" => Some(CodeKA),
                "Kosher Certificate" => Some(KosherCertificate),
                "Engineering Drawing List" => Some(EngineeringDrawingList),
                "Purchased Engineering Data List" => Some(PurchasedEngineeringDataList),
                "Support Documents" => Some(SupportDocuments),
                "Purchased Documents" => Some(PurchasedDocuments),
                "Proposal Support Data" => Some(ProposalSupportData),
                "Purchased Drawings" => Some(PurchasedDrawings),
                "Change Proposal Data" => Some(ChangeProposalData),
                "Report of Assignment or Modification of Key Events" => {
                    Some(ReportOfAssignmentOrModificationOfKeyEvents)
                }
                "Request for Assignment or Modification of Key Events" => {
                    Some(RequestForAssignmentOrModificationOfKeyEvents)
                }
                "Laboratory Results" => Some(LaboratoryResults),
                "Legalized Bill of Lading" => Some(LegalizedBillOfLading),
                "Location Inventory Report" => Some(LocationInventoryReport),
                "Laboratory Quality Review Variation, Deviation" => Some(CodeLD),
                "Latest Revised Estimate" => Some(LatestRevisedEstimate),
                "Legalized Certificate of Origin" => Some(LegalizedCertificateOfOrigin),
                "Legalized Invoice" => Some(LegalizedInvoice),
                "Laboratory Quality Review Order, Waiver" => Some(CodeLO),
                "Labor Plan" => Some(LaborPlan),
                "Laboratory Quality Review Order, Deviation" => Some(CodeLR),
                "Lease Settlement Statement" => Some(LeaseSettlementStatement),
                "License Application Attachment" => Some(LicenseApplicationAttachment),
                "Laboratory Quality Review Variation, Waiver" => Some(CodeLW),
                "Medical Record Attachment" => Some(MedicalRecordAttachment),
                "Manufacturer/Agent Inventory Report" => {
                    Some(ManufacturerAgentInventoryReport)
                }
                "Manufacturer/Distributor Inventory Report" => {
                    Some(ManufacturerDistributorInventoryReport)
                }
                "Manufacturer/Customer Inventory Report" => {
                    Some(ManufacturerCustomerInventoryReport)
                }
                "Material Data Sheets" => Some(MaterialDataSheets),
                "Major Deviation Request" => Some(MajorDeviationRequest),
                "Manufacturing Specification" => Some(ManufacturingSpecification),
                "Migrant Student Records Transfer System (MSRTS) Record" => Some(CodeMG),
                "Report of Full Maintenance Period Detail" => {
                    Some(ReportOfFullMaintenancePeriodDetail)
                }
                "Mortgage Insurance Certification" => {
                    Some(MortgageInsuranceCertification)
                }
                "Request for Maintenance Period Status" => {
                    Some(RequestForMaintenancePeriodStatus)
                }
                "Report of Maintenance Period Status" => {
                    Some(ReportOfMaintenancePeriodStatus)
                }
                "Request for Full Maintenance Period Detail" => {
                    Some(RequestForFullMaintenancePeriodDetail)
                }
                "Manufacturer Inventory Report" => Some(ManufacturerInventoryReport),
                "Minor Deviation Request" => Some(MinorDeviationRequest),
                "Manufacturer's Statement of Origin" => {
                    Some(ManufacturersStatementOfOrigin)
                }
                "Request for Establishment, Modification, or Cancellation of Maintenance Period" => {
                    Some(CodeMP)
                }
                "Report of Establishment, Modification, or Cancellation of Maintenance Period" => {
                    Some(CodeMQ)
                }
                "Material Inspection and Receiving Report" => {
                    Some(MaterialInspectionAndReceivingReport)
                }
                "Material Safety Data Sheet" => Some(MaterialSafetyDataSheet),
                "Models" => Some(Models),
                "Metered Volumes" => Some(MeteredVolumes),
                "Motor Vehicle Report" => Some(MotorVehicleReport),
                "National Insurance Crime Bureau Assignment" => {
                    Some(NationalInsuranceCrimeBureauAssignment)
                }
                "Certificate of Quantity" => Some(CertificateOfQuantity),
                "Commercial Invoice" => Some(CommercialInvoice),
                "National Insurance Crime Bureau" => Some(NationalInsuranceCrimeBureau),
                "National Insurance Crime Bureau Total Loss" => {
                    Some(NationalInsuranceCrimeBureauTotalLoss)
                }
                "Monthly Contractor Financial Management Report" => {
                    Some(MonthlyContractorFinancialManagementReport)
                }
                "Nursing Notes" => Some(NursingNotes),
                "National Insurance Crime Bureau Other than Theft" => {
                    Some(NationalInsuranceCrimeBureauOtherThanTheft)
                }
                "Quarterly Contractor Financial Management Report" => {
                    Some(QuarterlyContractorFinancialManagementReport)
                }
                "NOx Emissions Averaging Report" => Some(NOxEmissionsAveragingReport),
                "National Insurance Crime Bureau Total Theft" => {
                    Some(NationalInsuranceCrimeBureauTotalTheft)
                }
                "Operative Note" => Some(OperativeNote),
                "Oxygen Content Averaging Report" => Some(OxygenContentAveragingReport),
                "Orders and Treatments Document" => Some(OrdersAndTreatmentsDocument),
                "Objective Physical Examination (including vital signs) Document" => {
                    Some(CodeOE)
                }
                "Ocean Bill of Lading" => Some(OceanBillOfLading),
                "Outside Production Operation Sheet" => {
                    Some(OutsideProductionOperationSheet)
                }
                "Oil Storer's Report" => Some(OilStorersReport),
                "Organization Breakdown Structure" => {
                    Some(OrganizationBreakdownStructure)
                }
                "Oil Transporter's Report" => Some(OilTransportersReport),
                "Oxygen Therapy Certification" => Some(OxygenTherapyCertification),
                "Support Data for Claim" => Some(SupportDataForClaim),
                "Packing List" => Some(PackingList),
                "Protest" => Some(Protest),
                "Receipt" => Some(Receipt),
                "Pathology Report" => Some(PathologyReport),
                "Patient Medical History Document" => Some(PatientMedicalHistoryDocument),
                "Periodontal Charts" => Some(PeriodontalCharts),
                "Periodontal Reports" => Some(PeriodontalReports),
                "Property Claim Report" => Some(PropertyClaimReport),
                "Part Drawing" => Some(PartDrawing),
                "Product Catalog" => Some(ProductCatalog),
                "Process Change Notice" => Some(ProcessChangeNotice),
                "Proof of Delivery" => Some(ProofOfDelivery),
                "Parenteral or Enteral Certification" => {
                    Some(ParenteralOrEnteralCertification)
                }
                "Product Specification" => Some(ProductSpecification),
                "Packaging Specification" => Some(PackagingSpecification),
                "Production History - Property Level" => {
                    Some(ProductionHistoryPropertyLevel)
                }
                "Product Availability Inquiry" => Some(ProductAvailabilityInquiry),
                "Purchasing Specification" => Some(PurchasingSpecification),
                "Storage Information Inquiry" => Some(StorageInformationInquiry),
                "Property Insurance Loss Register" => Some(PropertyInsuranceLossRegister),
                "Proof of Insurance" => Some(ProofOfInsurance),
                "Physical Therapy Notes" => Some(PhysicalTherapyNotes),
                "Prosthetics or Orthotic Certification" => {
                    Some(ProstheticsOrOrthoticCertification)
                }
                "Proposal" => Some(Proposal),
                "Paramedical Results" => Some(ParamedicalResults),
                "Purchase Report" => Some(PurchaseReport),
                "Pipeline/Shipper Inventory Report" => {
                    Some(PipelineShipperInventoryReport)
                }
                "Inter-Plant Inventory Report" => Some(InterPlantInventoryReport),
                "Police Report" => Some(PoliceReport),
                "Production History - Well Level" => Some(ProductionHistoryWellLevel),
                "Production, Injection and Disposition Report" => Some(CodePX),
                "Physician's Report" => Some(PhysiciansReport),
                "Physical Therapy Certification" => Some(PhysicalTherapyCertification),
                "Cause and Corrective Action Report" => {
                    Some(CauseAndCorrectiveActionReport)
                }
                "Quality Review Order, Purchasing" => Some(CodeQD),
                "Quality Detail" => Some(QualityDetail),
                "Quality Review Order, Manufacturing" => Some(CodeQM),
                "Quality Report" => Some(QualityReport),
                "Quality Review Order Supplement" => Some(QualityReviewOrderSupplement),
                "Quality Summary" => Some(QualitySummary),
                "Reformulated Gasoline/Anti-Dumping Company Registration" => {
                    Some(ReformulatedGasolineAntiDumpingCompanyRegistration)
                }
                "Reformulated Gasoline/Anti-Dumping Facility Registration" => {
                    Some(ReformulatedGasolineAntiDumpingFacilityRegistration)
                }
                "Technical Information Package" => Some(TechnicalInformationPackage),
                "Purchased Technical Information Package" => {
                    Some(PurchasedTechnicalInformationPackage)
                }
                "Technical Information" => Some(Technical),
                "Miscellaneous Information" => Some(Miscellaneous),
                "Compliance Review" => Some(ComplianceReview),
                "Accident" => Some(Accident),
                "Revision Announcement" => Some(RevisionAnnouncement),
                "Radiology Films" => Some(RadiologyFilms),
                "Request for Cause and Corrective Action Report" => {
                    Some(RequestForCauseAndCorrectiveActionReport)
                }
                "Payment Bond" => Some(PaymentBond),
                "Performance Bond" => Some(PerformanceBond),
                "Reliability Fail Rate Report" => Some(ReliabilityFailRateReport),
                "Residential" => Some(Residential),
                "Bid Bond" => Some(BidBond),
                "Request for Manufacturing Engineer Appraisal" => {
                    Some(RequestForManufacturingEngineerAppraisal)
                }
                "Supplier's Report of Nonconformance" => {
                    Some(SuppliersReportOfNonconformance)
                }
                "Regular Order" => Some(RegularOrder),
                "Radiology Reports" => Some(RadiologyReports),
                "Report of Tests and Analysis Report" => {
                    Some(ReportOfTestsAndAnalysisReport)
                }
                "Reid Vapor Pressure (RVP) Averaging Report" => Some(CodeRV),
                "Renewable Oxygen Content Averaging Report" => {
                    Some(RenewableOxygenContentAveragingReport)
                }
                "Supply and Shipment Status Report" => {
                    Some(SupplyAndShipmentStatusReport)
                }
                "Supply Status Report" => Some(SupplyStatusReport),
                "Exception Supply Status Report" => Some(ExceptionSupplyStatusReport),
                "Exception Supply and Shipment Status Report" => {
                    Some(ExceptionSupplyAndShipmentStatusReport)
                }
                "Product Quality Deficiency Report Category I" => {
                    Some(ProductQualityDeficiencyReportCategoryI)
                }
                "Product Quality Deficiency Report Category II" => {
                    Some(ProductQualityDeficiencyReportCategoryIi)
                }
                "\"Walsh-Healey Act\" Manufacturer or Regular Dealer" => Some(CodeS7),
                "Report of Findings" => Some(ReportOfFindings),
                "Representation" => Some(Representation),
                "State Police Report" => Some(StatePoliceReport),
                "Sample Approval and Rejection List" => {
                    Some(SampleApprovalAndRejectionList)
                }
                "Sanitary Certificate" => Some(SanitaryCertificate),
                "Support Data for a Request for Quote" => {
                    Some(SupportDataForARequestForQuote)
                }
                "Security Police Report" => Some(SecurityPoliceReport),
                "Contract Security Classification Specification" => {
                    Some(ContractSecurityClassificationSpecification)
                }
                "Symptoms Document" => Some(SymptomsDocument),
                "Sheriff Report" => Some(SheriffReport),
                "Seller Inventory Report" => Some(SellerInventoryReport),
                "Statement of Work" => Some(StatementOfWork),
                "Sample Bale List" => Some(SampleBaleList),
                "Shipping Manifests" => Some(ShippingManifests),
                "Shipping Notice" => Some(ShippingNotice),
                "Secretary Certificate" => Some(SecretaryCertificate),
                "Specification" => Some(Specification),
                "Statistical Quality Documents" => Some(StatisticalQualityDocuments),
                "Statistical Report" => Some(StatisticalReport),
                "Seller Sales Report" => Some(SellerSalesReport),
                "Student Educational Record (Transcript)" => Some(CodeST),
                "Supplier's Certificate" => Some(SuppliersCertificate),
                "Survey" => Some(Survey),
                "Sea Waybill" => Some(SeaWaybill),
                "Steamship Due Bill" => Some(SteamshipDueBill),
                "Train Sheet" => Some(TrainSheet),
                "Title Bill" => Some(TitleBill),
                "Preliminary Title Work" => Some(PreliminaryTitleWork),
                "Loan Documents" => Some(LoanDocuments),
                "Tax Information" => Some(Tax),
                "Toxics Emissions Performance Averaging Report" => {
                    Some(ToxicsEmissionsPerformanceAveragingReport)
                }
                "Toxics Release Inventory" => Some(ToxicsReleaseInventory),
                "Therapy Notes" => Some(TherapyNotes),
                "Asset Support Inquiry" => Some(AssetSupportInquiry),
                "Asset Support Advice" => Some(AssetSupportAdvice),
                "Physical Inventory Request" => Some(PhysicalInventoryRequest),
                "Asset Reclassification Response" => Some(AssetReclassificationResponse),
                "Asset Reclassification Request" => Some(AssetReclassificationRequest),
                "Transaction History Request" => Some(TransactionHistoryRequest),
                "Two to Four Family" => Some(TwoToFourFamily),
                "Total Theft Claim Report" => Some(TotalTheftClaimReport),
                "Asset Status Inquiry" => Some(AssetStatusInquiry),
                "Asset Status Advice" => Some(AssetStatusAdvice),
                "Logistics Transfer Inquiry" => Some(LogisticsTransferInquiry),
                "Logistics Transfer Advice" => Some(LogisticsTransferAdvice),
                "Stock Sale Report" => Some(StockSaleReport),
                "Delayed Sale Report" => Some(DelayedSaleReport),
                "Demand Report" => Some(DemandReport),
                "Treatments Certificate" => Some(TreatmentsCertificate),
                "Storage Information Advice" => Some(StorageInformationAdvice),
                "Transmittal Letter" => Some(TransmittalLetter),
                "Sulfur, Olefins, and T90 Averaging Report" => Some(CodeTS),
                "Title Transfer" => Some(TitleTransfer),
                "Tax-exempt Certificate" => Some(TaxExemptCertificate),
                "Survey Report" => Some(SurveyReport),
                "Union Agreement" => Some(UnionAgreement),
                "Certificate of Designation of Registered Agent" => {
                    Some(CertificateOfDesignationOfRegisteredAgent)
                }
                "List of Officers and Directors" => Some(ListOfOfficersAndDirectors),
                "Resolution and Consent Form" => Some(ResolutionAndConsentForm),
                "Domestic Business Corporation Initial Report" => {
                    Some(DomesticBusinessCorporationInitialReport)
                }
                "Registered Agent Application" => Some(RegisteredAgentApplication),
                "Articles of Incorporation" => Some(ArticlesOfIncorporation),
                "Certificate of Compliance" => Some(CertificateOfCompliance),
                "Certificate of Authorization" => Some(CertificateOfAuthorization),
                "Charter" => Some(Charter),
                "Other Type of Report" => Some(OtherTypeOfReport),
                "Affidavit of Acceptance" => Some(AffidavitOfAcceptance),
                "Resolution Adopting Fictitious Name" => {
                    Some(ResolutionAdoptingFictitiousName)
                }
                "Trade Name Application" => Some(TradeNameApplication),
                "Declaration of Solicitor" => Some(DeclarationOfSolicitor),
                "Memorandum of Association" => Some(MemorandumOfAssociation),
                "Notice of Registered Agent" => Some(NoticeOfRegisteredAgent),
                "\"BUY AMERICA\" Certification of Compliance" => Some(CodeUS),
                "Dissolution of Existing Registration" => {
                    Some(DissolutionOfExistingRegistration)
                }
                "Appointment of Statutory Agent" => Some(AppointmentOfStatutoryAgent),
                "Regulatory Approval for Professional Association" => {
                    Some(RegulatoryApprovalForProfessionalAssociation)
                }
                "Initial Annual Report" => Some(InitialAnnualReport),
                "Certificate of Fact" => Some(CertificateOfFact),
                "Voter Registration Application" => Some(VoterRegistrationApplication),
                "Voter Registration Application Disposition" => {
                    Some(VoterRegistrationApplicationDisposition)
                }
                "Voter Information Record" => Some(VoterInformationRecord),
                "Change of Name and/or Address" => Some(ChangeOfNameAndOrAddress),
                "Death Notification" => Some(DeathNotification),
                "Felony Conviction Notification" => Some(FelonyConvictionNotification),
                "Incompetency Notification" => Some(IncompetencyNotification),
                "Variance Analysis" => Some(VarianceAnalysis),
                "Volatile Organic Compounds (VOC) Emissions Averaging Report" => {
                    Some(CodeVC)
                }
                "Data Request for Vendor's Specifications or Drawings." => {
                    Some(DataRequestForVendorsSpecificationsOrDrawings)
                }
                "Visual/Mechanical Average Outgoing Quality Report" => {
                    Some(VisualMechanicalAverageOutgoingQualityReport)
                }
                "Safe Drinking Water Bacteriological Report" => {
                    Some(SafeDrinkingWaterBacteriologicalReport)
                }
                "Safe Drinking Water Report" => Some(SafeDrinkingWaterReport),
                "Fictitious Name Statement" => Some(FictitiousNameStatement),
                "Work Breakdown Structure" => Some(WorkBreakdownStructure),
                "Request for Assignment or Deletion of Work Candidate" => {
                    Some(RequestForAssignmentOrDeletionOfWorkCandidate)
                }
                "Report of Assignment or Deletion of Work Candidate" => {
                    Some(ReportOfAssignmentOrDeletionOfWorkCandidate)
                }
                "Business Conducted Prior to Qualification Form" => {
                    Some(BusinessConductedPriorToQualificationForm)
                }
                "By-Laws" => Some(ByLaws),
                "Appointment of Agent for Service and Consent to Act" => {
                    Some(AppointmentOfAgentForServiceAndConsentToAct)
                }
                "Certificate of Name Clearance" => Some(CertificateOfNameClearance),
                "Well Information" => Some(Well),
                "Work Progress" => Some(WorkProgress),
                "Well Test Information" => Some(WellTest),
                "Complete Appraisal" => Some(CompleteAppraisal),
                "Limited Appraisal" => Some(LimitedAppraisal),
                "Self-contained Report" => Some(SelfContainedReport),
                "Summary Report" => Some(SummaryReport),
                "Restricted Report" => Some(RestrictedReport),
                "Equipment Test Results" => Some(EquipmentTestResults),
                "Photographs" => Some(Photographs),
                "Appraisal" => Some(Appraisal),
                "Broker Price Opinion" => Some(BrokerPriceOpinion),
                "Real Estate Property Information" => Some(RealEstateProperty),
                "Flood Determination Report" => Some(FloodDeterminationReport),
                "Conventional Ammunition Suspension Report" => {
                    Some(ConventionalAmmunitionSuspensionReport)
                }
                "Self Monitoring Report" => Some(SelfMonitoringReport),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for ReportTypeCode {
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
    type Value = ReportTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Report Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ReportTypeCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Report Type Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ReportTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Report Type Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for ReportTypeCode {
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