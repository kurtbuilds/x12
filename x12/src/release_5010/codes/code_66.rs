use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**66

See docs at <https://www.stedi.com/edi/x12-005010/element/66>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IdentificationCodeQualifier {
    ///0 - Petroleum Industry Exchange (PETROEX) Number
    Code0,
    ///1 - D-U-N-S Number, Dun & Bradstreet
    Code1,
    ///2 - Standard Carrier Alpha Code (SCAC)
    Code2,
    ///3 - Federal Maritime Commission (Ocean) (FMC)
    Code3,
    ///4 - International Air Transport Association (IATA)
    Code4,
    ///6 - Plant Code
    PlantCode,
    ///7 - Loading Dock
    LoadingDock,
    ///9 - D-U-N-S+4, D-U-N-S Number with Four Character Suffix
    Code9,
    ///10 - Department of Defense Activity Address Code (DODAAC)
    Code10,
    ///11 - Drug Enforcement Administration (DEA)
    Code11,
    ///12 - Telephone Number (Phone)
    Code12,
    ///13 - Federal Reserve Routing Code (FRRC)
    Code13,
    ///15 - Standard Address Number (SAN)
    Code15,
    ///16 - ZIP Code
    ZipCode,
    ///17 - Automated Broker Interface (ABI) Routing Code
    Code17,
    ///19 - FIPS-55 (Named Populated Places)
    Code19,
    ///20 - Standard Point Location Code (SPLC)
    Code20,
    ///21 - Health Industry Number (HIN)
    Code21,
    ///22 - Council of Petroleum Accounting Societies code (COPAS)
    Code22,
    ///23 - Journal of Commerce (JOC)
    Code23,
    ///24 - Employer's Identification Number
    EmployersIdentificationNumber,
    ///25 - Carrier's Customer Code
    CarriersCustomerCode,
    ///26 - Petroleum Accountants Society of Canada Company Code
    PetroleumAccountantsSocietyOfCanadaCompanyCode,
    ///27 - Government Bill Of Lading Office Code (GBLOC)
    Code27,
    ///28 - American Paper Institute
    AmericanPaperInstitute,
    ///29 - Grid Location and Facility Code
    GridLocationAndFacilityCode,
    ///30 - American Petroleum Institute Location Code
    AmericanPetroleumInstituteLocationCode,
    ///31 - Bank Identification Code
    BankIdentificationCode,
    ///32 - Assigned by Property Operator
    AssignedByPropertyOperator,
    ///33 - Commercial and Government Entity (CAGE)
    Code33,
    ///34 - Social Security Number
    SocialSecurityNumber,
    ///35 - Electronic Mail Internal System Address Code
    ElectronicMailInternalSystemAddressCode,
    ///36 - Customs House Broker License Number
    CustomsHouseBrokerLicenseNumber,
    ///37 - United Nations Vendor Code
    UnitedNationsVendorCode,
    ///38 - Country Code
    CountryCode,
    ///39 - Local Union Number
    LocalUnionNumber,
    ///40 - Electronic Mail User Code
    ElectronicMailUserCode,
    ///41 - Telecommunications Carrier Identification Code
    TelecommunicationsCarrierIdentificationCode,
    ///42 - Telecommunications Pseudo Carrier Identification Code
    TelecommunicationsPseudoCarrierIdentificationCode,
    ///43 - Alternate Social Security Number
    AlternateSocialSecurityNumber,
    ///44 - Return Sequence Number
    ReturnSequenceNumber,
    ///45 - Declaration Control Number
    DeclarationControlNumber,
    ///46 - Electronic Transmitter Identification Number (ETIN)
    Code46,
    ///47 - Tax Authority Identification
    TaxAuthorityIdentification,
    ///48 - Electronic Filer Identification Number (EFIN)
    Code48,
    ///49 - State Identification Number
    StateIdentificationNumber,
    ///50 - Business License Number
    BusinessLicenseNumber,
    ///51 - Fuel Inventory Adjustment Identification
    FuelInventoryAdjustmentIdentification,
    ///53 - Building
    Building,
    ///54 - Warehouse
    Warehouse,
    ///55 - Post Office Box
    PostOfficeBox,
    ///56 - Division
    Division,
    ///57 - Department
    Department,
    ///58 - Originating Company Number
    OriginatingCompanyNumber,
    ///59 - Receiving Company Number
    ReceivingCompanyNumber,
    ///61 - Holding Mortgagee Number
    HoldingMortgageeNumber,
    ///62 - Servicing Mortgagee Number
    ServicingMortgageeNumber,
    ///63 - Servicer-holder Mortgagee Number
    ServicerHolderMortgageeNumber,
    ///64 - One Call Agency
    OneCallAgency,
    ///71 - Integrated Postsecondary Education Data System (IPEDS) set of codes maintained by the U.S. Department of Education's National Center of Education Statistics, Washington, D.C.
    Code71,
    ///72 - The College Board's Admission Testing Program (ATP), administered by the Educational Testing Service (ETS), 4-digit list of postsecondary educational institutions.
    Code72,
    ///73 - Federal Interagency Commission on Education (FICE) number. Available from the United States Department of Education, National Center for Education Statistics.
    Code73,
    ///74 - American College Testing (ACT) list of postsecondary educational institutions.
    Code74,
    ///75 - State or Province Assigned Number
    StateOrProvinceAssignedNumber,
    ///76 - Local School District or Jurisdiction Number
    LocalSchoolDistrictOrJurisdictionNumber,
    ///77 - National Center for Education Statistics (NCES) Common Core of Data (CCD) number for PreK - 12 institutions
    Code77,
    ///78 - The College Board and ACT 6 digit code list of secondary educational institutions
    TheCollegeBoardAndAct6DigitCodeListOfSecondaryEducationalInstitutions,
    ///81 - Classification of Instructional Programs (CIP) coding structure maintained by the U.S. Department of Education's National Center for Education Statistics
    Code81,
    /**82 - Higher Education General Information Survey (HEGIS) maintained by the U.S.
Department of Education's National Center for Education Statistics*/
    Code82,
    ///83 - Congressional District
    CongressionalDistrict,
    ///90 - California Ethnic Subgroups Code Table
    CaliforniaEthnicSubgroupsCodeTable,
    ///91 - Assigned by Seller or Seller's Agent
    AssignedBySellerOrSellersAgent,
    ///92 - Assigned by Buyer or Buyer's Agent
    AssignedByBuyerOrBuyersAgent,
    ///93 - Code assigned by the organization originating the transaction set
    CodeAssignedByTheOrganizationOriginatingTheTransactionSet,
    ///94 - Code assigned by the organization that is the ultimate destination of the transaction set
    CodeAssignedByTheOrganizationThatIsTheUltimateDestinationOfTheTransactionSet,
    ///95 - Assigned By Transporter
    AssignedByTransporter,
    ///96 - Assigned By Pipeline Operator
    AssignedByPipelineOperator,
    ///97 - Receiver's Code
    ReceiversCode,
    ///98 - Purchasing Office
    PurchasingOffice,
    ///99 - Office of Workers Compensation Programs (OWCP) Agency Code
    Code99,
    ///A - U.S. Customs Carrier Identification
    USCustomsCarrierIdentification,
    ///A1 - Approver ID
    ApproverId,
    ///A2 - Military Assistance Program Address Code (MAPAC)
    CodeA2,
    ///A3 - Assigned by Third Party
    AssignedByThirdParty,
    ///A4 - Assigned by Clearinghouse
    AssignedByClearinghouse,
    ///A5 - Committee on Uniform Security Identification Procedures (CUSIP) Number
    CodeA5,
    ///A6 - Financial Identification Numbering System (FINS) Number
    CodeA6,
    ///A7 - Automated Commercial Environment Identification Code (ACEID)
    CodeA7,
    ///AA - Postal Service Code
    PostalServiceCode,
    ///AB - US Environmental Protection Agency (EPA) Identification Number
    CodeAB,
    ///AC - Attachment Control Number
    AttachmentControlNumber,
    ///AD - Blue Cross Blue Shield Association Plan Code
    BlueCrossBlueShieldAssociationPlanCode,
    ///AE - Alberta Energy Resources Conservation Board
    AlbertaEnergyResourcesConservationBoard,
    ///AF - Rental Location Identifier
    RentalLocation,
    ///AI - Automotive Identifier for Canada Customs
    AutomotiveIdentifierForCanadaCustoms,
    ///AL - Anesthesia License Number
    AnesthesiaLicenseNumber,
    ///AP - Alberta Petroleum Marketing Commission
    AlbertaPetroleumMarketingCommission,
    ///BC - British Columbia Ministry of Energy Mines and Petroleum Resources
    BritishColumbiaMinistryOfEnergyMinesAndPetroleumResources,
    ///BD - Blue Cross Provider Number
    BlueCrossProviderNumber,
    ///BE - Common Language Location Identification (CLLI)
    CodeBE,
    ///BG - Badge Number
    BadgeNumber,
    ///BN - Canada Customs & Revenue Agency (CCRA) Business Number
    CodeBN,
    ///BP - Benefit Plan
    BenefitPlan,
    ///BS - Blue Shield Provider Number
    BlueShieldProviderNumber,
    ///C - Insured's Changed Unique Identification Number
    InsuredsChangedUniqueIdentificationNumber,
    ///C1 - Insured or Subscriber
    InsuredOrSubscriber,
    ///C2 - Health Maintenance Organization (HMO) Provider Number
    CodeC2,
    ///C5 - Customer Identification File
    CustomerIdentificationFile,
    ///CA - Statistics Canada Canadian College Student Information System Course Codes
    StatisticsCanadaCanadianCollegeStudentInformationSystemCourseCodes,
    ///CB - Statistics Canada Canadian College Student Information System Institution Codes
    StatisticsCanadaCanadianCollegeStudentInformationSystemInstitutionCodes,
    ///CC - Statistics Canada University Student Information System Curriculum Codes
    StatisticsCanadaUniversityStudentInformationSystemCurriculumCodes,
    ///CD - Contract Division
    ContractDivision,
    ///CE - Bureau of the Census Filer Identification Code
    BureauOfTheCensusFilerIdentificationCode,
    ///CF - Canadian Financial Institution Routing Number
    CanadianFinancialInstitutionRoutingNumber,
    ///CI - CHAMPUS (Civilian Health and Medical Program of the Uniformed Services) Identification Number
    CodeCI,
    ///CL - Corrected Loan Number
    CorrectedLoanNumber,
    ///CM - U.S. Customs Service (USCS) Manufacturer Identifier (MID)
    CodeCM,
    ///CN - National Center for Education Statistics (NCES) Course Classification System for Secondary Schools
    CodeCN,
    ///CP - Canadian Petroleum Association
    CanadianPetroleumAssociation,
    ///CR - Credit Repository
    CreditRepository,
    ///CS - Statistics Canada University Student Information System University Codes
    StatisticsCanadaUniversityStudentInformationSystemUniversityCodes,
    ///CT - Court Identification Code
    CourtIdentificationCode,
    ///D - Census Schedule D
    CensusScheduleD,
    ///DG - United States Department of Education Guarantor Identification Code
    UnitedStatesDepartmentOfEducationGuarantorIdentificationCode,
    ///DL - United States Department of Education Lender Identification Code
    UnitedStatesDepartmentOfEducationLenderIdentificationCode,
    ///DN - Dentist License Number
    DentistLicenseNumber,
    ///DO - Door
    Door,
    ///DP - Data Processing Point
    DataProcessingPoint,
    ///DR - Gas Industry Standards Board (GISB) Data Reference Number (DRN)
    CodeDR,
    ///DS - United States Department of Education School Identification Code
    UnitedStatesDepartmentOfEducationSchoolIdentificationCode,
    ///E - Hazard Insurance Policy Number
    HazardInsurancePolicyNumber,
    ///EC - ARI Electronic Commerce Location ID Code
    AriElectronicCommerceLocationIdCode,
    ///EH - Theatre Number
    TheatreNumber,
    ///EI - Employee Identification Number
    EmployeeIdentificationNumber,
    ///EL - Elevator
    Elevator,
    ///EP - U.S. Environmental Protection Agency (EPA)
    CodeEP,
    ///EQ - Insurance Company Assigned Identification Number
    InsuranceCompanyAssignedIdentificationNumber,
    ///ER - Mortgagee Assigned Identification Number
    MortgageeAssignedIdentificationNumber,
    ///ES - Automated Export System (AES) Filer Identification Code
    CodeES,
    ///ET - Educational Testing Service List of International Postsecondary Institutions
    EducationalTestingServiceListOfInternationalPostsecondaryInstitutions,
    ///F - Document Custodian Identification Number
    DocumentCustodianIdentificationNumber,
    ///FA - Facility Identification
    FacilityIdentification,
    ///FB - Field Code
    FieldCode,
    ///FC - Federal Court Jurisdiction Identifier
    FederalCourtJurisdiction,
    ///FD - Federal Court Divisional Office Number
    FederalCourtDivisionalOfficeNumber,
    ///FE - Facility Federal Identification Number
    FacilityFederalIdentificationNumber,
    ///FI - Federal Taxpayer's Identification Number
    FederalTaxpayersIdentificationNumber,
    ///FJ - Federal Jurisdiction
    FederalJurisdiction,
    ///FL - Floor
    Floor,
    ///FN - U.S. Environmental Protection Agency (EPA) Laboratory Certification Identification
    CodeFN,
    ///G - Payee Identification Number
    PayeeIdentificationNumber,
    ///GA - Primary Agent Identification
    PrimaryAgentIdentification,
    ///GC - GAS*CODE
    CodeGC,
    ///HC - Centers for Medicare and Medicaid Services
    CentersForMedicareAndMedicaidServices,
    ///HN - Health Insurance Claim (HIC) Number
    CodeHN,
    ///HS - House (Canadian Grain Elevator)
    CodeHS,
    ///I - Secondary Marketing Investor Assigned Number
    SecondaryMarketingInvestorAssignedNumber,
    ///ID - UCC EDI Communications ID (Comm ID)
    CodeID,
    ///II - Standard Unique Health Identifier for each Individual in the United States
    StandardUniqueHealthIdentifierForEachIndividualInTheUnitedStates,
    ///IP - U.S. Customs Carrier Initiative Program (CIP) Participant Identification Number
    CodeIP,
    ///J - Mortgage Electronic Registration System Organization Identifier
    MortgageElectronicRegistrationSystemOrganization,
    ///K - Census Schedule K
    CensusScheduleK,
    ///L - Investor Assigned Identification Number
    InvestorAssignedIdentificationNumber,
    ///LC - Agency Location Code (U.S. Government)
    CodeLC,
    ///LD - NISO Z39.53 Language Codes
    NisoZ3953LanguageCodes,
    ///LE - ISO 639 Language Codes
    Iso639LanguageCodes,
    ///LI - Labeler Identification Code (LIC)
    CodeLI,
    ///LN - Loan Number
    LoanNumber,
    ///M - Certificate Number
    CertificateNumber,
    ///M3 - Disbursing Station
    DisbursingStation,
    ///M4 - Department of Defense Routing Identifier Code (RIC)
    CodeM4,
    ///M5 - Jurisdiction Code
    JurisdictionCode,
    ///M6 - Division Office Code
    DivisionOfficeCode,
    ///MA - Mail Stop
    MailStop,
    ///MB - Medical Information Bureau
    MedicalInformationBureau,
    ///MC - Medicaid Provider Number
    MedicaidProviderNumber,
    ///MD - Manitoba Department of Mines and Resources
    ManitobaDepartmentOfMinesAndResources,
    ///MI - Member Identification Number
    MemberIdentificationNumber,
    ///MK - Market
    Market,
    ///ML - Multiple Listing Service Vendor - Multiple Listing Service Identification
    MultipleListingServiceVendorMultipleListingServiceIdentification,
    ///MN - Mortgage Identification Number
    MortgageIdentificationNumber,
    ///MO - Major Organizational Entity
    MajorOrganizationalEntity,
    ///MP - Medicare Provider Number
    MedicareProviderNumber,
    ///MR - Medicaid Recipient Identification Number
    MedicaidRecipientIdentificationNumber,
    ///N - Insured's Unique Identification Number
    InsuredsUniqueIdentificationNumber,
    ///NA - National Association of Realtors - Multiple Listing Service Identification
    NationalAssociationOfRealtorsMultipleListingServiceIdentification,
    ///ND - Mode Designator
    ModeDesignator,
    ///NI - National Association of Insurance Commissioners (NAIC) Identification
    CodeNI,
    ///NO - National Criminal Information Center Originating Agency
    NationalCriminalInformationCenterOriginatingAgency,
    ///NR - Non Resident Alien Registration Number
    NonResidentAlienRegistrationNumber,
    ///OC - Occupation Code
    OccupationCode,
    ///OP - On-line Payment and Collection
    OnLinePaymentAndCollection,
    ///PA - Secondary Agent Identification
    SecondaryAgentIdentification,
    ///PB - Public Identification
    PublicIdentification,
    ///PC - Provider Commercial Number
    ProviderCommercialNumber,
    ///PI - Payor Identification
    PayorIdentification,
    ///PP - Pharmacy Processor Number
    PharmacyProcessorNumber,
    ///PR - Pier
    Pier,
    ///RA - Regulatory Agency Number
    RegulatoryAgencyNumber,
    ///RB - Real Estate Agent
    RealEstateAgent,
    ///RC - Real Estate Company
    RealEstateCompany,
    ///RD - Real Estate Broker Identification
    RealEstateBrokerIdentification,
    ///RE - Real Estate License Number
    RealEstateLicenseNumber,
    ///RI - Office of Regulatory Information Systems (ORIS) Code
    CodeRI,
    ///RP - Ramp
    Ramp,
    ///RT - Railroad Track
    RailroadTrack,
    ///S - Title Insurance Policy Number
    TitleInsurancePolicyNumber,
    ///SA - Tertiary Agent Identification
    TertiaryAgentIdentification,
    ///SB - Social Insurance Number
    SocialInsuranceNumber,
    ///SD - Saskatchewan Department of Energy Mines and Resources
    SaskatchewanDepartmentOfEnergyMinesAndResources,
    ///SF - Suffix Code
    SuffixCode,
    ///SI - Standard Industry Code (SIC)
    CodeSI,
    ///SJ - State or Province Jurisdiction
    StateOrProvinceJurisdiction,
    ///SK - State/Provincial Lottery License Number
    StateProvincialLotteryLicenseNumber,
    ///SL - State License Number
    StateLicenseNumber,
    ///SP - Specialty License Number
    SpecialtyLicenseNumber,
    ///ST - State/Province License Tag
    StateProvinceLicenseTag,
    ///SV - Service Provider Number
    ServiceProviderNumber,
    ///SW - Society for Worldwide Interbank Financial Telecommunications (SWIFT) Address
    CodeSW,
    ///TA - Taxpayer ID Number
    TaxpayerIdNumber,
    ///TC - Internal Revenue Service Terminal Code
    InternalRevenueServiceTerminalCode,
    ///TL - Transport4 Location Code
    Transport4LocationCode,
    ///TS - Transport4 Shipper Code
    Transport4ShipperCode,
    ///TZ - Department Code
    DepartmentCode,
    ///UC - Consumer Credit Identification Number
    ConsumerCreditIdentificationNumber,
    ///UI - Unit Identification Code
    UnitIdentificationCode,
    ///UL - Global Location Number (GLN)
    CodeUL,
    ///UP - Unique Physician Identification Number (UPIN)
    CodeUP,
    ///UR - Uniform Resource Locator (URL)
    CodeUR,
    ///US - Unique Supplier Identification Number (USIN)
    CodeUS,
    ///UT - Unit
    Unit,
    ///WR - Wine Region Code
    WineRegionCode,
    ///WS - Education Language Codes
    EducationLanguageCodes,
    ///X1 - National Center for Education Statistics Unit Identification Number
    NationalCenterForEducationStatisticsUnitIdentificationNumber,
    ///XV - Centers for Medicare and Medicaid Services PlanID
    CentersForMedicareAndMedicaidServicesPlanId,
    ///XX - Centers for Medicare and Medicaid Services National Provider Identifier
    CentersForMedicareAndMedicaidServicesNationalProvider,
    ///XY - District Assigned Number
    DistrictAssignedNumber,
    ///ZC - Contractor Establishment Code
    ContractorEstablishmentCode,
    ///ZN - Zone
    Zone,
    ///ZY - Temporary Identification Number
    TemporaryIdentificationNumber,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl IdentificationCodeQualifier {
    pub fn code(&self) -> &str {
        {
            use IdentificationCodeQualifier::*;
            match self {
                Code0 => "0",
                Code1 => "1",
                Code2 => "2",
                Code3 => "3",
                Code4 => "4",
                PlantCode => "6",
                LoadingDock => "7",
                Code9 => "9",
                Code10 => "10",
                Code11 => "11",
                Code12 => "12",
                Code13 => "13",
                Code15 => "15",
                ZipCode => "16",
                Code17 => "17",
                Code19 => "19",
                Code20 => "20",
                Code21 => "21",
                Code22 => "22",
                Code23 => "23",
                EmployersIdentificationNumber => "24",
                CarriersCustomerCode => "25",
                PetroleumAccountantsSocietyOfCanadaCompanyCode => "26",
                Code27 => "27",
                AmericanPaperInstitute => "28",
                GridLocationAndFacilityCode => "29",
                AmericanPetroleumInstituteLocationCode => "30",
                BankIdentificationCode => "31",
                AssignedByPropertyOperator => "32",
                Code33 => "33",
                SocialSecurityNumber => "34",
                ElectronicMailInternalSystemAddressCode => "35",
                CustomsHouseBrokerLicenseNumber => "36",
                UnitedNationsVendorCode => "37",
                CountryCode => "38",
                LocalUnionNumber => "39",
                ElectronicMailUserCode => "40",
                TelecommunicationsCarrierIdentificationCode => "41",
                TelecommunicationsPseudoCarrierIdentificationCode => "42",
                AlternateSocialSecurityNumber => "43",
                ReturnSequenceNumber => "44",
                DeclarationControlNumber => "45",
                Code46 => "46",
                TaxAuthorityIdentification => "47",
                Code48 => "48",
                StateIdentificationNumber => "49",
                BusinessLicenseNumber => "50",
                FuelInventoryAdjustmentIdentification => "51",
                Building => "53",
                Warehouse => "54",
                PostOfficeBox => "55",
                Division => "56",
                Department => "57",
                OriginatingCompanyNumber => "58",
                ReceivingCompanyNumber => "59",
                HoldingMortgageeNumber => "61",
                ServicingMortgageeNumber => "62",
                ServicerHolderMortgageeNumber => "63",
                OneCallAgency => "64",
                Code71 => "71",
                Code72 => "72",
                Code73 => "73",
                Code74 => "74",
                StateOrProvinceAssignedNumber => "75",
                LocalSchoolDistrictOrJurisdictionNumber => "76",
                Code77 => "77",
                TheCollegeBoardAndAct6DigitCodeListOfSecondaryEducationalInstitutions => {
                    "78"
                }
                Code81 => "81",
                Code82 => "82",
                CongressionalDistrict => "83",
                CaliforniaEthnicSubgroupsCodeTable => "90",
                AssignedBySellerOrSellersAgent => "91",
                AssignedByBuyerOrBuyersAgent => "92",
                CodeAssignedByTheOrganizationOriginatingTheTransactionSet => "93",
                CodeAssignedByTheOrganizationThatIsTheUltimateDestinationOfTheTransactionSet => {
                    "94"
                }
                AssignedByTransporter => "95",
                AssignedByPipelineOperator => "96",
                ReceiversCode => "97",
                PurchasingOffice => "98",
                Code99 => "99",
                USCustomsCarrierIdentification => "A",
                ApproverId => "A1",
                CodeA2 => "A2",
                AssignedByThirdParty => "A3",
                AssignedByClearinghouse => "A4",
                CodeA5 => "A5",
                CodeA6 => "A6",
                CodeA7 => "A7",
                PostalServiceCode => "AA",
                CodeAB => "AB",
                AttachmentControlNumber => "AC",
                BlueCrossBlueShieldAssociationPlanCode => "AD",
                AlbertaEnergyResourcesConservationBoard => "AE",
                RentalLocation => "AF",
                AutomotiveIdentifierForCanadaCustoms => "AI",
                AnesthesiaLicenseNumber => "AL",
                AlbertaPetroleumMarketingCommission => "AP",
                BritishColumbiaMinistryOfEnergyMinesAndPetroleumResources => "BC",
                BlueCrossProviderNumber => "BD",
                CodeBE => "BE",
                BadgeNumber => "BG",
                CodeBN => "BN",
                BenefitPlan => "BP",
                BlueShieldProviderNumber => "BS",
                InsuredsChangedUniqueIdentificationNumber => "C",
                InsuredOrSubscriber => "C1",
                CodeC2 => "C2",
                CustomerIdentificationFile => "C5",
                StatisticsCanadaCanadianCollegeStudentInformationSystemCourseCodes => {
                    "CA"
                }
                StatisticsCanadaCanadianCollegeStudentInformationSystemInstitutionCodes => {
                    "CB"
                }
                StatisticsCanadaUniversityStudentInformationSystemCurriculumCodes => "CC",
                ContractDivision => "CD",
                BureauOfTheCensusFilerIdentificationCode => "CE",
                CanadianFinancialInstitutionRoutingNumber => "CF",
                CodeCI => "CI",
                CorrectedLoanNumber => "CL",
                CodeCM => "CM",
                CodeCN => "CN",
                CanadianPetroleumAssociation => "CP",
                CreditRepository => "CR",
                StatisticsCanadaUniversityStudentInformationSystemUniversityCodes => "CS",
                CourtIdentificationCode => "CT",
                CensusScheduleD => "D",
                UnitedStatesDepartmentOfEducationGuarantorIdentificationCode => "DG",
                UnitedStatesDepartmentOfEducationLenderIdentificationCode => "DL",
                DentistLicenseNumber => "DN",
                Door => "DO",
                DataProcessingPoint => "DP",
                CodeDR => "DR",
                UnitedStatesDepartmentOfEducationSchoolIdentificationCode => "DS",
                HazardInsurancePolicyNumber => "E",
                AriElectronicCommerceLocationIdCode => "EC",
                TheatreNumber => "EH",
                EmployeeIdentificationNumber => "EI",
                Elevator => "EL",
                CodeEP => "EP",
                InsuranceCompanyAssignedIdentificationNumber => "EQ",
                MortgageeAssignedIdentificationNumber => "ER",
                CodeES => "ES",
                EducationalTestingServiceListOfInternationalPostsecondaryInstitutions => {
                    "ET"
                }
                DocumentCustodianIdentificationNumber => "F",
                FacilityIdentification => "FA",
                FieldCode => "FB",
                FederalCourtJurisdiction => "FC",
                FederalCourtDivisionalOfficeNumber => "FD",
                FacilityFederalIdentificationNumber => "FE",
                FederalTaxpayersIdentificationNumber => "FI",
                FederalJurisdiction => "FJ",
                Floor => "FL",
                CodeFN => "FN",
                PayeeIdentificationNumber => "G",
                PrimaryAgentIdentification => "GA",
                CodeGC => "GC",
                CentersForMedicareAndMedicaidServices => "HC",
                CodeHN => "HN",
                CodeHS => "HS",
                SecondaryMarketingInvestorAssignedNumber => "I",
                CodeID => "ID",
                StandardUniqueHealthIdentifierForEachIndividualInTheUnitedStates => "II",
                CodeIP => "IP",
                MortgageElectronicRegistrationSystemOrganization => "J",
                CensusScheduleK => "K",
                InvestorAssignedIdentificationNumber => "L",
                CodeLC => "LC",
                NisoZ3953LanguageCodes => "LD",
                Iso639LanguageCodes => "LE",
                CodeLI => "LI",
                LoanNumber => "LN",
                CertificateNumber => "M",
                DisbursingStation => "M3",
                CodeM4 => "M4",
                JurisdictionCode => "M5",
                DivisionOfficeCode => "M6",
                MailStop => "MA",
                MedicalInformationBureau => "MB",
                MedicaidProviderNumber => "MC",
                ManitobaDepartmentOfMinesAndResources => "MD",
                MemberIdentificationNumber => "MI",
                Market => "MK",
                MultipleListingServiceVendorMultipleListingServiceIdentification => "ML",
                MortgageIdentificationNumber => "MN",
                MajorOrganizationalEntity => "MO",
                MedicareProviderNumber => "MP",
                MedicaidRecipientIdentificationNumber => "MR",
                InsuredsUniqueIdentificationNumber => "N",
                NationalAssociationOfRealtorsMultipleListingServiceIdentification => "NA",
                ModeDesignator => "ND",
                CodeNI => "NI",
                NationalCriminalInformationCenterOriginatingAgency => "NO",
                NonResidentAlienRegistrationNumber => "NR",
                OccupationCode => "OC",
                OnLinePaymentAndCollection => "OP",
                SecondaryAgentIdentification => "PA",
                PublicIdentification => "PB",
                ProviderCommercialNumber => "PC",
                PayorIdentification => "PI",
                PharmacyProcessorNumber => "PP",
                Pier => "PR",
                RegulatoryAgencyNumber => "RA",
                RealEstateAgent => "RB",
                RealEstateCompany => "RC",
                RealEstateBrokerIdentification => "RD",
                RealEstateLicenseNumber => "RE",
                CodeRI => "RI",
                Ramp => "RP",
                RailroadTrack => "RT",
                TitleInsurancePolicyNumber => "S",
                TertiaryAgentIdentification => "SA",
                SocialInsuranceNumber => "SB",
                SaskatchewanDepartmentOfEnergyMinesAndResources => "SD",
                SuffixCode => "SF",
                CodeSI => "SI",
                StateOrProvinceJurisdiction => "SJ",
                StateProvincialLotteryLicenseNumber => "SK",
                StateLicenseNumber => "SL",
                SpecialtyLicenseNumber => "SP",
                StateProvinceLicenseTag => "ST",
                ServiceProviderNumber => "SV",
                CodeSW => "SW",
                TaxpayerIdNumber => "TA",
                InternalRevenueServiceTerminalCode => "TC",
                Transport4LocationCode => "TL",
                Transport4ShipperCode => "TS",
                DepartmentCode => "TZ",
                ConsumerCreditIdentificationNumber => "UC",
                UnitIdentificationCode => "UI",
                CodeUL => "UL",
                CodeUP => "UP",
                CodeUR => "UR",
                CodeUS => "US",
                Unit => "UT",
                WineRegionCode => "WR",
                EducationLanguageCodes => "WS",
                NationalCenterForEducationStatisticsUnitIdentificationNumber => "X1",
                CentersForMedicareAndMedicaidServicesPlanId => "XV",
                CentersForMedicareAndMedicaidServicesNationalProvider => "XX",
                DistrictAssignedNumber => "XY",
                ContractorEstablishmentCode => "ZC",
                Zone => "ZN",
                TemporaryIdentificationNumber => "ZY",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<IdentificationCodeQualifier> {
        use IdentificationCodeQualifier::*;
        match code {
            b"0" => Some(Code0),
            b"1" => Some(Code1),
            b"2" => Some(Code2),
            b"3" => Some(Code3),
            b"4" => Some(Code4),
            b"6" => Some(PlantCode),
            b"7" => Some(LoadingDock),
            b"9" => Some(Code9),
            b"10" => Some(Code10),
            b"11" => Some(Code11),
            b"12" => Some(Code12),
            b"13" => Some(Code13),
            b"15" => Some(Code15),
            b"16" => Some(ZipCode),
            b"17" => Some(Code17),
            b"19" => Some(Code19),
            b"20" => Some(Code20),
            b"21" => Some(Code21),
            b"22" => Some(Code22),
            b"23" => Some(Code23),
            b"24" => Some(EmployersIdentificationNumber),
            b"25" => Some(CarriersCustomerCode),
            b"26" => Some(PetroleumAccountantsSocietyOfCanadaCompanyCode),
            b"27" => Some(Code27),
            b"28" => Some(AmericanPaperInstitute),
            b"29" => Some(GridLocationAndFacilityCode),
            b"30" => Some(AmericanPetroleumInstituteLocationCode),
            b"31" => Some(BankIdentificationCode),
            b"32" => Some(AssignedByPropertyOperator),
            b"33" => Some(Code33),
            b"34" => Some(SocialSecurityNumber),
            b"35" => Some(ElectronicMailInternalSystemAddressCode),
            b"36" => Some(CustomsHouseBrokerLicenseNumber),
            b"37" => Some(UnitedNationsVendorCode),
            b"38" => Some(CountryCode),
            b"39" => Some(LocalUnionNumber),
            b"40" => Some(ElectronicMailUserCode),
            b"41" => Some(TelecommunicationsCarrierIdentificationCode),
            b"42" => Some(TelecommunicationsPseudoCarrierIdentificationCode),
            b"43" => Some(AlternateSocialSecurityNumber),
            b"44" => Some(ReturnSequenceNumber),
            b"45" => Some(DeclarationControlNumber),
            b"46" => Some(Code46),
            b"47" => Some(TaxAuthorityIdentification),
            b"48" => Some(Code48),
            b"49" => Some(StateIdentificationNumber),
            b"50" => Some(BusinessLicenseNumber),
            b"51" => Some(FuelInventoryAdjustmentIdentification),
            b"53" => Some(Building),
            b"54" => Some(Warehouse),
            b"55" => Some(PostOfficeBox),
            b"56" => Some(Division),
            b"57" => Some(Department),
            b"58" => Some(OriginatingCompanyNumber),
            b"59" => Some(ReceivingCompanyNumber),
            b"61" => Some(HoldingMortgageeNumber),
            b"62" => Some(ServicingMortgageeNumber),
            b"63" => Some(ServicerHolderMortgageeNumber),
            b"64" => Some(OneCallAgency),
            b"71" => Some(Code71),
            b"72" => Some(Code72),
            b"73" => Some(Code73),
            b"74" => Some(Code74),
            b"75" => Some(StateOrProvinceAssignedNumber),
            b"76" => Some(LocalSchoolDistrictOrJurisdictionNumber),
            b"77" => Some(Code77),
            b"78" => {
                Some(
                    TheCollegeBoardAndAct6DigitCodeListOfSecondaryEducationalInstitutions,
                )
            }
            b"81" => Some(Code81),
            b"82" => Some(Code82),
            b"83" => Some(CongressionalDistrict),
            b"90" => Some(CaliforniaEthnicSubgroupsCodeTable),
            b"91" => Some(AssignedBySellerOrSellersAgent),
            b"92" => Some(AssignedByBuyerOrBuyersAgent),
            b"93" => Some(CodeAssignedByTheOrganizationOriginatingTheTransactionSet),
            b"94" => {
                Some(
                    CodeAssignedByTheOrganizationThatIsTheUltimateDestinationOfTheTransactionSet,
                )
            }
            b"95" => Some(AssignedByTransporter),
            b"96" => Some(AssignedByPipelineOperator),
            b"97" => Some(ReceiversCode),
            b"98" => Some(PurchasingOffice),
            b"99" => Some(Code99),
            b"A" => Some(USCustomsCarrierIdentification),
            b"A1" => Some(ApproverId),
            b"A2" => Some(CodeA2),
            b"A3" => Some(AssignedByThirdParty),
            b"A4" => Some(AssignedByClearinghouse),
            b"A5" => Some(CodeA5),
            b"A6" => Some(CodeA6),
            b"A7" => Some(CodeA7),
            b"AA" => Some(PostalServiceCode),
            b"AB" => Some(CodeAB),
            b"AC" => Some(AttachmentControlNumber),
            b"AD" => Some(BlueCrossBlueShieldAssociationPlanCode),
            b"AE" => Some(AlbertaEnergyResourcesConservationBoard),
            b"AF" => Some(RentalLocation),
            b"AI" => Some(AutomotiveIdentifierForCanadaCustoms),
            b"AL" => Some(AnesthesiaLicenseNumber),
            b"AP" => Some(AlbertaPetroleumMarketingCommission),
            b"BC" => Some(BritishColumbiaMinistryOfEnergyMinesAndPetroleumResources),
            b"BD" => Some(BlueCrossProviderNumber),
            b"BE" => Some(CodeBE),
            b"BG" => Some(BadgeNumber),
            b"BN" => Some(CodeBN),
            b"BP" => Some(BenefitPlan),
            b"BS" => Some(BlueShieldProviderNumber),
            b"C" => Some(InsuredsChangedUniqueIdentificationNumber),
            b"C1" => Some(InsuredOrSubscriber),
            b"C2" => Some(CodeC2),
            b"C5" => Some(CustomerIdentificationFile),
            b"CA" => {
                Some(StatisticsCanadaCanadianCollegeStudentInformationSystemCourseCodes)
            }
            b"CB" => {
                Some(
                    StatisticsCanadaCanadianCollegeStudentInformationSystemInstitutionCodes,
                )
            }
            b"CC" => {
                Some(StatisticsCanadaUniversityStudentInformationSystemCurriculumCodes)
            }
            b"CD" => Some(ContractDivision),
            b"CE" => Some(BureauOfTheCensusFilerIdentificationCode),
            b"CF" => Some(CanadianFinancialInstitutionRoutingNumber),
            b"CI" => Some(CodeCI),
            b"CL" => Some(CorrectedLoanNumber),
            b"CM" => Some(CodeCM),
            b"CN" => Some(CodeCN),
            b"CP" => Some(CanadianPetroleumAssociation),
            b"CR" => Some(CreditRepository),
            b"CS" => {
                Some(StatisticsCanadaUniversityStudentInformationSystemUniversityCodes)
            }
            b"CT" => Some(CourtIdentificationCode),
            b"D" => Some(CensusScheduleD),
            b"DG" => Some(UnitedStatesDepartmentOfEducationGuarantorIdentificationCode),
            b"DL" => Some(UnitedStatesDepartmentOfEducationLenderIdentificationCode),
            b"DN" => Some(DentistLicenseNumber),
            b"DO" => Some(Door),
            b"DP" => Some(DataProcessingPoint),
            b"DR" => Some(CodeDR),
            b"DS" => Some(UnitedStatesDepartmentOfEducationSchoolIdentificationCode),
            b"E" => Some(HazardInsurancePolicyNumber),
            b"EC" => Some(AriElectronicCommerceLocationIdCode),
            b"EH" => Some(TheatreNumber),
            b"EI" => Some(EmployeeIdentificationNumber),
            b"EL" => Some(Elevator),
            b"EP" => Some(CodeEP),
            b"EQ" => Some(InsuranceCompanyAssignedIdentificationNumber),
            b"ER" => Some(MortgageeAssignedIdentificationNumber),
            b"ES" => Some(CodeES),
            b"ET" => {
                Some(
                    EducationalTestingServiceListOfInternationalPostsecondaryInstitutions,
                )
            }
            b"F" => Some(DocumentCustodianIdentificationNumber),
            b"FA" => Some(FacilityIdentification),
            b"FB" => Some(FieldCode),
            b"FC" => Some(FederalCourtJurisdiction),
            b"FD" => Some(FederalCourtDivisionalOfficeNumber),
            b"FE" => Some(FacilityFederalIdentificationNumber),
            b"FI" => Some(FederalTaxpayersIdentificationNumber),
            b"FJ" => Some(FederalJurisdiction),
            b"FL" => Some(Floor),
            b"FN" => Some(CodeFN),
            b"G" => Some(PayeeIdentificationNumber),
            b"GA" => Some(PrimaryAgentIdentification),
            b"GC" => Some(CodeGC),
            b"HC" => Some(CentersForMedicareAndMedicaidServices),
            b"HN" => Some(CodeHN),
            b"HS" => Some(CodeHS),
            b"I" => Some(SecondaryMarketingInvestorAssignedNumber),
            b"ID" => Some(CodeID),
            b"II" => {
                Some(StandardUniqueHealthIdentifierForEachIndividualInTheUnitedStates)
            }
            b"IP" => Some(CodeIP),
            b"J" => Some(MortgageElectronicRegistrationSystemOrganization),
            b"K" => Some(CensusScheduleK),
            b"L" => Some(InvestorAssignedIdentificationNumber),
            b"LC" => Some(CodeLC),
            b"LD" => Some(NisoZ3953LanguageCodes),
            b"LE" => Some(Iso639LanguageCodes),
            b"LI" => Some(CodeLI),
            b"LN" => Some(LoanNumber),
            b"M" => Some(CertificateNumber),
            b"M3" => Some(DisbursingStation),
            b"M4" => Some(CodeM4),
            b"M5" => Some(JurisdictionCode),
            b"M6" => Some(DivisionOfficeCode),
            b"MA" => Some(MailStop),
            b"MB" => Some(MedicalInformationBureau),
            b"MC" => Some(MedicaidProviderNumber),
            b"MD" => Some(ManitobaDepartmentOfMinesAndResources),
            b"MI" => Some(MemberIdentificationNumber),
            b"MK" => Some(Market),
            b"ML" => {
                Some(MultipleListingServiceVendorMultipleListingServiceIdentification)
            }
            b"MN" => Some(MortgageIdentificationNumber),
            b"MO" => Some(MajorOrganizationalEntity),
            b"MP" => Some(MedicareProviderNumber),
            b"MR" => Some(MedicaidRecipientIdentificationNumber),
            b"N" => Some(InsuredsUniqueIdentificationNumber),
            b"NA" => {
                Some(NationalAssociationOfRealtorsMultipleListingServiceIdentification)
            }
            b"ND" => Some(ModeDesignator),
            b"NI" => Some(CodeNI),
            b"NO" => Some(NationalCriminalInformationCenterOriginatingAgency),
            b"NR" => Some(NonResidentAlienRegistrationNumber),
            b"OC" => Some(OccupationCode),
            b"OP" => Some(OnLinePaymentAndCollection),
            b"PA" => Some(SecondaryAgentIdentification),
            b"PB" => Some(PublicIdentification),
            b"PC" => Some(ProviderCommercialNumber),
            b"PI" => Some(PayorIdentification),
            b"PP" => Some(PharmacyProcessorNumber),
            b"PR" => Some(Pier),
            b"RA" => Some(RegulatoryAgencyNumber),
            b"RB" => Some(RealEstateAgent),
            b"RC" => Some(RealEstateCompany),
            b"RD" => Some(RealEstateBrokerIdentification),
            b"RE" => Some(RealEstateLicenseNumber),
            b"RI" => Some(CodeRI),
            b"RP" => Some(Ramp),
            b"RT" => Some(RailroadTrack),
            b"S" => Some(TitleInsurancePolicyNumber),
            b"SA" => Some(TertiaryAgentIdentification),
            b"SB" => Some(SocialInsuranceNumber),
            b"SD" => Some(SaskatchewanDepartmentOfEnergyMinesAndResources),
            b"SF" => Some(SuffixCode),
            b"SI" => Some(CodeSI),
            b"SJ" => Some(StateOrProvinceJurisdiction),
            b"SK" => Some(StateProvincialLotteryLicenseNumber),
            b"SL" => Some(StateLicenseNumber),
            b"SP" => Some(SpecialtyLicenseNumber),
            b"ST" => Some(StateProvinceLicenseTag),
            b"SV" => Some(ServiceProviderNumber),
            b"SW" => Some(CodeSW),
            b"TA" => Some(TaxpayerIdNumber),
            b"TC" => Some(InternalRevenueServiceTerminalCode),
            b"TL" => Some(Transport4LocationCode),
            b"TS" => Some(Transport4ShipperCode),
            b"TZ" => Some(DepartmentCode),
            b"UC" => Some(ConsumerCreditIdentificationNumber),
            b"UI" => Some(UnitIdentificationCode),
            b"UL" => Some(CodeUL),
            b"UP" => Some(CodeUP),
            b"UR" => Some(CodeUR),
            b"US" => Some(CodeUS),
            b"UT" => Some(Unit),
            b"WR" => Some(WineRegionCode),
            b"WS" => Some(EducationLanguageCodes),
            b"X1" => Some(NationalCenterForEducationStatisticsUnitIdentificationNumber),
            b"XV" => Some(CentersForMedicareAndMedicaidServicesPlanId),
            b"XX" => Some(CentersForMedicareAndMedicaidServicesNationalProvider),
            b"XY" => Some(DistrictAssignedNumber),
            b"ZC" => Some(ContractorEstablishmentCode),
            b"ZN" => Some(Zone),
            b"ZY" => Some(TemporaryIdentificationNumber),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use IdentificationCodeQualifier::*;
        match self {
            Code0 => "Petroleum Industry Exchange (PETROEX) Number",
            Code1 => "D-U-N-S Number, Dun & Bradstreet",
            Code2 => "Standard Carrier Alpha Code (SCAC)",
            Code3 => "Federal Maritime Commission (Ocean) (FMC)",
            Code4 => "International Air Transport Association (IATA)",
            PlantCode => "Plant Code",
            LoadingDock => "Loading Dock",
            Code9 => "D-U-N-S+4, D-U-N-S Number with Four Character Suffix",
            Code10 => "Department of Defense Activity Address Code (DODAAC)",
            Code11 => "Drug Enforcement Administration (DEA)",
            Code12 => "Telephone Number (Phone)",
            Code13 => "Federal Reserve Routing Code (FRRC)",
            Code15 => "Standard Address Number (SAN)",
            ZipCode => "ZIP Code",
            Code17 => "Automated Broker Interface (ABI) Routing Code",
            Code19 => "FIPS-55 (Named Populated Places)",
            Code20 => "Standard Point Location Code (SPLC)",
            Code21 => "Health Industry Number (HIN)",
            Code22 => "Council of Petroleum Accounting Societies code (COPAS)",
            Code23 => "Journal of Commerce (JOC)",
            EmployersIdentificationNumber => "Employer's Identification Number",
            CarriersCustomerCode => "Carrier's Customer Code",
            PetroleumAccountantsSocietyOfCanadaCompanyCode => {
                "Petroleum Accountants Society of Canada Company Code"
            }
            Code27 => "Government Bill Of Lading Office Code (GBLOC)",
            AmericanPaperInstitute => "American Paper Institute",
            GridLocationAndFacilityCode => "Grid Location and Facility Code",
            AmericanPetroleumInstituteLocationCode => {
                "American Petroleum Institute Location Code"
            }
            BankIdentificationCode => "Bank Identification Code",
            AssignedByPropertyOperator => "Assigned by Property Operator",
            Code33 => "Commercial and Government Entity (CAGE)",
            SocialSecurityNumber => "Social Security Number",
            ElectronicMailInternalSystemAddressCode => {
                "Electronic Mail Internal System Address Code"
            }
            CustomsHouseBrokerLicenseNumber => "Customs House Broker License Number",
            UnitedNationsVendorCode => "United Nations Vendor Code",
            CountryCode => "Country Code",
            LocalUnionNumber => "Local Union Number",
            ElectronicMailUserCode => "Electronic Mail User Code",
            TelecommunicationsCarrierIdentificationCode => {
                "Telecommunications Carrier Identification Code"
            }
            TelecommunicationsPseudoCarrierIdentificationCode => {
                "Telecommunications Pseudo Carrier Identification Code"
            }
            AlternateSocialSecurityNumber => "Alternate Social Security Number",
            ReturnSequenceNumber => "Return Sequence Number",
            DeclarationControlNumber => "Declaration Control Number",
            Code46 => "Electronic Transmitter Identification Number (ETIN)",
            TaxAuthorityIdentification => "Tax Authority Identification",
            Code48 => "Electronic Filer Identification Number (EFIN)",
            StateIdentificationNumber => "State Identification Number",
            BusinessLicenseNumber => "Business License Number",
            FuelInventoryAdjustmentIdentification => {
                "Fuel Inventory Adjustment Identification"
            }
            Building => "Building",
            Warehouse => "Warehouse",
            PostOfficeBox => "Post Office Box",
            Division => "Division",
            Department => "Department",
            OriginatingCompanyNumber => "Originating Company Number",
            ReceivingCompanyNumber => "Receiving Company Number",
            HoldingMortgageeNumber => "Holding Mortgagee Number",
            ServicingMortgageeNumber => "Servicing Mortgagee Number",
            ServicerHolderMortgageeNumber => "Servicer-holder Mortgagee Number",
            OneCallAgency => "One Call Agency",
            Code71 => {
                "Integrated Postsecondary Education Data System (IPEDS) set of codes maintained by the U.S. Department of Education's National Center of Education Statistics, Washington, D.C."
            }
            Code72 => {
                "The College Board's Admission Testing Program (ATP), administered by the Educational Testing Service (ETS), 4-digit list of postsecondary educational institutions."
            }
            Code73 => {
                "Federal Interagency Commission on Education (FICE) number. Available from the United States Department of Education, National Center for Education Statistics."
            }
            Code74 => {
                "American College Testing (ACT) list of postsecondary educational institutions."
            }
            StateOrProvinceAssignedNumber => "State or Province Assigned Number",
            LocalSchoolDistrictOrJurisdictionNumber => {
                "Local School District or Jurisdiction Number"
            }
            Code77 => {
                "National Center for Education Statistics (NCES) Common Core of Data (CCD) number for PreK - 12 institutions"
            }
            TheCollegeBoardAndAct6DigitCodeListOfSecondaryEducationalInstitutions => {
                "The College Board and ACT 6 digit code list of secondary educational institutions"
            }
            Code81 => {
                "Classification of Instructional Programs (CIP) coding structure maintained by the U.S. Department of Education's National Center for Education Statistics"
            }
            Code82 => {
                "Higher Education General Information Survey (HEGIS) maintained by the U.S.\nDepartment of Education's National Center for Education Statistics"
            }
            CongressionalDistrict => "Congressional District",
            CaliforniaEthnicSubgroupsCodeTable => {
                "California Ethnic Subgroups Code Table"
            }
            AssignedBySellerOrSellersAgent => "Assigned by Seller or Seller's Agent",
            AssignedByBuyerOrBuyersAgent => "Assigned by Buyer or Buyer's Agent",
            CodeAssignedByTheOrganizationOriginatingTheTransactionSet => {
                "Code assigned by the organization originating the transaction set"
            }
            CodeAssignedByTheOrganizationThatIsTheUltimateDestinationOfTheTransactionSet => {
                "Code assigned by the organization that is the ultimate destination of the transaction set"
            }
            AssignedByTransporter => "Assigned By Transporter",
            AssignedByPipelineOperator => "Assigned By Pipeline Operator",
            ReceiversCode => "Receiver's Code",
            PurchasingOffice => "Purchasing Office",
            Code99 => "Office of Workers Compensation Programs (OWCP) Agency Code",
            USCustomsCarrierIdentification => "U.S. Customs Carrier Identification",
            ApproverId => "Approver ID",
            CodeA2 => "Military Assistance Program Address Code (MAPAC)",
            AssignedByThirdParty => "Assigned by Third Party",
            AssignedByClearinghouse => "Assigned by Clearinghouse",
            CodeA5 => {
                "Committee on Uniform Security Identification Procedures (CUSIP) Number"
            }
            CodeA6 => "Financial Identification Numbering System (FINS) Number",
            CodeA7 => "Automated Commercial Environment Identification Code (ACEID)",
            PostalServiceCode => "Postal Service Code",
            CodeAB => "US Environmental Protection Agency (EPA) Identification Number",
            AttachmentControlNumber => "Attachment Control Number",
            BlueCrossBlueShieldAssociationPlanCode => {
                "Blue Cross Blue Shield Association Plan Code"
            }
            AlbertaEnergyResourcesConservationBoard => {
                "Alberta Energy Resources Conservation Board"
            }
            RentalLocation => "Rental Location Identifier",
            AutomotiveIdentifierForCanadaCustoms => {
                "Automotive Identifier for Canada Customs"
            }
            AnesthesiaLicenseNumber => "Anesthesia License Number",
            AlbertaPetroleumMarketingCommission => {
                "Alberta Petroleum Marketing Commission"
            }
            BritishColumbiaMinistryOfEnergyMinesAndPetroleumResources => {
                "British Columbia Ministry of Energy Mines and Petroleum Resources"
            }
            BlueCrossProviderNumber => "Blue Cross Provider Number",
            CodeBE => "Common Language Location Identification (CLLI)",
            BadgeNumber => "Badge Number",
            CodeBN => "Canada Customs & Revenue Agency (CCRA) Business Number",
            BenefitPlan => "Benefit Plan",
            BlueShieldProviderNumber => "Blue Shield Provider Number",
            InsuredsChangedUniqueIdentificationNumber => {
                "Insured's Changed Unique Identification Number"
            }
            InsuredOrSubscriber => "Insured or Subscriber",
            CodeC2 => "Health Maintenance Organization (HMO) Provider Number",
            CustomerIdentificationFile => "Customer Identification File",
            StatisticsCanadaCanadianCollegeStudentInformationSystemCourseCodes => {
                "Statistics Canada Canadian College Student Information System Course Codes"
            }
            StatisticsCanadaCanadianCollegeStudentInformationSystemInstitutionCodes => {
                "Statistics Canada Canadian College Student Information System Institution Codes"
            }
            StatisticsCanadaUniversityStudentInformationSystemCurriculumCodes => {
                "Statistics Canada University Student Information System Curriculum Codes"
            }
            ContractDivision => "Contract Division",
            BureauOfTheCensusFilerIdentificationCode => {
                "Bureau of the Census Filer Identification Code"
            }
            CanadianFinancialInstitutionRoutingNumber => {
                "Canadian Financial Institution Routing Number"
            }
            CodeCI => {
                "CHAMPUS (Civilian Health and Medical Program of the Uniformed Services) Identification Number"
            }
            CorrectedLoanNumber => "Corrected Loan Number",
            CodeCM => "U.S. Customs Service (USCS) Manufacturer Identifier (MID)",
            CodeCN => {
                "National Center for Education Statistics (NCES) Course Classification System for Secondary Schools"
            }
            CanadianPetroleumAssociation => "Canadian Petroleum Association",
            CreditRepository => "Credit Repository",
            StatisticsCanadaUniversityStudentInformationSystemUniversityCodes => {
                "Statistics Canada University Student Information System University Codes"
            }
            CourtIdentificationCode => "Court Identification Code",
            CensusScheduleD => "Census Schedule D",
            UnitedStatesDepartmentOfEducationGuarantorIdentificationCode => {
                "United States Department of Education Guarantor Identification Code"
            }
            UnitedStatesDepartmentOfEducationLenderIdentificationCode => {
                "United States Department of Education Lender Identification Code"
            }
            DentistLicenseNumber => "Dentist License Number",
            Door => "Door",
            DataProcessingPoint => "Data Processing Point",
            CodeDR => "Gas Industry Standards Board (GISB) Data Reference Number (DRN)",
            UnitedStatesDepartmentOfEducationSchoolIdentificationCode => {
                "United States Department of Education School Identification Code"
            }
            HazardInsurancePolicyNumber => "Hazard Insurance Policy Number",
            AriElectronicCommerceLocationIdCode => {
                "ARI Electronic Commerce Location ID Code"
            }
            TheatreNumber => "Theatre Number",
            EmployeeIdentificationNumber => "Employee Identification Number",
            Elevator => "Elevator",
            CodeEP => "U.S. Environmental Protection Agency (EPA)",
            InsuranceCompanyAssignedIdentificationNumber => {
                "Insurance Company Assigned Identification Number"
            }
            MortgageeAssignedIdentificationNumber => {
                "Mortgagee Assigned Identification Number"
            }
            CodeES => "Automated Export System (AES) Filer Identification Code",
            EducationalTestingServiceListOfInternationalPostsecondaryInstitutions => {
                "Educational Testing Service List of International Postsecondary Institutions"
            }
            DocumentCustodianIdentificationNumber => {
                "Document Custodian Identification Number"
            }
            FacilityIdentification => "Facility Identification",
            FieldCode => "Field Code",
            FederalCourtJurisdiction => "Federal Court Jurisdiction Identifier",
            FederalCourtDivisionalOfficeNumber => {
                "Federal Court Divisional Office Number"
            }
            FacilityFederalIdentificationNumber => {
                "Facility Federal Identification Number"
            }
            FederalTaxpayersIdentificationNumber => {
                "Federal Taxpayer's Identification Number"
            }
            FederalJurisdiction => "Federal Jurisdiction",
            Floor => "Floor",
            CodeFN => {
                "U.S. Environmental Protection Agency (EPA) Laboratory Certification Identification"
            }
            PayeeIdentificationNumber => "Payee Identification Number",
            PrimaryAgentIdentification => "Primary Agent Identification",
            CodeGC => "GAS*CODE",
            CentersForMedicareAndMedicaidServices => {
                "Centers for Medicare and Medicaid Services"
            }
            CodeHN => "Health Insurance Claim (HIC) Number",
            CodeHS => "House (Canadian Grain Elevator)",
            SecondaryMarketingInvestorAssignedNumber => {
                "Secondary Marketing Investor Assigned Number"
            }
            CodeID => "UCC EDI Communications ID (Comm ID)",
            StandardUniqueHealthIdentifierForEachIndividualInTheUnitedStates => {
                "Standard Unique Health Identifier for each Individual in the United States"
            }
            CodeIP => {
                "U.S. Customs Carrier Initiative Program (CIP) Participant Identification Number"
            }
            MortgageElectronicRegistrationSystemOrganization => {
                "Mortgage Electronic Registration System Organization Identifier"
            }
            CensusScheduleK => "Census Schedule K",
            InvestorAssignedIdentificationNumber => {
                "Investor Assigned Identification Number"
            }
            CodeLC => "Agency Location Code (U.S. Government)",
            NisoZ3953LanguageCodes => "NISO Z39.53 Language Codes",
            Iso639LanguageCodes => "ISO 639 Language Codes",
            CodeLI => "Labeler Identification Code (LIC)",
            LoanNumber => "Loan Number",
            CertificateNumber => "Certificate Number",
            DisbursingStation => "Disbursing Station",
            CodeM4 => "Department of Defense Routing Identifier Code (RIC)",
            JurisdictionCode => "Jurisdiction Code",
            DivisionOfficeCode => "Division Office Code",
            MailStop => "Mail Stop",
            MedicalInformationBureau => "Medical Information Bureau",
            MedicaidProviderNumber => "Medicaid Provider Number",
            ManitobaDepartmentOfMinesAndResources => {
                "Manitoba Department of Mines and Resources"
            }
            MemberIdentificationNumber => "Member Identification Number",
            Market => "Market",
            MultipleListingServiceVendorMultipleListingServiceIdentification => {
                "Multiple Listing Service Vendor - Multiple Listing Service Identification"
            }
            MortgageIdentificationNumber => "Mortgage Identification Number",
            MajorOrganizationalEntity => "Major Organizational Entity",
            MedicareProviderNumber => "Medicare Provider Number",
            MedicaidRecipientIdentificationNumber => {
                "Medicaid Recipient Identification Number"
            }
            InsuredsUniqueIdentificationNumber => {
                "Insured's Unique Identification Number"
            }
            NationalAssociationOfRealtorsMultipleListingServiceIdentification => {
                "National Association of Realtors - Multiple Listing Service Identification"
            }
            ModeDesignator => "Mode Designator",
            CodeNI => {
                "National Association of Insurance Commissioners (NAIC) Identification"
            }
            NationalCriminalInformationCenterOriginatingAgency => {
                "National Criminal Information Center Originating Agency"
            }
            NonResidentAlienRegistrationNumber => {
                "Non Resident Alien Registration Number"
            }
            OccupationCode => "Occupation Code",
            OnLinePaymentAndCollection => "On-line Payment and Collection",
            SecondaryAgentIdentification => "Secondary Agent Identification",
            PublicIdentification => "Public Identification",
            ProviderCommercialNumber => "Provider Commercial Number",
            PayorIdentification => "Payor Identification",
            PharmacyProcessorNumber => "Pharmacy Processor Number",
            Pier => "Pier",
            RegulatoryAgencyNumber => "Regulatory Agency Number",
            RealEstateAgent => "Real Estate Agent",
            RealEstateCompany => "Real Estate Company",
            RealEstateBrokerIdentification => "Real Estate Broker Identification",
            RealEstateLicenseNumber => "Real Estate License Number",
            CodeRI => "Office of Regulatory Information Systems (ORIS) Code",
            Ramp => "Ramp",
            RailroadTrack => "Railroad Track",
            TitleInsurancePolicyNumber => "Title Insurance Policy Number",
            TertiaryAgentIdentification => "Tertiary Agent Identification",
            SocialInsuranceNumber => "Social Insurance Number",
            SaskatchewanDepartmentOfEnergyMinesAndResources => {
                "Saskatchewan Department of Energy Mines and Resources"
            }
            SuffixCode => "Suffix Code",
            CodeSI => "Standard Industry Code (SIC)",
            StateOrProvinceJurisdiction => "State or Province Jurisdiction",
            StateProvincialLotteryLicenseNumber => {
                "State/Provincial Lottery License Number"
            }
            StateLicenseNumber => "State License Number",
            SpecialtyLicenseNumber => "Specialty License Number",
            StateProvinceLicenseTag => "State/Province License Tag",
            ServiceProviderNumber => "Service Provider Number",
            CodeSW => {
                "Society for Worldwide Interbank Financial Telecommunications (SWIFT) Address"
            }
            TaxpayerIdNumber => "Taxpayer ID Number",
            InternalRevenueServiceTerminalCode => {
                "Internal Revenue Service Terminal Code"
            }
            Transport4LocationCode => "Transport4 Location Code",
            Transport4ShipperCode => "Transport4 Shipper Code",
            DepartmentCode => "Department Code",
            ConsumerCreditIdentificationNumber => "Consumer Credit Identification Number",
            UnitIdentificationCode => "Unit Identification Code",
            CodeUL => "Global Location Number (GLN)",
            CodeUP => "Unique Physician Identification Number (UPIN)",
            CodeUR => "Uniform Resource Locator (URL)",
            CodeUS => "Unique Supplier Identification Number (USIN)",
            Unit => "Unit",
            WineRegionCode => "Wine Region Code",
            EducationLanguageCodes => "Education Language Codes",
            NationalCenterForEducationStatisticsUnitIdentificationNumber => {
                "National Center for Education Statistics Unit Identification Number"
            }
            CentersForMedicareAndMedicaidServicesPlanId => {
                "Centers for Medicare and Medicaid Services PlanID"
            }
            CentersForMedicareAndMedicaidServicesNationalProvider => {
                "Centers for Medicare and Medicaid Services National Provider Identifier"
            }
            DistrictAssignedNumber => "District Assigned Number",
            ContractorEstablishmentCode => "Contractor Establishment Code",
            Zone => "Zone",
            TemporaryIdentificationNumber => "Temporary Identification Number",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<IdentificationCodeQualifier> {
        {
            use IdentificationCodeQualifier::*;
            match description {
                "Petroleum Industry Exchange (PETROEX) Number" => Some(Code0),
                "D-U-N-S Number, Dun & Bradstreet" => Some(Code1),
                "Standard Carrier Alpha Code (SCAC)" => Some(Code2),
                "Federal Maritime Commission (Ocean) (FMC)" => Some(Code3),
                "International Air Transport Association (IATA)" => Some(Code4),
                "Plant Code" => Some(PlantCode),
                "Loading Dock" => Some(LoadingDock),
                "D-U-N-S+4, D-U-N-S Number with Four Character Suffix" => Some(Code9),
                "Department of Defense Activity Address Code (DODAAC)" => Some(Code10),
                "Drug Enforcement Administration (DEA)" => Some(Code11),
                "Telephone Number (Phone)" => Some(Code12),
                "Federal Reserve Routing Code (FRRC)" => Some(Code13),
                "Standard Address Number (SAN)" => Some(Code15),
                "ZIP Code" => Some(ZipCode),
                "Automated Broker Interface (ABI) Routing Code" => Some(Code17),
                "FIPS-55 (Named Populated Places)" => Some(Code19),
                "Standard Point Location Code (SPLC)" => Some(Code20),
                "Health Industry Number (HIN)" => Some(Code21),
                "Council of Petroleum Accounting Societies code (COPAS)" => Some(Code22),
                "Journal of Commerce (JOC)" => Some(Code23),
                "Employer's Identification Number" => Some(EmployersIdentificationNumber),
                "Carrier's Customer Code" => Some(CarriersCustomerCode),
                "Petroleum Accountants Society of Canada Company Code" => {
                    Some(PetroleumAccountantsSocietyOfCanadaCompanyCode)
                }
                "Government Bill Of Lading Office Code (GBLOC)" => Some(Code27),
                "American Paper Institute" => Some(AmericanPaperInstitute),
                "Grid Location and Facility Code" => Some(GridLocationAndFacilityCode),
                "American Petroleum Institute Location Code" => {
                    Some(AmericanPetroleumInstituteLocationCode)
                }
                "Bank Identification Code" => Some(BankIdentificationCode),
                "Assigned by Property Operator" => Some(AssignedByPropertyOperator),
                "Commercial and Government Entity (CAGE)" => Some(Code33),
                "Social Security Number" => Some(SocialSecurityNumber),
                "Electronic Mail Internal System Address Code" => {
                    Some(ElectronicMailInternalSystemAddressCode)
                }
                "Customs House Broker License Number" => {
                    Some(CustomsHouseBrokerLicenseNumber)
                }
                "United Nations Vendor Code" => Some(UnitedNationsVendorCode),
                "Country Code" => Some(CountryCode),
                "Local Union Number" => Some(LocalUnionNumber),
                "Electronic Mail User Code" => Some(ElectronicMailUserCode),
                "Telecommunications Carrier Identification Code" => {
                    Some(TelecommunicationsCarrierIdentificationCode)
                }
                "Telecommunications Pseudo Carrier Identification Code" => {
                    Some(TelecommunicationsPseudoCarrierIdentificationCode)
                }
                "Alternate Social Security Number" => Some(AlternateSocialSecurityNumber),
                "Return Sequence Number" => Some(ReturnSequenceNumber),
                "Declaration Control Number" => Some(DeclarationControlNumber),
                "Electronic Transmitter Identification Number (ETIN)" => Some(Code46),
                "Tax Authority Identification" => Some(TaxAuthorityIdentification),
                "Electronic Filer Identification Number (EFIN)" => Some(Code48),
                "State Identification Number" => Some(StateIdentificationNumber),
                "Business License Number" => Some(BusinessLicenseNumber),
                "Fuel Inventory Adjustment Identification" => {
                    Some(FuelInventoryAdjustmentIdentification)
                }
                "Building" => Some(Building),
                "Warehouse" => Some(Warehouse),
                "Post Office Box" => Some(PostOfficeBox),
                "Division" => Some(Division),
                "Department" => Some(Department),
                "Originating Company Number" => Some(OriginatingCompanyNumber),
                "Receiving Company Number" => Some(ReceivingCompanyNumber),
                "Holding Mortgagee Number" => Some(HoldingMortgageeNumber),
                "Servicing Mortgagee Number" => Some(ServicingMortgageeNumber),
                "Servicer-holder Mortgagee Number" => Some(ServicerHolderMortgageeNumber),
                "One Call Agency" => Some(OneCallAgency),
                "Integrated Postsecondary Education Data System (IPEDS) set of codes maintained by the U.S. Department of Education's National Center of Education Statistics, Washington, D.C." => {
                    Some(Code71)
                }
                "The College Board's Admission Testing Program (ATP), administered by the Educational Testing Service (ETS), 4-digit list of postsecondary educational institutions." => {
                    Some(Code72)
                }
                "Federal Interagency Commission on Education (FICE) number. Available from the United States Department of Education, National Center for Education Statistics." => {
                    Some(Code73)
                }
                "American College Testing (ACT) list of postsecondary educational institutions." => {
                    Some(Code74)
                }
                "State or Province Assigned Number" => {
                    Some(StateOrProvinceAssignedNumber)
                }
                "Local School District or Jurisdiction Number" => {
                    Some(LocalSchoolDistrictOrJurisdictionNumber)
                }
                "National Center for Education Statistics (NCES) Common Core of Data (CCD) number for PreK - 12 institutions" => {
                    Some(Code77)
                }
                "The College Board and ACT 6 digit code list of secondary educational institutions" => {
                    Some(
                        TheCollegeBoardAndAct6DigitCodeListOfSecondaryEducationalInstitutions,
                    )
                }
                "Classification of Instructional Programs (CIP) coding structure maintained by the U.S. Department of Education's National Center for Education Statistics" => {
                    Some(Code81)
                }
                "Higher Education General Information Survey (HEGIS) maintained by the U.S.\nDepartment of Education's National Center for Education Statistics" => {
                    Some(Code82)
                }
                "Congressional District" => Some(CongressionalDistrict),
                "California Ethnic Subgroups Code Table" => {
                    Some(CaliforniaEthnicSubgroupsCodeTable)
                }
                "Assigned by Seller or Seller's Agent" => {
                    Some(AssignedBySellerOrSellersAgent)
                }
                "Assigned by Buyer or Buyer's Agent" => {
                    Some(AssignedByBuyerOrBuyersAgent)
                }
                "Code assigned by the organization originating the transaction set" => {
                    Some(CodeAssignedByTheOrganizationOriginatingTheTransactionSet)
                }
                "Code assigned by the organization that is the ultimate destination of the transaction set" => {
                    Some(
                        CodeAssignedByTheOrganizationThatIsTheUltimateDestinationOfTheTransactionSet,
                    )
                }
                "Assigned By Transporter" => Some(AssignedByTransporter),
                "Assigned By Pipeline Operator" => Some(AssignedByPipelineOperator),
                "Receiver's Code" => Some(ReceiversCode),
                "Purchasing Office" => Some(PurchasingOffice),
                "Office of Workers Compensation Programs (OWCP) Agency Code" => {
                    Some(Code99)
                }
                "U.S. Customs Carrier Identification" => {
                    Some(USCustomsCarrierIdentification)
                }
                "Approver ID" => Some(ApproverId),
                "Military Assistance Program Address Code (MAPAC)" => Some(CodeA2),
                "Assigned by Third Party" => Some(AssignedByThirdParty),
                "Assigned by Clearinghouse" => Some(AssignedByClearinghouse),
                "Committee on Uniform Security Identification Procedures (CUSIP) Number" => {
                    Some(CodeA5)
                }
                "Financial Identification Numbering System (FINS) Number" => Some(CodeA6),
                "Automated Commercial Environment Identification Code (ACEID)" => {
                    Some(CodeA7)
                }
                "Postal Service Code" => Some(PostalServiceCode),
                "US Environmental Protection Agency (EPA) Identification Number" => {
                    Some(CodeAB)
                }
                "Attachment Control Number" => Some(AttachmentControlNumber),
                "Blue Cross Blue Shield Association Plan Code" => {
                    Some(BlueCrossBlueShieldAssociationPlanCode)
                }
                "Alberta Energy Resources Conservation Board" => {
                    Some(AlbertaEnergyResourcesConservationBoard)
                }
                "Rental Location Identifier" => Some(RentalLocation),
                "Automotive Identifier for Canada Customs" => {
                    Some(AutomotiveIdentifierForCanadaCustoms)
                }
                "Anesthesia License Number" => Some(AnesthesiaLicenseNumber),
                "Alberta Petroleum Marketing Commission" => {
                    Some(AlbertaPetroleumMarketingCommission)
                }
                "British Columbia Ministry of Energy Mines and Petroleum Resources" => {
                    Some(BritishColumbiaMinistryOfEnergyMinesAndPetroleumResources)
                }
                "Blue Cross Provider Number" => Some(BlueCrossProviderNumber),
                "Common Language Location Identification (CLLI)" => Some(CodeBE),
                "Badge Number" => Some(BadgeNumber),
                "Canada Customs & Revenue Agency (CCRA) Business Number" => Some(CodeBN),
                "Benefit Plan" => Some(BenefitPlan),
                "Blue Shield Provider Number" => Some(BlueShieldProviderNumber),
                "Insured's Changed Unique Identification Number" => {
                    Some(InsuredsChangedUniqueIdentificationNumber)
                }
                "Insured or Subscriber" => Some(InsuredOrSubscriber),
                "Health Maintenance Organization (HMO) Provider Number" => Some(CodeC2),
                "Customer Identification File" => Some(CustomerIdentificationFile),
                "Statistics Canada Canadian College Student Information System Course Codes" => {
                    Some(
                        StatisticsCanadaCanadianCollegeStudentInformationSystemCourseCodes,
                    )
                }
                "Statistics Canada Canadian College Student Information System Institution Codes" => {
                    Some(
                        StatisticsCanadaCanadianCollegeStudentInformationSystemInstitutionCodes,
                    )
                }
                "Statistics Canada University Student Information System Curriculum Codes" => {
                    Some(
                        StatisticsCanadaUniversityStudentInformationSystemCurriculumCodes,
                    )
                }
                "Contract Division" => Some(ContractDivision),
                "Bureau of the Census Filer Identification Code" => {
                    Some(BureauOfTheCensusFilerIdentificationCode)
                }
                "Canadian Financial Institution Routing Number" => {
                    Some(CanadianFinancialInstitutionRoutingNumber)
                }
                "CHAMPUS (Civilian Health and Medical Program of the Uniformed Services) Identification Number" => {
                    Some(CodeCI)
                }
                "Corrected Loan Number" => Some(CorrectedLoanNumber),
                "U.S. Customs Service (USCS) Manufacturer Identifier (MID)" => {
                    Some(CodeCM)
                }
                "National Center for Education Statistics (NCES) Course Classification System for Secondary Schools" => {
                    Some(CodeCN)
                }
                "Canadian Petroleum Association" => Some(CanadianPetroleumAssociation),
                "Credit Repository" => Some(CreditRepository),
                "Statistics Canada University Student Information System University Codes" => {
                    Some(
                        StatisticsCanadaUniversityStudentInformationSystemUniversityCodes,
                    )
                }
                "Court Identification Code" => Some(CourtIdentificationCode),
                "Census Schedule D" => Some(CensusScheduleD),
                "United States Department of Education Guarantor Identification Code" => {
                    Some(UnitedStatesDepartmentOfEducationGuarantorIdentificationCode)
                }
                "United States Department of Education Lender Identification Code" => {
                    Some(UnitedStatesDepartmentOfEducationLenderIdentificationCode)
                }
                "Dentist License Number" => Some(DentistLicenseNumber),
                "Door" => Some(Door),
                "Data Processing Point" => Some(DataProcessingPoint),
                "Gas Industry Standards Board (GISB) Data Reference Number (DRN)" => {
                    Some(CodeDR)
                }
                "United States Department of Education School Identification Code" => {
                    Some(UnitedStatesDepartmentOfEducationSchoolIdentificationCode)
                }
                "Hazard Insurance Policy Number" => Some(HazardInsurancePolicyNumber),
                "ARI Electronic Commerce Location ID Code" => {
                    Some(AriElectronicCommerceLocationIdCode)
                }
                "Theatre Number" => Some(TheatreNumber),
                "Employee Identification Number" => Some(EmployeeIdentificationNumber),
                "Elevator" => Some(Elevator),
                "U.S. Environmental Protection Agency (EPA)" => Some(CodeEP),
                "Insurance Company Assigned Identification Number" => {
                    Some(InsuranceCompanyAssignedIdentificationNumber)
                }
                "Mortgagee Assigned Identification Number" => {
                    Some(MortgageeAssignedIdentificationNumber)
                }
                "Automated Export System (AES) Filer Identification Code" => Some(CodeES),
                "Educational Testing Service List of International Postsecondary Institutions" => {
                    Some(
                        EducationalTestingServiceListOfInternationalPostsecondaryInstitutions,
                    )
                }
                "Document Custodian Identification Number" => {
                    Some(DocumentCustodianIdentificationNumber)
                }
                "Facility Identification" => Some(FacilityIdentification),
                "Field Code" => Some(FieldCode),
                "Federal Court Jurisdiction Identifier" => Some(FederalCourtJurisdiction),
                "Federal Court Divisional Office Number" => {
                    Some(FederalCourtDivisionalOfficeNumber)
                }
                "Facility Federal Identification Number" => {
                    Some(FacilityFederalIdentificationNumber)
                }
                "Federal Taxpayer's Identification Number" => {
                    Some(FederalTaxpayersIdentificationNumber)
                }
                "Federal Jurisdiction" => Some(FederalJurisdiction),
                "Floor" => Some(Floor),
                "U.S. Environmental Protection Agency (EPA) Laboratory Certification Identification" => {
                    Some(CodeFN)
                }
                "Payee Identification Number" => Some(PayeeIdentificationNumber),
                "Primary Agent Identification" => Some(PrimaryAgentIdentification),
                "GAS*CODE" => Some(CodeGC),
                "Centers for Medicare and Medicaid Services" => {
                    Some(CentersForMedicareAndMedicaidServices)
                }
                "Health Insurance Claim (HIC) Number" => Some(CodeHN),
                "House (Canadian Grain Elevator)" => Some(CodeHS),
                "Secondary Marketing Investor Assigned Number" => {
                    Some(SecondaryMarketingInvestorAssignedNumber)
                }
                "UCC EDI Communications ID (Comm ID)" => Some(CodeID),
                "Standard Unique Health Identifier for each Individual in the United States" => {
                    Some(
                        StandardUniqueHealthIdentifierForEachIndividualInTheUnitedStates,
                    )
                }
                "U.S. Customs Carrier Initiative Program (CIP) Participant Identification Number" => {
                    Some(CodeIP)
                }
                "Mortgage Electronic Registration System Organization Identifier" => {
                    Some(MortgageElectronicRegistrationSystemOrganization)
                }
                "Census Schedule K" => Some(CensusScheduleK),
                "Investor Assigned Identification Number" => {
                    Some(InvestorAssignedIdentificationNumber)
                }
                "Agency Location Code (U.S. Government)" => Some(CodeLC),
                "NISO Z39.53 Language Codes" => Some(NisoZ3953LanguageCodes),
                "ISO 639 Language Codes" => Some(Iso639LanguageCodes),
                "Labeler Identification Code (LIC)" => Some(CodeLI),
                "Loan Number" => Some(LoanNumber),
                "Certificate Number" => Some(CertificateNumber),
                "Disbursing Station" => Some(DisbursingStation),
                "Department of Defense Routing Identifier Code (RIC)" => Some(CodeM4),
                "Jurisdiction Code" => Some(JurisdictionCode),
                "Division Office Code" => Some(DivisionOfficeCode),
                "Mail Stop" => Some(MailStop),
                "Medical Information Bureau" => Some(MedicalInformationBureau),
                "Medicaid Provider Number" => Some(MedicaidProviderNumber),
                "Manitoba Department of Mines and Resources" => {
                    Some(ManitobaDepartmentOfMinesAndResources)
                }
                "Member Identification Number" => Some(MemberIdentificationNumber),
                "Market" => Some(Market),
                "Multiple Listing Service Vendor - Multiple Listing Service Identification" => {
                    Some(
                        MultipleListingServiceVendorMultipleListingServiceIdentification,
                    )
                }
                "Mortgage Identification Number" => Some(MortgageIdentificationNumber),
                "Major Organizational Entity" => Some(MajorOrganizationalEntity),
                "Medicare Provider Number" => Some(MedicareProviderNumber),
                "Medicaid Recipient Identification Number" => {
                    Some(MedicaidRecipientIdentificationNumber)
                }
                "Insured's Unique Identification Number" => {
                    Some(InsuredsUniqueIdentificationNumber)
                }
                "National Association of Realtors - Multiple Listing Service Identification" => {
                    Some(
                        NationalAssociationOfRealtorsMultipleListingServiceIdentification,
                    )
                }
                "Mode Designator" => Some(ModeDesignator),
                "National Association of Insurance Commissioners (NAIC) Identification" => {
                    Some(CodeNI)
                }
                "National Criminal Information Center Originating Agency" => {
                    Some(NationalCriminalInformationCenterOriginatingAgency)
                }
                "Non Resident Alien Registration Number" => {
                    Some(NonResidentAlienRegistrationNumber)
                }
                "Occupation Code" => Some(OccupationCode),
                "On-line Payment and Collection" => Some(OnLinePaymentAndCollection),
                "Secondary Agent Identification" => Some(SecondaryAgentIdentification),
                "Public Identification" => Some(PublicIdentification),
                "Provider Commercial Number" => Some(ProviderCommercialNumber),
                "Payor Identification" => Some(PayorIdentification),
                "Pharmacy Processor Number" => Some(PharmacyProcessorNumber),
                "Pier" => Some(Pier),
                "Regulatory Agency Number" => Some(RegulatoryAgencyNumber),
                "Real Estate Agent" => Some(RealEstateAgent),
                "Real Estate Company" => Some(RealEstateCompany),
                "Real Estate Broker Identification" => {
                    Some(RealEstateBrokerIdentification)
                }
                "Real Estate License Number" => Some(RealEstateLicenseNumber),
                "Office of Regulatory Information Systems (ORIS) Code" => Some(CodeRI),
                "Ramp" => Some(Ramp),
                "Railroad Track" => Some(RailroadTrack),
                "Title Insurance Policy Number" => Some(TitleInsurancePolicyNumber),
                "Tertiary Agent Identification" => Some(TertiaryAgentIdentification),
                "Social Insurance Number" => Some(SocialInsuranceNumber),
                "Saskatchewan Department of Energy Mines and Resources" => {
                    Some(SaskatchewanDepartmentOfEnergyMinesAndResources)
                }
                "Suffix Code" => Some(SuffixCode),
                "Standard Industry Code (SIC)" => Some(CodeSI),
                "State or Province Jurisdiction" => Some(StateOrProvinceJurisdiction),
                "State/Provincial Lottery License Number" => {
                    Some(StateProvincialLotteryLicenseNumber)
                }
                "State License Number" => Some(StateLicenseNumber),
                "Specialty License Number" => Some(SpecialtyLicenseNumber),
                "State/Province License Tag" => Some(StateProvinceLicenseTag),
                "Service Provider Number" => Some(ServiceProviderNumber),
                "Society for Worldwide Interbank Financial Telecommunications (SWIFT) Address" => {
                    Some(CodeSW)
                }
                "Taxpayer ID Number" => Some(TaxpayerIdNumber),
                "Internal Revenue Service Terminal Code" => {
                    Some(InternalRevenueServiceTerminalCode)
                }
                "Transport4 Location Code" => Some(Transport4LocationCode),
                "Transport4 Shipper Code" => Some(Transport4ShipperCode),
                "Department Code" => Some(DepartmentCode),
                "Consumer Credit Identification Number" => {
                    Some(ConsumerCreditIdentificationNumber)
                }
                "Unit Identification Code" => Some(UnitIdentificationCode),
                "Global Location Number (GLN)" => Some(CodeUL),
                "Unique Physician Identification Number (UPIN)" => Some(CodeUP),
                "Uniform Resource Locator (URL)" => Some(CodeUR),
                "Unique Supplier Identification Number (USIN)" => Some(CodeUS),
                "Unit" => Some(Unit),
                "Wine Region Code" => Some(WineRegionCode),
                "Education Language Codes" => Some(EducationLanguageCodes),
                "National Center for Education Statistics Unit Identification Number" => {
                    Some(NationalCenterForEducationStatisticsUnitIdentificationNumber)
                }
                "Centers for Medicare and Medicaid Services PlanID" => {
                    Some(CentersForMedicareAndMedicaidServicesPlanId)
                }
                "Centers for Medicare and Medicaid Services National Provider Identifier" => {
                    Some(CentersForMedicareAndMedicaidServicesNationalProvider)
                }
                "District Assigned Number" => Some(DistrictAssignedNumber),
                "Contractor Establishment Code" => Some(ContractorEstablishmentCode),
                "Zone" => Some(Zone),
                "Temporary Identification Number" => Some(TemporaryIdentificationNumber),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for IdentificationCodeQualifier {
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
    type Value = IdentificationCodeQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Identification Code Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        IdentificationCodeQualifier::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Identification Code Qualifier: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        IdentificationCodeQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Identification Code Qualifier: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for IdentificationCodeQualifier {
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