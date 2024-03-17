use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1029

See docs at <https://www.stedi.com/edi/x12-005010/element/1029>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClaimStatusCode {
    ///1 - Processed as Primary
    ProcessedAsPrimary,
    ///2 - Processed as Secondary
    ProcessedAsSecondary,
    ///3 - Processed as Tertiary
    ProcessedAsTertiary,
    ///4 - Denied
    Denied,
    ///5 - Pended
    Pended,
    ///6 - Approved as amended
    ApprovedAsAmended,
    ///7 - Approved as submitted
    ApprovedAsSubmitted,
    ///8 - Cancelled due to inactivity
    CancelledDueToInactivity,
    ///9 - Pending - under investigation
    PendingUnderInvestigation,
    ///10 - Received, but not in process
    Code10,
    ///11 - Rejected, duplicate claim
    Code11,
    ///12 - Rejected, please resubmit with corrections
    Code12,
    ///13 - Suspended
    Suspended,
    ///14 - Suspended - incomplete claim
    SuspendedIncompleteClaim,
    ///15 - Suspended - investigation with field
    SuspendedInvestigationWithField,
    ///16 - Suspended - return with material
    SuspendedReturnWithMaterial,
    ///17 - Suspended - review pending
    SuspendedReviewPending,
    ///18 - Suspended Product Registration
    SuspendedProductRegistration,
    ///19 - Processed as Primary, Forwarded to Additional Payer(s)
    Code19,
    ///20 - Processed as Secondary, Forwarded to Additional Payer(s)
    Code20,
    ///21 - Processed as Tertiary, Forwarded to Additional Payer(s)
    Code21,
    ///22 - Reversal of Previous Payment
    ReversalOfPreviousPayment,
    ///23 - Not Our Claim, Forwarded to Additional Payer(s)
    Code23,
    ///24 - Transferred to Proper Carrier
    TransferredToProperCarrier,
    ///25 - Predetermination Pricing Only - No Payment
    PredeterminationPricingOnlyNoPayment,
    ///26 - Documentation Claim - No Payment Associated
    DocumentationClaimNoPaymentAssociated,
    ///27 - Reviewed
    Reviewed,
    ///28 - Repriced
    Repriced,
    ///29 - Audited
    Audited,
    ///30 - Processed as Conditional
    ProcessedAsConditional,
    ///AD - Additional
    Additional,
    ///AP - Appealed
    Appealed,
    ///CC - Weekly Certification
    WeeklyCertification,
    ///CL - Closed
    Closed,
    ///CP - Open
    Open,
    ///I - Initial
    Initial,
    ///RA - Reaudited
    Reaudited,
    ///RB - Reissue
    Reissue,
    ///RC - Reopened and Closed
    ReopenedAndClosed,
    ///RD - Redetermination
    Redetermination,
    ///RO - Reopened
    Reopened,
}
impl ClaimStatusCode {
    pub fn code(&self) -> &str {
        {
            use ClaimStatusCode::*;
            match self {
                ProcessedAsPrimary => "1",
                ProcessedAsSecondary => "2",
                ProcessedAsTertiary => "3",
                Denied => "4",
                Pended => "5",
                ApprovedAsAmended => "6",
                ApprovedAsSubmitted => "7",
                CancelledDueToInactivity => "8",
                PendingUnderInvestigation => "9",
                Code10 => "10",
                Code11 => "11",
                Code12 => "12",
                Suspended => "13",
                SuspendedIncompleteClaim => "14",
                SuspendedInvestigationWithField => "15",
                SuspendedReturnWithMaterial => "16",
                SuspendedReviewPending => "17",
                SuspendedProductRegistration => "18",
                Code19 => "19",
                Code20 => "20",
                Code21 => "21",
                ReversalOfPreviousPayment => "22",
                Code23 => "23",
                TransferredToProperCarrier => "24",
                PredeterminationPricingOnlyNoPayment => "25",
                DocumentationClaimNoPaymentAssociated => "26",
                Reviewed => "27",
                Repriced => "28",
                Audited => "29",
                ProcessedAsConditional => "30",
                Additional => "AD",
                Appealed => "AP",
                WeeklyCertification => "CC",
                Closed => "CL",
                Open => "CP",
                Initial => "I",
                Reaudited => "RA",
                Reissue => "RB",
                ReopenedAndClosed => "RC",
                Redetermination => "RD",
                Reopened => "RO",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ClaimStatusCode> {
        use ClaimStatusCode::*;
        match code {
            b"1" => Some(ProcessedAsPrimary),
            b"2" => Some(ProcessedAsSecondary),
            b"3" => Some(ProcessedAsTertiary),
            b"4" => Some(Denied),
            b"5" => Some(Pended),
            b"6" => Some(ApprovedAsAmended),
            b"7" => Some(ApprovedAsSubmitted),
            b"8" => Some(CancelledDueToInactivity),
            b"9" => Some(PendingUnderInvestigation),
            b"10" => Some(Code10),
            b"11" => Some(Code11),
            b"12" => Some(Code12),
            b"13" => Some(Suspended),
            b"14" => Some(SuspendedIncompleteClaim),
            b"15" => Some(SuspendedInvestigationWithField),
            b"16" => Some(SuspendedReturnWithMaterial),
            b"17" => Some(SuspendedReviewPending),
            b"18" => Some(SuspendedProductRegistration),
            b"19" => Some(Code19),
            b"20" => Some(Code20),
            b"21" => Some(Code21),
            b"22" => Some(ReversalOfPreviousPayment),
            b"23" => Some(Code23),
            b"24" => Some(TransferredToProperCarrier),
            b"25" => Some(PredeterminationPricingOnlyNoPayment),
            b"26" => Some(DocumentationClaimNoPaymentAssociated),
            b"27" => Some(Reviewed),
            b"28" => Some(Repriced),
            b"29" => Some(Audited),
            b"30" => Some(ProcessedAsConditional),
            b"AD" => Some(Additional),
            b"AP" => Some(Appealed),
            b"CC" => Some(WeeklyCertification),
            b"CL" => Some(Closed),
            b"CP" => Some(Open),
            b"I" => Some(Initial),
            b"RA" => Some(Reaudited),
            b"RB" => Some(Reissue),
            b"RC" => Some(ReopenedAndClosed),
            b"RD" => Some(Redetermination),
            b"RO" => Some(Reopened),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ClaimStatusCode::*;
        match self {
            ProcessedAsPrimary => "Processed as Primary",
            ProcessedAsSecondary => "Processed as Secondary",
            ProcessedAsTertiary => "Processed as Tertiary",
            Denied => "Denied",
            Pended => "Pended",
            ApprovedAsAmended => "Approved as amended",
            ApprovedAsSubmitted => "Approved as submitted",
            CancelledDueToInactivity => "Cancelled due to inactivity",
            PendingUnderInvestigation => "Pending - under investigation",
            Code10 => "Received, but not in process",
            Code11 => "Rejected, duplicate claim",
            Code12 => "Rejected, please resubmit with corrections",
            Suspended => "Suspended",
            SuspendedIncompleteClaim => "Suspended - incomplete claim",
            SuspendedInvestigationWithField => "Suspended - investigation with field",
            SuspendedReturnWithMaterial => "Suspended - return with material",
            SuspendedReviewPending => "Suspended - review pending",
            SuspendedProductRegistration => "Suspended Product Registration",
            Code19 => "Processed as Primary, Forwarded to Additional Payer(s)",
            Code20 => "Processed as Secondary, Forwarded to Additional Payer(s)",
            Code21 => "Processed as Tertiary, Forwarded to Additional Payer(s)",
            ReversalOfPreviousPayment => "Reversal of Previous Payment",
            Code23 => "Not Our Claim, Forwarded to Additional Payer(s)",
            TransferredToProperCarrier => "Transferred to Proper Carrier",
            PredeterminationPricingOnlyNoPayment => {
                "Predetermination Pricing Only - No Payment"
            }
            DocumentationClaimNoPaymentAssociated => {
                "Documentation Claim - No Payment Associated"
            }
            Reviewed => "Reviewed",
            Repriced => "Repriced",
            Audited => "Audited",
            ProcessedAsConditional => "Processed as Conditional",
            Additional => "Additional",
            Appealed => "Appealed",
            WeeklyCertification => "Weekly Certification",
            Closed => "Closed",
            Open => "Open",
            Initial => "Initial",
            Reaudited => "Reaudited",
            Reissue => "Reissue",
            ReopenedAndClosed => "Reopened and Closed",
            Redetermination => "Redetermination",
            Reopened => "Reopened",
        }
    }
    fn from_description(description: &str) -> Option<ClaimStatusCode> {
        {
            use ClaimStatusCode::*;
            match description {
                "Processed as Primary" => Some(ProcessedAsPrimary),
                "Processed as Secondary" => Some(ProcessedAsSecondary),
                "Processed as Tertiary" => Some(ProcessedAsTertiary),
                "Denied" => Some(Denied),
                "Pended" => Some(Pended),
                "Approved as amended" => Some(ApprovedAsAmended),
                "Approved as submitted" => Some(ApprovedAsSubmitted),
                "Cancelled due to inactivity" => Some(CancelledDueToInactivity),
                "Pending - under investigation" => Some(PendingUnderInvestigation),
                "Received, but not in process" => Some(Code10),
                "Rejected, duplicate claim" => Some(Code11),
                "Rejected, please resubmit with corrections" => Some(Code12),
                "Suspended" => Some(Suspended),
                "Suspended - incomplete claim" => Some(SuspendedIncompleteClaim),
                "Suspended - investigation with field" => {
                    Some(SuspendedInvestigationWithField)
                }
                "Suspended - return with material" => Some(SuspendedReturnWithMaterial),
                "Suspended - review pending" => Some(SuspendedReviewPending),
                "Suspended Product Registration" => Some(SuspendedProductRegistration),
                "Processed as Primary, Forwarded to Additional Payer(s)" => Some(Code19),
                "Processed as Secondary, Forwarded to Additional Payer(s)" => {
                    Some(Code20)
                }
                "Processed as Tertiary, Forwarded to Additional Payer(s)" => Some(Code21),
                "Reversal of Previous Payment" => Some(ReversalOfPreviousPayment),
                "Not Our Claim, Forwarded to Additional Payer(s)" => Some(Code23),
                "Transferred to Proper Carrier" => Some(TransferredToProperCarrier),
                "Predetermination Pricing Only - No Payment" => {
                    Some(PredeterminationPricingOnlyNoPayment)
                }
                "Documentation Claim - No Payment Associated" => {
                    Some(DocumentationClaimNoPaymentAssociated)
                }
                "Reviewed" => Some(Reviewed),
                "Repriced" => Some(Repriced),
                "Audited" => Some(Audited),
                "Processed as Conditional" => Some(ProcessedAsConditional),
                "Additional" => Some(Additional),
                "Appealed" => Some(Appealed),
                "Weekly Certification" => Some(WeeklyCertification),
                "Closed" => Some(Closed),
                "Open" => Some(Open),
                "Initial" => Some(Initial),
                "Reaudited" => Some(Reaudited),
                "Reissue" => Some(Reissue),
                "Reopened and Closed" => Some(ReopenedAndClosed),
                "Redetermination" => Some(Redetermination),
                "Reopened" => Some(Reopened),
                _ => None,
            }
        }
    }
}
impl Serialize for ClaimStatusCode {
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
    type Value = ClaimStatusCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Claim Status Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ClaimStatusCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Claim Status Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ClaimStatusCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Claim Status Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for ClaimStatusCode {
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