use serde::{Serialize, Deserialize};
use crate::numeric::Numeric;
use crate::fixed::Fixed;
use super::super::elements::ActionsIndicated;
/**To identify the type or transmission or both of paperwork or supporting information

See docs at <https://www.stedi.com/edi/x12/segment/PWK>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "PWK")]
pub struct Paperwork {
    /**PWK-01 (755)
Code indicating the title or contents of a document, report or supporting item*/
    pub report_type_code: Fixed<2>,
    /**PWK-02 (756)
Code specifying timing, transmission method or format by which reports are to be sent*/
    pub report_transmission_code: Option<String>,
    /**PWK-03 (757)
The number of copies of a report that should be sent to the addressee*/
    pub report_copies_needed: Option<Numeric<0>>,
    /**PWK-04 (98)
Code identifying an organizational entity, a physical location, property or an individual*/
    pub entity_identifier_code: Option<String>,
    /**PWK-05 (66)
Code specifying the system/method of code structure used for Identification Code (67)*/
    pub identification_code_qualifier: Option<String>,
    /**PWK-06 (67)
Code identifying a party or other code*/
    pub identification_code: Option<String>,
    /**PWK-07 (352)
A free-form description to clarify the related data elements and their content*/
    pub description: Option<String>,
    /**PWK-08 (C002)
Actions to be performed on the piece of paperwork identified*/
    pub actions_indicated: Option<ActionsIndicated>,
    /**PWK-09 (1525)
Code indicating a type of request*/
    pub request_category_code: Option<String>,
    /**PWK-10 (1270)
Code identifying a specific industry code list*/
    pub code_list_qualifier_code: Option<String>,
    /**PWK-11 (1271)
Code indicating a code from a specific industry code list*/
    pub industry_code: Option<String>,
}
impl Paperwork {}