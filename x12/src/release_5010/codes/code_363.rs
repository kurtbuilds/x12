use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**363

See docs at <https://www.stedi.com/edi/x12/element/363>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoteReferenceCode {
    ///AAA - Agent Details
    AgentDetails,
    ///AAB - Associated Business Areas
    AssociatedBusinessAreas,
    ///AAC - Borrower
    Borrower,
    ///AAD - Nationality Details
    NationalityDetails,
    ///AAE - Assets
    Assets,
    ///AAF - Liabilities
    Liabilities,
    ///AAH - Additional Facility Details
    AdditionalFacilityDetails,
    ///AAI - Exemption Law Location Description
    ExemptionLawLocationDescription,
    ///AAJ - Forecast Details
    ForecastDetails,
    ///AAK - Import and Export Details
    ImportAndExportDetails,
    ///AAL - Inventory Valuation
    InventoryValuation,
    ///AAM - Product Brands Sold Description
    ProductBrandsSoldDescription,
    ///AAN - Purchase Territory
    PurchaseTerritory,
    ///AAO - Responsibilities
    Responsibilities,
    ///AAP - Supplier Description
    SupplierDescription,
    ///AAQ - Education Description
    EducationDescription,
    ///AAR - Liquidity Details
    LiquidityDetails,
    ///AAS - Former Activity Description
    FormerActivityDescription,
    ///AAT - Division Description
    DivisionDescription,
    ///ABN - Abbreviated Nomenclature
    AbbreviatedNomenclature,
    ///ACC - Access Instructions
    AccessInstructions,
    ///ACI - Additional Claim Information
    AdditionalClaim,
    ///ACN - Action Taken
    ActionTaken,
    ///ACS - Actual Solution
    ActualSolution,
    ///ACT - Action
    Action,
    ///ADD - Additional Information
    Additional,
    ///AES - Actual Evaluation Summary
    ActualEvaluationSummary,
    ///AET - Adverse Event Terms
    AdverseEventTerms,
    ///AFA - Description
    Description,
    ///AFB - Generic Chemical Name
    GenericChemicalName,
    ///AFC - Prevention Program Description
    PreventionProgramDescription,
    ///AFD - Risk Management Plan Description
    RiskManagementPlanDescription,
    ///AFE - Safety Comments
    SafetyComments,
    ///AFF - Summary
    Summary,
    ///ALG - Allergies
    Allergies,
    ///ALL - All Documents
    AllDocuments,
    ///ALT - Alerts
    Alerts,
    ///AMN - Additional Manufacturer Narrative
    AdditionalManufacturerNarrative,
    ///AOO - Area of Operation
    AreaOfOperation,
    ///APN - Application Notes
    ApplicationNotes,
    ///APS - Appropriation Specifications
    AppropriationSpecifications,
    ///BBD - Bank Description
    BankDescription,
    ///BBF - Business Founder
    BusinessFounder,
    ///BBH - Business History
    BusinessHistory,
    ///BBN - Banking Notes
    BankingNotes,
    ///BBO - Business Origin Description
    BusinessOriginDescription,
    ///BBT - Brand Names
    BrandNames,
    ///BFD - Business Financing Details
    BusinessFinancingDetails,
    ///BOL - Bill of Lading Note
    BillOfLadingNote,
    ///BUR - Bureau Remarks
    BureauRemarks,
    ///CAA - Authentication Information
    Authentication,
    ///CAB - Line of In-State Business
    LineOfInStateBusiness,
    ///CAC - Relationship Information
    Relationship,
    ///CAD - Basis for Amount Due
    BasisForAmountDue,
    ///CAE - Type of Debt
    TypeOfDebt,
    ///CAF - Land Use Purpose
    LandUsePurpose,
    ///CAG - Land Description
    LandDescription,
    ///CAH - Basis of Calculation
    BasisOfCalculation,
    ///CAI - General Business Description
    GeneralBusinessDescription,
    ///CAJ - Type of Business
    TypeOfBusiness,
    ///CAK - Character of Business
    CharacterOfBusiness,
    ///CAL - Representation of Value
    RepresentationOfValue,
    ///CAM - Supporting Statement, Tax, and Fee Computation
    CodeCAM,
    ///CAN - Cooperative Corporation Statement
    CooperativeCorporationStatement,
    ///CAO - Close Corporation Statement
    CloseCorporationStatement,
    ///CAP - Agreement to Abide by Laws
    AgreementToAbideByLaws,
    ///CAQ - Stock Restrictions
    StockRestrictions,
    ///CAR - Other Related Information
    OtherRelated,
    ///CAS - Prohibition Against Being an Officer
    ProhibitionAgainstBeingAnOfficer,
    ///CAT - Qualification of Director
    QualificationOfDirector,
    ///CAU - Nature of Charter
    NatureOfCharter,
    ///CAV - Statement of Assets and Liabilities
    StatementOfAssetsAndLiabilities,
    ///CAW - Bankruptcy Information
    Bankruptcy,
    ///CAX - Certificate of Disclosure
    CertificateOfDisclosure,
    ///CAZ - Asset Detail
    AssetDetail,
    ///CBA - Statement Related to Regulation
    StatementRelatedToRegulation,
    ///CBB - Consideration to be Received
    ConsiderationToBeReceived,
    ///CBC - Other Lawful Provisions
    OtherLawfulProvisions,
    ///CBH - Monetary Amount Description
    MonetaryAmountDescription,
    ///CBI - Description of Title
    DescriptionOfTitle,
    ///CCA - Competition
    Competition,
    ///CCB - Construction Details
    ConstructionDetails,
    ///CCC - Construction Financing
    ConstructionFinancing,
    ///CCD - Construction Line of Business
    ConstructionLineOfBusiness,
    ///CCE - Contract Details
    ContractDetails,
    ///CCF - Corporate Filing Details
    CorporateFilingDetails,
    ///CCG - Customer Description
    CustomerDescription,
    ///CCN - Copyright Notice
    CopyrightNotice,
    ///CDD - Contingent Debt Details
    ContingentDebtDetails,
    ///CER - Certification Narrative
    CertificationNarrative,
    ///CHG - Change
    Change,
    ///CIG - Cigarette Information
    Cigarette,
    ///CIR - Circumstances Prior to Difficulty
    CircumstancesPriorToDifficulty,
    ///CLN - Classifying Information
    Classifying,
    ///CLR - Security Clearance Instructions
    SecurityClearanceInstructions,
    ///CMP - Concomitant Medical Product Description
    ConcomitantMedicalProductDescription,
    ///CMT - Maintenance Comment
    MaintenanceComment,
    ///COD - Corrected Data
    CorrectedData,
    ///COM - Consumer Comments
    ConsumerComments,
    ///CON - Conviction Act Details
    ConvictionActDetails,
    ///CRA - Credit Report Alerts
    CreditReportAlerts,
    ///CRK - Closing Comment
    ClosingComment,
    ///CRN - Credit Report Notes
    CreditReportNotes,
    ///CUS - Customs declaration
    CustomsDeclaration,
    ///DCP - Goals, Rehabilitation Potential, or Discharge Plans
    CodeDCP,
    ///DCS - Destination Control Statement
    DestinationControlStatement,
    ///DDC - Deficiency Description Changes
    DeficiencyDescriptionChanges,
    ///DEE - Event Description
    EventDescription,
    ///DEL - Delivery
    Delivery,
    ///DEP - Problem Description
    ProblemDescription,
    ///DFR - Dose, Frequency and Route Description
    CodeDFR,
    ///DFS - Departure from Specification Comment
    DepartureFromSpecificationComment,
    ///DGN - Diagnosis Description
    DiagnosisDescription,
    ///DME - Durable Medical Equipment (DME) and Supplies
    CodeDME,
    ///DOD - Description of Damage
    DescriptionOfDamage,
    ///DOI - Outcome Description
    OutcomeDescription,
    ///DRV - Driver Out of Service Notice
    DriverOutOfServiceNotice,
    ///DRW - Driver Out of Service Resolution
    DriverOutOfServiceResolution,
    ///DSW - Detailed Statement of Work
    DetailedStatementOfWork,
    ///EAA - Other Type of Group
    OtherTypeOfGroup,
    ///EAB - Ballot Measure
    BallotMeasure,
    ///EAC - Attachment
    Attachment,
    ///EAD - Board
    Board,
    ///EAE - Prohibited Contribution Circumstance
    ProhibitedContributionCircumstance,
    ///EAF - Committee Activity
    CommitteeActivity,
    ///EAG - Compensation Arrangement
    CompensationArrangement,
    ///EAH - Country Sub-Entity
    CountrySubEntity,
    ///EAI - Faction
    Faction,
    ///EAJ - Gift
    Gift,
    ///EAK - In-Kind Contribution Use
    InKindContributionUse,
    ///EAL - Industry Group
    IndustryGroup,
    ///EAM - Jurisdiction
    Jurisdiction,
    ///EAN - Nature and Purpose of Other Lobbyist Employers
    NatureAndPurposeOfOtherLobbyistEmployers,
    ///EAO - Description of Office
    DescriptionOfOffice,
    ///EAP - Party Considering Legislation
    PartyConsideringLegislation,
    ///EAQ - Description of Position
    DescriptionOfPosition,
    ///EAR - Description of Sponsor
    DescriptionOfSponsor,
    ///EAS - Affiliation
    Affiliation,
    ///EAT - Asset Disposition
    AssetDisposition,
    ///EAV - Committee Interest
    CommitteeInterest,
    ///EAW - Compensation or Services Exchanged for Consideration
    CompensationOrServicesExchangedForConsideration,
    ///EAX - Contributor Interest
    ContributorInterest,
    ///EAY - Description of Debt
    DescriptionOfDebt,
    ///EAZ - Employer Description
    EmployerDescription,
    ///EBA - Purpose of Employment
    PurposeOfEmployment,
    ///EBB - Description of Agency and Position
    DescriptionOfAgencyAndPosition,
    ///EBC - Description of Goods and Services
    DescriptionOfGoodsAndServices,
    ///EBD - Length of Lobbying Actions
    LengthOfLobbyingActions,
    ///EBE - Lobbying Interest
    LobbyingInterest,
    ///EBF - Method of Disposal
    MethodOfDisposal,
    ///EBG - Purpose of Credit
    PurposeOfCredit,
    ///EBH - Description of Other Business Sub-Category
    DescriptionOfOtherBusinessSubCategory,
    ///EBI - Other Type of Compensation
    OtherTypeOfCompensation,
    ///EBJ - Description of Other Legislative Interest
    DescriptionOfOtherLegislativeInterest,
    ///EBK - Other Reason for Withdrawal
    OtherReasonForWithdrawal,
    ///EBL - Other Type of Relationship
    OtherTypeOfRelationship,
    ///EBM - Other Temporary Residence
    OtherTemporaryResidence,
    ///EBN - Other Type of Committee
    OtherTypeOfCommittee,
    ///EBO - Place Paid
    PlacePaid,
    ///EBP - Proposition
    Proposition,
    ///EBQ - Reason for Purchase
    ReasonForPurchase,
    ///EBR - Reason for Contribution
    ReasonForContribution,
    ///EBS - Description of Repayment Schedule
    DescriptionOfRepaymentSchedule,
    ///EBU - Service Description
    ServiceDescription,
    ///EBV - Initiative
    Initiative,
    ///EBW - Description of Amendment
    DescriptionOfAmendment,
    ///EBX - Type of Election
    TypeOfElection,
    ///EBY - Other Type of Account
    OtherTypeOfAccount,
    ///EBZ - Interest Rate Description
    InterestRateDescription,
    ///ECA - In-Kind Contribution
    InKindContribution,
    ///ECB - Reason for Refund
    ReasonForRefund,
    ///ECC - Incidental Expenses
    IncidentalExpenses,
    ///ECD - Environmental Conditions Description
    EnvironmentalConditionsDescription,
    ///ECE - Other Expenses
    OtherExpenses,
    ///ECF - Other Income
    OtherIncome,
    ///ECG - Description of Receipt
    DescriptionOfReceipt,
    ///ECH - Surplus Funds
    SurplusFunds,
    ///ECI - Collateral
    Collateral,
    ///ECJ - Contributor
    Contributor,
    ///ECK - Miscellaneous Receipt Transaction
    MiscellaneousReceiptTransaction,
    ///ECL - Other Advertising
    OtherAdvertising,
    ///ECM - Estimate Comment
    EstimateComment,
    ///ECN - Equipment Condition Description
    EquipmentConditionDescription,
    ///ECO - Other Necessary Personal Expense
    OtherNecessaryPersonalExpense,
    ///ECP - Other Campaign Expense
    OtherCampaignExpense,
    ///ECQ - Long-Term Liability
    LongTermLiability,
    ///ECR - Short-Term Liability
    ShortTermLiability,
    ///ECS - Other Sponsor Expense
    OtherSponsorExpense,
    ///ECT - Emergency Certification
    EmergencyCertification,
    ///ECU - Other Transportation Expense
    OtherTransportationExpense,
    ///ECV - Refund
    Refund,
    ///ECW - Reason for Return
    ReasonForReturn,
    ///EED - Equipment Description
    EquipmentDescription,
    ///EFD - Equipment Function Description
    EquipmentFunctionDescription,
    ///ELE - Equipment Log Entry
    EquipmentLogEntry,
    ///EMC - Employment Comments
    EmploymentComments,
    ///EMD - Estimate Method Description
    EstimateMethodDescription,
    ///ENR - Explanation for Non-Return of Device to Manufacturer
    ExplanationForNonReturnOfDeviceToManufacturer,
    ///ERN - Error Notes
    ErrorNotes,
    ///EVL - Event Location
    EventLocation,
    ///EXE - Exemption Description
    ExemptionDescription,
    ///EXN - Exhibit Notes
    ExhibitNotes,
    ///EXR - Exercise Routine
    ExerciseRoutine,
    ///EXT - Exterior Description
    ExteriorDescription,
    ///FDD - Final Deficiency Description
    FinalDeficiencyDescription,
    ///FEE - Fee Description
    FeeDescription,
    ///FIX - Repair Summary
    RepairSummary,
    ///FUT - Future Plans
    FuturePlans,
    ///GEN - Entire Transaction Set
    EntireTransactionSet,
    ///GPI - General Product or Process Information
    GeneralProductOrProcess,
    ///GPL - General Policy
    GeneralPolicy,
    ///GSI - General Specification Information
    GeneralSpecification,
    ///HHI - Household Goods Information
    HouseholdGoods,
    ///ICN - Interviewee Conversation
    IntervieweeConversation,
    ///IDT - Intangible Description
    IntangibleDescription,
    ///IID - Inventory (Stock) Description
    CodeIID,
    ///IIE - Investment Description
    InvestmentDescription,
    ///IIR - Intercompany Relations
    IntercompanyRelations,
    ///IMP - Problem Impact
    ProblemImpact,
    ///INP - Reason for Inspection
    ReasonForInspection,
    ///INS - Insurance
    Insurance,
    ///INT - General Order Instructions
    GeneralOrderInstructions,
    ///INV - Invoice Instruction
    InvoiceInstruction,
    ///IVC - Income Verification Comments
    IncomeVerificationComments,
    ///JVD - Joint Venture Description
    JointVentureDescription,
    ///LAB - Labeling Instructions
    LabelingInstructions,
    ///LBD - Laboratory Data
    LaboratoryData,
    ///LBS - Labeled Strength
    LabeledStrength,
    ///LEN - Lender Use
    LenderUse,
    ///LIN - Line Item
    LineItem,
    ///LIQ - Liquor Information
    Liquor,
    ///LLA - Letters of Liability Agreements
    LettersOfLiabilityAgreements,
    ///LLB - Loan Details
    LoanDetails,
    ///LLC - Long Term Debt Description
    LongTermDebtDescription,
    ///LOC - Location Description Information
    LocationDescription,
    ///LOI - Loading Instructions
    LoadingInstructions,
    ///LSD - Legal Structure Details
    LegalStructureDetails,
    ///MCD - Marital Contract Details
    MaritalContractDetails,
    ///MCN - Motor Carrier Instructions
    MotorCarrierInstructions,
    ///MDO - Device Operator Description
    DeviceOperatorDescription,
    ///MED - Medications
    Medications,
    ///MFG - Manufacturing Instructions
    ManufacturingInstructions,
    ///MKN - Marketing Notes
    MarketingNotes,
    ///MMD - Merger Description
    MergerDescription,
    ///MSD - Marketable Securities Description
    MarketableSecuritiesDescription,
    ///NCD - Nonconformance Specification
    NonconformanceSpecification,
    ///NPD - Nameplate Data
    NameplateData,
    ///NTR - Nutritional Requirements
    NutritionalRequirements,
    ///OBI - Originator to Beneficiary Instructions
    OriginatorToBeneficiaryInstructions,
    ///OBL - Obligation Description
    ObligationDescription,
    ///OCA - Other Current Asset Description
    OtherCurrentAssetDescription,
    ///OCC - Occupancy Information
    Occupancy,
    ///OCL - Other Current Liability Description
    OtherCurrentLiabilityDescription,
    ///OCP - Occupation
    Occupation,
    ///OCR - Outside the Continental U.S. (OCONUS) Rating Information
    CodeOCR,
    ///ODD - Originator Deficiency Description
    OriginatorDeficiencyDescription,
    ///ODT - Orders for Disciplines and Treatments
    OrdersForDisciplinesAndTreatments,
    ///OLS - Original Legal Structure
    OriginalLegalStructure,
    ///OPO - Occupation Definition
    OccupationDefinition,
    ///ORA - Test Results Other Than Room Air
    TestResultsOtherThanRoomAir,
    ///ORD - Ordering Restrictions
    OrderingRestrictions,
    ///ORE - Other Remedial Action
    OtherRemedialAction,
    ///ORI - Order Instructions
    OrderInstructions,
    ///OTH - Other Instructions
    OtherInstructions,
    ///OTN - Another Type of Number Description
    AnotherTypeOfNumberDescription,
    ///OTS - Report Source Description
    ReportSourceDescription,
    ///PAY - Payables
    Payables,
    ///PCS - Process Specification
    ProcessSpecification,
    ///PDS - Product Specification
    ProductSpecification,
    ///PED - Employee Sharing Arrangements
    EmployeeSharingArrangements,
    ///PEN - Penalty Description
    PenaltyDescription,
    ///PES - Partial Pressure of Oxygen (PO2) is 60 millimeters (MM) of Mercury (Hg) or above, or arterial blood oxygen saturation is 90% or above
    CodePES,
    ///PID - Property Improvement Description
    PropertyImprovementDescription,
    ///PKG - Packaging Instructions
    PackagingInstructions,
    ///PMT - Payment
    Payment,
    ///POB - Primary Observation
    PrimaryObservation,
    ///POC - Principals or Organization Comments
    PrincipalsOrOrganizationComments,
    ///POL - Property Owner Location Information
    PropertyOwnerLocation,
    ///PPC - Principal Procedure Code Description
    PrincipalProcedureCodeDescription,
    ///PRI - Priority
    Priority,
    ///PRN - Public Record Notes
    PublicRecordNotes,
    ///PRO - Previous Registered Office
    PreviousRegisteredOffice,
    ///PRR - Price Range
    PriceRange,
    ///PSY - Problem Summary
    ProblemSummary,
    ///PUR - Purchasing
    Purchasing,
    ///QUL - Qualifications
    Qualifications,
    ///QUT - Quotation Instruction
    QuotationInstruction,
    ///RDI - Reason for Delinquency Information
    ReasonForDelinquency,
    ///REC - Recommendation
    Recommendation,
    ///REF - Treatment Refusal Note
    TreatmentRefusalNote,
    ///REG - Registered Activity
    RegisteredActivity,
    ///REP - Report
    Report,
    ///REV - Receivables
    Receivables,
    ///RFL - Reason for Leaving
    ReasonForLeaving,
    ///RHB - Functional Limitations, Reason Homebound, or Both
    CodeRHB,
    ///RLA - Reason for Leave of Absence
    ReasonForLeaveOfAbsence,
    ///RLH - Reasons Patient Leaves Home
    ReasonsPatientLeavesHome,
    ///RNH - Times and Reasons Patient Not at Home
    TimesAndReasonsPatientNotAtHome,
    ///RNI - Missing Report Explanation
    MissingReportExplanation,
    ///ROU - Circuit Routing Instructions
    CircuitRoutingInstructions,
    ///RPT - Report Remarks
    ReportRemarks,
    ///RST - Place Where a Report was Submitted
    PlaceWhereAReportWasSubmitted,
    ///RVC - Rent Verification Comments
    RentVerificationComments,
    ///SAN - Settlement Amount Notes
    SettlementAmountNotes,
    ///SCN - Ocean Shipping Container Information
    OceanShippingContainer,
    ///SDD - Sentence Description
    SentenceDescription,
    ///SET - Unusual Home, Social Environment, or Both
    CodeSET,
    ///SFM - Safety Measures
    SafetyMeasures,
    ///SHR - Shipping Restrictions
    ShippingRestrictions,
    ///SMD - Selling Means Description
    SellingMeansDescription,
    ///SOB - Secondary Observation
    SecondaryObservation,
    ///SOW - Statement of Work
    StatementOfWork,
    ///SPH - Special Handling
    SpecialHandling,
    ///SPT - Supplementary Plan of Treatment
    SupplementaryPlanOfTreatment,
    ///SPV - Closing Instructions
    ClosingInstructions,
    ///SSA - School Attended Details
    SchoolAttendedDetails,
    ///SSC - Status Comment
    StatusComment,
    ///SSD - Sales Description
    SalesDescription,
    ///SSE - Spouse Information
    Spouse,
    ///SSG - School Graduated Details
    SchoolGraduatedDetails,
    ///SSH - Security Service Information
    SecurityService,
    ///SSI - Shareholding Information
    Shareholding,
    ///SSS - Signing Authority
    SigningAuthority,
    ///SST - Sales Territory
    SalesTerritory,
    ///TAF - Tariff Abbreviation
    TariffAbbreviation,
    ///TCF - Tariff Commodity Footnotes
    TariffCommodityFootnotes,
    ///TDA - Turkish Defense Affairs Authorization Information
    TurkishDefenseAffairsAuthorization,
    ///TES - Task Statement
    TaskStatement,
    ///TIL - Tariff Index
    TariffIndex,
    ///TLF - Tariff Rule
    TariffRule,
    ///TLR - Tradeline Remarks
    TradelineRemarks,
    ///TMP - Toxic Organic Management Plan
    ToxicOrganicManagementPlan,
    ///TPO - Third Party Organization Notes
    ThirdPartyOrganizationNotes,
    ///TRA - Transportation
    Transportation,
    ///TRE - Reportable Event Description
    ReportableEventDescription,
    ///TRF - Tariff Rate Footnotes
    TariffRateFootnotes,
    ///TRS - Quality Information
    Quality,
    ///TSD - Terms of Sale Description
    TermsOfSaleDescription,
    ///TSF - Tariff Section Footnotes
    TariffSectionFootnotes,
    ///TST - Test Results
    TestResults,
    ///UPI - Updated Information
    Updated,
    ///VEC - Verification Comments
    VerificationComments,
    ///VEH - Vehicle Out of Service Notice
    VehicleOutOfServiceNotice,
    ///VNN - Variation Notes
    VariationNotes,
    ///WHI - Warehouse Instruction
    WarehouseInstruction,
    ///WRP - Wrapping Instructions
    WrappingInstructions,
    ///ZED - Zero Discharge Certification Statement
    ZeroDischargeCertificationStatement,
    ///ZZZ - Mutually Defined
    MutuallyDefined,
}
impl NoteReferenceCode {
    pub fn code(&self) -> &str {
        {
            use NoteReferenceCode::*;
            match self {
                AgentDetails => "AAA",
                AssociatedBusinessAreas => "AAB",
                Borrower => "AAC",
                NationalityDetails => "AAD",
                Assets => "AAE",
                Liabilities => "AAF",
                AdditionalFacilityDetails => "AAH",
                ExemptionLawLocationDescription => "AAI",
                ForecastDetails => "AAJ",
                ImportAndExportDetails => "AAK",
                InventoryValuation => "AAL",
                ProductBrandsSoldDescription => "AAM",
                PurchaseTerritory => "AAN",
                Responsibilities => "AAO",
                SupplierDescription => "AAP",
                EducationDescription => "AAQ",
                LiquidityDetails => "AAR",
                FormerActivityDescription => "AAS",
                DivisionDescription => "AAT",
                AbbreviatedNomenclature => "ABN",
                AccessInstructions => "ACC",
                AdditionalClaim => "ACI",
                ActionTaken => "ACN",
                ActualSolution => "ACS",
                Action => "ACT",
                Additional => "ADD",
                ActualEvaluationSummary => "AES",
                AdverseEventTerms => "AET",
                Description => "AFA",
                GenericChemicalName => "AFB",
                PreventionProgramDescription => "AFC",
                RiskManagementPlanDescription => "AFD",
                SafetyComments => "AFE",
                Summary => "AFF",
                Allergies => "ALG",
                AllDocuments => "ALL",
                Alerts => "ALT",
                AdditionalManufacturerNarrative => "AMN",
                AreaOfOperation => "AOO",
                ApplicationNotes => "APN",
                AppropriationSpecifications => "APS",
                BankDescription => "BBD",
                BusinessFounder => "BBF",
                BusinessHistory => "BBH",
                BankingNotes => "BBN",
                BusinessOriginDescription => "BBO",
                BrandNames => "BBT",
                BusinessFinancingDetails => "BFD",
                BillOfLadingNote => "BOL",
                BureauRemarks => "BUR",
                Authentication => "CAA",
                LineOfInStateBusiness => "CAB",
                Relationship => "CAC",
                BasisForAmountDue => "CAD",
                TypeOfDebt => "CAE",
                LandUsePurpose => "CAF",
                LandDescription => "CAG",
                BasisOfCalculation => "CAH",
                GeneralBusinessDescription => "CAI",
                TypeOfBusiness => "CAJ",
                CharacterOfBusiness => "CAK",
                RepresentationOfValue => "CAL",
                CodeCAM => "CAM",
                CooperativeCorporationStatement => "CAN",
                CloseCorporationStatement => "CAO",
                AgreementToAbideByLaws => "CAP",
                StockRestrictions => "CAQ",
                OtherRelated => "CAR",
                ProhibitionAgainstBeingAnOfficer => "CAS",
                QualificationOfDirector => "CAT",
                NatureOfCharter => "CAU",
                StatementOfAssetsAndLiabilities => "CAV",
                Bankruptcy => "CAW",
                CertificateOfDisclosure => "CAX",
                AssetDetail => "CAZ",
                StatementRelatedToRegulation => "CBA",
                ConsiderationToBeReceived => "CBB",
                OtherLawfulProvisions => "CBC",
                MonetaryAmountDescription => "CBH",
                DescriptionOfTitle => "CBI",
                Competition => "CCA",
                ConstructionDetails => "CCB",
                ConstructionFinancing => "CCC",
                ConstructionLineOfBusiness => "CCD",
                ContractDetails => "CCE",
                CorporateFilingDetails => "CCF",
                CustomerDescription => "CCG",
                CopyrightNotice => "CCN",
                ContingentDebtDetails => "CDD",
                CertificationNarrative => "CER",
                Change => "CHG",
                Cigarette => "CIG",
                CircumstancesPriorToDifficulty => "CIR",
                Classifying => "CLN",
                SecurityClearanceInstructions => "CLR",
                ConcomitantMedicalProductDescription => "CMP",
                MaintenanceComment => "CMT",
                CorrectedData => "COD",
                ConsumerComments => "COM",
                ConvictionActDetails => "CON",
                CreditReportAlerts => "CRA",
                ClosingComment => "CRK",
                CreditReportNotes => "CRN",
                CustomsDeclaration => "CUS",
                CodeDCP => "DCP",
                DestinationControlStatement => "DCS",
                DeficiencyDescriptionChanges => "DDC",
                EventDescription => "DEE",
                Delivery => "DEL",
                ProblemDescription => "DEP",
                CodeDFR => "DFR",
                DepartureFromSpecificationComment => "DFS",
                DiagnosisDescription => "DGN",
                CodeDME => "DME",
                DescriptionOfDamage => "DOD",
                OutcomeDescription => "DOI",
                DriverOutOfServiceNotice => "DRV",
                DriverOutOfServiceResolution => "DRW",
                DetailedStatementOfWork => "DSW",
                OtherTypeOfGroup => "EAA",
                BallotMeasure => "EAB",
                Attachment => "EAC",
                Board => "EAD",
                ProhibitedContributionCircumstance => "EAE",
                CommitteeActivity => "EAF",
                CompensationArrangement => "EAG",
                CountrySubEntity => "EAH",
                Faction => "EAI",
                Gift => "EAJ",
                InKindContributionUse => "EAK",
                IndustryGroup => "EAL",
                Jurisdiction => "EAM",
                NatureAndPurposeOfOtherLobbyistEmployers => "EAN",
                DescriptionOfOffice => "EAO",
                PartyConsideringLegislation => "EAP",
                DescriptionOfPosition => "EAQ",
                DescriptionOfSponsor => "EAR",
                Affiliation => "EAS",
                AssetDisposition => "EAT",
                CommitteeInterest => "EAV",
                CompensationOrServicesExchangedForConsideration => "EAW",
                ContributorInterest => "EAX",
                DescriptionOfDebt => "EAY",
                EmployerDescription => "EAZ",
                PurposeOfEmployment => "EBA",
                DescriptionOfAgencyAndPosition => "EBB",
                DescriptionOfGoodsAndServices => "EBC",
                LengthOfLobbyingActions => "EBD",
                LobbyingInterest => "EBE",
                MethodOfDisposal => "EBF",
                PurposeOfCredit => "EBG",
                DescriptionOfOtherBusinessSubCategory => "EBH",
                OtherTypeOfCompensation => "EBI",
                DescriptionOfOtherLegislativeInterest => "EBJ",
                OtherReasonForWithdrawal => "EBK",
                OtherTypeOfRelationship => "EBL",
                OtherTemporaryResidence => "EBM",
                OtherTypeOfCommittee => "EBN",
                PlacePaid => "EBO",
                Proposition => "EBP",
                ReasonForPurchase => "EBQ",
                ReasonForContribution => "EBR",
                DescriptionOfRepaymentSchedule => "EBS",
                ServiceDescription => "EBU",
                Initiative => "EBV",
                DescriptionOfAmendment => "EBW",
                TypeOfElection => "EBX",
                OtherTypeOfAccount => "EBY",
                InterestRateDescription => "EBZ",
                InKindContribution => "ECA",
                ReasonForRefund => "ECB",
                IncidentalExpenses => "ECC",
                EnvironmentalConditionsDescription => "ECD",
                OtherExpenses => "ECE",
                OtherIncome => "ECF",
                DescriptionOfReceipt => "ECG",
                SurplusFunds => "ECH",
                Collateral => "ECI",
                Contributor => "ECJ",
                MiscellaneousReceiptTransaction => "ECK",
                OtherAdvertising => "ECL",
                EstimateComment => "ECM",
                EquipmentConditionDescription => "ECN",
                OtherNecessaryPersonalExpense => "ECO",
                OtherCampaignExpense => "ECP",
                LongTermLiability => "ECQ",
                ShortTermLiability => "ECR",
                OtherSponsorExpense => "ECS",
                EmergencyCertification => "ECT",
                OtherTransportationExpense => "ECU",
                Refund => "ECV",
                ReasonForReturn => "ECW",
                EquipmentDescription => "EED",
                EquipmentFunctionDescription => "EFD",
                EquipmentLogEntry => "ELE",
                EmploymentComments => "EMC",
                EstimateMethodDescription => "EMD",
                ExplanationForNonReturnOfDeviceToManufacturer => "ENR",
                ErrorNotes => "ERN",
                EventLocation => "EVL",
                ExemptionDescription => "EXE",
                ExhibitNotes => "EXN",
                ExerciseRoutine => "EXR",
                ExteriorDescription => "EXT",
                FinalDeficiencyDescription => "FDD",
                FeeDescription => "FEE",
                RepairSummary => "FIX",
                FuturePlans => "FUT",
                EntireTransactionSet => "GEN",
                GeneralProductOrProcess => "GPI",
                GeneralPolicy => "GPL",
                GeneralSpecification => "GSI",
                HouseholdGoods => "HHI",
                IntervieweeConversation => "ICN",
                IntangibleDescription => "IDT",
                CodeIID => "IID",
                InvestmentDescription => "IIE",
                IntercompanyRelations => "IIR",
                ProblemImpact => "IMP",
                ReasonForInspection => "INP",
                Insurance => "INS",
                GeneralOrderInstructions => "INT",
                InvoiceInstruction => "INV",
                IncomeVerificationComments => "IVC",
                JointVentureDescription => "JVD",
                LabelingInstructions => "LAB",
                LaboratoryData => "LBD",
                LabeledStrength => "LBS",
                LenderUse => "LEN",
                LineItem => "LIN",
                Liquor => "LIQ",
                LettersOfLiabilityAgreements => "LLA",
                LoanDetails => "LLB",
                LongTermDebtDescription => "LLC",
                LocationDescription => "LOC",
                LoadingInstructions => "LOI",
                LegalStructureDetails => "LSD",
                MaritalContractDetails => "MCD",
                MotorCarrierInstructions => "MCN",
                DeviceOperatorDescription => "MDO",
                Medications => "MED",
                ManufacturingInstructions => "MFG",
                MarketingNotes => "MKN",
                MergerDescription => "MMD",
                MarketableSecuritiesDescription => "MSD",
                NonconformanceSpecification => "NCD",
                NameplateData => "NPD",
                NutritionalRequirements => "NTR",
                OriginatorToBeneficiaryInstructions => "OBI",
                ObligationDescription => "OBL",
                OtherCurrentAssetDescription => "OCA",
                Occupancy => "OCC",
                OtherCurrentLiabilityDescription => "OCL",
                Occupation => "OCP",
                CodeOCR => "OCR",
                OriginatorDeficiencyDescription => "ODD",
                OrdersForDisciplinesAndTreatments => "ODT",
                OriginalLegalStructure => "OLS",
                OccupationDefinition => "OPO",
                TestResultsOtherThanRoomAir => "ORA",
                OrderingRestrictions => "ORD",
                OtherRemedialAction => "ORE",
                OrderInstructions => "ORI",
                OtherInstructions => "OTH",
                AnotherTypeOfNumberDescription => "OTN",
                ReportSourceDescription => "OTS",
                Payables => "PAY",
                ProcessSpecification => "PCS",
                ProductSpecification => "PDS",
                EmployeeSharingArrangements => "PED",
                PenaltyDescription => "PEN",
                CodePES => "PES",
                PropertyImprovementDescription => "PID",
                PackagingInstructions => "PKG",
                Payment => "PMT",
                PrimaryObservation => "POB",
                PrincipalsOrOrganizationComments => "POC",
                PropertyOwnerLocation => "POL",
                PrincipalProcedureCodeDescription => "PPC",
                Priority => "PRI",
                PublicRecordNotes => "PRN",
                PreviousRegisteredOffice => "PRO",
                PriceRange => "PRR",
                ProblemSummary => "PSY",
                Purchasing => "PUR",
                Qualifications => "QUL",
                QuotationInstruction => "QUT",
                ReasonForDelinquency => "RDI",
                Recommendation => "REC",
                TreatmentRefusalNote => "REF",
                RegisteredActivity => "REG",
                Report => "REP",
                Receivables => "REV",
                ReasonForLeaving => "RFL",
                CodeRHB => "RHB",
                ReasonForLeaveOfAbsence => "RLA",
                ReasonsPatientLeavesHome => "RLH",
                TimesAndReasonsPatientNotAtHome => "RNH",
                MissingReportExplanation => "RNI",
                CircuitRoutingInstructions => "ROU",
                ReportRemarks => "RPT",
                PlaceWhereAReportWasSubmitted => "RST",
                RentVerificationComments => "RVC",
                SettlementAmountNotes => "SAN",
                OceanShippingContainer => "SCN",
                SentenceDescription => "SDD",
                CodeSET => "SET",
                SafetyMeasures => "SFM",
                ShippingRestrictions => "SHR",
                SellingMeansDescription => "SMD",
                SecondaryObservation => "SOB",
                StatementOfWork => "SOW",
                SpecialHandling => "SPH",
                SupplementaryPlanOfTreatment => "SPT",
                ClosingInstructions => "SPV",
                SchoolAttendedDetails => "SSA",
                StatusComment => "SSC",
                SalesDescription => "SSD",
                Spouse => "SSE",
                SchoolGraduatedDetails => "SSG",
                SecurityService => "SSH",
                Shareholding => "SSI",
                SigningAuthority => "SSS",
                SalesTerritory => "SST",
                TariffAbbreviation => "TAF",
                TariffCommodityFootnotes => "TCF",
                TurkishDefenseAffairsAuthorization => "TDA",
                TaskStatement => "TES",
                TariffIndex => "TIL",
                TariffRule => "TLF",
                TradelineRemarks => "TLR",
                ToxicOrganicManagementPlan => "TMP",
                ThirdPartyOrganizationNotes => "TPO",
                Transportation => "TRA",
                ReportableEventDescription => "TRE",
                TariffRateFootnotes => "TRF",
                Quality => "TRS",
                TermsOfSaleDescription => "TSD",
                TariffSectionFootnotes => "TSF",
                TestResults => "TST",
                Updated => "UPI",
                VerificationComments => "VEC",
                VehicleOutOfServiceNotice => "VEH",
                VariationNotes => "VNN",
                WarehouseInstruction => "WHI",
                WrappingInstructions => "WRP",
                ZeroDischargeCertificationStatement => "ZED",
                MutuallyDefined => "ZZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<NoteReferenceCode> {
        use NoteReferenceCode::*;
        match code {
            b"AAA" => Some(AgentDetails),
            b"AAB" => Some(AssociatedBusinessAreas),
            b"AAC" => Some(Borrower),
            b"AAD" => Some(NationalityDetails),
            b"AAE" => Some(Assets),
            b"AAF" => Some(Liabilities),
            b"AAH" => Some(AdditionalFacilityDetails),
            b"AAI" => Some(ExemptionLawLocationDescription),
            b"AAJ" => Some(ForecastDetails),
            b"AAK" => Some(ImportAndExportDetails),
            b"AAL" => Some(InventoryValuation),
            b"AAM" => Some(ProductBrandsSoldDescription),
            b"AAN" => Some(PurchaseTerritory),
            b"AAO" => Some(Responsibilities),
            b"AAP" => Some(SupplierDescription),
            b"AAQ" => Some(EducationDescription),
            b"AAR" => Some(LiquidityDetails),
            b"AAS" => Some(FormerActivityDescription),
            b"AAT" => Some(DivisionDescription),
            b"ABN" => Some(AbbreviatedNomenclature),
            b"ACC" => Some(AccessInstructions),
            b"ACI" => Some(AdditionalClaim),
            b"ACN" => Some(ActionTaken),
            b"ACS" => Some(ActualSolution),
            b"ACT" => Some(Action),
            b"ADD" => Some(Additional),
            b"AES" => Some(ActualEvaluationSummary),
            b"AET" => Some(AdverseEventTerms),
            b"AFA" => Some(Description),
            b"AFB" => Some(GenericChemicalName),
            b"AFC" => Some(PreventionProgramDescription),
            b"AFD" => Some(RiskManagementPlanDescription),
            b"AFE" => Some(SafetyComments),
            b"AFF" => Some(Summary),
            b"ALG" => Some(Allergies),
            b"ALL" => Some(AllDocuments),
            b"ALT" => Some(Alerts),
            b"AMN" => Some(AdditionalManufacturerNarrative),
            b"AOO" => Some(AreaOfOperation),
            b"APN" => Some(ApplicationNotes),
            b"APS" => Some(AppropriationSpecifications),
            b"BBD" => Some(BankDescription),
            b"BBF" => Some(BusinessFounder),
            b"BBH" => Some(BusinessHistory),
            b"BBN" => Some(BankingNotes),
            b"BBO" => Some(BusinessOriginDescription),
            b"BBT" => Some(BrandNames),
            b"BFD" => Some(BusinessFinancingDetails),
            b"BOL" => Some(BillOfLadingNote),
            b"BUR" => Some(BureauRemarks),
            b"CAA" => Some(Authentication),
            b"CAB" => Some(LineOfInStateBusiness),
            b"CAC" => Some(Relationship),
            b"CAD" => Some(BasisForAmountDue),
            b"CAE" => Some(TypeOfDebt),
            b"CAF" => Some(LandUsePurpose),
            b"CAG" => Some(LandDescription),
            b"CAH" => Some(BasisOfCalculation),
            b"CAI" => Some(GeneralBusinessDescription),
            b"CAJ" => Some(TypeOfBusiness),
            b"CAK" => Some(CharacterOfBusiness),
            b"CAL" => Some(RepresentationOfValue),
            b"CAM" => Some(CodeCAM),
            b"CAN" => Some(CooperativeCorporationStatement),
            b"CAO" => Some(CloseCorporationStatement),
            b"CAP" => Some(AgreementToAbideByLaws),
            b"CAQ" => Some(StockRestrictions),
            b"CAR" => Some(OtherRelated),
            b"CAS" => Some(ProhibitionAgainstBeingAnOfficer),
            b"CAT" => Some(QualificationOfDirector),
            b"CAU" => Some(NatureOfCharter),
            b"CAV" => Some(StatementOfAssetsAndLiabilities),
            b"CAW" => Some(Bankruptcy),
            b"CAX" => Some(CertificateOfDisclosure),
            b"CAZ" => Some(AssetDetail),
            b"CBA" => Some(StatementRelatedToRegulation),
            b"CBB" => Some(ConsiderationToBeReceived),
            b"CBC" => Some(OtherLawfulProvisions),
            b"CBH" => Some(MonetaryAmountDescription),
            b"CBI" => Some(DescriptionOfTitle),
            b"CCA" => Some(Competition),
            b"CCB" => Some(ConstructionDetails),
            b"CCC" => Some(ConstructionFinancing),
            b"CCD" => Some(ConstructionLineOfBusiness),
            b"CCE" => Some(ContractDetails),
            b"CCF" => Some(CorporateFilingDetails),
            b"CCG" => Some(CustomerDescription),
            b"CCN" => Some(CopyrightNotice),
            b"CDD" => Some(ContingentDebtDetails),
            b"CER" => Some(CertificationNarrative),
            b"CHG" => Some(Change),
            b"CIG" => Some(Cigarette),
            b"CIR" => Some(CircumstancesPriorToDifficulty),
            b"CLN" => Some(Classifying),
            b"CLR" => Some(SecurityClearanceInstructions),
            b"CMP" => Some(ConcomitantMedicalProductDescription),
            b"CMT" => Some(MaintenanceComment),
            b"COD" => Some(CorrectedData),
            b"COM" => Some(ConsumerComments),
            b"CON" => Some(ConvictionActDetails),
            b"CRA" => Some(CreditReportAlerts),
            b"CRK" => Some(ClosingComment),
            b"CRN" => Some(CreditReportNotes),
            b"CUS" => Some(CustomsDeclaration),
            b"DCP" => Some(CodeDCP),
            b"DCS" => Some(DestinationControlStatement),
            b"DDC" => Some(DeficiencyDescriptionChanges),
            b"DEE" => Some(EventDescription),
            b"DEL" => Some(Delivery),
            b"DEP" => Some(ProblemDescription),
            b"DFR" => Some(CodeDFR),
            b"DFS" => Some(DepartureFromSpecificationComment),
            b"DGN" => Some(DiagnosisDescription),
            b"DME" => Some(CodeDME),
            b"DOD" => Some(DescriptionOfDamage),
            b"DOI" => Some(OutcomeDescription),
            b"DRV" => Some(DriverOutOfServiceNotice),
            b"DRW" => Some(DriverOutOfServiceResolution),
            b"DSW" => Some(DetailedStatementOfWork),
            b"EAA" => Some(OtherTypeOfGroup),
            b"EAB" => Some(BallotMeasure),
            b"EAC" => Some(Attachment),
            b"EAD" => Some(Board),
            b"EAE" => Some(ProhibitedContributionCircumstance),
            b"EAF" => Some(CommitteeActivity),
            b"EAG" => Some(CompensationArrangement),
            b"EAH" => Some(CountrySubEntity),
            b"EAI" => Some(Faction),
            b"EAJ" => Some(Gift),
            b"EAK" => Some(InKindContributionUse),
            b"EAL" => Some(IndustryGroup),
            b"EAM" => Some(Jurisdiction),
            b"EAN" => Some(NatureAndPurposeOfOtherLobbyistEmployers),
            b"EAO" => Some(DescriptionOfOffice),
            b"EAP" => Some(PartyConsideringLegislation),
            b"EAQ" => Some(DescriptionOfPosition),
            b"EAR" => Some(DescriptionOfSponsor),
            b"EAS" => Some(Affiliation),
            b"EAT" => Some(AssetDisposition),
            b"EAV" => Some(CommitteeInterest),
            b"EAW" => Some(CompensationOrServicesExchangedForConsideration),
            b"EAX" => Some(ContributorInterest),
            b"EAY" => Some(DescriptionOfDebt),
            b"EAZ" => Some(EmployerDescription),
            b"EBA" => Some(PurposeOfEmployment),
            b"EBB" => Some(DescriptionOfAgencyAndPosition),
            b"EBC" => Some(DescriptionOfGoodsAndServices),
            b"EBD" => Some(LengthOfLobbyingActions),
            b"EBE" => Some(LobbyingInterest),
            b"EBF" => Some(MethodOfDisposal),
            b"EBG" => Some(PurposeOfCredit),
            b"EBH" => Some(DescriptionOfOtherBusinessSubCategory),
            b"EBI" => Some(OtherTypeOfCompensation),
            b"EBJ" => Some(DescriptionOfOtherLegislativeInterest),
            b"EBK" => Some(OtherReasonForWithdrawal),
            b"EBL" => Some(OtherTypeOfRelationship),
            b"EBM" => Some(OtherTemporaryResidence),
            b"EBN" => Some(OtherTypeOfCommittee),
            b"EBO" => Some(PlacePaid),
            b"EBP" => Some(Proposition),
            b"EBQ" => Some(ReasonForPurchase),
            b"EBR" => Some(ReasonForContribution),
            b"EBS" => Some(DescriptionOfRepaymentSchedule),
            b"EBU" => Some(ServiceDescription),
            b"EBV" => Some(Initiative),
            b"EBW" => Some(DescriptionOfAmendment),
            b"EBX" => Some(TypeOfElection),
            b"EBY" => Some(OtherTypeOfAccount),
            b"EBZ" => Some(InterestRateDescription),
            b"ECA" => Some(InKindContribution),
            b"ECB" => Some(ReasonForRefund),
            b"ECC" => Some(IncidentalExpenses),
            b"ECD" => Some(EnvironmentalConditionsDescription),
            b"ECE" => Some(OtherExpenses),
            b"ECF" => Some(OtherIncome),
            b"ECG" => Some(DescriptionOfReceipt),
            b"ECH" => Some(SurplusFunds),
            b"ECI" => Some(Collateral),
            b"ECJ" => Some(Contributor),
            b"ECK" => Some(MiscellaneousReceiptTransaction),
            b"ECL" => Some(OtherAdvertising),
            b"ECM" => Some(EstimateComment),
            b"ECN" => Some(EquipmentConditionDescription),
            b"ECO" => Some(OtherNecessaryPersonalExpense),
            b"ECP" => Some(OtherCampaignExpense),
            b"ECQ" => Some(LongTermLiability),
            b"ECR" => Some(ShortTermLiability),
            b"ECS" => Some(OtherSponsorExpense),
            b"ECT" => Some(EmergencyCertification),
            b"ECU" => Some(OtherTransportationExpense),
            b"ECV" => Some(Refund),
            b"ECW" => Some(ReasonForReturn),
            b"EED" => Some(EquipmentDescription),
            b"EFD" => Some(EquipmentFunctionDescription),
            b"ELE" => Some(EquipmentLogEntry),
            b"EMC" => Some(EmploymentComments),
            b"EMD" => Some(EstimateMethodDescription),
            b"ENR" => Some(ExplanationForNonReturnOfDeviceToManufacturer),
            b"ERN" => Some(ErrorNotes),
            b"EVL" => Some(EventLocation),
            b"EXE" => Some(ExemptionDescription),
            b"EXN" => Some(ExhibitNotes),
            b"EXR" => Some(ExerciseRoutine),
            b"EXT" => Some(ExteriorDescription),
            b"FDD" => Some(FinalDeficiencyDescription),
            b"FEE" => Some(FeeDescription),
            b"FIX" => Some(RepairSummary),
            b"FUT" => Some(FuturePlans),
            b"GEN" => Some(EntireTransactionSet),
            b"GPI" => Some(GeneralProductOrProcess),
            b"GPL" => Some(GeneralPolicy),
            b"GSI" => Some(GeneralSpecification),
            b"HHI" => Some(HouseholdGoods),
            b"ICN" => Some(IntervieweeConversation),
            b"IDT" => Some(IntangibleDescription),
            b"IID" => Some(CodeIID),
            b"IIE" => Some(InvestmentDescription),
            b"IIR" => Some(IntercompanyRelations),
            b"IMP" => Some(ProblemImpact),
            b"INP" => Some(ReasonForInspection),
            b"INS" => Some(Insurance),
            b"INT" => Some(GeneralOrderInstructions),
            b"INV" => Some(InvoiceInstruction),
            b"IVC" => Some(IncomeVerificationComments),
            b"JVD" => Some(JointVentureDescription),
            b"LAB" => Some(LabelingInstructions),
            b"LBD" => Some(LaboratoryData),
            b"LBS" => Some(LabeledStrength),
            b"LEN" => Some(LenderUse),
            b"LIN" => Some(LineItem),
            b"LIQ" => Some(Liquor),
            b"LLA" => Some(LettersOfLiabilityAgreements),
            b"LLB" => Some(LoanDetails),
            b"LLC" => Some(LongTermDebtDescription),
            b"LOC" => Some(LocationDescription),
            b"LOI" => Some(LoadingInstructions),
            b"LSD" => Some(LegalStructureDetails),
            b"MCD" => Some(MaritalContractDetails),
            b"MCN" => Some(MotorCarrierInstructions),
            b"MDO" => Some(DeviceOperatorDescription),
            b"MED" => Some(Medications),
            b"MFG" => Some(ManufacturingInstructions),
            b"MKN" => Some(MarketingNotes),
            b"MMD" => Some(MergerDescription),
            b"MSD" => Some(MarketableSecuritiesDescription),
            b"NCD" => Some(NonconformanceSpecification),
            b"NPD" => Some(NameplateData),
            b"NTR" => Some(NutritionalRequirements),
            b"OBI" => Some(OriginatorToBeneficiaryInstructions),
            b"OBL" => Some(ObligationDescription),
            b"OCA" => Some(OtherCurrentAssetDescription),
            b"OCC" => Some(Occupancy),
            b"OCL" => Some(OtherCurrentLiabilityDescription),
            b"OCP" => Some(Occupation),
            b"OCR" => Some(CodeOCR),
            b"ODD" => Some(OriginatorDeficiencyDescription),
            b"ODT" => Some(OrdersForDisciplinesAndTreatments),
            b"OLS" => Some(OriginalLegalStructure),
            b"OPO" => Some(OccupationDefinition),
            b"ORA" => Some(TestResultsOtherThanRoomAir),
            b"ORD" => Some(OrderingRestrictions),
            b"ORE" => Some(OtherRemedialAction),
            b"ORI" => Some(OrderInstructions),
            b"OTH" => Some(OtherInstructions),
            b"OTN" => Some(AnotherTypeOfNumberDescription),
            b"OTS" => Some(ReportSourceDescription),
            b"PAY" => Some(Payables),
            b"PCS" => Some(ProcessSpecification),
            b"PDS" => Some(ProductSpecification),
            b"PED" => Some(EmployeeSharingArrangements),
            b"PEN" => Some(PenaltyDescription),
            b"PES" => Some(CodePES),
            b"PID" => Some(PropertyImprovementDescription),
            b"PKG" => Some(PackagingInstructions),
            b"PMT" => Some(Payment),
            b"POB" => Some(PrimaryObservation),
            b"POC" => Some(PrincipalsOrOrganizationComments),
            b"POL" => Some(PropertyOwnerLocation),
            b"PPC" => Some(PrincipalProcedureCodeDescription),
            b"PRI" => Some(Priority),
            b"PRN" => Some(PublicRecordNotes),
            b"PRO" => Some(PreviousRegisteredOffice),
            b"PRR" => Some(PriceRange),
            b"PSY" => Some(ProblemSummary),
            b"PUR" => Some(Purchasing),
            b"QUL" => Some(Qualifications),
            b"QUT" => Some(QuotationInstruction),
            b"RDI" => Some(ReasonForDelinquency),
            b"REC" => Some(Recommendation),
            b"REF" => Some(TreatmentRefusalNote),
            b"REG" => Some(RegisteredActivity),
            b"REP" => Some(Report),
            b"REV" => Some(Receivables),
            b"RFL" => Some(ReasonForLeaving),
            b"RHB" => Some(CodeRHB),
            b"RLA" => Some(ReasonForLeaveOfAbsence),
            b"RLH" => Some(ReasonsPatientLeavesHome),
            b"RNH" => Some(TimesAndReasonsPatientNotAtHome),
            b"RNI" => Some(MissingReportExplanation),
            b"ROU" => Some(CircuitRoutingInstructions),
            b"RPT" => Some(ReportRemarks),
            b"RST" => Some(PlaceWhereAReportWasSubmitted),
            b"RVC" => Some(RentVerificationComments),
            b"SAN" => Some(SettlementAmountNotes),
            b"SCN" => Some(OceanShippingContainer),
            b"SDD" => Some(SentenceDescription),
            b"SET" => Some(CodeSET),
            b"SFM" => Some(SafetyMeasures),
            b"SHR" => Some(ShippingRestrictions),
            b"SMD" => Some(SellingMeansDescription),
            b"SOB" => Some(SecondaryObservation),
            b"SOW" => Some(StatementOfWork),
            b"SPH" => Some(SpecialHandling),
            b"SPT" => Some(SupplementaryPlanOfTreatment),
            b"SPV" => Some(ClosingInstructions),
            b"SSA" => Some(SchoolAttendedDetails),
            b"SSC" => Some(StatusComment),
            b"SSD" => Some(SalesDescription),
            b"SSE" => Some(Spouse),
            b"SSG" => Some(SchoolGraduatedDetails),
            b"SSH" => Some(SecurityService),
            b"SSI" => Some(Shareholding),
            b"SSS" => Some(SigningAuthority),
            b"SST" => Some(SalesTerritory),
            b"TAF" => Some(TariffAbbreviation),
            b"TCF" => Some(TariffCommodityFootnotes),
            b"TDA" => Some(TurkishDefenseAffairsAuthorization),
            b"TES" => Some(TaskStatement),
            b"TIL" => Some(TariffIndex),
            b"TLF" => Some(TariffRule),
            b"TLR" => Some(TradelineRemarks),
            b"TMP" => Some(ToxicOrganicManagementPlan),
            b"TPO" => Some(ThirdPartyOrganizationNotes),
            b"TRA" => Some(Transportation),
            b"TRE" => Some(ReportableEventDescription),
            b"TRF" => Some(TariffRateFootnotes),
            b"TRS" => Some(Quality),
            b"TSD" => Some(TermsOfSaleDescription),
            b"TSF" => Some(TariffSectionFootnotes),
            b"TST" => Some(TestResults),
            b"UPI" => Some(Updated),
            b"VEC" => Some(VerificationComments),
            b"VEH" => Some(VehicleOutOfServiceNotice),
            b"VNN" => Some(VariationNotes),
            b"WHI" => Some(WarehouseInstruction),
            b"WRP" => Some(WrappingInstructions),
            b"ZED" => Some(ZeroDischargeCertificationStatement),
            b"ZZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use NoteReferenceCode::*;
        match self {
            AgentDetails => "Agent Details",
            AssociatedBusinessAreas => "Associated Business Areas",
            Borrower => "Borrower",
            NationalityDetails => "Nationality Details",
            Assets => "Assets",
            Liabilities => "Liabilities",
            AdditionalFacilityDetails => "Additional Facility Details",
            ExemptionLawLocationDescription => "Exemption Law Location Description",
            ForecastDetails => "Forecast Details",
            ImportAndExportDetails => "Import and Export Details",
            InventoryValuation => "Inventory Valuation",
            ProductBrandsSoldDescription => "Product Brands Sold Description",
            PurchaseTerritory => "Purchase Territory",
            Responsibilities => "Responsibilities",
            SupplierDescription => "Supplier Description",
            EducationDescription => "Education Description",
            LiquidityDetails => "Liquidity Details",
            FormerActivityDescription => "Former Activity Description",
            DivisionDescription => "Division Description",
            AbbreviatedNomenclature => "Abbreviated Nomenclature",
            AccessInstructions => "Access Instructions",
            AdditionalClaim => "Additional Claim Information",
            ActionTaken => "Action Taken",
            ActualSolution => "Actual Solution",
            Action => "Action",
            Additional => "Additional Information",
            ActualEvaluationSummary => "Actual Evaluation Summary",
            AdverseEventTerms => "Adverse Event Terms",
            Description => "Description",
            GenericChemicalName => "Generic Chemical Name",
            PreventionProgramDescription => "Prevention Program Description",
            RiskManagementPlanDescription => "Risk Management Plan Description",
            SafetyComments => "Safety Comments",
            Summary => "Summary",
            Allergies => "Allergies",
            AllDocuments => "All Documents",
            Alerts => "Alerts",
            AdditionalManufacturerNarrative => "Additional Manufacturer Narrative",
            AreaOfOperation => "Area of Operation",
            ApplicationNotes => "Application Notes",
            AppropriationSpecifications => "Appropriation Specifications",
            BankDescription => "Bank Description",
            BusinessFounder => "Business Founder",
            BusinessHistory => "Business History",
            BankingNotes => "Banking Notes",
            BusinessOriginDescription => "Business Origin Description",
            BrandNames => "Brand Names",
            BusinessFinancingDetails => "Business Financing Details",
            BillOfLadingNote => "Bill of Lading Note",
            BureauRemarks => "Bureau Remarks",
            Authentication => "Authentication Information",
            LineOfInStateBusiness => "Line of In-State Business",
            Relationship => "Relationship Information",
            BasisForAmountDue => "Basis for Amount Due",
            TypeOfDebt => "Type of Debt",
            LandUsePurpose => "Land Use Purpose",
            LandDescription => "Land Description",
            BasisOfCalculation => "Basis of Calculation",
            GeneralBusinessDescription => "General Business Description",
            TypeOfBusiness => "Type of Business",
            CharacterOfBusiness => "Character of Business",
            RepresentationOfValue => "Representation of Value",
            CodeCAM => "Supporting Statement, Tax, and Fee Computation",
            CooperativeCorporationStatement => "Cooperative Corporation Statement",
            CloseCorporationStatement => "Close Corporation Statement",
            AgreementToAbideByLaws => "Agreement to Abide by Laws",
            StockRestrictions => "Stock Restrictions",
            OtherRelated => "Other Related Information",
            ProhibitionAgainstBeingAnOfficer => "Prohibition Against Being an Officer",
            QualificationOfDirector => "Qualification of Director",
            NatureOfCharter => "Nature of Charter",
            StatementOfAssetsAndLiabilities => "Statement of Assets and Liabilities",
            Bankruptcy => "Bankruptcy Information",
            CertificateOfDisclosure => "Certificate of Disclosure",
            AssetDetail => "Asset Detail",
            StatementRelatedToRegulation => "Statement Related to Regulation",
            ConsiderationToBeReceived => "Consideration to be Received",
            OtherLawfulProvisions => "Other Lawful Provisions",
            MonetaryAmountDescription => "Monetary Amount Description",
            DescriptionOfTitle => "Description of Title",
            Competition => "Competition",
            ConstructionDetails => "Construction Details",
            ConstructionFinancing => "Construction Financing",
            ConstructionLineOfBusiness => "Construction Line of Business",
            ContractDetails => "Contract Details",
            CorporateFilingDetails => "Corporate Filing Details",
            CustomerDescription => "Customer Description",
            CopyrightNotice => "Copyright Notice",
            ContingentDebtDetails => "Contingent Debt Details",
            CertificationNarrative => "Certification Narrative",
            Change => "Change",
            Cigarette => "Cigarette Information",
            CircumstancesPriorToDifficulty => "Circumstances Prior to Difficulty",
            Classifying => "Classifying Information",
            SecurityClearanceInstructions => "Security Clearance Instructions",
            ConcomitantMedicalProductDescription => {
                "Concomitant Medical Product Description"
            }
            MaintenanceComment => "Maintenance Comment",
            CorrectedData => "Corrected Data",
            ConsumerComments => "Consumer Comments",
            ConvictionActDetails => "Conviction Act Details",
            CreditReportAlerts => "Credit Report Alerts",
            ClosingComment => "Closing Comment",
            CreditReportNotes => "Credit Report Notes",
            CustomsDeclaration => "Customs declaration",
            CodeDCP => "Goals, Rehabilitation Potential, or Discharge Plans",
            DestinationControlStatement => "Destination Control Statement",
            DeficiencyDescriptionChanges => "Deficiency Description Changes",
            EventDescription => "Event Description",
            Delivery => "Delivery",
            ProblemDescription => "Problem Description",
            CodeDFR => "Dose, Frequency and Route Description",
            DepartureFromSpecificationComment => "Departure from Specification Comment",
            DiagnosisDescription => "Diagnosis Description",
            CodeDME => "Durable Medical Equipment (DME) and Supplies",
            DescriptionOfDamage => "Description of Damage",
            OutcomeDescription => "Outcome Description",
            DriverOutOfServiceNotice => "Driver Out of Service Notice",
            DriverOutOfServiceResolution => "Driver Out of Service Resolution",
            DetailedStatementOfWork => "Detailed Statement of Work",
            OtherTypeOfGroup => "Other Type of Group",
            BallotMeasure => "Ballot Measure",
            Attachment => "Attachment",
            Board => "Board",
            ProhibitedContributionCircumstance => "Prohibited Contribution Circumstance",
            CommitteeActivity => "Committee Activity",
            CompensationArrangement => "Compensation Arrangement",
            CountrySubEntity => "Country Sub-Entity",
            Faction => "Faction",
            Gift => "Gift",
            InKindContributionUse => "In-Kind Contribution Use",
            IndustryGroup => "Industry Group",
            Jurisdiction => "Jurisdiction",
            NatureAndPurposeOfOtherLobbyistEmployers => {
                "Nature and Purpose of Other Lobbyist Employers"
            }
            DescriptionOfOffice => "Description of Office",
            PartyConsideringLegislation => "Party Considering Legislation",
            DescriptionOfPosition => "Description of Position",
            DescriptionOfSponsor => "Description of Sponsor",
            Affiliation => "Affiliation",
            AssetDisposition => "Asset Disposition",
            CommitteeInterest => "Committee Interest",
            CompensationOrServicesExchangedForConsideration => {
                "Compensation or Services Exchanged for Consideration"
            }
            ContributorInterest => "Contributor Interest",
            DescriptionOfDebt => "Description of Debt",
            EmployerDescription => "Employer Description",
            PurposeOfEmployment => "Purpose of Employment",
            DescriptionOfAgencyAndPosition => "Description of Agency and Position",
            DescriptionOfGoodsAndServices => "Description of Goods and Services",
            LengthOfLobbyingActions => "Length of Lobbying Actions",
            LobbyingInterest => "Lobbying Interest",
            MethodOfDisposal => "Method of Disposal",
            PurposeOfCredit => "Purpose of Credit",
            DescriptionOfOtherBusinessSubCategory => {
                "Description of Other Business Sub-Category"
            }
            OtherTypeOfCompensation => "Other Type of Compensation",
            DescriptionOfOtherLegislativeInterest => {
                "Description of Other Legislative Interest"
            }
            OtherReasonForWithdrawal => "Other Reason for Withdrawal",
            OtherTypeOfRelationship => "Other Type of Relationship",
            OtherTemporaryResidence => "Other Temporary Residence",
            OtherTypeOfCommittee => "Other Type of Committee",
            PlacePaid => "Place Paid",
            Proposition => "Proposition",
            ReasonForPurchase => "Reason for Purchase",
            ReasonForContribution => "Reason for Contribution",
            DescriptionOfRepaymentSchedule => "Description of Repayment Schedule",
            ServiceDescription => "Service Description",
            Initiative => "Initiative",
            DescriptionOfAmendment => "Description of Amendment",
            TypeOfElection => "Type of Election",
            OtherTypeOfAccount => "Other Type of Account",
            InterestRateDescription => "Interest Rate Description",
            InKindContribution => "In-Kind Contribution",
            ReasonForRefund => "Reason for Refund",
            IncidentalExpenses => "Incidental Expenses",
            EnvironmentalConditionsDescription => "Environmental Conditions Description",
            OtherExpenses => "Other Expenses",
            OtherIncome => "Other Income",
            DescriptionOfReceipt => "Description of Receipt",
            SurplusFunds => "Surplus Funds",
            Collateral => "Collateral",
            Contributor => "Contributor",
            MiscellaneousReceiptTransaction => "Miscellaneous Receipt Transaction",
            OtherAdvertising => "Other Advertising",
            EstimateComment => "Estimate Comment",
            EquipmentConditionDescription => "Equipment Condition Description",
            OtherNecessaryPersonalExpense => "Other Necessary Personal Expense",
            OtherCampaignExpense => "Other Campaign Expense",
            LongTermLiability => "Long-Term Liability",
            ShortTermLiability => "Short-Term Liability",
            OtherSponsorExpense => "Other Sponsor Expense",
            EmergencyCertification => "Emergency Certification",
            OtherTransportationExpense => "Other Transportation Expense",
            Refund => "Refund",
            ReasonForReturn => "Reason for Return",
            EquipmentDescription => "Equipment Description",
            EquipmentFunctionDescription => "Equipment Function Description",
            EquipmentLogEntry => "Equipment Log Entry",
            EmploymentComments => "Employment Comments",
            EstimateMethodDescription => "Estimate Method Description",
            ExplanationForNonReturnOfDeviceToManufacturer => {
                "Explanation for Non-Return of Device to Manufacturer"
            }
            ErrorNotes => "Error Notes",
            EventLocation => "Event Location",
            ExemptionDescription => "Exemption Description",
            ExhibitNotes => "Exhibit Notes",
            ExerciseRoutine => "Exercise Routine",
            ExteriorDescription => "Exterior Description",
            FinalDeficiencyDescription => "Final Deficiency Description",
            FeeDescription => "Fee Description",
            RepairSummary => "Repair Summary",
            FuturePlans => "Future Plans",
            EntireTransactionSet => "Entire Transaction Set",
            GeneralProductOrProcess => "General Product or Process Information",
            GeneralPolicy => "General Policy",
            GeneralSpecification => "General Specification Information",
            HouseholdGoods => "Household Goods Information",
            IntervieweeConversation => "Interviewee Conversation",
            IntangibleDescription => "Intangible Description",
            CodeIID => "Inventory (Stock) Description",
            InvestmentDescription => "Investment Description",
            IntercompanyRelations => "Intercompany Relations",
            ProblemImpact => "Problem Impact",
            ReasonForInspection => "Reason for Inspection",
            Insurance => "Insurance",
            GeneralOrderInstructions => "General Order Instructions",
            InvoiceInstruction => "Invoice Instruction",
            IncomeVerificationComments => "Income Verification Comments",
            JointVentureDescription => "Joint Venture Description",
            LabelingInstructions => "Labeling Instructions",
            LaboratoryData => "Laboratory Data",
            LabeledStrength => "Labeled Strength",
            LenderUse => "Lender Use",
            LineItem => "Line Item",
            Liquor => "Liquor Information",
            LettersOfLiabilityAgreements => "Letters of Liability Agreements",
            LoanDetails => "Loan Details",
            LongTermDebtDescription => "Long Term Debt Description",
            LocationDescription => "Location Description Information",
            LoadingInstructions => "Loading Instructions",
            LegalStructureDetails => "Legal Structure Details",
            MaritalContractDetails => "Marital Contract Details",
            MotorCarrierInstructions => "Motor Carrier Instructions",
            DeviceOperatorDescription => "Device Operator Description",
            Medications => "Medications",
            ManufacturingInstructions => "Manufacturing Instructions",
            MarketingNotes => "Marketing Notes",
            MergerDescription => "Merger Description",
            MarketableSecuritiesDescription => "Marketable Securities Description",
            NonconformanceSpecification => "Nonconformance Specification",
            NameplateData => "Nameplate Data",
            NutritionalRequirements => "Nutritional Requirements",
            OriginatorToBeneficiaryInstructions => {
                "Originator to Beneficiary Instructions"
            }
            ObligationDescription => "Obligation Description",
            OtherCurrentAssetDescription => "Other Current Asset Description",
            Occupancy => "Occupancy Information",
            OtherCurrentLiabilityDescription => "Other Current Liability Description",
            Occupation => "Occupation",
            CodeOCR => "Outside the Continental U.S. (OCONUS) Rating Information",
            OriginatorDeficiencyDescription => "Originator Deficiency Description",
            OrdersForDisciplinesAndTreatments => "Orders for Disciplines and Treatments",
            OriginalLegalStructure => "Original Legal Structure",
            OccupationDefinition => "Occupation Definition",
            TestResultsOtherThanRoomAir => "Test Results Other Than Room Air",
            OrderingRestrictions => "Ordering Restrictions",
            OtherRemedialAction => "Other Remedial Action",
            OrderInstructions => "Order Instructions",
            OtherInstructions => "Other Instructions",
            AnotherTypeOfNumberDescription => "Another Type of Number Description",
            ReportSourceDescription => "Report Source Description",
            Payables => "Payables",
            ProcessSpecification => "Process Specification",
            ProductSpecification => "Product Specification",
            EmployeeSharingArrangements => "Employee Sharing Arrangements",
            PenaltyDescription => "Penalty Description",
            CodePES => {
                "Partial Pressure of Oxygen (PO2) is 60 millimeters (MM) of Mercury (Hg) or above, or arterial blood oxygen saturation is 90% or above"
            }
            PropertyImprovementDescription => "Property Improvement Description",
            PackagingInstructions => "Packaging Instructions",
            Payment => "Payment",
            PrimaryObservation => "Primary Observation",
            PrincipalsOrOrganizationComments => "Principals or Organization Comments",
            PropertyOwnerLocation => "Property Owner Location Information",
            PrincipalProcedureCodeDescription => "Principal Procedure Code Description",
            Priority => "Priority",
            PublicRecordNotes => "Public Record Notes",
            PreviousRegisteredOffice => "Previous Registered Office",
            PriceRange => "Price Range",
            ProblemSummary => "Problem Summary",
            Purchasing => "Purchasing",
            Qualifications => "Qualifications",
            QuotationInstruction => "Quotation Instruction",
            ReasonForDelinquency => "Reason for Delinquency Information",
            Recommendation => "Recommendation",
            TreatmentRefusalNote => "Treatment Refusal Note",
            RegisteredActivity => "Registered Activity",
            Report => "Report",
            Receivables => "Receivables",
            ReasonForLeaving => "Reason for Leaving",
            CodeRHB => "Functional Limitations, Reason Homebound, or Both",
            ReasonForLeaveOfAbsence => "Reason for Leave of Absence",
            ReasonsPatientLeavesHome => "Reasons Patient Leaves Home",
            TimesAndReasonsPatientNotAtHome => "Times and Reasons Patient Not at Home",
            MissingReportExplanation => "Missing Report Explanation",
            CircuitRoutingInstructions => "Circuit Routing Instructions",
            ReportRemarks => "Report Remarks",
            PlaceWhereAReportWasSubmitted => "Place Where a Report was Submitted",
            RentVerificationComments => "Rent Verification Comments",
            SettlementAmountNotes => "Settlement Amount Notes",
            OceanShippingContainer => "Ocean Shipping Container Information",
            SentenceDescription => "Sentence Description",
            CodeSET => "Unusual Home, Social Environment, or Both",
            SafetyMeasures => "Safety Measures",
            ShippingRestrictions => "Shipping Restrictions",
            SellingMeansDescription => "Selling Means Description",
            SecondaryObservation => "Secondary Observation",
            StatementOfWork => "Statement of Work",
            SpecialHandling => "Special Handling",
            SupplementaryPlanOfTreatment => "Supplementary Plan of Treatment",
            ClosingInstructions => "Closing Instructions",
            SchoolAttendedDetails => "School Attended Details",
            StatusComment => "Status Comment",
            SalesDescription => "Sales Description",
            Spouse => "Spouse Information",
            SchoolGraduatedDetails => "School Graduated Details",
            SecurityService => "Security Service Information",
            Shareholding => "Shareholding Information",
            SigningAuthority => "Signing Authority",
            SalesTerritory => "Sales Territory",
            TariffAbbreviation => "Tariff Abbreviation",
            TariffCommodityFootnotes => "Tariff Commodity Footnotes",
            TurkishDefenseAffairsAuthorization => {
                "Turkish Defense Affairs Authorization Information"
            }
            TaskStatement => "Task Statement",
            TariffIndex => "Tariff Index",
            TariffRule => "Tariff Rule",
            TradelineRemarks => "Tradeline Remarks",
            ToxicOrganicManagementPlan => "Toxic Organic Management Plan",
            ThirdPartyOrganizationNotes => "Third Party Organization Notes",
            Transportation => "Transportation",
            ReportableEventDescription => "Reportable Event Description",
            TariffRateFootnotes => "Tariff Rate Footnotes",
            Quality => "Quality Information",
            TermsOfSaleDescription => "Terms of Sale Description",
            TariffSectionFootnotes => "Tariff Section Footnotes",
            TestResults => "Test Results",
            Updated => "Updated Information",
            VerificationComments => "Verification Comments",
            VehicleOutOfServiceNotice => "Vehicle Out of Service Notice",
            VariationNotes => "Variation Notes",
            WarehouseInstruction => "Warehouse Instruction",
            WrappingInstructions => "Wrapping Instructions",
            ZeroDischargeCertificationStatement => {
                "Zero Discharge Certification Statement"
            }
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<NoteReferenceCode> {
        {
            use NoteReferenceCode::*;
            match description {
                "Agent Details" => Some(AgentDetails),
                "Associated Business Areas" => Some(AssociatedBusinessAreas),
                "Borrower" => Some(Borrower),
                "Nationality Details" => Some(NationalityDetails),
                "Assets" => Some(Assets),
                "Liabilities" => Some(Liabilities),
                "Additional Facility Details" => Some(AdditionalFacilityDetails),
                "Exemption Law Location Description" => {
                    Some(ExemptionLawLocationDescription)
                }
                "Forecast Details" => Some(ForecastDetails),
                "Import and Export Details" => Some(ImportAndExportDetails),
                "Inventory Valuation" => Some(InventoryValuation),
                "Product Brands Sold Description" => Some(ProductBrandsSoldDescription),
                "Purchase Territory" => Some(PurchaseTerritory),
                "Responsibilities" => Some(Responsibilities),
                "Supplier Description" => Some(SupplierDescription),
                "Education Description" => Some(EducationDescription),
                "Liquidity Details" => Some(LiquidityDetails),
                "Former Activity Description" => Some(FormerActivityDescription),
                "Division Description" => Some(DivisionDescription),
                "Abbreviated Nomenclature" => Some(AbbreviatedNomenclature),
                "Access Instructions" => Some(AccessInstructions),
                "Additional Claim Information" => Some(AdditionalClaim),
                "Action Taken" => Some(ActionTaken),
                "Actual Solution" => Some(ActualSolution),
                "Action" => Some(Action),
                "Additional Information" => Some(Additional),
                "Actual Evaluation Summary" => Some(ActualEvaluationSummary),
                "Adverse Event Terms" => Some(AdverseEventTerms),
                "Description" => Some(Description),
                "Generic Chemical Name" => Some(GenericChemicalName),
                "Prevention Program Description" => Some(PreventionProgramDescription),
                "Risk Management Plan Description" => Some(RiskManagementPlanDescription),
                "Safety Comments" => Some(SafetyComments),
                "Summary" => Some(Summary),
                "Allergies" => Some(Allergies),
                "All Documents" => Some(AllDocuments),
                "Alerts" => Some(Alerts),
                "Additional Manufacturer Narrative" => {
                    Some(AdditionalManufacturerNarrative)
                }
                "Area of Operation" => Some(AreaOfOperation),
                "Application Notes" => Some(ApplicationNotes),
                "Appropriation Specifications" => Some(AppropriationSpecifications),
                "Bank Description" => Some(BankDescription),
                "Business Founder" => Some(BusinessFounder),
                "Business History" => Some(BusinessHistory),
                "Banking Notes" => Some(BankingNotes),
                "Business Origin Description" => Some(BusinessOriginDescription),
                "Brand Names" => Some(BrandNames),
                "Business Financing Details" => Some(BusinessFinancingDetails),
                "Bill of Lading Note" => Some(BillOfLadingNote),
                "Bureau Remarks" => Some(BureauRemarks),
                "Authentication Information" => Some(Authentication),
                "Line of In-State Business" => Some(LineOfInStateBusiness),
                "Relationship Information" => Some(Relationship),
                "Basis for Amount Due" => Some(BasisForAmountDue),
                "Type of Debt" => Some(TypeOfDebt),
                "Land Use Purpose" => Some(LandUsePurpose),
                "Land Description" => Some(LandDescription),
                "Basis of Calculation" => Some(BasisOfCalculation),
                "General Business Description" => Some(GeneralBusinessDescription),
                "Type of Business" => Some(TypeOfBusiness),
                "Character of Business" => Some(CharacterOfBusiness),
                "Representation of Value" => Some(RepresentationOfValue),
                "Supporting Statement, Tax, and Fee Computation" => Some(CodeCAM),
                "Cooperative Corporation Statement" => {
                    Some(CooperativeCorporationStatement)
                }
                "Close Corporation Statement" => Some(CloseCorporationStatement),
                "Agreement to Abide by Laws" => Some(AgreementToAbideByLaws),
                "Stock Restrictions" => Some(StockRestrictions),
                "Other Related Information" => Some(OtherRelated),
                "Prohibition Against Being an Officer" => {
                    Some(ProhibitionAgainstBeingAnOfficer)
                }
                "Qualification of Director" => Some(QualificationOfDirector),
                "Nature of Charter" => Some(NatureOfCharter),
                "Statement of Assets and Liabilities" => {
                    Some(StatementOfAssetsAndLiabilities)
                }
                "Bankruptcy Information" => Some(Bankruptcy),
                "Certificate of Disclosure" => Some(CertificateOfDisclosure),
                "Asset Detail" => Some(AssetDetail),
                "Statement Related to Regulation" => Some(StatementRelatedToRegulation),
                "Consideration to be Received" => Some(ConsiderationToBeReceived),
                "Other Lawful Provisions" => Some(OtherLawfulProvisions),
                "Monetary Amount Description" => Some(MonetaryAmountDescription),
                "Description of Title" => Some(DescriptionOfTitle),
                "Competition" => Some(Competition),
                "Construction Details" => Some(ConstructionDetails),
                "Construction Financing" => Some(ConstructionFinancing),
                "Construction Line of Business" => Some(ConstructionLineOfBusiness),
                "Contract Details" => Some(ContractDetails),
                "Corporate Filing Details" => Some(CorporateFilingDetails),
                "Customer Description" => Some(CustomerDescription),
                "Copyright Notice" => Some(CopyrightNotice),
                "Contingent Debt Details" => Some(ContingentDebtDetails),
                "Certification Narrative" => Some(CertificationNarrative),
                "Change" => Some(Change),
                "Cigarette Information" => Some(Cigarette),
                "Circumstances Prior to Difficulty" => {
                    Some(CircumstancesPriorToDifficulty)
                }
                "Classifying Information" => Some(Classifying),
                "Security Clearance Instructions" => Some(SecurityClearanceInstructions),
                "Concomitant Medical Product Description" => {
                    Some(ConcomitantMedicalProductDescription)
                }
                "Maintenance Comment" => Some(MaintenanceComment),
                "Corrected Data" => Some(CorrectedData),
                "Consumer Comments" => Some(ConsumerComments),
                "Conviction Act Details" => Some(ConvictionActDetails),
                "Credit Report Alerts" => Some(CreditReportAlerts),
                "Closing Comment" => Some(ClosingComment),
                "Credit Report Notes" => Some(CreditReportNotes),
                "Customs declaration" => Some(CustomsDeclaration),
                "Goals, Rehabilitation Potential, or Discharge Plans" => Some(CodeDCP),
                "Destination Control Statement" => Some(DestinationControlStatement),
                "Deficiency Description Changes" => Some(DeficiencyDescriptionChanges),
                "Event Description" => Some(EventDescription),
                "Delivery" => Some(Delivery),
                "Problem Description" => Some(ProblemDescription),
                "Dose, Frequency and Route Description" => Some(CodeDFR),
                "Departure from Specification Comment" => {
                    Some(DepartureFromSpecificationComment)
                }
                "Diagnosis Description" => Some(DiagnosisDescription),
                "Durable Medical Equipment (DME) and Supplies" => Some(CodeDME),
                "Description of Damage" => Some(DescriptionOfDamage),
                "Outcome Description" => Some(OutcomeDescription),
                "Driver Out of Service Notice" => Some(DriverOutOfServiceNotice),
                "Driver Out of Service Resolution" => Some(DriverOutOfServiceResolution),
                "Detailed Statement of Work" => Some(DetailedStatementOfWork),
                "Other Type of Group" => Some(OtherTypeOfGroup),
                "Ballot Measure" => Some(BallotMeasure),
                "Attachment" => Some(Attachment),
                "Board" => Some(Board),
                "Prohibited Contribution Circumstance" => {
                    Some(ProhibitedContributionCircumstance)
                }
                "Committee Activity" => Some(CommitteeActivity),
                "Compensation Arrangement" => Some(CompensationArrangement),
                "Country Sub-Entity" => Some(CountrySubEntity),
                "Faction" => Some(Faction),
                "Gift" => Some(Gift),
                "In-Kind Contribution Use" => Some(InKindContributionUse),
                "Industry Group" => Some(IndustryGroup),
                "Jurisdiction" => Some(Jurisdiction),
                "Nature and Purpose of Other Lobbyist Employers" => {
                    Some(NatureAndPurposeOfOtherLobbyistEmployers)
                }
                "Description of Office" => Some(DescriptionOfOffice),
                "Party Considering Legislation" => Some(PartyConsideringLegislation),
                "Description of Position" => Some(DescriptionOfPosition),
                "Description of Sponsor" => Some(DescriptionOfSponsor),
                "Affiliation" => Some(Affiliation),
                "Asset Disposition" => Some(AssetDisposition),
                "Committee Interest" => Some(CommitteeInterest),
                "Compensation or Services Exchanged for Consideration" => {
                    Some(CompensationOrServicesExchangedForConsideration)
                }
                "Contributor Interest" => Some(ContributorInterest),
                "Description of Debt" => Some(DescriptionOfDebt),
                "Employer Description" => Some(EmployerDescription),
                "Purpose of Employment" => Some(PurposeOfEmployment),
                "Description of Agency and Position" => {
                    Some(DescriptionOfAgencyAndPosition)
                }
                "Description of Goods and Services" => {
                    Some(DescriptionOfGoodsAndServices)
                }
                "Length of Lobbying Actions" => Some(LengthOfLobbyingActions),
                "Lobbying Interest" => Some(LobbyingInterest),
                "Method of Disposal" => Some(MethodOfDisposal),
                "Purpose of Credit" => Some(PurposeOfCredit),
                "Description of Other Business Sub-Category" => {
                    Some(DescriptionOfOtherBusinessSubCategory)
                }
                "Other Type of Compensation" => Some(OtherTypeOfCompensation),
                "Description of Other Legislative Interest" => {
                    Some(DescriptionOfOtherLegislativeInterest)
                }
                "Other Reason for Withdrawal" => Some(OtherReasonForWithdrawal),
                "Other Type of Relationship" => Some(OtherTypeOfRelationship),
                "Other Temporary Residence" => Some(OtherTemporaryResidence),
                "Other Type of Committee" => Some(OtherTypeOfCommittee),
                "Place Paid" => Some(PlacePaid),
                "Proposition" => Some(Proposition),
                "Reason for Purchase" => Some(ReasonForPurchase),
                "Reason for Contribution" => Some(ReasonForContribution),
                "Description of Repayment Schedule" => {
                    Some(DescriptionOfRepaymentSchedule)
                }
                "Service Description" => Some(ServiceDescription),
                "Initiative" => Some(Initiative),
                "Description of Amendment" => Some(DescriptionOfAmendment),
                "Type of Election" => Some(TypeOfElection),
                "Other Type of Account" => Some(OtherTypeOfAccount),
                "Interest Rate Description" => Some(InterestRateDescription),
                "In-Kind Contribution" => Some(InKindContribution),
                "Reason for Refund" => Some(ReasonForRefund),
                "Incidental Expenses" => Some(IncidentalExpenses),
                "Environmental Conditions Description" => {
                    Some(EnvironmentalConditionsDescription)
                }
                "Other Expenses" => Some(OtherExpenses),
                "Other Income" => Some(OtherIncome),
                "Description of Receipt" => Some(DescriptionOfReceipt),
                "Surplus Funds" => Some(SurplusFunds),
                "Collateral" => Some(Collateral),
                "Contributor" => Some(Contributor),
                "Miscellaneous Receipt Transaction" => {
                    Some(MiscellaneousReceiptTransaction)
                }
                "Other Advertising" => Some(OtherAdvertising),
                "Estimate Comment" => Some(EstimateComment),
                "Equipment Condition Description" => Some(EquipmentConditionDescription),
                "Other Necessary Personal Expense" => Some(OtherNecessaryPersonalExpense),
                "Other Campaign Expense" => Some(OtherCampaignExpense),
                "Long-Term Liability" => Some(LongTermLiability),
                "Short-Term Liability" => Some(ShortTermLiability),
                "Other Sponsor Expense" => Some(OtherSponsorExpense),
                "Emergency Certification" => Some(EmergencyCertification),
                "Other Transportation Expense" => Some(OtherTransportationExpense),
                "Refund" => Some(Refund),
                "Reason for Return" => Some(ReasonForReturn),
                "Equipment Description" => Some(EquipmentDescription),
                "Equipment Function Description" => Some(EquipmentFunctionDescription),
                "Equipment Log Entry" => Some(EquipmentLogEntry),
                "Employment Comments" => Some(EmploymentComments),
                "Estimate Method Description" => Some(EstimateMethodDescription),
                "Explanation for Non-Return of Device to Manufacturer" => {
                    Some(ExplanationForNonReturnOfDeviceToManufacturer)
                }
                "Error Notes" => Some(ErrorNotes),
                "Event Location" => Some(EventLocation),
                "Exemption Description" => Some(ExemptionDescription),
                "Exhibit Notes" => Some(ExhibitNotes),
                "Exercise Routine" => Some(ExerciseRoutine),
                "Exterior Description" => Some(ExteriorDescription),
                "Final Deficiency Description" => Some(FinalDeficiencyDescription),
                "Fee Description" => Some(FeeDescription),
                "Repair Summary" => Some(RepairSummary),
                "Future Plans" => Some(FuturePlans),
                "Entire Transaction Set" => Some(EntireTransactionSet),
                "General Product or Process Information" => Some(GeneralProductOrProcess),
                "General Policy" => Some(GeneralPolicy),
                "General Specification Information" => Some(GeneralSpecification),
                "Household Goods Information" => Some(HouseholdGoods),
                "Interviewee Conversation" => Some(IntervieweeConversation),
                "Intangible Description" => Some(IntangibleDescription),
                "Inventory (Stock) Description" => Some(CodeIID),
                "Investment Description" => Some(InvestmentDescription),
                "Intercompany Relations" => Some(IntercompanyRelations),
                "Problem Impact" => Some(ProblemImpact),
                "Reason for Inspection" => Some(ReasonForInspection),
                "Insurance" => Some(Insurance),
                "General Order Instructions" => Some(GeneralOrderInstructions),
                "Invoice Instruction" => Some(InvoiceInstruction),
                "Income Verification Comments" => Some(IncomeVerificationComments),
                "Joint Venture Description" => Some(JointVentureDescription),
                "Labeling Instructions" => Some(LabelingInstructions),
                "Laboratory Data" => Some(LaboratoryData),
                "Labeled Strength" => Some(LabeledStrength),
                "Lender Use" => Some(LenderUse),
                "Line Item" => Some(LineItem),
                "Liquor Information" => Some(Liquor),
                "Letters of Liability Agreements" => Some(LettersOfLiabilityAgreements),
                "Loan Details" => Some(LoanDetails),
                "Long Term Debt Description" => Some(LongTermDebtDescription),
                "Location Description Information" => Some(LocationDescription),
                "Loading Instructions" => Some(LoadingInstructions),
                "Legal Structure Details" => Some(LegalStructureDetails),
                "Marital Contract Details" => Some(MaritalContractDetails),
                "Motor Carrier Instructions" => Some(MotorCarrierInstructions),
                "Device Operator Description" => Some(DeviceOperatorDescription),
                "Medications" => Some(Medications),
                "Manufacturing Instructions" => Some(ManufacturingInstructions),
                "Marketing Notes" => Some(MarketingNotes),
                "Merger Description" => Some(MergerDescription),
                "Marketable Securities Description" => {
                    Some(MarketableSecuritiesDescription)
                }
                "Nonconformance Specification" => Some(NonconformanceSpecification),
                "Nameplate Data" => Some(NameplateData),
                "Nutritional Requirements" => Some(NutritionalRequirements),
                "Originator to Beneficiary Instructions" => {
                    Some(OriginatorToBeneficiaryInstructions)
                }
                "Obligation Description" => Some(ObligationDescription),
                "Other Current Asset Description" => Some(OtherCurrentAssetDescription),
                "Occupancy Information" => Some(Occupancy),
                "Other Current Liability Description" => {
                    Some(OtherCurrentLiabilityDescription)
                }
                "Occupation" => Some(Occupation),
                "Outside the Continental U.S. (OCONUS) Rating Information" => {
                    Some(CodeOCR)
                }
                "Originator Deficiency Description" => {
                    Some(OriginatorDeficiencyDescription)
                }
                "Orders for Disciplines and Treatments" => {
                    Some(OrdersForDisciplinesAndTreatments)
                }
                "Original Legal Structure" => Some(OriginalLegalStructure),
                "Occupation Definition" => Some(OccupationDefinition),
                "Test Results Other Than Room Air" => Some(TestResultsOtherThanRoomAir),
                "Ordering Restrictions" => Some(OrderingRestrictions),
                "Other Remedial Action" => Some(OtherRemedialAction),
                "Order Instructions" => Some(OrderInstructions),
                "Other Instructions" => Some(OtherInstructions),
                "Another Type of Number Description" => {
                    Some(AnotherTypeOfNumberDescription)
                }
                "Report Source Description" => Some(ReportSourceDescription),
                "Payables" => Some(Payables),
                "Process Specification" => Some(ProcessSpecification),
                "Product Specification" => Some(ProductSpecification),
                "Employee Sharing Arrangements" => Some(EmployeeSharingArrangements),
                "Penalty Description" => Some(PenaltyDescription),
                "Partial Pressure of Oxygen (PO2) is 60 millimeters (MM) of Mercury (Hg) or above, or arterial blood oxygen saturation is 90% or above" => {
                    Some(CodePES)
                }
                "Property Improvement Description" => {
                    Some(PropertyImprovementDescription)
                }
                "Packaging Instructions" => Some(PackagingInstructions),
                "Payment" => Some(Payment),
                "Primary Observation" => Some(PrimaryObservation),
                "Principals or Organization Comments" => {
                    Some(PrincipalsOrOrganizationComments)
                }
                "Property Owner Location Information" => Some(PropertyOwnerLocation),
                "Principal Procedure Code Description" => {
                    Some(PrincipalProcedureCodeDescription)
                }
                "Priority" => Some(Priority),
                "Public Record Notes" => Some(PublicRecordNotes),
                "Previous Registered Office" => Some(PreviousRegisteredOffice),
                "Price Range" => Some(PriceRange),
                "Problem Summary" => Some(ProblemSummary),
                "Purchasing" => Some(Purchasing),
                "Qualifications" => Some(Qualifications),
                "Quotation Instruction" => Some(QuotationInstruction),
                "Reason for Delinquency Information" => Some(ReasonForDelinquency),
                "Recommendation" => Some(Recommendation),
                "Treatment Refusal Note" => Some(TreatmentRefusalNote),
                "Registered Activity" => Some(RegisteredActivity),
                "Report" => Some(Report),
                "Receivables" => Some(Receivables),
                "Reason for Leaving" => Some(ReasonForLeaving),
                "Functional Limitations, Reason Homebound, or Both" => Some(CodeRHB),
                "Reason for Leave of Absence" => Some(ReasonForLeaveOfAbsence),
                "Reasons Patient Leaves Home" => Some(ReasonsPatientLeavesHome),
                "Times and Reasons Patient Not at Home" => {
                    Some(TimesAndReasonsPatientNotAtHome)
                }
                "Missing Report Explanation" => Some(MissingReportExplanation),
                "Circuit Routing Instructions" => Some(CircuitRoutingInstructions),
                "Report Remarks" => Some(ReportRemarks),
                "Place Where a Report was Submitted" => {
                    Some(PlaceWhereAReportWasSubmitted)
                }
                "Rent Verification Comments" => Some(RentVerificationComments),
                "Settlement Amount Notes" => Some(SettlementAmountNotes),
                "Ocean Shipping Container Information" => Some(OceanShippingContainer),
                "Sentence Description" => Some(SentenceDescription),
                "Unusual Home, Social Environment, or Both" => Some(CodeSET),
                "Safety Measures" => Some(SafetyMeasures),
                "Shipping Restrictions" => Some(ShippingRestrictions),
                "Selling Means Description" => Some(SellingMeansDescription),
                "Secondary Observation" => Some(SecondaryObservation),
                "Statement of Work" => Some(StatementOfWork),
                "Special Handling" => Some(SpecialHandling),
                "Supplementary Plan of Treatment" => Some(SupplementaryPlanOfTreatment),
                "Closing Instructions" => Some(ClosingInstructions),
                "School Attended Details" => Some(SchoolAttendedDetails),
                "Status Comment" => Some(StatusComment),
                "Sales Description" => Some(SalesDescription),
                "Spouse Information" => Some(Spouse),
                "School Graduated Details" => Some(SchoolGraduatedDetails),
                "Security Service Information" => Some(SecurityService),
                "Shareholding Information" => Some(Shareholding),
                "Signing Authority" => Some(SigningAuthority),
                "Sales Territory" => Some(SalesTerritory),
                "Tariff Abbreviation" => Some(TariffAbbreviation),
                "Tariff Commodity Footnotes" => Some(TariffCommodityFootnotes),
                "Turkish Defense Affairs Authorization Information" => {
                    Some(TurkishDefenseAffairsAuthorization)
                }
                "Task Statement" => Some(TaskStatement),
                "Tariff Index" => Some(TariffIndex),
                "Tariff Rule" => Some(TariffRule),
                "Tradeline Remarks" => Some(TradelineRemarks),
                "Toxic Organic Management Plan" => Some(ToxicOrganicManagementPlan),
                "Third Party Organization Notes" => Some(ThirdPartyOrganizationNotes),
                "Transportation" => Some(Transportation),
                "Reportable Event Description" => Some(ReportableEventDescription),
                "Tariff Rate Footnotes" => Some(TariffRateFootnotes),
                "Quality Information" => Some(Quality),
                "Terms of Sale Description" => Some(TermsOfSaleDescription),
                "Tariff Section Footnotes" => Some(TariffSectionFootnotes),
                "Test Results" => Some(TestResults),
                "Updated Information" => Some(Updated),
                "Verification Comments" => Some(VerificationComments),
                "Vehicle Out of Service Notice" => Some(VehicleOutOfServiceNotice),
                "Variation Notes" => Some(VariationNotes),
                "Warehouse Instruction" => Some(WarehouseInstruction),
                "Wrapping Instructions" => Some(WrappingInstructions),
                "Zero Discharge Certification Statement" => {
                    Some(ZeroDischargeCertificationStatement)
                }
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for NoteReferenceCode {
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
    type Value = NoteReferenceCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Note Reference Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NoteReferenceCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Note Reference Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NoteReferenceCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Note Reference Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for NoteReferenceCode {
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