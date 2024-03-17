use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**735

See docs at <https://www.stedi.com/edi/x12/element/735>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HierarchicalLevelCode {
    ///0 - Region
    Region,
    ///1 - Service/Billing Provider
    ServiceBillingProvider,
    ///2 - Billing Arrangement
    BillingArrangement,
    ///2A - Branch
    Branch,
    ///2B - Direct Affiliate
    DirectAffiliate,
    ///2C - Director
    Director,
    ///2D - Headquarters
    Headquarters,
    ///2E - Indirect Affiliate
    IndirectAffiliate,
    ///2F - Management Antecedents
    ManagementAntecedents,
    ///2G - Management or Principal
    ManagementOrPrincipal,
    ///2H - Parent Company
    ParentCompany,
    ///2I - Stockholder
    Stockholder,
    ///2J - Subsidiary
    Subsidiary,
    ///2K - Ultimate Domestic Parent Company
    UltimateDomesticParentCompany,
    ///2L - Ultimate Parent Company
    UltimateParentCompany,
    ///3 - Sub-Billing Arrangement
    SubBillingArrangement,
    ///4 - Group
    Group,
    ///5 - Category
    Category,
    ///6 - Sub-Category
    SubCategory,
    ///7 - Type
    Type,
    ///8 - Charge Detail
    ChargeDetail,
    ///9 - Line Detail
    LineDetail,
    ///19 - Provider of Service
    ProviderOfService,
    ///20 - Information Source
    InformationSource,
    ///21 - Information Receiver
    InformationReceiver,
    ///22 - Subscriber
    Subscriber,
    ///23 - Dependent
    Dependent,
    ///24 - Supergroup
    Supergroup,
    ///25 - Subgroup
    Subgroup,
    ///26 - Member
    Member,
    ///27 - Ancillary Facility or Department
    AncillaryFacilityOrDepartment,
    ///28 - Hospital
    Hospital,
    ///29 - Franchisor
    Franchisor,
    ///30 - Franchisee
    Franchisee,
    ///31 - Franchisee Association
    FranchiseeAssociation,
    ///32 - Health Industry Business Communications Council (HIBCC) Health Industry Number (HIN) Database
    Code32,
    ///33 - Activity
    Activity,
    ///34 - Location Record
    LocationRecord,
    ///35 - Company/Corporation
    CompanyCorporation,
    ///36 - Operating Unit
    OperatingUnit,
    ///37 - Property
    Property,
    ///38 - Tradename
    Tradename,
    ///39 - Accountant
    Accountant,
    ///40 - Financial Institution
    FinancialInstitution,
    ///41 - Product Level
    ProductLevel,
    ///42 - Activity Details
    ActivityDetails,
    ///43 - Payment Summary Score
    PaymentSummaryScore,
    ///44 - Corporate Registration Filings
    CorporateRegistrationFilings,
    ///45 - Bankruptcy Details
    BankruptcyDetails,
    ///46 - Company History
    CompanyHistory,
    ///47 - Complete Financial History
    CompleteFinancialHistory,
    ///48 - Balance Sheet
    BalanceSheet,
    ///49 - Comparative Figures
    ComparativeFigures,
    ///50 - Payment Analysis
    PaymentAnalysis,
    ///51 - Special Notification
    SpecialNotification,
    ///52 - Public Record Financing Details
    PublicRecordFinancingDetails,
    ///53 - Public Record Financing Summary
    PublicRecordFinancingSummary,
    ///54 - Public Record Claim Details
    PublicRecordClaimDetails,
    ///55 - Public Record Claim Summary
    PublicRecordClaimSummary,
    ///56 - Statement of Work
    StatementOfWork,
    ///57 - Legal Action Details
    LegalActionDetails,
    ///58 - Legal Action Summary
    LegalActionSummary,
    ///59 - Company Evaluation
    CompanyEvaluation,
    ///60 - Company Summary
    CompanySummary,
    ///61 - Credit Scores
    CreditScores,
    ///62 - Industry Averages
    IndustryAverages,
    ///63 - Referring Provider
    ReferringProvider,
    ///64 - Employee
    Employee,
    ///65 - Insurance Policy
    InsurancePolicy,
    ///66 - Vehicle
    Vehicle,
    ///67 - Key Contributor
    KeyContributor,
    ///68 - Public Record Summary
    PublicRecordSummary,
    ///69 - Delinquency Projections
    DelinquencyProjections,
    ///70 - Temporary Services Detail
    TemporaryServicesDetail,
    ///71 - Overnight Shipping Detail
    OvernightShippingDetail,
    ///72 - Medical Supply Detail
    MedicalSupplyDetail,
    ///73 - Equipment Leasing Detail
    EquipmentLeasingDetail,
    ///A - Assembly
    Assembly,
    ///AA - Insurer
    Insurer,
    ///AB - Claim Administrator
    ClaimAdministrator,
    ///AC - Insured
    Insured,
    ///AD - Administrative Information
    Administrative,
    ///AE - Car Rental Detail
    CarRentalDetail,
    ///AF - Lodging Detail
    LodgingDetail,
    ///AG - Agent
    Agent,
    ///AH - Transportation Detail
    TransportationDetail,
    ///AI - Purchase Card Detail
    PurchaseCardDetail,
    ///AJ - Alternate Taxing Authority
    AlternateTaxingAuthority,
    ///AL - Alternate Specification - Lift Level
    AlternateSpecificationLiftLevel,
    ///AM - Amount Information
    Amount,
    ///AP - Credential Action
    CredentialAction,
    ///AS - Animal Subject Group
    AnimalSubjectGroup,
    ///AT - Account
    Account,
    ///B - Buyer's Location
    BuyersLocation,
    ///BD - Building
    Building,
    ///BE - Business Entity
    BusinessEntity,
    ///BP - Body Part
    BodyPart,
    ///C - Date
    Date,
    ///CB - Contractholder Branch Office
    ContractholderBranchOffice,
    ///CC - Cost Center
    CostCenter,
    ///CE - Cost Element
    CostElement,
    ///CH - Contractholder
    Contractholder,
    ///CI - Cause of Injury
    CauseOfInjury,
    ///CL - Claimant
    Claimant,
    ///CN - Container
    Container,
    ///CO - Consortium
    Consortium,
    ///CP - Client or Party
    ClientOrParty,
    ///CT - Cost Type
    CostType,
    ///CV - Coverage, Rider, or Supplementary Benefit
    CodeCV,
    ///D - Product Description
    ProductDescription,
    ///DG - Drawing
    Drawing,
    ///DM - Damage
    Damage,
    ///DP - Department
    Department,
    ///DS - District
    District,
    ///E - Transportation Equipment
    TransportationEquipment,
    ///EB - Filer
    Filer,
    ///EC - Receipts
    Receipts,
    ///ED - Engineering Data List
    EngineeringDataList,
    ///EF - Expenditures
    Expenditures,
    ///EG - Receivables
    Receivables,
    ///EH - Payables
    Payables,
    ///EI - Organizational Information
    Organizational,
    ///EL - Exhibit Line Item
    ExhibitLineItem,
    ///EM - Employer
    Employer,
    ///EN - End Item
    EndItem,
    ///EV - Event
    Event,
    ///EX - Exception
    Exception,
    ///F - Component
    Component,
    ///FC - Function Code
    FunctionCode,
    ///FG - Functional Group
    FunctionalGroup,
    ///FI - Financial Information
    Financial,
    ///FL - Fleet
    Fleet,
    ///FR - Frame
    Frame,
    ///G - Quality Characteristics
    QualityCharacteristics,
    ///GC - Group Coverage Options
    GroupCoverageOptions,
    ///GP - Group Purchasing Organization
    GroupPurchasingOrganization,
    ///GW - Group Work Candidate
    GroupWorkCandidate,
    ///H - Bill of Materials
    BillOfMaterials,
    ///I - Item
    Item,
    ///IA - Subline Item
    SublineItem,
    ///IB - Contract
    Contract,
    ///IC - Contract Data Requirements List (CDRL)
    CodeIC,
    ///IN - Interchange
    Interchange,
    ///IS - Installments
    Installments,
    ///IT - Institution
    Institution,
    ///IV - Individual
    Individual,
    ///J - Part Characteristic
    PartCharacteristic,
    ///JU - Jurisdiction
    Jurisdiction,
    ///K - Kit
    Kit,
    ///KA - Accident History
    AccidentHistory,
    ///KB - Chemical
    Chemical,
    ///KC - Control Device
    ControlDevice,
    ///KD - Discharge
    Discharge,
    ///KE - Emergency Response Plan
    EmergencyResponsePlan,
    ///KF - Emission
    Emission,
    ///KG - Emission Activity
    EmissionActivity,
    ///KH - Emission Release Point
    EmissionReleasePoint,
    ///KI - Emission Unit
    EmissionUnit,
    ///KJ - Flammable Mixture
    FlammableMixture,
    ///KK - Flammables Alternate Release
    FlammablesAlternateRelease,
    ///KL - Flammables Worst Case
    FlammablesWorstCase,
    ///KM - Hazardous Waste Generation
    HazardousWasteGeneration,
    ///KN - Hazardous Waste Received
    HazardousWasteReceived,
    ///KO - Off-Site Process
    OffSiteProcess,
    ///KP - On-Site Process
    OnSiteProcess,
    ///KQ - Parameter
    Parameter,
    ///KR - Prevention Program
    PreventionProgram,
    ///KS - Process
    Process,
    ///KT - Reduction and Recycling
    ReductionAndRecycling,
    ///KV - Toxics Alternate Release
    ToxicsAlternateRelease,
    ///KW - Toxics Worst Case
    ToxicsWorstCase,
    ///KX - Transfer
    Transfer,
    ///L - Supplier's Location
    SuppliersLocation,
    ///LD - Lender or Mortgage Company
    LenderOrMortgageCompany,
    ///LN - Loan Data
    LoanData,
    ///LP - Party to the Loan
    PartyToTheLoan,
    ///M - Measurement
    Measurement,
    ///ML - Manufacturing Level
    ManufacturingLevel,
    ///N - Site of Service
    SiteOfService,
    ///NI - Nature of Injury
    NatureOfInjury,
    ///NS - National Stock Number
    NationalStockNumber,
    ///O - Order
    Order,
    ///OS - Support
    Support,
    ///P - Pack
    Pack,
    ///PA - Primary Administrator
    PrimaryAdministrator,
    ///PB - Personal Property
    PersonalProperty,
    ///PC - Project Code
    ProjectCode,
    ///PD - Procedure
    Procedure,
    ///PE - Person
    Person,
    ///PH - Product Characteristic
    ProductCharacteristic,
    ///PI - Property Identification
    PropertyIdentification,
    ///PK - Property Tax
    PropertyTax,
    ///PL - Primary Specification - Lift Level
    PrimarySpecificationLiftLevel,
    ///PP - Related Parties
    RelatedParties,
    ///PR - Principal
    Principal,
    ///PS - Property Segment Group
    PropertySegmentGroup,
    ///PT - Patient
    Patient,
    ///PY - Payment Detail
    PaymentDetail,
    ///Q - Subpack
    Subpack,
    ///R - Quantity
    Quantity,
    ///RA - Reporting Agency
    ReportingAgency,
    ///RB - Response
    Response,
    ///RC - Response Details
    ResponseDetails,
    ///RD - Response Sub-details
    ResponseSubDetails,
    ///RE - Response Particular
    ResponseParticular,
    ///RF - Medication
    Medication,
    ///RG - Recommendation
    Recommendation,
    ///RH - Review History
    ReviewHistory,
    ///RL - Reference Location
    ReferenceLocation,
    ///RM - Room
    Room,
    ///RP - Report
    Report,
    ///S - Shipment
    Shipment,
    ///S1 - Site
    Site,
    ///S2 - Sample
    Sample,
    ///S3 - Test
    Test,
    ///SA - Secondary Administrator
    SecondaryAdministrator,
    ///SB - Substitute
    Substitute,
    ///SC - Subcontract Line Item
    SubcontractLineItem,
    ///SD - Support Document
    SupportDocument,
    ///SE - Subexhibit Line Item
    SubexhibitLineItem,
    ///SF - Safety Fitness
    SafetyFitness,
    ///SG - Safety Factor
    SafetyFactor,
    ///SH - Sheet
    Sheet,
    ///SI - Source of Injury
    SourceOfInjury,
    ///SL - Solicitation
    Solicitation,
    ///SP - Sub-Project
    SubProject,
    ///SR - Subroom
    Subroom,
    ///SS - Services
    Services,
    ///ST - State
    State,
    ///SY - System
    System,
    ///T - Shipping Tare
    ShippingTare,
    ///TA - Taxing Authority
    TaxingAuthority,
    ///TD - Tax Delinquency
    TaxDelinquency,
    ///TI - Technical Information Package
    TechnicalInformationPackage,
    ///TS - Transaction Set
    TransactionSet,
    ///TU - Traffic Unit
    TrafficUnit,
    ///TX - Tax Installment
    TaxInstallment,
    ///U - Subassembly
    Subassembly,
    ///UT - Unit or Lot
    UnitOrLot,
    ///V - Address Information
    Address,
    ///VI - Violation
    Violation,
    ///W - Transaction Reference Number
    TransactionReferenceNumber,
    ///WB - Work Breakdown Structure
    WorkBreakdownStructure,
    ///WC - Work Candidate
    WorkCandidate,
    ///WL - Well
    Well,
    ///WP - Well Completion
    WellCompletion,
    ///WR - Wellbore
    Wellbore,
    ///X - Serial Number
    SerialNumber,
    ///Y - Suffix
    Suffix,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl HierarchicalLevelCode {
    pub fn code(&self) -> &str {
        {
            use HierarchicalLevelCode::*;
            match self {
                Region => "0",
                ServiceBillingProvider => "1",
                BillingArrangement => "2",
                Branch => "2A",
                DirectAffiliate => "2B",
                Director => "2C",
                Headquarters => "2D",
                IndirectAffiliate => "2E",
                ManagementAntecedents => "2F",
                ManagementOrPrincipal => "2G",
                ParentCompany => "2H",
                Stockholder => "2I",
                Subsidiary => "2J",
                UltimateDomesticParentCompany => "2K",
                UltimateParentCompany => "2L",
                SubBillingArrangement => "3",
                Group => "4",
                Category => "5",
                SubCategory => "6",
                Type => "7",
                ChargeDetail => "8",
                LineDetail => "9",
                ProviderOfService => "19",
                InformationSource => "20",
                InformationReceiver => "21",
                Subscriber => "22",
                Dependent => "23",
                Supergroup => "24",
                Subgroup => "25",
                Member => "26",
                AncillaryFacilityOrDepartment => "27",
                Hospital => "28",
                Franchisor => "29",
                Franchisee => "30",
                FranchiseeAssociation => "31",
                Code32 => "32",
                Activity => "33",
                LocationRecord => "34",
                CompanyCorporation => "35",
                OperatingUnit => "36",
                Property => "37",
                Tradename => "38",
                Accountant => "39",
                FinancialInstitution => "40",
                ProductLevel => "41",
                ActivityDetails => "42",
                PaymentSummaryScore => "43",
                CorporateRegistrationFilings => "44",
                BankruptcyDetails => "45",
                CompanyHistory => "46",
                CompleteFinancialHistory => "47",
                BalanceSheet => "48",
                ComparativeFigures => "49",
                PaymentAnalysis => "50",
                SpecialNotification => "51",
                PublicRecordFinancingDetails => "52",
                PublicRecordFinancingSummary => "53",
                PublicRecordClaimDetails => "54",
                PublicRecordClaimSummary => "55",
                StatementOfWork => "56",
                LegalActionDetails => "57",
                LegalActionSummary => "58",
                CompanyEvaluation => "59",
                CompanySummary => "60",
                CreditScores => "61",
                IndustryAverages => "62",
                ReferringProvider => "63",
                Employee => "64",
                InsurancePolicy => "65",
                Vehicle => "66",
                KeyContributor => "67",
                PublicRecordSummary => "68",
                DelinquencyProjections => "69",
                TemporaryServicesDetail => "70",
                OvernightShippingDetail => "71",
                MedicalSupplyDetail => "72",
                EquipmentLeasingDetail => "73",
                Assembly => "A",
                Insurer => "AA",
                ClaimAdministrator => "AB",
                Insured => "AC",
                Administrative => "AD",
                CarRentalDetail => "AE",
                LodgingDetail => "AF",
                Agent => "AG",
                TransportationDetail => "AH",
                PurchaseCardDetail => "AI",
                AlternateTaxingAuthority => "AJ",
                AlternateSpecificationLiftLevel => "AL",
                Amount => "AM",
                CredentialAction => "AP",
                AnimalSubjectGroup => "AS",
                Account => "AT",
                BuyersLocation => "B",
                Building => "BD",
                BusinessEntity => "BE",
                BodyPart => "BP",
                Date => "C",
                ContractholderBranchOffice => "CB",
                CostCenter => "CC",
                CostElement => "CE",
                Contractholder => "CH",
                CauseOfInjury => "CI",
                Claimant => "CL",
                Container => "CN",
                Consortium => "CO",
                ClientOrParty => "CP",
                CostType => "CT",
                CodeCV => "CV",
                ProductDescription => "D",
                Drawing => "DG",
                Damage => "DM",
                Department => "DP",
                District => "DS",
                TransportationEquipment => "E",
                Filer => "EB",
                Receipts => "EC",
                EngineeringDataList => "ED",
                Expenditures => "EF",
                Receivables => "EG",
                Payables => "EH",
                Organizational => "EI",
                ExhibitLineItem => "EL",
                Employer => "EM",
                EndItem => "EN",
                Event => "EV",
                Exception => "EX",
                Component => "F",
                FunctionCode => "FC",
                FunctionalGroup => "FG",
                Financial => "FI",
                Fleet => "FL",
                Frame => "FR",
                QualityCharacteristics => "G",
                GroupCoverageOptions => "GC",
                GroupPurchasingOrganization => "GP",
                GroupWorkCandidate => "GW",
                BillOfMaterials => "H",
                Item => "I",
                SublineItem => "IA",
                Contract => "IB",
                CodeIC => "IC",
                Interchange => "IN",
                Installments => "IS",
                Institution => "IT",
                Individual => "IV",
                PartCharacteristic => "J",
                Jurisdiction => "JU",
                Kit => "K",
                AccidentHistory => "KA",
                Chemical => "KB",
                ControlDevice => "KC",
                Discharge => "KD",
                EmergencyResponsePlan => "KE",
                Emission => "KF",
                EmissionActivity => "KG",
                EmissionReleasePoint => "KH",
                EmissionUnit => "KI",
                FlammableMixture => "KJ",
                FlammablesAlternateRelease => "KK",
                FlammablesWorstCase => "KL",
                HazardousWasteGeneration => "KM",
                HazardousWasteReceived => "KN",
                OffSiteProcess => "KO",
                OnSiteProcess => "KP",
                Parameter => "KQ",
                PreventionProgram => "KR",
                Process => "KS",
                ReductionAndRecycling => "KT",
                ToxicsAlternateRelease => "KV",
                ToxicsWorstCase => "KW",
                Transfer => "KX",
                SuppliersLocation => "L",
                LenderOrMortgageCompany => "LD",
                LoanData => "LN",
                PartyToTheLoan => "LP",
                Measurement => "M",
                ManufacturingLevel => "ML",
                SiteOfService => "N",
                NatureOfInjury => "NI",
                NationalStockNumber => "NS",
                Order => "O",
                Support => "OS",
                Pack => "P",
                PrimaryAdministrator => "PA",
                PersonalProperty => "PB",
                ProjectCode => "PC",
                Procedure => "PD",
                Person => "PE",
                ProductCharacteristic => "PH",
                PropertyIdentification => "PI",
                PropertyTax => "PK",
                PrimarySpecificationLiftLevel => "PL",
                RelatedParties => "PP",
                Principal => "PR",
                PropertySegmentGroup => "PS",
                Patient => "PT",
                PaymentDetail => "PY",
                Subpack => "Q",
                Quantity => "R",
                ReportingAgency => "RA",
                Response => "RB",
                ResponseDetails => "RC",
                ResponseSubDetails => "RD",
                ResponseParticular => "RE",
                Medication => "RF",
                Recommendation => "RG",
                ReviewHistory => "RH",
                ReferenceLocation => "RL",
                Room => "RM",
                Report => "RP",
                Shipment => "S",
                Site => "S1",
                Sample => "S2",
                Test => "S3",
                SecondaryAdministrator => "SA",
                Substitute => "SB",
                SubcontractLineItem => "SC",
                SupportDocument => "SD",
                SubexhibitLineItem => "SE",
                SafetyFitness => "SF",
                SafetyFactor => "SG",
                Sheet => "SH",
                SourceOfInjury => "SI",
                Solicitation => "SL",
                SubProject => "SP",
                Subroom => "SR",
                Services => "SS",
                State => "ST",
                System => "SY",
                ShippingTare => "T",
                TaxingAuthority => "TA",
                TaxDelinquency => "TD",
                TechnicalInformationPackage => "TI",
                TransactionSet => "TS",
                TrafficUnit => "TU",
                TaxInstallment => "TX",
                Subassembly => "U",
                UnitOrLot => "UT",
                Address => "V",
                Violation => "VI",
                TransactionReferenceNumber => "W",
                WorkBreakdownStructure => "WB",
                WorkCandidate => "WC",
                Well => "WL",
                WellCompletion => "WP",
                Wellbore => "WR",
                SerialNumber => "X",
                Suffix => "Y",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<HierarchicalLevelCode> {
        use HierarchicalLevelCode::*;
        match code {
            b"0" => Some(Region),
            b"1" => Some(ServiceBillingProvider),
            b"2" => Some(BillingArrangement),
            b"2A" => Some(Branch),
            b"2B" => Some(DirectAffiliate),
            b"2C" => Some(Director),
            b"2D" => Some(Headquarters),
            b"2E" => Some(IndirectAffiliate),
            b"2F" => Some(ManagementAntecedents),
            b"2G" => Some(ManagementOrPrincipal),
            b"2H" => Some(ParentCompany),
            b"2I" => Some(Stockholder),
            b"2J" => Some(Subsidiary),
            b"2K" => Some(UltimateDomesticParentCompany),
            b"2L" => Some(UltimateParentCompany),
            b"3" => Some(SubBillingArrangement),
            b"4" => Some(Group),
            b"5" => Some(Category),
            b"6" => Some(SubCategory),
            b"7" => Some(Type),
            b"8" => Some(ChargeDetail),
            b"9" => Some(LineDetail),
            b"19" => Some(ProviderOfService),
            b"20" => Some(InformationSource),
            b"21" => Some(InformationReceiver),
            b"22" => Some(Subscriber),
            b"23" => Some(Dependent),
            b"24" => Some(Supergroup),
            b"25" => Some(Subgroup),
            b"26" => Some(Member),
            b"27" => Some(AncillaryFacilityOrDepartment),
            b"28" => Some(Hospital),
            b"29" => Some(Franchisor),
            b"30" => Some(Franchisee),
            b"31" => Some(FranchiseeAssociation),
            b"32" => Some(Code32),
            b"33" => Some(Activity),
            b"34" => Some(LocationRecord),
            b"35" => Some(CompanyCorporation),
            b"36" => Some(OperatingUnit),
            b"37" => Some(Property),
            b"38" => Some(Tradename),
            b"39" => Some(Accountant),
            b"40" => Some(FinancialInstitution),
            b"41" => Some(ProductLevel),
            b"42" => Some(ActivityDetails),
            b"43" => Some(PaymentSummaryScore),
            b"44" => Some(CorporateRegistrationFilings),
            b"45" => Some(BankruptcyDetails),
            b"46" => Some(CompanyHistory),
            b"47" => Some(CompleteFinancialHistory),
            b"48" => Some(BalanceSheet),
            b"49" => Some(ComparativeFigures),
            b"50" => Some(PaymentAnalysis),
            b"51" => Some(SpecialNotification),
            b"52" => Some(PublicRecordFinancingDetails),
            b"53" => Some(PublicRecordFinancingSummary),
            b"54" => Some(PublicRecordClaimDetails),
            b"55" => Some(PublicRecordClaimSummary),
            b"56" => Some(StatementOfWork),
            b"57" => Some(LegalActionDetails),
            b"58" => Some(LegalActionSummary),
            b"59" => Some(CompanyEvaluation),
            b"60" => Some(CompanySummary),
            b"61" => Some(CreditScores),
            b"62" => Some(IndustryAverages),
            b"63" => Some(ReferringProvider),
            b"64" => Some(Employee),
            b"65" => Some(InsurancePolicy),
            b"66" => Some(Vehicle),
            b"67" => Some(KeyContributor),
            b"68" => Some(PublicRecordSummary),
            b"69" => Some(DelinquencyProjections),
            b"70" => Some(TemporaryServicesDetail),
            b"71" => Some(OvernightShippingDetail),
            b"72" => Some(MedicalSupplyDetail),
            b"73" => Some(EquipmentLeasingDetail),
            b"A" => Some(Assembly),
            b"AA" => Some(Insurer),
            b"AB" => Some(ClaimAdministrator),
            b"AC" => Some(Insured),
            b"AD" => Some(Administrative),
            b"AE" => Some(CarRentalDetail),
            b"AF" => Some(LodgingDetail),
            b"AG" => Some(Agent),
            b"AH" => Some(TransportationDetail),
            b"AI" => Some(PurchaseCardDetail),
            b"AJ" => Some(AlternateTaxingAuthority),
            b"AL" => Some(AlternateSpecificationLiftLevel),
            b"AM" => Some(Amount),
            b"AP" => Some(CredentialAction),
            b"AS" => Some(AnimalSubjectGroup),
            b"AT" => Some(Account),
            b"B" => Some(BuyersLocation),
            b"BD" => Some(Building),
            b"BE" => Some(BusinessEntity),
            b"BP" => Some(BodyPart),
            b"C" => Some(Date),
            b"CB" => Some(ContractholderBranchOffice),
            b"CC" => Some(CostCenter),
            b"CE" => Some(CostElement),
            b"CH" => Some(Contractholder),
            b"CI" => Some(CauseOfInjury),
            b"CL" => Some(Claimant),
            b"CN" => Some(Container),
            b"CO" => Some(Consortium),
            b"CP" => Some(ClientOrParty),
            b"CT" => Some(CostType),
            b"CV" => Some(CodeCV),
            b"D" => Some(ProductDescription),
            b"DG" => Some(Drawing),
            b"DM" => Some(Damage),
            b"DP" => Some(Department),
            b"DS" => Some(District),
            b"E" => Some(TransportationEquipment),
            b"EB" => Some(Filer),
            b"EC" => Some(Receipts),
            b"ED" => Some(EngineeringDataList),
            b"EF" => Some(Expenditures),
            b"EG" => Some(Receivables),
            b"EH" => Some(Payables),
            b"EI" => Some(Organizational),
            b"EL" => Some(ExhibitLineItem),
            b"EM" => Some(Employer),
            b"EN" => Some(EndItem),
            b"EV" => Some(Event),
            b"EX" => Some(Exception),
            b"F" => Some(Component),
            b"FC" => Some(FunctionCode),
            b"FG" => Some(FunctionalGroup),
            b"FI" => Some(Financial),
            b"FL" => Some(Fleet),
            b"FR" => Some(Frame),
            b"G" => Some(QualityCharacteristics),
            b"GC" => Some(GroupCoverageOptions),
            b"GP" => Some(GroupPurchasingOrganization),
            b"GW" => Some(GroupWorkCandidate),
            b"H" => Some(BillOfMaterials),
            b"I" => Some(Item),
            b"IA" => Some(SublineItem),
            b"IB" => Some(Contract),
            b"IC" => Some(CodeIC),
            b"IN" => Some(Interchange),
            b"IS" => Some(Installments),
            b"IT" => Some(Institution),
            b"IV" => Some(Individual),
            b"J" => Some(PartCharacteristic),
            b"JU" => Some(Jurisdiction),
            b"K" => Some(Kit),
            b"KA" => Some(AccidentHistory),
            b"KB" => Some(Chemical),
            b"KC" => Some(ControlDevice),
            b"KD" => Some(Discharge),
            b"KE" => Some(EmergencyResponsePlan),
            b"KF" => Some(Emission),
            b"KG" => Some(EmissionActivity),
            b"KH" => Some(EmissionReleasePoint),
            b"KI" => Some(EmissionUnit),
            b"KJ" => Some(FlammableMixture),
            b"KK" => Some(FlammablesAlternateRelease),
            b"KL" => Some(FlammablesWorstCase),
            b"KM" => Some(HazardousWasteGeneration),
            b"KN" => Some(HazardousWasteReceived),
            b"KO" => Some(OffSiteProcess),
            b"KP" => Some(OnSiteProcess),
            b"KQ" => Some(Parameter),
            b"KR" => Some(PreventionProgram),
            b"KS" => Some(Process),
            b"KT" => Some(ReductionAndRecycling),
            b"KV" => Some(ToxicsAlternateRelease),
            b"KW" => Some(ToxicsWorstCase),
            b"KX" => Some(Transfer),
            b"L" => Some(SuppliersLocation),
            b"LD" => Some(LenderOrMortgageCompany),
            b"LN" => Some(LoanData),
            b"LP" => Some(PartyToTheLoan),
            b"M" => Some(Measurement),
            b"ML" => Some(ManufacturingLevel),
            b"N" => Some(SiteOfService),
            b"NI" => Some(NatureOfInjury),
            b"NS" => Some(NationalStockNumber),
            b"O" => Some(Order),
            b"OS" => Some(Support),
            b"P" => Some(Pack),
            b"PA" => Some(PrimaryAdministrator),
            b"PB" => Some(PersonalProperty),
            b"PC" => Some(ProjectCode),
            b"PD" => Some(Procedure),
            b"PE" => Some(Person),
            b"PH" => Some(ProductCharacteristic),
            b"PI" => Some(PropertyIdentification),
            b"PK" => Some(PropertyTax),
            b"PL" => Some(PrimarySpecificationLiftLevel),
            b"PP" => Some(RelatedParties),
            b"PR" => Some(Principal),
            b"PS" => Some(PropertySegmentGroup),
            b"PT" => Some(Patient),
            b"PY" => Some(PaymentDetail),
            b"Q" => Some(Subpack),
            b"R" => Some(Quantity),
            b"RA" => Some(ReportingAgency),
            b"RB" => Some(Response),
            b"RC" => Some(ResponseDetails),
            b"RD" => Some(ResponseSubDetails),
            b"RE" => Some(ResponseParticular),
            b"RF" => Some(Medication),
            b"RG" => Some(Recommendation),
            b"RH" => Some(ReviewHistory),
            b"RL" => Some(ReferenceLocation),
            b"RM" => Some(Room),
            b"RP" => Some(Report),
            b"S" => Some(Shipment),
            b"S1" => Some(Site),
            b"S2" => Some(Sample),
            b"S3" => Some(Test),
            b"SA" => Some(SecondaryAdministrator),
            b"SB" => Some(Substitute),
            b"SC" => Some(SubcontractLineItem),
            b"SD" => Some(SupportDocument),
            b"SE" => Some(SubexhibitLineItem),
            b"SF" => Some(SafetyFitness),
            b"SG" => Some(SafetyFactor),
            b"SH" => Some(Sheet),
            b"SI" => Some(SourceOfInjury),
            b"SL" => Some(Solicitation),
            b"SP" => Some(SubProject),
            b"SR" => Some(Subroom),
            b"SS" => Some(Services),
            b"ST" => Some(State),
            b"SY" => Some(System),
            b"T" => Some(ShippingTare),
            b"TA" => Some(TaxingAuthority),
            b"TD" => Some(TaxDelinquency),
            b"TI" => Some(TechnicalInformationPackage),
            b"TS" => Some(TransactionSet),
            b"TU" => Some(TrafficUnit),
            b"TX" => Some(TaxInstallment),
            b"U" => Some(Subassembly),
            b"UT" => Some(UnitOrLot),
            b"V" => Some(Address),
            b"VI" => Some(Violation),
            b"W" => Some(TransactionReferenceNumber),
            b"WB" => Some(WorkBreakdownStructure),
            b"WC" => Some(WorkCandidate),
            b"WL" => Some(Well),
            b"WP" => Some(WellCompletion),
            b"WR" => Some(Wellbore),
            b"X" => Some(SerialNumber),
            b"Y" => Some(Suffix),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use HierarchicalLevelCode::*;
        match self {
            Region => "Region",
            ServiceBillingProvider => "Service/Billing Provider",
            BillingArrangement => "Billing Arrangement",
            Branch => "Branch",
            DirectAffiliate => "Direct Affiliate",
            Director => "Director",
            Headquarters => "Headquarters",
            IndirectAffiliate => "Indirect Affiliate",
            ManagementAntecedents => "Management Antecedents",
            ManagementOrPrincipal => "Management or Principal",
            ParentCompany => "Parent Company",
            Stockholder => "Stockholder",
            Subsidiary => "Subsidiary",
            UltimateDomesticParentCompany => "Ultimate Domestic Parent Company",
            UltimateParentCompany => "Ultimate Parent Company",
            SubBillingArrangement => "Sub-Billing Arrangement",
            Group => "Group",
            Category => "Category",
            SubCategory => "Sub-Category",
            Type => "Type",
            ChargeDetail => "Charge Detail",
            LineDetail => "Line Detail",
            ProviderOfService => "Provider of Service",
            InformationSource => "Information Source",
            InformationReceiver => "Information Receiver",
            Subscriber => "Subscriber",
            Dependent => "Dependent",
            Supergroup => "Supergroup",
            Subgroup => "Subgroup",
            Member => "Member",
            AncillaryFacilityOrDepartment => "Ancillary Facility or Department",
            Hospital => "Hospital",
            Franchisor => "Franchisor",
            Franchisee => "Franchisee",
            FranchiseeAssociation => "Franchisee Association",
            Code32 => {
                "Health Industry Business Communications Council (HIBCC) Health Industry Number (HIN) Database"
            }
            Activity => "Activity",
            LocationRecord => "Location Record",
            CompanyCorporation => "Company/Corporation",
            OperatingUnit => "Operating Unit",
            Property => "Property",
            Tradename => "Tradename",
            Accountant => "Accountant",
            FinancialInstitution => "Financial Institution",
            ProductLevel => "Product Level",
            ActivityDetails => "Activity Details",
            PaymentSummaryScore => "Payment Summary Score",
            CorporateRegistrationFilings => "Corporate Registration Filings",
            BankruptcyDetails => "Bankruptcy Details",
            CompanyHistory => "Company History",
            CompleteFinancialHistory => "Complete Financial History",
            BalanceSheet => "Balance Sheet",
            ComparativeFigures => "Comparative Figures",
            PaymentAnalysis => "Payment Analysis",
            SpecialNotification => "Special Notification",
            PublicRecordFinancingDetails => "Public Record Financing Details",
            PublicRecordFinancingSummary => "Public Record Financing Summary",
            PublicRecordClaimDetails => "Public Record Claim Details",
            PublicRecordClaimSummary => "Public Record Claim Summary",
            StatementOfWork => "Statement of Work",
            LegalActionDetails => "Legal Action Details",
            LegalActionSummary => "Legal Action Summary",
            CompanyEvaluation => "Company Evaluation",
            CompanySummary => "Company Summary",
            CreditScores => "Credit Scores",
            IndustryAverages => "Industry Averages",
            ReferringProvider => "Referring Provider",
            Employee => "Employee",
            InsurancePolicy => "Insurance Policy",
            Vehicle => "Vehicle",
            KeyContributor => "Key Contributor",
            PublicRecordSummary => "Public Record Summary",
            DelinquencyProjections => "Delinquency Projections",
            TemporaryServicesDetail => "Temporary Services Detail",
            OvernightShippingDetail => "Overnight Shipping Detail",
            MedicalSupplyDetail => "Medical Supply Detail",
            EquipmentLeasingDetail => "Equipment Leasing Detail",
            Assembly => "Assembly",
            Insurer => "Insurer",
            ClaimAdministrator => "Claim Administrator",
            Insured => "Insured",
            Administrative => "Administrative Information",
            CarRentalDetail => "Car Rental Detail",
            LodgingDetail => "Lodging Detail",
            Agent => "Agent",
            TransportationDetail => "Transportation Detail",
            PurchaseCardDetail => "Purchase Card Detail",
            AlternateTaxingAuthority => "Alternate Taxing Authority",
            AlternateSpecificationLiftLevel => "Alternate Specification - Lift Level",
            Amount => "Amount Information",
            CredentialAction => "Credential Action",
            AnimalSubjectGroup => "Animal Subject Group",
            Account => "Account",
            BuyersLocation => "Buyer's Location",
            Building => "Building",
            BusinessEntity => "Business Entity",
            BodyPart => "Body Part",
            Date => "Date",
            ContractholderBranchOffice => "Contractholder Branch Office",
            CostCenter => "Cost Center",
            CostElement => "Cost Element",
            Contractholder => "Contractholder",
            CauseOfInjury => "Cause of Injury",
            Claimant => "Claimant",
            Container => "Container",
            Consortium => "Consortium",
            ClientOrParty => "Client or Party",
            CostType => "Cost Type",
            CodeCV => "Coverage, Rider, or Supplementary Benefit",
            ProductDescription => "Product Description",
            Drawing => "Drawing",
            Damage => "Damage",
            Department => "Department",
            District => "District",
            TransportationEquipment => "Transportation Equipment",
            Filer => "Filer",
            Receipts => "Receipts",
            EngineeringDataList => "Engineering Data List",
            Expenditures => "Expenditures",
            Receivables => "Receivables",
            Payables => "Payables",
            Organizational => "Organizational Information",
            ExhibitLineItem => "Exhibit Line Item",
            Employer => "Employer",
            EndItem => "End Item",
            Event => "Event",
            Exception => "Exception",
            Component => "Component",
            FunctionCode => "Function Code",
            FunctionalGroup => "Functional Group",
            Financial => "Financial Information",
            Fleet => "Fleet",
            Frame => "Frame",
            QualityCharacteristics => "Quality Characteristics",
            GroupCoverageOptions => "Group Coverage Options",
            GroupPurchasingOrganization => "Group Purchasing Organization",
            GroupWorkCandidate => "Group Work Candidate",
            BillOfMaterials => "Bill of Materials",
            Item => "Item",
            SublineItem => "Subline Item",
            Contract => "Contract",
            CodeIC => "Contract Data Requirements List (CDRL)",
            Interchange => "Interchange",
            Installments => "Installments",
            Institution => "Institution",
            Individual => "Individual",
            PartCharacteristic => "Part Characteristic",
            Jurisdiction => "Jurisdiction",
            Kit => "Kit",
            AccidentHistory => "Accident History",
            Chemical => "Chemical",
            ControlDevice => "Control Device",
            Discharge => "Discharge",
            EmergencyResponsePlan => "Emergency Response Plan",
            Emission => "Emission",
            EmissionActivity => "Emission Activity",
            EmissionReleasePoint => "Emission Release Point",
            EmissionUnit => "Emission Unit",
            FlammableMixture => "Flammable Mixture",
            FlammablesAlternateRelease => "Flammables Alternate Release",
            FlammablesWorstCase => "Flammables Worst Case",
            HazardousWasteGeneration => "Hazardous Waste Generation",
            HazardousWasteReceived => "Hazardous Waste Received",
            OffSiteProcess => "Off-Site Process",
            OnSiteProcess => "On-Site Process",
            Parameter => "Parameter",
            PreventionProgram => "Prevention Program",
            Process => "Process",
            ReductionAndRecycling => "Reduction and Recycling",
            ToxicsAlternateRelease => "Toxics Alternate Release",
            ToxicsWorstCase => "Toxics Worst Case",
            Transfer => "Transfer",
            SuppliersLocation => "Supplier's Location",
            LenderOrMortgageCompany => "Lender or Mortgage Company",
            LoanData => "Loan Data",
            PartyToTheLoan => "Party to the Loan",
            Measurement => "Measurement",
            ManufacturingLevel => "Manufacturing Level",
            SiteOfService => "Site of Service",
            NatureOfInjury => "Nature of Injury",
            NationalStockNumber => "National Stock Number",
            Order => "Order",
            Support => "Support",
            Pack => "Pack",
            PrimaryAdministrator => "Primary Administrator",
            PersonalProperty => "Personal Property",
            ProjectCode => "Project Code",
            Procedure => "Procedure",
            Person => "Person",
            ProductCharacteristic => "Product Characteristic",
            PropertyIdentification => "Property Identification",
            PropertyTax => "Property Tax",
            PrimarySpecificationLiftLevel => "Primary Specification - Lift Level",
            RelatedParties => "Related Parties",
            Principal => "Principal",
            PropertySegmentGroup => "Property Segment Group",
            Patient => "Patient",
            PaymentDetail => "Payment Detail",
            Subpack => "Subpack",
            Quantity => "Quantity",
            ReportingAgency => "Reporting Agency",
            Response => "Response",
            ResponseDetails => "Response Details",
            ResponseSubDetails => "Response Sub-details",
            ResponseParticular => "Response Particular",
            Medication => "Medication",
            Recommendation => "Recommendation",
            ReviewHistory => "Review History",
            ReferenceLocation => "Reference Location",
            Room => "Room",
            Report => "Report",
            Shipment => "Shipment",
            Site => "Site",
            Sample => "Sample",
            Test => "Test",
            SecondaryAdministrator => "Secondary Administrator",
            Substitute => "Substitute",
            SubcontractLineItem => "Subcontract Line Item",
            SupportDocument => "Support Document",
            SubexhibitLineItem => "Subexhibit Line Item",
            SafetyFitness => "Safety Fitness",
            SafetyFactor => "Safety Factor",
            Sheet => "Sheet",
            SourceOfInjury => "Source of Injury",
            Solicitation => "Solicitation",
            SubProject => "Sub-Project",
            Subroom => "Subroom",
            Services => "Services",
            State => "State",
            System => "System",
            ShippingTare => "Shipping Tare",
            TaxingAuthority => "Taxing Authority",
            TaxDelinquency => "Tax Delinquency",
            TechnicalInformationPackage => "Technical Information Package",
            TransactionSet => "Transaction Set",
            TrafficUnit => "Traffic Unit",
            TaxInstallment => "Tax Installment",
            Subassembly => "Subassembly",
            UnitOrLot => "Unit or Lot",
            Address => "Address Information",
            Violation => "Violation",
            TransactionReferenceNumber => "Transaction Reference Number",
            WorkBreakdownStructure => "Work Breakdown Structure",
            WorkCandidate => "Work Candidate",
            Well => "Well",
            WellCompletion => "Well Completion",
            Wellbore => "Wellbore",
            SerialNumber => "Serial Number",
            Suffix => "Suffix",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<HierarchicalLevelCode> {
        {
            use HierarchicalLevelCode::*;
            match description {
                "Region" => Some(Region),
                "Service/Billing Provider" => Some(ServiceBillingProvider),
                "Billing Arrangement" => Some(BillingArrangement),
                "Branch" => Some(Branch),
                "Direct Affiliate" => Some(DirectAffiliate),
                "Director" => Some(Director),
                "Headquarters" => Some(Headquarters),
                "Indirect Affiliate" => Some(IndirectAffiliate),
                "Management Antecedents" => Some(ManagementAntecedents),
                "Management or Principal" => Some(ManagementOrPrincipal),
                "Parent Company" => Some(ParentCompany),
                "Stockholder" => Some(Stockholder),
                "Subsidiary" => Some(Subsidiary),
                "Ultimate Domestic Parent Company" => Some(UltimateDomesticParentCompany),
                "Ultimate Parent Company" => Some(UltimateParentCompany),
                "Sub-Billing Arrangement" => Some(SubBillingArrangement),
                "Group" => Some(Group),
                "Category" => Some(Category),
                "Sub-Category" => Some(SubCategory),
                "Type" => Some(Type),
                "Charge Detail" => Some(ChargeDetail),
                "Line Detail" => Some(LineDetail),
                "Provider of Service" => Some(ProviderOfService),
                "Information Source" => Some(InformationSource),
                "Information Receiver" => Some(InformationReceiver),
                "Subscriber" => Some(Subscriber),
                "Dependent" => Some(Dependent),
                "Supergroup" => Some(Supergroup),
                "Subgroup" => Some(Subgroup),
                "Member" => Some(Member),
                "Ancillary Facility or Department" => Some(AncillaryFacilityOrDepartment),
                "Hospital" => Some(Hospital),
                "Franchisor" => Some(Franchisor),
                "Franchisee" => Some(Franchisee),
                "Franchisee Association" => Some(FranchiseeAssociation),
                "Health Industry Business Communications Council (HIBCC) Health Industry Number (HIN) Database" => {
                    Some(Code32)
                }
                "Activity" => Some(Activity),
                "Location Record" => Some(LocationRecord),
                "Company/Corporation" => Some(CompanyCorporation),
                "Operating Unit" => Some(OperatingUnit),
                "Property" => Some(Property),
                "Tradename" => Some(Tradename),
                "Accountant" => Some(Accountant),
                "Financial Institution" => Some(FinancialInstitution),
                "Product Level" => Some(ProductLevel),
                "Activity Details" => Some(ActivityDetails),
                "Payment Summary Score" => Some(PaymentSummaryScore),
                "Corporate Registration Filings" => Some(CorporateRegistrationFilings),
                "Bankruptcy Details" => Some(BankruptcyDetails),
                "Company History" => Some(CompanyHistory),
                "Complete Financial History" => Some(CompleteFinancialHistory),
                "Balance Sheet" => Some(BalanceSheet),
                "Comparative Figures" => Some(ComparativeFigures),
                "Payment Analysis" => Some(PaymentAnalysis),
                "Special Notification" => Some(SpecialNotification),
                "Public Record Financing Details" => Some(PublicRecordFinancingDetails),
                "Public Record Financing Summary" => Some(PublicRecordFinancingSummary),
                "Public Record Claim Details" => Some(PublicRecordClaimDetails),
                "Public Record Claim Summary" => Some(PublicRecordClaimSummary),
                "Statement of Work" => Some(StatementOfWork),
                "Legal Action Details" => Some(LegalActionDetails),
                "Legal Action Summary" => Some(LegalActionSummary),
                "Company Evaluation" => Some(CompanyEvaluation),
                "Company Summary" => Some(CompanySummary),
                "Credit Scores" => Some(CreditScores),
                "Industry Averages" => Some(IndustryAverages),
                "Referring Provider" => Some(ReferringProvider),
                "Employee" => Some(Employee),
                "Insurance Policy" => Some(InsurancePolicy),
                "Vehicle" => Some(Vehicle),
                "Key Contributor" => Some(KeyContributor),
                "Public Record Summary" => Some(PublicRecordSummary),
                "Delinquency Projections" => Some(DelinquencyProjections),
                "Temporary Services Detail" => Some(TemporaryServicesDetail),
                "Overnight Shipping Detail" => Some(OvernightShippingDetail),
                "Medical Supply Detail" => Some(MedicalSupplyDetail),
                "Equipment Leasing Detail" => Some(EquipmentLeasingDetail),
                "Assembly" => Some(Assembly),
                "Insurer" => Some(Insurer),
                "Claim Administrator" => Some(ClaimAdministrator),
                "Insured" => Some(Insured),
                "Administrative Information" => Some(Administrative),
                "Car Rental Detail" => Some(CarRentalDetail),
                "Lodging Detail" => Some(LodgingDetail),
                "Agent" => Some(Agent),
                "Transportation Detail" => Some(TransportationDetail),
                "Purchase Card Detail" => Some(PurchaseCardDetail),
                "Alternate Taxing Authority" => Some(AlternateTaxingAuthority),
                "Alternate Specification - Lift Level" => {
                    Some(AlternateSpecificationLiftLevel)
                }
                "Amount Information" => Some(Amount),
                "Credential Action" => Some(CredentialAction),
                "Animal Subject Group" => Some(AnimalSubjectGroup),
                "Account" => Some(Account),
                "Buyer's Location" => Some(BuyersLocation),
                "Building" => Some(Building),
                "Business Entity" => Some(BusinessEntity),
                "Body Part" => Some(BodyPart),
                "Date" => Some(Date),
                "Contractholder Branch Office" => Some(ContractholderBranchOffice),
                "Cost Center" => Some(CostCenter),
                "Cost Element" => Some(CostElement),
                "Contractholder" => Some(Contractholder),
                "Cause of Injury" => Some(CauseOfInjury),
                "Claimant" => Some(Claimant),
                "Container" => Some(Container),
                "Consortium" => Some(Consortium),
                "Client or Party" => Some(ClientOrParty),
                "Cost Type" => Some(CostType),
                "Coverage, Rider, or Supplementary Benefit" => Some(CodeCV),
                "Product Description" => Some(ProductDescription),
                "Drawing" => Some(Drawing),
                "Damage" => Some(Damage),
                "Department" => Some(Department),
                "District" => Some(District),
                "Transportation Equipment" => Some(TransportationEquipment),
                "Filer" => Some(Filer),
                "Receipts" => Some(Receipts),
                "Engineering Data List" => Some(EngineeringDataList),
                "Expenditures" => Some(Expenditures),
                "Receivables" => Some(Receivables),
                "Payables" => Some(Payables),
                "Organizational Information" => Some(Organizational),
                "Exhibit Line Item" => Some(ExhibitLineItem),
                "Employer" => Some(Employer),
                "End Item" => Some(EndItem),
                "Event" => Some(Event),
                "Exception" => Some(Exception),
                "Component" => Some(Component),
                "Function Code" => Some(FunctionCode),
                "Functional Group" => Some(FunctionalGroup),
                "Financial Information" => Some(Financial),
                "Fleet" => Some(Fleet),
                "Frame" => Some(Frame),
                "Quality Characteristics" => Some(QualityCharacteristics),
                "Group Coverage Options" => Some(GroupCoverageOptions),
                "Group Purchasing Organization" => Some(GroupPurchasingOrganization),
                "Group Work Candidate" => Some(GroupWorkCandidate),
                "Bill of Materials" => Some(BillOfMaterials),
                "Item" => Some(Item),
                "Subline Item" => Some(SublineItem),
                "Contract" => Some(Contract),
                "Contract Data Requirements List (CDRL)" => Some(CodeIC),
                "Interchange" => Some(Interchange),
                "Installments" => Some(Installments),
                "Institution" => Some(Institution),
                "Individual" => Some(Individual),
                "Part Characteristic" => Some(PartCharacteristic),
                "Jurisdiction" => Some(Jurisdiction),
                "Kit" => Some(Kit),
                "Accident History" => Some(AccidentHistory),
                "Chemical" => Some(Chemical),
                "Control Device" => Some(ControlDevice),
                "Discharge" => Some(Discharge),
                "Emergency Response Plan" => Some(EmergencyResponsePlan),
                "Emission" => Some(Emission),
                "Emission Activity" => Some(EmissionActivity),
                "Emission Release Point" => Some(EmissionReleasePoint),
                "Emission Unit" => Some(EmissionUnit),
                "Flammable Mixture" => Some(FlammableMixture),
                "Flammables Alternate Release" => Some(FlammablesAlternateRelease),
                "Flammables Worst Case" => Some(FlammablesWorstCase),
                "Hazardous Waste Generation" => Some(HazardousWasteGeneration),
                "Hazardous Waste Received" => Some(HazardousWasteReceived),
                "Off-Site Process" => Some(OffSiteProcess),
                "On-Site Process" => Some(OnSiteProcess),
                "Parameter" => Some(Parameter),
                "Prevention Program" => Some(PreventionProgram),
                "Process" => Some(Process),
                "Reduction and Recycling" => Some(ReductionAndRecycling),
                "Toxics Alternate Release" => Some(ToxicsAlternateRelease),
                "Toxics Worst Case" => Some(ToxicsWorstCase),
                "Transfer" => Some(Transfer),
                "Supplier's Location" => Some(SuppliersLocation),
                "Lender or Mortgage Company" => Some(LenderOrMortgageCompany),
                "Loan Data" => Some(LoanData),
                "Party to the Loan" => Some(PartyToTheLoan),
                "Measurement" => Some(Measurement),
                "Manufacturing Level" => Some(ManufacturingLevel),
                "Site of Service" => Some(SiteOfService),
                "Nature of Injury" => Some(NatureOfInjury),
                "National Stock Number" => Some(NationalStockNumber),
                "Order" => Some(Order),
                "Support" => Some(Support),
                "Pack" => Some(Pack),
                "Primary Administrator" => Some(PrimaryAdministrator),
                "Personal Property" => Some(PersonalProperty),
                "Project Code" => Some(ProjectCode),
                "Procedure" => Some(Procedure),
                "Person" => Some(Person),
                "Product Characteristic" => Some(ProductCharacteristic),
                "Property Identification" => Some(PropertyIdentification),
                "Property Tax" => Some(PropertyTax),
                "Primary Specification - Lift Level" => {
                    Some(PrimarySpecificationLiftLevel)
                }
                "Related Parties" => Some(RelatedParties),
                "Principal" => Some(Principal),
                "Property Segment Group" => Some(PropertySegmentGroup),
                "Patient" => Some(Patient),
                "Payment Detail" => Some(PaymentDetail),
                "Subpack" => Some(Subpack),
                "Quantity" => Some(Quantity),
                "Reporting Agency" => Some(ReportingAgency),
                "Response" => Some(Response),
                "Response Details" => Some(ResponseDetails),
                "Response Sub-details" => Some(ResponseSubDetails),
                "Response Particular" => Some(ResponseParticular),
                "Medication" => Some(Medication),
                "Recommendation" => Some(Recommendation),
                "Review History" => Some(ReviewHistory),
                "Reference Location" => Some(ReferenceLocation),
                "Room" => Some(Room),
                "Report" => Some(Report),
                "Shipment" => Some(Shipment),
                "Site" => Some(Site),
                "Sample" => Some(Sample),
                "Test" => Some(Test),
                "Secondary Administrator" => Some(SecondaryAdministrator),
                "Substitute" => Some(Substitute),
                "Subcontract Line Item" => Some(SubcontractLineItem),
                "Support Document" => Some(SupportDocument),
                "Subexhibit Line Item" => Some(SubexhibitLineItem),
                "Safety Fitness" => Some(SafetyFitness),
                "Safety Factor" => Some(SafetyFactor),
                "Sheet" => Some(Sheet),
                "Source of Injury" => Some(SourceOfInjury),
                "Solicitation" => Some(Solicitation),
                "Sub-Project" => Some(SubProject),
                "Subroom" => Some(Subroom),
                "Services" => Some(Services),
                "State" => Some(State),
                "System" => Some(System),
                "Shipping Tare" => Some(ShippingTare),
                "Taxing Authority" => Some(TaxingAuthority),
                "Tax Delinquency" => Some(TaxDelinquency),
                "Technical Information Package" => Some(TechnicalInformationPackage),
                "Transaction Set" => Some(TransactionSet),
                "Traffic Unit" => Some(TrafficUnit),
                "Tax Installment" => Some(TaxInstallment),
                "Subassembly" => Some(Subassembly),
                "Unit or Lot" => Some(UnitOrLot),
                "Address Information" => Some(Address),
                "Violation" => Some(Violation),
                "Transaction Reference Number" => Some(TransactionReferenceNumber),
                "Work Breakdown Structure" => Some(WorkBreakdownStructure),
                "Work Candidate" => Some(WorkCandidate),
                "Well" => Some(Well),
                "Well Completion" => Some(WellCompletion),
                "Wellbore" => Some(Wellbore),
                "Serial Number" => Some(SerialNumber),
                "Suffix" => Some(Suffix),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for HierarchicalLevelCode {
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
    type Value = HierarchicalLevelCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Hierarchical Level Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        HierarchicalLevelCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Hierarchical Level Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        HierarchicalLevelCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Hierarchical Level Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for HierarchicalLevelCode {
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