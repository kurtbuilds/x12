use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1069

See docs at <https://www.stedi.com/edi/x12/element/1069>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndividualRelationshipCode {
    ///00 - Guarantor
    Guarantor,
    ///01 - Spouse
    Spouse,
    ///02 - Son or Daughter
    SonOrDaughter,
    ///03 - Father or Mother
    FatherOrMother,
    ///04 - Grandfather or Grandmother
    GrandfatherOrGrandmother,
    ///05 - Grandson or Granddaughter
    GrandsonOrGranddaughter,
    ///06 - Uncle or Aunt
    UncleOrAunt,
    ///07 - Nephew or Niece
    NephewOrNiece,
    ///08 - Cousin
    Cousin,
    ///09 - Adopted Child
    AdoptedChild,
    ///10 - Foster Child
    FosterChild,
    ///11 - Son-in-law or Daughter-in-law
    SonInLawOrDaughterInLaw,
    ///12 - Brother-in-law or Sister-in-law
    BrotherInLawOrSisterInLaw,
    ///13 - Mother-in-law or Father-in-law
    MotherInLawOrFatherInLaw,
    ///14 - Brother or Sister
    BrotherOrSister,
    ///15 - Ward
    Ward,
    ///16 - Stepparent
    Stepparent,
    ///17 - Stepson or Stepdaughter
    StepsonOrStepdaughter,
    ///18 - Self
    Self_,
    ///19 - Child
    Child,
    ///20 - Employee
    Employee,
    ///21 - Unknown
    Unknown,
    ///22 - Handicapped Dependent
    HandicappedDependent,
    ///23 - Sponsored Dependent
    SponsoredDependent,
    ///24 - Dependent of a Minor Dependent
    DependentOfAMinorDependent,
    ///25 - Ex-spouse
    ExSpouse,
    ///26 - Guardian
    Guardian,
    ///27 - Student
    Student,
    ///28 - Friend
    Friend,
    ///29 - Significant Other
    SignificantOther,
    ///30 - Both Parents
    BothParents,
    ///31 - Court Appointed Guardian
    CourtAppointedGuardian,
    ///32 - Mother
    Mother,
    ///33 - Father
    Father,
    ///34 - Other Adult
    OtherAdult,
    ///36 - Emancipated Minor
    EmancipatedMinor,
    ///37 - Agency Representative
    AgencyRepresentative,
    ///38 - Collateral Dependent
    CollateralDependent,
    ///39 - Organ Donor
    OrganDonor,
    ///40 - Cadaver Donor
    CadaverDonor,
    ///41 - Injured Plaintiff
    InjuredPlaintiff,
    ///43 - Child Where Insured Has No Financial Responsibility
    ChildWhereInsuredHasNoFinancialResponsibility,
    ///45 - Widow
    Widow,
    ///46 - Widower
    Widower,
    ///47 - State Fund
    StateFund,
    ///48 - Stepfather
    Stepfather,
    ///49 - Stepmother
    Stepmother,
    ///50 - Foster Parent
    FosterParent,
    ///51 - Emergency Contact
    EmergencyContact,
    ///52 - Employer
    Employer,
    ///53 - Life Partner
    LifePartner,
    ///55 - Adopted Daughter
    AdoptedDaughter,
    ///56 - Adopted Son
    AdoptedSon,
    ///57 - Adoptive Father
    AdoptiveFather,
    ///58 - Adoptive Mother
    AdoptiveMother,
    ///59 - Adoptive Parents
    AdoptiveParents,
    ///60 - Annuitant
    Annuitant,
    ///61 - Aunt
    Aunt,
    ///62 - Brother
    Brother,
    ///63 - Brother-in-Law
    BrotherInLaw,
    ///64 - Business
    Business,
    ///65 - Business Associate
    BusinessAssociate,
    ///66 - Business Insurance Trust
    BusinessInsuranceTrust,
    ///67 - Business Partner
    BusinessPartner,
    ///68 - Charity
    Charity,
    ///70 - Children of Marriage
    ChildrenOfMarriage,
    ///71 - Company
    Company,
    ///72 - Corporation
    Corporation,
    ///73 - Creditor
    Creditor,
    ///74 - Daughter
    Daughter,
    ///75 - Daughter-in-Law
    DaughterInLaw,
    ///76 - Dependent
    Dependent,
    ///78 - Estate
    Estate,
    ///79 - Ex-wife
    ExWife,
    ///80 - Family Member
    FamilyMember,
    ///81 - Father-in-Law
    FatherInLaw,
    ///82 - Fiancé (Male)
    Code82,
    ///83 - Fiancée (Female)
    Code83,
    ///84 - Fiduciary
    Fiduciary,
    ///86 - Foster Daughter
    FosterDaughter,
    ///87 - Foster Father
    FosterFather,
    ///88 - Foster Mother
    FosterMother,
    ///90 - Foster Son
    FosterSon,
    ///91 - God Daughter
    GodDaughter,
    ///92 - God Father
    GodFather,
    ///93 - God Parents
    GodParents,
    ///94 - God Son
    GodSon,
    ///95 - Grandchildren
    Grandchildren,
    ///96 - Granddaughter
    Granddaughter,
    ///97 - Grandfather
    Grandfather,
    ///98 - Grandmother
    Grandmother,
    ///99 - Grandparents
    Grandparents,
    ///A1 - Grandson
    Grandson,
    ///A2 - Great Aunt
    GreatAunt,
    ///A3 - Ex-husband
    ExHusband,
    ///A4 - Half Brother
    HalfBrother,
    ///A5 - Half Sister
    HalfSister,
    ///A6 - Husband
    Husband,
    ///A7 - Institution
    Institution,
    ///A8 - Mortgage Holder
    MortgageHolder,
    ///A9 - Mother-in-Law
    MotherInLaw,
    ///B1 - Nephew
    Nephew,
    ///B2 - Niece
    Niece,
    ///B3 - Parents-in-Law
    ParentsInLaw,
    ///B4 - Partnership
    Partnership,
    ///B5 - Partner
    Partner,
    ///B6 - Personal Insurance Trust
    PersonalInsuranceTrust,
    ///B7 - Sister
    Sister,
    ///B8 - Sister-in-Law
    SisterInLaw,
    ///B9 - Sole Proprietorship
    SoleProprietorship,
    ///C1 - Son
    Son,
    ///C2 - Son-in-Law
    SonInLaw,
    ///C3 - Step Brother
    StepBrother,
    ///C4 - Step Children
    StepChildren,
    ///C5 - Step Daughter
    StepDaughter,
    ///C8 - Step Sister
    StepSister,
    ///C9 - Step Son
    StepSon,
    ///D1 - Trust
    Trust,
    ///D2 - Trustee
    Trustee,
    ///D3 - Uncle
    Uncle,
    ///D4 - Wife
    Wife,
    ///D5 - Teacher
    Teacher,
    ///D6 - School Counselor
    SchoolCounselor,
    ///D7 - School Principal
    SchoolPrincipal,
    ///D8 - Other School Administrator
    OtherSchoolAdministrator,
    ///D9 - Coach
    Coach,
    ///DC - Child of a Domestic Partner
    ChildOfADomesticPartner,
    ///E1 - Activity Sponsor
    ActivitySponsor,
    ///E2 - Supervisor
    Supervisor,
    ///E3 - Co-worker
    CoWorker,
    ///E4 - Minister or Priest
    MinisterOrPriest,
    ///E5 - Ecclesiastical or Religious Leader
    EcclesiasticalOrReligiousLeader,
    ///E6 - God Mother
    GodMother,
    ///E7 - Probation Officer
    ProbationOfficer,
    ///E8 - Accountant
    Accountant,
    ///E9 - Advisor
    Advisor,
    ///F1 - Alma Mater
    AlmaMater,
    ///F2 - Applicant
    Applicant,
    ///F3 - Banker
    Banker,
    ///F6 - Clergyman
    Clergyman,
    ///F7 - Client
    Client,
    ///F8 - Club or Organization Officer
    ClubOrOrganizationOfficer,
    ///F9 - Doctor
    Doctor,
    ///G2 - Educator/Teacher/Instructor
    EducatorTeacherInstructor,
    ///G3 - Betrothed
    Betrothed,
    ///G4 - Insured
    Insured,
    ///G5 - Lawyer
    Lawyer,
    ///G6 - Medical Care Provider
    MedicalCareProvider,
    ///G7 - Neighbor
    Neighbor,
    ///G8 - Other Relationship
    OtherRelationship,
    ///G9 - Other Relative
    OtherRelative,
    ///H1 - Owner
    Owner,
    ///H4 - Payer
    Payer,
    ///N1 - None
    None_,
    ///OT - Non-applicable Individual Relationship Category
    NonApplicableIndividualRelationshipCategory,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl IndividualRelationshipCode {
    pub fn code(&self) -> &str {
        {
            use IndividualRelationshipCode::*;
            match self {
                Guarantor => "00",
                Spouse => "01",
                SonOrDaughter => "02",
                FatherOrMother => "03",
                GrandfatherOrGrandmother => "04",
                GrandsonOrGranddaughter => "05",
                UncleOrAunt => "06",
                NephewOrNiece => "07",
                Cousin => "08",
                AdoptedChild => "09",
                FosterChild => "10",
                SonInLawOrDaughterInLaw => "11",
                BrotherInLawOrSisterInLaw => "12",
                MotherInLawOrFatherInLaw => "13",
                BrotherOrSister => "14",
                Ward => "15",
                Stepparent => "16",
                StepsonOrStepdaughter => "17",
                Self_ => "18",
                Child => "19",
                Employee => "20",
                Unknown => "21",
                HandicappedDependent => "22",
                SponsoredDependent => "23",
                DependentOfAMinorDependent => "24",
                ExSpouse => "25",
                Guardian => "26",
                Student => "27",
                Friend => "28",
                SignificantOther => "29",
                BothParents => "30",
                CourtAppointedGuardian => "31",
                Mother => "32",
                Father => "33",
                OtherAdult => "34",
                EmancipatedMinor => "36",
                AgencyRepresentative => "37",
                CollateralDependent => "38",
                OrganDonor => "39",
                CadaverDonor => "40",
                InjuredPlaintiff => "41",
                ChildWhereInsuredHasNoFinancialResponsibility => "43",
                Widow => "45",
                Widower => "46",
                StateFund => "47",
                Stepfather => "48",
                Stepmother => "49",
                FosterParent => "50",
                EmergencyContact => "51",
                Employer => "52",
                LifePartner => "53",
                AdoptedDaughter => "55",
                AdoptedSon => "56",
                AdoptiveFather => "57",
                AdoptiveMother => "58",
                AdoptiveParents => "59",
                Annuitant => "60",
                Aunt => "61",
                Brother => "62",
                BrotherInLaw => "63",
                Business => "64",
                BusinessAssociate => "65",
                BusinessInsuranceTrust => "66",
                BusinessPartner => "67",
                Charity => "68",
                ChildrenOfMarriage => "70",
                Company => "71",
                Corporation => "72",
                Creditor => "73",
                Daughter => "74",
                DaughterInLaw => "75",
                Dependent => "76",
                Estate => "78",
                ExWife => "79",
                FamilyMember => "80",
                FatherInLaw => "81",
                Code82 => "82",
                Code83 => "83",
                Fiduciary => "84",
                FosterDaughter => "86",
                FosterFather => "87",
                FosterMother => "88",
                FosterSon => "90",
                GodDaughter => "91",
                GodFather => "92",
                GodParents => "93",
                GodSon => "94",
                Grandchildren => "95",
                Granddaughter => "96",
                Grandfather => "97",
                Grandmother => "98",
                Grandparents => "99",
                Grandson => "A1",
                GreatAunt => "A2",
                ExHusband => "A3",
                HalfBrother => "A4",
                HalfSister => "A5",
                Husband => "A6",
                Institution => "A7",
                MortgageHolder => "A8",
                MotherInLaw => "A9",
                Nephew => "B1",
                Niece => "B2",
                ParentsInLaw => "B3",
                Partnership => "B4",
                Partner => "B5",
                PersonalInsuranceTrust => "B6",
                Sister => "B7",
                SisterInLaw => "B8",
                SoleProprietorship => "B9",
                Son => "C1",
                SonInLaw => "C2",
                StepBrother => "C3",
                StepChildren => "C4",
                StepDaughter => "C5",
                StepSister => "C8",
                StepSon => "C9",
                Trust => "D1",
                Trustee => "D2",
                Uncle => "D3",
                Wife => "D4",
                Teacher => "D5",
                SchoolCounselor => "D6",
                SchoolPrincipal => "D7",
                OtherSchoolAdministrator => "D8",
                Coach => "D9",
                ChildOfADomesticPartner => "DC",
                ActivitySponsor => "E1",
                Supervisor => "E2",
                CoWorker => "E3",
                MinisterOrPriest => "E4",
                EcclesiasticalOrReligiousLeader => "E5",
                GodMother => "E6",
                ProbationOfficer => "E7",
                Accountant => "E8",
                Advisor => "E9",
                AlmaMater => "F1",
                Applicant => "F2",
                Banker => "F3",
                Clergyman => "F6",
                Client => "F7",
                ClubOrOrganizationOfficer => "F8",
                Doctor => "F9",
                EducatorTeacherInstructor => "G2",
                Betrothed => "G3",
                Insured => "G4",
                Lawyer => "G5",
                MedicalCareProvider => "G6",
                Neighbor => "G7",
                OtherRelationship => "G8",
                OtherRelative => "G9",
                Owner => "H1",
                Payer => "H4",
                None_ => "N1",
                NonApplicableIndividualRelationshipCategory => "OT",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<IndividualRelationshipCode> {
        use IndividualRelationshipCode::*;
        match code {
            b"00" => Some(Guarantor),
            b"01" => Some(Spouse),
            b"02" => Some(SonOrDaughter),
            b"03" => Some(FatherOrMother),
            b"04" => Some(GrandfatherOrGrandmother),
            b"05" => Some(GrandsonOrGranddaughter),
            b"06" => Some(UncleOrAunt),
            b"07" => Some(NephewOrNiece),
            b"08" => Some(Cousin),
            b"09" => Some(AdoptedChild),
            b"10" => Some(FosterChild),
            b"11" => Some(SonInLawOrDaughterInLaw),
            b"12" => Some(BrotherInLawOrSisterInLaw),
            b"13" => Some(MotherInLawOrFatherInLaw),
            b"14" => Some(BrotherOrSister),
            b"15" => Some(Ward),
            b"16" => Some(Stepparent),
            b"17" => Some(StepsonOrStepdaughter),
            b"18" => Some(Self_),
            b"19" => Some(Child),
            b"20" => Some(Employee),
            b"21" => Some(Unknown),
            b"22" => Some(HandicappedDependent),
            b"23" => Some(SponsoredDependent),
            b"24" => Some(DependentOfAMinorDependent),
            b"25" => Some(ExSpouse),
            b"26" => Some(Guardian),
            b"27" => Some(Student),
            b"28" => Some(Friend),
            b"29" => Some(SignificantOther),
            b"30" => Some(BothParents),
            b"31" => Some(CourtAppointedGuardian),
            b"32" => Some(Mother),
            b"33" => Some(Father),
            b"34" => Some(OtherAdult),
            b"36" => Some(EmancipatedMinor),
            b"37" => Some(AgencyRepresentative),
            b"38" => Some(CollateralDependent),
            b"39" => Some(OrganDonor),
            b"40" => Some(CadaverDonor),
            b"41" => Some(InjuredPlaintiff),
            b"43" => Some(ChildWhereInsuredHasNoFinancialResponsibility),
            b"45" => Some(Widow),
            b"46" => Some(Widower),
            b"47" => Some(StateFund),
            b"48" => Some(Stepfather),
            b"49" => Some(Stepmother),
            b"50" => Some(FosterParent),
            b"51" => Some(EmergencyContact),
            b"52" => Some(Employer),
            b"53" => Some(LifePartner),
            b"55" => Some(AdoptedDaughter),
            b"56" => Some(AdoptedSon),
            b"57" => Some(AdoptiveFather),
            b"58" => Some(AdoptiveMother),
            b"59" => Some(AdoptiveParents),
            b"60" => Some(Annuitant),
            b"61" => Some(Aunt),
            b"62" => Some(Brother),
            b"63" => Some(BrotherInLaw),
            b"64" => Some(Business),
            b"65" => Some(BusinessAssociate),
            b"66" => Some(BusinessInsuranceTrust),
            b"67" => Some(BusinessPartner),
            b"68" => Some(Charity),
            b"70" => Some(ChildrenOfMarriage),
            b"71" => Some(Company),
            b"72" => Some(Corporation),
            b"73" => Some(Creditor),
            b"74" => Some(Daughter),
            b"75" => Some(DaughterInLaw),
            b"76" => Some(Dependent),
            b"78" => Some(Estate),
            b"79" => Some(ExWife),
            b"80" => Some(FamilyMember),
            b"81" => Some(FatherInLaw),
            b"82" => Some(Code82),
            b"83" => Some(Code83),
            b"84" => Some(Fiduciary),
            b"86" => Some(FosterDaughter),
            b"87" => Some(FosterFather),
            b"88" => Some(FosterMother),
            b"90" => Some(FosterSon),
            b"91" => Some(GodDaughter),
            b"92" => Some(GodFather),
            b"93" => Some(GodParents),
            b"94" => Some(GodSon),
            b"95" => Some(Grandchildren),
            b"96" => Some(Granddaughter),
            b"97" => Some(Grandfather),
            b"98" => Some(Grandmother),
            b"99" => Some(Grandparents),
            b"A1" => Some(Grandson),
            b"A2" => Some(GreatAunt),
            b"A3" => Some(ExHusband),
            b"A4" => Some(HalfBrother),
            b"A5" => Some(HalfSister),
            b"A6" => Some(Husband),
            b"A7" => Some(Institution),
            b"A8" => Some(MortgageHolder),
            b"A9" => Some(MotherInLaw),
            b"B1" => Some(Nephew),
            b"B2" => Some(Niece),
            b"B3" => Some(ParentsInLaw),
            b"B4" => Some(Partnership),
            b"B5" => Some(Partner),
            b"B6" => Some(PersonalInsuranceTrust),
            b"B7" => Some(Sister),
            b"B8" => Some(SisterInLaw),
            b"B9" => Some(SoleProprietorship),
            b"C1" => Some(Son),
            b"C2" => Some(SonInLaw),
            b"C3" => Some(StepBrother),
            b"C4" => Some(StepChildren),
            b"C5" => Some(StepDaughter),
            b"C8" => Some(StepSister),
            b"C9" => Some(StepSon),
            b"D1" => Some(Trust),
            b"D2" => Some(Trustee),
            b"D3" => Some(Uncle),
            b"D4" => Some(Wife),
            b"D5" => Some(Teacher),
            b"D6" => Some(SchoolCounselor),
            b"D7" => Some(SchoolPrincipal),
            b"D8" => Some(OtherSchoolAdministrator),
            b"D9" => Some(Coach),
            b"DC" => Some(ChildOfADomesticPartner),
            b"E1" => Some(ActivitySponsor),
            b"E2" => Some(Supervisor),
            b"E3" => Some(CoWorker),
            b"E4" => Some(MinisterOrPriest),
            b"E5" => Some(EcclesiasticalOrReligiousLeader),
            b"E6" => Some(GodMother),
            b"E7" => Some(ProbationOfficer),
            b"E8" => Some(Accountant),
            b"E9" => Some(Advisor),
            b"F1" => Some(AlmaMater),
            b"F2" => Some(Applicant),
            b"F3" => Some(Banker),
            b"F6" => Some(Clergyman),
            b"F7" => Some(Client),
            b"F8" => Some(ClubOrOrganizationOfficer),
            b"F9" => Some(Doctor),
            b"G2" => Some(EducatorTeacherInstructor),
            b"G3" => Some(Betrothed),
            b"G4" => Some(Insured),
            b"G5" => Some(Lawyer),
            b"G6" => Some(MedicalCareProvider),
            b"G7" => Some(Neighbor),
            b"G8" => Some(OtherRelationship),
            b"G9" => Some(OtherRelative),
            b"H1" => Some(Owner),
            b"H4" => Some(Payer),
            b"N1" => Some(None_),
            b"OT" => Some(NonApplicableIndividualRelationshipCategory),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use IndividualRelationshipCode::*;
        match self {
            Guarantor => "Guarantor",
            Spouse => "Spouse",
            SonOrDaughter => "Son or Daughter",
            FatherOrMother => "Father or Mother",
            GrandfatherOrGrandmother => "Grandfather or Grandmother",
            GrandsonOrGranddaughter => "Grandson or Granddaughter",
            UncleOrAunt => "Uncle or Aunt",
            NephewOrNiece => "Nephew or Niece",
            Cousin => "Cousin",
            AdoptedChild => "Adopted Child",
            FosterChild => "Foster Child",
            SonInLawOrDaughterInLaw => "Son-in-law or Daughter-in-law",
            BrotherInLawOrSisterInLaw => "Brother-in-law or Sister-in-law",
            MotherInLawOrFatherInLaw => "Mother-in-law or Father-in-law",
            BrotherOrSister => "Brother or Sister",
            Ward => "Ward",
            Stepparent => "Stepparent",
            StepsonOrStepdaughter => "Stepson or Stepdaughter",
            Self_ => "Self",
            Child => "Child",
            Employee => "Employee",
            Unknown => "Unknown",
            HandicappedDependent => "Handicapped Dependent",
            SponsoredDependent => "Sponsored Dependent",
            DependentOfAMinorDependent => "Dependent of a Minor Dependent",
            ExSpouse => "Ex-spouse",
            Guardian => "Guardian",
            Student => "Student",
            Friend => "Friend",
            SignificantOther => "Significant Other",
            BothParents => "Both Parents",
            CourtAppointedGuardian => "Court Appointed Guardian",
            Mother => "Mother",
            Father => "Father",
            OtherAdult => "Other Adult",
            EmancipatedMinor => "Emancipated Minor",
            AgencyRepresentative => "Agency Representative",
            CollateralDependent => "Collateral Dependent",
            OrganDonor => "Organ Donor",
            CadaverDonor => "Cadaver Donor",
            InjuredPlaintiff => "Injured Plaintiff",
            ChildWhereInsuredHasNoFinancialResponsibility => {
                "Child Where Insured Has No Financial Responsibility"
            }
            Widow => "Widow",
            Widower => "Widower",
            StateFund => "State Fund",
            Stepfather => "Stepfather",
            Stepmother => "Stepmother",
            FosterParent => "Foster Parent",
            EmergencyContact => "Emergency Contact",
            Employer => "Employer",
            LifePartner => "Life Partner",
            AdoptedDaughter => "Adopted Daughter",
            AdoptedSon => "Adopted Son",
            AdoptiveFather => "Adoptive Father",
            AdoptiveMother => "Adoptive Mother",
            AdoptiveParents => "Adoptive Parents",
            Annuitant => "Annuitant",
            Aunt => "Aunt",
            Brother => "Brother",
            BrotherInLaw => "Brother-in-Law",
            Business => "Business",
            BusinessAssociate => "Business Associate",
            BusinessInsuranceTrust => "Business Insurance Trust",
            BusinessPartner => "Business Partner",
            Charity => "Charity",
            ChildrenOfMarriage => "Children of Marriage",
            Company => "Company",
            Corporation => "Corporation",
            Creditor => "Creditor",
            Daughter => "Daughter",
            DaughterInLaw => "Daughter-in-Law",
            Dependent => "Dependent",
            Estate => "Estate",
            ExWife => "Ex-wife",
            FamilyMember => "Family Member",
            FatherInLaw => "Father-in-Law",
            Code82 => "Fiancé (Male)",
            Code83 => "Fiancée (Female)",
            Fiduciary => "Fiduciary",
            FosterDaughter => "Foster Daughter",
            FosterFather => "Foster Father",
            FosterMother => "Foster Mother",
            FosterSon => "Foster Son",
            GodDaughter => "God Daughter",
            GodFather => "God Father",
            GodParents => "God Parents",
            GodSon => "God Son",
            Grandchildren => "Grandchildren",
            Granddaughter => "Granddaughter",
            Grandfather => "Grandfather",
            Grandmother => "Grandmother",
            Grandparents => "Grandparents",
            Grandson => "Grandson",
            GreatAunt => "Great Aunt",
            ExHusband => "Ex-husband",
            HalfBrother => "Half Brother",
            HalfSister => "Half Sister",
            Husband => "Husband",
            Institution => "Institution",
            MortgageHolder => "Mortgage Holder",
            MotherInLaw => "Mother-in-Law",
            Nephew => "Nephew",
            Niece => "Niece",
            ParentsInLaw => "Parents-in-Law",
            Partnership => "Partnership",
            Partner => "Partner",
            PersonalInsuranceTrust => "Personal Insurance Trust",
            Sister => "Sister",
            SisterInLaw => "Sister-in-Law",
            SoleProprietorship => "Sole Proprietorship",
            Son => "Son",
            SonInLaw => "Son-in-Law",
            StepBrother => "Step Brother",
            StepChildren => "Step Children",
            StepDaughter => "Step Daughter",
            StepSister => "Step Sister",
            StepSon => "Step Son",
            Trust => "Trust",
            Trustee => "Trustee",
            Uncle => "Uncle",
            Wife => "Wife",
            Teacher => "Teacher",
            SchoolCounselor => "School Counselor",
            SchoolPrincipal => "School Principal",
            OtherSchoolAdministrator => "Other School Administrator",
            Coach => "Coach",
            ChildOfADomesticPartner => "Child of a Domestic Partner",
            ActivitySponsor => "Activity Sponsor",
            Supervisor => "Supervisor",
            CoWorker => "Co-worker",
            MinisterOrPriest => "Minister or Priest",
            EcclesiasticalOrReligiousLeader => "Ecclesiastical or Religious Leader",
            GodMother => "God Mother",
            ProbationOfficer => "Probation Officer",
            Accountant => "Accountant",
            Advisor => "Advisor",
            AlmaMater => "Alma Mater",
            Applicant => "Applicant",
            Banker => "Banker",
            Clergyman => "Clergyman",
            Client => "Client",
            ClubOrOrganizationOfficer => "Club or Organization Officer",
            Doctor => "Doctor",
            EducatorTeacherInstructor => "Educator/Teacher/Instructor",
            Betrothed => "Betrothed",
            Insured => "Insured",
            Lawyer => "Lawyer",
            MedicalCareProvider => "Medical Care Provider",
            Neighbor => "Neighbor",
            OtherRelationship => "Other Relationship",
            OtherRelative => "Other Relative",
            Owner => "Owner",
            Payer => "Payer",
            None_ => "None",
            NonApplicableIndividualRelationshipCategory => {
                "Non-applicable Individual Relationship Category"
            }
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<IndividualRelationshipCode> {
        {
            use IndividualRelationshipCode::*;
            match description {
                "Guarantor" => Some(Guarantor),
                "Spouse" => Some(Spouse),
                "Son or Daughter" => Some(SonOrDaughter),
                "Father or Mother" => Some(FatherOrMother),
                "Grandfather or Grandmother" => Some(GrandfatherOrGrandmother),
                "Grandson or Granddaughter" => Some(GrandsonOrGranddaughter),
                "Uncle or Aunt" => Some(UncleOrAunt),
                "Nephew or Niece" => Some(NephewOrNiece),
                "Cousin" => Some(Cousin),
                "Adopted Child" => Some(AdoptedChild),
                "Foster Child" => Some(FosterChild),
                "Son-in-law or Daughter-in-law" => Some(SonInLawOrDaughterInLaw),
                "Brother-in-law or Sister-in-law" => Some(BrotherInLawOrSisterInLaw),
                "Mother-in-law or Father-in-law" => Some(MotherInLawOrFatherInLaw),
                "Brother or Sister" => Some(BrotherOrSister),
                "Ward" => Some(Ward),
                "Stepparent" => Some(Stepparent),
                "Stepson or Stepdaughter" => Some(StepsonOrStepdaughter),
                "Self" => Some(Self_),
                "Child" => Some(Child),
                "Employee" => Some(Employee),
                "Unknown" => Some(Unknown),
                "Handicapped Dependent" => Some(HandicappedDependent),
                "Sponsored Dependent" => Some(SponsoredDependent),
                "Dependent of a Minor Dependent" => Some(DependentOfAMinorDependent),
                "Ex-spouse" => Some(ExSpouse),
                "Guardian" => Some(Guardian),
                "Student" => Some(Student),
                "Friend" => Some(Friend),
                "Significant Other" => Some(SignificantOther),
                "Both Parents" => Some(BothParents),
                "Court Appointed Guardian" => Some(CourtAppointedGuardian),
                "Mother" => Some(Mother),
                "Father" => Some(Father),
                "Other Adult" => Some(OtherAdult),
                "Emancipated Minor" => Some(EmancipatedMinor),
                "Agency Representative" => Some(AgencyRepresentative),
                "Collateral Dependent" => Some(CollateralDependent),
                "Organ Donor" => Some(OrganDonor),
                "Cadaver Donor" => Some(CadaverDonor),
                "Injured Plaintiff" => Some(InjuredPlaintiff),
                "Child Where Insured Has No Financial Responsibility" => {
                    Some(ChildWhereInsuredHasNoFinancialResponsibility)
                }
                "Widow" => Some(Widow),
                "Widower" => Some(Widower),
                "State Fund" => Some(StateFund),
                "Stepfather" => Some(Stepfather),
                "Stepmother" => Some(Stepmother),
                "Foster Parent" => Some(FosterParent),
                "Emergency Contact" => Some(EmergencyContact),
                "Employer" => Some(Employer),
                "Life Partner" => Some(LifePartner),
                "Adopted Daughter" => Some(AdoptedDaughter),
                "Adopted Son" => Some(AdoptedSon),
                "Adoptive Father" => Some(AdoptiveFather),
                "Adoptive Mother" => Some(AdoptiveMother),
                "Adoptive Parents" => Some(AdoptiveParents),
                "Annuitant" => Some(Annuitant),
                "Aunt" => Some(Aunt),
                "Brother" => Some(Brother),
                "Brother-in-Law" => Some(BrotherInLaw),
                "Business" => Some(Business),
                "Business Associate" => Some(BusinessAssociate),
                "Business Insurance Trust" => Some(BusinessInsuranceTrust),
                "Business Partner" => Some(BusinessPartner),
                "Charity" => Some(Charity),
                "Children of Marriage" => Some(ChildrenOfMarriage),
                "Company" => Some(Company),
                "Corporation" => Some(Corporation),
                "Creditor" => Some(Creditor),
                "Daughter" => Some(Daughter),
                "Daughter-in-Law" => Some(DaughterInLaw),
                "Dependent" => Some(Dependent),
                "Estate" => Some(Estate),
                "Ex-wife" => Some(ExWife),
                "Family Member" => Some(FamilyMember),
                "Father-in-Law" => Some(FatherInLaw),
                "Fiancé (Male)" => Some(Code82),
                "Fiancée (Female)" => Some(Code83),
                "Fiduciary" => Some(Fiduciary),
                "Foster Daughter" => Some(FosterDaughter),
                "Foster Father" => Some(FosterFather),
                "Foster Mother" => Some(FosterMother),
                "Foster Son" => Some(FosterSon),
                "God Daughter" => Some(GodDaughter),
                "God Father" => Some(GodFather),
                "God Parents" => Some(GodParents),
                "God Son" => Some(GodSon),
                "Grandchildren" => Some(Grandchildren),
                "Granddaughter" => Some(Granddaughter),
                "Grandfather" => Some(Grandfather),
                "Grandmother" => Some(Grandmother),
                "Grandparents" => Some(Grandparents),
                "Grandson" => Some(Grandson),
                "Great Aunt" => Some(GreatAunt),
                "Ex-husband" => Some(ExHusband),
                "Half Brother" => Some(HalfBrother),
                "Half Sister" => Some(HalfSister),
                "Husband" => Some(Husband),
                "Institution" => Some(Institution),
                "Mortgage Holder" => Some(MortgageHolder),
                "Mother-in-Law" => Some(MotherInLaw),
                "Nephew" => Some(Nephew),
                "Niece" => Some(Niece),
                "Parents-in-Law" => Some(ParentsInLaw),
                "Partnership" => Some(Partnership),
                "Partner" => Some(Partner),
                "Personal Insurance Trust" => Some(PersonalInsuranceTrust),
                "Sister" => Some(Sister),
                "Sister-in-Law" => Some(SisterInLaw),
                "Sole Proprietorship" => Some(SoleProprietorship),
                "Son" => Some(Son),
                "Son-in-Law" => Some(SonInLaw),
                "Step Brother" => Some(StepBrother),
                "Step Children" => Some(StepChildren),
                "Step Daughter" => Some(StepDaughter),
                "Step Sister" => Some(StepSister),
                "Step Son" => Some(StepSon),
                "Trust" => Some(Trust),
                "Trustee" => Some(Trustee),
                "Uncle" => Some(Uncle),
                "Wife" => Some(Wife),
                "Teacher" => Some(Teacher),
                "School Counselor" => Some(SchoolCounselor),
                "School Principal" => Some(SchoolPrincipal),
                "Other School Administrator" => Some(OtherSchoolAdministrator),
                "Coach" => Some(Coach),
                "Child of a Domestic Partner" => Some(ChildOfADomesticPartner),
                "Activity Sponsor" => Some(ActivitySponsor),
                "Supervisor" => Some(Supervisor),
                "Co-worker" => Some(CoWorker),
                "Minister or Priest" => Some(MinisterOrPriest),
                "Ecclesiastical or Religious Leader" => {
                    Some(EcclesiasticalOrReligiousLeader)
                }
                "God Mother" => Some(GodMother),
                "Probation Officer" => Some(ProbationOfficer),
                "Accountant" => Some(Accountant),
                "Advisor" => Some(Advisor),
                "Alma Mater" => Some(AlmaMater),
                "Applicant" => Some(Applicant),
                "Banker" => Some(Banker),
                "Clergyman" => Some(Clergyman),
                "Client" => Some(Client),
                "Club or Organization Officer" => Some(ClubOrOrganizationOfficer),
                "Doctor" => Some(Doctor),
                "Educator/Teacher/Instructor" => Some(EducatorTeacherInstructor),
                "Betrothed" => Some(Betrothed),
                "Insured" => Some(Insured),
                "Lawyer" => Some(Lawyer),
                "Medical Care Provider" => Some(MedicalCareProvider),
                "Neighbor" => Some(Neighbor),
                "Other Relationship" => Some(OtherRelationship),
                "Other Relative" => Some(OtherRelative),
                "Owner" => Some(Owner),
                "Payer" => Some(Payer),
                "None" => Some(None_),
                "Non-applicable Individual Relationship Category" => {
                    Some(NonApplicableIndividualRelationshipCategory)
                }
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for IndividualRelationshipCode {
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
    type Value = IndividualRelationshipCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Individual Relationship Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        IndividualRelationshipCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Individual Relationship Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        IndividualRelationshipCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Individual Relationship Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for IndividualRelationshipCode {
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