use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**704

See docs at <https://www.stedi.com/edi/x12-005010/element/704>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaperworkReportActionCode {
    ///1 - Used to Initiate This Report
    UsedToInitiateThisReport,
    ///2 - Report to be Filed
    ReportToBeFiled,
    ///3 - Complete
    Complete,
    ///4 - Notarize
    Notarize,
    ///5 - Generate
    Generate,
    ///6 - Letterhead Required
    LetterheadRequired,
    ///CH - Chamberize
    Chamberize,
    ///CO - Consularize
    Consularize,
    ///DM - Document is Missing
    DocumentIsMissing,
    ///NT - Generate in Language of Ultimate Destination
    GenerateInLanguageOfUltimateDestination,
    ///ON - Original Not Required (Copies Acceptable)
    CodeON,
    ///OR - Original Required
    OriginalRequired,
    ///PV - Provided
    Provided,
    ///SG - Sign (Power of Attorney)
    CodeSG,
}
impl PaperworkReportActionCode {
    pub fn code(&self) -> &str {
        {
            use PaperworkReportActionCode::*;
            match self {
                UsedToInitiateThisReport => "1",
                ReportToBeFiled => "2",
                Complete => "3",
                Notarize => "4",
                Generate => "5",
                LetterheadRequired => "6",
                Chamberize => "CH",
                Consularize => "CO",
                DocumentIsMissing => "DM",
                GenerateInLanguageOfUltimateDestination => "NT",
                CodeON => "ON",
                OriginalRequired => "OR",
                Provided => "PV",
                CodeSG => "SG",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<PaperworkReportActionCode> {
        use PaperworkReportActionCode::*;
        match code {
            b"1" => Some(UsedToInitiateThisReport),
            b"2" => Some(ReportToBeFiled),
            b"3" => Some(Complete),
            b"4" => Some(Notarize),
            b"5" => Some(Generate),
            b"6" => Some(LetterheadRequired),
            b"CH" => Some(Chamberize),
            b"CO" => Some(Consularize),
            b"DM" => Some(DocumentIsMissing),
            b"NT" => Some(GenerateInLanguageOfUltimateDestination),
            b"ON" => Some(CodeON),
            b"OR" => Some(OriginalRequired),
            b"PV" => Some(Provided),
            b"SG" => Some(CodeSG),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use PaperworkReportActionCode::*;
        match self {
            UsedToInitiateThisReport => "Used to Initiate This Report",
            ReportToBeFiled => "Report to be Filed",
            Complete => "Complete",
            Notarize => "Notarize",
            Generate => "Generate",
            LetterheadRequired => "Letterhead Required",
            Chamberize => "Chamberize",
            Consularize => "Consularize",
            DocumentIsMissing => "Document is Missing",
            GenerateInLanguageOfUltimateDestination => {
                "Generate in Language of Ultimate Destination"
            }
            CodeON => "Original Not Required (Copies Acceptable)",
            OriginalRequired => "Original Required",
            Provided => "Provided",
            CodeSG => "Sign (Power of Attorney)",
        }
    }
    fn from_description(description: &str) -> Option<PaperworkReportActionCode> {
        {
            use PaperworkReportActionCode::*;
            match description {
                "Used to Initiate This Report" => Some(UsedToInitiateThisReport),
                "Report to be Filed" => Some(ReportToBeFiled),
                "Complete" => Some(Complete),
                "Notarize" => Some(Notarize),
                "Generate" => Some(Generate),
                "Letterhead Required" => Some(LetterheadRequired),
                "Chamberize" => Some(Chamberize),
                "Consularize" => Some(Consularize),
                "Document is Missing" => Some(DocumentIsMissing),
                "Generate in Language of Ultimate Destination" => {
                    Some(GenerateInLanguageOfUltimateDestination)
                }
                "Original Not Required (Copies Acceptable)" => Some(CodeON),
                "Original Required" => Some(OriginalRequired),
                "Provided" => Some(Provided),
                "Sign (Power of Attorney)" => Some(CodeSG),
                _ => None,
            }
        }
    }
}
impl Serialize for PaperworkReportActionCode {
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
    type Value = PaperworkReportActionCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Paperwork/Report Action Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PaperworkReportActionCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Paperwork/Report Action Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PaperworkReportActionCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Paperwork/Report Action Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for PaperworkReportActionCode {
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