use serde::{de, Deserialize, ser, Serialize};
use std::fmt;

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

/**I01

See docs at <https://www.stedi.com/edi/x12/element/I01>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AuthorizationInformationQualifier {
    ///00
    NoAuthorizationInformationPresent,
    ///01
    UcsCommunicationsId,
    ///02
    EdxCommunicationsId,
    ///03
    AdditionalDataIdentification,
    ///04
    RailCommunicationsId,
    ///05
    DepartmentOfDefenseCommunicationIdentifier,
    ///06
    UnitedStatesFederalGovernmentCommunicationIdentifier,
    ///07
    TruckCommunicationsId,
    ///08
    OceanCommunicationsId,
}
impl AuthorizationInformationQualifier {
    pub fn code(&self) -> &str {
        {
            use AuthorizationInformationQualifier::*;
            match self {
                NoAuthorizationInformationPresent => "00",
                UcsCommunicationsId => "01",
                EdxCommunicationsId => "02",
                AdditionalDataIdentification => "03",
                RailCommunicationsId => "04",
                DepartmentOfDefenseCommunicationIdentifier => "05",
                UnitedStatesFederalGovernmentCommunicationIdentifier => "06",
                TruckCommunicationsId => "07",
                OceanCommunicationsId => "08",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<AuthorizationInformationQualifier> {
        use AuthorizationInformationQualifier::*;
        match code {
            b"00" => Some(NoAuthorizationInformationPresent),
            b"01" => Some(UcsCommunicationsId),
            b"02" => Some(EdxCommunicationsId),
            b"03" => Some(AdditionalDataIdentification),
            b"04" => Some(RailCommunicationsId),
            b"05" => Some(DepartmentOfDefenseCommunicationIdentifier),
            b"06" => Some(UnitedStatesFederalGovernmentCommunicationIdentifier),
            b"07" => Some(TruckCommunicationsId),
            b"08" => Some(OceanCommunicationsId),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use AuthorizationInformationQualifier::*;
        match self {
            NoAuthorizationInformationPresent => "No Authorization Information Present ",
            UcsCommunicationsId => "UCS Communications ID",
            EdxCommunicationsId => "EDX Communications ID",
            AdditionalDataIdentification => "Additional Data Identification",
            RailCommunicationsId => "Rail Communications ID",
            DepartmentOfDefenseCommunicationIdentifier => {
                "Department of Defense Communication Identifier"
            }
            UnitedStatesFederalGovernmentCommunicationIdentifier => {
                "United States Federal Government Communication Identifier"
            }
            TruckCommunicationsId => "Truck Communications ID",
            OceanCommunicationsId => "Ocean Communications ID",
        }
    }
    fn from_description(description: &str) -> Option<AuthorizationInformationQualifier> {
        {
            use AuthorizationInformationQualifier::*;
            match description {
                "No Authorization Information Present " => {
                    Some(NoAuthorizationInformationPresent)
                }
                "UCS Communications ID" => Some(UcsCommunicationsId),
                "EDX Communications ID" => Some(EdxCommunicationsId),
                "Additional Data Identification" => Some(AdditionalDataIdentification),
                "Rail Communications ID" => Some(RailCommunicationsId),
                "Department of Defense Communication Identifier" => {
                    Some(DepartmentOfDefenseCommunicationIdentifier)
                }
                "United States Federal Government Communication Identifier" => {
                    Some(UnitedStatesFederalGovernmentCommunicationIdentifier)
                }
                "Truck Communications ID" => Some(TruckCommunicationsId),
                "Ocean Communications ID" => Some(OceanCommunicationsId),
                _ => None,
            }
        }
    }
}
impl Serialize for AuthorizationInformationQualifier {
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
    type Value = AuthorizationInformationQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Authorization Information Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        AuthorizationInformationQualifier::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Authorization Information Qualifier: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: de::Error, {
        AuthorizationInformationQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Authorization Information Qualifier: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for AuthorizationInformationQualifier {
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