use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**143

See docs at <https://www.stedi.com/edi/x12/element/143>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransactionSetIdentifierCode {
    ///100 - Insurance Plan Description
    InsurancePlanDescription,
    ///101 - Name and Address Lists
    NameAndAddressLists,
    ///102 - Associated Data
    AssociatedData,
    ///103 - Abandoned Property Filings
    AbandonedPropertyFilings,
    ///104 - Air Shipment Information
    AirShipment,
    ///105 - Business Entity Filings
    BusinessEntityFilings,
    ///106 - Motor Carrier Rate Proposal
    MotorCarrierRateProposal,
    ///107 - Request for Motor Carrier Rate Proposal
    RequestForMotorCarrierRateProposal,
    ///108 - Response to a Motor Carrier Rate Proposal
    ResponseToAMotorCarrierRateProposal,
    ///109 - Vessel Content Details
    VesselContentDetails,
    ///110 - Air Freight Details and Invoice
    AirFreightDetailsAndInvoice,
    ///111 - Individual Insurance Policy and Client Information
    IndividualInsurancePolicyAndClient,
    ///112 - Property Damage Report
    PropertyDamageReport,
    ///113 - Election Campaign and Lobbyist Reporting
    ElectionCampaignAndLobbyistReporting,
    ///120 - Vehicle Shipping Order
    VehicleShippingOrder,
    ///121 - Vehicle Service
    VehicleService,
    ///124 - Vehicle Damage
    VehicleDamage,
    ///125 - Multilevel Railcar Load Details
    MultilevelRailcarLoadDetails,
    ///126 - Vehicle Application Advice
    VehicleApplicationAdvice,
    ///127 - Vehicle Baying Order
    VehicleBayingOrder,
    ///128 - Dealer Information
    Dealer,
    ///129 - Vehicle Carrier Rate Update
    VehicleCarrierRateUpdate,
    ///130 - Student Educational Record (Transcript)
    Code130,
    ///131 - Student Educational Record (Transcript) Acknowledgment
    Code131,
    ///132 - Human Resource Information
    HumanResource,
    ///133 - Educational Institution Record
    EducationalInstitutionRecord,
    ///135 - Student Aid Origination Record
    StudentAidOriginationRecord,
    ///138 - Educational Testing and Prospect Request and Report
    EducationalTestingAndProspectRequestAndReport,
    ///139 - Student Loan Guarantee Result
    StudentLoanGuaranteeResult,
    ///140 - Product Registration
    ProductRegistration,
    ///141 - Product Service Claim Response
    ProductServiceClaimResponse,
    ///142 - Product Service Claim
    ProductServiceClaim,
    ///143 - Product Service Notification
    ProductServiceNotification,
    ///144 - Student Loan Transfer and Status Verification
    StudentLoanTransferAndStatusVerification,
    ///146 - Request for Student Educational Record (Transcript)
    Code146,
    ///147 - Response to Request for Student Educational Record (Transcript)
    Code147,
    ///148 - Report of Injury, Illness or Incident
    Code148,
    ///149 - Notice of Tax Adjustment or Assessment
    NoticeOfTaxAdjustmentOrAssessment,
    ///150 - Tax Rate Notification
    TaxRateNotification,
    ///151 - Electronic Filing of Tax Return Data Acknowledgment
    ElectronicFilingOfTaxReturnDataAcknowledgment,
    ///152 - Statistical Government Information
    StatisticalGovernment,
    ///153 - Unemployment Insurance Tax Claim or Charge Information
    UnemploymentInsuranceTaxClaimOrCharge,
    ///154 - Secured Interest Filing
    SecuredInterestFiling,
    ///155 - Business Credit Report
    BusinessCreditReport,
    ///157 - Notice of Power of Attorney
    NoticeOfPowerOfAttorney,
    ///158 - Tax Jurisdiction Sourcing
    TaxJurisdictionSourcing,
    ///159 - Motion Picture Booking Confirmation
    MotionPictureBookingConfirmation,
    ///160 - Transportation Automatic Equipment Identification
    TransportationAutomaticEquipmentIdentification,
    ///161 - Train Sheet
    TrainSheet,
    ///163 - Transportation Appointment Schedule Information
    TransportationAppointmentSchedule,
    ///170 - Revenue Receipts Statement
    RevenueReceiptsStatement,
    ///175 - Court and Law Enforcement Notice
    CourtAndLawEnforcementNotice,
    ///176 - Court Submission
    CourtSubmission,
    ///179 - Environmental Compliance Reporting
    EnvironmentalComplianceReporting,
    ///180 - Return Merchandise Authorization and Notification
    ReturnMerchandiseAuthorizationAndNotification,
    ///185 - Royalty Regulatory Report
    RoyaltyRegulatoryReport,
    ///186 - Insurance Underwriting Requirements Reporting
    InsuranceUnderwritingRequirementsReporting,
    ///187 - Premium Audit Request and Return
    PremiumAuditRequestAndReturn,
    ///188 - Educational Course Inventory
    EducationalCourseInventory,
    ///189 - Application for Admission to Educational Institutions
    ApplicationForAdmissionToEducationalInstitutions,
    ///190 - Student Enrollment Verification
    StudentEnrollmentVerification,
    ///191 - Student Loan Pre-Claims and Claims
    StudentLoanPreClaimsAndClaims,
    ///194 - Grant or Assistance Application
    GrantOrAssistanceApplication,
    ///195 - Federal Communications Commission (FCC) License Application
    Code195,
    ///196 - Contractor Cost Data Reporting
    ContractorCostDataReporting,
    ///197 - Real Estate Title Evidence
    RealEstateTitleEvidence,
    ///198 - Loan Verification Information
    LoanVerification,
    ///199 - Real Estate Settlement Information
    RealEstateSettlement,
    ///200 - Mortgage Credit Report
    MortgageCreditReport,
    ///201 - Residential Loan Application
    ResidentialLoanApplication,
    ///202 - Secondary Mortgage Market Loan Delivery
    SecondaryMortgageMarketLoanDelivery,
    ///203 - Secondary Mortgage Market Investor Report
    SecondaryMortgageMarketInvestorReport,
    ///204 - Motor Carrier Load Tender
    MotorCarrierLoadTender,
    ///205 - Mortgage Note
    MortgageNote,
    ///206 - Real Estate Inspection
    RealEstateInspection,
    ///210 - Motor Carrier Freight Details and Invoice
    MotorCarrierFreightDetailsAndInvoice,
    ///211 - Motor Carrier Bill of Lading
    MotorCarrierBillOfLading,
    ///212 - Motor Carrier Delivery Trailer Manifest
    MotorCarrierDeliveryTrailerManifest,
    ///213 - Motor Carrier Shipment Status Inquiry
    MotorCarrierShipmentStatusInquiry,
    ///214 - Transportation Carrier Shipment Status Message
    TransportationCarrierShipmentStatusMessage,
    ///215 - Motor Carrier Pickup Manifest
    MotorCarrierPickupManifest,
    ///216 - Motor Carrier Shipment Pickup Notification
    MotorCarrierShipmentPickupNotification,
    ///217 - Motor Carrier Loading and Route Guide
    MotorCarrierLoadingAndRouteGuide,
    ///219 - Logistics Service Request
    LogisticsServiceRequest,
    ///220 - Logistics Service Response
    LogisticsServiceResponse,
    ///222 - Cartage Work Assignment
    CartageWorkAssignment,
    ///223 - Consolidators Freight Bill and Invoice
    ConsolidatorsFreightBillAndInvoice,
    ///224 - Motor Carrier Summary Freight Bill Manifest
    MotorCarrierSummaryFreightBillManifest,
    ///225 - Response to a Cartage Work Assignment
    ResponseToACartageWorkAssignment,
    ///227 - Trailer Usage Report
    TrailerUsageReport,
    ///228 - Equipment Inspection Report
    EquipmentInspectionReport,
    ///240 - Motor Carrier Package Status
    MotorCarrierPackageStatus,
    ///242 - Data Status Tracking
    DataStatusTracking,
    ///244 - Product Source Information
    ProductSource,
    ///245 - Real Estate Tax Service Response
    RealEstateTaxServiceResponse,
    ///248 - Account Assignment/Inquiry and Service/Status
    AccountAssignmentInquiryAndServiceStatus,
    ///249 - Animal Toxicological Data
    AnimalToxicologicalData,
    ///250 - Purchase Order Shipment Management Document
    PurchaseOrderShipmentManagementDocument,
    ///251 - Pricing Support
    PricingSupport,
    ///252 - Insurance Producer Administration
    InsuranceProducerAdministration,
    ///255 - Underwriting Information Services
    UnderwritingInformationServices,
    ///256 - Periodic Compensation
    PeriodicCompensation,
    ///259 - Residential Mortgage Insurance Explanation of Benefits
    ResidentialMortgageInsuranceExplanationOfBenefits,
    ///260 - Application for Mortgage Insurance Benefits
    ApplicationForMortgageInsuranceBenefits,
    ///261 - Real Estate Information Request
    RealEstateInformationRequest,
    ///262 - Real Estate Information Report
    RealEstateInformationReport,
    ///263 - Residential Mortgage Insurance Application Response
    ResidentialMortgageInsuranceApplicationResponse,
    ///264 - Mortgage Loan Default Status
    MortgageLoanDefaultStatus,
    ///265 - Real Estate Title Insurance Services Order
    RealEstateTitleInsuranceServicesOrder,
    ///266 - Mortgage or Property Record Change Notification
    MortgageOrPropertyRecordChangeNotification,
    ///267 - Individual Life, Annuity and Disability Application
    Code267,
    ///268 - Annuity Activity
    AnnuityActivity,
    ///269 - Health Care Benefit Coordination Verification
    HealthCareBenefitCoordinationVerification,
    ///270 - Eligibility, Coverage or Benefit Inquiry
    Code270,
    ///271 - Eligibility, Coverage or Benefit Information
    Code271,
    ///272 - Property and Casualty Loss Notification
    PropertyAndCasualtyLossNotification,
    ///273 - Insurance/Annuity Application Status
    InsuranceAnnuityApplicationStatus,
    ///274 - Healthcare Provider Information
    HealthcareProvider,
    ///275 - Patient Information
    Patient,
    ///276 - Health Care Claim Status Request
    HealthCareClaimStatusRequest,
    ///277 - Health Care Information Status Notification
    HealthCareInformationStatusNotification,
    ///278 - Health Care Services Review Information
    HealthCareServicesReview,
    ///280 - Voter Registration Information
    VoterRegistration,
    ///283 - Tax or Fee Exemption Certification
    TaxOrFeeExemptionCertification,
    ///284 - Commercial Vehicle Safety Reports
    CommercialVehicleSafetyReports,
    ///285 - Commercial Vehicle Safety and Credentials Information Exchange
    CommercialVehicleSafetyAndCredentialsInformationExchange,
    ///286 - Commercial Vehicle Credentials
    CommercialVehicleCredentials,
    ///288 - Wage Determination
    WageDetermination,
    ///290 - Cooperative Advertising Agreements
    CooperativeAdvertisingAgreements,
    ///300 - Reservation (Booking Request) (Ocean)
    Code300,
    ///301 - Confirmation (Ocean)
    Code301,
    ///303 - Booking Cancellation (Ocean)
    Code303,
    ///304 - Shipping Instructions
    ShippingInstructions,
    ///309 - Customs Manifest
    CustomsManifest,
    ///310 - Freight Receipt and Invoice (Ocean)
    Code310,
    ///311 - Canada Customs Information
    CanadaCustoms,
    ///312 - Arrival Notice (Ocean)
    Code312,
    ///313 - Shipment Status Inquiry (Ocean)
    Code313,
    ///315 - Status Details (Ocean)
    Code315,
    ///317 - Delivery/Pickup Order
    DeliveryPickupOrder,
    ///319 - Terminal Information
    Terminal,
    ///322 - Terminal Operations and Intermodal Ramp Activity
    TerminalOperationsAndIntermodalRampActivity,
    ///323 - Vessel Schedule and Itinerary (Ocean)
    Code323,
    ///324 - Vessel Stow Plan (Ocean)
    Code324,
    ///325 - Consolidation of Goods In Container
    ConsolidationOfGoodsInContainer,
    ///326 - Consignment Summary List
    ConsignmentSummaryList,
    ///350 - Customs Status Information
    CustomsStatus,
    ///352 - U.S. Customs Carrier General Order Status
    USCustomsCarrierGeneralOrderStatus,
    ///353 - Customs Events Advisory Details
    CustomsEventsAdvisoryDetails,
    ///354 - U.S. Customs Automated Manifest Archive Status
    USCustomsAutomatedManifestArchiveStatus,
    ///355 - U.S. Customs Acceptance/Rejection
    USCustomsAcceptanceRejection,
    ///356 - U.S. Customs Permit to Transfer Request
    USCustomsPermitToTransferRequest,
    ///357 - U.S. Customs In-Bond Information
    USCustomsInBond,
    ///358 - Customs Consist Information
    CustomsConsist,
    ///361 - Carrier Interchange Agreement (Ocean)
    Code361,
    ///362 - Cargo Insurance Advice of Shipment
    CargoInsuranceAdviceOfShipment,
    ///404 - Rail Carrier Shipment Information
    RailCarrierShipment,
    ///410 - Rail Carrier Freight Details and Invoice
    RailCarrierFreightDetailsAndInvoice,
    ///412 - Trailer or Container Repair Billing
    TrailerOrContainerRepairBilling,
    ///414 - Rail Carhire Settlements
    RailCarhireSettlements,
    ///417 - Rail Carrier Waybill Interchange
    RailCarrierWaybillInterchange,
    ///418 - Rail Advance Interchange Consist
    RailAdvanceInterchangeConsist,
    ///419 - Advance Car Disposition
    AdvanceCarDisposition,
    ///420 - Car Handling Information
    CarHandling,
    ///421 - Estimated Time of Arrival and Car Scheduling
    EstimatedTimeOfArrivalAndCarScheduling,
    ///422 - Equipment Order
    EquipmentOrder,
    ///423 - Rail Industrial Switch List
    RailIndustrialSwitchList,
    ///424 - Rail Carrier Services Settlement
    RailCarrierServicesSettlement,
    ///425 - Rail Waybill Request
    RailWaybillRequest,
    ///426 - Rail Revenue Waybill
    RailRevenueWaybill,
    ///429 - Railroad Retirement Activity
    RailroadRetirementActivity,
    ///431 - Railroad Station Master File
    RailroadStationMasterFile,
    ///432 - Rail Deprescription
    RailDeprescription,
    ///433 - Railroad Reciprocal Switch File
    RailroadReciprocalSwitchFile,
    ///434 - Railroad Mark Register Update Activity
    RailroadMarkRegisterUpdateActivity,
    ///435 - Standard Transportation Commodity Code Master
    StandardTransportationCommodityCodeMaster,
    ///436 - Locomotive Information
    Locomotive,
    ///437 - Railroad Junctions and Interchanges Activity
    RailroadJunctionsAndInterchangesActivity,
    ///440 - Shipment Weights
    ShipmentWeights,
    ///451 - Railroad Event Report
    RailroadEventReport,
    ///452 - Railroad Problem Log Inquiry or Advice
    RailroadProblemLogInquiryOrAdvice,
    ///453 - Railroad Service Commitment Advice
    RailroadServiceCommitmentAdvice,
    ///455 - Railroad Parameter Trace Registration
    RailroadParameterTraceRegistration,
    ///456 - Railroad Equipment Inquiry or Advice
    RailroadEquipmentInquiryOrAdvice,
    ///460 - Railroad Price Distribution Request or Response
    RailroadPriceDistributionRequestOrResponse,
    ///463 - Rail Rate Reply
    RailRateReply,
    ///466 - Rate Request
    RateRequest,
    ///468 - Rate Docket Journal Log
    RateDocketJournalLog,
    ///470 - Railroad Clearance
    RailroadClearance,
    ///475 - Rail Route File Maintenance
    RailRouteFileMaintenance,
    ///485 - Ratemaking Action
    RatemakingAction,
    ///486 - Rate Docket Expiration
    RateDocketExpiration,
    ///490 - Rate Group Definition
    RateGroupDefinition,
    ///492 - Miscellaneous Rates
    MiscellaneousRates,
    ///494 - Rail Scale Rates
    RailScaleRates,
    ///500 - Medical Event Reporting
    MedicalEventReporting,
    ///501 - Vendor Performance Review
    VendorPerformanceReview,
    ///503 - Pricing History
    PricingHistory,
    ///504 - Clauses and Provisions
    ClausesAndProvisions,
    ///511 - Requisition
    Requisition,
    ///517 - Material Obligation Validation
    MaterialObligationValidation,
    ///521 - Income or Asset Offset
    IncomeOrAssetOffset,
    ///527 - Material Due-In and Receipt
    MaterialDueInAndReceipt,
    ///536 - Logistics Reassignment
    LogisticsReassignment,
    ///540 - Notice of Employment Status
    NoticeOfEmploymentStatus,
    ///561 - Contract Abstract
    ContractAbstract,
    ///567 - Contract Completion Status
    ContractCompletionStatus,
    ///568 - Contract Payment Management Report
    ContractPaymentManagementReport,
    ///601 - U.S. Customs Export Shipment Information
    USCustomsExportShipment,
    ///602 - Transportation Services Tender
    TransportationServicesTender,
    ///620 - Excavation Communication
    ExcavationCommunication,
    ///625 - Well Information
    Well,
    ///650 - Maintenance Service Order
    MaintenanceServiceOrder,
    ///715 - Intermodal Group Loading Plan
    IntermodalGroupLoadingPlan,
    ///753 - Request for Routing Instructions
    RequestForRoutingInstructions,
    ///754 - Routing Instructions
    RoutingInstructions,
    ///805 - Contract Pricing Proposal
    ContractPricingProposal,
    ///806 - Project Schedule Reporting
    ProjectScheduleReporting,
    ///810 - Invoice
    Invoice,
    ///811 - Consolidated Service Invoice/Statement
    ConsolidatedServiceInvoiceStatement,
    ///812 - Credit/Debit Adjustment
    CreditDebitAdjustment,
    ///813 - Electronic Filing of Tax Return Data
    ElectronicFilingOfTaxReturnData,
    ///814 - General Request, Response or Confirmation
    Code814,
    ///815 - Cryptographic Service Message
    CryptographicServiceMessage,
    ///816 - Organizational Relationships
    OrganizationalRelationships,
    ///818 - Commission Sales Report
    CommissionSalesReport,
    ///819 - Joint Interest Billing and Operating Expense Statement
    JointInterestBillingAndOperatingExpenseStatement,
    ///820 - Payment Order/Remittance Advice
    PaymentOrderRemittanceAdvice,
    ///821 - Financial Information Reporting
    FinancialInformationReporting,
    ///822 - Account Analysis
    AccountAnalysis,
    ///823 - Lockbox
    Lockbox,
    ///824 - Application Advice
    ApplicationAdvice,
    ///826 - Tax Information Exchange
    TaxInformationExchange,
    ///827 - Financial Return Notice
    FinancialReturnNotice,
    ///828 - Debit Authorization
    DebitAuthorization,
    ///829 - Payment Cancellation Request
    PaymentCancellationRequest,
    ///830 - Planning Schedule with Release Capability
    PlanningScheduleWithReleaseCapability,
    ///831 - Application Control Totals
    ApplicationControlTotals,
    ///832 - Price/Sales Catalog
    PriceSalesCatalog,
    ///833 - Mortgage Credit Report Order
    MortgageCreditReportOrder,
    ///834 - Benefit Enrollment and Maintenance
    BenefitEnrollmentAndMaintenance,
    ///835 - Health Care Claim Payment/Advice
    HealthCareClaimPaymentAdvice,
    ///836 - Procurement Notices
    ProcurementNotices,
    ///837 - Health Care Claim
    HealthCareClaim,
    ///838 - Trading Partner Profile
    TradingPartnerProfile,
    ///839 - Project Cost Reporting
    ProjectCostReporting,
    ///840 - Request for Quotation
    RequestForQuotation,
    ///841 - Specifications/Technical Information
    SpecificationsTechnical,
    ///842 - Nonconformance Report
    NonconformanceReport,
    ///843 - Response to Request for Quotation
    ResponseToRequestForQuotation,
    ///844 - Product Transfer Account Adjustment
    ProductTransferAccountAdjustment,
    ///845 - Price Authorization Acknowledgment/Status
    PriceAuthorizationAcknowledgmentStatus,
    ///846 - Inventory Inquiry/Advice
    InventoryInquiryAdvice,
    ///847 - Material Claim
    MaterialClaim,
    ///848 - Material Safety Data Sheet
    MaterialSafetyDataSheet,
    ///849 - Response to Product Transfer Account Adjustment
    ResponseToProductTransferAccountAdjustment,
    ///850 - Purchase Order
    PurchaseOrder,
    ///851 - Asset Schedule
    AssetSchedule,
    ///852 - Product Activity Data
    ProductActivityData,
    ///853 - Routing and Carrier Instruction
    RoutingAndCarrierInstruction,
    ///854 - Shipment Delivery Discrepancy Information
    ShipmentDeliveryDiscrepancy,
    ///855 - Purchase Order Acknowledgment
    PurchaseOrderAcknowledgment,
    ///856 - Ship Notice/Manifest
    ShipNoticeManifest,
    ///857 - Shipment and Billing Notice
    ShipmentAndBillingNotice,
    ///858 - Shipment Information
    Shipment,
    ///859 - Freight Invoice
    FreightInvoice,
    ///860 - Purchase Order Change Request - Buyer Initiated
    PurchaseOrderChangeRequestBuyerInitiated,
    ///861 - Receiving Advice/Acceptance Certificate
    ReceivingAdviceAcceptanceCertificate,
    ///862 - Shipping Schedule
    ShippingSchedule,
    ///863 - Report of Test Results
    ReportOfTestResults,
    ///864 - Text Message
    TextMessage,
    ///865 - Purchase Order Change Acknowledgment/Request - Seller Initiated
    PurchaseOrderChangeAcknowledgmentRequestSellerInitiated,
    ///866 - Production Sequence
    ProductionSequence,
    ///867 - Product Transfer and Resale Report
    ProductTransferAndResaleReport,
    ///868 - Electronic Form Structure
    ElectronicFormStructure,
    ///869 - Order Status Inquiry
    OrderStatusInquiry,
    ///870 - Order Status Report
    OrderStatusReport,
    ///871 - Component Parts Content
    ComponentPartsContent,
    ///872 - Residential Mortgage Insurance Application
    ResidentialMortgageInsuranceApplication,
    ///873 - Commodity Movement Services
    CommodityMovementServices,
    ///874 - Commodity Movement Services Response
    CommodityMovementServicesResponse,
    ///875 - Grocery Products Purchase Order
    GroceryProductsPurchaseOrder,
    ///876 - Grocery Products Purchase Order Change
    GroceryProductsPurchaseOrderChange,
    ///877 - Manufacturer Coupon Family Code Structure
    ManufacturerCouponFamilyCodeStructure,
    ///878 - Product Authorization/De-authorization
    ProductAuthorizationDeAuthorization,
    ///879 - Price Information
    Price,
    ///880 - Grocery Products Invoice
    GroceryProductsInvoice,
    ///881 - Manufacturer Coupon Redemption Detail
    ManufacturerCouponRedemptionDetail,
    ///882 - Direct Store Delivery Summary Information
    DirectStoreDeliverySummary,
    ///883 - Market Development Fund Allocation
    MarketDevelopmentFundAllocation,
    ///884 - Market Development Fund Settlement
    MarketDevelopmentFundSettlement,
    ///885 - Retail Account Characteristics
    RetailAccountCharacteristics,
    ///886 - Customer Call Reporting
    CustomerCallReporting,
    ///887 - Coupon Notification
    CouponNotification,
    ///888 - Item Maintenance
    ItemMaintenance,
    ///889 - Promotion Announcement
    PromotionAnnouncement,
    ///891 - Deduction Research Report
    DeductionResearchReport,
    ///893 - Item Information Request
    ItemInformationRequest,
    ///894 - Delivery/Return Base Record
    DeliveryReturnBaseRecord,
    ///895 - Delivery/Return Acknowledgment or Adjustment
    DeliveryReturnAcknowledgmentOrAdjustment,
    ///896 - Product Dimension Maintenance
    ProductDimensionMaintenance,
    ///920 - Loss or Damage Claim - General Commodities
    LossOrDamageClaimGeneralCommodities,
    ///924 - Loss or Damage Claim - Motor Vehicle
    LossOrDamageClaimMotorVehicle,
    ///925 - Claim Tracer
    ClaimTracer,
    ///926 - Claim Status Report and Tracer Reply
    ClaimStatusReportAndTracerReply,
    ///928 - Automotive Inspection Detail
    AutomotiveInspectionDetail,
    ///940 - Warehouse Shipping Order
    WarehouseShippingOrder,
    ///943 - Warehouse Stock Transfer Shipment Advice
    WarehouseStockTransferShipmentAdvice,
    ///944 - Warehouse Stock Transfer Receipt Advice
    WarehouseStockTransferReceiptAdvice,
    ///945 - Warehouse Shipping Advice
    WarehouseShippingAdvice,
    ///947 - Warehouse Inventory Adjustment Advice
    WarehouseInventoryAdjustmentAdvice,
    ///980 - Functional Group Totals
    FunctionalGroupTotals,
    ///990 - Response to a Load Tender
    ResponseToALoadTender,
    ///993 - Secured Receipt or Acknowledgment
    SecuredReceiptOrAcknowledgment,
    ///996 - File Transfer
    FileTransfer,
    ///997 - Functional Acknowledgment
    FunctionalAcknowledgment,
    ///998 - Set Cancellation
    SetCancellation,
    ///999 - Implementation Acknowledgment
    ImplementationAcknowledgment,
}
impl TransactionSetIdentifierCode {
    pub fn code(&self) -> &str {
        {
            use TransactionSetIdentifierCode::*;
            match self {
                InsurancePlanDescription => "100",
                NameAndAddressLists => "101",
                AssociatedData => "102",
                AbandonedPropertyFilings => "103",
                AirShipment => "104",
                BusinessEntityFilings => "105",
                MotorCarrierRateProposal => "106",
                RequestForMotorCarrierRateProposal => "107",
                ResponseToAMotorCarrierRateProposal => "108",
                VesselContentDetails => "109",
                AirFreightDetailsAndInvoice => "110",
                IndividualInsurancePolicyAndClient => "111",
                PropertyDamageReport => "112",
                ElectionCampaignAndLobbyistReporting => "113",
                VehicleShippingOrder => "120",
                VehicleService => "121",
                VehicleDamage => "124",
                MultilevelRailcarLoadDetails => "125",
                VehicleApplicationAdvice => "126",
                VehicleBayingOrder => "127",
                Dealer => "128",
                VehicleCarrierRateUpdate => "129",
                Code130 => "130",
                Code131 => "131",
                HumanResource => "132",
                EducationalInstitutionRecord => "133",
                StudentAidOriginationRecord => "135",
                EducationalTestingAndProspectRequestAndReport => "138",
                StudentLoanGuaranteeResult => "139",
                ProductRegistration => "140",
                ProductServiceClaimResponse => "141",
                ProductServiceClaim => "142",
                ProductServiceNotification => "143",
                StudentLoanTransferAndStatusVerification => "144",
                Code146 => "146",
                Code147 => "147",
                Code148 => "148",
                NoticeOfTaxAdjustmentOrAssessment => "149",
                TaxRateNotification => "150",
                ElectronicFilingOfTaxReturnDataAcknowledgment => "151",
                StatisticalGovernment => "152",
                UnemploymentInsuranceTaxClaimOrCharge => "153",
                SecuredInterestFiling => "154",
                BusinessCreditReport => "155",
                NoticeOfPowerOfAttorney => "157",
                TaxJurisdictionSourcing => "158",
                MotionPictureBookingConfirmation => "159",
                TransportationAutomaticEquipmentIdentification => "160",
                TrainSheet => "161",
                TransportationAppointmentSchedule => "163",
                RevenueReceiptsStatement => "170",
                CourtAndLawEnforcementNotice => "175",
                CourtSubmission => "176",
                EnvironmentalComplianceReporting => "179",
                ReturnMerchandiseAuthorizationAndNotification => "180",
                RoyaltyRegulatoryReport => "185",
                InsuranceUnderwritingRequirementsReporting => "186",
                PremiumAuditRequestAndReturn => "187",
                EducationalCourseInventory => "188",
                ApplicationForAdmissionToEducationalInstitutions => "189",
                StudentEnrollmentVerification => "190",
                StudentLoanPreClaimsAndClaims => "191",
                GrantOrAssistanceApplication => "194",
                Code195 => "195",
                ContractorCostDataReporting => "196",
                RealEstateTitleEvidence => "197",
                LoanVerification => "198",
                RealEstateSettlement => "199",
                MortgageCreditReport => "200",
                ResidentialLoanApplication => "201",
                SecondaryMortgageMarketLoanDelivery => "202",
                SecondaryMortgageMarketInvestorReport => "203",
                MotorCarrierLoadTender => "204",
                MortgageNote => "205",
                RealEstateInspection => "206",
                MotorCarrierFreightDetailsAndInvoice => "210",
                MotorCarrierBillOfLading => "211",
                MotorCarrierDeliveryTrailerManifest => "212",
                MotorCarrierShipmentStatusInquiry => "213",
                TransportationCarrierShipmentStatusMessage => "214",
                MotorCarrierPickupManifest => "215",
                MotorCarrierShipmentPickupNotification => "216",
                MotorCarrierLoadingAndRouteGuide => "217",
                LogisticsServiceRequest => "219",
                LogisticsServiceResponse => "220",
                CartageWorkAssignment => "222",
                ConsolidatorsFreightBillAndInvoice => "223",
                MotorCarrierSummaryFreightBillManifest => "224",
                ResponseToACartageWorkAssignment => "225",
                TrailerUsageReport => "227",
                EquipmentInspectionReport => "228",
                MotorCarrierPackageStatus => "240",
                DataStatusTracking => "242",
                ProductSource => "244",
                RealEstateTaxServiceResponse => "245",
                AccountAssignmentInquiryAndServiceStatus => "248",
                AnimalToxicologicalData => "249",
                PurchaseOrderShipmentManagementDocument => "250",
                PricingSupport => "251",
                InsuranceProducerAdministration => "252",
                UnderwritingInformationServices => "255",
                PeriodicCompensation => "256",
                ResidentialMortgageInsuranceExplanationOfBenefits => "259",
                ApplicationForMortgageInsuranceBenefits => "260",
                RealEstateInformationRequest => "261",
                RealEstateInformationReport => "262",
                ResidentialMortgageInsuranceApplicationResponse => "263",
                MortgageLoanDefaultStatus => "264",
                RealEstateTitleInsuranceServicesOrder => "265",
                MortgageOrPropertyRecordChangeNotification => "266",
                Code267 => "267",
                AnnuityActivity => "268",
                HealthCareBenefitCoordinationVerification => "269",
                Code270 => "270",
                Code271 => "271",
                PropertyAndCasualtyLossNotification => "272",
                InsuranceAnnuityApplicationStatus => "273",
                HealthcareProvider => "274",
                Patient => "275",
                HealthCareClaimStatusRequest => "276",
                HealthCareInformationStatusNotification => "277",
                HealthCareServicesReview => "278",
                VoterRegistration => "280",
                TaxOrFeeExemptionCertification => "283",
                CommercialVehicleSafetyReports => "284",
                CommercialVehicleSafetyAndCredentialsInformationExchange => "285",
                CommercialVehicleCredentials => "286",
                WageDetermination => "288",
                CooperativeAdvertisingAgreements => "290",
                Code300 => "300",
                Code301 => "301",
                Code303 => "303",
                ShippingInstructions => "304",
                CustomsManifest => "309",
                Code310 => "310",
                CanadaCustoms => "311",
                Code312 => "312",
                Code313 => "313",
                Code315 => "315",
                DeliveryPickupOrder => "317",
                Terminal => "319",
                TerminalOperationsAndIntermodalRampActivity => "322",
                Code323 => "323",
                Code324 => "324",
                ConsolidationOfGoodsInContainer => "325",
                ConsignmentSummaryList => "326",
                CustomsStatus => "350",
                USCustomsCarrierGeneralOrderStatus => "352",
                CustomsEventsAdvisoryDetails => "353",
                USCustomsAutomatedManifestArchiveStatus => "354",
                USCustomsAcceptanceRejection => "355",
                USCustomsPermitToTransferRequest => "356",
                USCustomsInBond => "357",
                CustomsConsist => "358",
                Code361 => "361",
                CargoInsuranceAdviceOfShipment => "362",
                RailCarrierShipment => "404",
                RailCarrierFreightDetailsAndInvoice => "410",
                TrailerOrContainerRepairBilling => "412",
                RailCarhireSettlements => "414",
                RailCarrierWaybillInterchange => "417",
                RailAdvanceInterchangeConsist => "418",
                AdvanceCarDisposition => "419",
                CarHandling => "420",
                EstimatedTimeOfArrivalAndCarScheduling => "421",
                EquipmentOrder => "422",
                RailIndustrialSwitchList => "423",
                RailCarrierServicesSettlement => "424",
                RailWaybillRequest => "425",
                RailRevenueWaybill => "426",
                RailroadRetirementActivity => "429",
                RailroadStationMasterFile => "431",
                RailDeprescription => "432",
                RailroadReciprocalSwitchFile => "433",
                RailroadMarkRegisterUpdateActivity => "434",
                StandardTransportationCommodityCodeMaster => "435",
                Locomotive => "436",
                RailroadJunctionsAndInterchangesActivity => "437",
                ShipmentWeights => "440",
                RailroadEventReport => "451",
                RailroadProblemLogInquiryOrAdvice => "452",
                RailroadServiceCommitmentAdvice => "453",
                RailroadParameterTraceRegistration => "455",
                RailroadEquipmentInquiryOrAdvice => "456",
                RailroadPriceDistributionRequestOrResponse => "460",
                RailRateReply => "463",
                RateRequest => "466",
                RateDocketJournalLog => "468",
                RailroadClearance => "470",
                RailRouteFileMaintenance => "475",
                RatemakingAction => "485",
                RateDocketExpiration => "486",
                RateGroupDefinition => "490",
                MiscellaneousRates => "492",
                RailScaleRates => "494",
                MedicalEventReporting => "500",
                VendorPerformanceReview => "501",
                PricingHistory => "503",
                ClausesAndProvisions => "504",
                Requisition => "511",
                MaterialObligationValidation => "517",
                IncomeOrAssetOffset => "521",
                MaterialDueInAndReceipt => "527",
                LogisticsReassignment => "536",
                NoticeOfEmploymentStatus => "540",
                ContractAbstract => "561",
                ContractCompletionStatus => "567",
                ContractPaymentManagementReport => "568",
                USCustomsExportShipment => "601",
                TransportationServicesTender => "602",
                ExcavationCommunication => "620",
                Well => "625",
                MaintenanceServiceOrder => "650",
                IntermodalGroupLoadingPlan => "715",
                RequestForRoutingInstructions => "753",
                RoutingInstructions => "754",
                ContractPricingProposal => "805",
                ProjectScheduleReporting => "806",
                Invoice => "810",
                ConsolidatedServiceInvoiceStatement => "811",
                CreditDebitAdjustment => "812",
                ElectronicFilingOfTaxReturnData => "813",
                Code814 => "814",
                CryptographicServiceMessage => "815",
                OrganizationalRelationships => "816",
                CommissionSalesReport => "818",
                JointInterestBillingAndOperatingExpenseStatement => "819",
                PaymentOrderRemittanceAdvice => "820",
                FinancialInformationReporting => "821",
                AccountAnalysis => "822",
                Lockbox => "823",
                ApplicationAdvice => "824",
                TaxInformationExchange => "826",
                FinancialReturnNotice => "827",
                DebitAuthorization => "828",
                PaymentCancellationRequest => "829",
                PlanningScheduleWithReleaseCapability => "830",
                ApplicationControlTotals => "831",
                PriceSalesCatalog => "832",
                MortgageCreditReportOrder => "833",
                BenefitEnrollmentAndMaintenance => "834",
                HealthCareClaimPaymentAdvice => "835",
                ProcurementNotices => "836",
                HealthCareClaim => "837",
                TradingPartnerProfile => "838",
                ProjectCostReporting => "839",
                RequestForQuotation => "840",
                SpecificationsTechnical => "841",
                NonconformanceReport => "842",
                ResponseToRequestForQuotation => "843",
                ProductTransferAccountAdjustment => "844",
                PriceAuthorizationAcknowledgmentStatus => "845",
                InventoryInquiryAdvice => "846",
                MaterialClaim => "847",
                MaterialSafetyDataSheet => "848",
                ResponseToProductTransferAccountAdjustment => "849",
                PurchaseOrder => "850",
                AssetSchedule => "851",
                ProductActivityData => "852",
                RoutingAndCarrierInstruction => "853",
                ShipmentDeliveryDiscrepancy => "854",
                PurchaseOrderAcknowledgment => "855",
                ShipNoticeManifest => "856",
                ShipmentAndBillingNotice => "857",
                Shipment => "858",
                FreightInvoice => "859",
                PurchaseOrderChangeRequestBuyerInitiated => "860",
                ReceivingAdviceAcceptanceCertificate => "861",
                ShippingSchedule => "862",
                ReportOfTestResults => "863",
                TextMessage => "864",
                PurchaseOrderChangeAcknowledgmentRequestSellerInitiated => "865",
                ProductionSequence => "866",
                ProductTransferAndResaleReport => "867",
                ElectronicFormStructure => "868",
                OrderStatusInquiry => "869",
                OrderStatusReport => "870",
                ComponentPartsContent => "871",
                ResidentialMortgageInsuranceApplication => "872",
                CommodityMovementServices => "873",
                CommodityMovementServicesResponse => "874",
                GroceryProductsPurchaseOrder => "875",
                GroceryProductsPurchaseOrderChange => "876",
                ManufacturerCouponFamilyCodeStructure => "877",
                ProductAuthorizationDeAuthorization => "878",
                Price => "879",
                GroceryProductsInvoice => "880",
                ManufacturerCouponRedemptionDetail => "881",
                DirectStoreDeliverySummary => "882",
                MarketDevelopmentFundAllocation => "883",
                MarketDevelopmentFundSettlement => "884",
                RetailAccountCharacteristics => "885",
                CustomerCallReporting => "886",
                CouponNotification => "887",
                ItemMaintenance => "888",
                PromotionAnnouncement => "889",
                DeductionResearchReport => "891",
                ItemInformationRequest => "893",
                DeliveryReturnBaseRecord => "894",
                DeliveryReturnAcknowledgmentOrAdjustment => "895",
                ProductDimensionMaintenance => "896",
                LossOrDamageClaimGeneralCommodities => "920",
                LossOrDamageClaimMotorVehicle => "924",
                ClaimTracer => "925",
                ClaimStatusReportAndTracerReply => "926",
                AutomotiveInspectionDetail => "928",
                WarehouseShippingOrder => "940",
                WarehouseStockTransferShipmentAdvice => "943",
                WarehouseStockTransferReceiptAdvice => "944",
                WarehouseShippingAdvice => "945",
                WarehouseInventoryAdjustmentAdvice => "947",
                FunctionalGroupTotals => "980",
                ResponseToALoadTender => "990",
                SecuredReceiptOrAcknowledgment => "993",
                FileTransfer => "996",
                FunctionalAcknowledgment => "997",
                SetCancellation => "998",
                ImplementationAcknowledgment => "999",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<TransactionSetIdentifierCode> {
        use TransactionSetIdentifierCode::*;
        match code {
            b"100" => Some(InsurancePlanDescription),
            b"101" => Some(NameAndAddressLists),
            b"102" => Some(AssociatedData),
            b"103" => Some(AbandonedPropertyFilings),
            b"104" => Some(AirShipment),
            b"105" => Some(BusinessEntityFilings),
            b"106" => Some(MotorCarrierRateProposal),
            b"107" => Some(RequestForMotorCarrierRateProposal),
            b"108" => Some(ResponseToAMotorCarrierRateProposal),
            b"109" => Some(VesselContentDetails),
            b"110" => Some(AirFreightDetailsAndInvoice),
            b"111" => Some(IndividualInsurancePolicyAndClient),
            b"112" => Some(PropertyDamageReport),
            b"113" => Some(ElectionCampaignAndLobbyistReporting),
            b"120" => Some(VehicleShippingOrder),
            b"121" => Some(VehicleService),
            b"124" => Some(VehicleDamage),
            b"125" => Some(MultilevelRailcarLoadDetails),
            b"126" => Some(VehicleApplicationAdvice),
            b"127" => Some(VehicleBayingOrder),
            b"128" => Some(Dealer),
            b"129" => Some(VehicleCarrierRateUpdate),
            b"130" => Some(Code130),
            b"131" => Some(Code131),
            b"132" => Some(HumanResource),
            b"133" => Some(EducationalInstitutionRecord),
            b"135" => Some(StudentAidOriginationRecord),
            b"138" => Some(EducationalTestingAndProspectRequestAndReport),
            b"139" => Some(StudentLoanGuaranteeResult),
            b"140" => Some(ProductRegistration),
            b"141" => Some(ProductServiceClaimResponse),
            b"142" => Some(ProductServiceClaim),
            b"143" => Some(ProductServiceNotification),
            b"144" => Some(StudentLoanTransferAndStatusVerification),
            b"146" => Some(Code146),
            b"147" => Some(Code147),
            b"148" => Some(Code148),
            b"149" => Some(NoticeOfTaxAdjustmentOrAssessment),
            b"150" => Some(TaxRateNotification),
            b"151" => Some(ElectronicFilingOfTaxReturnDataAcknowledgment),
            b"152" => Some(StatisticalGovernment),
            b"153" => Some(UnemploymentInsuranceTaxClaimOrCharge),
            b"154" => Some(SecuredInterestFiling),
            b"155" => Some(BusinessCreditReport),
            b"157" => Some(NoticeOfPowerOfAttorney),
            b"158" => Some(TaxJurisdictionSourcing),
            b"159" => Some(MotionPictureBookingConfirmation),
            b"160" => Some(TransportationAutomaticEquipmentIdentification),
            b"161" => Some(TrainSheet),
            b"163" => Some(TransportationAppointmentSchedule),
            b"170" => Some(RevenueReceiptsStatement),
            b"175" => Some(CourtAndLawEnforcementNotice),
            b"176" => Some(CourtSubmission),
            b"179" => Some(EnvironmentalComplianceReporting),
            b"180" => Some(ReturnMerchandiseAuthorizationAndNotification),
            b"185" => Some(RoyaltyRegulatoryReport),
            b"186" => Some(InsuranceUnderwritingRequirementsReporting),
            b"187" => Some(PremiumAuditRequestAndReturn),
            b"188" => Some(EducationalCourseInventory),
            b"189" => Some(ApplicationForAdmissionToEducationalInstitutions),
            b"190" => Some(StudentEnrollmentVerification),
            b"191" => Some(StudentLoanPreClaimsAndClaims),
            b"194" => Some(GrantOrAssistanceApplication),
            b"195" => Some(Code195),
            b"196" => Some(ContractorCostDataReporting),
            b"197" => Some(RealEstateTitleEvidence),
            b"198" => Some(LoanVerification),
            b"199" => Some(RealEstateSettlement),
            b"200" => Some(MortgageCreditReport),
            b"201" => Some(ResidentialLoanApplication),
            b"202" => Some(SecondaryMortgageMarketLoanDelivery),
            b"203" => Some(SecondaryMortgageMarketInvestorReport),
            b"204" => Some(MotorCarrierLoadTender),
            b"205" => Some(MortgageNote),
            b"206" => Some(RealEstateInspection),
            b"210" => Some(MotorCarrierFreightDetailsAndInvoice),
            b"211" => Some(MotorCarrierBillOfLading),
            b"212" => Some(MotorCarrierDeliveryTrailerManifest),
            b"213" => Some(MotorCarrierShipmentStatusInquiry),
            b"214" => Some(TransportationCarrierShipmentStatusMessage),
            b"215" => Some(MotorCarrierPickupManifest),
            b"216" => Some(MotorCarrierShipmentPickupNotification),
            b"217" => Some(MotorCarrierLoadingAndRouteGuide),
            b"219" => Some(LogisticsServiceRequest),
            b"220" => Some(LogisticsServiceResponse),
            b"222" => Some(CartageWorkAssignment),
            b"223" => Some(ConsolidatorsFreightBillAndInvoice),
            b"224" => Some(MotorCarrierSummaryFreightBillManifest),
            b"225" => Some(ResponseToACartageWorkAssignment),
            b"227" => Some(TrailerUsageReport),
            b"228" => Some(EquipmentInspectionReport),
            b"240" => Some(MotorCarrierPackageStatus),
            b"242" => Some(DataStatusTracking),
            b"244" => Some(ProductSource),
            b"245" => Some(RealEstateTaxServiceResponse),
            b"248" => Some(AccountAssignmentInquiryAndServiceStatus),
            b"249" => Some(AnimalToxicologicalData),
            b"250" => Some(PurchaseOrderShipmentManagementDocument),
            b"251" => Some(PricingSupport),
            b"252" => Some(InsuranceProducerAdministration),
            b"255" => Some(UnderwritingInformationServices),
            b"256" => Some(PeriodicCompensation),
            b"259" => Some(ResidentialMortgageInsuranceExplanationOfBenefits),
            b"260" => Some(ApplicationForMortgageInsuranceBenefits),
            b"261" => Some(RealEstateInformationRequest),
            b"262" => Some(RealEstateInformationReport),
            b"263" => Some(ResidentialMortgageInsuranceApplicationResponse),
            b"264" => Some(MortgageLoanDefaultStatus),
            b"265" => Some(RealEstateTitleInsuranceServicesOrder),
            b"266" => Some(MortgageOrPropertyRecordChangeNotification),
            b"267" => Some(Code267),
            b"268" => Some(AnnuityActivity),
            b"269" => Some(HealthCareBenefitCoordinationVerification),
            b"270" => Some(Code270),
            b"271" => Some(Code271),
            b"272" => Some(PropertyAndCasualtyLossNotification),
            b"273" => Some(InsuranceAnnuityApplicationStatus),
            b"274" => Some(HealthcareProvider),
            b"275" => Some(Patient),
            b"276" => Some(HealthCareClaimStatusRequest),
            b"277" => Some(HealthCareInformationStatusNotification),
            b"278" => Some(HealthCareServicesReview),
            b"280" => Some(VoterRegistration),
            b"283" => Some(TaxOrFeeExemptionCertification),
            b"284" => Some(CommercialVehicleSafetyReports),
            b"285" => Some(CommercialVehicleSafetyAndCredentialsInformationExchange),
            b"286" => Some(CommercialVehicleCredentials),
            b"288" => Some(WageDetermination),
            b"290" => Some(CooperativeAdvertisingAgreements),
            b"300" => Some(Code300),
            b"301" => Some(Code301),
            b"303" => Some(Code303),
            b"304" => Some(ShippingInstructions),
            b"309" => Some(CustomsManifest),
            b"310" => Some(Code310),
            b"311" => Some(CanadaCustoms),
            b"312" => Some(Code312),
            b"313" => Some(Code313),
            b"315" => Some(Code315),
            b"317" => Some(DeliveryPickupOrder),
            b"319" => Some(Terminal),
            b"322" => Some(TerminalOperationsAndIntermodalRampActivity),
            b"323" => Some(Code323),
            b"324" => Some(Code324),
            b"325" => Some(ConsolidationOfGoodsInContainer),
            b"326" => Some(ConsignmentSummaryList),
            b"350" => Some(CustomsStatus),
            b"352" => Some(USCustomsCarrierGeneralOrderStatus),
            b"353" => Some(CustomsEventsAdvisoryDetails),
            b"354" => Some(USCustomsAutomatedManifestArchiveStatus),
            b"355" => Some(USCustomsAcceptanceRejection),
            b"356" => Some(USCustomsPermitToTransferRequest),
            b"357" => Some(USCustomsInBond),
            b"358" => Some(CustomsConsist),
            b"361" => Some(Code361),
            b"362" => Some(CargoInsuranceAdviceOfShipment),
            b"404" => Some(RailCarrierShipment),
            b"410" => Some(RailCarrierFreightDetailsAndInvoice),
            b"412" => Some(TrailerOrContainerRepairBilling),
            b"414" => Some(RailCarhireSettlements),
            b"417" => Some(RailCarrierWaybillInterchange),
            b"418" => Some(RailAdvanceInterchangeConsist),
            b"419" => Some(AdvanceCarDisposition),
            b"420" => Some(CarHandling),
            b"421" => Some(EstimatedTimeOfArrivalAndCarScheduling),
            b"422" => Some(EquipmentOrder),
            b"423" => Some(RailIndustrialSwitchList),
            b"424" => Some(RailCarrierServicesSettlement),
            b"425" => Some(RailWaybillRequest),
            b"426" => Some(RailRevenueWaybill),
            b"429" => Some(RailroadRetirementActivity),
            b"431" => Some(RailroadStationMasterFile),
            b"432" => Some(RailDeprescription),
            b"433" => Some(RailroadReciprocalSwitchFile),
            b"434" => Some(RailroadMarkRegisterUpdateActivity),
            b"435" => Some(StandardTransportationCommodityCodeMaster),
            b"436" => Some(Locomotive),
            b"437" => Some(RailroadJunctionsAndInterchangesActivity),
            b"440" => Some(ShipmentWeights),
            b"451" => Some(RailroadEventReport),
            b"452" => Some(RailroadProblemLogInquiryOrAdvice),
            b"453" => Some(RailroadServiceCommitmentAdvice),
            b"455" => Some(RailroadParameterTraceRegistration),
            b"456" => Some(RailroadEquipmentInquiryOrAdvice),
            b"460" => Some(RailroadPriceDistributionRequestOrResponse),
            b"463" => Some(RailRateReply),
            b"466" => Some(RateRequest),
            b"468" => Some(RateDocketJournalLog),
            b"470" => Some(RailroadClearance),
            b"475" => Some(RailRouteFileMaintenance),
            b"485" => Some(RatemakingAction),
            b"486" => Some(RateDocketExpiration),
            b"490" => Some(RateGroupDefinition),
            b"492" => Some(MiscellaneousRates),
            b"494" => Some(RailScaleRates),
            b"500" => Some(MedicalEventReporting),
            b"501" => Some(VendorPerformanceReview),
            b"503" => Some(PricingHistory),
            b"504" => Some(ClausesAndProvisions),
            b"511" => Some(Requisition),
            b"517" => Some(MaterialObligationValidation),
            b"521" => Some(IncomeOrAssetOffset),
            b"527" => Some(MaterialDueInAndReceipt),
            b"536" => Some(LogisticsReassignment),
            b"540" => Some(NoticeOfEmploymentStatus),
            b"561" => Some(ContractAbstract),
            b"567" => Some(ContractCompletionStatus),
            b"568" => Some(ContractPaymentManagementReport),
            b"601" => Some(USCustomsExportShipment),
            b"602" => Some(TransportationServicesTender),
            b"620" => Some(ExcavationCommunication),
            b"625" => Some(Well),
            b"650" => Some(MaintenanceServiceOrder),
            b"715" => Some(IntermodalGroupLoadingPlan),
            b"753" => Some(RequestForRoutingInstructions),
            b"754" => Some(RoutingInstructions),
            b"805" => Some(ContractPricingProposal),
            b"806" => Some(ProjectScheduleReporting),
            b"810" => Some(Invoice),
            b"811" => Some(ConsolidatedServiceInvoiceStatement),
            b"812" => Some(CreditDebitAdjustment),
            b"813" => Some(ElectronicFilingOfTaxReturnData),
            b"814" => Some(Code814),
            b"815" => Some(CryptographicServiceMessage),
            b"816" => Some(OrganizationalRelationships),
            b"818" => Some(CommissionSalesReport),
            b"819" => Some(JointInterestBillingAndOperatingExpenseStatement),
            b"820" => Some(PaymentOrderRemittanceAdvice),
            b"821" => Some(FinancialInformationReporting),
            b"822" => Some(AccountAnalysis),
            b"823" => Some(Lockbox),
            b"824" => Some(ApplicationAdvice),
            b"826" => Some(TaxInformationExchange),
            b"827" => Some(FinancialReturnNotice),
            b"828" => Some(DebitAuthorization),
            b"829" => Some(PaymentCancellationRequest),
            b"830" => Some(PlanningScheduleWithReleaseCapability),
            b"831" => Some(ApplicationControlTotals),
            b"832" => Some(PriceSalesCatalog),
            b"833" => Some(MortgageCreditReportOrder),
            b"834" => Some(BenefitEnrollmentAndMaintenance),
            b"835" => Some(HealthCareClaimPaymentAdvice),
            b"836" => Some(ProcurementNotices),
            b"837" => Some(HealthCareClaim),
            b"838" => Some(TradingPartnerProfile),
            b"839" => Some(ProjectCostReporting),
            b"840" => Some(RequestForQuotation),
            b"841" => Some(SpecificationsTechnical),
            b"842" => Some(NonconformanceReport),
            b"843" => Some(ResponseToRequestForQuotation),
            b"844" => Some(ProductTransferAccountAdjustment),
            b"845" => Some(PriceAuthorizationAcknowledgmentStatus),
            b"846" => Some(InventoryInquiryAdvice),
            b"847" => Some(MaterialClaim),
            b"848" => Some(MaterialSafetyDataSheet),
            b"849" => Some(ResponseToProductTransferAccountAdjustment),
            b"850" => Some(PurchaseOrder),
            b"851" => Some(AssetSchedule),
            b"852" => Some(ProductActivityData),
            b"853" => Some(RoutingAndCarrierInstruction),
            b"854" => Some(ShipmentDeliveryDiscrepancy),
            b"855" => Some(PurchaseOrderAcknowledgment),
            b"856" => Some(ShipNoticeManifest),
            b"857" => Some(ShipmentAndBillingNotice),
            b"858" => Some(Shipment),
            b"859" => Some(FreightInvoice),
            b"860" => Some(PurchaseOrderChangeRequestBuyerInitiated),
            b"861" => Some(ReceivingAdviceAcceptanceCertificate),
            b"862" => Some(ShippingSchedule),
            b"863" => Some(ReportOfTestResults),
            b"864" => Some(TextMessage),
            b"865" => Some(PurchaseOrderChangeAcknowledgmentRequestSellerInitiated),
            b"866" => Some(ProductionSequence),
            b"867" => Some(ProductTransferAndResaleReport),
            b"868" => Some(ElectronicFormStructure),
            b"869" => Some(OrderStatusInquiry),
            b"870" => Some(OrderStatusReport),
            b"871" => Some(ComponentPartsContent),
            b"872" => Some(ResidentialMortgageInsuranceApplication),
            b"873" => Some(CommodityMovementServices),
            b"874" => Some(CommodityMovementServicesResponse),
            b"875" => Some(GroceryProductsPurchaseOrder),
            b"876" => Some(GroceryProductsPurchaseOrderChange),
            b"877" => Some(ManufacturerCouponFamilyCodeStructure),
            b"878" => Some(ProductAuthorizationDeAuthorization),
            b"879" => Some(Price),
            b"880" => Some(GroceryProductsInvoice),
            b"881" => Some(ManufacturerCouponRedemptionDetail),
            b"882" => Some(DirectStoreDeliverySummary),
            b"883" => Some(MarketDevelopmentFundAllocation),
            b"884" => Some(MarketDevelopmentFundSettlement),
            b"885" => Some(RetailAccountCharacteristics),
            b"886" => Some(CustomerCallReporting),
            b"887" => Some(CouponNotification),
            b"888" => Some(ItemMaintenance),
            b"889" => Some(PromotionAnnouncement),
            b"891" => Some(DeductionResearchReport),
            b"893" => Some(ItemInformationRequest),
            b"894" => Some(DeliveryReturnBaseRecord),
            b"895" => Some(DeliveryReturnAcknowledgmentOrAdjustment),
            b"896" => Some(ProductDimensionMaintenance),
            b"920" => Some(LossOrDamageClaimGeneralCommodities),
            b"924" => Some(LossOrDamageClaimMotorVehicle),
            b"925" => Some(ClaimTracer),
            b"926" => Some(ClaimStatusReportAndTracerReply),
            b"928" => Some(AutomotiveInspectionDetail),
            b"940" => Some(WarehouseShippingOrder),
            b"943" => Some(WarehouseStockTransferShipmentAdvice),
            b"944" => Some(WarehouseStockTransferReceiptAdvice),
            b"945" => Some(WarehouseShippingAdvice),
            b"947" => Some(WarehouseInventoryAdjustmentAdvice),
            b"980" => Some(FunctionalGroupTotals),
            b"990" => Some(ResponseToALoadTender),
            b"993" => Some(SecuredReceiptOrAcknowledgment),
            b"996" => Some(FileTransfer),
            b"997" => Some(FunctionalAcknowledgment),
            b"998" => Some(SetCancellation),
            b"999" => Some(ImplementationAcknowledgment),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use TransactionSetIdentifierCode::*;
        match self {
            InsurancePlanDescription => "Insurance Plan Description",
            NameAndAddressLists => "Name and Address Lists",
            AssociatedData => "Associated Data",
            AbandonedPropertyFilings => "Abandoned Property Filings",
            AirShipment => "Air Shipment Information",
            BusinessEntityFilings => "Business Entity Filings",
            MotorCarrierRateProposal => "Motor Carrier Rate Proposal",
            RequestForMotorCarrierRateProposal => {
                "Request for Motor Carrier Rate Proposal"
            }
            ResponseToAMotorCarrierRateProposal => {
                "Response to a Motor Carrier Rate Proposal"
            }
            VesselContentDetails => "Vessel Content Details",
            AirFreightDetailsAndInvoice => "Air Freight Details and Invoice",
            IndividualInsurancePolicyAndClient => {
                "Individual Insurance Policy and Client Information"
            }
            PropertyDamageReport => "Property Damage Report",
            ElectionCampaignAndLobbyistReporting => {
                "Election Campaign and Lobbyist Reporting"
            }
            VehicleShippingOrder => "Vehicle Shipping Order",
            VehicleService => "Vehicle Service",
            VehicleDamage => "Vehicle Damage",
            MultilevelRailcarLoadDetails => "Multilevel Railcar Load Details",
            VehicleApplicationAdvice => "Vehicle Application Advice",
            VehicleBayingOrder => "Vehicle Baying Order",
            Dealer => "Dealer Information",
            VehicleCarrierRateUpdate => "Vehicle Carrier Rate Update",
            Code130 => "Student Educational Record (Transcript)",
            Code131 => "Student Educational Record (Transcript) Acknowledgment",
            HumanResource => "Human Resource Information",
            EducationalInstitutionRecord => "Educational Institution Record",
            StudentAidOriginationRecord => "Student Aid Origination Record",
            EducationalTestingAndProspectRequestAndReport => {
                "Educational Testing and Prospect Request and Report"
            }
            StudentLoanGuaranteeResult => "Student Loan Guarantee Result",
            ProductRegistration => "Product Registration",
            ProductServiceClaimResponse => "Product Service Claim Response",
            ProductServiceClaim => "Product Service Claim",
            ProductServiceNotification => "Product Service Notification",
            StudentLoanTransferAndStatusVerification => {
                "Student Loan Transfer and Status Verification"
            }
            Code146 => "Request for Student Educational Record (Transcript)",
            Code147 => "Response to Request for Student Educational Record (Transcript)",
            Code148 => "Report of Injury, Illness or Incident",
            NoticeOfTaxAdjustmentOrAssessment => "Notice of Tax Adjustment or Assessment",
            TaxRateNotification => "Tax Rate Notification",
            ElectronicFilingOfTaxReturnDataAcknowledgment => {
                "Electronic Filing of Tax Return Data Acknowledgment"
            }
            StatisticalGovernment => "Statistical Government Information",
            UnemploymentInsuranceTaxClaimOrCharge => {
                "Unemployment Insurance Tax Claim or Charge Information"
            }
            SecuredInterestFiling => "Secured Interest Filing",
            BusinessCreditReport => "Business Credit Report",
            NoticeOfPowerOfAttorney => "Notice of Power of Attorney",
            TaxJurisdictionSourcing => "Tax Jurisdiction Sourcing",
            MotionPictureBookingConfirmation => "Motion Picture Booking Confirmation",
            TransportationAutomaticEquipmentIdentification => {
                "Transportation Automatic Equipment Identification"
            }
            TrainSheet => "Train Sheet",
            TransportationAppointmentSchedule => {
                "Transportation Appointment Schedule Information"
            }
            RevenueReceiptsStatement => "Revenue Receipts Statement",
            CourtAndLawEnforcementNotice => "Court and Law Enforcement Notice",
            CourtSubmission => "Court Submission",
            EnvironmentalComplianceReporting => "Environmental Compliance Reporting",
            ReturnMerchandiseAuthorizationAndNotification => {
                "Return Merchandise Authorization and Notification"
            }
            RoyaltyRegulatoryReport => "Royalty Regulatory Report",
            InsuranceUnderwritingRequirementsReporting => {
                "Insurance Underwriting Requirements Reporting"
            }
            PremiumAuditRequestAndReturn => "Premium Audit Request and Return",
            EducationalCourseInventory => "Educational Course Inventory",
            ApplicationForAdmissionToEducationalInstitutions => {
                "Application for Admission to Educational Institutions"
            }
            StudentEnrollmentVerification => "Student Enrollment Verification",
            StudentLoanPreClaimsAndClaims => "Student Loan Pre-Claims and Claims",
            GrantOrAssistanceApplication => "Grant or Assistance Application",
            Code195 => "Federal Communications Commission (FCC) License Application",
            ContractorCostDataReporting => "Contractor Cost Data Reporting",
            RealEstateTitleEvidence => "Real Estate Title Evidence",
            LoanVerification => "Loan Verification Information",
            RealEstateSettlement => "Real Estate Settlement Information",
            MortgageCreditReport => "Mortgage Credit Report",
            ResidentialLoanApplication => "Residential Loan Application",
            SecondaryMortgageMarketLoanDelivery => {
                "Secondary Mortgage Market Loan Delivery"
            }
            SecondaryMortgageMarketInvestorReport => {
                "Secondary Mortgage Market Investor Report"
            }
            MotorCarrierLoadTender => "Motor Carrier Load Tender",
            MortgageNote => "Mortgage Note",
            RealEstateInspection => "Real Estate Inspection",
            MotorCarrierFreightDetailsAndInvoice => {
                "Motor Carrier Freight Details and Invoice"
            }
            MotorCarrierBillOfLading => "Motor Carrier Bill of Lading",
            MotorCarrierDeliveryTrailerManifest => {
                "Motor Carrier Delivery Trailer Manifest"
            }
            MotorCarrierShipmentStatusInquiry => "Motor Carrier Shipment Status Inquiry",
            TransportationCarrierShipmentStatusMessage => {
                "Transportation Carrier Shipment Status Message"
            }
            MotorCarrierPickupManifest => "Motor Carrier Pickup Manifest",
            MotorCarrierShipmentPickupNotification => {
                "Motor Carrier Shipment Pickup Notification"
            }
            MotorCarrierLoadingAndRouteGuide => "Motor Carrier Loading and Route Guide",
            LogisticsServiceRequest => "Logistics Service Request",
            LogisticsServiceResponse => "Logistics Service Response",
            CartageWorkAssignment => "Cartage Work Assignment",
            ConsolidatorsFreightBillAndInvoice => {
                "Consolidators Freight Bill and Invoice"
            }
            MotorCarrierSummaryFreightBillManifest => {
                "Motor Carrier Summary Freight Bill Manifest"
            }
            ResponseToACartageWorkAssignment => "Response to a Cartage Work Assignment",
            TrailerUsageReport => "Trailer Usage Report",
            EquipmentInspectionReport => "Equipment Inspection Report",
            MotorCarrierPackageStatus => "Motor Carrier Package Status",
            DataStatusTracking => "Data Status Tracking",
            ProductSource => "Product Source Information",
            RealEstateTaxServiceResponse => "Real Estate Tax Service Response",
            AccountAssignmentInquiryAndServiceStatus => {
                "Account Assignment/Inquiry and Service/Status"
            }
            AnimalToxicologicalData => "Animal Toxicological Data",
            PurchaseOrderShipmentManagementDocument => {
                "Purchase Order Shipment Management Document"
            }
            PricingSupport => "Pricing Support",
            InsuranceProducerAdministration => "Insurance Producer Administration",
            UnderwritingInformationServices => "Underwriting Information Services",
            PeriodicCompensation => "Periodic Compensation",
            ResidentialMortgageInsuranceExplanationOfBenefits => {
                "Residential Mortgage Insurance Explanation of Benefits"
            }
            ApplicationForMortgageInsuranceBenefits => {
                "Application for Mortgage Insurance Benefits"
            }
            RealEstateInformationRequest => "Real Estate Information Request",
            RealEstateInformationReport => "Real Estate Information Report",
            ResidentialMortgageInsuranceApplicationResponse => {
                "Residential Mortgage Insurance Application Response"
            }
            MortgageLoanDefaultStatus => "Mortgage Loan Default Status",
            RealEstateTitleInsuranceServicesOrder => {
                "Real Estate Title Insurance Services Order"
            }
            MortgageOrPropertyRecordChangeNotification => {
                "Mortgage or Property Record Change Notification"
            }
            Code267 => "Individual Life, Annuity and Disability Application",
            AnnuityActivity => "Annuity Activity",
            HealthCareBenefitCoordinationVerification => {
                "Health Care Benefit Coordination Verification"
            }
            Code270 => "Eligibility, Coverage or Benefit Inquiry",
            Code271 => "Eligibility, Coverage or Benefit Information",
            PropertyAndCasualtyLossNotification => {
                "Property and Casualty Loss Notification"
            }
            InsuranceAnnuityApplicationStatus => "Insurance/Annuity Application Status",
            HealthcareProvider => "Healthcare Provider Information",
            Patient => "Patient Information",
            HealthCareClaimStatusRequest => "Health Care Claim Status Request",
            HealthCareInformationStatusNotification => {
                "Health Care Information Status Notification"
            }
            HealthCareServicesReview => "Health Care Services Review Information",
            VoterRegistration => "Voter Registration Information",
            TaxOrFeeExemptionCertification => "Tax or Fee Exemption Certification",
            CommercialVehicleSafetyReports => "Commercial Vehicle Safety Reports",
            CommercialVehicleSafetyAndCredentialsInformationExchange => {
                "Commercial Vehicle Safety and Credentials Information Exchange"
            }
            CommercialVehicleCredentials => "Commercial Vehicle Credentials",
            WageDetermination => "Wage Determination",
            CooperativeAdvertisingAgreements => "Cooperative Advertising Agreements",
            Code300 => "Reservation (Booking Request) (Ocean)",
            Code301 => "Confirmation (Ocean)",
            Code303 => "Booking Cancellation (Ocean)",
            ShippingInstructions => "Shipping Instructions",
            CustomsManifest => "Customs Manifest",
            Code310 => "Freight Receipt and Invoice (Ocean)",
            CanadaCustoms => "Canada Customs Information",
            Code312 => "Arrival Notice (Ocean)",
            Code313 => "Shipment Status Inquiry (Ocean)",
            Code315 => "Status Details (Ocean)",
            DeliveryPickupOrder => "Delivery/Pickup Order",
            Terminal => "Terminal Information",
            TerminalOperationsAndIntermodalRampActivity => {
                "Terminal Operations and Intermodal Ramp Activity"
            }
            Code323 => "Vessel Schedule and Itinerary (Ocean)",
            Code324 => "Vessel Stow Plan (Ocean)",
            ConsolidationOfGoodsInContainer => "Consolidation of Goods In Container",
            ConsignmentSummaryList => "Consignment Summary List",
            CustomsStatus => "Customs Status Information",
            USCustomsCarrierGeneralOrderStatus => {
                "U.S. Customs Carrier General Order Status"
            }
            CustomsEventsAdvisoryDetails => "Customs Events Advisory Details",
            USCustomsAutomatedManifestArchiveStatus => {
                "U.S. Customs Automated Manifest Archive Status"
            }
            USCustomsAcceptanceRejection => "U.S. Customs Acceptance/Rejection",
            USCustomsPermitToTransferRequest => "U.S. Customs Permit to Transfer Request",
            USCustomsInBond => "U.S. Customs In-Bond Information",
            CustomsConsist => "Customs Consist Information",
            Code361 => "Carrier Interchange Agreement (Ocean)",
            CargoInsuranceAdviceOfShipment => "Cargo Insurance Advice of Shipment",
            RailCarrierShipment => "Rail Carrier Shipment Information",
            RailCarrierFreightDetailsAndInvoice => {
                "Rail Carrier Freight Details and Invoice"
            }
            TrailerOrContainerRepairBilling => "Trailer or Container Repair Billing",
            RailCarhireSettlements => "Rail Carhire Settlements",
            RailCarrierWaybillInterchange => "Rail Carrier Waybill Interchange",
            RailAdvanceInterchangeConsist => "Rail Advance Interchange Consist",
            AdvanceCarDisposition => "Advance Car Disposition",
            CarHandling => "Car Handling Information",
            EstimatedTimeOfArrivalAndCarScheduling => {
                "Estimated Time of Arrival and Car Scheduling"
            }
            EquipmentOrder => "Equipment Order",
            RailIndustrialSwitchList => "Rail Industrial Switch List",
            RailCarrierServicesSettlement => "Rail Carrier Services Settlement",
            RailWaybillRequest => "Rail Waybill Request",
            RailRevenueWaybill => "Rail Revenue Waybill",
            RailroadRetirementActivity => "Railroad Retirement Activity",
            RailroadStationMasterFile => "Railroad Station Master File",
            RailDeprescription => "Rail Deprescription",
            RailroadReciprocalSwitchFile => "Railroad Reciprocal Switch File",
            RailroadMarkRegisterUpdateActivity => {
                "Railroad Mark Register Update Activity"
            }
            StandardTransportationCommodityCodeMaster => {
                "Standard Transportation Commodity Code Master"
            }
            Locomotive => "Locomotive Information",
            RailroadJunctionsAndInterchangesActivity => {
                "Railroad Junctions and Interchanges Activity"
            }
            ShipmentWeights => "Shipment Weights",
            RailroadEventReport => "Railroad Event Report",
            RailroadProblemLogInquiryOrAdvice => "Railroad Problem Log Inquiry or Advice",
            RailroadServiceCommitmentAdvice => "Railroad Service Commitment Advice",
            RailroadParameterTraceRegistration => "Railroad Parameter Trace Registration",
            RailroadEquipmentInquiryOrAdvice => "Railroad Equipment Inquiry or Advice",
            RailroadPriceDistributionRequestOrResponse => {
                "Railroad Price Distribution Request or Response"
            }
            RailRateReply => "Rail Rate Reply",
            RateRequest => "Rate Request",
            RateDocketJournalLog => "Rate Docket Journal Log",
            RailroadClearance => "Railroad Clearance",
            RailRouteFileMaintenance => "Rail Route File Maintenance",
            RatemakingAction => "Ratemaking Action",
            RateDocketExpiration => "Rate Docket Expiration",
            RateGroupDefinition => "Rate Group Definition",
            MiscellaneousRates => "Miscellaneous Rates",
            RailScaleRates => "Rail Scale Rates",
            MedicalEventReporting => "Medical Event Reporting",
            VendorPerformanceReview => "Vendor Performance Review",
            PricingHistory => "Pricing History",
            ClausesAndProvisions => "Clauses and Provisions",
            Requisition => "Requisition",
            MaterialObligationValidation => "Material Obligation Validation",
            IncomeOrAssetOffset => "Income or Asset Offset",
            MaterialDueInAndReceipt => "Material Due-In and Receipt",
            LogisticsReassignment => "Logistics Reassignment",
            NoticeOfEmploymentStatus => "Notice of Employment Status",
            ContractAbstract => "Contract Abstract",
            ContractCompletionStatus => "Contract Completion Status",
            ContractPaymentManagementReport => "Contract Payment Management Report",
            USCustomsExportShipment => "U.S. Customs Export Shipment Information",
            TransportationServicesTender => "Transportation Services Tender",
            ExcavationCommunication => "Excavation Communication",
            Well => "Well Information",
            MaintenanceServiceOrder => "Maintenance Service Order",
            IntermodalGroupLoadingPlan => "Intermodal Group Loading Plan",
            RequestForRoutingInstructions => "Request for Routing Instructions",
            RoutingInstructions => "Routing Instructions",
            ContractPricingProposal => "Contract Pricing Proposal",
            ProjectScheduleReporting => "Project Schedule Reporting",
            Invoice => "Invoice",
            ConsolidatedServiceInvoiceStatement => {
                "Consolidated Service Invoice/Statement"
            }
            CreditDebitAdjustment => "Credit/Debit Adjustment",
            ElectronicFilingOfTaxReturnData => "Electronic Filing of Tax Return Data",
            Code814 => "General Request, Response or Confirmation",
            CryptographicServiceMessage => "Cryptographic Service Message",
            OrganizationalRelationships => "Organizational Relationships",
            CommissionSalesReport => "Commission Sales Report",
            JointInterestBillingAndOperatingExpenseStatement => {
                "Joint Interest Billing and Operating Expense Statement"
            }
            PaymentOrderRemittanceAdvice => "Payment Order/Remittance Advice",
            FinancialInformationReporting => "Financial Information Reporting",
            AccountAnalysis => "Account Analysis",
            Lockbox => "Lockbox",
            ApplicationAdvice => "Application Advice",
            TaxInformationExchange => "Tax Information Exchange",
            FinancialReturnNotice => "Financial Return Notice",
            DebitAuthorization => "Debit Authorization",
            PaymentCancellationRequest => "Payment Cancellation Request",
            PlanningScheduleWithReleaseCapability => {
                "Planning Schedule with Release Capability"
            }
            ApplicationControlTotals => "Application Control Totals",
            PriceSalesCatalog => "Price/Sales Catalog",
            MortgageCreditReportOrder => "Mortgage Credit Report Order",
            BenefitEnrollmentAndMaintenance => "Benefit Enrollment and Maintenance",
            HealthCareClaimPaymentAdvice => "Health Care Claim Payment/Advice",
            ProcurementNotices => "Procurement Notices",
            HealthCareClaim => "Health Care Claim",
            TradingPartnerProfile => "Trading Partner Profile",
            ProjectCostReporting => "Project Cost Reporting",
            RequestForQuotation => "Request for Quotation",
            SpecificationsTechnical => "Specifications/Technical Information",
            NonconformanceReport => "Nonconformance Report",
            ResponseToRequestForQuotation => "Response to Request for Quotation",
            ProductTransferAccountAdjustment => "Product Transfer Account Adjustment",
            PriceAuthorizationAcknowledgmentStatus => {
                "Price Authorization Acknowledgment/Status"
            }
            InventoryInquiryAdvice => "Inventory Inquiry/Advice",
            MaterialClaim => "Material Claim",
            MaterialSafetyDataSheet => "Material Safety Data Sheet",
            ResponseToProductTransferAccountAdjustment => {
                "Response to Product Transfer Account Adjustment"
            }
            PurchaseOrder => "Purchase Order",
            AssetSchedule => "Asset Schedule",
            ProductActivityData => "Product Activity Data",
            RoutingAndCarrierInstruction => "Routing and Carrier Instruction",
            ShipmentDeliveryDiscrepancy => "Shipment Delivery Discrepancy Information",
            PurchaseOrderAcknowledgment => "Purchase Order Acknowledgment",
            ShipNoticeManifest => "Ship Notice/Manifest",
            ShipmentAndBillingNotice => "Shipment and Billing Notice",
            Shipment => "Shipment Information",
            FreightInvoice => "Freight Invoice",
            PurchaseOrderChangeRequestBuyerInitiated => {
                "Purchase Order Change Request - Buyer Initiated"
            }
            ReceivingAdviceAcceptanceCertificate => {
                "Receiving Advice/Acceptance Certificate"
            }
            ShippingSchedule => "Shipping Schedule",
            ReportOfTestResults => "Report of Test Results",
            TextMessage => "Text Message",
            PurchaseOrderChangeAcknowledgmentRequestSellerInitiated => {
                "Purchase Order Change Acknowledgment/Request - Seller Initiated"
            }
            ProductionSequence => "Production Sequence",
            ProductTransferAndResaleReport => "Product Transfer and Resale Report",
            ElectronicFormStructure => "Electronic Form Structure",
            OrderStatusInquiry => "Order Status Inquiry",
            OrderStatusReport => "Order Status Report",
            ComponentPartsContent => "Component Parts Content",
            ResidentialMortgageInsuranceApplication => {
                "Residential Mortgage Insurance Application"
            }
            CommodityMovementServices => "Commodity Movement Services",
            CommodityMovementServicesResponse => "Commodity Movement Services Response",
            GroceryProductsPurchaseOrder => "Grocery Products Purchase Order",
            GroceryProductsPurchaseOrderChange => {
                "Grocery Products Purchase Order Change"
            }
            ManufacturerCouponFamilyCodeStructure => {
                "Manufacturer Coupon Family Code Structure"
            }
            ProductAuthorizationDeAuthorization => {
                "Product Authorization/De-authorization"
            }
            Price => "Price Information",
            GroceryProductsInvoice => "Grocery Products Invoice",
            ManufacturerCouponRedemptionDetail => "Manufacturer Coupon Redemption Detail",
            DirectStoreDeliverySummary => "Direct Store Delivery Summary Information",
            MarketDevelopmentFundAllocation => "Market Development Fund Allocation",
            MarketDevelopmentFundSettlement => "Market Development Fund Settlement",
            RetailAccountCharacteristics => "Retail Account Characteristics",
            CustomerCallReporting => "Customer Call Reporting",
            CouponNotification => "Coupon Notification",
            ItemMaintenance => "Item Maintenance",
            PromotionAnnouncement => "Promotion Announcement",
            DeductionResearchReport => "Deduction Research Report",
            ItemInformationRequest => "Item Information Request",
            DeliveryReturnBaseRecord => "Delivery/Return Base Record",
            DeliveryReturnAcknowledgmentOrAdjustment => {
                "Delivery/Return Acknowledgment or Adjustment"
            }
            ProductDimensionMaintenance => "Product Dimension Maintenance",
            LossOrDamageClaimGeneralCommodities => {
                "Loss or Damage Claim - General Commodities"
            }
            LossOrDamageClaimMotorVehicle => "Loss or Damage Claim - Motor Vehicle",
            ClaimTracer => "Claim Tracer",
            ClaimStatusReportAndTracerReply => "Claim Status Report and Tracer Reply",
            AutomotiveInspectionDetail => "Automotive Inspection Detail",
            WarehouseShippingOrder => "Warehouse Shipping Order",
            WarehouseStockTransferShipmentAdvice => {
                "Warehouse Stock Transfer Shipment Advice"
            }
            WarehouseStockTransferReceiptAdvice => {
                "Warehouse Stock Transfer Receipt Advice"
            }
            WarehouseShippingAdvice => "Warehouse Shipping Advice",
            WarehouseInventoryAdjustmentAdvice => "Warehouse Inventory Adjustment Advice",
            FunctionalGroupTotals => "Functional Group Totals",
            ResponseToALoadTender => "Response to a Load Tender",
            SecuredReceiptOrAcknowledgment => "Secured Receipt or Acknowledgment",
            FileTransfer => "File Transfer",
            FunctionalAcknowledgment => "Functional Acknowledgment",
            SetCancellation => "Set Cancellation",
            ImplementationAcknowledgment => "Implementation Acknowledgment",
        }
    }
    fn from_description(description: &str) -> Option<TransactionSetIdentifierCode> {
        {
            use TransactionSetIdentifierCode::*;
            match description {
                "Insurance Plan Description" => Some(InsurancePlanDescription),
                "Name and Address Lists" => Some(NameAndAddressLists),
                "Associated Data" => Some(AssociatedData),
                "Abandoned Property Filings" => Some(AbandonedPropertyFilings),
                "Air Shipment Information" => Some(AirShipment),
                "Business Entity Filings" => Some(BusinessEntityFilings),
                "Motor Carrier Rate Proposal" => Some(MotorCarrierRateProposal),
                "Request for Motor Carrier Rate Proposal" => {
                    Some(RequestForMotorCarrierRateProposal)
                }
                "Response to a Motor Carrier Rate Proposal" => {
                    Some(ResponseToAMotorCarrierRateProposal)
                }
                "Vessel Content Details" => Some(VesselContentDetails),
                "Air Freight Details and Invoice" => Some(AirFreightDetailsAndInvoice),
                "Individual Insurance Policy and Client Information" => {
                    Some(IndividualInsurancePolicyAndClient)
                }
                "Property Damage Report" => Some(PropertyDamageReport),
                "Election Campaign and Lobbyist Reporting" => {
                    Some(ElectionCampaignAndLobbyistReporting)
                }
                "Vehicle Shipping Order" => Some(VehicleShippingOrder),
                "Vehicle Service" => Some(VehicleService),
                "Vehicle Damage" => Some(VehicleDamage),
                "Multilevel Railcar Load Details" => Some(MultilevelRailcarLoadDetails),
                "Vehicle Application Advice" => Some(VehicleApplicationAdvice),
                "Vehicle Baying Order" => Some(VehicleBayingOrder),
                "Dealer Information" => Some(Dealer),
                "Vehicle Carrier Rate Update" => Some(VehicleCarrierRateUpdate),
                "Student Educational Record (Transcript)" => Some(Code130),
                "Student Educational Record (Transcript) Acknowledgment" => Some(Code131),
                "Human Resource Information" => Some(HumanResource),
                "Educational Institution Record" => Some(EducationalInstitutionRecord),
                "Student Aid Origination Record" => Some(StudentAidOriginationRecord),
                "Educational Testing and Prospect Request and Report" => {
                    Some(EducationalTestingAndProspectRequestAndReport)
                }
                "Student Loan Guarantee Result" => Some(StudentLoanGuaranteeResult),
                "Product Registration" => Some(ProductRegistration),
                "Product Service Claim Response" => Some(ProductServiceClaimResponse),
                "Product Service Claim" => Some(ProductServiceClaim),
                "Product Service Notification" => Some(ProductServiceNotification),
                "Student Loan Transfer and Status Verification" => {
                    Some(StudentLoanTransferAndStatusVerification)
                }
                "Request for Student Educational Record (Transcript)" => Some(Code146),
                "Response to Request for Student Educational Record (Transcript)" => {
                    Some(Code147)
                }
                "Report of Injury, Illness or Incident" => Some(Code148),
                "Notice of Tax Adjustment or Assessment" => {
                    Some(NoticeOfTaxAdjustmentOrAssessment)
                }
                "Tax Rate Notification" => Some(TaxRateNotification),
                "Electronic Filing of Tax Return Data Acknowledgment" => {
                    Some(ElectronicFilingOfTaxReturnDataAcknowledgment)
                }
                "Statistical Government Information" => Some(StatisticalGovernment),
                "Unemployment Insurance Tax Claim or Charge Information" => {
                    Some(UnemploymentInsuranceTaxClaimOrCharge)
                }
                "Secured Interest Filing" => Some(SecuredInterestFiling),
                "Business Credit Report" => Some(BusinessCreditReport),
                "Notice of Power of Attorney" => Some(NoticeOfPowerOfAttorney),
                "Tax Jurisdiction Sourcing" => Some(TaxJurisdictionSourcing),
                "Motion Picture Booking Confirmation" => {
                    Some(MotionPictureBookingConfirmation)
                }
                "Transportation Automatic Equipment Identification" => {
                    Some(TransportationAutomaticEquipmentIdentification)
                }
                "Train Sheet" => Some(TrainSheet),
                "Transportation Appointment Schedule Information" => {
                    Some(TransportationAppointmentSchedule)
                }
                "Revenue Receipts Statement" => Some(RevenueReceiptsStatement),
                "Court and Law Enforcement Notice" => Some(CourtAndLawEnforcementNotice),
                "Court Submission" => Some(CourtSubmission),
                "Environmental Compliance Reporting" => {
                    Some(EnvironmentalComplianceReporting)
                }
                "Return Merchandise Authorization and Notification" => {
                    Some(ReturnMerchandiseAuthorizationAndNotification)
                }
                "Royalty Regulatory Report" => Some(RoyaltyRegulatoryReport),
                "Insurance Underwriting Requirements Reporting" => {
                    Some(InsuranceUnderwritingRequirementsReporting)
                }
                "Premium Audit Request and Return" => Some(PremiumAuditRequestAndReturn),
                "Educational Course Inventory" => Some(EducationalCourseInventory),
                "Application for Admission to Educational Institutions" => {
                    Some(ApplicationForAdmissionToEducationalInstitutions)
                }
                "Student Enrollment Verification" => Some(StudentEnrollmentVerification),
                "Student Loan Pre-Claims and Claims" => {
                    Some(StudentLoanPreClaimsAndClaims)
                }
                "Grant or Assistance Application" => Some(GrantOrAssistanceApplication),
                "Federal Communications Commission (FCC) License Application" => {
                    Some(Code195)
                }
                "Contractor Cost Data Reporting" => Some(ContractorCostDataReporting),
                "Real Estate Title Evidence" => Some(RealEstateTitleEvidence),
                "Loan Verification Information" => Some(LoanVerification),
                "Real Estate Settlement Information" => Some(RealEstateSettlement),
                "Mortgage Credit Report" => Some(MortgageCreditReport),
                "Residential Loan Application" => Some(ResidentialLoanApplication),
                "Secondary Mortgage Market Loan Delivery" => {
                    Some(SecondaryMortgageMarketLoanDelivery)
                }
                "Secondary Mortgage Market Investor Report" => {
                    Some(SecondaryMortgageMarketInvestorReport)
                }
                "Motor Carrier Load Tender" => Some(MotorCarrierLoadTender),
                "Mortgage Note" => Some(MortgageNote),
                "Real Estate Inspection" => Some(RealEstateInspection),
                "Motor Carrier Freight Details and Invoice" => {
                    Some(MotorCarrierFreightDetailsAndInvoice)
                }
                "Motor Carrier Bill of Lading" => Some(MotorCarrierBillOfLading),
                "Motor Carrier Delivery Trailer Manifest" => {
                    Some(MotorCarrierDeliveryTrailerManifest)
                }
                "Motor Carrier Shipment Status Inquiry" => {
                    Some(MotorCarrierShipmentStatusInquiry)
                }
                "Transportation Carrier Shipment Status Message" => {
                    Some(TransportationCarrierShipmentStatusMessage)
                }
                "Motor Carrier Pickup Manifest" => Some(MotorCarrierPickupManifest),
                "Motor Carrier Shipment Pickup Notification" => {
                    Some(MotorCarrierShipmentPickupNotification)
                }
                "Motor Carrier Loading and Route Guide" => {
                    Some(MotorCarrierLoadingAndRouteGuide)
                }
                "Logistics Service Request" => Some(LogisticsServiceRequest),
                "Logistics Service Response" => Some(LogisticsServiceResponse),
                "Cartage Work Assignment" => Some(CartageWorkAssignment),
                "Consolidators Freight Bill and Invoice" => {
                    Some(ConsolidatorsFreightBillAndInvoice)
                }
                "Motor Carrier Summary Freight Bill Manifest" => {
                    Some(MotorCarrierSummaryFreightBillManifest)
                }
                "Response to a Cartage Work Assignment" => {
                    Some(ResponseToACartageWorkAssignment)
                }
                "Trailer Usage Report" => Some(TrailerUsageReport),
                "Equipment Inspection Report" => Some(EquipmentInspectionReport),
                "Motor Carrier Package Status" => Some(MotorCarrierPackageStatus),
                "Data Status Tracking" => Some(DataStatusTracking),
                "Product Source Information" => Some(ProductSource),
                "Real Estate Tax Service Response" => Some(RealEstateTaxServiceResponse),
                "Account Assignment/Inquiry and Service/Status" => {
                    Some(AccountAssignmentInquiryAndServiceStatus)
                }
                "Animal Toxicological Data" => Some(AnimalToxicologicalData),
                "Purchase Order Shipment Management Document" => {
                    Some(PurchaseOrderShipmentManagementDocument)
                }
                "Pricing Support" => Some(PricingSupport),
                "Insurance Producer Administration" => {
                    Some(InsuranceProducerAdministration)
                }
                "Underwriting Information Services" => {
                    Some(UnderwritingInformationServices)
                }
                "Periodic Compensation" => Some(PeriodicCompensation),
                "Residential Mortgage Insurance Explanation of Benefits" => {
                    Some(ResidentialMortgageInsuranceExplanationOfBenefits)
                }
                "Application for Mortgage Insurance Benefits" => {
                    Some(ApplicationForMortgageInsuranceBenefits)
                }
                "Real Estate Information Request" => Some(RealEstateInformationRequest),
                "Real Estate Information Report" => Some(RealEstateInformationReport),
                "Residential Mortgage Insurance Application Response" => {
                    Some(ResidentialMortgageInsuranceApplicationResponse)
                }
                "Mortgage Loan Default Status" => Some(MortgageLoanDefaultStatus),
                "Real Estate Title Insurance Services Order" => {
                    Some(RealEstateTitleInsuranceServicesOrder)
                }
                "Mortgage or Property Record Change Notification" => {
                    Some(MortgageOrPropertyRecordChangeNotification)
                }
                "Individual Life, Annuity and Disability Application" => Some(Code267),
                "Annuity Activity" => Some(AnnuityActivity),
                "Health Care Benefit Coordination Verification" => {
                    Some(HealthCareBenefitCoordinationVerification)
                }
                "Eligibility, Coverage or Benefit Inquiry" => Some(Code270),
                "Eligibility, Coverage or Benefit Information" => Some(Code271),
                "Property and Casualty Loss Notification" => {
                    Some(PropertyAndCasualtyLossNotification)
                }
                "Insurance/Annuity Application Status" => {
                    Some(InsuranceAnnuityApplicationStatus)
                }
                "Healthcare Provider Information" => Some(HealthcareProvider),
                "Patient Information" => Some(Patient),
                "Health Care Claim Status Request" => Some(HealthCareClaimStatusRequest),
                "Health Care Information Status Notification" => {
                    Some(HealthCareInformationStatusNotification)
                }
                "Health Care Services Review Information" => {
                    Some(HealthCareServicesReview)
                }
                "Voter Registration Information" => Some(VoterRegistration),
                "Tax or Fee Exemption Certification" => {
                    Some(TaxOrFeeExemptionCertification)
                }
                "Commercial Vehicle Safety Reports" => {
                    Some(CommercialVehicleSafetyReports)
                }
                "Commercial Vehicle Safety and Credentials Information Exchange" => {
                    Some(CommercialVehicleSafetyAndCredentialsInformationExchange)
                }
                "Commercial Vehicle Credentials" => Some(CommercialVehicleCredentials),
                "Wage Determination" => Some(WageDetermination),
                "Cooperative Advertising Agreements" => {
                    Some(CooperativeAdvertisingAgreements)
                }
                "Reservation (Booking Request) (Ocean)" => Some(Code300),
                "Confirmation (Ocean)" => Some(Code301),
                "Booking Cancellation (Ocean)" => Some(Code303),
                "Shipping Instructions" => Some(ShippingInstructions),
                "Customs Manifest" => Some(CustomsManifest),
                "Freight Receipt and Invoice (Ocean)" => Some(Code310),
                "Canada Customs Information" => Some(CanadaCustoms),
                "Arrival Notice (Ocean)" => Some(Code312),
                "Shipment Status Inquiry (Ocean)" => Some(Code313),
                "Status Details (Ocean)" => Some(Code315),
                "Delivery/Pickup Order" => Some(DeliveryPickupOrder),
                "Terminal Information" => Some(Terminal),
                "Terminal Operations and Intermodal Ramp Activity" => {
                    Some(TerminalOperationsAndIntermodalRampActivity)
                }
                "Vessel Schedule and Itinerary (Ocean)" => Some(Code323),
                "Vessel Stow Plan (Ocean)" => Some(Code324),
                "Consolidation of Goods In Container" => {
                    Some(ConsolidationOfGoodsInContainer)
                }
                "Consignment Summary List" => Some(ConsignmentSummaryList),
                "Customs Status Information" => Some(CustomsStatus),
                "U.S. Customs Carrier General Order Status" => {
                    Some(USCustomsCarrierGeneralOrderStatus)
                }
                "Customs Events Advisory Details" => Some(CustomsEventsAdvisoryDetails),
                "U.S. Customs Automated Manifest Archive Status" => {
                    Some(USCustomsAutomatedManifestArchiveStatus)
                }
                "U.S. Customs Acceptance/Rejection" => Some(USCustomsAcceptanceRejection),
                "U.S. Customs Permit to Transfer Request" => {
                    Some(USCustomsPermitToTransferRequest)
                }
                "U.S. Customs In-Bond Information" => Some(USCustomsInBond),
                "Customs Consist Information" => Some(CustomsConsist),
                "Carrier Interchange Agreement (Ocean)" => Some(Code361),
                "Cargo Insurance Advice of Shipment" => {
                    Some(CargoInsuranceAdviceOfShipment)
                }
                "Rail Carrier Shipment Information" => Some(RailCarrierShipment),
                "Rail Carrier Freight Details and Invoice" => {
                    Some(RailCarrierFreightDetailsAndInvoice)
                }
                "Trailer or Container Repair Billing" => {
                    Some(TrailerOrContainerRepairBilling)
                }
                "Rail Carhire Settlements" => Some(RailCarhireSettlements),
                "Rail Carrier Waybill Interchange" => Some(RailCarrierWaybillInterchange),
                "Rail Advance Interchange Consist" => Some(RailAdvanceInterchangeConsist),
                "Advance Car Disposition" => Some(AdvanceCarDisposition),
                "Car Handling Information" => Some(CarHandling),
                "Estimated Time of Arrival and Car Scheduling" => {
                    Some(EstimatedTimeOfArrivalAndCarScheduling)
                }
                "Equipment Order" => Some(EquipmentOrder),
                "Rail Industrial Switch List" => Some(RailIndustrialSwitchList),
                "Rail Carrier Services Settlement" => Some(RailCarrierServicesSettlement),
                "Rail Waybill Request" => Some(RailWaybillRequest),
                "Rail Revenue Waybill" => Some(RailRevenueWaybill),
                "Railroad Retirement Activity" => Some(RailroadRetirementActivity),
                "Railroad Station Master File" => Some(RailroadStationMasterFile),
                "Rail Deprescription" => Some(RailDeprescription),
                "Railroad Reciprocal Switch File" => Some(RailroadReciprocalSwitchFile),
                "Railroad Mark Register Update Activity" => {
                    Some(RailroadMarkRegisterUpdateActivity)
                }
                "Standard Transportation Commodity Code Master" => {
                    Some(StandardTransportationCommodityCodeMaster)
                }
                "Locomotive Information" => Some(Locomotive),
                "Railroad Junctions and Interchanges Activity" => {
                    Some(RailroadJunctionsAndInterchangesActivity)
                }
                "Shipment Weights" => Some(ShipmentWeights),
                "Railroad Event Report" => Some(RailroadEventReport),
                "Railroad Problem Log Inquiry or Advice" => {
                    Some(RailroadProblemLogInquiryOrAdvice)
                }
                "Railroad Service Commitment Advice" => {
                    Some(RailroadServiceCommitmentAdvice)
                }
                "Railroad Parameter Trace Registration" => {
                    Some(RailroadParameterTraceRegistration)
                }
                "Railroad Equipment Inquiry or Advice" => {
                    Some(RailroadEquipmentInquiryOrAdvice)
                }
                "Railroad Price Distribution Request or Response" => {
                    Some(RailroadPriceDistributionRequestOrResponse)
                }
                "Rail Rate Reply" => Some(RailRateReply),
                "Rate Request" => Some(RateRequest),
                "Rate Docket Journal Log" => Some(RateDocketJournalLog),
                "Railroad Clearance" => Some(RailroadClearance),
                "Rail Route File Maintenance" => Some(RailRouteFileMaintenance),
                "Ratemaking Action" => Some(RatemakingAction),
                "Rate Docket Expiration" => Some(RateDocketExpiration),
                "Rate Group Definition" => Some(RateGroupDefinition),
                "Miscellaneous Rates" => Some(MiscellaneousRates),
                "Rail Scale Rates" => Some(RailScaleRates),
                "Medical Event Reporting" => Some(MedicalEventReporting),
                "Vendor Performance Review" => Some(VendorPerformanceReview),
                "Pricing History" => Some(PricingHistory),
                "Clauses and Provisions" => Some(ClausesAndProvisions),
                "Requisition" => Some(Requisition),
                "Material Obligation Validation" => Some(MaterialObligationValidation),
                "Income or Asset Offset" => Some(IncomeOrAssetOffset),
                "Material Due-In and Receipt" => Some(MaterialDueInAndReceipt),
                "Logistics Reassignment" => Some(LogisticsReassignment),
                "Notice of Employment Status" => Some(NoticeOfEmploymentStatus),
                "Contract Abstract" => Some(ContractAbstract),
                "Contract Completion Status" => Some(ContractCompletionStatus),
                "Contract Payment Management Report" => {
                    Some(ContractPaymentManagementReport)
                }
                "U.S. Customs Export Shipment Information" => {
                    Some(USCustomsExportShipment)
                }
                "Transportation Services Tender" => Some(TransportationServicesTender),
                "Excavation Communication" => Some(ExcavationCommunication),
                "Well Information" => Some(Well),
                "Maintenance Service Order" => Some(MaintenanceServiceOrder),
                "Intermodal Group Loading Plan" => Some(IntermodalGroupLoadingPlan),
                "Request for Routing Instructions" => Some(RequestForRoutingInstructions),
                "Routing Instructions" => Some(RoutingInstructions),
                "Contract Pricing Proposal" => Some(ContractPricingProposal),
                "Project Schedule Reporting" => Some(ProjectScheduleReporting),
                "Invoice" => Some(Invoice),
                "Consolidated Service Invoice/Statement" => {
                    Some(ConsolidatedServiceInvoiceStatement)
                }
                "Credit/Debit Adjustment" => Some(CreditDebitAdjustment),
                "Electronic Filing of Tax Return Data" => {
                    Some(ElectronicFilingOfTaxReturnData)
                }
                "General Request, Response or Confirmation" => Some(Code814),
                "Cryptographic Service Message" => Some(CryptographicServiceMessage),
                "Organizational Relationships" => Some(OrganizationalRelationships),
                "Commission Sales Report" => Some(CommissionSalesReport),
                "Joint Interest Billing and Operating Expense Statement" => {
                    Some(JointInterestBillingAndOperatingExpenseStatement)
                }
                "Payment Order/Remittance Advice" => Some(PaymentOrderRemittanceAdvice),
                "Financial Information Reporting" => Some(FinancialInformationReporting),
                "Account Analysis" => Some(AccountAnalysis),
                "Lockbox" => Some(Lockbox),
                "Application Advice" => Some(ApplicationAdvice),
                "Tax Information Exchange" => Some(TaxInformationExchange),
                "Financial Return Notice" => Some(FinancialReturnNotice),
                "Debit Authorization" => Some(DebitAuthorization),
                "Payment Cancellation Request" => Some(PaymentCancellationRequest),
                "Planning Schedule with Release Capability" => {
                    Some(PlanningScheduleWithReleaseCapability)
                }
                "Application Control Totals" => Some(ApplicationControlTotals),
                "Price/Sales Catalog" => Some(PriceSalesCatalog),
                "Mortgage Credit Report Order" => Some(MortgageCreditReportOrder),
                "Benefit Enrollment and Maintenance" => {
                    Some(BenefitEnrollmentAndMaintenance)
                }
                "Health Care Claim Payment/Advice" => Some(HealthCareClaimPaymentAdvice),
                "Procurement Notices" => Some(ProcurementNotices),
                "Health Care Claim" => Some(HealthCareClaim),
                "Trading Partner Profile" => Some(TradingPartnerProfile),
                "Project Cost Reporting" => Some(ProjectCostReporting),
                "Request for Quotation" => Some(RequestForQuotation),
                "Specifications/Technical Information" => Some(SpecificationsTechnical),
                "Nonconformance Report" => Some(NonconformanceReport),
                "Response to Request for Quotation" => {
                    Some(ResponseToRequestForQuotation)
                }
                "Product Transfer Account Adjustment" => {
                    Some(ProductTransferAccountAdjustment)
                }
                "Price Authorization Acknowledgment/Status" => {
                    Some(PriceAuthorizationAcknowledgmentStatus)
                }
                "Inventory Inquiry/Advice" => Some(InventoryInquiryAdvice),
                "Material Claim" => Some(MaterialClaim),
                "Material Safety Data Sheet" => Some(MaterialSafetyDataSheet),
                "Response to Product Transfer Account Adjustment" => {
                    Some(ResponseToProductTransferAccountAdjustment)
                }
                "Purchase Order" => Some(PurchaseOrder),
                "Asset Schedule" => Some(AssetSchedule),
                "Product Activity Data" => Some(ProductActivityData),
                "Routing and Carrier Instruction" => Some(RoutingAndCarrierInstruction),
                "Shipment Delivery Discrepancy Information" => {
                    Some(ShipmentDeliveryDiscrepancy)
                }
                "Purchase Order Acknowledgment" => Some(PurchaseOrderAcknowledgment),
                "Ship Notice/Manifest" => Some(ShipNoticeManifest),
                "Shipment and Billing Notice" => Some(ShipmentAndBillingNotice),
                "Shipment Information" => Some(Shipment),
                "Freight Invoice" => Some(FreightInvoice),
                "Purchase Order Change Request - Buyer Initiated" => {
                    Some(PurchaseOrderChangeRequestBuyerInitiated)
                }
                "Receiving Advice/Acceptance Certificate" => {
                    Some(ReceivingAdviceAcceptanceCertificate)
                }
                "Shipping Schedule" => Some(ShippingSchedule),
                "Report of Test Results" => Some(ReportOfTestResults),
                "Text Message" => Some(TextMessage),
                "Purchase Order Change Acknowledgment/Request - Seller Initiated" => {
                    Some(PurchaseOrderChangeAcknowledgmentRequestSellerInitiated)
                }
                "Production Sequence" => Some(ProductionSequence),
                "Product Transfer and Resale Report" => {
                    Some(ProductTransferAndResaleReport)
                }
                "Electronic Form Structure" => Some(ElectronicFormStructure),
                "Order Status Inquiry" => Some(OrderStatusInquiry),
                "Order Status Report" => Some(OrderStatusReport),
                "Component Parts Content" => Some(ComponentPartsContent),
                "Residential Mortgage Insurance Application" => {
                    Some(ResidentialMortgageInsuranceApplication)
                }
                "Commodity Movement Services" => Some(CommodityMovementServices),
                "Commodity Movement Services Response" => {
                    Some(CommodityMovementServicesResponse)
                }
                "Grocery Products Purchase Order" => Some(GroceryProductsPurchaseOrder),
                "Grocery Products Purchase Order Change" => {
                    Some(GroceryProductsPurchaseOrderChange)
                }
                "Manufacturer Coupon Family Code Structure" => {
                    Some(ManufacturerCouponFamilyCodeStructure)
                }
                "Product Authorization/De-authorization" => {
                    Some(ProductAuthorizationDeAuthorization)
                }
                "Price Information" => Some(Price),
                "Grocery Products Invoice" => Some(GroceryProductsInvoice),
                "Manufacturer Coupon Redemption Detail" => {
                    Some(ManufacturerCouponRedemptionDetail)
                }
                "Direct Store Delivery Summary Information" => {
                    Some(DirectStoreDeliverySummary)
                }
                "Market Development Fund Allocation" => {
                    Some(MarketDevelopmentFundAllocation)
                }
                "Market Development Fund Settlement" => {
                    Some(MarketDevelopmentFundSettlement)
                }
                "Retail Account Characteristics" => Some(RetailAccountCharacteristics),
                "Customer Call Reporting" => Some(CustomerCallReporting),
                "Coupon Notification" => Some(CouponNotification),
                "Item Maintenance" => Some(ItemMaintenance),
                "Promotion Announcement" => Some(PromotionAnnouncement),
                "Deduction Research Report" => Some(DeductionResearchReport),
                "Item Information Request" => Some(ItemInformationRequest),
                "Delivery/Return Base Record" => Some(DeliveryReturnBaseRecord),
                "Delivery/Return Acknowledgment or Adjustment" => {
                    Some(DeliveryReturnAcknowledgmentOrAdjustment)
                }
                "Product Dimension Maintenance" => Some(ProductDimensionMaintenance),
                "Loss or Damage Claim - General Commodities" => {
                    Some(LossOrDamageClaimGeneralCommodities)
                }
                "Loss or Damage Claim - Motor Vehicle" => {
                    Some(LossOrDamageClaimMotorVehicle)
                }
                "Claim Tracer" => Some(ClaimTracer),
                "Claim Status Report and Tracer Reply" => {
                    Some(ClaimStatusReportAndTracerReply)
                }
                "Automotive Inspection Detail" => Some(AutomotiveInspectionDetail),
                "Warehouse Shipping Order" => Some(WarehouseShippingOrder),
                "Warehouse Stock Transfer Shipment Advice" => {
                    Some(WarehouseStockTransferShipmentAdvice)
                }
                "Warehouse Stock Transfer Receipt Advice" => {
                    Some(WarehouseStockTransferReceiptAdvice)
                }
                "Warehouse Shipping Advice" => Some(WarehouseShippingAdvice),
                "Warehouse Inventory Adjustment Advice" => {
                    Some(WarehouseInventoryAdjustmentAdvice)
                }
                "Functional Group Totals" => Some(FunctionalGroupTotals),
                "Response to a Load Tender" => Some(ResponseToALoadTender),
                "Secured Receipt or Acknowledgment" => {
                    Some(SecuredReceiptOrAcknowledgment)
                }
                "File Transfer" => Some(FileTransfer),
                "Functional Acknowledgment" => Some(FunctionalAcknowledgment),
                "Set Cancellation" => Some(SetCancellation),
                "Implementation Acknowledgment" => Some(ImplementationAcknowledgment),
                _ => None,
            }
        }
    }
}
impl Serialize for TransactionSetIdentifierCode {
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
    type Value = TransactionSetIdentifierCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Transaction Set Identifier Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        TransactionSetIdentifierCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Transaction Set Identifier Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        TransactionSetIdentifierCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Transaction Set Identifier Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for TransactionSetIdentifierCode {
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