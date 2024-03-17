use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**309

See docs at <https://www.stedi.com/edi/x12/element/309>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocationQualifier {
    ///10 - Nearest Cross Street
    NearestCrossStreet,
    ///11 - Secondary Cross Street
    SecondaryCrossStreet,
    ///12 - Range
    Range,
    ///13 - Section
    Section,
    ///14 - Quarter Section
    QuarterSection,
    ///18 - Marker Identifier Location
    MarkerIdentifierLocation,
    ///19 - Route
    Route,
    ///20 - Route Subdivision
    RouteSubdivision,
    ///21 - Grid Location
    GridLocation,
    ///22 - Page
    Page,
    ///27 - Marker Type
    MarkerType,
    ///28 - Latitude-Longitude Source
    LatitudeLongitudeSource,
    ///29 - Map Source
    MapSource,
    ///30 - Map Reference
    MapReference,
    ///31 - Grid Source
    GridSource,
    ///32 - Aliquot
    Aliquot,
    ///33 - Block
    Block,
    ///34 - District
    District,
    ///35 - Drainhole Number
    DrainholeNumber,
    ///36 - City Block
    CityBlock,
    ///38 - Footage Call Direction
    FootageCallDirection,
    ///39 - Location Direction
    LocationDirection,
    ///40 - Outer Continental Lease Location
    OuterContinentalLeaseLocation,
    ///41 - Lot
    Lot,
    ///42 - Map Quadrangle
    MapQuadrangle,
    ///43 - Principal Meridian
    PrincipalMeridian,
    ///44 - Outer Continental Shelf Area
    OuterContinentalShelfArea,
    ///45 - Outer Continental Shelf Block
    OuterContinentalShelfBlock,
    ///46 - Official Protraction Diagram
    OfficialProtractionDiagram,
    ///47 - Quarter Quarter Quarter Section
    QuarterQuarterQuarterSection,
    ///48 - Quarter Quarter Section
    QuarterQuarterSection,
    ///49 - Section Type
    SectionType,
    ///50 - Abstract
    Abstract,
    ///52 - Labor
    Labor,
    ///53 - League
    League,
    ///54 - Survey
    Survey,
    ///55 - Tier
    Tier,
    ///57 - Tract
    Tract,
    ///58 - Universal Transverse Mercator Quadrant
    UniversalTransverseMercatorQuadrant,
    ///59 - Course Direction
    CourseDirection,
    ///60 - Area
    Area,
    ///93 - Sender's Location Code
    SendersLocationCode,
    ///94 - Receiver's Location Code
    ReceiversLocationCode,
    ///A - Jurisdiction to Receive Credit for Uniform Commercial Code Filing
    JurisdictionToReceiveCreditForUniformCommercialCodeFiling,
    ///A1 - Office
    Office,
    ///AA - Annual Statements Mailing Address
    AnnualStatementsMailingAddress,
    ///AC - City and State
    CityAndState,
    ///AP - All Points
    AllPoints,
    ///AR - Armed Services Location Designation
    ArmedServicesLocationDesignation,
    ///B - Transmitting Utility
    TransmittingUtility,
    ///B1 - Branch
    Branch,
    ///BE - Business Economic Area (BEA) Region Code
    CodeBE,
    ///BL - Government Bill of Lading Office Code (GBLOC)
    CodeBL,
    ///BS - Place of Business
    PlaceOfBusiness,
    ///C - Consignor
    Consignor,
    ///C2 - Geopolitical Name Code
    GeopoliticalNameCode,
    ///CA - Country of Origin
    CountryOfOrigin,
    ///CB - Confirmation Mailing Address
    ConfirmationMailingAddress,
    ///CC - Country
    Country,
    ///CD - Canada Customs Office Code
    CanadaCustomsOfficeCode,
    ///CE - Correspondence Mailing Address
    CorrespondenceMailingAddress,
    ///CG - Congressional District
    CongressionalDistrict,
    ///CI - City
    City,
    ///CL - National Rate Basis (NRB)
    CodeCL,
    ///CM - Consolidated Metropolitan Statistical Area (CMSA)
    CodeCM,
    ///CO - County/Parish and State
    CountyParishAndState,
    ///CR - In Tank Car
    InTankCar,
    ///CS - Canadian SPLC
    CanadianSplc,
    ///CY - County/Parish
    CountyParish,
    ///D - Census Schedule D
    CensusScheduleD,
    ///DC - Distribution Center Number
    DistributionCenterNumber,
    ///DE - Destination (Shipping)
    CodeDE,
    ///DL - Delivery Location
    DeliveryLocation,
    ///DO - District Office
    DistrictOffice,
    ///DP - Department
    Department,
    ///DR - District of Residence
    DistrictOfResidence,
    ///DT - Domicile Type Code
    DomicileTypeCode,
    ///E - Uniform Commercial Code Filing Office
    UniformCommercialCodeFilingOffice,
    ///EA - Event Location
    EventLocation,
    ///EB - Borough
    Borough,
    ///EL - Employer Location
    EmployerLocation,
    ///F - Current Address
    CurrentAddress,
    ///FA - Factory
    Factory,
    ///FE - Freight Equalization Point
    FreightEqualizationPoint,
    ///FF - Foreign Freight Forwarder Location
    ForeignFreightForwarderLocation,
    ///FI - Federal Information Processing Standards (FIPS) 55 (Named Populated Places)
    CodeFI,
    ///FR - U.S. Custom's Facilities Information and Resource Management Systems (FIRMS)
    CodeFR,
    ///FS - Freight Station Accounting Code
    FreightStationAccountingCode,
    ///FT - Foreign Trade Zone
    ForeignTradeZone,
    ///FV - Free Alongside Vessel (Free On Board [F.O.B.] Point)
    CodeFV,
    ///G - Census Block Group
    CensusBlockGroup,
    ///GL - Freight Station Geographic Location
    FreightStationGeographicLocation,
    ///H - Home Address
    HomeAddress,
    ///I - Home Base Address
    HomeBaseAddress,
    ///IA - International Air Transport Association (IATA) Location Qualifier
    CodeIA,
    ///IB - Issue Location
    IssueLocation,
    ///IM - Military Standard Movement Procedures (MILSTAMP)
    CodeIM,
    ///IP - Postal
    Postal,
    ///IS - In Store
    InStore,
    ///IT - Intermediate FOB Point
    IntermediateFobPoint,
    ///J - Census Tract
    CensusTract,
    ///K - Census Schedule K
    CensusScheduleK,
    ///KE - Port of Embarkation
    PortOfEmbarkation,
    ///KL - Port of Loading
    PortOfLoading,
    ///KP - Government Furnished Property FOB Point
    GovernmentFurnishedPropertyFobPoint,
    ///L - Local Address
    LocalAddress,
    ///LO - Local Office
    LocalOffice,
    ///M - Mailing Address
    MailingAddress,
    ///MI - Mill
    Mill,
    ///MO - Main Campus
    MainCampus,
    ///MS - Metropolitan Sampling Area (MSA) Region Code
    CodeMS,
    ///MZ - Mexican Postal Code
    MexicanPostalCode,
    ///N - Bureau International des Containers et du Transport Intermodal (BIC) facility depot codes
    CodeN,
    ///NS - City/State from Points
    CityStateFromPoints,
    ///O - Office Address
    OfficeAddress,
    ///OA - Origin (After Loading on Equipment)
    CodeOA,
    ///OF - Other Unlisted Free On Board (FOB) Point
    CodeOF,
    ///OL - Open and Prepay Station List Code(SCAC & Number)
    CodeOL,
    ///OP - Other Unlisted Acceptance Point
    OtherUnlistedAcceptancePoint,
    ///OR - Origin (Shipping Point)
    CodeOR,
    ///OV - On Vessel (Free On Board [FOB] point)
    CodeOV,
    ///P - Permanent Address
    PermanentAddress,
    ///PA - Port of Arrival
    PortOfArrival,
    ///PB - Port of Discharge
    PortOfDischarge,
    ///PC - Policy Mailing Address
    PolicyMailingAddress,
    ///PD - Place of Delivery
    PlaceOfDelivery,
    ///PE - Port of Entry
    PortOfEntry,
    ///PF - Parents Address
    ParentsAddress,
    ///PG - Primary
    Primary,
    ///PH - Prior Business
    PriorBusiness,
    ///PL - Plant
    Plant,
    ///PM - Primary Metropolitan Statistical Area (PMSA)
    CodePM,
    ///PO - Principal Servicing Office
    PrincipalServicingOffice,
    ///PP - Pool Point
    PoolPoint,
    ///PQ - 3 Digit U.S. ZIP
    CodePQ,
    ///PR - 4 Digit U.S. ZIP
    CodePR,
    ///PS - 5 Digit U.S. ZIP
    CodePS,
    ///PT - 3 Digit Canadian Postal Code
    CodePT,
    ///PU - 6 Digit Canadian Postal Code
    CodePU,
    ///PV - 9 DIGIT U.S. ZIP
    CodePV,
    ///PZ - 11 DIGIT U.S. ZIP
    CodePZ,
    ///Q - Birthplace
    Birthplace,
    ///R - Ship Message Design Group (SMDG) terminal codes
    CodeR,
    ///RA - Rate Area Code
    RateAreaCode,
    ///RC - In Rail Car
    InRailCar,
    ///RE - Regional Education Service Agency
    RegionalEducationServiceAgency,
    ///RG - Region Code
    RegionCode,
    ///RJ - Region
    Region,
    ///RL - Rural
    Rural,
    ///RS - Standard Carrier Alpha Code
    StandardCarrierAlphaCode,
    ///RT - Route Administrative Message To
    RouteAdministrativeMessageTo,
    ///SA - Secondary
    Secondary,
    ///SB - Suburban
    Suburban,
    ///SC - City/State and Points Within
    CityStateAndPointsWithin,
    ///SD - School District
    SchoolDistrict,
    ///SE - Summer
    Summer,
    ///SG - Storage
    Storage,
    ///SH - School Campus Code
    SchoolCampusCode,
    ///SL - U.S. SPLC
    USSplc,
    ///SN - Store Number
    StoreNumber,
    ///SP - State/Province
    StateProvince,
    ///SS - School
    School,
    ///ST - In Storage Tank
    InStorageTank,
    ///SW - Switching District
    SwitchingDistrict,
    ///TA - Tank
    Tank,
    ///TC - Transcontinental Freight Bureau
    TranscontinentalFreightBureau,
    ///TI - Tribal Land
    TribalLand,
    ///TL - Terminal Cargo Location
    TerminalCargoLocation,
    ///TM - Terminal
    Terminal,
    ///TN - Township
    Township,
    ///TP - Temporary
    Temporary,
    ///TR - Rail Territory
    RailTerritory,
    ///TX - Taxing District
    TaxingDistrict,
    ///UN - United Nations Location Code (UNLOCODE)
    CodeUN,
    ///UR - Urban
    Urban,
    ///UT - Business Unit
    BusinessUnit,
    ///VA - Vacation
    Vacation,
    ///VI - Village
    Village,
    ///VS - Vessel Stowage Location
    VesselStowageLocation,
    ///W - Worldwide Geographic Location Code
    WorldwideGeographicLocationCode,
    ///WF - Wharf
    Wharf,
    ///WH - Warehouse
    Warehouse,
    ///WI - Winter
    Winter,
    ///X1 - National Center for Education Statistics Locale Code
    NationalCenterForEducationStatisticsLocaleCode,
    ///ZN - Zone Code
    ZoneCode,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl LocationQualifier {
    pub fn code(&self) -> &str {
        {
            use LocationQualifier::*;
            match self {
                NearestCrossStreet => "10",
                SecondaryCrossStreet => "11",
                Range => "12",
                Section => "13",
                QuarterSection => "14",
                MarkerIdentifierLocation => "18",
                Route => "19",
                RouteSubdivision => "20",
                GridLocation => "21",
                Page => "22",
                MarkerType => "27",
                LatitudeLongitudeSource => "28",
                MapSource => "29",
                MapReference => "30",
                GridSource => "31",
                Aliquot => "32",
                Block => "33",
                District => "34",
                DrainholeNumber => "35",
                CityBlock => "36",
                FootageCallDirection => "38",
                LocationDirection => "39",
                OuterContinentalLeaseLocation => "40",
                Lot => "41",
                MapQuadrangle => "42",
                PrincipalMeridian => "43",
                OuterContinentalShelfArea => "44",
                OuterContinentalShelfBlock => "45",
                OfficialProtractionDiagram => "46",
                QuarterQuarterQuarterSection => "47",
                QuarterQuarterSection => "48",
                SectionType => "49",
                Abstract => "50",
                Labor => "52",
                League => "53",
                Survey => "54",
                Tier => "55",
                Tract => "57",
                UniversalTransverseMercatorQuadrant => "58",
                CourseDirection => "59",
                Area => "60",
                SendersLocationCode => "93",
                ReceiversLocationCode => "94",
                JurisdictionToReceiveCreditForUniformCommercialCodeFiling => "A",
                Office => "A1",
                AnnualStatementsMailingAddress => "AA",
                CityAndState => "AC",
                AllPoints => "AP",
                ArmedServicesLocationDesignation => "AR",
                TransmittingUtility => "B",
                Branch => "B1",
                CodeBE => "BE",
                CodeBL => "BL",
                PlaceOfBusiness => "BS",
                Consignor => "C",
                GeopoliticalNameCode => "C2",
                CountryOfOrigin => "CA",
                ConfirmationMailingAddress => "CB",
                Country => "CC",
                CanadaCustomsOfficeCode => "CD",
                CorrespondenceMailingAddress => "CE",
                CongressionalDistrict => "CG",
                City => "CI",
                CodeCL => "CL",
                CodeCM => "CM",
                CountyParishAndState => "CO",
                InTankCar => "CR",
                CanadianSplc => "CS",
                CountyParish => "CY",
                CensusScheduleD => "D",
                DistributionCenterNumber => "DC",
                CodeDE => "DE",
                DeliveryLocation => "DL",
                DistrictOffice => "DO",
                Department => "DP",
                DistrictOfResidence => "DR",
                DomicileTypeCode => "DT",
                UniformCommercialCodeFilingOffice => "E",
                EventLocation => "EA",
                Borough => "EB",
                EmployerLocation => "EL",
                CurrentAddress => "F",
                Factory => "FA",
                FreightEqualizationPoint => "FE",
                ForeignFreightForwarderLocation => "FF",
                CodeFI => "FI",
                CodeFR => "FR",
                FreightStationAccountingCode => "FS",
                ForeignTradeZone => "FT",
                CodeFV => "FV",
                CensusBlockGroup => "G",
                FreightStationGeographicLocation => "GL",
                HomeAddress => "H",
                HomeBaseAddress => "I",
                CodeIA => "IA",
                IssueLocation => "IB",
                CodeIM => "IM",
                Postal => "IP",
                InStore => "IS",
                IntermediateFobPoint => "IT",
                CensusTract => "J",
                CensusScheduleK => "K",
                PortOfEmbarkation => "KE",
                PortOfLoading => "KL",
                GovernmentFurnishedPropertyFobPoint => "KP",
                LocalAddress => "L",
                LocalOffice => "LO",
                MailingAddress => "M",
                Mill => "MI",
                MainCampus => "MO",
                CodeMS => "MS",
                MexicanPostalCode => "MZ",
                CodeN => "N",
                CityStateFromPoints => "NS",
                OfficeAddress => "O",
                CodeOA => "OA",
                CodeOF => "OF",
                CodeOL => "OL",
                OtherUnlistedAcceptancePoint => "OP",
                CodeOR => "OR",
                CodeOV => "OV",
                PermanentAddress => "P",
                PortOfArrival => "PA",
                PortOfDischarge => "PB",
                PolicyMailingAddress => "PC",
                PlaceOfDelivery => "PD",
                PortOfEntry => "PE",
                ParentsAddress => "PF",
                Primary => "PG",
                PriorBusiness => "PH",
                Plant => "PL",
                CodePM => "PM",
                PrincipalServicingOffice => "PO",
                PoolPoint => "PP",
                CodePQ => "PQ",
                CodePR => "PR",
                CodePS => "PS",
                CodePT => "PT",
                CodePU => "PU",
                CodePV => "PV",
                CodePZ => "PZ",
                Birthplace => "Q",
                CodeR => "R",
                RateAreaCode => "RA",
                InRailCar => "RC",
                RegionalEducationServiceAgency => "RE",
                RegionCode => "RG",
                Region => "RJ",
                Rural => "RL",
                StandardCarrierAlphaCode => "RS",
                RouteAdministrativeMessageTo => "RT",
                Secondary => "SA",
                Suburban => "SB",
                CityStateAndPointsWithin => "SC",
                SchoolDistrict => "SD",
                Summer => "SE",
                Storage => "SG",
                SchoolCampusCode => "SH",
                USSplc => "SL",
                StoreNumber => "SN",
                StateProvince => "SP",
                School => "SS",
                InStorageTank => "ST",
                SwitchingDistrict => "SW",
                Tank => "TA",
                TranscontinentalFreightBureau => "TC",
                TribalLand => "TI",
                TerminalCargoLocation => "TL",
                Terminal => "TM",
                Township => "TN",
                Temporary => "TP",
                RailTerritory => "TR",
                TaxingDistrict => "TX",
                CodeUN => "UN",
                Urban => "UR",
                BusinessUnit => "UT",
                Vacation => "VA",
                Village => "VI",
                VesselStowageLocation => "VS",
                WorldwideGeographicLocationCode => "W",
                Wharf => "WF",
                Warehouse => "WH",
                Winter => "WI",
                NationalCenterForEducationStatisticsLocaleCode => "X1",
                ZoneCode => "ZN",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<LocationQualifier> {
        use LocationQualifier::*;
        match code {
            b"10" => Some(NearestCrossStreet),
            b"11" => Some(SecondaryCrossStreet),
            b"12" => Some(Range),
            b"13" => Some(Section),
            b"14" => Some(QuarterSection),
            b"18" => Some(MarkerIdentifierLocation),
            b"19" => Some(Route),
            b"20" => Some(RouteSubdivision),
            b"21" => Some(GridLocation),
            b"22" => Some(Page),
            b"27" => Some(MarkerType),
            b"28" => Some(LatitudeLongitudeSource),
            b"29" => Some(MapSource),
            b"30" => Some(MapReference),
            b"31" => Some(GridSource),
            b"32" => Some(Aliquot),
            b"33" => Some(Block),
            b"34" => Some(District),
            b"35" => Some(DrainholeNumber),
            b"36" => Some(CityBlock),
            b"38" => Some(FootageCallDirection),
            b"39" => Some(LocationDirection),
            b"40" => Some(OuterContinentalLeaseLocation),
            b"41" => Some(Lot),
            b"42" => Some(MapQuadrangle),
            b"43" => Some(PrincipalMeridian),
            b"44" => Some(OuterContinentalShelfArea),
            b"45" => Some(OuterContinentalShelfBlock),
            b"46" => Some(OfficialProtractionDiagram),
            b"47" => Some(QuarterQuarterQuarterSection),
            b"48" => Some(QuarterQuarterSection),
            b"49" => Some(SectionType),
            b"50" => Some(Abstract),
            b"52" => Some(Labor),
            b"53" => Some(League),
            b"54" => Some(Survey),
            b"55" => Some(Tier),
            b"57" => Some(Tract),
            b"58" => Some(UniversalTransverseMercatorQuadrant),
            b"59" => Some(CourseDirection),
            b"60" => Some(Area),
            b"93" => Some(SendersLocationCode),
            b"94" => Some(ReceiversLocationCode),
            b"A" => Some(JurisdictionToReceiveCreditForUniformCommercialCodeFiling),
            b"A1" => Some(Office),
            b"AA" => Some(AnnualStatementsMailingAddress),
            b"AC" => Some(CityAndState),
            b"AP" => Some(AllPoints),
            b"AR" => Some(ArmedServicesLocationDesignation),
            b"B" => Some(TransmittingUtility),
            b"B1" => Some(Branch),
            b"BE" => Some(CodeBE),
            b"BL" => Some(CodeBL),
            b"BS" => Some(PlaceOfBusiness),
            b"C" => Some(Consignor),
            b"C2" => Some(GeopoliticalNameCode),
            b"CA" => Some(CountryOfOrigin),
            b"CB" => Some(ConfirmationMailingAddress),
            b"CC" => Some(Country),
            b"CD" => Some(CanadaCustomsOfficeCode),
            b"CE" => Some(CorrespondenceMailingAddress),
            b"CG" => Some(CongressionalDistrict),
            b"CI" => Some(City),
            b"CL" => Some(CodeCL),
            b"CM" => Some(CodeCM),
            b"CO" => Some(CountyParishAndState),
            b"CR" => Some(InTankCar),
            b"CS" => Some(CanadianSplc),
            b"CY" => Some(CountyParish),
            b"D" => Some(CensusScheduleD),
            b"DC" => Some(DistributionCenterNumber),
            b"DE" => Some(CodeDE),
            b"DL" => Some(DeliveryLocation),
            b"DO" => Some(DistrictOffice),
            b"DP" => Some(Department),
            b"DR" => Some(DistrictOfResidence),
            b"DT" => Some(DomicileTypeCode),
            b"E" => Some(UniformCommercialCodeFilingOffice),
            b"EA" => Some(EventLocation),
            b"EB" => Some(Borough),
            b"EL" => Some(EmployerLocation),
            b"F" => Some(CurrentAddress),
            b"FA" => Some(Factory),
            b"FE" => Some(FreightEqualizationPoint),
            b"FF" => Some(ForeignFreightForwarderLocation),
            b"FI" => Some(CodeFI),
            b"FR" => Some(CodeFR),
            b"FS" => Some(FreightStationAccountingCode),
            b"FT" => Some(ForeignTradeZone),
            b"FV" => Some(CodeFV),
            b"G" => Some(CensusBlockGroup),
            b"GL" => Some(FreightStationGeographicLocation),
            b"H" => Some(HomeAddress),
            b"I" => Some(HomeBaseAddress),
            b"IA" => Some(CodeIA),
            b"IB" => Some(IssueLocation),
            b"IM" => Some(CodeIM),
            b"IP" => Some(Postal),
            b"IS" => Some(InStore),
            b"IT" => Some(IntermediateFobPoint),
            b"J" => Some(CensusTract),
            b"K" => Some(CensusScheduleK),
            b"KE" => Some(PortOfEmbarkation),
            b"KL" => Some(PortOfLoading),
            b"KP" => Some(GovernmentFurnishedPropertyFobPoint),
            b"L" => Some(LocalAddress),
            b"LO" => Some(LocalOffice),
            b"M" => Some(MailingAddress),
            b"MI" => Some(Mill),
            b"MO" => Some(MainCampus),
            b"MS" => Some(CodeMS),
            b"MZ" => Some(MexicanPostalCode),
            b"N" => Some(CodeN),
            b"NS" => Some(CityStateFromPoints),
            b"O" => Some(OfficeAddress),
            b"OA" => Some(CodeOA),
            b"OF" => Some(CodeOF),
            b"OL" => Some(CodeOL),
            b"OP" => Some(OtherUnlistedAcceptancePoint),
            b"OR" => Some(CodeOR),
            b"OV" => Some(CodeOV),
            b"P" => Some(PermanentAddress),
            b"PA" => Some(PortOfArrival),
            b"PB" => Some(PortOfDischarge),
            b"PC" => Some(PolicyMailingAddress),
            b"PD" => Some(PlaceOfDelivery),
            b"PE" => Some(PortOfEntry),
            b"PF" => Some(ParentsAddress),
            b"PG" => Some(Primary),
            b"PH" => Some(PriorBusiness),
            b"PL" => Some(Plant),
            b"PM" => Some(CodePM),
            b"PO" => Some(PrincipalServicingOffice),
            b"PP" => Some(PoolPoint),
            b"PQ" => Some(CodePQ),
            b"PR" => Some(CodePR),
            b"PS" => Some(CodePS),
            b"PT" => Some(CodePT),
            b"PU" => Some(CodePU),
            b"PV" => Some(CodePV),
            b"PZ" => Some(CodePZ),
            b"Q" => Some(Birthplace),
            b"R" => Some(CodeR),
            b"RA" => Some(RateAreaCode),
            b"RC" => Some(InRailCar),
            b"RE" => Some(RegionalEducationServiceAgency),
            b"RG" => Some(RegionCode),
            b"RJ" => Some(Region),
            b"RL" => Some(Rural),
            b"RS" => Some(StandardCarrierAlphaCode),
            b"RT" => Some(RouteAdministrativeMessageTo),
            b"SA" => Some(Secondary),
            b"SB" => Some(Suburban),
            b"SC" => Some(CityStateAndPointsWithin),
            b"SD" => Some(SchoolDistrict),
            b"SE" => Some(Summer),
            b"SG" => Some(Storage),
            b"SH" => Some(SchoolCampusCode),
            b"SL" => Some(USSplc),
            b"SN" => Some(StoreNumber),
            b"SP" => Some(StateProvince),
            b"SS" => Some(School),
            b"ST" => Some(InStorageTank),
            b"SW" => Some(SwitchingDistrict),
            b"TA" => Some(Tank),
            b"TC" => Some(TranscontinentalFreightBureau),
            b"TI" => Some(TribalLand),
            b"TL" => Some(TerminalCargoLocation),
            b"TM" => Some(Terminal),
            b"TN" => Some(Township),
            b"TP" => Some(Temporary),
            b"TR" => Some(RailTerritory),
            b"TX" => Some(TaxingDistrict),
            b"UN" => Some(CodeUN),
            b"UR" => Some(Urban),
            b"UT" => Some(BusinessUnit),
            b"VA" => Some(Vacation),
            b"VI" => Some(Village),
            b"VS" => Some(VesselStowageLocation),
            b"W" => Some(WorldwideGeographicLocationCode),
            b"WF" => Some(Wharf),
            b"WH" => Some(Warehouse),
            b"WI" => Some(Winter),
            b"X1" => Some(NationalCenterForEducationStatisticsLocaleCode),
            b"ZN" => Some(ZoneCode),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use LocationQualifier::*;
        match self {
            NearestCrossStreet => "Nearest Cross Street",
            SecondaryCrossStreet => "Secondary Cross Street",
            Range => "Range",
            Section => "Section",
            QuarterSection => "Quarter Section",
            MarkerIdentifierLocation => "Marker Identifier Location",
            Route => "Route",
            RouteSubdivision => "Route Subdivision",
            GridLocation => "Grid Location",
            Page => "Page",
            MarkerType => "Marker Type",
            LatitudeLongitudeSource => "Latitude-Longitude Source",
            MapSource => "Map Source",
            MapReference => "Map Reference",
            GridSource => "Grid Source",
            Aliquot => "Aliquot",
            Block => "Block",
            District => "District",
            DrainholeNumber => "Drainhole Number",
            CityBlock => "City Block",
            FootageCallDirection => "Footage Call Direction",
            LocationDirection => "Location Direction",
            OuterContinentalLeaseLocation => "Outer Continental Lease Location",
            Lot => "Lot",
            MapQuadrangle => "Map Quadrangle",
            PrincipalMeridian => "Principal Meridian",
            OuterContinentalShelfArea => "Outer Continental Shelf Area",
            OuterContinentalShelfBlock => "Outer Continental Shelf Block",
            OfficialProtractionDiagram => "Official Protraction Diagram",
            QuarterQuarterQuarterSection => "Quarter Quarter Quarter Section",
            QuarterQuarterSection => "Quarter Quarter Section",
            SectionType => "Section Type",
            Abstract => "Abstract",
            Labor => "Labor",
            League => "League",
            Survey => "Survey",
            Tier => "Tier",
            Tract => "Tract",
            UniversalTransverseMercatorQuadrant => {
                "Universal Transverse Mercator Quadrant"
            }
            CourseDirection => "Course Direction",
            Area => "Area",
            SendersLocationCode => "Sender's Location Code",
            ReceiversLocationCode => "Receiver's Location Code",
            JurisdictionToReceiveCreditForUniformCommercialCodeFiling => {
                "Jurisdiction to Receive Credit for Uniform Commercial Code Filing"
            }
            Office => "Office",
            AnnualStatementsMailingAddress => "Annual Statements Mailing Address",
            CityAndState => "City and State",
            AllPoints => "All Points",
            ArmedServicesLocationDesignation => "Armed Services Location Designation",
            TransmittingUtility => "Transmitting Utility",
            Branch => "Branch",
            CodeBE => "Business Economic Area (BEA) Region Code",
            CodeBL => "Government Bill of Lading Office Code (GBLOC)",
            PlaceOfBusiness => "Place of Business",
            Consignor => "Consignor",
            GeopoliticalNameCode => "Geopolitical Name Code",
            CountryOfOrigin => "Country of Origin",
            ConfirmationMailingAddress => "Confirmation Mailing Address",
            Country => "Country",
            CanadaCustomsOfficeCode => "Canada Customs Office Code",
            CorrespondenceMailingAddress => "Correspondence Mailing Address",
            CongressionalDistrict => "Congressional District",
            City => "City",
            CodeCL => "National Rate Basis (NRB)",
            CodeCM => "Consolidated Metropolitan Statistical Area (CMSA)",
            CountyParishAndState => "County/Parish and State",
            InTankCar => "In Tank Car",
            CanadianSplc => "Canadian SPLC",
            CountyParish => "County/Parish",
            CensusScheduleD => "Census Schedule D",
            DistributionCenterNumber => "Distribution Center Number",
            CodeDE => "Destination (Shipping)",
            DeliveryLocation => "Delivery Location",
            DistrictOffice => "District Office",
            Department => "Department",
            DistrictOfResidence => "District of Residence",
            DomicileTypeCode => "Domicile Type Code",
            UniformCommercialCodeFilingOffice => "Uniform Commercial Code Filing Office",
            EventLocation => "Event Location",
            Borough => "Borough",
            EmployerLocation => "Employer Location",
            CurrentAddress => "Current Address",
            Factory => "Factory",
            FreightEqualizationPoint => "Freight Equalization Point",
            ForeignFreightForwarderLocation => "Foreign Freight Forwarder Location",
            CodeFI => {
                "Federal Information Processing Standards (FIPS) 55 (Named Populated Places)"
            }
            CodeFR => {
                "U.S. Custom's Facilities Information and Resource Management Systems (FIRMS)"
            }
            FreightStationAccountingCode => "Freight Station Accounting Code",
            ForeignTradeZone => "Foreign Trade Zone",
            CodeFV => "Free Alongside Vessel (Free On Board [F.O.B.] Point)",
            CensusBlockGroup => "Census Block Group",
            FreightStationGeographicLocation => "Freight Station Geographic Location",
            HomeAddress => "Home Address",
            HomeBaseAddress => "Home Base Address",
            CodeIA => "International Air Transport Association (IATA) Location Qualifier",
            IssueLocation => "Issue Location",
            CodeIM => "Military Standard Movement Procedures (MILSTAMP)",
            Postal => "Postal",
            InStore => "In Store",
            IntermediateFobPoint => "Intermediate FOB Point",
            CensusTract => "Census Tract",
            CensusScheduleK => "Census Schedule K",
            PortOfEmbarkation => "Port of Embarkation",
            PortOfLoading => "Port of Loading",
            GovernmentFurnishedPropertyFobPoint => {
                "Government Furnished Property FOB Point"
            }
            LocalAddress => "Local Address",
            LocalOffice => "Local Office",
            MailingAddress => "Mailing Address",
            Mill => "Mill",
            MainCampus => "Main Campus",
            CodeMS => "Metropolitan Sampling Area (MSA) Region Code",
            MexicanPostalCode => "Mexican Postal Code",
            CodeN => {
                "Bureau International des Containers et du Transport Intermodal (BIC) facility depot codes"
            }
            CityStateFromPoints => "City/State from Points",
            OfficeAddress => "Office Address",
            CodeOA => "Origin (After Loading on Equipment)",
            CodeOF => "Other Unlisted Free On Board (FOB) Point",
            CodeOL => "Open and Prepay Station List Code(SCAC & Number)",
            OtherUnlistedAcceptancePoint => "Other Unlisted Acceptance Point",
            CodeOR => "Origin (Shipping Point)",
            CodeOV => "On Vessel (Free On Board [FOB] point)",
            PermanentAddress => "Permanent Address",
            PortOfArrival => "Port of Arrival",
            PortOfDischarge => "Port of Discharge",
            PolicyMailingAddress => "Policy Mailing Address",
            PlaceOfDelivery => "Place of Delivery",
            PortOfEntry => "Port of Entry",
            ParentsAddress => "Parents Address",
            Primary => "Primary",
            PriorBusiness => "Prior Business",
            Plant => "Plant",
            CodePM => "Primary Metropolitan Statistical Area (PMSA)",
            PrincipalServicingOffice => "Principal Servicing Office",
            PoolPoint => "Pool Point",
            CodePQ => "3 Digit U.S. ZIP",
            CodePR => "4 Digit U.S. ZIP",
            CodePS => "5 Digit U.S. ZIP",
            CodePT => "3 Digit Canadian Postal Code",
            CodePU => "6 Digit Canadian Postal Code",
            CodePV => "9 DIGIT U.S. ZIP",
            CodePZ => "11 DIGIT U.S. ZIP",
            Birthplace => "Birthplace",
            CodeR => "Ship Message Design Group (SMDG) terminal codes",
            RateAreaCode => "Rate Area Code",
            InRailCar => "In Rail Car",
            RegionalEducationServiceAgency => "Regional Education Service Agency",
            RegionCode => "Region Code",
            Region => "Region",
            Rural => "Rural",
            StandardCarrierAlphaCode => "Standard Carrier Alpha Code",
            RouteAdministrativeMessageTo => "Route Administrative Message To",
            Secondary => "Secondary",
            Suburban => "Suburban",
            CityStateAndPointsWithin => "City/State and Points Within",
            SchoolDistrict => "School District",
            Summer => "Summer",
            Storage => "Storage",
            SchoolCampusCode => "School Campus Code",
            USSplc => "U.S. SPLC",
            StoreNumber => "Store Number",
            StateProvince => "State/Province",
            School => "School",
            InStorageTank => "In Storage Tank",
            SwitchingDistrict => "Switching District",
            Tank => "Tank",
            TranscontinentalFreightBureau => "Transcontinental Freight Bureau",
            TribalLand => "Tribal Land",
            TerminalCargoLocation => "Terminal Cargo Location",
            Terminal => "Terminal",
            Township => "Township",
            Temporary => "Temporary",
            RailTerritory => "Rail Territory",
            TaxingDistrict => "Taxing District",
            CodeUN => "United Nations Location Code (UNLOCODE)",
            Urban => "Urban",
            BusinessUnit => "Business Unit",
            Vacation => "Vacation",
            Village => "Village",
            VesselStowageLocation => "Vessel Stowage Location",
            WorldwideGeographicLocationCode => "Worldwide Geographic Location Code",
            Wharf => "Wharf",
            Warehouse => "Warehouse",
            Winter => "Winter",
            NationalCenterForEducationStatisticsLocaleCode => {
                "National Center for Education Statistics Locale Code"
            }
            ZoneCode => "Zone Code",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<LocationQualifier> {
        {
            use LocationQualifier::*;
            match description {
                "Nearest Cross Street" => Some(NearestCrossStreet),
                "Secondary Cross Street" => Some(SecondaryCrossStreet),
                "Range" => Some(Range),
                "Section" => Some(Section),
                "Quarter Section" => Some(QuarterSection),
                "Marker Identifier Location" => Some(MarkerIdentifierLocation),
                "Route" => Some(Route),
                "Route Subdivision" => Some(RouteSubdivision),
                "Grid Location" => Some(GridLocation),
                "Page" => Some(Page),
                "Marker Type" => Some(MarkerType),
                "Latitude-Longitude Source" => Some(LatitudeLongitudeSource),
                "Map Source" => Some(MapSource),
                "Map Reference" => Some(MapReference),
                "Grid Source" => Some(GridSource),
                "Aliquot" => Some(Aliquot),
                "Block" => Some(Block),
                "District" => Some(District),
                "Drainhole Number" => Some(DrainholeNumber),
                "City Block" => Some(CityBlock),
                "Footage Call Direction" => Some(FootageCallDirection),
                "Location Direction" => Some(LocationDirection),
                "Outer Continental Lease Location" => Some(OuterContinentalLeaseLocation),
                "Lot" => Some(Lot),
                "Map Quadrangle" => Some(MapQuadrangle),
                "Principal Meridian" => Some(PrincipalMeridian),
                "Outer Continental Shelf Area" => Some(OuterContinentalShelfArea),
                "Outer Continental Shelf Block" => Some(OuterContinentalShelfBlock),
                "Official Protraction Diagram" => Some(OfficialProtractionDiagram),
                "Quarter Quarter Quarter Section" => Some(QuarterQuarterQuarterSection),
                "Quarter Quarter Section" => Some(QuarterQuarterSection),
                "Section Type" => Some(SectionType),
                "Abstract" => Some(Abstract),
                "Labor" => Some(Labor),
                "League" => Some(League),
                "Survey" => Some(Survey),
                "Tier" => Some(Tier),
                "Tract" => Some(Tract),
                "Universal Transverse Mercator Quadrant" => {
                    Some(UniversalTransverseMercatorQuadrant)
                }
                "Course Direction" => Some(CourseDirection),
                "Area" => Some(Area),
                "Sender's Location Code" => Some(SendersLocationCode),
                "Receiver's Location Code" => Some(ReceiversLocationCode),
                "Jurisdiction to Receive Credit for Uniform Commercial Code Filing" => {
                    Some(JurisdictionToReceiveCreditForUniformCommercialCodeFiling)
                }
                "Office" => Some(Office),
                "Annual Statements Mailing Address" => {
                    Some(AnnualStatementsMailingAddress)
                }
                "City and State" => Some(CityAndState),
                "All Points" => Some(AllPoints),
                "Armed Services Location Designation" => {
                    Some(ArmedServicesLocationDesignation)
                }
                "Transmitting Utility" => Some(TransmittingUtility),
                "Branch" => Some(Branch),
                "Business Economic Area (BEA) Region Code" => Some(CodeBE),
                "Government Bill of Lading Office Code (GBLOC)" => Some(CodeBL),
                "Place of Business" => Some(PlaceOfBusiness),
                "Consignor" => Some(Consignor),
                "Geopolitical Name Code" => Some(GeopoliticalNameCode),
                "Country of Origin" => Some(CountryOfOrigin),
                "Confirmation Mailing Address" => Some(ConfirmationMailingAddress),
                "Country" => Some(Country),
                "Canada Customs Office Code" => Some(CanadaCustomsOfficeCode),
                "Correspondence Mailing Address" => Some(CorrespondenceMailingAddress),
                "Congressional District" => Some(CongressionalDistrict),
                "City" => Some(City),
                "National Rate Basis (NRB)" => Some(CodeCL),
                "Consolidated Metropolitan Statistical Area (CMSA)" => Some(CodeCM),
                "County/Parish and State" => Some(CountyParishAndState),
                "In Tank Car" => Some(InTankCar),
                "Canadian SPLC" => Some(CanadianSplc),
                "County/Parish" => Some(CountyParish),
                "Census Schedule D" => Some(CensusScheduleD),
                "Distribution Center Number" => Some(DistributionCenterNumber),
                "Destination (Shipping)" => Some(CodeDE),
                "Delivery Location" => Some(DeliveryLocation),
                "District Office" => Some(DistrictOffice),
                "Department" => Some(Department),
                "District of Residence" => Some(DistrictOfResidence),
                "Domicile Type Code" => Some(DomicileTypeCode),
                "Uniform Commercial Code Filing Office" => {
                    Some(UniformCommercialCodeFilingOffice)
                }
                "Event Location" => Some(EventLocation),
                "Borough" => Some(Borough),
                "Employer Location" => Some(EmployerLocation),
                "Current Address" => Some(CurrentAddress),
                "Factory" => Some(Factory),
                "Freight Equalization Point" => Some(FreightEqualizationPoint),
                "Foreign Freight Forwarder Location" => {
                    Some(ForeignFreightForwarderLocation)
                }
                "Federal Information Processing Standards (FIPS) 55 (Named Populated Places)" => {
                    Some(CodeFI)
                }
                "U.S. Custom's Facilities Information and Resource Management Systems (FIRMS)" => {
                    Some(CodeFR)
                }
                "Freight Station Accounting Code" => Some(FreightStationAccountingCode),
                "Foreign Trade Zone" => Some(ForeignTradeZone),
                "Free Alongside Vessel (Free On Board [F.O.B.] Point)" => Some(CodeFV),
                "Census Block Group" => Some(CensusBlockGroup),
                "Freight Station Geographic Location" => {
                    Some(FreightStationGeographicLocation)
                }
                "Home Address" => Some(HomeAddress),
                "Home Base Address" => Some(HomeBaseAddress),
                "International Air Transport Association (IATA) Location Qualifier" => {
                    Some(CodeIA)
                }
                "Issue Location" => Some(IssueLocation),
                "Military Standard Movement Procedures (MILSTAMP)" => Some(CodeIM),
                "Postal" => Some(Postal),
                "In Store" => Some(InStore),
                "Intermediate FOB Point" => Some(IntermediateFobPoint),
                "Census Tract" => Some(CensusTract),
                "Census Schedule K" => Some(CensusScheduleK),
                "Port of Embarkation" => Some(PortOfEmbarkation),
                "Port of Loading" => Some(PortOfLoading),
                "Government Furnished Property FOB Point" => {
                    Some(GovernmentFurnishedPropertyFobPoint)
                }
                "Local Address" => Some(LocalAddress),
                "Local Office" => Some(LocalOffice),
                "Mailing Address" => Some(MailingAddress),
                "Mill" => Some(Mill),
                "Main Campus" => Some(MainCampus),
                "Metropolitan Sampling Area (MSA) Region Code" => Some(CodeMS),
                "Mexican Postal Code" => Some(MexicanPostalCode),
                "Bureau International des Containers et du Transport Intermodal (BIC) facility depot codes" => {
                    Some(CodeN)
                }
                "City/State from Points" => Some(CityStateFromPoints),
                "Office Address" => Some(OfficeAddress),
                "Origin (After Loading on Equipment)" => Some(CodeOA),
                "Other Unlisted Free On Board (FOB) Point" => Some(CodeOF),
                "Open and Prepay Station List Code(SCAC & Number)" => Some(CodeOL),
                "Other Unlisted Acceptance Point" => Some(OtherUnlistedAcceptancePoint),
                "Origin (Shipping Point)" => Some(CodeOR),
                "On Vessel (Free On Board [FOB] point)" => Some(CodeOV),
                "Permanent Address" => Some(PermanentAddress),
                "Port of Arrival" => Some(PortOfArrival),
                "Port of Discharge" => Some(PortOfDischarge),
                "Policy Mailing Address" => Some(PolicyMailingAddress),
                "Place of Delivery" => Some(PlaceOfDelivery),
                "Port of Entry" => Some(PortOfEntry),
                "Parents Address" => Some(ParentsAddress),
                "Primary" => Some(Primary),
                "Prior Business" => Some(PriorBusiness),
                "Plant" => Some(Plant),
                "Primary Metropolitan Statistical Area (PMSA)" => Some(CodePM),
                "Principal Servicing Office" => Some(PrincipalServicingOffice),
                "Pool Point" => Some(PoolPoint),
                "3 Digit U.S. ZIP" => Some(CodePQ),
                "4 Digit U.S. ZIP" => Some(CodePR),
                "5 Digit U.S. ZIP" => Some(CodePS),
                "3 Digit Canadian Postal Code" => Some(CodePT),
                "6 Digit Canadian Postal Code" => Some(CodePU),
                "9 DIGIT U.S. ZIP" => Some(CodePV),
                "11 DIGIT U.S. ZIP" => Some(CodePZ),
                "Birthplace" => Some(Birthplace),
                "Ship Message Design Group (SMDG) terminal codes" => Some(CodeR),
                "Rate Area Code" => Some(RateAreaCode),
                "In Rail Car" => Some(InRailCar),
                "Regional Education Service Agency" => {
                    Some(RegionalEducationServiceAgency)
                }
                "Region Code" => Some(RegionCode),
                "Region" => Some(Region),
                "Rural" => Some(Rural),
                "Standard Carrier Alpha Code" => Some(StandardCarrierAlphaCode),
                "Route Administrative Message To" => Some(RouteAdministrativeMessageTo),
                "Secondary" => Some(Secondary),
                "Suburban" => Some(Suburban),
                "City/State and Points Within" => Some(CityStateAndPointsWithin),
                "School District" => Some(SchoolDistrict),
                "Summer" => Some(Summer),
                "Storage" => Some(Storage),
                "School Campus Code" => Some(SchoolCampusCode),
                "U.S. SPLC" => Some(USSplc),
                "Store Number" => Some(StoreNumber),
                "State/Province" => Some(StateProvince),
                "School" => Some(School),
                "In Storage Tank" => Some(InStorageTank),
                "Switching District" => Some(SwitchingDistrict),
                "Tank" => Some(Tank),
                "Transcontinental Freight Bureau" => Some(TranscontinentalFreightBureau),
                "Tribal Land" => Some(TribalLand),
                "Terminal Cargo Location" => Some(TerminalCargoLocation),
                "Terminal" => Some(Terminal),
                "Township" => Some(Township),
                "Temporary" => Some(Temporary),
                "Rail Territory" => Some(RailTerritory),
                "Taxing District" => Some(TaxingDistrict),
                "United Nations Location Code (UNLOCODE)" => Some(CodeUN),
                "Urban" => Some(Urban),
                "Business Unit" => Some(BusinessUnit),
                "Vacation" => Some(Vacation),
                "Village" => Some(Village),
                "Vessel Stowage Location" => Some(VesselStowageLocation),
                "Worldwide Geographic Location Code" => {
                    Some(WorldwideGeographicLocationCode)
                }
                "Wharf" => Some(Wharf),
                "Warehouse" => Some(Warehouse),
                "Winter" => Some(Winter),
                "National Center for Education Statistics Locale Code" => {
                    Some(NationalCenterForEducationStatisticsLocaleCode)
                }
                "Zone Code" => Some(ZoneCode),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for LocationQualifier {
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
    type Value = LocationQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Location Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        LocationQualifier::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Location Qualifier: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        LocationQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Location Qualifier: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for LocationQualifier {
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