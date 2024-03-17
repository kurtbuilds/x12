use serde::{Serialize, Deserialize};
use crate::release_8010::codes::AuthorizationInformationQualifier;
use crate::fixed::Fixed;
/**To start and identify an interchange of zero or more functional groups and interchange-related control segments

See docs at <https://www.stedi.com/edi/x12/segment/ISA>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "ISA")]
pub struct InterchangeControlHeader {
    /**ISA-01 (I01)
Code identifying the type of information in the Authorization Information*/
    pub authorization_information_qualifier: AuthorizationInformationQualifier,
    /**ISA-02 (I02)
Information used for additional identification or authorization of the interchange sender or the data in the interchange; the type of information is set by the Authorization Information Qualifier (I01)*/
    pub authorization_information: Fixed<10>,
    /**ISA-03 (I03)
Code identifying the type of information in the Security Information*/
    pub security_information_qualifier: Fixed<2>,
    /**ISA-04 (I04)
This is used for identifying the security information about the interchange sender or the data in the interchange; the type of information is set by the Security Information Qualifier (I03)*/
    pub security_information: Fixed<10>,
    /**ISA-05 (I05)
Code indicating the system/method of code structure used to designate the sender or receiver ID element being qualified*/
    pub interchange_id_qualifier: Fixed<2>,
    /**ISA-06 (I06)
Identification code published by the sender for other parties to use as the receiver ID to route data to them; the sender always codes this value in the sender ID element*/
    pub interchange_sender_id: Fixed<15>,
    /**ISA-07 (I05)
Code indicating the system/method of code structure used to designate the sender or receiver ID element being qualified*/
    pub isa07: Fixed<2>,
    /**ISA-08 (I07)
Identification code published by the receiver of the data; When sending, it is used by the sender as their sending ID, thus other parties sending to them will use this as a receiving ID to route data to them*/
    pub interchange_receiver_id: Fixed<15>,
    /**ISA-09 (I08)
Date of the interchange*/
    pub interchange_date: Fixed<6>,
    /**ISA-10 (I09)
Time of the interchange*/
    pub interchange_time: Fixed<4>,
    /**ISA-11 (I65)
Type is not applicable; the repetition separator is a delimiter and not a data element; this field provides the delimiter used to separate repeated occurrences of a simple data element or a composite data structure; this value must be different than the data element separator, component element separator, and the segment terminator*/
    pub repetition_separator: Fixed<1>,
    /**ISA-12 (I11)
Code specifying the version number of the interchange control segments, the version of the data elements within the control segments, and the code values within those data elements.*/
    pub interchange_control_version_number_code: Fixed<5>,
    /**ISA-13 (I12)
A control number assigned by the interchange sender*/
    pub interchange_control_number: Fixed<9>,
    /**ISA-14 (I13)
Code indicating sender's request for an interchange acknowledgment*/
    pub acknowledgment_requested_code: Fixed<1>,
    /**ISA-15 (I14)
Code indicating whether data enclosed by this interchange envelope is test, production or information*/
    pub interchange_usage_indicator_code: Fixed<1>,
    /**ISA-16 (I15)
Type is not applicable; the component data element separator is a delimiter and not a data element; this field provides the delimiter used to separate component data elements within a composite data structure; this value must be different than the data element separator and the segment terminator*/
    pub component_data_element_separator: Fixed<1>,
}