use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To specify the geographic place of the named party

See docs at <https://www.stedi.com/edi/x12/segment/N4>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "N4")]
pub struct GeographicLocation {
    /**N4-01 (19)
Free-form text for city name*/
    pub city_name: Option<String>,
    /**N4-02 (156)
Code specifying the Standard State/Province as defined by appropriate government agency*/
    pub state_or_province_code: Option<Fixed<2>>,
    /**N4-03 (116)
Code specifying international postal zone code excluding punctuation and blanks (zip code for United States)*/
    pub postal_code: Option<String>,
    /**N4-04 (26)
Code identifying the country*/
    pub country_code: Option<String>,
    /**N4-05 (309)
Code identifying type of location*/
    pub location_qualifier: Option<String>,
    /**N4-06 (310)
Code which identifies a specific location*/
    pub location: Option<String>,
    /**N4-07 (1715)
Code identifying the country subdivision*/
    pub country_subdivision_code: Option<String>,
    /**N4-08 (1702)
A postal code value including punctuation, blanks and other characters as defined by the postal authority*/
    pub postal_code_formatted: Option<String>,
}
impl GeographicLocation {}