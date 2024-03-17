use serde::{Serialize, Deserialize};
use super::super::segments::TransactionSetLineNumber;
use super::super::segments::TransactionSetTrailer;
use super::super::segments::HomeHealthCareCertification;
use super::super::segments::MonetaryAmount;
use super::super::segments::DurableMedicalEquipmentService;
use super::super::segments::PartyLocation;
use super::super::segments::LoopHeader;
use super::super::segments::HealthClaim;
use super::super::segments::Currency;
use super::super::segments::InpatientAdjudication;
use super::super::segments::DateOrTimeOrPeriod;
use super::super::segments::HierarchicalLevel;
use super::super::segments::Subscriber;
use super::super::segments::OutpatientAdjudication;
use super::super::segments::DrugAdjudication;
use super::super::segments::ClaimsAdjustment;
use super::super::segments::ItemIdentification;
use super::super::segments::Reference;
use super::super::segments::ChiropracticCertification;
use super::super::segments::AnesthesiaService;
use super::super::segments::AmbulanceCertification;
use super::super::segments::Provider;
use super::super::segments::TransactionSetHeader;
use super::super::segments::AdministrativeCommunicationsContact;
use super::super::segments::Orthodontic;
use super::super::segments::InstitutionalService;
use super::super::segments::ClaimCodes;
use super::super::segments::ImmunizationStatus;
use super::super::segments::PurchaseService;
use super::super::segments::HomeHealthTreatmentPlanCertification;
use super::super::segments::NoteSpecialInstruction;
use super::super::segments::GeographicLocation;
use super::super::segments::Measurements;
use super::super::segments::ProfessionalService;
use super::super::segments::AdditionalName;
use super::super::segments::Contract;
use super::super::segments::PeerReviewOrganizationOrUtilizationReview;
use super::super::segments::HealthCareServicesDelivery;
use super::super::segments::Disability;
use super::super::segments::ToothSummary;
use super::super::segments::IndividualOrOrganizationalName;
use super::super::segments::File;
use super::super::segments::OxygenTherapyCertification;
use super::super::segments::ServiceLineAdjudication;
use super::super::segments::ConditionsIndicator;
use super::super::segments::IndustryCodeIdentification;
use super::super::segments::Patient;
use super::super::segments::DrugService;
use super::super::segments::DentalService;
use super::super::segments::SupportingDocumentation;
use super::super::segments::Demographic;
use super::super::segments::LanguageUse;
use super::super::segments::Paperwork;
use super::super::segments::HealthCareInformationCodes;
use super::super::segments::BeginningOfHierarchicalTransaction;
use super::super::segments::ReasonAdjustment;
use super::super::segments::Quantity;
use super::super::segments::OtherHealthInsurance;
use super::super::segments::ToothIdentification;
use super::super::segments::EnteralOrParenteralTherapyCertification;
use super::super::segments::ImplantCertification;
use super::super::segments::LoopTrailer;
use super::super::segments::Pricing;
use super::super::segments::HealthCarePricing;
use super::super::segments::DurableMedicalEquipmentCertification;
/**This X12 Transaction Set contains the format and establishes the data contents of the Health Care Claim Transaction Set (837) for use within the context of an Electronic Data Interchange (EDI) environment. This transaction set can be used to submit health care claim billing information, encounter information, or both, from providers of health care services to payers, either directly or via intermediary billers and claims clearinghouses. It can also be used to transmit health care claims and billing payment information between payers with different payment responsibilities where coordination of benefits is required or between payers and regulatory agencies to monitor the rendering, billing, and/or payment of health care services within a specific health care/insurance industry segment.

For purposes of this standard, providers of health care products or services may include entities such as physicians, hospitals and other medical facilities or suppliers, dentists, and pharmacies, and entities providing medical information to meet regulatory requirements. The payer refers to a third party entity that pays claims or administers the insurance product or benefit or both. For example, a payer may be an insurance company, health maintenance organization (HMO), preferred provider organization (PPO), government agency (Medicare, Medicaid, Civilian Health and Medical Program of the Uniformed Services (CHAMPUS), etc.) or an entity such as a third party administrator (TPA) or third party organization (TPO) that may be contracted by one of those groups. A regulatory agency is an entity responsible, by law or rule, for administering and monitoring a statutory benefits program or a specific health care/insurance industry segment.



See full docs at <https://www.stedi.com/edi/x12/transaction-set/837>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct HealthCareClaim {
    /**Transaction Set Header (ST)
To indicate the start of a transaction set and to assign a control number*/
    pub transaction_set_header: TransactionSetHeader,
    /**Beginning of Hierarchical Transaction (BHT)
To define the business hierarchical structure of the transaction set and identify the business application purpose and reference data, i.e., number, date, and time*/
    pub beginning_of_hierarchical_transaction: BeginningOfHierarchicalTransaction,
    /**Reference Information (REF)
To specify identifying information*/
    pub reference: Vec<Reference>,
    ///0200
    pub _1000: Vec<Loop1000>,
    ///0010
    pub _2000: Vec<Loop2000>,
    /**Transaction Set Trailer (SE)
To indicate the end of the transaction set and provide the count of the transmitted segments (including the beginning (ST) and ending (SE) segments)*/
    pub transaction_set_trailer: TransactionSetTrailer,
}
impl HealthCareClaim {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2000 {
    /**Hierarchical Level (HL)
To identify dependencies among and the content of hierarchically related groups of data segments*/
    pub hierarchical_level: HierarchicalLevel,
    /**Provider Information (PRV)
To specify the identifying characteristics of a provider*/
    pub provider: Option<Provider>,
    /**Subscriber Information (SBR)
To record information specific to the primary insured and the insurance carrier for that insured*/
    pub subscriber: Option<Subscriber>,
    /**Patient Information (PAT)
To supply patient information*/
    pub patient: Option<Patient>,
    /**Date or Time or Period (DTP)
To specify any or all of a date, a time, or a time period*/
    pub date_or_time_or_period: Vec<DateOrTimeOrPeriod>,
    /**Currency (CUR)
To specify the currency (dollars, pounds, francs, etc.) used in a transaction*/
    pub currency: Option<Currency>,
    ///0150
    pub _2010: Vec<Loop2010>,
    ///1300
    pub _2300: Vec<Loop2300>,
}
impl Loop2000 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2300 {
    /**Health Claim (CLM)
To specify basic data about the claim*/
    pub health_claim: HealthClaim,
    /**Date or Time or Period (DTP)
To specify any or all of a date, a time, or a time period*/
    pub date_or_time_or_period: Vec<DateOrTimeOrPeriod>,
    /**Claim Codes (CL1)
To supply information specific to hospital claims*/
    pub claim_codes: Option<ClaimCodes>,
    /**Orthodontic Information (DN1)
To supply orthodontic information*/
    pub orthodontic: Option<Orthodontic>,
    /**Tooth Summary (DN2)
To specify the status of individual teeth*/
    pub tooth_summary: Vec<ToothSummary>,
    /**Paperwork (PWK)
To identify the type or transmission or both of paperwork or supporting information*/
    pub paperwork: Vec<Paperwork>,
    /**Contract Information (CN1)
To specify basic data about the contract or contract line item*/
    pub contract: Option<Contract>,
    /**Disability Information (DSB)
To supply disability information*/
    pub disability: Option<Disability>,
    /**Peer Review Organization or Utilization Review (UR)
To specify the results of the utilization review*/
    pub peer_review_organization_or_utilization_review: Option<
        PeerReviewOrganizationOrUtilizationReview,
    >,
    /**Monetary Amount Information (AMT)
To indicate the total monetary amount*/
    pub monetary_amount: Vec<MonetaryAmount>,
    /**Reference Information (REF)
To specify identifying information*/
    pub reference: Vec<Reference>,
    /**File Information (K3)
To transmit a fixed-format record or matrix contents*/
    pub file: Vec<File>,
    /**Note/Special Instruction (NTE)
To transmit information in a free-form format, if necessary, for comment or special instruction*/
    pub note_special_instruction: Vec<NoteSpecialInstruction>,
    /**Ambulance Certification (CR1)
To supply information related to the ambulance service rendered to a patient*/
    pub ambulance_certification: Option<AmbulanceCertification>,
    /**Chiropractic Certification (CR2)
To supply information related to the chiropractic service rendered to a patient*/
    pub chiropractic_certification: Option<ChiropracticCertification>,
    /**Durable Medical Equipment Certification (CR3)
To supply information regarding a physician's certification for durable medical equipment*/
    pub durable_medical_equipment_certification: Option<
        DurableMedicalEquipmentCertification,
    >,
    /**Enteral or Parenteral Therapy Certification (CR4)
To supply information regarding certification of medical necessity for enteral or parenteral nutrition therapy*/
    pub enteral_or_parenteral_therapy_certification: Vec<
        EnteralOrParenteralTherapyCertification,
    >,
    /**Oxygen Therapy Certification (CR5)
To supply information regarding certification of medical necessity for home oxygen therapy*/
    pub oxygen_therapy_certification: Option<OxygenTherapyCertification>,
    /**Home Health Care Certification (CR6)
To supply information related to the certification of a home health care patient*/
    pub home_health_care_certification: Option<HomeHealthCareCertification>,
    /**Implant Certification (CR8)
To supply information related to medical implant registries*/
    pub implant_certification: Vec<ImplantCertification>,
    /**Conditions Indicator (CRC)
To supply information on conditions*/
    pub conditions_indicator: Vec<ConditionsIndicator>,
    /**Health Care Information Codes (HI)
To supply information related to the delivery of health care*/
    pub health_care_information_codes: Vec<HealthCareInformationCodes>,
    /**Quantity Information (QTY)
To specify quantity information*/
    pub quantity: Vec<Quantity>,
    /**Health Care Pricing (HCP)
To specify pricing or repricing information about a health care claim or line item*/
    pub health_care_pricing: Option<HealthCarePricing>,
    ///2420
    pub _2305: Vec<Loop2305>,
    ///2500
    pub _2310: Vec<Loop2310>,
    ///2900
    pub _2320: Vec<Loop2320>,
    ///3650
    pub _2400: Vec<Loop2400>,
}
impl Loop2300 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2400 {
    /**Transaction Set Line Number (LX)
To reference a line number in a transaction set*/
    pub transaction_set_line_number: TransactionSetLineNumber,
    /**Professional Service (SV1)
To specify the service line item detail for a health care professional*/
    pub professional_service: Option<ProfessionalService>,
    /**Institutional Service (SV2)
To specify the service line item detail for a health care institution*/
    pub institutional_service: Option<InstitutionalService>,
    /**Dental Service (SV3)
To specify the service line item detail for dental work*/
    pub dental_service: Option<DentalService>,
    /**Tooth Identification (TOO)
To identify a tooth by number and, if applicable, one or more tooth surfaces*/
    pub tooth_identification: Vec<ToothIdentification>,
    /**Drug Service (SV4)
To specify the claim service detail for prescription drugs*/
    pub drug_service: Option<DrugService>,
    /**Durable Medical Equipment Service (SV5)
To specify the claim service detail for durable medical equipment*/
    pub durable_medical_equipment_service: Option<DurableMedicalEquipmentService>,
    /**Anesthesia Service (SV6)
To specify the claim service detail for anesthesia*/
    pub anesthesia_service: Option<AnesthesiaService>,
    /**Drug Adjudication (SV7)
To specify the claim service detail for drug services that have been adjudicated*/
    pub drug_adjudication: Option<DrugAdjudication>,
    /**Health Care Information Codes (HI)
To supply information related to the delivery of health care*/
    pub health_care_information_codes: Vec<HealthCareInformationCodes>,
    /**Paperwork (PWK)
To identify the type or transmission or both of paperwork or supporting information*/
    pub paperwork: Vec<Paperwork>,
    /**Ambulance Certification (CR1)
To supply information related to the ambulance service rendered to a patient*/
    pub ambulance_certification: Option<AmbulanceCertification>,
    /**Chiropractic Certification (CR2)
To supply information related to the chiropractic service rendered to a patient*/
    pub chiropractic_certification: Vec<ChiropracticCertification>,
    /**Durable Medical Equipment Certification (CR3)
To supply information regarding a physician's certification for durable medical equipment*/
    pub durable_medical_equipment_certification: Option<
        DurableMedicalEquipmentCertification,
    >,
    /**Enteral or Parenteral Therapy Certification (CR4)
To supply information regarding certification of medical necessity for enteral or parenteral nutrition therapy*/
    pub enteral_or_parenteral_therapy_certification: Vec<
        EnteralOrParenteralTherapyCertification,
    >,
    /**Oxygen Therapy Certification (CR5)
To supply information regarding certification of medical necessity for home oxygen therapy*/
    pub oxygen_therapy_certification: Option<OxygenTherapyCertification>,
    /**Conditions Indicator (CRC)
To supply information on conditions*/
    pub conditions_indicator: Vec<ConditionsIndicator>,
    /**Date or Time or Period (DTP)
To specify any or all of a date, a time, or a time period*/
    pub date_or_time_or_period: Vec<DateOrTimeOrPeriod>,
    /**Quantity Information (QTY)
To specify quantity information*/
    pub quantity: Vec<Quantity>,
    /**Measurements (MEA)
To specify physical measurements or counts, including dimensions, tolerances, variances, and weights

(See Figures Appendix for example of use of C001)*/
    pub measurements: Vec<Measurements>,
    /**Contract Information (CN1)
To specify basic data about the contract or contract line item*/
    pub contract: Option<Contract>,
    /**Reference Information (REF)
To specify identifying information*/
    pub reference: Vec<Reference>,
    /**Monetary Amount Information (AMT)
To indicate the total monetary amount*/
    pub monetary_amount: Vec<MonetaryAmount>,
    /**File Information (K3)
To transmit a fixed-format record or matrix contents*/
    pub file: Vec<File>,
    /**Note/Special Instruction (NTE)
To transmit information in a free-form format, if necessary, for comment or special instruction*/
    pub note_special_instruction: Vec<NoteSpecialInstruction>,
    /**Purchase Service (PS1)
To specify the information about services that are purchased*/
    pub purchase_service: Option<PurchaseService>,
    /**Immunization Status (IMM)
To provide the receiving school district or postsecondary institution with a notice of the immunization status of the student*/
    pub immunization_status: Option<ImmunizationStatus>,
    /**Health Care Services Delivery (HSD)
To specify the delivery pattern of health care services*/
    pub health_care_services_delivery: Option<HealthCareServicesDelivery>,
    /**Health Care Pricing (HCP)
To specify pricing or repricing information about a health care claim or line item*/
    pub health_care_pricing: Option<HealthCarePricing>,
    /**Implant Certification (CR8)
To supply information related to medical implant registries*/
    pub implant_certification: Option<ImplantCertification>,
    ///4930
    pub _2410: Vec<Loop2410>,
    ///5000
    pub _2420: Vec<Loop2420>,
    ///5400
    pub _2430: Vec<Loop2430>,
    /**Loop Header (LS)
To indicate that the next segment begins a loop*/
    pub loop_header: Option<LoopHeader>,
    ///5510
    pub _2440: Vec<Loop2440>,
    /**Loop Trailer (LE)
To indicate that the loop immediately preceding this segment is complete*/
    pub loop_trailer: Option<LoopTrailer>,
}
impl Loop2400 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2440 {
    /**Industry Code Identification (LQ)
To identify standard industry codes*/
    pub industry_code_identification: IndustryCodeIdentification,
    /**Supporting Documentation (FRM)
To specify information in response to a codified questionnaire document*/
    pub supporting_documentation: Vec<SupportingDocumentation>,
}
impl Loop2440 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2430 {
    /**Service Line Adjudication (SVD)
To convey service line adjudication information for coordination of benefits between the initial payers of a health care claim and all subsequent payers*/
    pub service_line_adjudication: ServiceLineAdjudication,
    /**Claims Adjustment (CAS)
To supply adjustment reason codes and amounts as needed for an entire claim or for a particular service within the claim being paid*/
    pub claims_adjustment: Vec<ClaimsAdjustment>,
    /**Reason Adjustment (RAS)
To supply Claim Adjustment Reason Codes and amounts as needed for an entire claim or for a particular service within the claim being paid*/
    pub reason_adjustment: Vec<ReasonAdjustment>,
    /**Date or Time or Period (DTP)
To specify any or all of a date, a time, or a time period*/
    pub date_or_time_or_period: Vec<DateOrTimeOrPeriod>,
    /**Monetary Amount Information (AMT)
To indicate the total monetary amount*/
    pub monetary_amount: Vec<MonetaryAmount>,
    /**Industry Code Identification (LQ)
To identify standard industry codes*/
    pub industry_code_identification: Vec<IndustryCodeIdentification>,
}
impl Loop2430 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2420 {
    /**Individual or Organizational Name (NM1)
To supply the full name of an individual or organizational entity*/
    pub individual_or_organizational_name: IndividualOrOrganizationalName,
    /**Provider Information (PRV)
To specify the identifying characteristics of a provider*/
    pub provider: Option<Provider>,
    /**Additional Name Information (N2)
To specify additional names*/
    pub additional_name: Vec<AdditionalName>,
    /**Party Location (N3)
To specify the location of the named party*/
    pub party_location: Vec<PartyLocation>,
    /**Geographic Location (N4)
To specify the geographic place of the named party*/
    pub geographic_location: Option<GeographicLocation>,
    /**Reference Information (REF)
To specify identifying information*/
    pub reference: Vec<Reference>,
    /**Administrative Communications Contact (PER)
To identify a person or office to whom administrative communications should be directed*/
    pub administrative_communications_contact: Vec<AdministrativeCommunicationsContact>,
}
impl Loop2420 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2410 {
    /**Item Identification (LIN)
To specify basic item identification data*/
    pub item_identification: ItemIdentification,
    /**Pricing Information (CTP)
To specify pricing information*/
    pub pricing: Option<Pricing>,
    /**Reference Information (REF)
To specify identifying information*/
    pub reference: Option<Reference>,
    /**Drug Service (SV4)
To specify the claim service detail for prescription drugs*/
    pub drug_service: Option<DrugService>,
    /**Drug Adjudication (SV7)
To specify the claim service detail for drug services that have been adjudicated*/
    pub drug_adjudication: Option<DrugAdjudication>,
}
impl Loop2410 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2320 {
    /**Subscriber Information (SBR)
To record information specific to the primary insured and the insurance carrier for that insured*/
    pub subscriber: Subscriber,
    /**Claims Adjustment (CAS)
To supply adjustment reason codes and amounts as needed for an entire claim or for a particular service within the claim being paid*/
    pub claims_adjustment: Vec<ClaimsAdjustment>,
    /**Reason Adjustment (RAS)
To supply Claim Adjustment Reason Codes and amounts as needed for an entire claim or for a particular service within the claim being paid*/
    pub reason_adjustment: Vec<ReasonAdjustment>,
    /**Monetary Amount Information (AMT)
To indicate the total monetary amount*/
    pub monetary_amount: Vec<MonetaryAmount>,
    /**Demographic Information (DMG)
To supply demographic information*/
    pub demographic: Option<Demographic>,
    /**Other Health Insurance Information (OI)
To specify information associated with other health insurance coverage*/
    pub other_health_insurance: Option<OtherHealthInsurance>,
    /**Inpatient Adjudication (MIA)
To provide claim level data related to the adjudication of inpatient claims*/
    pub inpatient_adjudication: Option<InpatientAdjudication>,
    /**Outpatient Adjudication (MOA)
To provide claim level data related to the adjudication of outpatient claims*/
    pub outpatient_adjudication: Option<OutpatientAdjudication>,
    /**Health Care Information Codes (HI)
To supply information related to the delivery of health care*/
    pub health_care_information_codes: Option<HealthCareInformationCodes>,
    /**Industry Code Identification (LQ)
To identify standard industry codes*/
    pub industry_code_identification: Vec<IndustryCodeIdentification>,
    ///3250
    pub _2330: Vec<Loop2330>,
}
impl Loop2320 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2330 {
    /**Individual or Organizational Name (NM1)
To supply the full name of an individual or organizational entity*/
    pub individual_or_organizational_name: IndividualOrOrganizationalName,
    /**Additional Name Information (N2)
To specify additional names*/
    pub additional_name: Vec<AdditionalName>,
    /**Party Location (N3)
To specify the location of the named party*/
    pub party_location: Vec<PartyLocation>,
    /**Geographic Location (N4)
To specify the geographic place of the named party*/
    pub geographic_location: Option<GeographicLocation>,
    /**Administrative Communications Contact (PER)
To identify a person or office to whom administrative communications should be directed*/
    pub administrative_communications_contact: Vec<AdministrativeCommunicationsContact>,
    /**Date or Time or Period (DTP)
To specify any or all of a date, a time, or a time period*/
    pub date_or_time_or_period: Vec<DateOrTimeOrPeriod>,
    /**Reference Information (REF)
To specify identifying information*/
    pub reference: Option<Reference>,
}
impl Loop2330 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2310 {
    /**Individual or Organizational Name (NM1)
To supply the full name of an individual or organizational entity*/
    pub individual_or_organizational_name: IndividualOrOrganizationalName,
    /**Provider Information (PRV)
To specify the identifying characteristics of a provider*/
    pub provider: Option<Provider>,
    /**Additional Name Information (N2)
To specify additional names*/
    pub additional_name: Vec<AdditionalName>,
    /**Party Location (N3)
To specify the location of the named party*/
    pub party_location: Vec<PartyLocation>,
    /**Geographic Location (N4)
To specify the geographic place of the named party*/
    pub geographic_location: Option<GeographicLocation>,
    /**Reference Information (REF)
To specify identifying information*/
    pub reference: Vec<Reference>,
    /**Administrative Communications Contact (PER)
To identify a person or office to whom administrative communications should be directed*/
    pub administrative_communications_contact: Vec<AdministrativeCommunicationsContact>,
}
impl Loop2310 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2305 {
    /**Home Health Treatment Plan Certification (CR7)
To supply information related to the home health care plan of treatment and services*/
    pub home_health_treatment_plan_certification: HomeHealthTreatmentPlanCertification,
    /**Health Care Services Delivery (HSD)
To specify the delivery pattern of health care services*/
    pub health_care_services_delivery: Vec<HealthCareServicesDelivery>,
}
impl Loop2305 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2010 {
    /**Individual or Organizational Name (NM1)
To supply the full name of an individual or organizational entity*/
    pub individual_or_organizational_name: IndividualOrOrganizationalName,
    /**Additional Name Information (N2)
To specify additional names*/
    pub additional_name: Vec<AdditionalName>,
    /**Party Location (N3)
To specify the location of the named party*/
    pub party_location: Vec<PartyLocation>,
    /**Geographic Location (N4)
To specify the geographic place of the named party*/
    pub geographic_location: Option<GeographicLocation>,
    /**Demographic Information (DMG)
To supply demographic information*/
    pub demographic: Option<Demographic>,
    /**Reference Information (REF)
To specify identifying information*/
    pub reference: Vec<Reference>,
    /**Administrative Communications Contact (PER)
To identify a person or office to whom administrative communications should be directed*/
    pub administrative_communications_contact: Vec<AdministrativeCommunicationsContact>,
    /**Language Use (LUI)
To specify language, type of usage, and proficiency or fluency*/
    pub language_use: Option<LanguageUse>,
}
impl Loop2010 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop1000 {
    /**Individual or Organizational Name (NM1)
To supply the full name of an individual or organizational entity*/
    pub individual_or_organizational_name: IndividualOrOrganizationalName,
    /**Additional Name Information (N2)
To specify additional names*/
    pub additional_name: Vec<AdditionalName>,
    /**Party Location (N3)
To specify the location of the named party*/
    pub party_location: Vec<PartyLocation>,
    /**Geographic Location (N4)
To specify the geographic place of the named party*/
    pub geographic_location: Option<GeographicLocation>,
    /**Reference Information (REF)
To specify identifying information*/
    pub reference: Vec<Reference>,
    /**Administrative Communications Contact (PER)
To identify a person or office to whom administrative communications should be directed*/
    pub administrative_communications_contact: Vec<AdministrativeCommunicationsContact>,
}
impl Loop1000 {}