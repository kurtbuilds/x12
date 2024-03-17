use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**756

See docs at <https://www.stedi.com/edi/x12-005010/element/756>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReportTransmissionCode {
    ///1 - Summary Electronic Invoice
    SummaryElectronicInvoice,
    ///2 - Full Electronic Invoice
    FullElectronicInvoice,
    ///3 - Printed Invoice Sent by Mail
    PrintedInvoiceSentByMail,
    ///4 - Summary Electronic Invoice and Printed Invoice Sent by Mail
    SummaryElectronicInvoiceAndPrintedInvoiceSentByMail,
    ///5 - Full Electronic Invoice and Printed Invoice Sent by Mail
    FullElectronicInvoiceAndPrintedInvoiceSentByMail,
    ///6 - No paper Invoice
    NoPaperInvoice,
    ///7 - Summary Electronic Invoice and No paper Invoice
    SummaryElectronicInvoiceAndNoPaperInvoice,
    ///8 - Full Electronic Invoice and No Paper Invoice
    FullElectronicInvoiceAndNoPaperInvoice,
    ///9 - Electronic Mail
    ElectronicMail,
    ///AA - Available on Request at Provider Site
    AvailableOnRequestAtProviderSite,
    ///AB - Previously Submitted to Payer
    PreviouslySubmittedToPayer,
    ///AC - American College of Radiology/National Electronic Manufacturers Association (ACR/NEMA DICOM) Format
    CodeAC,
    ///AD - Certification Included in this Claim
    CertificationIncludedInThisClaim,
    ///AE - Electronically After Shipping
    ElectronicallyAfterShipping,
    ///AF - Narrative Segment Included in this Claim
    NarrativeSegmentIncludedInThisClaim,
    ///AG - No Documentation is Required
    NoDocumentationIsRequired,
    ///AM - By Mail After Shipping
    ByMailAfterShipping,
    ///AS - American Society for Testing and Materials Format (ASTM E1238)
    CodeAS,
    ///AT - American Society for Testing and Materials Format (ASTM E1384)
    CodeAT,
    ///AU - By Data Pattern
    ByDataPattern,
    ///BE - By Mail and Electronically
    ByMailAndElectronically,
    ///BM - By Mail
    ByMail,
    ///BW - Best Way (Sender's Option)
    CodeBW,
    ///CD - Courier Diskette
    CourierDiskette,
    ///CF - Courier
    Courier,
    ///CP - Courier Paper
    CourierPaper,
    ///CT - Courier Tape
    CourierTape,
    ///DA - Data
    Data,
    ///EL - Electronically Only
    ElectronicallyOnly,
    ///EM - E-Mail
    EMail,
    ///FT - File Transfer
    FileTransfer,
    ///FX - By Fax
    ByFax,
    ///GS - On General Services Administration (GSA) Form 10050
    CodeGS,
    ///HL - Health Industry Level 7 Interface Standards (HL/7) Format
    CodeHL,
    ///IA - Electronic Image
    ElectronicImage,
    ///IE - Electronically with Invoice
    ElectronicallyWithInvoice,
    ///IM - By Mail with Invoice
    ByMailWithInvoice,
    ///MB - Binary Image
    BinaryImage,
    ///MD - Mail Diskette
    MailDiskette,
    ///MN - Magnetic Media
    MagneticMedia,
    ///MP - Mail Paper
    MailPaper,
    ///MT - Mail Tape
    MailTape,
    ///NS - Not Specified
    NotSpecified,
    ///OL - On-Line
    OnLine,
    ///PO - Printed Original Required
    PrintedOriginalRequired,
    ///SE - Electronically Before Shipping
    ElectronicallyBeforeShipping,
    ///SM - By Mail Before Shipping
    ByMailBeforeShipping,
    ///SN - With Ship Notice
    WithShipNotice,
    ///SW - Society for Worldwide Interbank Financial Telecommunication (SWIFT)
    CodeSW,
    ///TA - Telex
    Telex,
    ///TE - Separately, Electronically at Time of Shipping
    CodeTE,
    ///TM - Separately, by Mail at Time of Shipping
    CodeTM,
    ///TX - Text
    Text,
    ///VO - Voice
    Voice,
    ///WS - With Shipment (With Package)
    CodeWS,
}
impl ReportTransmissionCode {
    pub fn code(&self) -> &str {
        {
            use ReportTransmissionCode::*;
            match self {
                SummaryElectronicInvoice => "1",
                FullElectronicInvoice => "2",
                PrintedInvoiceSentByMail => "3",
                SummaryElectronicInvoiceAndPrintedInvoiceSentByMail => "4",
                FullElectronicInvoiceAndPrintedInvoiceSentByMail => "5",
                NoPaperInvoice => "6",
                SummaryElectronicInvoiceAndNoPaperInvoice => "7",
                FullElectronicInvoiceAndNoPaperInvoice => "8",
                ElectronicMail => "9",
                AvailableOnRequestAtProviderSite => "AA",
                PreviouslySubmittedToPayer => "AB",
                CodeAC => "AC",
                CertificationIncludedInThisClaim => "AD",
                ElectronicallyAfterShipping => "AE",
                NarrativeSegmentIncludedInThisClaim => "AF",
                NoDocumentationIsRequired => "AG",
                ByMailAfterShipping => "AM",
                CodeAS => "AS",
                CodeAT => "AT",
                ByDataPattern => "AU",
                ByMailAndElectronically => "BE",
                ByMail => "BM",
                CodeBW => "BW",
                CourierDiskette => "CD",
                Courier => "CF",
                CourierPaper => "CP",
                CourierTape => "CT",
                Data => "DA",
                ElectronicallyOnly => "EL",
                EMail => "EM",
                FileTransfer => "FT",
                ByFax => "FX",
                CodeGS => "GS",
                CodeHL => "HL",
                ElectronicImage => "IA",
                ElectronicallyWithInvoice => "IE",
                ByMailWithInvoice => "IM",
                BinaryImage => "MB",
                MailDiskette => "MD",
                MagneticMedia => "MN",
                MailPaper => "MP",
                MailTape => "MT",
                NotSpecified => "NS",
                OnLine => "OL",
                PrintedOriginalRequired => "PO",
                ElectronicallyBeforeShipping => "SE",
                ByMailBeforeShipping => "SM",
                WithShipNotice => "SN",
                CodeSW => "SW",
                Telex => "TA",
                CodeTE => "TE",
                CodeTM => "TM",
                Text => "TX",
                Voice => "VO",
                CodeWS => "WS",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ReportTransmissionCode> {
        use ReportTransmissionCode::*;
        match code {
            b"1" => Some(SummaryElectronicInvoice),
            b"2" => Some(FullElectronicInvoice),
            b"3" => Some(PrintedInvoiceSentByMail),
            b"4" => Some(SummaryElectronicInvoiceAndPrintedInvoiceSentByMail),
            b"5" => Some(FullElectronicInvoiceAndPrintedInvoiceSentByMail),
            b"6" => Some(NoPaperInvoice),
            b"7" => Some(SummaryElectronicInvoiceAndNoPaperInvoice),
            b"8" => Some(FullElectronicInvoiceAndNoPaperInvoice),
            b"9" => Some(ElectronicMail),
            b"AA" => Some(AvailableOnRequestAtProviderSite),
            b"AB" => Some(PreviouslySubmittedToPayer),
            b"AC" => Some(CodeAC),
            b"AD" => Some(CertificationIncludedInThisClaim),
            b"AE" => Some(ElectronicallyAfterShipping),
            b"AF" => Some(NarrativeSegmentIncludedInThisClaim),
            b"AG" => Some(NoDocumentationIsRequired),
            b"AM" => Some(ByMailAfterShipping),
            b"AS" => Some(CodeAS),
            b"AT" => Some(CodeAT),
            b"AU" => Some(ByDataPattern),
            b"BE" => Some(ByMailAndElectronically),
            b"BM" => Some(ByMail),
            b"BW" => Some(CodeBW),
            b"CD" => Some(CourierDiskette),
            b"CF" => Some(Courier),
            b"CP" => Some(CourierPaper),
            b"CT" => Some(CourierTape),
            b"DA" => Some(Data),
            b"EL" => Some(ElectronicallyOnly),
            b"EM" => Some(EMail),
            b"FT" => Some(FileTransfer),
            b"FX" => Some(ByFax),
            b"GS" => Some(CodeGS),
            b"HL" => Some(CodeHL),
            b"IA" => Some(ElectronicImage),
            b"IE" => Some(ElectronicallyWithInvoice),
            b"IM" => Some(ByMailWithInvoice),
            b"MB" => Some(BinaryImage),
            b"MD" => Some(MailDiskette),
            b"MN" => Some(MagneticMedia),
            b"MP" => Some(MailPaper),
            b"MT" => Some(MailTape),
            b"NS" => Some(NotSpecified),
            b"OL" => Some(OnLine),
            b"PO" => Some(PrintedOriginalRequired),
            b"SE" => Some(ElectronicallyBeforeShipping),
            b"SM" => Some(ByMailBeforeShipping),
            b"SN" => Some(WithShipNotice),
            b"SW" => Some(CodeSW),
            b"TA" => Some(Telex),
            b"TE" => Some(CodeTE),
            b"TM" => Some(CodeTM),
            b"TX" => Some(Text),
            b"VO" => Some(Voice),
            b"WS" => Some(CodeWS),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ReportTransmissionCode::*;
        match self {
            SummaryElectronicInvoice => "Summary Electronic Invoice",
            FullElectronicInvoice => "Full Electronic Invoice",
            PrintedInvoiceSentByMail => "Printed Invoice Sent by Mail",
            SummaryElectronicInvoiceAndPrintedInvoiceSentByMail => {
                "Summary Electronic Invoice and Printed Invoice Sent by Mail"
            }
            FullElectronicInvoiceAndPrintedInvoiceSentByMail => {
                "Full Electronic Invoice and Printed Invoice Sent by Mail"
            }
            NoPaperInvoice => "No paper Invoice",
            SummaryElectronicInvoiceAndNoPaperInvoice => {
                "Summary Electronic Invoice and No paper Invoice"
            }
            FullElectronicInvoiceAndNoPaperInvoice => {
                "Full Electronic Invoice and No Paper Invoice"
            }
            ElectronicMail => "Electronic Mail",
            AvailableOnRequestAtProviderSite => "Available on Request at Provider Site",
            PreviouslySubmittedToPayer => "Previously Submitted to Payer",
            CodeAC => {
                "American College of Radiology/National Electronic Manufacturers Association (ACR/NEMA DICOM) Format"
            }
            CertificationIncludedInThisClaim => "Certification Included in this Claim",
            ElectronicallyAfterShipping => "Electronically After Shipping",
            NarrativeSegmentIncludedInThisClaim => {
                "Narrative Segment Included in this Claim"
            }
            NoDocumentationIsRequired => "No Documentation is Required",
            ByMailAfterShipping => "By Mail After Shipping",
            CodeAS => "American Society for Testing and Materials Format (ASTM E1238)",
            CodeAT => "American Society for Testing and Materials Format (ASTM E1384)",
            ByDataPattern => "By Data Pattern",
            ByMailAndElectronically => "By Mail and Electronically",
            ByMail => "By Mail",
            CodeBW => "Best Way (Sender's Option)",
            CourierDiskette => "Courier Diskette",
            Courier => "Courier",
            CourierPaper => "Courier Paper",
            CourierTape => "Courier Tape",
            Data => "Data",
            ElectronicallyOnly => "Electronically Only",
            EMail => "E-Mail",
            FileTransfer => "File Transfer",
            ByFax => "By Fax",
            CodeGS => "On General Services Administration (GSA) Form 10050",
            CodeHL => "Health Industry Level 7 Interface Standards (HL/7) Format",
            ElectronicImage => "Electronic Image",
            ElectronicallyWithInvoice => "Electronically with Invoice",
            ByMailWithInvoice => "By Mail with Invoice",
            BinaryImage => "Binary Image",
            MailDiskette => "Mail Diskette",
            MagneticMedia => "Magnetic Media",
            MailPaper => "Mail Paper",
            MailTape => "Mail Tape",
            NotSpecified => "Not Specified",
            OnLine => "On-Line",
            PrintedOriginalRequired => "Printed Original Required",
            ElectronicallyBeforeShipping => "Electronically Before Shipping",
            ByMailBeforeShipping => "By Mail Before Shipping",
            WithShipNotice => "With Ship Notice",
            CodeSW => {
                "Society for Worldwide Interbank Financial Telecommunication (SWIFT)"
            }
            Telex => "Telex",
            CodeTE => "Separately, Electronically at Time of Shipping",
            CodeTM => "Separately, by Mail at Time of Shipping",
            Text => "Text",
            Voice => "Voice",
            CodeWS => "With Shipment (With Package)",
        }
    }
    fn from_description(description: &str) -> Option<ReportTransmissionCode> {
        {
            use ReportTransmissionCode::*;
            match description {
                "Summary Electronic Invoice" => Some(SummaryElectronicInvoice),
                "Full Electronic Invoice" => Some(FullElectronicInvoice),
                "Printed Invoice Sent by Mail" => Some(PrintedInvoiceSentByMail),
                "Summary Electronic Invoice and Printed Invoice Sent by Mail" => {
                    Some(SummaryElectronicInvoiceAndPrintedInvoiceSentByMail)
                }
                "Full Electronic Invoice and Printed Invoice Sent by Mail" => {
                    Some(FullElectronicInvoiceAndPrintedInvoiceSentByMail)
                }
                "No paper Invoice" => Some(NoPaperInvoice),
                "Summary Electronic Invoice and No paper Invoice" => {
                    Some(SummaryElectronicInvoiceAndNoPaperInvoice)
                }
                "Full Electronic Invoice and No Paper Invoice" => {
                    Some(FullElectronicInvoiceAndNoPaperInvoice)
                }
                "Electronic Mail" => Some(ElectronicMail),
                "Available on Request at Provider Site" => {
                    Some(AvailableOnRequestAtProviderSite)
                }
                "Previously Submitted to Payer" => Some(PreviouslySubmittedToPayer),
                "American College of Radiology/National Electronic Manufacturers Association (ACR/NEMA DICOM) Format" => {
                    Some(CodeAC)
                }
                "Certification Included in this Claim" => {
                    Some(CertificationIncludedInThisClaim)
                }
                "Electronically After Shipping" => Some(ElectronicallyAfterShipping),
                "Narrative Segment Included in this Claim" => {
                    Some(NarrativeSegmentIncludedInThisClaim)
                }
                "No Documentation is Required" => Some(NoDocumentationIsRequired),
                "By Mail After Shipping" => Some(ByMailAfterShipping),
                "American Society for Testing and Materials Format (ASTM E1238)" => {
                    Some(CodeAS)
                }
                "American Society for Testing and Materials Format (ASTM E1384)" => {
                    Some(CodeAT)
                }
                "By Data Pattern" => Some(ByDataPattern),
                "By Mail and Electronically" => Some(ByMailAndElectronically),
                "By Mail" => Some(ByMail),
                "Best Way (Sender's Option)" => Some(CodeBW),
                "Courier Diskette" => Some(CourierDiskette),
                "Courier" => Some(Courier),
                "Courier Paper" => Some(CourierPaper),
                "Courier Tape" => Some(CourierTape),
                "Data" => Some(Data),
                "Electronically Only" => Some(ElectronicallyOnly),
                "E-Mail" => Some(EMail),
                "File Transfer" => Some(FileTransfer),
                "By Fax" => Some(ByFax),
                "On General Services Administration (GSA) Form 10050" => Some(CodeGS),
                "Health Industry Level 7 Interface Standards (HL/7) Format" => {
                    Some(CodeHL)
                }
                "Electronic Image" => Some(ElectronicImage),
                "Electronically with Invoice" => Some(ElectronicallyWithInvoice),
                "By Mail with Invoice" => Some(ByMailWithInvoice),
                "Binary Image" => Some(BinaryImage),
                "Mail Diskette" => Some(MailDiskette),
                "Magnetic Media" => Some(MagneticMedia),
                "Mail Paper" => Some(MailPaper),
                "Mail Tape" => Some(MailTape),
                "Not Specified" => Some(NotSpecified),
                "On-Line" => Some(OnLine),
                "Printed Original Required" => Some(PrintedOriginalRequired),
                "Electronically Before Shipping" => Some(ElectronicallyBeforeShipping),
                "By Mail Before Shipping" => Some(ByMailBeforeShipping),
                "With Ship Notice" => Some(WithShipNotice),
                "Society for Worldwide Interbank Financial Telecommunication (SWIFT)" => {
                    Some(CodeSW)
                }
                "Telex" => Some(Telex),
                "Separately, Electronically at Time of Shipping" => Some(CodeTE),
                "Separately, by Mail at Time of Shipping" => Some(CodeTM),
                "Text" => Some(Text),
                "Voice" => Some(Voice),
                "With Shipment (With Package)" => Some(CodeWS),
                _ => None,
            }
        }
    }
}
impl Serialize for ReportTransmissionCode {
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
    type Value = ReportTransmissionCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Report Transmission Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ReportTransmissionCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Report Transmission Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ReportTransmissionCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Report Transmission Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ReportTransmissionCode {
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