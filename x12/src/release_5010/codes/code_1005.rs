use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1005

See docs at <https://www.stedi.com/edi/x12-005010/element/1005>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HierarchicalStructureCode {
    ///0001 - Shipment, Order, Packaging, Item
    Code0001,
    ///0002 - Shipment, Order, Item, Packaging
    Code0002,
    ///0003 - Shipment, Packaging, Order, Item
    Code0003,
    ///0004 - Shipment, Order, Item
    Code0004,
    ///0005 - Financial Institution; Client or Party; Contract; Component; Account; Item; Coverage, Rider or Supplementary Benefit
    Code0005,
    ///0006 - Financial Institution; Client or Party; Contract; Account; Coverage, Rider or Supplementary Benefit
    Code0006,
    ///0007 - Information Source, Information Receiver, Subscriber, Dependent, Event, Services
    Code0007,
    ///0010 - Information Source, Information Receiver, Provider of Service, Subscriber, Dependent
    Code0010,
    ///0011 - Information Receiver, Information Source, Provider of Service, Subscriber, Dependent
    Code0011,
    ///0012 - Information Source, Provider of Service, Subscriber, Dependent
    Code0012,
    ///0013 - Provider of Service, Information Source, Subscriber, Dependent
    Code0013,
    ///0014 - Provider of Service, Information Source, Information Receiver, Subscriber, Dependent
    Code0014,
    ///0015 - Information Receiver, Provider of Service, Subscriber, Dependent
    Code0015,
    ///0016 - Provider of Service, Subscriber, Dependent
    Code0016,
    ///0017 - Subscriber, Dependent
    Code0017,
    ///0018 - Information Receiver, Subscriber, Dependent
    Code0018,
    ///0019 - Information Source, Subscriber, Dependent
    Code0019,
    ///0020 - Information Source, Information Receiver, Group Coverage Options, Subscriber, Dependent
    Code0020,
    ///0021 - Information Source, Information Receiver, Group Coverage Options
    Code0021,
    ///0022 - Information Source, Information Receiver, Subscriber, Dependent
    Code0022,
    ///0023 - Information Receiver, Information Source, Subscriber, Dependent
    Code0023,
    ///0024 - Information Source, Information Receiver, Provider, Group, Site of Service
    Code0024,
    ///0025 - Information Source, Information Receiver, Group, Provider, Site of Service
    Code0025,
    ///0026 - Information Source, Information Receiver, Site of Service, Provider, Group
    Code0026,
    ///0027 - Information Source, Information Receiver, Provider, Site of Service, Group
    Code0027,
    ///0028 - Information Source, Information Receiver, Group, Site of Service, Provider
    Code0028,
    ///0029 - Information Source, Information Receiver, Site of Service, Group, Provider
    Code0029,
    ///0035 - Address, Shipment, Order
    Code0035,
    ///0036 - Address, Transaction Reference Number, Suffix, Serial Number
    Code0036,
    ///0055 - Supergroup, Group, Subgroup, Member
    Code0055,
    ///0056 - Supergroup, subgroup, member, ancillary facility or department
    Code0056,
    ///0057 - Supergroup, subgroup, member
    Code0057,
    ///0058 - Group, member, ancillary facility or department
    Code0058,
    ///0059 - Group, member
    Code0059,
    ///0060 - Hospital, ancillary facility or department
    Code0060,
    ///0061 - Health Industry Business Communications Council (HIBCC) Health Industry Number (HIN) database, facility record, location record
    Code0061,
    ///0062 - Franchisor, franchisee
    Code0062,
    ///0063 - Franchisee association, franchisee
    Code0063,
    ///0064 - Company, Company
    Code0064,
    ///0065 - Company, Operating Unit
    Code0065,
    ///0066 - Operating Unit, Operating Unit
    Code0066,
    ///0067 - Company, Property
    Code0067,
    ///0068 - Company, Property Property, Property
    Code0068,
    ///0069 - Operating Unit, Property
    Code0069,
    ///0070 - Property, Property
    Code0070,
    ///0071 - Company, Tradename
    Code0071,
    ///0072 - Operating Unit, Tradename
    Code0072,
    ///0073 - Property, Tradename
    Code0073,
    ///0074 - Company, Operating Unit, Operating Unit, Operating Unit
    Code0074,
    ///0075 - Operating Unit, Operating Unit, Operating Unit, Operating Unit
    Code0075,
    ///0076 - Company, Operating Unit, Operating Unit, Property
    Code0076,
    ///0077 - Tradename, Property
    Code0077,
    ///0078 - Information Source, Information Receiver, Subscriber, Dependent, Provider of Service, Services
    Code0078,
    ///0079 - Information Source, Information Receiver, Company/Corporation, Operating Unit
    Code0079,
    ///0080 - Information Source, Employer, Patient
    Code0080,
    ///0081 - Information Source, Patient
    Code0081,
    ///0082 - Information Source, Employer, Subscriber, Dependent
    Code0082,
    ///0083 - Information Source, Information Receiver, Subscriber, Dependent, Referring Provider, Provider of Service, Services
    Code0083,
    ///0084 - Report, Site, Sample, Test
    Code0084,
    ///0085 - Information Source, Information Receiver, Provider of Service, Patient
    Code0085,
    ///0200 - Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Fleet (FL), Jurisdiction (JU)
    Code0200,
    ///0201 - Credential Action (AP), Company/Corporation (35), Transportation Equipment (E)
    Code0201,
    ///0202 - Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Jurisdiction (JU)
    Code0202,
    ///0203 - Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E)
    Code0203,
    ///0204 - Report (RP), Jurisdiction (JU), Company/Corporation (35)
    Code0204,
    ///0205 - Report (RP), Company/Corporation (35), Fleet (FL), Jurisdiction (JU), Transportation Equipment (E)
    Code0205,
    ///0206 - Credential Action (AP), Company/Corporation (35), Jurisdiction (JU), Transportation Equipment (E)
    Code0206,
    ///0207 - Credential Action (AP), Company/Corporation (35), Transportation Equipment (E), Component (F), Measurement (M), Jurisdiction (JU)
    Code0207,
    ///0208 - Credential Action (AP), Company/Corporation (35), Cost Type (CT), Jurisdiction (JU)
    Code0208,
    ///0209 - Credential Action (AP), Company/Corporation (35)
    Code0209,
    ///0210 - Credential Action (AP)
    Code0210,
    ///0211 - Reporting Agency, Claim Administrator, Insurer, Insured, Employer, Claimant, Payment
    Code0211,
    ///0212 - Reporting Agency, Employer, Claimant
    Code0212,
    ///0213 - Reporting Agency, Claim Administrator, Employer, Claimant
    Code0213,
    ///0214 - Information Source, Information Receiver, Response Details
    Code0214,
    ///0400 - Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Fleet (FL), Jurisdiction (JU), Payment Detail (PY)
    Code0400,
    ///0401 - Credential Action (AP), Company/Corporation (35), Transportation Equipment (E), Payment Detail (PY)
    Code0401,
    ///0402 - Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Jurisdiction (JU), Payment Detail (PY)
    Code0402,
    ///0403 - Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Payment Detail (PY)
    Code0403,
    ///0407 - Credential Action (AP), Company/Corporation (35), Transportation Equipment (E), Component (F), Measurement (M), Jurisdiction (JU), Payment Detail (PY)
    Code0407,
    ///ZZZZ - Mutually Defined
    MutuallyDefined,
}
impl HierarchicalStructureCode {
    pub fn code(&self) -> &str {
        {
            use HierarchicalStructureCode::*;
            match self {
                Code0001 => "0001",
                Code0002 => "0002",
                Code0003 => "0003",
                Code0004 => "0004",
                Code0005 => "0005",
                Code0006 => "0006",
                Code0007 => "0007",
                Code0010 => "0010",
                Code0011 => "0011",
                Code0012 => "0012",
                Code0013 => "0013",
                Code0014 => "0014",
                Code0015 => "0015",
                Code0016 => "0016",
                Code0017 => "0017",
                Code0018 => "0018",
                Code0019 => "0019",
                Code0020 => "0020",
                Code0021 => "0021",
                Code0022 => "0022",
                Code0023 => "0023",
                Code0024 => "0024",
                Code0025 => "0025",
                Code0026 => "0026",
                Code0027 => "0027",
                Code0028 => "0028",
                Code0029 => "0029",
                Code0035 => "0035",
                Code0036 => "0036",
                Code0055 => "0055",
                Code0056 => "0056",
                Code0057 => "0057",
                Code0058 => "0058",
                Code0059 => "0059",
                Code0060 => "0060",
                Code0061 => "0061",
                Code0062 => "0062",
                Code0063 => "0063",
                Code0064 => "0064",
                Code0065 => "0065",
                Code0066 => "0066",
                Code0067 => "0067",
                Code0068 => "0068",
                Code0069 => "0069",
                Code0070 => "0070",
                Code0071 => "0071",
                Code0072 => "0072",
                Code0073 => "0073",
                Code0074 => "0074",
                Code0075 => "0075",
                Code0076 => "0076",
                Code0077 => "0077",
                Code0078 => "0078",
                Code0079 => "0079",
                Code0080 => "0080",
                Code0081 => "0081",
                Code0082 => "0082",
                Code0083 => "0083",
                Code0084 => "0084",
                Code0085 => "0085",
                Code0200 => "0200",
                Code0201 => "0201",
                Code0202 => "0202",
                Code0203 => "0203",
                Code0204 => "0204",
                Code0205 => "0205",
                Code0206 => "0206",
                Code0207 => "0207",
                Code0208 => "0208",
                Code0209 => "0209",
                Code0210 => "0210",
                Code0211 => "0211",
                Code0212 => "0212",
                Code0213 => "0213",
                Code0214 => "0214",
                Code0400 => "0400",
                Code0401 => "0401",
                Code0402 => "0402",
                Code0403 => "0403",
                Code0407 => "0407",
                MutuallyDefined => "ZZZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<HierarchicalStructureCode> {
        use HierarchicalStructureCode::*;
        match code {
            b"0001" => Some(Code0001),
            b"0002" => Some(Code0002),
            b"0003" => Some(Code0003),
            b"0004" => Some(Code0004),
            b"0005" => Some(Code0005),
            b"0006" => Some(Code0006),
            b"0007" => Some(Code0007),
            b"0010" => Some(Code0010),
            b"0011" => Some(Code0011),
            b"0012" => Some(Code0012),
            b"0013" => Some(Code0013),
            b"0014" => Some(Code0014),
            b"0015" => Some(Code0015),
            b"0016" => Some(Code0016),
            b"0017" => Some(Code0017),
            b"0018" => Some(Code0018),
            b"0019" => Some(Code0019),
            b"0020" => Some(Code0020),
            b"0021" => Some(Code0021),
            b"0022" => Some(Code0022),
            b"0023" => Some(Code0023),
            b"0024" => Some(Code0024),
            b"0025" => Some(Code0025),
            b"0026" => Some(Code0026),
            b"0027" => Some(Code0027),
            b"0028" => Some(Code0028),
            b"0029" => Some(Code0029),
            b"0035" => Some(Code0035),
            b"0036" => Some(Code0036),
            b"0055" => Some(Code0055),
            b"0056" => Some(Code0056),
            b"0057" => Some(Code0057),
            b"0058" => Some(Code0058),
            b"0059" => Some(Code0059),
            b"0060" => Some(Code0060),
            b"0061" => Some(Code0061),
            b"0062" => Some(Code0062),
            b"0063" => Some(Code0063),
            b"0064" => Some(Code0064),
            b"0065" => Some(Code0065),
            b"0066" => Some(Code0066),
            b"0067" => Some(Code0067),
            b"0068" => Some(Code0068),
            b"0069" => Some(Code0069),
            b"0070" => Some(Code0070),
            b"0071" => Some(Code0071),
            b"0072" => Some(Code0072),
            b"0073" => Some(Code0073),
            b"0074" => Some(Code0074),
            b"0075" => Some(Code0075),
            b"0076" => Some(Code0076),
            b"0077" => Some(Code0077),
            b"0078" => Some(Code0078),
            b"0079" => Some(Code0079),
            b"0080" => Some(Code0080),
            b"0081" => Some(Code0081),
            b"0082" => Some(Code0082),
            b"0083" => Some(Code0083),
            b"0084" => Some(Code0084),
            b"0085" => Some(Code0085),
            b"0200" => Some(Code0200),
            b"0201" => Some(Code0201),
            b"0202" => Some(Code0202),
            b"0203" => Some(Code0203),
            b"0204" => Some(Code0204),
            b"0205" => Some(Code0205),
            b"0206" => Some(Code0206),
            b"0207" => Some(Code0207),
            b"0208" => Some(Code0208),
            b"0209" => Some(Code0209),
            b"0210" => Some(Code0210),
            b"0211" => Some(Code0211),
            b"0212" => Some(Code0212),
            b"0213" => Some(Code0213),
            b"0214" => Some(Code0214),
            b"0400" => Some(Code0400),
            b"0401" => Some(Code0401),
            b"0402" => Some(Code0402),
            b"0403" => Some(Code0403),
            b"0407" => Some(Code0407),
            b"ZZZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use HierarchicalStructureCode::*;
        match self {
            Code0001 => "Shipment, Order, Packaging, Item",
            Code0002 => "Shipment, Order, Item, Packaging",
            Code0003 => "Shipment, Packaging, Order, Item",
            Code0004 => "Shipment, Order, Item",
            Code0005 => {
                "Financial Institution; Client or Party; Contract; Component; Account; Item; Coverage, Rider or Supplementary Benefit"
            }
            Code0006 => {
                "Financial Institution; Client or Party; Contract; Account; Coverage, Rider or Supplementary Benefit"
            }
            Code0007 => {
                "Information Source, Information Receiver, Subscriber, Dependent, Event, Services"
            }
            Code0010 => {
                "Information Source, Information Receiver, Provider of Service, Subscriber, Dependent"
            }
            Code0011 => {
                "Information Receiver, Information Source, Provider of Service, Subscriber, Dependent"
            }
            Code0012 => "Information Source, Provider of Service, Subscriber, Dependent",
            Code0013 => "Provider of Service, Information Source, Subscriber, Dependent",
            Code0014 => {
                "Provider of Service, Information Source, Information Receiver, Subscriber, Dependent"
            }
            Code0015 => {
                "Information Receiver, Provider of Service, Subscriber, Dependent"
            }
            Code0016 => "Provider of Service, Subscriber, Dependent",
            Code0017 => "Subscriber, Dependent",
            Code0018 => "Information Receiver, Subscriber, Dependent",
            Code0019 => "Information Source, Subscriber, Dependent",
            Code0020 => {
                "Information Source, Information Receiver, Group Coverage Options, Subscriber, Dependent"
            }
            Code0021 => {
                "Information Source, Information Receiver, Group Coverage Options"
            }
            Code0022 => "Information Source, Information Receiver, Subscriber, Dependent",
            Code0023 => "Information Receiver, Information Source, Subscriber, Dependent",
            Code0024 => {
                "Information Source, Information Receiver, Provider, Group, Site of Service"
            }
            Code0025 => {
                "Information Source, Information Receiver, Group, Provider, Site of Service"
            }
            Code0026 => {
                "Information Source, Information Receiver, Site of Service, Provider, Group"
            }
            Code0027 => {
                "Information Source, Information Receiver, Provider, Site of Service, Group"
            }
            Code0028 => {
                "Information Source, Information Receiver, Group, Site of Service, Provider"
            }
            Code0029 => {
                "Information Source, Information Receiver, Site of Service, Group, Provider"
            }
            Code0035 => "Address, Shipment, Order",
            Code0036 => "Address, Transaction Reference Number, Suffix, Serial Number",
            Code0055 => "Supergroup, Group, Subgroup, Member",
            Code0056 => "Supergroup, subgroup, member, ancillary facility or department",
            Code0057 => "Supergroup, subgroup, member",
            Code0058 => "Group, member, ancillary facility or department",
            Code0059 => "Group, member",
            Code0060 => "Hospital, ancillary facility or department",
            Code0061 => {
                "Health Industry Business Communications Council (HIBCC) Health Industry Number (HIN) database, facility record, location record"
            }
            Code0062 => "Franchisor, franchisee",
            Code0063 => "Franchisee association, franchisee",
            Code0064 => "Company, Company",
            Code0065 => "Company, Operating Unit",
            Code0066 => "Operating Unit, Operating Unit",
            Code0067 => "Company, Property",
            Code0068 => "Company, Property Property, Property",
            Code0069 => "Operating Unit, Property",
            Code0070 => "Property, Property",
            Code0071 => "Company, Tradename",
            Code0072 => "Operating Unit, Tradename",
            Code0073 => "Property, Tradename",
            Code0074 => "Company, Operating Unit, Operating Unit, Operating Unit",
            Code0075 => "Operating Unit, Operating Unit, Operating Unit, Operating Unit",
            Code0076 => "Company, Operating Unit, Operating Unit, Property",
            Code0077 => "Tradename, Property",
            Code0078 => {
                "Information Source, Information Receiver, Subscriber, Dependent, Provider of Service, Services"
            }
            Code0079 => {
                "Information Source, Information Receiver, Company/Corporation, Operating Unit"
            }
            Code0080 => "Information Source, Employer, Patient",
            Code0081 => "Information Source, Patient",
            Code0082 => "Information Source, Employer, Subscriber, Dependent",
            Code0083 => {
                "Information Source, Information Receiver, Subscriber, Dependent, Referring Provider, Provider of Service, Services"
            }
            Code0084 => "Report, Site, Sample, Test",
            Code0085 => {
                "Information Source, Information Receiver, Provider of Service, Patient"
            }
            Code0200 => {
                "Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Fleet (FL), Jurisdiction (JU)"
            }
            Code0201 => {
                "Credential Action (AP), Company/Corporation (35), Transportation Equipment (E)"
            }
            Code0202 => {
                "Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Jurisdiction (JU)"
            }
            Code0203 => {
                "Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E)"
            }
            Code0204 => "Report (RP), Jurisdiction (JU), Company/Corporation (35)",
            Code0205 => {
                "Report (RP), Company/Corporation (35), Fleet (FL), Jurisdiction (JU), Transportation Equipment (E)"
            }
            Code0206 => {
                "Credential Action (AP), Company/Corporation (35), Jurisdiction (JU), Transportation Equipment (E)"
            }
            Code0207 => {
                "Credential Action (AP), Company/Corporation (35), Transportation Equipment (E), Component (F), Measurement (M), Jurisdiction (JU)"
            }
            Code0208 => {
                "Credential Action (AP), Company/Corporation (35), Cost Type (CT), Jurisdiction (JU)"
            }
            Code0209 => "Credential Action (AP), Company/Corporation (35)",
            Code0210 => "Credential Action (AP)",
            Code0211 => {
                "Reporting Agency, Claim Administrator, Insurer, Insured, Employer, Claimant, Payment"
            }
            Code0212 => "Reporting Agency, Employer, Claimant",
            Code0213 => "Reporting Agency, Claim Administrator, Employer, Claimant",
            Code0214 => "Information Source, Information Receiver, Response Details",
            Code0400 => {
                "Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Fleet (FL), Jurisdiction (JU), Payment Detail (PY)"
            }
            Code0401 => {
                "Credential Action (AP), Company/Corporation (35), Transportation Equipment (E), Payment Detail (PY)"
            }
            Code0402 => {
                "Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Jurisdiction (JU), Payment Detail (PY)"
            }
            Code0403 => {
                "Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Payment Detail (PY)"
            }
            Code0407 => {
                "Credential Action (AP), Company/Corporation (35), Transportation Equipment (E), Component (F), Measurement (M), Jurisdiction (JU), Payment Detail (PY)"
            }
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<HierarchicalStructureCode> {
        {
            use HierarchicalStructureCode::*;
            match description {
                "Shipment, Order, Packaging, Item" => Some(Code0001),
                "Shipment, Order, Item, Packaging" => Some(Code0002),
                "Shipment, Packaging, Order, Item" => Some(Code0003),
                "Shipment, Order, Item" => Some(Code0004),
                "Financial Institution; Client or Party; Contract; Component; Account; Item; Coverage, Rider or Supplementary Benefit" => {
                    Some(Code0005)
                }
                "Financial Institution; Client or Party; Contract; Account; Coverage, Rider or Supplementary Benefit" => {
                    Some(Code0006)
                }
                "Information Source, Information Receiver, Subscriber, Dependent, Event, Services" => {
                    Some(Code0007)
                }
                "Information Source, Information Receiver, Provider of Service, Subscriber, Dependent" => {
                    Some(Code0010)
                }
                "Information Receiver, Information Source, Provider of Service, Subscriber, Dependent" => {
                    Some(Code0011)
                }
                "Information Source, Provider of Service, Subscriber, Dependent" => {
                    Some(Code0012)
                }
                "Provider of Service, Information Source, Subscriber, Dependent" => {
                    Some(Code0013)
                }
                "Provider of Service, Information Source, Information Receiver, Subscriber, Dependent" => {
                    Some(Code0014)
                }
                "Information Receiver, Provider of Service, Subscriber, Dependent" => {
                    Some(Code0015)
                }
                "Provider of Service, Subscriber, Dependent" => Some(Code0016),
                "Subscriber, Dependent" => Some(Code0017),
                "Information Receiver, Subscriber, Dependent" => Some(Code0018),
                "Information Source, Subscriber, Dependent" => Some(Code0019),
                "Information Source, Information Receiver, Group Coverage Options, Subscriber, Dependent" => {
                    Some(Code0020)
                }
                "Information Source, Information Receiver, Group Coverage Options" => {
                    Some(Code0021)
                }
                "Information Source, Information Receiver, Subscriber, Dependent" => {
                    Some(Code0022)
                }
                "Information Receiver, Information Source, Subscriber, Dependent" => {
                    Some(Code0023)
                }
                "Information Source, Information Receiver, Provider, Group, Site of Service" => {
                    Some(Code0024)
                }
                "Information Source, Information Receiver, Group, Provider, Site of Service" => {
                    Some(Code0025)
                }
                "Information Source, Information Receiver, Site of Service, Provider, Group" => {
                    Some(Code0026)
                }
                "Information Source, Information Receiver, Provider, Site of Service, Group" => {
                    Some(Code0027)
                }
                "Information Source, Information Receiver, Group, Site of Service, Provider" => {
                    Some(Code0028)
                }
                "Information Source, Information Receiver, Site of Service, Group, Provider" => {
                    Some(Code0029)
                }
                "Address, Shipment, Order" => Some(Code0035),
                "Address, Transaction Reference Number, Suffix, Serial Number" => {
                    Some(Code0036)
                }
                "Supergroup, Group, Subgroup, Member" => Some(Code0055),
                "Supergroup, subgroup, member, ancillary facility or department" => {
                    Some(Code0056)
                }
                "Supergroup, subgroup, member" => Some(Code0057),
                "Group, member, ancillary facility or department" => Some(Code0058),
                "Group, member" => Some(Code0059),
                "Hospital, ancillary facility or department" => Some(Code0060),
                "Health Industry Business Communications Council (HIBCC) Health Industry Number (HIN) database, facility record, location record" => {
                    Some(Code0061)
                }
                "Franchisor, franchisee" => Some(Code0062),
                "Franchisee association, franchisee" => Some(Code0063),
                "Company, Company" => Some(Code0064),
                "Company, Operating Unit" => Some(Code0065),
                "Operating Unit, Operating Unit" => Some(Code0066),
                "Company, Property" => Some(Code0067),
                "Company, Property Property, Property" => Some(Code0068),
                "Operating Unit, Property" => Some(Code0069),
                "Property, Property" => Some(Code0070),
                "Company, Tradename" => Some(Code0071),
                "Operating Unit, Tradename" => Some(Code0072),
                "Property, Tradename" => Some(Code0073),
                "Company, Operating Unit, Operating Unit, Operating Unit" => {
                    Some(Code0074)
                }
                "Operating Unit, Operating Unit, Operating Unit, Operating Unit" => {
                    Some(Code0075)
                }
                "Company, Operating Unit, Operating Unit, Property" => Some(Code0076),
                "Tradename, Property" => Some(Code0077),
                "Information Source, Information Receiver, Subscriber, Dependent, Provider of Service, Services" => {
                    Some(Code0078)
                }
                "Information Source, Information Receiver, Company/Corporation, Operating Unit" => {
                    Some(Code0079)
                }
                "Information Source, Employer, Patient" => Some(Code0080),
                "Information Source, Patient" => Some(Code0081),
                "Information Source, Employer, Subscriber, Dependent" => Some(Code0082),
                "Information Source, Information Receiver, Subscriber, Dependent, Referring Provider, Provider of Service, Services" => {
                    Some(Code0083)
                }
                "Report, Site, Sample, Test" => Some(Code0084),
                "Information Source, Information Receiver, Provider of Service, Patient" => {
                    Some(Code0085)
                }
                "Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Fleet (FL), Jurisdiction (JU)" => {
                    Some(Code0200)
                }
                "Credential Action (AP), Company/Corporation (35), Transportation Equipment (E)" => {
                    Some(Code0201)
                }
                "Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Jurisdiction (JU)" => {
                    Some(Code0202)
                }
                "Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E)" => {
                    Some(Code0203)
                }
                "Report (RP), Jurisdiction (JU), Company/Corporation (35)" => {
                    Some(Code0204)
                }
                "Report (RP), Company/Corporation (35), Fleet (FL), Jurisdiction (JU), Transportation Equipment (E)" => {
                    Some(Code0205)
                }
                "Credential Action (AP), Company/Corporation (35), Jurisdiction (JU), Transportation Equipment (E)" => {
                    Some(Code0206)
                }
                "Credential Action (AP), Company/Corporation (35), Transportation Equipment (E), Component (F), Measurement (M), Jurisdiction (JU)" => {
                    Some(Code0207)
                }
                "Credential Action (AP), Company/Corporation (35), Cost Type (CT), Jurisdiction (JU)" => {
                    Some(Code0208)
                }
                "Credential Action (AP), Company/Corporation (35)" => Some(Code0209),
                "Credential Action (AP)" => Some(Code0210),
                "Reporting Agency, Claim Administrator, Insurer, Insured, Employer, Claimant, Payment" => {
                    Some(Code0211)
                }
                "Reporting Agency, Employer, Claimant" => Some(Code0212),
                "Reporting Agency, Claim Administrator, Employer, Claimant" => {
                    Some(Code0213)
                }
                "Information Source, Information Receiver, Response Details" => {
                    Some(Code0214)
                }
                "Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Fleet (FL), Jurisdiction (JU), Payment Detail (PY)" => {
                    Some(Code0400)
                }
                "Credential Action (AP), Company/Corporation (35), Transportation Equipment (E), Payment Detail (PY)" => {
                    Some(Code0401)
                }
                "Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Jurisdiction (JU), Payment Detail (PY)" => {
                    Some(Code0402)
                }
                "Credential Action (AP), Company/Corporation (35), Quantity (R), Transportation Equipment (E), Payment Detail (PY)" => {
                    Some(Code0403)
                }
                "Credential Action (AP), Company/Corporation (35), Transportation Equipment (E), Component (F), Measurement (M), Jurisdiction (JU), Payment Detail (PY)" => {
                    Some(Code0407)
                }
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for HierarchicalStructureCode {
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
    type Value = HierarchicalStructureCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Hierarchical Structure Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        HierarchicalStructureCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Hierarchical Structure Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        HierarchicalStructureCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Hierarchical Structure Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for HierarchicalStructureCode {
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