use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**687

See docs at <https://www.stedi.com/edi/x12-005010/element/687>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClassOfTradeCode {
    ///AA - Branches
    Branches,
    ///AB - Members
    Members,
    ///AC - Commercial Enterprises
    CommercialEnterprises,
    ///AD - Special Trades
    SpecialTrades,
    ///AE - Financial Institutions
    FinancialInstitutions,
    ///AF - Fast Food
    FastFood,
    ///AG - Agent
    Agent,
    ///AH - Non-Profit
    NonProfit,
    ///AI - Importer
    Importer,
    ///AJ - Schools
    Schools,
    ///AL - Gaming
    Gaming,
    ///AM - Prisons
    Prisons,
    ///AN - Theme Park
    ThemePark,
    ///AO - Multi-Purpose
    MultiPurpose,
    ///AP - Purchaser
    Purchaser,
    ///AQ - Restaurant
    Restaurant,
    ///AR - Vending
    Vending,
    ///AS - Seller
    Seller,
    ///BG - Buying Group
    BuyingGroup,
    ///BR - Broker
    Broker,
    ///CB - Combined
    Combined,
    ///CN - Consolidator (Master Distributor)
    CodeCN,
    ///CO - Contractor
    Contractor,
    ///CR - Chain Store Retail
    ChainStoreRetail,
    ///CX - Confection
    Confection,
    ///CY - Convenience Store
    ConvenienceStore,
    ///DE - Dealer
    Dealer,
    ///DF - Doctors' Offices
    DoctorsOffices,
    ///DI - Distributor
    Distributor,
    ///DR - Drug Store
    DrugStore,
    ///EX - Exporter
    Exporter,
    ///FS - Food Service
    FoodService,
    ///GA - Grocery Accounts
    GroceryAccounts,
    ///GM - General Merchandise
    GeneralMerchandise,
    ///GR - General Retail
    GeneralRetail,
    ///GV - Government
    Government,
    ///HS - Hospitals
    Hospitals,
    ///ID - Industrial
    Industrial,
    ///IN - Institutional
    Institutional,
    ///IR - Independent Retail
    IndependentRetail,
    ///JB - Jobber
    Jobber,
    ///LC - Long-term Care
    LongTermCare,
    ///MC - Managed Care
    ManagedCare,
    ///MF - Manufacturer
    Manufacturer,
    ///ML - Military
    Military,
    ///OE - OEM
    Oem,
    ///OF - Off Premise
    OffPremise,
    ///ON - On Premise
    OnPremise,
    ///PF - Professional
    Professional,
    ///PH - Outpatient Pharmacy
    OutpatientPharmacy,
    ///PT - Pet Supply Store
    PetSupplyStore,
    ///PY - Private Label
    PrivateLabel,
    ///RS - Resale
    Resale,
    ///SA - Services
    Services,
    ///SB - Support
    Support,
    ///SE - Securities
    Securities,
    ///ST - Stationer
    Stationer,
    ///TR - General Trade
    GeneralTrade,
    ///WA - Warehousing
    Warehousing,
    ///WC - Warehouse Clubs
    WarehouseClubs,
    ///WH - Wholesaler
    Wholesaler,
    ///WS - User
    User,
}
impl ClassOfTradeCode {
    pub fn code(&self) -> &str {
        {
            use ClassOfTradeCode::*;
            match self {
                Branches => "AA",
                Members => "AB",
                CommercialEnterprises => "AC",
                SpecialTrades => "AD",
                FinancialInstitutions => "AE",
                FastFood => "AF",
                Agent => "AG",
                NonProfit => "AH",
                Importer => "AI",
                Schools => "AJ",
                Gaming => "AL",
                Prisons => "AM",
                ThemePark => "AN",
                MultiPurpose => "AO",
                Purchaser => "AP",
                Restaurant => "AQ",
                Vending => "AR",
                Seller => "AS",
                BuyingGroup => "BG",
                Broker => "BR",
                Combined => "CB",
                CodeCN => "CN",
                Contractor => "CO",
                ChainStoreRetail => "CR",
                Confection => "CX",
                ConvenienceStore => "CY",
                Dealer => "DE",
                DoctorsOffices => "DF",
                Distributor => "DI",
                DrugStore => "DR",
                Exporter => "EX",
                FoodService => "FS",
                GroceryAccounts => "GA",
                GeneralMerchandise => "GM",
                GeneralRetail => "GR",
                Government => "GV",
                Hospitals => "HS",
                Industrial => "ID",
                Institutional => "IN",
                IndependentRetail => "IR",
                Jobber => "JB",
                LongTermCare => "LC",
                ManagedCare => "MC",
                Manufacturer => "MF",
                Military => "ML",
                Oem => "OE",
                OffPremise => "OF",
                OnPremise => "ON",
                Professional => "PF",
                OutpatientPharmacy => "PH",
                PetSupplyStore => "PT",
                PrivateLabel => "PY",
                Resale => "RS",
                Services => "SA",
                Support => "SB",
                Securities => "SE",
                Stationer => "ST",
                GeneralTrade => "TR",
                Warehousing => "WA",
                WarehouseClubs => "WC",
                Wholesaler => "WH",
                User => "WS",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ClassOfTradeCode> {
        use ClassOfTradeCode::*;
        match code {
            b"AA" => Some(Branches),
            b"AB" => Some(Members),
            b"AC" => Some(CommercialEnterprises),
            b"AD" => Some(SpecialTrades),
            b"AE" => Some(FinancialInstitutions),
            b"AF" => Some(FastFood),
            b"AG" => Some(Agent),
            b"AH" => Some(NonProfit),
            b"AI" => Some(Importer),
            b"AJ" => Some(Schools),
            b"AL" => Some(Gaming),
            b"AM" => Some(Prisons),
            b"AN" => Some(ThemePark),
            b"AO" => Some(MultiPurpose),
            b"AP" => Some(Purchaser),
            b"AQ" => Some(Restaurant),
            b"AR" => Some(Vending),
            b"AS" => Some(Seller),
            b"BG" => Some(BuyingGroup),
            b"BR" => Some(Broker),
            b"CB" => Some(Combined),
            b"CN" => Some(CodeCN),
            b"CO" => Some(Contractor),
            b"CR" => Some(ChainStoreRetail),
            b"CX" => Some(Confection),
            b"CY" => Some(ConvenienceStore),
            b"DE" => Some(Dealer),
            b"DF" => Some(DoctorsOffices),
            b"DI" => Some(Distributor),
            b"DR" => Some(DrugStore),
            b"EX" => Some(Exporter),
            b"FS" => Some(FoodService),
            b"GA" => Some(GroceryAccounts),
            b"GM" => Some(GeneralMerchandise),
            b"GR" => Some(GeneralRetail),
            b"GV" => Some(Government),
            b"HS" => Some(Hospitals),
            b"ID" => Some(Industrial),
            b"IN" => Some(Institutional),
            b"IR" => Some(IndependentRetail),
            b"JB" => Some(Jobber),
            b"LC" => Some(LongTermCare),
            b"MC" => Some(ManagedCare),
            b"MF" => Some(Manufacturer),
            b"ML" => Some(Military),
            b"OE" => Some(Oem),
            b"OF" => Some(OffPremise),
            b"ON" => Some(OnPremise),
            b"PF" => Some(Professional),
            b"PH" => Some(OutpatientPharmacy),
            b"PT" => Some(PetSupplyStore),
            b"PY" => Some(PrivateLabel),
            b"RS" => Some(Resale),
            b"SA" => Some(Services),
            b"SB" => Some(Support),
            b"SE" => Some(Securities),
            b"ST" => Some(Stationer),
            b"TR" => Some(GeneralTrade),
            b"WA" => Some(Warehousing),
            b"WC" => Some(WarehouseClubs),
            b"WH" => Some(Wholesaler),
            b"WS" => Some(User),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ClassOfTradeCode::*;
        match self {
            Branches => "Branches",
            Members => "Members",
            CommercialEnterprises => "Commercial Enterprises",
            SpecialTrades => "Special Trades",
            FinancialInstitutions => "Financial Institutions",
            FastFood => "Fast Food",
            Agent => "Agent",
            NonProfit => "Non-Profit",
            Importer => "Importer",
            Schools => "Schools",
            Gaming => "Gaming",
            Prisons => "Prisons",
            ThemePark => "Theme Park",
            MultiPurpose => "Multi-Purpose",
            Purchaser => "Purchaser",
            Restaurant => "Restaurant",
            Vending => "Vending",
            Seller => "Seller",
            BuyingGroup => "Buying Group",
            Broker => "Broker",
            Combined => "Combined",
            CodeCN => "Consolidator (Master Distributor)",
            Contractor => "Contractor",
            ChainStoreRetail => "Chain Store Retail",
            Confection => "Confection",
            ConvenienceStore => "Convenience Store",
            Dealer => "Dealer",
            DoctorsOffices => "Doctors' Offices",
            Distributor => "Distributor",
            DrugStore => "Drug Store",
            Exporter => "Exporter",
            FoodService => "Food Service",
            GroceryAccounts => "Grocery Accounts",
            GeneralMerchandise => "General Merchandise",
            GeneralRetail => "General Retail",
            Government => "Government",
            Hospitals => "Hospitals",
            Industrial => "Industrial",
            Institutional => "Institutional",
            IndependentRetail => "Independent Retail",
            Jobber => "Jobber",
            LongTermCare => "Long-term Care",
            ManagedCare => "Managed Care",
            Manufacturer => "Manufacturer",
            Military => "Military",
            Oem => "OEM",
            OffPremise => "Off Premise",
            OnPremise => "On Premise",
            Professional => "Professional",
            OutpatientPharmacy => "Outpatient Pharmacy",
            PetSupplyStore => "Pet Supply Store",
            PrivateLabel => "Private Label",
            Resale => "Resale",
            Services => "Services",
            Support => "Support",
            Securities => "Securities",
            Stationer => "Stationer",
            GeneralTrade => "General Trade",
            Warehousing => "Warehousing",
            WarehouseClubs => "Warehouse Clubs",
            Wholesaler => "Wholesaler",
            User => "User",
        }
    }
    fn from_description(description: &str) -> Option<ClassOfTradeCode> {
        {
            use ClassOfTradeCode::*;
            match description {
                "Branches" => Some(Branches),
                "Members" => Some(Members),
                "Commercial Enterprises" => Some(CommercialEnterprises),
                "Special Trades" => Some(SpecialTrades),
                "Financial Institutions" => Some(FinancialInstitutions),
                "Fast Food" => Some(FastFood),
                "Agent" => Some(Agent),
                "Non-Profit" => Some(NonProfit),
                "Importer" => Some(Importer),
                "Schools" => Some(Schools),
                "Gaming" => Some(Gaming),
                "Prisons" => Some(Prisons),
                "Theme Park" => Some(ThemePark),
                "Multi-Purpose" => Some(MultiPurpose),
                "Purchaser" => Some(Purchaser),
                "Restaurant" => Some(Restaurant),
                "Vending" => Some(Vending),
                "Seller" => Some(Seller),
                "Buying Group" => Some(BuyingGroup),
                "Broker" => Some(Broker),
                "Combined" => Some(Combined),
                "Consolidator (Master Distributor)" => Some(CodeCN),
                "Contractor" => Some(Contractor),
                "Chain Store Retail" => Some(ChainStoreRetail),
                "Confection" => Some(Confection),
                "Convenience Store" => Some(ConvenienceStore),
                "Dealer" => Some(Dealer),
                "Doctors' Offices" => Some(DoctorsOffices),
                "Distributor" => Some(Distributor),
                "Drug Store" => Some(DrugStore),
                "Exporter" => Some(Exporter),
                "Food Service" => Some(FoodService),
                "Grocery Accounts" => Some(GroceryAccounts),
                "General Merchandise" => Some(GeneralMerchandise),
                "General Retail" => Some(GeneralRetail),
                "Government" => Some(Government),
                "Hospitals" => Some(Hospitals),
                "Industrial" => Some(Industrial),
                "Institutional" => Some(Institutional),
                "Independent Retail" => Some(IndependentRetail),
                "Jobber" => Some(Jobber),
                "Long-term Care" => Some(LongTermCare),
                "Managed Care" => Some(ManagedCare),
                "Manufacturer" => Some(Manufacturer),
                "Military" => Some(Military),
                "OEM" => Some(Oem),
                "Off Premise" => Some(OffPremise),
                "On Premise" => Some(OnPremise),
                "Professional" => Some(Professional),
                "Outpatient Pharmacy" => Some(OutpatientPharmacy),
                "Pet Supply Store" => Some(PetSupplyStore),
                "Private Label" => Some(PrivateLabel),
                "Resale" => Some(Resale),
                "Services" => Some(Services),
                "Support" => Some(Support),
                "Securities" => Some(Securities),
                "Stationer" => Some(Stationer),
                "General Trade" => Some(GeneralTrade),
                "Warehousing" => Some(Warehousing),
                "Warehouse Clubs" => Some(WarehouseClubs),
                "Wholesaler" => Some(Wholesaler),
                "User" => Some(User),
                _ => None,
            }
        }
    }
}
impl Serialize for ClassOfTradeCode {
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
    type Value = ClassOfTradeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Class of Trade Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ClassOfTradeCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Class of Trade Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ClassOfTradeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Class of Trade Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ClassOfTradeCode {
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