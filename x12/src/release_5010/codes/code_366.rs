use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**366

See docs at <https://www.stedi.com/edi/x12-005010/element/366>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContactFunctionCode {
    ///1A - Review/Repricing Contact
    ReviewRepricingContact,
    ///1B - Preferred Provider Organization (PPO) Contact
    Code1B,
    ///1C - Health Maintenance Organization (HMO) Contact
    Code1C,
    ///1D - Third-party Administrator (TPA) Contact
    Code1D,
    ///1E - New Business Processing
    NewBusinessProcessing,
    ///1F - Physician
    Physician,
    ///1G - Service Team
    ServiceTeam,
    ///1H - Underwriter
    Underwriter,
    ///1I - Local Contact
    LocalContact,
    ///3A - Automated Clearinghouse (ACH) Contact
    Code3A,
    ///A1 - Claim Approver
    ClaimApprover,
    ///A2 - Applicant
    Applicant,
    ///A3 - Interviewer
    Interviewer,
    ///A4 - Owner Representative
    OwnerRepresentative,
    ///A5 - Appointment Scheduler
    AppointmentScheduler,
    ///A6 - Concurrent Employer Contact
    ConcurrentEmployerContact,
    ///AA - Authorized Representative
    AuthorizedRepresentative,
    ///AB - Appraiser
    Appraiser,
    ///AC - Administrative Contracting Officer
    AdministrativeContractingOfficer,
    ///AD - Accounting Department
    AccountingDepartment,
    ///AE - Corporate Purchasing Agent
    CorporatePurchasingAgent,
    ///AF - Authorized Financial Contact
    AuthorizedFinancialContact,
    ///AG - Agent
    Agent,
    ///AH - After-Hours Contact
    AfterHoursContact,
    ///AI - Additional Insurance Information Contact
    AdditionalInsuranceInformationContact,
    ///AJ - Primary Contact
    PrimaryContact,
    ///AL - Alternate Contact
    AlternateContact,
    ///AM - Administrator
    Administrator,
    ///AN - Attention-to Party
    AttentionToParty,
    ///AP - Accounts Payable Department
    AccountsPayableDepartment,
    ///AR - Accounts Receivable Department
    AccountsReceivableDepartment,
    ///AS - Authorized Signature
    AuthorizedSignature,
    ///AT - Material Safety Data Sheet Contact
    MaterialSafetyDataSheetContact,
    ///AU - Report Authorizer
    ReportAuthorizer,
    ///AV - Advisor
    Advisor,
    ///AW - Co-investigator
    CoInvestigator,
    ///AX - Additional Contact Points at Institutions
    AdditionalContactPointsAtInstitutions,
    ///BA - Broker
    Broker,
    ///BB - Local Purchasing Agent
    LocalPurchasingAgent,
    ///BC - Broker Contact
    BrokerContact,
    ///BD - Buyer Name or Department
    BuyerNameOrDepartment,
    ///BI - Bill Inquiry Contact
    BillInquiryContact,
    ///BJ - Operations
    Operations,
    ///BK - Marketing Department
    MarketingDepartment,
    ///BL - Technical Department
    TechnicalDepartment,
    ///BM - Work Broker Maintenance Manager
    WorkBrokerMaintenanceManager,
    ///BP - School Principal
    SchoolPrincipal,
    ///BS - Board Staff
    BoardStaff,
    ///BU - Business Unit Manager
    BusinessUnitManager,
    ///C1 - Carrier Contact
    CarrierContact,
    ///C2 - Co-borrower
    CoBorrower,
    ///CA - Customer Contact Granting Appointment
    CustomerContactGrantingAppointment,
    ///CB - Changed By
    ChangedBy,
    ///CC - Computer Systems Contact
    ComputerSystemsContact,
    ///CD - Contract Contact
    ContractContact,
    ///CE - Certifier
    Certifier,
    ///CF - Customer Engineer
    CustomerEngineer,
    ///CG - Chief Executive Officer
    ChiefExecutiveOfficer,
    ///CH - Change Order Approver
    ChangeOrderApprover,
    ///CI - Chief Financial Officer
    ChiefFinancialOfficer,
    ///CJ - Chief Information Officer
    ChiefInformationOfficer,
    ///CK - Chairman of the Board
    ChairmanOfTheBoard,
    ///CL - Chief Operating Officer
    ChiefOperatingOfficer,
    ///CM - Container Manager
    ContainerManager,
    ///CN - General Contact
    GeneralContact,
    ///CO - Component Engineer
    ComponentEngineer,
    ///CP - Cost and Schedule Coordinator
    CostAndScheduleCoordinator,
    ///CR - Customer Relations
    CustomerRelations,
    ///CS - CAD/CAM Specialist
    CadCamSpecialist,
    ///CT - Claimant
    Claimant,
    ///CU - Auditing Contact
    AuditingContact,
    ///CV - Clearinghouse Contact
    ClearinghouseContact,
    ///CW - Confirmed With
    ConfirmedWith,
    ///CX - Payers Claim Office
    PayersClaimOffice,
    ///CY - Case Manager
    CaseManager,
    ///CZ - Claim Recipient
    ClaimRecipient,
    ///DA - Directory Advertising Contact
    DirectoryAdvertisingContact,
    ///DC - Delivery Contact
    DeliveryContact,
    ///DD - Division Director
    DivisionDirector,
    ///DE - Design Engineer
    DesignEngineer,
    ///DF - Director
    Director,
    ///DI - Delivery Instructions Contact
    DeliveryInstructionsContact,
    ///DM - Division Manager
    DivisionManager,
    ///DN - Dental School Admissions Office
    DentalSchoolAdmissionsOffice,
    ///DV - Development
    Development,
    ///E1 - Estimator
    Estimator,
    ///E2 - Evening Programs Office
    EveningProgramsOffice,
    ///EA - EDI Coordinator
    EdiCoordinator,
    ///EB - Entered By
    EnteredBy,
    ///EC - Emergency Contact-Shipper
    EmergencyContactShipper,
    ///ED - Emergency Contact-Consignee
    EmergencyContactConsignee,
    ///EF - Emergency Contact-Military Traffic Management Command (MTMC)
    CodeEF,
    ///EG - Engineering
    Engineering,
    ///EM - Emergency Contact
    EmergencyContact,
    ///EN - Engineer
    Engineer,
    ///EO - Executive Officer
    ExecutiveOfficer,
    ///EP - Employer Contact
    EmployerContact,
    ///ES - Electronic Submission Recipient
    ElectronicSubmissionRecipient,
    ///EV - Executive Vice-President
    ExecutiveVicePresident,
    ///EX - Expeditor
    Expeditor,
    ///FA - Financial Aid Office
    FinancialAidOffice,
    ///FB - Coordinator
    Coordinator,
    ///FC - Forwarder Contact
    ForwarderContact,
    ///FD - Primary Control Point
    PrimaryControlPoint,
    ///FF - Licensee
    Licensee,
    ///FL - Foreclosing Lender Administrative Contact
    ForeclosingLenderAdministrativeContact,
    ///FM - Functional Manager
    FunctionalManager,
    ///FN - Joint Work Agent
    JointWorkAgent,
    ///FO - Office Manager
    OfficeManager,
    ///FP - Marketing Director
    MarketingDirector,
    ///FQ - Staff
    Staff,
    ///FR - Compliance Officer
    ComplianceOfficer,
    ///GA - Graduate Fine Arts Office
    GraduateFineArtsOffice,
    ///GB - Graduate Business Office
    GraduateBusinessOffice,
    ///GC - Guidance Counselor
    GuidanceCounselor,
    ///GE - Graduate Engineering Office
    GraduateEngineeringOffice,
    ///GR - Graduate Admissions Office
    GraduateAdmissionsOffice,
    ///HM - Hazardous Material Contact
    HazardousMaterialContact,
    ///HR - Human Resources
    HumanResources,
    ///IC - Information Contact
    InformationContact,
    ///IO - Issuing Officer
    IssuingOfficer,
    ///IP - Insured Party
    InsuredParty,
    ///IS - Law Firm
    LawFirm,
    ///KA - Authorized Negotiator
    AuthorizedNegotiator,
    ///KB - Preaward Survey Manager
    PreawardSurveyManager,
    ///KC - Accepting Official
    AcceptingOfficial,
    ///KP - Attorney
    Attorney,
    ///KT - Clerk of Court
    ClerkOfCourt,
    ///LD - Law School Admissions Office
    LawSchoolAdmissionsOffice,
    ///LG - Logistics Contact
    LogisticsContact,
    ///MA - Maintenance Contact
    MaintenanceContact,
    ///MB - Mayor
    Mayor,
    ///MC - Medical Contact
    MedicalContact,
    ///MD - Medical Admissions Office
    MedicalAdmissionsOffice,
    ///ME - Manufacturing
    Manufacturing,
    ///MG - Manager
    Manager,
    ///MK - Multiple Listing Service Staff
    MultipleListingServiceStaff,
    ///ML - Multiple Listing Service Vendor
    MultipleListingServiceVendor,
    ///MM - Customer Maintenance Manager
    CustomerMaintenanceManager,
    ///NA - National Agent
    NationalAgent,
    ///NC - Numerical Control Engineer
    NumericalControlEngineer,
    ///NP - Notary Public
    NotaryPublic,
    ///NT - Notification Contact
    NotificationContact,
    ///OA - Other Adult
    OtherAdult,
    ///OC - Order Contact
    OrderContact,
    ///OD - Order Department
    OrderDepartment,
    ///OS - Office Staff
    OfficeStaff,
    ///OW - Owner
    Owner,
    ///PA - President
    President,
    ///PB - Plant Manager
    PlantManager,
    ///PC - Purchasing Contracting Officer (PCO)
    CodePC,
    ///PD - Project Director
    ProjectDirector,
    ///PE - Process Engineer
    ProcessEngineer,
    ///PF - Price Administration
    PriceAdministration,
    ///PG - Program Director
    ProgramDirector,
    ///PH - Provider
    Provider,
    ///PI - Preparer
    Preparer,
    ///PJ - Project Manager
    ProjectManager,
    ///PK - Performance Evaluation Committee
    PerformanceEvaluationCommittee,
    ///PL - Manufacturing Plant Contact
    ManufacturingPlantContact,
    ///PM - Product Manager
    ProductManager,
    ///PN - Probation or Legal Officer
    ProbationOrLegalOfficer,
    ///PO - Production Representative
    ProductionRepresentative,
    ///PP - Program Manager
    ProgramManager,
    ///PQ - Parent or Guardian
    ParentOrGuardian,
    ///PR - Prototype Coordinator
    PrototypeCoordinator,
    ///PS - Personnel Department
    PersonnelDepartment,
    ///PT - Partner
    Partner,
    ///PU - Report Preparer
    ReportPreparer,
    ///PV - Participating Laboratory Contact
    ParticipatingLaboratoryContact,
    ///PW - Principal Study Contact or Author
    PrincipalStudyContactOrAuthor,
    ///PX - Purchase Service Provider
    PurchaseServiceProvider,
    ///PY - Packager
    Packager,
    ///PZ - Patient
    Patient,
    ///QA - Quality Assurance Contact
    QualityAssuranceContact,
    ///QC - Quality Coordinator
    QualityCoordinator,
    ///QI - Quality Inspector
    QualityInspector,
    ///QM - Quality Manager
    QualityManager,
    ///QP - Quoting Party
    QuotingParty,
    ///QR - Ordering Officer
    OrderingOfficer,
    ///QY - Port Engineer
    PortEngineer,
    ///RA - Rental Company Administrative Contact
    RentalCompanyAdministrativeContact,
    ///RB - Real Estate Property Occupant
    RealEstatePropertyOccupant,
    ///RC - Rebate/Chargeback Contact
    RebateChargebackContact,
    ///RD - Receiving Dock
    ReceivingDock,
    ///RE - Receiving Contact
    ReceivingContact,
    ///RF - Real Estate Property Key Holder
    RealEstatePropertyKeyHolder,
    ///RG - Registrar
    Registrar,
    ///RP - Responsible Person
    ResponsiblePerson,
    ///RQ - Requestor
    Requestor,
    ///RS - Rate Supervisor or Clerk
    RateSupervisorOrClerk,
    ///RZ - Respondant
    Respondant,
    ///SA - Sales Administration
    SalesAdministration,
    ///SB - Student
    Student,
    ///SC - Schedule Contact
    ScheduleContact,
    ///SD - Shipping Department
    ShippingDepartment,
    ///SE - Service Organization
    ServiceOrganization,
    ///SF - Student in Absentia
    StudentInAbsentia,
    ///SG - Secretary
    Secretary,
    ///SH - Shipper Contact
    ShipperContact,
    ///SI - Investigator
    Investigator,
    ///SJ - Spouse
    Spouse,
    ///SK - School Clerk
    SchoolClerk,
    ///SL - Collector
    Collector,
    ///SM - Submitting Contact
    SubmittingContact,
    ///SN - Study Submitter Contact
    StudySubmitterContact,
    ///SO - Service Order Writer
    ServiceOrderWriter,
    ///SP - Special Program Contact
    SpecialProgramContact,
    ///SQ - Systems Administrator
    SystemsAdministrator,
    ///SR - Sales Representative or Department
    SalesRepresentativeOrDepartment,
    ///SS - Supervisor
    Supervisor,
    ///ST - Service Technician
    ServiceTechnician,
    ///SU - Supplier Contact
    SupplierContact,
    ///SV - Service Manager
    ServiceManager,
    ///SW - Social Services Worker
    SocialServicesWorker,
    ///SY - Secondary Taxpayer
    SecondaryTaxpayer,
    ///TA - Traffic Administrator
    TrafficAdministrator,
    ///TB - Telephone Answering Service Contact
    TelephoneAnsweringServiceContact,
    ///TC - College of Education Admissions Office
    CollegeOfEducationAdmissionsOffice,
    ///TD - Tender Developer
    TenderDeveloper,
    ///TE - Treasurer
    Treasurer,
    ///TH - School of Theology Admissions Office
    SchoolOfTheologyAdmissionsOffice,
    ///TM - Transmitter
    Transmitter,
    ///TN - Tenant
    Tenant,
    ///TP - Primary Taxpayer
    PrimaryTaxpayer,
    ///TR - Technical Marketing Representative
    TechnicalMarketingRepresentative,
    ///TY - Platform Maintenance Manager
    PlatformMaintenanceManager,
    ///UG - Undergraduate Admissions Office
    UndergraduateAdmissionsOffice,
    ///UP - Union President
    UnionPresident,
    ///UQ - Processor
    Processor,
    ///UR - Ultimate Receiver
    UltimateReceiver,
    ///VM - School of Veterinary Medicine Admissions Office
    SchoolOfVeterinaryMedicineAdmissionsOffice,
    ///VP - Vice President
    VicePresident,
    ///WH - Warehouse
    Warehouse,
    ///WI - Witness
    Witness,
    ///WR - Technical Writer
    TechnicalWriter,
    ///WV - Waiver Application Contact
    WaiverApplicationContact,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl ContactFunctionCode {
    pub fn code(&self) -> &str {
        {
            use ContactFunctionCode::*;
            match self {
                ReviewRepricingContact => "1A",
                Code1B => "1B",
                Code1C => "1C",
                Code1D => "1D",
                NewBusinessProcessing => "1E",
                Physician => "1F",
                ServiceTeam => "1G",
                Underwriter => "1H",
                LocalContact => "1I",
                Code3A => "3A",
                ClaimApprover => "A1",
                Applicant => "A2",
                Interviewer => "A3",
                OwnerRepresentative => "A4",
                AppointmentScheduler => "A5",
                ConcurrentEmployerContact => "A6",
                AuthorizedRepresentative => "AA",
                Appraiser => "AB",
                AdministrativeContractingOfficer => "AC",
                AccountingDepartment => "AD",
                CorporatePurchasingAgent => "AE",
                AuthorizedFinancialContact => "AF",
                Agent => "AG",
                AfterHoursContact => "AH",
                AdditionalInsuranceInformationContact => "AI",
                PrimaryContact => "AJ",
                AlternateContact => "AL",
                Administrator => "AM",
                AttentionToParty => "AN",
                AccountsPayableDepartment => "AP",
                AccountsReceivableDepartment => "AR",
                AuthorizedSignature => "AS",
                MaterialSafetyDataSheetContact => "AT",
                ReportAuthorizer => "AU",
                Advisor => "AV",
                CoInvestigator => "AW",
                AdditionalContactPointsAtInstitutions => "AX",
                Broker => "BA",
                LocalPurchasingAgent => "BB",
                BrokerContact => "BC",
                BuyerNameOrDepartment => "BD",
                BillInquiryContact => "BI",
                Operations => "BJ",
                MarketingDepartment => "BK",
                TechnicalDepartment => "BL",
                WorkBrokerMaintenanceManager => "BM",
                SchoolPrincipal => "BP",
                BoardStaff => "BS",
                BusinessUnitManager => "BU",
                CarrierContact => "C1",
                CoBorrower => "C2",
                CustomerContactGrantingAppointment => "CA",
                ChangedBy => "CB",
                ComputerSystemsContact => "CC",
                ContractContact => "CD",
                Certifier => "CE",
                CustomerEngineer => "CF",
                ChiefExecutiveOfficer => "CG",
                ChangeOrderApprover => "CH",
                ChiefFinancialOfficer => "CI",
                ChiefInformationOfficer => "CJ",
                ChairmanOfTheBoard => "CK",
                ChiefOperatingOfficer => "CL",
                ContainerManager => "CM",
                GeneralContact => "CN",
                ComponentEngineer => "CO",
                CostAndScheduleCoordinator => "CP",
                CustomerRelations => "CR",
                CadCamSpecialist => "CS",
                Claimant => "CT",
                AuditingContact => "CU",
                ClearinghouseContact => "CV",
                ConfirmedWith => "CW",
                PayersClaimOffice => "CX",
                CaseManager => "CY",
                ClaimRecipient => "CZ",
                DirectoryAdvertisingContact => "DA",
                DeliveryContact => "DC",
                DivisionDirector => "DD",
                DesignEngineer => "DE",
                Director => "DF",
                DeliveryInstructionsContact => "DI",
                DivisionManager => "DM",
                DentalSchoolAdmissionsOffice => "DN",
                Development => "DV",
                Estimator => "E1",
                EveningProgramsOffice => "E2",
                EdiCoordinator => "EA",
                EnteredBy => "EB",
                EmergencyContactShipper => "EC",
                EmergencyContactConsignee => "ED",
                CodeEF => "EF",
                Engineering => "EG",
                EmergencyContact => "EM",
                Engineer => "EN",
                ExecutiveOfficer => "EO",
                EmployerContact => "EP",
                ElectronicSubmissionRecipient => "ES",
                ExecutiveVicePresident => "EV",
                Expeditor => "EX",
                FinancialAidOffice => "FA",
                Coordinator => "FB",
                ForwarderContact => "FC",
                PrimaryControlPoint => "FD",
                Licensee => "FF",
                ForeclosingLenderAdministrativeContact => "FL",
                FunctionalManager => "FM",
                JointWorkAgent => "FN",
                OfficeManager => "FO",
                MarketingDirector => "FP",
                Staff => "FQ",
                ComplianceOfficer => "FR",
                GraduateFineArtsOffice => "GA",
                GraduateBusinessOffice => "GB",
                GuidanceCounselor => "GC",
                GraduateEngineeringOffice => "GE",
                GraduateAdmissionsOffice => "GR",
                HazardousMaterialContact => "HM",
                HumanResources => "HR",
                InformationContact => "IC",
                IssuingOfficer => "IO",
                InsuredParty => "IP",
                LawFirm => "IS",
                AuthorizedNegotiator => "KA",
                PreawardSurveyManager => "KB",
                AcceptingOfficial => "KC",
                Attorney => "KP",
                ClerkOfCourt => "KT",
                LawSchoolAdmissionsOffice => "LD",
                LogisticsContact => "LG",
                MaintenanceContact => "MA",
                Mayor => "MB",
                MedicalContact => "MC",
                MedicalAdmissionsOffice => "MD",
                Manufacturing => "ME",
                Manager => "MG",
                MultipleListingServiceStaff => "MK",
                MultipleListingServiceVendor => "ML",
                CustomerMaintenanceManager => "MM",
                NationalAgent => "NA",
                NumericalControlEngineer => "NC",
                NotaryPublic => "NP",
                NotificationContact => "NT",
                OtherAdult => "OA",
                OrderContact => "OC",
                OrderDepartment => "OD",
                OfficeStaff => "OS",
                Owner => "OW",
                President => "PA",
                PlantManager => "PB",
                CodePC => "PC",
                ProjectDirector => "PD",
                ProcessEngineer => "PE",
                PriceAdministration => "PF",
                ProgramDirector => "PG",
                Provider => "PH",
                Preparer => "PI",
                ProjectManager => "PJ",
                PerformanceEvaluationCommittee => "PK",
                ManufacturingPlantContact => "PL",
                ProductManager => "PM",
                ProbationOrLegalOfficer => "PN",
                ProductionRepresentative => "PO",
                ProgramManager => "PP",
                ParentOrGuardian => "PQ",
                PrototypeCoordinator => "PR",
                PersonnelDepartment => "PS",
                Partner => "PT",
                ReportPreparer => "PU",
                ParticipatingLaboratoryContact => "PV",
                PrincipalStudyContactOrAuthor => "PW",
                PurchaseServiceProvider => "PX",
                Packager => "PY",
                Patient => "PZ",
                QualityAssuranceContact => "QA",
                QualityCoordinator => "QC",
                QualityInspector => "QI",
                QualityManager => "QM",
                QuotingParty => "QP",
                OrderingOfficer => "QR",
                PortEngineer => "QY",
                RentalCompanyAdministrativeContact => "RA",
                RealEstatePropertyOccupant => "RB",
                RebateChargebackContact => "RC",
                ReceivingDock => "RD",
                ReceivingContact => "RE",
                RealEstatePropertyKeyHolder => "RF",
                Registrar => "RG",
                ResponsiblePerson => "RP",
                Requestor => "RQ",
                RateSupervisorOrClerk => "RS",
                Respondant => "RZ",
                SalesAdministration => "SA",
                Student => "SB",
                ScheduleContact => "SC",
                ShippingDepartment => "SD",
                ServiceOrganization => "SE",
                StudentInAbsentia => "SF",
                Secretary => "SG",
                ShipperContact => "SH",
                Investigator => "SI",
                Spouse => "SJ",
                SchoolClerk => "SK",
                Collector => "SL",
                SubmittingContact => "SM",
                StudySubmitterContact => "SN",
                ServiceOrderWriter => "SO",
                SpecialProgramContact => "SP",
                SystemsAdministrator => "SQ",
                SalesRepresentativeOrDepartment => "SR",
                Supervisor => "SS",
                ServiceTechnician => "ST",
                SupplierContact => "SU",
                ServiceManager => "SV",
                SocialServicesWorker => "SW",
                SecondaryTaxpayer => "SY",
                TrafficAdministrator => "TA",
                TelephoneAnsweringServiceContact => "TB",
                CollegeOfEducationAdmissionsOffice => "TC",
                TenderDeveloper => "TD",
                Treasurer => "TE",
                SchoolOfTheologyAdmissionsOffice => "TH",
                Transmitter => "TM",
                Tenant => "TN",
                PrimaryTaxpayer => "TP",
                TechnicalMarketingRepresentative => "TR",
                PlatformMaintenanceManager => "TY",
                UndergraduateAdmissionsOffice => "UG",
                UnionPresident => "UP",
                Processor => "UQ",
                UltimateReceiver => "UR",
                SchoolOfVeterinaryMedicineAdmissionsOffice => "VM",
                VicePresident => "VP",
                Warehouse => "WH",
                Witness => "WI",
                TechnicalWriter => "WR",
                WaiverApplicationContact => "WV",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ContactFunctionCode> {
        use ContactFunctionCode::*;
        match code {
            b"1A" => Some(ReviewRepricingContact),
            b"1B" => Some(Code1B),
            b"1C" => Some(Code1C),
            b"1D" => Some(Code1D),
            b"1E" => Some(NewBusinessProcessing),
            b"1F" => Some(Physician),
            b"1G" => Some(ServiceTeam),
            b"1H" => Some(Underwriter),
            b"1I" => Some(LocalContact),
            b"3A" => Some(Code3A),
            b"A1" => Some(ClaimApprover),
            b"A2" => Some(Applicant),
            b"A3" => Some(Interviewer),
            b"A4" => Some(OwnerRepresentative),
            b"A5" => Some(AppointmentScheduler),
            b"A6" => Some(ConcurrentEmployerContact),
            b"AA" => Some(AuthorizedRepresentative),
            b"AB" => Some(Appraiser),
            b"AC" => Some(AdministrativeContractingOfficer),
            b"AD" => Some(AccountingDepartment),
            b"AE" => Some(CorporatePurchasingAgent),
            b"AF" => Some(AuthorizedFinancialContact),
            b"AG" => Some(Agent),
            b"AH" => Some(AfterHoursContact),
            b"AI" => Some(AdditionalInsuranceInformationContact),
            b"AJ" => Some(PrimaryContact),
            b"AL" => Some(AlternateContact),
            b"AM" => Some(Administrator),
            b"AN" => Some(AttentionToParty),
            b"AP" => Some(AccountsPayableDepartment),
            b"AR" => Some(AccountsReceivableDepartment),
            b"AS" => Some(AuthorizedSignature),
            b"AT" => Some(MaterialSafetyDataSheetContact),
            b"AU" => Some(ReportAuthorizer),
            b"AV" => Some(Advisor),
            b"AW" => Some(CoInvestigator),
            b"AX" => Some(AdditionalContactPointsAtInstitutions),
            b"BA" => Some(Broker),
            b"BB" => Some(LocalPurchasingAgent),
            b"BC" => Some(BrokerContact),
            b"BD" => Some(BuyerNameOrDepartment),
            b"BI" => Some(BillInquiryContact),
            b"BJ" => Some(Operations),
            b"BK" => Some(MarketingDepartment),
            b"BL" => Some(TechnicalDepartment),
            b"BM" => Some(WorkBrokerMaintenanceManager),
            b"BP" => Some(SchoolPrincipal),
            b"BS" => Some(BoardStaff),
            b"BU" => Some(BusinessUnitManager),
            b"C1" => Some(CarrierContact),
            b"C2" => Some(CoBorrower),
            b"CA" => Some(CustomerContactGrantingAppointment),
            b"CB" => Some(ChangedBy),
            b"CC" => Some(ComputerSystemsContact),
            b"CD" => Some(ContractContact),
            b"CE" => Some(Certifier),
            b"CF" => Some(CustomerEngineer),
            b"CG" => Some(ChiefExecutiveOfficer),
            b"CH" => Some(ChangeOrderApprover),
            b"CI" => Some(ChiefFinancialOfficer),
            b"CJ" => Some(ChiefInformationOfficer),
            b"CK" => Some(ChairmanOfTheBoard),
            b"CL" => Some(ChiefOperatingOfficer),
            b"CM" => Some(ContainerManager),
            b"CN" => Some(GeneralContact),
            b"CO" => Some(ComponentEngineer),
            b"CP" => Some(CostAndScheduleCoordinator),
            b"CR" => Some(CustomerRelations),
            b"CS" => Some(CadCamSpecialist),
            b"CT" => Some(Claimant),
            b"CU" => Some(AuditingContact),
            b"CV" => Some(ClearinghouseContact),
            b"CW" => Some(ConfirmedWith),
            b"CX" => Some(PayersClaimOffice),
            b"CY" => Some(CaseManager),
            b"CZ" => Some(ClaimRecipient),
            b"DA" => Some(DirectoryAdvertisingContact),
            b"DC" => Some(DeliveryContact),
            b"DD" => Some(DivisionDirector),
            b"DE" => Some(DesignEngineer),
            b"DF" => Some(Director),
            b"DI" => Some(DeliveryInstructionsContact),
            b"DM" => Some(DivisionManager),
            b"DN" => Some(DentalSchoolAdmissionsOffice),
            b"DV" => Some(Development),
            b"E1" => Some(Estimator),
            b"E2" => Some(EveningProgramsOffice),
            b"EA" => Some(EdiCoordinator),
            b"EB" => Some(EnteredBy),
            b"EC" => Some(EmergencyContactShipper),
            b"ED" => Some(EmergencyContactConsignee),
            b"EF" => Some(CodeEF),
            b"EG" => Some(Engineering),
            b"EM" => Some(EmergencyContact),
            b"EN" => Some(Engineer),
            b"EO" => Some(ExecutiveOfficer),
            b"EP" => Some(EmployerContact),
            b"ES" => Some(ElectronicSubmissionRecipient),
            b"EV" => Some(ExecutiveVicePresident),
            b"EX" => Some(Expeditor),
            b"FA" => Some(FinancialAidOffice),
            b"FB" => Some(Coordinator),
            b"FC" => Some(ForwarderContact),
            b"FD" => Some(PrimaryControlPoint),
            b"FF" => Some(Licensee),
            b"FL" => Some(ForeclosingLenderAdministrativeContact),
            b"FM" => Some(FunctionalManager),
            b"FN" => Some(JointWorkAgent),
            b"FO" => Some(OfficeManager),
            b"FP" => Some(MarketingDirector),
            b"FQ" => Some(Staff),
            b"FR" => Some(ComplianceOfficer),
            b"GA" => Some(GraduateFineArtsOffice),
            b"GB" => Some(GraduateBusinessOffice),
            b"GC" => Some(GuidanceCounselor),
            b"GE" => Some(GraduateEngineeringOffice),
            b"GR" => Some(GraduateAdmissionsOffice),
            b"HM" => Some(HazardousMaterialContact),
            b"HR" => Some(HumanResources),
            b"IC" => Some(InformationContact),
            b"IO" => Some(IssuingOfficer),
            b"IP" => Some(InsuredParty),
            b"IS" => Some(LawFirm),
            b"KA" => Some(AuthorizedNegotiator),
            b"KB" => Some(PreawardSurveyManager),
            b"KC" => Some(AcceptingOfficial),
            b"KP" => Some(Attorney),
            b"KT" => Some(ClerkOfCourt),
            b"LD" => Some(LawSchoolAdmissionsOffice),
            b"LG" => Some(LogisticsContact),
            b"MA" => Some(MaintenanceContact),
            b"MB" => Some(Mayor),
            b"MC" => Some(MedicalContact),
            b"MD" => Some(MedicalAdmissionsOffice),
            b"ME" => Some(Manufacturing),
            b"MG" => Some(Manager),
            b"MK" => Some(MultipleListingServiceStaff),
            b"ML" => Some(MultipleListingServiceVendor),
            b"MM" => Some(CustomerMaintenanceManager),
            b"NA" => Some(NationalAgent),
            b"NC" => Some(NumericalControlEngineer),
            b"NP" => Some(NotaryPublic),
            b"NT" => Some(NotificationContact),
            b"OA" => Some(OtherAdult),
            b"OC" => Some(OrderContact),
            b"OD" => Some(OrderDepartment),
            b"OS" => Some(OfficeStaff),
            b"OW" => Some(Owner),
            b"PA" => Some(President),
            b"PB" => Some(PlantManager),
            b"PC" => Some(CodePC),
            b"PD" => Some(ProjectDirector),
            b"PE" => Some(ProcessEngineer),
            b"PF" => Some(PriceAdministration),
            b"PG" => Some(ProgramDirector),
            b"PH" => Some(Provider),
            b"PI" => Some(Preparer),
            b"PJ" => Some(ProjectManager),
            b"PK" => Some(PerformanceEvaluationCommittee),
            b"PL" => Some(ManufacturingPlantContact),
            b"PM" => Some(ProductManager),
            b"PN" => Some(ProbationOrLegalOfficer),
            b"PO" => Some(ProductionRepresentative),
            b"PP" => Some(ProgramManager),
            b"PQ" => Some(ParentOrGuardian),
            b"PR" => Some(PrototypeCoordinator),
            b"PS" => Some(PersonnelDepartment),
            b"PT" => Some(Partner),
            b"PU" => Some(ReportPreparer),
            b"PV" => Some(ParticipatingLaboratoryContact),
            b"PW" => Some(PrincipalStudyContactOrAuthor),
            b"PX" => Some(PurchaseServiceProvider),
            b"PY" => Some(Packager),
            b"PZ" => Some(Patient),
            b"QA" => Some(QualityAssuranceContact),
            b"QC" => Some(QualityCoordinator),
            b"QI" => Some(QualityInspector),
            b"QM" => Some(QualityManager),
            b"QP" => Some(QuotingParty),
            b"QR" => Some(OrderingOfficer),
            b"QY" => Some(PortEngineer),
            b"RA" => Some(RentalCompanyAdministrativeContact),
            b"RB" => Some(RealEstatePropertyOccupant),
            b"RC" => Some(RebateChargebackContact),
            b"RD" => Some(ReceivingDock),
            b"RE" => Some(ReceivingContact),
            b"RF" => Some(RealEstatePropertyKeyHolder),
            b"RG" => Some(Registrar),
            b"RP" => Some(ResponsiblePerson),
            b"RQ" => Some(Requestor),
            b"RS" => Some(RateSupervisorOrClerk),
            b"RZ" => Some(Respondant),
            b"SA" => Some(SalesAdministration),
            b"SB" => Some(Student),
            b"SC" => Some(ScheduleContact),
            b"SD" => Some(ShippingDepartment),
            b"SE" => Some(ServiceOrganization),
            b"SF" => Some(StudentInAbsentia),
            b"SG" => Some(Secretary),
            b"SH" => Some(ShipperContact),
            b"SI" => Some(Investigator),
            b"SJ" => Some(Spouse),
            b"SK" => Some(SchoolClerk),
            b"SL" => Some(Collector),
            b"SM" => Some(SubmittingContact),
            b"SN" => Some(StudySubmitterContact),
            b"SO" => Some(ServiceOrderWriter),
            b"SP" => Some(SpecialProgramContact),
            b"SQ" => Some(SystemsAdministrator),
            b"SR" => Some(SalesRepresentativeOrDepartment),
            b"SS" => Some(Supervisor),
            b"ST" => Some(ServiceTechnician),
            b"SU" => Some(SupplierContact),
            b"SV" => Some(ServiceManager),
            b"SW" => Some(SocialServicesWorker),
            b"SY" => Some(SecondaryTaxpayer),
            b"TA" => Some(TrafficAdministrator),
            b"TB" => Some(TelephoneAnsweringServiceContact),
            b"TC" => Some(CollegeOfEducationAdmissionsOffice),
            b"TD" => Some(TenderDeveloper),
            b"TE" => Some(Treasurer),
            b"TH" => Some(SchoolOfTheologyAdmissionsOffice),
            b"TM" => Some(Transmitter),
            b"TN" => Some(Tenant),
            b"TP" => Some(PrimaryTaxpayer),
            b"TR" => Some(TechnicalMarketingRepresentative),
            b"TY" => Some(PlatformMaintenanceManager),
            b"UG" => Some(UndergraduateAdmissionsOffice),
            b"UP" => Some(UnionPresident),
            b"UQ" => Some(Processor),
            b"UR" => Some(UltimateReceiver),
            b"VM" => Some(SchoolOfVeterinaryMedicineAdmissionsOffice),
            b"VP" => Some(VicePresident),
            b"WH" => Some(Warehouse),
            b"WI" => Some(Witness),
            b"WR" => Some(TechnicalWriter),
            b"WV" => Some(WaiverApplicationContact),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ContactFunctionCode::*;
        match self {
            ReviewRepricingContact => "Review/Repricing Contact",
            Code1B => "Preferred Provider Organization (PPO) Contact",
            Code1C => "Health Maintenance Organization (HMO) Contact",
            Code1D => "Third-party Administrator (TPA) Contact",
            NewBusinessProcessing => "New Business Processing",
            Physician => "Physician",
            ServiceTeam => "Service Team",
            Underwriter => "Underwriter",
            LocalContact => "Local Contact",
            Code3A => "Automated Clearinghouse (ACH) Contact",
            ClaimApprover => "Claim Approver",
            Applicant => "Applicant",
            Interviewer => "Interviewer",
            OwnerRepresentative => "Owner Representative",
            AppointmentScheduler => "Appointment Scheduler",
            ConcurrentEmployerContact => "Concurrent Employer Contact",
            AuthorizedRepresentative => "Authorized Representative",
            Appraiser => "Appraiser",
            AdministrativeContractingOfficer => "Administrative Contracting Officer",
            AccountingDepartment => "Accounting Department",
            CorporatePurchasingAgent => "Corporate Purchasing Agent",
            AuthorizedFinancialContact => "Authorized Financial Contact",
            Agent => "Agent",
            AfterHoursContact => "After-Hours Contact",
            AdditionalInsuranceInformationContact => {
                "Additional Insurance Information Contact"
            }
            PrimaryContact => "Primary Contact",
            AlternateContact => "Alternate Contact",
            Administrator => "Administrator",
            AttentionToParty => "Attention-to Party",
            AccountsPayableDepartment => "Accounts Payable Department",
            AccountsReceivableDepartment => "Accounts Receivable Department",
            AuthorizedSignature => "Authorized Signature",
            MaterialSafetyDataSheetContact => "Material Safety Data Sheet Contact",
            ReportAuthorizer => "Report Authorizer",
            Advisor => "Advisor",
            CoInvestigator => "Co-investigator",
            AdditionalContactPointsAtInstitutions => {
                "Additional Contact Points at Institutions"
            }
            Broker => "Broker",
            LocalPurchasingAgent => "Local Purchasing Agent",
            BrokerContact => "Broker Contact",
            BuyerNameOrDepartment => "Buyer Name or Department",
            BillInquiryContact => "Bill Inquiry Contact",
            Operations => "Operations",
            MarketingDepartment => "Marketing Department",
            TechnicalDepartment => "Technical Department",
            WorkBrokerMaintenanceManager => "Work Broker Maintenance Manager",
            SchoolPrincipal => "School Principal",
            BoardStaff => "Board Staff",
            BusinessUnitManager => "Business Unit Manager",
            CarrierContact => "Carrier Contact",
            CoBorrower => "Co-borrower",
            CustomerContactGrantingAppointment => "Customer Contact Granting Appointment",
            ChangedBy => "Changed By",
            ComputerSystemsContact => "Computer Systems Contact",
            ContractContact => "Contract Contact",
            Certifier => "Certifier",
            CustomerEngineer => "Customer Engineer",
            ChiefExecutiveOfficer => "Chief Executive Officer",
            ChangeOrderApprover => "Change Order Approver",
            ChiefFinancialOfficer => "Chief Financial Officer",
            ChiefInformationOfficer => "Chief Information Officer",
            ChairmanOfTheBoard => "Chairman of the Board",
            ChiefOperatingOfficer => "Chief Operating Officer",
            ContainerManager => "Container Manager",
            GeneralContact => "General Contact",
            ComponentEngineer => "Component Engineer",
            CostAndScheduleCoordinator => "Cost and Schedule Coordinator",
            CustomerRelations => "Customer Relations",
            CadCamSpecialist => "CAD/CAM Specialist",
            Claimant => "Claimant",
            AuditingContact => "Auditing Contact",
            ClearinghouseContact => "Clearinghouse Contact",
            ConfirmedWith => "Confirmed With",
            PayersClaimOffice => "Payers Claim Office",
            CaseManager => "Case Manager",
            ClaimRecipient => "Claim Recipient",
            DirectoryAdvertisingContact => "Directory Advertising Contact",
            DeliveryContact => "Delivery Contact",
            DivisionDirector => "Division Director",
            DesignEngineer => "Design Engineer",
            Director => "Director",
            DeliveryInstructionsContact => "Delivery Instructions Contact",
            DivisionManager => "Division Manager",
            DentalSchoolAdmissionsOffice => "Dental School Admissions Office",
            Development => "Development",
            Estimator => "Estimator",
            EveningProgramsOffice => "Evening Programs Office",
            EdiCoordinator => "EDI Coordinator",
            EnteredBy => "Entered By",
            EmergencyContactShipper => "Emergency Contact-Shipper",
            EmergencyContactConsignee => "Emergency Contact-Consignee",
            CodeEF => "Emergency Contact-Military Traffic Management Command (MTMC)",
            Engineering => "Engineering",
            EmergencyContact => "Emergency Contact",
            Engineer => "Engineer",
            ExecutiveOfficer => "Executive Officer",
            EmployerContact => "Employer Contact",
            ElectronicSubmissionRecipient => "Electronic Submission Recipient",
            ExecutiveVicePresident => "Executive Vice-President",
            Expeditor => "Expeditor",
            FinancialAidOffice => "Financial Aid Office",
            Coordinator => "Coordinator",
            ForwarderContact => "Forwarder Contact",
            PrimaryControlPoint => "Primary Control Point",
            Licensee => "Licensee",
            ForeclosingLenderAdministrativeContact => {
                "Foreclosing Lender Administrative Contact"
            }
            FunctionalManager => "Functional Manager",
            JointWorkAgent => "Joint Work Agent",
            OfficeManager => "Office Manager",
            MarketingDirector => "Marketing Director",
            Staff => "Staff",
            ComplianceOfficer => "Compliance Officer",
            GraduateFineArtsOffice => "Graduate Fine Arts Office",
            GraduateBusinessOffice => "Graduate Business Office",
            GuidanceCounselor => "Guidance Counselor",
            GraduateEngineeringOffice => "Graduate Engineering Office",
            GraduateAdmissionsOffice => "Graduate Admissions Office",
            HazardousMaterialContact => "Hazardous Material Contact",
            HumanResources => "Human Resources",
            InformationContact => "Information Contact",
            IssuingOfficer => "Issuing Officer",
            InsuredParty => "Insured Party",
            LawFirm => "Law Firm",
            AuthorizedNegotiator => "Authorized Negotiator",
            PreawardSurveyManager => "Preaward Survey Manager",
            AcceptingOfficial => "Accepting Official",
            Attorney => "Attorney",
            ClerkOfCourt => "Clerk of Court",
            LawSchoolAdmissionsOffice => "Law School Admissions Office",
            LogisticsContact => "Logistics Contact",
            MaintenanceContact => "Maintenance Contact",
            Mayor => "Mayor",
            MedicalContact => "Medical Contact",
            MedicalAdmissionsOffice => "Medical Admissions Office",
            Manufacturing => "Manufacturing",
            Manager => "Manager",
            MultipleListingServiceStaff => "Multiple Listing Service Staff",
            MultipleListingServiceVendor => "Multiple Listing Service Vendor",
            CustomerMaintenanceManager => "Customer Maintenance Manager",
            NationalAgent => "National Agent",
            NumericalControlEngineer => "Numerical Control Engineer",
            NotaryPublic => "Notary Public",
            NotificationContact => "Notification Contact",
            OtherAdult => "Other Adult",
            OrderContact => "Order Contact",
            OrderDepartment => "Order Department",
            OfficeStaff => "Office Staff",
            Owner => "Owner",
            President => "President",
            PlantManager => "Plant Manager",
            CodePC => "Purchasing Contracting Officer (PCO)",
            ProjectDirector => "Project Director",
            ProcessEngineer => "Process Engineer",
            PriceAdministration => "Price Administration",
            ProgramDirector => "Program Director",
            Provider => "Provider",
            Preparer => "Preparer",
            ProjectManager => "Project Manager",
            PerformanceEvaluationCommittee => "Performance Evaluation Committee",
            ManufacturingPlantContact => "Manufacturing Plant Contact",
            ProductManager => "Product Manager",
            ProbationOrLegalOfficer => "Probation or Legal Officer",
            ProductionRepresentative => "Production Representative",
            ProgramManager => "Program Manager",
            ParentOrGuardian => "Parent or Guardian",
            PrototypeCoordinator => "Prototype Coordinator",
            PersonnelDepartment => "Personnel Department",
            Partner => "Partner",
            ReportPreparer => "Report Preparer",
            ParticipatingLaboratoryContact => "Participating Laboratory Contact",
            PrincipalStudyContactOrAuthor => "Principal Study Contact or Author",
            PurchaseServiceProvider => "Purchase Service Provider",
            Packager => "Packager",
            Patient => "Patient",
            QualityAssuranceContact => "Quality Assurance Contact",
            QualityCoordinator => "Quality Coordinator",
            QualityInspector => "Quality Inspector",
            QualityManager => "Quality Manager",
            QuotingParty => "Quoting Party",
            OrderingOfficer => "Ordering Officer",
            PortEngineer => "Port Engineer",
            RentalCompanyAdministrativeContact => "Rental Company Administrative Contact",
            RealEstatePropertyOccupant => "Real Estate Property Occupant",
            RebateChargebackContact => "Rebate/Chargeback Contact",
            ReceivingDock => "Receiving Dock",
            ReceivingContact => "Receiving Contact",
            RealEstatePropertyKeyHolder => "Real Estate Property Key Holder",
            Registrar => "Registrar",
            ResponsiblePerson => "Responsible Person",
            Requestor => "Requestor",
            RateSupervisorOrClerk => "Rate Supervisor or Clerk",
            Respondant => "Respondant",
            SalesAdministration => "Sales Administration",
            Student => "Student",
            ScheduleContact => "Schedule Contact",
            ShippingDepartment => "Shipping Department",
            ServiceOrganization => "Service Organization",
            StudentInAbsentia => "Student in Absentia",
            Secretary => "Secretary",
            ShipperContact => "Shipper Contact",
            Investigator => "Investigator",
            Spouse => "Spouse",
            SchoolClerk => "School Clerk",
            Collector => "Collector",
            SubmittingContact => "Submitting Contact",
            StudySubmitterContact => "Study Submitter Contact",
            ServiceOrderWriter => "Service Order Writer",
            SpecialProgramContact => "Special Program Contact",
            SystemsAdministrator => "Systems Administrator",
            SalesRepresentativeOrDepartment => "Sales Representative or Department",
            Supervisor => "Supervisor",
            ServiceTechnician => "Service Technician",
            SupplierContact => "Supplier Contact",
            ServiceManager => "Service Manager",
            SocialServicesWorker => "Social Services Worker",
            SecondaryTaxpayer => "Secondary Taxpayer",
            TrafficAdministrator => "Traffic Administrator",
            TelephoneAnsweringServiceContact => "Telephone Answering Service Contact",
            CollegeOfEducationAdmissionsOffice => {
                "College of Education Admissions Office"
            }
            TenderDeveloper => "Tender Developer",
            Treasurer => "Treasurer",
            SchoolOfTheologyAdmissionsOffice => "School of Theology Admissions Office",
            Transmitter => "Transmitter",
            Tenant => "Tenant",
            PrimaryTaxpayer => "Primary Taxpayer",
            TechnicalMarketingRepresentative => "Technical Marketing Representative",
            PlatformMaintenanceManager => "Platform Maintenance Manager",
            UndergraduateAdmissionsOffice => "Undergraduate Admissions Office",
            UnionPresident => "Union President",
            Processor => "Processor",
            UltimateReceiver => "Ultimate Receiver",
            SchoolOfVeterinaryMedicineAdmissionsOffice => {
                "School of Veterinary Medicine Admissions Office"
            }
            VicePresident => "Vice President",
            Warehouse => "Warehouse",
            Witness => "Witness",
            TechnicalWriter => "Technical Writer",
            WaiverApplicationContact => "Waiver Application Contact",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<ContactFunctionCode> {
        {
            use ContactFunctionCode::*;
            match description {
                "Review/Repricing Contact" => Some(ReviewRepricingContact),
                "Preferred Provider Organization (PPO) Contact" => Some(Code1B),
                "Health Maintenance Organization (HMO) Contact" => Some(Code1C),
                "Third-party Administrator (TPA) Contact" => Some(Code1D),
                "New Business Processing" => Some(NewBusinessProcessing),
                "Physician" => Some(Physician),
                "Service Team" => Some(ServiceTeam),
                "Underwriter" => Some(Underwriter),
                "Local Contact" => Some(LocalContact),
                "Automated Clearinghouse (ACH) Contact" => Some(Code3A),
                "Claim Approver" => Some(ClaimApprover),
                "Applicant" => Some(Applicant),
                "Interviewer" => Some(Interviewer),
                "Owner Representative" => Some(OwnerRepresentative),
                "Appointment Scheduler" => Some(AppointmentScheduler),
                "Concurrent Employer Contact" => Some(ConcurrentEmployerContact),
                "Authorized Representative" => Some(AuthorizedRepresentative),
                "Appraiser" => Some(Appraiser),
                "Administrative Contracting Officer" => {
                    Some(AdministrativeContractingOfficer)
                }
                "Accounting Department" => Some(AccountingDepartment),
                "Corporate Purchasing Agent" => Some(CorporatePurchasingAgent),
                "Authorized Financial Contact" => Some(AuthorizedFinancialContact),
                "Agent" => Some(Agent),
                "After-Hours Contact" => Some(AfterHoursContact),
                "Additional Insurance Information Contact" => {
                    Some(AdditionalInsuranceInformationContact)
                }
                "Primary Contact" => Some(PrimaryContact),
                "Alternate Contact" => Some(AlternateContact),
                "Administrator" => Some(Administrator),
                "Attention-to Party" => Some(AttentionToParty),
                "Accounts Payable Department" => Some(AccountsPayableDepartment),
                "Accounts Receivable Department" => Some(AccountsReceivableDepartment),
                "Authorized Signature" => Some(AuthorizedSignature),
                "Material Safety Data Sheet Contact" => {
                    Some(MaterialSafetyDataSheetContact)
                }
                "Report Authorizer" => Some(ReportAuthorizer),
                "Advisor" => Some(Advisor),
                "Co-investigator" => Some(CoInvestigator),
                "Additional Contact Points at Institutions" => {
                    Some(AdditionalContactPointsAtInstitutions)
                }
                "Broker" => Some(Broker),
                "Local Purchasing Agent" => Some(LocalPurchasingAgent),
                "Broker Contact" => Some(BrokerContact),
                "Buyer Name or Department" => Some(BuyerNameOrDepartment),
                "Bill Inquiry Contact" => Some(BillInquiryContact),
                "Operations" => Some(Operations),
                "Marketing Department" => Some(MarketingDepartment),
                "Technical Department" => Some(TechnicalDepartment),
                "Work Broker Maintenance Manager" => Some(WorkBrokerMaintenanceManager),
                "School Principal" => Some(SchoolPrincipal),
                "Board Staff" => Some(BoardStaff),
                "Business Unit Manager" => Some(BusinessUnitManager),
                "Carrier Contact" => Some(CarrierContact),
                "Co-borrower" => Some(CoBorrower),
                "Customer Contact Granting Appointment" => {
                    Some(CustomerContactGrantingAppointment)
                }
                "Changed By" => Some(ChangedBy),
                "Computer Systems Contact" => Some(ComputerSystemsContact),
                "Contract Contact" => Some(ContractContact),
                "Certifier" => Some(Certifier),
                "Customer Engineer" => Some(CustomerEngineer),
                "Chief Executive Officer" => Some(ChiefExecutiveOfficer),
                "Change Order Approver" => Some(ChangeOrderApprover),
                "Chief Financial Officer" => Some(ChiefFinancialOfficer),
                "Chief Information Officer" => Some(ChiefInformationOfficer),
                "Chairman of the Board" => Some(ChairmanOfTheBoard),
                "Chief Operating Officer" => Some(ChiefOperatingOfficer),
                "Container Manager" => Some(ContainerManager),
                "General Contact" => Some(GeneralContact),
                "Component Engineer" => Some(ComponentEngineer),
                "Cost and Schedule Coordinator" => Some(CostAndScheduleCoordinator),
                "Customer Relations" => Some(CustomerRelations),
                "CAD/CAM Specialist" => Some(CadCamSpecialist),
                "Claimant" => Some(Claimant),
                "Auditing Contact" => Some(AuditingContact),
                "Clearinghouse Contact" => Some(ClearinghouseContact),
                "Confirmed With" => Some(ConfirmedWith),
                "Payers Claim Office" => Some(PayersClaimOffice),
                "Case Manager" => Some(CaseManager),
                "Claim Recipient" => Some(ClaimRecipient),
                "Directory Advertising Contact" => Some(DirectoryAdvertisingContact),
                "Delivery Contact" => Some(DeliveryContact),
                "Division Director" => Some(DivisionDirector),
                "Design Engineer" => Some(DesignEngineer),
                "Director" => Some(Director),
                "Delivery Instructions Contact" => Some(DeliveryInstructionsContact),
                "Division Manager" => Some(DivisionManager),
                "Dental School Admissions Office" => Some(DentalSchoolAdmissionsOffice),
                "Development" => Some(Development),
                "Estimator" => Some(Estimator),
                "Evening Programs Office" => Some(EveningProgramsOffice),
                "EDI Coordinator" => Some(EdiCoordinator),
                "Entered By" => Some(EnteredBy),
                "Emergency Contact-Shipper" => Some(EmergencyContactShipper),
                "Emergency Contact-Consignee" => Some(EmergencyContactConsignee),
                "Emergency Contact-Military Traffic Management Command (MTMC)" => {
                    Some(CodeEF)
                }
                "Engineering" => Some(Engineering),
                "Emergency Contact" => Some(EmergencyContact),
                "Engineer" => Some(Engineer),
                "Executive Officer" => Some(ExecutiveOfficer),
                "Employer Contact" => Some(EmployerContact),
                "Electronic Submission Recipient" => Some(ElectronicSubmissionRecipient),
                "Executive Vice-President" => Some(ExecutiveVicePresident),
                "Expeditor" => Some(Expeditor),
                "Financial Aid Office" => Some(FinancialAidOffice),
                "Coordinator" => Some(Coordinator),
                "Forwarder Contact" => Some(ForwarderContact),
                "Primary Control Point" => Some(PrimaryControlPoint),
                "Licensee" => Some(Licensee),
                "Foreclosing Lender Administrative Contact" => {
                    Some(ForeclosingLenderAdministrativeContact)
                }
                "Functional Manager" => Some(FunctionalManager),
                "Joint Work Agent" => Some(JointWorkAgent),
                "Office Manager" => Some(OfficeManager),
                "Marketing Director" => Some(MarketingDirector),
                "Staff" => Some(Staff),
                "Compliance Officer" => Some(ComplianceOfficer),
                "Graduate Fine Arts Office" => Some(GraduateFineArtsOffice),
                "Graduate Business Office" => Some(GraduateBusinessOffice),
                "Guidance Counselor" => Some(GuidanceCounselor),
                "Graduate Engineering Office" => Some(GraduateEngineeringOffice),
                "Graduate Admissions Office" => Some(GraduateAdmissionsOffice),
                "Hazardous Material Contact" => Some(HazardousMaterialContact),
                "Human Resources" => Some(HumanResources),
                "Information Contact" => Some(InformationContact),
                "Issuing Officer" => Some(IssuingOfficer),
                "Insured Party" => Some(InsuredParty),
                "Law Firm" => Some(LawFirm),
                "Authorized Negotiator" => Some(AuthorizedNegotiator),
                "Preaward Survey Manager" => Some(PreawardSurveyManager),
                "Accepting Official" => Some(AcceptingOfficial),
                "Attorney" => Some(Attorney),
                "Clerk of Court" => Some(ClerkOfCourt),
                "Law School Admissions Office" => Some(LawSchoolAdmissionsOffice),
                "Logistics Contact" => Some(LogisticsContact),
                "Maintenance Contact" => Some(MaintenanceContact),
                "Mayor" => Some(Mayor),
                "Medical Contact" => Some(MedicalContact),
                "Medical Admissions Office" => Some(MedicalAdmissionsOffice),
                "Manufacturing" => Some(Manufacturing),
                "Manager" => Some(Manager),
                "Multiple Listing Service Staff" => Some(MultipleListingServiceStaff),
                "Multiple Listing Service Vendor" => Some(MultipleListingServiceVendor),
                "Customer Maintenance Manager" => Some(CustomerMaintenanceManager),
                "National Agent" => Some(NationalAgent),
                "Numerical Control Engineer" => Some(NumericalControlEngineer),
                "Notary Public" => Some(NotaryPublic),
                "Notification Contact" => Some(NotificationContact),
                "Other Adult" => Some(OtherAdult),
                "Order Contact" => Some(OrderContact),
                "Order Department" => Some(OrderDepartment),
                "Office Staff" => Some(OfficeStaff),
                "Owner" => Some(Owner),
                "President" => Some(President),
                "Plant Manager" => Some(PlantManager),
                "Purchasing Contracting Officer (PCO)" => Some(CodePC),
                "Project Director" => Some(ProjectDirector),
                "Process Engineer" => Some(ProcessEngineer),
                "Price Administration" => Some(PriceAdministration),
                "Program Director" => Some(ProgramDirector),
                "Provider" => Some(Provider),
                "Preparer" => Some(Preparer),
                "Project Manager" => Some(ProjectManager),
                "Performance Evaluation Committee" => {
                    Some(PerformanceEvaluationCommittee)
                }
                "Manufacturing Plant Contact" => Some(ManufacturingPlantContact),
                "Product Manager" => Some(ProductManager),
                "Probation or Legal Officer" => Some(ProbationOrLegalOfficer),
                "Production Representative" => Some(ProductionRepresentative),
                "Program Manager" => Some(ProgramManager),
                "Parent or Guardian" => Some(ParentOrGuardian),
                "Prototype Coordinator" => Some(PrototypeCoordinator),
                "Personnel Department" => Some(PersonnelDepartment),
                "Partner" => Some(Partner),
                "Report Preparer" => Some(ReportPreparer),
                "Participating Laboratory Contact" => {
                    Some(ParticipatingLaboratoryContact)
                }
                "Principal Study Contact or Author" => {
                    Some(PrincipalStudyContactOrAuthor)
                }
                "Purchase Service Provider" => Some(PurchaseServiceProvider),
                "Packager" => Some(Packager),
                "Patient" => Some(Patient),
                "Quality Assurance Contact" => Some(QualityAssuranceContact),
                "Quality Coordinator" => Some(QualityCoordinator),
                "Quality Inspector" => Some(QualityInspector),
                "Quality Manager" => Some(QualityManager),
                "Quoting Party" => Some(QuotingParty),
                "Ordering Officer" => Some(OrderingOfficer),
                "Port Engineer" => Some(PortEngineer),
                "Rental Company Administrative Contact" => {
                    Some(RentalCompanyAdministrativeContact)
                }
                "Real Estate Property Occupant" => Some(RealEstatePropertyOccupant),
                "Rebate/Chargeback Contact" => Some(RebateChargebackContact),
                "Receiving Dock" => Some(ReceivingDock),
                "Receiving Contact" => Some(ReceivingContact),
                "Real Estate Property Key Holder" => Some(RealEstatePropertyKeyHolder),
                "Registrar" => Some(Registrar),
                "Responsible Person" => Some(ResponsiblePerson),
                "Requestor" => Some(Requestor),
                "Rate Supervisor or Clerk" => Some(RateSupervisorOrClerk),
                "Respondant" => Some(Respondant),
                "Sales Administration" => Some(SalesAdministration),
                "Student" => Some(Student),
                "Schedule Contact" => Some(ScheduleContact),
                "Shipping Department" => Some(ShippingDepartment),
                "Service Organization" => Some(ServiceOrganization),
                "Student in Absentia" => Some(StudentInAbsentia),
                "Secretary" => Some(Secretary),
                "Shipper Contact" => Some(ShipperContact),
                "Investigator" => Some(Investigator),
                "Spouse" => Some(Spouse),
                "School Clerk" => Some(SchoolClerk),
                "Collector" => Some(Collector),
                "Submitting Contact" => Some(SubmittingContact),
                "Study Submitter Contact" => Some(StudySubmitterContact),
                "Service Order Writer" => Some(ServiceOrderWriter),
                "Special Program Contact" => Some(SpecialProgramContact),
                "Systems Administrator" => Some(SystemsAdministrator),
                "Sales Representative or Department" => {
                    Some(SalesRepresentativeOrDepartment)
                }
                "Supervisor" => Some(Supervisor),
                "Service Technician" => Some(ServiceTechnician),
                "Supplier Contact" => Some(SupplierContact),
                "Service Manager" => Some(ServiceManager),
                "Social Services Worker" => Some(SocialServicesWorker),
                "Secondary Taxpayer" => Some(SecondaryTaxpayer),
                "Traffic Administrator" => Some(TrafficAdministrator),
                "Telephone Answering Service Contact" => {
                    Some(TelephoneAnsweringServiceContact)
                }
                "College of Education Admissions Office" => {
                    Some(CollegeOfEducationAdmissionsOffice)
                }
                "Tender Developer" => Some(TenderDeveloper),
                "Treasurer" => Some(Treasurer),
                "School of Theology Admissions Office" => {
                    Some(SchoolOfTheologyAdmissionsOffice)
                }
                "Transmitter" => Some(Transmitter),
                "Tenant" => Some(Tenant),
                "Primary Taxpayer" => Some(PrimaryTaxpayer),
                "Technical Marketing Representative" => {
                    Some(TechnicalMarketingRepresentative)
                }
                "Platform Maintenance Manager" => Some(PlatformMaintenanceManager),
                "Undergraduate Admissions Office" => Some(UndergraduateAdmissionsOffice),
                "Union President" => Some(UnionPresident),
                "Processor" => Some(Processor),
                "Ultimate Receiver" => Some(UltimateReceiver),
                "School of Veterinary Medicine Admissions Office" => {
                    Some(SchoolOfVeterinaryMedicineAdmissionsOffice)
                }
                "Vice President" => Some(VicePresident),
                "Warehouse" => Some(Warehouse),
                "Witness" => Some(Witness),
                "Technical Writer" => Some(TechnicalWriter),
                "Waiver Application Contact" => Some(WaiverApplicationContact),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for ContactFunctionCode {
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
    type Value = ContactFunctionCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Contact Function Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ContactFunctionCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Contact Function Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ContactFunctionCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Contact Function Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ContactFunctionCode {
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