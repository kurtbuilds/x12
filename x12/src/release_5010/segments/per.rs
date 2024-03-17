use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To identify a person or office to whom administrative communications should be directed

See docs at <https://www.stedi.com/edi/x12/segment/PER>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "PER")]
pub struct AdministrativeCommunicationsContact {
    /**PER-01 (366)
Code identifying the major duty or responsibility of the person or group named*/
    pub contact_function_code: Fixed<2>,
    /**PER-02 (93)
Free-form name*/
    pub name: Option<String>,
    /**PER-03 (365)
Code identifying the type of communication number*/
    pub communication_number_qualifier: Option<Fixed<2>>,
    /**PER-04 (364)
Complete communications number including country or area code when applicable*/
    pub communication_number: Option<String>,
    /**PER-05 (365)
Code identifying the type of communication number*/
    pub per05: Option<Fixed<2>>,
    /**PER-06 (364)
Complete communications number including country or area code when applicable*/
    pub per06: Option<String>,
    /**PER-07 (365)
Code identifying the type of communication number*/
    pub per07: Option<Fixed<2>>,
    /**PER-08 (364)
Complete communications number including country or area code when applicable*/
    pub per08: Option<String>,
    /**PER-09 (443)
Additional reference number or description to clarify a contact number*/
    pub contact_inquiry_reference: Option<String>,
}
impl AdministrativeCommunicationsContact {}