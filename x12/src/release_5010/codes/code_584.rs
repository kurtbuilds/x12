use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**584

See docs at <https://www.stedi.com/edi/x12-005010/element/584>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmploymentStatusCode {
    ///00 - Substitute
    Substitute,
    ///AA - Leave of Absence with Pay
    LeaveOfAbsenceWithPay,
    ///AB - Leave of Absence without Pay
    LeaveOfAbsenceWithoutPay,
    ///AC - Active
    Active,
    ///AD - Apprenticeship Full-time
    ApprenticeshipFullTime,
    ///AE - Active Reserve
    ActiveReserve,
    ///AF - Flexible Work Plan
    FlexibleWorkPlan,
    ///AG - Alerted
    Alerted,
    ///AH - Assigned
    Assigned,
    ///AI - Affiliated with Outside Organization
    AffiliatedWithOutsideOrganization,
    ///AJ - Adjunct
    Adjunct,
    ///AO - Active Military - Overseas
    ActiveMilitaryOverseas,
    ///AP - Apprenticeship Part-time
    ApprenticeshipPartTime,
    ///AQ - Apprenticeship
    Apprenticeship,
    ///AS - Academy Student
    AcademyStudent,
    ///AT - Presidential Appointee
    PresidentialAppointee,
    ///AU - Active Military - USA
    ActiveMilitaryUsa,
    ///CA - Non-applicable Employment Status Category
    NonApplicableEmploymentStatusCategory,
    ///CC - Contractor
    Contractor,
    ///CO - Consolidated Omnibus Budget Reconciliation Act (COBRA)
    CodeCO,
    ///CT - Continued
    Continued,
    ///DC - Discharged or Terminated for Cause
    DischargedOrTerminatedForCause,
    ///DD - Dishonorably Discharged
    DishonorablyDischarged,
    ///DI - Deceased
    Deceased,
    ///DQ - Disqualified: Medical or Physical Condition
    DisqualifiedMedicalOrPhysicalCondition,
    ///DR - Disqualified: Other
    DisqualifiedOther,
    ///DS - Disabled
    Disabled,
    ///EO - Employed by Outside Organization
    EmployedByOutsideOrganization,
    ///FA - Furloughed: Job Abolished, Force Reduction
    CodeFA,
    ///FB - Furloughed: Bumped or Displaced
    FurloughedBumpedOrDisplaced,
    ///FC - Furloughed: Facility Closed
    FurloughedFacilityClosed,
    ///FO - Furloughed: Other
    FurloughedOther,
    ///FT - Full-time
    FullTime,
    ///HD - Honorably Discharged
    HonorablyDischarged,
    ///IA - Inactive
    Inactive,
    ///IR - Inactive Reserves
    InactiveReserves,
    ///L1 - Leave of Absence
    LeaveOfAbsence,
    ///L2 - Administrative Leave of Absence
    AdministrativeLeaveOfAbsence,
    ///L3 - Annual Leave of Absence
    AnnualLeaveOfAbsence,
    ///L4 - Leave of Absence due to Bereavement
    LeaveOfAbsenceDueToBereavement,
    ///L5 - Jury Duty
    JuryDuty,
    ///L6 - Suspension
    Suspension,
    ///L7 - Sabbatical Leave of Absence
    SabbaticalLeaveOfAbsence,
    ///LA - Leave of Absence: Personal
    LeaveOfAbsencePersonal,
    ///LE - Leave of Absence: Education
    LeaveOfAbsenceEducation,
    ///LF - Leave of Absence: Family Medical Leave Act (FMLA)
    CodeLF,
    ///LM - Leave of Absence: Maternity
    LeaveOfAbsenceMaternity,
    ///LO - Leave of Absence for Non-Military Government Request Other Than Jury Duty
    LeaveOfAbsenceForNonMilitaryGovernmentRequestOtherThanJuryDuty,
    ///LS - Leave of Absence: Sickness
    LeaveOfAbsenceSickness,
    ///LU - Leave of Absence: Union
    LeaveOfAbsenceUnion,
    ///LW - Leave of Absence: Without Permission, Unauthorized
    CodeLW,
    ///LX - Leave of Absence: Military
    LeaveOfAbsenceMilitary,
    ///NE - Not Employed
    NotEmployed,
    ///OS - On Strike
    OnStrike,
    ///OT - Other
    Other,
    ///PA - Promoted
    Promoted,
    ///PC - Part-time Contractual
    PartTimeContractual,
    ///PE - Plan to Enlist
    PlanToEnlist,
    ///PM - Permanent
    Permanent,
    ///PN - Part-time Noncontractual
    PartTimeNoncontractual,
    ///PR - Probationary
    Probationary,
    ///PT - Part-time
    PartTime,
    ///PV - Previous
    Previous,
    ///PW - Piece Worker
    PieceWorker,
    ///RA - Resigned: Retired
    ResignedRetired,
    ///RB - Relocated
    Relocated,
    ///RC - Reassigned
    Reassigned,
    ///RD - Resigned: Moved
    ResignedMoved,
    ///RE - Recommissioned
    Recommissioned,
    ///RI - Resigned: Injury
    ResignedInjury,
    ///RM - Retired Military - Overseas
    RetiredMilitaryOverseas,
    ///RP - Resigned: Personal Reasons
    ResignedPersonalReasons,
    ///RR - Retired Without Recall
    RetiredWithoutRecall,
    ///RT - Retired
    Retired,
    ///RU - Retired Military - USA
    RetiredMilitaryUsa,
    ///RW - Dual Retired Status
    DualRetiredStatus,
    ///SA - Resigned: Accepted Separation Allowance
    ResignedAcceptedSeparationAllowance,
    ///SB - Separated
    Separated,
    ///SE - Self-Employed
    SelfEmployed,
    ///SL - Seasonal
    Seasonal,
    ///SU - Suspended
    Suspended,
    ///TE - Terminated
    Terminated,
    ///TF - Temporary Full-Time
    TemporaryFullTime,
    ///TM - Temporary
    Temporary,
    ///TN - Tenured
    Tenured,
    ///TP - Temporary Part-Time
    TemporaryPartTime,
    ///TR - Transferred
    Transferred,
    ///UK - Unknown
    Unknown,
    ///VO - Volunteer
    Volunteer,
    ///XD - Extra Duties Not Requiring Certification
    ExtraDutiesNotRequiringCertification,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl EmploymentStatusCode {
    pub fn code(&self) -> &str {
        {
            use EmploymentStatusCode::*;
            match self {
                Substitute => "00",
                LeaveOfAbsenceWithPay => "AA",
                LeaveOfAbsenceWithoutPay => "AB",
                Active => "AC",
                ApprenticeshipFullTime => "AD",
                ActiveReserve => "AE",
                FlexibleWorkPlan => "AF",
                Alerted => "AG",
                Assigned => "AH",
                AffiliatedWithOutsideOrganization => "AI",
                Adjunct => "AJ",
                ActiveMilitaryOverseas => "AO",
                ApprenticeshipPartTime => "AP",
                Apprenticeship => "AQ",
                AcademyStudent => "AS",
                PresidentialAppointee => "AT",
                ActiveMilitaryUsa => "AU",
                NonApplicableEmploymentStatusCategory => "CA",
                Contractor => "CC",
                CodeCO => "CO",
                Continued => "CT",
                DischargedOrTerminatedForCause => "DC",
                DishonorablyDischarged => "DD",
                Deceased => "DI",
                DisqualifiedMedicalOrPhysicalCondition => "DQ",
                DisqualifiedOther => "DR",
                Disabled => "DS",
                EmployedByOutsideOrganization => "EO",
                CodeFA => "FA",
                FurloughedBumpedOrDisplaced => "FB",
                FurloughedFacilityClosed => "FC",
                FurloughedOther => "FO",
                FullTime => "FT",
                HonorablyDischarged => "HD",
                Inactive => "IA",
                InactiveReserves => "IR",
                LeaveOfAbsence => "L1",
                AdministrativeLeaveOfAbsence => "L2",
                AnnualLeaveOfAbsence => "L3",
                LeaveOfAbsenceDueToBereavement => "L4",
                JuryDuty => "L5",
                Suspension => "L6",
                SabbaticalLeaveOfAbsence => "L7",
                LeaveOfAbsencePersonal => "LA",
                LeaveOfAbsenceEducation => "LE",
                CodeLF => "LF",
                LeaveOfAbsenceMaternity => "LM",
                LeaveOfAbsenceForNonMilitaryGovernmentRequestOtherThanJuryDuty => "LO",
                LeaveOfAbsenceSickness => "LS",
                LeaveOfAbsenceUnion => "LU",
                CodeLW => "LW",
                LeaveOfAbsenceMilitary => "LX",
                NotEmployed => "NE",
                OnStrike => "OS",
                Other => "OT",
                Promoted => "PA",
                PartTimeContractual => "PC",
                PlanToEnlist => "PE",
                Permanent => "PM",
                PartTimeNoncontractual => "PN",
                Probationary => "PR",
                PartTime => "PT",
                Previous => "PV",
                PieceWorker => "PW",
                ResignedRetired => "RA",
                Relocated => "RB",
                Reassigned => "RC",
                ResignedMoved => "RD",
                Recommissioned => "RE",
                ResignedInjury => "RI",
                RetiredMilitaryOverseas => "RM",
                ResignedPersonalReasons => "RP",
                RetiredWithoutRecall => "RR",
                Retired => "RT",
                RetiredMilitaryUsa => "RU",
                DualRetiredStatus => "RW",
                ResignedAcceptedSeparationAllowance => "SA",
                Separated => "SB",
                SelfEmployed => "SE",
                Seasonal => "SL",
                Suspended => "SU",
                Terminated => "TE",
                TemporaryFullTime => "TF",
                Temporary => "TM",
                Tenured => "TN",
                TemporaryPartTime => "TP",
                Transferred => "TR",
                Unknown => "UK",
                Volunteer => "VO",
                ExtraDutiesNotRequiringCertification => "XD",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<EmploymentStatusCode> {
        use EmploymentStatusCode::*;
        match code {
            b"00" => Some(Substitute),
            b"AA" => Some(LeaveOfAbsenceWithPay),
            b"AB" => Some(LeaveOfAbsenceWithoutPay),
            b"AC" => Some(Active),
            b"AD" => Some(ApprenticeshipFullTime),
            b"AE" => Some(ActiveReserve),
            b"AF" => Some(FlexibleWorkPlan),
            b"AG" => Some(Alerted),
            b"AH" => Some(Assigned),
            b"AI" => Some(AffiliatedWithOutsideOrganization),
            b"AJ" => Some(Adjunct),
            b"AO" => Some(ActiveMilitaryOverseas),
            b"AP" => Some(ApprenticeshipPartTime),
            b"AQ" => Some(Apprenticeship),
            b"AS" => Some(AcademyStudent),
            b"AT" => Some(PresidentialAppointee),
            b"AU" => Some(ActiveMilitaryUsa),
            b"CA" => Some(NonApplicableEmploymentStatusCategory),
            b"CC" => Some(Contractor),
            b"CO" => Some(CodeCO),
            b"CT" => Some(Continued),
            b"DC" => Some(DischargedOrTerminatedForCause),
            b"DD" => Some(DishonorablyDischarged),
            b"DI" => Some(Deceased),
            b"DQ" => Some(DisqualifiedMedicalOrPhysicalCondition),
            b"DR" => Some(DisqualifiedOther),
            b"DS" => Some(Disabled),
            b"EO" => Some(EmployedByOutsideOrganization),
            b"FA" => Some(CodeFA),
            b"FB" => Some(FurloughedBumpedOrDisplaced),
            b"FC" => Some(FurloughedFacilityClosed),
            b"FO" => Some(FurloughedOther),
            b"FT" => Some(FullTime),
            b"HD" => Some(HonorablyDischarged),
            b"IA" => Some(Inactive),
            b"IR" => Some(InactiveReserves),
            b"L1" => Some(LeaveOfAbsence),
            b"L2" => Some(AdministrativeLeaveOfAbsence),
            b"L3" => Some(AnnualLeaveOfAbsence),
            b"L4" => Some(LeaveOfAbsenceDueToBereavement),
            b"L5" => Some(JuryDuty),
            b"L6" => Some(Suspension),
            b"L7" => Some(SabbaticalLeaveOfAbsence),
            b"LA" => Some(LeaveOfAbsencePersonal),
            b"LE" => Some(LeaveOfAbsenceEducation),
            b"LF" => Some(CodeLF),
            b"LM" => Some(LeaveOfAbsenceMaternity),
            b"LO" => Some(LeaveOfAbsenceForNonMilitaryGovernmentRequestOtherThanJuryDuty),
            b"LS" => Some(LeaveOfAbsenceSickness),
            b"LU" => Some(LeaveOfAbsenceUnion),
            b"LW" => Some(CodeLW),
            b"LX" => Some(LeaveOfAbsenceMilitary),
            b"NE" => Some(NotEmployed),
            b"OS" => Some(OnStrike),
            b"OT" => Some(Other),
            b"PA" => Some(Promoted),
            b"PC" => Some(PartTimeContractual),
            b"PE" => Some(PlanToEnlist),
            b"PM" => Some(Permanent),
            b"PN" => Some(PartTimeNoncontractual),
            b"PR" => Some(Probationary),
            b"PT" => Some(PartTime),
            b"PV" => Some(Previous),
            b"PW" => Some(PieceWorker),
            b"RA" => Some(ResignedRetired),
            b"RB" => Some(Relocated),
            b"RC" => Some(Reassigned),
            b"RD" => Some(ResignedMoved),
            b"RE" => Some(Recommissioned),
            b"RI" => Some(ResignedInjury),
            b"RM" => Some(RetiredMilitaryOverseas),
            b"RP" => Some(ResignedPersonalReasons),
            b"RR" => Some(RetiredWithoutRecall),
            b"RT" => Some(Retired),
            b"RU" => Some(RetiredMilitaryUsa),
            b"RW" => Some(DualRetiredStatus),
            b"SA" => Some(ResignedAcceptedSeparationAllowance),
            b"SB" => Some(Separated),
            b"SE" => Some(SelfEmployed),
            b"SL" => Some(Seasonal),
            b"SU" => Some(Suspended),
            b"TE" => Some(Terminated),
            b"TF" => Some(TemporaryFullTime),
            b"TM" => Some(Temporary),
            b"TN" => Some(Tenured),
            b"TP" => Some(TemporaryPartTime),
            b"TR" => Some(Transferred),
            b"UK" => Some(Unknown),
            b"VO" => Some(Volunteer),
            b"XD" => Some(ExtraDutiesNotRequiringCertification),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use EmploymentStatusCode::*;
        match self {
            Substitute => "Substitute",
            LeaveOfAbsenceWithPay => "Leave of Absence with Pay",
            LeaveOfAbsenceWithoutPay => "Leave of Absence without Pay",
            Active => "Active",
            ApprenticeshipFullTime => "Apprenticeship Full-time",
            ActiveReserve => "Active Reserve",
            FlexibleWorkPlan => "Flexible Work Plan",
            Alerted => "Alerted",
            Assigned => "Assigned",
            AffiliatedWithOutsideOrganization => "Affiliated with Outside Organization",
            Adjunct => "Adjunct",
            ActiveMilitaryOverseas => "Active Military - Overseas",
            ApprenticeshipPartTime => "Apprenticeship Part-time",
            Apprenticeship => "Apprenticeship",
            AcademyStudent => "Academy Student",
            PresidentialAppointee => "Presidential Appointee",
            ActiveMilitaryUsa => "Active Military - USA",
            NonApplicableEmploymentStatusCategory => {
                "Non-applicable Employment Status Category"
            }
            Contractor => "Contractor",
            CodeCO => "Consolidated Omnibus Budget Reconciliation Act (COBRA)",
            Continued => "Continued",
            DischargedOrTerminatedForCause => "Discharged or Terminated for Cause",
            DishonorablyDischarged => "Dishonorably Discharged",
            Deceased => "Deceased",
            DisqualifiedMedicalOrPhysicalCondition => {
                "Disqualified: Medical or Physical Condition"
            }
            DisqualifiedOther => "Disqualified: Other",
            Disabled => "Disabled",
            EmployedByOutsideOrganization => "Employed by Outside Organization",
            CodeFA => "Furloughed: Job Abolished, Force Reduction",
            FurloughedBumpedOrDisplaced => "Furloughed: Bumped or Displaced",
            FurloughedFacilityClosed => "Furloughed: Facility Closed",
            FurloughedOther => "Furloughed: Other",
            FullTime => "Full-time",
            HonorablyDischarged => "Honorably Discharged",
            Inactive => "Inactive",
            InactiveReserves => "Inactive Reserves",
            LeaveOfAbsence => "Leave of Absence",
            AdministrativeLeaveOfAbsence => "Administrative Leave of Absence",
            AnnualLeaveOfAbsence => "Annual Leave of Absence",
            LeaveOfAbsenceDueToBereavement => "Leave of Absence due to Bereavement",
            JuryDuty => "Jury Duty",
            Suspension => "Suspension",
            SabbaticalLeaveOfAbsence => "Sabbatical Leave of Absence",
            LeaveOfAbsencePersonal => "Leave of Absence: Personal",
            LeaveOfAbsenceEducation => "Leave of Absence: Education",
            CodeLF => "Leave of Absence: Family Medical Leave Act (FMLA)",
            LeaveOfAbsenceMaternity => "Leave of Absence: Maternity",
            LeaveOfAbsenceForNonMilitaryGovernmentRequestOtherThanJuryDuty => {
                "Leave of Absence for Non-Military Government Request Other Than Jury Duty"
            }
            LeaveOfAbsenceSickness => "Leave of Absence: Sickness",
            LeaveOfAbsenceUnion => "Leave of Absence: Union",
            CodeLW => "Leave of Absence: Without Permission, Unauthorized",
            LeaveOfAbsenceMilitary => "Leave of Absence: Military",
            NotEmployed => "Not Employed",
            OnStrike => "On Strike",
            Other => "Other",
            Promoted => "Promoted",
            PartTimeContractual => "Part-time Contractual",
            PlanToEnlist => "Plan to Enlist",
            Permanent => "Permanent",
            PartTimeNoncontractual => "Part-time Noncontractual",
            Probationary => "Probationary",
            PartTime => "Part-time",
            Previous => "Previous",
            PieceWorker => "Piece Worker",
            ResignedRetired => "Resigned: Retired",
            Relocated => "Relocated",
            Reassigned => "Reassigned",
            ResignedMoved => "Resigned: Moved",
            Recommissioned => "Recommissioned",
            ResignedInjury => "Resigned: Injury",
            RetiredMilitaryOverseas => "Retired Military - Overseas",
            ResignedPersonalReasons => "Resigned: Personal Reasons",
            RetiredWithoutRecall => "Retired Without Recall",
            Retired => "Retired",
            RetiredMilitaryUsa => "Retired Military - USA",
            DualRetiredStatus => "Dual Retired Status",
            ResignedAcceptedSeparationAllowance => {
                "Resigned: Accepted Separation Allowance"
            }
            Separated => "Separated",
            SelfEmployed => "Self-Employed",
            Seasonal => "Seasonal",
            Suspended => "Suspended",
            Terminated => "Terminated",
            TemporaryFullTime => "Temporary Full-Time",
            Temporary => "Temporary",
            Tenured => "Tenured",
            TemporaryPartTime => "Temporary Part-Time",
            Transferred => "Transferred",
            Unknown => "Unknown",
            Volunteer => "Volunteer",
            ExtraDutiesNotRequiringCertification => {
                "Extra Duties Not Requiring Certification"
            }
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<EmploymentStatusCode> {
        {
            use EmploymentStatusCode::*;
            match description {
                "Substitute" => Some(Substitute),
                "Leave of Absence with Pay" => Some(LeaveOfAbsenceWithPay),
                "Leave of Absence without Pay" => Some(LeaveOfAbsenceWithoutPay),
                "Active" => Some(Active),
                "Apprenticeship Full-time" => Some(ApprenticeshipFullTime),
                "Active Reserve" => Some(ActiveReserve),
                "Flexible Work Plan" => Some(FlexibleWorkPlan),
                "Alerted" => Some(Alerted),
                "Assigned" => Some(Assigned),
                "Affiliated with Outside Organization" => {
                    Some(AffiliatedWithOutsideOrganization)
                }
                "Adjunct" => Some(Adjunct),
                "Active Military - Overseas" => Some(ActiveMilitaryOverseas),
                "Apprenticeship Part-time" => Some(ApprenticeshipPartTime),
                "Apprenticeship" => Some(Apprenticeship),
                "Academy Student" => Some(AcademyStudent),
                "Presidential Appointee" => Some(PresidentialAppointee),
                "Active Military - USA" => Some(ActiveMilitaryUsa),
                "Non-applicable Employment Status Category" => {
                    Some(NonApplicableEmploymentStatusCategory)
                }
                "Contractor" => Some(Contractor),
                "Consolidated Omnibus Budget Reconciliation Act (COBRA)" => Some(CodeCO),
                "Continued" => Some(Continued),
                "Discharged or Terminated for Cause" => {
                    Some(DischargedOrTerminatedForCause)
                }
                "Dishonorably Discharged" => Some(DishonorablyDischarged),
                "Deceased" => Some(Deceased),
                "Disqualified: Medical or Physical Condition" => {
                    Some(DisqualifiedMedicalOrPhysicalCondition)
                }
                "Disqualified: Other" => Some(DisqualifiedOther),
                "Disabled" => Some(Disabled),
                "Employed by Outside Organization" => Some(EmployedByOutsideOrganization),
                "Furloughed: Job Abolished, Force Reduction" => Some(CodeFA),
                "Furloughed: Bumped or Displaced" => Some(FurloughedBumpedOrDisplaced),
                "Furloughed: Facility Closed" => Some(FurloughedFacilityClosed),
                "Furloughed: Other" => Some(FurloughedOther),
                "Full-time" => Some(FullTime),
                "Honorably Discharged" => Some(HonorablyDischarged),
                "Inactive" => Some(Inactive),
                "Inactive Reserves" => Some(InactiveReserves),
                "Leave of Absence" => Some(LeaveOfAbsence),
                "Administrative Leave of Absence" => Some(AdministrativeLeaveOfAbsence),
                "Annual Leave of Absence" => Some(AnnualLeaveOfAbsence),
                "Leave of Absence due to Bereavement" => {
                    Some(LeaveOfAbsenceDueToBereavement)
                }
                "Jury Duty" => Some(JuryDuty),
                "Suspension" => Some(Suspension),
                "Sabbatical Leave of Absence" => Some(SabbaticalLeaveOfAbsence),
                "Leave of Absence: Personal" => Some(LeaveOfAbsencePersonal),
                "Leave of Absence: Education" => Some(LeaveOfAbsenceEducation),
                "Leave of Absence: Family Medical Leave Act (FMLA)" => Some(CodeLF),
                "Leave of Absence: Maternity" => Some(LeaveOfAbsenceMaternity),
                "Leave of Absence for Non-Military Government Request Other Than Jury Duty" => {
                    Some(LeaveOfAbsenceForNonMilitaryGovernmentRequestOtherThanJuryDuty)
                }
                "Leave of Absence: Sickness" => Some(LeaveOfAbsenceSickness),
                "Leave of Absence: Union" => Some(LeaveOfAbsenceUnion),
                "Leave of Absence: Without Permission, Unauthorized" => Some(CodeLW),
                "Leave of Absence: Military" => Some(LeaveOfAbsenceMilitary),
                "Not Employed" => Some(NotEmployed),
                "On Strike" => Some(OnStrike),
                "Other" => Some(Other),
                "Promoted" => Some(Promoted),
                "Part-time Contractual" => Some(PartTimeContractual),
                "Plan to Enlist" => Some(PlanToEnlist),
                "Permanent" => Some(Permanent),
                "Part-time Noncontractual" => Some(PartTimeNoncontractual),
                "Probationary" => Some(Probationary),
                "Part-time" => Some(PartTime),
                "Previous" => Some(Previous),
                "Piece Worker" => Some(PieceWorker),
                "Resigned: Retired" => Some(ResignedRetired),
                "Relocated" => Some(Relocated),
                "Reassigned" => Some(Reassigned),
                "Resigned: Moved" => Some(ResignedMoved),
                "Recommissioned" => Some(Recommissioned),
                "Resigned: Injury" => Some(ResignedInjury),
                "Retired Military - Overseas" => Some(RetiredMilitaryOverseas),
                "Resigned: Personal Reasons" => Some(ResignedPersonalReasons),
                "Retired Without Recall" => Some(RetiredWithoutRecall),
                "Retired" => Some(Retired),
                "Retired Military - USA" => Some(RetiredMilitaryUsa),
                "Dual Retired Status" => Some(DualRetiredStatus),
                "Resigned: Accepted Separation Allowance" => {
                    Some(ResignedAcceptedSeparationAllowance)
                }
                "Separated" => Some(Separated),
                "Self-Employed" => Some(SelfEmployed),
                "Seasonal" => Some(Seasonal),
                "Suspended" => Some(Suspended),
                "Terminated" => Some(Terminated),
                "Temporary Full-Time" => Some(TemporaryFullTime),
                "Temporary" => Some(Temporary),
                "Tenured" => Some(Tenured),
                "Temporary Part-Time" => Some(TemporaryPartTime),
                "Transferred" => Some(Transferred),
                "Unknown" => Some(Unknown),
                "Volunteer" => Some(Volunteer),
                "Extra Duties Not Requiring Certification" => {
                    Some(ExtraDutiesNotRequiringCertification)
                }
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for EmploymentStatusCode {
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
    type Value = EmploymentStatusCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Employment Status Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        EmploymentStatusCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Employment Status Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        EmploymentStatusCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Employment Status Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for EmploymentStatusCode {
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