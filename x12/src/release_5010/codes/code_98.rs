use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**98

See docs at <https://www.stedi.com/edi/x12/element/98>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityIdentifierCode {
    ///00 - Alternate Insurer
    AlternateInsurer,
    ///0A - Comparable Rentals
    ComparableRentals,
    ///0B - Interim Funding Organization
    InterimFundingOrganization,
    ///0D - Non-occupant Co-borrower
    NonOccupantCoBorrower,
    ///0E - List Owner
    ListOwner,
    ///0F - List Mailer
    ListMailer,
    ///0G - Primary Electronic Business Contact
    PrimaryElectronicBusinessContact,
    ///0H - State Division
    StateDivision,
    ///0I - Alternate Electronic Business Contact
    AlternateElectronicBusinessContact,
    ///0J - Primary Practice Location
    PrimaryPracticeLocation,
    ///0P - Party to Declare Goods
    PartyToDeclareGoods,
    ///01 - Loan Applicant
    LoanApplicant,
    ///001 - Pumper
    Pumper,
    ///1A - Subgroup
    Subgroup,
    ///1B - Applicant
    Applicant,
    ///1C - Group Purchasing Organization (GPO)
    Code1C,
    ///1D - Co-operative
    CoOperative,
    ///1E - Health Maintenance Organization (HMO)
    Code1E,
    ///1F - Alliance
    Alliance,
    ///1G - Oncology Center
    OncologyCenter,
    ///1H - Kidney Dialysis Unit
    KidneyDialysisUnit,
    ///1I - Preferred Provider Organization (PPO)
    Code1I,
    ///1J - Connection
    Connection,
    ///1K - Franchisor
    Franchisor,
    ///1L - Franchisee
    Franchisee,
    ///1M - Previous Group
    PreviousGroup,
    ///1N - Shareholder
    Shareholder,
    ///1O - Acute Care Hospital
    AcuteCareHospital,
    ///1P - Provider
    Provider,
    ///1Q - Military Facility
    MilitaryFacility,
    ///1R - University, College or School
    Code1R,
    ///1S - Outpatient Surgicenter
    OutpatientSurgicenter,
    ///1T - Physician, Clinic or Group Practice
    Code1T,
    ///1U - Long Term Care Facility
    LongTermCareFacility,
    ///1V - Extended Care Facility
    ExtendedCareFacility,
    ///1W - Psychiatric Health Facility
    PsychiatricHealthFacility,
    ///1X - Laboratory
    Laboratory,
    ///1Y - Retail Pharmacy
    RetailPharmacy,
    ///1Z - Home Health Care
    HomeHealthCare,
    ///02 - Loan Broker
    LoanBroker,
    ///002 - Surface Management Entity
    SurfaceManagementEntity,
    ///2A - Federal, State, County or City Facility
    Code2A,
    ///2B - Third-Party Administrator
    ThirdPartyAdministrator,
    ///2C - Co-Participant
    CoParticipant,
    ///2D - Miscellaneous Health Care Facility
    MiscellaneousHealthCareFacility,
    ///2E - Non-Health Care Miscellaneous Facility
    NonHealthCareMiscellaneousFacility,
    ///2F - State
    State,
    ///2G - Assigner
    Assigner,
    ///2H - Hospital District or Authority
    HospitalDistrictOrAuthority,
    ///2I - Church Operated Facility
    ChurchOperatedFacility,
    ///2J - Individual
    Individual,
    ///2K - Partnership
    Partnership,
    ///2L - Corporation
    Corporation,
    ///2M - Air Force Facility
    AirForceFacility,
    ///2N - Army Facility
    ArmyFacility,
    ///2O - Navy Facility
    NavyFacility,
    ///2P - Public Health Service Facility
    PublicHealthServiceFacility,
    ///2Q - Veterans Administration Facility
    VeteransAdministrationFacility,
    ///2R - Federal Facility
    FederalFacility,
    ///2S - Public Health Service Indian Service Facility
    PublicHealthServiceIndianServiceFacility,
    ///2T - Department of Justice Facility
    DepartmentOfJusticeFacility,
    ///2U - Other Not-for-profit Facility
    OtherNotForProfitFacility,
    ///2V - Individual for-profit Facility
    IndividualForProfitFacility,
    ///2W - Partnership for-profit Facility
    PartnershipForProfitFacility,
    ///2X - Corporation for-profit Facility
    CorporationForProfitFacility,
    ///2Y - General Medical and Surgical Facility
    GeneralMedicalAndSurgicalFacility,
    ///2Z - Hospital Unit of an Institution (prison hospital, college infirmary, etc.)
    Code2Z,
    ///03 - Dependent
    Dependent,
    ///003 - Application Party
    ApplicationParty,
    ///3A - Hospital Unit Within an Institution for the Mentally Retarded
    HospitalUnitWithinAnInstitutionForTheMentallyRetarded,
    ///3B - Psychiatric Facility
    PsychiatricFacility,
    ///3C - Tuberculosis and Other Respiratory Diseases Facility
    TuberculosisAndOtherRespiratoryDiseasesFacility,
    ///3D - Obstetrics and Gynecology Facility
    ObstetricsAndGynecologyFacility,
    ///3E - Eye, Ear, Nose and Throat Facility
    Code3E,
    ///3F - Rehabilitation Facility
    RehabilitationFacility,
    ///3G - Orthopedic Facility
    OrthopedicFacility,
    ///3H - Chronic Disease Facility
    ChronicDiseaseFacility,
    ///3I - Other Specialty Facility
    OtherSpecialtyFacility,
    ///3J - Children's General Facility
    ChildrensGeneralFacility,
    ///3K - Children's Hospital Unit of an Institution
    ChildrensHospitalUnitOfAnInstitution,
    ///3L - Children's Psychiatric Facility
    ChildrensPsychiatricFacility,
    ///3M - Children's Tuberculosis and Other Respiratory Diseases Facility
    ChildrensTuberculosisAndOtherRespiratoryDiseasesFacility,
    ///3N - Children's Eye, Ear, Nose and Throat Facility
    Code3N,
    ///3O - Children's Rehabilitation Facility
    ChildrensRehabilitationFacility,
    ///3P - Children's Orthopedic Facility
    ChildrensOrthopedicFacility,
    ///3Q - Children's Chronic Disease Facility
    ChildrensChronicDiseaseFacility,
    ///3R - Children's Other Specialty Facility
    ChildrensOtherSpecialtyFacility,
    ///3S - Institution for Mental Retardation
    InstitutionForMentalRetardation,
    ///3T - Alcoholism and Other Chemical Dependency Facility
    AlcoholismAndOtherChemicalDependencyFacility,
    ///3U - General Inpatient Care for AIDS/ARC Facility
    GeneralInpatientCareForAidsArcFacility,
    ///3V - AIDS/ARC Unit
    AidsArcUnit,
    ///3W - Specialized Outpatient Program for AIDS/ARC
    SpecializedOutpatientProgramForAidsArc,
    ///3X - Alcohol/Drug Abuse or Dependency Inpatient Unit
    AlcoholDrugAbuseOrDependencyInpatientUnit,
    ///3Y - Alcohol/Drug Abuse or Dependency Outpatient Services
    AlcoholDrugAbuseOrDependencyOutpatientServices,
    ///3Z - Arthritis Treatment Center
    ArthritisTreatmentCenter,
    ///04 - Asset Account Holder
    AssetAccountHolder,
    ///004 - Site Operator
    SiteOperator,
    ///4A - Birthing Room/LDRP Room
    BirthingRoomLdrpRoom,
    ///4B - Burn Care Unit
    BurnCareUnit,
    ///4C - Cardiac Catherization Laboratory
    CardiacCatherizationLaboratory,
    ///4D - Open-Heart Surgery Facility
    OpenHeartSurgeryFacility,
    ///4E - Cardiac Intensive Care Unit
    CardiacIntensiveCareUnit,
    ///4F - Angioplasty Facility
    AngioplastyFacility,
    ///4G - Chronic Obstructive Pulmonary Disease Service Facility
    ChronicObstructivePulmonaryDiseaseServiceFacility,
    ///4H - Emergency Department
    EmergencyDepartment,
    ///4I - Trauma Center (Certified)
    Code4I,
    ///4J - Extracorporeal Shock-Wave Lithotripter (ESWL) Unit
    Code4J,
    ///4K - Fitness Center
    FitnessCenter,
    ///4L - Genetic Counseling/Screening Services
    GeneticCounselingScreeningServices,
    ///4M - Adult Day Care Program Facility
    AdultDayCareProgramFacility,
    ///4N - Alzheimer's Diagnostic/Assessment Services
    AlzheimersDiagnosticAssessmentServices,
    ///4O - Comprehensive Geriatric Assessment Facility
    ComprehensiveGeriatricAssessmentFacility,
    ///4P - Emergency Response (Geriatric) Unit
    Code4P,
    ///4Q - Geriatric Acute Care Unit
    GeriatricAcuteCareUnit,
    ///4R - Geriatric Clinics
    GeriatricClinics,
    ///4S - Respite Care Facility
    RespiteCareFacility,
    ///4T - Senior Membership Program
    SeniorMembershipProgram,
    ///4U - Patient Education Unit
    PatientEducationUnit,
    ///4V - Community Health Promotion Facility
    CommunityHealthPromotionFacility,
    ///4W - Worksite Health Promotion Facility
    WorksiteHealthPromotionFacility,
    ///4X - Hemodialysis Facility
    HemodialysisFacility,
    ///4Y - Home Health Services
    HomeHealthServices,
    ///4Z - Hospice
    Hospice,
    ///05 - Tenant
    Tenant,
    ///005 - Construction Contractor
    ConstructionContractor,
    ///5A - Medical Surgical or Other Intensive Care Unit
    MedicalSurgicalOrOtherIntensiveCareUnit,
    ///5B - Hisopathology Laboratory
    HisopathologyLaboratory,
    ///5C - Blood Bank
    BloodBank,
    ///5D - Neonatal Intensive Care Unit
    NeonatalIntensiveCareUnit,
    ///5E - Obstetrics Unit
    ObstetricsUnit,
    ///5F - Occupational Health Services
    OccupationalHealthServices,
    ///5G - Organized Outpatient Services
    OrganizedOutpatientServices,
    ///5H - Pediatric Acute Inpatient Unit
    PediatricAcuteInpatientUnit,
    ///5I - Psychiatric Child/Adolescent Services
    PsychiatricChildAdolescentServices,
    ///5J - Psychiatric Consultation-Liaison Services
    PsychiatricConsultationLiaisonServices,
    ///5K - Psychiatric Education Services
    PsychiatricEducationServices,
    ///5L - Psychiatric Emergency Services
    PsychiatricEmergencyServices,
    ///5M - Psychiatric Geriatric Services
    PsychiatricGeriatricServices,
    ///5N - Psychiatric Inpatient Unit
    PsychiatricInpatientUnit,
    ///5O - Psychiatric Outpatient Services
    PsychiatricOutpatientServices,
    ///5P - Psychiatric Partial Hospitalization Program
    PsychiatricPartialHospitalizationProgram,
    ///5Q - Megavoltage Radiation Therapy Unit
    MegavoltageRadiationTherapyUnit,
    ///5R - Radioactive Implants Unit
    RadioactiveImplantsUnit,
    ///5S - Therapeutic Radioisotope Facility
    TherapeuticRadioisotopeFacility,
    ///5T - X-Ray Radiation Therapy Unit
    XRayRadiationTherapyUnit,
    ///5U - CT Scanner Unit
    CtScannerUnit,
    ///5V - Diagnostic Radioisotope Facility
    DiagnosticRadioisotopeFacility,
    ///5W - Magnetic Resonance Imaging (MRI) Facility
    Code5W,
    ///5X - Ultrasound Unit
    UltrasoundUnit,
    ///5Y - Rehabilitation Inpatient Unit
    RehabilitationInpatientUnit,
    ///5Z - Rehabilitation Outpatient Services
    RehabilitationOutpatientServices,
    ///06 - Recipient of Civil or Legal Liability Payment
    RecipientOfCivilOrLegalLiabilityPayment,
    ///006 - Drilling Contractor
    DrillingContractor,
    ///6A - Reproductive Health Services
    ReproductiveHealthServices,
    ///6B - Skilled Nursing or Other Long-Term Care Unit
    SkilledNursingOrOtherLongTermCareUnit,
    ///6C - Single Photon Emission Computerized Tomography (SPECT) Unit
    Code6C,
    ///6D - Organized Social Work Service Facility
    OrganizedSocialWorkServiceFacility,
    ///6E - Outpatient Social Work Services
    OutpatientSocialWorkServices,
    ///6F - Emergency Department Social Work Services
    EmergencyDepartmentSocialWorkServices,
    ///6G - Sports Medicine Clinic/Services
    SportsMedicineClinicServices,
    ///6H - Hospital Auxiliary Unit
    HospitalAuxiliaryUnit,
    ///6I - Patient Representative Services
    PatientRepresentativeServices,
    ///6J - Volunteer Services Department
    VolunteerServicesDepartment,
    ///6K - Outpatient Surgery Services
    OutpatientSurgeryServices,
    ///6L - Organ/Tissue Transplant Unit
    OrganTissueTransplantUnit,
    ///6M - Orthopedic Surgery Facility
    OrthopedicSurgeryFacility,
    ///6N - Occupational Therapy Services
    OccupationalTherapyServices,
    ///6O - Physical Therapy Services
    PhysicalTherapyServices,
    ///6P - Recreational Therapy Services
    RecreationalTherapyServices,
    ///6Q - Respiratory Therapy Services
    RespiratoryTherapyServices,
    ///6R - Speech Therapy Services
    SpeechTherapyServices,
    ///6S - Women's Health Center/Services
    WomensHealthCenterServices,
    ///6T - Health Sciences Library
    HealthSciencesLibrary,
    ///6U - Cardiac Rehabilitation Program Facility
    CardiacRehabilitationProgramFacility,
    ///6V - Non-Invasive Cardiac Assessment Services
    NonInvasiveCardiacAssessmentServices,
    ///6W - Emergency Medical Technician
    EmergencyMedicalTechnician,
    ///6X - Disciplinary Contact
    DisciplinaryContact,
    ///6Y - Case Manager
    CaseManager,
    ///6Z - Advisor
    Advisor,
    ///07 - Titleholder
    Titleholder,
    ///007 - Spud Contractor
    SpudContractor,
    ///7A - Premises
    Premises,
    ///7B - Bottler
    Bottler,
    ///7C - Place of Occurrence
    PlaceOfOccurrence,
    ///7D - Contracting Officer Representative
    ContractingOfficerRepresentative,
    ///7E - Party Authorized to Definitize Contract Action
    PartyAuthorizedToDefinitizeContractAction,
    ///7F - Filing Address
    FilingAddress,
    ///7G - Hazardous Material Office
    HazardousMaterialOffice,
    ///7H - Government Furnished Property FOB Point
    GovernmentFurnishedPropertyFobPoint,
    ///7I - Project Name
    ProjectName,
    ///7J - Codefendant
    Codefendant,
    ///7K - Co-occupant
    CoOccupant,
    ///7L - Preliminary Inspection Location
    PreliminaryInspectionLocation,
    ///7M - Inspection and Acceptance Location
    InspectionAndAcceptanceLocation,
    ///7N - Party to Receive Proposal
    PartyToReceiveProposal,
    ///7O - Federally Chartered Facility
    FederallyCharteredFacility,
    ///7P - Transportation Office
    TransportationOffice,
    ///7Q - Party to Whom Protest Submitted
    PartyToWhomProtestSubmitted,
    ///7R - Birthplace
    Birthplace,
    ///7S - Pipeline Segment
    PipelineSegment,
    ///7T - Home State Name
    HomeStateName,
    ///7U - Liquidator
    Liquidator,
    ///7V - Petitioning Creditor's Attorney
    PetitioningCreditorsAttorney,
    ///7W - Merged Name
    MergedName,
    ///7X - Party Represented
    PartyRepresented,
    ///7Y - Professional Organization
    ProfessionalOrganization,
    ///7Z - Referee
    Referee,
    ///08 - Non-Mortgage Liability Account Holder
    NonMortgageLiabilityAccountHolder,
    ///008 - Lien Holder
    LienHolder,
    ///8A - Vacation Home
    VacationHome,
    ///8B - Primary Residence
    PrimaryResidence,
    ///8C - Second Home
    SecondHome,
    ///8D - Permit Holder
    PermitHolder,
    ///8E - Minority Institution
    MinorityInstitution,
    ///8F - Bailment Warehouse
    BailmentWarehouse,
    ///8G - First Appraiser
    FirstAppraiser,
    ///8H - Tax Exempt Organization
    TaxExemptOrganization,
    ///8I - Service Organization
    ServiceOrganization,
    ///8J - Emerging Small Business
    EmergingSmallBusiness,
    ///8K - Surplus Dealer
    SurplusDealer,
    ///8L - Polling Site
    PollingSite,
    ///8M - Socially Disadvantaged Individual
    SociallyDisadvantagedIndividual,
    ///8N - Economically Disadvantaged Individual
    EconomicallyDisadvantagedIndividual,
    ///8O - Disabled Individual
    DisabledIndividual,
    ///8P - Producer
    Producer,
    ///8Q - Public or Private Organization for the Disabled
    PublicOrPrivateOrganizationForTheDisabled,
    ///8R - Consumer Service Provider (CSP) Customer
    Code8R,
    ///8S - Consumer Service Provider (CSP)
    Code8S,
    ///8T - Voter
    Voter,
    ///8U - Native Hawaiian Organization
    NativeHawaiianOrganization,
    ///8V - Primary Intra-LATA (Local Access Transport Area) Carrier
    Code8V,
    ///8W - Payment Address
    PaymentAddress,
    ///8X - Oil and Gas Custodian
    OilAndGasCustodian,
    ///8Y - Registered Office
    RegisteredOffice,
    ///09 - Note Co-Signer
    NoteCoSigner,
    ///9A - Debtor Individual
    DebtorIndividual,
    ///9B - Country of Export
    CountryOfExport,
    ///9C - Country of Destination
    CountryOfDestination,
    ///9D - New Service Provider
    NewServiceProvider,
    ///9E - Sub-servicer
    SubServicer,
    ///9F - Loss Payee
    LossPayee,
    ///9G - Nickname
    Nickname,
    ///9H - Assignee
    Assignee,
    ///9I - Registered Principal
    RegisteredPrincipal,
    ///9J - Additional Debtor
    AdditionalDebtor,
    ///9K - Key Person
    KeyPerson,
    ///9L - Incorporated By
    IncorporatedBy,
    ///9N - Party to Lease
    PartyToLease,
    ///9O - Party to Contract
    PartyToContract,
    ///9P - Investigator
    Investigator,
    ///9Q - Last Supplier
    LastSupplier,
    ///9R - Downstream First Supplier
    DownstreamFirstSupplier,
    ///9S - Co-Investigator
    CoInvestigator,
    ///9T - Telephone Answering Service Bureau
    TelephoneAnsweringServiceBureau,
    ///9U - Author
    Author,
    ///9V - First Supplier
    FirstSupplier,
    ///9W - Ultimate Parent Company
    UltimateParentCompany,
    ///9X - Contractual Receipt Meter
    ContractualReceiptMeter,
    ///9Y - Contractual Delivery Meter
    ContractualDeliveryMeter,
    ///9Z - Co-debtor
    CoDebtor,
    ///10 - Conduit
    Conduit,
    ///11 - Party to be billed(AAR Accounting Rule 11)
    Code11,
    ///12 - Regional Office
    RegionalOffice,
    ///13 - Contracted Service Provider
    ContractedServiceProvider,
    ///14 - Wholly-Owned Subsidiary
    WhollyOwnedSubsidiary,
    ///15 - Accounts Payable Office
    AccountsPayableOffice,
    ///16 - Plant
    Plant,
    ///17 - Consultant's Office
    ConsultantsOffice,
    ///18 - Production
    Production,
    ///19 - Non-Production Supplier
    NonProductionSupplier,
    ///20 - Foreign Supplier
    ForeignSupplier,
    ///21 - Small Business
    SmallBusiness,
    ///22 - Minority-Owned Business, Small
    Code22,
    ///23 - Minority-Owned Business, Large
    Code23,
    ///24 - Woman-Owned Business, Small
    Code24,
    ///25 - Woman-Owned Business, Large
    Code25,
    ///26 - Socially Disadvantaged Business
    SociallyDisadvantagedBusiness,
    ///27 - Small Disadvantaged Business
    SmallDisadvantagedBusiness,
    ///28 - Subcontractor
    Subcontractor,
    ///29 - Prototype Supplier
    PrototypeSupplier,
    ///30 - Service Supplier
    ServiceSupplier,
    ///31 - Postal Mailing Address
    PostalMailingAddress,
    ///32 - Party to Receive Material Release
    PartyToReceiveMaterialRelease,
    ///33 - Inquiry Address
    InquiryAddress,
    ///34 - Material Change Notice Address
    MaterialChangeNoticeAddress,
    ///35 - Electronic Data Interchange (EDI) Coordinator Point Address
    Code35,
    ///36 - Employer
    Employer,
    ///37 - Previous Debt Holder
    PreviousDebtHolder,
    ///38 - Mortgage Liability Account Holder
    MortgageLiabilityAccountHolder,
    ///39 - Appraisal Company
    AppraisalCompany,
    ///40 - Receiver
    Receiver,
    ///41 - Submitter
    Submitter,
    ///42 - Component Manufacturer
    ComponentManufacturer,
    ///43 - Claimant Authorized Representative
    ClaimantAuthorizedRepresentative,
    ///44 - Data Processing Service Bureau
    DataProcessingServiceBureau,
    ///45 - Drop-off Location
    DropOffLocation,
    ///46 - Invoicing Dealer
    InvoicingDealer,
    ///47 - Estimator
    Estimator,
    ///48 - In-service Source
    InServiceSource,
    ///49 - Initial Dealer
    InitialDealer,
    ///50 - Manufacturer's Representative
    ManufacturersRepresentative,
    ///51 - Parts Distributor
    PartsDistributor,
    ///52 - Part Remanufacturer
    PartRemanufacturer,
    ///53 - Registered Owner
    RegisteredOwner,
    ///54 - Order Writer
    OrderWriter,
    ///55 - Service Manager
    ServiceManager,
    ///56 - Servicing Dealer
    ServicingDealer,
    ///57 - Servicing Organization
    ServicingOrganization,
    ///58 - Store Manager
    StoreManager,
    ///59 - Party to Approve Specification
    PartyToApproveSpecification,
    ///60 - Salesperson
    Salesperson,
    ///61 - Performed At
    PerformedAt,
    ///62 - Applicant's Employer
    ApplicantsEmployer,
    ///63 - Reference's Employer
    ReferencesEmployer,
    ///64 - Cosigner's Employer
    CosignersEmployer,
    ///65 - Applicant's Reference
    ApplicantsReference,
    ///66 - Applicant's Cosigner
    ApplicantsCosigner,
    ///67 - Applicant's Comaker
    ApplicantsComaker,
    ///68 - Owner's Representative
    OwnersRepresentative,
    ///69 - Repairing Outlet
    RepairingOutlet,
    ///70 - Prior Incorrect Insured
    PriorIncorrectInsured,
    ///71 - Attending Physician
    AttendingPhysician,
    ///72 - Operating Physician
    OperatingPhysician,
    ///73 - Other Physician
    OtherPhysician,
    ///74 - Corrected Insured
    CorrectedInsured,
    ///75 - Participant
    Participant,
    ///76 - Secondary Warranter
    SecondaryWarranter,
    ///77 - Service Location
    ServiceLocation,
    ///78 - Service Requester
    ServiceRequester,
    ///79 - Warranter
    Warranter,
    ///80 - Hospital
    Hospital,
    ///81 - Part Source
    PartSource,
    ///82 - Rendering Provider
    RenderingProvider,
    ///83 - Subscriber's School
    SubscribersSchool,
    ///84 - Subscriber's Employer
    SubscribersEmployer,
    ///85 - Billing Provider
    BillingProvider,
    ///86 - Conductor
    Conductor,
    ///87 - Pay-to Provider
    PayToProvider,
    ///88 - Approver
    Approver,
    ///89 - Investor
    Investor,
    ///90 - Previous Business Partner
    PreviousBusinessPartner,
    ///91 - Action Party
    ActionParty,
    ///92 - Support Party
    SupportParty,
    ///93 - Insurance Institute
    InsuranceInstitute,
    ///94 - New Supply Source
    NewSupplySource,
    ///95 - Research Institute
    ResearchInstitute,
    ///96 - Debtor Company
    DebtorCompany,
    ///97 - Party Waiving Requirements
    PartyWaivingRequirements,
    ///98 - Freight Management Facilitator
    FreightManagementFacilitator,
    ///99 - Outer Continental Shelf (OCS) Area Location
    Code99,
    ///A1 - Adjuster
    Adjuster,
    ///A2 - Woman-Owned Business
    WomanOwnedBusiness,
    ///A3 - Labor Surplus Area Firm
    LaborSurplusAreaFirm,
    ///A4 - Other Disadvantaged Business
    OtherDisadvantagedBusiness,
    ///A5 - Veteran-Owned Business
    VeteranOwnedBusiness,
    ///A6 - Section 8(a) Program Participant Firm
    CodeA6,
    ///A7 - Sheltered Workshop
    ShelteredWorkshop,
    ///A8 - Nonprofit Institution
    NonprofitInstitution,
    ///A9 - Sales Office
    SalesOffice,
    ///AA - Authority For Shipment
    AuthorityForShipment,
    ///AA1 - Chief Executive Officer (CEO)
    CodeAA1,
    ///AA2 - Financial Aid Office
    FinancialAidOffice,
    ///AA3 - Respondent
    Respondent,
    ///AA4 - Admission Office
    AdmissionOffice,
    ///AA5 - Multi-Campus Administrative Unit
    MultiCampusAdministrativeUnit,
    ///AA6 - Headmaster
    Headmaster,
    ///AA7 - Business Officer
    BusinessOfficer,
    ///AA8 - Superintendent
    Superintendent,
    ///AA9 - School Principal
    SchoolPrincipal,
    ///AAA - Sub-account
    SubAccount,
    ///AAB - Management Non-Officer
    ManagementNonOfficer,
    ///AAC - Incorporated Location
    IncorporatedLocation,
    ///AAD - Name not to be Confused with
    NameNotToBeConfusedWith,
    ///AAE - Lot
    Lot,
    ///AAF - Previous Occupant
    PreviousOccupant,
    ///AAG - Ground Ambulance Services
    GroundAmbulanceServices,
    ///AAH - Air Ambulance Services
    AirAmbulanceServices,
    ///AAI - Water Ambulance Services
    WaterAmbulanceServices,
    ///AAJ - Admitting Services
    AdmittingServices,
    ///AAK - Primary Surgeon
    PrimarySurgeon,
    ///AAL - Medical Nurse
    MedicalNurse,
    ///AAM - Cardiac Rehabilitation Services
    CardiacRehabilitationServices,
    ///AAN - Skilled Nursing Services
    SkilledNursingServices,
    ///AAO - Observation Room Services
    ObservationRoomServices,
    ///AAP - Employee
    Employee,
    ///AAQ - Anesthesiology Services
    AnesthesiologyServices,
    ///AAS - Prior Base Jurisdiction
    PriorBaseJurisdiction,
    ///AAT - Incorporation Jurisdiction
    IncorporationJurisdiction,
    ///AAU - Marker Owner
    MarkerOwner,
    ///AAV - Reclamation Center
    ReclamationCenter,
    ///AAW - Party Providing Financing
    PartyProvidingFinancing,
    ///AB - Additional Pickup Address
    AdditionalPickupAddress,
    ///AB1 - Private School System
    PrivateSchoolSystem,
    ///AB2 - State Operated School System
    StateOperatedSchoolSystem,
    ///AB3 - Vocational Regions School System
    VocationalRegionsSchoolSystem,
    ///AB4 - Chartered School District
    CharteredSchoolDistrict,
    ///AB5 - Schooling of Indian Children School System
    SchoolingOfIndianChildrenSchoolSystem,
    ///AB6 - Unorganized Territories School System
    UnorganizedTerritoriesSchoolSystem,
    ///AB7 - State Administered District
    StateAdministeredDistrict,
    ///AB8 - Towns in Unions School System
    TownsInUnionsSchoolSystem,
    ///AB9 - Agent Towns School System
    AgentTownsSchoolSystem,
    ///ABB - Master Property
    MasterProperty,
    ///ABC - Project Property
    ProjectProperty,
    ///ABD - Unit Property
    UnitProperty,
    ///ABE - Additional Address
    AdditionalAddress,
    ///ABF - Society of Property Information Compilers and Analysts
    SocietyOfPropertyInformationCompilersAndAnalysts,
    ///ABG - Organization
    Organization,
    ///ABH - Joint Owner Annuitant
    JointOwnerAnnuitant,
    ///ABI - Joint Annuitant Owner
    JointAnnuitantOwner,
    ///ABJ - Joint Owner Annuitant Payor
    JointOwnerAnnuitantPayor,
    ///ABK - Joint Owner Joint Annuitant
    JointOwnerJointAnnuitant,
    ///ABL - Joint Owner Joint Annuitant Payor
    JointOwnerJointAnnuitantPayor,
    ///ABM - Joint Owner Payor
    JointOwnerPayor,
    ///ABN - Acronym
    Acronym,
    ///ABO - New Address
    NewAddress,
    ///ABP - Chairperson
    Chairperson,
    ///ABQ - Decision Maker
    DecisionMaker,
    ///ABR - Former President
    FormerPresident,
    ///ABS - Founder
    Founder,
    ///ABT - Imported from Location
    ImportedFromLocation,
    ///ABU - Literally Translated Name
    LiterallyTranslatedName,
    ///ABV - Original Location
    OriginalLocation,
    ///ABW - President
    President,
    ///ABX - Rating Organization
    RatingOrganization,
    ///AC - Air Cargo Company
    AirCargoCompany,
    ///AC1 - Regional Center
    RegionalCenter,
    ///AC2 - Local Education Agency (LEA)
    CodeAC2,
    ///AC3 - State Education Agency
    StateEducationAgency,
    ///ACB - Initial Medical Provider
    InitialMedicalProvider,
    ///ACC - Concurrent Employer
    ConcurrentEmployer,
    ///ACE - Routing Point
    RoutingPoint,
    ///ACF - Border Crossing
    BorderCrossing,
    ///ACG - Bobtail Service Point
    BobtailServicePoint,
    ///ACH - Auditor
    Auditor,
    ///ACI - Insured Location
    InsuredLocation,
    ///ACJ - Referral Provider
    ReferralProvider,
    ///ACK - Affiliate
    Affiliate,
    ///ACL - Allied Health Professional
    AlliedHealthProfessional,
    ///ACM - Emergency Provider
    EmergencyProvider,
    ///ACN - Federal Government
    FederalGovernment,
    ///ACO - Fellowship Institution
    FellowshipInstitution,
    ///ACP - Government - Combined Control
    GovernmentCombinedControl,
    ///ACQ - Government - Federal - Military
    GovernmentFederalMilitary,
    ///ACR - Government - Federal - Other
    GovernmentFederalOther,
    ///ACS - Government - Federal - Veterans
    GovernmentFederalVeterans,
    ///ACT - Government - Local
    GovernmentLocal,
    ///ACU - Group Affiliation
    GroupAffiliation,
    ///ACV - Information Source
    InformationSource,
    ///ACW - Internship Entity
    InternshipEntity,
    ///ACX - Medical School
    MedicalSchool,
    ///ACY - National Organization
    NationalOrganization,
    /**ACZ - Non-Profit Health Care Provider
See U.S. Internal Revenue Code Chapter 1, Subchapter F, Part 1, Section 501(c)*/
    CodeACZ,
    ///AD - Party to be advised (Written orders)
    CodeAD,
    /**ADA - Not for Profit Health Care Provider
See U.S. Internal Revenue Code Chapter 1, Subchapter F, Part 1, Section 501(c)*/
    CodeADA,
    ///ADB - For Profit Health Care Provider
    ForProfitHealthCareProvider,
    ///ADC - Office Manager
    OfficeManager,
    ///ADD - On-call Provider
    OnCallProvider,
    ///ADE - Physician Hospital Organization (PHO)
    CodeADE,
    ///ADF - Point of Service (POS)
    CodeADF,
    ///ADH - Residency Institution
    ResidencyInstitution,
    ///ADJ - Shared Service
    SharedService,
    ///ADK - Supporting Personnel
    SupportingPersonnel,
    ///ADL - Training Institution
    TrainingInstitution,
    ///ADM - Public School
    PublicSchool,
    ///ADN - Private School
    PrivateSchool,
    ///ADO - Public Pre-K Education
    PublicPreKEducation,
    ///ADP - Private Pre-K Education
    PrivatePreKEducation,
    ///ADQ - Pre-K Day Care
    PreKDayCare,
    ///ADR - Charter School
    CharterSchool,
    ///ADS - Home School
    HomeSchool,
    ///ADT - Public Alternative School
    PublicAlternativeSchool,
    ///ADU - Neglected/Delinquent Institution
    NeglectedDelinquentInstitution,
    ///ADV - Post-Secondary Institution
    PostSecondaryInstitution,
    ///ADW - Food Service Operator
    FoodServiceOperator,
    ///ADX - Future Address
    FutureAddress,
    ///ADY - Former Registered Address
    FormerRegisteredAddress,
    ///ADZ - Top Parent Company in Same Country
    TopParentCompanyInSameCountry,
    ///AE - Additional Delivery Address
    AdditionalDeliveryAddress,
    ///AEA - Second Level Parent Company
    SecondLevelParentCompany,
    ///AEB - Airport Authority
    AirportAuthority,
    ///AEC - Council of Governments
    CouncilOfGovernments,
    ///AED - Foundation
    Foundation,
    ///AEE - Port Authority
    PortAuthority,
    ///AEF - Planning Commission
    PlanningCommission,
    ///AEG - Car Rental Location
    CarRentalLocation,
    ///AEI - Lodging Facility
    LodgingFacility,
    ///AEJ - Party to Receive Transportation Credit
    PartyToReceiveTransportationCredit,
    ///AEK - Party to Receive Packing, Crating, and Handling Credit
    CodeAEK,
    ///AEL - Primary International Telecom Carrier
    PrimaryInternationalTelecomCarrier,
    ///AF - Authorized Accepting Official
    AuthorizedAcceptingOfficial,
    ///AG - Agent/Agency
    AgentAgency,
    ///AH - Advertiser
    Advertiser,
    ///AHM - Agency Hazardous Material Information System Location
    AgencyHazardousMaterialInformationSystemLocation,
    ///AI - Airline
    Airline,
    ///AJ - Alleged Debtor
    AllegedDebtor,
    ///AK - Party to Whom Acknowledgment Should Be Sent
    PartyToWhomAcknowledgmentShouldBeSent,
    ///AL - Allotment Customer
    AllotmentCustomer,
    ///ALA - Alternative Addressee
    AlternativeAddressee,
    ///ALO - Activity Location
    ActivityLocation,
    ///AM - Assistant U.S. Trustee
    AssistantUSTrustee,
    ///AN - Authorized From
    AuthorizedFrom,
    ///AO - Account Of
    AccountOf,
    ///AP - Account of (Origin Party)
    CodeAP,
    ///APR - Activity Provider
    ActivityProvider,
    ///AQ - Account of (Destination Party)
    CodeAQ,
    ///AR - Armed Services Location Designation
    ArmedServicesLocationDesignation,
    ///AS - Postsecondary Education Sender
    PostsecondaryEducationSender,
    ///AT - Postsecondary Education Recipient
    PostsecondaryEducationRecipient,
    ///ATA - Alternate Tax Authority
    AlternateTaxAuthority,
    ///AU - Party Authorizing Disposition
    PartyAuthorizingDisposition,
    ///AUO - Authorizing Official
    AuthorizingOfficial,
    ///AV - Authorized To
    AuthorizedTo,
    ///AW - Accountant
    Accountant,
    ///AX - Plaintiff
    Plaintiff,
    ///AY - Clearinghouse
    Clearinghouse,
    ///AZ - Previous Name
    PreviousName,
    ///B1 - Construction Firm
    ConstructionFirm,
    ///B2 - Other Unlisted Type of Organizational Entity
    OtherUnlistedTypeOfOrganizationalEntity,
    ///B3 - Previous Name of Firm
    PreviousNameOfFirm,
    ///B4 - Parent Company
    ParentCompany,
    ///B5 - Affiliated Company
    AffiliatedCompany,
    ///B6 - Registering Parent Party
    RegisteringParentParty,
    ///B7 - Registering Nonparent Party
    RegisteringNonparentParty,
    ///B8 - Regular Dealer
    RegularDealer,
    ///B9 - Large Business
    LargeBusiness,
    ///BA - Battery
    Battery,
    ///BAL - Bailiff
    Bailiff,
    ///BB - Business Partner
    BusinessPartner,
    ///BC - Broadcaster
    Broadcaster,
    ///BD - Bill-to Party for Diversion Charges
    BillToPartyForDiversionCharges,
    ///BE - Beneficiary
    Beneficiary,
    ///BF - Billed From
    BilledFrom,
    ///BG - Buying Group
    BuyingGroup,
    ///BH - Interim Trustee
    InterimTrustee,
    ///BI - Trustee's Attorney
    TrusteesAttorney,
    ///BJ - Co-Counsel
    CoCounsel,
    ///BK - Bank
    Bank,
    ///BKR - Bookkeeper
    Bookkeeper,
    ///BL - Party to Receive Bill of Lading
    PartyToReceiveBillOfLading,
    ///BLD - Building
    Building,
    ///BLT - Structure
    Structure,
    ///BM - Brakeman
    Brakeman,
    ///BN - Beneficial Owner
    BeneficialOwner,
    ///BO - Broker or Sales Office
    BrokerOrSalesOffice,
    ///BOW - Body of Water
    BodyOfWater,
    ///BP - Special Counsel
    SpecialCounsel,
    ///BQ - Attorney for Defendant Private
    AttorneyForDefendantPrivate,
    ///BR - Broker
    Broker,
    ///BRN - Brand Name
    BrandName,
    ///BS - Bill and Ship To
    BillAndShipTo,
    ///BT - Bill-to-Party
    BillToParty,
    ///BU - Place of Business
    PlaceOfBusiness,
    ///BUS - Business
    Business,
    ///BV - Billing Service
    BillingService,
    ///BW - Borrower
    Borrower,
    ///BX - Attorney for Plaintiff
    AttorneyForPlaintiff,
    ///BY - Buying Party (Purchaser)
    CodeBY,
    ///BZ - Business Associate
    BusinessAssociate,
    ///C0 - Assistant Conductor
    AssistantConductor,
    ///C1 - In Care Of Party no. 1
    InCareOfPartyNo1,
    ///C2 - In Care Of Party no. 2
    InCareOfPartyNo2,
    ///C3 - Circuit Location Identifier
    CircuitLocation,
    ///C4 - Contract Administration Office
    ContractAdministrationOffice,
    ///C4A - Secondary Contract Administration Office
    SecondaryContractAdministrationOffice,
    ///C5 - Party Submitting Quote
    PartySubmittingQuote,
    ///C6 - Municipality
    Municipality,
    ///C7 - County
    County,
    ///C8 - City
    City,
    ///C9 - Contract Holder
    ContractHolder,
    ///CA - Carrier
    Carrier,
    ///CB - Customs Broker
    CustomsBroker,
    ///CC - Claimant
    Claimant,
    ///CD - Consignee (To Receive Mail and Small Parcels)
    CodeCD,
    ///CE - Consignee (To receive large parcels and freight)
    CodeCE,
    ///CF - Subsidiary/Division
    SubsidiaryDivision,
    ///CG - Carnet Issuer
    CarnetIssuer,
    ///CH - Chassis Provider
    ChassisProvider,
    ///CHA - Changed Address
    ChangedAddress,
    ///CI - Consignor
    Consignor,
    ///CJ - Automated Data Processing (ADP) Point
    CodeCJ,
    ///CK - Pharmacist
    Pharmacist,
    ///CL - Container Location
    ContainerLocation,
    ///CLT - Building Cluster
    BuildingCluster,
    ///CM - Customs
    Customs,
    ///CMW - Company Merged With
    CompanyMergedWith,
    ///CN - Consignee
    Consignee,
    ///CNP - Confirming Party
    ConfirmingParty,
    ///CNR - Confirmation Requester
    ConfirmationRequester,
    ///CNS - Confirmation Service Identifier Code
    ConfirmationServiceIdentifierCode,
    ///CO - Ocean Tariff Conference
    OceanTariffConference,
    ///COD - Co-Driver
    CoDriver,
    ///COL - Collateral Assignee
    CollateralAssignee,
    ///COM - Complainant
    Complainant,
    ///COR - Corrected Name
    CorrectedName,
    ///CP - Party to Receive Cert. of Compliance
    PartyToReceiveCertOfCompliance,
    ///CQ - Corporate Office
    CorporateOffice,
    ///CR - Container Return Company
    ContainerReturnCompany,
    ///CRW - Crew Member
    CrewMember,
    ///CS - Consolidator
    Consolidator,
    ///CT - Country of Origin
    CountryOfOrigin,
    ///CU - Coating or Paint Supplier
    CoatingOrPaintSupplier,
    ///CV - Converter
    Converter,
    ///CW - Accounting Station
    AccountingStation,
    ///CX - Claim Administrator
    ClaimAdministrator,
    ///CY - Country
    Country,
    ///CZ - Admitting Surgeon
    AdmittingSurgeon,
    ///D1 - Driver
    Driver,
    ///D2 - Commercial Insurer
    CommercialInsurer,
    ///D3 - Defendant
    Defendant,
    ///D4 - Debtor
    Debtor,
    ///D5 - Debtor-In-Possession
    DebtorInPossession,
    ///D6 - Consolidated Debtor
    ConsolidatedDebtor,
    ///D7 - Petitioning Creditor
    PetitioningCreditor,
    ///D8 - Dispatcher
    Dispatcher,
    ///D9 - Creditor's Attorney
    CreditorsAttorney,
    ///DA - Delivery Address
    DeliveryAddress,
    ///DAM - Damaged By
    DamagedBy,
    ///DB - Distributor Branch
    DistributorBranch,
    ///DC - Destination Carrier
    DestinationCarrier,
    ///DCC - Chief Deputy Clerk of Court
    ChiefDeputyClerkOfCourt,
    ///DD - Assistant Surgeon
    AssistantSurgeon,
    ///DE - Depositor
    Depositor,
    ///DF - Material Disposition Authorization Location
    MaterialDispositionAuthorizationLocation,
    ///DG - Design Engineering
    DesignEngineering,
    ///DH - Doing Business As
    DoingBusinessAs,
    ///DI - Different Premise Address (DPA)
    CodeDI,
    ///DIR - Distribution Recipient
    DistributionRecipient,
    ///DJ - Consulting Physician
    ConsultingPhysician,
    ///DK - Ordering Physician
    OrderingPhysician,
    ///DL - Dealer
    Dealer,
    ///DM - Destination Mail Facility
    DestinationMailFacility,
    ///DN - Referring Provider
    ReferringProvider,
    ///DO - Dependent Name
    DependentName,
    ///DP - Party to Provide Discount
    PartyToProvideDiscount,
    ///DQ - Supervising Physician
    SupervisingPhysician,
    ///DR - Destination Drayman
    DestinationDrayman,
    ///DS - Distributor
    Distributor,
    ///DT - Destination Terminal
    DestinationTerminal,
    ///DU - Resale Dealer
    ResaleDealer,
    ///DV - Division
    Division,
    ///DW - Downstream Party
    DownstreamParty,
    ///DX - Distiller
    Distiller,
    ///DY - Default/Foreclosure Specialist
    DefaultForeclosureSpecialist,
    ///DZ - Delivery Zone
    DeliveryZone,
    ///E0 - Assistant Engineer
    AssistantEngineer,
    ///E1 - Person or Other Entity Legally Responsible for a Child
    PersonOrOtherEntityLegallyResponsibleForAChild,
    ///E2 - Person or Other Entity With Whom a Child Resides
    PersonOrOtherEntityWithWhomAChildResides,
    ///E3 - Person or Other Entity Legally Responsible for and With Whom a Child Resides
    PersonOrOtherEntityLegallyResponsibleForAndWithWhomAChildResides,
    ///E4 - Other Person or Entity Associated with Student
    OtherPersonOrEntityAssociatedWithStudent,
    ///E5 - Examiner
    Examiner,
    ///E6 - Engineering
    Engineering,
    ///E7 - Previous Employer
    PreviousEmployer,
    ///E8 - Inquiring Party
    InquiringParty,
    ///E9 - Participating Laboratory
    ParticipatingLaboratory,
    ///EA - Study Submitter
    StudySubmitter,
    ///EAA - Assistant
    Assistant,
    ///EAB - Campaign Manager
    CampaignManager,
    ///EAD - Client
    Client,
    ///EAE - Commissioner
    Commissioner,
    ///EAF - Committee
    Committee,
    ///EAG - Contestant
    Contestant,
    ///EAH - Contributor
    Contributor,
    ///EAI - Deputy Chairperson
    DeputyChairperson,
    ///EAJ - Deputy Treasurer
    DeputyTreasurer,
    ///EAK - Donor
    Donor,
    ///EAL - Endorser
    Endorser,
    ///EAM - Guarantor
    Guarantor,
    ///EAN - Headquarters
    Headquarters,
    ///EAO - Independent Contractor
    IndependentContractor,
    ///EAP - Leader
    Leader,
    ///EAQ - Party Performing Liaison
    PartyPerformingLiaison,
    ///EAR - Lobbying Firm
    LobbyingFirm,
    ///EAS - Lobbyist
    Lobbyist,
    ///EAT - Media Contact
    MediaContact,
    ///EAU - Office Holder
    OfficeHolder,
    ///EAV - Party Authorized to Administer Oaths
    PartyAuthorizedToAdministerOaths,
    ///EAW - Party to Benefit
    PartyToBenefit,
    ///EAX - Party Holding Interest
    PartyHoldingInterest,
    ///EAY - Party Making Pledge
    PartyMakingPledge,
    ///EAZ - Party Returning Contribution
    PartyReturningContribution,
    ///EB - Eligible Party To The Contract
    EligiblePartyToTheContract,
    ///EBA - Party Returning Transfer
    PartyReturningTransfer,
    ///EBB - Lobbied Party
    LobbiedParty,
    ///EBC - Political Action Committee
    PoliticalActionCommittee,
    ///EBD - Political Party
    PoliticalParty,
    ///EBE - Proponent
    Proponent,
    ///EBF - Public Official
    PublicOfficial,
    ///EBG - Receiving Committee
    ReceivingCommittee,
    ///EBH - Affiliated Committee
    AffiliatedCommittee,
    ///EBI - Source
    Source,
    ///EBJ - Sponsor
    Sponsor,
    ///EBK - Sponsored Committee
    SponsoredCommittee,
    ///EBL - Designee
    Designee,
    ///EBM - Temporary Residence
    TemporaryResidence,
    ///EBN - Treasurer
    Treasurer,
    ///EBO - Vice-Chairperson
    ViceChairperson,
    ///EBP - Slate Mailer Organization
    SlateMailerOrganization,
    ///EBQ - Lodging Location
    LodgingLocation,
    ///EBR - Independent Expenditure Committee
    IndependentExpenditureCommittee,
    ///EBS - Major Donor
    MajorDonor,
    ///EC - Exchanger
    Exchanger,
    ///ED - Excluded Party
    ExcludedParty,
    ///EE - Location of Goods for Customs Examination Before Clearance
    LocationOfGoodsForCustomsExaminationBeforeClearance,
    ///EF - Electronic Filer
    ElectronicFiler,
    ///EG - Engineer
    Engineer,
    ///EH - Exhibitor
    Exhibitor,
    ///EI - Executor of Estate
    ExecutorOfEstate,
    ///EJ - Principal Person
    PrincipalPerson,
    ///EK - Animal Source
    AnimalSource,
    ///EL - Established Location
    EstablishedLocation,
    ///EM - Party to Receive Electronic Memo of Invoice
    PartyToReceiveElectronicMemoOfInvoice,
    ///EN - End User
    EndUser,
    ///ENR - Enroller
    Enroller,
    ///EO - Limited Liability Partnership
    LimitedLiabilityPartnership,
    ///EP - Eligible Party to the Rate
    EligiblePartyToTheRate,
    ///EQ - Old Debtor
    OldDebtor,
    ///ER - New Debtor
    NewDebtor,
    ///ET - Plan Administrator
    PlanAdministrator,
    ///EU - Old Secured Party
    OldSecuredParty,
    ///EV - Selling Agent
    SellingAgent,
    ///EW - Servicing Broker
    ServicingBroker,
    ///EX - Exporter
    Exporter,
    ///EXS - Ex-spouse
    ExSpouse,
    ///EY - Employee Name
    EmployeeName,
    ///EZ - New Secured Party
    NewSecuredParty,
    ///F1 - Company - Owned Oil Field
    CompanyOwnedOilField,
    ///F2 - Energy Information Administration (Department of Energy) - Owned Oil Field
    CodeF2,
    ///F3 - Specialized Mobile Radio Service (SMRS) Licensee
    CodeF3,
    ///F4 - Former Residence
    FormerResidence,
    ///F5 - Radio Control Station Location
    RadioControlStationLocation,
    ///F6 - Small Control Station Location
    SmallControlStationLocation,
    ///F7 - Small Base Station Location
    SmallBaseStationLocation,
    ///F8 - Antenna Site
    AntennaSite,
    ///F9 - Area of Operation
    AreaOfOperation,
    ///FA - Facility
    Facility,
    ///FB - First Break Terminal
    FirstBreakTerminal,
    ///FC - Customer Identification File (CIF) Customer Identifier
    CodeFC,
    ///FD - Physical Address
    PhysicalAddress,
    ///FE - Mail Address
    MailAddress,
    ///FF - Foreign Language Synonym
    ForeignLanguageSynonym,
    ///FG - Trade Name Synonym
    TradeNameSynonym,
    ///FGT - Foreign Government
    ForeignGovernment,
    ///FH - Party to Receive Limitations of Heavy Elements Report
    PartyToReceiveLimitationsOfHeavyElementsReport,
    ///FI - Name Variation Synonym
    NameVariationSynonym,
    ///FJ - First Contact
    FirstContact,
    ///FL - Primary Control Point Location
    PrimaryControlPointLocation,
    ///FM - Fireman
    Fireman,
    ///FN - Filer Name
    FilerName,
    ///FO - Field or Branch Office
    FieldOrBranchOffice,
    ///FP - Name on Credit Card
    NameOnCreditCard,
    ///FQ - Pier Name
    PierName,
    ///FR - Message From
    MessageFrom,
    ///FRL - Foreign Registration Location
    ForeignRegistrationLocation,
    ///FS - Final Scheduled Destination
    FinalScheduledDestination,
    ///FSI - Party to Receive Sensitive Foreign Disclosure Information
    PartyToReceiveSensitiveForeignDisclosure,
    ///FSR - Financial Statement Recipient
    FinancialStatementRecipient,
    ///FT - New Assignee
    NewAssignee,
    ///FU - Old Assignee
    OldAssignee,
    ///FV - Vessel Name
    VesselName,
    ///FW - Forwarder
    Forwarder,
    ///FX - Closed Door Pharmacy
    ClosedDoorPharmacy,
    ///FY - Veterinary Hospital
    VeterinaryHospital,
    ///FZ - Children's Day Care Center
    ChildrensDayCareCenter,
    ///G0 - Dependent Insured
    DependentInsured,
    ///G1 - Bankruptcy Trustee
    BankruptcyTrustee,
    ///G2 - Annuitant
    Annuitant,
    ///G3 - Clinic
    Clinic,
    ///G5 - Contingent Beneficiary
    ContingentBeneficiary,
    ///G6 - Entity Holding the Information
    EntityHoldingThe,
    ///G7 - Entity Providing the Service
    EntityProvidingTheService,
    ///G8 - Entity Responsible for Follow-up
    EntityResponsibleForFollowUp,
    ///G9 - Family Member
    FamilyMember,
    ///GA - Gas Plant
    GasPlant,
    ///GB - Other Insured
    OtherInsured,
    ///GBA - Alternate Government Business Contact
    AlternateGovernmentBusinessContact,
    ///GBO - Gate Booth
    GateBooth,
    ///GBP - Primary Government Business Contact
    PrimaryGovernmentBusinessContact,
    ///GC - Previous Credit Grantor
    PreviousCreditGrantor,
    ///GD - Guardian
    Guardian,
    ///GE - General Agency
    GeneralAgency,
    ///GF - Inspection Company
    InspectionCompany,
    ///GG - Intermediary
    Intermediary,
    ///GH - Motor Vehicle Report Provider Company
    MotorVehicleReportProviderCompany,
    ///GI - Paramedic
    Paramedic,
    ///GIR - Gift Recipient
    GiftRecipient,
    ///GJ - Paramedical Company
    ParamedicalCompany,
    ///GK - Previous Insured
    PreviousInsured,
    ///GL - Previous Residence
    PreviousResidence,
    ///GM - Spouse Insured
    SpouseInsured,
    ///GN - Garnishee
    Garnishee,
    ///GO - Primary Beneficiary
    PrimaryBeneficiary,
    ///GP - Gateway Provider
    GatewayProvider,
    ///GQ - Proposed Insured
    ProposedInsured,
    ///GR - Reinsurer
    Reinsurer,
    ///GS - Garaged Location
    GaragedLocation,
    ///GT - Credit Grantor
    CreditGrantor,
    ///GU - Guarantee Agency
    GuaranteeAgency,
    ///GV - Gas Transaction Ending Point
    GasTransactionEndingPoint,
    ///GW - Group
    Group,
    ///GX - Retrocessionaire
    Retrocessionaire,
    ///GY - Treatment Facility
    TreatmentFacility,
    ///GZ - Grandparent
    Grandparent,
    ///H1 - Representative
    Representative,
    ///H2 - Sub-Office
    SubOffice,
    ///H3 - District
    District,
    ///H5 - Paying Agent
    PayingAgent,
    ///H6 - School District
    SchoolDistrict,
    ///H7 - Group Affiliate
    GroupAffiliate,
    ///H9 - Designer
    Designer,
    ///HA - Owner
    Owner,
    ///HB - Historically Black College or University
    HistoricallyBlackCollegeOrUniversity,
    ///HC - Joint Annuitant
    JointAnnuitant,
    ///HD - Contingent Annuitant
    ContingentAnnuitant,
    ///HE - Contingent Owner
    ContingentOwner,
    ///HF - Healthcare Professional Shortage Area (HPSA) Facility
    CodeHF,
    ///HG - Broker Opinion or Analysis Requester
    BrokerOpinionOrAnalysisRequester,
    ///HH - Home Health Agency
    HomeHealthAgency,
    ///HI - Listing Company
    ListingCompany,
    ///HJ - Automated Underwriting System
    AutomatedUnderwritingSystem,
    ///HK - Subscriber
    Subscriber,
    ///HL - Document Custodian
    DocumentCustodian,
    ///HM - Competitive Property Listing
    CompetitivePropertyListing,
    ///HMI - Material Safety Data Sheet (MSDS) Recipient
    CodeHMI,
    ///HN - Competing Property
    CompetingProperty,
    ///HO - Comparable Property Listing
    ComparablePropertyListing,
    ///HOM - Home Office
    HomeOffice,
    ///HON - Honorary Society
    HonorarySociety,
    ///HP - Closed Sale
    ClosedSale,
    ///HQ - Source Party of Information
    SourcePartyOf,
    ///HR - Subject of Inquiry
    SubjectOfInquiry,
    ///HS - High School
    HighSchool,
    ///HT - State Chartered Facility
    StateCharteredFacility,
    ///HU - Subsidiary
    Subsidiary,
    ///HV - Tax Address
    TaxAddress,
    ///HW - Designated Hazardous Waste Facility
    DesignatedHazardousWasteFacility,
    ///HX - Transporter of Hazardous Waste
    TransporterOfHazardousWaste,
    ///HY - Charity
    Charity,
    ///HZ - Hazardous Waste Generator
    HazardousWasteGenerator,
    ///I1 - Interested Party
    InterestedParty,
    ///I3 - Independent Physicians Association (IPA)
    CodeI3,
    ///I4 - Intellectual Property Owner
    IntellectualPropertyOwner,
    ///I9 - Interviewer
    Interviewer,
    ///IA - Installed At
    InstalledAt,
    ///IAA - Business Entity
    BusinessEntity,
    ///IAC - Principal Executive Office
    PrincipalExecutiveOffice,
    ///IAD - Foreign Office
    ForeignOffice,
    ///IAE - Member
    Member,
    ///IAF - Executive Committee Member
    ExecutiveCommitteeMember,
    ///IAG - Director
    Director,
    ///IAH - Clerk
    Clerk,
    ///IAI - Party with Knowledge of Affairs of the Company
    PartyWithKnowledgeOfAffairsOfTheCompany,
    ///IAK - Party to Receive Statement of Fees Due
    PartyToReceiveStatementOfFeesDue,
    ///IAL - Company in which Interest Held
    CompanyInWhichInterestHeld,
    ///IAM - Company which Holds Interest
    CompanyWhichHoldsInterest,
    ///IAN - Notary
    Notary,
    ///IAO - Manager
    Manager,
    ///IAP - Alien Affiliate
    AlienAffiliate,
    ///IAQ - Incorporation State Principal Office
    IncorporationStatePrincipalOffice,
    ///IAR - Incorporation State Place of Business
    IncorporationStatePlaceOfBusiness,
    ///IAS - Out-of-State Principal Office
    OutOfStatePrincipalOffice,
    ///IAT - Party Executing and Verifying
    PartyExecutingAndVerifying,
    ///IAU - Felon
    Felon,
    ///IAV - Other Related Party
    OtherRelatedParty,
    ///IAW - Record-Keeping Address
    RecordKeepingAddress,
    ///IAY - Initial Subscriber
    InitialSubscriber,
    ///IAZ - Original Jurisdiction
    OriginalJurisdiction,
    ///IB - Industry Bureau
    IndustryBureau,
    ///IC - Intermediate Consignee
    IntermediateConsignee,
    ///ICP - Inventory Control Point
    InventoryControlPoint,
    ///ID - Issuer of Debit or Credit Memo
    IssuerOfDebitOrCreditMemo,
    ///IE - Other Individual Disability Carrier
    OtherIndividualDisabilityCarrier,
    ///IF - International Freight Forwarder
    InternationalFreightForwarder,
    ///IG - Insolvent Insurer
    InsolventInsurer,
    ///II - Issuer of Invoice
    IssuerOfInvoice,
    ///IJ - Injection Point
    InjectionPoint,
    ///IK - Intermediate Carrier
    IntermediateCarrier,
    ///IL - Insured or Subscriber
    InsuredOrSubscriber,
    ///IM - Importer
    Importer,
    ///IMM - Integrated Material Manager
    IntegratedMaterialManager,
    ///IN - Insurer
    Insurer,
    ///INT - Interviewee
    Interviewee,
    ///INV - Investment Advisor
    InvestmentAdvisor,
    ///IO - Inspector
    Inspector,
    ///IP - Independent Adjuster
    IndependentAdjuster,
    ///IQ - In-patient Pharmacy
    InPatientPharmacy,
    ///IR - Self Insured
    SelfInsured,
    ///IS - Party to Receive Certified Inspection Report
    PartyToReceiveCertifiedInspectionReport,
    ///IT - Installation on Site
    InstallationOnSite,
    ///IU - Issuer
    Issuer,
    ///IV - Renter
    Renter,
    ///J1 - Associate General Agent
    AssociateGeneralAgent,
    ///J2 - Authorized Entity
    AuthorizedEntity,
    ///J3 - Broker's Assistant
    BrokersAssistant,
    ///J4 - Custodian
    Custodian,
    ///J5 - Irrevocable Beneficiary
    IrrevocableBeneficiary,
    ///J6 - Power of Attorney
    PowerOfAttorney,
    ///J7 - Trust Officer
    TrustOfficer,
    ///J8 - Broker Dealer
    BrokerDealer,
    ///J9 - Community Agent
    CommunityAgent,
    ///JA - Dairy Department
    DairyDepartment,
    ///JB - Delicatessen Department
    DelicatessenDepartment,
    ///JC - Dry Grocery Department
    DryGroceryDepartment,
    ///JD - Judge
    Judge,
    ///JE - Frozen Department
    FrozenDepartment,
    ///JF - General Merchandise Department
    GeneralMerchandiseDepartment,
    ///JG - Health & Beauty Department
    CodeJG,
    ///JH - Alcohol Beverage Department
    AlcoholBeverageDepartment,
    ///JI - Meat Department
    MeatDepartment,
    ///JJ - Produce Department
    ProduceDepartment,
    ///JK - Bakery Department
    BakeryDepartment,
    ///JL - Video Department
    VideoDepartment,
    ///JM - Candy and Confections Department
    CandyAndConfectionsDepartment,
    ///JN - Cigarettes and Tobacco Department
    CigarettesAndTobaccoDepartment,
    ///JO - In-Store Bakery Department
    InStoreBakeryDepartment,
    ///JP - Floral Department
    FloralDepartment,
    ///JQ - Pharmacy Department
    PharmacyDepartment,
    ///JR - Bidder
    Bidder,
    ///JS - Joint Debtor Attorney
    JointDebtorAttorney,
    ///JT - Joint Debtor
    JointDebtor,
    ///JU - Jurisdiction
    Jurisdiction,
    ///JV - Joint Owner
    JointOwner,
    ///JW - Joint Venture
    JointVenture,
    ///JX - Closing Agent
    ClosingAgent,
    ///JY - Financial Planner
    FinancialPlanner,
    ///JZ - Managing General Agent
    ManagingGeneralAgent,
    ///K1 - Contractor Cognizant Security Office
    ContractorCognizantSecurityOffice,
    ///K2 - Subcontractor Cognizant Security Office
    SubcontractorCognizantSecurityOffice,
    ///K3 - Place of Performance Cognizant Security Office
    PlaceOfPerformanceCognizantSecurityOffice,
    ///K4 - Party Authorizing Release of Security Information
    PartyAuthorizingReleaseOfSecurity,
    ///K5 - Party To Receive Contract Security Classification Specification
    PartyToReceiveContractSecurityClassificationSpecification,
    ///K6 - Policy Writing Agent
    PolicyWritingAgent,
    ///K7 - Radio Station
    RadioStation,
    ///K8 - Filing Location
    FilingLocation,
    ///K9 - Previous Distributor
    PreviousDistributor,
    ///KA - Item Manager
    ItemManager,
    ///KB - Customer for Whom Same or Similar Work Was Performed
    CustomerForWhomSameOrSimilarWorkWasPerformed,
    ///KC - Party That Received Disclosure Statement
    PartyThatReceivedDisclosureStatement,
    ///KD - Proposer
    Proposer,
    ///KE - Contact Office
    ContactOffice,
    ///KF - Audit Office
    AuditOffice,
    ///KG - Project Manager
    ProjectManager,
    ///KH - Organization Having Source Control
    OrganizationHavingSourceControl,
    ///KI - United States Overseas Security Administration Office
    UnitedStatesOverseasSecurityAdministrationOffice,
    ///KJ - Qualifying Officer
    QualifyingOfficer,
    ///KK - Registering Party
    RegisteringParty,
    ///KL - Clerk of Court
    ClerkOfCourt,
    ///KM - Coordinator
    Coordinator,
    ///KN - Former Address
    FormerAddress,
    ///KO - Plant Clearance Officer
    PlantClearanceOfficer,
    ///KP - Name Under Which Filed
    NameUnderWhichFiled,
    ///KQ - Licensee
    Licensee,
    ///KR - Pre-kindergarten to Grade 12 Recipient
    PreKindergartenToGrade12Recipient,
    ///KS - Pre-kindergarten to Grade 12 Sender
    PreKindergartenToGrade12Sender,
    ///KT - Court
    Court,
    ///KU - Receiver Site
    ReceiverSite,
    ///KV - Disbursing Officer
    DisbursingOfficer,
    ///KW - Bid Opening Location
    BidOpeningLocation,
    ///KX - Free on Board Point
    FreeOnBoardPoint,
    ///KY - Technical Office
    TechnicalOffice,
    ///KZ - Acceptance Location
    AcceptanceLocation,
    ///L1 - Inspection Location
    InspectionLocation,
    ///L2 - Location of Principal Assets
    LocationOfPrincipalAssets,
    ///L3 - Loan Correspondent
    LoanCorrespondent,
    ///L5 - Contact
    Contact,
    ///L8 - Head Office
    HeadOffice,
    ///L9 - Information Provider
    InformationProvider,
    ///LA - Attorney
    Attorney,
    ///LB - Last Break Terminal
    LastBreakTerminal,
    ///LC - Location of Spot for Storage
    LocationOfSpotForStorage,
    ///LCN - Gas Nomination Location
    GasNominationLocation,
    ///LD - Liability Holder
    LiabilityHolder,
    ///LE - Lessor
    Lessor,
    ///LF - Limited Partner
    LimitedPartner,
    ///LG - Location of Goods
    LocationOfGoods,
    ///LGS - Local Government Sponsor
    LocalGovernmentSponsor,
    ///LH - Pipeline
    Pipeline,
    ///LI - Independent Lab
    IndependentLab,
    ///LJ - Limited Liability Company
    LimitedLiabilityCompany,
    ///LK - Juvenile Owner
    JuvenileOwner,
    ///LL - Location of Load Exchange (Export)
    CodeLL,
    ///LM - Lending Institution
    LendingInstitution,
    ///LN - Lender
    Lender,
    ///LO - Loan Originator
    LoanOriginator,
    ///LP - Loading Party
    LoadingParty,
    ///LQ - Law Firm
    LawFirm,
    ///LR - Legal Representative
    LegalRepresentative,
    ///LS - Lessee
    Lessee,
    ///LT - Long-term Disability Carrier
    LongTermDisabilityCarrier,
    ///LU - Master Agent
    MasterAgent,
    ///LV - Loan Servicer
    LoanServicer,
    ///LW - Customer
    Customer,
    ///LY - Labeler
    Labeler,
    ///LYM - Amended Name
    AmendedName,
    ///LYN - Stockholder
    Stockholder,
    ///LYO - Managing Agent
    ManagingAgent,
    ///LYP - Organizer
    Organizer,
    ///LZ - Local Chain
    LocalChain,
    ///M1 - Source Meter Location
    SourceMeterLocation,
    ///M2 - Receipt Location
    ReceiptLocation,
    ///M3 - Upstream Meter Location
    UpstreamMeterLocation,
    ///M4 - Downstream Meter Location
    DownstreamMeterLocation,
    ///M5 - Migrant Health Clinic
    MigrantHealthClinic,
    ///M6 - Landlord
    Landlord,
    ///M7 - Foreclosing Lender
    ForeclosingLender,
    ///M8 - Educational Institution
    EducationalInstitution,
    ///M9 - Manufacturing
    Manufacturing,
    ///MA - Party for whom Item is Ultimately Intended
    PartyForWhomItemIsUltimatelyIntended,
    ///MB - Company Interviewer Works For
    CompanyInterviewerWorksFor,
    ///MC - Motor Carrier
    MotorCarrier,
    ///MD - Veterans Administration Loan Guaranty Authority
    VeteransAdministrationLoanGuarantyAuthority,
    ///ME - Veterans Administration Loan Authorized Supplier
    VeteransAdministrationLoanAuthorizedSupplier,
    ///MF - Manufacturer of Goods
    ManufacturerOfGoods,
    ///MG - Government Loan Agency Sponsor or Agent
    GovernmentLoanAgencySponsorOrAgent,
    ///MH - Mortgage Insurer
    MortgageInsurer,
    ///MI - Planning Schedule/Material Release Issuer
    PlanningScheduleMaterialReleaseIssuer,
    ///MJ - Financial Institution
    FinancialInstitution,
    ///MK - Loan Holder for Real Estate Asset
    LoanHolderForRealEstateAsset,
    ///ML - Consumer Credit Account Company
    ConsumerCreditAccountCompany,
    ///MM - Mortgage Company
    MortgageCompany,
    ///MN - Authorized Marketer
    AuthorizedMarketer,
    ///MO - Release Drayman
    ReleaseDrayman,
    ///MP - Manufacturing Plant
    ManufacturingPlant,
    ///MQ - Delivery Location
    DeliveryLocation,
    ///MR - Medical Insurance Carrier
    MedicalInsuranceCarrier,
    ///MS - Bureau of Land Management (Minerals Management Service) Property Unit
    CodeMS,
    ///MSC - Mammography Screening Center
    MammographyScreeningCenter,
    ///MT - Material
    Material,
    ///MTR - Meter Location
    MeterLocation,
    ///MU - Meeting Location
    MeetingLocation,
    ///MV - Mainline
    Mainline,
    ///MW - Marine Surveyor
    MarineSurveyor,
    ///MX - Juvenile Witness
    JuvenileWitness,
    ///MY - Master General Agent
    MasterGeneralAgent,
    ///MZ - Minister
    Minister,
    ///N1 - Notify Party no. 1
    NotifyPartyNo1,
    ///N2 - Notify Party no. 2
    NotifyPartyNo2,
    ///N3 - Ineligible Party
    IneligibleParty,
    ///N4 - Price Administration
    PriceAdministration,
    ///N5 - Party Who Signed the Delivery Receipt
    PartyWhoSignedTheDeliveryReceipt,
    ///N6 - Nonemployment Income Source
    NonemploymentIncomeSource,
    ///N7 - Previous Neighbor
    PreviousNeighbor,
    ///N8 - Relative
    Relative,
    ///N9 - Neighborhood
    Neighborhood,
    ///NB - Neighbor
    Neighbor,
    ///NC - Cross-Town Switch
    CrossTownSwitch,
    ///NCT - Name Changed To
    NameChangedTo,
    ///ND - Next Destination
    NextDestination,
    ///NE - Newspaper
    Newspaper,
    ///NF - Owner Annuitant
    OwnerAnnuitant,
    ///NG - Administrator
    Administrator,
    ///NH - Association
    Association,
    ///NI - Non-insured
    NonInsured,
    ///NJ - Trust or Estate
    TrustOrEstate,
    ///NK - National Chain
    NationalChain,
    ///NL - Non-railroad Entity
    NonRailroadEntity,
    ///NM - Physician - Specialists
    PhysicianSpecialists,
    ///NN - Network Name
    NetworkName,
    ///NP - Notify Party for Shipper's Order
    NotifyPartyForShippersOrder,
    ///NPC - Notary Public
    NotaryPublic,
    ///NQ - Pipeline Segment Boundary
    PipelineSegmentBoundary,
    ///NR - Gas Transaction Starting Point
    GasTransactionStartingPoint,
    ///NS - Non-Temporary Storage Facility
    NonTemporaryStorageFacility,
    ///NT - Magistrate Judge
    MagistrateJudge,
    ///NU - Formerly Known As
    FormerlyKnownAs,
    ///NV - Formerly Doing Business As
    FormerlyDoingBusinessAs,
    ///NW - Maiden Name
    MaidenName,
    ///NX - Primary Owner
    PrimaryOwner,
    ///NY - Birth Name
    BirthName,
    ///NZ - Primary Physician
    PrimaryPhysician,
    ///O1 - Originating Bank
    OriginatingBank,
    ///O2 - Originating Company
    OriginatingCompany,
    ///O3 - Receiving Company
    ReceivingCompany,
    ///O4 - Factor
    Factor,
    ///O5 - Merchant Banker
    MerchantBanker,
    ///O6 - Non Registered Business Name
    NonRegisteredBusinessName,
    ///O7 - Registered Business Name
    RegisteredBusinessName,
    ///O8 - Registrar
    Registrar,
    ///OA - Electronic Return Originator
    ElectronicReturnOriginator,
    ///OB - Ordered By
    OrderedBy,
    ///OC - Origin Carrier
    OriginCarrier,
    ///OD - Doctor of Optometry
    DoctorOfOptometry,
    ///OE - Booking Office
    BookingOffice,
    ///OF - Offset Operator
    OffsetOperator,
    ///OG - Co-owner
    CoOwner,
    ///OH - Other Departments
    OtherDepartments,
    ///OI - Outside Inspection Agency
    OutsideInspectionAgency,
    ///OL - Officer
    Officer,
    ///OM - Origin Mail Facility
    OriginMailFacility,
    ///ON - Product Position Holder
    ProductPositionHolder,
    ///OO - Order Of (Shippers Orders) - (Transportation)
    CodeOO,
    ///OP - Operator of property or unit
    OperatorOfPropertyOrUnit,
    ///OR - Origin Drayman
    OriginDrayman,
    ///ORI - Original Name
    OriginalName,
    ///OS - Override Institution; this is not the institution sending the record, but another institution the student previously attended or is currently attending
    CodeOS,
    ///OSH - Off-Site Handler
    OffSiteHandler,
    ///OT - Origin Terminal
    OriginTerminal,
    ///OU - Outside Processor
    OutsideProcessor,
    ///OUC - Other Unlisted Type of Corporation
    OtherUnlistedTypeOfCorporation,
    ///OV - Owner of Vessel
    OwnerOfVessel,
    ///OW - Owner of Property or Unit
    OwnerOfPropertyOrUnit,
    ///OX - Oxygen Therapy Facility
    OxygenTherapyFacility,
    ///OY - Owner of Vehicle
    OwnerOfVehicle,
    ///OZ - Outside Testing Agency
    OutsideTestingAgency,
    ///P0 - Patient Facility
    PatientFacility,
    ///P1 - Preparer
    Preparer,
    ///P2 - Primary Insured or Subscriber
    PrimaryInsuredOrSubscriber,
    ///P3 - Primary Care Provider
    PrimaryCareProvider,
    ///P4 - Prior Insurance Carrier
    PriorInsuranceCarrier,
    ///P5 - Plan Sponsor
    PlanSponsor,
    ///P6 - Third Party Reviewing Preferred Provider Organization (PPO)
    CodeP6,
    ///P7 - Third Party Repricing Preferred Provider Organization (PPO)
    CodeP7,
    ///P8 - Personnel Office
    PersonnelOffice,
    ///P9 - Primary Interexchange Carrier (PIC)
    CodeP9,
    ///PA - Party to Receive Inspection Report
    PartyToReceiveInspectionReport,
    ///PB - Paying Bank
    PayingBank,
    ///PC - Party to Receive Cert. of Conformance (C.A.A.)
    CodePC,
    ///PD - Purchaser's Department Buyer
    PurchasersDepartmentBuyer,
    ///PE - Payee
    Payee,
    ///PF - Party to Receive Freight Bill
    PartyToReceiveFreightBill,
    ///PG - Prime Contractor
    PrimeContractor,
    ///PH - Printer
    Printer,
    ///PI - Publisher
    Publisher,
    ///PIC - Primary Inventory Control Activity
    PrimaryInventoryControlActivity,
    ///PJ - Party to Receive Correspondence
    PartyToReceiveCorrespondence,
    ///PK - Party to Receive Copy
    PartyToReceiveCopy,
    ///PL - Party to Receive Purchase Order
    PartyToReceivePurchaseOrder,
    ///PLC - Law Enforcement Agency
    LawEnforcementAgency,
    ///PLR - Payer of Last Resort
    PayerOfLastResort,
    ///PM - Party to receive paper Memo of Invoice
    PartyToReceivePaperMemoOfInvoice,
    ///PMC - Prior Mortgage Company
    PriorMortgageCompany,
    ///PMF - Party Manufactured For
    PartyManufacturedFor,
    ///PMG - Program Manager
    ProgramManager,
    ///PN - Party to Receive Shipping Notice
    PartyToReceiveShippingNotice,
    ///PO - Party to Receive Invoice for Goods or Services
    PartyToReceiveInvoiceForGoodsOrServices,
    ///PP - Property
    Property,
    ///PPC - Past Performance Contact
    PastPerformanceContact,
    ///PPS - Person for Whose Benefit Property was Seized
    PersonForWhoseBenefitPropertyWasSeized,
    ///PQ - Party to Receive Invoice for Lease Payments
    PartyToReceiveInvoiceForLeasePayments,
    ///PR - Payer
    Payer,
    ///PRE - Previous Owner
    PreviousOwner,
    ///PRO - Prospect Service
    ProspectService,
    ///PRP - Primary Payer
    PrimaryPayer,
    ///PS - Previous Station
    PreviousStation,
    ///PT - Party to Receive Test Report
    PartyToReceiveTestReport,
    ///PU - Party at Pickup Location
    PartyAtPickupLocation,
    ///PUR - Purchased Company
    PurchasedCompany,
    ///PV - Party performing certification
    PartyPerformingCertification,
    ///PW - Pickup Address
    PickupAddress,
    ///PX - Party Performing Count
    PartyPerformingCount,
    ///PY - Party to File Personal Property Tax
    PartyToFilePersonalPropertyTax,
    ///PZ - Party to Receive Equipment
    PartyToReceiveEquipment,
    ///Q1 - Conductor Pilot
    ConductorPilot,
    ///Q2 - Engineer Pilot
    EngineerPilot,
    ///Q3 - Retail Account
    RetailAccount,
    ///Q4 - Cooperative Buying Group
    CooperativeBuyingGroup,
    ///Q5 - Advertising Group
    AdvertisingGroup,
    ///Q6 - Interpreter
    Interpreter,
    ///Q7 - Partner
    Partner,
    ///Q8 - Base Period Employer
    BasePeriodEmployer,
    ///Q9 - Last Employer
    LastEmployer,
    ///QA - Pharmacy
    Pharmacy,
    ///QB - Purchase Service Provider
    PurchaseServiceProvider,
    ///QC - Patient
    Patient,
    ///QD - Responsible Party
    ResponsibleParty,
    ///QE - Policyholder
    Policyholder,
    ///QF - Passenger
    Passenger,
    ///QG - Pedestrian
    Pedestrian,
    ///QH - Physician
    Physician,
    ///QI - Party in Possession
    PartyInPossession,
    ///QJ - Most Recent Employer (Chargeable)
    CodeQJ,
    ///QK - Managed Care
    ManagedCare,
    ///QL - Chiropractor
    Chiropractor,
    ///QM - Dialysis Centers
    DialysisCenters,
    ///QN - Dentist
    Dentist,
    ///QO - Doctor of Osteopathy
    DoctorOfOsteopathy,
    ///QP - Principal Borrower
    PrincipalBorrower,
    ///QQ - Quality Control
    QualityControl,
    ///QR - Buyer's Quality Review Board
    BuyersQualityReviewBoard,
    ///QS - Podiatrist
    Podiatrist,
    ///QT - Psychiatrist
    Psychiatrist,
    ///QU - Veterinarian
    Veterinarian,
    ///QV - Group Practice
    GroupPractice,
    ///QW - Government
    Government,
    ///QX - Home Health Corporation
    HomeHealthCorporation,
    ///QY - Medical Doctor
    MedicalDoctor,
    ///QZ - Co-borrower
    CoBorrower,
    ///R0 - Royalty Owner
    RoyaltyOwner,
    ///R1 - Party to Receive Scale Ticket
    PartyToReceiveScaleTicket,
    ///R2 - Reporting Officer
    ReportingOfficer,
    ///R3 - Next Scheduled Destination
    NextScheduledDestination,
    ///R4 - Regulatory (State) District
    CodeR4,
    ///R5 - Regulatory (State) Entity
    CodeR5,
    ///R6 - Requester
    Requester,
    ///R7 - Consumer Referral Contact
    ConsumerReferralContact,
    ///R8 - Credit Reporting Agency
    CreditReportingAgency,
    ///R9 - Requested Lender
    RequestedLender,
    ///RA - Alternate Return Address
    AlternateReturnAddress,
    ///RB - Receiving Bank
    ReceivingBank,
    ///RC - Receiving Location
    ReceivingLocation,
    ///RCR - Recovery Room
    RecoveryRoom,
    ///RD - Destination Intermodal Ramp
    DestinationIntermodalRamp,
    ///REC - Receiver Manager
    ReceiverManager,
    ///RF - Refinery
    Refinery,
    ///RG - Responsible Installation, Origin
    CodeRG,
    ///RGA - Responsible Government Agency
    ResponsibleGovernmentAgency,
    ///RH - Responsible Installation, Destination
    CodeRH,
    ///RI - Remit To
    RemitTo,
    ///RJ - Residence or Domicile
    ResidenceOrDomicile,
    ///RK - Refinery Operator
    RefineryOperator,
    ///RL - Reporting Location
    ReportingLocation,
    ///RM - Party that remits payment
    PartyThatRemitsPayment,
    ///RN - Repair or Refurbish Location
    RepairOrRefurbishLocation,
    ///RO - Original Intermodal Ramp
    OriginalIntermodalRamp,
    ///RP - Receiving Point for Customer Samples
    ReceivingPointForCustomerSamples,
    ///RQ - Resale Customer
    ResaleCustomer,
    ///RR - Railroad
    Railroad,
    ///RR2 - Class II Railroad
    ClassIiRailroad,
    ///RR3 - Class III Railroad
    ClassIiiRailroad,
    ///RS - Receiving Facility Scheduler
    ReceivingFacilityScheduler,
    ///RT - Returned to
    ReturnedTo,
    ///RU - Receiving Sub-Location
    ReceivingSubLocation,
    ///RV - Reservoir
    Reservoir,
    ///RW - Rural Health Clinic
    RuralHealthClinic,
    ///RX - Responsible Exhibitor
    ResponsibleExhibitor,
    ///RY - Specified Repository
    SpecifiedRepository,
    ///RZ - Receipt Zone
    ReceiptZone,
    ///S0 - Sole Proprietor
    SoleProprietor,
    ///S1 - Parent
    Parent,
    ///S2 - Student
    Student,
    ///S3 - Custodial Parent
    CustodialParent,
    ///S4 - Skilled Nursing Facility
    SkilledNursingFacility,
    ///S5 - Secured Party
    SecuredParty,
    ///S6 - Agency Granting Security Clearance
    AgencyGrantingSecurityClearance,
    ///S7 - Secured Party Company
    SecuredPartyCompany,
    ///S8 - Secured Party Individual
    SecuredPartyIndividual,
    ///S9 - Sibling
    Sibling,
    ///SA - Salvage Carrier
    SalvageCarrier,
    ///SB - Storage Area
    StorageArea,
    ///SC - Store Class
    StoreClass,
    ///SD - Sold To and Ship To
    SoldToAndShipTo,
    ///SE - Selling Party
    SellingParty,
    ///SEP - Secondary Payer
    SecondaryPayer,
    ///SF - Ship From
    ShipFrom,
    ///SG - Store Group
    StoreGroup,
    ///SH - Shipper
    Shipper,
    ///SI - Shipping Schedule Issuer
    ShippingScheduleIssuer,
    ///SIC - Secondary Inventory Control Activity
    SecondaryInventoryControlActivity,
    ///SIP - Ship-in-Place Location
    ShipInPlaceLocation,
    ///SJ - Service Provider
    ServiceProvider,
    ///SK - Secondary Location Address (SLA)
    CodeSK,
    ///SL - Origin Sublocation
    OriginSublocation,
    ///SM - Party to Receive Shipping Manifest
    PartyToReceiveShippingManifest,
    ///SN - Store
    Store,
    ///SNP - US Customs & Border Protection Second Notify Party
    CodeSNP,
    ///SO - Sold To If Different From Bill To
    SoldToIfDifferentFromBillTo,
    ///SP - Party filling Shipper's Order
    PartyFillingShippersOrder,
    ///SQ - Service Bureau
    ServiceBureau,
    ///SR - Samples to be Returned To
    SamplesToBeReturnedTo,
    ///SS - Steamship Company
    SteamshipCompany,
    ///ST - Ship To
    ShipTo,
    ///STC - Switching and Terminal Carrier
    SwitchingAndTerminalCarrier,
    ///SU - Supplier/Manufacturer
    SupplierManufacturer,
    ///SUS - Supply Source
    SupplySource,
    ///SV - Service Performance Site
    ServicePerformanceSite,
    ///SW - Sealing Company
    SealingCompany,
    ///SX - School-based Service Provider
    SchoolBasedServiceProvider,
    ///SY - Secondary Taxpayer
    SecondaryTaxpayer,
    ///SZ - Supervisor
    Supervisor,
    ///T1 - Operator of the Transfer Point
    OperatorOfTheTransferPoint,
    ///T2 - Operator of the Source Transfer Point
    OperatorOfTheSourceTransferPoint,
    ///T3 - Terminal Location
    TerminalLocation,
    ///T4 - Transfer Point
    TransferPoint,
    ///T6 - Terminal Operator
    TerminalOperator,
    ///T8 - Previous Title Company
    PreviousTitleCompany,
    ///T9 - Prior Title Evidence Holder
    PriorTitleEvidenceHolder,
    ///TA - Title Insurance Services Provider
    TitleInsuranceServicesProvider,
    ///TB - Tooling
    Tooling,
    ///TC - Tool Source
    ToolSource,
    ///TD - Tooling Design
    ToolingDesign,
    ///TE - Theatre
    Theatre,
    ///TEC - Tax Exempt Corporation
    TaxExemptCorporation,
    ///TF - Tank Farm
    TankFarm,
    ///TG - Tooling Fabrication
    ToolingFabrication,
    ///TH - Theater Circuit
    TheaterCircuit,
    ///TI - Tariff Issuer
    TariffIssuer,
    ///TJ - Cosigner
    Cosigner,
    ///TK - Test Sponsor
    TestSponsor,
    ///TL - Testing Laboratory
    TestingLaboratory,
    ///TM - Transmitter
    Transmitter,
    ///TN - Tradename
    Tradename,
    ///TO - Message To
    MessageTo,
    ///TOW - Towing Agency
    TowingAgency,
    ///TP - Primary Taxpayer
    PrimaryTaxpayer,
    ///TPM - Third Party Marketer
    ThirdPartyMarketer,
    ///TQ - Third Party Reviewing Organization (TPO)
    CodeTQ,
    ///TR - Terminal
    Terminal,
    ///TS - Party to Receive Certified Test Results
    PartyToReceiveCertifiedTestResults,
    ///TSD - Treatment, Storage or Disposal Facility
    CodeTSD,
    ///TSE - Consignee Courier Transfer Station
    ConsigneeCourierTransferStation,
    ///TSR - Consignor Courier Transfer Station
    ConsignorCourierTransferStation,
    ///TT - Transfer To
    TransferTo,
    ///TTP - Tertiary Payer
    TertiaryPayer,
    ///TU - Third Party Repricing Organization (TPO)
    CodeTU,
    ///TV - Third Party Administrator (TPA)
    CodeTV,
    ///TW - Transit Authority
    TransitAuthority,
    ///TX - Tax Authority
    TaxAuthority,
    ///TY - Trustee
    Trustee,
    ///TZ - Significant Other
    SignificantOther,
    ///U1 - Gas Transaction Point 1
    GasTransactionPoint1,
    ///U2 - Gas Transaction Point 2
    GasTransactionPoint2,
    ///U3 - Servicing Agent
    ServicingAgent,
    ///U4 - Team
    Team,
    ///U5 - Underwriter
    Underwriter,
    ///U6 - Title Underwriter
    TitleUnderwriter,
    ///U7 - Psychologist
    Psychologist,
    ///U8 - Reference
    Reference,
    ///U9 - Non-Registered Investment Advisor
    NonRegisteredInvestmentAdvisor,
    ///UA - Place of Bottling
    PlaceOfBottling,
    ///UB - Place of Distilling
    PlaceOfDistilling,
    ///UC - Ultimate Consignee
    UltimateConsignee,
    ///UD - Region
    Region,
    ///UE - Testing Service
    TestingService,
    ///UF - Health Miscellaneous
    HealthMiscellaneous,
    ///UG - Nursing Home Chain
    NursingHomeChain,
    ///UH - Nursing Home
    NursingHome,
    ///UI - Registered Investment Advisor
    RegisteredInvestmentAdvisor,
    ///UJ - Sales Assistant
    SalesAssistant,
    ///UK - System
    System,
    ///UL - Special Account
    SpecialAccount,
    ///UM - Current Employer (Primary)
    CodeUM,
    ///UN - Union
    Union,
    ///UO - Current Employer (Secondary)
    CodeUO,
    ///UP - Unloading Party
    UnloadingParty,
    ///UQ - Subsequent Owner
    SubsequentOwner,
    ///UR - Surgeon
    Surgeon,
    ///US - Upstream Party
    UpstreamParty,
    ///UT - U.S. Trustee
    USTrustee,
    ///UU - Annuitant Payor
    AnnuitantPayor,
    ///UW - Unassigned Agent
    UnassignedAgent,
    ///UX - Base Jurisdiction
    BaseJurisdiction,
    ///UY - Vehicle
    Vehicle,
    ///UZ - Signer
    Signer,
    ///V1 - Surety
    Surety,
    ///V2 - Grantor
    Grantor,
    ///V3 - Well Pad Construction Contractor
    WellPadConstructionContractor,
    ///V4 - Oil and Gas Regulatory Agency
    OilAndGasRegulatoryAgency,
    ///V5 - Surface Discharge Agency
    SurfaceDischargeAgency,
    ///V6 - Well Casing Depth Authority
    WellCasingDepthAuthority,
    ///V8 - Market Timer
    MarketTimer,
    ///V9 - Owner Annuitant Payor
    OwnerAnnuitantPayor,
    ///VA - Second Contact
    SecondContact,
    ///VB - Candidate
    Candidate,
    ///VC - Vehicle Custodian
    VehicleCustodian,
    ///VD - Multiple Listing Service
    MultipleListingService,
    ///VE - Board of Realtors
    BoardOfRealtors,
    ///VER - Party Performing Verification
    PartyPerformingVerification,
    ///VF - Selling Office
    SellingOffice,
    ///VG - Listing Agent
    ListingAgent,
    ///VH - Showing Agent
    ShowingAgent,
    ///VI - Contact Person
    ContactPerson,
    ///VIC - Victim
    Victim,
    ///VJ - Owner Joint Annuitant Payor
    OwnerJointAnnuitantPayor,
    ///VK - Property or Building Manager
    PropertyOrBuildingManager,
    ///VL - Builder Name
    BuilderName,
    ///VM - Occupant
    Occupant,
    ///VN - Vendor
    Vendor,
    ///VO - Elementary School
    ElementarySchool,
    ///VP - Party with Power to Vote Securities
    PartyWithPowerToVoteSecurities,
    ///VQ - Middle School
    MiddleSchool,
    ///VR - Junior High School
    JuniorHighSchool,
    ///VS - Vehicle Salvage Assignment
    VehicleSalvageAssignment,
    ///VT - Listing Office
    ListingOffice,
    ///VU - Second Contact Organization
    SecondContactOrganization,
    ///VV - Owner Payor
    OwnerPayor,
    ///VW - Winner
    Winner,
    ///VX - Production Manager
    ProductionManager,
    ///VY - Organization Completing Configuration Change
    OrganizationCompletingConfigurationChange,
    ///W1 - Work Team
    WorkTeam,
    ///W2 - Supplier Work Team
    SupplierWorkTeam,
    ///W3 - Third Party Investment Advisor
    ThirdPartyInvestmentAdvisor,
    ///W4 - Trust
    Trust,
    ///W8 - Interline Service Commitment Customer
    InterlineServiceCommitmentCustomer,
    ///W9 - Sampling Location
    SamplingLocation,
    ///WA - Writing Agent
    WritingAgent,
    ///WB - Appraiser Name
    AppraiserName,
    ///WC - Comparable Property
    ComparableProperty,
    ///WD - Storage Facility at Destination
    StorageFacilityAtDestination,
    ///WE - Subject Property
    SubjectProperty,
    ///WF - Tank Farm Owner
    TankFarmOwner,
    ///WG - Wage Earner
    WageEarner,
    ///WH - Warehouse
    Warehouse,
    ///WI - Witness
    Witness,
    ///WJ - Supervisory Appraiser Name
    SupervisoryAppraiserName,
    ///WL - Wholesaler
    Wholesaler,
    ///WN - Company Assigned Well
    CompanyAssignedWell,
    ///WO - Storage Facility at Origin
    StorageFacilityAtOrigin,
    ///WP - Witness for Plaintiff
    WitnessForPlaintiff,
    ///WR - Withdrawal Point
    WithdrawalPoint,
    ///WS - Water System
    WaterSystem,
    ///WT - Witness for Defendant
    WitnessForDefendant,
    ///WU - Primary Support Organization
    PrimarySupportOrganization,
    ///WV - Preliminary Maintenance Period Designating Organization
    PreliminaryMaintenancePeriodDesignatingOrganization,
    ///WW - Preliminary Maintenance Organization
    PreliminaryMaintenanceOrganization,
    ///WX - Preliminary Referred To Organization
    PreliminaryReferredToOrganization,
    ///WY - Final Maintenance Period Designating Organization
    FinalMaintenancePeriodDesignatingOrganization,
    ///WZ - Final Maintenance Organization
    FinalMaintenanceOrganization,
    ///X1 - Mail to
    MailTo,
    ///X2 - Party to Perform Packaging
    PartyToPerformPackaging,
    ///X3 - Utilization Management Organization
    UtilizationManagementOrganization,
    ///X4 - Spouse
    Spouse,
    ///X5 - Durable Medical Equipment Supplier
    DurableMedicalEquipmentSupplier,
    ///X6 - International Organization
    InternationalOrganization,
    ///X7 - Inventor
    Inventor,
    ///X8 - Hispanic Service Institute
    HispanicServiceInstitute,
    ///XA - Creditor
    Creditor,
    ///XC - Debtor's Attorney
    DebtorsAttorney,
    ///XD - Alias
    Alias,
    ///XE - Claim Recipient
    ClaimRecipient,
    ///XF - Auctioneer
    Auctioneer,
    ///XG - Event Location
    EventLocation,
    ///XH - Final Referred To Organization
    FinalReferredToOrganization,
    ///XI - Original Claimant
    OriginalClaimant,
    ///XJ - Actual Referred By Organization
    ActualReferredByOrganization,
    ///XK - Actual Referred To Organization
    ActualReferredToOrganization,
    ///XL - Borrower's Employer
    BorrowersEmployer,
    ///XM - Maintenance Organization Used for Estimate
    MaintenanceOrganizationUsedForEstimate,
    ///XN - Planning/Maintenance Organization
    PlanningMaintenanceOrganization,
    ///XO - Preliminary Customer Organization
    PreliminaryCustomerOrganization,
    ///XP - Party to Receive Solicitation
    PartyToReceiveSolicitation,
    ///XQ - Canadian Customs Broker
    CanadianCustomsBroker,
    ///XR - Mexican Customs Broker
    MexicanCustomsBroker,
    ///XS - S Corporation
    SCorporation,
    ///XT - Final Customer Organization
    FinalCustomerOrganization,
    ///XU - United States Customs Broker
    UnitedStatesCustomsBroker,
    ///XV - Cross Claimant
    CrossClaimant,
    ///XW - Counter Claimant
    CounterClaimant,
    ///XX - Business Area
    BusinessArea,
    ///XY - Tribal Government
    TribalGovernment,
    ///XZ - American Indian-Owned Business
    AmericanIndianOwnedBusiness,
    ///Y2 - Managed Care Organization
    ManagedCareOrganization,
    ///YA - Affiant
    Affiant,
    ///YB - Arbitrator
    Arbitrator,
    ///YC - Bail Payor
    BailPayor,
    ///YD - District Justice
    DistrictJustice,
    ///YE - Third Party
    ThirdParty,
    ///YF - Witness for Prosecution
    WitnessForProsecution,
    ///YG - Expert Witness
    ExpertWitness,
    ///YH - Crime Victim
    CrimeVictim,
    ///YI - Juvenile Victim
    JuvenileVictim,
    ///YJ - Juvenile Defendant
    JuvenileDefendant,
    ///YK - Bondsman
    Bondsman,
    ///YL - Court Appointed Attorney
    CourtAppointedAttorney,
    ///YM - Complainant's Attorney
    ComplainantsAttorney,
    ///YN - District Attorney
    DistrictAttorney,
    ///YO - Attorney for Defendant, Public
    CodeYO,
    ///YP - Pro Bono Attorney
    ProBonoAttorney,
    ///YQ - Pro Se Counsel
    ProSeCounsel,
    ///YR - Party to Appear Before
    PartyToAppearBefore,
    ///YS - Appellant
    Appellant,
    ///YT - Appellee
    Appellee,
    ///YU - Arresting Officer
    ArrestingOfficer,
    ///YV - Hostile Witness
    HostileWitness,
    ///YW - Discharge Point
    DischargePoint,
    ///YX - Flood Certifier
    FloodCertifier,
    ///YY - Flood Determination Provider
    FloodDeterminationProvider,
    ///YZ - Electronic Registration Utility
    ElectronicRegistrationUtility,
    ///Z1 - Party to Receive Status
    PartyToReceiveStatus,
    ///Z2 - Unserviceable Material Consignee
    UnserviceableMaterialConsignee,
    ///Z3 - Potential Source of Supply
    PotentialSourceOfSupply,
    ///Z4 - Owning Inventory Control Point
    OwningInventoryControlPoint,
    ///Z5 - Management Control Activity
    ManagementControlActivity,
    ///Z6 - Transferring Party
    TransferringParty,
    ///Z7 - Mark-for Party
    MarkForParty,
    ///Z8 - Last Known Source of Supply
    LastKnownSourceOfSupply,
    ///Z9 - Banker
    Banker,
    ///ZA - Corrected Address
    CorrectedAddress,
    ///ZB - Party to Receive Credit
    PartyToReceiveCredit,
    ///ZC - Rent Payor
    RentPayor,
    ///ZD - Party to Receive Reports
    PartyToReceiveReports,
    ///ZE - End Item Manufacturer
    EndItemManufacturer,
    ///ZF - Break Bulk Point
    BreakBulkPoint,
    ///ZG - Present Address
    PresentAddress,
    ///ZH - Child
    Child,
    ///ZJ - Branch
    Branch,
    ///ZK - Reporter
    Reporter,
    ///ZL - Party Passing the Transaction
    PartyPassingTheTransaction,
    ///ZM - Lease Location
    LeaseLocation,
    ///ZN - Losing Inventory Manager
    LosingInventoryManager,
    ///ZO - Minimum Royalty Payor
    MinimumRoyaltyPayor,
    ///ZP - Gaining Inventory Manager
    GainingInventoryManager,
    ///ZQ - Screening Point
    ScreeningPoint,
    ///ZR - Validating Party
    ValidatingParty,
    ///ZS - Monitoring Party
    MonitoringParty,
    ///ZT - Participating Area
    ParticipatingArea,
    ///ZU - Formation
    Formation,
    ///ZV - Allowable Recipient
    AllowableRecipient,
    ///ZW - Field
    Field,
    ///ZX - Attorney of Record
    AttorneyOfRecord,
    ///ZY - Amicus Curiae
    AmicusCuriae,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl EntityIdentifierCode {
    pub fn code(&self) -> &str {
        {
            use EntityIdentifierCode::*;
            match self {
                AlternateInsurer => "00",
                ComparableRentals => "0A",
                InterimFundingOrganization => "0B",
                NonOccupantCoBorrower => "0D",
                ListOwner => "0E",
                ListMailer => "0F",
                PrimaryElectronicBusinessContact => "0G",
                StateDivision => "0H",
                AlternateElectronicBusinessContact => "0I",
                PrimaryPracticeLocation => "0J",
                PartyToDeclareGoods => "0P",
                LoanApplicant => "01",
                Pumper => "001",
                Subgroup => "1A",
                Applicant => "1B",
                Code1C => "1C",
                CoOperative => "1D",
                Code1E => "1E",
                Alliance => "1F",
                OncologyCenter => "1G",
                KidneyDialysisUnit => "1H",
                Code1I => "1I",
                Connection => "1J",
                Franchisor => "1K",
                Franchisee => "1L",
                PreviousGroup => "1M",
                Shareholder => "1N",
                AcuteCareHospital => "1O",
                Provider => "1P",
                MilitaryFacility => "1Q",
                Code1R => "1R",
                OutpatientSurgicenter => "1S",
                Code1T => "1T",
                LongTermCareFacility => "1U",
                ExtendedCareFacility => "1V",
                PsychiatricHealthFacility => "1W",
                Laboratory => "1X",
                RetailPharmacy => "1Y",
                HomeHealthCare => "1Z",
                LoanBroker => "02",
                SurfaceManagementEntity => "002",
                Code2A => "2A",
                ThirdPartyAdministrator => "2B",
                CoParticipant => "2C",
                MiscellaneousHealthCareFacility => "2D",
                NonHealthCareMiscellaneousFacility => "2E",
                State => "2F",
                Assigner => "2G",
                HospitalDistrictOrAuthority => "2H",
                ChurchOperatedFacility => "2I",
                Individual => "2J",
                Partnership => "2K",
                Corporation => "2L",
                AirForceFacility => "2M",
                ArmyFacility => "2N",
                NavyFacility => "2O",
                PublicHealthServiceFacility => "2P",
                VeteransAdministrationFacility => "2Q",
                FederalFacility => "2R",
                PublicHealthServiceIndianServiceFacility => "2S",
                DepartmentOfJusticeFacility => "2T",
                OtherNotForProfitFacility => "2U",
                IndividualForProfitFacility => "2V",
                PartnershipForProfitFacility => "2W",
                CorporationForProfitFacility => "2X",
                GeneralMedicalAndSurgicalFacility => "2Y",
                Code2Z => "2Z",
                Dependent => "03",
                ApplicationParty => "003",
                HospitalUnitWithinAnInstitutionForTheMentallyRetarded => "3A",
                PsychiatricFacility => "3B",
                TuberculosisAndOtherRespiratoryDiseasesFacility => "3C",
                ObstetricsAndGynecologyFacility => "3D",
                Code3E => "3E",
                RehabilitationFacility => "3F",
                OrthopedicFacility => "3G",
                ChronicDiseaseFacility => "3H",
                OtherSpecialtyFacility => "3I",
                ChildrensGeneralFacility => "3J",
                ChildrensHospitalUnitOfAnInstitution => "3K",
                ChildrensPsychiatricFacility => "3L",
                ChildrensTuberculosisAndOtherRespiratoryDiseasesFacility => "3M",
                Code3N => "3N",
                ChildrensRehabilitationFacility => "3O",
                ChildrensOrthopedicFacility => "3P",
                ChildrensChronicDiseaseFacility => "3Q",
                ChildrensOtherSpecialtyFacility => "3R",
                InstitutionForMentalRetardation => "3S",
                AlcoholismAndOtherChemicalDependencyFacility => "3T",
                GeneralInpatientCareForAidsArcFacility => "3U",
                AidsArcUnit => "3V",
                SpecializedOutpatientProgramForAidsArc => "3W",
                AlcoholDrugAbuseOrDependencyInpatientUnit => "3X",
                AlcoholDrugAbuseOrDependencyOutpatientServices => "3Y",
                ArthritisTreatmentCenter => "3Z",
                AssetAccountHolder => "04",
                SiteOperator => "004",
                BirthingRoomLdrpRoom => "4A",
                BurnCareUnit => "4B",
                CardiacCatherizationLaboratory => "4C",
                OpenHeartSurgeryFacility => "4D",
                CardiacIntensiveCareUnit => "4E",
                AngioplastyFacility => "4F",
                ChronicObstructivePulmonaryDiseaseServiceFacility => "4G",
                EmergencyDepartment => "4H",
                Code4I => "4I",
                Code4J => "4J",
                FitnessCenter => "4K",
                GeneticCounselingScreeningServices => "4L",
                AdultDayCareProgramFacility => "4M",
                AlzheimersDiagnosticAssessmentServices => "4N",
                ComprehensiveGeriatricAssessmentFacility => "4O",
                Code4P => "4P",
                GeriatricAcuteCareUnit => "4Q",
                GeriatricClinics => "4R",
                RespiteCareFacility => "4S",
                SeniorMembershipProgram => "4T",
                PatientEducationUnit => "4U",
                CommunityHealthPromotionFacility => "4V",
                WorksiteHealthPromotionFacility => "4W",
                HemodialysisFacility => "4X",
                HomeHealthServices => "4Y",
                Hospice => "4Z",
                Tenant => "05",
                ConstructionContractor => "005",
                MedicalSurgicalOrOtherIntensiveCareUnit => "5A",
                HisopathologyLaboratory => "5B",
                BloodBank => "5C",
                NeonatalIntensiveCareUnit => "5D",
                ObstetricsUnit => "5E",
                OccupationalHealthServices => "5F",
                OrganizedOutpatientServices => "5G",
                PediatricAcuteInpatientUnit => "5H",
                PsychiatricChildAdolescentServices => "5I",
                PsychiatricConsultationLiaisonServices => "5J",
                PsychiatricEducationServices => "5K",
                PsychiatricEmergencyServices => "5L",
                PsychiatricGeriatricServices => "5M",
                PsychiatricInpatientUnit => "5N",
                PsychiatricOutpatientServices => "5O",
                PsychiatricPartialHospitalizationProgram => "5P",
                MegavoltageRadiationTherapyUnit => "5Q",
                RadioactiveImplantsUnit => "5R",
                TherapeuticRadioisotopeFacility => "5S",
                XRayRadiationTherapyUnit => "5T",
                CtScannerUnit => "5U",
                DiagnosticRadioisotopeFacility => "5V",
                Code5W => "5W",
                UltrasoundUnit => "5X",
                RehabilitationInpatientUnit => "5Y",
                RehabilitationOutpatientServices => "5Z",
                RecipientOfCivilOrLegalLiabilityPayment => "06",
                DrillingContractor => "006",
                ReproductiveHealthServices => "6A",
                SkilledNursingOrOtherLongTermCareUnit => "6B",
                Code6C => "6C",
                OrganizedSocialWorkServiceFacility => "6D",
                OutpatientSocialWorkServices => "6E",
                EmergencyDepartmentSocialWorkServices => "6F",
                SportsMedicineClinicServices => "6G",
                HospitalAuxiliaryUnit => "6H",
                PatientRepresentativeServices => "6I",
                VolunteerServicesDepartment => "6J",
                OutpatientSurgeryServices => "6K",
                OrganTissueTransplantUnit => "6L",
                OrthopedicSurgeryFacility => "6M",
                OccupationalTherapyServices => "6N",
                PhysicalTherapyServices => "6O",
                RecreationalTherapyServices => "6P",
                RespiratoryTherapyServices => "6Q",
                SpeechTherapyServices => "6R",
                WomensHealthCenterServices => "6S",
                HealthSciencesLibrary => "6T",
                CardiacRehabilitationProgramFacility => "6U",
                NonInvasiveCardiacAssessmentServices => "6V",
                EmergencyMedicalTechnician => "6W",
                DisciplinaryContact => "6X",
                CaseManager => "6Y",
                Advisor => "6Z",
                Titleholder => "07",
                SpudContractor => "007",
                Premises => "7A",
                Bottler => "7B",
                PlaceOfOccurrence => "7C",
                ContractingOfficerRepresentative => "7D",
                PartyAuthorizedToDefinitizeContractAction => "7E",
                FilingAddress => "7F",
                HazardousMaterialOffice => "7G",
                GovernmentFurnishedPropertyFobPoint => "7H",
                ProjectName => "7I",
                Codefendant => "7J",
                CoOccupant => "7K",
                PreliminaryInspectionLocation => "7L",
                InspectionAndAcceptanceLocation => "7M",
                PartyToReceiveProposal => "7N",
                FederallyCharteredFacility => "7O",
                TransportationOffice => "7P",
                PartyToWhomProtestSubmitted => "7Q",
                Birthplace => "7R",
                PipelineSegment => "7S",
                HomeStateName => "7T",
                Liquidator => "7U",
                PetitioningCreditorsAttorney => "7V",
                MergedName => "7W",
                PartyRepresented => "7X",
                ProfessionalOrganization => "7Y",
                Referee => "7Z",
                NonMortgageLiabilityAccountHolder => "08",
                LienHolder => "008",
                VacationHome => "8A",
                PrimaryResidence => "8B",
                SecondHome => "8C",
                PermitHolder => "8D",
                MinorityInstitution => "8E",
                BailmentWarehouse => "8F",
                FirstAppraiser => "8G",
                TaxExemptOrganization => "8H",
                ServiceOrganization => "8I",
                EmergingSmallBusiness => "8J",
                SurplusDealer => "8K",
                PollingSite => "8L",
                SociallyDisadvantagedIndividual => "8M",
                EconomicallyDisadvantagedIndividual => "8N",
                DisabledIndividual => "8O",
                Producer => "8P",
                PublicOrPrivateOrganizationForTheDisabled => "8Q",
                Code8R => "8R",
                Code8S => "8S",
                Voter => "8T",
                NativeHawaiianOrganization => "8U",
                Code8V => "8V",
                PaymentAddress => "8W",
                OilAndGasCustodian => "8X",
                RegisteredOffice => "8Y",
                NoteCoSigner => "09",
                DebtorIndividual => "9A",
                CountryOfExport => "9B",
                CountryOfDestination => "9C",
                NewServiceProvider => "9D",
                SubServicer => "9E",
                LossPayee => "9F",
                Nickname => "9G",
                Assignee => "9H",
                RegisteredPrincipal => "9I",
                AdditionalDebtor => "9J",
                KeyPerson => "9K",
                IncorporatedBy => "9L",
                PartyToLease => "9N",
                PartyToContract => "9O",
                Investigator => "9P",
                LastSupplier => "9Q",
                DownstreamFirstSupplier => "9R",
                CoInvestigator => "9S",
                TelephoneAnsweringServiceBureau => "9T",
                Author => "9U",
                FirstSupplier => "9V",
                UltimateParentCompany => "9W",
                ContractualReceiptMeter => "9X",
                ContractualDeliveryMeter => "9Y",
                CoDebtor => "9Z",
                Conduit => "10",
                Code11 => "11",
                RegionalOffice => "12",
                ContractedServiceProvider => "13",
                WhollyOwnedSubsidiary => "14",
                AccountsPayableOffice => "15",
                Plant => "16",
                ConsultantsOffice => "17",
                Production => "18",
                NonProductionSupplier => "19",
                ForeignSupplier => "20",
                SmallBusiness => "21",
                Code22 => "22",
                Code23 => "23",
                Code24 => "24",
                Code25 => "25",
                SociallyDisadvantagedBusiness => "26",
                SmallDisadvantagedBusiness => "27",
                Subcontractor => "28",
                PrototypeSupplier => "29",
                ServiceSupplier => "30",
                PostalMailingAddress => "31",
                PartyToReceiveMaterialRelease => "32",
                InquiryAddress => "33",
                MaterialChangeNoticeAddress => "34",
                Code35 => "35",
                Employer => "36",
                PreviousDebtHolder => "37",
                MortgageLiabilityAccountHolder => "38",
                AppraisalCompany => "39",
                Receiver => "40",
                Submitter => "41",
                ComponentManufacturer => "42",
                ClaimantAuthorizedRepresentative => "43",
                DataProcessingServiceBureau => "44",
                DropOffLocation => "45",
                InvoicingDealer => "46",
                Estimator => "47",
                InServiceSource => "48",
                InitialDealer => "49",
                ManufacturersRepresentative => "50",
                PartsDistributor => "51",
                PartRemanufacturer => "52",
                RegisteredOwner => "53",
                OrderWriter => "54",
                ServiceManager => "55",
                ServicingDealer => "56",
                ServicingOrganization => "57",
                StoreManager => "58",
                PartyToApproveSpecification => "59",
                Salesperson => "60",
                PerformedAt => "61",
                ApplicantsEmployer => "62",
                ReferencesEmployer => "63",
                CosignersEmployer => "64",
                ApplicantsReference => "65",
                ApplicantsCosigner => "66",
                ApplicantsComaker => "67",
                OwnersRepresentative => "68",
                RepairingOutlet => "69",
                PriorIncorrectInsured => "70",
                AttendingPhysician => "71",
                OperatingPhysician => "72",
                OtherPhysician => "73",
                CorrectedInsured => "74",
                Participant => "75",
                SecondaryWarranter => "76",
                ServiceLocation => "77",
                ServiceRequester => "78",
                Warranter => "79",
                Hospital => "80",
                PartSource => "81",
                RenderingProvider => "82",
                SubscribersSchool => "83",
                SubscribersEmployer => "84",
                BillingProvider => "85",
                Conductor => "86",
                PayToProvider => "87",
                Approver => "88",
                Investor => "89",
                PreviousBusinessPartner => "90",
                ActionParty => "91",
                SupportParty => "92",
                InsuranceInstitute => "93",
                NewSupplySource => "94",
                ResearchInstitute => "95",
                DebtorCompany => "96",
                PartyWaivingRequirements => "97",
                FreightManagementFacilitator => "98",
                Code99 => "99",
                Adjuster => "A1",
                WomanOwnedBusiness => "A2",
                LaborSurplusAreaFirm => "A3",
                OtherDisadvantagedBusiness => "A4",
                VeteranOwnedBusiness => "A5",
                CodeA6 => "A6",
                ShelteredWorkshop => "A7",
                NonprofitInstitution => "A8",
                SalesOffice => "A9",
                AuthorityForShipment => "AA",
                CodeAA1 => "AA1",
                FinancialAidOffice => "AA2",
                Respondent => "AA3",
                AdmissionOffice => "AA4",
                MultiCampusAdministrativeUnit => "AA5",
                Headmaster => "AA6",
                BusinessOfficer => "AA7",
                Superintendent => "AA8",
                SchoolPrincipal => "AA9",
                SubAccount => "AAA",
                ManagementNonOfficer => "AAB",
                IncorporatedLocation => "AAC",
                NameNotToBeConfusedWith => "AAD",
                Lot => "AAE",
                PreviousOccupant => "AAF",
                GroundAmbulanceServices => "AAG",
                AirAmbulanceServices => "AAH",
                WaterAmbulanceServices => "AAI",
                AdmittingServices => "AAJ",
                PrimarySurgeon => "AAK",
                MedicalNurse => "AAL",
                CardiacRehabilitationServices => "AAM",
                SkilledNursingServices => "AAN",
                ObservationRoomServices => "AAO",
                Employee => "AAP",
                AnesthesiologyServices => "AAQ",
                PriorBaseJurisdiction => "AAS",
                IncorporationJurisdiction => "AAT",
                MarkerOwner => "AAU",
                ReclamationCenter => "AAV",
                PartyProvidingFinancing => "AAW",
                AdditionalPickupAddress => "AB",
                PrivateSchoolSystem => "AB1",
                StateOperatedSchoolSystem => "AB2",
                VocationalRegionsSchoolSystem => "AB3",
                CharteredSchoolDistrict => "AB4",
                SchoolingOfIndianChildrenSchoolSystem => "AB5",
                UnorganizedTerritoriesSchoolSystem => "AB6",
                StateAdministeredDistrict => "AB7",
                TownsInUnionsSchoolSystem => "AB8",
                AgentTownsSchoolSystem => "AB9",
                MasterProperty => "ABB",
                ProjectProperty => "ABC",
                UnitProperty => "ABD",
                AdditionalAddress => "ABE",
                SocietyOfPropertyInformationCompilersAndAnalysts => "ABF",
                Organization => "ABG",
                JointOwnerAnnuitant => "ABH",
                JointAnnuitantOwner => "ABI",
                JointOwnerAnnuitantPayor => "ABJ",
                JointOwnerJointAnnuitant => "ABK",
                JointOwnerJointAnnuitantPayor => "ABL",
                JointOwnerPayor => "ABM",
                Acronym => "ABN",
                NewAddress => "ABO",
                Chairperson => "ABP",
                DecisionMaker => "ABQ",
                FormerPresident => "ABR",
                Founder => "ABS",
                ImportedFromLocation => "ABT",
                LiterallyTranslatedName => "ABU",
                OriginalLocation => "ABV",
                President => "ABW",
                RatingOrganization => "ABX",
                AirCargoCompany => "AC",
                RegionalCenter => "AC1",
                CodeAC2 => "AC2",
                StateEducationAgency => "AC3",
                InitialMedicalProvider => "ACB",
                ConcurrentEmployer => "ACC",
                RoutingPoint => "ACE",
                BorderCrossing => "ACF",
                BobtailServicePoint => "ACG",
                Auditor => "ACH",
                InsuredLocation => "ACI",
                ReferralProvider => "ACJ",
                Affiliate => "ACK",
                AlliedHealthProfessional => "ACL",
                EmergencyProvider => "ACM",
                FederalGovernment => "ACN",
                FellowshipInstitution => "ACO",
                GovernmentCombinedControl => "ACP",
                GovernmentFederalMilitary => "ACQ",
                GovernmentFederalOther => "ACR",
                GovernmentFederalVeterans => "ACS",
                GovernmentLocal => "ACT",
                GroupAffiliation => "ACU",
                InformationSource => "ACV",
                InternshipEntity => "ACW",
                MedicalSchool => "ACX",
                NationalOrganization => "ACY",
                CodeACZ => "ACZ",
                CodeAD => "AD",
                CodeADA => "ADA",
                ForProfitHealthCareProvider => "ADB",
                OfficeManager => "ADC",
                OnCallProvider => "ADD",
                CodeADE => "ADE",
                CodeADF => "ADF",
                ResidencyInstitution => "ADH",
                SharedService => "ADJ",
                SupportingPersonnel => "ADK",
                TrainingInstitution => "ADL",
                PublicSchool => "ADM",
                PrivateSchool => "ADN",
                PublicPreKEducation => "ADO",
                PrivatePreKEducation => "ADP",
                PreKDayCare => "ADQ",
                CharterSchool => "ADR",
                HomeSchool => "ADS",
                PublicAlternativeSchool => "ADT",
                NeglectedDelinquentInstitution => "ADU",
                PostSecondaryInstitution => "ADV",
                FoodServiceOperator => "ADW",
                FutureAddress => "ADX",
                FormerRegisteredAddress => "ADY",
                TopParentCompanyInSameCountry => "ADZ",
                AdditionalDeliveryAddress => "AE",
                SecondLevelParentCompany => "AEA",
                AirportAuthority => "AEB",
                CouncilOfGovernments => "AEC",
                Foundation => "AED",
                PortAuthority => "AEE",
                PlanningCommission => "AEF",
                CarRentalLocation => "AEG",
                LodgingFacility => "AEI",
                PartyToReceiveTransportationCredit => "AEJ",
                CodeAEK => "AEK",
                PrimaryInternationalTelecomCarrier => "AEL",
                AuthorizedAcceptingOfficial => "AF",
                AgentAgency => "AG",
                Advertiser => "AH",
                AgencyHazardousMaterialInformationSystemLocation => "AHM",
                Airline => "AI",
                AllegedDebtor => "AJ",
                PartyToWhomAcknowledgmentShouldBeSent => "AK",
                AllotmentCustomer => "AL",
                AlternativeAddressee => "ALA",
                ActivityLocation => "ALO",
                AssistantUSTrustee => "AM",
                AuthorizedFrom => "AN",
                AccountOf => "AO",
                CodeAP => "AP",
                ActivityProvider => "APR",
                CodeAQ => "AQ",
                ArmedServicesLocationDesignation => "AR",
                PostsecondaryEducationSender => "AS",
                PostsecondaryEducationRecipient => "AT",
                AlternateTaxAuthority => "ATA",
                PartyAuthorizingDisposition => "AU",
                AuthorizingOfficial => "AUO",
                AuthorizedTo => "AV",
                Accountant => "AW",
                Plaintiff => "AX",
                Clearinghouse => "AY",
                PreviousName => "AZ",
                ConstructionFirm => "B1",
                OtherUnlistedTypeOfOrganizationalEntity => "B2",
                PreviousNameOfFirm => "B3",
                ParentCompany => "B4",
                AffiliatedCompany => "B5",
                RegisteringParentParty => "B6",
                RegisteringNonparentParty => "B7",
                RegularDealer => "B8",
                LargeBusiness => "B9",
                Battery => "BA",
                Bailiff => "BAL",
                BusinessPartner => "BB",
                Broadcaster => "BC",
                BillToPartyForDiversionCharges => "BD",
                Beneficiary => "BE",
                BilledFrom => "BF",
                BuyingGroup => "BG",
                InterimTrustee => "BH",
                TrusteesAttorney => "BI",
                CoCounsel => "BJ",
                Bank => "BK",
                Bookkeeper => "BKR",
                PartyToReceiveBillOfLading => "BL",
                Building => "BLD",
                Structure => "BLT",
                Brakeman => "BM",
                BeneficialOwner => "BN",
                BrokerOrSalesOffice => "BO",
                BodyOfWater => "BOW",
                SpecialCounsel => "BP",
                AttorneyForDefendantPrivate => "BQ",
                Broker => "BR",
                BrandName => "BRN",
                BillAndShipTo => "BS",
                BillToParty => "BT",
                PlaceOfBusiness => "BU",
                Business => "BUS",
                BillingService => "BV",
                Borrower => "BW",
                AttorneyForPlaintiff => "BX",
                CodeBY => "BY",
                BusinessAssociate => "BZ",
                AssistantConductor => "C0",
                InCareOfPartyNo1 => "C1",
                InCareOfPartyNo2 => "C2",
                CircuitLocation => "C3",
                ContractAdministrationOffice => "C4",
                SecondaryContractAdministrationOffice => "C4A",
                PartySubmittingQuote => "C5",
                Municipality => "C6",
                County => "C7",
                City => "C8",
                ContractHolder => "C9",
                Carrier => "CA",
                CustomsBroker => "CB",
                Claimant => "CC",
                CodeCD => "CD",
                CodeCE => "CE",
                SubsidiaryDivision => "CF",
                CarnetIssuer => "CG",
                ChassisProvider => "CH",
                ChangedAddress => "CHA",
                Consignor => "CI",
                CodeCJ => "CJ",
                Pharmacist => "CK",
                ContainerLocation => "CL",
                BuildingCluster => "CLT",
                Customs => "CM",
                CompanyMergedWith => "CMW",
                Consignee => "CN",
                ConfirmingParty => "CNP",
                ConfirmationRequester => "CNR",
                ConfirmationServiceIdentifierCode => "CNS",
                OceanTariffConference => "CO",
                CoDriver => "COD",
                CollateralAssignee => "COL",
                Complainant => "COM",
                CorrectedName => "COR",
                PartyToReceiveCertOfCompliance => "CP",
                CorporateOffice => "CQ",
                ContainerReturnCompany => "CR",
                CrewMember => "CRW",
                Consolidator => "CS",
                CountryOfOrigin => "CT",
                CoatingOrPaintSupplier => "CU",
                Converter => "CV",
                AccountingStation => "CW",
                ClaimAdministrator => "CX",
                Country => "CY",
                AdmittingSurgeon => "CZ",
                Driver => "D1",
                CommercialInsurer => "D2",
                Defendant => "D3",
                Debtor => "D4",
                DebtorInPossession => "D5",
                ConsolidatedDebtor => "D6",
                PetitioningCreditor => "D7",
                Dispatcher => "D8",
                CreditorsAttorney => "D9",
                DeliveryAddress => "DA",
                DamagedBy => "DAM",
                DistributorBranch => "DB",
                DestinationCarrier => "DC",
                ChiefDeputyClerkOfCourt => "DCC",
                AssistantSurgeon => "DD",
                Depositor => "DE",
                MaterialDispositionAuthorizationLocation => "DF",
                DesignEngineering => "DG",
                DoingBusinessAs => "DH",
                CodeDI => "DI",
                DistributionRecipient => "DIR",
                ConsultingPhysician => "DJ",
                OrderingPhysician => "DK",
                Dealer => "DL",
                DestinationMailFacility => "DM",
                ReferringProvider => "DN",
                DependentName => "DO",
                PartyToProvideDiscount => "DP",
                SupervisingPhysician => "DQ",
                DestinationDrayman => "DR",
                Distributor => "DS",
                DestinationTerminal => "DT",
                ResaleDealer => "DU",
                Division => "DV",
                DownstreamParty => "DW",
                Distiller => "DX",
                DefaultForeclosureSpecialist => "DY",
                DeliveryZone => "DZ",
                AssistantEngineer => "E0",
                PersonOrOtherEntityLegallyResponsibleForAChild => "E1",
                PersonOrOtherEntityWithWhomAChildResides => "E2",
                PersonOrOtherEntityLegallyResponsibleForAndWithWhomAChildResides => "E3",
                OtherPersonOrEntityAssociatedWithStudent => "E4",
                Examiner => "E5",
                Engineering => "E6",
                PreviousEmployer => "E7",
                InquiringParty => "E8",
                ParticipatingLaboratory => "E9",
                StudySubmitter => "EA",
                Assistant => "EAA",
                CampaignManager => "EAB",
                Client => "EAD",
                Commissioner => "EAE",
                Committee => "EAF",
                Contestant => "EAG",
                Contributor => "EAH",
                DeputyChairperson => "EAI",
                DeputyTreasurer => "EAJ",
                Donor => "EAK",
                Endorser => "EAL",
                Guarantor => "EAM",
                Headquarters => "EAN",
                IndependentContractor => "EAO",
                Leader => "EAP",
                PartyPerformingLiaison => "EAQ",
                LobbyingFirm => "EAR",
                Lobbyist => "EAS",
                MediaContact => "EAT",
                OfficeHolder => "EAU",
                PartyAuthorizedToAdministerOaths => "EAV",
                PartyToBenefit => "EAW",
                PartyHoldingInterest => "EAX",
                PartyMakingPledge => "EAY",
                PartyReturningContribution => "EAZ",
                EligiblePartyToTheContract => "EB",
                PartyReturningTransfer => "EBA",
                LobbiedParty => "EBB",
                PoliticalActionCommittee => "EBC",
                PoliticalParty => "EBD",
                Proponent => "EBE",
                PublicOfficial => "EBF",
                ReceivingCommittee => "EBG",
                AffiliatedCommittee => "EBH",
                Source => "EBI",
                Sponsor => "EBJ",
                SponsoredCommittee => "EBK",
                Designee => "EBL",
                TemporaryResidence => "EBM",
                Treasurer => "EBN",
                ViceChairperson => "EBO",
                SlateMailerOrganization => "EBP",
                LodgingLocation => "EBQ",
                IndependentExpenditureCommittee => "EBR",
                MajorDonor => "EBS",
                Exchanger => "EC",
                ExcludedParty => "ED",
                LocationOfGoodsForCustomsExaminationBeforeClearance => "EE",
                ElectronicFiler => "EF",
                Engineer => "EG",
                Exhibitor => "EH",
                ExecutorOfEstate => "EI",
                PrincipalPerson => "EJ",
                AnimalSource => "EK",
                EstablishedLocation => "EL",
                PartyToReceiveElectronicMemoOfInvoice => "EM",
                EndUser => "EN",
                Enroller => "ENR",
                LimitedLiabilityPartnership => "EO",
                EligiblePartyToTheRate => "EP",
                OldDebtor => "EQ",
                NewDebtor => "ER",
                PlanAdministrator => "ET",
                OldSecuredParty => "EU",
                SellingAgent => "EV",
                ServicingBroker => "EW",
                Exporter => "EX",
                ExSpouse => "EXS",
                EmployeeName => "EY",
                NewSecuredParty => "EZ",
                CompanyOwnedOilField => "F1",
                CodeF2 => "F2",
                CodeF3 => "F3",
                FormerResidence => "F4",
                RadioControlStationLocation => "F5",
                SmallControlStationLocation => "F6",
                SmallBaseStationLocation => "F7",
                AntennaSite => "F8",
                AreaOfOperation => "F9",
                Facility => "FA",
                FirstBreakTerminal => "FB",
                CodeFC => "FC",
                PhysicalAddress => "FD",
                MailAddress => "FE",
                ForeignLanguageSynonym => "FF",
                TradeNameSynonym => "FG",
                ForeignGovernment => "FGT",
                PartyToReceiveLimitationsOfHeavyElementsReport => "FH",
                NameVariationSynonym => "FI",
                FirstContact => "FJ",
                PrimaryControlPointLocation => "FL",
                Fireman => "FM",
                FilerName => "FN",
                FieldOrBranchOffice => "FO",
                NameOnCreditCard => "FP",
                PierName => "FQ",
                MessageFrom => "FR",
                ForeignRegistrationLocation => "FRL",
                FinalScheduledDestination => "FS",
                PartyToReceiveSensitiveForeignDisclosure => "FSI",
                FinancialStatementRecipient => "FSR",
                NewAssignee => "FT",
                OldAssignee => "FU",
                VesselName => "FV",
                Forwarder => "FW",
                ClosedDoorPharmacy => "FX",
                VeterinaryHospital => "FY",
                ChildrensDayCareCenter => "FZ",
                DependentInsured => "G0",
                BankruptcyTrustee => "G1",
                Annuitant => "G2",
                Clinic => "G3",
                ContingentBeneficiary => "G5",
                EntityHoldingThe => "G6",
                EntityProvidingTheService => "G7",
                EntityResponsibleForFollowUp => "G8",
                FamilyMember => "G9",
                GasPlant => "GA",
                OtherInsured => "GB",
                AlternateGovernmentBusinessContact => "GBA",
                GateBooth => "GBO",
                PrimaryGovernmentBusinessContact => "GBP",
                PreviousCreditGrantor => "GC",
                Guardian => "GD",
                GeneralAgency => "GE",
                InspectionCompany => "GF",
                Intermediary => "GG",
                MotorVehicleReportProviderCompany => "GH",
                Paramedic => "GI",
                GiftRecipient => "GIR",
                ParamedicalCompany => "GJ",
                PreviousInsured => "GK",
                PreviousResidence => "GL",
                SpouseInsured => "GM",
                Garnishee => "GN",
                PrimaryBeneficiary => "GO",
                GatewayProvider => "GP",
                ProposedInsured => "GQ",
                Reinsurer => "GR",
                GaragedLocation => "GS",
                CreditGrantor => "GT",
                GuaranteeAgency => "GU",
                GasTransactionEndingPoint => "GV",
                Group => "GW",
                Retrocessionaire => "GX",
                TreatmentFacility => "GY",
                Grandparent => "GZ",
                Representative => "H1",
                SubOffice => "H2",
                District => "H3",
                PayingAgent => "H5",
                SchoolDistrict => "H6",
                GroupAffiliate => "H7",
                Designer => "H9",
                Owner => "HA",
                HistoricallyBlackCollegeOrUniversity => "HB",
                JointAnnuitant => "HC",
                ContingentAnnuitant => "HD",
                ContingentOwner => "HE",
                CodeHF => "HF",
                BrokerOpinionOrAnalysisRequester => "HG",
                HomeHealthAgency => "HH",
                ListingCompany => "HI",
                AutomatedUnderwritingSystem => "HJ",
                Subscriber => "HK",
                DocumentCustodian => "HL",
                CompetitivePropertyListing => "HM",
                CodeHMI => "HMI",
                CompetingProperty => "HN",
                ComparablePropertyListing => "HO",
                HomeOffice => "HOM",
                HonorarySociety => "HON",
                ClosedSale => "HP",
                SourcePartyOf => "HQ",
                SubjectOfInquiry => "HR",
                HighSchool => "HS",
                StateCharteredFacility => "HT",
                Subsidiary => "HU",
                TaxAddress => "HV",
                DesignatedHazardousWasteFacility => "HW",
                TransporterOfHazardousWaste => "HX",
                Charity => "HY",
                HazardousWasteGenerator => "HZ",
                InterestedParty => "I1",
                CodeI3 => "I3",
                IntellectualPropertyOwner => "I4",
                Interviewer => "I9",
                InstalledAt => "IA",
                BusinessEntity => "IAA",
                PrincipalExecutiveOffice => "IAC",
                ForeignOffice => "IAD",
                Member => "IAE",
                ExecutiveCommitteeMember => "IAF",
                Director => "IAG",
                Clerk => "IAH",
                PartyWithKnowledgeOfAffairsOfTheCompany => "IAI",
                PartyToReceiveStatementOfFeesDue => "IAK",
                CompanyInWhichInterestHeld => "IAL",
                CompanyWhichHoldsInterest => "IAM",
                Notary => "IAN",
                Manager => "IAO",
                AlienAffiliate => "IAP",
                IncorporationStatePrincipalOffice => "IAQ",
                IncorporationStatePlaceOfBusiness => "IAR",
                OutOfStatePrincipalOffice => "IAS",
                PartyExecutingAndVerifying => "IAT",
                Felon => "IAU",
                OtherRelatedParty => "IAV",
                RecordKeepingAddress => "IAW",
                InitialSubscriber => "IAY",
                OriginalJurisdiction => "IAZ",
                IndustryBureau => "IB",
                IntermediateConsignee => "IC",
                InventoryControlPoint => "ICP",
                IssuerOfDebitOrCreditMemo => "ID",
                OtherIndividualDisabilityCarrier => "IE",
                InternationalFreightForwarder => "IF",
                InsolventInsurer => "IG",
                IssuerOfInvoice => "II",
                InjectionPoint => "IJ",
                IntermediateCarrier => "IK",
                InsuredOrSubscriber => "IL",
                Importer => "IM",
                IntegratedMaterialManager => "IMM",
                Insurer => "IN",
                Interviewee => "INT",
                InvestmentAdvisor => "INV",
                Inspector => "IO",
                IndependentAdjuster => "IP",
                InPatientPharmacy => "IQ",
                SelfInsured => "IR",
                PartyToReceiveCertifiedInspectionReport => "IS",
                InstallationOnSite => "IT",
                Issuer => "IU",
                Renter => "IV",
                AssociateGeneralAgent => "J1",
                AuthorizedEntity => "J2",
                BrokersAssistant => "J3",
                Custodian => "J4",
                IrrevocableBeneficiary => "J5",
                PowerOfAttorney => "J6",
                TrustOfficer => "J7",
                BrokerDealer => "J8",
                CommunityAgent => "J9",
                DairyDepartment => "JA",
                DelicatessenDepartment => "JB",
                DryGroceryDepartment => "JC",
                Judge => "JD",
                FrozenDepartment => "JE",
                GeneralMerchandiseDepartment => "JF",
                CodeJG => "JG",
                AlcoholBeverageDepartment => "JH",
                MeatDepartment => "JI",
                ProduceDepartment => "JJ",
                BakeryDepartment => "JK",
                VideoDepartment => "JL",
                CandyAndConfectionsDepartment => "JM",
                CigarettesAndTobaccoDepartment => "JN",
                InStoreBakeryDepartment => "JO",
                FloralDepartment => "JP",
                PharmacyDepartment => "JQ",
                Bidder => "JR",
                JointDebtorAttorney => "JS",
                JointDebtor => "JT",
                Jurisdiction => "JU",
                JointOwner => "JV",
                JointVenture => "JW",
                ClosingAgent => "JX",
                FinancialPlanner => "JY",
                ManagingGeneralAgent => "JZ",
                ContractorCognizantSecurityOffice => "K1",
                SubcontractorCognizantSecurityOffice => "K2",
                PlaceOfPerformanceCognizantSecurityOffice => "K3",
                PartyAuthorizingReleaseOfSecurity => "K4",
                PartyToReceiveContractSecurityClassificationSpecification => "K5",
                PolicyWritingAgent => "K6",
                RadioStation => "K7",
                FilingLocation => "K8",
                PreviousDistributor => "K9",
                ItemManager => "KA",
                CustomerForWhomSameOrSimilarWorkWasPerformed => "KB",
                PartyThatReceivedDisclosureStatement => "KC",
                Proposer => "KD",
                ContactOffice => "KE",
                AuditOffice => "KF",
                ProjectManager => "KG",
                OrganizationHavingSourceControl => "KH",
                UnitedStatesOverseasSecurityAdministrationOffice => "KI",
                QualifyingOfficer => "KJ",
                RegisteringParty => "KK",
                ClerkOfCourt => "KL",
                Coordinator => "KM",
                FormerAddress => "KN",
                PlantClearanceOfficer => "KO",
                NameUnderWhichFiled => "KP",
                Licensee => "KQ",
                PreKindergartenToGrade12Recipient => "KR",
                PreKindergartenToGrade12Sender => "KS",
                Court => "KT",
                ReceiverSite => "KU",
                DisbursingOfficer => "KV",
                BidOpeningLocation => "KW",
                FreeOnBoardPoint => "KX",
                TechnicalOffice => "KY",
                AcceptanceLocation => "KZ",
                InspectionLocation => "L1",
                LocationOfPrincipalAssets => "L2",
                LoanCorrespondent => "L3",
                Contact => "L5",
                HeadOffice => "L8",
                InformationProvider => "L9",
                Attorney => "LA",
                LastBreakTerminal => "LB",
                LocationOfSpotForStorage => "LC",
                GasNominationLocation => "LCN",
                LiabilityHolder => "LD",
                Lessor => "LE",
                LimitedPartner => "LF",
                LocationOfGoods => "LG",
                LocalGovernmentSponsor => "LGS",
                Pipeline => "LH",
                IndependentLab => "LI",
                LimitedLiabilityCompany => "LJ",
                JuvenileOwner => "LK",
                CodeLL => "LL",
                LendingInstitution => "LM",
                Lender => "LN",
                LoanOriginator => "LO",
                LoadingParty => "LP",
                LawFirm => "LQ",
                LegalRepresentative => "LR",
                Lessee => "LS",
                LongTermDisabilityCarrier => "LT",
                MasterAgent => "LU",
                LoanServicer => "LV",
                Customer => "LW",
                Labeler => "LY",
                AmendedName => "LYM",
                Stockholder => "LYN",
                ManagingAgent => "LYO",
                Organizer => "LYP",
                LocalChain => "LZ",
                SourceMeterLocation => "M1",
                ReceiptLocation => "M2",
                UpstreamMeterLocation => "M3",
                DownstreamMeterLocation => "M4",
                MigrantHealthClinic => "M5",
                Landlord => "M6",
                ForeclosingLender => "M7",
                EducationalInstitution => "M8",
                Manufacturing => "M9",
                PartyForWhomItemIsUltimatelyIntended => "MA",
                CompanyInterviewerWorksFor => "MB",
                MotorCarrier => "MC",
                VeteransAdministrationLoanGuarantyAuthority => "MD",
                VeteransAdministrationLoanAuthorizedSupplier => "ME",
                ManufacturerOfGoods => "MF",
                GovernmentLoanAgencySponsorOrAgent => "MG",
                MortgageInsurer => "MH",
                PlanningScheduleMaterialReleaseIssuer => "MI",
                FinancialInstitution => "MJ",
                LoanHolderForRealEstateAsset => "MK",
                ConsumerCreditAccountCompany => "ML",
                MortgageCompany => "MM",
                AuthorizedMarketer => "MN",
                ReleaseDrayman => "MO",
                ManufacturingPlant => "MP",
                DeliveryLocation => "MQ",
                MedicalInsuranceCarrier => "MR",
                CodeMS => "MS",
                MammographyScreeningCenter => "MSC",
                Material => "MT",
                MeterLocation => "MTR",
                MeetingLocation => "MU",
                Mainline => "MV",
                MarineSurveyor => "MW",
                JuvenileWitness => "MX",
                MasterGeneralAgent => "MY",
                Minister => "MZ",
                NotifyPartyNo1 => "N1",
                NotifyPartyNo2 => "N2",
                IneligibleParty => "N3",
                PriceAdministration => "N4",
                PartyWhoSignedTheDeliveryReceipt => "N5",
                NonemploymentIncomeSource => "N6",
                PreviousNeighbor => "N7",
                Relative => "N8",
                Neighborhood => "N9",
                Neighbor => "NB",
                CrossTownSwitch => "NC",
                NameChangedTo => "NCT",
                NextDestination => "ND",
                Newspaper => "NE",
                OwnerAnnuitant => "NF",
                Administrator => "NG",
                Association => "NH",
                NonInsured => "NI",
                TrustOrEstate => "NJ",
                NationalChain => "NK",
                NonRailroadEntity => "NL",
                PhysicianSpecialists => "NM",
                NetworkName => "NN",
                NotifyPartyForShippersOrder => "NP",
                NotaryPublic => "NPC",
                PipelineSegmentBoundary => "NQ",
                GasTransactionStartingPoint => "NR",
                NonTemporaryStorageFacility => "NS",
                MagistrateJudge => "NT",
                FormerlyKnownAs => "NU",
                FormerlyDoingBusinessAs => "NV",
                MaidenName => "NW",
                PrimaryOwner => "NX",
                BirthName => "NY",
                PrimaryPhysician => "NZ",
                OriginatingBank => "O1",
                OriginatingCompany => "O2",
                ReceivingCompany => "O3",
                Factor => "O4",
                MerchantBanker => "O5",
                NonRegisteredBusinessName => "O6",
                RegisteredBusinessName => "O7",
                Registrar => "O8",
                ElectronicReturnOriginator => "OA",
                OrderedBy => "OB",
                OriginCarrier => "OC",
                DoctorOfOptometry => "OD",
                BookingOffice => "OE",
                OffsetOperator => "OF",
                CoOwner => "OG",
                OtherDepartments => "OH",
                OutsideInspectionAgency => "OI",
                Officer => "OL",
                OriginMailFacility => "OM",
                ProductPositionHolder => "ON",
                CodeOO => "OO",
                OperatorOfPropertyOrUnit => "OP",
                OriginDrayman => "OR",
                OriginalName => "ORI",
                CodeOS => "OS",
                OffSiteHandler => "OSH",
                OriginTerminal => "OT",
                OutsideProcessor => "OU",
                OtherUnlistedTypeOfCorporation => "OUC",
                OwnerOfVessel => "OV",
                OwnerOfPropertyOrUnit => "OW",
                OxygenTherapyFacility => "OX",
                OwnerOfVehicle => "OY",
                OutsideTestingAgency => "OZ",
                PatientFacility => "P0",
                Preparer => "P1",
                PrimaryInsuredOrSubscriber => "P2",
                PrimaryCareProvider => "P3",
                PriorInsuranceCarrier => "P4",
                PlanSponsor => "P5",
                CodeP6 => "P6",
                CodeP7 => "P7",
                PersonnelOffice => "P8",
                CodeP9 => "P9",
                PartyToReceiveInspectionReport => "PA",
                PayingBank => "PB",
                CodePC => "PC",
                PurchasersDepartmentBuyer => "PD",
                Payee => "PE",
                PartyToReceiveFreightBill => "PF",
                PrimeContractor => "PG",
                Printer => "PH",
                Publisher => "PI",
                PrimaryInventoryControlActivity => "PIC",
                PartyToReceiveCorrespondence => "PJ",
                PartyToReceiveCopy => "PK",
                PartyToReceivePurchaseOrder => "PL",
                LawEnforcementAgency => "PLC",
                PayerOfLastResort => "PLR",
                PartyToReceivePaperMemoOfInvoice => "PM",
                PriorMortgageCompany => "PMC",
                PartyManufacturedFor => "PMF",
                ProgramManager => "PMG",
                PartyToReceiveShippingNotice => "PN",
                PartyToReceiveInvoiceForGoodsOrServices => "PO",
                Property => "PP",
                PastPerformanceContact => "PPC",
                PersonForWhoseBenefitPropertyWasSeized => "PPS",
                PartyToReceiveInvoiceForLeasePayments => "PQ",
                Payer => "PR",
                PreviousOwner => "PRE",
                ProspectService => "PRO",
                PrimaryPayer => "PRP",
                PreviousStation => "PS",
                PartyToReceiveTestReport => "PT",
                PartyAtPickupLocation => "PU",
                PurchasedCompany => "PUR",
                PartyPerformingCertification => "PV",
                PickupAddress => "PW",
                PartyPerformingCount => "PX",
                PartyToFilePersonalPropertyTax => "PY",
                PartyToReceiveEquipment => "PZ",
                ConductorPilot => "Q1",
                EngineerPilot => "Q2",
                RetailAccount => "Q3",
                CooperativeBuyingGroup => "Q4",
                AdvertisingGroup => "Q5",
                Interpreter => "Q6",
                Partner => "Q7",
                BasePeriodEmployer => "Q8",
                LastEmployer => "Q9",
                Pharmacy => "QA",
                PurchaseServiceProvider => "QB",
                Patient => "QC",
                ResponsibleParty => "QD",
                Policyholder => "QE",
                Passenger => "QF",
                Pedestrian => "QG",
                Physician => "QH",
                PartyInPossession => "QI",
                CodeQJ => "QJ",
                ManagedCare => "QK",
                Chiropractor => "QL",
                DialysisCenters => "QM",
                Dentist => "QN",
                DoctorOfOsteopathy => "QO",
                PrincipalBorrower => "QP",
                QualityControl => "QQ",
                BuyersQualityReviewBoard => "QR",
                Podiatrist => "QS",
                Psychiatrist => "QT",
                Veterinarian => "QU",
                GroupPractice => "QV",
                Government => "QW",
                HomeHealthCorporation => "QX",
                MedicalDoctor => "QY",
                CoBorrower => "QZ",
                RoyaltyOwner => "R0",
                PartyToReceiveScaleTicket => "R1",
                ReportingOfficer => "R2",
                NextScheduledDestination => "R3",
                CodeR4 => "R4",
                CodeR5 => "R5",
                Requester => "R6",
                ConsumerReferralContact => "R7",
                CreditReportingAgency => "R8",
                RequestedLender => "R9",
                AlternateReturnAddress => "RA",
                ReceivingBank => "RB",
                ReceivingLocation => "RC",
                RecoveryRoom => "RCR",
                DestinationIntermodalRamp => "RD",
                ReceiverManager => "REC",
                Refinery => "RF",
                CodeRG => "RG",
                ResponsibleGovernmentAgency => "RGA",
                CodeRH => "RH",
                RemitTo => "RI",
                ResidenceOrDomicile => "RJ",
                RefineryOperator => "RK",
                ReportingLocation => "RL",
                PartyThatRemitsPayment => "RM",
                RepairOrRefurbishLocation => "RN",
                OriginalIntermodalRamp => "RO",
                ReceivingPointForCustomerSamples => "RP",
                ResaleCustomer => "RQ",
                Railroad => "RR",
                ClassIiRailroad => "RR2",
                ClassIiiRailroad => "RR3",
                ReceivingFacilityScheduler => "RS",
                ReturnedTo => "RT",
                ReceivingSubLocation => "RU",
                Reservoir => "RV",
                RuralHealthClinic => "RW",
                ResponsibleExhibitor => "RX",
                SpecifiedRepository => "RY",
                ReceiptZone => "RZ",
                SoleProprietor => "S0",
                Parent => "S1",
                Student => "S2",
                CustodialParent => "S3",
                SkilledNursingFacility => "S4",
                SecuredParty => "S5",
                AgencyGrantingSecurityClearance => "S6",
                SecuredPartyCompany => "S7",
                SecuredPartyIndividual => "S8",
                Sibling => "S9",
                SalvageCarrier => "SA",
                StorageArea => "SB",
                StoreClass => "SC",
                SoldToAndShipTo => "SD",
                SellingParty => "SE",
                SecondaryPayer => "SEP",
                ShipFrom => "SF",
                StoreGroup => "SG",
                Shipper => "SH",
                ShippingScheduleIssuer => "SI",
                SecondaryInventoryControlActivity => "SIC",
                ShipInPlaceLocation => "SIP",
                ServiceProvider => "SJ",
                CodeSK => "SK",
                OriginSublocation => "SL",
                PartyToReceiveShippingManifest => "SM",
                Store => "SN",
                CodeSNP => "SNP",
                SoldToIfDifferentFromBillTo => "SO",
                PartyFillingShippersOrder => "SP",
                ServiceBureau => "SQ",
                SamplesToBeReturnedTo => "SR",
                SteamshipCompany => "SS",
                ShipTo => "ST",
                SwitchingAndTerminalCarrier => "STC",
                SupplierManufacturer => "SU",
                SupplySource => "SUS",
                ServicePerformanceSite => "SV",
                SealingCompany => "SW",
                SchoolBasedServiceProvider => "SX",
                SecondaryTaxpayer => "SY",
                Supervisor => "SZ",
                OperatorOfTheTransferPoint => "T1",
                OperatorOfTheSourceTransferPoint => "T2",
                TerminalLocation => "T3",
                TransferPoint => "T4",
                TerminalOperator => "T6",
                PreviousTitleCompany => "T8",
                PriorTitleEvidenceHolder => "T9",
                TitleInsuranceServicesProvider => "TA",
                Tooling => "TB",
                ToolSource => "TC",
                ToolingDesign => "TD",
                Theatre => "TE",
                TaxExemptCorporation => "TEC",
                TankFarm => "TF",
                ToolingFabrication => "TG",
                TheaterCircuit => "TH",
                TariffIssuer => "TI",
                Cosigner => "TJ",
                TestSponsor => "TK",
                TestingLaboratory => "TL",
                Transmitter => "TM",
                Tradename => "TN",
                MessageTo => "TO",
                TowingAgency => "TOW",
                PrimaryTaxpayer => "TP",
                ThirdPartyMarketer => "TPM",
                CodeTQ => "TQ",
                Terminal => "TR",
                PartyToReceiveCertifiedTestResults => "TS",
                CodeTSD => "TSD",
                ConsigneeCourierTransferStation => "TSE",
                ConsignorCourierTransferStation => "TSR",
                TransferTo => "TT",
                TertiaryPayer => "TTP",
                CodeTU => "TU",
                CodeTV => "TV",
                TransitAuthority => "TW",
                TaxAuthority => "TX",
                Trustee => "TY",
                SignificantOther => "TZ",
                GasTransactionPoint1 => "U1",
                GasTransactionPoint2 => "U2",
                ServicingAgent => "U3",
                Team => "U4",
                Underwriter => "U5",
                TitleUnderwriter => "U6",
                Psychologist => "U7",
                Reference => "U8",
                NonRegisteredInvestmentAdvisor => "U9",
                PlaceOfBottling => "UA",
                PlaceOfDistilling => "UB",
                UltimateConsignee => "UC",
                Region => "UD",
                TestingService => "UE",
                HealthMiscellaneous => "UF",
                NursingHomeChain => "UG",
                NursingHome => "UH",
                RegisteredInvestmentAdvisor => "UI",
                SalesAssistant => "UJ",
                System => "UK",
                SpecialAccount => "UL",
                CodeUM => "UM",
                Union => "UN",
                CodeUO => "UO",
                UnloadingParty => "UP",
                SubsequentOwner => "UQ",
                Surgeon => "UR",
                UpstreamParty => "US",
                USTrustee => "UT",
                AnnuitantPayor => "UU",
                UnassignedAgent => "UW",
                BaseJurisdiction => "UX",
                Vehicle => "UY",
                Signer => "UZ",
                Surety => "V1",
                Grantor => "V2",
                WellPadConstructionContractor => "V3",
                OilAndGasRegulatoryAgency => "V4",
                SurfaceDischargeAgency => "V5",
                WellCasingDepthAuthority => "V6",
                MarketTimer => "V8",
                OwnerAnnuitantPayor => "V9",
                SecondContact => "VA",
                Candidate => "VB",
                VehicleCustodian => "VC",
                MultipleListingService => "VD",
                BoardOfRealtors => "VE",
                PartyPerformingVerification => "VER",
                SellingOffice => "VF",
                ListingAgent => "VG",
                ShowingAgent => "VH",
                ContactPerson => "VI",
                Victim => "VIC",
                OwnerJointAnnuitantPayor => "VJ",
                PropertyOrBuildingManager => "VK",
                BuilderName => "VL",
                Occupant => "VM",
                Vendor => "VN",
                ElementarySchool => "VO",
                PartyWithPowerToVoteSecurities => "VP",
                MiddleSchool => "VQ",
                JuniorHighSchool => "VR",
                VehicleSalvageAssignment => "VS",
                ListingOffice => "VT",
                SecondContactOrganization => "VU",
                OwnerPayor => "VV",
                Winner => "VW",
                ProductionManager => "VX",
                OrganizationCompletingConfigurationChange => "VY",
                WorkTeam => "W1",
                SupplierWorkTeam => "W2",
                ThirdPartyInvestmentAdvisor => "W3",
                Trust => "W4",
                InterlineServiceCommitmentCustomer => "W8",
                SamplingLocation => "W9",
                WritingAgent => "WA",
                AppraiserName => "WB",
                ComparableProperty => "WC",
                StorageFacilityAtDestination => "WD",
                SubjectProperty => "WE",
                TankFarmOwner => "WF",
                WageEarner => "WG",
                Warehouse => "WH",
                Witness => "WI",
                SupervisoryAppraiserName => "WJ",
                Wholesaler => "WL",
                CompanyAssignedWell => "WN",
                StorageFacilityAtOrigin => "WO",
                WitnessForPlaintiff => "WP",
                WithdrawalPoint => "WR",
                WaterSystem => "WS",
                WitnessForDefendant => "WT",
                PrimarySupportOrganization => "WU",
                PreliminaryMaintenancePeriodDesignatingOrganization => "WV",
                PreliminaryMaintenanceOrganization => "WW",
                PreliminaryReferredToOrganization => "WX",
                FinalMaintenancePeriodDesignatingOrganization => "WY",
                FinalMaintenanceOrganization => "WZ",
                MailTo => "X1",
                PartyToPerformPackaging => "X2",
                UtilizationManagementOrganization => "X3",
                Spouse => "X4",
                DurableMedicalEquipmentSupplier => "X5",
                InternationalOrganization => "X6",
                Inventor => "X7",
                HispanicServiceInstitute => "X8",
                Creditor => "XA",
                DebtorsAttorney => "XC",
                Alias => "XD",
                ClaimRecipient => "XE",
                Auctioneer => "XF",
                EventLocation => "XG",
                FinalReferredToOrganization => "XH",
                OriginalClaimant => "XI",
                ActualReferredByOrganization => "XJ",
                ActualReferredToOrganization => "XK",
                BorrowersEmployer => "XL",
                MaintenanceOrganizationUsedForEstimate => "XM",
                PlanningMaintenanceOrganization => "XN",
                PreliminaryCustomerOrganization => "XO",
                PartyToReceiveSolicitation => "XP",
                CanadianCustomsBroker => "XQ",
                MexicanCustomsBroker => "XR",
                SCorporation => "XS",
                FinalCustomerOrganization => "XT",
                UnitedStatesCustomsBroker => "XU",
                CrossClaimant => "XV",
                CounterClaimant => "XW",
                BusinessArea => "XX",
                TribalGovernment => "XY",
                AmericanIndianOwnedBusiness => "XZ",
                ManagedCareOrganization => "Y2",
                Affiant => "YA",
                Arbitrator => "YB",
                BailPayor => "YC",
                DistrictJustice => "YD",
                ThirdParty => "YE",
                WitnessForProsecution => "YF",
                ExpertWitness => "YG",
                CrimeVictim => "YH",
                JuvenileVictim => "YI",
                JuvenileDefendant => "YJ",
                Bondsman => "YK",
                CourtAppointedAttorney => "YL",
                ComplainantsAttorney => "YM",
                DistrictAttorney => "YN",
                CodeYO => "YO",
                ProBonoAttorney => "YP",
                ProSeCounsel => "YQ",
                PartyToAppearBefore => "YR",
                Appellant => "YS",
                Appellee => "YT",
                ArrestingOfficer => "YU",
                HostileWitness => "YV",
                DischargePoint => "YW",
                FloodCertifier => "YX",
                FloodDeterminationProvider => "YY",
                ElectronicRegistrationUtility => "YZ",
                PartyToReceiveStatus => "Z1",
                UnserviceableMaterialConsignee => "Z2",
                PotentialSourceOfSupply => "Z3",
                OwningInventoryControlPoint => "Z4",
                ManagementControlActivity => "Z5",
                TransferringParty => "Z6",
                MarkForParty => "Z7",
                LastKnownSourceOfSupply => "Z8",
                Banker => "Z9",
                CorrectedAddress => "ZA",
                PartyToReceiveCredit => "ZB",
                RentPayor => "ZC",
                PartyToReceiveReports => "ZD",
                EndItemManufacturer => "ZE",
                BreakBulkPoint => "ZF",
                PresentAddress => "ZG",
                Child => "ZH",
                Branch => "ZJ",
                Reporter => "ZK",
                PartyPassingTheTransaction => "ZL",
                LeaseLocation => "ZM",
                LosingInventoryManager => "ZN",
                MinimumRoyaltyPayor => "ZO",
                GainingInventoryManager => "ZP",
                ScreeningPoint => "ZQ",
                ValidatingParty => "ZR",
                MonitoringParty => "ZS",
                ParticipatingArea => "ZT",
                Formation => "ZU",
                AllowableRecipient => "ZV",
                Field => "ZW",
                AttorneyOfRecord => "ZX",
                AmicusCuriae => "ZY",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<EntityIdentifierCode> {
        use EntityIdentifierCode::*;
        match code {
            b"00" => Some(AlternateInsurer),
            b"0A" => Some(ComparableRentals),
            b"0B" => Some(InterimFundingOrganization),
            b"0D" => Some(NonOccupantCoBorrower),
            b"0E" => Some(ListOwner),
            b"0F" => Some(ListMailer),
            b"0G" => Some(PrimaryElectronicBusinessContact),
            b"0H" => Some(StateDivision),
            b"0I" => Some(AlternateElectronicBusinessContact),
            b"0J" => Some(PrimaryPracticeLocation),
            b"0P" => Some(PartyToDeclareGoods),
            b"01" => Some(LoanApplicant),
            b"001" => Some(Pumper),
            b"1A" => Some(Subgroup),
            b"1B" => Some(Applicant),
            b"1C" => Some(Code1C),
            b"1D" => Some(CoOperative),
            b"1E" => Some(Code1E),
            b"1F" => Some(Alliance),
            b"1G" => Some(OncologyCenter),
            b"1H" => Some(KidneyDialysisUnit),
            b"1I" => Some(Code1I),
            b"1J" => Some(Connection),
            b"1K" => Some(Franchisor),
            b"1L" => Some(Franchisee),
            b"1M" => Some(PreviousGroup),
            b"1N" => Some(Shareholder),
            b"1O" => Some(AcuteCareHospital),
            b"1P" => Some(Provider),
            b"1Q" => Some(MilitaryFacility),
            b"1R" => Some(Code1R),
            b"1S" => Some(OutpatientSurgicenter),
            b"1T" => Some(Code1T),
            b"1U" => Some(LongTermCareFacility),
            b"1V" => Some(ExtendedCareFacility),
            b"1W" => Some(PsychiatricHealthFacility),
            b"1X" => Some(Laboratory),
            b"1Y" => Some(RetailPharmacy),
            b"1Z" => Some(HomeHealthCare),
            b"02" => Some(LoanBroker),
            b"002" => Some(SurfaceManagementEntity),
            b"2A" => Some(Code2A),
            b"2B" => Some(ThirdPartyAdministrator),
            b"2C" => Some(CoParticipant),
            b"2D" => Some(MiscellaneousHealthCareFacility),
            b"2E" => Some(NonHealthCareMiscellaneousFacility),
            b"2F" => Some(State),
            b"2G" => Some(Assigner),
            b"2H" => Some(HospitalDistrictOrAuthority),
            b"2I" => Some(ChurchOperatedFacility),
            b"2J" => Some(Individual),
            b"2K" => Some(Partnership),
            b"2L" => Some(Corporation),
            b"2M" => Some(AirForceFacility),
            b"2N" => Some(ArmyFacility),
            b"2O" => Some(NavyFacility),
            b"2P" => Some(PublicHealthServiceFacility),
            b"2Q" => Some(VeteransAdministrationFacility),
            b"2R" => Some(FederalFacility),
            b"2S" => Some(PublicHealthServiceIndianServiceFacility),
            b"2T" => Some(DepartmentOfJusticeFacility),
            b"2U" => Some(OtherNotForProfitFacility),
            b"2V" => Some(IndividualForProfitFacility),
            b"2W" => Some(PartnershipForProfitFacility),
            b"2X" => Some(CorporationForProfitFacility),
            b"2Y" => Some(GeneralMedicalAndSurgicalFacility),
            b"2Z" => Some(Code2Z),
            b"03" => Some(Dependent),
            b"003" => Some(ApplicationParty),
            b"3A" => Some(HospitalUnitWithinAnInstitutionForTheMentallyRetarded),
            b"3B" => Some(PsychiatricFacility),
            b"3C" => Some(TuberculosisAndOtherRespiratoryDiseasesFacility),
            b"3D" => Some(ObstetricsAndGynecologyFacility),
            b"3E" => Some(Code3E),
            b"3F" => Some(RehabilitationFacility),
            b"3G" => Some(OrthopedicFacility),
            b"3H" => Some(ChronicDiseaseFacility),
            b"3I" => Some(OtherSpecialtyFacility),
            b"3J" => Some(ChildrensGeneralFacility),
            b"3K" => Some(ChildrensHospitalUnitOfAnInstitution),
            b"3L" => Some(ChildrensPsychiatricFacility),
            b"3M" => Some(ChildrensTuberculosisAndOtherRespiratoryDiseasesFacility),
            b"3N" => Some(Code3N),
            b"3O" => Some(ChildrensRehabilitationFacility),
            b"3P" => Some(ChildrensOrthopedicFacility),
            b"3Q" => Some(ChildrensChronicDiseaseFacility),
            b"3R" => Some(ChildrensOtherSpecialtyFacility),
            b"3S" => Some(InstitutionForMentalRetardation),
            b"3T" => Some(AlcoholismAndOtherChemicalDependencyFacility),
            b"3U" => Some(GeneralInpatientCareForAidsArcFacility),
            b"3V" => Some(AidsArcUnit),
            b"3W" => Some(SpecializedOutpatientProgramForAidsArc),
            b"3X" => Some(AlcoholDrugAbuseOrDependencyInpatientUnit),
            b"3Y" => Some(AlcoholDrugAbuseOrDependencyOutpatientServices),
            b"3Z" => Some(ArthritisTreatmentCenter),
            b"04" => Some(AssetAccountHolder),
            b"004" => Some(SiteOperator),
            b"4A" => Some(BirthingRoomLdrpRoom),
            b"4B" => Some(BurnCareUnit),
            b"4C" => Some(CardiacCatherizationLaboratory),
            b"4D" => Some(OpenHeartSurgeryFacility),
            b"4E" => Some(CardiacIntensiveCareUnit),
            b"4F" => Some(AngioplastyFacility),
            b"4G" => Some(ChronicObstructivePulmonaryDiseaseServiceFacility),
            b"4H" => Some(EmergencyDepartment),
            b"4I" => Some(Code4I),
            b"4J" => Some(Code4J),
            b"4K" => Some(FitnessCenter),
            b"4L" => Some(GeneticCounselingScreeningServices),
            b"4M" => Some(AdultDayCareProgramFacility),
            b"4N" => Some(AlzheimersDiagnosticAssessmentServices),
            b"4O" => Some(ComprehensiveGeriatricAssessmentFacility),
            b"4P" => Some(Code4P),
            b"4Q" => Some(GeriatricAcuteCareUnit),
            b"4R" => Some(GeriatricClinics),
            b"4S" => Some(RespiteCareFacility),
            b"4T" => Some(SeniorMembershipProgram),
            b"4U" => Some(PatientEducationUnit),
            b"4V" => Some(CommunityHealthPromotionFacility),
            b"4W" => Some(WorksiteHealthPromotionFacility),
            b"4X" => Some(HemodialysisFacility),
            b"4Y" => Some(HomeHealthServices),
            b"4Z" => Some(Hospice),
            b"05" => Some(Tenant),
            b"005" => Some(ConstructionContractor),
            b"5A" => Some(MedicalSurgicalOrOtherIntensiveCareUnit),
            b"5B" => Some(HisopathologyLaboratory),
            b"5C" => Some(BloodBank),
            b"5D" => Some(NeonatalIntensiveCareUnit),
            b"5E" => Some(ObstetricsUnit),
            b"5F" => Some(OccupationalHealthServices),
            b"5G" => Some(OrganizedOutpatientServices),
            b"5H" => Some(PediatricAcuteInpatientUnit),
            b"5I" => Some(PsychiatricChildAdolescentServices),
            b"5J" => Some(PsychiatricConsultationLiaisonServices),
            b"5K" => Some(PsychiatricEducationServices),
            b"5L" => Some(PsychiatricEmergencyServices),
            b"5M" => Some(PsychiatricGeriatricServices),
            b"5N" => Some(PsychiatricInpatientUnit),
            b"5O" => Some(PsychiatricOutpatientServices),
            b"5P" => Some(PsychiatricPartialHospitalizationProgram),
            b"5Q" => Some(MegavoltageRadiationTherapyUnit),
            b"5R" => Some(RadioactiveImplantsUnit),
            b"5S" => Some(TherapeuticRadioisotopeFacility),
            b"5T" => Some(XRayRadiationTherapyUnit),
            b"5U" => Some(CtScannerUnit),
            b"5V" => Some(DiagnosticRadioisotopeFacility),
            b"5W" => Some(Code5W),
            b"5X" => Some(UltrasoundUnit),
            b"5Y" => Some(RehabilitationInpatientUnit),
            b"5Z" => Some(RehabilitationOutpatientServices),
            b"06" => Some(RecipientOfCivilOrLegalLiabilityPayment),
            b"006" => Some(DrillingContractor),
            b"6A" => Some(ReproductiveHealthServices),
            b"6B" => Some(SkilledNursingOrOtherLongTermCareUnit),
            b"6C" => Some(Code6C),
            b"6D" => Some(OrganizedSocialWorkServiceFacility),
            b"6E" => Some(OutpatientSocialWorkServices),
            b"6F" => Some(EmergencyDepartmentSocialWorkServices),
            b"6G" => Some(SportsMedicineClinicServices),
            b"6H" => Some(HospitalAuxiliaryUnit),
            b"6I" => Some(PatientRepresentativeServices),
            b"6J" => Some(VolunteerServicesDepartment),
            b"6K" => Some(OutpatientSurgeryServices),
            b"6L" => Some(OrganTissueTransplantUnit),
            b"6M" => Some(OrthopedicSurgeryFacility),
            b"6N" => Some(OccupationalTherapyServices),
            b"6O" => Some(PhysicalTherapyServices),
            b"6P" => Some(RecreationalTherapyServices),
            b"6Q" => Some(RespiratoryTherapyServices),
            b"6R" => Some(SpeechTherapyServices),
            b"6S" => Some(WomensHealthCenterServices),
            b"6T" => Some(HealthSciencesLibrary),
            b"6U" => Some(CardiacRehabilitationProgramFacility),
            b"6V" => Some(NonInvasiveCardiacAssessmentServices),
            b"6W" => Some(EmergencyMedicalTechnician),
            b"6X" => Some(DisciplinaryContact),
            b"6Y" => Some(CaseManager),
            b"6Z" => Some(Advisor),
            b"07" => Some(Titleholder),
            b"007" => Some(SpudContractor),
            b"7A" => Some(Premises),
            b"7B" => Some(Bottler),
            b"7C" => Some(PlaceOfOccurrence),
            b"7D" => Some(ContractingOfficerRepresentative),
            b"7E" => Some(PartyAuthorizedToDefinitizeContractAction),
            b"7F" => Some(FilingAddress),
            b"7G" => Some(HazardousMaterialOffice),
            b"7H" => Some(GovernmentFurnishedPropertyFobPoint),
            b"7I" => Some(ProjectName),
            b"7J" => Some(Codefendant),
            b"7K" => Some(CoOccupant),
            b"7L" => Some(PreliminaryInspectionLocation),
            b"7M" => Some(InspectionAndAcceptanceLocation),
            b"7N" => Some(PartyToReceiveProposal),
            b"7O" => Some(FederallyCharteredFacility),
            b"7P" => Some(TransportationOffice),
            b"7Q" => Some(PartyToWhomProtestSubmitted),
            b"7R" => Some(Birthplace),
            b"7S" => Some(PipelineSegment),
            b"7T" => Some(HomeStateName),
            b"7U" => Some(Liquidator),
            b"7V" => Some(PetitioningCreditorsAttorney),
            b"7W" => Some(MergedName),
            b"7X" => Some(PartyRepresented),
            b"7Y" => Some(ProfessionalOrganization),
            b"7Z" => Some(Referee),
            b"08" => Some(NonMortgageLiabilityAccountHolder),
            b"008" => Some(LienHolder),
            b"8A" => Some(VacationHome),
            b"8B" => Some(PrimaryResidence),
            b"8C" => Some(SecondHome),
            b"8D" => Some(PermitHolder),
            b"8E" => Some(MinorityInstitution),
            b"8F" => Some(BailmentWarehouse),
            b"8G" => Some(FirstAppraiser),
            b"8H" => Some(TaxExemptOrganization),
            b"8I" => Some(ServiceOrganization),
            b"8J" => Some(EmergingSmallBusiness),
            b"8K" => Some(SurplusDealer),
            b"8L" => Some(PollingSite),
            b"8M" => Some(SociallyDisadvantagedIndividual),
            b"8N" => Some(EconomicallyDisadvantagedIndividual),
            b"8O" => Some(DisabledIndividual),
            b"8P" => Some(Producer),
            b"8Q" => Some(PublicOrPrivateOrganizationForTheDisabled),
            b"8R" => Some(Code8R),
            b"8S" => Some(Code8S),
            b"8T" => Some(Voter),
            b"8U" => Some(NativeHawaiianOrganization),
            b"8V" => Some(Code8V),
            b"8W" => Some(PaymentAddress),
            b"8X" => Some(OilAndGasCustodian),
            b"8Y" => Some(RegisteredOffice),
            b"09" => Some(NoteCoSigner),
            b"9A" => Some(DebtorIndividual),
            b"9B" => Some(CountryOfExport),
            b"9C" => Some(CountryOfDestination),
            b"9D" => Some(NewServiceProvider),
            b"9E" => Some(SubServicer),
            b"9F" => Some(LossPayee),
            b"9G" => Some(Nickname),
            b"9H" => Some(Assignee),
            b"9I" => Some(RegisteredPrincipal),
            b"9J" => Some(AdditionalDebtor),
            b"9K" => Some(KeyPerson),
            b"9L" => Some(IncorporatedBy),
            b"9N" => Some(PartyToLease),
            b"9O" => Some(PartyToContract),
            b"9P" => Some(Investigator),
            b"9Q" => Some(LastSupplier),
            b"9R" => Some(DownstreamFirstSupplier),
            b"9S" => Some(CoInvestigator),
            b"9T" => Some(TelephoneAnsweringServiceBureau),
            b"9U" => Some(Author),
            b"9V" => Some(FirstSupplier),
            b"9W" => Some(UltimateParentCompany),
            b"9X" => Some(ContractualReceiptMeter),
            b"9Y" => Some(ContractualDeliveryMeter),
            b"9Z" => Some(CoDebtor),
            b"10" => Some(Conduit),
            b"11" => Some(Code11),
            b"12" => Some(RegionalOffice),
            b"13" => Some(ContractedServiceProvider),
            b"14" => Some(WhollyOwnedSubsidiary),
            b"15" => Some(AccountsPayableOffice),
            b"16" => Some(Plant),
            b"17" => Some(ConsultantsOffice),
            b"18" => Some(Production),
            b"19" => Some(NonProductionSupplier),
            b"20" => Some(ForeignSupplier),
            b"21" => Some(SmallBusiness),
            b"22" => Some(Code22),
            b"23" => Some(Code23),
            b"24" => Some(Code24),
            b"25" => Some(Code25),
            b"26" => Some(SociallyDisadvantagedBusiness),
            b"27" => Some(SmallDisadvantagedBusiness),
            b"28" => Some(Subcontractor),
            b"29" => Some(PrototypeSupplier),
            b"30" => Some(ServiceSupplier),
            b"31" => Some(PostalMailingAddress),
            b"32" => Some(PartyToReceiveMaterialRelease),
            b"33" => Some(InquiryAddress),
            b"34" => Some(MaterialChangeNoticeAddress),
            b"35" => Some(Code35),
            b"36" => Some(Employer),
            b"37" => Some(PreviousDebtHolder),
            b"38" => Some(MortgageLiabilityAccountHolder),
            b"39" => Some(AppraisalCompany),
            b"40" => Some(Receiver),
            b"41" => Some(Submitter),
            b"42" => Some(ComponentManufacturer),
            b"43" => Some(ClaimantAuthorizedRepresentative),
            b"44" => Some(DataProcessingServiceBureau),
            b"45" => Some(DropOffLocation),
            b"46" => Some(InvoicingDealer),
            b"47" => Some(Estimator),
            b"48" => Some(InServiceSource),
            b"49" => Some(InitialDealer),
            b"50" => Some(ManufacturersRepresentative),
            b"51" => Some(PartsDistributor),
            b"52" => Some(PartRemanufacturer),
            b"53" => Some(RegisteredOwner),
            b"54" => Some(OrderWriter),
            b"55" => Some(ServiceManager),
            b"56" => Some(ServicingDealer),
            b"57" => Some(ServicingOrganization),
            b"58" => Some(StoreManager),
            b"59" => Some(PartyToApproveSpecification),
            b"60" => Some(Salesperson),
            b"61" => Some(PerformedAt),
            b"62" => Some(ApplicantsEmployer),
            b"63" => Some(ReferencesEmployer),
            b"64" => Some(CosignersEmployer),
            b"65" => Some(ApplicantsReference),
            b"66" => Some(ApplicantsCosigner),
            b"67" => Some(ApplicantsComaker),
            b"68" => Some(OwnersRepresentative),
            b"69" => Some(RepairingOutlet),
            b"70" => Some(PriorIncorrectInsured),
            b"71" => Some(AttendingPhysician),
            b"72" => Some(OperatingPhysician),
            b"73" => Some(OtherPhysician),
            b"74" => Some(CorrectedInsured),
            b"75" => Some(Participant),
            b"76" => Some(SecondaryWarranter),
            b"77" => Some(ServiceLocation),
            b"78" => Some(ServiceRequester),
            b"79" => Some(Warranter),
            b"80" => Some(Hospital),
            b"81" => Some(PartSource),
            b"82" => Some(RenderingProvider),
            b"83" => Some(SubscribersSchool),
            b"84" => Some(SubscribersEmployer),
            b"85" => Some(BillingProvider),
            b"86" => Some(Conductor),
            b"87" => Some(PayToProvider),
            b"88" => Some(Approver),
            b"89" => Some(Investor),
            b"90" => Some(PreviousBusinessPartner),
            b"91" => Some(ActionParty),
            b"92" => Some(SupportParty),
            b"93" => Some(InsuranceInstitute),
            b"94" => Some(NewSupplySource),
            b"95" => Some(ResearchInstitute),
            b"96" => Some(DebtorCompany),
            b"97" => Some(PartyWaivingRequirements),
            b"98" => Some(FreightManagementFacilitator),
            b"99" => Some(Code99),
            b"A1" => Some(Adjuster),
            b"A2" => Some(WomanOwnedBusiness),
            b"A3" => Some(LaborSurplusAreaFirm),
            b"A4" => Some(OtherDisadvantagedBusiness),
            b"A5" => Some(VeteranOwnedBusiness),
            b"A6" => Some(CodeA6),
            b"A7" => Some(ShelteredWorkshop),
            b"A8" => Some(NonprofitInstitution),
            b"A9" => Some(SalesOffice),
            b"AA" => Some(AuthorityForShipment),
            b"AA1" => Some(CodeAA1),
            b"AA2" => Some(FinancialAidOffice),
            b"AA3" => Some(Respondent),
            b"AA4" => Some(AdmissionOffice),
            b"AA5" => Some(MultiCampusAdministrativeUnit),
            b"AA6" => Some(Headmaster),
            b"AA7" => Some(BusinessOfficer),
            b"AA8" => Some(Superintendent),
            b"AA9" => Some(SchoolPrincipal),
            b"AAA" => Some(SubAccount),
            b"AAB" => Some(ManagementNonOfficer),
            b"AAC" => Some(IncorporatedLocation),
            b"AAD" => Some(NameNotToBeConfusedWith),
            b"AAE" => Some(Lot),
            b"AAF" => Some(PreviousOccupant),
            b"AAG" => Some(GroundAmbulanceServices),
            b"AAH" => Some(AirAmbulanceServices),
            b"AAI" => Some(WaterAmbulanceServices),
            b"AAJ" => Some(AdmittingServices),
            b"AAK" => Some(PrimarySurgeon),
            b"AAL" => Some(MedicalNurse),
            b"AAM" => Some(CardiacRehabilitationServices),
            b"AAN" => Some(SkilledNursingServices),
            b"AAO" => Some(ObservationRoomServices),
            b"AAP" => Some(Employee),
            b"AAQ" => Some(AnesthesiologyServices),
            b"AAS" => Some(PriorBaseJurisdiction),
            b"AAT" => Some(IncorporationJurisdiction),
            b"AAU" => Some(MarkerOwner),
            b"AAV" => Some(ReclamationCenter),
            b"AAW" => Some(PartyProvidingFinancing),
            b"AB" => Some(AdditionalPickupAddress),
            b"AB1" => Some(PrivateSchoolSystem),
            b"AB2" => Some(StateOperatedSchoolSystem),
            b"AB3" => Some(VocationalRegionsSchoolSystem),
            b"AB4" => Some(CharteredSchoolDistrict),
            b"AB5" => Some(SchoolingOfIndianChildrenSchoolSystem),
            b"AB6" => Some(UnorganizedTerritoriesSchoolSystem),
            b"AB7" => Some(StateAdministeredDistrict),
            b"AB8" => Some(TownsInUnionsSchoolSystem),
            b"AB9" => Some(AgentTownsSchoolSystem),
            b"ABB" => Some(MasterProperty),
            b"ABC" => Some(ProjectProperty),
            b"ABD" => Some(UnitProperty),
            b"ABE" => Some(AdditionalAddress),
            b"ABF" => Some(SocietyOfPropertyInformationCompilersAndAnalysts),
            b"ABG" => Some(Organization),
            b"ABH" => Some(JointOwnerAnnuitant),
            b"ABI" => Some(JointAnnuitantOwner),
            b"ABJ" => Some(JointOwnerAnnuitantPayor),
            b"ABK" => Some(JointOwnerJointAnnuitant),
            b"ABL" => Some(JointOwnerJointAnnuitantPayor),
            b"ABM" => Some(JointOwnerPayor),
            b"ABN" => Some(Acronym),
            b"ABO" => Some(NewAddress),
            b"ABP" => Some(Chairperson),
            b"ABQ" => Some(DecisionMaker),
            b"ABR" => Some(FormerPresident),
            b"ABS" => Some(Founder),
            b"ABT" => Some(ImportedFromLocation),
            b"ABU" => Some(LiterallyTranslatedName),
            b"ABV" => Some(OriginalLocation),
            b"ABW" => Some(President),
            b"ABX" => Some(RatingOrganization),
            b"AC" => Some(AirCargoCompany),
            b"AC1" => Some(RegionalCenter),
            b"AC2" => Some(CodeAC2),
            b"AC3" => Some(StateEducationAgency),
            b"ACB" => Some(InitialMedicalProvider),
            b"ACC" => Some(ConcurrentEmployer),
            b"ACE" => Some(RoutingPoint),
            b"ACF" => Some(BorderCrossing),
            b"ACG" => Some(BobtailServicePoint),
            b"ACH" => Some(Auditor),
            b"ACI" => Some(InsuredLocation),
            b"ACJ" => Some(ReferralProvider),
            b"ACK" => Some(Affiliate),
            b"ACL" => Some(AlliedHealthProfessional),
            b"ACM" => Some(EmergencyProvider),
            b"ACN" => Some(FederalGovernment),
            b"ACO" => Some(FellowshipInstitution),
            b"ACP" => Some(GovernmentCombinedControl),
            b"ACQ" => Some(GovernmentFederalMilitary),
            b"ACR" => Some(GovernmentFederalOther),
            b"ACS" => Some(GovernmentFederalVeterans),
            b"ACT" => Some(GovernmentLocal),
            b"ACU" => Some(GroupAffiliation),
            b"ACV" => Some(InformationSource),
            b"ACW" => Some(InternshipEntity),
            b"ACX" => Some(MedicalSchool),
            b"ACY" => Some(NationalOrganization),
            b"ACZ" => Some(CodeACZ),
            b"AD" => Some(CodeAD),
            b"ADA" => Some(CodeADA),
            b"ADB" => Some(ForProfitHealthCareProvider),
            b"ADC" => Some(OfficeManager),
            b"ADD" => Some(OnCallProvider),
            b"ADE" => Some(CodeADE),
            b"ADF" => Some(CodeADF),
            b"ADH" => Some(ResidencyInstitution),
            b"ADJ" => Some(SharedService),
            b"ADK" => Some(SupportingPersonnel),
            b"ADL" => Some(TrainingInstitution),
            b"ADM" => Some(PublicSchool),
            b"ADN" => Some(PrivateSchool),
            b"ADO" => Some(PublicPreKEducation),
            b"ADP" => Some(PrivatePreKEducation),
            b"ADQ" => Some(PreKDayCare),
            b"ADR" => Some(CharterSchool),
            b"ADS" => Some(HomeSchool),
            b"ADT" => Some(PublicAlternativeSchool),
            b"ADU" => Some(NeglectedDelinquentInstitution),
            b"ADV" => Some(PostSecondaryInstitution),
            b"ADW" => Some(FoodServiceOperator),
            b"ADX" => Some(FutureAddress),
            b"ADY" => Some(FormerRegisteredAddress),
            b"ADZ" => Some(TopParentCompanyInSameCountry),
            b"AE" => Some(AdditionalDeliveryAddress),
            b"AEA" => Some(SecondLevelParentCompany),
            b"AEB" => Some(AirportAuthority),
            b"AEC" => Some(CouncilOfGovernments),
            b"AED" => Some(Foundation),
            b"AEE" => Some(PortAuthority),
            b"AEF" => Some(PlanningCommission),
            b"AEG" => Some(CarRentalLocation),
            b"AEI" => Some(LodgingFacility),
            b"AEJ" => Some(PartyToReceiveTransportationCredit),
            b"AEK" => Some(CodeAEK),
            b"AEL" => Some(PrimaryInternationalTelecomCarrier),
            b"AF" => Some(AuthorizedAcceptingOfficial),
            b"AG" => Some(AgentAgency),
            b"AH" => Some(Advertiser),
            b"AHM" => Some(AgencyHazardousMaterialInformationSystemLocation),
            b"AI" => Some(Airline),
            b"AJ" => Some(AllegedDebtor),
            b"AK" => Some(PartyToWhomAcknowledgmentShouldBeSent),
            b"AL" => Some(AllotmentCustomer),
            b"ALA" => Some(AlternativeAddressee),
            b"ALO" => Some(ActivityLocation),
            b"AM" => Some(AssistantUSTrustee),
            b"AN" => Some(AuthorizedFrom),
            b"AO" => Some(AccountOf),
            b"AP" => Some(CodeAP),
            b"APR" => Some(ActivityProvider),
            b"AQ" => Some(CodeAQ),
            b"AR" => Some(ArmedServicesLocationDesignation),
            b"AS" => Some(PostsecondaryEducationSender),
            b"AT" => Some(PostsecondaryEducationRecipient),
            b"ATA" => Some(AlternateTaxAuthority),
            b"AU" => Some(PartyAuthorizingDisposition),
            b"AUO" => Some(AuthorizingOfficial),
            b"AV" => Some(AuthorizedTo),
            b"AW" => Some(Accountant),
            b"AX" => Some(Plaintiff),
            b"AY" => Some(Clearinghouse),
            b"AZ" => Some(PreviousName),
            b"B1" => Some(ConstructionFirm),
            b"B2" => Some(OtherUnlistedTypeOfOrganizationalEntity),
            b"B3" => Some(PreviousNameOfFirm),
            b"B4" => Some(ParentCompany),
            b"B5" => Some(AffiliatedCompany),
            b"B6" => Some(RegisteringParentParty),
            b"B7" => Some(RegisteringNonparentParty),
            b"B8" => Some(RegularDealer),
            b"B9" => Some(LargeBusiness),
            b"BA" => Some(Battery),
            b"BAL" => Some(Bailiff),
            b"BB" => Some(BusinessPartner),
            b"BC" => Some(Broadcaster),
            b"BD" => Some(BillToPartyForDiversionCharges),
            b"BE" => Some(Beneficiary),
            b"BF" => Some(BilledFrom),
            b"BG" => Some(BuyingGroup),
            b"BH" => Some(InterimTrustee),
            b"BI" => Some(TrusteesAttorney),
            b"BJ" => Some(CoCounsel),
            b"BK" => Some(Bank),
            b"BKR" => Some(Bookkeeper),
            b"BL" => Some(PartyToReceiveBillOfLading),
            b"BLD" => Some(Building),
            b"BLT" => Some(Structure),
            b"BM" => Some(Brakeman),
            b"BN" => Some(BeneficialOwner),
            b"BO" => Some(BrokerOrSalesOffice),
            b"BOW" => Some(BodyOfWater),
            b"BP" => Some(SpecialCounsel),
            b"BQ" => Some(AttorneyForDefendantPrivate),
            b"BR" => Some(Broker),
            b"BRN" => Some(BrandName),
            b"BS" => Some(BillAndShipTo),
            b"BT" => Some(BillToParty),
            b"BU" => Some(PlaceOfBusiness),
            b"BUS" => Some(Business),
            b"BV" => Some(BillingService),
            b"BW" => Some(Borrower),
            b"BX" => Some(AttorneyForPlaintiff),
            b"BY" => Some(CodeBY),
            b"BZ" => Some(BusinessAssociate),
            b"C0" => Some(AssistantConductor),
            b"C1" => Some(InCareOfPartyNo1),
            b"C2" => Some(InCareOfPartyNo2),
            b"C3" => Some(CircuitLocation),
            b"C4" => Some(ContractAdministrationOffice),
            b"C4A" => Some(SecondaryContractAdministrationOffice),
            b"C5" => Some(PartySubmittingQuote),
            b"C6" => Some(Municipality),
            b"C7" => Some(County),
            b"C8" => Some(City),
            b"C9" => Some(ContractHolder),
            b"CA" => Some(Carrier),
            b"CB" => Some(CustomsBroker),
            b"CC" => Some(Claimant),
            b"CD" => Some(CodeCD),
            b"CE" => Some(CodeCE),
            b"CF" => Some(SubsidiaryDivision),
            b"CG" => Some(CarnetIssuer),
            b"CH" => Some(ChassisProvider),
            b"CHA" => Some(ChangedAddress),
            b"CI" => Some(Consignor),
            b"CJ" => Some(CodeCJ),
            b"CK" => Some(Pharmacist),
            b"CL" => Some(ContainerLocation),
            b"CLT" => Some(BuildingCluster),
            b"CM" => Some(Customs),
            b"CMW" => Some(CompanyMergedWith),
            b"CN" => Some(Consignee),
            b"CNP" => Some(ConfirmingParty),
            b"CNR" => Some(ConfirmationRequester),
            b"CNS" => Some(ConfirmationServiceIdentifierCode),
            b"CO" => Some(OceanTariffConference),
            b"COD" => Some(CoDriver),
            b"COL" => Some(CollateralAssignee),
            b"COM" => Some(Complainant),
            b"COR" => Some(CorrectedName),
            b"CP" => Some(PartyToReceiveCertOfCompliance),
            b"CQ" => Some(CorporateOffice),
            b"CR" => Some(ContainerReturnCompany),
            b"CRW" => Some(CrewMember),
            b"CS" => Some(Consolidator),
            b"CT" => Some(CountryOfOrigin),
            b"CU" => Some(CoatingOrPaintSupplier),
            b"CV" => Some(Converter),
            b"CW" => Some(AccountingStation),
            b"CX" => Some(ClaimAdministrator),
            b"CY" => Some(Country),
            b"CZ" => Some(AdmittingSurgeon),
            b"D1" => Some(Driver),
            b"D2" => Some(CommercialInsurer),
            b"D3" => Some(Defendant),
            b"D4" => Some(Debtor),
            b"D5" => Some(DebtorInPossession),
            b"D6" => Some(ConsolidatedDebtor),
            b"D7" => Some(PetitioningCreditor),
            b"D8" => Some(Dispatcher),
            b"D9" => Some(CreditorsAttorney),
            b"DA" => Some(DeliveryAddress),
            b"DAM" => Some(DamagedBy),
            b"DB" => Some(DistributorBranch),
            b"DC" => Some(DestinationCarrier),
            b"DCC" => Some(ChiefDeputyClerkOfCourt),
            b"DD" => Some(AssistantSurgeon),
            b"DE" => Some(Depositor),
            b"DF" => Some(MaterialDispositionAuthorizationLocation),
            b"DG" => Some(DesignEngineering),
            b"DH" => Some(DoingBusinessAs),
            b"DI" => Some(CodeDI),
            b"DIR" => Some(DistributionRecipient),
            b"DJ" => Some(ConsultingPhysician),
            b"DK" => Some(OrderingPhysician),
            b"DL" => Some(Dealer),
            b"DM" => Some(DestinationMailFacility),
            b"DN" => Some(ReferringProvider),
            b"DO" => Some(DependentName),
            b"DP" => Some(PartyToProvideDiscount),
            b"DQ" => Some(SupervisingPhysician),
            b"DR" => Some(DestinationDrayman),
            b"DS" => Some(Distributor),
            b"DT" => Some(DestinationTerminal),
            b"DU" => Some(ResaleDealer),
            b"DV" => Some(Division),
            b"DW" => Some(DownstreamParty),
            b"DX" => Some(Distiller),
            b"DY" => Some(DefaultForeclosureSpecialist),
            b"DZ" => Some(DeliveryZone),
            b"E0" => Some(AssistantEngineer),
            b"E1" => Some(PersonOrOtherEntityLegallyResponsibleForAChild),
            b"E2" => Some(PersonOrOtherEntityWithWhomAChildResides),
            b"E3" => {
                Some(PersonOrOtherEntityLegallyResponsibleForAndWithWhomAChildResides)
            }
            b"E4" => Some(OtherPersonOrEntityAssociatedWithStudent),
            b"E5" => Some(Examiner),
            b"E6" => Some(Engineering),
            b"E7" => Some(PreviousEmployer),
            b"E8" => Some(InquiringParty),
            b"E9" => Some(ParticipatingLaboratory),
            b"EA" => Some(StudySubmitter),
            b"EAA" => Some(Assistant),
            b"EAB" => Some(CampaignManager),
            b"EAD" => Some(Client),
            b"EAE" => Some(Commissioner),
            b"EAF" => Some(Committee),
            b"EAG" => Some(Contestant),
            b"EAH" => Some(Contributor),
            b"EAI" => Some(DeputyChairperson),
            b"EAJ" => Some(DeputyTreasurer),
            b"EAK" => Some(Donor),
            b"EAL" => Some(Endorser),
            b"EAM" => Some(Guarantor),
            b"EAN" => Some(Headquarters),
            b"EAO" => Some(IndependentContractor),
            b"EAP" => Some(Leader),
            b"EAQ" => Some(PartyPerformingLiaison),
            b"EAR" => Some(LobbyingFirm),
            b"EAS" => Some(Lobbyist),
            b"EAT" => Some(MediaContact),
            b"EAU" => Some(OfficeHolder),
            b"EAV" => Some(PartyAuthorizedToAdministerOaths),
            b"EAW" => Some(PartyToBenefit),
            b"EAX" => Some(PartyHoldingInterest),
            b"EAY" => Some(PartyMakingPledge),
            b"EAZ" => Some(PartyReturningContribution),
            b"EB" => Some(EligiblePartyToTheContract),
            b"EBA" => Some(PartyReturningTransfer),
            b"EBB" => Some(LobbiedParty),
            b"EBC" => Some(PoliticalActionCommittee),
            b"EBD" => Some(PoliticalParty),
            b"EBE" => Some(Proponent),
            b"EBF" => Some(PublicOfficial),
            b"EBG" => Some(ReceivingCommittee),
            b"EBH" => Some(AffiliatedCommittee),
            b"EBI" => Some(Source),
            b"EBJ" => Some(Sponsor),
            b"EBK" => Some(SponsoredCommittee),
            b"EBL" => Some(Designee),
            b"EBM" => Some(TemporaryResidence),
            b"EBN" => Some(Treasurer),
            b"EBO" => Some(ViceChairperson),
            b"EBP" => Some(SlateMailerOrganization),
            b"EBQ" => Some(LodgingLocation),
            b"EBR" => Some(IndependentExpenditureCommittee),
            b"EBS" => Some(MajorDonor),
            b"EC" => Some(Exchanger),
            b"ED" => Some(ExcludedParty),
            b"EE" => Some(LocationOfGoodsForCustomsExaminationBeforeClearance),
            b"EF" => Some(ElectronicFiler),
            b"EG" => Some(Engineer),
            b"EH" => Some(Exhibitor),
            b"EI" => Some(ExecutorOfEstate),
            b"EJ" => Some(PrincipalPerson),
            b"EK" => Some(AnimalSource),
            b"EL" => Some(EstablishedLocation),
            b"EM" => Some(PartyToReceiveElectronicMemoOfInvoice),
            b"EN" => Some(EndUser),
            b"ENR" => Some(Enroller),
            b"EO" => Some(LimitedLiabilityPartnership),
            b"EP" => Some(EligiblePartyToTheRate),
            b"EQ" => Some(OldDebtor),
            b"ER" => Some(NewDebtor),
            b"ET" => Some(PlanAdministrator),
            b"EU" => Some(OldSecuredParty),
            b"EV" => Some(SellingAgent),
            b"EW" => Some(ServicingBroker),
            b"EX" => Some(Exporter),
            b"EXS" => Some(ExSpouse),
            b"EY" => Some(EmployeeName),
            b"EZ" => Some(NewSecuredParty),
            b"F1" => Some(CompanyOwnedOilField),
            b"F2" => Some(CodeF2),
            b"F3" => Some(CodeF3),
            b"F4" => Some(FormerResidence),
            b"F5" => Some(RadioControlStationLocation),
            b"F6" => Some(SmallControlStationLocation),
            b"F7" => Some(SmallBaseStationLocation),
            b"F8" => Some(AntennaSite),
            b"F9" => Some(AreaOfOperation),
            b"FA" => Some(Facility),
            b"FB" => Some(FirstBreakTerminal),
            b"FC" => Some(CodeFC),
            b"FD" => Some(PhysicalAddress),
            b"FE" => Some(MailAddress),
            b"FF" => Some(ForeignLanguageSynonym),
            b"FG" => Some(TradeNameSynonym),
            b"FGT" => Some(ForeignGovernment),
            b"FH" => Some(PartyToReceiveLimitationsOfHeavyElementsReport),
            b"FI" => Some(NameVariationSynonym),
            b"FJ" => Some(FirstContact),
            b"FL" => Some(PrimaryControlPointLocation),
            b"FM" => Some(Fireman),
            b"FN" => Some(FilerName),
            b"FO" => Some(FieldOrBranchOffice),
            b"FP" => Some(NameOnCreditCard),
            b"FQ" => Some(PierName),
            b"FR" => Some(MessageFrom),
            b"FRL" => Some(ForeignRegistrationLocation),
            b"FS" => Some(FinalScheduledDestination),
            b"FSI" => Some(PartyToReceiveSensitiveForeignDisclosure),
            b"FSR" => Some(FinancialStatementRecipient),
            b"FT" => Some(NewAssignee),
            b"FU" => Some(OldAssignee),
            b"FV" => Some(VesselName),
            b"FW" => Some(Forwarder),
            b"FX" => Some(ClosedDoorPharmacy),
            b"FY" => Some(VeterinaryHospital),
            b"FZ" => Some(ChildrensDayCareCenter),
            b"G0" => Some(DependentInsured),
            b"G1" => Some(BankruptcyTrustee),
            b"G2" => Some(Annuitant),
            b"G3" => Some(Clinic),
            b"G5" => Some(ContingentBeneficiary),
            b"G6" => Some(EntityHoldingThe),
            b"G7" => Some(EntityProvidingTheService),
            b"G8" => Some(EntityResponsibleForFollowUp),
            b"G9" => Some(FamilyMember),
            b"GA" => Some(GasPlant),
            b"GB" => Some(OtherInsured),
            b"GBA" => Some(AlternateGovernmentBusinessContact),
            b"GBO" => Some(GateBooth),
            b"GBP" => Some(PrimaryGovernmentBusinessContact),
            b"GC" => Some(PreviousCreditGrantor),
            b"GD" => Some(Guardian),
            b"GE" => Some(GeneralAgency),
            b"GF" => Some(InspectionCompany),
            b"GG" => Some(Intermediary),
            b"GH" => Some(MotorVehicleReportProviderCompany),
            b"GI" => Some(Paramedic),
            b"GIR" => Some(GiftRecipient),
            b"GJ" => Some(ParamedicalCompany),
            b"GK" => Some(PreviousInsured),
            b"GL" => Some(PreviousResidence),
            b"GM" => Some(SpouseInsured),
            b"GN" => Some(Garnishee),
            b"GO" => Some(PrimaryBeneficiary),
            b"GP" => Some(GatewayProvider),
            b"GQ" => Some(ProposedInsured),
            b"GR" => Some(Reinsurer),
            b"GS" => Some(GaragedLocation),
            b"GT" => Some(CreditGrantor),
            b"GU" => Some(GuaranteeAgency),
            b"GV" => Some(GasTransactionEndingPoint),
            b"GW" => Some(Group),
            b"GX" => Some(Retrocessionaire),
            b"GY" => Some(TreatmentFacility),
            b"GZ" => Some(Grandparent),
            b"H1" => Some(Representative),
            b"H2" => Some(SubOffice),
            b"H3" => Some(District),
            b"H5" => Some(PayingAgent),
            b"H6" => Some(SchoolDistrict),
            b"H7" => Some(GroupAffiliate),
            b"H9" => Some(Designer),
            b"HA" => Some(Owner),
            b"HB" => Some(HistoricallyBlackCollegeOrUniversity),
            b"HC" => Some(JointAnnuitant),
            b"HD" => Some(ContingentAnnuitant),
            b"HE" => Some(ContingentOwner),
            b"HF" => Some(CodeHF),
            b"HG" => Some(BrokerOpinionOrAnalysisRequester),
            b"HH" => Some(HomeHealthAgency),
            b"HI" => Some(ListingCompany),
            b"HJ" => Some(AutomatedUnderwritingSystem),
            b"HK" => Some(Subscriber),
            b"HL" => Some(DocumentCustodian),
            b"HM" => Some(CompetitivePropertyListing),
            b"HMI" => Some(CodeHMI),
            b"HN" => Some(CompetingProperty),
            b"HO" => Some(ComparablePropertyListing),
            b"HOM" => Some(HomeOffice),
            b"HON" => Some(HonorarySociety),
            b"HP" => Some(ClosedSale),
            b"HQ" => Some(SourcePartyOf),
            b"HR" => Some(SubjectOfInquiry),
            b"HS" => Some(HighSchool),
            b"HT" => Some(StateCharteredFacility),
            b"HU" => Some(Subsidiary),
            b"HV" => Some(TaxAddress),
            b"HW" => Some(DesignatedHazardousWasteFacility),
            b"HX" => Some(TransporterOfHazardousWaste),
            b"HY" => Some(Charity),
            b"HZ" => Some(HazardousWasteGenerator),
            b"I1" => Some(InterestedParty),
            b"I3" => Some(CodeI3),
            b"I4" => Some(IntellectualPropertyOwner),
            b"I9" => Some(Interviewer),
            b"IA" => Some(InstalledAt),
            b"IAA" => Some(BusinessEntity),
            b"IAC" => Some(PrincipalExecutiveOffice),
            b"IAD" => Some(ForeignOffice),
            b"IAE" => Some(Member),
            b"IAF" => Some(ExecutiveCommitteeMember),
            b"IAG" => Some(Director),
            b"IAH" => Some(Clerk),
            b"IAI" => Some(PartyWithKnowledgeOfAffairsOfTheCompany),
            b"IAK" => Some(PartyToReceiveStatementOfFeesDue),
            b"IAL" => Some(CompanyInWhichInterestHeld),
            b"IAM" => Some(CompanyWhichHoldsInterest),
            b"IAN" => Some(Notary),
            b"IAO" => Some(Manager),
            b"IAP" => Some(AlienAffiliate),
            b"IAQ" => Some(IncorporationStatePrincipalOffice),
            b"IAR" => Some(IncorporationStatePlaceOfBusiness),
            b"IAS" => Some(OutOfStatePrincipalOffice),
            b"IAT" => Some(PartyExecutingAndVerifying),
            b"IAU" => Some(Felon),
            b"IAV" => Some(OtherRelatedParty),
            b"IAW" => Some(RecordKeepingAddress),
            b"IAY" => Some(InitialSubscriber),
            b"IAZ" => Some(OriginalJurisdiction),
            b"IB" => Some(IndustryBureau),
            b"IC" => Some(IntermediateConsignee),
            b"ICP" => Some(InventoryControlPoint),
            b"ID" => Some(IssuerOfDebitOrCreditMemo),
            b"IE" => Some(OtherIndividualDisabilityCarrier),
            b"IF" => Some(InternationalFreightForwarder),
            b"IG" => Some(InsolventInsurer),
            b"II" => Some(IssuerOfInvoice),
            b"IJ" => Some(InjectionPoint),
            b"IK" => Some(IntermediateCarrier),
            b"IL" => Some(InsuredOrSubscriber),
            b"IM" => Some(Importer),
            b"IMM" => Some(IntegratedMaterialManager),
            b"IN" => Some(Insurer),
            b"INT" => Some(Interviewee),
            b"INV" => Some(InvestmentAdvisor),
            b"IO" => Some(Inspector),
            b"IP" => Some(IndependentAdjuster),
            b"IQ" => Some(InPatientPharmacy),
            b"IR" => Some(SelfInsured),
            b"IS" => Some(PartyToReceiveCertifiedInspectionReport),
            b"IT" => Some(InstallationOnSite),
            b"IU" => Some(Issuer),
            b"IV" => Some(Renter),
            b"J1" => Some(AssociateGeneralAgent),
            b"J2" => Some(AuthorizedEntity),
            b"J3" => Some(BrokersAssistant),
            b"J4" => Some(Custodian),
            b"J5" => Some(IrrevocableBeneficiary),
            b"J6" => Some(PowerOfAttorney),
            b"J7" => Some(TrustOfficer),
            b"J8" => Some(BrokerDealer),
            b"J9" => Some(CommunityAgent),
            b"JA" => Some(DairyDepartment),
            b"JB" => Some(DelicatessenDepartment),
            b"JC" => Some(DryGroceryDepartment),
            b"JD" => Some(Judge),
            b"JE" => Some(FrozenDepartment),
            b"JF" => Some(GeneralMerchandiseDepartment),
            b"JG" => Some(CodeJG),
            b"JH" => Some(AlcoholBeverageDepartment),
            b"JI" => Some(MeatDepartment),
            b"JJ" => Some(ProduceDepartment),
            b"JK" => Some(BakeryDepartment),
            b"JL" => Some(VideoDepartment),
            b"JM" => Some(CandyAndConfectionsDepartment),
            b"JN" => Some(CigarettesAndTobaccoDepartment),
            b"JO" => Some(InStoreBakeryDepartment),
            b"JP" => Some(FloralDepartment),
            b"JQ" => Some(PharmacyDepartment),
            b"JR" => Some(Bidder),
            b"JS" => Some(JointDebtorAttorney),
            b"JT" => Some(JointDebtor),
            b"JU" => Some(Jurisdiction),
            b"JV" => Some(JointOwner),
            b"JW" => Some(JointVenture),
            b"JX" => Some(ClosingAgent),
            b"JY" => Some(FinancialPlanner),
            b"JZ" => Some(ManagingGeneralAgent),
            b"K1" => Some(ContractorCognizantSecurityOffice),
            b"K2" => Some(SubcontractorCognizantSecurityOffice),
            b"K3" => Some(PlaceOfPerformanceCognizantSecurityOffice),
            b"K4" => Some(PartyAuthorizingReleaseOfSecurity),
            b"K5" => Some(PartyToReceiveContractSecurityClassificationSpecification),
            b"K6" => Some(PolicyWritingAgent),
            b"K7" => Some(RadioStation),
            b"K8" => Some(FilingLocation),
            b"K9" => Some(PreviousDistributor),
            b"KA" => Some(ItemManager),
            b"KB" => Some(CustomerForWhomSameOrSimilarWorkWasPerformed),
            b"KC" => Some(PartyThatReceivedDisclosureStatement),
            b"KD" => Some(Proposer),
            b"KE" => Some(ContactOffice),
            b"KF" => Some(AuditOffice),
            b"KG" => Some(ProjectManager),
            b"KH" => Some(OrganizationHavingSourceControl),
            b"KI" => Some(UnitedStatesOverseasSecurityAdministrationOffice),
            b"KJ" => Some(QualifyingOfficer),
            b"KK" => Some(RegisteringParty),
            b"KL" => Some(ClerkOfCourt),
            b"KM" => Some(Coordinator),
            b"KN" => Some(FormerAddress),
            b"KO" => Some(PlantClearanceOfficer),
            b"KP" => Some(NameUnderWhichFiled),
            b"KQ" => Some(Licensee),
            b"KR" => Some(PreKindergartenToGrade12Recipient),
            b"KS" => Some(PreKindergartenToGrade12Sender),
            b"KT" => Some(Court),
            b"KU" => Some(ReceiverSite),
            b"KV" => Some(DisbursingOfficer),
            b"KW" => Some(BidOpeningLocation),
            b"KX" => Some(FreeOnBoardPoint),
            b"KY" => Some(TechnicalOffice),
            b"KZ" => Some(AcceptanceLocation),
            b"L1" => Some(InspectionLocation),
            b"L2" => Some(LocationOfPrincipalAssets),
            b"L3" => Some(LoanCorrespondent),
            b"L5" => Some(Contact),
            b"L8" => Some(HeadOffice),
            b"L9" => Some(InformationProvider),
            b"LA" => Some(Attorney),
            b"LB" => Some(LastBreakTerminal),
            b"LC" => Some(LocationOfSpotForStorage),
            b"LCN" => Some(GasNominationLocation),
            b"LD" => Some(LiabilityHolder),
            b"LE" => Some(Lessor),
            b"LF" => Some(LimitedPartner),
            b"LG" => Some(LocationOfGoods),
            b"LGS" => Some(LocalGovernmentSponsor),
            b"LH" => Some(Pipeline),
            b"LI" => Some(IndependentLab),
            b"LJ" => Some(LimitedLiabilityCompany),
            b"LK" => Some(JuvenileOwner),
            b"LL" => Some(CodeLL),
            b"LM" => Some(LendingInstitution),
            b"LN" => Some(Lender),
            b"LO" => Some(LoanOriginator),
            b"LP" => Some(LoadingParty),
            b"LQ" => Some(LawFirm),
            b"LR" => Some(LegalRepresentative),
            b"LS" => Some(Lessee),
            b"LT" => Some(LongTermDisabilityCarrier),
            b"LU" => Some(MasterAgent),
            b"LV" => Some(LoanServicer),
            b"LW" => Some(Customer),
            b"LY" => Some(Labeler),
            b"LYM" => Some(AmendedName),
            b"LYN" => Some(Stockholder),
            b"LYO" => Some(ManagingAgent),
            b"LYP" => Some(Organizer),
            b"LZ" => Some(LocalChain),
            b"M1" => Some(SourceMeterLocation),
            b"M2" => Some(ReceiptLocation),
            b"M3" => Some(UpstreamMeterLocation),
            b"M4" => Some(DownstreamMeterLocation),
            b"M5" => Some(MigrantHealthClinic),
            b"M6" => Some(Landlord),
            b"M7" => Some(ForeclosingLender),
            b"M8" => Some(EducationalInstitution),
            b"M9" => Some(Manufacturing),
            b"MA" => Some(PartyForWhomItemIsUltimatelyIntended),
            b"MB" => Some(CompanyInterviewerWorksFor),
            b"MC" => Some(MotorCarrier),
            b"MD" => Some(VeteransAdministrationLoanGuarantyAuthority),
            b"ME" => Some(VeteransAdministrationLoanAuthorizedSupplier),
            b"MF" => Some(ManufacturerOfGoods),
            b"MG" => Some(GovernmentLoanAgencySponsorOrAgent),
            b"MH" => Some(MortgageInsurer),
            b"MI" => Some(PlanningScheduleMaterialReleaseIssuer),
            b"MJ" => Some(FinancialInstitution),
            b"MK" => Some(LoanHolderForRealEstateAsset),
            b"ML" => Some(ConsumerCreditAccountCompany),
            b"MM" => Some(MortgageCompany),
            b"MN" => Some(AuthorizedMarketer),
            b"MO" => Some(ReleaseDrayman),
            b"MP" => Some(ManufacturingPlant),
            b"MQ" => Some(DeliveryLocation),
            b"MR" => Some(MedicalInsuranceCarrier),
            b"MS" => Some(CodeMS),
            b"MSC" => Some(MammographyScreeningCenter),
            b"MT" => Some(Material),
            b"MTR" => Some(MeterLocation),
            b"MU" => Some(MeetingLocation),
            b"MV" => Some(Mainline),
            b"MW" => Some(MarineSurveyor),
            b"MX" => Some(JuvenileWitness),
            b"MY" => Some(MasterGeneralAgent),
            b"MZ" => Some(Minister),
            b"N1" => Some(NotifyPartyNo1),
            b"N2" => Some(NotifyPartyNo2),
            b"N3" => Some(IneligibleParty),
            b"N4" => Some(PriceAdministration),
            b"N5" => Some(PartyWhoSignedTheDeliveryReceipt),
            b"N6" => Some(NonemploymentIncomeSource),
            b"N7" => Some(PreviousNeighbor),
            b"N8" => Some(Relative),
            b"N9" => Some(Neighborhood),
            b"NB" => Some(Neighbor),
            b"NC" => Some(CrossTownSwitch),
            b"NCT" => Some(NameChangedTo),
            b"ND" => Some(NextDestination),
            b"NE" => Some(Newspaper),
            b"NF" => Some(OwnerAnnuitant),
            b"NG" => Some(Administrator),
            b"NH" => Some(Association),
            b"NI" => Some(NonInsured),
            b"NJ" => Some(TrustOrEstate),
            b"NK" => Some(NationalChain),
            b"NL" => Some(NonRailroadEntity),
            b"NM" => Some(PhysicianSpecialists),
            b"NN" => Some(NetworkName),
            b"NP" => Some(NotifyPartyForShippersOrder),
            b"NPC" => Some(NotaryPublic),
            b"NQ" => Some(PipelineSegmentBoundary),
            b"NR" => Some(GasTransactionStartingPoint),
            b"NS" => Some(NonTemporaryStorageFacility),
            b"NT" => Some(MagistrateJudge),
            b"NU" => Some(FormerlyKnownAs),
            b"NV" => Some(FormerlyDoingBusinessAs),
            b"NW" => Some(MaidenName),
            b"NX" => Some(PrimaryOwner),
            b"NY" => Some(BirthName),
            b"NZ" => Some(PrimaryPhysician),
            b"O1" => Some(OriginatingBank),
            b"O2" => Some(OriginatingCompany),
            b"O3" => Some(ReceivingCompany),
            b"O4" => Some(Factor),
            b"O5" => Some(MerchantBanker),
            b"O6" => Some(NonRegisteredBusinessName),
            b"O7" => Some(RegisteredBusinessName),
            b"O8" => Some(Registrar),
            b"OA" => Some(ElectronicReturnOriginator),
            b"OB" => Some(OrderedBy),
            b"OC" => Some(OriginCarrier),
            b"OD" => Some(DoctorOfOptometry),
            b"OE" => Some(BookingOffice),
            b"OF" => Some(OffsetOperator),
            b"OG" => Some(CoOwner),
            b"OH" => Some(OtherDepartments),
            b"OI" => Some(OutsideInspectionAgency),
            b"OL" => Some(Officer),
            b"OM" => Some(OriginMailFacility),
            b"ON" => Some(ProductPositionHolder),
            b"OO" => Some(CodeOO),
            b"OP" => Some(OperatorOfPropertyOrUnit),
            b"OR" => Some(OriginDrayman),
            b"ORI" => Some(OriginalName),
            b"OS" => Some(CodeOS),
            b"OSH" => Some(OffSiteHandler),
            b"OT" => Some(OriginTerminal),
            b"OU" => Some(OutsideProcessor),
            b"OUC" => Some(OtherUnlistedTypeOfCorporation),
            b"OV" => Some(OwnerOfVessel),
            b"OW" => Some(OwnerOfPropertyOrUnit),
            b"OX" => Some(OxygenTherapyFacility),
            b"OY" => Some(OwnerOfVehicle),
            b"OZ" => Some(OutsideTestingAgency),
            b"P0" => Some(PatientFacility),
            b"P1" => Some(Preparer),
            b"P2" => Some(PrimaryInsuredOrSubscriber),
            b"P3" => Some(PrimaryCareProvider),
            b"P4" => Some(PriorInsuranceCarrier),
            b"P5" => Some(PlanSponsor),
            b"P6" => Some(CodeP6),
            b"P7" => Some(CodeP7),
            b"P8" => Some(PersonnelOffice),
            b"P9" => Some(CodeP9),
            b"PA" => Some(PartyToReceiveInspectionReport),
            b"PB" => Some(PayingBank),
            b"PC" => Some(CodePC),
            b"PD" => Some(PurchasersDepartmentBuyer),
            b"PE" => Some(Payee),
            b"PF" => Some(PartyToReceiveFreightBill),
            b"PG" => Some(PrimeContractor),
            b"PH" => Some(Printer),
            b"PI" => Some(Publisher),
            b"PIC" => Some(PrimaryInventoryControlActivity),
            b"PJ" => Some(PartyToReceiveCorrespondence),
            b"PK" => Some(PartyToReceiveCopy),
            b"PL" => Some(PartyToReceivePurchaseOrder),
            b"PLC" => Some(LawEnforcementAgency),
            b"PLR" => Some(PayerOfLastResort),
            b"PM" => Some(PartyToReceivePaperMemoOfInvoice),
            b"PMC" => Some(PriorMortgageCompany),
            b"PMF" => Some(PartyManufacturedFor),
            b"PMG" => Some(ProgramManager),
            b"PN" => Some(PartyToReceiveShippingNotice),
            b"PO" => Some(PartyToReceiveInvoiceForGoodsOrServices),
            b"PP" => Some(Property),
            b"PPC" => Some(PastPerformanceContact),
            b"PPS" => Some(PersonForWhoseBenefitPropertyWasSeized),
            b"PQ" => Some(PartyToReceiveInvoiceForLeasePayments),
            b"PR" => Some(Payer),
            b"PRE" => Some(PreviousOwner),
            b"PRO" => Some(ProspectService),
            b"PRP" => Some(PrimaryPayer),
            b"PS" => Some(PreviousStation),
            b"PT" => Some(PartyToReceiveTestReport),
            b"PU" => Some(PartyAtPickupLocation),
            b"PUR" => Some(PurchasedCompany),
            b"PV" => Some(PartyPerformingCertification),
            b"PW" => Some(PickupAddress),
            b"PX" => Some(PartyPerformingCount),
            b"PY" => Some(PartyToFilePersonalPropertyTax),
            b"PZ" => Some(PartyToReceiveEquipment),
            b"Q1" => Some(ConductorPilot),
            b"Q2" => Some(EngineerPilot),
            b"Q3" => Some(RetailAccount),
            b"Q4" => Some(CooperativeBuyingGroup),
            b"Q5" => Some(AdvertisingGroup),
            b"Q6" => Some(Interpreter),
            b"Q7" => Some(Partner),
            b"Q8" => Some(BasePeriodEmployer),
            b"Q9" => Some(LastEmployer),
            b"QA" => Some(Pharmacy),
            b"QB" => Some(PurchaseServiceProvider),
            b"QC" => Some(Patient),
            b"QD" => Some(ResponsibleParty),
            b"QE" => Some(Policyholder),
            b"QF" => Some(Passenger),
            b"QG" => Some(Pedestrian),
            b"QH" => Some(Physician),
            b"QI" => Some(PartyInPossession),
            b"QJ" => Some(CodeQJ),
            b"QK" => Some(ManagedCare),
            b"QL" => Some(Chiropractor),
            b"QM" => Some(DialysisCenters),
            b"QN" => Some(Dentist),
            b"QO" => Some(DoctorOfOsteopathy),
            b"QP" => Some(PrincipalBorrower),
            b"QQ" => Some(QualityControl),
            b"QR" => Some(BuyersQualityReviewBoard),
            b"QS" => Some(Podiatrist),
            b"QT" => Some(Psychiatrist),
            b"QU" => Some(Veterinarian),
            b"QV" => Some(GroupPractice),
            b"QW" => Some(Government),
            b"QX" => Some(HomeHealthCorporation),
            b"QY" => Some(MedicalDoctor),
            b"QZ" => Some(CoBorrower),
            b"R0" => Some(RoyaltyOwner),
            b"R1" => Some(PartyToReceiveScaleTicket),
            b"R2" => Some(ReportingOfficer),
            b"R3" => Some(NextScheduledDestination),
            b"R4" => Some(CodeR4),
            b"R5" => Some(CodeR5),
            b"R6" => Some(Requester),
            b"R7" => Some(ConsumerReferralContact),
            b"R8" => Some(CreditReportingAgency),
            b"R9" => Some(RequestedLender),
            b"RA" => Some(AlternateReturnAddress),
            b"RB" => Some(ReceivingBank),
            b"RC" => Some(ReceivingLocation),
            b"RCR" => Some(RecoveryRoom),
            b"RD" => Some(DestinationIntermodalRamp),
            b"REC" => Some(ReceiverManager),
            b"RF" => Some(Refinery),
            b"RG" => Some(CodeRG),
            b"RGA" => Some(ResponsibleGovernmentAgency),
            b"RH" => Some(CodeRH),
            b"RI" => Some(RemitTo),
            b"RJ" => Some(ResidenceOrDomicile),
            b"RK" => Some(RefineryOperator),
            b"RL" => Some(ReportingLocation),
            b"RM" => Some(PartyThatRemitsPayment),
            b"RN" => Some(RepairOrRefurbishLocation),
            b"RO" => Some(OriginalIntermodalRamp),
            b"RP" => Some(ReceivingPointForCustomerSamples),
            b"RQ" => Some(ResaleCustomer),
            b"RR" => Some(Railroad),
            b"RR2" => Some(ClassIiRailroad),
            b"RR3" => Some(ClassIiiRailroad),
            b"RS" => Some(ReceivingFacilityScheduler),
            b"RT" => Some(ReturnedTo),
            b"RU" => Some(ReceivingSubLocation),
            b"RV" => Some(Reservoir),
            b"RW" => Some(RuralHealthClinic),
            b"RX" => Some(ResponsibleExhibitor),
            b"RY" => Some(SpecifiedRepository),
            b"RZ" => Some(ReceiptZone),
            b"S0" => Some(SoleProprietor),
            b"S1" => Some(Parent),
            b"S2" => Some(Student),
            b"S3" => Some(CustodialParent),
            b"S4" => Some(SkilledNursingFacility),
            b"S5" => Some(SecuredParty),
            b"S6" => Some(AgencyGrantingSecurityClearance),
            b"S7" => Some(SecuredPartyCompany),
            b"S8" => Some(SecuredPartyIndividual),
            b"S9" => Some(Sibling),
            b"SA" => Some(SalvageCarrier),
            b"SB" => Some(StorageArea),
            b"SC" => Some(StoreClass),
            b"SD" => Some(SoldToAndShipTo),
            b"SE" => Some(SellingParty),
            b"SEP" => Some(SecondaryPayer),
            b"SF" => Some(ShipFrom),
            b"SG" => Some(StoreGroup),
            b"SH" => Some(Shipper),
            b"SI" => Some(ShippingScheduleIssuer),
            b"SIC" => Some(SecondaryInventoryControlActivity),
            b"SIP" => Some(ShipInPlaceLocation),
            b"SJ" => Some(ServiceProvider),
            b"SK" => Some(CodeSK),
            b"SL" => Some(OriginSublocation),
            b"SM" => Some(PartyToReceiveShippingManifest),
            b"SN" => Some(Store),
            b"SNP" => Some(CodeSNP),
            b"SO" => Some(SoldToIfDifferentFromBillTo),
            b"SP" => Some(PartyFillingShippersOrder),
            b"SQ" => Some(ServiceBureau),
            b"SR" => Some(SamplesToBeReturnedTo),
            b"SS" => Some(SteamshipCompany),
            b"ST" => Some(ShipTo),
            b"STC" => Some(SwitchingAndTerminalCarrier),
            b"SU" => Some(SupplierManufacturer),
            b"SUS" => Some(SupplySource),
            b"SV" => Some(ServicePerformanceSite),
            b"SW" => Some(SealingCompany),
            b"SX" => Some(SchoolBasedServiceProvider),
            b"SY" => Some(SecondaryTaxpayer),
            b"SZ" => Some(Supervisor),
            b"T1" => Some(OperatorOfTheTransferPoint),
            b"T2" => Some(OperatorOfTheSourceTransferPoint),
            b"T3" => Some(TerminalLocation),
            b"T4" => Some(TransferPoint),
            b"T6" => Some(TerminalOperator),
            b"T8" => Some(PreviousTitleCompany),
            b"T9" => Some(PriorTitleEvidenceHolder),
            b"TA" => Some(TitleInsuranceServicesProvider),
            b"TB" => Some(Tooling),
            b"TC" => Some(ToolSource),
            b"TD" => Some(ToolingDesign),
            b"TE" => Some(Theatre),
            b"TEC" => Some(TaxExemptCorporation),
            b"TF" => Some(TankFarm),
            b"TG" => Some(ToolingFabrication),
            b"TH" => Some(TheaterCircuit),
            b"TI" => Some(TariffIssuer),
            b"TJ" => Some(Cosigner),
            b"TK" => Some(TestSponsor),
            b"TL" => Some(TestingLaboratory),
            b"TM" => Some(Transmitter),
            b"TN" => Some(Tradename),
            b"TO" => Some(MessageTo),
            b"TOW" => Some(TowingAgency),
            b"TP" => Some(PrimaryTaxpayer),
            b"TPM" => Some(ThirdPartyMarketer),
            b"TQ" => Some(CodeTQ),
            b"TR" => Some(Terminal),
            b"TS" => Some(PartyToReceiveCertifiedTestResults),
            b"TSD" => Some(CodeTSD),
            b"TSE" => Some(ConsigneeCourierTransferStation),
            b"TSR" => Some(ConsignorCourierTransferStation),
            b"TT" => Some(TransferTo),
            b"TTP" => Some(TertiaryPayer),
            b"TU" => Some(CodeTU),
            b"TV" => Some(CodeTV),
            b"TW" => Some(TransitAuthority),
            b"TX" => Some(TaxAuthority),
            b"TY" => Some(Trustee),
            b"TZ" => Some(SignificantOther),
            b"U1" => Some(GasTransactionPoint1),
            b"U2" => Some(GasTransactionPoint2),
            b"U3" => Some(ServicingAgent),
            b"U4" => Some(Team),
            b"U5" => Some(Underwriter),
            b"U6" => Some(TitleUnderwriter),
            b"U7" => Some(Psychologist),
            b"U8" => Some(Reference),
            b"U9" => Some(NonRegisteredInvestmentAdvisor),
            b"UA" => Some(PlaceOfBottling),
            b"UB" => Some(PlaceOfDistilling),
            b"UC" => Some(UltimateConsignee),
            b"UD" => Some(Region),
            b"UE" => Some(TestingService),
            b"UF" => Some(HealthMiscellaneous),
            b"UG" => Some(NursingHomeChain),
            b"UH" => Some(NursingHome),
            b"UI" => Some(RegisteredInvestmentAdvisor),
            b"UJ" => Some(SalesAssistant),
            b"UK" => Some(System),
            b"UL" => Some(SpecialAccount),
            b"UM" => Some(CodeUM),
            b"UN" => Some(Union),
            b"UO" => Some(CodeUO),
            b"UP" => Some(UnloadingParty),
            b"UQ" => Some(SubsequentOwner),
            b"UR" => Some(Surgeon),
            b"US" => Some(UpstreamParty),
            b"UT" => Some(USTrustee),
            b"UU" => Some(AnnuitantPayor),
            b"UW" => Some(UnassignedAgent),
            b"UX" => Some(BaseJurisdiction),
            b"UY" => Some(Vehicle),
            b"UZ" => Some(Signer),
            b"V1" => Some(Surety),
            b"V2" => Some(Grantor),
            b"V3" => Some(WellPadConstructionContractor),
            b"V4" => Some(OilAndGasRegulatoryAgency),
            b"V5" => Some(SurfaceDischargeAgency),
            b"V6" => Some(WellCasingDepthAuthority),
            b"V8" => Some(MarketTimer),
            b"V9" => Some(OwnerAnnuitantPayor),
            b"VA" => Some(SecondContact),
            b"VB" => Some(Candidate),
            b"VC" => Some(VehicleCustodian),
            b"VD" => Some(MultipleListingService),
            b"VE" => Some(BoardOfRealtors),
            b"VER" => Some(PartyPerformingVerification),
            b"VF" => Some(SellingOffice),
            b"VG" => Some(ListingAgent),
            b"VH" => Some(ShowingAgent),
            b"VI" => Some(ContactPerson),
            b"VIC" => Some(Victim),
            b"VJ" => Some(OwnerJointAnnuitantPayor),
            b"VK" => Some(PropertyOrBuildingManager),
            b"VL" => Some(BuilderName),
            b"VM" => Some(Occupant),
            b"VN" => Some(Vendor),
            b"VO" => Some(ElementarySchool),
            b"VP" => Some(PartyWithPowerToVoteSecurities),
            b"VQ" => Some(MiddleSchool),
            b"VR" => Some(JuniorHighSchool),
            b"VS" => Some(VehicleSalvageAssignment),
            b"VT" => Some(ListingOffice),
            b"VU" => Some(SecondContactOrganization),
            b"VV" => Some(OwnerPayor),
            b"VW" => Some(Winner),
            b"VX" => Some(ProductionManager),
            b"VY" => Some(OrganizationCompletingConfigurationChange),
            b"W1" => Some(WorkTeam),
            b"W2" => Some(SupplierWorkTeam),
            b"W3" => Some(ThirdPartyInvestmentAdvisor),
            b"W4" => Some(Trust),
            b"W8" => Some(InterlineServiceCommitmentCustomer),
            b"W9" => Some(SamplingLocation),
            b"WA" => Some(WritingAgent),
            b"WB" => Some(AppraiserName),
            b"WC" => Some(ComparableProperty),
            b"WD" => Some(StorageFacilityAtDestination),
            b"WE" => Some(SubjectProperty),
            b"WF" => Some(TankFarmOwner),
            b"WG" => Some(WageEarner),
            b"WH" => Some(Warehouse),
            b"WI" => Some(Witness),
            b"WJ" => Some(SupervisoryAppraiserName),
            b"WL" => Some(Wholesaler),
            b"WN" => Some(CompanyAssignedWell),
            b"WO" => Some(StorageFacilityAtOrigin),
            b"WP" => Some(WitnessForPlaintiff),
            b"WR" => Some(WithdrawalPoint),
            b"WS" => Some(WaterSystem),
            b"WT" => Some(WitnessForDefendant),
            b"WU" => Some(PrimarySupportOrganization),
            b"WV" => Some(PreliminaryMaintenancePeriodDesignatingOrganization),
            b"WW" => Some(PreliminaryMaintenanceOrganization),
            b"WX" => Some(PreliminaryReferredToOrganization),
            b"WY" => Some(FinalMaintenancePeriodDesignatingOrganization),
            b"WZ" => Some(FinalMaintenanceOrganization),
            b"X1" => Some(MailTo),
            b"X2" => Some(PartyToPerformPackaging),
            b"X3" => Some(UtilizationManagementOrganization),
            b"X4" => Some(Spouse),
            b"X5" => Some(DurableMedicalEquipmentSupplier),
            b"X6" => Some(InternationalOrganization),
            b"X7" => Some(Inventor),
            b"X8" => Some(HispanicServiceInstitute),
            b"XA" => Some(Creditor),
            b"XC" => Some(DebtorsAttorney),
            b"XD" => Some(Alias),
            b"XE" => Some(ClaimRecipient),
            b"XF" => Some(Auctioneer),
            b"XG" => Some(EventLocation),
            b"XH" => Some(FinalReferredToOrganization),
            b"XI" => Some(OriginalClaimant),
            b"XJ" => Some(ActualReferredByOrganization),
            b"XK" => Some(ActualReferredToOrganization),
            b"XL" => Some(BorrowersEmployer),
            b"XM" => Some(MaintenanceOrganizationUsedForEstimate),
            b"XN" => Some(PlanningMaintenanceOrganization),
            b"XO" => Some(PreliminaryCustomerOrganization),
            b"XP" => Some(PartyToReceiveSolicitation),
            b"XQ" => Some(CanadianCustomsBroker),
            b"XR" => Some(MexicanCustomsBroker),
            b"XS" => Some(SCorporation),
            b"XT" => Some(FinalCustomerOrganization),
            b"XU" => Some(UnitedStatesCustomsBroker),
            b"XV" => Some(CrossClaimant),
            b"XW" => Some(CounterClaimant),
            b"XX" => Some(BusinessArea),
            b"XY" => Some(TribalGovernment),
            b"XZ" => Some(AmericanIndianOwnedBusiness),
            b"Y2" => Some(ManagedCareOrganization),
            b"YA" => Some(Affiant),
            b"YB" => Some(Arbitrator),
            b"YC" => Some(BailPayor),
            b"YD" => Some(DistrictJustice),
            b"YE" => Some(ThirdParty),
            b"YF" => Some(WitnessForProsecution),
            b"YG" => Some(ExpertWitness),
            b"YH" => Some(CrimeVictim),
            b"YI" => Some(JuvenileVictim),
            b"YJ" => Some(JuvenileDefendant),
            b"YK" => Some(Bondsman),
            b"YL" => Some(CourtAppointedAttorney),
            b"YM" => Some(ComplainantsAttorney),
            b"YN" => Some(DistrictAttorney),
            b"YO" => Some(CodeYO),
            b"YP" => Some(ProBonoAttorney),
            b"YQ" => Some(ProSeCounsel),
            b"YR" => Some(PartyToAppearBefore),
            b"YS" => Some(Appellant),
            b"YT" => Some(Appellee),
            b"YU" => Some(ArrestingOfficer),
            b"YV" => Some(HostileWitness),
            b"YW" => Some(DischargePoint),
            b"YX" => Some(FloodCertifier),
            b"YY" => Some(FloodDeterminationProvider),
            b"YZ" => Some(ElectronicRegistrationUtility),
            b"Z1" => Some(PartyToReceiveStatus),
            b"Z2" => Some(UnserviceableMaterialConsignee),
            b"Z3" => Some(PotentialSourceOfSupply),
            b"Z4" => Some(OwningInventoryControlPoint),
            b"Z5" => Some(ManagementControlActivity),
            b"Z6" => Some(TransferringParty),
            b"Z7" => Some(MarkForParty),
            b"Z8" => Some(LastKnownSourceOfSupply),
            b"Z9" => Some(Banker),
            b"ZA" => Some(CorrectedAddress),
            b"ZB" => Some(PartyToReceiveCredit),
            b"ZC" => Some(RentPayor),
            b"ZD" => Some(PartyToReceiveReports),
            b"ZE" => Some(EndItemManufacturer),
            b"ZF" => Some(BreakBulkPoint),
            b"ZG" => Some(PresentAddress),
            b"ZH" => Some(Child),
            b"ZJ" => Some(Branch),
            b"ZK" => Some(Reporter),
            b"ZL" => Some(PartyPassingTheTransaction),
            b"ZM" => Some(LeaseLocation),
            b"ZN" => Some(LosingInventoryManager),
            b"ZO" => Some(MinimumRoyaltyPayor),
            b"ZP" => Some(GainingInventoryManager),
            b"ZQ" => Some(ScreeningPoint),
            b"ZR" => Some(ValidatingParty),
            b"ZS" => Some(MonitoringParty),
            b"ZT" => Some(ParticipatingArea),
            b"ZU" => Some(Formation),
            b"ZV" => Some(AllowableRecipient),
            b"ZW" => Some(Field),
            b"ZX" => Some(AttorneyOfRecord),
            b"ZY" => Some(AmicusCuriae),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use EntityIdentifierCode::*;
        match self {
            AlternateInsurer => "Alternate Insurer",
            ComparableRentals => "Comparable Rentals",
            InterimFundingOrganization => "Interim Funding Organization",
            NonOccupantCoBorrower => "Non-occupant Co-borrower",
            ListOwner => "List Owner",
            ListMailer => "List Mailer",
            PrimaryElectronicBusinessContact => "Primary Electronic Business Contact",
            StateDivision => "State Division",
            AlternateElectronicBusinessContact => "Alternate Electronic Business Contact",
            PrimaryPracticeLocation => "Primary Practice Location",
            PartyToDeclareGoods => "Party to Declare Goods",
            LoanApplicant => "Loan Applicant",
            Pumper => "Pumper",
            Subgroup => "Subgroup",
            Applicant => "Applicant",
            Code1C => "Group Purchasing Organization (GPO)",
            CoOperative => "Co-operative",
            Code1E => "Health Maintenance Organization (HMO)",
            Alliance => "Alliance",
            OncologyCenter => "Oncology Center",
            KidneyDialysisUnit => "Kidney Dialysis Unit",
            Code1I => "Preferred Provider Organization (PPO)",
            Connection => "Connection",
            Franchisor => "Franchisor",
            Franchisee => "Franchisee",
            PreviousGroup => "Previous Group",
            Shareholder => "Shareholder",
            AcuteCareHospital => "Acute Care Hospital",
            Provider => "Provider",
            MilitaryFacility => "Military Facility",
            Code1R => "University, College or School",
            OutpatientSurgicenter => "Outpatient Surgicenter",
            Code1T => "Physician, Clinic or Group Practice",
            LongTermCareFacility => "Long Term Care Facility",
            ExtendedCareFacility => "Extended Care Facility",
            PsychiatricHealthFacility => "Psychiatric Health Facility",
            Laboratory => "Laboratory",
            RetailPharmacy => "Retail Pharmacy",
            HomeHealthCare => "Home Health Care",
            LoanBroker => "Loan Broker",
            SurfaceManagementEntity => "Surface Management Entity",
            Code2A => "Federal, State, County or City Facility",
            ThirdPartyAdministrator => "Third-Party Administrator",
            CoParticipant => "Co-Participant",
            MiscellaneousHealthCareFacility => "Miscellaneous Health Care Facility",
            NonHealthCareMiscellaneousFacility => {
                "Non-Health Care Miscellaneous Facility"
            }
            State => "State",
            Assigner => "Assigner",
            HospitalDistrictOrAuthority => "Hospital District or Authority",
            ChurchOperatedFacility => "Church Operated Facility",
            Individual => "Individual",
            Partnership => "Partnership",
            Corporation => "Corporation",
            AirForceFacility => "Air Force Facility",
            ArmyFacility => "Army Facility",
            NavyFacility => "Navy Facility",
            PublicHealthServiceFacility => "Public Health Service Facility",
            VeteransAdministrationFacility => "Veterans Administration Facility",
            FederalFacility => "Federal Facility",
            PublicHealthServiceIndianServiceFacility => {
                "Public Health Service Indian Service Facility"
            }
            DepartmentOfJusticeFacility => "Department of Justice Facility",
            OtherNotForProfitFacility => "Other Not-for-profit Facility",
            IndividualForProfitFacility => "Individual for-profit Facility",
            PartnershipForProfitFacility => "Partnership for-profit Facility",
            CorporationForProfitFacility => "Corporation for-profit Facility",
            GeneralMedicalAndSurgicalFacility => "General Medical and Surgical Facility",
            Code2Z => {
                "Hospital Unit of an Institution (prison hospital, college infirmary, etc.)"
            }
            Dependent => "Dependent",
            ApplicationParty => "Application Party",
            HospitalUnitWithinAnInstitutionForTheMentallyRetarded => {
                "Hospital Unit Within an Institution for the Mentally Retarded"
            }
            PsychiatricFacility => "Psychiatric Facility",
            TuberculosisAndOtherRespiratoryDiseasesFacility => {
                "Tuberculosis and Other Respiratory Diseases Facility"
            }
            ObstetricsAndGynecologyFacility => "Obstetrics and Gynecology Facility",
            Code3E => "Eye, Ear, Nose and Throat Facility",
            RehabilitationFacility => "Rehabilitation Facility",
            OrthopedicFacility => "Orthopedic Facility",
            ChronicDiseaseFacility => "Chronic Disease Facility",
            OtherSpecialtyFacility => "Other Specialty Facility",
            ChildrensGeneralFacility => "Children's General Facility",
            ChildrensHospitalUnitOfAnInstitution => {
                "Children's Hospital Unit of an Institution"
            }
            ChildrensPsychiatricFacility => "Children's Psychiatric Facility",
            ChildrensTuberculosisAndOtherRespiratoryDiseasesFacility => {
                "Children's Tuberculosis and Other Respiratory Diseases Facility"
            }
            Code3N => "Children's Eye, Ear, Nose and Throat Facility",
            ChildrensRehabilitationFacility => "Children's Rehabilitation Facility",
            ChildrensOrthopedicFacility => "Children's Orthopedic Facility",
            ChildrensChronicDiseaseFacility => "Children's Chronic Disease Facility",
            ChildrensOtherSpecialtyFacility => "Children's Other Specialty Facility",
            InstitutionForMentalRetardation => "Institution for Mental Retardation",
            AlcoholismAndOtherChemicalDependencyFacility => {
                "Alcoholism and Other Chemical Dependency Facility"
            }
            GeneralInpatientCareForAidsArcFacility => {
                "General Inpatient Care for AIDS/ARC Facility"
            }
            AidsArcUnit => "AIDS/ARC Unit",
            SpecializedOutpatientProgramForAidsArc => {
                "Specialized Outpatient Program for AIDS/ARC"
            }
            AlcoholDrugAbuseOrDependencyInpatientUnit => {
                "Alcohol/Drug Abuse or Dependency Inpatient Unit"
            }
            AlcoholDrugAbuseOrDependencyOutpatientServices => {
                "Alcohol/Drug Abuse or Dependency Outpatient Services"
            }
            ArthritisTreatmentCenter => "Arthritis Treatment Center",
            AssetAccountHolder => "Asset Account Holder",
            SiteOperator => "Site Operator",
            BirthingRoomLdrpRoom => "Birthing Room/LDRP Room",
            BurnCareUnit => "Burn Care Unit",
            CardiacCatherizationLaboratory => "Cardiac Catherization Laboratory",
            OpenHeartSurgeryFacility => "Open-Heart Surgery Facility",
            CardiacIntensiveCareUnit => "Cardiac Intensive Care Unit",
            AngioplastyFacility => "Angioplasty Facility",
            ChronicObstructivePulmonaryDiseaseServiceFacility => {
                "Chronic Obstructive Pulmonary Disease Service Facility"
            }
            EmergencyDepartment => "Emergency Department",
            Code4I => "Trauma Center (Certified)",
            Code4J => "Extracorporeal Shock-Wave Lithotripter (ESWL) Unit",
            FitnessCenter => "Fitness Center",
            GeneticCounselingScreeningServices => "Genetic Counseling/Screening Services",
            AdultDayCareProgramFacility => "Adult Day Care Program Facility",
            AlzheimersDiagnosticAssessmentServices => {
                "Alzheimer's Diagnostic/Assessment Services"
            }
            ComprehensiveGeriatricAssessmentFacility => {
                "Comprehensive Geriatric Assessment Facility"
            }
            Code4P => "Emergency Response (Geriatric) Unit",
            GeriatricAcuteCareUnit => "Geriatric Acute Care Unit",
            GeriatricClinics => "Geriatric Clinics",
            RespiteCareFacility => "Respite Care Facility",
            SeniorMembershipProgram => "Senior Membership Program",
            PatientEducationUnit => "Patient Education Unit",
            CommunityHealthPromotionFacility => "Community Health Promotion Facility",
            WorksiteHealthPromotionFacility => "Worksite Health Promotion Facility",
            HemodialysisFacility => "Hemodialysis Facility",
            HomeHealthServices => "Home Health Services",
            Hospice => "Hospice",
            Tenant => "Tenant",
            ConstructionContractor => "Construction Contractor",
            MedicalSurgicalOrOtherIntensiveCareUnit => {
                "Medical Surgical or Other Intensive Care Unit"
            }
            HisopathologyLaboratory => "Hisopathology Laboratory",
            BloodBank => "Blood Bank",
            NeonatalIntensiveCareUnit => "Neonatal Intensive Care Unit",
            ObstetricsUnit => "Obstetrics Unit",
            OccupationalHealthServices => "Occupational Health Services",
            OrganizedOutpatientServices => "Organized Outpatient Services",
            PediatricAcuteInpatientUnit => "Pediatric Acute Inpatient Unit",
            PsychiatricChildAdolescentServices => "Psychiatric Child/Adolescent Services",
            PsychiatricConsultationLiaisonServices => {
                "Psychiatric Consultation-Liaison Services"
            }
            PsychiatricEducationServices => "Psychiatric Education Services",
            PsychiatricEmergencyServices => "Psychiatric Emergency Services",
            PsychiatricGeriatricServices => "Psychiatric Geriatric Services",
            PsychiatricInpatientUnit => "Psychiatric Inpatient Unit",
            PsychiatricOutpatientServices => "Psychiatric Outpatient Services",
            PsychiatricPartialHospitalizationProgram => {
                "Psychiatric Partial Hospitalization Program"
            }
            MegavoltageRadiationTherapyUnit => "Megavoltage Radiation Therapy Unit",
            RadioactiveImplantsUnit => "Radioactive Implants Unit",
            TherapeuticRadioisotopeFacility => "Therapeutic Radioisotope Facility",
            XRayRadiationTherapyUnit => "X-Ray Radiation Therapy Unit",
            CtScannerUnit => "CT Scanner Unit",
            DiagnosticRadioisotopeFacility => "Diagnostic Radioisotope Facility",
            Code5W => "Magnetic Resonance Imaging (MRI) Facility",
            UltrasoundUnit => "Ultrasound Unit",
            RehabilitationInpatientUnit => "Rehabilitation Inpatient Unit",
            RehabilitationOutpatientServices => "Rehabilitation Outpatient Services",
            RecipientOfCivilOrLegalLiabilityPayment => {
                "Recipient of Civil or Legal Liability Payment"
            }
            DrillingContractor => "Drilling Contractor",
            ReproductiveHealthServices => "Reproductive Health Services",
            SkilledNursingOrOtherLongTermCareUnit => {
                "Skilled Nursing or Other Long-Term Care Unit"
            }
            Code6C => "Single Photon Emission Computerized Tomography (SPECT) Unit",
            OrganizedSocialWorkServiceFacility => {
                "Organized Social Work Service Facility"
            }
            OutpatientSocialWorkServices => "Outpatient Social Work Services",
            EmergencyDepartmentSocialWorkServices => {
                "Emergency Department Social Work Services"
            }
            SportsMedicineClinicServices => "Sports Medicine Clinic/Services",
            HospitalAuxiliaryUnit => "Hospital Auxiliary Unit",
            PatientRepresentativeServices => "Patient Representative Services",
            VolunteerServicesDepartment => "Volunteer Services Department",
            OutpatientSurgeryServices => "Outpatient Surgery Services",
            OrganTissueTransplantUnit => "Organ/Tissue Transplant Unit",
            OrthopedicSurgeryFacility => "Orthopedic Surgery Facility",
            OccupationalTherapyServices => "Occupational Therapy Services",
            PhysicalTherapyServices => "Physical Therapy Services",
            RecreationalTherapyServices => "Recreational Therapy Services",
            RespiratoryTherapyServices => "Respiratory Therapy Services",
            SpeechTherapyServices => "Speech Therapy Services",
            WomensHealthCenterServices => "Women's Health Center/Services",
            HealthSciencesLibrary => "Health Sciences Library",
            CardiacRehabilitationProgramFacility => {
                "Cardiac Rehabilitation Program Facility"
            }
            NonInvasiveCardiacAssessmentServices => {
                "Non-Invasive Cardiac Assessment Services"
            }
            EmergencyMedicalTechnician => "Emergency Medical Technician",
            DisciplinaryContact => "Disciplinary Contact",
            CaseManager => "Case Manager",
            Advisor => "Advisor",
            Titleholder => "Titleholder",
            SpudContractor => "Spud Contractor",
            Premises => "Premises",
            Bottler => "Bottler",
            PlaceOfOccurrence => "Place of Occurrence",
            ContractingOfficerRepresentative => "Contracting Officer Representative",
            PartyAuthorizedToDefinitizeContractAction => {
                "Party Authorized to Definitize Contract Action"
            }
            FilingAddress => "Filing Address",
            HazardousMaterialOffice => "Hazardous Material Office",
            GovernmentFurnishedPropertyFobPoint => {
                "Government Furnished Property FOB Point"
            }
            ProjectName => "Project Name",
            Codefendant => "Codefendant",
            CoOccupant => "Co-occupant",
            PreliminaryInspectionLocation => "Preliminary Inspection Location",
            InspectionAndAcceptanceLocation => "Inspection and Acceptance Location",
            PartyToReceiveProposal => "Party to Receive Proposal",
            FederallyCharteredFacility => "Federally Chartered Facility",
            TransportationOffice => "Transportation Office",
            PartyToWhomProtestSubmitted => "Party to Whom Protest Submitted",
            Birthplace => "Birthplace",
            PipelineSegment => "Pipeline Segment",
            HomeStateName => "Home State Name",
            Liquidator => "Liquidator",
            PetitioningCreditorsAttorney => "Petitioning Creditor's Attorney",
            MergedName => "Merged Name",
            PartyRepresented => "Party Represented",
            ProfessionalOrganization => "Professional Organization",
            Referee => "Referee",
            NonMortgageLiabilityAccountHolder => "Non-Mortgage Liability Account Holder",
            LienHolder => "Lien Holder",
            VacationHome => "Vacation Home",
            PrimaryResidence => "Primary Residence",
            SecondHome => "Second Home",
            PermitHolder => "Permit Holder",
            MinorityInstitution => "Minority Institution",
            BailmentWarehouse => "Bailment Warehouse",
            FirstAppraiser => "First Appraiser",
            TaxExemptOrganization => "Tax Exempt Organization",
            ServiceOrganization => "Service Organization",
            EmergingSmallBusiness => "Emerging Small Business",
            SurplusDealer => "Surplus Dealer",
            PollingSite => "Polling Site",
            SociallyDisadvantagedIndividual => "Socially Disadvantaged Individual",
            EconomicallyDisadvantagedIndividual => {
                "Economically Disadvantaged Individual"
            }
            DisabledIndividual => "Disabled Individual",
            Producer => "Producer",
            PublicOrPrivateOrganizationForTheDisabled => {
                "Public or Private Organization for the Disabled"
            }
            Code8R => "Consumer Service Provider (CSP) Customer",
            Code8S => "Consumer Service Provider (CSP)",
            Voter => "Voter",
            NativeHawaiianOrganization => "Native Hawaiian Organization",
            Code8V => "Primary Intra-LATA (Local Access Transport Area) Carrier",
            PaymentAddress => "Payment Address",
            OilAndGasCustodian => "Oil and Gas Custodian",
            RegisteredOffice => "Registered Office",
            NoteCoSigner => "Note Co-Signer",
            DebtorIndividual => "Debtor Individual",
            CountryOfExport => "Country of Export",
            CountryOfDestination => "Country of Destination",
            NewServiceProvider => "New Service Provider",
            SubServicer => "Sub-servicer",
            LossPayee => "Loss Payee",
            Nickname => "Nickname",
            Assignee => "Assignee",
            RegisteredPrincipal => "Registered Principal",
            AdditionalDebtor => "Additional Debtor",
            KeyPerson => "Key Person",
            IncorporatedBy => "Incorporated By",
            PartyToLease => "Party to Lease",
            PartyToContract => "Party to Contract",
            Investigator => "Investigator",
            LastSupplier => "Last Supplier",
            DownstreamFirstSupplier => "Downstream First Supplier",
            CoInvestigator => "Co-Investigator",
            TelephoneAnsweringServiceBureau => "Telephone Answering Service Bureau",
            Author => "Author",
            FirstSupplier => "First Supplier",
            UltimateParentCompany => "Ultimate Parent Company",
            ContractualReceiptMeter => "Contractual Receipt Meter",
            ContractualDeliveryMeter => "Contractual Delivery Meter",
            CoDebtor => "Co-debtor",
            Conduit => "Conduit",
            Code11 => "Party to be billed(AAR Accounting Rule 11)",
            RegionalOffice => "Regional Office",
            ContractedServiceProvider => "Contracted Service Provider",
            WhollyOwnedSubsidiary => "Wholly-Owned Subsidiary",
            AccountsPayableOffice => "Accounts Payable Office",
            Plant => "Plant",
            ConsultantsOffice => "Consultant's Office",
            Production => "Production",
            NonProductionSupplier => "Non-Production Supplier",
            ForeignSupplier => "Foreign Supplier",
            SmallBusiness => "Small Business",
            Code22 => "Minority-Owned Business, Small",
            Code23 => "Minority-Owned Business, Large",
            Code24 => "Woman-Owned Business, Small",
            Code25 => "Woman-Owned Business, Large",
            SociallyDisadvantagedBusiness => "Socially Disadvantaged Business",
            SmallDisadvantagedBusiness => "Small Disadvantaged Business",
            Subcontractor => "Subcontractor",
            PrototypeSupplier => "Prototype Supplier",
            ServiceSupplier => "Service Supplier",
            PostalMailingAddress => "Postal Mailing Address",
            PartyToReceiveMaterialRelease => "Party to Receive Material Release",
            InquiryAddress => "Inquiry Address",
            MaterialChangeNoticeAddress => "Material Change Notice Address",
            Code35 => "Electronic Data Interchange (EDI) Coordinator Point Address",
            Employer => "Employer",
            PreviousDebtHolder => "Previous Debt Holder",
            MortgageLiabilityAccountHolder => "Mortgage Liability Account Holder",
            AppraisalCompany => "Appraisal Company",
            Receiver => "Receiver",
            Submitter => "Submitter",
            ComponentManufacturer => "Component Manufacturer",
            ClaimantAuthorizedRepresentative => "Claimant Authorized Representative",
            DataProcessingServiceBureau => "Data Processing Service Bureau",
            DropOffLocation => "Drop-off Location",
            InvoicingDealer => "Invoicing Dealer",
            Estimator => "Estimator",
            InServiceSource => "In-service Source",
            InitialDealer => "Initial Dealer",
            ManufacturersRepresentative => "Manufacturer's Representative",
            PartsDistributor => "Parts Distributor",
            PartRemanufacturer => "Part Remanufacturer",
            RegisteredOwner => "Registered Owner",
            OrderWriter => "Order Writer",
            ServiceManager => "Service Manager",
            ServicingDealer => "Servicing Dealer",
            ServicingOrganization => "Servicing Organization",
            StoreManager => "Store Manager",
            PartyToApproveSpecification => "Party to Approve Specification",
            Salesperson => "Salesperson",
            PerformedAt => "Performed At",
            ApplicantsEmployer => "Applicant's Employer",
            ReferencesEmployer => "Reference's Employer",
            CosignersEmployer => "Cosigner's Employer",
            ApplicantsReference => "Applicant's Reference",
            ApplicantsCosigner => "Applicant's Cosigner",
            ApplicantsComaker => "Applicant's Comaker",
            OwnersRepresentative => "Owner's Representative",
            RepairingOutlet => "Repairing Outlet",
            PriorIncorrectInsured => "Prior Incorrect Insured",
            AttendingPhysician => "Attending Physician",
            OperatingPhysician => "Operating Physician",
            OtherPhysician => "Other Physician",
            CorrectedInsured => "Corrected Insured",
            Participant => "Participant",
            SecondaryWarranter => "Secondary Warranter",
            ServiceLocation => "Service Location",
            ServiceRequester => "Service Requester",
            Warranter => "Warranter",
            Hospital => "Hospital",
            PartSource => "Part Source",
            RenderingProvider => "Rendering Provider",
            SubscribersSchool => "Subscriber's School",
            SubscribersEmployer => "Subscriber's Employer",
            BillingProvider => "Billing Provider",
            Conductor => "Conductor",
            PayToProvider => "Pay-to Provider",
            Approver => "Approver",
            Investor => "Investor",
            PreviousBusinessPartner => "Previous Business Partner",
            ActionParty => "Action Party",
            SupportParty => "Support Party",
            InsuranceInstitute => "Insurance Institute",
            NewSupplySource => "New Supply Source",
            ResearchInstitute => "Research Institute",
            DebtorCompany => "Debtor Company",
            PartyWaivingRequirements => "Party Waiving Requirements",
            FreightManagementFacilitator => "Freight Management Facilitator",
            Code99 => "Outer Continental Shelf (OCS) Area Location",
            Adjuster => "Adjuster",
            WomanOwnedBusiness => "Woman-Owned Business",
            LaborSurplusAreaFirm => "Labor Surplus Area Firm",
            OtherDisadvantagedBusiness => "Other Disadvantaged Business",
            VeteranOwnedBusiness => "Veteran-Owned Business",
            CodeA6 => "Section 8(a) Program Participant Firm",
            ShelteredWorkshop => "Sheltered Workshop",
            NonprofitInstitution => "Nonprofit Institution",
            SalesOffice => "Sales Office",
            AuthorityForShipment => "Authority For Shipment",
            CodeAA1 => "Chief Executive Officer (CEO)",
            FinancialAidOffice => "Financial Aid Office",
            Respondent => "Respondent",
            AdmissionOffice => "Admission Office",
            MultiCampusAdministrativeUnit => "Multi-Campus Administrative Unit",
            Headmaster => "Headmaster",
            BusinessOfficer => "Business Officer",
            Superintendent => "Superintendent",
            SchoolPrincipal => "School Principal",
            SubAccount => "Sub-account",
            ManagementNonOfficer => "Management Non-Officer",
            IncorporatedLocation => "Incorporated Location",
            NameNotToBeConfusedWith => "Name not to be Confused with",
            Lot => "Lot",
            PreviousOccupant => "Previous Occupant",
            GroundAmbulanceServices => "Ground Ambulance Services",
            AirAmbulanceServices => "Air Ambulance Services",
            WaterAmbulanceServices => "Water Ambulance Services",
            AdmittingServices => "Admitting Services",
            PrimarySurgeon => "Primary Surgeon",
            MedicalNurse => "Medical Nurse",
            CardiacRehabilitationServices => "Cardiac Rehabilitation Services",
            SkilledNursingServices => "Skilled Nursing Services",
            ObservationRoomServices => "Observation Room Services",
            Employee => "Employee",
            AnesthesiologyServices => "Anesthesiology Services",
            PriorBaseJurisdiction => "Prior Base Jurisdiction",
            IncorporationJurisdiction => "Incorporation Jurisdiction",
            MarkerOwner => "Marker Owner",
            ReclamationCenter => "Reclamation Center",
            PartyProvidingFinancing => "Party Providing Financing",
            AdditionalPickupAddress => "Additional Pickup Address",
            PrivateSchoolSystem => "Private School System",
            StateOperatedSchoolSystem => "State Operated School System",
            VocationalRegionsSchoolSystem => "Vocational Regions School System",
            CharteredSchoolDistrict => "Chartered School District",
            SchoolingOfIndianChildrenSchoolSystem => {
                "Schooling of Indian Children School System"
            }
            UnorganizedTerritoriesSchoolSystem => "Unorganized Territories School System",
            StateAdministeredDistrict => "State Administered District",
            TownsInUnionsSchoolSystem => "Towns in Unions School System",
            AgentTownsSchoolSystem => "Agent Towns School System",
            MasterProperty => "Master Property",
            ProjectProperty => "Project Property",
            UnitProperty => "Unit Property",
            AdditionalAddress => "Additional Address",
            SocietyOfPropertyInformationCompilersAndAnalysts => {
                "Society of Property Information Compilers and Analysts"
            }
            Organization => "Organization",
            JointOwnerAnnuitant => "Joint Owner Annuitant",
            JointAnnuitantOwner => "Joint Annuitant Owner",
            JointOwnerAnnuitantPayor => "Joint Owner Annuitant Payor",
            JointOwnerJointAnnuitant => "Joint Owner Joint Annuitant",
            JointOwnerJointAnnuitantPayor => "Joint Owner Joint Annuitant Payor",
            JointOwnerPayor => "Joint Owner Payor",
            Acronym => "Acronym",
            NewAddress => "New Address",
            Chairperson => "Chairperson",
            DecisionMaker => "Decision Maker",
            FormerPresident => "Former President",
            Founder => "Founder",
            ImportedFromLocation => "Imported from Location",
            LiterallyTranslatedName => "Literally Translated Name",
            OriginalLocation => "Original Location",
            President => "President",
            RatingOrganization => "Rating Organization",
            AirCargoCompany => "Air Cargo Company",
            RegionalCenter => "Regional Center",
            CodeAC2 => "Local Education Agency (LEA)",
            StateEducationAgency => "State Education Agency",
            InitialMedicalProvider => "Initial Medical Provider",
            ConcurrentEmployer => "Concurrent Employer",
            RoutingPoint => "Routing Point",
            BorderCrossing => "Border Crossing",
            BobtailServicePoint => "Bobtail Service Point",
            Auditor => "Auditor",
            InsuredLocation => "Insured Location",
            ReferralProvider => "Referral Provider",
            Affiliate => "Affiliate",
            AlliedHealthProfessional => "Allied Health Professional",
            EmergencyProvider => "Emergency Provider",
            FederalGovernment => "Federal Government",
            FellowshipInstitution => "Fellowship Institution",
            GovernmentCombinedControl => "Government - Combined Control",
            GovernmentFederalMilitary => "Government - Federal - Military",
            GovernmentFederalOther => "Government - Federal - Other",
            GovernmentFederalVeterans => "Government - Federal - Veterans",
            GovernmentLocal => "Government - Local",
            GroupAffiliation => "Group Affiliation",
            InformationSource => "Information Source",
            InternshipEntity => "Internship Entity",
            MedicalSchool => "Medical School",
            NationalOrganization => "National Organization",
            CodeACZ => {
                "Non-Profit Health Care Provider\nSee U.S. Internal Revenue Code Chapter 1, Subchapter F, Part 1, Section 501(c)"
            }
            CodeAD => "Party to be advised (Written orders)",
            CodeADA => {
                "Not for Profit Health Care Provider\nSee U.S. Internal Revenue Code Chapter 1, Subchapter F, Part 1, Section 501(c)"
            }
            ForProfitHealthCareProvider => "For Profit Health Care Provider",
            OfficeManager => "Office Manager",
            OnCallProvider => "On-call Provider",
            CodeADE => "Physician Hospital Organization (PHO)",
            CodeADF => "Point of Service (POS)",
            ResidencyInstitution => "Residency Institution",
            SharedService => "Shared Service",
            SupportingPersonnel => "Supporting Personnel",
            TrainingInstitution => "Training Institution",
            PublicSchool => "Public School",
            PrivateSchool => "Private School",
            PublicPreKEducation => "Public Pre-K Education",
            PrivatePreKEducation => "Private Pre-K Education",
            PreKDayCare => "Pre-K Day Care",
            CharterSchool => "Charter School",
            HomeSchool => "Home School",
            PublicAlternativeSchool => "Public Alternative School",
            NeglectedDelinquentInstitution => "Neglected/Delinquent Institution",
            PostSecondaryInstitution => "Post-Secondary Institution",
            FoodServiceOperator => "Food Service Operator",
            FutureAddress => "Future Address",
            FormerRegisteredAddress => "Former Registered Address",
            TopParentCompanyInSameCountry => "Top Parent Company in Same Country",
            AdditionalDeliveryAddress => "Additional Delivery Address",
            SecondLevelParentCompany => "Second Level Parent Company",
            AirportAuthority => "Airport Authority",
            CouncilOfGovernments => "Council of Governments",
            Foundation => "Foundation",
            PortAuthority => "Port Authority",
            PlanningCommission => "Planning Commission",
            CarRentalLocation => "Car Rental Location",
            LodgingFacility => "Lodging Facility",
            PartyToReceiveTransportationCredit => {
                "Party to Receive Transportation Credit"
            }
            CodeAEK => "Party to Receive Packing, Crating, and Handling Credit",
            PrimaryInternationalTelecomCarrier => "Primary International Telecom Carrier",
            AuthorizedAcceptingOfficial => "Authorized Accepting Official",
            AgentAgency => "Agent/Agency",
            Advertiser => "Advertiser",
            AgencyHazardousMaterialInformationSystemLocation => {
                "Agency Hazardous Material Information System Location"
            }
            Airline => "Airline",
            AllegedDebtor => "Alleged Debtor",
            PartyToWhomAcknowledgmentShouldBeSent => {
                "Party to Whom Acknowledgment Should Be Sent"
            }
            AllotmentCustomer => "Allotment Customer",
            AlternativeAddressee => "Alternative Addressee",
            ActivityLocation => "Activity Location",
            AssistantUSTrustee => "Assistant U.S. Trustee",
            AuthorizedFrom => "Authorized From",
            AccountOf => "Account Of",
            CodeAP => "Account of (Origin Party)",
            ActivityProvider => "Activity Provider",
            CodeAQ => "Account of (Destination Party)",
            ArmedServicesLocationDesignation => "Armed Services Location Designation",
            PostsecondaryEducationSender => "Postsecondary Education Sender",
            PostsecondaryEducationRecipient => "Postsecondary Education Recipient",
            AlternateTaxAuthority => "Alternate Tax Authority",
            PartyAuthorizingDisposition => "Party Authorizing Disposition",
            AuthorizingOfficial => "Authorizing Official",
            AuthorizedTo => "Authorized To",
            Accountant => "Accountant",
            Plaintiff => "Plaintiff",
            Clearinghouse => "Clearinghouse",
            PreviousName => "Previous Name",
            ConstructionFirm => "Construction Firm",
            OtherUnlistedTypeOfOrganizationalEntity => {
                "Other Unlisted Type of Organizational Entity"
            }
            PreviousNameOfFirm => "Previous Name of Firm",
            ParentCompany => "Parent Company",
            AffiliatedCompany => "Affiliated Company",
            RegisteringParentParty => "Registering Parent Party",
            RegisteringNonparentParty => "Registering Nonparent Party",
            RegularDealer => "Regular Dealer",
            LargeBusiness => "Large Business",
            Battery => "Battery",
            Bailiff => "Bailiff",
            BusinessPartner => "Business Partner",
            Broadcaster => "Broadcaster",
            BillToPartyForDiversionCharges => "Bill-to Party for Diversion Charges",
            Beneficiary => "Beneficiary",
            BilledFrom => "Billed From",
            BuyingGroup => "Buying Group",
            InterimTrustee => "Interim Trustee",
            TrusteesAttorney => "Trustee's Attorney",
            CoCounsel => "Co-Counsel",
            Bank => "Bank",
            Bookkeeper => "Bookkeeper",
            PartyToReceiveBillOfLading => "Party to Receive Bill of Lading",
            Building => "Building",
            Structure => "Structure",
            Brakeman => "Brakeman",
            BeneficialOwner => "Beneficial Owner",
            BrokerOrSalesOffice => "Broker or Sales Office",
            BodyOfWater => "Body of Water",
            SpecialCounsel => "Special Counsel",
            AttorneyForDefendantPrivate => "Attorney for Defendant Private",
            Broker => "Broker",
            BrandName => "Brand Name",
            BillAndShipTo => "Bill and Ship To",
            BillToParty => "Bill-to-Party",
            PlaceOfBusiness => "Place of Business",
            Business => "Business",
            BillingService => "Billing Service",
            Borrower => "Borrower",
            AttorneyForPlaintiff => "Attorney for Plaintiff",
            CodeBY => "Buying Party (Purchaser)",
            BusinessAssociate => "Business Associate",
            AssistantConductor => "Assistant Conductor",
            InCareOfPartyNo1 => "In Care Of Party no. 1",
            InCareOfPartyNo2 => "In Care Of Party no. 2",
            CircuitLocation => "Circuit Location Identifier",
            ContractAdministrationOffice => "Contract Administration Office",
            SecondaryContractAdministrationOffice => {
                "Secondary Contract Administration Office"
            }
            PartySubmittingQuote => "Party Submitting Quote",
            Municipality => "Municipality",
            County => "County",
            City => "City",
            ContractHolder => "Contract Holder",
            Carrier => "Carrier",
            CustomsBroker => "Customs Broker",
            Claimant => "Claimant",
            CodeCD => "Consignee (To Receive Mail and Small Parcels)",
            CodeCE => "Consignee (To receive large parcels and freight)",
            SubsidiaryDivision => "Subsidiary/Division",
            CarnetIssuer => "Carnet Issuer",
            ChassisProvider => "Chassis Provider",
            ChangedAddress => "Changed Address",
            Consignor => "Consignor",
            CodeCJ => "Automated Data Processing (ADP) Point",
            Pharmacist => "Pharmacist",
            ContainerLocation => "Container Location",
            BuildingCluster => "Building Cluster",
            Customs => "Customs",
            CompanyMergedWith => "Company Merged With",
            Consignee => "Consignee",
            ConfirmingParty => "Confirming Party",
            ConfirmationRequester => "Confirmation Requester",
            ConfirmationServiceIdentifierCode => "Confirmation Service Identifier Code",
            OceanTariffConference => "Ocean Tariff Conference",
            CoDriver => "Co-Driver",
            CollateralAssignee => "Collateral Assignee",
            Complainant => "Complainant",
            CorrectedName => "Corrected Name",
            PartyToReceiveCertOfCompliance => "Party to Receive Cert. of Compliance",
            CorporateOffice => "Corporate Office",
            ContainerReturnCompany => "Container Return Company",
            CrewMember => "Crew Member",
            Consolidator => "Consolidator",
            CountryOfOrigin => "Country of Origin",
            CoatingOrPaintSupplier => "Coating or Paint Supplier",
            Converter => "Converter",
            AccountingStation => "Accounting Station",
            ClaimAdministrator => "Claim Administrator",
            Country => "Country",
            AdmittingSurgeon => "Admitting Surgeon",
            Driver => "Driver",
            CommercialInsurer => "Commercial Insurer",
            Defendant => "Defendant",
            Debtor => "Debtor",
            DebtorInPossession => "Debtor-In-Possession",
            ConsolidatedDebtor => "Consolidated Debtor",
            PetitioningCreditor => "Petitioning Creditor",
            Dispatcher => "Dispatcher",
            CreditorsAttorney => "Creditor's Attorney",
            DeliveryAddress => "Delivery Address",
            DamagedBy => "Damaged By",
            DistributorBranch => "Distributor Branch",
            DestinationCarrier => "Destination Carrier",
            ChiefDeputyClerkOfCourt => "Chief Deputy Clerk of Court",
            AssistantSurgeon => "Assistant Surgeon",
            Depositor => "Depositor",
            MaterialDispositionAuthorizationLocation => {
                "Material Disposition Authorization Location"
            }
            DesignEngineering => "Design Engineering",
            DoingBusinessAs => "Doing Business As",
            CodeDI => "Different Premise Address (DPA)",
            DistributionRecipient => "Distribution Recipient",
            ConsultingPhysician => "Consulting Physician",
            OrderingPhysician => "Ordering Physician",
            Dealer => "Dealer",
            DestinationMailFacility => "Destination Mail Facility",
            ReferringProvider => "Referring Provider",
            DependentName => "Dependent Name",
            PartyToProvideDiscount => "Party to Provide Discount",
            SupervisingPhysician => "Supervising Physician",
            DestinationDrayman => "Destination Drayman",
            Distributor => "Distributor",
            DestinationTerminal => "Destination Terminal",
            ResaleDealer => "Resale Dealer",
            Division => "Division",
            DownstreamParty => "Downstream Party",
            Distiller => "Distiller",
            DefaultForeclosureSpecialist => "Default/Foreclosure Specialist",
            DeliveryZone => "Delivery Zone",
            AssistantEngineer => "Assistant Engineer",
            PersonOrOtherEntityLegallyResponsibleForAChild => {
                "Person or Other Entity Legally Responsible for a Child"
            }
            PersonOrOtherEntityWithWhomAChildResides => {
                "Person or Other Entity With Whom a Child Resides"
            }
            PersonOrOtherEntityLegallyResponsibleForAndWithWhomAChildResides => {
                "Person or Other Entity Legally Responsible for and With Whom a Child Resides"
            }
            OtherPersonOrEntityAssociatedWithStudent => {
                "Other Person or Entity Associated with Student"
            }
            Examiner => "Examiner",
            Engineering => "Engineering",
            PreviousEmployer => "Previous Employer",
            InquiringParty => "Inquiring Party",
            ParticipatingLaboratory => "Participating Laboratory",
            StudySubmitter => "Study Submitter",
            Assistant => "Assistant",
            CampaignManager => "Campaign Manager",
            Client => "Client",
            Commissioner => "Commissioner",
            Committee => "Committee",
            Contestant => "Contestant",
            Contributor => "Contributor",
            DeputyChairperson => "Deputy Chairperson",
            DeputyTreasurer => "Deputy Treasurer",
            Donor => "Donor",
            Endorser => "Endorser",
            Guarantor => "Guarantor",
            Headquarters => "Headquarters",
            IndependentContractor => "Independent Contractor",
            Leader => "Leader",
            PartyPerformingLiaison => "Party Performing Liaison",
            LobbyingFirm => "Lobbying Firm",
            Lobbyist => "Lobbyist",
            MediaContact => "Media Contact",
            OfficeHolder => "Office Holder",
            PartyAuthorizedToAdministerOaths => "Party Authorized to Administer Oaths",
            PartyToBenefit => "Party to Benefit",
            PartyHoldingInterest => "Party Holding Interest",
            PartyMakingPledge => "Party Making Pledge",
            PartyReturningContribution => "Party Returning Contribution",
            EligiblePartyToTheContract => "Eligible Party To The Contract",
            PartyReturningTransfer => "Party Returning Transfer",
            LobbiedParty => "Lobbied Party",
            PoliticalActionCommittee => "Political Action Committee",
            PoliticalParty => "Political Party",
            Proponent => "Proponent",
            PublicOfficial => "Public Official",
            ReceivingCommittee => "Receiving Committee",
            AffiliatedCommittee => "Affiliated Committee",
            Source => "Source",
            Sponsor => "Sponsor",
            SponsoredCommittee => "Sponsored Committee",
            Designee => "Designee",
            TemporaryResidence => "Temporary Residence",
            Treasurer => "Treasurer",
            ViceChairperson => "Vice-Chairperson",
            SlateMailerOrganization => "Slate Mailer Organization",
            LodgingLocation => "Lodging Location",
            IndependentExpenditureCommittee => "Independent Expenditure Committee",
            MajorDonor => "Major Donor",
            Exchanger => "Exchanger",
            ExcludedParty => "Excluded Party",
            LocationOfGoodsForCustomsExaminationBeforeClearance => {
                "Location of Goods for Customs Examination Before Clearance"
            }
            ElectronicFiler => "Electronic Filer",
            Engineer => "Engineer",
            Exhibitor => "Exhibitor",
            ExecutorOfEstate => "Executor of Estate",
            PrincipalPerson => "Principal Person",
            AnimalSource => "Animal Source",
            EstablishedLocation => "Established Location",
            PartyToReceiveElectronicMemoOfInvoice => {
                "Party to Receive Electronic Memo of Invoice"
            }
            EndUser => "End User",
            Enroller => "Enroller",
            LimitedLiabilityPartnership => "Limited Liability Partnership",
            EligiblePartyToTheRate => "Eligible Party to the Rate",
            OldDebtor => "Old Debtor",
            NewDebtor => "New Debtor",
            PlanAdministrator => "Plan Administrator",
            OldSecuredParty => "Old Secured Party",
            SellingAgent => "Selling Agent",
            ServicingBroker => "Servicing Broker",
            Exporter => "Exporter",
            ExSpouse => "Ex-spouse",
            EmployeeName => "Employee Name",
            NewSecuredParty => "New Secured Party",
            CompanyOwnedOilField => "Company - Owned Oil Field",
            CodeF2 => {
                "Energy Information Administration (Department of Energy) - Owned Oil Field"
            }
            CodeF3 => "Specialized Mobile Radio Service (SMRS) Licensee",
            FormerResidence => "Former Residence",
            RadioControlStationLocation => "Radio Control Station Location",
            SmallControlStationLocation => "Small Control Station Location",
            SmallBaseStationLocation => "Small Base Station Location",
            AntennaSite => "Antenna Site",
            AreaOfOperation => "Area of Operation",
            Facility => "Facility",
            FirstBreakTerminal => "First Break Terminal",
            CodeFC => "Customer Identification File (CIF) Customer Identifier",
            PhysicalAddress => "Physical Address",
            MailAddress => "Mail Address",
            ForeignLanguageSynonym => "Foreign Language Synonym",
            TradeNameSynonym => "Trade Name Synonym",
            ForeignGovernment => "Foreign Government",
            PartyToReceiveLimitationsOfHeavyElementsReport => {
                "Party to Receive Limitations of Heavy Elements Report"
            }
            NameVariationSynonym => "Name Variation Synonym",
            FirstContact => "First Contact",
            PrimaryControlPointLocation => "Primary Control Point Location",
            Fireman => "Fireman",
            FilerName => "Filer Name",
            FieldOrBranchOffice => "Field or Branch Office",
            NameOnCreditCard => "Name on Credit Card",
            PierName => "Pier Name",
            MessageFrom => "Message From",
            ForeignRegistrationLocation => "Foreign Registration Location",
            FinalScheduledDestination => "Final Scheduled Destination",
            PartyToReceiveSensitiveForeignDisclosure => {
                "Party to Receive Sensitive Foreign Disclosure Information"
            }
            FinancialStatementRecipient => "Financial Statement Recipient",
            NewAssignee => "New Assignee",
            OldAssignee => "Old Assignee",
            VesselName => "Vessel Name",
            Forwarder => "Forwarder",
            ClosedDoorPharmacy => "Closed Door Pharmacy",
            VeterinaryHospital => "Veterinary Hospital",
            ChildrensDayCareCenter => "Children's Day Care Center",
            DependentInsured => "Dependent Insured",
            BankruptcyTrustee => "Bankruptcy Trustee",
            Annuitant => "Annuitant",
            Clinic => "Clinic",
            ContingentBeneficiary => "Contingent Beneficiary",
            EntityHoldingThe => "Entity Holding the Information",
            EntityProvidingTheService => "Entity Providing the Service",
            EntityResponsibleForFollowUp => "Entity Responsible for Follow-up",
            FamilyMember => "Family Member",
            GasPlant => "Gas Plant",
            OtherInsured => "Other Insured",
            AlternateGovernmentBusinessContact => "Alternate Government Business Contact",
            GateBooth => "Gate Booth",
            PrimaryGovernmentBusinessContact => "Primary Government Business Contact",
            PreviousCreditGrantor => "Previous Credit Grantor",
            Guardian => "Guardian",
            GeneralAgency => "General Agency",
            InspectionCompany => "Inspection Company",
            Intermediary => "Intermediary",
            MotorVehicleReportProviderCompany => "Motor Vehicle Report Provider Company",
            Paramedic => "Paramedic",
            GiftRecipient => "Gift Recipient",
            ParamedicalCompany => "Paramedical Company",
            PreviousInsured => "Previous Insured",
            PreviousResidence => "Previous Residence",
            SpouseInsured => "Spouse Insured",
            Garnishee => "Garnishee",
            PrimaryBeneficiary => "Primary Beneficiary",
            GatewayProvider => "Gateway Provider",
            ProposedInsured => "Proposed Insured",
            Reinsurer => "Reinsurer",
            GaragedLocation => "Garaged Location",
            CreditGrantor => "Credit Grantor",
            GuaranteeAgency => "Guarantee Agency",
            GasTransactionEndingPoint => "Gas Transaction Ending Point",
            Group => "Group",
            Retrocessionaire => "Retrocessionaire",
            TreatmentFacility => "Treatment Facility",
            Grandparent => "Grandparent",
            Representative => "Representative",
            SubOffice => "Sub-Office",
            District => "District",
            PayingAgent => "Paying Agent",
            SchoolDistrict => "School District",
            GroupAffiliate => "Group Affiliate",
            Designer => "Designer",
            Owner => "Owner",
            HistoricallyBlackCollegeOrUniversity => {
                "Historically Black College or University"
            }
            JointAnnuitant => "Joint Annuitant",
            ContingentAnnuitant => "Contingent Annuitant",
            ContingentOwner => "Contingent Owner",
            CodeHF => "Healthcare Professional Shortage Area (HPSA) Facility",
            BrokerOpinionOrAnalysisRequester => "Broker Opinion or Analysis Requester",
            HomeHealthAgency => "Home Health Agency",
            ListingCompany => "Listing Company",
            AutomatedUnderwritingSystem => "Automated Underwriting System",
            Subscriber => "Subscriber",
            DocumentCustodian => "Document Custodian",
            CompetitivePropertyListing => "Competitive Property Listing",
            CodeHMI => "Material Safety Data Sheet (MSDS) Recipient",
            CompetingProperty => "Competing Property",
            ComparablePropertyListing => "Comparable Property Listing",
            HomeOffice => "Home Office",
            HonorarySociety => "Honorary Society",
            ClosedSale => "Closed Sale",
            SourcePartyOf => "Source Party of Information",
            SubjectOfInquiry => "Subject of Inquiry",
            HighSchool => "High School",
            StateCharteredFacility => "State Chartered Facility",
            Subsidiary => "Subsidiary",
            TaxAddress => "Tax Address",
            DesignatedHazardousWasteFacility => "Designated Hazardous Waste Facility",
            TransporterOfHazardousWaste => "Transporter of Hazardous Waste",
            Charity => "Charity",
            HazardousWasteGenerator => "Hazardous Waste Generator",
            InterestedParty => "Interested Party",
            CodeI3 => "Independent Physicians Association (IPA)",
            IntellectualPropertyOwner => "Intellectual Property Owner",
            Interviewer => "Interviewer",
            InstalledAt => "Installed At",
            BusinessEntity => "Business Entity",
            PrincipalExecutiveOffice => "Principal Executive Office",
            ForeignOffice => "Foreign Office",
            Member => "Member",
            ExecutiveCommitteeMember => "Executive Committee Member",
            Director => "Director",
            Clerk => "Clerk",
            PartyWithKnowledgeOfAffairsOfTheCompany => {
                "Party with Knowledge of Affairs of the Company"
            }
            PartyToReceiveStatementOfFeesDue => "Party to Receive Statement of Fees Due",
            CompanyInWhichInterestHeld => "Company in which Interest Held",
            CompanyWhichHoldsInterest => "Company which Holds Interest",
            Notary => "Notary",
            Manager => "Manager",
            AlienAffiliate => "Alien Affiliate",
            IncorporationStatePrincipalOffice => "Incorporation State Principal Office",
            IncorporationStatePlaceOfBusiness => "Incorporation State Place of Business",
            OutOfStatePrincipalOffice => "Out-of-State Principal Office",
            PartyExecutingAndVerifying => "Party Executing and Verifying",
            Felon => "Felon",
            OtherRelatedParty => "Other Related Party",
            RecordKeepingAddress => "Record-Keeping Address",
            InitialSubscriber => "Initial Subscriber",
            OriginalJurisdiction => "Original Jurisdiction",
            IndustryBureau => "Industry Bureau",
            IntermediateConsignee => "Intermediate Consignee",
            InventoryControlPoint => "Inventory Control Point",
            IssuerOfDebitOrCreditMemo => "Issuer of Debit or Credit Memo",
            OtherIndividualDisabilityCarrier => "Other Individual Disability Carrier",
            InternationalFreightForwarder => "International Freight Forwarder",
            InsolventInsurer => "Insolvent Insurer",
            IssuerOfInvoice => "Issuer of Invoice",
            InjectionPoint => "Injection Point",
            IntermediateCarrier => "Intermediate Carrier",
            InsuredOrSubscriber => "Insured or Subscriber",
            Importer => "Importer",
            IntegratedMaterialManager => "Integrated Material Manager",
            Insurer => "Insurer",
            Interviewee => "Interviewee",
            InvestmentAdvisor => "Investment Advisor",
            Inspector => "Inspector",
            IndependentAdjuster => "Independent Adjuster",
            InPatientPharmacy => "In-patient Pharmacy",
            SelfInsured => "Self Insured",
            PartyToReceiveCertifiedInspectionReport => {
                "Party to Receive Certified Inspection Report"
            }
            InstallationOnSite => "Installation on Site",
            Issuer => "Issuer",
            Renter => "Renter",
            AssociateGeneralAgent => "Associate General Agent",
            AuthorizedEntity => "Authorized Entity",
            BrokersAssistant => "Broker's Assistant",
            Custodian => "Custodian",
            IrrevocableBeneficiary => "Irrevocable Beneficiary",
            PowerOfAttorney => "Power of Attorney",
            TrustOfficer => "Trust Officer",
            BrokerDealer => "Broker Dealer",
            CommunityAgent => "Community Agent",
            DairyDepartment => "Dairy Department",
            DelicatessenDepartment => "Delicatessen Department",
            DryGroceryDepartment => "Dry Grocery Department",
            Judge => "Judge",
            FrozenDepartment => "Frozen Department",
            GeneralMerchandiseDepartment => "General Merchandise Department",
            CodeJG => "Health & Beauty Department",
            AlcoholBeverageDepartment => "Alcohol Beverage Department",
            MeatDepartment => "Meat Department",
            ProduceDepartment => "Produce Department",
            BakeryDepartment => "Bakery Department",
            VideoDepartment => "Video Department",
            CandyAndConfectionsDepartment => "Candy and Confections Department",
            CigarettesAndTobaccoDepartment => "Cigarettes and Tobacco Department",
            InStoreBakeryDepartment => "In-Store Bakery Department",
            FloralDepartment => "Floral Department",
            PharmacyDepartment => "Pharmacy Department",
            Bidder => "Bidder",
            JointDebtorAttorney => "Joint Debtor Attorney",
            JointDebtor => "Joint Debtor",
            Jurisdiction => "Jurisdiction",
            JointOwner => "Joint Owner",
            JointVenture => "Joint Venture",
            ClosingAgent => "Closing Agent",
            FinancialPlanner => "Financial Planner",
            ManagingGeneralAgent => "Managing General Agent",
            ContractorCognizantSecurityOffice => "Contractor Cognizant Security Office",
            SubcontractorCognizantSecurityOffice => {
                "Subcontractor Cognizant Security Office"
            }
            PlaceOfPerformanceCognizantSecurityOffice => {
                "Place of Performance Cognizant Security Office"
            }
            PartyAuthorizingReleaseOfSecurity => {
                "Party Authorizing Release of Security Information"
            }
            PartyToReceiveContractSecurityClassificationSpecification => {
                "Party To Receive Contract Security Classification Specification"
            }
            PolicyWritingAgent => "Policy Writing Agent",
            RadioStation => "Radio Station",
            FilingLocation => "Filing Location",
            PreviousDistributor => "Previous Distributor",
            ItemManager => "Item Manager",
            CustomerForWhomSameOrSimilarWorkWasPerformed => {
                "Customer for Whom Same or Similar Work Was Performed"
            }
            PartyThatReceivedDisclosureStatement => {
                "Party That Received Disclosure Statement"
            }
            Proposer => "Proposer",
            ContactOffice => "Contact Office",
            AuditOffice => "Audit Office",
            ProjectManager => "Project Manager",
            OrganizationHavingSourceControl => "Organization Having Source Control",
            UnitedStatesOverseasSecurityAdministrationOffice => {
                "United States Overseas Security Administration Office"
            }
            QualifyingOfficer => "Qualifying Officer",
            RegisteringParty => "Registering Party",
            ClerkOfCourt => "Clerk of Court",
            Coordinator => "Coordinator",
            FormerAddress => "Former Address",
            PlantClearanceOfficer => "Plant Clearance Officer",
            NameUnderWhichFiled => "Name Under Which Filed",
            Licensee => "Licensee",
            PreKindergartenToGrade12Recipient => "Pre-kindergarten to Grade 12 Recipient",
            PreKindergartenToGrade12Sender => "Pre-kindergarten to Grade 12 Sender",
            Court => "Court",
            ReceiverSite => "Receiver Site",
            DisbursingOfficer => "Disbursing Officer",
            BidOpeningLocation => "Bid Opening Location",
            FreeOnBoardPoint => "Free on Board Point",
            TechnicalOffice => "Technical Office",
            AcceptanceLocation => "Acceptance Location",
            InspectionLocation => "Inspection Location",
            LocationOfPrincipalAssets => "Location of Principal Assets",
            LoanCorrespondent => "Loan Correspondent",
            Contact => "Contact",
            HeadOffice => "Head Office",
            InformationProvider => "Information Provider",
            Attorney => "Attorney",
            LastBreakTerminal => "Last Break Terminal",
            LocationOfSpotForStorage => "Location of Spot for Storage",
            GasNominationLocation => "Gas Nomination Location",
            LiabilityHolder => "Liability Holder",
            Lessor => "Lessor",
            LimitedPartner => "Limited Partner",
            LocationOfGoods => "Location of Goods",
            LocalGovernmentSponsor => "Local Government Sponsor",
            Pipeline => "Pipeline",
            IndependentLab => "Independent Lab",
            LimitedLiabilityCompany => "Limited Liability Company",
            JuvenileOwner => "Juvenile Owner",
            CodeLL => "Location of Load Exchange (Export)",
            LendingInstitution => "Lending Institution",
            Lender => "Lender",
            LoanOriginator => "Loan Originator",
            LoadingParty => "Loading Party",
            LawFirm => "Law Firm",
            LegalRepresentative => "Legal Representative",
            Lessee => "Lessee",
            LongTermDisabilityCarrier => "Long-term Disability Carrier",
            MasterAgent => "Master Agent",
            LoanServicer => "Loan Servicer",
            Customer => "Customer",
            Labeler => "Labeler",
            AmendedName => "Amended Name",
            Stockholder => "Stockholder",
            ManagingAgent => "Managing Agent",
            Organizer => "Organizer",
            LocalChain => "Local Chain",
            SourceMeterLocation => "Source Meter Location",
            ReceiptLocation => "Receipt Location",
            UpstreamMeterLocation => "Upstream Meter Location",
            DownstreamMeterLocation => "Downstream Meter Location",
            MigrantHealthClinic => "Migrant Health Clinic",
            Landlord => "Landlord",
            ForeclosingLender => "Foreclosing Lender",
            EducationalInstitution => "Educational Institution",
            Manufacturing => "Manufacturing",
            PartyForWhomItemIsUltimatelyIntended => {
                "Party for whom Item is Ultimately Intended"
            }
            CompanyInterviewerWorksFor => "Company Interviewer Works For",
            MotorCarrier => "Motor Carrier",
            VeteransAdministrationLoanGuarantyAuthority => {
                "Veterans Administration Loan Guaranty Authority"
            }
            VeteransAdministrationLoanAuthorizedSupplier => {
                "Veterans Administration Loan Authorized Supplier"
            }
            ManufacturerOfGoods => "Manufacturer of Goods",
            GovernmentLoanAgencySponsorOrAgent => {
                "Government Loan Agency Sponsor or Agent"
            }
            MortgageInsurer => "Mortgage Insurer",
            PlanningScheduleMaterialReleaseIssuer => {
                "Planning Schedule/Material Release Issuer"
            }
            FinancialInstitution => "Financial Institution",
            LoanHolderForRealEstateAsset => "Loan Holder for Real Estate Asset",
            ConsumerCreditAccountCompany => "Consumer Credit Account Company",
            MortgageCompany => "Mortgage Company",
            AuthorizedMarketer => "Authorized Marketer",
            ReleaseDrayman => "Release Drayman",
            ManufacturingPlant => "Manufacturing Plant",
            DeliveryLocation => "Delivery Location",
            MedicalInsuranceCarrier => "Medical Insurance Carrier",
            CodeMS => {
                "Bureau of Land Management (Minerals Management Service) Property Unit"
            }
            MammographyScreeningCenter => "Mammography Screening Center",
            Material => "Material",
            MeterLocation => "Meter Location",
            MeetingLocation => "Meeting Location",
            Mainline => "Mainline",
            MarineSurveyor => "Marine Surveyor",
            JuvenileWitness => "Juvenile Witness",
            MasterGeneralAgent => "Master General Agent",
            Minister => "Minister",
            NotifyPartyNo1 => "Notify Party no. 1",
            NotifyPartyNo2 => "Notify Party no. 2",
            IneligibleParty => "Ineligible Party",
            PriceAdministration => "Price Administration",
            PartyWhoSignedTheDeliveryReceipt => "Party Who Signed the Delivery Receipt",
            NonemploymentIncomeSource => "Nonemployment Income Source",
            PreviousNeighbor => "Previous Neighbor",
            Relative => "Relative",
            Neighborhood => "Neighborhood",
            Neighbor => "Neighbor",
            CrossTownSwitch => "Cross-Town Switch",
            NameChangedTo => "Name Changed To",
            NextDestination => "Next Destination",
            Newspaper => "Newspaper",
            OwnerAnnuitant => "Owner Annuitant",
            Administrator => "Administrator",
            Association => "Association",
            NonInsured => "Non-insured",
            TrustOrEstate => "Trust or Estate",
            NationalChain => "National Chain",
            NonRailroadEntity => "Non-railroad Entity",
            PhysicianSpecialists => "Physician - Specialists",
            NetworkName => "Network Name",
            NotifyPartyForShippersOrder => "Notify Party for Shipper's Order",
            NotaryPublic => "Notary Public",
            PipelineSegmentBoundary => "Pipeline Segment Boundary",
            GasTransactionStartingPoint => "Gas Transaction Starting Point",
            NonTemporaryStorageFacility => "Non-Temporary Storage Facility",
            MagistrateJudge => "Magistrate Judge",
            FormerlyKnownAs => "Formerly Known As",
            FormerlyDoingBusinessAs => "Formerly Doing Business As",
            MaidenName => "Maiden Name",
            PrimaryOwner => "Primary Owner",
            BirthName => "Birth Name",
            PrimaryPhysician => "Primary Physician",
            OriginatingBank => "Originating Bank",
            OriginatingCompany => "Originating Company",
            ReceivingCompany => "Receiving Company",
            Factor => "Factor",
            MerchantBanker => "Merchant Banker",
            NonRegisteredBusinessName => "Non Registered Business Name",
            RegisteredBusinessName => "Registered Business Name",
            Registrar => "Registrar",
            ElectronicReturnOriginator => "Electronic Return Originator",
            OrderedBy => "Ordered By",
            OriginCarrier => "Origin Carrier",
            DoctorOfOptometry => "Doctor of Optometry",
            BookingOffice => "Booking Office",
            OffsetOperator => "Offset Operator",
            CoOwner => "Co-owner",
            OtherDepartments => "Other Departments",
            OutsideInspectionAgency => "Outside Inspection Agency",
            Officer => "Officer",
            OriginMailFacility => "Origin Mail Facility",
            ProductPositionHolder => "Product Position Holder",
            CodeOO => "Order Of (Shippers Orders) - (Transportation)",
            OperatorOfPropertyOrUnit => "Operator of property or unit",
            OriginDrayman => "Origin Drayman",
            OriginalName => "Original Name",
            CodeOS => {
                "Override Institution; this is not the institution sending the record, but another institution the student previously attended or is currently attending"
            }
            OffSiteHandler => "Off-Site Handler",
            OriginTerminal => "Origin Terminal",
            OutsideProcessor => "Outside Processor",
            OtherUnlistedTypeOfCorporation => "Other Unlisted Type of Corporation",
            OwnerOfVessel => "Owner of Vessel",
            OwnerOfPropertyOrUnit => "Owner of Property or Unit",
            OxygenTherapyFacility => "Oxygen Therapy Facility",
            OwnerOfVehicle => "Owner of Vehicle",
            OutsideTestingAgency => "Outside Testing Agency",
            PatientFacility => "Patient Facility",
            Preparer => "Preparer",
            PrimaryInsuredOrSubscriber => "Primary Insured or Subscriber",
            PrimaryCareProvider => "Primary Care Provider",
            PriorInsuranceCarrier => "Prior Insurance Carrier",
            PlanSponsor => "Plan Sponsor",
            CodeP6 => "Third Party Reviewing Preferred Provider Organization (PPO)",
            CodeP7 => "Third Party Repricing Preferred Provider Organization (PPO)",
            PersonnelOffice => "Personnel Office",
            CodeP9 => "Primary Interexchange Carrier (PIC)",
            PartyToReceiveInspectionReport => "Party to Receive Inspection Report",
            PayingBank => "Paying Bank",
            CodePC => "Party to Receive Cert. of Conformance (C.A.A.)",
            PurchasersDepartmentBuyer => "Purchaser's Department Buyer",
            Payee => "Payee",
            PartyToReceiveFreightBill => "Party to Receive Freight Bill",
            PrimeContractor => "Prime Contractor",
            Printer => "Printer",
            Publisher => "Publisher",
            PrimaryInventoryControlActivity => "Primary Inventory Control Activity",
            PartyToReceiveCorrespondence => "Party to Receive Correspondence",
            PartyToReceiveCopy => "Party to Receive Copy",
            PartyToReceivePurchaseOrder => "Party to Receive Purchase Order",
            LawEnforcementAgency => "Law Enforcement Agency",
            PayerOfLastResort => "Payer of Last Resort",
            PartyToReceivePaperMemoOfInvoice => "Party to receive paper Memo of Invoice",
            PriorMortgageCompany => "Prior Mortgage Company",
            PartyManufacturedFor => "Party Manufactured For",
            ProgramManager => "Program Manager",
            PartyToReceiveShippingNotice => "Party to Receive Shipping Notice",
            PartyToReceiveInvoiceForGoodsOrServices => {
                "Party to Receive Invoice for Goods or Services"
            }
            Property => "Property",
            PastPerformanceContact => "Past Performance Contact",
            PersonForWhoseBenefitPropertyWasSeized => {
                "Person for Whose Benefit Property was Seized"
            }
            PartyToReceiveInvoiceForLeasePayments => {
                "Party to Receive Invoice for Lease Payments"
            }
            Payer => "Payer",
            PreviousOwner => "Previous Owner",
            ProspectService => "Prospect Service",
            PrimaryPayer => "Primary Payer",
            PreviousStation => "Previous Station",
            PartyToReceiveTestReport => "Party to Receive Test Report",
            PartyAtPickupLocation => "Party at Pickup Location",
            PurchasedCompany => "Purchased Company",
            PartyPerformingCertification => "Party performing certification",
            PickupAddress => "Pickup Address",
            PartyPerformingCount => "Party Performing Count",
            PartyToFilePersonalPropertyTax => "Party to File Personal Property Tax",
            PartyToReceiveEquipment => "Party to Receive Equipment",
            ConductorPilot => "Conductor Pilot",
            EngineerPilot => "Engineer Pilot",
            RetailAccount => "Retail Account",
            CooperativeBuyingGroup => "Cooperative Buying Group",
            AdvertisingGroup => "Advertising Group",
            Interpreter => "Interpreter",
            Partner => "Partner",
            BasePeriodEmployer => "Base Period Employer",
            LastEmployer => "Last Employer",
            Pharmacy => "Pharmacy",
            PurchaseServiceProvider => "Purchase Service Provider",
            Patient => "Patient",
            ResponsibleParty => "Responsible Party",
            Policyholder => "Policyholder",
            Passenger => "Passenger",
            Pedestrian => "Pedestrian",
            Physician => "Physician",
            PartyInPossession => "Party in Possession",
            CodeQJ => "Most Recent Employer (Chargeable)",
            ManagedCare => "Managed Care",
            Chiropractor => "Chiropractor",
            DialysisCenters => "Dialysis Centers",
            Dentist => "Dentist",
            DoctorOfOsteopathy => "Doctor of Osteopathy",
            PrincipalBorrower => "Principal Borrower",
            QualityControl => "Quality Control",
            BuyersQualityReviewBoard => "Buyer's Quality Review Board",
            Podiatrist => "Podiatrist",
            Psychiatrist => "Psychiatrist",
            Veterinarian => "Veterinarian",
            GroupPractice => "Group Practice",
            Government => "Government",
            HomeHealthCorporation => "Home Health Corporation",
            MedicalDoctor => "Medical Doctor",
            CoBorrower => "Co-borrower",
            RoyaltyOwner => "Royalty Owner",
            PartyToReceiveScaleTicket => "Party to Receive Scale Ticket",
            ReportingOfficer => "Reporting Officer",
            NextScheduledDestination => "Next Scheduled Destination",
            CodeR4 => "Regulatory (State) District",
            CodeR5 => "Regulatory (State) Entity",
            Requester => "Requester",
            ConsumerReferralContact => "Consumer Referral Contact",
            CreditReportingAgency => "Credit Reporting Agency",
            RequestedLender => "Requested Lender",
            AlternateReturnAddress => "Alternate Return Address",
            ReceivingBank => "Receiving Bank",
            ReceivingLocation => "Receiving Location",
            RecoveryRoom => "Recovery Room",
            DestinationIntermodalRamp => "Destination Intermodal Ramp",
            ReceiverManager => "Receiver Manager",
            Refinery => "Refinery",
            CodeRG => "Responsible Installation, Origin",
            ResponsibleGovernmentAgency => "Responsible Government Agency",
            CodeRH => "Responsible Installation, Destination",
            RemitTo => "Remit To",
            ResidenceOrDomicile => "Residence or Domicile",
            RefineryOperator => "Refinery Operator",
            ReportingLocation => "Reporting Location",
            PartyThatRemitsPayment => "Party that remits payment",
            RepairOrRefurbishLocation => "Repair or Refurbish Location",
            OriginalIntermodalRamp => "Original Intermodal Ramp",
            ReceivingPointForCustomerSamples => "Receiving Point for Customer Samples",
            ResaleCustomer => "Resale Customer",
            Railroad => "Railroad",
            ClassIiRailroad => "Class II Railroad",
            ClassIiiRailroad => "Class III Railroad",
            ReceivingFacilityScheduler => "Receiving Facility Scheduler",
            ReturnedTo => "Returned to",
            ReceivingSubLocation => "Receiving Sub-Location",
            Reservoir => "Reservoir",
            RuralHealthClinic => "Rural Health Clinic",
            ResponsibleExhibitor => "Responsible Exhibitor",
            SpecifiedRepository => "Specified Repository",
            ReceiptZone => "Receipt Zone",
            SoleProprietor => "Sole Proprietor",
            Parent => "Parent",
            Student => "Student",
            CustodialParent => "Custodial Parent",
            SkilledNursingFacility => "Skilled Nursing Facility",
            SecuredParty => "Secured Party",
            AgencyGrantingSecurityClearance => "Agency Granting Security Clearance",
            SecuredPartyCompany => "Secured Party Company",
            SecuredPartyIndividual => "Secured Party Individual",
            Sibling => "Sibling",
            SalvageCarrier => "Salvage Carrier",
            StorageArea => "Storage Area",
            StoreClass => "Store Class",
            SoldToAndShipTo => "Sold To and Ship To",
            SellingParty => "Selling Party",
            SecondaryPayer => "Secondary Payer",
            ShipFrom => "Ship From",
            StoreGroup => "Store Group",
            Shipper => "Shipper",
            ShippingScheduleIssuer => "Shipping Schedule Issuer",
            SecondaryInventoryControlActivity => "Secondary Inventory Control Activity",
            ShipInPlaceLocation => "Ship-in-Place Location",
            ServiceProvider => "Service Provider",
            CodeSK => "Secondary Location Address (SLA)",
            OriginSublocation => "Origin Sublocation",
            PartyToReceiveShippingManifest => "Party to Receive Shipping Manifest",
            Store => "Store",
            CodeSNP => "US Customs & Border Protection Second Notify Party",
            SoldToIfDifferentFromBillTo => "Sold To If Different From Bill To",
            PartyFillingShippersOrder => "Party filling Shipper's Order",
            ServiceBureau => "Service Bureau",
            SamplesToBeReturnedTo => "Samples to be Returned To",
            SteamshipCompany => "Steamship Company",
            ShipTo => "Ship To",
            SwitchingAndTerminalCarrier => "Switching and Terminal Carrier",
            SupplierManufacturer => "Supplier/Manufacturer",
            SupplySource => "Supply Source",
            ServicePerformanceSite => "Service Performance Site",
            SealingCompany => "Sealing Company",
            SchoolBasedServiceProvider => "School-based Service Provider",
            SecondaryTaxpayer => "Secondary Taxpayer",
            Supervisor => "Supervisor",
            OperatorOfTheTransferPoint => "Operator of the Transfer Point",
            OperatorOfTheSourceTransferPoint => "Operator of the Source Transfer Point",
            TerminalLocation => "Terminal Location",
            TransferPoint => "Transfer Point",
            TerminalOperator => "Terminal Operator",
            PreviousTitleCompany => "Previous Title Company",
            PriorTitleEvidenceHolder => "Prior Title Evidence Holder",
            TitleInsuranceServicesProvider => "Title Insurance Services Provider",
            Tooling => "Tooling",
            ToolSource => "Tool Source",
            ToolingDesign => "Tooling Design",
            Theatre => "Theatre",
            TaxExemptCorporation => "Tax Exempt Corporation",
            TankFarm => "Tank Farm",
            ToolingFabrication => "Tooling Fabrication",
            TheaterCircuit => "Theater Circuit",
            TariffIssuer => "Tariff Issuer",
            Cosigner => "Cosigner",
            TestSponsor => "Test Sponsor",
            TestingLaboratory => "Testing Laboratory",
            Transmitter => "Transmitter",
            Tradename => "Tradename",
            MessageTo => "Message To",
            TowingAgency => "Towing Agency",
            PrimaryTaxpayer => "Primary Taxpayer",
            ThirdPartyMarketer => "Third Party Marketer",
            CodeTQ => "Third Party Reviewing Organization (TPO)",
            Terminal => "Terminal",
            PartyToReceiveCertifiedTestResults => {
                "Party to Receive Certified Test Results"
            }
            CodeTSD => "Treatment, Storage or Disposal Facility",
            ConsigneeCourierTransferStation => "Consignee Courier Transfer Station",
            ConsignorCourierTransferStation => "Consignor Courier Transfer Station",
            TransferTo => "Transfer To",
            TertiaryPayer => "Tertiary Payer",
            CodeTU => "Third Party Repricing Organization (TPO)",
            CodeTV => "Third Party Administrator (TPA)",
            TransitAuthority => "Transit Authority",
            TaxAuthority => "Tax Authority",
            Trustee => "Trustee",
            SignificantOther => "Significant Other",
            GasTransactionPoint1 => "Gas Transaction Point 1",
            GasTransactionPoint2 => "Gas Transaction Point 2",
            ServicingAgent => "Servicing Agent",
            Team => "Team",
            Underwriter => "Underwriter",
            TitleUnderwriter => "Title Underwriter",
            Psychologist => "Psychologist",
            Reference => "Reference",
            NonRegisteredInvestmentAdvisor => "Non-Registered Investment Advisor",
            PlaceOfBottling => "Place of Bottling",
            PlaceOfDistilling => "Place of Distilling",
            UltimateConsignee => "Ultimate Consignee",
            Region => "Region",
            TestingService => "Testing Service",
            HealthMiscellaneous => "Health Miscellaneous",
            NursingHomeChain => "Nursing Home Chain",
            NursingHome => "Nursing Home",
            RegisteredInvestmentAdvisor => "Registered Investment Advisor",
            SalesAssistant => "Sales Assistant",
            System => "System",
            SpecialAccount => "Special Account",
            CodeUM => "Current Employer (Primary)",
            Union => "Union",
            CodeUO => "Current Employer (Secondary)",
            UnloadingParty => "Unloading Party",
            SubsequentOwner => "Subsequent Owner",
            Surgeon => "Surgeon",
            UpstreamParty => "Upstream Party",
            USTrustee => "U.S. Trustee",
            AnnuitantPayor => "Annuitant Payor",
            UnassignedAgent => "Unassigned Agent",
            BaseJurisdiction => "Base Jurisdiction",
            Vehicle => "Vehicle",
            Signer => "Signer",
            Surety => "Surety",
            Grantor => "Grantor",
            WellPadConstructionContractor => "Well Pad Construction Contractor",
            OilAndGasRegulatoryAgency => "Oil and Gas Regulatory Agency",
            SurfaceDischargeAgency => "Surface Discharge Agency",
            WellCasingDepthAuthority => "Well Casing Depth Authority",
            MarketTimer => "Market Timer",
            OwnerAnnuitantPayor => "Owner Annuitant Payor",
            SecondContact => "Second Contact",
            Candidate => "Candidate",
            VehicleCustodian => "Vehicle Custodian",
            MultipleListingService => "Multiple Listing Service",
            BoardOfRealtors => "Board of Realtors",
            PartyPerformingVerification => "Party Performing Verification",
            SellingOffice => "Selling Office",
            ListingAgent => "Listing Agent",
            ShowingAgent => "Showing Agent",
            ContactPerson => "Contact Person",
            Victim => "Victim",
            OwnerJointAnnuitantPayor => "Owner Joint Annuitant Payor",
            PropertyOrBuildingManager => "Property or Building Manager",
            BuilderName => "Builder Name",
            Occupant => "Occupant",
            Vendor => "Vendor",
            ElementarySchool => "Elementary School",
            PartyWithPowerToVoteSecurities => "Party with Power to Vote Securities",
            MiddleSchool => "Middle School",
            JuniorHighSchool => "Junior High School",
            VehicleSalvageAssignment => "Vehicle Salvage Assignment",
            ListingOffice => "Listing Office",
            SecondContactOrganization => "Second Contact Organization",
            OwnerPayor => "Owner Payor",
            Winner => "Winner",
            ProductionManager => "Production Manager",
            OrganizationCompletingConfigurationChange => {
                "Organization Completing Configuration Change"
            }
            WorkTeam => "Work Team",
            SupplierWorkTeam => "Supplier Work Team",
            ThirdPartyInvestmentAdvisor => "Third Party Investment Advisor",
            Trust => "Trust",
            InterlineServiceCommitmentCustomer => "Interline Service Commitment Customer",
            SamplingLocation => "Sampling Location",
            WritingAgent => "Writing Agent",
            AppraiserName => "Appraiser Name",
            ComparableProperty => "Comparable Property",
            StorageFacilityAtDestination => "Storage Facility at Destination",
            SubjectProperty => "Subject Property",
            TankFarmOwner => "Tank Farm Owner",
            WageEarner => "Wage Earner",
            Warehouse => "Warehouse",
            Witness => "Witness",
            SupervisoryAppraiserName => "Supervisory Appraiser Name",
            Wholesaler => "Wholesaler",
            CompanyAssignedWell => "Company Assigned Well",
            StorageFacilityAtOrigin => "Storage Facility at Origin",
            WitnessForPlaintiff => "Witness for Plaintiff",
            WithdrawalPoint => "Withdrawal Point",
            WaterSystem => "Water System",
            WitnessForDefendant => "Witness for Defendant",
            PrimarySupportOrganization => "Primary Support Organization",
            PreliminaryMaintenancePeriodDesignatingOrganization => {
                "Preliminary Maintenance Period Designating Organization"
            }
            PreliminaryMaintenanceOrganization => "Preliminary Maintenance Organization",
            PreliminaryReferredToOrganization => "Preliminary Referred To Organization",
            FinalMaintenancePeriodDesignatingOrganization => {
                "Final Maintenance Period Designating Organization"
            }
            FinalMaintenanceOrganization => "Final Maintenance Organization",
            MailTo => "Mail to",
            PartyToPerformPackaging => "Party to Perform Packaging",
            UtilizationManagementOrganization => "Utilization Management Organization",
            Spouse => "Spouse",
            DurableMedicalEquipmentSupplier => "Durable Medical Equipment Supplier",
            InternationalOrganization => "International Organization",
            Inventor => "Inventor",
            HispanicServiceInstitute => "Hispanic Service Institute",
            Creditor => "Creditor",
            DebtorsAttorney => "Debtor's Attorney",
            Alias => "Alias",
            ClaimRecipient => "Claim Recipient",
            Auctioneer => "Auctioneer",
            EventLocation => "Event Location",
            FinalReferredToOrganization => "Final Referred To Organization",
            OriginalClaimant => "Original Claimant",
            ActualReferredByOrganization => "Actual Referred By Organization",
            ActualReferredToOrganization => "Actual Referred To Organization",
            BorrowersEmployer => "Borrower's Employer",
            MaintenanceOrganizationUsedForEstimate => {
                "Maintenance Organization Used for Estimate"
            }
            PlanningMaintenanceOrganization => "Planning/Maintenance Organization",
            PreliminaryCustomerOrganization => "Preliminary Customer Organization",
            PartyToReceiveSolicitation => "Party to Receive Solicitation",
            CanadianCustomsBroker => "Canadian Customs Broker",
            MexicanCustomsBroker => "Mexican Customs Broker",
            SCorporation => "S Corporation",
            FinalCustomerOrganization => "Final Customer Organization",
            UnitedStatesCustomsBroker => "United States Customs Broker",
            CrossClaimant => "Cross Claimant",
            CounterClaimant => "Counter Claimant",
            BusinessArea => "Business Area",
            TribalGovernment => "Tribal Government",
            AmericanIndianOwnedBusiness => "American Indian-Owned Business",
            ManagedCareOrganization => "Managed Care Organization",
            Affiant => "Affiant",
            Arbitrator => "Arbitrator",
            BailPayor => "Bail Payor",
            DistrictJustice => "District Justice",
            ThirdParty => "Third Party",
            WitnessForProsecution => "Witness for Prosecution",
            ExpertWitness => "Expert Witness",
            CrimeVictim => "Crime Victim",
            JuvenileVictim => "Juvenile Victim",
            JuvenileDefendant => "Juvenile Defendant",
            Bondsman => "Bondsman",
            CourtAppointedAttorney => "Court Appointed Attorney",
            ComplainantsAttorney => "Complainant's Attorney",
            DistrictAttorney => "District Attorney",
            CodeYO => "Attorney for Defendant, Public",
            ProBonoAttorney => "Pro Bono Attorney",
            ProSeCounsel => "Pro Se Counsel",
            PartyToAppearBefore => "Party to Appear Before",
            Appellant => "Appellant",
            Appellee => "Appellee",
            ArrestingOfficer => "Arresting Officer",
            HostileWitness => "Hostile Witness",
            DischargePoint => "Discharge Point",
            FloodCertifier => "Flood Certifier",
            FloodDeterminationProvider => "Flood Determination Provider",
            ElectronicRegistrationUtility => "Electronic Registration Utility",
            PartyToReceiveStatus => "Party to Receive Status",
            UnserviceableMaterialConsignee => "Unserviceable Material Consignee",
            PotentialSourceOfSupply => "Potential Source of Supply",
            OwningInventoryControlPoint => "Owning Inventory Control Point",
            ManagementControlActivity => "Management Control Activity",
            TransferringParty => "Transferring Party",
            MarkForParty => "Mark-for Party",
            LastKnownSourceOfSupply => "Last Known Source of Supply",
            Banker => "Banker",
            CorrectedAddress => "Corrected Address",
            PartyToReceiveCredit => "Party to Receive Credit",
            RentPayor => "Rent Payor",
            PartyToReceiveReports => "Party to Receive Reports",
            EndItemManufacturer => "End Item Manufacturer",
            BreakBulkPoint => "Break Bulk Point",
            PresentAddress => "Present Address",
            Child => "Child",
            Branch => "Branch",
            Reporter => "Reporter",
            PartyPassingTheTransaction => "Party Passing the Transaction",
            LeaseLocation => "Lease Location",
            LosingInventoryManager => "Losing Inventory Manager",
            MinimumRoyaltyPayor => "Minimum Royalty Payor",
            GainingInventoryManager => "Gaining Inventory Manager",
            ScreeningPoint => "Screening Point",
            ValidatingParty => "Validating Party",
            MonitoringParty => "Monitoring Party",
            ParticipatingArea => "Participating Area",
            Formation => "Formation",
            AllowableRecipient => "Allowable Recipient",
            Field => "Field",
            AttorneyOfRecord => "Attorney of Record",
            AmicusCuriae => "Amicus Curiae",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<EntityIdentifierCode> {
        {
            use EntityIdentifierCode::*;
            match description {
                "Alternate Insurer" => Some(AlternateInsurer),
                "Comparable Rentals" => Some(ComparableRentals),
                "Interim Funding Organization" => Some(InterimFundingOrganization),
                "Non-occupant Co-borrower" => Some(NonOccupantCoBorrower),
                "List Owner" => Some(ListOwner),
                "List Mailer" => Some(ListMailer),
                "Primary Electronic Business Contact" => {
                    Some(PrimaryElectronicBusinessContact)
                }
                "State Division" => Some(StateDivision),
                "Alternate Electronic Business Contact" => {
                    Some(AlternateElectronicBusinessContact)
                }
                "Primary Practice Location" => Some(PrimaryPracticeLocation),
                "Party to Declare Goods" => Some(PartyToDeclareGoods),
                "Loan Applicant" => Some(LoanApplicant),
                "Pumper" => Some(Pumper),
                "Subgroup" => Some(Subgroup),
                "Applicant" => Some(Applicant),
                "Group Purchasing Organization (GPO)" => Some(Code1C),
                "Co-operative" => Some(CoOperative),
                "Health Maintenance Organization (HMO)" => Some(Code1E),
                "Alliance" => Some(Alliance),
                "Oncology Center" => Some(OncologyCenter),
                "Kidney Dialysis Unit" => Some(KidneyDialysisUnit),
                "Preferred Provider Organization (PPO)" => Some(Code1I),
                "Connection" => Some(Connection),
                "Franchisor" => Some(Franchisor),
                "Franchisee" => Some(Franchisee),
                "Previous Group" => Some(PreviousGroup),
                "Shareholder" => Some(Shareholder),
                "Acute Care Hospital" => Some(AcuteCareHospital),
                "Provider" => Some(Provider),
                "Military Facility" => Some(MilitaryFacility),
                "University, College or School" => Some(Code1R),
                "Outpatient Surgicenter" => Some(OutpatientSurgicenter),
                "Physician, Clinic or Group Practice" => Some(Code1T),
                "Long Term Care Facility" => Some(LongTermCareFacility),
                "Extended Care Facility" => Some(ExtendedCareFacility),
                "Psychiatric Health Facility" => Some(PsychiatricHealthFacility),
                "Laboratory" => Some(Laboratory),
                "Retail Pharmacy" => Some(RetailPharmacy),
                "Home Health Care" => Some(HomeHealthCare),
                "Loan Broker" => Some(LoanBroker),
                "Surface Management Entity" => Some(SurfaceManagementEntity),
                "Federal, State, County or City Facility" => Some(Code2A),
                "Third-Party Administrator" => Some(ThirdPartyAdministrator),
                "Co-Participant" => Some(CoParticipant),
                "Miscellaneous Health Care Facility" => {
                    Some(MiscellaneousHealthCareFacility)
                }
                "Non-Health Care Miscellaneous Facility" => {
                    Some(NonHealthCareMiscellaneousFacility)
                }
                "State" => Some(State),
                "Assigner" => Some(Assigner),
                "Hospital District or Authority" => Some(HospitalDistrictOrAuthority),
                "Church Operated Facility" => Some(ChurchOperatedFacility),
                "Individual" => Some(Individual),
                "Partnership" => Some(Partnership),
                "Corporation" => Some(Corporation),
                "Air Force Facility" => Some(AirForceFacility),
                "Army Facility" => Some(ArmyFacility),
                "Navy Facility" => Some(NavyFacility),
                "Public Health Service Facility" => Some(PublicHealthServiceFacility),
                "Veterans Administration Facility" => {
                    Some(VeteransAdministrationFacility)
                }
                "Federal Facility" => Some(FederalFacility),
                "Public Health Service Indian Service Facility" => {
                    Some(PublicHealthServiceIndianServiceFacility)
                }
                "Department of Justice Facility" => Some(DepartmentOfJusticeFacility),
                "Other Not-for-profit Facility" => Some(OtherNotForProfitFacility),
                "Individual for-profit Facility" => Some(IndividualForProfitFacility),
                "Partnership for-profit Facility" => Some(PartnershipForProfitFacility),
                "Corporation for-profit Facility" => Some(CorporationForProfitFacility),
                "General Medical and Surgical Facility" => {
                    Some(GeneralMedicalAndSurgicalFacility)
                }
                "Hospital Unit of an Institution (prison hospital, college infirmary, etc.)" => {
                    Some(Code2Z)
                }
                "Dependent" => Some(Dependent),
                "Application Party" => Some(ApplicationParty),
                "Hospital Unit Within an Institution for the Mentally Retarded" => {
                    Some(HospitalUnitWithinAnInstitutionForTheMentallyRetarded)
                }
                "Psychiatric Facility" => Some(PsychiatricFacility),
                "Tuberculosis and Other Respiratory Diseases Facility" => {
                    Some(TuberculosisAndOtherRespiratoryDiseasesFacility)
                }
                "Obstetrics and Gynecology Facility" => {
                    Some(ObstetricsAndGynecologyFacility)
                }
                "Eye, Ear, Nose and Throat Facility" => Some(Code3E),
                "Rehabilitation Facility" => Some(RehabilitationFacility),
                "Orthopedic Facility" => Some(OrthopedicFacility),
                "Chronic Disease Facility" => Some(ChronicDiseaseFacility),
                "Other Specialty Facility" => Some(OtherSpecialtyFacility),
                "Children's General Facility" => Some(ChildrensGeneralFacility),
                "Children's Hospital Unit of an Institution" => {
                    Some(ChildrensHospitalUnitOfAnInstitution)
                }
                "Children's Psychiatric Facility" => Some(ChildrensPsychiatricFacility),
                "Children's Tuberculosis and Other Respiratory Diseases Facility" => {
                    Some(ChildrensTuberculosisAndOtherRespiratoryDiseasesFacility)
                }
                "Children's Eye, Ear, Nose and Throat Facility" => Some(Code3N),
                "Children's Rehabilitation Facility" => {
                    Some(ChildrensRehabilitationFacility)
                }
                "Children's Orthopedic Facility" => Some(ChildrensOrthopedicFacility),
                "Children's Chronic Disease Facility" => {
                    Some(ChildrensChronicDiseaseFacility)
                }
                "Children's Other Specialty Facility" => {
                    Some(ChildrensOtherSpecialtyFacility)
                }
                "Institution for Mental Retardation" => {
                    Some(InstitutionForMentalRetardation)
                }
                "Alcoholism and Other Chemical Dependency Facility" => {
                    Some(AlcoholismAndOtherChemicalDependencyFacility)
                }
                "General Inpatient Care for AIDS/ARC Facility" => {
                    Some(GeneralInpatientCareForAidsArcFacility)
                }
                "AIDS/ARC Unit" => Some(AidsArcUnit),
                "Specialized Outpatient Program for AIDS/ARC" => {
                    Some(SpecializedOutpatientProgramForAidsArc)
                }
                "Alcohol/Drug Abuse or Dependency Inpatient Unit" => {
                    Some(AlcoholDrugAbuseOrDependencyInpatientUnit)
                }
                "Alcohol/Drug Abuse or Dependency Outpatient Services" => {
                    Some(AlcoholDrugAbuseOrDependencyOutpatientServices)
                }
                "Arthritis Treatment Center" => Some(ArthritisTreatmentCenter),
                "Asset Account Holder" => Some(AssetAccountHolder),
                "Site Operator" => Some(SiteOperator),
                "Birthing Room/LDRP Room" => Some(BirthingRoomLdrpRoom),
                "Burn Care Unit" => Some(BurnCareUnit),
                "Cardiac Catherization Laboratory" => {
                    Some(CardiacCatherizationLaboratory)
                }
                "Open-Heart Surgery Facility" => Some(OpenHeartSurgeryFacility),
                "Cardiac Intensive Care Unit" => Some(CardiacIntensiveCareUnit),
                "Angioplasty Facility" => Some(AngioplastyFacility),
                "Chronic Obstructive Pulmonary Disease Service Facility" => {
                    Some(ChronicObstructivePulmonaryDiseaseServiceFacility)
                }
                "Emergency Department" => Some(EmergencyDepartment),
                "Trauma Center (Certified)" => Some(Code4I),
                "Extracorporeal Shock-Wave Lithotripter (ESWL) Unit" => Some(Code4J),
                "Fitness Center" => Some(FitnessCenter),
                "Genetic Counseling/Screening Services" => {
                    Some(GeneticCounselingScreeningServices)
                }
                "Adult Day Care Program Facility" => Some(AdultDayCareProgramFacility),
                "Alzheimer's Diagnostic/Assessment Services" => {
                    Some(AlzheimersDiagnosticAssessmentServices)
                }
                "Comprehensive Geriatric Assessment Facility" => {
                    Some(ComprehensiveGeriatricAssessmentFacility)
                }
                "Emergency Response (Geriatric) Unit" => Some(Code4P),
                "Geriatric Acute Care Unit" => Some(GeriatricAcuteCareUnit),
                "Geriatric Clinics" => Some(GeriatricClinics),
                "Respite Care Facility" => Some(RespiteCareFacility),
                "Senior Membership Program" => Some(SeniorMembershipProgram),
                "Patient Education Unit" => Some(PatientEducationUnit),
                "Community Health Promotion Facility" => {
                    Some(CommunityHealthPromotionFacility)
                }
                "Worksite Health Promotion Facility" => {
                    Some(WorksiteHealthPromotionFacility)
                }
                "Hemodialysis Facility" => Some(HemodialysisFacility),
                "Home Health Services" => Some(HomeHealthServices),
                "Hospice" => Some(Hospice),
                "Tenant" => Some(Tenant),
                "Construction Contractor" => Some(ConstructionContractor),
                "Medical Surgical or Other Intensive Care Unit" => {
                    Some(MedicalSurgicalOrOtherIntensiveCareUnit)
                }
                "Hisopathology Laboratory" => Some(HisopathologyLaboratory),
                "Blood Bank" => Some(BloodBank),
                "Neonatal Intensive Care Unit" => Some(NeonatalIntensiveCareUnit),
                "Obstetrics Unit" => Some(ObstetricsUnit),
                "Occupational Health Services" => Some(OccupationalHealthServices),
                "Organized Outpatient Services" => Some(OrganizedOutpatientServices),
                "Pediatric Acute Inpatient Unit" => Some(PediatricAcuteInpatientUnit),
                "Psychiatric Child/Adolescent Services" => {
                    Some(PsychiatricChildAdolescentServices)
                }
                "Psychiatric Consultation-Liaison Services" => {
                    Some(PsychiatricConsultationLiaisonServices)
                }
                "Psychiatric Education Services" => Some(PsychiatricEducationServices),
                "Psychiatric Emergency Services" => Some(PsychiatricEmergencyServices),
                "Psychiatric Geriatric Services" => Some(PsychiatricGeriatricServices),
                "Psychiatric Inpatient Unit" => Some(PsychiatricInpatientUnit),
                "Psychiatric Outpatient Services" => Some(PsychiatricOutpatientServices),
                "Psychiatric Partial Hospitalization Program" => {
                    Some(PsychiatricPartialHospitalizationProgram)
                }
                "Megavoltage Radiation Therapy Unit" => {
                    Some(MegavoltageRadiationTherapyUnit)
                }
                "Radioactive Implants Unit" => Some(RadioactiveImplantsUnit),
                "Therapeutic Radioisotope Facility" => {
                    Some(TherapeuticRadioisotopeFacility)
                }
                "X-Ray Radiation Therapy Unit" => Some(XRayRadiationTherapyUnit),
                "CT Scanner Unit" => Some(CtScannerUnit),
                "Diagnostic Radioisotope Facility" => {
                    Some(DiagnosticRadioisotopeFacility)
                }
                "Magnetic Resonance Imaging (MRI) Facility" => Some(Code5W),
                "Ultrasound Unit" => Some(UltrasoundUnit),
                "Rehabilitation Inpatient Unit" => Some(RehabilitationInpatientUnit),
                "Rehabilitation Outpatient Services" => {
                    Some(RehabilitationOutpatientServices)
                }
                "Recipient of Civil or Legal Liability Payment" => {
                    Some(RecipientOfCivilOrLegalLiabilityPayment)
                }
                "Drilling Contractor" => Some(DrillingContractor),
                "Reproductive Health Services" => Some(ReproductiveHealthServices),
                "Skilled Nursing or Other Long-Term Care Unit" => {
                    Some(SkilledNursingOrOtherLongTermCareUnit)
                }
                "Single Photon Emission Computerized Tomography (SPECT) Unit" => {
                    Some(Code6C)
                }
                "Organized Social Work Service Facility" => {
                    Some(OrganizedSocialWorkServiceFacility)
                }
                "Outpatient Social Work Services" => Some(OutpatientSocialWorkServices),
                "Emergency Department Social Work Services" => {
                    Some(EmergencyDepartmentSocialWorkServices)
                }
                "Sports Medicine Clinic/Services" => Some(SportsMedicineClinicServices),
                "Hospital Auxiliary Unit" => Some(HospitalAuxiliaryUnit),
                "Patient Representative Services" => Some(PatientRepresentativeServices),
                "Volunteer Services Department" => Some(VolunteerServicesDepartment),
                "Outpatient Surgery Services" => Some(OutpatientSurgeryServices),
                "Organ/Tissue Transplant Unit" => Some(OrganTissueTransplantUnit),
                "Orthopedic Surgery Facility" => Some(OrthopedicSurgeryFacility),
                "Occupational Therapy Services" => Some(OccupationalTherapyServices),
                "Physical Therapy Services" => Some(PhysicalTherapyServices),
                "Recreational Therapy Services" => Some(RecreationalTherapyServices),
                "Respiratory Therapy Services" => Some(RespiratoryTherapyServices),
                "Speech Therapy Services" => Some(SpeechTherapyServices),
                "Women's Health Center/Services" => Some(WomensHealthCenterServices),
                "Health Sciences Library" => Some(HealthSciencesLibrary),
                "Cardiac Rehabilitation Program Facility" => {
                    Some(CardiacRehabilitationProgramFacility)
                }
                "Non-Invasive Cardiac Assessment Services" => {
                    Some(NonInvasiveCardiacAssessmentServices)
                }
                "Emergency Medical Technician" => Some(EmergencyMedicalTechnician),
                "Disciplinary Contact" => Some(DisciplinaryContact),
                "Case Manager" => Some(CaseManager),
                "Advisor" => Some(Advisor),
                "Titleholder" => Some(Titleholder),
                "Spud Contractor" => Some(SpudContractor),
                "Premises" => Some(Premises),
                "Bottler" => Some(Bottler),
                "Place of Occurrence" => Some(PlaceOfOccurrence),
                "Contracting Officer Representative" => {
                    Some(ContractingOfficerRepresentative)
                }
                "Party Authorized to Definitize Contract Action" => {
                    Some(PartyAuthorizedToDefinitizeContractAction)
                }
                "Filing Address" => Some(FilingAddress),
                "Hazardous Material Office" => Some(HazardousMaterialOffice),
                "Government Furnished Property FOB Point" => {
                    Some(GovernmentFurnishedPropertyFobPoint)
                }
                "Project Name" => Some(ProjectName),
                "Codefendant" => Some(Codefendant),
                "Co-occupant" => Some(CoOccupant),
                "Preliminary Inspection Location" => Some(PreliminaryInspectionLocation),
                "Inspection and Acceptance Location" => {
                    Some(InspectionAndAcceptanceLocation)
                }
                "Party to Receive Proposal" => Some(PartyToReceiveProposal),
                "Federally Chartered Facility" => Some(FederallyCharteredFacility),
                "Transportation Office" => Some(TransportationOffice),
                "Party to Whom Protest Submitted" => Some(PartyToWhomProtestSubmitted),
                "Birthplace" => Some(Birthplace),
                "Pipeline Segment" => Some(PipelineSegment),
                "Home State Name" => Some(HomeStateName),
                "Liquidator" => Some(Liquidator),
                "Petitioning Creditor's Attorney" => Some(PetitioningCreditorsAttorney),
                "Merged Name" => Some(MergedName),
                "Party Represented" => Some(PartyRepresented),
                "Professional Organization" => Some(ProfessionalOrganization),
                "Referee" => Some(Referee),
                "Non-Mortgage Liability Account Holder" => {
                    Some(NonMortgageLiabilityAccountHolder)
                }
                "Lien Holder" => Some(LienHolder),
                "Vacation Home" => Some(VacationHome),
                "Primary Residence" => Some(PrimaryResidence),
                "Second Home" => Some(SecondHome),
                "Permit Holder" => Some(PermitHolder),
                "Minority Institution" => Some(MinorityInstitution),
                "Bailment Warehouse" => Some(BailmentWarehouse),
                "First Appraiser" => Some(FirstAppraiser),
                "Tax Exempt Organization" => Some(TaxExemptOrganization),
                "Service Organization" => Some(ServiceOrganization),
                "Emerging Small Business" => Some(EmergingSmallBusiness),
                "Surplus Dealer" => Some(SurplusDealer),
                "Polling Site" => Some(PollingSite),
                "Socially Disadvantaged Individual" => {
                    Some(SociallyDisadvantagedIndividual)
                }
                "Economically Disadvantaged Individual" => {
                    Some(EconomicallyDisadvantagedIndividual)
                }
                "Disabled Individual" => Some(DisabledIndividual),
                "Producer" => Some(Producer),
                "Public or Private Organization for the Disabled" => {
                    Some(PublicOrPrivateOrganizationForTheDisabled)
                }
                "Consumer Service Provider (CSP) Customer" => Some(Code8R),
                "Consumer Service Provider (CSP)" => Some(Code8S),
                "Voter" => Some(Voter),
                "Native Hawaiian Organization" => Some(NativeHawaiianOrganization),
                "Primary Intra-LATA (Local Access Transport Area) Carrier" => {
                    Some(Code8V)
                }
                "Payment Address" => Some(PaymentAddress),
                "Oil and Gas Custodian" => Some(OilAndGasCustodian),
                "Registered Office" => Some(RegisteredOffice),
                "Note Co-Signer" => Some(NoteCoSigner),
                "Debtor Individual" => Some(DebtorIndividual),
                "Country of Export" => Some(CountryOfExport),
                "Country of Destination" => Some(CountryOfDestination),
                "New Service Provider" => Some(NewServiceProvider),
                "Sub-servicer" => Some(SubServicer),
                "Loss Payee" => Some(LossPayee),
                "Nickname" => Some(Nickname),
                "Assignee" => Some(Assignee),
                "Registered Principal" => Some(RegisteredPrincipal),
                "Additional Debtor" => Some(AdditionalDebtor),
                "Key Person" => Some(KeyPerson),
                "Incorporated By" => Some(IncorporatedBy),
                "Party to Lease" => Some(PartyToLease),
                "Party to Contract" => Some(PartyToContract),
                "Investigator" => Some(Investigator),
                "Last Supplier" => Some(LastSupplier),
                "Downstream First Supplier" => Some(DownstreamFirstSupplier),
                "Co-Investigator" => Some(CoInvestigator),
                "Telephone Answering Service Bureau" => {
                    Some(TelephoneAnsweringServiceBureau)
                }
                "Author" => Some(Author),
                "First Supplier" => Some(FirstSupplier),
                "Ultimate Parent Company" => Some(UltimateParentCompany),
                "Contractual Receipt Meter" => Some(ContractualReceiptMeter),
                "Contractual Delivery Meter" => Some(ContractualDeliveryMeter),
                "Co-debtor" => Some(CoDebtor),
                "Conduit" => Some(Conduit),
                "Party to be billed(AAR Accounting Rule 11)" => Some(Code11),
                "Regional Office" => Some(RegionalOffice),
                "Contracted Service Provider" => Some(ContractedServiceProvider),
                "Wholly-Owned Subsidiary" => Some(WhollyOwnedSubsidiary),
                "Accounts Payable Office" => Some(AccountsPayableOffice),
                "Plant" => Some(Plant),
                "Consultant's Office" => Some(ConsultantsOffice),
                "Production" => Some(Production),
                "Non-Production Supplier" => Some(NonProductionSupplier),
                "Foreign Supplier" => Some(ForeignSupplier),
                "Small Business" => Some(SmallBusiness),
                "Minority-Owned Business, Small" => Some(Code22),
                "Minority-Owned Business, Large" => Some(Code23),
                "Woman-Owned Business, Small" => Some(Code24),
                "Woman-Owned Business, Large" => Some(Code25),
                "Socially Disadvantaged Business" => Some(SociallyDisadvantagedBusiness),
                "Small Disadvantaged Business" => Some(SmallDisadvantagedBusiness),
                "Subcontractor" => Some(Subcontractor),
                "Prototype Supplier" => Some(PrototypeSupplier),
                "Service Supplier" => Some(ServiceSupplier),
                "Postal Mailing Address" => Some(PostalMailingAddress),
                "Party to Receive Material Release" => {
                    Some(PartyToReceiveMaterialRelease)
                }
                "Inquiry Address" => Some(InquiryAddress),
                "Material Change Notice Address" => Some(MaterialChangeNoticeAddress),
                "Electronic Data Interchange (EDI) Coordinator Point Address" => {
                    Some(Code35)
                }
                "Employer" => Some(Employer),
                "Previous Debt Holder" => Some(PreviousDebtHolder),
                "Mortgage Liability Account Holder" => {
                    Some(MortgageLiabilityAccountHolder)
                }
                "Appraisal Company" => Some(AppraisalCompany),
                "Receiver" => Some(Receiver),
                "Submitter" => Some(Submitter),
                "Component Manufacturer" => Some(ComponentManufacturer),
                "Claimant Authorized Representative" => {
                    Some(ClaimantAuthorizedRepresentative)
                }
                "Data Processing Service Bureau" => Some(DataProcessingServiceBureau),
                "Drop-off Location" => Some(DropOffLocation),
                "Invoicing Dealer" => Some(InvoicingDealer),
                "Estimator" => Some(Estimator),
                "In-service Source" => Some(InServiceSource),
                "Initial Dealer" => Some(InitialDealer),
                "Manufacturer's Representative" => Some(ManufacturersRepresentative),
                "Parts Distributor" => Some(PartsDistributor),
                "Part Remanufacturer" => Some(PartRemanufacturer),
                "Registered Owner" => Some(RegisteredOwner),
                "Order Writer" => Some(OrderWriter),
                "Service Manager" => Some(ServiceManager),
                "Servicing Dealer" => Some(ServicingDealer),
                "Servicing Organization" => Some(ServicingOrganization),
                "Store Manager" => Some(StoreManager),
                "Party to Approve Specification" => Some(PartyToApproveSpecification),
                "Salesperson" => Some(Salesperson),
                "Performed At" => Some(PerformedAt),
                "Applicant's Employer" => Some(ApplicantsEmployer),
                "Reference's Employer" => Some(ReferencesEmployer),
                "Cosigner's Employer" => Some(CosignersEmployer),
                "Applicant's Reference" => Some(ApplicantsReference),
                "Applicant's Cosigner" => Some(ApplicantsCosigner),
                "Applicant's Comaker" => Some(ApplicantsComaker),
                "Owner's Representative" => Some(OwnersRepresentative),
                "Repairing Outlet" => Some(RepairingOutlet),
                "Prior Incorrect Insured" => Some(PriorIncorrectInsured),
                "Attending Physician" => Some(AttendingPhysician),
                "Operating Physician" => Some(OperatingPhysician),
                "Other Physician" => Some(OtherPhysician),
                "Corrected Insured" => Some(CorrectedInsured),
                "Participant" => Some(Participant),
                "Secondary Warranter" => Some(SecondaryWarranter),
                "Service Location" => Some(ServiceLocation),
                "Service Requester" => Some(ServiceRequester),
                "Warranter" => Some(Warranter),
                "Hospital" => Some(Hospital),
                "Part Source" => Some(PartSource),
                "Rendering Provider" => Some(RenderingProvider),
                "Subscriber's School" => Some(SubscribersSchool),
                "Subscriber's Employer" => Some(SubscribersEmployer),
                "Billing Provider" => Some(BillingProvider),
                "Conductor" => Some(Conductor),
                "Pay-to Provider" => Some(PayToProvider),
                "Approver" => Some(Approver),
                "Investor" => Some(Investor),
                "Previous Business Partner" => Some(PreviousBusinessPartner),
                "Action Party" => Some(ActionParty),
                "Support Party" => Some(SupportParty),
                "Insurance Institute" => Some(InsuranceInstitute),
                "New Supply Source" => Some(NewSupplySource),
                "Research Institute" => Some(ResearchInstitute),
                "Debtor Company" => Some(DebtorCompany),
                "Party Waiving Requirements" => Some(PartyWaivingRequirements),
                "Freight Management Facilitator" => Some(FreightManagementFacilitator),
                "Outer Continental Shelf (OCS) Area Location" => Some(Code99),
                "Adjuster" => Some(Adjuster),
                "Woman-Owned Business" => Some(WomanOwnedBusiness),
                "Labor Surplus Area Firm" => Some(LaborSurplusAreaFirm),
                "Other Disadvantaged Business" => Some(OtherDisadvantagedBusiness),
                "Veteran-Owned Business" => Some(VeteranOwnedBusiness),
                "Section 8(a) Program Participant Firm" => Some(CodeA6),
                "Sheltered Workshop" => Some(ShelteredWorkshop),
                "Nonprofit Institution" => Some(NonprofitInstitution),
                "Sales Office" => Some(SalesOffice),
                "Authority For Shipment" => Some(AuthorityForShipment),
                "Chief Executive Officer (CEO)" => Some(CodeAA1),
                "Financial Aid Office" => Some(FinancialAidOffice),
                "Respondent" => Some(Respondent),
                "Admission Office" => Some(AdmissionOffice),
                "Multi-Campus Administrative Unit" => Some(MultiCampusAdministrativeUnit),
                "Headmaster" => Some(Headmaster),
                "Business Officer" => Some(BusinessOfficer),
                "Superintendent" => Some(Superintendent),
                "School Principal" => Some(SchoolPrincipal),
                "Sub-account" => Some(SubAccount),
                "Management Non-Officer" => Some(ManagementNonOfficer),
                "Incorporated Location" => Some(IncorporatedLocation),
                "Name not to be Confused with" => Some(NameNotToBeConfusedWith),
                "Lot" => Some(Lot),
                "Previous Occupant" => Some(PreviousOccupant),
                "Ground Ambulance Services" => Some(GroundAmbulanceServices),
                "Air Ambulance Services" => Some(AirAmbulanceServices),
                "Water Ambulance Services" => Some(WaterAmbulanceServices),
                "Admitting Services" => Some(AdmittingServices),
                "Primary Surgeon" => Some(PrimarySurgeon),
                "Medical Nurse" => Some(MedicalNurse),
                "Cardiac Rehabilitation Services" => Some(CardiacRehabilitationServices),
                "Skilled Nursing Services" => Some(SkilledNursingServices),
                "Observation Room Services" => Some(ObservationRoomServices),
                "Employee" => Some(Employee),
                "Anesthesiology Services" => Some(AnesthesiologyServices),
                "Prior Base Jurisdiction" => Some(PriorBaseJurisdiction),
                "Incorporation Jurisdiction" => Some(IncorporationJurisdiction),
                "Marker Owner" => Some(MarkerOwner),
                "Reclamation Center" => Some(ReclamationCenter),
                "Party Providing Financing" => Some(PartyProvidingFinancing),
                "Additional Pickup Address" => Some(AdditionalPickupAddress),
                "Private School System" => Some(PrivateSchoolSystem),
                "State Operated School System" => Some(StateOperatedSchoolSystem),
                "Vocational Regions School System" => Some(VocationalRegionsSchoolSystem),
                "Chartered School District" => Some(CharteredSchoolDistrict),
                "Schooling of Indian Children School System" => {
                    Some(SchoolingOfIndianChildrenSchoolSystem)
                }
                "Unorganized Territories School System" => {
                    Some(UnorganizedTerritoriesSchoolSystem)
                }
                "State Administered District" => Some(StateAdministeredDistrict),
                "Towns in Unions School System" => Some(TownsInUnionsSchoolSystem),
                "Agent Towns School System" => Some(AgentTownsSchoolSystem),
                "Master Property" => Some(MasterProperty),
                "Project Property" => Some(ProjectProperty),
                "Unit Property" => Some(UnitProperty),
                "Additional Address" => Some(AdditionalAddress),
                "Society of Property Information Compilers and Analysts" => {
                    Some(SocietyOfPropertyInformationCompilersAndAnalysts)
                }
                "Organization" => Some(Organization),
                "Joint Owner Annuitant" => Some(JointOwnerAnnuitant),
                "Joint Annuitant Owner" => Some(JointAnnuitantOwner),
                "Joint Owner Annuitant Payor" => Some(JointOwnerAnnuitantPayor),
                "Joint Owner Joint Annuitant" => Some(JointOwnerJointAnnuitant),
                "Joint Owner Joint Annuitant Payor" => {
                    Some(JointOwnerJointAnnuitantPayor)
                }
                "Joint Owner Payor" => Some(JointOwnerPayor),
                "Acronym" => Some(Acronym),
                "New Address" => Some(NewAddress),
                "Chairperson" => Some(Chairperson),
                "Decision Maker" => Some(DecisionMaker),
                "Former President" => Some(FormerPresident),
                "Founder" => Some(Founder),
                "Imported from Location" => Some(ImportedFromLocation),
                "Literally Translated Name" => Some(LiterallyTranslatedName),
                "Original Location" => Some(OriginalLocation),
                "President" => Some(President),
                "Rating Organization" => Some(RatingOrganization),
                "Air Cargo Company" => Some(AirCargoCompany),
                "Regional Center" => Some(RegionalCenter),
                "Local Education Agency (LEA)" => Some(CodeAC2),
                "State Education Agency" => Some(StateEducationAgency),
                "Initial Medical Provider" => Some(InitialMedicalProvider),
                "Concurrent Employer" => Some(ConcurrentEmployer),
                "Routing Point" => Some(RoutingPoint),
                "Border Crossing" => Some(BorderCrossing),
                "Bobtail Service Point" => Some(BobtailServicePoint),
                "Auditor" => Some(Auditor),
                "Insured Location" => Some(InsuredLocation),
                "Referral Provider" => Some(ReferralProvider),
                "Affiliate" => Some(Affiliate),
                "Allied Health Professional" => Some(AlliedHealthProfessional),
                "Emergency Provider" => Some(EmergencyProvider),
                "Federal Government" => Some(FederalGovernment),
                "Fellowship Institution" => Some(FellowshipInstitution),
                "Government - Combined Control" => Some(GovernmentCombinedControl),
                "Government - Federal - Military" => Some(GovernmentFederalMilitary),
                "Government - Federal - Other" => Some(GovernmentFederalOther),
                "Government - Federal - Veterans" => Some(GovernmentFederalVeterans),
                "Government - Local" => Some(GovernmentLocal),
                "Group Affiliation" => Some(GroupAffiliation),
                "Information Source" => Some(InformationSource),
                "Internship Entity" => Some(InternshipEntity),
                "Medical School" => Some(MedicalSchool),
                "National Organization" => Some(NationalOrganization),
                "Non-Profit Health Care Provider\nSee U.S. Internal Revenue Code Chapter 1, Subchapter F, Part 1, Section 501(c)" => {
                    Some(CodeACZ)
                }
                "Party to be advised (Written orders)" => Some(CodeAD),
                "Not for Profit Health Care Provider\nSee U.S. Internal Revenue Code Chapter 1, Subchapter F, Part 1, Section 501(c)" => {
                    Some(CodeADA)
                }
                "For Profit Health Care Provider" => Some(ForProfitHealthCareProvider),
                "Office Manager" => Some(OfficeManager),
                "On-call Provider" => Some(OnCallProvider),
                "Physician Hospital Organization (PHO)" => Some(CodeADE),
                "Point of Service (POS)" => Some(CodeADF),
                "Residency Institution" => Some(ResidencyInstitution),
                "Shared Service" => Some(SharedService),
                "Supporting Personnel" => Some(SupportingPersonnel),
                "Training Institution" => Some(TrainingInstitution),
                "Public School" => Some(PublicSchool),
                "Private School" => Some(PrivateSchool),
                "Public Pre-K Education" => Some(PublicPreKEducation),
                "Private Pre-K Education" => Some(PrivatePreKEducation),
                "Pre-K Day Care" => Some(PreKDayCare),
                "Charter School" => Some(CharterSchool),
                "Home School" => Some(HomeSchool),
                "Public Alternative School" => Some(PublicAlternativeSchool),
                "Neglected/Delinquent Institution" => {
                    Some(NeglectedDelinquentInstitution)
                }
                "Post-Secondary Institution" => Some(PostSecondaryInstitution),
                "Food Service Operator" => Some(FoodServiceOperator),
                "Future Address" => Some(FutureAddress),
                "Former Registered Address" => Some(FormerRegisteredAddress),
                "Top Parent Company in Same Country" => {
                    Some(TopParentCompanyInSameCountry)
                }
                "Additional Delivery Address" => Some(AdditionalDeliveryAddress),
                "Second Level Parent Company" => Some(SecondLevelParentCompany),
                "Airport Authority" => Some(AirportAuthority),
                "Council of Governments" => Some(CouncilOfGovernments),
                "Foundation" => Some(Foundation),
                "Port Authority" => Some(PortAuthority),
                "Planning Commission" => Some(PlanningCommission),
                "Car Rental Location" => Some(CarRentalLocation),
                "Lodging Facility" => Some(LodgingFacility),
                "Party to Receive Transportation Credit" => {
                    Some(PartyToReceiveTransportationCredit)
                }
                "Party to Receive Packing, Crating, and Handling Credit" => Some(CodeAEK),
                "Primary International Telecom Carrier" => {
                    Some(PrimaryInternationalTelecomCarrier)
                }
                "Authorized Accepting Official" => Some(AuthorizedAcceptingOfficial),
                "Agent/Agency" => Some(AgentAgency),
                "Advertiser" => Some(Advertiser),
                "Agency Hazardous Material Information System Location" => {
                    Some(AgencyHazardousMaterialInformationSystemLocation)
                }
                "Airline" => Some(Airline),
                "Alleged Debtor" => Some(AllegedDebtor),
                "Party to Whom Acknowledgment Should Be Sent" => {
                    Some(PartyToWhomAcknowledgmentShouldBeSent)
                }
                "Allotment Customer" => Some(AllotmentCustomer),
                "Alternative Addressee" => Some(AlternativeAddressee),
                "Activity Location" => Some(ActivityLocation),
                "Assistant U.S. Trustee" => Some(AssistantUSTrustee),
                "Authorized From" => Some(AuthorizedFrom),
                "Account Of" => Some(AccountOf),
                "Account of (Origin Party)" => Some(CodeAP),
                "Activity Provider" => Some(ActivityProvider),
                "Account of (Destination Party)" => Some(CodeAQ),
                "Armed Services Location Designation" => {
                    Some(ArmedServicesLocationDesignation)
                }
                "Postsecondary Education Sender" => Some(PostsecondaryEducationSender),
                "Postsecondary Education Recipient" => {
                    Some(PostsecondaryEducationRecipient)
                }
                "Alternate Tax Authority" => Some(AlternateTaxAuthority),
                "Party Authorizing Disposition" => Some(PartyAuthorizingDisposition),
                "Authorizing Official" => Some(AuthorizingOfficial),
                "Authorized To" => Some(AuthorizedTo),
                "Accountant" => Some(Accountant),
                "Plaintiff" => Some(Plaintiff),
                "Clearinghouse" => Some(Clearinghouse),
                "Previous Name" => Some(PreviousName),
                "Construction Firm" => Some(ConstructionFirm),
                "Other Unlisted Type of Organizational Entity" => {
                    Some(OtherUnlistedTypeOfOrganizationalEntity)
                }
                "Previous Name of Firm" => Some(PreviousNameOfFirm),
                "Parent Company" => Some(ParentCompany),
                "Affiliated Company" => Some(AffiliatedCompany),
                "Registering Parent Party" => Some(RegisteringParentParty),
                "Registering Nonparent Party" => Some(RegisteringNonparentParty),
                "Regular Dealer" => Some(RegularDealer),
                "Large Business" => Some(LargeBusiness),
                "Battery" => Some(Battery),
                "Bailiff" => Some(Bailiff),
                "Business Partner" => Some(BusinessPartner),
                "Broadcaster" => Some(Broadcaster),
                "Bill-to Party for Diversion Charges" => {
                    Some(BillToPartyForDiversionCharges)
                }
                "Beneficiary" => Some(Beneficiary),
                "Billed From" => Some(BilledFrom),
                "Buying Group" => Some(BuyingGroup),
                "Interim Trustee" => Some(InterimTrustee),
                "Trustee's Attorney" => Some(TrusteesAttorney),
                "Co-Counsel" => Some(CoCounsel),
                "Bank" => Some(Bank),
                "Bookkeeper" => Some(Bookkeeper),
                "Party to Receive Bill of Lading" => Some(PartyToReceiveBillOfLading),
                "Building" => Some(Building),
                "Structure" => Some(Structure),
                "Brakeman" => Some(Brakeman),
                "Beneficial Owner" => Some(BeneficialOwner),
                "Broker or Sales Office" => Some(BrokerOrSalesOffice),
                "Body of Water" => Some(BodyOfWater),
                "Special Counsel" => Some(SpecialCounsel),
                "Attorney for Defendant Private" => Some(AttorneyForDefendantPrivate),
                "Broker" => Some(Broker),
                "Brand Name" => Some(BrandName),
                "Bill and Ship To" => Some(BillAndShipTo),
                "Bill-to-Party" => Some(BillToParty),
                "Place of Business" => Some(PlaceOfBusiness),
                "Business" => Some(Business),
                "Billing Service" => Some(BillingService),
                "Borrower" => Some(Borrower),
                "Attorney for Plaintiff" => Some(AttorneyForPlaintiff),
                "Buying Party (Purchaser)" => Some(CodeBY),
                "Business Associate" => Some(BusinessAssociate),
                "Assistant Conductor" => Some(AssistantConductor),
                "In Care Of Party no. 1" => Some(InCareOfPartyNo1),
                "In Care Of Party no. 2" => Some(InCareOfPartyNo2),
                "Circuit Location Identifier" => Some(CircuitLocation),
                "Contract Administration Office" => Some(ContractAdministrationOffice),
                "Secondary Contract Administration Office" => {
                    Some(SecondaryContractAdministrationOffice)
                }
                "Party Submitting Quote" => Some(PartySubmittingQuote),
                "Municipality" => Some(Municipality),
                "County" => Some(County),
                "City" => Some(City),
                "Contract Holder" => Some(ContractHolder),
                "Carrier" => Some(Carrier),
                "Customs Broker" => Some(CustomsBroker),
                "Claimant" => Some(Claimant),
                "Consignee (To Receive Mail and Small Parcels)" => Some(CodeCD),
                "Consignee (To receive large parcels and freight)" => Some(CodeCE),
                "Subsidiary/Division" => Some(SubsidiaryDivision),
                "Carnet Issuer" => Some(CarnetIssuer),
                "Chassis Provider" => Some(ChassisProvider),
                "Changed Address" => Some(ChangedAddress),
                "Consignor" => Some(Consignor),
                "Automated Data Processing (ADP) Point" => Some(CodeCJ),
                "Pharmacist" => Some(Pharmacist),
                "Container Location" => Some(ContainerLocation),
                "Building Cluster" => Some(BuildingCluster),
                "Customs" => Some(Customs),
                "Company Merged With" => Some(CompanyMergedWith),
                "Consignee" => Some(Consignee),
                "Confirming Party" => Some(ConfirmingParty),
                "Confirmation Requester" => Some(ConfirmationRequester),
                "Confirmation Service Identifier Code" => {
                    Some(ConfirmationServiceIdentifierCode)
                }
                "Ocean Tariff Conference" => Some(OceanTariffConference),
                "Co-Driver" => Some(CoDriver),
                "Collateral Assignee" => Some(CollateralAssignee),
                "Complainant" => Some(Complainant),
                "Corrected Name" => Some(CorrectedName),
                "Party to Receive Cert. of Compliance" => {
                    Some(PartyToReceiveCertOfCompliance)
                }
                "Corporate Office" => Some(CorporateOffice),
                "Container Return Company" => Some(ContainerReturnCompany),
                "Crew Member" => Some(CrewMember),
                "Consolidator" => Some(Consolidator),
                "Country of Origin" => Some(CountryOfOrigin),
                "Coating or Paint Supplier" => Some(CoatingOrPaintSupplier),
                "Converter" => Some(Converter),
                "Accounting Station" => Some(AccountingStation),
                "Claim Administrator" => Some(ClaimAdministrator),
                "Country" => Some(Country),
                "Admitting Surgeon" => Some(AdmittingSurgeon),
                "Driver" => Some(Driver),
                "Commercial Insurer" => Some(CommercialInsurer),
                "Defendant" => Some(Defendant),
                "Debtor" => Some(Debtor),
                "Debtor-In-Possession" => Some(DebtorInPossession),
                "Consolidated Debtor" => Some(ConsolidatedDebtor),
                "Petitioning Creditor" => Some(PetitioningCreditor),
                "Dispatcher" => Some(Dispatcher),
                "Creditor's Attorney" => Some(CreditorsAttorney),
                "Delivery Address" => Some(DeliveryAddress),
                "Damaged By" => Some(DamagedBy),
                "Distributor Branch" => Some(DistributorBranch),
                "Destination Carrier" => Some(DestinationCarrier),
                "Chief Deputy Clerk of Court" => Some(ChiefDeputyClerkOfCourt),
                "Assistant Surgeon" => Some(AssistantSurgeon),
                "Depositor" => Some(Depositor),
                "Material Disposition Authorization Location" => {
                    Some(MaterialDispositionAuthorizationLocation)
                }
                "Design Engineering" => Some(DesignEngineering),
                "Doing Business As" => Some(DoingBusinessAs),
                "Different Premise Address (DPA)" => Some(CodeDI),
                "Distribution Recipient" => Some(DistributionRecipient),
                "Consulting Physician" => Some(ConsultingPhysician),
                "Ordering Physician" => Some(OrderingPhysician),
                "Dealer" => Some(Dealer),
                "Destination Mail Facility" => Some(DestinationMailFacility),
                "Referring Provider" => Some(ReferringProvider),
                "Dependent Name" => Some(DependentName),
                "Party to Provide Discount" => Some(PartyToProvideDiscount),
                "Supervising Physician" => Some(SupervisingPhysician),
                "Destination Drayman" => Some(DestinationDrayman),
                "Distributor" => Some(Distributor),
                "Destination Terminal" => Some(DestinationTerminal),
                "Resale Dealer" => Some(ResaleDealer),
                "Division" => Some(Division),
                "Downstream Party" => Some(DownstreamParty),
                "Distiller" => Some(Distiller),
                "Default/Foreclosure Specialist" => Some(DefaultForeclosureSpecialist),
                "Delivery Zone" => Some(DeliveryZone),
                "Assistant Engineer" => Some(AssistantEngineer),
                "Person or Other Entity Legally Responsible for a Child" => {
                    Some(PersonOrOtherEntityLegallyResponsibleForAChild)
                }
                "Person or Other Entity With Whom a Child Resides" => {
                    Some(PersonOrOtherEntityWithWhomAChildResides)
                }
                "Person or Other Entity Legally Responsible for and With Whom a Child Resides" => {
                    Some(
                        PersonOrOtherEntityLegallyResponsibleForAndWithWhomAChildResides,
                    )
                }
                "Other Person or Entity Associated with Student" => {
                    Some(OtherPersonOrEntityAssociatedWithStudent)
                }
                "Examiner" => Some(Examiner),
                "Engineering" => Some(Engineering),
                "Previous Employer" => Some(PreviousEmployer),
                "Inquiring Party" => Some(InquiringParty),
                "Participating Laboratory" => Some(ParticipatingLaboratory),
                "Study Submitter" => Some(StudySubmitter),
                "Assistant" => Some(Assistant),
                "Campaign Manager" => Some(CampaignManager),
                "Client" => Some(Client),
                "Commissioner" => Some(Commissioner),
                "Committee" => Some(Committee),
                "Contestant" => Some(Contestant),
                "Contributor" => Some(Contributor),
                "Deputy Chairperson" => Some(DeputyChairperson),
                "Deputy Treasurer" => Some(DeputyTreasurer),
                "Donor" => Some(Donor),
                "Endorser" => Some(Endorser),
                "Guarantor" => Some(Guarantor),
                "Headquarters" => Some(Headquarters),
                "Independent Contractor" => Some(IndependentContractor),
                "Leader" => Some(Leader),
                "Party Performing Liaison" => Some(PartyPerformingLiaison),
                "Lobbying Firm" => Some(LobbyingFirm),
                "Lobbyist" => Some(Lobbyist),
                "Media Contact" => Some(MediaContact),
                "Office Holder" => Some(OfficeHolder),
                "Party Authorized to Administer Oaths" => {
                    Some(PartyAuthorizedToAdministerOaths)
                }
                "Party to Benefit" => Some(PartyToBenefit),
                "Party Holding Interest" => Some(PartyHoldingInterest),
                "Party Making Pledge" => Some(PartyMakingPledge),
                "Party Returning Contribution" => Some(PartyReturningContribution),
                "Eligible Party To The Contract" => Some(EligiblePartyToTheContract),
                "Party Returning Transfer" => Some(PartyReturningTransfer),
                "Lobbied Party" => Some(LobbiedParty),
                "Political Action Committee" => Some(PoliticalActionCommittee),
                "Political Party" => Some(PoliticalParty),
                "Proponent" => Some(Proponent),
                "Public Official" => Some(PublicOfficial),
                "Receiving Committee" => Some(ReceivingCommittee),
                "Affiliated Committee" => Some(AffiliatedCommittee),
                "Source" => Some(Source),
                "Sponsor" => Some(Sponsor),
                "Sponsored Committee" => Some(SponsoredCommittee),
                "Designee" => Some(Designee),
                "Temporary Residence" => Some(TemporaryResidence),
                "Treasurer" => Some(Treasurer),
                "Vice-Chairperson" => Some(ViceChairperson),
                "Slate Mailer Organization" => Some(SlateMailerOrganization),
                "Lodging Location" => Some(LodgingLocation),
                "Independent Expenditure Committee" => {
                    Some(IndependentExpenditureCommittee)
                }
                "Major Donor" => Some(MajorDonor),
                "Exchanger" => Some(Exchanger),
                "Excluded Party" => Some(ExcludedParty),
                "Location of Goods for Customs Examination Before Clearance" => {
                    Some(LocationOfGoodsForCustomsExaminationBeforeClearance)
                }
                "Electronic Filer" => Some(ElectronicFiler),
                "Engineer" => Some(Engineer),
                "Exhibitor" => Some(Exhibitor),
                "Executor of Estate" => Some(ExecutorOfEstate),
                "Principal Person" => Some(PrincipalPerson),
                "Animal Source" => Some(AnimalSource),
                "Established Location" => Some(EstablishedLocation),
                "Party to Receive Electronic Memo of Invoice" => {
                    Some(PartyToReceiveElectronicMemoOfInvoice)
                }
                "End User" => Some(EndUser),
                "Enroller" => Some(Enroller),
                "Limited Liability Partnership" => Some(LimitedLiabilityPartnership),
                "Eligible Party to the Rate" => Some(EligiblePartyToTheRate),
                "Old Debtor" => Some(OldDebtor),
                "New Debtor" => Some(NewDebtor),
                "Plan Administrator" => Some(PlanAdministrator),
                "Old Secured Party" => Some(OldSecuredParty),
                "Selling Agent" => Some(SellingAgent),
                "Servicing Broker" => Some(ServicingBroker),
                "Exporter" => Some(Exporter),
                "Ex-spouse" => Some(ExSpouse),
                "Employee Name" => Some(EmployeeName),
                "New Secured Party" => Some(NewSecuredParty),
                "Company - Owned Oil Field" => Some(CompanyOwnedOilField),
                "Energy Information Administration (Department of Energy) - Owned Oil Field" => {
                    Some(CodeF2)
                }
                "Specialized Mobile Radio Service (SMRS) Licensee" => Some(CodeF3),
                "Former Residence" => Some(FormerResidence),
                "Radio Control Station Location" => Some(RadioControlStationLocation),
                "Small Control Station Location" => Some(SmallControlStationLocation),
                "Small Base Station Location" => Some(SmallBaseStationLocation),
                "Antenna Site" => Some(AntennaSite),
                "Area of Operation" => Some(AreaOfOperation),
                "Facility" => Some(Facility),
                "First Break Terminal" => Some(FirstBreakTerminal),
                "Customer Identification File (CIF) Customer Identifier" => Some(CodeFC),
                "Physical Address" => Some(PhysicalAddress),
                "Mail Address" => Some(MailAddress),
                "Foreign Language Synonym" => Some(ForeignLanguageSynonym),
                "Trade Name Synonym" => Some(TradeNameSynonym),
                "Foreign Government" => Some(ForeignGovernment),
                "Party to Receive Limitations of Heavy Elements Report" => {
                    Some(PartyToReceiveLimitationsOfHeavyElementsReport)
                }
                "Name Variation Synonym" => Some(NameVariationSynonym),
                "First Contact" => Some(FirstContact),
                "Primary Control Point Location" => Some(PrimaryControlPointLocation),
                "Fireman" => Some(Fireman),
                "Filer Name" => Some(FilerName),
                "Field or Branch Office" => Some(FieldOrBranchOffice),
                "Name on Credit Card" => Some(NameOnCreditCard),
                "Pier Name" => Some(PierName),
                "Message From" => Some(MessageFrom),
                "Foreign Registration Location" => Some(ForeignRegistrationLocation),
                "Final Scheduled Destination" => Some(FinalScheduledDestination),
                "Party to Receive Sensitive Foreign Disclosure Information" => {
                    Some(PartyToReceiveSensitiveForeignDisclosure)
                }
                "Financial Statement Recipient" => Some(FinancialStatementRecipient),
                "New Assignee" => Some(NewAssignee),
                "Old Assignee" => Some(OldAssignee),
                "Vessel Name" => Some(VesselName),
                "Forwarder" => Some(Forwarder),
                "Closed Door Pharmacy" => Some(ClosedDoorPharmacy),
                "Veterinary Hospital" => Some(VeterinaryHospital),
                "Children's Day Care Center" => Some(ChildrensDayCareCenter),
                "Dependent Insured" => Some(DependentInsured),
                "Bankruptcy Trustee" => Some(BankruptcyTrustee),
                "Annuitant" => Some(Annuitant),
                "Clinic" => Some(Clinic),
                "Contingent Beneficiary" => Some(ContingentBeneficiary),
                "Entity Holding the Information" => Some(EntityHoldingThe),
                "Entity Providing the Service" => Some(EntityProvidingTheService),
                "Entity Responsible for Follow-up" => Some(EntityResponsibleForFollowUp),
                "Family Member" => Some(FamilyMember),
                "Gas Plant" => Some(GasPlant),
                "Other Insured" => Some(OtherInsured),
                "Alternate Government Business Contact" => {
                    Some(AlternateGovernmentBusinessContact)
                }
                "Gate Booth" => Some(GateBooth),
                "Primary Government Business Contact" => {
                    Some(PrimaryGovernmentBusinessContact)
                }
                "Previous Credit Grantor" => Some(PreviousCreditGrantor),
                "Guardian" => Some(Guardian),
                "General Agency" => Some(GeneralAgency),
                "Inspection Company" => Some(InspectionCompany),
                "Intermediary" => Some(Intermediary),
                "Motor Vehicle Report Provider Company" => {
                    Some(MotorVehicleReportProviderCompany)
                }
                "Paramedic" => Some(Paramedic),
                "Gift Recipient" => Some(GiftRecipient),
                "Paramedical Company" => Some(ParamedicalCompany),
                "Previous Insured" => Some(PreviousInsured),
                "Previous Residence" => Some(PreviousResidence),
                "Spouse Insured" => Some(SpouseInsured),
                "Garnishee" => Some(Garnishee),
                "Primary Beneficiary" => Some(PrimaryBeneficiary),
                "Gateway Provider" => Some(GatewayProvider),
                "Proposed Insured" => Some(ProposedInsured),
                "Reinsurer" => Some(Reinsurer),
                "Garaged Location" => Some(GaragedLocation),
                "Credit Grantor" => Some(CreditGrantor),
                "Guarantee Agency" => Some(GuaranteeAgency),
                "Gas Transaction Ending Point" => Some(GasTransactionEndingPoint),
                "Group" => Some(Group),
                "Retrocessionaire" => Some(Retrocessionaire),
                "Treatment Facility" => Some(TreatmentFacility),
                "Grandparent" => Some(Grandparent),
                "Representative" => Some(Representative),
                "Sub-Office" => Some(SubOffice),
                "District" => Some(District),
                "Paying Agent" => Some(PayingAgent),
                "School District" => Some(SchoolDistrict),
                "Group Affiliate" => Some(GroupAffiliate),
                "Designer" => Some(Designer),
                "Owner" => Some(Owner),
                "Historically Black College or University" => {
                    Some(HistoricallyBlackCollegeOrUniversity)
                }
                "Joint Annuitant" => Some(JointAnnuitant),
                "Contingent Annuitant" => Some(ContingentAnnuitant),
                "Contingent Owner" => Some(ContingentOwner),
                "Healthcare Professional Shortage Area (HPSA) Facility" => Some(CodeHF),
                "Broker Opinion or Analysis Requester" => {
                    Some(BrokerOpinionOrAnalysisRequester)
                }
                "Home Health Agency" => Some(HomeHealthAgency),
                "Listing Company" => Some(ListingCompany),
                "Automated Underwriting System" => Some(AutomatedUnderwritingSystem),
                "Subscriber" => Some(Subscriber),
                "Document Custodian" => Some(DocumentCustodian),
                "Competitive Property Listing" => Some(CompetitivePropertyListing),
                "Material Safety Data Sheet (MSDS) Recipient" => Some(CodeHMI),
                "Competing Property" => Some(CompetingProperty),
                "Comparable Property Listing" => Some(ComparablePropertyListing),
                "Home Office" => Some(HomeOffice),
                "Honorary Society" => Some(HonorarySociety),
                "Closed Sale" => Some(ClosedSale),
                "Source Party of Information" => Some(SourcePartyOf),
                "Subject of Inquiry" => Some(SubjectOfInquiry),
                "High School" => Some(HighSchool),
                "State Chartered Facility" => Some(StateCharteredFacility),
                "Subsidiary" => Some(Subsidiary),
                "Tax Address" => Some(TaxAddress),
                "Designated Hazardous Waste Facility" => {
                    Some(DesignatedHazardousWasteFacility)
                }
                "Transporter of Hazardous Waste" => Some(TransporterOfHazardousWaste),
                "Charity" => Some(Charity),
                "Hazardous Waste Generator" => Some(HazardousWasteGenerator),
                "Interested Party" => Some(InterestedParty),
                "Independent Physicians Association (IPA)" => Some(CodeI3),
                "Intellectual Property Owner" => Some(IntellectualPropertyOwner),
                "Interviewer" => Some(Interviewer),
                "Installed At" => Some(InstalledAt),
                "Business Entity" => Some(BusinessEntity),
                "Principal Executive Office" => Some(PrincipalExecutiveOffice),
                "Foreign Office" => Some(ForeignOffice),
                "Member" => Some(Member),
                "Executive Committee Member" => Some(ExecutiveCommitteeMember),
                "Director" => Some(Director),
                "Clerk" => Some(Clerk),
                "Party with Knowledge of Affairs of the Company" => {
                    Some(PartyWithKnowledgeOfAffairsOfTheCompany)
                }
                "Party to Receive Statement of Fees Due" => {
                    Some(PartyToReceiveStatementOfFeesDue)
                }
                "Company in which Interest Held" => Some(CompanyInWhichInterestHeld),
                "Company which Holds Interest" => Some(CompanyWhichHoldsInterest),
                "Notary" => Some(Notary),
                "Manager" => Some(Manager),
                "Alien Affiliate" => Some(AlienAffiliate),
                "Incorporation State Principal Office" => {
                    Some(IncorporationStatePrincipalOffice)
                }
                "Incorporation State Place of Business" => {
                    Some(IncorporationStatePlaceOfBusiness)
                }
                "Out-of-State Principal Office" => Some(OutOfStatePrincipalOffice),
                "Party Executing and Verifying" => Some(PartyExecutingAndVerifying),
                "Felon" => Some(Felon),
                "Other Related Party" => Some(OtherRelatedParty),
                "Record-Keeping Address" => Some(RecordKeepingAddress),
                "Initial Subscriber" => Some(InitialSubscriber),
                "Original Jurisdiction" => Some(OriginalJurisdiction),
                "Industry Bureau" => Some(IndustryBureau),
                "Intermediate Consignee" => Some(IntermediateConsignee),
                "Inventory Control Point" => Some(InventoryControlPoint),
                "Issuer of Debit or Credit Memo" => Some(IssuerOfDebitOrCreditMemo),
                "Other Individual Disability Carrier" => {
                    Some(OtherIndividualDisabilityCarrier)
                }
                "International Freight Forwarder" => Some(InternationalFreightForwarder),
                "Insolvent Insurer" => Some(InsolventInsurer),
                "Issuer of Invoice" => Some(IssuerOfInvoice),
                "Injection Point" => Some(InjectionPoint),
                "Intermediate Carrier" => Some(IntermediateCarrier),
                "Insured or Subscriber" => Some(InsuredOrSubscriber),
                "Importer" => Some(Importer),
                "Integrated Material Manager" => Some(IntegratedMaterialManager),
                "Insurer" => Some(Insurer),
                "Interviewee" => Some(Interviewee),
                "Investment Advisor" => Some(InvestmentAdvisor),
                "Inspector" => Some(Inspector),
                "Independent Adjuster" => Some(IndependentAdjuster),
                "In-patient Pharmacy" => Some(InPatientPharmacy),
                "Self Insured" => Some(SelfInsured),
                "Party to Receive Certified Inspection Report" => {
                    Some(PartyToReceiveCertifiedInspectionReport)
                }
                "Installation on Site" => Some(InstallationOnSite),
                "Issuer" => Some(Issuer),
                "Renter" => Some(Renter),
                "Associate General Agent" => Some(AssociateGeneralAgent),
                "Authorized Entity" => Some(AuthorizedEntity),
                "Broker's Assistant" => Some(BrokersAssistant),
                "Custodian" => Some(Custodian),
                "Irrevocable Beneficiary" => Some(IrrevocableBeneficiary),
                "Power of Attorney" => Some(PowerOfAttorney),
                "Trust Officer" => Some(TrustOfficer),
                "Broker Dealer" => Some(BrokerDealer),
                "Community Agent" => Some(CommunityAgent),
                "Dairy Department" => Some(DairyDepartment),
                "Delicatessen Department" => Some(DelicatessenDepartment),
                "Dry Grocery Department" => Some(DryGroceryDepartment),
                "Judge" => Some(Judge),
                "Frozen Department" => Some(FrozenDepartment),
                "General Merchandise Department" => Some(GeneralMerchandiseDepartment),
                "Health & Beauty Department" => Some(CodeJG),
                "Alcohol Beverage Department" => Some(AlcoholBeverageDepartment),
                "Meat Department" => Some(MeatDepartment),
                "Produce Department" => Some(ProduceDepartment),
                "Bakery Department" => Some(BakeryDepartment),
                "Video Department" => Some(VideoDepartment),
                "Candy and Confections Department" => Some(CandyAndConfectionsDepartment),
                "Cigarettes and Tobacco Department" => {
                    Some(CigarettesAndTobaccoDepartment)
                }
                "In-Store Bakery Department" => Some(InStoreBakeryDepartment),
                "Floral Department" => Some(FloralDepartment),
                "Pharmacy Department" => Some(PharmacyDepartment),
                "Bidder" => Some(Bidder),
                "Joint Debtor Attorney" => Some(JointDebtorAttorney),
                "Joint Debtor" => Some(JointDebtor),
                "Jurisdiction" => Some(Jurisdiction),
                "Joint Owner" => Some(JointOwner),
                "Joint Venture" => Some(JointVenture),
                "Closing Agent" => Some(ClosingAgent),
                "Financial Planner" => Some(FinancialPlanner),
                "Managing General Agent" => Some(ManagingGeneralAgent),
                "Contractor Cognizant Security Office" => {
                    Some(ContractorCognizantSecurityOffice)
                }
                "Subcontractor Cognizant Security Office" => {
                    Some(SubcontractorCognizantSecurityOffice)
                }
                "Place of Performance Cognizant Security Office" => {
                    Some(PlaceOfPerformanceCognizantSecurityOffice)
                }
                "Party Authorizing Release of Security Information" => {
                    Some(PartyAuthorizingReleaseOfSecurity)
                }
                "Party To Receive Contract Security Classification Specification" => {
                    Some(PartyToReceiveContractSecurityClassificationSpecification)
                }
                "Policy Writing Agent" => Some(PolicyWritingAgent),
                "Radio Station" => Some(RadioStation),
                "Filing Location" => Some(FilingLocation),
                "Previous Distributor" => Some(PreviousDistributor),
                "Item Manager" => Some(ItemManager),
                "Customer for Whom Same or Similar Work Was Performed" => {
                    Some(CustomerForWhomSameOrSimilarWorkWasPerformed)
                }
                "Party That Received Disclosure Statement" => {
                    Some(PartyThatReceivedDisclosureStatement)
                }
                "Proposer" => Some(Proposer),
                "Contact Office" => Some(ContactOffice),
                "Audit Office" => Some(AuditOffice),
                "Project Manager" => Some(ProjectManager),
                "Organization Having Source Control" => {
                    Some(OrganizationHavingSourceControl)
                }
                "United States Overseas Security Administration Office" => {
                    Some(UnitedStatesOverseasSecurityAdministrationOffice)
                }
                "Qualifying Officer" => Some(QualifyingOfficer),
                "Registering Party" => Some(RegisteringParty),
                "Clerk of Court" => Some(ClerkOfCourt),
                "Coordinator" => Some(Coordinator),
                "Former Address" => Some(FormerAddress),
                "Plant Clearance Officer" => Some(PlantClearanceOfficer),
                "Name Under Which Filed" => Some(NameUnderWhichFiled),
                "Licensee" => Some(Licensee),
                "Pre-kindergarten to Grade 12 Recipient" => {
                    Some(PreKindergartenToGrade12Recipient)
                }
                "Pre-kindergarten to Grade 12 Sender" => {
                    Some(PreKindergartenToGrade12Sender)
                }
                "Court" => Some(Court),
                "Receiver Site" => Some(ReceiverSite),
                "Disbursing Officer" => Some(DisbursingOfficer),
                "Bid Opening Location" => Some(BidOpeningLocation),
                "Free on Board Point" => Some(FreeOnBoardPoint),
                "Technical Office" => Some(TechnicalOffice),
                "Acceptance Location" => Some(AcceptanceLocation),
                "Inspection Location" => Some(InspectionLocation),
                "Location of Principal Assets" => Some(LocationOfPrincipalAssets),
                "Loan Correspondent" => Some(LoanCorrespondent),
                "Contact" => Some(Contact),
                "Head Office" => Some(HeadOffice),
                "Information Provider" => Some(InformationProvider),
                "Attorney" => Some(Attorney),
                "Last Break Terminal" => Some(LastBreakTerminal),
                "Location of Spot for Storage" => Some(LocationOfSpotForStorage),
                "Gas Nomination Location" => Some(GasNominationLocation),
                "Liability Holder" => Some(LiabilityHolder),
                "Lessor" => Some(Lessor),
                "Limited Partner" => Some(LimitedPartner),
                "Location of Goods" => Some(LocationOfGoods),
                "Local Government Sponsor" => Some(LocalGovernmentSponsor),
                "Pipeline" => Some(Pipeline),
                "Independent Lab" => Some(IndependentLab),
                "Limited Liability Company" => Some(LimitedLiabilityCompany),
                "Juvenile Owner" => Some(JuvenileOwner),
                "Location of Load Exchange (Export)" => Some(CodeLL),
                "Lending Institution" => Some(LendingInstitution),
                "Lender" => Some(Lender),
                "Loan Originator" => Some(LoanOriginator),
                "Loading Party" => Some(LoadingParty),
                "Law Firm" => Some(LawFirm),
                "Legal Representative" => Some(LegalRepresentative),
                "Lessee" => Some(Lessee),
                "Long-term Disability Carrier" => Some(LongTermDisabilityCarrier),
                "Master Agent" => Some(MasterAgent),
                "Loan Servicer" => Some(LoanServicer),
                "Customer" => Some(Customer),
                "Labeler" => Some(Labeler),
                "Amended Name" => Some(AmendedName),
                "Stockholder" => Some(Stockholder),
                "Managing Agent" => Some(ManagingAgent),
                "Organizer" => Some(Organizer),
                "Local Chain" => Some(LocalChain),
                "Source Meter Location" => Some(SourceMeterLocation),
                "Receipt Location" => Some(ReceiptLocation),
                "Upstream Meter Location" => Some(UpstreamMeterLocation),
                "Downstream Meter Location" => Some(DownstreamMeterLocation),
                "Migrant Health Clinic" => Some(MigrantHealthClinic),
                "Landlord" => Some(Landlord),
                "Foreclosing Lender" => Some(ForeclosingLender),
                "Educational Institution" => Some(EducationalInstitution),
                "Manufacturing" => Some(Manufacturing),
                "Party for whom Item is Ultimately Intended" => {
                    Some(PartyForWhomItemIsUltimatelyIntended)
                }
                "Company Interviewer Works For" => Some(CompanyInterviewerWorksFor),
                "Motor Carrier" => Some(MotorCarrier),
                "Veterans Administration Loan Guaranty Authority" => {
                    Some(VeteransAdministrationLoanGuarantyAuthority)
                }
                "Veterans Administration Loan Authorized Supplier" => {
                    Some(VeteransAdministrationLoanAuthorizedSupplier)
                }
                "Manufacturer of Goods" => Some(ManufacturerOfGoods),
                "Government Loan Agency Sponsor or Agent" => {
                    Some(GovernmentLoanAgencySponsorOrAgent)
                }
                "Mortgage Insurer" => Some(MortgageInsurer),
                "Planning Schedule/Material Release Issuer" => {
                    Some(PlanningScheduleMaterialReleaseIssuer)
                }
                "Financial Institution" => Some(FinancialInstitution),
                "Loan Holder for Real Estate Asset" => Some(LoanHolderForRealEstateAsset),
                "Consumer Credit Account Company" => Some(ConsumerCreditAccountCompany),
                "Mortgage Company" => Some(MortgageCompany),
                "Authorized Marketer" => Some(AuthorizedMarketer),
                "Release Drayman" => Some(ReleaseDrayman),
                "Manufacturing Plant" => Some(ManufacturingPlant),
                "Delivery Location" => Some(DeliveryLocation),
                "Medical Insurance Carrier" => Some(MedicalInsuranceCarrier),
                "Bureau of Land Management (Minerals Management Service) Property Unit" => {
                    Some(CodeMS)
                }
                "Mammography Screening Center" => Some(MammographyScreeningCenter),
                "Material" => Some(Material),
                "Meter Location" => Some(MeterLocation),
                "Meeting Location" => Some(MeetingLocation),
                "Mainline" => Some(Mainline),
                "Marine Surveyor" => Some(MarineSurveyor),
                "Juvenile Witness" => Some(JuvenileWitness),
                "Master General Agent" => Some(MasterGeneralAgent),
                "Minister" => Some(Minister),
                "Notify Party no. 1" => Some(NotifyPartyNo1),
                "Notify Party no. 2" => Some(NotifyPartyNo2),
                "Ineligible Party" => Some(IneligibleParty),
                "Price Administration" => Some(PriceAdministration),
                "Party Who Signed the Delivery Receipt" => {
                    Some(PartyWhoSignedTheDeliveryReceipt)
                }
                "Nonemployment Income Source" => Some(NonemploymentIncomeSource),
                "Previous Neighbor" => Some(PreviousNeighbor),
                "Relative" => Some(Relative),
                "Neighborhood" => Some(Neighborhood),
                "Neighbor" => Some(Neighbor),
                "Cross-Town Switch" => Some(CrossTownSwitch),
                "Name Changed To" => Some(NameChangedTo),
                "Next Destination" => Some(NextDestination),
                "Newspaper" => Some(Newspaper),
                "Owner Annuitant" => Some(OwnerAnnuitant),
                "Administrator" => Some(Administrator),
                "Association" => Some(Association),
                "Non-insured" => Some(NonInsured),
                "Trust or Estate" => Some(TrustOrEstate),
                "National Chain" => Some(NationalChain),
                "Non-railroad Entity" => Some(NonRailroadEntity),
                "Physician - Specialists" => Some(PhysicianSpecialists),
                "Network Name" => Some(NetworkName),
                "Notify Party for Shipper's Order" => Some(NotifyPartyForShippersOrder),
                "Notary Public" => Some(NotaryPublic),
                "Pipeline Segment Boundary" => Some(PipelineSegmentBoundary),
                "Gas Transaction Starting Point" => Some(GasTransactionStartingPoint),
                "Non-Temporary Storage Facility" => Some(NonTemporaryStorageFacility),
                "Magistrate Judge" => Some(MagistrateJudge),
                "Formerly Known As" => Some(FormerlyKnownAs),
                "Formerly Doing Business As" => Some(FormerlyDoingBusinessAs),
                "Maiden Name" => Some(MaidenName),
                "Primary Owner" => Some(PrimaryOwner),
                "Birth Name" => Some(BirthName),
                "Primary Physician" => Some(PrimaryPhysician),
                "Originating Bank" => Some(OriginatingBank),
                "Originating Company" => Some(OriginatingCompany),
                "Receiving Company" => Some(ReceivingCompany),
                "Factor" => Some(Factor),
                "Merchant Banker" => Some(MerchantBanker),
                "Non Registered Business Name" => Some(NonRegisteredBusinessName),
                "Registered Business Name" => Some(RegisteredBusinessName),
                "Registrar" => Some(Registrar),
                "Electronic Return Originator" => Some(ElectronicReturnOriginator),
                "Ordered By" => Some(OrderedBy),
                "Origin Carrier" => Some(OriginCarrier),
                "Doctor of Optometry" => Some(DoctorOfOptometry),
                "Booking Office" => Some(BookingOffice),
                "Offset Operator" => Some(OffsetOperator),
                "Co-owner" => Some(CoOwner),
                "Other Departments" => Some(OtherDepartments),
                "Outside Inspection Agency" => Some(OutsideInspectionAgency),
                "Officer" => Some(Officer),
                "Origin Mail Facility" => Some(OriginMailFacility),
                "Product Position Holder" => Some(ProductPositionHolder),
                "Order Of (Shippers Orders) - (Transportation)" => Some(CodeOO),
                "Operator of property or unit" => Some(OperatorOfPropertyOrUnit),
                "Origin Drayman" => Some(OriginDrayman),
                "Original Name" => Some(OriginalName),
                "Override Institution; this is not the institution sending the record, but another institution the student previously attended or is currently attending" => {
                    Some(CodeOS)
                }
                "Off-Site Handler" => Some(OffSiteHandler),
                "Origin Terminal" => Some(OriginTerminal),
                "Outside Processor" => Some(OutsideProcessor),
                "Other Unlisted Type of Corporation" => {
                    Some(OtherUnlistedTypeOfCorporation)
                }
                "Owner of Vessel" => Some(OwnerOfVessel),
                "Owner of Property or Unit" => Some(OwnerOfPropertyOrUnit),
                "Oxygen Therapy Facility" => Some(OxygenTherapyFacility),
                "Owner of Vehicle" => Some(OwnerOfVehicle),
                "Outside Testing Agency" => Some(OutsideTestingAgency),
                "Patient Facility" => Some(PatientFacility),
                "Preparer" => Some(Preparer),
                "Primary Insured or Subscriber" => Some(PrimaryInsuredOrSubscriber),
                "Primary Care Provider" => Some(PrimaryCareProvider),
                "Prior Insurance Carrier" => Some(PriorInsuranceCarrier),
                "Plan Sponsor" => Some(PlanSponsor),
                "Third Party Reviewing Preferred Provider Organization (PPO)" => {
                    Some(CodeP6)
                }
                "Third Party Repricing Preferred Provider Organization (PPO)" => {
                    Some(CodeP7)
                }
                "Personnel Office" => Some(PersonnelOffice),
                "Primary Interexchange Carrier (PIC)" => Some(CodeP9),
                "Party to Receive Inspection Report" => {
                    Some(PartyToReceiveInspectionReport)
                }
                "Paying Bank" => Some(PayingBank),
                "Party to Receive Cert. of Conformance (C.A.A.)" => Some(CodePC),
                "Purchaser's Department Buyer" => Some(PurchasersDepartmentBuyer),
                "Payee" => Some(Payee),
                "Party to Receive Freight Bill" => Some(PartyToReceiveFreightBill),
                "Prime Contractor" => Some(PrimeContractor),
                "Printer" => Some(Printer),
                "Publisher" => Some(Publisher),
                "Primary Inventory Control Activity" => {
                    Some(PrimaryInventoryControlActivity)
                }
                "Party to Receive Correspondence" => Some(PartyToReceiveCorrespondence),
                "Party to Receive Copy" => Some(PartyToReceiveCopy),
                "Party to Receive Purchase Order" => Some(PartyToReceivePurchaseOrder),
                "Law Enforcement Agency" => Some(LawEnforcementAgency),
                "Payer of Last Resort" => Some(PayerOfLastResort),
                "Party to receive paper Memo of Invoice" => {
                    Some(PartyToReceivePaperMemoOfInvoice)
                }
                "Prior Mortgage Company" => Some(PriorMortgageCompany),
                "Party Manufactured For" => Some(PartyManufacturedFor),
                "Program Manager" => Some(ProgramManager),
                "Party to Receive Shipping Notice" => Some(PartyToReceiveShippingNotice),
                "Party to Receive Invoice for Goods or Services" => {
                    Some(PartyToReceiveInvoiceForGoodsOrServices)
                }
                "Property" => Some(Property),
                "Past Performance Contact" => Some(PastPerformanceContact),
                "Person for Whose Benefit Property was Seized" => {
                    Some(PersonForWhoseBenefitPropertyWasSeized)
                }
                "Party to Receive Invoice for Lease Payments" => {
                    Some(PartyToReceiveInvoiceForLeasePayments)
                }
                "Payer" => Some(Payer),
                "Previous Owner" => Some(PreviousOwner),
                "Prospect Service" => Some(ProspectService),
                "Primary Payer" => Some(PrimaryPayer),
                "Previous Station" => Some(PreviousStation),
                "Party to Receive Test Report" => Some(PartyToReceiveTestReport),
                "Party at Pickup Location" => Some(PartyAtPickupLocation),
                "Purchased Company" => Some(PurchasedCompany),
                "Party performing certification" => Some(PartyPerformingCertification),
                "Pickup Address" => Some(PickupAddress),
                "Party Performing Count" => Some(PartyPerformingCount),
                "Party to File Personal Property Tax" => {
                    Some(PartyToFilePersonalPropertyTax)
                }
                "Party to Receive Equipment" => Some(PartyToReceiveEquipment),
                "Conductor Pilot" => Some(ConductorPilot),
                "Engineer Pilot" => Some(EngineerPilot),
                "Retail Account" => Some(RetailAccount),
                "Cooperative Buying Group" => Some(CooperativeBuyingGroup),
                "Advertising Group" => Some(AdvertisingGroup),
                "Interpreter" => Some(Interpreter),
                "Partner" => Some(Partner),
                "Base Period Employer" => Some(BasePeriodEmployer),
                "Last Employer" => Some(LastEmployer),
                "Pharmacy" => Some(Pharmacy),
                "Purchase Service Provider" => Some(PurchaseServiceProvider),
                "Patient" => Some(Patient),
                "Responsible Party" => Some(ResponsibleParty),
                "Policyholder" => Some(Policyholder),
                "Passenger" => Some(Passenger),
                "Pedestrian" => Some(Pedestrian),
                "Physician" => Some(Physician),
                "Party in Possession" => Some(PartyInPossession),
                "Most Recent Employer (Chargeable)" => Some(CodeQJ),
                "Managed Care" => Some(ManagedCare),
                "Chiropractor" => Some(Chiropractor),
                "Dialysis Centers" => Some(DialysisCenters),
                "Dentist" => Some(Dentist),
                "Doctor of Osteopathy" => Some(DoctorOfOsteopathy),
                "Principal Borrower" => Some(PrincipalBorrower),
                "Quality Control" => Some(QualityControl),
                "Buyer's Quality Review Board" => Some(BuyersQualityReviewBoard),
                "Podiatrist" => Some(Podiatrist),
                "Psychiatrist" => Some(Psychiatrist),
                "Veterinarian" => Some(Veterinarian),
                "Group Practice" => Some(GroupPractice),
                "Government" => Some(Government),
                "Home Health Corporation" => Some(HomeHealthCorporation),
                "Medical Doctor" => Some(MedicalDoctor),
                "Co-borrower" => Some(CoBorrower),
                "Royalty Owner" => Some(RoyaltyOwner),
                "Party to Receive Scale Ticket" => Some(PartyToReceiveScaleTicket),
                "Reporting Officer" => Some(ReportingOfficer),
                "Next Scheduled Destination" => Some(NextScheduledDestination),
                "Regulatory (State) District" => Some(CodeR4),
                "Regulatory (State) Entity" => Some(CodeR5),
                "Requester" => Some(Requester),
                "Consumer Referral Contact" => Some(ConsumerReferralContact),
                "Credit Reporting Agency" => Some(CreditReportingAgency),
                "Requested Lender" => Some(RequestedLender),
                "Alternate Return Address" => Some(AlternateReturnAddress),
                "Receiving Bank" => Some(ReceivingBank),
                "Receiving Location" => Some(ReceivingLocation),
                "Recovery Room" => Some(RecoveryRoom),
                "Destination Intermodal Ramp" => Some(DestinationIntermodalRamp),
                "Receiver Manager" => Some(ReceiverManager),
                "Refinery" => Some(Refinery),
                "Responsible Installation, Origin" => Some(CodeRG),
                "Responsible Government Agency" => Some(ResponsibleGovernmentAgency),
                "Responsible Installation, Destination" => Some(CodeRH),
                "Remit To" => Some(RemitTo),
                "Residence or Domicile" => Some(ResidenceOrDomicile),
                "Refinery Operator" => Some(RefineryOperator),
                "Reporting Location" => Some(ReportingLocation),
                "Party that remits payment" => Some(PartyThatRemitsPayment),
                "Repair or Refurbish Location" => Some(RepairOrRefurbishLocation),
                "Original Intermodal Ramp" => Some(OriginalIntermodalRamp),
                "Receiving Point for Customer Samples" => {
                    Some(ReceivingPointForCustomerSamples)
                }
                "Resale Customer" => Some(ResaleCustomer),
                "Railroad" => Some(Railroad),
                "Class II Railroad" => Some(ClassIiRailroad),
                "Class III Railroad" => Some(ClassIiiRailroad),
                "Receiving Facility Scheduler" => Some(ReceivingFacilityScheduler),
                "Returned to" => Some(ReturnedTo),
                "Receiving Sub-Location" => Some(ReceivingSubLocation),
                "Reservoir" => Some(Reservoir),
                "Rural Health Clinic" => Some(RuralHealthClinic),
                "Responsible Exhibitor" => Some(ResponsibleExhibitor),
                "Specified Repository" => Some(SpecifiedRepository),
                "Receipt Zone" => Some(ReceiptZone),
                "Sole Proprietor" => Some(SoleProprietor),
                "Parent" => Some(Parent),
                "Student" => Some(Student),
                "Custodial Parent" => Some(CustodialParent),
                "Skilled Nursing Facility" => Some(SkilledNursingFacility),
                "Secured Party" => Some(SecuredParty),
                "Agency Granting Security Clearance" => {
                    Some(AgencyGrantingSecurityClearance)
                }
                "Secured Party Company" => Some(SecuredPartyCompany),
                "Secured Party Individual" => Some(SecuredPartyIndividual),
                "Sibling" => Some(Sibling),
                "Salvage Carrier" => Some(SalvageCarrier),
                "Storage Area" => Some(StorageArea),
                "Store Class" => Some(StoreClass),
                "Sold To and Ship To" => Some(SoldToAndShipTo),
                "Selling Party" => Some(SellingParty),
                "Secondary Payer" => Some(SecondaryPayer),
                "Ship From" => Some(ShipFrom),
                "Store Group" => Some(StoreGroup),
                "Shipper" => Some(Shipper),
                "Shipping Schedule Issuer" => Some(ShippingScheduleIssuer),
                "Secondary Inventory Control Activity" => {
                    Some(SecondaryInventoryControlActivity)
                }
                "Ship-in-Place Location" => Some(ShipInPlaceLocation),
                "Service Provider" => Some(ServiceProvider),
                "Secondary Location Address (SLA)" => Some(CodeSK),
                "Origin Sublocation" => Some(OriginSublocation),
                "Party to Receive Shipping Manifest" => {
                    Some(PartyToReceiveShippingManifest)
                }
                "Store" => Some(Store),
                "US Customs & Border Protection Second Notify Party" => Some(CodeSNP),
                "Sold To If Different From Bill To" => Some(SoldToIfDifferentFromBillTo),
                "Party filling Shipper's Order" => Some(PartyFillingShippersOrder),
                "Service Bureau" => Some(ServiceBureau),
                "Samples to be Returned To" => Some(SamplesToBeReturnedTo),
                "Steamship Company" => Some(SteamshipCompany),
                "Ship To" => Some(ShipTo),
                "Switching and Terminal Carrier" => Some(SwitchingAndTerminalCarrier),
                "Supplier/Manufacturer" => Some(SupplierManufacturer),
                "Supply Source" => Some(SupplySource),
                "Service Performance Site" => Some(ServicePerformanceSite),
                "Sealing Company" => Some(SealingCompany),
                "School-based Service Provider" => Some(SchoolBasedServiceProvider),
                "Secondary Taxpayer" => Some(SecondaryTaxpayer),
                "Supervisor" => Some(Supervisor),
                "Operator of the Transfer Point" => Some(OperatorOfTheTransferPoint),
                "Operator of the Source Transfer Point" => {
                    Some(OperatorOfTheSourceTransferPoint)
                }
                "Terminal Location" => Some(TerminalLocation),
                "Transfer Point" => Some(TransferPoint),
                "Terminal Operator" => Some(TerminalOperator),
                "Previous Title Company" => Some(PreviousTitleCompany),
                "Prior Title Evidence Holder" => Some(PriorTitleEvidenceHolder),
                "Title Insurance Services Provider" => {
                    Some(TitleInsuranceServicesProvider)
                }
                "Tooling" => Some(Tooling),
                "Tool Source" => Some(ToolSource),
                "Tooling Design" => Some(ToolingDesign),
                "Theatre" => Some(Theatre),
                "Tax Exempt Corporation" => Some(TaxExemptCorporation),
                "Tank Farm" => Some(TankFarm),
                "Tooling Fabrication" => Some(ToolingFabrication),
                "Theater Circuit" => Some(TheaterCircuit),
                "Tariff Issuer" => Some(TariffIssuer),
                "Cosigner" => Some(Cosigner),
                "Test Sponsor" => Some(TestSponsor),
                "Testing Laboratory" => Some(TestingLaboratory),
                "Transmitter" => Some(Transmitter),
                "Tradename" => Some(Tradename),
                "Message To" => Some(MessageTo),
                "Towing Agency" => Some(TowingAgency),
                "Primary Taxpayer" => Some(PrimaryTaxpayer),
                "Third Party Marketer" => Some(ThirdPartyMarketer),
                "Third Party Reviewing Organization (TPO)" => Some(CodeTQ),
                "Terminal" => Some(Terminal),
                "Party to Receive Certified Test Results" => {
                    Some(PartyToReceiveCertifiedTestResults)
                }
                "Treatment, Storage or Disposal Facility" => Some(CodeTSD),
                "Consignee Courier Transfer Station" => {
                    Some(ConsigneeCourierTransferStation)
                }
                "Consignor Courier Transfer Station" => {
                    Some(ConsignorCourierTransferStation)
                }
                "Transfer To" => Some(TransferTo),
                "Tertiary Payer" => Some(TertiaryPayer),
                "Third Party Repricing Organization (TPO)" => Some(CodeTU),
                "Third Party Administrator (TPA)" => Some(CodeTV),
                "Transit Authority" => Some(TransitAuthority),
                "Tax Authority" => Some(TaxAuthority),
                "Trustee" => Some(Trustee),
                "Significant Other" => Some(SignificantOther),
                "Gas Transaction Point 1" => Some(GasTransactionPoint1),
                "Gas Transaction Point 2" => Some(GasTransactionPoint2),
                "Servicing Agent" => Some(ServicingAgent),
                "Team" => Some(Team),
                "Underwriter" => Some(Underwriter),
                "Title Underwriter" => Some(TitleUnderwriter),
                "Psychologist" => Some(Psychologist),
                "Reference" => Some(Reference),
                "Non-Registered Investment Advisor" => {
                    Some(NonRegisteredInvestmentAdvisor)
                }
                "Place of Bottling" => Some(PlaceOfBottling),
                "Place of Distilling" => Some(PlaceOfDistilling),
                "Ultimate Consignee" => Some(UltimateConsignee),
                "Region" => Some(Region),
                "Testing Service" => Some(TestingService),
                "Health Miscellaneous" => Some(HealthMiscellaneous),
                "Nursing Home Chain" => Some(NursingHomeChain),
                "Nursing Home" => Some(NursingHome),
                "Registered Investment Advisor" => Some(RegisteredInvestmentAdvisor),
                "Sales Assistant" => Some(SalesAssistant),
                "System" => Some(System),
                "Special Account" => Some(SpecialAccount),
                "Current Employer (Primary)" => Some(CodeUM),
                "Union" => Some(Union),
                "Current Employer (Secondary)" => Some(CodeUO),
                "Unloading Party" => Some(UnloadingParty),
                "Subsequent Owner" => Some(SubsequentOwner),
                "Surgeon" => Some(Surgeon),
                "Upstream Party" => Some(UpstreamParty),
                "U.S. Trustee" => Some(USTrustee),
                "Annuitant Payor" => Some(AnnuitantPayor),
                "Unassigned Agent" => Some(UnassignedAgent),
                "Base Jurisdiction" => Some(BaseJurisdiction),
                "Vehicle" => Some(Vehicle),
                "Signer" => Some(Signer),
                "Surety" => Some(Surety),
                "Grantor" => Some(Grantor),
                "Well Pad Construction Contractor" => Some(WellPadConstructionContractor),
                "Oil and Gas Regulatory Agency" => Some(OilAndGasRegulatoryAgency),
                "Surface Discharge Agency" => Some(SurfaceDischargeAgency),
                "Well Casing Depth Authority" => Some(WellCasingDepthAuthority),
                "Market Timer" => Some(MarketTimer),
                "Owner Annuitant Payor" => Some(OwnerAnnuitantPayor),
                "Second Contact" => Some(SecondContact),
                "Candidate" => Some(Candidate),
                "Vehicle Custodian" => Some(VehicleCustodian),
                "Multiple Listing Service" => Some(MultipleListingService),
                "Board of Realtors" => Some(BoardOfRealtors),
                "Party Performing Verification" => Some(PartyPerformingVerification),
                "Selling Office" => Some(SellingOffice),
                "Listing Agent" => Some(ListingAgent),
                "Showing Agent" => Some(ShowingAgent),
                "Contact Person" => Some(ContactPerson),
                "Victim" => Some(Victim),
                "Owner Joint Annuitant Payor" => Some(OwnerJointAnnuitantPayor),
                "Property or Building Manager" => Some(PropertyOrBuildingManager),
                "Builder Name" => Some(BuilderName),
                "Occupant" => Some(Occupant),
                "Vendor" => Some(Vendor),
                "Elementary School" => Some(ElementarySchool),
                "Party with Power to Vote Securities" => {
                    Some(PartyWithPowerToVoteSecurities)
                }
                "Middle School" => Some(MiddleSchool),
                "Junior High School" => Some(JuniorHighSchool),
                "Vehicle Salvage Assignment" => Some(VehicleSalvageAssignment),
                "Listing Office" => Some(ListingOffice),
                "Second Contact Organization" => Some(SecondContactOrganization),
                "Owner Payor" => Some(OwnerPayor),
                "Winner" => Some(Winner),
                "Production Manager" => Some(ProductionManager),
                "Organization Completing Configuration Change" => {
                    Some(OrganizationCompletingConfigurationChange)
                }
                "Work Team" => Some(WorkTeam),
                "Supplier Work Team" => Some(SupplierWorkTeam),
                "Third Party Investment Advisor" => Some(ThirdPartyInvestmentAdvisor),
                "Trust" => Some(Trust),
                "Interline Service Commitment Customer" => {
                    Some(InterlineServiceCommitmentCustomer)
                }
                "Sampling Location" => Some(SamplingLocation),
                "Writing Agent" => Some(WritingAgent),
                "Appraiser Name" => Some(AppraiserName),
                "Comparable Property" => Some(ComparableProperty),
                "Storage Facility at Destination" => Some(StorageFacilityAtDestination),
                "Subject Property" => Some(SubjectProperty),
                "Tank Farm Owner" => Some(TankFarmOwner),
                "Wage Earner" => Some(WageEarner),
                "Warehouse" => Some(Warehouse),
                "Witness" => Some(Witness),
                "Supervisory Appraiser Name" => Some(SupervisoryAppraiserName),
                "Wholesaler" => Some(Wholesaler),
                "Company Assigned Well" => Some(CompanyAssignedWell),
                "Storage Facility at Origin" => Some(StorageFacilityAtOrigin),
                "Witness for Plaintiff" => Some(WitnessForPlaintiff),
                "Withdrawal Point" => Some(WithdrawalPoint),
                "Water System" => Some(WaterSystem),
                "Witness for Defendant" => Some(WitnessForDefendant),
                "Primary Support Organization" => Some(PrimarySupportOrganization),
                "Preliminary Maintenance Period Designating Organization" => {
                    Some(PreliminaryMaintenancePeriodDesignatingOrganization)
                }
                "Preliminary Maintenance Organization" => {
                    Some(PreliminaryMaintenanceOrganization)
                }
                "Preliminary Referred To Organization" => {
                    Some(PreliminaryReferredToOrganization)
                }
                "Final Maintenance Period Designating Organization" => {
                    Some(FinalMaintenancePeriodDesignatingOrganization)
                }
                "Final Maintenance Organization" => Some(FinalMaintenanceOrganization),
                "Mail to" => Some(MailTo),
                "Party to Perform Packaging" => Some(PartyToPerformPackaging),
                "Utilization Management Organization" => {
                    Some(UtilizationManagementOrganization)
                }
                "Spouse" => Some(Spouse),
                "Durable Medical Equipment Supplier" => {
                    Some(DurableMedicalEquipmentSupplier)
                }
                "International Organization" => Some(InternationalOrganization),
                "Inventor" => Some(Inventor),
                "Hispanic Service Institute" => Some(HispanicServiceInstitute),
                "Creditor" => Some(Creditor),
                "Debtor's Attorney" => Some(DebtorsAttorney),
                "Alias" => Some(Alias),
                "Claim Recipient" => Some(ClaimRecipient),
                "Auctioneer" => Some(Auctioneer),
                "Event Location" => Some(EventLocation),
                "Final Referred To Organization" => Some(FinalReferredToOrganization),
                "Original Claimant" => Some(OriginalClaimant),
                "Actual Referred By Organization" => Some(ActualReferredByOrganization),
                "Actual Referred To Organization" => Some(ActualReferredToOrganization),
                "Borrower's Employer" => Some(BorrowersEmployer),
                "Maintenance Organization Used for Estimate" => {
                    Some(MaintenanceOrganizationUsedForEstimate)
                }
                "Planning/Maintenance Organization" => {
                    Some(PlanningMaintenanceOrganization)
                }
                "Preliminary Customer Organization" => {
                    Some(PreliminaryCustomerOrganization)
                }
                "Party to Receive Solicitation" => Some(PartyToReceiveSolicitation),
                "Canadian Customs Broker" => Some(CanadianCustomsBroker),
                "Mexican Customs Broker" => Some(MexicanCustomsBroker),
                "S Corporation" => Some(SCorporation),
                "Final Customer Organization" => Some(FinalCustomerOrganization),
                "United States Customs Broker" => Some(UnitedStatesCustomsBroker),
                "Cross Claimant" => Some(CrossClaimant),
                "Counter Claimant" => Some(CounterClaimant),
                "Business Area" => Some(BusinessArea),
                "Tribal Government" => Some(TribalGovernment),
                "American Indian-Owned Business" => Some(AmericanIndianOwnedBusiness),
                "Managed Care Organization" => Some(ManagedCareOrganization),
                "Affiant" => Some(Affiant),
                "Arbitrator" => Some(Arbitrator),
                "Bail Payor" => Some(BailPayor),
                "District Justice" => Some(DistrictJustice),
                "Third Party" => Some(ThirdParty),
                "Witness for Prosecution" => Some(WitnessForProsecution),
                "Expert Witness" => Some(ExpertWitness),
                "Crime Victim" => Some(CrimeVictim),
                "Juvenile Victim" => Some(JuvenileVictim),
                "Juvenile Defendant" => Some(JuvenileDefendant),
                "Bondsman" => Some(Bondsman),
                "Court Appointed Attorney" => Some(CourtAppointedAttorney),
                "Complainant's Attorney" => Some(ComplainantsAttorney),
                "District Attorney" => Some(DistrictAttorney),
                "Attorney for Defendant, Public" => Some(CodeYO),
                "Pro Bono Attorney" => Some(ProBonoAttorney),
                "Pro Se Counsel" => Some(ProSeCounsel),
                "Party to Appear Before" => Some(PartyToAppearBefore),
                "Appellant" => Some(Appellant),
                "Appellee" => Some(Appellee),
                "Arresting Officer" => Some(ArrestingOfficer),
                "Hostile Witness" => Some(HostileWitness),
                "Discharge Point" => Some(DischargePoint),
                "Flood Certifier" => Some(FloodCertifier),
                "Flood Determination Provider" => Some(FloodDeterminationProvider),
                "Electronic Registration Utility" => Some(ElectronicRegistrationUtility),
                "Party to Receive Status" => Some(PartyToReceiveStatus),
                "Unserviceable Material Consignee" => {
                    Some(UnserviceableMaterialConsignee)
                }
                "Potential Source of Supply" => Some(PotentialSourceOfSupply),
                "Owning Inventory Control Point" => Some(OwningInventoryControlPoint),
                "Management Control Activity" => Some(ManagementControlActivity),
                "Transferring Party" => Some(TransferringParty),
                "Mark-for Party" => Some(MarkForParty),
                "Last Known Source of Supply" => Some(LastKnownSourceOfSupply),
                "Banker" => Some(Banker),
                "Corrected Address" => Some(CorrectedAddress),
                "Party to Receive Credit" => Some(PartyToReceiveCredit),
                "Rent Payor" => Some(RentPayor),
                "Party to Receive Reports" => Some(PartyToReceiveReports),
                "End Item Manufacturer" => Some(EndItemManufacturer),
                "Break Bulk Point" => Some(BreakBulkPoint),
                "Present Address" => Some(PresentAddress),
                "Child" => Some(Child),
                "Branch" => Some(Branch),
                "Reporter" => Some(Reporter),
                "Party Passing the Transaction" => Some(PartyPassingTheTransaction),
                "Lease Location" => Some(LeaseLocation),
                "Losing Inventory Manager" => Some(LosingInventoryManager),
                "Minimum Royalty Payor" => Some(MinimumRoyaltyPayor),
                "Gaining Inventory Manager" => Some(GainingInventoryManager),
                "Screening Point" => Some(ScreeningPoint),
                "Validating Party" => Some(ValidatingParty),
                "Monitoring Party" => Some(MonitoringParty),
                "Participating Area" => Some(ParticipatingArea),
                "Formation" => Some(Formation),
                "Allowable Recipient" => Some(AllowableRecipient),
                "Field" => Some(Field),
                "Attorney of Record" => Some(AttorneyOfRecord),
                "Amicus Curiae" => Some(AmicusCuriae),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for EntityIdentifierCode {
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
    type Value = EntityIdentifierCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Entity Identifier Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        EntityIdentifierCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Entity Identifier Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        EntityIdentifierCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Entity Identifier Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for EntityIdentifierCode {
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