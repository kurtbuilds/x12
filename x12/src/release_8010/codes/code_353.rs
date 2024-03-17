use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**353

See docs at <https://www.stedi.com/edi/x12/element/353>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransactionSetPurposeCode {
    ///00 - Original
    Original,
    ///01 - Cancellation
    Cancellation,
    ///02 - Add
    Add,
    ///03 - Delete
    Delete,
    ///04 - Change
    Change,
    ///05 - Replace
    Replace,
    ///5C - Chargeable Resubmission
    ChargeableResubmission,
    ///06 - Confirmation
    Confirmation,
    ///07 - Duplicate
    Duplicate,
    ///08 - Status
    Status,
    ///10 - Not Found
    NotFound,
    ///11 - Response
    Response,
    ///12 - Not Processed
    NotProcessed,
    ///13 - Request
    Request,
    ///14 - Advance Notification
    AdvanceNotification,
    ///15 - Re-Submission
    ReSubmission,
    ///16 - Proposed
    Proposed,
    ///17 - Cancel, to be Reissued
    Code17,
    ///18 - Reissue
    Reissue,
    ///19 - Seller initiated change
    SellerInitiatedChange,
    ///20 - Final Transmission
    FinalTransmission,
    ///21 - Transaction on Hold
    TransactionOnHold,
    ///22 - Information Copy
    InformationCopy,
    ///24 - Draft
    Draft,
    ///25 - Incremental
    Incremental,
    ///26 - Replace - Specified Buyers Parts Only
    ReplaceSpecifiedBuyersPartsOnly,
    ///27 - Verify
    Verify,
    ///28 - Query
    Query,
    ///30 - Renewal
    Renewal,
    ///31 - Allowance/Addition
    AllowanceAddition,
    ///32 - Recovery/Deduction
    RecoveryDeduction,
    ///33 - Request for Payment
    RequestForPayment,
    ///34 - Payment Declined
    PaymentDeclined,
    ///35 - Request Authority
    RequestAuthority,
    ///36 - Authority to Deduct (Reply)
    Code36,
    ///37 - Authority Declined (Reply)
    Code37,
    ///38 - No Financial Value
    NoFinancialValue,
    ///39 - Response to Proposed Trip Plan
    ResponseToProposedTripPlan,
    ///40 - Commitment Advice
    CommitmentAdvice,
    ///41 - Corrected and Verified
    CorrectedAndVerified,
    ///42 - Temporary Record
    TemporaryRecord,
    ///43 - Request Permission to Service
    RequestPermissionToService,
    ///44 - Rejection
    Rejection,
    ///45 - Follow-up
    FollowUp,
    ///46 - Cancellation with Refund
    CancellationWithRefund,
    ///47 - Transfer
    Transfer,
    ///48 - Suspended
    Suspended,
    ///49 - Original - No Response Necessary
    OriginalNoResponseNecessary,
    ///50 - Register
    Register,
    ///51 - Historical Inquiry
    HistoricalInquiry,
    ///52 - Response to Historical Inquiry
    ResponseToHistoricalInquiry,
    ///53 - Completion
    Completion,
    ///54 - Approval
    Approval,
    ///55 - Excavation
    Excavation,
    ///56 - Expiration Notification
    ExpirationNotification,
    ///57 - Initial
    Initial,
    ///77 - Simulation Exercise
    SimulationExercise,
    ///CN - Completion Notification
    CompletionNotification,
    ///CO - Corrected
    Corrected,
    ///DA - Delegate to Alternative
    DelegateToAlternative,
    ///ED - Exhibit Disposition
    ExhibitDisposition,
    ///ER - Exhibit Receipt
    ExhibitReceipt,
    ///EX - Final Loading Configuration
    FinalLoadingConfiguration,
    ///FA - Forward to Action Point
    ForwardToActionPoint,
    ///FC - Forward to Contractor
    ForwardToContractor,
    ///FS - Forward to Support Point
    ForwardToSupportPoint,
    ///GR - Granted
    Granted,
    ///MD - Materiel Disposition
    MaterielDisposition,
    ///PR - Proposed Loading Configuration
    ProposedLoadingConfiguration,
    ///RH - Release Hold
    ReleaseHold,
    ///RO - Re-open
    ReOpen,
    ///RR - Reply Rebuttal
    ReplyRebuttal,
    ///RV - Revised Loading Configuration
    RevisedLoadingConfiguration,
    ///SB - Scan Based Trading
    ScanBasedTrading,
    ///SU - Status Update
    StatusUpdate,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl TransactionSetPurposeCode {
    pub fn code(&self) -> &str {
        {
            use TransactionSetPurposeCode::*;
            match self {
                Original => "00",
                Cancellation => "01",
                Add => "02",
                Delete => "03",
                Change => "04",
                Replace => "05",
                ChargeableResubmission => "5C",
                Confirmation => "06",
                Duplicate => "07",
                Status => "08",
                NotFound => "10",
                Response => "11",
                NotProcessed => "12",
                Request => "13",
                AdvanceNotification => "14",
                ReSubmission => "15",
                Proposed => "16",
                Code17 => "17",
                Reissue => "18",
                SellerInitiatedChange => "19",
                FinalTransmission => "20",
                TransactionOnHold => "21",
                InformationCopy => "22",
                Draft => "24",
                Incremental => "25",
                ReplaceSpecifiedBuyersPartsOnly => "26",
                Verify => "27",
                Query => "28",
                Renewal => "30",
                AllowanceAddition => "31",
                RecoveryDeduction => "32",
                RequestForPayment => "33",
                PaymentDeclined => "34",
                RequestAuthority => "35",
                Code36 => "36",
                Code37 => "37",
                NoFinancialValue => "38",
                ResponseToProposedTripPlan => "39",
                CommitmentAdvice => "40",
                CorrectedAndVerified => "41",
                TemporaryRecord => "42",
                RequestPermissionToService => "43",
                Rejection => "44",
                FollowUp => "45",
                CancellationWithRefund => "46",
                Transfer => "47",
                Suspended => "48",
                OriginalNoResponseNecessary => "49",
                Register => "50",
                HistoricalInquiry => "51",
                ResponseToHistoricalInquiry => "52",
                Completion => "53",
                Approval => "54",
                Excavation => "55",
                ExpirationNotification => "56",
                Initial => "57",
                SimulationExercise => "77",
                CompletionNotification => "CN",
                Corrected => "CO",
                DelegateToAlternative => "DA",
                ExhibitDisposition => "ED",
                ExhibitReceipt => "ER",
                FinalLoadingConfiguration => "EX",
                ForwardToActionPoint => "FA",
                ForwardToContractor => "FC",
                ForwardToSupportPoint => "FS",
                Granted => "GR",
                MaterielDisposition => "MD",
                ProposedLoadingConfiguration => "PR",
                ReleaseHold => "RH",
                ReOpen => "RO",
                ReplyRebuttal => "RR",
                RevisedLoadingConfiguration => "RV",
                ScanBasedTrading => "SB",
                StatusUpdate => "SU",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<TransactionSetPurposeCode> {
        use TransactionSetPurposeCode::*;
        match code {
            b"00" => Some(Original),
            b"01" => Some(Cancellation),
            b"02" => Some(Add),
            b"03" => Some(Delete),
            b"04" => Some(Change),
            b"05" => Some(Replace),
            b"5C" => Some(ChargeableResubmission),
            b"06" => Some(Confirmation),
            b"07" => Some(Duplicate),
            b"08" => Some(Status),
            b"10" => Some(NotFound),
            b"11" => Some(Response),
            b"12" => Some(NotProcessed),
            b"13" => Some(Request),
            b"14" => Some(AdvanceNotification),
            b"15" => Some(ReSubmission),
            b"16" => Some(Proposed),
            b"17" => Some(Code17),
            b"18" => Some(Reissue),
            b"19" => Some(SellerInitiatedChange),
            b"20" => Some(FinalTransmission),
            b"21" => Some(TransactionOnHold),
            b"22" => Some(InformationCopy),
            b"24" => Some(Draft),
            b"25" => Some(Incremental),
            b"26" => Some(ReplaceSpecifiedBuyersPartsOnly),
            b"27" => Some(Verify),
            b"28" => Some(Query),
            b"30" => Some(Renewal),
            b"31" => Some(AllowanceAddition),
            b"32" => Some(RecoveryDeduction),
            b"33" => Some(RequestForPayment),
            b"34" => Some(PaymentDeclined),
            b"35" => Some(RequestAuthority),
            b"36" => Some(Code36),
            b"37" => Some(Code37),
            b"38" => Some(NoFinancialValue),
            b"39" => Some(ResponseToProposedTripPlan),
            b"40" => Some(CommitmentAdvice),
            b"41" => Some(CorrectedAndVerified),
            b"42" => Some(TemporaryRecord),
            b"43" => Some(RequestPermissionToService),
            b"44" => Some(Rejection),
            b"45" => Some(FollowUp),
            b"46" => Some(CancellationWithRefund),
            b"47" => Some(Transfer),
            b"48" => Some(Suspended),
            b"49" => Some(OriginalNoResponseNecessary),
            b"50" => Some(Register),
            b"51" => Some(HistoricalInquiry),
            b"52" => Some(ResponseToHistoricalInquiry),
            b"53" => Some(Completion),
            b"54" => Some(Approval),
            b"55" => Some(Excavation),
            b"56" => Some(ExpirationNotification),
            b"57" => Some(Initial),
            b"77" => Some(SimulationExercise),
            b"CN" => Some(CompletionNotification),
            b"CO" => Some(Corrected),
            b"DA" => Some(DelegateToAlternative),
            b"ED" => Some(ExhibitDisposition),
            b"ER" => Some(ExhibitReceipt),
            b"EX" => Some(FinalLoadingConfiguration),
            b"FA" => Some(ForwardToActionPoint),
            b"FC" => Some(ForwardToContractor),
            b"FS" => Some(ForwardToSupportPoint),
            b"GR" => Some(Granted),
            b"MD" => Some(MaterielDisposition),
            b"PR" => Some(ProposedLoadingConfiguration),
            b"RH" => Some(ReleaseHold),
            b"RO" => Some(ReOpen),
            b"RR" => Some(ReplyRebuttal),
            b"RV" => Some(RevisedLoadingConfiguration),
            b"SB" => Some(ScanBasedTrading),
            b"SU" => Some(StatusUpdate),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use TransactionSetPurposeCode::*;
        match self {
            Original => "Original",
            Cancellation => "Cancellation",
            Add => "Add",
            Delete => "Delete",
            Change => "Change",
            Replace => "Replace",
            ChargeableResubmission => "Chargeable Resubmission",
            Confirmation => "Confirmation",
            Duplicate => "Duplicate",
            Status => "Status",
            NotFound => "Not Found",
            Response => "Response",
            NotProcessed => "Not Processed",
            Request => "Request",
            AdvanceNotification => "Advance Notification",
            ReSubmission => "Re-Submission",
            Proposed => "Proposed",
            Code17 => "Cancel, to be Reissued",
            Reissue => "Reissue",
            SellerInitiatedChange => "Seller initiated change",
            FinalTransmission => "Final Transmission",
            TransactionOnHold => "Transaction on Hold",
            InformationCopy => "Information Copy",
            Draft => "Draft",
            Incremental => "Incremental",
            ReplaceSpecifiedBuyersPartsOnly => "Replace - Specified Buyers Parts Only",
            Verify => "Verify",
            Query => "Query",
            Renewal => "Renewal",
            AllowanceAddition => "Allowance/Addition",
            RecoveryDeduction => "Recovery/Deduction",
            RequestForPayment => "Request for Payment",
            PaymentDeclined => "Payment Declined",
            RequestAuthority => "Request Authority",
            Code36 => "Authority to Deduct (Reply)",
            Code37 => "Authority Declined (Reply)",
            NoFinancialValue => "No Financial Value",
            ResponseToProposedTripPlan => "Response to Proposed Trip Plan",
            CommitmentAdvice => "Commitment Advice",
            CorrectedAndVerified => "Corrected and Verified",
            TemporaryRecord => "Temporary Record",
            RequestPermissionToService => "Request Permission to Service",
            Rejection => "Rejection",
            FollowUp => "Follow-up",
            CancellationWithRefund => "Cancellation with Refund",
            Transfer => "Transfer",
            Suspended => "Suspended",
            OriginalNoResponseNecessary => "Original - No Response Necessary",
            Register => "Register",
            HistoricalInquiry => "Historical Inquiry",
            ResponseToHistoricalInquiry => "Response to Historical Inquiry",
            Completion => "Completion",
            Approval => "Approval",
            Excavation => "Excavation",
            ExpirationNotification => "Expiration Notification",
            Initial => "Initial",
            SimulationExercise => "Simulation Exercise",
            CompletionNotification => "Completion Notification",
            Corrected => "Corrected",
            DelegateToAlternative => "Delegate to Alternative",
            ExhibitDisposition => "Exhibit Disposition",
            ExhibitReceipt => "Exhibit Receipt",
            FinalLoadingConfiguration => "Final Loading Configuration",
            ForwardToActionPoint => "Forward to Action Point",
            ForwardToContractor => "Forward to Contractor",
            ForwardToSupportPoint => "Forward to Support Point",
            Granted => "Granted",
            MaterielDisposition => "Materiel Disposition",
            ProposedLoadingConfiguration => "Proposed Loading Configuration",
            ReleaseHold => "Release Hold",
            ReOpen => "Re-open",
            ReplyRebuttal => "Reply Rebuttal",
            RevisedLoadingConfiguration => "Revised Loading Configuration",
            ScanBasedTrading => "Scan Based Trading",
            StatusUpdate => "Status Update",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<TransactionSetPurposeCode> {
        {
            use TransactionSetPurposeCode::*;
            match description {
                "Original" => Some(Original),
                "Cancellation" => Some(Cancellation),
                "Add" => Some(Add),
                "Delete" => Some(Delete),
                "Change" => Some(Change),
                "Replace" => Some(Replace),
                "Chargeable Resubmission" => Some(ChargeableResubmission),
                "Confirmation" => Some(Confirmation),
                "Duplicate" => Some(Duplicate),
                "Status" => Some(Status),
                "Not Found" => Some(NotFound),
                "Response" => Some(Response),
                "Not Processed" => Some(NotProcessed),
                "Request" => Some(Request),
                "Advance Notification" => Some(AdvanceNotification),
                "Re-Submission" => Some(ReSubmission),
                "Proposed" => Some(Proposed),
                "Cancel, to be Reissued" => Some(Code17),
                "Reissue" => Some(Reissue),
                "Seller initiated change" => Some(SellerInitiatedChange),
                "Final Transmission" => Some(FinalTransmission),
                "Transaction on Hold" => Some(TransactionOnHold),
                "Information Copy" => Some(InformationCopy),
                "Draft" => Some(Draft),
                "Incremental" => Some(Incremental),
                "Replace - Specified Buyers Parts Only" => {
                    Some(ReplaceSpecifiedBuyersPartsOnly)
                }
                "Verify" => Some(Verify),
                "Query" => Some(Query),
                "Renewal" => Some(Renewal),
                "Allowance/Addition" => Some(AllowanceAddition),
                "Recovery/Deduction" => Some(RecoveryDeduction),
                "Request for Payment" => Some(RequestForPayment),
                "Payment Declined" => Some(PaymentDeclined),
                "Request Authority" => Some(RequestAuthority),
                "Authority to Deduct (Reply)" => Some(Code36),
                "Authority Declined (Reply)" => Some(Code37),
                "No Financial Value" => Some(NoFinancialValue),
                "Response to Proposed Trip Plan" => Some(ResponseToProposedTripPlan),
                "Commitment Advice" => Some(CommitmentAdvice),
                "Corrected and Verified" => Some(CorrectedAndVerified),
                "Temporary Record" => Some(TemporaryRecord),
                "Request Permission to Service" => Some(RequestPermissionToService),
                "Rejection" => Some(Rejection),
                "Follow-up" => Some(FollowUp),
                "Cancellation with Refund" => Some(CancellationWithRefund),
                "Transfer" => Some(Transfer),
                "Suspended" => Some(Suspended),
                "Original - No Response Necessary" => Some(OriginalNoResponseNecessary),
                "Register" => Some(Register),
                "Historical Inquiry" => Some(HistoricalInquiry),
                "Response to Historical Inquiry" => Some(ResponseToHistoricalInquiry),
                "Completion" => Some(Completion),
                "Approval" => Some(Approval),
                "Excavation" => Some(Excavation),
                "Expiration Notification" => Some(ExpirationNotification),
                "Initial" => Some(Initial),
                "Simulation Exercise" => Some(SimulationExercise),
                "Completion Notification" => Some(CompletionNotification),
                "Corrected" => Some(Corrected),
                "Delegate to Alternative" => Some(DelegateToAlternative),
                "Exhibit Disposition" => Some(ExhibitDisposition),
                "Exhibit Receipt" => Some(ExhibitReceipt),
                "Final Loading Configuration" => Some(FinalLoadingConfiguration),
                "Forward to Action Point" => Some(ForwardToActionPoint),
                "Forward to Contractor" => Some(ForwardToContractor),
                "Forward to Support Point" => Some(ForwardToSupportPoint),
                "Granted" => Some(Granted),
                "Materiel Disposition" => Some(MaterielDisposition),
                "Proposed Loading Configuration" => Some(ProposedLoadingConfiguration),
                "Release Hold" => Some(ReleaseHold),
                "Re-open" => Some(ReOpen),
                "Reply Rebuttal" => Some(ReplyRebuttal),
                "Revised Loading Configuration" => Some(RevisedLoadingConfiguration),
                "Scan Based Trading" => Some(ScanBasedTrading),
                "Status Update" => Some(StatusUpdate),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for TransactionSetPurposeCode {
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
    type Value = TransactionSetPurposeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Transaction Set Purpose Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        TransactionSetPurposeCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Transaction Set Purpose Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        TransactionSetPurposeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Transaction Set Purpose Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for TransactionSetPurposeCode {
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