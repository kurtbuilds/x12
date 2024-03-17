use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1351

See docs at <https://www.stedi.com/edi/x12/element/1351>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatientSignatureSourceCode {
    ///B - Signed signature authorization form or forms for both HCFA-1500 Claim Form block 12 and block 13 are on file
    SignedSignatureAuthorizationFormOrFormsForBothHcfa1500ClaimFormBlock12AndBlock13AreOnFile,
    ///C - Signed HCFA-1500 Claim Form on file
    SignedHcfa1500ClaimFormOnFile,
    ///M - Signed signature authorization form for HCFA-1500 Claim Form block 13 on file
    SignedSignatureAuthorizationFormForHcfa1500ClaimFormBlock13OnFile,
    ///P - Signature generated by provider because the patient was not physically present for services
    SignatureGeneratedByProviderBecauseThePatientWasNotPhysicallyPresentForServices,
    ///S - Signed signature authorization form for HCFA-1500 Claim Form block 12 on file
    SignedSignatureAuthorizationFormForHcfa1500ClaimFormBlock12OnFile,
}
impl PatientSignatureSourceCode {
    pub fn code(&self) -> &str {
        {
            use PatientSignatureSourceCode::*;
            match self {
                SignedSignatureAuthorizationFormOrFormsForBothHcfa1500ClaimFormBlock12AndBlock13AreOnFile => {
                    "B"
                }
                SignedHcfa1500ClaimFormOnFile => "C",
                SignedSignatureAuthorizationFormForHcfa1500ClaimFormBlock13OnFile => "M",
                SignatureGeneratedByProviderBecauseThePatientWasNotPhysicallyPresentForServices => {
                    "P"
                }
                SignedSignatureAuthorizationFormForHcfa1500ClaimFormBlock12OnFile => "S",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<PatientSignatureSourceCode> {
        use PatientSignatureSourceCode::*;
        match code {
            b"B" => {
                Some(
                    SignedSignatureAuthorizationFormOrFormsForBothHcfa1500ClaimFormBlock12AndBlock13AreOnFile,
                )
            }
            b"C" => Some(SignedHcfa1500ClaimFormOnFile),
            b"M" => {
                Some(SignedSignatureAuthorizationFormForHcfa1500ClaimFormBlock13OnFile)
            }
            b"P" => {
                Some(
                    SignatureGeneratedByProviderBecauseThePatientWasNotPhysicallyPresentForServices,
                )
            }
            b"S" => {
                Some(SignedSignatureAuthorizationFormForHcfa1500ClaimFormBlock12OnFile)
            }
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use PatientSignatureSourceCode::*;
        match self {
            SignedSignatureAuthorizationFormOrFormsForBothHcfa1500ClaimFormBlock12AndBlock13AreOnFile => {
                "Signed signature authorization form or forms for both HCFA-1500 Claim Form block 12 and block 13 are on file"
            }
            SignedHcfa1500ClaimFormOnFile => "Signed HCFA-1500 Claim Form on file",
            SignedSignatureAuthorizationFormForHcfa1500ClaimFormBlock13OnFile => {
                "Signed signature authorization form for HCFA-1500 Claim Form block 13 on file"
            }
            SignatureGeneratedByProviderBecauseThePatientWasNotPhysicallyPresentForServices => {
                "Signature generated by provider because the patient was not physically present for services"
            }
            SignedSignatureAuthorizationFormForHcfa1500ClaimFormBlock12OnFile => {
                "Signed signature authorization form for HCFA-1500 Claim Form block 12 on file"
            }
        }
    }
    fn from_description(description: &str) -> Option<PatientSignatureSourceCode> {
        {
            use PatientSignatureSourceCode::*;
            match description {
                "Signed signature authorization form or forms for both HCFA-1500 Claim Form block 12 and block 13 are on file" => {
                    Some(
                        SignedSignatureAuthorizationFormOrFormsForBothHcfa1500ClaimFormBlock12AndBlock13AreOnFile,
                    )
                }
                "Signed HCFA-1500 Claim Form on file" => {
                    Some(SignedHcfa1500ClaimFormOnFile)
                }
                "Signed signature authorization form for HCFA-1500 Claim Form block 13 on file" => {
                    Some(
                        SignedSignatureAuthorizationFormForHcfa1500ClaimFormBlock13OnFile,
                    )
                }
                "Signature generated by provider because the patient was not physically present for services" => {
                    Some(
                        SignatureGeneratedByProviderBecauseThePatientWasNotPhysicallyPresentForServices,
                    )
                }
                "Signed signature authorization form for HCFA-1500 Claim Form block 12 on file" => {
                    Some(
                        SignedSignatureAuthorizationFormForHcfa1500ClaimFormBlock12OnFile,
                    )
                }
                _ => None,
            }
        }
    }
}
impl Serialize for PatientSignatureSourceCode {
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
    type Value = PatientSignatureSourceCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Patient Signature Source Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PatientSignatureSourceCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Patient Signature Source Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PatientSignatureSourceCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Patient Signature Source Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for PatientSignatureSourceCode {
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