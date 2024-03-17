use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**706

See docs at <https://www.stedi.com/edi/x12/element/706>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityRelationshipCode {
    ///01 - Parent
    Parent,
    ///02 - Child
    Child,
    ///03 - Corporation
    Corporation,
    ///04 - Subsidiary
    Subsidiary,
    ///05 - Wholly-Owned Subsidiary
    WhollyOwnedSubsidiary,
    ///06 - Division
    Division,
    ///07 - Company
    Company,
    ///08 - Doing Business As
    DoingBusinessAs,
    ///09 - Component
    Component,
    ///10 - Partnership
    Partnership,
    ///11 - Partner
    Partner,
    ///12 - Member
    Member,
    ///13 - Association
    Association,
    ///14 - Headquarters
    Headquarters,
    ///15 - Profit Center
    ProfitCenter,
    ///16 - Cost Center
    CostCenter,
    ///17 - Product Line
    ProductLine,
    ///18 - Union
    Union,
    ///19 - Group
    Group,
    ///20 - Department
    Department,
    ///21 - Multinational Corporation
    MultinationalCorporation,
    ///22 - Sibling
    Sibling,
    ///23 - Affiliate
    Affiliate,
    ///24 - Direct Affiliate
    DirectAffiliate,
    ///25 - Established Patient
    EstablishedPatient,
    ///26 - Not Established Patient
    NotEstablishedPatient,
    ///27 - Domestic Partner
    DomesticPartner,
    ///29 - Power of Attorney Delegee
    PowerOfAttorneyDelegee,
    ///30 - Significant Other
    SignificantOther,
    ///31 - Ultimate Parent Company
    UltimateParentCompany,
    ///32 - Branch
    Branch,
    ///33 - Owned
    Owned,
    ///34 - Managed
    Managed,
    ///35 - Leased
    Leased,
    ///36 - Group Affiliate
    GroupAffiliate,
    ///37 - Owner Affiliate
    OwnerAffiliate,
    ///38 - Owner
    Owner,
    ///39 - Related for U.S. Customs Purposes
    RelatedForUSCustomsPurposes,
    ///40 - Related for U.S. Bureau of the Census Purposes
    RelatedForUSBureauOfTheCensusPurposes,
    ///41 - Spouse
    Spouse,
    ///42 - Adoptive Parent
    AdoptiveParent,
    ///43 - Bank
    Bank,
    ///44 - Brother
    Brother,
    ///45 - Business Associate
    BusinessAssociate,
    ///46 - Daughter
    Daughter,
    ///47 - Dependent
    Dependent,
    ///48 - Employee
    Employee,
    ///49 - Employer
    Employer,
    ///50 - Father
    Father,
    ///51 - Fiancée
    Fiancee,
    ///52 - Foreman
    Foreman,
    ///53 - Foster Parent
    FosterParent,
    ///54 - Friend
    Friend,
    ///55 - Grand Child
    GrandChild,
    ///56 - Grand Parent
    GrandParent,
    ///57 - Guardian
    Guardian,
    ///58 - Inforce Policyholder
    InforcePolicyholder,
    ///59 - Institution
    Institution,
    ///60 - Minister
    Minister,
    ///61 - Mother
    Mother,
    ///62 - Neighbor
    Neighbor,
    ///63 - Non Family
    NonFamily,
    ///64 - None
    None_,
    ///65 - Other
    Other,
    ///66 - Other Family
    OtherFamily,
    ///67 - Self
    Self_,
    ///68 - Sister
    Sister,
    ///69 - Step Child
    StepChild,
    ///70 - Supervisor
    Supervisor,
    ///71 - Teacher
    Teacher,
    ///72 - Unknown
    Unknown,
    ///73 - Business Name
    BusinessName,
    ///74 - Counselor
    Counselor,
    ///75 - Sanctioning Organization
    SanctioningOrganization,
    ///76 - Sponsoring Organization
    SponsoringOrganization,
    ///77 - Same Job as Applicant
    SameJobAsApplicant,
    ///78 - Stockholder
    Stockholder,
    ///79 - Attorney
    Attorney,
    ///80 - Aunt
    Aunt,
    ///81 - Brother-in-law
    BrotherInLaw,
    ///82 - Cousin
    Cousin,
    ///83 - Daughter-in-law
    DaughterInLaw,
    ///84 - Family
    Family,
    ///85 - Father-in-law
    FatherInLaw,
    ///86 - Financial Interest
    FinancialInterest,
    ///87 - Marketing Unit
    MarketingUnit,
    ///88 - Mother-in-law
    MotherInLaw,
    ///89 - Nephew
    Nephew,
    ///90 - Niece
    Niece,
    ///91 - Officer
    Officer,
    ///92 - Principal Customer
    PrincipalCustomer,
    ///93 - Principal Supplier
    PrincipalSupplier,
    ///94 - Sister-in-law
    SisterInLaw,
    ///95 - Son
    Son,
    ///96 - Son-in-law
    SonInLaw,
    ///97 - Uncle
    Uncle,
    ///98 - Descendant
    Descendant,
    ///99 - Director
    Director,
    ///AA - Principal Stockholder
    PrincipalStockholder,
    ///AB - Insured Entity
    InsuredEntity,
    ///AC - Allied Professional
    AlliedProfessional,
    ///AD - Ancillary Referral
    AncillaryReferral,
    ///AE - Contact
    Contact,
    ///AF - Contract
    Contract,
    ///AG - Health Care Facility Affiliation
    HealthCareFacilityAffiliation,
    ///AH - Independent Physician/Practice Association (IPA) Affiliation
    CodeAH,
    ///AI - Referral Lab Provider
    ReferralLabProvider,
    ///AJ - Managed Care Organization Affiliation
    ManagedCareOrganizationAffiliation,
    ///AK - Medical Director
    MedicalDirector,
    ///AL - Health Care Network Affiliation
    HealthCareNetworkAffiliation,
    ///AM - Office Manager
    OfficeManager,
    ///AN - On-call Physician
    OnCallPhysician,
    ///AO - Physician Hospital Organization (PHO) Affiliation
    CodeAO,
    ///AP - Provider in Practice
    ProviderInPractice,
    ///AQ - Referred by Provider
    ReferredByProvider,
    ///AR - Referred to Provider
    ReferredToProvider,
    ///AS - Referral X-ray Provider
    ReferralXRayProvider,
    ///AT - Parent-in-law
    ParentInLaw,
    ///AU - Step Parent
    StepParent,
    ///AV - Former Spouse
    FormerSpouse,
    ///AW - Ward
    Ward,
    ///CP - Custodial Parent
    CustodialParent,
    ///OP - Obligated Parent
    ObligatedParent,
    ///PI - Principal
    Principal,
}
impl EntityRelationshipCode {
    pub fn code(&self) -> &str {
        {
            use EntityRelationshipCode::*;
            match self {
                Parent => "01",
                Child => "02",
                Corporation => "03",
                Subsidiary => "04",
                WhollyOwnedSubsidiary => "05",
                Division => "06",
                Company => "07",
                DoingBusinessAs => "08",
                Component => "09",
                Partnership => "10",
                Partner => "11",
                Member => "12",
                Association => "13",
                Headquarters => "14",
                ProfitCenter => "15",
                CostCenter => "16",
                ProductLine => "17",
                Union => "18",
                Group => "19",
                Department => "20",
                MultinationalCorporation => "21",
                Sibling => "22",
                Affiliate => "23",
                DirectAffiliate => "24",
                EstablishedPatient => "25",
                NotEstablishedPatient => "26",
                DomesticPartner => "27",
                PowerOfAttorneyDelegee => "29",
                SignificantOther => "30",
                UltimateParentCompany => "31",
                Branch => "32",
                Owned => "33",
                Managed => "34",
                Leased => "35",
                GroupAffiliate => "36",
                OwnerAffiliate => "37",
                Owner => "38",
                RelatedForUSCustomsPurposes => "39",
                RelatedForUSBureauOfTheCensusPurposes => "40",
                Spouse => "41",
                AdoptiveParent => "42",
                Bank => "43",
                Brother => "44",
                BusinessAssociate => "45",
                Daughter => "46",
                Dependent => "47",
                Employee => "48",
                Employer => "49",
                Father => "50",
                Fiancee => "51",
                Foreman => "52",
                FosterParent => "53",
                Friend => "54",
                GrandChild => "55",
                GrandParent => "56",
                Guardian => "57",
                InforcePolicyholder => "58",
                Institution => "59",
                Minister => "60",
                Mother => "61",
                Neighbor => "62",
                NonFamily => "63",
                None_ => "64",
                Other => "65",
                OtherFamily => "66",
                Self_ => "67",
                Sister => "68",
                StepChild => "69",
                Supervisor => "70",
                Teacher => "71",
                Unknown => "72",
                BusinessName => "73",
                Counselor => "74",
                SanctioningOrganization => "75",
                SponsoringOrganization => "76",
                SameJobAsApplicant => "77",
                Stockholder => "78",
                Attorney => "79",
                Aunt => "80",
                BrotherInLaw => "81",
                Cousin => "82",
                DaughterInLaw => "83",
                Family => "84",
                FatherInLaw => "85",
                FinancialInterest => "86",
                MarketingUnit => "87",
                MotherInLaw => "88",
                Nephew => "89",
                Niece => "90",
                Officer => "91",
                PrincipalCustomer => "92",
                PrincipalSupplier => "93",
                SisterInLaw => "94",
                Son => "95",
                SonInLaw => "96",
                Uncle => "97",
                Descendant => "98",
                Director => "99",
                PrincipalStockholder => "AA",
                InsuredEntity => "AB",
                AlliedProfessional => "AC",
                AncillaryReferral => "AD",
                Contact => "AE",
                Contract => "AF",
                HealthCareFacilityAffiliation => "AG",
                CodeAH => "AH",
                ReferralLabProvider => "AI",
                ManagedCareOrganizationAffiliation => "AJ",
                MedicalDirector => "AK",
                HealthCareNetworkAffiliation => "AL",
                OfficeManager => "AM",
                OnCallPhysician => "AN",
                CodeAO => "AO",
                ProviderInPractice => "AP",
                ReferredByProvider => "AQ",
                ReferredToProvider => "AR",
                ReferralXRayProvider => "AS",
                ParentInLaw => "AT",
                StepParent => "AU",
                FormerSpouse => "AV",
                Ward => "AW",
                CustodialParent => "CP",
                ObligatedParent => "OP",
                Principal => "PI",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<EntityRelationshipCode> {
        use EntityRelationshipCode::*;
        match code {
            b"01" => Some(Parent),
            b"02" => Some(Child),
            b"03" => Some(Corporation),
            b"04" => Some(Subsidiary),
            b"05" => Some(WhollyOwnedSubsidiary),
            b"06" => Some(Division),
            b"07" => Some(Company),
            b"08" => Some(DoingBusinessAs),
            b"09" => Some(Component),
            b"10" => Some(Partnership),
            b"11" => Some(Partner),
            b"12" => Some(Member),
            b"13" => Some(Association),
            b"14" => Some(Headquarters),
            b"15" => Some(ProfitCenter),
            b"16" => Some(CostCenter),
            b"17" => Some(ProductLine),
            b"18" => Some(Union),
            b"19" => Some(Group),
            b"20" => Some(Department),
            b"21" => Some(MultinationalCorporation),
            b"22" => Some(Sibling),
            b"23" => Some(Affiliate),
            b"24" => Some(DirectAffiliate),
            b"25" => Some(EstablishedPatient),
            b"26" => Some(NotEstablishedPatient),
            b"27" => Some(DomesticPartner),
            b"29" => Some(PowerOfAttorneyDelegee),
            b"30" => Some(SignificantOther),
            b"31" => Some(UltimateParentCompany),
            b"32" => Some(Branch),
            b"33" => Some(Owned),
            b"34" => Some(Managed),
            b"35" => Some(Leased),
            b"36" => Some(GroupAffiliate),
            b"37" => Some(OwnerAffiliate),
            b"38" => Some(Owner),
            b"39" => Some(RelatedForUSCustomsPurposes),
            b"40" => Some(RelatedForUSBureauOfTheCensusPurposes),
            b"41" => Some(Spouse),
            b"42" => Some(AdoptiveParent),
            b"43" => Some(Bank),
            b"44" => Some(Brother),
            b"45" => Some(BusinessAssociate),
            b"46" => Some(Daughter),
            b"47" => Some(Dependent),
            b"48" => Some(Employee),
            b"49" => Some(Employer),
            b"50" => Some(Father),
            b"51" => Some(Fiancee),
            b"52" => Some(Foreman),
            b"53" => Some(FosterParent),
            b"54" => Some(Friend),
            b"55" => Some(GrandChild),
            b"56" => Some(GrandParent),
            b"57" => Some(Guardian),
            b"58" => Some(InforcePolicyholder),
            b"59" => Some(Institution),
            b"60" => Some(Minister),
            b"61" => Some(Mother),
            b"62" => Some(Neighbor),
            b"63" => Some(NonFamily),
            b"64" => Some(None_),
            b"65" => Some(Other),
            b"66" => Some(OtherFamily),
            b"67" => Some(Self_),
            b"68" => Some(Sister),
            b"69" => Some(StepChild),
            b"70" => Some(Supervisor),
            b"71" => Some(Teacher),
            b"72" => Some(Unknown),
            b"73" => Some(BusinessName),
            b"74" => Some(Counselor),
            b"75" => Some(SanctioningOrganization),
            b"76" => Some(SponsoringOrganization),
            b"77" => Some(SameJobAsApplicant),
            b"78" => Some(Stockholder),
            b"79" => Some(Attorney),
            b"80" => Some(Aunt),
            b"81" => Some(BrotherInLaw),
            b"82" => Some(Cousin),
            b"83" => Some(DaughterInLaw),
            b"84" => Some(Family),
            b"85" => Some(FatherInLaw),
            b"86" => Some(FinancialInterest),
            b"87" => Some(MarketingUnit),
            b"88" => Some(MotherInLaw),
            b"89" => Some(Nephew),
            b"90" => Some(Niece),
            b"91" => Some(Officer),
            b"92" => Some(PrincipalCustomer),
            b"93" => Some(PrincipalSupplier),
            b"94" => Some(SisterInLaw),
            b"95" => Some(Son),
            b"96" => Some(SonInLaw),
            b"97" => Some(Uncle),
            b"98" => Some(Descendant),
            b"99" => Some(Director),
            b"AA" => Some(PrincipalStockholder),
            b"AB" => Some(InsuredEntity),
            b"AC" => Some(AlliedProfessional),
            b"AD" => Some(AncillaryReferral),
            b"AE" => Some(Contact),
            b"AF" => Some(Contract),
            b"AG" => Some(HealthCareFacilityAffiliation),
            b"AH" => Some(CodeAH),
            b"AI" => Some(ReferralLabProvider),
            b"AJ" => Some(ManagedCareOrganizationAffiliation),
            b"AK" => Some(MedicalDirector),
            b"AL" => Some(HealthCareNetworkAffiliation),
            b"AM" => Some(OfficeManager),
            b"AN" => Some(OnCallPhysician),
            b"AO" => Some(CodeAO),
            b"AP" => Some(ProviderInPractice),
            b"AQ" => Some(ReferredByProvider),
            b"AR" => Some(ReferredToProvider),
            b"AS" => Some(ReferralXRayProvider),
            b"AT" => Some(ParentInLaw),
            b"AU" => Some(StepParent),
            b"AV" => Some(FormerSpouse),
            b"AW" => Some(Ward),
            b"CP" => Some(CustodialParent),
            b"OP" => Some(ObligatedParent),
            b"PI" => Some(Principal),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use EntityRelationshipCode::*;
        match self {
            Parent => "Parent",
            Child => "Child",
            Corporation => "Corporation",
            Subsidiary => "Subsidiary",
            WhollyOwnedSubsidiary => "Wholly-Owned Subsidiary",
            Division => "Division",
            Company => "Company",
            DoingBusinessAs => "Doing Business As",
            Component => "Component",
            Partnership => "Partnership",
            Partner => "Partner",
            Member => "Member",
            Association => "Association",
            Headquarters => "Headquarters",
            ProfitCenter => "Profit Center",
            CostCenter => "Cost Center",
            ProductLine => "Product Line",
            Union => "Union",
            Group => "Group",
            Department => "Department",
            MultinationalCorporation => "Multinational Corporation",
            Sibling => "Sibling",
            Affiliate => "Affiliate",
            DirectAffiliate => "Direct Affiliate",
            EstablishedPatient => "Established Patient",
            NotEstablishedPatient => "Not Established Patient",
            DomesticPartner => "Domestic Partner",
            PowerOfAttorneyDelegee => "Power of Attorney Delegee",
            SignificantOther => "Significant Other",
            UltimateParentCompany => "Ultimate Parent Company",
            Branch => "Branch",
            Owned => "Owned",
            Managed => "Managed",
            Leased => "Leased",
            GroupAffiliate => "Group Affiliate",
            OwnerAffiliate => "Owner Affiliate",
            Owner => "Owner",
            RelatedForUSCustomsPurposes => "Related for U.S. Customs Purposes",
            RelatedForUSBureauOfTheCensusPurposes => {
                "Related for U.S. Bureau of the Census Purposes"
            }
            Spouse => "Spouse",
            AdoptiveParent => "Adoptive Parent",
            Bank => "Bank",
            Brother => "Brother",
            BusinessAssociate => "Business Associate",
            Daughter => "Daughter",
            Dependent => "Dependent",
            Employee => "Employee",
            Employer => "Employer",
            Father => "Father",
            Fiancee => "Fiancée",
            Foreman => "Foreman",
            FosterParent => "Foster Parent",
            Friend => "Friend",
            GrandChild => "Grand Child",
            GrandParent => "Grand Parent",
            Guardian => "Guardian",
            InforcePolicyholder => "Inforce Policyholder",
            Institution => "Institution",
            Minister => "Minister",
            Mother => "Mother",
            Neighbor => "Neighbor",
            NonFamily => "Non Family",
            None_ => "None",
            Other => "Other",
            OtherFamily => "Other Family",
            Self_ => "Self",
            Sister => "Sister",
            StepChild => "Step Child",
            Supervisor => "Supervisor",
            Teacher => "Teacher",
            Unknown => "Unknown",
            BusinessName => "Business Name",
            Counselor => "Counselor",
            SanctioningOrganization => "Sanctioning Organization",
            SponsoringOrganization => "Sponsoring Organization",
            SameJobAsApplicant => "Same Job as Applicant",
            Stockholder => "Stockholder",
            Attorney => "Attorney",
            Aunt => "Aunt",
            BrotherInLaw => "Brother-in-law",
            Cousin => "Cousin",
            DaughterInLaw => "Daughter-in-law",
            Family => "Family",
            FatherInLaw => "Father-in-law",
            FinancialInterest => "Financial Interest",
            MarketingUnit => "Marketing Unit",
            MotherInLaw => "Mother-in-law",
            Nephew => "Nephew",
            Niece => "Niece",
            Officer => "Officer",
            PrincipalCustomer => "Principal Customer",
            PrincipalSupplier => "Principal Supplier",
            SisterInLaw => "Sister-in-law",
            Son => "Son",
            SonInLaw => "Son-in-law",
            Uncle => "Uncle",
            Descendant => "Descendant",
            Director => "Director",
            PrincipalStockholder => "Principal Stockholder",
            InsuredEntity => "Insured Entity",
            AlliedProfessional => "Allied Professional",
            AncillaryReferral => "Ancillary Referral",
            Contact => "Contact",
            Contract => "Contract",
            HealthCareFacilityAffiliation => "Health Care Facility Affiliation",
            CodeAH => "Independent Physician/Practice Association (IPA) Affiliation",
            ReferralLabProvider => "Referral Lab Provider",
            ManagedCareOrganizationAffiliation => "Managed Care Organization Affiliation",
            MedicalDirector => "Medical Director",
            HealthCareNetworkAffiliation => "Health Care Network Affiliation",
            OfficeManager => "Office Manager",
            OnCallPhysician => "On-call Physician",
            CodeAO => "Physician Hospital Organization (PHO) Affiliation",
            ProviderInPractice => "Provider in Practice",
            ReferredByProvider => "Referred by Provider",
            ReferredToProvider => "Referred to Provider",
            ReferralXRayProvider => "Referral X-ray Provider",
            ParentInLaw => "Parent-in-law",
            StepParent => "Step Parent",
            FormerSpouse => "Former Spouse",
            Ward => "Ward",
            CustodialParent => "Custodial Parent",
            ObligatedParent => "Obligated Parent",
            Principal => "Principal",
        }
    }
    fn from_description(description: &str) -> Option<EntityRelationshipCode> {
        {
            use EntityRelationshipCode::*;
            match description {
                "Parent" => Some(Parent),
                "Child" => Some(Child),
                "Corporation" => Some(Corporation),
                "Subsidiary" => Some(Subsidiary),
                "Wholly-Owned Subsidiary" => Some(WhollyOwnedSubsidiary),
                "Division" => Some(Division),
                "Company" => Some(Company),
                "Doing Business As" => Some(DoingBusinessAs),
                "Component" => Some(Component),
                "Partnership" => Some(Partnership),
                "Partner" => Some(Partner),
                "Member" => Some(Member),
                "Association" => Some(Association),
                "Headquarters" => Some(Headquarters),
                "Profit Center" => Some(ProfitCenter),
                "Cost Center" => Some(CostCenter),
                "Product Line" => Some(ProductLine),
                "Union" => Some(Union),
                "Group" => Some(Group),
                "Department" => Some(Department),
                "Multinational Corporation" => Some(MultinationalCorporation),
                "Sibling" => Some(Sibling),
                "Affiliate" => Some(Affiliate),
                "Direct Affiliate" => Some(DirectAffiliate),
                "Established Patient" => Some(EstablishedPatient),
                "Not Established Patient" => Some(NotEstablishedPatient),
                "Domestic Partner" => Some(DomesticPartner),
                "Power of Attorney Delegee" => Some(PowerOfAttorneyDelegee),
                "Significant Other" => Some(SignificantOther),
                "Ultimate Parent Company" => Some(UltimateParentCompany),
                "Branch" => Some(Branch),
                "Owned" => Some(Owned),
                "Managed" => Some(Managed),
                "Leased" => Some(Leased),
                "Group Affiliate" => Some(GroupAffiliate),
                "Owner Affiliate" => Some(OwnerAffiliate),
                "Owner" => Some(Owner),
                "Related for U.S. Customs Purposes" => Some(RelatedForUSCustomsPurposes),
                "Related for U.S. Bureau of the Census Purposes" => {
                    Some(RelatedForUSBureauOfTheCensusPurposes)
                }
                "Spouse" => Some(Spouse),
                "Adoptive Parent" => Some(AdoptiveParent),
                "Bank" => Some(Bank),
                "Brother" => Some(Brother),
                "Business Associate" => Some(BusinessAssociate),
                "Daughter" => Some(Daughter),
                "Dependent" => Some(Dependent),
                "Employee" => Some(Employee),
                "Employer" => Some(Employer),
                "Father" => Some(Father),
                "Fiancée" => Some(Fiancee),
                "Foreman" => Some(Foreman),
                "Foster Parent" => Some(FosterParent),
                "Friend" => Some(Friend),
                "Grand Child" => Some(GrandChild),
                "Grand Parent" => Some(GrandParent),
                "Guardian" => Some(Guardian),
                "Inforce Policyholder" => Some(InforcePolicyholder),
                "Institution" => Some(Institution),
                "Minister" => Some(Minister),
                "Mother" => Some(Mother),
                "Neighbor" => Some(Neighbor),
                "Non Family" => Some(NonFamily),
                "None" => Some(None_),
                "Other" => Some(Other),
                "Other Family" => Some(OtherFamily),
                "Self" => Some(Self_),
                "Sister" => Some(Sister),
                "Step Child" => Some(StepChild),
                "Supervisor" => Some(Supervisor),
                "Teacher" => Some(Teacher),
                "Unknown" => Some(Unknown),
                "Business Name" => Some(BusinessName),
                "Counselor" => Some(Counselor),
                "Sanctioning Organization" => Some(SanctioningOrganization),
                "Sponsoring Organization" => Some(SponsoringOrganization),
                "Same Job as Applicant" => Some(SameJobAsApplicant),
                "Stockholder" => Some(Stockholder),
                "Attorney" => Some(Attorney),
                "Aunt" => Some(Aunt),
                "Brother-in-law" => Some(BrotherInLaw),
                "Cousin" => Some(Cousin),
                "Daughter-in-law" => Some(DaughterInLaw),
                "Family" => Some(Family),
                "Father-in-law" => Some(FatherInLaw),
                "Financial Interest" => Some(FinancialInterest),
                "Marketing Unit" => Some(MarketingUnit),
                "Mother-in-law" => Some(MotherInLaw),
                "Nephew" => Some(Nephew),
                "Niece" => Some(Niece),
                "Officer" => Some(Officer),
                "Principal Customer" => Some(PrincipalCustomer),
                "Principal Supplier" => Some(PrincipalSupplier),
                "Sister-in-law" => Some(SisterInLaw),
                "Son" => Some(Son),
                "Son-in-law" => Some(SonInLaw),
                "Uncle" => Some(Uncle),
                "Descendant" => Some(Descendant),
                "Director" => Some(Director),
                "Principal Stockholder" => Some(PrincipalStockholder),
                "Insured Entity" => Some(InsuredEntity),
                "Allied Professional" => Some(AlliedProfessional),
                "Ancillary Referral" => Some(AncillaryReferral),
                "Contact" => Some(Contact),
                "Contract" => Some(Contract),
                "Health Care Facility Affiliation" => Some(HealthCareFacilityAffiliation),
                "Independent Physician/Practice Association (IPA) Affiliation" => {
                    Some(CodeAH)
                }
                "Referral Lab Provider" => Some(ReferralLabProvider),
                "Managed Care Organization Affiliation" => {
                    Some(ManagedCareOrganizationAffiliation)
                }
                "Medical Director" => Some(MedicalDirector),
                "Health Care Network Affiliation" => Some(HealthCareNetworkAffiliation),
                "Office Manager" => Some(OfficeManager),
                "On-call Physician" => Some(OnCallPhysician),
                "Physician Hospital Organization (PHO) Affiliation" => Some(CodeAO),
                "Provider in Practice" => Some(ProviderInPractice),
                "Referred by Provider" => Some(ReferredByProvider),
                "Referred to Provider" => Some(ReferredToProvider),
                "Referral X-ray Provider" => Some(ReferralXRayProvider),
                "Parent-in-law" => Some(ParentInLaw),
                "Step Parent" => Some(StepParent),
                "Former Spouse" => Some(FormerSpouse),
                "Ward" => Some(Ward),
                "Custodial Parent" => Some(CustodialParent),
                "Obligated Parent" => Some(ObligatedParent),
                "Principal" => Some(Principal),
                _ => None,
            }
        }
    }
}
impl Serialize for EntityRelationshipCode {
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
    type Value = EntityRelationshipCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Entity Relationship Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        EntityRelationshipCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Entity Relationship Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        EntityRelationshipCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Entity Relationship Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for EntityRelationshipCode {
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