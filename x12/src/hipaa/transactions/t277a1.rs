use serde::{Serialize, Deserialize};
use crate::release_5010::segments::AdministrativeCommunicationsContact;
use crate::release_5010::segments::DateOrTimeOrPeriod;
use crate::release_5010::segments::TransactionSetHeader;
use crate::release_5010::segments::Reference;
use crate::release_5010::segments::Status;
use crate::release_5010::segments::TransactionSetTrailer;
use crate::release_5010::segments::Service;
use crate::release_5010::segments::Trace;
use crate::release_5010::segments::HierarchicalLevel;
use crate::release_5010::segments::BeginningOfHierarchicalTransaction;
use crate::release_5010::segments::IndividualOrOrganizationalName;
/**This X12 Transaction Set contains the format and establishes the data contents of the Health Care Information Status Notification Transaction Set (277) for use within the context of an Electronic Data Interchange (EDI) environment. This transaction set can be used by a health care payer or authorized agent to notify a provider, recipient, or authorized agent regarding the status of a health care claim or encounter or to request additional information from the provider regarding a health care claim or encounter, health care services review, or transactions related to the provisions of health care. This transaction set is not intended to replace the Health Care Claim Payment/Advice Transaction Set (835) and therefore, will not be used for account payment posting. The notification may be at a summary or service line detail level. The notification may be solicited or unsolicited.


See full docs at <https://www.stedi.com/edi/hipaa/transaction-set/277-A1>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct X212StatusRequestResponse {
    /**Transaction Set Header (ST)
To indicate the start of a transaction set and to assign a control number*/
    pub transaction_set_header: TransactionSetHeader,
    /**Beginning of Hierarchical Transaction (BHT)
To define the business hierarchical structure of the transaction set and identify the business application purpose and reference data, i.e., number, date, and time*/
    pub beginning_of_hierarchical_transaction: BeginningOfHierarchicalTransaction,
    ///0100
    pub _2000_a: Vec<Loop2000A>,
    /**Transaction Set Trailer (SE)
To indicate the end of the transaction set and provide the count of the transmitted segments (including the beginning (ST) and ending (SE) segments)*/
    pub transaction_set_trailer: TransactionSetTrailer,
}
impl X212StatusRequestResponse {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2000A {
    /**Information Source Level (HL)
To identify dependencies among and the content of hierarchically related groups of data segments*/
    pub information_source_level: HierarchicalLevel,
    ///0500
    pub _2100_a: Vec<Loop2100A>,
    ///0100
    pub _2000_b: Vec<Loop2000B>,
}
impl Loop2000A {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2000B {
    /**Information Receiver Level (HL)
To identify dependencies among and the content of hierarchically related groups of data segments*/
    pub information_receiver_level: HierarchicalLevel,
    ///0500
    pub _2100_b: Vec<Loop2100B>,
    ///0900
    pub _2200_b: Vec<Loop2200B>,
    ///0100
    pub _2000_c: Vec<Loop2000C>,
}
impl Loop2000B {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2000C {
    /**Service Provider Level (HL)
To identify dependencies among and the content of hierarchically related groups of data segments*/
    pub service_provider_level: HierarchicalLevel,
    ///0500
    pub _2100_c: Vec<Loop2100C>,
    ///0900
    pub _2200_c: Vec<Loop2200C>,
    ///0100
    pub _2000_d: Vec<Loop2000D>,
}
impl Loop2000C {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2000D {
    /**Subscriber Level (HL)
To identify dependencies among and the content of hierarchically related groups of data segments*/
    pub subscriber_level: HierarchicalLevel,
    ///0500
    pub _2100_d: Vec<Loop2100D>,
    ///0900
    pub _2200_d: Vec<Loop2200D>,
    ///0100
    pub _2000_e: Vec<Loop2000E>,
}
impl Loop2000D {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2000E {
    /**Dependent Level (HL)
To identify dependencies among and the content of hierarchically related groups of data segments*/
    pub dependent_level: HierarchicalLevel,
    ///0500
    pub _2100_e: Vec<Loop2100E>,
    ///0900
    pub _2200_e: Vec<Loop2200E>,
}
impl Loop2000E {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2200E {
    /**Claim Status Tracking Number (TRN)
To uniquely identify a transaction to an application*/
    pub claim_status_tracking_number: Trace,
    /**Claim Level Status Information (STC)
To report the status, required action, and paid information of a claim or service line*/
    pub claim_level_status: Status,
    /**Payer Claim Control Number (REF)
To specify identifying information*/
    pub payer_claim_control_number: Option<Reference>,
    /**Institutional Bill Type Identification (REF)
To specify identifying information*/
    pub institutional_bill_type_identification: Option<Reference>,
    /**Patient Control Number (REF)
To specify identifying information*/
    pub patient_control_number: Option<Reference>,
    /**Pharmacy Prescription Number (REF)
To specify identifying information*/
    pub pharmacy_prescription_number: Option<Reference>,
    /**Voucher Identifier (REF)
To specify identifying information*/
    pub voucher: Option<Reference>,
    /**Claim Identification Number For Clearinghouses and Other Transmission Intermediaries (REF)
To specify identifying information*/
    pub claim_identification_number_for_clearinghouses_and_other_transmission_intermediaries: Option<
        Reference,
    >,
    /**Claim Service Date (DTP)
To specify any or all of a date, a time, or a time period*/
    pub claim_service_date: Option<DateOrTimeOrPeriod>,
    ///1800
    pub _2220_e: Vec<Loop2220E>,
}
impl Loop2200E {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2220E {
    /**Service Line Information (SVC)
To supply payment and control information to a provider for a particular service*/
    pub service_line: Service,
    /**Service Line Status Information (STC)
To report the status, required action, and paid information of a claim or service line*/
    pub service_line_status: Status,
    /**Service Line Item Identification (REF)
To specify identifying information*/
    pub service_line_item_identification: Option<Reference>,
    /**Service Line Date (DTP)
To specify any or all of a date, a time, or a time period*/
    pub service_line_date: DateOrTimeOrPeriod,
}
impl Loop2220E {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2100E {
    /**Dependent Name (NM1)
To supply the full name of an individual or organizational entity*/
    pub dependent_name: IndividualOrOrganizationalName,
}
impl Loop2100E {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2200D {
    /**Claim Status Tracking Number (TRN)
To uniquely identify a transaction to an application*/
    pub claim_status_tracking_number: Trace,
    /**Claim Level Status Information (STC)
To report the status, required action, and paid information of a claim or service line*/
    pub claim_level_status: Status,
    /**Payer Claim Control Number (REF)
To specify identifying information*/
    pub payer_claim_control_number: Option<Reference>,
    /**Institutional Bill Type Identification (REF)
To specify identifying information*/
    pub institutional_bill_type_identification: Option<Reference>,
    /**Patient Control Number (REF)
To specify identifying information*/
    pub patient_control_number: Option<Reference>,
    /**Pharmacy Prescription Number (REF)
To specify identifying information*/
    pub pharmacy_prescription_number: Option<Reference>,
    /**Voucher Identifier (REF)
To specify identifying information*/
    pub voucher: Option<Reference>,
    /**Claim Identification Number For Clearinghouses and Other Transmission Intermediaries (REF)
To specify identifying information*/
    pub claim_identification_number_for_clearinghouses_and_other_transmission_intermediaries: Option<
        Reference,
    >,
    /**Claim Service Date (DTP)
To specify any or all of a date, a time, or a time period*/
    pub claim_service_date: Option<DateOrTimeOrPeriod>,
    ///1800
    pub _2220_d: Vec<Loop2220D>,
}
impl Loop2200D {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2220D {
    /**Service Line Information (SVC)
To supply payment and control information to a provider for a particular service*/
    pub service_line: Service,
    /**Service Line Status Information (STC)
To report the status, required action, and paid information of a claim or service line*/
    pub service_line_status: Status,
    /**Service Line Item Identification (REF)
To specify identifying information*/
    pub service_line_item_identification: Option<Reference>,
    /**Service Line Date (DTP)
To specify any or all of a date, a time, or a time period*/
    pub service_line_date: DateOrTimeOrPeriod,
}
impl Loop2220D {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2100D {
    /**Subscriber Name (NM1)
To supply the full name of an individual or organizational entity*/
    pub subscriber_name: IndividualOrOrganizationalName,
}
impl Loop2100D {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2200C {
    /**Provider of Service Trace Identifier (TRN)
To uniquely identify a transaction to an application*/
    pub provider_of_service_trace: Trace,
    /**Provider Status Information (STC)
To report the status, required action, and paid information of a claim or service line*/
    pub provider_status: Status,
}
impl Loop2200C {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2100C {
    /**Provider Name (NM1)
To supply the full name of an individual or organizational entity*/
    pub provider_name: IndividualOrOrganizationalName,
}
impl Loop2100C {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2200B {
    /**Information Receiver Trace Identifier (TRN)
To uniquely identify a transaction to an application*/
    pub information_receiver_trace: Trace,
    /**Information Receiver Status Information (STC)
To report the status, required action, and paid information of a claim or service line*/
    pub information_receiver_status: Status,
}
impl Loop2200B {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2100B {
    /**Information Receiver Name (NM1)
To supply the full name of an individual or organizational entity*/
    pub information_receiver_name: IndividualOrOrganizationalName,
}
impl Loop2100B {}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Loop2100A {
    /**Payer Name (NM1)
To supply the full name of an individual or organizational entity*/
    pub payer_name: IndividualOrOrganizationalName,
    /**Payer Contact Information (PER)
To identify a person or office to whom administrative communications should be directed*/
    pub payer_contact: Option<AdministrativeCommunicationsContact>,
}
impl Loop2100A {}