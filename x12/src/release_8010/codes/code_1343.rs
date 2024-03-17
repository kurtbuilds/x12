use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1343

See docs at <https://www.stedi.com/edi/x12/element/1343>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NonInstitutionalClaimTypeCode {
    ///A - Anesthesia
    Anesthesia,
    ///AA - Alcohol Abuse
    AlcoholAbuse,
    ///AM - Ambulance
    Ambulance,
    ///AN - Anesthesia Nurse
    AnesthesiaNurse,
    ///CF - Convalescent Care Facility
    ConvalescentCareFacility,
    ///CH - Chiropractic
    Chiropractic,
    ///CS - Christian Science Practitioner
    ChristianSciencePractitioner,
    ///DA - Drug Abuse
    DrugAbuse,
    ///DD - Dentist or Dental
    DentistOrDental,
    ///DM - Durable Medical Equipment Supplier
    DurableMedicalEquipmentSupplier,
    ///DN - Denturist
    Denturist,
    ///ER - Emergency Room
    EmergencyRoom,
    ///HH - Home Health
    HomeHealth,
    ///HS - Hospital
    Hospital,
    ///IF - Intermediate Care Facility
    IntermediateCareFacility,
    ///IL - Independent Lab
    IndependentLab,
    ///L - Livery
    Livery,
    ///LT - Long-term Care
    LongTermCare,
    ///MD - Physician or Medical
    PhysicianOrMedical,
    ///MF - Marriage Family and Child Counselor
    MarriageFamilyAndChildCounselor,
    ///MH - Mental Health
    MentalHealth,
    ///MI - Midwife
    Midwife,
    ///MO - Mail Order Drug
    MailOrderDrug,
    ///MS - Multiple Type of Service
    MultipleTypeOfService,
    ///NM - Non-medical Provider
    NonMedicalProvider,
    ///NP - Nurse Practitioner
    NursePractitioner,
    ///NS - Nursing Service
    NursingService,
    ///O - Other
    Other,
    ///OC - Occupational Therapy
    OccupationalTherapy,
    ///OP - Optometrist
    Optometrist,
    ///OT - Optician
    Optician,
    ///OX - Oxygen
    Oxygen,
    ///P - Psychologist
    Psychologist,
    ///PA - Physician's Assistant
    PhysiciansAssistant,
    ///PD - Podiatry
    Podiatry,
    ///PE - Parental or Enteral (PEN)
    CodePE,
    ///PF - Physician's Office-based Facility
    PhysiciansOfficeBasedFacility,
    ///PS - Psychiatric Social Worker
    PsychiatricSocialWorker,
    ///PT - Physical Therapy
    PhysicalTherapy,
    ///RX - Pharmacy or Drug
    PharmacyOrDrug,
    ///S - Surgery
    Surgery,
    ///SC - Screening
    Screening,
    ///SN - Skilled Nursing
    SkilledNursing,
    ///ST - Speech or Language Therapy
    SpeechOrLanguageTherapy,
    ///T - Transportation
    Transportation,
    ///TX - Taxi
    Taxi,
    ///V - Vision
    Vision,
}
impl NonInstitutionalClaimTypeCode {
    pub fn code(&self) -> &str {
        {
            use NonInstitutionalClaimTypeCode::*;
            match self {
                Anesthesia => "A",
                AlcoholAbuse => "AA",
                Ambulance => "AM",
                AnesthesiaNurse => "AN",
                ConvalescentCareFacility => "CF",
                Chiropractic => "CH",
                ChristianSciencePractitioner => "CS",
                DrugAbuse => "DA",
                DentistOrDental => "DD",
                DurableMedicalEquipmentSupplier => "DM",
                Denturist => "DN",
                EmergencyRoom => "ER",
                HomeHealth => "HH",
                Hospital => "HS",
                IntermediateCareFacility => "IF",
                IndependentLab => "IL",
                Livery => "L",
                LongTermCare => "LT",
                PhysicianOrMedical => "MD",
                MarriageFamilyAndChildCounselor => "MF",
                MentalHealth => "MH",
                Midwife => "MI",
                MailOrderDrug => "MO",
                MultipleTypeOfService => "MS",
                NonMedicalProvider => "NM",
                NursePractitioner => "NP",
                NursingService => "NS",
                Other => "O",
                OccupationalTherapy => "OC",
                Optometrist => "OP",
                Optician => "OT",
                Oxygen => "OX",
                Psychologist => "P",
                PhysiciansAssistant => "PA",
                Podiatry => "PD",
                CodePE => "PE",
                PhysiciansOfficeBasedFacility => "PF",
                PsychiatricSocialWorker => "PS",
                PhysicalTherapy => "PT",
                PharmacyOrDrug => "RX",
                Surgery => "S",
                Screening => "SC",
                SkilledNursing => "SN",
                SpeechOrLanguageTherapy => "ST",
                Transportation => "T",
                Taxi => "TX",
                Vision => "V",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<NonInstitutionalClaimTypeCode> {
        use NonInstitutionalClaimTypeCode::*;
        match code {
            b"A" => Some(Anesthesia),
            b"AA" => Some(AlcoholAbuse),
            b"AM" => Some(Ambulance),
            b"AN" => Some(AnesthesiaNurse),
            b"CF" => Some(ConvalescentCareFacility),
            b"CH" => Some(Chiropractic),
            b"CS" => Some(ChristianSciencePractitioner),
            b"DA" => Some(DrugAbuse),
            b"DD" => Some(DentistOrDental),
            b"DM" => Some(DurableMedicalEquipmentSupplier),
            b"DN" => Some(Denturist),
            b"ER" => Some(EmergencyRoom),
            b"HH" => Some(HomeHealth),
            b"HS" => Some(Hospital),
            b"IF" => Some(IntermediateCareFacility),
            b"IL" => Some(IndependentLab),
            b"L" => Some(Livery),
            b"LT" => Some(LongTermCare),
            b"MD" => Some(PhysicianOrMedical),
            b"MF" => Some(MarriageFamilyAndChildCounselor),
            b"MH" => Some(MentalHealth),
            b"MI" => Some(Midwife),
            b"MO" => Some(MailOrderDrug),
            b"MS" => Some(MultipleTypeOfService),
            b"NM" => Some(NonMedicalProvider),
            b"NP" => Some(NursePractitioner),
            b"NS" => Some(NursingService),
            b"O" => Some(Other),
            b"OC" => Some(OccupationalTherapy),
            b"OP" => Some(Optometrist),
            b"OT" => Some(Optician),
            b"OX" => Some(Oxygen),
            b"P" => Some(Psychologist),
            b"PA" => Some(PhysiciansAssistant),
            b"PD" => Some(Podiatry),
            b"PE" => Some(CodePE),
            b"PF" => Some(PhysiciansOfficeBasedFacility),
            b"PS" => Some(PsychiatricSocialWorker),
            b"PT" => Some(PhysicalTherapy),
            b"RX" => Some(PharmacyOrDrug),
            b"S" => Some(Surgery),
            b"SC" => Some(Screening),
            b"SN" => Some(SkilledNursing),
            b"ST" => Some(SpeechOrLanguageTherapy),
            b"T" => Some(Transportation),
            b"TX" => Some(Taxi),
            b"V" => Some(Vision),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use NonInstitutionalClaimTypeCode::*;
        match self {
            Anesthesia => "Anesthesia",
            AlcoholAbuse => "Alcohol Abuse",
            Ambulance => "Ambulance",
            AnesthesiaNurse => "Anesthesia Nurse",
            ConvalescentCareFacility => "Convalescent Care Facility",
            Chiropractic => "Chiropractic",
            ChristianSciencePractitioner => "Christian Science Practitioner",
            DrugAbuse => "Drug Abuse",
            DentistOrDental => "Dentist or Dental",
            DurableMedicalEquipmentSupplier => "Durable Medical Equipment Supplier",
            Denturist => "Denturist",
            EmergencyRoom => "Emergency Room",
            HomeHealth => "Home Health",
            Hospital => "Hospital",
            IntermediateCareFacility => "Intermediate Care Facility",
            IndependentLab => "Independent Lab",
            Livery => "Livery",
            LongTermCare => "Long-term Care",
            PhysicianOrMedical => "Physician or Medical",
            MarriageFamilyAndChildCounselor => "Marriage Family and Child Counselor",
            MentalHealth => "Mental Health",
            Midwife => "Midwife",
            MailOrderDrug => "Mail Order Drug",
            MultipleTypeOfService => "Multiple Type of Service",
            NonMedicalProvider => "Non-medical Provider",
            NursePractitioner => "Nurse Practitioner",
            NursingService => "Nursing Service",
            Other => "Other",
            OccupationalTherapy => "Occupational Therapy",
            Optometrist => "Optometrist",
            Optician => "Optician",
            Oxygen => "Oxygen",
            Psychologist => "Psychologist",
            PhysiciansAssistant => "Physician's Assistant",
            Podiatry => "Podiatry",
            CodePE => "Parental or Enteral (PEN)",
            PhysiciansOfficeBasedFacility => "Physician's Office-based Facility",
            PsychiatricSocialWorker => "Psychiatric Social Worker",
            PhysicalTherapy => "Physical Therapy",
            PharmacyOrDrug => "Pharmacy or Drug",
            Surgery => "Surgery",
            Screening => "Screening",
            SkilledNursing => "Skilled Nursing",
            SpeechOrLanguageTherapy => "Speech or Language Therapy",
            Transportation => "Transportation",
            Taxi => "Taxi",
            Vision => "Vision",
        }
    }
    fn from_description(description: &str) -> Option<NonInstitutionalClaimTypeCode> {
        {
            use NonInstitutionalClaimTypeCode::*;
            match description {
                "Anesthesia" => Some(Anesthesia),
                "Alcohol Abuse" => Some(AlcoholAbuse),
                "Ambulance" => Some(Ambulance),
                "Anesthesia Nurse" => Some(AnesthesiaNurse),
                "Convalescent Care Facility" => Some(ConvalescentCareFacility),
                "Chiropractic" => Some(Chiropractic),
                "Christian Science Practitioner" => Some(ChristianSciencePractitioner),
                "Drug Abuse" => Some(DrugAbuse),
                "Dentist or Dental" => Some(DentistOrDental),
                "Durable Medical Equipment Supplier" => {
                    Some(DurableMedicalEquipmentSupplier)
                }
                "Denturist" => Some(Denturist),
                "Emergency Room" => Some(EmergencyRoom),
                "Home Health" => Some(HomeHealth),
                "Hospital" => Some(Hospital),
                "Intermediate Care Facility" => Some(IntermediateCareFacility),
                "Independent Lab" => Some(IndependentLab),
                "Livery" => Some(Livery),
                "Long-term Care" => Some(LongTermCare),
                "Physician or Medical" => Some(PhysicianOrMedical),
                "Marriage Family and Child Counselor" => {
                    Some(MarriageFamilyAndChildCounselor)
                }
                "Mental Health" => Some(MentalHealth),
                "Midwife" => Some(Midwife),
                "Mail Order Drug" => Some(MailOrderDrug),
                "Multiple Type of Service" => Some(MultipleTypeOfService),
                "Non-medical Provider" => Some(NonMedicalProvider),
                "Nurse Practitioner" => Some(NursePractitioner),
                "Nursing Service" => Some(NursingService),
                "Other" => Some(Other),
                "Occupational Therapy" => Some(OccupationalTherapy),
                "Optometrist" => Some(Optometrist),
                "Optician" => Some(Optician),
                "Oxygen" => Some(Oxygen),
                "Psychologist" => Some(Psychologist),
                "Physician's Assistant" => Some(PhysiciansAssistant),
                "Podiatry" => Some(Podiatry),
                "Parental or Enteral (PEN)" => Some(CodePE),
                "Physician's Office-based Facility" => {
                    Some(PhysiciansOfficeBasedFacility)
                }
                "Psychiatric Social Worker" => Some(PsychiatricSocialWorker),
                "Physical Therapy" => Some(PhysicalTherapy),
                "Pharmacy or Drug" => Some(PharmacyOrDrug),
                "Surgery" => Some(Surgery),
                "Screening" => Some(Screening),
                "Skilled Nursing" => Some(SkilledNursing),
                "Speech or Language Therapy" => Some(SpeechOrLanguageTherapy),
                "Transportation" => Some(Transportation),
                "Taxi" => Some(Taxi),
                "Vision" => Some(Vision),
                _ => None,
            }
        }
    }
}
impl Serialize for NonInstitutionalClaimTypeCode {
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
    type Value = NonInstitutionalClaimTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Non-Institutional Claim Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NonInstitutionalClaimTypeCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Non-Institutional Claim Type Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NonInstitutionalClaimTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Non-Institutional Claim Type Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for NonInstitutionalClaimTypeCode {
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