use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To define the business hierarchical structure of the transaction set and identify the business application purpose and reference data, i.e., number, date, and time

See docs at <https://www.stedi.com/edi/x12/segment/BHT>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "BHT")]
pub struct BeginningOfHierarchicalTransaction {
    /**BHT-01 (1005)
Code indicating the hierarchical application structure of a transaction set that utilizes the HL segment to define the structure of the transaction set*/
    pub hierarchical_structure_code: Fixed<4>,
    /**BHT-02 (353)
Code identifying purpose of transaction set*/
    pub transaction_set_purpose_code: Fixed<2>,
    /**BHT-03 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: Option<String>,
    /**BHT-04 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub date: Option<Fixed<8>>,
    /**BHT-05 (337)
Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)*/
    pub time: Option<String>,
    /**BHT-06 (640)
Code specifying the type of transaction*/
    pub transaction_type_code: Option<Fixed<2>>,
}
impl BeginningOfHierarchicalTransaction {}