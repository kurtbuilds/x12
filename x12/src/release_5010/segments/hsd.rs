use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
use crate::numeric::Numeric;
/**To specify the delivery pattern of health care services

See docs at <https://www.stedi.com/edi/x12-005010/segment/HSD>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "HSD")]
pub struct HealthCareServicesDelivery {
    /**HSD-01 (673)
Code specifying the type of quantity*/
    pub quantity_qualifier: Option<Fixed<2>>,
    /**HSD-02 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**HSD-03 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub unit_or_basis_for_measurement_code: Option<Fixed<2>>,
    /**HSD-04 (1167)
To specify the sampling frequency in terms of a modulus of the Unit of Measure, e.g., every fifth bag, every 1.5 minutes*/
    pub sample_selection_modulus: Option<String>,
    /**HSD-05 (615)
Code defining periods*/
    pub time_period_qualifier: Option<String>,
    /**HSD-06 (616)
Total number of periods*/
    pub number_of_periods: Option<Numeric<0>>,
    /**HSD-07 (678)
Code which specifies the routine shipments, deliveries, or calendar pattern*/
    pub ship_delivery_or_calendar_pattern_code: Option<String>,
    /**HSD-08 (679)
Code which specifies the time for routine shipments or deliveries*/
    pub ship_delivery_pattern_time_code: Option<Fixed<1>>,
}
impl HealthCareServicesDelivery {}