use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**659

See docs at <https://www.stedi.com/edi/x12/element/659>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BasisOfVerificationCode {
    ///1 - Birth Certificate
    BirthCertificate,
    ///2 - Passport
    Passport,
    ///3 - Hospital Certificate
    HospitalCertificate,
    ///4 - Affidavit
    Affidavit,
    ///5 - Immigration Document
    ImmigrationDocument,
    ///6 - Baptismal or Church Certificate
    BaptismalOrChurchCertificate,
    ///7 - Physician's Certificate
    PhysiciansCertificate,
    ///8 - Undocumented
    Undocumented,
    ///9 - Driver's License
    DriversLicense,
    ///A - Photo ID
    PhotoId,
    ///B - Social Insurance Certificate
    SocialInsuranceCertificate,
    ///C - US Passport
    UsPassport,
    ///D - Certificate of US Citizenship
    CertificateOfUsCitizenship,
    ///E - Certificate of Naturalization
    CertificateOfNaturalization,
    ///F - Unexpired Foreign Passport with Stamp or Attached Immigration and Naturalization Service (INS) Form Indicating Unexpired Employment Authorization
    CodeF,
    ///G - Alien Registration Receipt Card with Photograph
    AlienRegistrationReceiptCardWithPhotograph,
    ///H - Unexpired Temporary Resident Card
    UnexpiredTemporaryResidentCard,
    ///I - Unexpired Employment Authorization Card
    UnexpiredEmploymentAuthorizationCard,
    ///J - Unexpired Reentry Permit
    UnexpiredReentryPermit,
    ///K - Unexpired Refugee Travel Document
    UnexpiredRefugeeTravelDocument,
}
impl BasisOfVerificationCode {
    pub fn code(&self) -> &str {
        {
            use BasisOfVerificationCode::*;
            match self {
                BirthCertificate => "1",
                Passport => "2",
                HospitalCertificate => "3",
                Affidavit => "4",
                ImmigrationDocument => "5",
                BaptismalOrChurchCertificate => "6",
                PhysiciansCertificate => "7",
                Undocumented => "8",
                DriversLicense => "9",
                PhotoId => "A",
                SocialInsuranceCertificate => "B",
                UsPassport => "C",
                CertificateOfUsCitizenship => "D",
                CertificateOfNaturalization => "E",
                CodeF => "F",
                AlienRegistrationReceiptCardWithPhotograph => "G",
                UnexpiredTemporaryResidentCard => "H",
                UnexpiredEmploymentAuthorizationCard => "I",
                UnexpiredReentryPermit => "J",
                UnexpiredRefugeeTravelDocument => "K",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<BasisOfVerificationCode> {
        use BasisOfVerificationCode::*;
        match code {
            b"1" => Some(BirthCertificate),
            b"2" => Some(Passport),
            b"3" => Some(HospitalCertificate),
            b"4" => Some(Affidavit),
            b"5" => Some(ImmigrationDocument),
            b"6" => Some(BaptismalOrChurchCertificate),
            b"7" => Some(PhysiciansCertificate),
            b"8" => Some(Undocumented),
            b"9" => Some(DriversLicense),
            b"A" => Some(PhotoId),
            b"B" => Some(SocialInsuranceCertificate),
            b"C" => Some(UsPassport),
            b"D" => Some(CertificateOfUsCitizenship),
            b"E" => Some(CertificateOfNaturalization),
            b"F" => Some(CodeF),
            b"G" => Some(AlienRegistrationReceiptCardWithPhotograph),
            b"H" => Some(UnexpiredTemporaryResidentCard),
            b"I" => Some(UnexpiredEmploymentAuthorizationCard),
            b"J" => Some(UnexpiredReentryPermit),
            b"K" => Some(UnexpiredRefugeeTravelDocument),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use BasisOfVerificationCode::*;
        match self {
            BirthCertificate => "Birth Certificate",
            Passport => "Passport",
            HospitalCertificate => "Hospital Certificate",
            Affidavit => "Affidavit",
            ImmigrationDocument => "Immigration Document",
            BaptismalOrChurchCertificate => "Baptismal or Church Certificate",
            PhysiciansCertificate => "Physician's Certificate",
            Undocumented => "Undocumented",
            DriversLicense => "Driver's License",
            PhotoId => "Photo ID",
            SocialInsuranceCertificate => "Social Insurance Certificate",
            UsPassport => "US Passport",
            CertificateOfUsCitizenship => "Certificate of US Citizenship",
            CertificateOfNaturalization => "Certificate of Naturalization",
            CodeF => {
                "Unexpired Foreign Passport with Stamp or Attached Immigration and Naturalization Service (INS) Form Indicating Unexpired Employment Authorization"
            }
            AlienRegistrationReceiptCardWithPhotograph => {
                "Alien Registration Receipt Card with Photograph"
            }
            UnexpiredTemporaryResidentCard => "Unexpired Temporary Resident Card",
            UnexpiredEmploymentAuthorizationCard => {
                "Unexpired Employment Authorization Card"
            }
            UnexpiredReentryPermit => "Unexpired Reentry Permit",
            UnexpiredRefugeeTravelDocument => "Unexpired Refugee Travel Document",
        }
    }
    fn from_description(description: &str) -> Option<BasisOfVerificationCode> {
        {
            use BasisOfVerificationCode::*;
            match description {
                "Birth Certificate" => Some(BirthCertificate),
                "Passport" => Some(Passport),
                "Hospital Certificate" => Some(HospitalCertificate),
                "Affidavit" => Some(Affidavit),
                "Immigration Document" => Some(ImmigrationDocument),
                "Baptismal or Church Certificate" => Some(BaptismalOrChurchCertificate),
                "Physician's Certificate" => Some(PhysiciansCertificate),
                "Undocumented" => Some(Undocumented),
                "Driver's License" => Some(DriversLicense),
                "Photo ID" => Some(PhotoId),
                "Social Insurance Certificate" => Some(SocialInsuranceCertificate),
                "US Passport" => Some(UsPassport),
                "Certificate of US Citizenship" => Some(CertificateOfUsCitizenship),
                "Certificate of Naturalization" => Some(CertificateOfNaturalization),
                "Unexpired Foreign Passport with Stamp or Attached Immigration and Naturalization Service (INS) Form Indicating Unexpired Employment Authorization" => {
                    Some(CodeF)
                }
                "Alien Registration Receipt Card with Photograph" => {
                    Some(AlienRegistrationReceiptCardWithPhotograph)
                }
                "Unexpired Temporary Resident Card" => {
                    Some(UnexpiredTemporaryResidentCard)
                }
                "Unexpired Employment Authorization Card" => {
                    Some(UnexpiredEmploymentAuthorizationCard)
                }
                "Unexpired Reentry Permit" => Some(UnexpiredReentryPermit),
                "Unexpired Refugee Travel Document" => {
                    Some(UnexpiredRefugeeTravelDocument)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for BasisOfVerificationCode {
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
    type Value = BasisOfVerificationCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Basis of Verification Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        BasisOfVerificationCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Basis of Verification Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        BasisOfVerificationCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Basis of Verification Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for BasisOfVerificationCode {
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