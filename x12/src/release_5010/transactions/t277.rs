use serde::{Serialize, Deserialize};
use crate::release_5010::segments::Paperwork;
use crate::release_5010::segments::Trace;
use crate::release_5010::segments::Reference;
use crate::release_5010::segments::Service;
use crate::release_5010::segments::HierarchicalLevel;
use crate::release_5010::segments::GeographicLocation;
use crate::release_5010::segments::BeginningOfHierarchicalTransaction;
use crate::release_5010::segments::DateOrTimeOrPeriod;
use crate::release_5010::segments::AdditionalName;
use crate::release_5010::segments::Subscriber;
use crate::release_5010::segments::Patient;
use crate::release_5010::segments::AdministrativeCommunicationsContact;
use crate::release_5010::segments::PartyIdentification;
use crate::release_5010::segments::IndividualOrOrganizationalName;
use crate::release_5010::segments::Status;
use crate::release_5010::segments::TransactionSetTrailer;
use crate::release_5010::segments::MonetaryAmount;
use crate::release_5010::segments::PartyLocation;
use crate::release_5010::segments::Quantity;
use crate::release_5010::segments::TransactionSetHeader;
use crate::release_5010::segments::Demographic;
/**This X12 Transaction Set contains the format and establishes the data contents of the Health Care Information Status Notification Transaction Set (277) for use within the context of an Electronic Data Interchange (EDI) environment.
This transaction set can be used by a health care payer or authorized agent to notify a provider, recipient, or authorized agent regarding the status of a health care claim or encounter or to request additional information from the provider regarding a health care claim or encounter, health care services review, or transactions related to the provisions of health care. This transaction set is not intended to replace the Health Care Claim Payment/Advice Transaction Set (835) and therefore, will not be used for account payment posting. The notification may be at a summary or service line detail level. The notification may be solicited or unsolicited.


See full docs at <https://www.stedi.com/edi/release_5010/transaction-set/277>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct HealthCareInformationStatusNotification {
    /**Transaction Set Header (ST)
To indicate the start of a transaction set and to assign a control number*/
    pub transaction_set_header: TransactionSetHeader,
    /**Beginning of Hierarchical Transaction (BHT)
To define the business hierarchical structure of the transaction set and identify the business application purpose and reference data, i.e., number, date, and time*/
    pub beginning_of_hierarchical_transaction: BeginningOfHierarchicalTransaction,
    /**Reference Information (REF)
To specify identifying information*/
    pub reference: Vec<Reference>,
    ///0400
    pub _1000: Vec<Loop1000>,
    ///0100
    pub _2000: Vec<Loop2000>,
    /**Transaction Set Trailer (SE)
To indicate the end of the transaction set and provide the count of the transmitted segments (including the beginning (ST) and ending (SE) segments)*/
    pub transaction_set_trailer: TransactionSetTrailer,
}
impl HealthCareInformationStatusNotification {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2000 {
    /**Hierarchical Level (HL)
To identify dependencies among and the content of hierarchically related groups of data segments*/
    pub hierarchical_level: HierarchicalLevel,
    /**Subscriber Information (SBR)
To record information specific to the primary insured and the insurance carrier for that insured*/
    pub subscriber: Option<Subscriber>,
    /**Patient Information (PAT)
To supply patient information*/
    pub patient: Option<Patient>,
    /**Demographic Information (DMG)
To supply demographic information*/
    pub demographic: Option<Demographic>,
    ///0500
    pub _2100: Vec<Loop2100>,
    ///0900
    pub _2200: Vec<Loop2200>,
}
impl Loop2000 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2200 {
    /**Trace (TRN)
To uniquely identify a transaction to an application*/
    pub trace: Trace,
    /**Status Information (STC)
To report the status, required action, and paid information of a claim or service line*/
    pub status: Vec<Status>,
    /**Reference Information (REF)
To specify identifying information*/
    pub reference: Vec<Reference>,
    /**Date or Time or Period (DTP)
To specify any or all of a date, a time, or a time period*/
    pub date_or_time_or_period: Vec<DateOrTimeOrPeriod>,
    /**Quantity Information (QTY)
To specify quantity information*/
    pub quantity: Vec<Quantity>,
    /**Monetary Amount Information (AMT)
To indicate the total monetary amount*/
    pub monetary_amount: Vec<MonetaryAmount>,
    ///1300
    pub _2210: Vec<Loop2210>,
    ///1800
    pub _2220: Vec<Loop2220>,
}
impl Loop2200 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2220 {
    /**Service Information (SVC)
To supply payment and control information to a provider for a particular service*/
    pub service: Service,
    /**Status Information (STC)
To report the status, required action, and paid information of a claim or service line*/
    pub status: Vec<Status>,
    /**Reference Information (REF)
To specify identifying information*/
    pub reference: Option<Reference>,
    /**Date or Time or Period (DTP)
To specify any or all of a date, a time, or a time period*/
    pub date_or_time_or_period: Option<DateOrTimeOrPeriod>,
    ///2200
    pub _2225: Vec<Loop2225>,
}
impl Loop2220 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2225 {
    /**Paperwork (PWK)
To identify the type or transmission or both of paperwork or supporting information*/
    pub paperwork: Paperwork,
    /**Administrative Communications Contact (PER)
To identify a person or office to whom administrative communications should be directed*/
    pub administrative_communications_contact: Option<
        AdministrativeCommunicationsContact,
    >,
    /**Party Identification (N1)
To identify a party by type of organization, name, and code*/
    pub party_identification: Option<PartyIdentification>,
    /**Party Location (N3)
To specify the location of the named party*/
    pub party_location: Option<PartyLocation>,
    /**Geographic Location (N4)
To specify the geographic place of the named party*/
    pub geographic_location: Option<GeographicLocation>,
}
impl Loop2225 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2210 {
    /**Paperwork (PWK)
To identify the type or transmission or both of paperwork or supporting information*/
    pub paperwork: Paperwork,
    /**Administrative Communications Contact (PER)
To identify a person or office to whom administrative communications should be directed*/
    pub administrative_communications_contact: Option<
        AdministrativeCommunicationsContact,
    >,
    /**Party Identification (N1)
To identify a party by type of organization, name, and code*/
    pub party_identification: Option<PartyIdentification>,
    /**Party Location (N3)
To specify the location of the named party*/
    pub party_location: Option<PartyLocation>,
    /**Geographic Location (N4)
To specify the geographic place of the named party*/
    pub geographic_location: Option<GeographicLocation>,
}
impl Loop2210 {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2100 {
    /**Individual or Organizational Name (NM1)
To supply the full name of an individual or organizational entity*/
    pub individual_or_organizational_name: IndividualOrOrganizationalName,
    /**Party Location (N3)
To specify the location of the named party*/
    pub party_location: Vec<PartyLocation>,
    /**Geographic Location (N4)
To specify the geographic place of the named party*/
    pub geographic_location: Option<GeographicLocation>,
    /**Administrative Communications Contact (PER)
To identify a person or office to whom administrative communications should be directed*/
    pub administrative_communications_contact: Option<
        AdministrativeCommunicationsContact,
    >,
}
impl Loop2100 {}
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
    pub administrative_communications_contact: Option<
        AdministrativeCommunicationsContact,
    >,
}
impl Loop1000 {}