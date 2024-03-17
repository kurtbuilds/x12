use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**559

See docs at <https://www.stedi.com/edi/x12/element/559>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AgencyQualifierCode {
    ///10 - Alabama
    Alabama,
    ///11 - Alaska
    Alaska,
    ///12 - Arizona
    Arizona,
    ///13 - Arkansas
    Arkansas,
    ///14 - California
    California,
    ///15 - Colorado
    Colorado,
    ///16 - Connecticut
    Connecticut,
    ///17 - Delaware
    Delaware,
    ///18 - District of Columbia
    DistrictOfColumbia,
    ///19 - Florida
    Florida,
    ///20 - Georgia
    Georgia,
    ///21 - Hawaii
    Hawaii,
    ///22 - Idaho
    Idaho,
    ///23 - Illinois
    Illinois,
    ///24 - Indiana
    Indiana,
    ///25 - Iowa
    Iowa,
    ///26 - Kansas
    Kansas,
    ///27 - Louisiana
    Louisiana,
    ///28 - Kentucky
    Kentucky,
    ///29 - Maine
    Maine,
    ///30 - Maryland
    Maryland,
    ///31 - Massachusetts
    Massachusetts,
    ///32 - Michigan
    Michigan,
    ///33 - Minnesota
    Minnesota,
    ///34 - Mississippi
    Mississippi,
    ///35 - Missouri
    Missouri,
    ///36 - Montana
    Montana,
    ///37 - Nebraska
    Nebraska,
    ///38 - Nevada
    Nevada,
    ///39 - New Hampshire
    NewHampshire,
    ///40 - New Jersey
    NewJersey,
    ///41 - New Mexico
    NewMexico,
    ///42 - New York
    NewYork,
    ///43 - North Carolina
    NorthCarolina,
    ///44 - North Dakota
    NorthDakota,
    ///45 - Ohio
    Ohio,
    ///46 - Oklahoma
    Oklahoma,
    ///47 - Oregon
    Oregon,
    ///48 - Pennsylvania
    Pennsylvania,
    ///49 - Rhode Island
    RhodeIsland,
    ///50 - South Carolina
    SouthCarolina,
    ///51 - South Dakota
    SouthDakota,
    ///52 - Tennessee
    Tennessee,
    ///53 - Texas
    Texas,
    ///54 - Utah
    Utah,
    ///55 - Vermont
    Vermont,
    ///56 - Virginia
    Virginia,
    ///57 - Washington
    Washington,
    ///58 - West Virginia
    WestVirginia,
    ///59 - Wisconsin
    Wisconsin,
    ///60 - Wyoming
    Wyoming,
    ///61 - Insurance Services Office (ISO)
    Code61,
    ///62 - National Crime Information Center (NCIC)
    Code62,
    ///64 - U.S. National Center for Health Statistics Commission of Professional and Hospital Activities
    USNationalCenterForHealthStatisticsCommissionOfProfessionalAndHospitalActivities,
    ///65 - Office of Workers Compensation Programs
    OfficeOfWorkersCompensationPrograms,
    ///66 - National Association of Convenience Stores
    NationalAssociationOfConvenienceStores,
    ///93 - Dun & Bradstreet
    Code93,
    ///94 - Code Assigned by the Organization that is the Ultimate Destination of the Transaction Set
    CodeAssignedByTheOrganizationThatIsTheUltimateDestinationOfTheTransactionSet,
    ///A1 - American Land Title Association
    AmericanLandTitleAssociation,
    ///A2 - California Land Title Association
    CaliforniaLandTitleAssociation,
    ///A3 - Texas Land Title Association
    TexasLandTitleAssociation,
    ///A4 - Assigned by Carrier
    AssignedByCarrier,
    ///AA - Aluminum Association
    AluminumAssociation,
    ///AB - Assigned by Buyer
    AssignedByBuyer,
    ///AC - American Conference of Government Industrial Hygienists (ACGIH)
    CodeAC,
    ///AD - Agency Company Organization for Research and Development (ACORD)
    CodeAD,
    ///AE - Advertising Industry
    AdvertisingIndustry,
    ///AF - Automotive Aftermarket Industry Association (AAIA)
    CodeAF,
    ///AG - State Agency Assigned
    StateAgencyAssigned,
    ///AH - American Industrial Hygiene Association (AIHA)
    CodeAH,
    ///AJ - Real Estate Information Industry
    RealEstateInformationIndustry,
    ///AL - National Alcohol Beverage Control Association
    NationalAlcoholBeverageControlAssociation,
    ///AM - American Medical Association
    AmericanMedicalAssociation,
    ///AP - American Petroleum Institute
    AmericanPetroleumInstitute,
    ///AQ - American Public Works Association (APWA) One Call Systems International (OCSI)
    CodeAQ,
    ///AR - Association of American Railroads
    AssociationOfAmericanRailroads,
    ///AS - Assigned by Seller
    AssignedBySeller,
    ///AT - American Society for Testing and Materials (ASTM)
    CodeAT,
    ///AW - American Welding Society (AWS)
    CodeAW,
    ///AX - ANSI Accredited Standards Committee, X12
    CodeAX,
    ///AY - American Yarn Spinners Association (AYSA)
    CodeAY,
    ///BA - Business and Institutional Furniture Manufacturer's Association (BIFMA)
    CodeBA,
    ///BE - Telcordia Technologies
    TelcordiaTechnologies,
    ///BF - National Business Forms Association
    NationalBusinessFormsAssociation,
    ///BI - Book Industry Systems Advisory Committee
    BookIndustrySystemsAdvisoryCommittee,
    ///CA - Chemical Abstract Services (CAS)
    CodeCA,
    ///CB - Copper and Brass Fabricators Council, Inc.
    CodeCB,
    ///CC - National Cotton Council of America
    NationalCottonCouncilOfAmerica,
    ///CE - Construction Specification Institute (CSI) Extended
    CodeCE,
    ///CI - Chemical Industry Data Exchange (CIDX)
    CodeCI,
    ///CL - Collision Industry Electronic Commerce Association (CIECA)
    CodeCL,
    ///CM - Carbide Manufacturers
    CarbideManufacturers,
    ///CO - County Designator Code
    CountyDesignatorCode,
    ///CP - United States Department of Agriculture, Agricultural Marketing Service (AMS), Cotton Programs
    CodeCP,
    ///CR - Commander - Rome Air Development Center
    CommanderRomeAirDevelopmentCenter,
    ///CS - Construction Specification Institute (CSI)
    CodeCS,
    ///CU - Committee on Uniform Security Identification Procedures (CUSIP)
    CodeCU,
    ///CX - National Association of Corrosion Engineers (NACE)
    CodeCX,
    ///DA - Food and Drug Administration (FDA)
    CodeDA,
    ///DD - Department of Defense (Military Specifications)
    CodeDD,
    ///DE - Drug Enforcement Administration
    DrugEnforcementAdministration,
    ///DF - Department of Defense (DoD)
    CodeDF,
    ///DH - Defense Logistics Information Service
    DefenseLogisticsInformationService,
    ///DI - Deutsches Institut fur Normung (DIN)
    CodeDI,
    ///DL - Defense Logistics Agency
    DefenseLogisticsAgency,
    ///DN - Department of the Navy
    DepartmentOfTheNavy,
    ///DO - United States Department of Transportation (DOT)
    CodeDO,
    ///DR - Healthcare Distribution Management Association
    HealthcareDistributionManagementAssociation,
    ///DS - Defense Finance and Accounting Service (DFAS)
    CodeDS,
    ///DX - United States Marine Corps
    UnitedStatesMarineCorps,
    ///DY - Department of Air Force
    DepartmentOfAirForce,
    ///DZ - Department of Army
    DepartmentOfArmy,
    ///EI - Electronic Industries Association
    ElectronicIndustriesAssociation,
    ///EP - United States Environmental Protection Agency (EPA)
    CodeEP,
    ///ES - Environment and Safety Data Exchange (ESDX)
    CodeES,
    ///ET - Temporary Help Industry
    TemporaryHelpIndustry,
    ///EU - Electric Utilities
    ElectricUtilities,
    ///EX - Electronics Industry Data Exchange (EIDX)
    CodeEX,
    ///FA - Fabric and Supplier Linkage Council (FASLINC)
    CodeFA,
    ///FC - Federal Communications Commission (FCC)
    CodeFC,
    ///FD - Uniform Code Council (UCS)
    CodeFD,
    ///FG - Federal Government
    FederalGovernment,
    ///FH - Federal Highway Administration
    FederalHighwayAdministration,
    ///FI - American Furniture Manufacturers Association
    AmericanFurnitureManufacturersAssociation,
    ///GC - Graphics Communications Association
    GraphicsCommunicationsAssociation,
    ///GI - Gas Industry Standards Board
    GasIndustryStandardsBoard,
    ///GS - General Services Administration (GSA)
    CodeGS,
    ///GU - Natural Gas Utilities
    NaturalGasUtilities,
    ///HC - Centers for Medicare and Medicaid Services
    CentersForMedicareAndMedicaidServices,
    ///HF - Human Factors and Ergonomics Society
    HumanFactorsAndErgonomicsSociety,
    ///HI - Health Insurance Association of America
    HealthInsuranceAssociationOfAmerica,
    ///HS - Department of Health and Human Services
    DepartmentOfHealthAndHumanServices,
    ///HU - Department of Housing and Urban Development
    DepartmentOfHousingAndUrbanDevelopment,
    ///IA - International Agency for Research on Cancer (IARC)
    CodeIA,
    ///IB - International Association of Industrial Accident Boards and Commissions
    InternationalAssociationOfIndustrialAccidentBoardsAndCommissions,
    ///IC - International Air Transport Association (IATA)
    CodeIC,
    ///IM - Iron and Steel Standards Committee ISM
    IronAndSteelStandardsCommitteeIsm,
    ///IN - International Association of Corporation Administrators
    InternationalAssociationOfCorporationAdministrators,
    ///IS - International Standards Organization
    InternationalStandardsOrganization,
    ///JA - Japanese Standards Association
    JapaneseStandardsAssociation,
    ///LA - Life and Annuity Industry Committee
    LifeAndAnnuityIndustryCommittee,
    ///LB - Department of Labor
    DepartmentOfLabor,
    ///LI - Leasing Industry
    LeasingIndustry,
    ///MA - Mortgage Bankers Association of America
    MortgageBankersAssociationOfAmerica,
    ///MB - Office of Management and Budget
    OfficeOfManagementAndBudget,
    ///MC - Manufacturing Company
    ManufacturingCompany,
    ///ME - American Society of Mechanical Engineers (ASME)
    CodeME,
    ///MI - ABCD - The Microcomputer Industry Association
    AbcdTheMicrocomputerIndustryAssociation,
    ///MP - Material Safety Data Sheet (MSDS) Provider
    CodeMP,
    ///MS - Military Standard
    MilitaryStandard,
    ///MV - American Association of Motor Vehicle Administrators (AAMVA)
    CodeMV,
    ///NA - National Insurance Crime Bureau (NICB)
    CodeNA,
    ///NB - National Association of Business and Educational Radio
    NationalAssociationOfBusinessAndEducationalRadio,
    ///NC - National Council on Compensation Insurance
    NationalCouncilOnCompensationInsurance,
    ///NE - National Electric Manufacturers Association (NEMA)
    CodeNE,
    ///NF - National Fire Protection Agency (NFPA)
    CodeNF,
    ///NG - National Auto Glass Specification (NAGS)
    CodeNG,
    ///NI - National Institute of Occupational Safety and Health (NIOSH)
    CodeNI,
    ///NP - National Association of Pharmacy Regulatory Authorities (NAPRA)
    CodeNP,
    ///NR - National Retail Merchants Association
    NationalRetailMerchantsAssociation,
    ///NS - National Center for State Courts
    NationalCenterForStateCourts,
    ///NT - National Toxicology Program (NTP)
    CodeNT,
    ///NU - United States Nuclear Regulatory Commission
    UnitedStatesNuclearRegulatoryCommission,
    ///NW - Newspaper Association of America
    NewspaperAssociationOfAmerica,
    ///OI - Optical Industry
    OpticalIndustry,
    ///OP - Office Products
    OfficeProducts,
    ///OS - United States Occupational Safety and Health Administration (OSHA)
    CodeOS,
    ///PA - American Paper Institute
    AmericanPaperInstitute,
    ///PC - Pennsylvania Courts
    PennsylvaniaCourts,
    ///PI - Society for the Plastics Industry (SPI)
    CodePI,
    ///RN - RosettaNet
    RosettaNet,
    ///SA - Society of Automotive Engineers, Inc. (SAE)
    CodeSA,
    ///SE - Serials Industry Systems Advisory Committee (SISAC)
    CodeSE,
    ///SL - Student Loan Guarantor
    StudentLoanGuarantor,
    ///SP - American Society for Automation in Pharmacy
    AmericanSocietyForAutomationInPharmacy,
    ///ST - American Iron & Steel Institute
    CodeST,
    ///TA - Air Transport Association of America
    AirTransportAssociationOfAmerica,
    ///TB - Textile Distributors Association, Inc.
    CodeTB,
    ///TC - Textile Apparel Linkage Council (TALC)
    CodeTC,
    ///TD - Transportation Data Coordinating Committee: Electronic Data Interchange Association (TDCC:EDIA)
    CodeTD,
    ///TI - Telecommunications Industry
    TelecommunicationsIndustry,
    ///TM - American Textile Manufacturers Institute
    AmericanTextileManufacturersInstitute,
    ///TP - Canadian Freight Classification
    CanadianFreightClassification,
    ///TR - American Trucking Associations
    AmericanTruckingAssociations,
    ///TX - American Apparel Manufacturers Association
    AmericanApparelManufacturersAssociation,
    ///UA - (UN/SPSC) United Nations Products and Services Classification Code
    CodeUA,
    ///UC - United States Courts
    UnitedStatesCourts,
    ///UI - Industrial/Commercial (I/C) Electronic Data Interchange
    CodeUI,
    ///UL - Underwriters Laboratories
    UnderwritersLaboratories,
    ///UN - United Nations (UN)
    CodeUN,
    ///UT - Utility Industry Group
    UtilityIndustryGroup,
    ///VI - Voluntary Inter-Industry Commerce Standard (VICS) EDI
    CodeVI,
    ///WH - Canadian Workplace Hazardous Materials Information System (WHMIS)
    CodeWH,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl AgencyQualifierCode {
    pub fn code(&self) -> &str {
        {
            use AgencyQualifierCode::*;
            match self {
                Alabama => "10",
                Alaska => "11",
                Arizona => "12",
                Arkansas => "13",
                California => "14",
                Colorado => "15",
                Connecticut => "16",
                Delaware => "17",
                DistrictOfColumbia => "18",
                Florida => "19",
                Georgia => "20",
                Hawaii => "21",
                Idaho => "22",
                Illinois => "23",
                Indiana => "24",
                Iowa => "25",
                Kansas => "26",
                Louisiana => "27",
                Kentucky => "28",
                Maine => "29",
                Maryland => "30",
                Massachusetts => "31",
                Michigan => "32",
                Minnesota => "33",
                Mississippi => "34",
                Missouri => "35",
                Montana => "36",
                Nebraska => "37",
                Nevada => "38",
                NewHampshire => "39",
                NewJersey => "40",
                NewMexico => "41",
                NewYork => "42",
                NorthCarolina => "43",
                NorthDakota => "44",
                Ohio => "45",
                Oklahoma => "46",
                Oregon => "47",
                Pennsylvania => "48",
                RhodeIsland => "49",
                SouthCarolina => "50",
                SouthDakota => "51",
                Tennessee => "52",
                Texas => "53",
                Utah => "54",
                Vermont => "55",
                Virginia => "56",
                Washington => "57",
                WestVirginia => "58",
                Wisconsin => "59",
                Wyoming => "60",
                Code61 => "61",
                Code62 => "62",
                USNationalCenterForHealthStatisticsCommissionOfProfessionalAndHospitalActivities => {
                    "64"
                }
                OfficeOfWorkersCompensationPrograms => "65",
                NationalAssociationOfConvenienceStores => "66",
                Code93 => "93",
                CodeAssignedByTheOrganizationThatIsTheUltimateDestinationOfTheTransactionSet => {
                    "94"
                }
                AmericanLandTitleAssociation => "A1",
                CaliforniaLandTitleAssociation => "A2",
                TexasLandTitleAssociation => "A3",
                AssignedByCarrier => "A4",
                AluminumAssociation => "AA",
                AssignedByBuyer => "AB",
                CodeAC => "AC",
                CodeAD => "AD",
                AdvertisingIndustry => "AE",
                CodeAF => "AF",
                StateAgencyAssigned => "AG",
                CodeAH => "AH",
                RealEstateInformationIndustry => "AJ",
                NationalAlcoholBeverageControlAssociation => "AL",
                AmericanMedicalAssociation => "AM",
                AmericanPetroleumInstitute => "AP",
                CodeAQ => "AQ",
                AssociationOfAmericanRailroads => "AR",
                AssignedBySeller => "AS",
                CodeAT => "AT",
                CodeAW => "AW",
                CodeAX => "AX",
                CodeAY => "AY",
                CodeBA => "BA",
                TelcordiaTechnologies => "BE",
                NationalBusinessFormsAssociation => "BF",
                BookIndustrySystemsAdvisoryCommittee => "BI",
                CodeCA => "CA",
                CodeCB => "CB",
                NationalCottonCouncilOfAmerica => "CC",
                CodeCE => "CE",
                CodeCI => "CI",
                CodeCL => "CL",
                CarbideManufacturers => "CM",
                CountyDesignatorCode => "CO",
                CodeCP => "CP",
                CommanderRomeAirDevelopmentCenter => "CR",
                CodeCS => "CS",
                CodeCU => "CU",
                CodeCX => "CX",
                CodeDA => "DA",
                CodeDD => "DD",
                DrugEnforcementAdministration => "DE",
                CodeDF => "DF",
                DefenseLogisticsInformationService => "DH",
                CodeDI => "DI",
                DefenseLogisticsAgency => "DL",
                DepartmentOfTheNavy => "DN",
                CodeDO => "DO",
                HealthcareDistributionManagementAssociation => "DR",
                CodeDS => "DS",
                UnitedStatesMarineCorps => "DX",
                DepartmentOfAirForce => "DY",
                DepartmentOfArmy => "DZ",
                ElectronicIndustriesAssociation => "EI",
                CodeEP => "EP",
                CodeES => "ES",
                TemporaryHelpIndustry => "ET",
                ElectricUtilities => "EU",
                CodeEX => "EX",
                CodeFA => "FA",
                CodeFC => "FC",
                CodeFD => "FD",
                FederalGovernment => "FG",
                FederalHighwayAdministration => "FH",
                AmericanFurnitureManufacturersAssociation => "FI",
                GraphicsCommunicationsAssociation => "GC",
                GasIndustryStandardsBoard => "GI",
                CodeGS => "GS",
                NaturalGasUtilities => "GU",
                CentersForMedicareAndMedicaidServices => "HC",
                HumanFactorsAndErgonomicsSociety => "HF",
                HealthInsuranceAssociationOfAmerica => "HI",
                DepartmentOfHealthAndHumanServices => "HS",
                DepartmentOfHousingAndUrbanDevelopment => "HU",
                CodeIA => "IA",
                InternationalAssociationOfIndustrialAccidentBoardsAndCommissions => "IB",
                CodeIC => "IC",
                IronAndSteelStandardsCommitteeIsm => "IM",
                InternationalAssociationOfCorporationAdministrators => "IN",
                InternationalStandardsOrganization => "IS",
                JapaneseStandardsAssociation => "JA",
                LifeAndAnnuityIndustryCommittee => "LA",
                DepartmentOfLabor => "LB",
                LeasingIndustry => "LI",
                MortgageBankersAssociationOfAmerica => "MA",
                OfficeOfManagementAndBudget => "MB",
                ManufacturingCompany => "MC",
                CodeME => "ME",
                AbcdTheMicrocomputerIndustryAssociation => "MI",
                CodeMP => "MP",
                MilitaryStandard => "MS",
                CodeMV => "MV",
                CodeNA => "NA",
                NationalAssociationOfBusinessAndEducationalRadio => "NB",
                NationalCouncilOnCompensationInsurance => "NC",
                CodeNE => "NE",
                CodeNF => "NF",
                CodeNG => "NG",
                CodeNI => "NI",
                CodeNP => "NP",
                NationalRetailMerchantsAssociation => "NR",
                NationalCenterForStateCourts => "NS",
                CodeNT => "NT",
                UnitedStatesNuclearRegulatoryCommission => "NU",
                NewspaperAssociationOfAmerica => "NW",
                OpticalIndustry => "OI",
                OfficeProducts => "OP",
                CodeOS => "OS",
                AmericanPaperInstitute => "PA",
                PennsylvaniaCourts => "PC",
                CodePI => "PI",
                RosettaNet => "RN",
                CodeSA => "SA",
                CodeSE => "SE",
                StudentLoanGuarantor => "SL",
                AmericanSocietyForAutomationInPharmacy => "SP",
                CodeST => "ST",
                AirTransportAssociationOfAmerica => "TA",
                CodeTB => "TB",
                CodeTC => "TC",
                CodeTD => "TD",
                TelecommunicationsIndustry => "TI",
                AmericanTextileManufacturersInstitute => "TM",
                CanadianFreightClassification => "TP",
                AmericanTruckingAssociations => "TR",
                AmericanApparelManufacturersAssociation => "TX",
                CodeUA => "UA",
                UnitedStatesCourts => "UC",
                CodeUI => "UI",
                UnderwritersLaboratories => "UL",
                CodeUN => "UN",
                UtilityIndustryGroup => "UT",
                CodeVI => "VI",
                CodeWH => "WH",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<AgencyQualifierCode> {
        use AgencyQualifierCode::*;
        match code {
            b"10" => Some(Alabama),
            b"11" => Some(Alaska),
            b"12" => Some(Arizona),
            b"13" => Some(Arkansas),
            b"14" => Some(California),
            b"15" => Some(Colorado),
            b"16" => Some(Connecticut),
            b"17" => Some(Delaware),
            b"18" => Some(DistrictOfColumbia),
            b"19" => Some(Florida),
            b"20" => Some(Georgia),
            b"21" => Some(Hawaii),
            b"22" => Some(Idaho),
            b"23" => Some(Illinois),
            b"24" => Some(Indiana),
            b"25" => Some(Iowa),
            b"26" => Some(Kansas),
            b"27" => Some(Louisiana),
            b"28" => Some(Kentucky),
            b"29" => Some(Maine),
            b"30" => Some(Maryland),
            b"31" => Some(Massachusetts),
            b"32" => Some(Michigan),
            b"33" => Some(Minnesota),
            b"34" => Some(Mississippi),
            b"35" => Some(Missouri),
            b"36" => Some(Montana),
            b"37" => Some(Nebraska),
            b"38" => Some(Nevada),
            b"39" => Some(NewHampshire),
            b"40" => Some(NewJersey),
            b"41" => Some(NewMexico),
            b"42" => Some(NewYork),
            b"43" => Some(NorthCarolina),
            b"44" => Some(NorthDakota),
            b"45" => Some(Ohio),
            b"46" => Some(Oklahoma),
            b"47" => Some(Oregon),
            b"48" => Some(Pennsylvania),
            b"49" => Some(RhodeIsland),
            b"50" => Some(SouthCarolina),
            b"51" => Some(SouthDakota),
            b"52" => Some(Tennessee),
            b"53" => Some(Texas),
            b"54" => Some(Utah),
            b"55" => Some(Vermont),
            b"56" => Some(Virginia),
            b"57" => Some(Washington),
            b"58" => Some(WestVirginia),
            b"59" => Some(Wisconsin),
            b"60" => Some(Wyoming),
            b"61" => Some(Code61),
            b"62" => Some(Code62),
            b"64" => {
                Some(
                    USNationalCenterForHealthStatisticsCommissionOfProfessionalAndHospitalActivities,
                )
            }
            b"65" => Some(OfficeOfWorkersCompensationPrograms),
            b"66" => Some(NationalAssociationOfConvenienceStores),
            b"93" => Some(Code93),
            b"94" => {
                Some(
                    CodeAssignedByTheOrganizationThatIsTheUltimateDestinationOfTheTransactionSet,
                )
            }
            b"A1" => Some(AmericanLandTitleAssociation),
            b"A2" => Some(CaliforniaLandTitleAssociation),
            b"A3" => Some(TexasLandTitleAssociation),
            b"A4" => Some(AssignedByCarrier),
            b"AA" => Some(AluminumAssociation),
            b"AB" => Some(AssignedByBuyer),
            b"AC" => Some(CodeAC),
            b"AD" => Some(CodeAD),
            b"AE" => Some(AdvertisingIndustry),
            b"AF" => Some(CodeAF),
            b"AG" => Some(StateAgencyAssigned),
            b"AH" => Some(CodeAH),
            b"AJ" => Some(RealEstateInformationIndustry),
            b"AL" => Some(NationalAlcoholBeverageControlAssociation),
            b"AM" => Some(AmericanMedicalAssociation),
            b"AP" => Some(AmericanPetroleumInstitute),
            b"AQ" => Some(CodeAQ),
            b"AR" => Some(AssociationOfAmericanRailroads),
            b"AS" => Some(AssignedBySeller),
            b"AT" => Some(CodeAT),
            b"AW" => Some(CodeAW),
            b"AX" => Some(CodeAX),
            b"AY" => Some(CodeAY),
            b"BA" => Some(CodeBA),
            b"BE" => Some(TelcordiaTechnologies),
            b"BF" => Some(NationalBusinessFormsAssociation),
            b"BI" => Some(BookIndustrySystemsAdvisoryCommittee),
            b"CA" => Some(CodeCA),
            b"CB" => Some(CodeCB),
            b"CC" => Some(NationalCottonCouncilOfAmerica),
            b"CE" => Some(CodeCE),
            b"CI" => Some(CodeCI),
            b"CL" => Some(CodeCL),
            b"CM" => Some(CarbideManufacturers),
            b"CO" => Some(CountyDesignatorCode),
            b"CP" => Some(CodeCP),
            b"CR" => Some(CommanderRomeAirDevelopmentCenter),
            b"CS" => Some(CodeCS),
            b"CU" => Some(CodeCU),
            b"CX" => Some(CodeCX),
            b"DA" => Some(CodeDA),
            b"DD" => Some(CodeDD),
            b"DE" => Some(DrugEnforcementAdministration),
            b"DF" => Some(CodeDF),
            b"DH" => Some(DefenseLogisticsInformationService),
            b"DI" => Some(CodeDI),
            b"DL" => Some(DefenseLogisticsAgency),
            b"DN" => Some(DepartmentOfTheNavy),
            b"DO" => Some(CodeDO),
            b"DR" => Some(HealthcareDistributionManagementAssociation),
            b"DS" => Some(CodeDS),
            b"DX" => Some(UnitedStatesMarineCorps),
            b"DY" => Some(DepartmentOfAirForce),
            b"DZ" => Some(DepartmentOfArmy),
            b"EI" => Some(ElectronicIndustriesAssociation),
            b"EP" => Some(CodeEP),
            b"ES" => Some(CodeES),
            b"ET" => Some(TemporaryHelpIndustry),
            b"EU" => Some(ElectricUtilities),
            b"EX" => Some(CodeEX),
            b"FA" => Some(CodeFA),
            b"FC" => Some(CodeFC),
            b"FD" => Some(CodeFD),
            b"FG" => Some(FederalGovernment),
            b"FH" => Some(FederalHighwayAdministration),
            b"FI" => Some(AmericanFurnitureManufacturersAssociation),
            b"GC" => Some(GraphicsCommunicationsAssociation),
            b"GI" => Some(GasIndustryStandardsBoard),
            b"GS" => Some(CodeGS),
            b"GU" => Some(NaturalGasUtilities),
            b"HC" => Some(CentersForMedicareAndMedicaidServices),
            b"HF" => Some(HumanFactorsAndErgonomicsSociety),
            b"HI" => Some(HealthInsuranceAssociationOfAmerica),
            b"HS" => Some(DepartmentOfHealthAndHumanServices),
            b"HU" => Some(DepartmentOfHousingAndUrbanDevelopment),
            b"IA" => Some(CodeIA),
            b"IB" => {
                Some(InternationalAssociationOfIndustrialAccidentBoardsAndCommissions)
            }
            b"IC" => Some(CodeIC),
            b"IM" => Some(IronAndSteelStandardsCommitteeIsm),
            b"IN" => Some(InternationalAssociationOfCorporationAdministrators),
            b"IS" => Some(InternationalStandardsOrganization),
            b"JA" => Some(JapaneseStandardsAssociation),
            b"LA" => Some(LifeAndAnnuityIndustryCommittee),
            b"LB" => Some(DepartmentOfLabor),
            b"LI" => Some(LeasingIndustry),
            b"MA" => Some(MortgageBankersAssociationOfAmerica),
            b"MB" => Some(OfficeOfManagementAndBudget),
            b"MC" => Some(ManufacturingCompany),
            b"ME" => Some(CodeME),
            b"MI" => Some(AbcdTheMicrocomputerIndustryAssociation),
            b"MP" => Some(CodeMP),
            b"MS" => Some(MilitaryStandard),
            b"MV" => Some(CodeMV),
            b"NA" => Some(CodeNA),
            b"NB" => Some(NationalAssociationOfBusinessAndEducationalRadio),
            b"NC" => Some(NationalCouncilOnCompensationInsurance),
            b"NE" => Some(CodeNE),
            b"NF" => Some(CodeNF),
            b"NG" => Some(CodeNG),
            b"NI" => Some(CodeNI),
            b"NP" => Some(CodeNP),
            b"NR" => Some(NationalRetailMerchantsAssociation),
            b"NS" => Some(NationalCenterForStateCourts),
            b"NT" => Some(CodeNT),
            b"NU" => Some(UnitedStatesNuclearRegulatoryCommission),
            b"NW" => Some(NewspaperAssociationOfAmerica),
            b"OI" => Some(OpticalIndustry),
            b"OP" => Some(OfficeProducts),
            b"OS" => Some(CodeOS),
            b"PA" => Some(AmericanPaperInstitute),
            b"PC" => Some(PennsylvaniaCourts),
            b"PI" => Some(CodePI),
            b"RN" => Some(RosettaNet),
            b"SA" => Some(CodeSA),
            b"SE" => Some(CodeSE),
            b"SL" => Some(StudentLoanGuarantor),
            b"SP" => Some(AmericanSocietyForAutomationInPharmacy),
            b"ST" => Some(CodeST),
            b"TA" => Some(AirTransportAssociationOfAmerica),
            b"TB" => Some(CodeTB),
            b"TC" => Some(CodeTC),
            b"TD" => Some(CodeTD),
            b"TI" => Some(TelecommunicationsIndustry),
            b"TM" => Some(AmericanTextileManufacturersInstitute),
            b"TP" => Some(CanadianFreightClassification),
            b"TR" => Some(AmericanTruckingAssociations),
            b"TX" => Some(AmericanApparelManufacturersAssociation),
            b"UA" => Some(CodeUA),
            b"UC" => Some(UnitedStatesCourts),
            b"UI" => Some(CodeUI),
            b"UL" => Some(UnderwritersLaboratories),
            b"UN" => Some(CodeUN),
            b"UT" => Some(UtilityIndustryGroup),
            b"VI" => Some(CodeVI),
            b"WH" => Some(CodeWH),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use AgencyQualifierCode::*;
        match self {
            Alabama => "Alabama",
            Alaska => "Alaska",
            Arizona => "Arizona",
            Arkansas => "Arkansas",
            California => "California",
            Colorado => "Colorado",
            Connecticut => "Connecticut",
            Delaware => "Delaware",
            DistrictOfColumbia => "District of Columbia",
            Florida => "Florida",
            Georgia => "Georgia",
            Hawaii => "Hawaii",
            Idaho => "Idaho",
            Illinois => "Illinois",
            Indiana => "Indiana",
            Iowa => "Iowa",
            Kansas => "Kansas",
            Louisiana => "Louisiana",
            Kentucky => "Kentucky",
            Maine => "Maine",
            Maryland => "Maryland",
            Massachusetts => "Massachusetts",
            Michigan => "Michigan",
            Minnesota => "Minnesota",
            Mississippi => "Mississippi",
            Missouri => "Missouri",
            Montana => "Montana",
            Nebraska => "Nebraska",
            Nevada => "Nevada",
            NewHampshire => "New Hampshire",
            NewJersey => "New Jersey",
            NewMexico => "New Mexico",
            NewYork => "New York",
            NorthCarolina => "North Carolina",
            NorthDakota => "North Dakota",
            Ohio => "Ohio",
            Oklahoma => "Oklahoma",
            Oregon => "Oregon",
            Pennsylvania => "Pennsylvania",
            RhodeIsland => "Rhode Island",
            SouthCarolina => "South Carolina",
            SouthDakota => "South Dakota",
            Tennessee => "Tennessee",
            Texas => "Texas",
            Utah => "Utah",
            Vermont => "Vermont",
            Virginia => "Virginia",
            Washington => "Washington",
            WestVirginia => "West Virginia",
            Wisconsin => "Wisconsin",
            Wyoming => "Wyoming",
            Code61 => "Insurance Services Office (ISO)",
            Code62 => "National Crime Information Center (NCIC)",
            USNationalCenterForHealthStatisticsCommissionOfProfessionalAndHospitalActivities => {
                "U.S. National Center for Health Statistics Commission of Professional and Hospital Activities"
            }
            OfficeOfWorkersCompensationPrograms => {
                "Office of Workers Compensation Programs"
            }
            NationalAssociationOfConvenienceStores => {
                "National Association of Convenience Stores"
            }
            Code93 => "Dun & Bradstreet",
            CodeAssignedByTheOrganizationThatIsTheUltimateDestinationOfTheTransactionSet => {
                "Code Assigned by the Organization that is the Ultimate Destination of the Transaction Set"
            }
            AmericanLandTitleAssociation => "American Land Title Association",
            CaliforniaLandTitleAssociation => "California Land Title Association",
            TexasLandTitleAssociation => "Texas Land Title Association",
            AssignedByCarrier => "Assigned by Carrier",
            AluminumAssociation => "Aluminum Association",
            AssignedByBuyer => "Assigned by Buyer",
            CodeAC => "American Conference of Government Industrial Hygienists (ACGIH)",
            CodeAD => "Agency Company Organization for Research and Development (ACORD)",
            AdvertisingIndustry => "Advertising Industry",
            CodeAF => "Automotive Aftermarket Industry Association (AAIA)",
            StateAgencyAssigned => "State Agency Assigned",
            CodeAH => "American Industrial Hygiene Association (AIHA)",
            RealEstateInformationIndustry => "Real Estate Information Industry",
            NationalAlcoholBeverageControlAssociation => {
                "National Alcohol Beverage Control Association"
            }
            AmericanMedicalAssociation => "American Medical Association",
            AmericanPetroleumInstitute => "American Petroleum Institute",
            CodeAQ => {
                "American Public Works Association (APWA) One Call Systems International (OCSI)"
            }
            AssociationOfAmericanRailroads => "Association of American Railroads",
            AssignedBySeller => "Assigned by Seller",
            CodeAT => "American Society for Testing and Materials (ASTM)",
            CodeAW => "American Welding Society (AWS)",
            CodeAX => "ANSI Accredited Standards Committee, X12",
            CodeAY => "American Yarn Spinners Association (AYSA)",
            CodeBA => {
                "Business and Institutional Furniture Manufacturer's Association (BIFMA)"
            }
            TelcordiaTechnologies => "Telcordia Technologies",
            NationalBusinessFormsAssociation => "National Business Forms Association",
            BookIndustrySystemsAdvisoryCommittee => {
                "Book Industry Systems Advisory Committee"
            }
            CodeCA => "Chemical Abstract Services (CAS)",
            CodeCB => "Copper and Brass Fabricators Council, Inc.",
            NationalCottonCouncilOfAmerica => "National Cotton Council of America",
            CodeCE => "Construction Specification Institute (CSI) Extended",
            CodeCI => "Chemical Industry Data Exchange (CIDX)",
            CodeCL => "Collision Industry Electronic Commerce Association (CIECA)",
            CarbideManufacturers => "Carbide Manufacturers",
            CountyDesignatorCode => "County Designator Code",
            CodeCP => {
                "United States Department of Agriculture, Agricultural Marketing Service (AMS), Cotton Programs"
            }
            CommanderRomeAirDevelopmentCenter => {
                "Commander - Rome Air Development Center"
            }
            CodeCS => "Construction Specification Institute (CSI)",
            CodeCU => "Committee on Uniform Security Identification Procedures (CUSIP)",
            CodeCX => "National Association of Corrosion Engineers (NACE)",
            CodeDA => "Food and Drug Administration (FDA)",
            CodeDD => "Department of Defense (Military Specifications)",
            DrugEnforcementAdministration => "Drug Enforcement Administration",
            CodeDF => "Department of Defense (DoD)",
            DefenseLogisticsInformationService => "Defense Logistics Information Service",
            CodeDI => "Deutsches Institut fur Normung (DIN)",
            DefenseLogisticsAgency => "Defense Logistics Agency",
            DepartmentOfTheNavy => "Department of the Navy",
            CodeDO => "United States Department of Transportation (DOT)",
            HealthcareDistributionManagementAssociation => {
                "Healthcare Distribution Management Association"
            }
            CodeDS => "Defense Finance and Accounting Service (DFAS)",
            UnitedStatesMarineCorps => "United States Marine Corps",
            DepartmentOfAirForce => "Department of Air Force",
            DepartmentOfArmy => "Department of Army",
            ElectronicIndustriesAssociation => "Electronic Industries Association",
            CodeEP => "United States Environmental Protection Agency (EPA)",
            CodeES => "Environment and Safety Data Exchange (ESDX)",
            TemporaryHelpIndustry => "Temporary Help Industry",
            ElectricUtilities => "Electric Utilities",
            CodeEX => "Electronics Industry Data Exchange (EIDX)",
            CodeFA => "Fabric and Supplier Linkage Council (FASLINC)",
            CodeFC => "Federal Communications Commission (FCC)",
            CodeFD => "Uniform Code Council (UCS)",
            FederalGovernment => "Federal Government",
            FederalHighwayAdministration => "Federal Highway Administration",
            AmericanFurnitureManufacturersAssociation => {
                "American Furniture Manufacturers Association"
            }
            GraphicsCommunicationsAssociation => "Graphics Communications Association",
            GasIndustryStandardsBoard => "Gas Industry Standards Board",
            CodeGS => "General Services Administration (GSA)",
            NaturalGasUtilities => "Natural Gas Utilities",
            CentersForMedicareAndMedicaidServices => {
                "Centers for Medicare and Medicaid Services"
            }
            HumanFactorsAndErgonomicsSociety => "Human Factors and Ergonomics Society",
            HealthInsuranceAssociationOfAmerica => {
                "Health Insurance Association of America"
            }
            DepartmentOfHealthAndHumanServices => {
                "Department of Health and Human Services"
            }
            DepartmentOfHousingAndUrbanDevelopment => {
                "Department of Housing and Urban Development"
            }
            CodeIA => "International Agency for Research on Cancer (IARC)",
            InternationalAssociationOfIndustrialAccidentBoardsAndCommissions => {
                "International Association of Industrial Accident Boards and Commissions"
            }
            CodeIC => "International Air Transport Association (IATA)",
            IronAndSteelStandardsCommitteeIsm => "Iron and Steel Standards Committee ISM",
            InternationalAssociationOfCorporationAdministrators => {
                "International Association of Corporation Administrators"
            }
            InternationalStandardsOrganization => "International Standards Organization",
            JapaneseStandardsAssociation => "Japanese Standards Association",
            LifeAndAnnuityIndustryCommittee => "Life and Annuity Industry Committee",
            DepartmentOfLabor => "Department of Labor",
            LeasingIndustry => "Leasing Industry",
            MortgageBankersAssociationOfAmerica => {
                "Mortgage Bankers Association of America"
            }
            OfficeOfManagementAndBudget => "Office of Management and Budget",
            ManufacturingCompany => "Manufacturing Company",
            CodeME => "American Society of Mechanical Engineers (ASME)",
            AbcdTheMicrocomputerIndustryAssociation => {
                "ABCD - The Microcomputer Industry Association"
            }
            CodeMP => "Material Safety Data Sheet (MSDS) Provider",
            MilitaryStandard => "Military Standard",
            CodeMV => "American Association of Motor Vehicle Administrators (AAMVA)",
            CodeNA => "National Insurance Crime Bureau (NICB)",
            NationalAssociationOfBusinessAndEducationalRadio => {
                "National Association of Business and Educational Radio"
            }
            NationalCouncilOnCompensationInsurance => {
                "National Council on Compensation Insurance"
            }
            CodeNE => "National Electric Manufacturers Association (NEMA)",
            CodeNF => "National Fire Protection Agency (NFPA)",
            CodeNG => "National Auto Glass Specification (NAGS)",
            CodeNI => "National Institute of Occupational Safety and Health (NIOSH)",
            CodeNP => "National Association of Pharmacy Regulatory Authorities (NAPRA)",
            NationalRetailMerchantsAssociation => "National Retail Merchants Association",
            NationalCenterForStateCourts => "National Center for State Courts",
            CodeNT => "National Toxicology Program (NTP)",
            UnitedStatesNuclearRegulatoryCommission => {
                "United States Nuclear Regulatory Commission"
            }
            NewspaperAssociationOfAmerica => "Newspaper Association of America",
            OpticalIndustry => "Optical Industry",
            OfficeProducts => "Office Products",
            CodeOS => {
                "United States Occupational Safety and Health Administration (OSHA)"
            }
            AmericanPaperInstitute => "American Paper Institute",
            PennsylvaniaCourts => "Pennsylvania Courts",
            CodePI => "Society for the Plastics Industry (SPI)",
            RosettaNet => "RosettaNet",
            CodeSA => "Society of Automotive Engineers, Inc. (SAE)",
            CodeSE => "Serials Industry Systems Advisory Committee (SISAC)",
            StudentLoanGuarantor => "Student Loan Guarantor",
            AmericanSocietyForAutomationInPharmacy => {
                "American Society for Automation in Pharmacy"
            }
            CodeST => "American Iron & Steel Institute",
            AirTransportAssociationOfAmerica => "Air Transport Association of America",
            CodeTB => "Textile Distributors Association, Inc.",
            CodeTC => "Textile Apparel Linkage Council (TALC)",
            CodeTD => {
                "Transportation Data Coordinating Committee: Electronic Data Interchange Association (TDCC:EDIA)"
            }
            TelecommunicationsIndustry => "Telecommunications Industry",
            AmericanTextileManufacturersInstitute => {
                "American Textile Manufacturers Institute"
            }
            CanadianFreightClassification => "Canadian Freight Classification",
            AmericanTruckingAssociations => "American Trucking Associations",
            AmericanApparelManufacturersAssociation => {
                "American Apparel Manufacturers Association"
            }
            CodeUA => {
                "(UN/SPSC) United Nations Products and Services Classification Code"
            }
            UnitedStatesCourts => "United States Courts",
            CodeUI => "Industrial/Commercial (I/C) Electronic Data Interchange",
            UnderwritersLaboratories => "Underwriters Laboratories",
            CodeUN => "United Nations (UN)",
            UtilityIndustryGroup => "Utility Industry Group",
            CodeVI => "Voluntary Inter-Industry Commerce Standard (VICS) EDI",
            CodeWH => "Canadian Workplace Hazardous Materials Information System (WHMIS)",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<AgencyQualifierCode> {
        {
            use AgencyQualifierCode::*;
            match description {
                "Alabama" => Some(Alabama),
                "Alaska" => Some(Alaska),
                "Arizona" => Some(Arizona),
                "Arkansas" => Some(Arkansas),
                "California" => Some(California),
                "Colorado" => Some(Colorado),
                "Connecticut" => Some(Connecticut),
                "Delaware" => Some(Delaware),
                "District of Columbia" => Some(DistrictOfColumbia),
                "Florida" => Some(Florida),
                "Georgia" => Some(Georgia),
                "Hawaii" => Some(Hawaii),
                "Idaho" => Some(Idaho),
                "Illinois" => Some(Illinois),
                "Indiana" => Some(Indiana),
                "Iowa" => Some(Iowa),
                "Kansas" => Some(Kansas),
                "Louisiana" => Some(Louisiana),
                "Kentucky" => Some(Kentucky),
                "Maine" => Some(Maine),
                "Maryland" => Some(Maryland),
                "Massachusetts" => Some(Massachusetts),
                "Michigan" => Some(Michigan),
                "Minnesota" => Some(Minnesota),
                "Mississippi" => Some(Mississippi),
                "Missouri" => Some(Missouri),
                "Montana" => Some(Montana),
                "Nebraska" => Some(Nebraska),
                "Nevada" => Some(Nevada),
                "New Hampshire" => Some(NewHampshire),
                "New Jersey" => Some(NewJersey),
                "New Mexico" => Some(NewMexico),
                "New York" => Some(NewYork),
                "North Carolina" => Some(NorthCarolina),
                "North Dakota" => Some(NorthDakota),
                "Ohio" => Some(Ohio),
                "Oklahoma" => Some(Oklahoma),
                "Oregon" => Some(Oregon),
                "Pennsylvania" => Some(Pennsylvania),
                "Rhode Island" => Some(RhodeIsland),
                "South Carolina" => Some(SouthCarolina),
                "South Dakota" => Some(SouthDakota),
                "Tennessee" => Some(Tennessee),
                "Texas" => Some(Texas),
                "Utah" => Some(Utah),
                "Vermont" => Some(Vermont),
                "Virginia" => Some(Virginia),
                "Washington" => Some(Washington),
                "West Virginia" => Some(WestVirginia),
                "Wisconsin" => Some(Wisconsin),
                "Wyoming" => Some(Wyoming),
                "Insurance Services Office (ISO)" => Some(Code61),
                "National Crime Information Center (NCIC)" => Some(Code62),
                "U.S. National Center for Health Statistics Commission of Professional and Hospital Activities" => {
                    Some(
                        USNationalCenterForHealthStatisticsCommissionOfProfessionalAndHospitalActivities,
                    )
                }
                "Office of Workers Compensation Programs" => {
                    Some(OfficeOfWorkersCompensationPrograms)
                }
                "National Association of Convenience Stores" => {
                    Some(NationalAssociationOfConvenienceStores)
                }
                "Dun & Bradstreet" => Some(Code93),
                "Code Assigned by the Organization that is the Ultimate Destination of the Transaction Set" => {
                    Some(
                        CodeAssignedByTheOrganizationThatIsTheUltimateDestinationOfTheTransactionSet,
                    )
                }
                "American Land Title Association" => Some(AmericanLandTitleAssociation),
                "California Land Title Association" => {
                    Some(CaliforniaLandTitleAssociation)
                }
                "Texas Land Title Association" => Some(TexasLandTitleAssociation),
                "Assigned by Carrier" => Some(AssignedByCarrier),
                "Aluminum Association" => Some(AluminumAssociation),
                "Assigned by Buyer" => Some(AssignedByBuyer),
                "American Conference of Government Industrial Hygienists (ACGIH)" => {
                    Some(CodeAC)
                }
                "Agency Company Organization for Research and Development (ACORD)" => {
                    Some(CodeAD)
                }
                "Advertising Industry" => Some(AdvertisingIndustry),
                "Automotive Aftermarket Industry Association (AAIA)" => Some(CodeAF),
                "State Agency Assigned" => Some(StateAgencyAssigned),
                "American Industrial Hygiene Association (AIHA)" => Some(CodeAH),
                "Real Estate Information Industry" => Some(RealEstateInformationIndustry),
                "National Alcohol Beverage Control Association" => {
                    Some(NationalAlcoholBeverageControlAssociation)
                }
                "American Medical Association" => Some(AmericanMedicalAssociation),
                "American Petroleum Institute" => Some(AmericanPetroleumInstitute),
                "American Public Works Association (APWA) One Call Systems International (OCSI)" => {
                    Some(CodeAQ)
                }
                "Association of American Railroads" => {
                    Some(AssociationOfAmericanRailroads)
                }
                "Assigned by Seller" => Some(AssignedBySeller),
                "American Society for Testing and Materials (ASTM)" => Some(CodeAT),
                "American Welding Society (AWS)" => Some(CodeAW),
                "ANSI Accredited Standards Committee, X12" => Some(CodeAX),
                "American Yarn Spinners Association (AYSA)" => Some(CodeAY),
                "Business and Institutional Furniture Manufacturer's Association (BIFMA)" => {
                    Some(CodeBA)
                }
                "Telcordia Technologies" => Some(TelcordiaTechnologies),
                "National Business Forms Association" => {
                    Some(NationalBusinessFormsAssociation)
                }
                "Book Industry Systems Advisory Committee" => {
                    Some(BookIndustrySystemsAdvisoryCommittee)
                }
                "Chemical Abstract Services (CAS)" => Some(CodeCA),
                "Copper and Brass Fabricators Council, Inc." => Some(CodeCB),
                "National Cotton Council of America" => {
                    Some(NationalCottonCouncilOfAmerica)
                }
                "Construction Specification Institute (CSI) Extended" => Some(CodeCE),
                "Chemical Industry Data Exchange (CIDX)" => Some(CodeCI),
                "Collision Industry Electronic Commerce Association (CIECA)" => {
                    Some(CodeCL)
                }
                "Carbide Manufacturers" => Some(CarbideManufacturers),
                "County Designator Code" => Some(CountyDesignatorCode),
                "United States Department of Agriculture, Agricultural Marketing Service (AMS), Cotton Programs" => {
                    Some(CodeCP)
                }
                "Commander - Rome Air Development Center" => {
                    Some(CommanderRomeAirDevelopmentCenter)
                }
                "Construction Specification Institute (CSI)" => Some(CodeCS),
                "Committee on Uniform Security Identification Procedures (CUSIP)" => {
                    Some(CodeCU)
                }
                "National Association of Corrosion Engineers (NACE)" => Some(CodeCX),
                "Food and Drug Administration (FDA)" => Some(CodeDA),
                "Department of Defense (Military Specifications)" => Some(CodeDD),
                "Drug Enforcement Administration" => Some(DrugEnforcementAdministration),
                "Department of Defense (DoD)" => Some(CodeDF),
                "Defense Logistics Information Service" => {
                    Some(DefenseLogisticsInformationService)
                }
                "Deutsches Institut fur Normung (DIN)" => Some(CodeDI),
                "Defense Logistics Agency" => Some(DefenseLogisticsAgency),
                "Department of the Navy" => Some(DepartmentOfTheNavy),
                "United States Department of Transportation (DOT)" => Some(CodeDO),
                "Healthcare Distribution Management Association" => {
                    Some(HealthcareDistributionManagementAssociation)
                }
                "Defense Finance and Accounting Service (DFAS)" => Some(CodeDS),
                "United States Marine Corps" => Some(UnitedStatesMarineCorps),
                "Department of Air Force" => Some(DepartmentOfAirForce),
                "Department of Army" => Some(DepartmentOfArmy),
                "Electronic Industries Association" => {
                    Some(ElectronicIndustriesAssociation)
                }
                "United States Environmental Protection Agency (EPA)" => Some(CodeEP),
                "Environment and Safety Data Exchange (ESDX)" => Some(CodeES),
                "Temporary Help Industry" => Some(TemporaryHelpIndustry),
                "Electric Utilities" => Some(ElectricUtilities),
                "Electronics Industry Data Exchange (EIDX)" => Some(CodeEX),
                "Fabric and Supplier Linkage Council (FASLINC)" => Some(CodeFA),
                "Federal Communications Commission (FCC)" => Some(CodeFC),
                "Uniform Code Council (UCS)" => Some(CodeFD),
                "Federal Government" => Some(FederalGovernment),
                "Federal Highway Administration" => Some(FederalHighwayAdministration),
                "American Furniture Manufacturers Association" => {
                    Some(AmericanFurnitureManufacturersAssociation)
                }
                "Graphics Communications Association" => {
                    Some(GraphicsCommunicationsAssociation)
                }
                "Gas Industry Standards Board" => Some(GasIndustryStandardsBoard),
                "General Services Administration (GSA)" => Some(CodeGS),
                "Natural Gas Utilities" => Some(NaturalGasUtilities),
                "Centers for Medicare and Medicaid Services" => {
                    Some(CentersForMedicareAndMedicaidServices)
                }
                "Human Factors and Ergonomics Society" => {
                    Some(HumanFactorsAndErgonomicsSociety)
                }
                "Health Insurance Association of America" => {
                    Some(HealthInsuranceAssociationOfAmerica)
                }
                "Department of Health and Human Services" => {
                    Some(DepartmentOfHealthAndHumanServices)
                }
                "Department of Housing and Urban Development" => {
                    Some(DepartmentOfHousingAndUrbanDevelopment)
                }
                "International Agency for Research on Cancer (IARC)" => Some(CodeIA),
                "International Association of Industrial Accident Boards and Commissions" => {
                    Some(
                        InternationalAssociationOfIndustrialAccidentBoardsAndCommissions,
                    )
                }
                "International Air Transport Association (IATA)" => Some(CodeIC),
                "Iron and Steel Standards Committee ISM" => {
                    Some(IronAndSteelStandardsCommitteeIsm)
                }
                "International Association of Corporation Administrators" => {
                    Some(InternationalAssociationOfCorporationAdministrators)
                }
                "International Standards Organization" => {
                    Some(InternationalStandardsOrganization)
                }
                "Japanese Standards Association" => Some(JapaneseStandardsAssociation),
                "Life and Annuity Industry Committee" => {
                    Some(LifeAndAnnuityIndustryCommittee)
                }
                "Department of Labor" => Some(DepartmentOfLabor),
                "Leasing Industry" => Some(LeasingIndustry),
                "Mortgage Bankers Association of America" => {
                    Some(MortgageBankersAssociationOfAmerica)
                }
                "Office of Management and Budget" => Some(OfficeOfManagementAndBudget),
                "Manufacturing Company" => Some(ManufacturingCompany),
                "American Society of Mechanical Engineers (ASME)" => Some(CodeME),
                "ABCD - The Microcomputer Industry Association" => {
                    Some(AbcdTheMicrocomputerIndustryAssociation)
                }
                "Material Safety Data Sheet (MSDS) Provider" => Some(CodeMP),
                "Military Standard" => Some(MilitaryStandard),
                "American Association of Motor Vehicle Administrators (AAMVA)" => {
                    Some(CodeMV)
                }
                "National Insurance Crime Bureau (NICB)" => Some(CodeNA),
                "National Association of Business and Educational Radio" => {
                    Some(NationalAssociationOfBusinessAndEducationalRadio)
                }
                "National Council on Compensation Insurance" => {
                    Some(NationalCouncilOnCompensationInsurance)
                }
                "National Electric Manufacturers Association (NEMA)" => Some(CodeNE),
                "National Fire Protection Agency (NFPA)" => Some(CodeNF),
                "National Auto Glass Specification (NAGS)" => Some(CodeNG),
                "National Institute of Occupational Safety and Health (NIOSH)" => {
                    Some(CodeNI)
                }
                "National Association of Pharmacy Regulatory Authorities (NAPRA)" => {
                    Some(CodeNP)
                }
                "National Retail Merchants Association" => {
                    Some(NationalRetailMerchantsAssociation)
                }
                "National Center for State Courts" => Some(NationalCenterForStateCourts),
                "National Toxicology Program (NTP)" => Some(CodeNT),
                "United States Nuclear Regulatory Commission" => {
                    Some(UnitedStatesNuclearRegulatoryCommission)
                }
                "Newspaper Association of America" => Some(NewspaperAssociationOfAmerica),
                "Optical Industry" => Some(OpticalIndustry),
                "Office Products" => Some(OfficeProducts),
                "United States Occupational Safety and Health Administration (OSHA)" => {
                    Some(CodeOS)
                }
                "American Paper Institute" => Some(AmericanPaperInstitute),
                "Pennsylvania Courts" => Some(PennsylvaniaCourts),
                "Society for the Plastics Industry (SPI)" => Some(CodePI),
                "RosettaNet" => Some(RosettaNet),
                "Society of Automotive Engineers, Inc. (SAE)" => Some(CodeSA),
                "Serials Industry Systems Advisory Committee (SISAC)" => Some(CodeSE),
                "Student Loan Guarantor" => Some(StudentLoanGuarantor),
                "American Society for Automation in Pharmacy" => {
                    Some(AmericanSocietyForAutomationInPharmacy)
                }
                "American Iron & Steel Institute" => Some(CodeST),
                "Air Transport Association of America" => {
                    Some(AirTransportAssociationOfAmerica)
                }
                "Textile Distributors Association, Inc." => Some(CodeTB),
                "Textile Apparel Linkage Council (TALC)" => Some(CodeTC),
                "Transportation Data Coordinating Committee: Electronic Data Interchange Association (TDCC:EDIA)" => {
                    Some(CodeTD)
                }
                "Telecommunications Industry" => Some(TelecommunicationsIndustry),
                "American Textile Manufacturers Institute" => {
                    Some(AmericanTextileManufacturersInstitute)
                }
                "Canadian Freight Classification" => Some(CanadianFreightClassification),
                "American Trucking Associations" => Some(AmericanTruckingAssociations),
                "American Apparel Manufacturers Association" => {
                    Some(AmericanApparelManufacturersAssociation)
                }
                "(UN/SPSC) United Nations Products and Services Classification Code" => {
                    Some(CodeUA)
                }
                "United States Courts" => Some(UnitedStatesCourts),
                "Industrial/Commercial (I/C) Electronic Data Interchange" => Some(CodeUI),
                "Underwriters Laboratories" => Some(UnderwritersLaboratories),
                "United Nations (UN)" => Some(CodeUN),
                "Utility Industry Group" => Some(UtilityIndustryGroup),
                "Voluntary Inter-Industry Commerce Standard (VICS) EDI" => Some(CodeVI),
                "Canadian Workplace Hazardous Materials Information System (WHMIS)" => {
                    Some(CodeWH)
                }
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for AgencyQualifierCode {
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
    type Value = AgencyQualifierCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Agency Qualifier Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        AgencyQualifierCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Agency Qualifier Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        AgencyQualifierCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Agency Qualifier Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for AgencyQualifierCode {
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