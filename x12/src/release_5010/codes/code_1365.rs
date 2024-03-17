use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1365

See docs at <https://www.stedi.com/edi/x12/element/1365>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServiceTypeCode {
    ///1 - Medical Care
    MedicalCare,
    ///2 - Surgical
    Surgical,
    ///3 - Consultation
    Consultation,
    ///4 - Diagnostic X-Ray
    DiagnosticXRay,
    ///5 - Diagnostic Lab
    DiagnosticLab,
    ///6 - Radiation Therapy
    RadiationTherapy,
    ///7 - Anesthesia
    Anesthesia,
    ///8 - Surgical Assistance
    SurgicalAssistance,
    ///9 - Other Medical
    OtherMedical,
    ///10 - Blood Charges
    BloodCharges,
    ///11 - Used Durable Medical Equipment
    UsedDurableMedicalEquipment,
    ///12 - Durable Medical Equipment Purchase
    DurableMedicalEquipmentPurchase,
    ///13 - Ambulatory Service Center Facility
    AmbulatoryServiceCenterFacility,
    ///14 - Renal Supplies in the Home
    RenalSuppliesInTheHome,
    ///15 - Alternate Method Dialysis
    AlternateMethodDialysis,
    ///16 - Chronic Renal Disease (CRD) Equipment
    Code16,
    ///17 - Pre-Admission Testing
    PreAdmissionTesting,
    ///18 - Durable Medical Equipment Rental
    DurableMedicalEquipmentRental,
    ///19 - Pneumonia Vaccine
    PneumoniaVaccine,
    ///20 - Second Surgical Opinion
    SecondSurgicalOpinion,
    ///21 - Third Surgical Opinion
    ThirdSurgicalOpinion,
    ///22 - Social Work
    SocialWork,
    ///23 - Diagnostic Dental
    DiagnosticDental,
    ///24 - Periodontics
    Periodontics,
    ///25 - Restorative
    Restorative,
    ///26 - Endodontics
    Endodontics,
    ///27 - Maxillofacial Prosthetics
    MaxillofacialProsthetics,
    ///28 - Adjunctive Dental Services
    AdjunctiveDentalServices,
    ///30 - Health Benefit Plan Coverage
    HealthBenefitPlanCoverage,
    ///31 - Benefit Disclaimer
    BenefitDisclaimer,
    ///32 - Plan Waiting Period
    PlanWaitingPeriod,
    ///33 - Chiropractic
    Chiropractic,
    ///34 - Chiropractic Office Visits
    ChiropracticOfficeVisits,
    ///35 - Dental Care
    DentalCare,
    ///36 - Dental Crowns
    DentalCrowns,
    ///37 - Dental Accident
    DentalAccident,
    ///38 - Orthodontics
    Orthodontics,
    ///39 - Prosthodontics
    Prosthodontics,
    ///40 - Oral Surgery
    OralSurgery,
    ///41 - Routine (Preventive) Dental
    Code41,
    ///42 - Home Health Care
    HomeHealthCare,
    ///43 - Home Health Prescriptions
    HomeHealthPrescriptions,
    ///44 - Home Health Visits
    HomeHealthVisits,
    ///45 - Hospice
    Hospice,
    ///46 - Respite Care
    RespiteCare,
    ///47 - Hospital
    Hospital,
    ///48 - Hospital - Inpatient
    HospitalInpatient,
    ///49 - Hospital - Room and Board
    HospitalRoomAndBoard,
    ///50 - Hospital - Outpatient
    HospitalOutpatient,
    ///51 - Hospital - Emergency Accident
    HospitalEmergencyAccident,
    ///52 - Hospital - Emergency Medical
    HospitalEmergencyMedical,
    ///53 - Hospital - Ambulatory Surgical
    HospitalAmbulatorySurgical,
    ///54 - Long Term Care
    LongTermCare,
    ///55 - Major Medical
    MajorMedical,
    ///56 - Medically Related Transportation
    MedicallyRelatedTransportation,
    ///57 - Air Transportation
    AirTransportation,
    ///58 - Cabulance
    Cabulance,
    ///59 - Licensed Ambulance
    LicensedAmbulance,
    ///60 - General Benefits
    GeneralBenefits,
    ///61 - In-vitro Fertilization
    InVitroFertilization,
    ///62 - MRI/CAT Scan
    MriCatScan,
    ///63 - Donor Procedures
    DonorProcedures,
    ///64 - Acupuncture
    Acupuncture,
    ///65 - Newborn Care
    NewbornCare,
    ///66 - Pathology
    Pathology,
    ///67 - Smoking Cessation
    SmokingCessation,
    ///68 - Well Baby Care
    WellBabyCare,
    ///69 - Maternity
    Maternity,
    ///70 - Transplants
    Transplants,
    ///71 - Audiology Exam
    AudiologyExam,
    ///72 - Inhalation Therapy
    InhalationTherapy,
    ///73 - Diagnostic Medical
    DiagnosticMedical,
    ///74 - Private Duty Nursing
    PrivateDutyNursing,
    ///75 - Prosthetic Device
    ProstheticDevice,
    ///76 - Dialysis
    Dialysis,
    ///77 - Otological Exam
    OtologicalExam,
    ///78 - Chemotherapy
    Chemotherapy,
    ///79 - Allergy Testing
    AllergyTesting,
    ///80 - Immunizations
    Immunizations,
    ///81 - Routine Physical
    RoutinePhysical,
    ///82 - Family Planning
    FamilyPlanning,
    ///83 - Infertility
    Infertility,
    ///84 - Abortion
    Abortion,
    ///85 - AIDS
    Aids,
    ///86 - Emergency Services
    EmergencyServices,
    ///87 - Cancer
    Cancer,
    ///88 - Pharmacy
    Pharmacy,
    ///89 - Free Standing Prescription Drug
    FreeStandingPrescriptionDrug,
    ///90 - Mail Order Prescription Drug
    MailOrderPrescriptionDrug,
    ///91 - Brand Name Prescription Drug
    BrandNamePrescriptionDrug,
    ///92 - Generic Prescription Drug
    GenericPrescriptionDrug,
    ///93 - Podiatry
    Podiatry,
    ///94 - Podiatry - Office Visits
    PodiatryOfficeVisits,
    ///95 - Podiatry - Nursing Home Visits
    PodiatryNursingHomeVisits,
    ///96 - Professional (Physician)
    Code96,
    ///97 - Anesthesiologist
    Anesthesiologist,
    ///98 - Professional (Physician) Visit - Office
    Code98,
    ///99 - Professional (Physician) Visit - Inpatient
    Code99,
    ///A0 - Professional (Physician) Visit - Outpatient
    CodeA0,
    ///A1 - Professional (Physician) Visit - Nursing Home
    CodeA1,
    ///A2 - Professional (Physician) Visit - Skilled Nursing Facility
    CodeA2,
    ///A3 - Professional (Physician) Visit - Home
    CodeA3,
    ///A4 - Psychiatric
    Psychiatric,
    ///A5 - Psychiatric - Room and Board
    PsychiatricRoomAndBoard,
    ///A6 - Psychotherapy
    Psychotherapy,
    ///A7 - Psychiatric - Inpatient
    PsychiatricInpatient,
    ///A8 - Psychiatric - Outpatient
    PsychiatricOutpatient,
    ///A9 - Rehabilitation
    Rehabilitation,
    ///AA - Rehabilitation - Room and Board
    RehabilitationRoomAndBoard,
    ///AB - Rehabilitation - Inpatient
    RehabilitationInpatient,
    ///AC - Rehabilitation - Outpatient
    RehabilitationOutpatient,
    ///AD - Occupational Therapy
    OccupationalTherapy,
    ///AE - Physical Medicine
    PhysicalMedicine,
    ///AF - Speech Therapy
    SpeechTherapy,
    ///AG - Skilled Nursing Care
    SkilledNursingCare,
    ///AH - Skilled Nursing Care - Room and Board
    SkilledNursingCareRoomAndBoard,
    ///AI - Substance Abuse
    SubstanceAbuse,
    ///AJ - Alcoholism
    Alcoholism,
    ///AK - Drug Addiction
    DrugAddiction,
    ///AL - Vision (Optometry)
    CodeAL,
    ///AM - Frames
    Frames,
    ///AN - Routine Exam
    RoutineExam,
    ///AO - Lenses
    Lenses,
    ///AQ - Nonmedically Necessary Physical
    NonmedicallyNecessaryPhysical,
    ///AR - Experimental Drug Therapy
    ExperimentalDrugTherapy,
    ///B - Non-escrow or Non-impound Service
    NonEscrowOrNonImpoundService,
    ///B1 - Burn Care
    BurnCare,
    ///B2 - Brand Name Prescription Drug - Formulary
    BrandNamePrescriptionDrugFormulary,
    ///B3 - Brand Name Prescription Drug - Non-Formulary
    BrandNamePrescriptionDrugNonFormulary,
    ///BA - Independent Medical Evaluation
    IndependentMedicalEvaluation,
    ///BB - Partial Hospitalization (Psychiatric)
    CodeBB,
    ///BC - Day Care (Psychiatric)
    CodeBC,
    ///BD - Cognitive Therapy
    CognitiveTherapy,
    ///BE - Massage Therapy
    MassageTherapy,
    ///BF - Pulmonary Rehabilitation
    PulmonaryRehabilitation,
    ///BG - Cardiac Rehabilitation
    CardiacRehabilitation,
    ///BH - Pediatric
    Pediatric,
    ///BI - Nursery
    Nursery,
    ///BJ - Skin
    Skin,
    ///BK - Orthopedic
    Orthopedic,
    ///BL - Cardiac
    Cardiac,
    ///BM - Lymphatic
    Lymphatic,
    ///BN - Gastrointestinal
    Gastrointestinal,
    ///BP - Endocrine
    Endocrine,
    ///BQ - Neurology
    Neurology,
    ///BR - Eye
    Eye,
    ///BS - Invasive Procedures
    InvasiveProcedures,
    ///BT - Gynecological
    Gynecological,
    ///BU - Obstetrical
    Obstetrical,
    ///BV - Obstetrical/Gynecological
    ObstetricalGynecological,
    ///BW - Mail Order Prescription Drug: Brand Name
    MailOrderPrescriptionDrugBrandName,
    ///BX - Mail Order Prescription Drug: Generic
    MailOrderPrescriptionDrugGeneric,
    ///BY - Physician Visit - Office: Sick
    PhysicianVisitOfficeSick,
    ///BZ - Physician Visit - Office: Well
    PhysicianVisitOfficeWell,
    ///C - Escrow or Impound Service
    EscrowOrImpoundService,
    ///C1 - Coronary Care
    CoronaryCare,
    ///CA - Private Duty Nursing - Inpatient
    PrivateDutyNursingInpatient,
    ///CB - Private Duty Nursing - Home
    PrivateDutyNursingHome,
    ///CC - Surgical Benefits - Professional (Physician)
    CodeCC,
    ///CD - Surgical Benefits - Facility
    SurgicalBenefitsFacility,
    ///CE - Mental Health Provider - Inpatient
    MentalHealthProviderInpatient,
    ///CF - Mental Health Provider - Outpatient
    MentalHealthProviderOutpatient,
    ///CG - Mental Health Facility - Inpatient
    MentalHealthFacilityInpatient,
    ///CH - Mental Health Facility - Outpatient
    MentalHealthFacilityOutpatient,
    ///CI - Substance Abuse Facility - Inpatient
    SubstanceAbuseFacilityInpatient,
    ///CJ - Substance Abuse Facility - Outpatient
    SubstanceAbuseFacilityOutpatient,
    ///CK - Screening X-ray
    ScreeningXRay,
    ///CL - Screening laboratory
    ScreeningLaboratory,
    ///CM - Mammogram, High Risk Patient
    CodeCM,
    ///CN - Mammogram, Low Risk Patient
    CodeCN,
    ///CO - Flu Vaccination
    FluVaccination,
    ///CP - Eyewear and Eyewear Accessories
    EyewearAndEyewearAccessories,
    ///CQ - Case Management
    CaseManagement,
    ///DG - Dermatology
    Dermatology,
    ///DM - Durable Medical Equipment
    DurableMedicalEquipment,
    ///DS - Diabetic Supplies
    DiabeticSupplies,
    ///GF - Generic Prescription Drug - Formulary
    GenericPrescriptionDrugFormulary,
    ///GN - Generic Prescription Drug - Non-Formulary
    GenericPrescriptionDrugNonFormulary,
    ///GY - Allergy
    Allergy,
    ///IC - Intensive Care
    IntensiveCare,
    ///MH - Mental Health
    MentalHealth,
    ///NI - Neonatal Intensive Care
    NeonatalIntensiveCare,
    ///ON - Oncology
    Oncology,
    ///PT - Physical Therapy
    PhysicalTherapy,
    ///PU - Pulmonary
    Pulmonary,
    ///RN - Renal
    Renal,
    ///RT - Residential Psychiatric Treatment
    ResidentialPsychiatricTreatment,
    ///TC - Transitional Care
    TransitionalCare,
    ///TN - Transitional Nursery Care
    TransitionalNurseryCare,
    ///UC - Urgent Care
    UrgentCare,
}
impl ServiceTypeCode {
    pub fn code(&self) -> &str {
        {
            use ServiceTypeCode::*;
            match self {
                MedicalCare => "1",
                Surgical => "2",
                Consultation => "3",
                DiagnosticXRay => "4",
                DiagnosticLab => "5",
                RadiationTherapy => "6",
                Anesthesia => "7",
                SurgicalAssistance => "8",
                OtherMedical => "9",
                BloodCharges => "10",
                UsedDurableMedicalEquipment => "11",
                DurableMedicalEquipmentPurchase => "12",
                AmbulatoryServiceCenterFacility => "13",
                RenalSuppliesInTheHome => "14",
                AlternateMethodDialysis => "15",
                Code16 => "16",
                PreAdmissionTesting => "17",
                DurableMedicalEquipmentRental => "18",
                PneumoniaVaccine => "19",
                SecondSurgicalOpinion => "20",
                ThirdSurgicalOpinion => "21",
                SocialWork => "22",
                DiagnosticDental => "23",
                Periodontics => "24",
                Restorative => "25",
                Endodontics => "26",
                MaxillofacialProsthetics => "27",
                AdjunctiveDentalServices => "28",
                HealthBenefitPlanCoverage => "30",
                BenefitDisclaimer => "31",
                PlanWaitingPeriod => "32",
                Chiropractic => "33",
                ChiropracticOfficeVisits => "34",
                DentalCare => "35",
                DentalCrowns => "36",
                DentalAccident => "37",
                Orthodontics => "38",
                Prosthodontics => "39",
                OralSurgery => "40",
                Code41 => "41",
                HomeHealthCare => "42",
                HomeHealthPrescriptions => "43",
                HomeHealthVisits => "44",
                Hospice => "45",
                RespiteCare => "46",
                Hospital => "47",
                HospitalInpatient => "48",
                HospitalRoomAndBoard => "49",
                HospitalOutpatient => "50",
                HospitalEmergencyAccident => "51",
                HospitalEmergencyMedical => "52",
                HospitalAmbulatorySurgical => "53",
                LongTermCare => "54",
                MajorMedical => "55",
                MedicallyRelatedTransportation => "56",
                AirTransportation => "57",
                Cabulance => "58",
                LicensedAmbulance => "59",
                GeneralBenefits => "60",
                InVitroFertilization => "61",
                MriCatScan => "62",
                DonorProcedures => "63",
                Acupuncture => "64",
                NewbornCare => "65",
                Pathology => "66",
                SmokingCessation => "67",
                WellBabyCare => "68",
                Maternity => "69",
                Transplants => "70",
                AudiologyExam => "71",
                InhalationTherapy => "72",
                DiagnosticMedical => "73",
                PrivateDutyNursing => "74",
                ProstheticDevice => "75",
                Dialysis => "76",
                OtologicalExam => "77",
                Chemotherapy => "78",
                AllergyTesting => "79",
                Immunizations => "80",
                RoutinePhysical => "81",
                FamilyPlanning => "82",
                Infertility => "83",
                Abortion => "84",
                Aids => "85",
                EmergencyServices => "86",
                Cancer => "87",
                Pharmacy => "88",
                FreeStandingPrescriptionDrug => "89",
                MailOrderPrescriptionDrug => "90",
                BrandNamePrescriptionDrug => "91",
                GenericPrescriptionDrug => "92",
                Podiatry => "93",
                PodiatryOfficeVisits => "94",
                PodiatryNursingHomeVisits => "95",
                Code96 => "96",
                Anesthesiologist => "97",
                Code98 => "98",
                Code99 => "99",
                CodeA0 => "A0",
                CodeA1 => "A1",
                CodeA2 => "A2",
                CodeA3 => "A3",
                Psychiatric => "A4",
                PsychiatricRoomAndBoard => "A5",
                Psychotherapy => "A6",
                PsychiatricInpatient => "A7",
                PsychiatricOutpatient => "A8",
                Rehabilitation => "A9",
                RehabilitationRoomAndBoard => "AA",
                RehabilitationInpatient => "AB",
                RehabilitationOutpatient => "AC",
                OccupationalTherapy => "AD",
                PhysicalMedicine => "AE",
                SpeechTherapy => "AF",
                SkilledNursingCare => "AG",
                SkilledNursingCareRoomAndBoard => "AH",
                SubstanceAbuse => "AI",
                Alcoholism => "AJ",
                DrugAddiction => "AK",
                CodeAL => "AL",
                Frames => "AM",
                RoutineExam => "AN",
                Lenses => "AO",
                NonmedicallyNecessaryPhysical => "AQ",
                ExperimentalDrugTherapy => "AR",
                NonEscrowOrNonImpoundService => "B",
                BurnCare => "B1",
                BrandNamePrescriptionDrugFormulary => "B2",
                BrandNamePrescriptionDrugNonFormulary => "B3",
                IndependentMedicalEvaluation => "BA",
                CodeBB => "BB",
                CodeBC => "BC",
                CognitiveTherapy => "BD",
                MassageTherapy => "BE",
                PulmonaryRehabilitation => "BF",
                CardiacRehabilitation => "BG",
                Pediatric => "BH",
                Nursery => "BI",
                Skin => "BJ",
                Orthopedic => "BK",
                Cardiac => "BL",
                Lymphatic => "BM",
                Gastrointestinal => "BN",
                Endocrine => "BP",
                Neurology => "BQ",
                Eye => "BR",
                InvasiveProcedures => "BS",
                Gynecological => "BT",
                Obstetrical => "BU",
                ObstetricalGynecological => "BV",
                MailOrderPrescriptionDrugBrandName => "BW",
                MailOrderPrescriptionDrugGeneric => "BX",
                PhysicianVisitOfficeSick => "BY",
                PhysicianVisitOfficeWell => "BZ",
                EscrowOrImpoundService => "C",
                CoronaryCare => "C1",
                PrivateDutyNursingInpatient => "CA",
                PrivateDutyNursingHome => "CB",
                CodeCC => "CC",
                SurgicalBenefitsFacility => "CD",
                MentalHealthProviderInpatient => "CE",
                MentalHealthProviderOutpatient => "CF",
                MentalHealthFacilityInpatient => "CG",
                MentalHealthFacilityOutpatient => "CH",
                SubstanceAbuseFacilityInpatient => "CI",
                SubstanceAbuseFacilityOutpatient => "CJ",
                ScreeningXRay => "CK",
                ScreeningLaboratory => "CL",
                CodeCM => "CM",
                CodeCN => "CN",
                FluVaccination => "CO",
                EyewearAndEyewearAccessories => "CP",
                CaseManagement => "CQ",
                Dermatology => "DG",
                DurableMedicalEquipment => "DM",
                DiabeticSupplies => "DS",
                GenericPrescriptionDrugFormulary => "GF",
                GenericPrescriptionDrugNonFormulary => "GN",
                Allergy => "GY",
                IntensiveCare => "IC",
                MentalHealth => "MH",
                NeonatalIntensiveCare => "NI",
                Oncology => "ON",
                PhysicalTherapy => "PT",
                Pulmonary => "PU",
                Renal => "RN",
                ResidentialPsychiatricTreatment => "RT",
                TransitionalCare => "TC",
                TransitionalNurseryCare => "TN",
                UrgentCare => "UC",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ServiceTypeCode> {
        use ServiceTypeCode::*;
        match code {
            b"1" => Some(MedicalCare),
            b"2" => Some(Surgical),
            b"3" => Some(Consultation),
            b"4" => Some(DiagnosticXRay),
            b"5" => Some(DiagnosticLab),
            b"6" => Some(RadiationTherapy),
            b"7" => Some(Anesthesia),
            b"8" => Some(SurgicalAssistance),
            b"9" => Some(OtherMedical),
            b"10" => Some(BloodCharges),
            b"11" => Some(UsedDurableMedicalEquipment),
            b"12" => Some(DurableMedicalEquipmentPurchase),
            b"13" => Some(AmbulatoryServiceCenterFacility),
            b"14" => Some(RenalSuppliesInTheHome),
            b"15" => Some(AlternateMethodDialysis),
            b"16" => Some(Code16),
            b"17" => Some(PreAdmissionTesting),
            b"18" => Some(DurableMedicalEquipmentRental),
            b"19" => Some(PneumoniaVaccine),
            b"20" => Some(SecondSurgicalOpinion),
            b"21" => Some(ThirdSurgicalOpinion),
            b"22" => Some(SocialWork),
            b"23" => Some(DiagnosticDental),
            b"24" => Some(Periodontics),
            b"25" => Some(Restorative),
            b"26" => Some(Endodontics),
            b"27" => Some(MaxillofacialProsthetics),
            b"28" => Some(AdjunctiveDentalServices),
            b"30" => Some(HealthBenefitPlanCoverage),
            b"31" => Some(BenefitDisclaimer),
            b"32" => Some(PlanWaitingPeriod),
            b"33" => Some(Chiropractic),
            b"34" => Some(ChiropracticOfficeVisits),
            b"35" => Some(DentalCare),
            b"36" => Some(DentalCrowns),
            b"37" => Some(DentalAccident),
            b"38" => Some(Orthodontics),
            b"39" => Some(Prosthodontics),
            b"40" => Some(OralSurgery),
            b"41" => Some(Code41),
            b"42" => Some(HomeHealthCare),
            b"43" => Some(HomeHealthPrescriptions),
            b"44" => Some(HomeHealthVisits),
            b"45" => Some(Hospice),
            b"46" => Some(RespiteCare),
            b"47" => Some(Hospital),
            b"48" => Some(HospitalInpatient),
            b"49" => Some(HospitalRoomAndBoard),
            b"50" => Some(HospitalOutpatient),
            b"51" => Some(HospitalEmergencyAccident),
            b"52" => Some(HospitalEmergencyMedical),
            b"53" => Some(HospitalAmbulatorySurgical),
            b"54" => Some(LongTermCare),
            b"55" => Some(MajorMedical),
            b"56" => Some(MedicallyRelatedTransportation),
            b"57" => Some(AirTransportation),
            b"58" => Some(Cabulance),
            b"59" => Some(LicensedAmbulance),
            b"60" => Some(GeneralBenefits),
            b"61" => Some(InVitroFertilization),
            b"62" => Some(MriCatScan),
            b"63" => Some(DonorProcedures),
            b"64" => Some(Acupuncture),
            b"65" => Some(NewbornCare),
            b"66" => Some(Pathology),
            b"67" => Some(SmokingCessation),
            b"68" => Some(WellBabyCare),
            b"69" => Some(Maternity),
            b"70" => Some(Transplants),
            b"71" => Some(AudiologyExam),
            b"72" => Some(InhalationTherapy),
            b"73" => Some(DiagnosticMedical),
            b"74" => Some(PrivateDutyNursing),
            b"75" => Some(ProstheticDevice),
            b"76" => Some(Dialysis),
            b"77" => Some(OtologicalExam),
            b"78" => Some(Chemotherapy),
            b"79" => Some(AllergyTesting),
            b"80" => Some(Immunizations),
            b"81" => Some(RoutinePhysical),
            b"82" => Some(FamilyPlanning),
            b"83" => Some(Infertility),
            b"84" => Some(Abortion),
            b"85" => Some(Aids),
            b"86" => Some(EmergencyServices),
            b"87" => Some(Cancer),
            b"88" => Some(Pharmacy),
            b"89" => Some(FreeStandingPrescriptionDrug),
            b"90" => Some(MailOrderPrescriptionDrug),
            b"91" => Some(BrandNamePrescriptionDrug),
            b"92" => Some(GenericPrescriptionDrug),
            b"93" => Some(Podiatry),
            b"94" => Some(PodiatryOfficeVisits),
            b"95" => Some(PodiatryNursingHomeVisits),
            b"96" => Some(Code96),
            b"97" => Some(Anesthesiologist),
            b"98" => Some(Code98),
            b"99" => Some(Code99),
            b"A0" => Some(CodeA0),
            b"A1" => Some(CodeA1),
            b"A2" => Some(CodeA2),
            b"A3" => Some(CodeA3),
            b"A4" => Some(Psychiatric),
            b"A5" => Some(PsychiatricRoomAndBoard),
            b"A6" => Some(Psychotherapy),
            b"A7" => Some(PsychiatricInpatient),
            b"A8" => Some(PsychiatricOutpatient),
            b"A9" => Some(Rehabilitation),
            b"AA" => Some(RehabilitationRoomAndBoard),
            b"AB" => Some(RehabilitationInpatient),
            b"AC" => Some(RehabilitationOutpatient),
            b"AD" => Some(OccupationalTherapy),
            b"AE" => Some(PhysicalMedicine),
            b"AF" => Some(SpeechTherapy),
            b"AG" => Some(SkilledNursingCare),
            b"AH" => Some(SkilledNursingCareRoomAndBoard),
            b"AI" => Some(SubstanceAbuse),
            b"AJ" => Some(Alcoholism),
            b"AK" => Some(DrugAddiction),
            b"AL" => Some(CodeAL),
            b"AM" => Some(Frames),
            b"AN" => Some(RoutineExam),
            b"AO" => Some(Lenses),
            b"AQ" => Some(NonmedicallyNecessaryPhysical),
            b"AR" => Some(ExperimentalDrugTherapy),
            b"B" => Some(NonEscrowOrNonImpoundService),
            b"B1" => Some(BurnCare),
            b"B2" => Some(BrandNamePrescriptionDrugFormulary),
            b"B3" => Some(BrandNamePrescriptionDrugNonFormulary),
            b"BA" => Some(IndependentMedicalEvaluation),
            b"BB" => Some(CodeBB),
            b"BC" => Some(CodeBC),
            b"BD" => Some(CognitiveTherapy),
            b"BE" => Some(MassageTherapy),
            b"BF" => Some(PulmonaryRehabilitation),
            b"BG" => Some(CardiacRehabilitation),
            b"BH" => Some(Pediatric),
            b"BI" => Some(Nursery),
            b"BJ" => Some(Skin),
            b"BK" => Some(Orthopedic),
            b"BL" => Some(Cardiac),
            b"BM" => Some(Lymphatic),
            b"BN" => Some(Gastrointestinal),
            b"BP" => Some(Endocrine),
            b"BQ" => Some(Neurology),
            b"BR" => Some(Eye),
            b"BS" => Some(InvasiveProcedures),
            b"BT" => Some(Gynecological),
            b"BU" => Some(Obstetrical),
            b"BV" => Some(ObstetricalGynecological),
            b"BW" => Some(MailOrderPrescriptionDrugBrandName),
            b"BX" => Some(MailOrderPrescriptionDrugGeneric),
            b"BY" => Some(PhysicianVisitOfficeSick),
            b"BZ" => Some(PhysicianVisitOfficeWell),
            b"C" => Some(EscrowOrImpoundService),
            b"C1" => Some(CoronaryCare),
            b"CA" => Some(PrivateDutyNursingInpatient),
            b"CB" => Some(PrivateDutyNursingHome),
            b"CC" => Some(CodeCC),
            b"CD" => Some(SurgicalBenefitsFacility),
            b"CE" => Some(MentalHealthProviderInpatient),
            b"CF" => Some(MentalHealthProviderOutpatient),
            b"CG" => Some(MentalHealthFacilityInpatient),
            b"CH" => Some(MentalHealthFacilityOutpatient),
            b"CI" => Some(SubstanceAbuseFacilityInpatient),
            b"CJ" => Some(SubstanceAbuseFacilityOutpatient),
            b"CK" => Some(ScreeningXRay),
            b"CL" => Some(ScreeningLaboratory),
            b"CM" => Some(CodeCM),
            b"CN" => Some(CodeCN),
            b"CO" => Some(FluVaccination),
            b"CP" => Some(EyewearAndEyewearAccessories),
            b"CQ" => Some(CaseManagement),
            b"DG" => Some(Dermatology),
            b"DM" => Some(DurableMedicalEquipment),
            b"DS" => Some(DiabeticSupplies),
            b"GF" => Some(GenericPrescriptionDrugFormulary),
            b"GN" => Some(GenericPrescriptionDrugNonFormulary),
            b"GY" => Some(Allergy),
            b"IC" => Some(IntensiveCare),
            b"MH" => Some(MentalHealth),
            b"NI" => Some(NeonatalIntensiveCare),
            b"ON" => Some(Oncology),
            b"PT" => Some(PhysicalTherapy),
            b"PU" => Some(Pulmonary),
            b"RN" => Some(Renal),
            b"RT" => Some(ResidentialPsychiatricTreatment),
            b"TC" => Some(TransitionalCare),
            b"TN" => Some(TransitionalNurseryCare),
            b"UC" => Some(UrgentCare),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ServiceTypeCode::*;
        match self {
            MedicalCare => "Medical Care",
            Surgical => "Surgical",
            Consultation => "Consultation",
            DiagnosticXRay => "Diagnostic X-Ray",
            DiagnosticLab => "Diagnostic Lab",
            RadiationTherapy => "Radiation Therapy",
            Anesthesia => "Anesthesia",
            SurgicalAssistance => "Surgical Assistance",
            OtherMedical => "Other Medical",
            BloodCharges => "Blood Charges",
            UsedDurableMedicalEquipment => "Used Durable Medical Equipment",
            DurableMedicalEquipmentPurchase => "Durable Medical Equipment Purchase",
            AmbulatoryServiceCenterFacility => "Ambulatory Service Center Facility",
            RenalSuppliesInTheHome => "Renal Supplies in the Home",
            AlternateMethodDialysis => "Alternate Method Dialysis",
            Code16 => "Chronic Renal Disease (CRD) Equipment",
            PreAdmissionTesting => "Pre-Admission Testing",
            DurableMedicalEquipmentRental => "Durable Medical Equipment Rental",
            PneumoniaVaccine => "Pneumonia Vaccine",
            SecondSurgicalOpinion => "Second Surgical Opinion",
            ThirdSurgicalOpinion => "Third Surgical Opinion",
            SocialWork => "Social Work",
            DiagnosticDental => "Diagnostic Dental",
            Periodontics => "Periodontics",
            Restorative => "Restorative",
            Endodontics => "Endodontics",
            MaxillofacialProsthetics => "Maxillofacial Prosthetics",
            AdjunctiveDentalServices => "Adjunctive Dental Services",
            HealthBenefitPlanCoverage => "Health Benefit Plan Coverage",
            BenefitDisclaimer => "Benefit Disclaimer",
            PlanWaitingPeriod => "Plan Waiting Period",
            Chiropractic => "Chiropractic",
            ChiropracticOfficeVisits => "Chiropractic Office Visits",
            DentalCare => "Dental Care",
            DentalCrowns => "Dental Crowns",
            DentalAccident => "Dental Accident",
            Orthodontics => "Orthodontics",
            Prosthodontics => "Prosthodontics",
            OralSurgery => "Oral Surgery",
            Code41 => "Routine (Preventive) Dental",
            HomeHealthCare => "Home Health Care",
            HomeHealthPrescriptions => "Home Health Prescriptions",
            HomeHealthVisits => "Home Health Visits",
            Hospice => "Hospice",
            RespiteCare => "Respite Care",
            Hospital => "Hospital",
            HospitalInpatient => "Hospital - Inpatient",
            HospitalRoomAndBoard => "Hospital - Room and Board",
            HospitalOutpatient => "Hospital - Outpatient",
            HospitalEmergencyAccident => "Hospital - Emergency Accident",
            HospitalEmergencyMedical => "Hospital - Emergency Medical",
            HospitalAmbulatorySurgical => "Hospital - Ambulatory Surgical",
            LongTermCare => "Long Term Care",
            MajorMedical => "Major Medical",
            MedicallyRelatedTransportation => "Medically Related Transportation",
            AirTransportation => "Air Transportation",
            Cabulance => "Cabulance",
            LicensedAmbulance => "Licensed Ambulance",
            GeneralBenefits => "General Benefits",
            InVitroFertilization => "In-vitro Fertilization",
            MriCatScan => "MRI/CAT Scan",
            DonorProcedures => "Donor Procedures",
            Acupuncture => "Acupuncture",
            NewbornCare => "Newborn Care",
            Pathology => "Pathology",
            SmokingCessation => "Smoking Cessation",
            WellBabyCare => "Well Baby Care",
            Maternity => "Maternity",
            Transplants => "Transplants",
            AudiologyExam => "Audiology Exam",
            InhalationTherapy => "Inhalation Therapy",
            DiagnosticMedical => "Diagnostic Medical",
            PrivateDutyNursing => "Private Duty Nursing",
            ProstheticDevice => "Prosthetic Device",
            Dialysis => "Dialysis",
            OtologicalExam => "Otological Exam",
            Chemotherapy => "Chemotherapy",
            AllergyTesting => "Allergy Testing",
            Immunizations => "Immunizations",
            RoutinePhysical => "Routine Physical",
            FamilyPlanning => "Family Planning",
            Infertility => "Infertility",
            Abortion => "Abortion",
            Aids => "AIDS",
            EmergencyServices => "Emergency Services",
            Cancer => "Cancer",
            Pharmacy => "Pharmacy",
            FreeStandingPrescriptionDrug => "Free Standing Prescription Drug",
            MailOrderPrescriptionDrug => "Mail Order Prescription Drug",
            BrandNamePrescriptionDrug => "Brand Name Prescription Drug",
            GenericPrescriptionDrug => "Generic Prescription Drug",
            Podiatry => "Podiatry",
            PodiatryOfficeVisits => "Podiatry - Office Visits",
            PodiatryNursingHomeVisits => "Podiatry - Nursing Home Visits",
            Code96 => "Professional (Physician)",
            Anesthesiologist => "Anesthesiologist",
            Code98 => "Professional (Physician) Visit - Office",
            Code99 => "Professional (Physician) Visit - Inpatient",
            CodeA0 => "Professional (Physician) Visit - Outpatient",
            CodeA1 => "Professional (Physician) Visit - Nursing Home",
            CodeA2 => "Professional (Physician) Visit - Skilled Nursing Facility",
            CodeA3 => "Professional (Physician) Visit - Home",
            Psychiatric => "Psychiatric",
            PsychiatricRoomAndBoard => "Psychiatric - Room and Board",
            Psychotherapy => "Psychotherapy",
            PsychiatricInpatient => "Psychiatric - Inpatient",
            PsychiatricOutpatient => "Psychiatric - Outpatient",
            Rehabilitation => "Rehabilitation",
            RehabilitationRoomAndBoard => "Rehabilitation - Room and Board",
            RehabilitationInpatient => "Rehabilitation - Inpatient",
            RehabilitationOutpatient => "Rehabilitation - Outpatient",
            OccupationalTherapy => "Occupational Therapy",
            PhysicalMedicine => "Physical Medicine",
            SpeechTherapy => "Speech Therapy",
            SkilledNursingCare => "Skilled Nursing Care",
            SkilledNursingCareRoomAndBoard => "Skilled Nursing Care - Room and Board",
            SubstanceAbuse => "Substance Abuse",
            Alcoholism => "Alcoholism",
            DrugAddiction => "Drug Addiction",
            CodeAL => "Vision (Optometry)",
            Frames => "Frames",
            RoutineExam => "Routine Exam",
            Lenses => "Lenses",
            NonmedicallyNecessaryPhysical => "Nonmedically Necessary Physical",
            ExperimentalDrugTherapy => "Experimental Drug Therapy",
            NonEscrowOrNonImpoundService => "Non-escrow or Non-impound Service",
            BurnCare => "Burn Care",
            BrandNamePrescriptionDrugFormulary => {
                "Brand Name Prescription Drug - Formulary"
            }
            BrandNamePrescriptionDrugNonFormulary => {
                "Brand Name Prescription Drug - Non-Formulary"
            }
            IndependentMedicalEvaluation => "Independent Medical Evaluation",
            CodeBB => "Partial Hospitalization (Psychiatric)",
            CodeBC => "Day Care (Psychiatric)",
            CognitiveTherapy => "Cognitive Therapy",
            MassageTherapy => "Massage Therapy",
            PulmonaryRehabilitation => "Pulmonary Rehabilitation",
            CardiacRehabilitation => "Cardiac Rehabilitation",
            Pediatric => "Pediatric",
            Nursery => "Nursery",
            Skin => "Skin",
            Orthopedic => "Orthopedic",
            Cardiac => "Cardiac",
            Lymphatic => "Lymphatic",
            Gastrointestinal => "Gastrointestinal",
            Endocrine => "Endocrine",
            Neurology => "Neurology",
            Eye => "Eye",
            InvasiveProcedures => "Invasive Procedures",
            Gynecological => "Gynecological",
            Obstetrical => "Obstetrical",
            ObstetricalGynecological => "Obstetrical/Gynecological",
            MailOrderPrescriptionDrugBrandName => {
                "Mail Order Prescription Drug: Brand Name"
            }
            MailOrderPrescriptionDrugGeneric => "Mail Order Prescription Drug: Generic",
            PhysicianVisitOfficeSick => "Physician Visit - Office: Sick",
            PhysicianVisitOfficeWell => "Physician Visit - Office: Well",
            EscrowOrImpoundService => "Escrow or Impound Service",
            CoronaryCare => "Coronary Care",
            PrivateDutyNursingInpatient => "Private Duty Nursing - Inpatient",
            PrivateDutyNursingHome => "Private Duty Nursing - Home",
            CodeCC => "Surgical Benefits - Professional (Physician)",
            SurgicalBenefitsFacility => "Surgical Benefits - Facility",
            MentalHealthProviderInpatient => "Mental Health Provider - Inpatient",
            MentalHealthProviderOutpatient => "Mental Health Provider - Outpatient",
            MentalHealthFacilityInpatient => "Mental Health Facility - Inpatient",
            MentalHealthFacilityOutpatient => "Mental Health Facility - Outpatient",
            SubstanceAbuseFacilityInpatient => "Substance Abuse Facility - Inpatient",
            SubstanceAbuseFacilityOutpatient => "Substance Abuse Facility - Outpatient",
            ScreeningXRay => "Screening X-ray",
            ScreeningLaboratory => "Screening laboratory",
            CodeCM => "Mammogram, High Risk Patient",
            CodeCN => "Mammogram, Low Risk Patient",
            FluVaccination => "Flu Vaccination",
            EyewearAndEyewearAccessories => "Eyewear and Eyewear Accessories",
            CaseManagement => "Case Management",
            Dermatology => "Dermatology",
            DurableMedicalEquipment => "Durable Medical Equipment",
            DiabeticSupplies => "Diabetic Supplies",
            GenericPrescriptionDrugFormulary => "Generic Prescription Drug - Formulary",
            GenericPrescriptionDrugNonFormulary => {
                "Generic Prescription Drug - Non-Formulary"
            }
            Allergy => "Allergy",
            IntensiveCare => "Intensive Care",
            MentalHealth => "Mental Health",
            NeonatalIntensiveCare => "Neonatal Intensive Care",
            Oncology => "Oncology",
            PhysicalTherapy => "Physical Therapy",
            Pulmonary => "Pulmonary",
            Renal => "Renal",
            ResidentialPsychiatricTreatment => "Residential Psychiatric Treatment",
            TransitionalCare => "Transitional Care",
            TransitionalNurseryCare => "Transitional Nursery Care",
            UrgentCare => "Urgent Care",
        }
    }
    fn from_description(description: &str) -> Option<ServiceTypeCode> {
        {
            use ServiceTypeCode::*;
            match description {
                "Medical Care" => Some(MedicalCare),
                "Surgical" => Some(Surgical),
                "Consultation" => Some(Consultation),
                "Diagnostic X-Ray" => Some(DiagnosticXRay),
                "Diagnostic Lab" => Some(DiagnosticLab),
                "Radiation Therapy" => Some(RadiationTherapy),
                "Anesthesia" => Some(Anesthesia),
                "Surgical Assistance" => Some(SurgicalAssistance),
                "Other Medical" => Some(OtherMedical),
                "Blood Charges" => Some(BloodCharges),
                "Used Durable Medical Equipment" => Some(UsedDurableMedicalEquipment),
                "Durable Medical Equipment Purchase" => {
                    Some(DurableMedicalEquipmentPurchase)
                }
                "Ambulatory Service Center Facility" => {
                    Some(AmbulatoryServiceCenterFacility)
                }
                "Renal Supplies in the Home" => Some(RenalSuppliesInTheHome),
                "Alternate Method Dialysis" => Some(AlternateMethodDialysis),
                "Chronic Renal Disease (CRD) Equipment" => Some(Code16),
                "Pre-Admission Testing" => Some(PreAdmissionTesting),
                "Durable Medical Equipment Rental" => Some(DurableMedicalEquipmentRental),
                "Pneumonia Vaccine" => Some(PneumoniaVaccine),
                "Second Surgical Opinion" => Some(SecondSurgicalOpinion),
                "Third Surgical Opinion" => Some(ThirdSurgicalOpinion),
                "Social Work" => Some(SocialWork),
                "Diagnostic Dental" => Some(DiagnosticDental),
                "Periodontics" => Some(Periodontics),
                "Restorative" => Some(Restorative),
                "Endodontics" => Some(Endodontics),
                "Maxillofacial Prosthetics" => Some(MaxillofacialProsthetics),
                "Adjunctive Dental Services" => Some(AdjunctiveDentalServices),
                "Health Benefit Plan Coverage" => Some(HealthBenefitPlanCoverage),
                "Benefit Disclaimer" => Some(BenefitDisclaimer),
                "Plan Waiting Period" => Some(PlanWaitingPeriod),
                "Chiropractic" => Some(Chiropractic),
                "Chiropractic Office Visits" => Some(ChiropracticOfficeVisits),
                "Dental Care" => Some(DentalCare),
                "Dental Crowns" => Some(DentalCrowns),
                "Dental Accident" => Some(DentalAccident),
                "Orthodontics" => Some(Orthodontics),
                "Prosthodontics" => Some(Prosthodontics),
                "Oral Surgery" => Some(OralSurgery),
                "Routine (Preventive) Dental" => Some(Code41),
                "Home Health Care" => Some(HomeHealthCare),
                "Home Health Prescriptions" => Some(HomeHealthPrescriptions),
                "Home Health Visits" => Some(HomeHealthVisits),
                "Hospice" => Some(Hospice),
                "Respite Care" => Some(RespiteCare),
                "Hospital" => Some(Hospital),
                "Hospital - Inpatient" => Some(HospitalInpatient),
                "Hospital - Room and Board" => Some(HospitalRoomAndBoard),
                "Hospital - Outpatient" => Some(HospitalOutpatient),
                "Hospital - Emergency Accident" => Some(HospitalEmergencyAccident),
                "Hospital - Emergency Medical" => Some(HospitalEmergencyMedical),
                "Hospital - Ambulatory Surgical" => Some(HospitalAmbulatorySurgical),
                "Long Term Care" => Some(LongTermCare),
                "Major Medical" => Some(MajorMedical),
                "Medically Related Transportation" => {
                    Some(MedicallyRelatedTransportation)
                }
                "Air Transportation" => Some(AirTransportation),
                "Cabulance" => Some(Cabulance),
                "Licensed Ambulance" => Some(LicensedAmbulance),
                "General Benefits" => Some(GeneralBenefits),
                "In-vitro Fertilization" => Some(InVitroFertilization),
                "MRI/CAT Scan" => Some(MriCatScan),
                "Donor Procedures" => Some(DonorProcedures),
                "Acupuncture" => Some(Acupuncture),
                "Newborn Care" => Some(NewbornCare),
                "Pathology" => Some(Pathology),
                "Smoking Cessation" => Some(SmokingCessation),
                "Well Baby Care" => Some(WellBabyCare),
                "Maternity" => Some(Maternity),
                "Transplants" => Some(Transplants),
                "Audiology Exam" => Some(AudiologyExam),
                "Inhalation Therapy" => Some(InhalationTherapy),
                "Diagnostic Medical" => Some(DiagnosticMedical),
                "Private Duty Nursing" => Some(PrivateDutyNursing),
                "Prosthetic Device" => Some(ProstheticDevice),
                "Dialysis" => Some(Dialysis),
                "Otological Exam" => Some(OtologicalExam),
                "Chemotherapy" => Some(Chemotherapy),
                "Allergy Testing" => Some(AllergyTesting),
                "Immunizations" => Some(Immunizations),
                "Routine Physical" => Some(RoutinePhysical),
                "Family Planning" => Some(FamilyPlanning),
                "Infertility" => Some(Infertility),
                "Abortion" => Some(Abortion),
                "AIDS" => Some(Aids),
                "Emergency Services" => Some(EmergencyServices),
                "Cancer" => Some(Cancer),
                "Pharmacy" => Some(Pharmacy),
                "Free Standing Prescription Drug" => Some(FreeStandingPrescriptionDrug),
                "Mail Order Prescription Drug" => Some(MailOrderPrescriptionDrug),
                "Brand Name Prescription Drug" => Some(BrandNamePrescriptionDrug),
                "Generic Prescription Drug" => Some(GenericPrescriptionDrug),
                "Podiatry" => Some(Podiatry),
                "Podiatry - Office Visits" => Some(PodiatryOfficeVisits),
                "Podiatry - Nursing Home Visits" => Some(PodiatryNursingHomeVisits),
                "Professional (Physician)" => Some(Code96),
                "Anesthesiologist" => Some(Anesthesiologist),
                "Professional (Physician) Visit - Office" => Some(Code98),
                "Professional (Physician) Visit - Inpatient" => Some(Code99),
                "Professional (Physician) Visit - Outpatient" => Some(CodeA0),
                "Professional (Physician) Visit - Nursing Home" => Some(CodeA1),
                "Professional (Physician) Visit - Skilled Nursing Facility" => {
                    Some(CodeA2)
                }
                "Professional (Physician) Visit - Home" => Some(CodeA3),
                "Psychiatric" => Some(Psychiatric),
                "Psychiatric - Room and Board" => Some(PsychiatricRoomAndBoard),
                "Psychotherapy" => Some(Psychotherapy),
                "Psychiatric - Inpatient" => Some(PsychiatricInpatient),
                "Psychiatric - Outpatient" => Some(PsychiatricOutpatient),
                "Rehabilitation" => Some(Rehabilitation),
                "Rehabilitation - Room and Board" => Some(RehabilitationRoomAndBoard),
                "Rehabilitation - Inpatient" => Some(RehabilitationInpatient),
                "Rehabilitation - Outpatient" => Some(RehabilitationOutpatient),
                "Occupational Therapy" => Some(OccupationalTherapy),
                "Physical Medicine" => Some(PhysicalMedicine),
                "Speech Therapy" => Some(SpeechTherapy),
                "Skilled Nursing Care" => Some(SkilledNursingCare),
                "Skilled Nursing Care - Room and Board" => {
                    Some(SkilledNursingCareRoomAndBoard)
                }
                "Substance Abuse" => Some(SubstanceAbuse),
                "Alcoholism" => Some(Alcoholism),
                "Drug Addiction" => Some(DrugAddiction),
                "Vision (Optometry)" => Some(CodeAL),
                "Frames" => Some(Frames),
                "Routine Exam" => Some(RoutineExam),
                "Lenses" => Some(Lenses),
                "Nonmedically Necessary Physical" => Some(NonmedicallyNecessaryPhysical),
                "Experimental Drug Therapy" => Some(ExperimentalDrugTherapy),
                "Non-escrow or Non-impound Service" => Some(NonEscrowOrNonImpoundService),
                "Burn Care" => Some(BurnCare),
                "Brand Name Prescription Drug - Formulary" => {
                    Some(BrandNamePrescriptionDrugFormulary)
                }
                "Brand Name Prescription Drug - Non-Formulary" => {
                    Some(BrandNamePrescriptionDrugNonFormulary)
                }
                "Independent Medical Evaluation" => Some(IndependentMedicalEvaluation),
                "Partial Hospitalization (Psychiatric)" => Some(CodeBB),
                "Day Care (Psychiatric)" => Some(CodeBC),
                "Cognitive Therapy" => Some(CognitiveTherapy),
                "Massage Therapy" => Some(MassageTherapy),
                "Pulmonary Rehabilitation" => Some(PulmonaryRehabilitation),
                "Cardiac Rehabilitation" => Some(CardiacRehabilitation),
                "Pediatric" => Some(Pediatric),
                "Nursery" => Some(Nursery),
                "Skin" => Some(Skin),
                "Orthopedic" => Some(Orthopedic),
                "Cardiac" => Some(Cardiac),
                "Lymphatic" => Some(Lymphatic),
                "Gastrointestinal" => Some(Gastrointestinal),
                "Endocrine" => Some(Endocrine),
                "Neurology" => Some(Neurology),
                "Eye" => Some(Eye),
                "Invasive Procedures" => Some(InvasiveProcedures),
                "Gynecological" => Some(Gynecological),
                "Obstetrical" => Some(Obstetrical),
                "Obstetrical/Gynecological" => Some(ObstetricalGynecological),
                "Mail Order Prescription Drug: Brand Name" => {
                    Some(MailOrderPrescriptionDrugBrandName)
                }
                "Mail Order Prescription Drug: Generic" => {
                    Some(MailOrderPrescriptionDrugGeneric)
                }
                "Physician Visit - Office: Sick" => Some(PhysicianVisitOfficeSick),
                "Physician Visit - Office: Well" => Some(PhysicianVisitOfficeWell),
                "Escrow or Impound Service" => Some(EscrowOrImpoundService),
                "Coronary Care" => Some(CoronaryCare),
                "Private Duty Nursing - Inpatient" => Some(PrivateDutyNursingInpatient),
                "Private Duty Nursing - Home" => Some(PrivateDutyNursingHome),
                "Surgical Benefits - Professional (Physician)" => Some(CodeCC),
                "Surgical Benefits - Facility" => Some(SurgicalBenefitsFacility),
                "Mental Health Provider - Inpatient" => {
                    Some(MentalHealthProviderInpatient)
                }
                "Mental Health Provider - Outpatient" => {
                    Some(MentalHealthProviderOutpatient)
                }
                "Mental Health Facility - Inpatient" => {
                    Some(MentalHealthFacilityInpatient)
                }
                "Mental Health Facility - Outpatient" => {
                    Some(MentalHealthFacilityOutpatient)
                }
                "Substance Abuse Facility - Inpatient" => {
                    Some(SubstanceAbuseFacilityInpatient)
                }
                "Substance Abuse Facility - Outpatient" => {
                    Some(SubstanceAbuseFacilityOutpatient)
                }
                "Screening X-ray" => Some(ScreeningXRay),
                "Screening laboratory" => Some(ScreeningLaboratory),
                "Mammogram, High Risk Patient" => Some(CodeCM),
                "Mammogram, Low Risk Patient" => Some(CodeCN),
                "Flu Vaccination" => Some(FluVaccination),
                "Eyewear and Eyewear Accessories" => Some(EyewearAndEyewearAccessories),
                "Case Management" => Some(CaseManagement),
                "Dermatology" => Some(Dermatology),
                "Durable Medical Equipment" => Some(DurableMedicalEquipment),
                "Diabetic Supplies" => Some(DiabeticSupplies),
                "Generic Prescription Drug - Formulary" => {
                    Some(GenericPrescriptionDrugFormulary)
                }
                "Generic Prescription Drug - Non-Formulary" => {
                    Some(GenericPrescriptionDrugNonFormulary)
                }
                "Allergy" => Some(Allergy),
                "Intensive Care" => Some(IntensiveCare),
                "Mental Health" => Some(MentalHealth),
                "Neonatal Intensive Care" => Some(NeonatalIntensiveCare),
                "Oncology" => Some(Oncology),
                "Physical Therapy" => Some(PhysicalTherapy),
                "Pulmonary" => Some(Pulmonary),
                "Renal" => Some(Renal),
                "Residential Psychiatric Treatment" => {
                    Some(ResidentialPsychiatricTreatment)
                }
                "Transitional Care" => Some(TransitionalCare),
                "Transitional Nursery Care" => Some(TransitionalNurseryCare),
                "Urgent Care" => Some(UrgentCare),
                _ => None,
            }
        }
    }
}
impl Serialize for ServiceTypeCode {
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
    type Value = ServiceTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Service Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ServiceTypeCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Service Type Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ServiceTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Service Type Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for ServiceTypeCode {
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