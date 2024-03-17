use serde::{Serialize, Deserialize};
use super::super::codes::PaperworkReportActionCode;
/**Actions to be performed on the piece of paperwork identified

See docs at <https://www.stedi.com/edi/x12/element/C002>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C002")]
pub struct ActionsIndicated {
    /**C002-01 (704)
Code specifying how the paperwork or report that is identified in the PWK segment relates to the transaction set or to identify the action that is required*/
    pub actions_indicated: PaperworkReportActionCode,
    /**C002-02 (704)
Code specifying how the paperwork or report that is identified in the PWK segment relates to the transaction set or to identify the action that is required*/
    pub c002_02: Option<PaperworkReportActionCode>,
    /**C002-03 (704)
Code specifying how the paperwork or report that is identified in the PWK segment relates to the transaction set or to identify the action that is required*/
    pub c002_03: Option<PaperworkReportActionCode>,
    /**C002-04 (704)
Code specifying how the paperwork or report that is identified in the PWK segment relates to the transaction set or to identify the action that is required*/
    pub c002_04: Option<PaperworkReportActionCode>,
    /**C002-05 (704)
Code specifying how the paperwork or report that is identified in the PWK segment relates to the transaction set or to identify the action that is required*/
    pub c002_05: Option<PaperworkReportActionCode>,
}
impl ActionsIndicated {}