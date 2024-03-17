use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
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