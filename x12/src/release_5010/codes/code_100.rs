use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**100

See docs at <https://www.stedi.com/edi/x12/element/100>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurrencyCode {
    ///AED - UAE Dirham
    UaeDirham,
    ///AFN - Afghani
    Afghani,
    ///ALL - Lek
    Lek,
    ///AMD - Armenian Dram
    ArmenianDram,
    ///ANG - Netherlands Antillean Guilder
    NetherlandsAntilleanGuilder,
    ///AOA - Kwanza
    Kwanza,
    ///ARS - Argentine Peso
    ArgentinePeso,
    ///AUD - Australian Dollar
    AustralianDollar,
    ///AWG - Aruban Florin
    ArubanFlorin,
    ///AZN - Azerbaijan Manat
    AzerbaijanManat,
    ///BAM - Convertible Mark
    ConvertibleMark,
    ///BBD - Barbados Dollar
    BarbadosDollar,
    ///BDT - Taka
    Taka,
    ///BGN - Bulgarian Lev
    BulgarianLev,
    ///BHD - Bahraini Dinar
    BahrainiDinar,
    ///BIF - Burundi Franc
    BurundiFranc,
    ///BMD - Bermudian Dollar
    BermudianDollar,
    ///BND - Brunei Dollar
    BruneiDollar,
    ///BOB - Boliviano
    Boliviano,
    ///BOV - Mvdol
    Mvdol,
    ///BRL - Brazilian Real
    BrazilianReal,
    ///BSD - Bahamian Dollar
    BahamianDollar,
    ///BTN - Ngultrum
    Ngultrum,
    ///BWP - Pula
    Pula,
    ///BYN - Belarusian Ruble
    BelarusianRuble,
    ///BZD - Belize Dollar
    BelizeDollar,
    ///CAD - Canadian Dollar
    CanadianDollar,
    ///CDF - Congolese Franc
    CongoleseFranc,
    ///CHE - WIR Euro
    WirEuro,
    ///CHF - Swiss Franc
    SwissFranc,
    ///CHW - WIR Franc
    WirFranc,
    ///CLF - Unidad de Fomento
    UnidadDeFomento,
    ///CLP - Chilean Peso
    ChileanPeso,
    ///CNY - Yuan Renminbi
    YuanRenminbi,
    ///COP - Colombian Peso
    ColombianPeso,
    ///COU - Unidad de Valor Real
    UnidadDeValorReal,
    ///CRC - Costa Rican Colon
    CostaRicanColon,
    ///CUC - Peso Convertible
    PesoConvertible,
    ///CUP - Cuban Peso
    CubanPeso,
    ///CVE - Cabo Verde Escudo
    CaboVerdeEscudo,
    ///CZK - Czech Koruna
    CzechKoruna,
    ///DJF - Djibouti Franc
    DjiboutiFranc,
    ///DKK - Danish Krone
    DanishKrone,
    ///DOP - Dominican Peso
    DominicanPeso,
    ///DZD - Algerian Dinar
    AlgerianDinar,
    ///EGP - Egyptian Pound
    EgyptianPound,
    ///ERN - Nakfa
    Nakfa,
    ///ETB - Ethiopian Birr
    EthiopianBirr,
    ///EUR - Euro
    Euro,
    ///FJD - Fiji Dollar
    FijiDollar,
    ///FKP - Falkland Islands Pound
    FalklandIslandsPound,
    ///GBP - Pound Sterling
    PoundSterling,
    ///GEL - Lari
    Lari,
    ///GHS - Ghana Cedi
    GhanaCedi,
    ///GIP - Gibraltar Pound
    GibraltarPound,
    ///GMD - Dalasi
    Dalasi,
    ///GNF - Guinean Franc
    GuineanFranc,
    ///GTQ - Quetzal
    Quetzal,
    ///GYD - Guyana Dollar
    GuyanaDollar,
    ///HKD - Hong Kong Dollar
    HongKongDollar,
    ///HNL - Lempira
    Lempira,
    ///HRK - Kuna
    Kuna,
    ///HTG - Gourde
    Gourde,
    ///HUF - Forint
    Forint,
    ///IDR - Rupiah
    Rupiah,
    ///ILS - New Israeli Sheqel
    NewIsraeliSheqel,
    ///INR - Indian Rupee
    IndianRupee,
    ///IQD - Iraqi Dinar
    IraqiDinar,
    ///IRR - Iranian Rial
    IranianRial,
    ///ISK - Iceland Krona
    IcelandKrona,
    ///JMD - Jamaican Dollar
    JamaicanDollar,
    ///JOD - Jordanian Dinar
    JordanianDinar,
    ///JPY - Yen
    Yen,
    ///KES - Kenyan Shilling
    KenyanShilling,
    ///KGS - Som
    Som,
    ///KHR - Riel
    Riel,
    ///KMF - Comorian Franc
    ComorianFranc,
    ///KPW - North Korean Won
    NorthKoreanWon,
    ///KRW - Won
    Won,
    ///KWD - Kuwaiti Dinar
    KuwaitiDinar,
    ///KYD - Cayman Islands Dollar
    CaymanIslandsDollar,
    ///KZT - Tenge
    Tenge,
    ///LAK - Lao Kip
    LaoKip,
    ///LBP - Lebanese Pound
    LebanesePound,
    ///LKR - Sri Lanka Rupee
    SriLankaRupee,
    ///LRD - Liberian Dollar
    LiberianDollar,
    ///LSL - Loti
    Loti,
    ///LYD - Libyan Dinar
    LibyanDinar,
    ///MAD - Moroccan Dirham
    MoroccanDirham,
    ///MDL - Moldovan Leu
    MoldovanLeu,
    ///MGA - Malagasy Ariary
    MalagasyAriary,
    ///MKD - Denar
    Denar,
    ///MMK - Kyat
    Kyat,
    ///MNT - Tugrik
    Tugrik,
    ///MOP - Pataca
    Pataca,
    ///MRU - Ouguiya
    Ouguiya,
    ///MUR - Mauritius Rupee
    MauritiusRupee,
    ///MVR - Rufiyaa
    Rufiyaa,
    ///MWK - Malawi Kwacha
    MalawiKwacha,
    ///MXN - Mexican Peso
    MexicanPeso,
    ///MXV - Mexican Unidad de Inversion (UDI)
    CodeMXV,
    ///MYR - Malaysian Ringgit
    MalaysianRinggit,
    ///MZN - Mozambique Metical
    MozambiqueMetical,
    ///NAD - Namibia Dollar
    NamibiaDollar,
    ///NGN - Naira
    Naira,
    ///NIO - Cordoba Oro
    CordobaOro,
    ///NOK - Norwegian Krone
    NorwegianKrone,
    ///NPR - Nepalese Rupee
    NepaleseRupee,
    ///NZD - New Zealand Dollar
    NewZealandDollar,
    ///OMR - Rial Omani
    RialOmani,
    ///PAB - Balboa
    Balboa,
    ///PEN - Sol
    Sol,
    ///PGK - Kina
    Kina,
    ///PHP - Philippine Peso
    PhilippinePeso,
    ///PKR - Pakistan Rupee
    PakistanRupee,
    ///PLN - Zloty
    Zloty,
    ///PYG - Guarani
    Guarani,
    ///QAR - Qatari Rial
    QatariRial,
    ///RON - Romanian Leu
    RomanianLeu,
    ///RSD - Serbian Dinar
    SerbianDinar,
    ///RUB - Russian Ruble
    RussianRuble,
    ///RWF - Rwanda Franc
    RwandaFranc,
    ///SAR - Saudi Riyal
    SaudiRiyal,
    ///SBD - Solomon Islands Dollar
    SolomonIslandsDollar,
    ///SCR - Seychelles Rupee
    SeychellesRupee,
    ///SDG - Sudanese Pound
    SudanesePound,
    ///SEK - Swedish Krona
    SwedishKrona,
    ///SGD - Singapore Dollar
    SingaporeDollar,
    ///SHP - Saint Helena Pound
    SaintHelenaPound,
    ///SLE, SLL - Leone
    Leone,
    ///SOS - Somali Shilling
    SomaliShilling,
    ///SRD - Surinam Dollar
    SurinamDollar,
    ///SSP - South Sudanese Pound
    SouthSudanesePound,
    ///STN - Dobra
    Dobra,
    ///SVC - El Salvador Colon
    ElSalvadorColon,
    ///SYP - Syrian Pound
    SyrianPound,
    ///SZL - Lilangeni
    Lilangeni,
    ///THB - Baht
    Baht,
    ///TJS - Somoni
    Somoni,
    ///TMT - Turkmenistan New Manat
    TurkmenistanNewManat,
    ///TND - Tunisian Dinar
    TunisianDinar,
    ///TOP - Pa’anga
    Paanga,
    ///TRY - Turkish Lira
    TurkishLira,
    ///TTD - Trinidad and Tobago Dollar
    TrinidadAndTobagoDollar,
    ///TWD - New Taiwan Dollar
    NewTaiwanDollar,
    ///TZS - Tanzanian Shilling
    TanzanianShilling,
    ///UAH - Hryvnia
    Hryvnia,
    ///UGX - Uganda Shilling
    UgandaShilling,
    ///USD - US Dollar
    UsDollar,
    ///USN - US Dollar (Next day)
    CodeUSN,
    ///UYI - Uruguay Peso en Unidades Indexadas (UI)
    CodeUYI,
    ///UYU - Peso Uruguayo
    PesoUruguayo,
    ///UYW - Unidad Previsional
    UnidadPrevisional,
    ///UZS - Uzbekistan Sum
    UzbekistanSum,
    ///VED, VES - Bolívar Soberano
    BolívarSoberano,
    ///VND - Dong
    Dong,
    ///VUV - Vatu
    Vatu,
    ///WST - Tala
    Tala,
    ///XAF - CFA Franc BEAC
    CfaFrancBeac,
    ///XAG - Silver
    Silver,
    ///XAU - Gold
    Gold,
    ///XBA - Bond Markets Unit European Composite Unit (EURCO)
    CodeXBA,
    ///XBB - Bond Markets Unit European Monetary Unit (E.M.U.-6)
    CodeXBB,
    ///XBC - Bond Markets Unit European Unit of Account 9 (E.U.A.-9)
    CodeXBC,
    ///XBD - Bond Markets Unit European Unit of Account 17 (E.U.A.-17)
    CodeXBD,
    ///XCD - East Caribbean Dollar
    EastCaribbeanDollar,
    ///XDR - SDR (Special Drawing Right)
    CodeXDR,
    ///XOF - CFA Franc BCEAO
    CfaFrancBceao,
    ///XPD - Palladium
    Palladium,
    ///XPF - CFP Franc
    CfpFranc,
    ///XPT - Platinum
    Platinum,
    ///XSU - Sucre
    Sucre,
    ///XTS - Codes specifically reserved for testing purposes
    CodesSpecificallyReservedForTestingPurposes,
    ///XUA - ADB Unit of Account
    AdbUnitOfAccount,
    ///XXX - The codes assigned for transactions where no currency is involved
    TheCodesAssignedForTransactionsWhereNoCurrencyIsInvolved,
    ///YER - Yemeni Rial
    YemeniRial,
    ///ZAR - Rand
    Rand,
    ///ZMW - Zambian Kwacha
    ZambianKwacha,
    ///ZWL - Zimbabwe Dollar
    ZimbabweDollar,
}
impl CurrencyCode {
    pub fn code(&self) -> &str {
        {
            use CurrencyCode::*;
            match self {
                UaeDirham => "AED",
                Afghani => "AFN",
                Lek => "ALL",
                ArmenianDram => "AMD",
                NetherlandsAntilleanGuilder => "ANG",
                Kwanza => "AOA",
                ArgentinePeso => "ARS",
                AustralianDollar => "AUD",
                ArubanFlorin => "AWG",
                AzerbaijanManat => "AZN",
                ConvertibleMark => "BAM",
                BarbadosDollar => "BBD",
                Taka => "BDT",
                BulgarianLev => "BGN",
                BahrainiDinar => "BHD",
                BurundiFranc => "BIF",
                BermudianDollar => "BMD",
                BruneiDollar => "BND",
                Boliviano => "BOB",
                Mvdol => "BOV",
                BrazilianReal => "BRL",
                BahamianDollar => "BSD",
                Ngultrum => "BTN",
                Pula => "BWP",
                BelarusianRuble => "BYN",
                BelizeDollar => "BZD",
                CanadianDollar => "CAD",
                CongoleseFranc => "CDF",
                WirEuro => "CHE",
                SwissFranc => "CHF",
                WirFranc => "CHW",
                UnidadDeFomento => "CLF",
                ChileanPeso => "CLP",
                YuanRenminbi => "CNY",
                ColombianPeso => "COP",
                UnidadDeValorReal => "COU",
                CostaRicanColon => "CRC",
                PesoConvertible => "CUC",
                CubanPeso => "CUP",
                CaboVerdeEscudo => "CVE",
                CzechKoruna => "CZK",
                DjiboutiFranc => "DJF",
                DanishKrone => "DKK",
                DominicanPeso => "DOP",
                AlgerianDinar => "DZD",
                EgyptianPound => "EGP",
                Nakfa => "ERN",
                EthiopianBirr => "ETB",
                Euro => "EUR",
                FijiDollar => "FJD",
                FalklandIslandsPound => "FKP",
                PoundSterling => "GBP",
                Lari => "GEL",
                GhanaCedi => "GHS",
                GibraltarPound => "GIP",
                Dalasi => "GMD",
                GuineanFranc => "GNF",
                Quetzal => "GTQ",
                GuyanaDollar => "GYD",
                HongKongDollar => "HKD",
                Lempira => "HNL",
                Kuna => "HRK",
                Gourde => "HTG",
                Forint => "HUF",
                Rupiah => "IDR",
                NewIsraeliSheqel => "ILS",
                IndianRupee => "INR",
                IraqiDinar => "IQD",
                IranianRial => "IRR",
                IcelandKrona => "ISK",
                JamaicanDollar => "JMD",
                JordanianDinar => "JOD",
                Yen => "JPY",
                KenyanShilling => "KES",
                Som => "KGS",
                Riel => "KHR",
                ComorianFranc => "KMF",
                NorthKoreanWon => "KPW",
                Won => "KRW",
                KuwaitiDinar => "KWD",
                CaymanIslandsDollar => "KYD",
                Tenge => "KZT",
                LaoKip => "LAK",
                LebanesePound => "LBP",
                SriLankaRupee => "LKR",
                LiberianDollar => "LRD",
                Loti => "LSL",
                LibyanDinar => "LYD",
                MoroccanDirham => "MAD",
                MoldovanLeu => "MDL",
                MalagasyAriary => "MGA",
                Denar => "MKD",
                Kyat => "MMK",
                Tugrik => "MNT",
                Pataca => "MOP",
                Ouguiya => "MRU",
                MauritiusRupee => "MUR",
                Rufiyaa => "MVR",
                MalawiKwacha => "MWK",
                MexicanPeso => "MXN",
                CodeMXV => "MXV",
                MalaysianRinggit => "MYR",
                MozambiqueMetical => "MZN",
                NamibiaDollar => "NAD",
                Naira => "NGN",
                CordobaOro => "NIO",
                NorwegianKrone => "NOK",
                NepaleseRupee => "NPR",
                NewZealandDollar => "NZD",
                RialOmani => "OMR",
                Balboa => "PAB",
                Sol => "PEN",
                Kina => "PGK",
                PhilippinePeso => "PHP",
                PakistanRupee => "PKR",
                Zloty => "PLN",
                Guarani => "PYG",
                QatariRial => "QAR",
                RomanianLeu => "RON",
                SerbianDinar => "RSD",
                RussianRuble => "RUB",
                RwandaFranc => "RWF",
                SaudiRiyal => "SAR",
                SolomonIslandsDollar => "SBD",
                SeychellesRupee => "SCR",
                SudanesePound => "SDG",
                SwedishKrona => "SEK",
                SingaporeDollar => "SGD",
                SaintHelenaPound => "SHP",
                Leone => "SLE",
                SomaliShilling => "SOS",
                SurinamDollar => "SRD",
                SouthSudanesePound => "SSP",
                Dobra => "STN",
                ElSalvadorColon => "SVC",
                SyrianPound => "SYP",
                Lilangeni => "SZL",
                Baht => "THB",
                Somoni => "TJS",
                TurkmenistanNewManat => "TMT",
                TunisianDinar => "TND",
                Paanga => "TOP",
                TurkishLira => "TRY",
                TrinidadAndTobagoDollar => "TTD",
                NewTaiwanDollar => "TWD",
                TanzanianShilling => "TZS",
                Hryvnia => "UAH",
                UgandaShilling => "UGX",
                UsDollar => "USD",
                CodeUSN => "USN",
                CodeUYI => "UYI",
                PesoUruguayo => "UYU",
                UnidadPrevisional => "UYW",
                UzbekistanSum => "UZS",
                BolívarSoberano => "VED",
                Dong => "VND",
                Vatu => "VUV",
                Tala => "WST",
                CfaFrancBeac => "XAF",
                Silver => "XAG",
                Gold => "XAU",
                CodeXBA => "XBA",
                CodeXBB => "XBB",
                CodeXBC => "XBC",
                CodeXBD => "XBD",
                EastCaribbeanDollar => "XCD",
                CodeXDR => "XDR",
                CfaFrancBceao => "XOF",
                Palladium => "XPD",
                CfpFranc => "XPF",
                Platinum => "XPT",
                Sucre => "XSU",
                CodesSpecificallyReservedForTestingPurposes => "XTS",
                AdbUnitOfAccount => "XUA",
                TheCodesAssignedForTransactionsWhereNoCurrencyIsInvolved => "XXX",
                YemeniRial => "YER",
                Rand => "ZAR",
                ZambianKwacha => "ZMW",
                ZimbabweDollar => "ZWL",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<CurrencyCode> {
        use CurrencyCode::*;
        match code {
            b"AED" => Some(UaeDirham),
            b"AFN" => Some(Afghani),
            b"ALL" => Some(Lek),
            b"AMD" => Some(ArmenianDram),
            b"ANG" => Some(NetherlandsAntilleanGuilder),
            b"AOA" => Some(Kwanza),
            b"ARS" => Some(ArgentinePeso),
            b"AUD" => Some(AustralianDollar),
            b"AWG" => Some(ArubanFlorin),
            b"AZN" => Some(AzerbaijanManat),
            b"BAM" => Some(ConvertibleMark),
            b"BBD" => Some(BarbadosDollar),
            b"BDT" => Some(Taka),
            b"BGN" => Some(BulgarianLev),
            b"BHD" => Some(BahrainiDinar),
            b"BIF" => Some(BurundiFranc),
            b"BMD" => Some(BermudianDollar),
            b"BND" => Some(BruneiDollar),
            b"BOB" => Some(Boliviano),
            b"BOV" => Some(Mvdol),
            b"BRL" => Some(BrazilianReal),
            b"BSD" => Some(BahamianDollar),
            b"BTN" => Some(Ngultrum),
            b"BWP" => Some(Pula),
            b"BYN" => Some(BelarusianRuble),
            b"BZD" => Some(BelizeDollar),
            b"CAD" => Some(CanadianDollar),
            b"CDF" => Some(CongoleseFranc),
            b"CHE" => Some(WirEuro),
            b"CHF" => Some(SwissFranc),
            b"CHW" => Some(WirFranc),
            b"CLF" => Some(UnidadDeFomento),
            b"CLP" => Some(ChileanPeso),
            b"CNY" => Some(YuanRenminbi),
            b"COP" => Some(ColombianPeso),
            b"COU" => Some(UnidadDeValorReal),
            b"CRC" => Some(CostaRicanColon),
            b"CUC" => Some(PesoConvertible),
            b"CUP" => Some(CubanPeso),
            b"CVE" => Some(CaboVerdeEscudo),
            b"CZK" => Some(CzechKoruna),
            b"DJF" => Some(DjiboutiFranc),
            b"DKK" => Some(DanishKrone),
            b"DOP" => Some(DominicanPeso),
            b"DZD" => Some(AlgerianDinar),
            b"EGP" => Some(EgyptianPound),
            b"ERN" => Some(Nakfa),
            b"ETB" => Some(EthiopianBirr),
            b"EUR" => Some(Euro),
            b"FJD" => Some(FijiDollar),
            b"FKP" => Some(FalklandIslandsPound),
            b"GBP" => Some(PoundSterling),
            b"GEL" => Some(Lari),
            b"GHS" => Some(GhanaCedi),
            b"GIP" => Some(GibraltarPound),
            b"GMD" => Some(Dalasi),
            b"GNF" => Some(GuineanFranc),
            b"GTQ" => Some(Quetzal),
            b"GYD" => Some(GuyanaDollar),
            b"HKD" => Some(HongKongDollar),
            b"HNL" => Some(Lempira),
            b"HRK" => Some(Kuna),
            b"HTG" => Some(Gourde),
            b"HUF" => Some(Forint),
            b"IDR" => Some(Rupiah),
            b"ILS" => Some(NewIsraeliSheqel),
            b"INR" => Some(IndianRupee),
            b"IQD" => Some(IraqiDinar),
            b"IRR" => Some(IranianRial),
            b"ISK" => Some(IcelandKrona),
            b"JMD" => Some(JamaicanDollar),
            b"JOD" => Some(JordanianDinar),
            b"JPY" => Some(Yen),
            b"KES" => Some(KenyanShilling),
            b"KGS" => Some(Som),
            b"KHR" => Some(Riel),
            b"KMF" => Some(ComorianFranc),
            b"KPW" => Some(NorthKoreanWon),
            b"KRW" => Some(Won),
            b"KWD" => Some(KuwaitiDinar),
            b"KYD" => Some(CaymanIslandsDollar),
            b"KZT" => Some(Tenge),
            b"LAK" => Some(LaoKip),
            b"LBP" => Some(LebanesePound),
            b"LKR" => Some(SriLankaRupee),
            b"LRD" => Some(LiberianDollar),
            b"LSL" => Some(Loti),
            b"LYD" => Some(LibyanDinar),
            b"MAD" => Some(MoroccanDirham),
            b"MDL" => Some(MoldovanLeu),
            b"MGA" => Some(MalagasyAriary),
            b"MKD" => Some(Denar),
            b"MMK" => Some(Kyat),
            b"MNT" => Some(Tugrik),
            b"MOP" => Some(Pataca),
            b"MRU" => Some(Ouguiya),
            b"MUR" => Some(MauritiusRupee),
            b"MVR" => Some(Rufiyaa),
            b"MWK" => Some(MalawiKwacha),
            b"MXN" => Some(MexicanPeso),
            b"MXV" => Some(CodeMXV),
            b"MYR" => Some(MalaysianRinggit),
            b"MZN" => Some(MozambiqueMetical),
            b"NAD" => Some(NamibiaDollar),
            b"NGN" => Some(Naira),
            b"NIO" => Some(CordobaOro),
            b"NOK" => Some(NorwegianKrone),
            b"NPR" => Some(NepaleseRupee),
            b"NZD" => Some(NewZealandDollar),
            b"OMR" => Some(RialOmani),
            b"PAB" => Some(Balboa),
            b"PEN" => Some(Sol),
            b"PGK" => Some(Kina),
            b"PHP" => Some(PhilippinePeso),
            b"PKR" => Some(PakistanRupee),
            b"PLN" => Some(Zloty),
            b"PYG" => Some(Guarani),
            b"QAR" => Some(QatariRial),
            b"RON" => Some(RomanianLeu),
            b"RSD" => Some(SerbianDinar),
            b"RUB" => Some(RussianRuble),
            b"RWF" => Some(RwandaFranc),
            b"SAR" => Some(SaudiRiyal),
            b"SBD" => Some(SolomonIslandsDollar),
            b"SCR" => Some(SeychellesRupee),
            b"SDG" => Some(SudanesePound),
            b"SEK" => Some(SwedishKrona),
            b"SGD" => Some(SingaporeDollar),
            b"SHP" => Some(SaintHelenaPound),
            b"SLE" => Some(Leone),
            b"SLL" => Some(Leone),
            b"SOS" => Some(SomaliShilling),
            b"SRD" => Some(SurinamDollar),
            b"SSP" => Some(SouthSudanesePound),
            b"STN" => Some(Dobra),
            b"SVC" => Some(ElSalvadorColon),
            b"SYP" => Some(SyrianPound),
            b"SZL" => Some(Lilangeni),
            b"THB" => Some(Baht),
            b"TJS" => Some(Somoni),
            b"TMT" => Some(TurkmenistanNewManat),
            b"TND" => Some(TunisianDinar),
            b"TOP" => Some(Paanga),
            b"TRY" => Some(TurkishLira),
            b"TTD" => Some(TrinidadAndTobagoDollar),
            b"TWD" => Some(NewTaiwanDollar),
            b"TZS" => Some(TanzanianShilling),
            b"UAH" => Some(Hryvnia),
            b"UGX" => Some(UgandaShilling),
            b"USD" => Some(UsDollar),
            b"USN" => Some(CodeUSN),
            b"UYI" => Some(CodeUYI),
            b"UYU" => Some(PesoUruguayo),
            b"UYW" => Some(UnidadPrevisional),
            b"UZS" => Some(UzbekistanSum),
            b"VED" => Some(BolívarSoberano),
            b"VES" => Some(BolívarSoberano),
            b"VND" => Some(Dong),
            b"VUV" => Some(Vatu),
            b"WST" => Some(Tala),
            b"XAF" => Some(CfaFrancBeac),
            b"XAG" => Some(Silver),
            b"XAU" => Some(Gold),
            b"XBA" => Some(CodeXBA),
            b"XBB" => Some(CodeXBB),
            b"XBC" => Some(CodeXBC),
            b"XBD" => Some(CodeXBD),
            b"XCD" => Some(EastCaribbeanDollar),
            b"XDR" => Some(CodeXDR),
            b"XOF" => Some(CfaFrancBceao),
            b"XPD" => Some(Palladium),
            b"XPF" => Some(CfpFranc),
            b"XPT" => Some(Platinum),
            b"XSU" => Some(Sucre),
            b"XTS" => Some(CodesSpecificallyReservedForTestingPurposes),
            b"XUA" => Some(AdbUnitOfAccount),
            b"XXX" => Some(TheCodesAssignedForTransactionsWhereNoCurrencyIsInvolved),
            b"YER" => Some(YemeniRial),
            b"ZAR" => Some(Rand),
            b"ZMW" => Some(ZambianKwacha),
            b"ZWL" => Some(ZimbabweDollar),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use CurrencyCode::*;
        match self {
            UaeDirham => "UAE Dirham",
            Afghani => "Afghani",
            Lek => "Lek",
            ArmenianDram => "Armenian Dram",
            NetherlandsAntilleanGuilder => "Netherlands Antillean Guilder",
            Kwanza => "Kwanza",
            ArgentinePeso => "Argentine Peso",
            AustralianDollar => "Australian Dollar",
            ArubanFlorin => "Aruban Florin",
            AzerbaijanManat => "Azerbaijan Manat",
            ConvertibleMark => "Convertible Mark",
            BarbadosDollar => "Barbados Dollar",
            Taka => "Taka",
            BulgarianLev => "Bulgarian Lev",
            BahrainiDinar => "Bahraini Dinar",
            BurundiFranc => "Burundi Franc",
            BermudianDollar => "Bermudian Dollar",
            BruneiDollar => "Brunei Dollar",
            Boliviano => "Boliviano",
            Mvdol => "Mvdol",
            BrazilianReal => "Brazilian Real",
            BahamianDollar => "Bahamian Dollar",
            Ngultrum => "Ngultrum",
            Pula => "Pula",
            BelarusianRuble => "Belarusian Ruble",
            BelizeDollar => "Belize Dollar",
            CanadianDollar => "Canadian Dollar",
            CongoleseFranc => "Congolese Franc",
            WirEuro => "WIR Euro",
            SwissFranc => "Swiss Franc",
            WirFranc => "WIR Franc",
            UnidadDeFomento => "Unidad de Fomento",
            ChileanPeso => "Chilean Peso",
            YuanRenminbi => "Yuan Renminbi",
            ColombianPeso => "Colombian Peso",
            UnidadDeValorReal => "Unidad de Valor Real",
            CostaRicanColon => "Costa Rican Colon",
            PesoConvertible => "Peso Convertible",
            CubanPeso => "Cuban Peso",
            CaboVerdeEscudo => "Cabo Verde Escudo",
            CzechKoruna => "Czech Koruna",
            DjiboutiFranc => "Djibouti Franc",
            DanishKrone => "Danish Krone",
            DominicanPeso => "Dominican Peso",
            AlgerianDinar => "Algerian Dinar",
            EgyptianPound => "Egyptian Pound",
            Nakfa => "Nakfa",
            EthiopianBirr => "Ethiopian Birr",
            Euro => "Euro",
            FijiDollar => "Fiji Dollar",
            FalklandIslandsPound => "Falkland Islands Pound",
            PoundSterling => "Pound Sterling",
            Lari => "Lari",
            GhanaCedi => "Ghana Cedi",
            GibraltarPound => "Gibraltar Pound",
            Dalasi => "Dalasi",
            GuineanFranc => "Guinean Franc",
            Quetzal => "Quetzal",
            GuyanaDollar => "Guyana Dollar",
            HongKongDollar => "Hong Kong Dollar",
            Lempira => "Lempira",
            Kuna => "Kuna",
            Gourde => "Gourde",
            Forint => "Forint",
            Rupiah => "Rupiah",
            NewIsraeliSheqel => "New Israeli Sheqel",
            IndianRupee => "Indian Rupee",
            IraqiDinar => "Iraqi Dinar",
            IranianRial => "Iranian Rial",
            IcelandKrona => "Iceland Krona",
            JamaicanDollar => "Jamaican Dollar",
            JordanianDinar => "Jordanian Dinar",
            Yen => "Yen",
            KenyanShilling => "Kenyan Shilling",
            Som => "Som",
            Riel => "Riel",
            ComorianFranc => "Comorian Franc",
            NorthKoreanWon => "North Korean Won",
            Won => "Won",
            KuwaitiDinar => "Kuwaiti Dinar",
            CaymanIslandsDollar => "Cayman Islands Dollar",
            Tenge => "Tenge",
            LaoKip => "Lao Kip",
            LebanesePound => "Lebanese Pound",
            SriLankaRupee => "Sri Lanka Rupee",
            LiberianDollar => "Liberian Dollar",
            Loti => "Loti",
            LibyanDinar => "Libyan Dinar",
            MoroccanDirham => "Moroccan Dirham",
            MoldovanLeu => "Moldovan Leu",
            MalagasyAriary => "Malagasy Ariary",
            Denar => "Denar",
            Kyat => "Kyat",
            Tugrik => "Tugrik",
            Pataca => "Pataca",
            Ouguiya => "Ouguiya",
            MauritiusRupee => "Mauritius Rupee",
            Rufiyaa => "Rufiyaa",
            MalawiKwacha => "Malawi Kwacha",
            MexicanPeso => "Mexican Peso",
            CodeMXV => "Mexican Unidad de Inversion (UDI)",
            MalaysianRinggit => "Malaysian Ringgit",
            MozambiqueMetical => "Mozambique Metical",
            NamibiaDollar => "Namibia Dollar",
            Naira => "Naira",
            CordobaOro => "Cordoba Oro",
            NorwegianKrone => "Norwegian Krone",
            NepaleseRupee => "Nepalese Rupee",
            NewZealandDollar => "New Zealand Dollar",
            RialOmani => "Rial Omani",
            Balboa => "Balboa",
            Sol => "Sol",
            Kina => "Kina",
            PhilippinePeso => "Philippine Peso",
            PakistanRupee => "Pakistan Rupee",
            Zloty => "Zloty",
            Guarani => "Guarani",
            QatariRial => "Qatari Rial",
            RomanianLeu => "Romanian Leu",
            SerbianDinar => "Serbian Dinar",
            RussianRuble => "Russian Ruble",
            RwandaFranc => "Rwanda Franc",
            SaudiRiyal => "Saudi Riyal",
            SolomonIslandsDollar => "Solomon Islands Dollar",
            SeychellesRupee => "Seychelles Rupee",
            SudanesePound => "Sudanese Pound",
            SwedishKrona => "Swedish Krona",
            SingaporeDollar => "Singapore Dollar",
            SaintHelenaPound => "Saint Helena Pound",
            Leone => "Leone",
            SomaliShilling => "Somali Shilling",
            SurinamDollar => "Surinam Dollar",
            SouthSudanesePound => "South Sudanese Pound",
            Dobra => "Dobra",
            ElSalvadorColon => "El Salvador Colon",
            SyrianPound => "Syrian Pound",
            Lilangeni => "Lilangeni",
            Baht => "Baht",
            Somoni => "Somoni",
            TurkmenistanNewManat => "Turkmenistan New Manat",
            TunisianDinar => "Tunisian Dinar",
            Paanga => "Pa’anga",
            TurkishLira => "Turkish Lira",
            TrinidadAndTobagoDollar => "Trinidad and Tobago Dollar",
            NewTaiwanDollar => "New Taiwan Dollar",
            TanzanianShilling => "Tanzanian Shilling",
            Hryvnia => "Hryvnia",
            UgandaShilling => "Uganda Shilling",
            UsDollar => "US Dollar",
            CodeUSN => "US Dollar (Next day)",
            CodeUYI => "Uruguay Peso en Unidades Indexadas (UI)",
            PesoUruguayo => "Peso Uruguayo",
            UnidadPrevisional => "Unidad Previsional",
            UzbekistanSum => "Uzbekistan Sum",
            BolívarSoberano => "Bolívar Soberano",
            Dong => "Dong",
            Vatu => "Vatu",
            Tala => "Tala",
            CfaFrancBeac => "CFA Franc BEAC",
            Silver => "Silver",
            Gold => "Gold",
            CodeXBA => "Bond Markets Unit European Composite Unit (EURCO)",
            CodeXBB => "Bond Markets Unit European Monetary Unit (E.M.U.-6)",
            CodeXBC => "Bond Markets Unit European Unit of Account 9 (E.U.A.-9)",
            CodeXBD => "Bond Markets Unit European Unit of Account 17 (E.U.A.-17)",
            EastCaribbeanDollar => "East Caribbean Dollar",
            CodeXDR => "SDR (Special Drawing Right)",
            CfaFrancBceao => "CFA Franc BCEAO",
            Palladium => "Palladium",
            CfpFranc => "CFP Franc",
            Platinum => "Platinum",
            Sucre => "Sucre",
            CodesSpecificallyReservedForTestingPurposes => {
                "Codes specifically reserved for testing purposes"
            }
            AdbUnitOfAccount => "ADB Unit of Account",
            TheCodesAssignedForTransactionsWhereNoCurrencyIsInvolved => {
                "The codes assigned for transactions where no currency is involved"
            }
            YemeniRial => "Yemeni Rial",
            Rand => "Rand",
            ZambianKwacha => "Zambian Kwacha",
            ZimbabweDollar => "Zimbabwe Dollar",
        }
    }
    fn from_description(description: &str) -> Option<CurrencyCode> {
        {
            use CurrencyCode::*;
            match description {
                "UAE Dirham" => Some(UaeDirham),
                "Afghani" => Some(Afghani),
                "Lek" => Some(Lek),
                "Armenian Dram" => Some(ArmenianDram),
                "Netherlands Antillean Guilder" => Some(NetherlandsAntilleanGuilder),
                "Kwanza" => Some(Kwanza),
                "Argentine Peso" => Some(ArgentinePeso),
                "Australian Dollar" => Some(AustralianDollar),
                "Aruban Florin" => Some(ArubanFlorin),
                "Azerbaijan Manat" => Some(AzerbaijanManat),
                "Convertible Mark" => Some(ConvertibleMark),
                "Barbados Dollar" => Some(BarbadosDollar),
                "Taka" => Some(Taka),
                "Bulgarian Lev" => Some(BulgarianLev),
                "Bahraini Dinar" => Some(BahrainiDinar),
                "Burundi Franc" => Some(BurundiFranc),
                "Bermudian Dollar" => Some(BermudianDollar),
                "Brunei Dollar" => Some(BruneiDollar),
                "Boliviano" => Some(Boliviano),
                "Mvdol" => Some(Mvdol),
                "Brazilian Real" => Some(BrazilianReal),
                "Bahamian Dollar" => Some(BahamianDollar),
                "Ngultrum" => Some(Ngultrum),
                "Pula" => Some(Pula),
                "Belarusian Ruble" => Some(BelarusianRuble),
                "Belize Dollar" => Some(BelizeDollar),
                "Canadian Dollar" => Some(CanadianDollar),
                "Congolese Franc" => Some(CongoleseFranc),
                "WIR Euro" => Some(WirEuro),
                "Swiss Franc" => Some(SwissFranc),
                "WIR Franc" => Some(WirFranc),
                "Unidad de Fomento" => Some(UnidadDeFomento),
                "Chilean Peso" => Some(ChileanPeso),
                "Yuan Renminbi" => Some(YuanRenminbi),
                "Colombian Peso" => Some(ColombianPeso),
                "Unidad de Valor Real" => Some(UnidadDeValorReal),
                "Costa Rican Colon" => Some(CostaRicanColon),
                "Peso Convertible" => Some(PesoConvertible),
                "Cuban Peso" => Some(CubanPeso),
                "Cabo Verde Escudo" => Some(CaboVerdeEscudo),
                "Czech Koruna" => Some(CzechKoruna),
                "Djibouti Franc" => Some(DjiboutiFranc),
                "Danish Krone" => Some(DanishKrone),
                "Dominican Peso" => Some(DominicanPeso),
                "Algerian Dinar" => Some(AlgerianDinar),
                "Egyptian Pound" => Some(EgyptianPound),
                "Nakfa" => Some(Nakfa),
                "Ethiopian Birr" => Some(EthiopianBirr),
                "Euro" => Some(Euro),
                "Fiji Dollar" => Some(FijiDollar),
                "Falkland Islands Pound" => Some(FalklandIslandsPound),
                "Pound Sterling" => Some(PoundSterling),
                "Lari" => Some(Lari),
                "Ghana Cedi" => Some(GhanaCedi),
                "Gibraltar Pound" => Some(GibraltarPound),
                "Dalasi" => Some(Dalasi),
                "Guinean Franc" => Some(GuineanFranc),
                "Quetzal" => Some(Quetzal),
                "Guyana Dollar" => Some(GuyanaDollar),
                "Hong Kong Dollar" => Some(HongKongDollar),
                "Lempira" => Some(Lempira),
                "Kuna" => Some(Kuna),
                "Gourde" => Some(Gourde),
                "Forint" => Some(Forint),
                "Rupiah" => Some(Rupiah),
                "New Israeli Sheqel" => Some(NewIsraeliSheqel),
                "Indian Rupee" => Some(IndianRupee),
                "Iraqi Dinar" => Some(IraqiDinar),
                "Iranian Rial" => Some(IranianRial),
                "Iceland Krona" => Some(IcelandKrona),
                "Jamaican Dollar" => Some(JamaicanDollar),
                "Jordanian Dinar" => Some(JordanianDinar),
                "Yen" => Some(Yen),
                "Kenyan Shilling" => Some(KenyanShilling),
                "Som" => Some(Som),
                "Riel" => Some(Riel),
                "Comorian Franc" => Some(ComorianFranc),
                "North Korean Won" => Some(NorthKoreanWon),
                "Won" => Some(Won),
                "Kuwaiti Dinar" => Some(KuwaitiDinar),
                "Cayman Islands Dollar" => Some(CaymanIslandsDollar),
                "Tenge" => Some(Tenge),
                "Lao Kip" => Some(LaoKip),
                "Lebanese Pound" => Some(LebanesePound),
                "Sri Lanka Rupee" => Some(SriLankaRupee),
                "Liberian Dollar" => Some(LiberianDollar),
                "Loti" => Some(Loti),
                "Libyan Dinar" => Some(LibyanDinar),
                "Moroccan Dirham" => Some(MoroccanDirham),
                "Moldovan Leu" => Some(MoldovanLeu),
                "Malagasy Ariary" => Some(MalagasyAriary),
                "Denar" => Some(Denar),
                "Kyat" => Some(Kyat),
                "Tugrik" => Some(Tugrik),
                "Pataca" => Some(Pataca),
                "Ouguiya" => Some(Ouguiya),
                "Mauritius Rupee" => Some(MauritiusRupee),
                "Rufiyaa" => Some(Rufiyaa),
                "Malawi Kwacha" => Some(MalawiKwacha),
                "Mexican Peso" => Some(MexicanPeso),
                "Mexican Unidad de Inversion (UDI)" => Some(CodeMXV),
                "Malaysian Ringgit" => Some(MalaysianRinggit),
                "Mozambique Metical" => Some(MozambiqueMetical),
                "Namibia Dollar" => Some(NamibiaDollar),
                "Naira" => Some(Naira),
                "Cordoba Oro" => Some(CordobaOro),
                "Norwegian Krone" => Some(NorwegianKrone),
                "Nepalese Rupee" => Some(NepaleseRupee),
                "New Zealand Dollar" => Some(NewZealandDollar),
                "Rial Omani" => Some(RialOmani),
                "Balboa" => Some(Balboa),
                "Sol" => Some(Sol),
                "Kina" => Some(Kina),
                "Philippine Peso" => Some(PhilippinePeso),
                "Pakistan Rupee" => Some(PakistanRupee),
                "Zloty" => Some(Zloty),
                "Guarani" => Some(Guarani),
                "Qatari Rial" => Some(QatariRial),
                "Romanian Leu" => Some(RomanianLeu),
                "Serbian Dinar" => Some(SerbianDinar),
                "Russian Ruble" => Some(RussianRuble),
                "Rwanda Franc" => Some(RwandaFranc),
                "Saudi Riyal" => Some(SaudiRiyal),
                "Solomon Islands Dollar" => Some(SolomonIslandsDollar),
                "Seychelles Rupee" => Some(SeychellesRupee),
                "Sudanese Pound" => Some(SudanesePound),
                "Swedish Krona" => Some(SwedishKrona),
                "Singapore Dollar" => Some(SingaporeDollar),
                "Saint Helena Pound" => Some(SaintHelenaPound),
                "Leone" => Some(Leone),
                "Somali Shilling" => Some(SomaliShilling),
                "Surinam Dollar" => Some(SurinamDollar),
                "South Sudanese Pound" => Some(SouthSudanesePound),
                "Dobra" => Some(Dobra),
                "El Salvador Colon" => Some(ElSalvadorColon),
                "Syrian Pound" => Some(SyrianPound),
                "Lilangeni" => Some(Lilangeni),
                "Baht" => Some(Baht),
                "Somoni" => Some(Somoni),
                "Turkmenistan New Manat" => Some(TurkmenistanNewManat),
                "Tunisian Dinar" => Some(TunisianDinar),
                "Pa’anga" => Some(Paanga),
                "Turkish Lira" => Some(TurkishLira),
                "Trinidad and Tobago Dollar" => Some(TrinidadAndTobagoDollar),
                "New Taiwan Dollar" => Some(NewTaiwanDollar),
                "Tanzanian Shilling" => Some(TanzanianShilling),
                "Hryvnia" => Some(Hryvnia),
                "Uganda Shilling" => Some(UgandaShilling),
                "US Dollar" => Some(UsDollar),
                "US Dollar (Next day)" => Some(CodeUSN),
                "Uruguay Peso en Unidades Indexadas (UI)" => Some(CodeUYI),
                "Peso Uruguayo" => Some(PesoUruguayo),
                "Unidad Previsional" => Some(UnidadPrevisional),
                "Uzbekistan Sum" => Some(UzbekistanSum),
                "Bolívar Soberano" => Some(BolívarSoberano),
                "Dong" => Some(Dong),
                "Vatu" => Some(Vatu),
                "Tala" => Some(Tala),
                "CFA Franc BEAC" => Some(CfaFrancBeac),
                "Silver" => Some(Silver),
                "Gold" => Some(Gold),
                "Bond Markets Unit European Composite Unit (EURCO)" => Some(CodeXBA),
                "Bond Markets Unit European Monetary Unit (E.M.U.-6)" => Some(CodeXBB),
                "Bond Markets Unit European Unit of Account 9 (E.U.A.-9)" => {
                    Some(CodeXBC)
                }
                "Bond Markets Unit European Unit of Account 17 (E.U.A.-17)" => {
                    Some(CodeXBD)
                }
                "East Caribbean Dollar" => Some(EastCaribbeanDollar),
                "SDR (Special Drawing Right)" => Some(CodeXDR),
                "CFA Franc BCEAO" => Some(CfaFrancBceao),
                "Palladium" => Some(Palladium),
                "CFP Franc" => Some(CfpFranc),
                "Platinum" => Some(Platinum),
                "Sucre" => Some(Sucre),
                "Codes specifically reserved for testing purposes" => {
                    Some(CodesSpecificallyReservedForTestingPurposes)
                }
                "ADB Unit of Account" => Some(AdbUnitOfAccount),
                "The codes assigned for transactions where no currency is involved" => {
                    Some(TheCodesAssignedForTransactionsWhereNoCurrencyIsInvolved)
                }
                "Yemeni Rial" => Some(YemeniRial),
                "Rand" => Some(Rand),
                "Zambian Kwacha" => Some(ZambianKwacha),
                "Zimbabwe Dollar" => Some(ZimbabweDollar),
                _ => None,
            }
        }
    }
}
impl Serialize for CurrencyCode {
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
    type Value = CurrencyCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Currency Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CurrencyCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Currency Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CurrencyCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Currency Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for CurrencyCode {
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