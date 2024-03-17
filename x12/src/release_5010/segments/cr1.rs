use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply information related to the ambulance service rendered to a patient

See docs at <https://www.stedi.com/edi/x12/segment/CR1>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CR1")]
pub struct AmbulanceCertification {
    /**CR1-01 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub unit_or_basis_for_measurement_code: Option<Fixed<2>>,
    /**CR1-02 (81)
Numeric value of weight*/
    pub weight: Option<String>,
    /**CR1-03 (1316)
Code indicating the type of ambulance transport*/
    pub ambulance_transport_code: Option<Fixed<1>>,
    /**CR1-04 (1317)
Code indicating the reason for ambulance transport*/
    pub ambulance_transport_reason_code: Option<Fixed<1>>,
    /**CR1-05 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub cr1_05: Option<Fixed<2>>,
    /**CR1-06 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**CR1-07 (166)
Address information*/
    pub address: Option<String>,
    /**CR1-08 (166)
Address information*/
    pub cr1_08: Option<String>,
    /**CR1-09 (352)
A free-form description to clarify the related data elements and their content*/
    pub description: Option<String>,
    /**CR1-10 (352)
A free-form description to clarify the related data elements and their content*/
    pub cr1_10: Option<String>,
}
impl AmbulanceCertification {}