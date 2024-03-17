use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To transmit information in a free-form format, if necessary, for comment or special instruction

See docs at <https://www.stedi.com/edi/x12/segment/NTE>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "NTE")]
pub struct NoteSpecialInstruction {
    /**NTE-01 (363)
Code identifying the functional area or purpose for which the note applies*/
    pub note_reference_code: Option<Fixed<3>>,
    /**NTE-02 (352)
A free-form description to clarify the related data elements and their content*/
    pub description: String,
}
impl NoteSpecialInstruction {}