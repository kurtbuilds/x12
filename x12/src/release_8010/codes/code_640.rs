use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**640

See docs at <https://www.stedi.com/edi/x12/element/640>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransactionTypeCode {
    ///01 - Location Address Message
    LocationAddressMessage,
    ///1A - Unique Item Tracking Control Report
    UniqueItemTrackingControlReport,
    ///1B - Unique Item Tracking Report Reconciliation
    UniqueItemTrackingReportReconciliation,
    ///1C - Unique Item Tracking Item Data Change
    UniqueItemTrackingItemDataChange,
    ///1E - New Group Initial Enrollment
    NewGroupInitialEnrollment,
    ///02 - Location Relation Information
    LocationRelation,
    ///03 - Report Message
    ReportMessage,
    ///3M - Supporting Information
    Supporting,
    ///04 - Electronic Mail Message
    ElectronicMailMessage,
    ///05 - Request for Co-op
    RequestForCoOp,
    ///06 - Guidelines
    Guidelines,
    ///6A - Accomplishment Based Renewal
    AccomplishmentBasedRenewal,
    ///6C - Competitive Renewal
    CompetitiveRenewal,
    ///6N - Non-competitive Renewal
    NonCompetitiveRenewal,
    ///6R - Resubmission
    Resubmission,
    ///6S - Supplemental
    Supplemental,
    ///07 - Budget
    Budget,
    ///08 - Commitment
    Commitment,
    ///09 - Co-op Actual
    CoOpActual,
    ///10 - Distribution
    Distribution,
    ///11 - National Property Registry System Real Estate Property Transaction
    NationalPropertyRegistrySystemRealEstatePropertyTransaction,
    ///12 - Physician's Report
    PhysiciansReport,
    ///13 - Maintenance Request
    MaintenanceRequest,
    ///14 - Maintenance Response
    MaintenanceResponse,
    ///15 - Request with Immediate Response Required (No Follow-up)
    Code15,
    ///16 - Request with Immediate Response Required (Follow-up Required)
    Code16,
    ///17 - Request with Immediate Response to Mailbox
    RequestWithImmediateResponseToMailbox,
    ///18 - Response - No Further Updates to Follow
    ResponseNoFurtherUpdatesToFollow,
    ///19 - Response - Further Updates to Follow
    ResponseFurtherUpdatesToFollow,
    ///20 - Air Export Waybill and Invoice
    AirExportWaybillAndInvoice,
    ///21 - Air Import Invoice
    AirImportInvoice,
    ///22 - Ocean Export Direct Invoice
    OceanExportDirectInvoice,
    ///23 - Ocean Export Indirect Invoice
    OceanExportIndirectInvoice,
    ///24 - Ocean Export Brokerage Invoice
    OceanExportBrokerageInvoice,
    ///25 - Ocean Import Invoice
    OceanImportInvoice,
    ///26 - Miscellaneous Services Invoice
    MiscellaneousServicesInvoice,
    ///27 - Warehouse Services Invoice
    WarehouseServicesInvoice,
    ///28 - Pre-registration
    PreRegistration,
    ///30 - Delivery
    Delivery,
    ///31 - Subrogation Demand
    SubrogationDemand,
    ///33 - Normal
    Normal,
    ///34 - Emergency Request
    EmergencyRequest,
    ///35 - Short Notice Request
    ShortNoticeRequest,
    ///36 - Damage Notification
    DamageNotification,
    ///37 - Design Report
    DesignReport,
    ///38 - Test
    Test,
    ///39 - Notice of Occupational Disease
    NoticeOfOccupationalDisease,
    ///40 - Notice of Traumatic Injury
    NoticeOfTraumaticInjury,
    ///41 - Statement of Financial Affairs
    StatementOfFinancialAffairs,
    ///42 - Customer Shipment
    CustomerShipment,
    ///43 - International Shipment
    InternationalShipment,
    ///44 - Interplant Shipment
    InterplantShipment,
    ///45 - Production to Carrier Shipment
    ProductionToCarrierShipment,
    ///46 - Advanced Payment
    AdvancedPayment,
    ///47 - Delinquency
    Delinquency,
    ///48 - Payment Information
    Payment,
    ///49 - Audit
    Audit,
    ///50 - Transfer Loan In
    TransferLoanIn,
    ///51 - Transfer Loan Out
    TransferLoanOut,
    ///52 - Mailing Address Services
    MailingAddressServices,
    ///54 - Information Only, No Response Required
    Code54,
    ///55 - New Service Order
    NewServiceOrder,
    ///56 - Denied Request
    DeniedRequest,
    ///57 - Service Termination
    ServiceTermination,
    ///58 - Tax Billing or Remittance
    TaxBillingOrRemittance,
    ///60 - Material In Transit
    MaterialInTransit,
    ///62 - Preaward Notification
    PreawardNotification,
    ///63 - Postaward Notification
    PostawardNotification,
    ///64 - Small Business Award Notification
    SmallBusinessAwardNotification,
    ///65 - Award Notification
    AwardNotification,
    ///66 - Notification to Late Bidders
    NotificationToLateBidders,
    ///67 - Notification of Protest Receipt
    NotificationOfProtestReceipt,
    ///68 - Abstract of Offers
    AbstractOfOffers,
    ///69 - Bid Rejection Notice
    BidRejectionNotice,
    ///70 - Unsolicited Bid Notice
    UnsolicitedBidNotice,
    ///71 - Filing and Search Request
    FilingAndSearchRequest,
    ///72 - Termination
    Termination,
    ///73 - Filing Officer Confirmation
    FilingOfficerConfirmation,
    ///74 - Amendment
    Amendment,
    ///75 - Full Assignment
    FullAssignment,
    ///76 - Partial Assignment
    PartialAssignment,
    ///77 - Full Release of Collateral
    FullReleaseOfCollateral,
    ///78 - Partial Release of Collateral
    PartialReleaseOfCollateral,
    ///79 - Continuation
    Continuation,
    ///80 - Uniform Commercial Code Filing with Broad Collateral Description
    UniformCommercialCodeFilingWithBroadCollateralDescription,
    ///81 - Uniform Commercial Code Filing with Specific Collateral Description
    UniformCommercialCodeFilingWithSpecificCollateralDescription,
    ///82 - Segregation by Purchase Order Number
    SegregationByPurchaseOrderNumber,
    ///83 - Segregation by Carrier PRO Number
    SegregationByCarrierProNumber,
    ///85 - Response - No Action Taken
    ResponseNoActionTaken,
    ///87 - Disability Benefits Proof of Coverage Notification
    DisabilityBenefitsProofOfCoverageNotification,
    ///88 - Debtor Filing
    DebtorFiling,
    ///89 - Federal Occupational Reporting
    FederalOccupationalReporting,
    ///91 - Secured Party Filing
    SecuredPartyFiling,
    ///94 - Partial Release Filing
    PartialReleaseFiling,
    ///95 - Full (Total) Release
    Code95,
    ///97 - Multiple Listing
    MultipleListing,
    ///98 - Sale
    Sale,
    ///99 - Loan
    Loan,
    ///A0 - Requisition
    Requisition,
    ///A1 - Storage Item Data Change
    StorageItemDataChange,
    ///A3 - Administrative Fees
    AdministrativeFees,
    ///A4 - Amended Return
    AmendedReturn,
    ///A5 - Notice of Charge
    NoticeOfCharge,
    ///A6 - Protest or Response to Charge
    ProtestOrResponseToCharge,
    ///A7 - Charge Decision
    ChargeDecision,
    ///AA - Due to Analysis
    DueToAnalysis,
    ///AB - Advance Shipment and Billing Notice
    AdvanceShipmentAndBillingNotice,
    ///AC - Requisition Cancellation
    RequisitionCancellation,
    ///AD - Notice of Availability
    NoticeOfAvailability,
    ///AE - Due to Reconciliation (Full)
    CodeAE,
    ///AF - Requisition Follow-Up
    RequisitionFollowUp,
    ///AG - Due to Reconciliation (Partial)
    CodeAG,
    ///AH - Pre-Employment Screening
    PreEmploymentScreening,
    ///AI - Adjusted Invoice
    AdjustedInvoice,
    ///AJ - Student Loan Interest Statement (1098-E)
    CodeAJ,
    ///AK - Tuition Payments Statement (1098-T)
    CodeAK,
    ///AL - Arbitration
    Arbitration,
    ///AM - Requisition Modification
    RequisitionModification,
    ///AN - Material Obligation Inquiry
    MaterialObligationInquiry,
    ///AP - Material Obligation Advice
    MaterialObligationAdvice,
    ///AQ - Quantity Verification Inquiry
    QuantityVerificationInquiry,
    ///AR - Supply Assistance
    SupplyAssistance,
    ///AS - Shipment Advice
    ShipmentAdvice,
    ///AT - Administrative Action
    AdministrativeAction,
    ///AV - Quantity Verification Advice
    QuantityVerificationAdvice,
    ///AW - Material Obligation Reinstatement
    MaterialObligationReinstatement,
    ///AZ - Disposal Shipment Confirmation
    DisposalShipmentConfirmation,
    ///BA - Backbill Invoice
    BackbillInvoice,
    ///BB - Billback
    Billback,
    ///BD - Statement of Balance Due
    StatementOfBalanceDue,
    ///BF - Best and Final Offer
    BestAndFinalOffer,
    ///BG - Non-escrow or Non-impound Tracking
    NonEscrowOrNonImpoundTracking,
    ///BH - Bill and Hold Goods
    BillAndHoldGoods,
    ///BJ - Bankruptcy Petition
    BankruptcyPetition,
    ///BK - Bankruptcy Schedules
    BankruptcySchedules,
    ///BL - Blanket Lease Schedule
    BlanketLeaseSchedule,
    ///BM - Requisition Passing Order
    RequisitionPassingOrder,
    ///BN - Requisition Referral Order
    RequisitionReferralOrder,
    ///BO - Procurement Cancellation
    ProcurementCancellation,
    ///BP - Procurement Modification
    ProcurementModification,
    ///BR - Business Taxpayer Registration
    BusinessTaxpayerRegistration,
    ///BS - Bill of Sale Schedule
    BillOfSaleSchedule,
    ///BT - Balance and Transaction Report
    BalanceAndTransactionReport,
    ///BU - Workers' Compensation 1st Report of Injury
    WorkersCompensation1StReportOfInjury,
    ///BV - Workers' Compensation Subsequent Report
    WorkersCompensationSubsequentReport,
    ///BW - Workers' Compensation Combined 1st and Subsequent Report
    WorkersCompensationCombined1StAndSubsequentReport,
    ///BX - Not for Resale Invoice
    NotForResaleInvoice,
    ///BZ - Freight Invoice
    FreightInvoice,
    ///C0 - Customs Entry Detail
    CustomsEntryDetail,
    ///C1 - Claim Information
    Claim,
    ///C2 - Case Opening
    CaseOpening,
    ///C3 - Consignment
    Consignment,
    ///C4 - Escrow or Impound Service Reporting
    EscrowOrImpoundServiceReporting,
    ///C5 - Tax Assessment Bill
    TaxAssessmentBill,
    ///C6 - Fixed Assets Tax Return
    FixedAssetsTaxReturn,
    ///C7 - Service Cancellation
    ServiceCancellation,
    ///C8 - Conformed Copy
    ConformedCopy,
    ///C9 - Subject to Availability of Funds
    SubjectToAvailabilityOfFunds,
    ///CA - Cash
    Cash,
    ///CB - Contribution
    Contribution,
    ///CC - Credit Commission Invoice
    CreditCommissionInvoice,
    ///CD - Consolidated Debit Invoice
    ConsolidatedDebitInvoice,
    ///CE - Consolidated Credit Invoice
    ConsolidatedCreditInvoice,
    ///CF - Consolidated Debit Memo
    ConsolidatedDebitMemo,
    ///CG - Consolidated Credit Memo
    ConsolidatedCreditMemo,
    ///CH - Chargeable
    Chargeable,
    ///CI - Consolidated Invoice
    ConsolidatedInvoice,
    ///CJ - Confirmation
    Confirmation,
    ///CK - Claim Submission
    ClaimSubmission,
    ///CL - Customer Allocation
    CustomerAllocation,
    ///CM - Call Detail Memo
    CallDetailMemo,
    ///CN - Credit Invoice
    CreditInvoice,
    ///CO - Corrected
    Corrected,
    ///CP - Commission Payment
    CommissionPayment,
    ///CR - Credit Memo
    CreditMemo,
    ///CS - Cash Surrender Distribution
    CashSurrenderDistribution,
    ///CT - Cost Type Invoice
    CostTypeInvoice,
    ///CU - Cargo Outturn Report
    CargoOutturnReport,
    ///CV - Cost Voucher
    CostVoucher,
    ///CW - Cash Letter
    CashLetter,
    ///CX - Check List
    CheckList,
    ///CY - Citation to Pay or Appear
    CitationToPayOrAppear,
    ///CZ - Conviction Notice
    ConvictionNotice,
    ///D1 - Dividend Payment
    DividendPayment,
    ///D4 - Receipt
    Receipt,
    ///DA - Due-In
    DueIn,
    ///DB - Detour Billing
    DetourBilling,
    ///DC - Debit Commission Invoice
    DebitCommissionInvoice,
    ///DD - Interdistrict (Pre-Kindergarten - Grade 12) Student Record
    CodeDD,
    ///DE - Advance Receipt
    AdvanceReceipt,
    ///DF - Diesel Fuel Bill
    DieselFuelBill,
    ///DG - Response
    Response,
    ///DH - Discretionary Additional Company Contribution
    DiscretionaryAdditionalCompanyContribution,
    ///DI - Debit Invoice
    DebitInvoice,
    ///DK - Duty Drawback
    DutyDrawback,
    ///DL - Deposit List
    DepositList,
    ///DM - Prior Damage Report
    PriorDamageReport,
    ///DN - Direct Nonqualified Rollover
    DirectNonqualifiedRollover,
    ///DO - Drop Shipment Invoice
    DropShipmentInvoice,
    ///DP - District to Postsecondary Student Record
    DistrictToPostsecondaryStudentRecord,
    ///DQ - Direct Qualified Rollover
    DirectQualifiedRollover,
    ///DR - Debit Memo
    DebitMemo,
    ///DS - Disposition
    Disposition,
    ///DT - Detail
    Detail,
    ///DU - Duplicate
    Duplicate,
    ///E1 - Administrative Records Submission
    AdministrativeRecordsSubmission,
    ///EA - Excess Interest Allocation
    ExcessInterestAllocation,
    ///EB - Engineering Final Bill
    EngineeringFinalBill,
    ///EC - Campaign Filing
    CampaignFiling,
    ///ED - Lobbyist Filing
    LobbyistFiling,
    ///EF - Engineering Installation, Right to Use, Final Bill
    CodeEF,
    ///EI - Engineering Installation, Final Bill
    CodeEI,
    ///EM - Estimate of Record
    EstimateOfRecord,
    ///EP - Expense Payment
    ExpensePayment,
    ///ER - Engineering Right to Use, Final Bill
    CodeER,
    ///EX - Excess Material Notification
    ExcessMaterialNotification,
    ///F1 - Final Report
    FinalReport,
    ///F2 - Pre-approved Bidders List
    PreApprovedBiddersList,
    ///F4 - Pre-determined Allocation
    PreDeterminedAllocation,
    ///F5 - Allocation
    Allocation,
    ///F6 - Shipper Imbalance
    ShipperImbalance,
    ///F7 - Producer Imbalance
    ProducerImbalance,
    ///F8 - Storage Report
    StorageReport,
    ///FA - Forfeiture Allocation
    ForfeitureAllocation,
    ///FB - Final Bill
    FinalBill,
    ///FC - Forfeiture Credit
    ForfeitureCredit,
    ///FD - Consolidated Invoice, Final Bill
    CodeFD,
    ///FE - Memorandum, Final Bill
    CodeFE,
    ///FF - Full Assignment Filing
    FullAssignmentFiling,
    ///FG - Filing
    Filing,
    ///FI - First Cost Invoice
    FirstCostInvoice,
    ///FL - Final Notice
    FinalNotice,
    ///FM - Funding Modification
    FundingModification,
    ///FN - First Notice of Loss
    FirstNoticeOfLoss,
    ///FP - Flat Rate Per Unit Bill
    FlatRatePerUnitBill,
    ///FQ - Full Enrollment File
    FullEnrollmentFile,
    ///FR - Federal Royalty
    FederalRoyalty,
    ///FS - Financial Statement Report
    FinancialStatementReport,
    ///FT - Material Returns
    MaterialReturns,
    ///G1 - Nomination
    Nomination,
    ///G2 - Request for Confirmation
    RequestForConfirmation,
    ///G3 - Confirmation Response
    ConfirmationResponse,
    ///GA - Government Furnished Material Inquiry Advice
    GovernmentFurnishedMaterialInquiryAdvice,
    ///GI - Government Furnished Material Inquiry
    GovernmentFurnishedMaterialInquiry,
    ///GR - Garnishment
    Garnishment,
    ///GS - Credit Report
    CreditReport,
    ///GT - Disability Notice
    DisabilityNotice,
    ///GU - Black Lung Claim
    BlackLungClaim,
    ///GV - Claim Experience Report
    ClaimExperienceReport,
    ///GW - Employer's Report
    EmployersReport,
    ///GX - Longshore Report
    LongshoreReport,
    ///GY - Unit Report
    UnitReport,
    ///HA - Hazardous Waste Report
    HazardousWasteReport,
    ///HB - Discharge Monitoring Report
    DischargeMonitoringReport,
    ///HC - Risk Management Plan
    RiskManagementPlan,
    ///HD - Self Monitoring Report
    SelfMonitoringReport,
    ///HE - Hazardous Air Pollutant Inventory Report
    HazardousAirPollutantInventoryReport,
    ///HF - Stationary Point Source Inventory Report
    StationaryPointSourceInventoryReport,
    ///HG - Toxic Release Inventory Report
    ToxicReleaseInventoryReport,
    ///HP - Horsepower Equalization Bill
    HorsepowerEqualizationBill,
    ///HX - Handling Carrier Agreement Update
    HandlingCarrierAgreementUpdate,
    ///I1 - In-Ad Coupon Notification
    InAdCouponNotification,
    ///IA - Inventory
    Inventory,
    ///IB - Installation Final Bill
    InstallationFinalBill,
    ///IC - Insurance Coverage Notification
    InsuranceCoverageNotification,
    ///ID - Employers Report of Disability
    EmployersReportOfDisability,
    ///IE - Indian Royalty
    IndianRoyalty,
    ///IF - Material, Engineering, Installation, Final Bill
    CodeIF,
    ///II - Interfund Transfer In
    InterfundTransferIn,
    ///IM - Incident Notice
    IncidentNotice,
    ///IN - Inquiry
    Inquiry,
    ///IO - Interfund Transfer Out
    InterfundTransferOut,
    ///IR - Installation, Right to Use, Final Bill
    CodeIR,
    ///IU - Material, Installation, Right to Use, Final Bill
    CodeIU,
    ///IW - Workers Compensation Report of Injury or Illness
    WorkersCompensationReportOfInjuryOrIllness,
    ///IX - Interchange Update
    InterchangeUpdate,
    ///IZ - Investment Fees
    InvestmentFees,
    ///JM - Maintenance and Operations Bill
    MaintenanceAndOperationsBill,
    ///JO - Joint Facility Miscellaneous Bill or Other
    JointFacilityMiscellaneousBillOrOther,
    ///JR - Rental Bill
    RentalBill,
    ///JS - Junction Settlement Update
    JunctionSettlementUpdate,
    ///JU - Judgment
    Judgment,
    ///JX - Junction Update
    JunctionUpdate,
    ///KB - Termination for Default
    TerminationForDefault,
    ///KC - Definitization of Contract
    DefinitizationOfContract,
    ///KD - Definitization of Order
    DefinitizationOfOrder,
    ///KE - Exercise of Option
    ExerciseOfOption,
    ///KF - Intent to Exercise Option
    IntentToExerciseOption,
    ///KG - Administrative Change
    AdministrativeChange,
    ///KH - Change Order
    ChangeOrder,
    ///KI - Supplemental Agreement
    SupplementalAgreement,
    ///KJ - Amended Shipping Instructions
    AmendedShippingInstructions,
    ///KK - Provisioned Item
    ProvisionedItem,
    ///KL - Withdrawal of Offer
    WithdrawalOfOffer,
    ///KM - Additions to General Provisions
    AdditionsToGeneralProvisions,
    ///KN - Request for Price Quote
    RequestForPriceQuote,
    ///KS - Addition to Solicitation Mailing List
    AdditionToSolicitationMailingList,
    ///KT - Termination for Convenience
    TerminationForConvenience,
    ///LC - Due-in Reconciliation Inquiry
    DueInReconciliationInquiry,
    ///LD - Loan Distribution
    LoanDistribution,
    ///LE - Loan Repayment Expense
    LoanRepaymentExpense,
    ///LF - Landed Costs
    LandedCosts,
    ///LN - Loss Notification
    LossNotification,
    ///LO - Loan Repayment to Principal Only
    LoanRepaymentToPrincipalOnly,
    ///LP - Loan Repayment
    LoanRepayment,
    ///LR - Logistics Reassignment
    LogisticsReassignment,
    ///LV - Levy
    Levy,
    ///M1 - Manufacturer Coupon Notification
    ManufacturerCouponNotification,
    ///M5 - Measurement Events and Alarms
    MeasurementEventsAndAlarms,
    ///MA - Mailing List
    MailingList,
    ///MB - Maintenance to Business Taxpayer Registration
    MaintenanceToBusinessTaxpayerRegistration,
    ///MC - Material Credit Invoice
    MaterialCreditInvoice,
    ///MD - Market Development Fund
    MarketDevelopmentFund,
    ///ME - Memorandum
    Memorandum,
    ///MF - Material, Engineering, Final Bill
    CodeMF,
    ///MI - Material, Installation, Final Bill
    CodeMI,
    ///ML - Membership List
    MembershipList,
    ///MM - Multiple Shippers, Multiple Consignees
    CodeMM,
    ///MP - Mise En Place (In Place)
    CodeMP,
    ///MR - Material, Right to Use, Final Bill
    CodeMR,
    ///MS - Material Final Bill
    MaterialFinalBill,
    ///MU - Multifamily Program
    MultifamilyProgram,
    ///N1 - Bilateral
    Bilateral,
    ///N2 - Notice of Assessment
    NoticeOfAssessment,
    ///N3 - Notice of Warrant
    NoticeOfWarrant,
    ///N4 - Notice of Adjustment
    NoticeOfAdjustment,
    ///N5 - Notice of Determination
    NoticeOfDetermination,
    ///N6 - Notice of Settlement
    NoticeOfSettlement,
    ///N7 - Notice of Recorded Lien
    NoticeOfRecordedLien,
    ///N8 - Notice of Deficiency
    NoticeOfDeficiency,
    ///NA - Material Release Order
    MaterialReleaseOrder,
    ///NB - Material Release Inquiry
    MaterialReleaseInquiry,
    ///NC - Material Release Order Forced Closure
    MaterialReleaseOrderForcedClosure,
    ///ND - Material Release Cancellation
    MaterialReleaseCancellation,
    ///NE - Disposal Release Order
    DisposalReleaseOrder,
    ///NF - Disposal Release Inquiry
    DisposalReleaseInquiry,
    ///NG - Disposal Release Cancellation
    DisposalReleaseCancellation,
    ///NH - Disposal Shipment Confirmation Inquiry
    DisposalShipmentConfirmationInquiry,
    ///NI - Redistribution Order
    RedistributionOrder,
    ///NJ - Material Release Confirmation
    MaterialReleaseConfirmation,
    ///NK - Material Release Denial
    MaterialReleaseDenial,
    ///NL - Material Release Advice
    MaterialReleaseAdvice,
    ///NM - Disposal Release Confirmation
    DisposalReleaseConfirmation,
    ///NO - Notice
    Notice,
    ///NP - Disposal Release Denial
    DisposalReleaseDenial,
    ///NQ - Disposal Release Advice
    DisposalReleaseAdvice,
    ///NR - Material Release Cancellation Advice
    MaterialReleaseCancellationAdvice,
    ///NS - In-Transit
    InTransit,
    ///NT - Disposal Release Cancellation Advice
    DisposalReleaseCancellationAdvice,
    ///NU - Inventory Adjustment
    InventoryAdjustment,
    ///OA - Operational Capacity
    OperationalCapacity,
    ///OC - On Approval
    OnApproval,
    ///OF - Offer
    Offer,
    ///OP - Opinion
    Opinion,
    ///OR - Order
    Order,
    ///P1 - Preliminary
    Preliminary,
    ///P2 - Employer Group Change
    EmployerGroupChange,
    ///P3 - Individual Change
    IndividualChange,
    ///P4 - Employer Open Enrollment
    EmployerOpenEnrollment,
    ///P5 - Predetermination - Medical
    PredeterminationMedical,
    ///P6 - Predetermination - Dental
    PredeterminationDental,
    ///PA - Progress Payment Invoice
    ProgressPaymentInvoice,
    ///PB - Partial Bill
    PartialBill,
    ///PC - Invention Report
    InventionReport,
    ///PD - Product Allocation
    ProductAllocation,
    ///PE - Pleading
    Pleading,
    ///PF - Partial Assignment Filing
    PartialAssignmentFiling,
    ///PG - Premium Routing Guide
    PremiumRoutingGuide,
    ///PH - Prospective Student Information
    ProspectiveStudent,
    ///PI - Personal Injury Bill
    PersonalInjuryBill,
    ///PJ - Component Packing Confirmation
    ComponentPackingConfirmation,
    ///PL - Plan Allocation
    PlanAllocation,
    ///PM - Premium Payment
    PremiumPayment,
    ///PO - Plan Takeover
    PlanTakeover,
    ///PP - Prepaid Invoice
    PrepaidInvoice,
    ///PQ - Partial Enrollment File
    PartialEnrollmentFile,
    ///PR - Product (or Service)
    CodePR,
    ///PS - Postsecondary Student Academic Record
    PostsecondaryStudentAcademicRecord,
    ///PT - Plan-to-plan Transfer
    PlanToPlanTransfer,
    ///PU - Notice of Claim
    NoticeOfClaim,
    ///PV - Protest or Response to Claim
    ProtestOrResponseToClaim,
    ///PW - Claim Decision
    ClaimDecision,
    ///PX - Wage Verification Notice
    WageVerificationNotice,
    ///PZ - Purchase Report
    PurchaseReport,
    ///Q1 - Scheduled Quantity
    ScheduledQuantity,
    ///Q2 - Scheduled Quantity for Operator
    ScheduledQuantityForOperator,
    ///QA - Coupon Regular Clearinghouse Invoice
    CouponRegularClearinghouseInvoice,
    ///QB - Coupon Direct Retailer Invoice
    CouponDirectRetailerInvoice,
    ///QC - Coupon Clearinghouse Pay Direct Invoice
    CouponClearinghousePayDirectInvoice,
    ///QD - Product Quality Deficiency
    ProductQualityDeficiency,
    ///QE - Coupon Scan Validate Invoice
    CouponScanValidateInvoice,
    ///QF - Scan Validate Adjustment
    ScanValidateAdjustment,
    ///QG - Quick Response Routing Guide
    QuickResponseRoutingGuide,
    ///QH - Full Coupon Redemption (No Adjustments)
    CodeQH,
    ///QJ - Adjusted Coupon Redemption
    AdjustedCouponRedemption,
    ///QK - Coupon Redemption Detail
    CouponRedemptionDetail,
    ///QL - Adjustments to Previous Coupon Redemption
    AdjustmentsToPreviousCouponRedemption,
    ///QP - Coupon Quick Pay Invoice
    CouponQuickPayInvoice,
    ///QR - Product Quality Deficiency Response
    ProductQualityDeficiencyResponse,
    ///R1 - Request for Enrollment Verification
    RequestForEnrollmentVerification,
    ///R2 - Response to Request for Enrollment Verification
    ResponseToRequestForEnrollmentVerification,
    ///R3 - Response to Garnishment
    ResponseToGarnishment,
    ///R4 - Release of Garnishment
    ReleaseOfGarnishment,
    ///R5 - Response to Levy
    ResponseToLevy,
    ///R6 - Release of Levy
    ReleaseOfLevy,
    ///R7 - Response to Order to Withhold
    ResponseToOrderToWithhold,
    ///R8 - Release of Order to Withhold
    ReleaseOfOrderToWithhold,
    ///RA - Request for Credit
    RequestForCredit,
    ///RB - Right to Use
    RightToUse,
    ///RC - Request for Quote
    RequestForQuote,
    ///RD - Returns Detail
    ReturnsDetail,
    ///RE - Rebill
    Rebill,
    ///RF - Material, Engineering, Right to Use, Final Bill
    CodeRF,
    ///RG - Revised Final Bill
    RevisedFinalBill,
    ///RH - Request for Additional Funds
    RequestForAdditionalFunds,
    ///RI - Routing Instructions
    RoutingInstructions,
    ///RJ - Response to Request for Routing Instructions
    ResponseToRequestForRoutingInstructions,
    ///RK - Registration
    Registration,
    ///RM - Reminder to File
    ReminderToFile,
    ///RP - Reporting
    Reporting,
    ///RQ - Request
    Request,
    ///RS - Response - Additional Response(s) Available
    CodeRS,
    ///RT - Spend Down
    SpendDown,
    ///RU - Medical Services Reservation
    MedicalServicesReservation,
    ///RZ - Removed from Solicitation Mailing List
    RemovedFromSolicitationMailingList,
    ///S1 - Special Routing Guide
    SpecialRoutingGuide,
    ///S2 - Standard Routing Guide
    StandardRoutingGuide,
    ///S3 - Supplemental Loan Repayment
    SupplementalLoanRepayment,
    ///S4 - Submission
    Submission,
    ///SA - Stand-alone Lease Schedule
    StandAloneLeaseSchedule,
    ///SB - Second Notice of Balance Due
    SecondNoticeOfBalanceDue,
    ///SC - Deprescription
    Deprescription,
    ///SD - Supply Process Deficiency
    SupplyProcessDeficiency,
    ///SE - Special Bilateral
    SpecialBilateral,
    ///SF - Single Family Program
    SingleFamilyProgram,
    ///SG - Sample Goods Invoice
    SampleGoodsInvoice,
    ///SH - Shipment Status Notification
    ShipmentStatusNotification,
    ///SI - Sight Certification Request
    SightCertificationRequest,
    ///SL - Summary Lease Schedule
    SummaryLeaseSchedule,
    ///SM - Single Shipper, Multiple Consignees
    CodeSM,
    ///SO - Spot Rate
    SpotRate,
    ///SP - Supplier Rating
    SupplierRating,
    ///SQ - Schedule Query
    ScheduleQuery,
    ///SR - Supply Process Deficiency Response
    SupplyProcessDeficiencyResponse,
    ///SS - Single Shipper, Single Consignee
    CodeSS,
    ///ST - State Royalty
    StateRoyalty,
    ///SU - Survey
    Survey,
    ///SV - Supplemental Invoice
    SupplementalInvoice,
    ///T1 - Report sent by National Center for Education Statistics (NCES)
    CodeT1,
    ///T2 - Report sent to National Center for Education Statistics (NCES)
    CodeT2,
    ///T3 - Common Core of Data (CCD) Report from the National Center for Education Statistics (NCES)
    CodeT3,
    ///T4 - Common Core of Data (CCD) Report to the National Center for Education Statistics (NCES)
    CodeT4,
    ///T5 - Integrated Postsecondary Education Database System (IPEDS) Report from National Center for Education Statistics (NCES)
    CodeT5,
    ///T6 - Integrated Postsecondary Education Database System (IPEDS) Report to National Center for Education Statistics (NCES)
    CodeT6,
    ///T7 - Transportation Invoice
    TransportationInvoice,
    ///T8 - Sales Invoice
    SalesInvoice,
    ///T9 - Service Requester Level Invoice
    ServiceRequesterLevelInvoice,
    ///TB - Buyer Managed Transportation
    BuyerManagedTransportation,
    ///TD - Shipment or Movement Deficiency
    ShipmentOrMovementDeficiency,
    ///TF - Tax or Fee Exemption Certification
    TaxOrFeeExemptionCertification,
    ///TG - Receipt Acknowledgment Inquiry
    ReceiptAcknowledgmentInquiry,
    ///TH - Receipt Acknowledgment Advice
    ReceiptAcknowledgmentAdvice,
    ///TI - Delinquent Due-in Advice
    DelinquentDueInAdvice,
    ///TJ - Delinquent Due-in Inquiry
    DelinquentDueInInquiry,
    ///TK - Due-in Reconciliation Advice
    DueInReconciliationAdvice,
    ///TL - Total Loss Evaluation
    TotalLossEvaluation,
    ///TP - Trading Partner Information
    TradingPartner,
    ///TR - Shipment or Movement Deficiency Response
    ShipmentOrMovementDeficiencyResponse,
    ///TS - Transfer Statement
    TransferStatement,
    ///TT - Testing Service Report
    TestingServiceReport,
    ///TX - Request for Testing Service Report
    RequestForTestingServiceReport,
    ///U1 - Contract Abstract
    ContractAbstract,
    ///U2 - Shipment Performance Notice
    ShipmentPerformanceNotice,
    ///U4 - Acceptance Alert
    AcceptanceAlert,
    ///U5 - Update
    Update,
    ///U9 - Contract Payment Notice
    ContractPaymentNotice,
    ///UA - Amendment Filing
    AmendmentFiling,
    ///UC - Uniform Commercial Code Filing
    UniformCommercialCodeFiling,
    ///UD - Unsalable Detail
    UnsalableDetail,
    ///UF - Material, Engineering, Installation, Right to Use, Final Bill
    CodeUF,
    ///UI - Uniform Commercial Code Filing Inquiry
    UniformCommercialCodeFilingInquiry,
    ///UM - Termination Filing
    TerminationFiling,
    ///UO - Original Filing
    OriginalFiling,
    ///UP - Unsalable Product Invoice
    UnsalableProductInvoice,
    ///UR - Uniform Commercial Code Filing Response to Inquiry
    UniformCommercialCodeFilingResponseToInquiry,
    ///US - Unsubscribed Capacity
    UnsubscribedCapacity,
    ///UT - Continuation Filing
    ContinuationFiling,
    ///V1 - Contract Completion Report
    ContractCompletionReport,
    ///V2 - Nomination Quick Response
    NominationQuickResponse,
    ///V3 - Confirmation Response Quick Response
    ConfirmationResponseQuickResponse,
    ///V4 - Pre-determined Allocation Quick Response
    PreDeterminedAllocationQuickResponse,
    ///V5 - Request for Confirmation Quick Response
    RequestForConfirmationQuickResponse,
    ///VH - Public Voucher
    PublicVoucher,
    ///VJ - Commercial Invoice
    CommercialInvoice,
    ///VL - Violation Notice
    ViolationNotice,
    ///VM - Voluntary MEDWATCH Report
    VoluntaryMedwatchReport,
    ///VN - Mandatory MEDWATCH Report
    MandatoryMedwatchReport,
    ///VO - Medical Device New Baseline Report
    MedicalDeviceNewBaselineReport,
    ///VP - Medical Device Annual Baseline Report
    MedicalDeviceAnnualBaselineReport,
    ///VQ - User Facility Annual Medical Device Report
    UserFacilityAnnualMedicalDeviceReport,
    ///VR - Annual Certification of Medical Device Report
    AnnualCertificationOfMedicalDeviceReport,
    ///W1 - Weapons Data Change
    WeaponsDataChange,
    ///W4 - Weapons Control Report
    WeaponsControlReport,
    ///W5 - Weapons Control Report Reconciliation
    WeaponsControlReportReconciliation,
    ///WA - Work Assignment
    WorkAssignment,
    ///WC - Workers Compensation Proof of Coverage Notification
    WorkersCompensationProofOfCoverageNotification,
    ///WD - Withdrawal
    Withdrawal,
    ///WH - Order to Withhold
    OrderToWithhold,
    ///WO - Work Order
    WorkOrder,
    ///WS - Waste
    Waste,
    ///WT - Warrant
    Warrant,
    ///X1 - Consolidator's Invoice
    ConsolidatorsInvoice,
    ///XA - Cancel Pending New Offer
    CancelPendingNewOffer,
    ///XB - Bilateral Spot Rate
    BilateralSpotRate,
    ///XC - Automatic Concurrence
    AutomaticConcurrence,
    ///XD - Special Deprescription
    SpecialDeprescription,
    ///XX - Firm Order Confirmation with Facility Information
    FirmOrderConfirmationWithFacility,
    ///XY - Firm Order Confirmation
    FirmOrderConfirmation,
    ///XZ - Facility Confirmation
    FacilityConfirmation,
    ///YI - Funds Validation Inquiry
    FundsValidationInquiry,
    ///YR - Funds Validation Response
    FundsValidationResponse,
    ///Z1 - Military Interdepartmental Purchase Request (MIPR)
    CodeZ1,
    ///Z2 - Project Directive
    ProjectDirective,
    ///Z3 - Request for Contractual Procurement
    RequestForContractualProcurement,
    ///Z4 - Reimbursable Work Order
    ReimbursableWorkOrder,
    ///ZA - Request Initiation of Work Candidate
    RequestInitiationOfWorkCandidate,
    ///ZB - Report of Work Candidate
    ReportOfWorkCandidate,
    ///ZC - Report of Assignment or Deletion of Work Candidate to Maintenance Period
    ReportOfAssignmentOrDeletionOfWorkCandidateToMaintenancePeriod,
    ///ZD - Request Assignment of Work Candidate to Planning Maintenance Activity
    RequestAssignmentOfWorkCandidateToPlanningMaintenanceActivity,
    ///ZE - Request for Full Work Candidate Detail
    RequestForFullWorkCandidateDetail,
    ///ZF - Report of Full Work Candidate Detail
    ReportOfFullWorkCandidateDetail,
    ///ZG - Report of Approved Work Candidate
    ReportOfApprovedWorkCandidate,
    ///ZH - Request Work Candidate Cost/Duration Estimate
    RequestWorkCandidateCostDurationEstimate,
    ///ZI - Report of Work Candidate Cost/Duration Estimate
    ReportOfWorkCandidateCostDurationEstimate,
    ///ZJ - Request Work Candidate Planning Services
    RequestWorkCandidatePlanningServices,
    ///ZK - Report of Work Candidate Planning Services
    ReportOfWorkCandidatePlanningServices,
    ///ZL - Report of Assignment or Deletion of Work Candidate to Planning/Maintenance Activity
    ReportOfAssignmentOrDeletionOfWorkCandidateToPlanningMaintenanceActivity,
    ///ZM - Request of Assignment or Deletion of Work Candidate to Maintenance Period
    RequestOfAssignmentOrDeletionOfWorkCandidateToMaintenancePeriod,
    ///ZN - Stop Work Order
    StopWorkOrder,
    ///ZO - Authorization to Continue Work
    AuthorizationToContinueWork,
    ///ZP - Request for Departure From Specification
    RequestForDepartureFromSpecification,
    ///ZQ - Report of Authorized Departure From Specification
    ReportOfAuthorizedDepartureFromSpecification,
    ///ZR - Request Work Progress Status
    RequestWorkProgressStatus,
    ///ZS - Report of Work Progress Status
    ReportOfWorkProgressStatus,
    ///ZT - Report of Rejection or Return of Work Candidate
    ReportOfRejectionOrReturnOfWorkCandidate,
    ///ZU - Request Work Candidate Change
    RequestWorkCandidateChange,
    ///ZV - Lien Filing
    LienFiling,
    ///ZW - Sort and Segregate Detail
    SortAndSegregateDetail,
    ///ZX - Expungement of Prior Filing
    ExpungementOfPriorFiling,
    ///ZY - Cancellation of Filing
    CancellationOfFiling,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl TransactionTypeCode {
    pub fn code(&self) -> &str {
        {
            use TransactionTypeCode::*;
            match self {
                LocationAddressMessage => "01",
                UniqueItemTrackingControlReport => "1A",
                UniqueItemTrackingReportReconciliation => "1B",
                UniqueItemTrackingItemDataChange => "1C",
                NewGroupInitialEnrollment => "1E",
                LocationRelation => "02",
                ReportMessage => "03",
                Supporting => "3M",
                ElectronicMailMessage => "04",
                RequestForCoOp => "05",
                Guidelines => "06",
                AccomplishmentBasedRenewal => "6A",
                CompetitiveRenewal => "6C",
                NonCompetitiveRenewal => "6N",
                Resubmission => "6R",
                Supplemental => "6S",
                Budget => "07",
                Commitment => "08",
                CoOpActual => "09",
                Distribution => "10",
                NationalPropertyRegistrySystemRealEstatePropertyTransaction => "11",
                PhysiciansReport => "12",
                MaintenanceRequest => "13",
                MaintenanceResponse => "14",
                Code15 => "15",
                Code16 => "16",
                RequestWithImmediateResponseToMailbox => "17",
                ResponseNoFurtherUpdatesToFollow => "18",
                ResponseFurtherUpdatesToFollow => "19",
                AirExportWaybillAndInvoice => "20",
                AirImportInvoice => "21",
                OceanExportDirectInvoice => "22",
                OceanExportIndirectInvoice => "23",
                OceanExportBrokerageInvoice => "24",
                OceanImportInvoice => "25",
                MiscellaneousServicesInvoice => "26",
                WarehouseServicesInvoice => "27",
                PreRegistration => "28",
                Delivery => "30",
                SubrogationDemand => "31",
                Normal => "33",
                EmergencyRequest => "34",
                ShortNoticeRequest => "35",
                DamageNotification => "36",
                DesignReport => "37",
                Test => "38",
                NoticeOfOccupationalDisease => "39",
                NoticeOfTraumaticInjury => "40",
                StatementOfFinancialAffairs => "41",
                CustomerShipment => "42",
                InternationalShipment => "43",
                InterplantShipment => "44",
                ProductionToCarrierShipment => "45",
                AdvancedPayment => "46",
                Delinquency => "47",
                Payment => "48",
                Audit => "49",
                TransferLoanIn => "50",
                TransferLoanOut => "51",
                MailingAddressServices => "52",
                Code54 => "54",
                NewServiceOrder => "55",
                DeniedRequest => "56",
                ServiceTermination => "57",
                TaxBillingOrRemittance => "58",
                MaterialInTransit => "60",
                PreawardNotification => "62",
                PostawardNotification => "63",
                SmallBusinessAwardNotification => "64",
                AwardNotification => "65",
                NotificationToLateBidders => "66",
                NotificationOfProtestReceipt => "67",
                AbstractOfOffers => "68",
                BidRejectionNotice => "69",
                UnsolicitedBidNotice => "70",
                FilingAndSearchRequest => "71",
                Termination => "72",
                FilingOfficerConfirmation => "73",
                Amendment => "74",
                FullAssignment => "75",
                PartialAssignment => "76",
                FullReleaseOfCollateral => "77",
                PartialReleaseOfCollateral => "78",
                Continuation => "79",
                UniformCommercialCodeFilingWithBroadCollateralDescription => "80",
                UniformCommercialCodeFilingWithSpecificCollateralDescription => "81",
                SegregationByPurchaseOrderNumber => "82",
                SegregationByCarrierProNumber => "83",
                ResponseNoActionTaken => "85",
                DisabilityBenefitsProofOfCoverageNotification => "87",
                DebtorFiling => "88",
                FederalOccupationalReporting => "89",
                SecuredPartyFiling => "91",
                PartialReleaseFiling => "94",
                Code95 => "95",
                MultipleListing => "97",
                Sale => "98",
                Loan => "99",
                Requisition => "A0",
                StorageItemDataChange => "A1",
                AdministrativeFees => "A3",
                AmendedReturn => "A4",
                NoticeOfCharge => "A5",
                ProtestOrResponseToCharge => "A6",
                ChargeDecision => "A7",
                DueToAnalysis => "AA",
                AdvanceShipmentAndBillingNotice => "AB",
                RequisitionCancellation => "AC",
                NoticeOfAvailability => "AD",
                CodeAE => "AE",
                RequisitionFollowUp => "AF",
                CodeAG => "AG",
                PreEmploymentScreening => "AH",
                AdjustedInvoice => "AI",
                CodeAJ => "AJ",
                CodeAK => "AK",
                Arbitration => "AL",
                RequisitionModification => "AM",
                MaterialObligationInquiry => "AN",
                MaterialObligationAdvice => "AP",
                QuantityVerificationInquiry => "AQ",
                SupplyAssistance => "AR",
                ShipmentAdvice => "AS",
                AdministrativeAction => "AT",
                QuantityVerificationAdvice => "AV",
                MaterialObligationReinstatement => "AW",
                DisposalShipmentConfirmation => "AZ",
                BackbillInvoice => "BA",
                Billback => "BB",
                StatementOfBalanceDue => "BD",
                BestAndFinalOffer => "BF",
                NonEscrowOrNonImpoundTracking => "BG",
                BillAndHoldGoods => "BH",
                BankruptcyPetition => "BJ",
                BankruptcySchedules => "BK",
                BlanketLeaseSchedule => "BL",
                RequisitionPassingOrder => "BM",
                RequisitionReferralOrder => "BN",
                ProcurementCancellation => "BO",
                ProcurementModification => "BP",
                BusinessTaxpayerRegistration => "BR",
                BillOfSaleSchedule => "BS",
                BalanceAndTransactionReport => "BT",
                WorkersCompensation1StReportOfInjury => "BU",
                WorkersCompensationSubsequentReport => "BV",
                WorkersCompensationCombined1StAndSubsequentReport => "BW",
                NotForResaleInvoice => "BX",
                FreightInvoice => "BZ",
                CustomsEntryDetail => "C0",
                Claim => "C1",
                CaseOpening => "C2",
                Consignment => "C3",
                EscrowOrImpoundServiceReporting => "C4",
                TaxAssessmentBill => "C5",
                FixedAssetsTaxReturn => "C6",
                ServiceCancellation => "C7",
                ConformedCopy => "C8",
                SubjectToAvailabilityOfFunds => "C9",
                Cash => "CA",
                Contribution => "CB",
                CreditCommissionInvoice => "CC",
                ConsolidatedDebitInvoice => "CD",
                ConsolidatedCreditInvoice => "CE",
                ConsolidatedDebitMemo => "CF",
                ConsolidatedCreditMemo => "CG",
                Chargeable => "CH",
                ConsolidatedInvoice => "CI",
                Confirmation => "CJ",
                ClaimSubmission => "CK",
                CustomerAllocation => "CL",
                CallDetailMemo => "CM",
                CreditInvoice => "CN",
                Corrected => "CO",
                CommissionPayment => "CP",
                CreditMemo => "CR",
                CashSurrenderDistribution => "CS",
                CostTypeInvoice => "CT",
                CargoOutturnReport => "CU",
                CostVoucher => "CV",
                CashLetter => "CW",
                CheckList => "CX",
                CitationToPayOrAppear => "CY",
                ConvictionNotice => "CZ",
                DividendPayment => "D1",
                Receipt => "D4",
                DueIn => "DA",
                DetourBilling => "DB",
                DebitCommissionInvoice => "DC",
                CodeDD => "DD",
                AdvanceReceipt => "DE",
                DieselFuelBill => "DF",
                Response => "DG",
                DiscretionaryAdditionalCompanyContribution => "DH",
                DebitInvoice => "DI",
                DutyDrawback => "DK",
                DepositList => "DL",
                PriorDamageReport => "DM",
                DirectNonqualifiedRollover => "DN",
                DropShipmentInvoice => "DO",
                DistrictToPostsecondaryStudentRecord => "DP",
                DirectQualifiedRollover => "DQ",
                DebitMemo => "DR",
                Disposition => "DS",
                Detail => "DT",
                Duplicate => "DU",
                AdministrativeRecordsSubmission => "E1",
                ExcessInterestAllocation => "EA",
                EngineeringFinalBill => "EB",
                CampaignFiling => "EC",
                LobbyistFiling => "ED",
                CodeEF => "EF",
                CodeEI => "EI",
                EstimateOfRecord => "EM",
                ExpensePayment => "EP",
                CodeER => "ER",
                ExcessMaterialNotification => "EX",
                FinalReport => "F1",
                PreApprovedBiddersList => "F2",
                PreDeterminedAllocation => "F4",
                Allocation => "F5",
                ShipperImbalance => "F6",
                ProducerImbalance => "F7",
                StorageReport => "F8",
                ForfeitureAllocation => "FA",
                FinalBill => "FB",
                ForfeitureCredit => "FC",
                CodeFD => "FD",
                CodeFE => "FE",
                FullAssignmentFiling => "FF",
                Filing => "FG",
                FirstCostInvoice => "FI",
                FinalNotice => "FL",
                FundingModification => "FM",
                FirstNoticeOfLoss => "FN",
                FlatRatePerUnitBill => "FP",
                FullEnrollmentFile => "FQ",
                FederalRoyalty => "FR",
                FinancialStatementReport => "FS",
                MaterialReturns => "FT",
                Nomination => "G1",
                RequestForConfirmation => "G2",
                ConfirmationResponse => "G3",
                GovernmentFurnishedMaterialInquiryAdvice => "GA",
                GovernmentFurnishedMaterialInquiry => "GI",
                Garnishment => "GR",
                CreditReport => "GS",
                DisabilityNotice => "GT",
                BlackLungClaim => "GU",
                ClaimExperienceReport => "GV",
                EmployersReport => "GW",
                LongshoreReport => "GX",
                UnitReport => "GY",
                HazardousWasteReport => "HA",
                DischargeMonitoringReport => "HB",
                RiskManagementPlan => "HC",
                SelfMonitoringReport => "HD",
                HazardousAirPollutantInventoryReport => "HE",
                StationaryPointSourceInventoryReport => "HF",
                ToxicReleaseInventoryReport => "HG",
                HorsepowerEqualizationBill => "HP",
                HandlingCarrierAgreementUpdate => "HX",
                InAdCouponNotification => "I1",
                Inventory => "IA",
                InstallationFinalBill => "IB",
                InsuranceCoverageNotification => "IC",
                EmployersReportOfDisability => "ID",
                IndianRoyalty => "IE",
                CodeIF => "IF",
                InterfundTransferIn => "II",
                IncidentNotice => "IM",
                Inquiry => "IN",
                InterfundTransferOut => "IO",
                CodeIR => "IR",
                CodeIU => "IU",
                WorkersCompensationReportOfInjuryOrIllness => "IW",
                InterchangeUpdate => "IX",
                InvestmentFees => "IZ",
                MaintenanceAndOperationsBill => "JM",
                JointFacilityMiscellaneousBillOrOther => "JO",
                RentalBill => "JR",
                JunctionSettlementUpdate => "JS",
                Judgment => "JU",
                JunctionUpdate => "JX",
                TerminationForDefault => "KB",
                DefinitizationOfContract => "KC",
                DefinitizationOfOrder => "KD",
                ExerciseOfOption => "KE",
                IntentToExerciseOption => "KF",
                AdministrativeChange => "KG",
                ChangeOrder => "KH",
                SupplementalAgreement => "KI",
                AmendedShippingInstructions => "KJ",
                ProvisionedItem => "KK",
                WithdrawalOfOffer => "KL",
                AdditionsToGeneralProvisions => "KM",
                RequestForPriceQuote => "KN",
                AdditionToSolicitationMailingList => "KS",
                TerminationForConvenience => "KT",
                DueInReconciliationInquiry => "LC",
                LoanDistribution => "LD",
                LoanRepaymentExpense => "LE",
                LandedCosts => "LF",
                LossNotification => "LN",
                LoanRepaymentToPrincipalOnly => "LO",
                LoanRepayment => "LP",
                LogisticsReassignment => "LR",
                Levy => "LV",
                ManufacturerCouponNotification => "M1",
                MeasurementEventsAndAlarms => "M5",
                MailingList => "MA",
                MaintenanceToBusinessTaxpayerRegistration => "MB",
                MaterialCreditInvoice => "MC",
                MarketDevelopmentFund => "MD",
                Memorandum => "ME",
                CodeMF => "MF",
                CodeMI => "MI",
                MembershipList => "ML",
                CodeMM => "MM",
                CodeMP => "MP",
                CodeMR => "MR",
                MaterialFinalBill => "MS",
                MultifamilyProgram => "MU",
                Bilateral => "N1",
                NoticeOfAssessment => "N2",
                NoticeOfWarrant => "N3",
                NoticeOfAdjustment => "N4",
                NoticeOfDetermination => "N5",
                NoticeOfSettlement => "N6",
                NoticeOfRecordedLien => "N7",
                NoticeOfDeficiency => "N8",
                MaterialReleaseOrder => "NA",
                MaterialReleaseInquiry => "NB",
                MaterialReleaseOrderForcedClosure => "NC",
                MaterialReleaseCancellation => "ND",
                DisposalReleaseOrder => "NE",
                DisposalReleaseInquiry => "NF",
                DisposalReleaseCancellation => "NG",
                DisposalShipmentConfirmationInquiry => "NH",
                RedistributionOrder => "NI",
                MaterialReleaseConfirmation => "NJ",
                MaterialReleaseDenial => "NK",
                MaterialReleaseAdvice => "NL",
                DisposalReleaseConfirmation => "NM",
                Notice => "NO",
                DisposalReleaseDenial => "NP",
                DisposalReleaseAdvice => "NQ",
                MaterialReleaseCancellationAdvice => "NR",
                InTransit => "NS",
                DisposalReleaseCancellationAdvice => "NT",
                InventoryAdjustment => "NU",
                OperationalCapacity => "OA",
                OnApproval => "OC",
                Offer => "OF",
                Opinion => "OP",
                Order => "OR",
                Preliminary => "P1",
                EmployerGroupChange => "P2",
                IndividualChange => "P3",
                EmployerOpenEnrollment => "P4",
                PredeterminationMedical => "P5",
                PredeterminationDental => "P6",
                ProgressPaymentInvoice => "PA",
                PartialBill => "PB",
                InventionReport => "PC",
                ProductAllocation => "PD",
                Pleading => "PE",
                PartialAssignmentFiling => "PF",
                PremiumRoutingGuide => "PG",
                ProspectiveStudent => "PH",
                PersonalInjuryBill => "PI",
                ComponentPackingConfirmation => "PJ",
                PlanAllocation => "PL",
                PremiumPayment => "PM",
                PlanTakeover => "PO",
                PrepaidInvoice => "PP",
                PartialEnrollmentFile => "PQ",
                CodePR => "PR",
                PostsecondaryStudentAcademicRecord => "PS",
                PlanToPlanTransfer => "PT",
                NoticeOfClaim => "PU",
                ProtestOrResponseToClaim => "PV",
                ClaimDecision => "PW",
                WageVerificationNotice => "PX",
                PurchaseReport => "PZ",
                ScheduledQuantity => "Q1",
                ScheduledQuantityForOperator => "Q2",
                CouponRegularClearinghouseInvoice => "QA",
                CouponDirectRetailerInvoice => "QB",
                CouponClearinghousePayDirectInvoice => "QC",
                ProductQualityDeficiency => "QD",
                CouponScanValidateInvoice => "QE",
                ScanValidateAdjustment => "QF",
                QuickResponseRoutingGuide => "QG",
                CodeQH => "QH",
                AdjustedCouponRedemption => "QJ",
                CouponRedemptionDetail => "QK",
                AdjustmentsToPreviousCouponRedemption => "QL",
                CouponQuickPayInvoice => "QP",
                ProductQualityDeficiencyResponse => "QR",
                RequestForEnrollmentVerification => "R1",
                ResponseToRequestForEnrollmentVerification => "R2",
                ResponseToGarnishment => "R3",
                ReleaseOfGarnishment => "R4",
                ResponseToLevy => "R5",
                ReleaseOfLevy => "R6",
                ResponseToOrderToWithhold => "R7",
                ReleaseOfOrderToWithhold => "R8",
                RequestForCredit => "RA",
                RightToUse => "RB",
                RequestForQuote => "RC",
                ReturnsDetail => "RD",
                Rebill => "RE",
                CodeRF => "RF",
                RevisedFinalBill => "RG",
                RequestForAdditionalFunds => "RH",
                RoutingInstructions => "RI",
                ResponseToRequestForRoutingInstructions => "RJ",
                Registration => "RK",
                ReminderToFile => "RM",
                Reporting => "RP",
                Request => "RQ",
                CodeRS => "RS",
                SpendDown => "RT",
                MedicalServicesReservation => "RU",
                RemovedFromSolicitationMailingList => "RZ",
                SpecialRoutingGuide => "S1",
                StandardRoutingGuide => "S2",
                SupplementalLoanRepayment => "S3",
                Submission => "S4",
                StandAloneLeaseSchedule => "SA",
                SecondNoticeOfBalanceDue => "SB",
                Deprescription => "SC",
                SupplyProcessDeficiency => "SD",
                SpecialBilateral => "SE",
                SingleFamilyProgram => "SF",
                SampleGoodsInvoice => "SG",
                ShipmentStatusNotification => "SH",
                SightCertificationRequest => "SI",
                SummaryLeaseSchedule => "SL",
                CodeSM => "SM",
                SpotRate => "SO",
                SupplierRating => "SP",
                ScheduleQuery => "SQ",
                SupplyProcessDeficiencyResponse => "SR",
                CodeSS => "SS",
                StateRoyalty => "ST",
                Survey => "SU",
                SupplementalInvoice => "SV",
                CodeT1 => "T1",
                CodeT2 => "T2",
                CodeT3 => "T3",
                CodeT4 => "T4",
                CodeT5 => "T5",
                CodeT6 => "T6",
                TransportationInvoice => "T7",
                SalesInvoice => "T8",
                ServiceRequesterLevelInvoice => "T9",
                BuyerManagedTransportation => "TB",
                ShipmentOrMovementDeficiency => "TD",
                TaxOrFeeExemptionCertification => "TF",
                ReceiptAcknowledgmentInquiry => "TG",
                ReceiptAcknowledgmentAdvice => "TH",
                DelinquentDueInAdvice => "TI",
                DelinquentDueInInquiry => "TJ",
                DueInReconciliationAdvice => "TK",
                TotalLossEvaluation => "TL",
                TradingPartner => "TP",
                ShipmentOrMovementDeficiencyResponse => "TR",
                TransferStatement => "TS",
                TestingServiceReport => "TT",
                RequestForTestingServiceReport => "TX",
                ContractAbstract => "U1",
                ShipmentPerformanceNotice => "U2",
                AcceptanceAlert => "U4",
                Update => "U5",
                ContractPaymentNotice => "U9",
                AmendmentFiling => "UA",
                UniformCommercialCodeFiling => "UC",
                UnsalableDetail => "UD",
                CodeUF => "UF",
                UniformCommercialCodeFilingInquiry => "UI",
                TerminationFiling => "UM",
                OriginalFiling => "UO",
                UnsalableProductInvoice => "UP",
                UniformCommercialCodeFilingResponseToInquiry => "UR",
                UnsubscribedCapacity => "US",
                ContinuationFiling => "UT",
                ContractCompletionReport => "V1",
                NominationQuickResponse => "V2",
                ConfirmationResponseQuickResponse => "V3",
                PreDeterminedAllocationQuickResponse => "V4",
                RequestForConfirmationQuickResponse => "V5",
                PublicVoucher => "VH",
                CommercialInvoice => "VJ",
                ViolationNotice => "VL",
                VoluntaryMedwatchReport => "VM",
                MandatoryMedwatchReport => "VN",
                MedicalDeviceNewBaselineReport => "VO",
                MedicalDeviceAnnualBaselineReport => "VP",
                UserFacilityAnnualMedicalDeviceReport => "VQ",
                AnnualCertificationOfMedicalDeviceReport => "VR",
                WeaponsDataChange => "W1",
                WeaponsControlReport => "W4",
                WeaponsControlReportReconciliation => "W5",
                WorkAssignment => "WA",
                WorkersCompensationProofOfCoverageNotification => "WC",
                Withdrawal => "WD",
                OrderToWithhold => "WH",
                WorkOrder => "WO",
                Waste => "WS",
                Warrant => "WT",
                ConsolidatorsInvoice => "X1",
                CancelPendingNewOffer => "XA",
                BilateralSpotRate => "XB",
                AutomaticConcurrence => "XC",
                SpecialDeprescription => "XD",
                FirmOrderConfirmationWithFacility => "XX",
                FirmOrderConfirmation => "XY",
                FacilityConfirmation => "XZ",
                FundsValidationInquiry => "YI",
                FundsValidationResponse => "YR",
                CodeZ1 => "Z1",
                ProjectDirective => "Z2",
                RequestForContractualProcurement => "Z3",
                ReimbursableWorkOrder => "Z4",
                RequestInitiationOfWorkCandidate => "ZA",
                ReportOfWorkCandidate => "ZB",
                ReportOfAssignmentOrDeletionOfWorkCandidateToMaintenancePeriod => "ZC",
                RequestAssignmentOfWorkCandidateToPlanningMaintenanceActivity => "ZD",
                RequestForFullWorkCandidateDetail => "ZE",
                ReportOfFullWorkCandidateDetail => "ZF",
                ReportOfApprovedWorkCandidate => "ZG",
                RequestWorkCandidateCostDurationEstimate => "ZH",
                ReportOfWorkCandidateCostDurationEstimate => "ZI",
                RequestWorkCandidatePlanningServices => "ZJ",
                ReportOfWorkCandidatePlanningServices => "ZK",
                ReportOfAssignmentOrDeletionOfWorkCandidateToPlanningMaintenanceActivity => {
                    "ZL"
                }
                RequestOfAssignmentOrDeletionOfWorkCandidateToMaintenancePeriod => "ZM",
                StopWorkOrder => "ZN",
                AuthorizationToContinueWork => "ZO",
                RequestForDepartureFromSpecification => "ZP",
                ReportOfAuthorizedDepartureFromSpecification => "ZQ",
                RequestWorkProgressStatus => "ZR",
                ReportOfWorkProgressStatus => "ZS",
                ReportOfRejectionOrReturnOfWorkCandidate => "ZT",
                RequestWorkCandidateChange => "ZU",
                LienFiling => "ZV",
                SortAndSegregateDetail => "ZW",
                ExpungementOfPriorFiling => "ZX",
                CancellationOfFiling => "ZY",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<TransactionTypeCode> {
        use TransactionTypeCode::*;
        match code {
            b"01" => Some(LocationAddressMessage),
            b"1A" => Some(UniqueItemTrackingControlReport),
            b"1B" => Some(UniqueItemTrackingReportReconciliation),
            b"1C" => Some(UniqueItemTrackingItemDataChange),
            b"1E" => Some(NewGroupInitialEnrollment),
            b"02" => Some(LocationRelation),
            b"03" => Some(ReportMessage),
            b"3M" => Some(Supporting),
            b"04" => Some(ElectronicMailMessage),
            b"05" => Some(RequestForCoOp),
            b"06" => Some(Guidelines),
            b"6A" => Some(AccomplishmentBasedRenewal),
            b"6C" => Some(CompetitiveRenewal),
            b"6N" => Some(NonCompetitiveRenewal),
            b"6R" => Some(Resubmission),
            b"6S" => Some(Supplemental),
            b"07" => Some(Budget),
            b"08" => Some(Commitment),
            b"09" => Some(CoOpActual),
            b"10" => Some(Distribution),
            b"11" => Some(NationalPropertyRegistrySystemRealEstatePropertyTransaction),
            b"12" => Some(PhysiciansReport),
            b"13" => Some(MaintenanceRequest),
            b"14" => Some(MaintenanceResponse),
            b"15" => Some(Code15),
            b"16" => Some(Code16),
            b"17" => Some(RequestWithImmediateResponseToMailbox),
            b"18" => Some(ResponseNoFurtherUpdatesToFollow),
            b"19" => Some(ResponseFurtherUpdatesToFollow),
            b"20" => Some(AirExportWaybillAndInvoice),
            b"21" => Some(AirImportInvoice),
            b"22" => Some(OceanExportDirectInvoice),
            b"23" => Some(OceanExportIndirectInvoice),
            b"24" => Some(OceanExportBrokerageInvoice),
            b"25" => Some(OceanImportInvoice),
            b"26" => Some(MiscellaneousServicesInvoice),
            b"27" => Some(WarehouseServicesInvoice),
            b"28" => Some(PreRegistration),
            b"30" => Some(Delivery),
            b"31" => Some(SubrogationDemand),
            b"33" => Some(Normal),
            b"34" => Some(EmergencyRequest),
            b"35" => Some(ShortNoticeRequest),
            b"36" => Some(DamageNotification),
            b"37" => Some(DesignReport),
            b"38" => Some(Test),
            b"39" => Some(NoticeOfOccupationalDisease),
            b"40" => Some(NoticeOfTraumaticInjury),
            b"41" => Some(StatementOfFinancialAffairs),
            b"42" => Some(CustomerShipment),
            b"43" => Some(InternationalShipment),
            b"44" => Some(InterplantShipment),
            b"45" => Some(ProductionToCarrierShipment),
            b"46" => Some(AdvancedPayment),
            b"47" => Some(Delinquency),
            b"48" => Some(Payment),
            b"49" => Some(Audit),
            b"50" => Some(TransferLoanIn),
            b"51" => Some(TransferLoanOut),
            b"52" => Some(MailingAddressServices),
            b"54" => Some(Code54),
            b"55" => Some(NewServiceOrder),
            b"56" => Some(DeniedRequest),
            b"57" => Some(ServiceTermination),
            b"58" => Some(TaxBillingOrRemittance),
            b"60" => Some(MaterialInTransit),
            b"62" => Some(PreawardNotification),
            b"63" => Some(PostawardNotification),
            b"64" => Some(SmallBusinessAwardNotification),
            b"65" => Some(AwardNotification),
            b"66" => Some(NotificationToLateBidders),
            b"67" => Some(NotificationOfProtestReceipt),
            b"68" => Some(AbstractOfOffers),
            b"69" => Some(BidRejectionNotice),
            b"70" => Some(UnsolicitedBidNotice),
            b"71" => Some(FilingAndSearchRequest),
            b"72" => Some(Termination),
            b"73" => Some(FilingOfficerConfirmation),
            b"74" => Some(Amendment),
            b"75" => Some(FullAssignment),
            b"76" => Some(PartialAssignment),
            b"77" => Some(FullReleaseOfCollateral),
            b"78" => Some(PartialReleaseOfCollateral),
            b"79" => Some(Continuation),
            b"80" => Some(UniformCommercialCodeFilingWithBroadCollateralDescription),
            b"81" => Some(UniformCommercialCodeFilingWithSpecificCollateralDescription),
            b"82" => Some(SegregationByPurchaseOrderNumber),
            b"83" => Some(SegregationByCarrierProNumber),
            b"85" => Some(ResponseNoActionTaken),
            b"87" => Some(DisabilityBenefitsProofOfCoverageNotification),
            b"88" => Some(DebtorFiling),
            b"89" => Some(FederalOccupationalReporting),
            b"91" => Some(SecuredPartyFiling),
            b"94" => Some(PartialReleaseFiling),
            b"95" => Some(Code95),
            b"97" => Some(MultipleListing),
            b"98" => Some(Sale),
            b"99" => Some(Loan),
            b"A0" => Some(Requisition),
            b"A1" => Some(StorageItemDataChange),
            b"A3" => Some(AdministrativeFees),
            b"A4" => Some(AmendedReturn),
            b"A5" => Some(NoticeOfCharge),
            b"A6" => Some(ProtestOrResponseToCharge),
            b"A7" => Some(ChargeDecision),
            b"AA" => Some(DueToAnalysis),
            b"AB" => Some(AdvanceShipmentAndBillingNotice),
            b"AC" => Some(RequisitionCancellation),
            b"AD" => Some(NoticeOfAvailability),
            b"AE" => Some(CodeAE),
            b"AF" => Some(RequisitionFollowUp),
            b"AG" => Some(CodeAG),
            b"AH" => Some(PreEmploymentScreening),
            b"AI" => Some(AdjustedInvoice),
            b"AJ" => Some(CodeAJ),
            b"AK" => Some(CodeAK),
            b"AL" => Some(Arbitration),
            b"AM" => Some(RequisitionModification),
            b"AN" => Some(MaterialObligationInquiry),
            b"AP" => Some(MaterialObligationAdvice),
            b"AQ" => Some(QuantityVerificationInquiry),
            b"AR" => Some(SupplyAssistance),
            b"AS" => Some(ShipmentAdvice),
            b"AT" => Some(AdministrativeAction),
            b"AV" => Some(QuantityVerificationAdvice),
            b"AW" => Some(MaterialObligationReinstatement),
            b"AZ" => Some(DisposalShipmentConfirmation),
            b"BA" => Some(BackbillInvoice),
            b"BB" => Some(Billback),
            b"BD" => Some(StatementOfBalanceDue),
            b"BF" => Some(BestAndFinalOffer),
            b"BG" => Some(NonEscrowOrNonImpoundTracking),
            b"BH" => Some(BillAndHoldGoods),
            b"BJ" => Some(BankruptcyPetition),
            b"BK" => Some(BankruptcySchedules),
            b"BL" => Some(BlanketLeaseSchedule),
            b"BM" => Some(RequisitionPassingOrder),
            b"BN" => Some(RequisitionReferralOrder),
            b"BO" => Some(ProcurementCancellation),
            b"BP" => Some(ProcurementModification),
            b"BR" => Some(BusinessTaxpayerRegistration),
            b"BS" => Some(BillOfSaleSchedule),
            b"BT" => Some(BalanceAndTransactionReport),
            b"BU" => Some(WorkersCompensation1StReportOfInjury),
            b"BV" => Some(WorkersCompensationSubsequentReport),
            b"BW" => Some(WorkersCompensationCombined1StAndSubsequentReport),
            b"BX" => Some(NotForResaleInvoice),
            b"BZ" => Some(FreightInvoice),
            b"C0" => Some(CustomsEntryDetail),
            b"C1" => Some(Claim),
            b"C2" => Some(CaseOpening),
            b"C3" => Some(Consignment),
            b"C4" => Some(EscrowOrImpoundServiceReporting),
            b"C5" => Some(TaxAssessmentBill),
            b"C6" => Some(FixedAssetsTaxReturn),
            b"C7" => Some(ServiceCancellation),
            b"C8" => Some(ConformedCopy),
            b"C9" => Some(SubjectToAvailabilityOfFunds),
            b"CA" => Some(Cash),
            b"CB" => Some(Contribution),
            b"CC" => Some(CreditCommissionInvoice),
            b"CD" => Some(ConsolidatedDebitInvoice),
            b"CE" => Some(ConsolidatedCreditInvoice),
            b"CF" => Some(ConsolidatedDebitMemo),
            b"CG" => Some(ConsolidatedCreditMemo),
            b"CH" => Some(Chargeable),
            b"CI" => Some(ConsolidatedInvoice),
            b"CJ" => Some(Confirmation),
            b"CK" => Some(ClaimSubmission),
            b"CL" => Some(CustomerAllocation),
            b"CM" => Some(CallDetailMemo),
            b"CN" => Some(CreditInvoice),
            b"CO" => Some(Corrected),
            b"CP" => Some(CommissionPayment),
            b"CR" => Some(CreditMemo),
            b"CS" => Some(CashSurrenderDistribution),
            b"CT" => Some(CostTypeInvoice),
            b"CU" => Some(CargoOutturnReport),
            b"CV" => Some(CostVoucher),
            b"CW" => Some(CashLetter),
            b"CX" => Some(CheckList),
            b"CY" => Some(CitationToPayOrAppear),
            b"CZ" => Some(ConvictionNotice),
            b"D1" => Some(DividendPayment),
            b"D4" => Some(Receipt),
            b"DA" => Some(DueIn),
            b"DB" => Some(DetourBilling),
            b"DC" => Some(DebitCommissionInvoice),
            b"DD" => Some(CodeDD),
            b"DE" => Some(AdvanceReceipt),
            b"DF" => Some(DieselFuelBill),
            b"DG" => Some(Response),
            b"DH" => Some(DiscretionaryAdditionalCompanyContribution),
            b"DI" => Some(DebitInvoice),
            b"DK" => Some(DutyDrawback),
            b"DL" => Some(DepositList),
            b"DM" => Some(PriorDamageReport),
            b"DN" => Some(DirectNonqualifiedRollover),
            b"DO" => Some(DropShipmentInvoice),
            b"DP" => Some(DistrictToPostsecondaryStudentRecord),
            b"DQ" => Some(DirectQualifiedRollover),
            b"DR" => Some(DebitMemo),
            b"DS" => Some(Disposition),
            b"DT" => Some(Detail),
            b"DU" => Some(Duplicate),
            b"E1" => Some(AdministrativeRecordsSubmission),
            b"EA" => Some(ExcessInterestAllocation),
            b"EB" => Some(EngineeringFinalBill),
            b"EC" => Some(CampaignFiling),
            b"ED" => Some(LobbyistFiling),
            b"EF" => Some(CodeEF),
            b"EI" => Some(CodeEI),
            b"EM" => Some(EstimateOfRecord),
            b"EP" => Some(ExpensePayment),
            b"ER" => Some(CodeER),
            b"EX" => Some(ExcessMaterialNotification),
            b"F1" => Some(FinalReport),
            b"F2" => Some(PreApprovedBiddersList),
            b"F4" => Some(PreDeterminedAllocation),
            b"F5" => Some(Allocation),
            b"F6" => Some(ShipperImbalance),
            b"F7" => Some(ProducerImbalance),
            b"F8" => Some(StorageReport),
            b"FA" => Some(ForfeitureAllocation),
            b"FB" => Some(FinalBill),
            b"FC" => Some(ForfeitureCredit),
            b"FD" => Some(CodeFD),
            b"FE" => Some(CodeFE),
            b"FF" => Some(FullAssignmentFiling),
            b"FG" => Some(Filing),
            b"FI" => Some(FirstCostInvoice),
            b"FL" => Some(FinalNotice),
            b"FM" => Some(FundingModification),
            b"FN" => Some(FirstNoticeOfLoss),
            b"FP" => Some(FlatRatePerUnitBill),
            b"FQ" => Some(FullEnrollmentFile),
            b"FR" => Some(FederalRoyalty),
            b"FS" => Some(FinancialStatementReport),
            b"FT" => Some(MaterialReturns),
            b"G1" => Some(Nomination),
            b"G2" => Some(RequestForConfirmation),
            b"G3" => Some(ConfirmationResponse),
            b"GA" => Some(GovernmentFurnishedMaterialInquiryAdvice),
            b"GI" => Some(GovernmentFurnishedMaterialInquiry),
            b"GR" => Some(Garnishment),
            b"GS" => Some(CreditReport),
            b"GT" => Some(DisabilityNotice),
            b"GU" => Some(BlackLungClaim),
            b"GV" => Some(ClaimExperienceReport),
            b"GW" => Some(EmployersReport),
            b"GX" => Some(LongshoreReport),
            b"GY" => Some(UnitReport),
            b"HA" => Some(HazardousWasteReport),
            b"HB" => Some(DischargeMonitoringReport),
            b"HC" => Some(RiskManagementPlan),
            b"HD" => Some(SelfMonitoringReport),
            b"HE" => Some(HazardousAirPollutantInventoryReport),
            b"HF" => Some(StationaryPointSourceInventoryReport),
            b"HG" => Some(ToxicReleaseInventoryReport),
            b"HP" => Some(HorsepowerEqualizationBill),
            b"HX" => Some(HandlingCarrierAgreementUpdate),
            b"I1" => Some(InAdCouponNotification),
            b"IA" => Some(Inventory),
            b"IB" => Some(InstallationFinalBill),
            b"IC" => Some(InsuranceCoverageNotification),
            b"ID" => Some(EmployersReportOfDisability),
            b"IE" => Some(IndianRoyalty),
            b"IF" => Some(CodeIF),
            b"II" => Some(InterfundTransferIn),
            b"IM" => Some(IncidentNotice),
            b"IN" => Some(Inquiry),
            b"IO" => Some(InterfundTransferOut),
            b"IR" => Some(CodeIR),
            b"IU" => Some(CodeIU),
            b"IW" => Some(WorkersCompensationReportOfInjuryOrIllness),
            b"IX" => Some(InterchangeUpdate),
            b"IZ" => Some(InvestmentFees),
            b"JM" => Some(MaintenanceAndOperationsBill),
            b"JO" => Some(JointFacilityMiscellaneousBillOrOther),
            b"JR" => Some(RentalBill),
            b"JS" => Some(JunctionSettlementUpdate),
            b"JU" => Some(Judgment),
            b"JX" => Some(JunctionUpdate),
            b"KB" => Some(TerminationForDefault),
            b"KC" => Some(DefinitizationOfContract),
            b"KD" => Some(DefinitizationOfOrder),
            b"KE" => Some(ExerciseOfOption),
            b"KF" => Some(IntentToExerciseOption),
            b"KG" => Some(AdministrativeChange),
            b"KH" => Some(ChangeOrder),
            b"KI" => Some(SupplementalAgreement),
            b"KJ" => Some(AmendedShippingInstructions),
            b"KK" => Some(ProvisionedItem),
            b"KL" => Some(WithdrawalOfOffer),
            b"KM" => Some(AdditionsToGeneralProvisions),
            b"KN" => Some(RequestForPriceQuote),
            b"KS" => Some(AdditionToSolicitationMailingList),
            b"KT" => Some(TerminationForConvenience),
            b"LC" => Some(DueInReconciliationInquiry),
            b"LD" => Some(LoanDistribution),
            b"LE" => Some(LoanRepaymentExpense),
            b"LF" => Some(LandedCosts),
            b"LN" => Some(LossNotification),
            b"LO" => Some(LoanRepaymentToPrincipalOnly),
            b"LP" => Some(LoanRepayment),
            b"LR" => Some(LogisticsReassignment),
            b"LV" => Some(Levy),
            b"M1" => Some(ManufacturerCouponNotification),
            b"M5" => Some(MeasurementEventsAndAlarms),
            b"MA" => Some(MailingList),
            b"MB" => Some(MaintenanceToBusinessTaxpayerRegistration),
            b"MC" => Some(MaterialCreditInvoice),
            b"MD" => Some(MarketDevelopmentFund),
            b"ME" => Some(Memorandum),
            b"MF" => Some(CodeMF),
            b"MI" => Some(CodeMI),
            b"ML" => Some(MembershipList),
            b"MM" => Some(CodeMM),
            b"MP" => Some(CodeMP),
            b"MR" => Some(CodeMR),
            b"MS" => Some(MaterialFinalBill),
            b"MU" => Some(MultifamilyProgram),
            b"N1" => Some(Bilateral),
            b"N2" => Some(NoticeOfAssessment),
            b"N3" => Some(NoticeOfWarrant),
            b"N4" => Some(NoticeOfAdjustment),
            b"N5" => Some(NoticeOfDetermination),
            b"N6" => Some(NoticeOfSettlement),
            b"N7" => Some(NoticeOfRecordedLien),
            b"N8" => Some(NoticeOfDeficiency),
            b"NA" => Some(MaterialReleaseOrder),
            b"NB" => Some(MaterialReleaseInquiry),
            b"NC" => Some(MaterialReleaseOrderForcedClosure),
            b"ND" => Some(MaterialReleaseCancellation),
            b"NE" => Some(DisposalReleaseOrder),
            b"NF" => Some(DisposalReleaseInquiry),
            b"NG" => Some(DisposalReleaseCancellation),
            b"NH" => Some(DisposalShipmentConfirmationInquiry),
            b"NI" => Some(RedistributionOrder),
            b"NJ" => Some(MaterialReleaseConfirmation),
            b"NK" => Some(MaterialReleaseDenial),
            b"NL" => Some(MaterialReleaseAdvice),
            b"NM" => Some(DisposalReleaseConfirmation),
            b"NO" => Some(Notice),
            b"NP" => Some(DisposalReleaseDenial),
            b"NQ" => Some(DisposalReleaseAdvice),
            b"NR" => Some(MaterialReleaseCancellationAdvice),
            b"NS" => Some(InTransit),
            b"NT" => Some(DisposalReleaseCancellationAdvice),
            b"NU" => Some(InventoryAdjustment),
            b"OA" => Some(OperationalCapacity),
            b"OC" => Some(OnApproval),
            b"OF" => Some(Offer),
            b"OP" => Some(Opinion),
            b"OR" => Some(Order),
            b"P1" => Some(Preliminary),
            b"P2" => Some(EmployerGroupChange),
            b"P3" => Some(IndividualChange),
            b"P4" => Some(EmployerOpenEnrollment),
            b"P5" => Some(PredeterminationMedical),
            b"P6" => Some(PredeterminationDental),
            b"PA" => Some(ProgressPaymentInvoice),
            b"PB" => Some(PartialBill),
            b"PC" => Some(InventionReport),
            b"PD" => Some(ProductAllocation),
            b"PE" => Some(Pleading),
            b"PF" => Some(PartialAssignmentFiling),
            b"PG" => Some(PremiumRoutingGuide),
            b"PH" => Some(ProspectiveStudent),
            b"PI" => Some(PersonalInjuryBill),
            b"PJ" => Some(ComponentPackingConfirmation),
            b"PL" => Some(PlanAllocation),
            b"PM" => Some(PremiumPayment),
            b"PO" => Some(PlanTakeover),
            b"PP" => Some(PrepaidInvoice),
            b"PQ" => Some(PartialEnrollmentFile),
            b"PR" => Some(CodePR),
            b"PS" => Some(PostsecondaryStudentAcademicRecord),
            b"PT" => Some(PlanToPlanTransfer),
            b"PU" => Some(NoticeOfClaim),
            b"PV" => Some(ProtestOrResponseToClaim),
            b"PW" => Some(ClaimDecision),
            b"PX" => Some(WageVerificationNotice),
            b"PZ" => Some(PurchaseReport),
            b"Q1" => Some(ScheduledQuantity),
            b"Q2" => Some(ScheduledQuantityForOperator),
            b"QA" => Some(CouponRegularClearinghouseInvoice),
            b"QB" => Some(CouponDirectRetailerInvoice),
            b"QC" => Some(CouponClearinghousePayDirectInvoice),
            b"QD" => Some(ProductQualityDeficiency),
            b"QE" => Some(CouponScanValidateInvoice),
            b"QF" => Some(ScanValidateAdjustment),
            b"QG" => Some(QuickResponseRoutingGuide),
            b"QH" => Some(CodeQH),
            b"QJ" => Some(AdjustedCouponRedemption),
            b"QK" => Some(CouponRedemptionDetail),
            b"QL" => Some(AdjustmentsToPreviousCouponRedemption),
            b"QP" => Some(CouponQuickPayInvoice),
            b"QR" => Some(ProductQualityDeficiencyResponse),
            b"R1" => Some(RequestForEnrollmentVerification),
            b"R2" => Some(ResponseToRequestForEnrollmentVerification),
            b"R3" => Some(ResponseToGarnishment),
            b"R4" => Some(ReleaseOfGarnishment),
            b"R5" => Some(ResponseToLevy),
            b"R6" => Some(ReleaseOfLevy),
            b"R7" => Some(ResponseToOrderToWithhold),
            b"R8" => Some(ReleaseOfOrderToWithhold),
            b"RA" => Some(RequestForCredit),
            b"RB" => Some(RightToUse),
            b"RC" => Some(RequestForQuote),
            b"RD" => Some(ReturnsDetail),
            b"RE" => Some(Rebill),
            b"RF" => Some(CodeRF),
            b"RG" => Some(RevisedFinalBill),
            b"RH" => Some(RequestForAdditionalFunds),
            b"RI" => Some(RoutingInstructions),
            b"RJ" => Some(ResponseToRequestForRoutingInstructions),
            b"RK" => Some(Registration),
            b"RM" => Some(ReminderToFile),
            b"RP" => Some(Reporting),
            b"RQ" => Some(Request),
            b"RS" => Some(CodeRS),
            b"RT" => Some(SpendDown),
            b"RU" => Some(MedicalServicesReservation),
            b"RZ" => Some(RemovedFromSolicitationMailingList),
            b"S1" => Some(SpecialRoutingGuide),
            b"S2" => Some(StandardRoutingGuide),
            b"S3" => Some(SupplementalLoanRepayment),
            b"S4" => Some(Submission),
            b"SA" => Some(StandAloneLeaseSchedule),
            b"SB" => Some(SecondNoticeOfBalanceDue),
            b"SC" => Some(Deprescription),
            b"SD" => Some(SupplyProcessDeficiency),
            b"SE" => Some(SpecialBilateral),
            b"SF" => Some(SingleFamilyProgram),
            b"SG" => Some(SampleGoodsInvoice),
            b"SH" => Some(ShipmentStatusNotification),
            b"SI" => Some(SightCertificationRequest),
            b"SL" => Some(SummaryLeaseSchedule),
            b"SM" => Some(CodeSM),
            b"SO" => Some(SpotRate),
            b"SP" => Some(SupplierRating),
            b"SQ" => Some(ScheduleQuery),
            b"SR" => Some(SupplyProcessDeficiencyResponse),
            b"SS" => Some(CodeSS),
            b"ST" => Some(StateRoyalty),
            b"SU" => Some(Survey),
            b"SV" => Some(SupplementalInvoice),
            b"T1" => Some(CodeT1),
            b"T2" => Some(CodeT2),
            b"T3" => Some(CodeT3),
            b"T4" => Some(CodeT4),
            b"T5" => Some(CodeT5),
            b"T6" => Some(CodeT6),
            b"T7" => Some(TransportationInvoice),
            b"T8" => Some(SalesInvoice),
            b"T9" => Some(ServiceRequesterLevelInvoice),
            b"TB" => Some(BuyerManagedTransportation),
            b"TD" => Some(ShipmentOrMovementDeficiency),
            b"TF" => Some(TaxOrFeeExemptionCertification),
            b"TG" => Some(ReceiptAcknowledgmentInquiry),
            b"TH" => Some(ReceiptAcknowledgmentAdvice),
            b"TI" => Some(DelinquentDueInAdvice),
            b"TJ" => Some(DelinquentDueInInquiry),
            b"TK" => Some(DueInReconciliationAdvice),
            b"TL" => Some(TotalLossEvaluation),
            b"TP" => Some(TradingPartner),
            b"TR" => Some(ShipmentOrMovementDeficiencyResponse),
            b"TS" => Some(TransferStatement),
            b"TT" => Some(TestingServiceReport),
            b"TX" => Some(RequestForTestingServiceReport),
            b"U1" => Some(ContractAbstract),
            b"U2" => Some(ShipmentPerformanceNotice),
            b"U4" => Some(AcceptanceAlert),
            b"U5" => Some(Update),
            b"U9" => Some(ContractPaymentNotice),
            b"UA" => Some(AmendmentFiling),
            b"UC" => Some(UniformCommercialCodeFiling),
            b"UD" => Some(UnsalableDetail),
            b"UF" => Some(CodeUF),
            b"UI" => Some(UniformCommercialCodeFilingInquiry),
            b"UM" => Some(TerminationFiling),
            b"UO" => Some(OriginalFiling),
            b"UP" => Some(UnsalableProductInvoice),
            b"UR" => Some(UniformCommercialCodeFilingResponseToInquiry),
            b"US" => Some(UnsubscribedCapacity),
            b"UT" => Some(ContinuationFiling),
            b"V1" => Some(ContractCompletionReport),
            b"V2" => Some(NominationQuickResponse),
            b"V3" => Some(ConfirmationResponseQuickResponse),
            b"V4" => Some(PreDeterminedAllocationQuickResponse),
            b"V5" => Some(RequestForConfirmationQuickResponse),
            b"VH" => Some(PublicVoucher),
            b"VJ" => Some(CommercialInvoice),
            b"VL" => Some(ViolationNotice),
            b"VM" => Some(VoluntaryMedwatchReport),
            b"VN" => Some(MandatoryMedwatchReport),
            b"VO" => Some(MedicalDeviceNewBaselineReport),
            b"VP" => Some(MedicalDeviceAnnualBaselineReport),
            b"VQ" => Some(UserFacilityAnnualMedicalDeviceReport),
            b"VR" => Some(AnnualCertificationOfMedicalDeviceReport),
            b"W1" => Some(WeaponsDataChange),
            b"W4" => Some(WeaponsControlReport),
            b"W5" => Some(WeaponsControlReportReconciliation),
            b"WA" => Some(WorkAssignment),
            b"WC" => Some(WorkersCompensationProofOfCoverageNotification),
            b"WD" => Some(Withdrawal),
            b"WH" => Some(OrderToWithhold),
            b"WO" => Some(WorkOrder),
            b"WS" => Some(Waste),
            b"WT" => Some(Warrant),
            b"X1" => Some(ConsolidatorsInvoice),
            b"XA" => Some(CancelPendingNewOffer),
            b"XB" => Some(BilateralSpotRate),
            b"XC" => Some(AutomaticConcurrence),
            b"XD" => Some(SpecialDeprescription),
            b"XX" => Some(FirmOrderConfirmationWithFacility),
            b"XY" => Some(FirmOrderConfirmation),
            b"XZ" => Some(FacilityConfirmation),
            b"YI" => Some(FundsValidationInquiry),
            b"YR" => Some(FundsValidationResponse),
            b"Z1" => Some(CodeZ1),
            b"Z2" => Some(ProjectDirective),
            b"Z3" => Some(RequestForContractualProcurement),
            b"Z4" => Some(ReimbursableWorkOrder),
            b"ZA" => Some(RequestInitiationOfWorkCandidate),
            b"ZB" => Some(ReportOfWorkCandidate),
            b"ZC" => Some(ReportOfAssignmentOrDeletionOfWorkCandidateToMaintenancePeriod),
            b"ZD" => Some(RequestAssignmentOfWorkCandidateToPlanningMaintenanceActivity),
            b"ZE" => Some(RequestForFullWorkCandidateDetail),
            b"ZF" => Some(ReportOfFullWorkCandidateDetail),
            b"ZG" => Some(ReportOfApprovedWorkCandidate),
            b"ZH" => Some(RequestWorkCandidateCostDurationEstimate),
            b"ZI" => Some(ReportOfWorkCandidateCostDurationEstimate),
            b"ZJ" => Some(RequestWorkCandidatePlanningServices),
            b"ZK" => Some(ReportOfWorkCandidatePlanningServices),
            b"ZL" => {
                Some(
                    ReportOfAssignmentOrDeletionOfWorkCandidateToPlanningMaintenanceActivity,
                )
            }
            b"ZM" => {
                Some(RequestOfAssignmentOrDeletionOfWorkCandidateToMaintenancePeriod)
            }
            b"ZN" => Some(StopWorkOrder),
            b"ZO" => Some(AuthorizationToContinueWork),
            b"ZP" => Some(RequestForDepartureFromSpecification),
            b"ZQ" => Some(ReportOfAuthorizedDepartureFromSpecification),
            b"ZR" => Some(RequestWorkProgressStatus),
            b"ZS" => Some(ReportOfWorkProgressStatus),
            b"ZT" => Some(ReportOfRejectionOrReturnOfWorkCandidate),
            b"ZU" => Some(RequestWorkCandidateChange),
            b"ZV" => Some(LienFiling),
            b"ZW" => Some(SortAndSegregateDetail),
            b"ZX" => Some(ExpungementOfPriorFiling),
            b"ZY" => Some(CancellationOfFiling),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use TransactionTypeCode::*;
        match self {
            LocationAddressMessage => "Location Address Message",
            UniqueItemTrackingControlReport => "Unique Item Tracking Control Report",
            UniqueItemTrackingReportReconciliation => {
                "Unique Item Tracking Report Reconciliation"
            }
            UniqueItemTrackingItemDataChange => "Unique Item Tracking Item Data Change",
            NewGroupInitialEnrollment => "New Group Initial Enrollment",
            LocationRelation => "Location Relation Information",
            ReportMessage => "Report Message",
            Supporting => "Supporting Information",
            ElectronicMailMessage => "Electronic Mail Message",
            RequestForCoOp => "Request for Co-op",
            Guidelines => "Guidelines",
            AccomplishmentBasedRenewal => "Accomplishment Based Renewal",
            CompetitiveRenewal => "Competitive Renewal",
            NonCompetitiveRenewal => "Non-competitive Renewal",
            Resubmission => "Resubmission",
            Supplemental => "Supplemental",
            Budget => "Budget",
            Commitment => "Commitment",
            CoOpActual => "Co-op Actual",
            Distribution => "Distribution",
            NationalPropertyRegistrySystemRealEstatePropertyTransaction => {
                "National Property Registry System Real Estate Property Transaction"
            }
            PhysiciansReport => "Physician's Report",
            MaintenanceRequest => "Maintenance Request",
            MaintenanceResponse => "Maintenance Response",
            Code15 => "Request with Immediate Response Required (No Follow-up)",
            Code16 => "Request with Immediate Response Required (Follow-up Required)",
            RequestWithImmediateResponseToMailbox => {
                "Request with Immediate Response to Mailbox"
            }
            ResponseNoFurtherUpdatesToFollow => "Response - No Further Updates to Follow",
            ResponseFurtherUpdatesToFollow => "Response - Further Updates to Follow",
            AirExportWaybillAndInvoice => "Air Export Waybill and Invoice",
            AirImportInvoice => "Air Import Invoice",
            OceanExportDirectInvoice => "Ocean Export Direct Invoice",
            OceanExportIndirectInvoice => "Ocean Export Indirect Invoice",
            OceanExportBrokerageInvoice => "Ocean Export Brokerage Invoice",
            OceanImportInvoice => "Ocean Import Invoice",
            MiscellaneousServicesInvoice => "Miscellaneous Services Invoice",
            WarehouseServicesInvoice => "Warehouse Services Invoice",
            PreRegistration => "Pre-registration",
            Delivery => "Delivery",
            SubrogationDemand => "Subrogation Demand",
            Normal => "Normal",
            EmergencyRequest => "Emergency Request",
            ShortNoticeRequest => "Short Notice Request",
            DamageNotification => "Damage Notification",
            DesignReport => "Design Report",
            Test => "Test",
            NoticeOfOccupationalDisease => "Notice of Occupational Disease",
            NoticeOfTraumaticInjury => "Notice of Traumatic Injury",
            StatementOfFinancialAffairs => "Statement of Financial Affairs",
            CustomerShipment => "Customer Shipment",
            InternationalShipment => "International Shipment",
            InterplantShipment => "Interplant Shipment",
            ProductionToCarrierShipment => "Production to Carrier Shipment",
            AdvancedPayment => "Advanced Payment",
            Delinquency => "Delinquency",
            Payment => "Payment Information",
            Audit => "Audit",
            TransferLoanIn => "Transfer Loan In",
            TransferLoanOut => "Transfer Loan Out",
            MailingAddressServices => "Mailing Address Services",
            Code54 => "Information Only, No Response Required",
            NewServiceOrder => "New Service Order",
            DeniedRequest => "Denied Request",
            ServiceTermination => "Service Termination",
            TaxBillingOrRemittance => "Tax Billing or Remittance",
            MaterialInTransit => "Material In Transit",
            PreawardNotification => "Preaward Notification",
            PostawardNotification => "Postaward Notification",
            SmallBusinessAwardNotification => "Small Business Award Notification",
            AwardNotification => "Award Notification",
            NotificationToLateBidders => "Notification to Late Bidders",
            NotificationOfProtestReceipt => "Notification of Protest Receipt",
            AbstractOfOffers => "Abstract of Offers",
            BidRejectionNotice => "Bid Rejection Notice",
            UnsolicitedBidNotice => "Unsolicited Bid Notice",
            FilingAndSearchRequest => "Filing and Search Request",
            Termination => "Termination",
            FilingOfficerConfirmation => "Filing Officer Confirmation",
            Amendment => "Amendment",
            FullAssignment => "Full Assignment",
            PartialAssignment => "Partial Assignment",
            FullReleaseOfCollateral => "Full Release of Collateral",
            PartialReleaseOfCollateral => "Partial Release of Collateral",
            Continuation => "Continuation",
            UniformCommercialCodeFilingWithBroadCollateralDescription => {
                "Uniform Commercial Code Filing with Broad Collateral Description"
            }
            UniformCommercialCodeFilingWithSpecificCollateralDescription => {
                "Uniform Commercial Code Filing with Specific Collateral Description"
            }
            SegregationByPurchaseOrderNumber => "Segregation by Purchase Order Number",
            SegregationByCarrierProNumber => "Segregation by Carrier PRO Number",
            ResponseNoActionTaken => "Response - No Action Taken",
            DisabilityBenefitsProofOfCoverageNotification => {
                "Disability Benefits Proof of Coverage Notification"
            }
            DebtorFiling => "Debtor Filing",
            FederalOccupationalReporting => "Federal Occupational Reporting",
            SecuredPartyFiling => "Secured Party Filing",
            PartialReleaseFiling => "Partial Release Filing",
            Code95 => "Full (Total) Release",
            MultipleListing => "Multiple Listing",
            Sale => "Sale",
            Loan => "Loan",
            Requisition => "Requisition",
            StorageItemDataChange => "Storage Item Data Change",
            AdministrativeFees => "Administrative Fees",
            AmendedReturn => "Amended Return",
            NoticeOfCharge => "Notice of Charge",
            ProtestOrResponseToCharge => "Protest or Response to Charge",
            ChargeDecision => "Charge Decision",
            DueToAnalysis => "Due to Analysis",
            AdvanceShipmentAndBillingNotice => "Advance Shipment and Billing Notice",
            RequisitionCancellation => "Requisition Cancellation",
            NoticeOfAvailability => "Notice of Availability",
            CodeAE => "Due to Reconciliation (Full)",
            RequisitionFollowUp => "Requisition Follow-Up",
            CodeAG => "Due to Reconciliation (Partial)",
            PreEmploymentScreening => "Pre-Employment Screening",
            AdjustedInvoice => "Adjusted Invoice",
            CodeAJ => "Student Loan Interest Statement (1098-E)",
            CodeAK => "Tuition Payments Statement (1098-T)",
            Arbitration => "Arbitration",
            RequisitionModification => "Requisition Modification",
            MaterialObligationInquiry => "Material Obligation Inquiry",
            MaterialObligationAdvice => "Material Obligation Advice",
            QuantityVerificationInquiry => "Quantity Verification Inquiry",
            SupplyAssistance => "Supply Assistance",
            ShipmentAdvice => "Shipment Advice",
            AdministrativeAction => "Administrative Action",
            QuantityVerificationAdvice => "Quantity Verification Advice",
            MaterialObligationReinstatement => "Material Obligation Reinstatement",
            DisposalShipmentConfirmation => "Disposal Shipment Confirmation",
            BackbillInvoice => "Backbill Invoice",
            Billback => "Billback",
            StatementOfBalanceDue => "Statement of Balance Due",
            BestAndFinalOffer => "Best and Final Offer",
            NonEscrowOrNonImpoundTracking => "Non-escrow or Non-impound Tracking",
            BillAndHoldGoods => "Bill and Hold Goods",
            BankruptcyPetition => "Bankruptcy Petition",
            BankruptcySchedules => "Bankruptcy Schedules",
            BlanketLeaseSchedule => "Blanket Lease Schedule",
            RequisitionPassingOrder => "Requisition Passing Order",
            RequisitionReferralOrder => "Requisition Referral Order",
            ProcurementCancellation => "Procurement Cancellation",
            ProcurementModification => "Procurement Modification",
            BusinessTaxpayerRegistration => "Business Taxpayer Registration",
            BillOfSaleSchedule => "Bill of Sale Schedule",
            BalanceAndTransactionReport => "Balance and Transaction Report",
            WorkersCompensation1StReportOfInjury => {
                "Workers' Compensation 1st Report of Injury"
            }
            WorkersCompensationSubsequentReport => {
                "Workers' Compensation Subsequent Report"
            }
            WorkersCompensationCombined1StAndSubsequentReport => {
                "Workers' Compensation Combined 1st and Subsequent Report"
            }
            NotForResaleInvoice => "Not for Resale Invoice",
            FreightInvoice => "Freight Invoice",
            CustomsEntryDetail => "Customs Entry Detail",
            Claim => "Claim Information",
            CaseOpening => "Case Opening",
            Consignment => "Consignment",
            EscrowOrImpoundServiceReporting => "Escrow or Impound Service Reporting",
            TaxAssessmentBill => "Tax Assessment Bill",
            FixedAssetsTaxReturn => "Fixed Assets Tax Return",
            ServiceCancellation => "Service Cancellation",
            ConformedCopy => "Conformed Copy",
            SubjectToAvailabilityOfFunds => "Subject to Availability of Funds",
            Cash => "Cash",
            Contribution => "Contribution",
            CreditCommissionInvoice => "Credit Commission Invoice",
            ConsolidatedDebitInvoice => "Consolidated Debit Invoice",
            ConsolidatedCreditInvoice => "Consolidated Credit Invoice",
            ConsolidatedDebitMemo => "Consolidated Debit Memo",
            ConsolidatedCreditMemo => "Consolidated Credit Memo",
            Chargeable => "Chargeable",
            ConsolidatedInvoice => "Consolidated Invoice",
            Confirmation => "Confirmation",
            ClaimSubmission => "Claim Submission",
            CustomerAllocation => "Customer Allocation",
            CallDetailMemo => "Call Detail Memo",
            CreditInvoice => "Credit Invoice",
            Corrected => "Corrected",
            CommissionPayment => "Commission Payment",
            CreditMemo => "Credit Memo",
            CashSurrenderDistribution => "Cash Surrender Distribution",
            CostTypeInvoice => "Cost Type Invoice",
            CargoOutturnReport => "Cargo Outturn Report",
            CostVoucher => "Cost Voucher",
            CashLetter => "Cash Letter",
            CheckList => "Check List",
            CitationToPayOrAppear => "Citation to Pay or Appear",
            ConvictionNotice => "Conviction Notice",
            DividendPayment => "Dividend Payment",
            Receipt => "Receipt",
            DueIn => "Due-In",
            DetourBilling => "Detour Billing",
            DebitCommissionInvoice => "Debit Commission Invoice",
            CodeDD => "Interdistrict (Pre-Kindergarten - Grade 12) Student Record",
            AdvanceReceipt => "Advance Receipt",
            DieselFuelBill => "Diesel Fuel Bill",
            Response => "Response",
            DiscretionaryAdditionalCompanyContribution => {
                "Discretionary Additional Company Contribution"
            }
            DebitInvoice => "Debit Invoice",
            DutyDrawback => "Duty Drawback",
            DepositList => "Deposit List",
            PriorDamageReport => "Prior Damage Report",
            DirectNonqualifiedRollover => "Direct Nonqualified Rollover",
            DropShipmentInvoice => "Drop Shipment Invoice",
            DistrictToPostsecondaryStudentRecord => {
                "District to Postsecondary Student Record"
            }
            DirectQualifiedRollover => "Direct Qualified Rollover",
            DebitMemo => "Debit Memo",
            Disposition => "Disposition",
            Detail => "Detail",
            Duplicate => "Duplicate",
            AdministrativeRecordsSubmission => "Administrative Records Submission",
            ExcessInterestAllocation => "Excess Interest Allocation",
            EngineeringFinalBill => "Engineering Final Bill",
            CampaignFiling => "Campaign Filing",
            LobbyistFiling => "Lobbyist Filing",
            CodeEF => "Engineering Installation, Right to Use, Final Bill",
            CodeEI => "Engineering Installation, Final Bill",
            EstimateOfRecord => "Estimate of Record",
            ExpensePayment => "Expense Payment",
            CodeER => "Engineering Right to Use, Final Bill",
            ExcessMaterialNotification => "Excess Material Notification",
            FinalReport => "Final Report",
            PreApprovedBiddersList => "Pre-approved Bidders List",
            PreDeterminedAllocation => "Pre-determined Allocation",
            Allocation => "Allocation",
            ShipperImbalance => "Shipper Imbalance",
            ProducerImbalance => "Producer Imbalance",
            StorageReport => "Storage Report",
            ForfeitureAllocation => "Forfeiture Allocation",
            FinalBill => "Final Bill",
            ForfeitureCredit => "Forfeiture Credit",
            CodeFD => "Consolidated Invoice, Final Bill",
            CodeFE => "Memorandum, Final Bill",
            FullAssignmentFiling => "Full Assignment Filing",
            Filing => "Filing",
            FirstCostInvoice => "First Cost Invoice",
            FinalNotice => "Final Notice",
            FundingModification => "Funding Modification",
            FirstNoticeOfLoss => "First Notice of Loss",
            FlatRatePerUnitBill => "Flat Rate Per Unit Bill",
            FullEnrollmentFile => "Full Enrollment File",
            FederalRoyalty => "Federal Royalty",
            FinancialStatementReport => "Financial Statement Report",
            MaterialReturns => "Material Returns",
            Nomination => "Nomination",
            RequestForConfirmation => "Request for Confirmation",
            ConfirmationResponse => "Confirmation Response",
            GovernmentFurnishedMaterialInquiryAdvice => {
                "Government Furnished Material Inquiry Advice"
            }
            GovernmentFurnishedMaterialInquiry => "Government Furnished Material Inquiry",
            Garnishment => "Garnishment",
            CreditReport => "Credit Report",
            DisabilityNotice => "Disability Notice",
            BlackLungClaim => "Black Lung Claim",
            ClaimExperienceReport => "Claim Experience Report",
            EmployersReport => "Employer's Report",
            LongshoreReport => "Longshore Report",
            UnitReport => "Unit Report",
            HazardousWasteReport => "Hazardous Waste Report",
            DischargeMonitoringReport => "Discharge Monitoring Report",
            RiskManagementPlan => "Risk Management Plan",
            SelfMonitoringReport => "Self Monitoring Report",
            HazardousAirPollutantInventoryReport => {
                "Hazardous Air Pollutant Inventory Report"
            }
            StationaryPointSourceInventoryReport => {
                "Stationary Point Source Inventory Report"
            }
            ToxicReleaseInventoryReport => "Toxic Release Inventory Report",
            HorsepowerEqualizationBill => "Horsepower Equalization Bill",
            HandlingCarrierAgreementUpdate => "Handling Carrier Agreement Update",
            InAdCouponNotification => "In-Ad Coupon Notification",
            Inventory => "Inventory",
            InstallationFinalBill => "Installation Final Bill",
            InsuranceCoverageNotification => "Insurance Coverage Notification",
            EmployersReportOfDisability => "Employers Report of Disability",
            IndianRoyalty => "Indian Royalty",
            CodeIF => "Material, Engineering, Installation, Final Bill",
            InterfundTransferIn => "Interfund Transfer In",
            IncidentNotice => "Incident Notice",
            Inquiry => "Inquiry",
            InterfundTransferOut => "Interfund Transfer Out",
            CodeIR => "Installation, Right to Use, Final Bill",
            CodeIU => "Material, Installation, Right to Use, Final Bill",
            WorkersCompensationReportOfInjuryOrIllness => {
                "Workers Compensation Report of Injury or Illness"
            }
            InterchangeUpdate => "Interchange Update",
            InvestmentFees => "Investment Fees",
            MaintenanceAndOperationsBill => "Maintenance and Operations Bill",
            JointFacilityMiscellaneousBillOrOther => {
                "Joint Facility Miscellaneous Bill or Other"
            }
            RentalBill => "Rental Bill",
            JunctionSettlementUpdate => "Junction Settlement Update",
            Judgment => "Judgment",
            JunctionUpdate => "Junction Update",
            TerminationForDefault => "Termination for Default",
            DefinitizationOfContract => "Definitization of Contract",
            DefinitizationOfOrder => "Definitization of Order",
            ExerciseOfOption => "Exercise of Option",
            IntentToExerciseOption => "Intent to Exercise Option",
            AdministrativeChange => "Administrative Change",
            ChangeOrder => "Change Order",
            SupplementalAgreement => "Supplemental Agreement",
            AmendedShippingInstructions => "Amended Shipping Instructions",
            ProvisionedItem => "Provisioned Item",
            WithdrawalOfOffer => "Withdrawal of Offer",
            AdditionsToGeneralProvisions => "Additions to General Provisions",
            RequestForPriceQuote => "Request for Price Quote",
            AdditionToSolicitationMailingList => "Addition to Solicitation Mailing List",
            TerminationForConvenience => "Termination for Convenience",
            DueInReconciliationInquiry => "Due-in Reconciliation Inquiry",
            LoanDistribution => "Loan Distribution",
            LoanRepaymentExpense => "Loan Repayment Expense",
            LandedCosts => "Landed Costs",
            LossNotification => "Loss Notification",
            LoanRepaymentToPrincipalOnly => "Loan Repayment to Principal Only",
            LoanRepayment => "Loan Repayment",
            LogisticsReassignment => "Logistics Reassignment",
            Levy => "Levy",
            ManufacturerCouponNotification => "Manufacturer Coupon Notification",
            MeasurementEventsAndAlarms => "Measurement Events and Alarms",
            MailingList => "Mailing List",
            MaintenanceToBusinessTaxpayerRegistration => {
                "Maintenance to Business Taxpayer Registration"
            }
            MaterialCreditInvoice => "Material Credit Invoice",
            MarketDevelopmentFund => "Market Development Fund",
            Memorandum => "Memorandum",
            CodeMF => "Material, Engineering, Final Bill",
            CodeMI => "Material, Installation, Final Bill",
            MembershipList => "Membership List",
            CodeMM => "Multiple Shippers, Multiple Consignees",
            CodeMP => "Mise En Place (In Place)",
            CodeMR => "Material, Right to Use, Final Bill",
            MaterialFinalBill => "Material Final Bill",
            MultifamilyProgram => "Multifamily Program",
            Bilateral => "Bilateral",
            NoticeOfAssessment => "Notice of Assessment",
            NoticeOfWarrant => "Notice of Warrant",
            NoticeOfAdjustment => "Notice of Adjustment",
            NoticeOfDetermination => "Notice of Determination",
            NoticeOfSettlement => "Notice of Settlement",
            NoticeOfRecordedLien => "Notice of Recorded Lien",
            NoticeOfDeficiency => "Notice of Deficiency",
            MaterialReleaseOrder => "Material Release Order",
            MaterialReleaseInquiry => "Material Release Inquiry",
            MaterialReleaseOrderForcedClosure => "Material Release Order Forced Closure",
            MaterialReleaseCancellation => "Material Release Cancellation",
            DisposalReleaseOrder => "Disposal Release Order",
            DisposalReleaseInquiry => "Disposal Release Inquiry",
            DisposalReleaseCancellation => "Disposal Release Cancellation",
            DisposalShipmentConfirmationInquiry => {
                "Disposal Shipment Confirmation Inquiry"
            }
            RedistributionOrder => "Redistribution Order",
            MaterialReleaseConfirmation => "Material Release Confirmation",
            MaterialReleaseDenial => "Material Release Denial",
            MaterialReleaseAdvice => "Material Release Advice",
            DisposalReleaseConfirmation => "Disposal Release Confirmation",
            Notice => "Notice",
            DisposalReleaseDenial => "Disposal Release Denial",
            DisposalReleaseAdvice => "Disposal Release Advice",
            MaterialReleaseCancellationAdvice => "Material Release Cancellation Advice",
            InTransit => "In-Transit",
            DisposalReleaseCancellationAdvice => "Disposal Release Cancellation Advice",
            InventoryAdjustment => "Inventory Adjustment",
            OperationalCapacity => "Operational Capacity",
            OnApproval => "On Approval",
            Offer => "Offer",
            Opinion => "Opinion",
            Order => "Order",
            Preliminary => "Preliminary",
            EmployerGroupChange => "Employer Group Change",
            IndividualChange => "Individual Change",
            EmployerOpenEnrollment => "Employer Open Enrollment",
            PredeterminationMedical => "Predetermination - Medical",
            PredeterminationDental => "Predetermination - Dental",
            ProgressPaymentInvoice => "Progress Payment Invoice",
            PartialBill => "Partial Bill",
            InventionReport => "Invention Report",
            ProductAllocation => "Product Allocation",
            Pleading => "Pleading",
            PartialAssignmentFiling => "Partial Assignment Filing",
            PremiumRoutingGuide => "Premium Routing Guide",
            ProspectiveStudent => "Prospective Student Information",
            PersonalInjuryBill => "Personal Injury Bill",
            ComponentPackingConfirmation => "Component Packing Confirmation",
            PlanAllocation => "Plan Allocation",
            PremiumPayment => "Premium Payment",
            PlanTakeover => "Plan Takeover",
            PrepaidInvoice => "Prepaid Invoice",
            PartialEnrollmentFile => "Partial Enrollment File",
            CodePR => "Product (or Service)",
            PostsecondaryStudentAcademicRecord => "Postsecondary Student Academic Record",
            PlanToPlanTransfer => "Plan-to-plan Transfer",
            NoticeOfClaim => "Notice of Claim",
            ProtestOrResponseToClaim => "Protest or Response to Claim",
            ClaimDecision => "Claim Decision",
            WageVerificationNotice => "Wage Verification Notice",
            PurchaseReport => "Purchase Report",
            ScheduledQuantity => "Scheduled Quantity",
            ScheduledQuantityForOperator => "Scheduled Quantity for Operator",
            CouponRegularClearinghouseInvoice => "Coupon Regular Clearinghouse Invoice",
            CouponDirectRetailerInvoice => "Coupon Direct Retailer Invoice",
            CouponClearinghousePayDirectInvoice => {
                "Coupon Clearinghouse Pay Direct Invoice"
            }
            ProductQualityDeficiency => "Product Quality Deficiency",
            CouponScanValidateInvoice => "Coupon Scan Validate Invoice",
            ScanValidateAdjustment => "Scan Validate Adjustment",
            QuickResponseRoutingGuide => "Quick Response Routing Guide",
            CodeQH => "Full Coupon Redemption (No Adjustments)",
            AdjustedCouponRedemption => "Adjusted Coupon Redemption",
            CouponRedemptionDetail => "Coupon Redemption Detail",
            AdjustmentsToPreviousCouponRedemption => {
                "Adjustments to Previous Coupon Redemption"
            }
            CouponQuickPayInvoice => "Coupon Quick Pay Invoice",
            ProductQualityDeficiencyResponse => "Product Quality Deficiency Response",
            RequestForEnrollmentVerification => "Request for Enrollment Verification",
            ResponseToRequestForEnrollmentVerification => {
                "Response to Request for Enrollment Verification"
            }
            ResponseToGarnishment => "Response to Garnishment",
            ReleaseOfGarnishment => "Release of Garnishment",
            ResponseToLevy => "Response to Levy",
            ReleaseOfLevy => "Release of Levy",
            ResponseToOrderToWithhold => "Response to Order to Withhold",
            ReleaseOfOrderToWithhold => "Release of Order to Withhold",
            RequestForCredit => "Request for Credit",
            RightToUse => "Right to Use",
            RequestForQuote => "Request for Quote",
            ReturnsDetail => "Returns Detail",
            Rebill => "Rebill",
            CodeRF => "Material, Engineering, Right to Use, Final Bill",
            RevisedFinalBill => "Revised Final Bill",
            RequestForAdditionalFunds => "Request for Additional Funds",
            RoutingInstructions => "Routing Instructions",
            ResponseToRequestForRoutingInstructions => {
                "Response to Request for Routing Instructions"
            }
            Registration => "Registration",
            ReminderToFile => "Reminder to File",
            Reporting => "Reporting",
            Request => "Request",
            CodeRS => "Response - Additional Response(s) Available",
            SpendDown => "Spend Down",
            MedicalServicesReservation => "Medical Services Reservation",
            RemovedFromSolicitationMailingList => {
                "Removed from Solicitation Mailing List"
            }
            SpecialRoutingGuide => "Special Routing Guide",
            StandardRoutingGuide => "Standard Routing Guide",
            SupplementalLoanRepayment => "Supplemental Loan Repayment",
            Submission => "Submission",
            StandAloneLeaseSchedule => "Stand-alone Lease Schedule",
            SecondNoticeOfBalanceDue => "Second Notice of Balance Due",
            Deprescription => "Deprescription",
            SupplyProcessDeficiency => "Supply Process Deficiency",
            SpecialBilateral => "Special Bilateral",
            SingleFamilyProgram => "Single Family Program",
            SampleGoodsInvoice => "Sample Goods Invoice",
            ShipmentStatusNotification => "Shipment Status Notification",
            SightCertificationRequest => "Sight Certification Request",
            SummaryLeaseSchedule => "Summary Lease Schedule",
            CodeSM => "Single Shipper, Multiple Consignees",
            SpotRate => "Spot Rate",
            SupplierRating => "Supplier Rating",
            ScheduleQuery => "Schedule Query",
            SupplyProcessDeficiencyResponse => "Supply Process Deficiency Response",
            CodeSS => "Single Shipper, Single Consignee",
            StateRoyalty => "State Royalty",
            Survey => "Survey",
            SupplementalInvoice => "Supplemental Invoice",
            CodeT1 => "Report sent by National Center for Education Statistics (NCES)",
            CodeT2 => "Report sent to National Center for Education Statistics (NCES)",
            CodeT3 => {
                "Common Core of Data (CCD) Report from the National Center for Education Statistics (NCES)"
            }
            CodeT4 => {
                "Common Core of Data (CCD) Report to the National Center for Education Statistics (NCES)"
            }
            CodeT5 => {
                "Integrated Postsecondary Education Database System (IPEDS) Report from National Center for Education Statistics (NCES)"
            }
            CodeT6 => {
                "Integrated Postsecondary Education Database System (IPEDS) Report to National Center for Education Statistics (NCES)"
            }
            TransportationInvoice => "Transportation Invoice",
            SalesInvoice => "Sales Invoice",
            ServiceRequesterLevelInvoice => "Service Requester Level Invoice",
            BuyerManagedTransportation => "Buyer Managed Transportation",
            ShipmentOrMovementDeficiency => "Shipment or Movement Deficiency",
            TaxOrFeeExemptionCertification => "Tax or Fee Exemption Certification",
            ReceiptAcknowledgmentInquiry => "Receipt Acknowledgment Inquiry",
            ReceiptAcknowledgmentAdvice => "Receipt Acknowledgment Advice",
            DelinquentDueInAdvice => "Delinquent Due-in Advice",
            DelinquentDueInInquiry => "Delinquent Due-in Inquiry",
            DueInReconciliationAdvice => "Due-in Reconciliation Advice",
            TotalLossEvaluation => "Total Loss Evaluation",
            TradingPartner => "Trading Partner Information",
            ShipmentOrMovementDeficiencyResponse => {
                "Shipment or Movement Deficiency Response"
            }
            TransferStatement => "Transfer Statement",
            TestingServiceReport => "Testing Service Report",
            RequestForTestingServiceReport => "Request for Testing Service Report",
            ContractAbstract => "Contract Abstract",
            ShipmentPerformanceNotice => "Shipment Performance Notice",
            AcceptanceAlert => "Acceptance Alert",
            Update => "Update",
            ContractPaymentNotice => "Contract Payment Notice",
            AmendmentFiling => "Amendment Filing",
            UniformCommercialCodeFiling => "Uniform Commercial Code Filing",
            UnsalableDetail => "Unsalable Detail",
            CodeUF => "Material, Engineering, Installation, Right to Use, Final Bill",
            UniformCommercialCodeFilingInquiry => {
                "Uniform Commercial Code Filing Inquiry"
            }
            TerminationFiling => "Termination Filing",
            OriginalFiling => "Original Filing",
            UnsalableProductInvoice => "Unsalable Product Invoice",
            UniformCommercialCodeFilingResponseToInquiry => {
                "Uniform Commercial Code Filing Response to Inquiry"
            }
            UnsubscribedCapacity => "Unsubscribed Capacity",
            ContinuationFiling => "Continuation Filing",
            ContractCompletionReport => "Contract Completion Report",
            NominationQuickResponse => "Nomination Quick Response",
            ConfirmationResponseQuickResponse => "Confirmation Response Quick Response",
            PreDeterminedAllocationQuickResponse => {
                "Pre-determined Allocation Quick Response"
            }
            RequestForConfirmationQuickResponse => {
                "Request for Confirmation Quick Response"
            }
            PublicVoucher => "Public Voucher",
            CommercialInvoice => "Commercial Invoice",
            ViolationNotice => "Violation Notice",
            VoluntaryMedwatchReport => "Voluntary MEDWATCH Report",
            MandatoryMedwatchReport => "Mandatory MEDWATCH Report",
            MedicalDeviceNewBaselineReport => "Medical Device New Baseline Report",
            MedicalDeviceAnnualBaselineReport => "Medical Device Annual Baseline Report",
            UserFacilityAnnualMedicalDeviceReport => {
                "User Facility Annual Medical Device Report"
            }
            AnnualCertificationOfMedicalDeviceReport => {
                "Annual Certification of Medical Device Report"
            }
            WeaponsDataChange => "Weapons Data Change",
            WeaponsControlReport => "Weapons Control Report",
            WeaponsControlReportReconciliation => "Weapons Control Report Reconciliation",
            WorkAssignment => "Work Assignment",
            WorkersCompensationProofOfCoverageNotification => {
                "Workers Compensation Proof of Coverage Notification"
            }
            Withdrawal => "Withdrawal",
            OrderToWithhold => "Order to Withhold",
            WorkOrder => "Work Order",
            Waste => "Waste",
            Warrant => "Warrant",
            ConsolidatorsInvoice => "Consolidator's Invoice",
            CancelPendingNewOffer => "Cancel Pending New Offer",
            BilateralSpotRate => "Bilateral Spot Rate",
            AutomaticConcurrence => "Automatic Concurrence",
            SpecialDeprescription => "Special Deprescription",
            FirmOrderConfirmationWithFacility => {
                "Firm Order Confirmation with Facility Information"
            }
            FirmOrderConfirmation => "Firm Order Confirmation",
            FacilityConfirmation => "Facility Confirmation",
            FundsValidationInquiry => "Funds Validation Inquiry",
            FundsValidationResponse => "Funds Validation Response",
            CodeZ1 => "Military Interdepartmental Purchase Request (MIPR)",
            ProjectDirective => "Project Directive",
            RequestForContractualProcurement => "Request for Contractual Procurement",
            ReimbursableWorkOrder => "Reimbursable Work Order",
            RequestInitiationOfWorkCandidate => "Request Initiation of Work Candidate",
            ReportOfWorkCandidate => "Report of Work Candidate",
            ReportOfAssignmentOrDeletionOfWorkCandidateToMaintenancePeriod => {
                "Report of Assignment or Deletion of Work Candidate to Maintenance Period"
            }
            RequestAssignmentOfWorkCandidateToPlanningMaintenanceActivity => {
                "Request Assignment of Work Candidate to Planning Maintenance Activity"
            }
            RequestForFullWorkCandidateDetail => "Request for Full Work Candidate Detail",
            ReportOfFullWorkCandidateDetail => "Report of Full Work Candidate Detail",
            ReportOfApprovedWorkCandidate => "Report of Approved Work Candidate",
            RequestWorkCandidateCostDurationEstimate => {
                "Request Work Candidate Cost/Duration Estimate"
            }
            ReportOfWorkCandidateCostDurationEstimate => {
                "Report of Work Candidate Cost/Duration Estimate"
            }
            RequestWorkCandidatePlanningServices => {
                "Request Work Candidate Planning Services"
            }
            ReportOfWorkCandidatePlanningServices => {
                "Report of Work Candidate Planning Services"
            }
            ReportOfAssignmentOrDeletionOfWorkCandidateToPlanningMaintenanceActivity => {
                "Report of Assignment or Deletion of Work Candidate to Planning/Maintenance Activity"
            }
            RequestOfAssignmentOrDeletionOfWorkCandidateToMaintenancePeriod => {
                "Request of Assignment or Deletion of Work Candidate to Maintenance Period"
            }
            StopWorkOrder => "Stop Work Order",
            AuthorizationToContinueWork => "Authorization to Continue Work",
            RequestForDepartureFromSpecification => {
                "Request for Departure From Specification"
            }
            ReportOfAuthorizedDepartureFromSpecification => {
                "Report of Authorized Departure From Specification"
            }
            RequestWorkProgressStatus => "Request Work Progress Status",
            ReportOfWorkProgressStatus => "Report of Work Progress Status",
            ReportOfRejectionOrReturnOfWorkCandidate => {
                "Report of Rejection or Return of Work Candidate"
            }
            RequestWorkCandidateChange => "Request Work Candidate Change",
            LienFiling => "Lien Filing",
            SortAndSegregateDetail => "Sort and Segregate Detail",
            ExpungementOfPriorFiling => "Expungement of Prior Filing",
            CancellationOfFiling => "Cancellation of Filing",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<TransactionTypeCode> {
        {
            use TransactionTypeCode::*;
            match description {
                "Location Address Message" => Some(LocationAddressMessage),
                "Unique Item Tracking Control Report" => {
                    Some(UniqueItemTrackingControlReport)
                }
                "Unique Item Tracking Report Reconciliation" => {
                    Some(UniqueItemTrackingReportReconciliation)
                }
                "Unique Item Tracking Item Data Change" => {
                    Some(UniqueItemTrackingItemDataChange)
                }
                "New Group Initial Enrollment" => Some(NewGroupInitialEnrollment),
                "Location Relation Information" => Some(LocationRelation),
                "Report Message" => Some(ReportMessage),
                "Supporting Information" => Some(Supporting),
                "Electronic Mail Message" => Some(ElectronicMailMessage),
                "Request for Co-op" => Some(RequestForCoOp),
                "Guidelines" => Some(Guidelines),
                "Accomplishment Based Renewal" => Some(AccomplishmentBasedRenewal),
                "Competitive Renewal" => Some(CompetitiveRenewal),
                "Non-competitive Renewal" => Some(NonCompetitiveRenewal),
                "Resubmission" => Some(Resubmission),
                "Supplemental" => Some(Supplemental),
                "Budget" => Some(Budget),
                "Commitment" => Some(Commitment),
                "Co-op Actual" => Some(CoOpActual),
                "Distribution" => Some(Distribution),
                "National Property Registry System Real Estate Property Transaction" => {
                    Some(NationalPropertyRegistrySystemRealEstatePropertyTransaction)
                }
                "Physician's Report" => Some(PhysiciansReport),
                "Maintenance Request" => Some(MaintenanceRequest),
                "Maintenance Response" => Some(MaintenanceResponse),
                "Request with Immediate Response Required (No Follow-up)" => Some(Code15),
                "Request with Immediate Response Required (Follow-up Required)" => {
                    Some(Code16)
                }
                "Request with Immediate Response to Mailbox" => {
                    Some(RequestWithImmediateResponseToMailbox)
                }
                "Response - No Further Updates to Follow" => {
                    Some(ResponseNoFurtherUpdatesToFollow)
                }
                "Response - Further Updates to Follow" => {
                    Some(ResponseFurtherUpdatesToFollow)
                }
                "Air Export Waybill and Invoice" => Some(AirExportWaybillAndInvoice),
                "Air Import Invoice" => Some(AirImportInvoice),
                "Ocean Export Direct Invoice" => Some(OceanExportDirectInvoice),
                "Ocean Export Indirect Invoice" => Some(OceanExportIndirectInvoice),
                "Ocean Export Brokerage Invoice" => Some(OceanExportBrokerageInvoice),
                "Ocean Import Invoice" => Some(OceanImportInvoice),
                "Miscellaneous Services Invoice" => Some(MiscellaneousServicesInvoice),
                "Warehouse Services Invoice" => Some(WarehouseServicesInvoice),
                "Pre-registration" => Some(PreRegistration),
                "Delivery" => Some(Delivery),
                "Subrogation Demand" => Some(SubrogationDemand),
                "Normal" => Some(Normal),
                "Emergency Request" => Some(EmergencyRequest),
                "Short Notice Request" => Some(ShortNoticeRequest),
                "Damage Notification" => Some(DamageNotification),
                "Design Report" => Some(DesignReport),
                "Test" => Some(Test),
                "Notice of Occupational Disease" => Some(NoticeOfOccupationalDisease),
                "Notice of Traumatic Injury" => Some(NoticeOfTraumaticInjury),
                "Statement of Financial Affairs" => Some(StatementOfFinancialAffairs),
                "Customer Shipment" => Some(CustomerShipment),
                "International Shipment" => Some(InternationalShipment),
                "Interplant Shipment" => Some(InterplantShipment),
                "Production to Carrier Shipment" => Some(ProductionToCarrierShipment),
                "Advanced Payment" => Some(AdvancedPayment),
                "Delinquency" => Some(Delinquency),
                "Payment Information" => Some(Payment),
                "Audit" => Some(Audit),
                "Transfer Loan In" => Some(TransferLoanIn),
                "Transfer Loan Out" => Some(TransferLoanOut),
                "Mailing Address Services" => Some(MailingAddressServices),
                "Information Only, No Response Required" => Some(Code54),
                "New Service Order" => Some(NewServiceOrder),
                "Denied Request" => Some(DeniedRequest),
                "Service Termination" => Some(ServiceTermination),
                "Tax Billing or Remittance" => Some(TaxBillingOrRemittance),
                "Material In Transit" => Some(MaterialInTransit),
                "Preaward Notification" => Some(PreawardNotification),
                "Postaward Notification" => Some(PostawardNotification),
                "Small Business Award Notification" => {
                    Some(SmallBusinessAwardNotification)
                }
                "Award Notification" => Some(AwardNotification),
                "Notification to Late Bidders" => Some(NotificationToLateBidders),
                "Notification of Protest Receipt" => Some(NotificationOfProtestReceipt),
                "Abstract of Offers" => Some(AbstractOfOffers),
                "Bid Rejection Notice" => Some(BidRejectionNotice),
                "Unsolicited Bid Notice" => Some(UnsolicitedBidNotice),
                "Filing and Search Request" => Some(FilingAndSearchRequest),
                "Termination" => Some(Termination),
                "Filing Officer Confirmation" => Some(FilingOfficerConfirmation),
                "Amendment" => Some(Amendment),
                "Full Assignment" => Some(FullAssignment),
                "Partial Assignment" => Some(PartialAssignment),
                "Full Release of Collateral" => Some(FullReleaseOfCollateral),
                "Partial Release of Collateral" => Some(PartialReleaseOfCollateral),
                "Continuation" => Some(Continuation),
                "Uniform Commercial Code Filing with Broad Collateral Description" => {
                    Some(UniformCommercialCodeFilingWithBroadCollateralDescription)
                }
                "Uniform Commercial Code Filing with Specific Collateral Description" => {
                    Some(UniformCommercialCodeFilingWithSpecificCollateralDescription)
                }
                "Segregation by Purchase Order Number" => {
                    Some(SegregationByPurchaseOrderNumber)
                }
                "Segregation by Carrier PRO Number" => {
                    Some(SegregationByCarrierProNumber)
                }
                "Response - No Action Taken" => Some(ResponseNoActionTaken),
                "Disability Benefits Proof of Coverage Notification" => {
                    Some(DisabilityBenefitsProofOfCoverageNotification)
                }
                "Debtor Filing" => Some(DebtorFiling),
                "Federal Occupational Reporting" => Some(FederalOccupationalReporting),
                "Secured Party Filing" => Some(SecuredPartyFiling),
                "Partial Release Filing" => Some(PartialReleaseFiling),
                "Full (Total) Release" => Some(Code95),
                "Multiple Listing" => Some(MultipleListing),
                "Sale" => Some(Sale),
                "Loan" => Some(Loan),
                "Requisition" => Some(Requisition),
                "Storage Item Data Change" => Some(StorageItemDataChange),
                "Administrative Fees" => Some(AdministrativeFees),
                "Amended Return" => Some(AmendedReturn),
                "Notice of Charge" => Some(NoticeOfCharge),
                "Protest or Response to Charge" => Some(ProtestOrResponseToCharge),
                "Charge Decision" => Some(ChargeDecision),
                "Due to Analysis" => Some(DueToAnalysis),
                "Advance Shipment and Billing Notice" => {
                    Some(AdvanceShipmentAndBillingNotice)
                }
                "Requisition Cancellation" => Some(RequisitionCancellation),
                "Notice of Availability" => Some(NoticeOfAvailability),
                "Due to Reconciliation (Full)" => Some(CodeAE),
                "Requisition Follow-Up" => Some(RequisitionFollowUp),
                "Due to Reconciliation (Partial)" => Some(CodeAG),
                "Pre-Employment Screening" => Some(PreEmploymentScreening),
                "Adjusted Invoice" => Some(AdjustedInvoice),
                "Student Loan Interest Statement (1098-E)" => Some(CodeAJ),
                "Tuition Payments Statement (1098-T)" => Some(CodeAK),
                "Arbitration" => Some(Arbitration),
                "Requisition Modification" => Some(RequisitionModification),
                "Material Obligation Inquiry" => Some(MaterialObligationInquiry),
                "Material Obligation Advice" => Some(MaterialObligationAdvice),
                "Quantity Verification Inquiry" => Some(QuantityVerificationInquiry),
                "Supply Assistance" => Some(SupplyAssistance),
                "Shipment Advice" => Some(ShipmentAdvice),
                "Administrative Action" => Some(AdministrativeAction),
                "Quantity Verification Advice" => Some(QuantityVerificationAdvice),
                "Material Obligation Reinstatement" => {
                    Some(MaterialObligationReinstatement)
                }
                "Disposal Shipment Confirmation" => Some(DisposalShipmentConfirmation),
                "Backbill Invoice" => Some(BackbillInvoice),
                "Billback" => Some(Billback),
                "Statement of Balance Due" => Some(StatementOfBalanceDue),
                "Best and Final Offer" => Some(BestAndFinalOffer),
                "Non-escrow or Non-impound Tracking" => {
                    Some(NonEscrowOrNonImpoundTracking)
                }
                "Bill and Hold Goods" => Some(BillAndHoldGoods),
                "Bankruptcy Petition" => Some(BankruptcyPetition),
                "Bankruptcy Schedules" => Some(BankruptcySchedules),
                "Blanket Lease Schedule" => Some(BlanketLeaseSchedule),
                "Requisition Passing Order" => Some(RequisitionPassingOrder),
                "Requisition Referral Order" => Some(RequisitionReferralOrder),
                "Procurement Cancellation" => Some(ProcurementCancellation),
                "Procurement Modification" => Some(ProcurementModification),
                "Business Taxpayer Registration" => Some(BusinessTaxpayerRegistration),
                "Bill of Sale Schedule" => Some(BillOfSaleSchedule),
                "Balance and Transaction Report" => Some(BalanceAndTransactionReport),
                "Workers' Compensation 1st Report of Injury" => {
                    Some(WorkersCompensation1StReportOfInjury)
                }
                "Workers' Compensation Subsequent Report" => {
                    Some(WorkersCompensationSubsequentReport)
                }
                "Workers' Compensation Combined 1st and Subsequent Report" => {
                    Some(WorkersCompensationCombined1StAndSubsequentReport)
                }
                "Not for Resale Invoice" => Some(NotForResaleInvoice),
                "Freight Invoice" => Some(FreightInvoice),
                "Customs Entry Detail" => Some(CustomsEntryDetail),
                "Claim Information" => Some(Claim),
                "Case Opening" => Some(CaseOpening),
                "Consignment" => Some(Consignment),
                "Escrow or Impound Service Reporting" => {
                    Some(EscrowOrImpoundServiceReporting)
                }
                "Tax Assessment Bill" => Some(TaxAssessmentBill),
                "Fixed Assets Tax Return" => Some(FixedAssetsTaxReturn),
                "Service Cancellation" => Some(ServiceCancellation),
                "Conformed Copy" => Some(ConformedCopy),
                "Subject to Availability of Funds" => Some(SubjectToAvailabilityOfFunds),
                "Cash" => Some(Cash),
                "Contribution" => Some(Contribution),
                "Credit Commission Invoice" => Some(CreditCommissionInvoice),
                "Consolidated Debit Invoice" => Some(ConsolidatedDebitInvoice),
                "Consolidated Credit Invoice" => Some(ConsolidatedCreditInvoice),
                "Consolidated Debit Memo" => Some(ConsolidatedDebitMemo),
                "Consolidated Credit Memo" => Some(ConsolidatedCreditMemo),
                "Chargeable" => Some(Chargeable),
                "Consolidated Invoice" => Some(ConsolidatedInvoice),
                "Confirmation" => Some(Confirmation),
                "Claim Submission" => Some(ClaimSubmission),
                "Customer Allocation" => Some(CustomerAllocation),
                "Call Detail Memo" => Some(CallDetailMemo),
                "Credit Invoice" => Some(CreditInvoice),
                "Corrected" => Some(Corrected),
                "Commission Payment" => Some(CommissionPayment),
                "Credit Memo" => Some(CreditMemo),
                "Cash Surrender Distribution" => Some(CashSurrenderDistribution),
                "Cost Type Invoice" => Some(CostTypeInvoice),
                "Cargo Outturn Report" => Some(CargoOutturnReport),
                "Cost Voucher" => Some(CostVoucher),
                "Cash Letter" => Some(CashLetter),
                "Check List" => Some(CheckList),
                "Citation to Pay or Appear" => Some(CitationToPayOrAppear),
                "Conviction Notice" => Some(ConvictionNotice),
                "Dividend Payment" => Some(DividendPayment),
                "Receipt" => Some(Receipt),
                "Due-In" => Some(DueIn),
                "Detour Billing" => Some(DetourBilling),
                "Debit Commission Invoice" => Some(DebitCommissionInvoice),
                "Interdistrict (Pre-Kindergarten - Grade 12) Student Record" => {
                    Some(CodeDD)
                }
                "Advance Receipt" => Some(AdvanceReceipt),
                "Diesel Fuel Bill" => Some(DieselFuelBill),
                "Response" => Some(Response),
                "Discretionary Additional Company Contribution" => {
                    Some(DiscretionaryAdditionalCompanyContribution)
                }
                "Debit Invoice" => Some(DebitInvoice),
                "Duty Drawback" => Some(DutyDrawback),
                "Deposit List" => Some(DepositList),
                "Prior Damage Report" => Some(PriorDamageReport),
                "Direct Nonqualified Rollover" => Some(DirectNonqualifiedRollover),
                "Drop Shipment Invoice" => Some(DropShipmentInvoice),
                "District to Postsecondary Student Record" => {
                    Some(DistrictToPostsecondaryStudentRecord)
                }
                "Direct Qualified Rollover" => Some(DirectQualifiedRollover),
                "Debit Memo" => Some(DebitMemo),
                "Disposition" => Some(Disposition),
                "Detail" => Some(Detail),
                "Duplicate" => Some(Duplicate),
                "Administrative Records Submission" => {
                    Some(AdministrativeRecordsSubmission)
                }
                "Excess Interest Allocation" => Some(ExcessInterestAllocation),
                "Engineering Final Bill" => Some(EngineeringFinalBill),
                "Campaign Filing" => Some(CampaignFiling),
                "Lobbyist Filing" => Some(LobbyistFiling),
                "Engineering Installation, Right to Use, Final Bill" => Some(CodeEF),
                "Engineering Installation, Final Bill" => Some(CodeEI),
                "Estimate of Record" => Some(EstimateOfRecord),
                "Expense Payment" => Some(ExpensePayment),
                "Engineering Right to Use, Final Bill" => Some(CodeER),
                "Excess Material Notification" => Some(ExcessMaterialNotification),
                "Final Report" => Some(FinalReport),
                "Pre-approved Bidders List" => Some(PreApprovedBiddersList),
                "Pre-determined Allocation" => Some(PreDeterminedAllocation),
                "Allocation" => Some(Allocation),
                "Shipper Imbalance" => Some(ShipperImbalance),
                "Producer Imbalance" => Some(ProducerImbalance),
                "Storage Report" => Some(StorageReport),
                "Forfeiture Allocation" => Some(ForfeitureAllocation),
                "Final Bill" => Some(FinalBill),
                "Forfeiture Credit" => Some(ForfeitureCredit),
                "Consolidated Invoice, Final Bill" => Some(CodeFD),
                "Memorandum, Final Bill" => Some(CodeFE),
                "Full Assignment Filing" => Some(FullAssignmentFiling),
                "Filing" => Some(Filing),
                "First Cost Invoice" => Some(FirstCostInvoice),
                "Final Notice" => Some(FinalNotice),
                "Funding Modification" => Some(FundingModification),
                "First Notice of Loss" => Some(FirstNoticeOfLoss),
                "Flat Rate Per Unit Bill" => Some(FlatRatePerUnitBill),
                "Full Enrollment File" => Some(FullEnrollmentFile),
                "Federal Royalty" => Some(FederalRoyalty),
                "Financial Statement Report" => Some(FinancialStatementReport),
                "Material Returns" => Some(MaterialReturns),
                "Nomination" => Some(Nomination),
                "Request for Confirmation" => Some(RequestForConfirmation),
                "Confirmation Response" => Some(ConfirmationResponse),
                "Government Furnished Material Inquiry Advice" => {
                    Some(GovernmentFurnishedMaterialInquiryAdvice)
                }
                "Government Furnished Material Inquiry" => {
                    Some(GovernmentFurnishedMaterialInquiry)
                }
                "Garnishment" => Some(Garnishment),
                "Credit Report" => Some(CreditReport),
                "Disability Notice" => Some(DisabilityNotice),
                "Black Lung Claim" => Some(BlackLungClaim),
                "Claim Experience Report" => Some(ClaimExperienceReport),
                "Employer's Report" => Some(EmployersReport),
                "Longshore Report" => Some(LongshoreReport),
                "Unit Report" => Some(UnitReport),
                "Hazardous Waste Report" => Some(HazardousWasteReport),
                "Discharge Monitoring Report" => Some(DischargeMonitoringReport),
                "Risk Management Plan" => Some(RiskManagementPlan),
                "Self Monitoring Report" => Some(SelfMonitoringReport),
                "Hazardous Air Pollutant Inventory Report" => {
                    Some(HazardousAirPollutantInventoryReport)
                }
                "Stationary Point Source Inventory Report" => {
                    Some(StationaryPointSourceInventoryReport)
                }
                "Toxic Release Inventory Report" => Some(ToxicReleaseInventoryReport),
                "Horsepower Equalization Bill" => Some(HorsepowerEqualizationBill),
                "Handling Carrier Agreement Update" => {
                    Some(HandlingCarrierAgreementUpdate)
                }
                "In-Ad Coupon Notification" => Some(InAdCouponNotification),
                "Inventory" => Some(Inventory),
                "Installation Final Bill" => Some(InstallationFinalBill),
                "Insurance Coverage Notification" => Some(InsuranceCoverageNotification),
                "Employers Report of Disability" => Some(EmployersReportOfDisability),
                "Indian Royalty" => Some(IndianRoyalty),
                "Material, Engineering, Installation, Final Bill" => Some(CodeIF),
                "Interfund Transfer In" => Some(InterfundTransferIn),
                "Incident Notice" => Some(IncidentNotice),
                "Inquiry" => Some(Inquiry),
                "Interfund Transfer Out" => Some(InterfundTransferOut),
                "Installation, Right to Use, Final Bill" => Some(CodeIR),
                "Material, Installation, Right to Use, Final Bill" => Some(CodeIU),
                "Workers Compensation Report of Injury or Illness" => {
                    Some(WorkersCompensationReportOfInjuryOrIllness)
                }
                "Interchange Update" => Some(InterchangeUpdate),
                "Investment Fees" => Some(InvestmentFees),
                "Maintenance and Operations Bill" => Some(MaintenanceAndOperationsBill),
                "Joint Facility Miscellaneous Bill or Other" => {
                    Some(JointFacilityMiscellaneousBillOrOther)
                }
                "Rental Bill" => Some(RentalBill),
                "Junction Settlement Update" => Some(JunctionSettlementUpdate),
                "Judgment" => Some(Judgment),
                "Junction Update" => Some(JunctionUpdate),
                "Termination for Default" => Some(TerminationForDefault),
                "Definitization of Contract" => Some(DefinitizationOfContract),
                "Definitization of Order" => Some(DefinitizationOfOrder),
                "Exercise of Option" => Some(ExerciseOfOption),
                "Intent to Exercise Option" => Some(IntentToExerciseOption),
                "Administrative Change" => Some(AdministrativeChange),
                "Change Order" => Some(ChangeOrder),
                "Supplemental Agreement" => Some(SupplementalAgreement),
                "Amended Shipping Instructions" => Some(AmendedShippingInstructions),
                "Provisioned Item" => Some(ProvisionedItem),
                "Withdrawal of Offer" => Some(WithdrawalOfOffer),
                "Additions to General Provisions" => Some(AdditionsToGeneralProvisions),
                "Request for Price Quote" => Some(RequestForPriceQuote),
                "Addition to Solicitation Mailing List" => {
                    Some(AdditionToSolicitationMailingList)
                }
                "Termination for Convenience" => Some(TerminationForConvenience),
                "Due-in Reconciliation Inquiry" => Some(DueInReconciliationInquiry),
                "Loan Distribution" => Some(LoanDistribution),
                "Loan Repayment Expense" => Some(LoanRepaymentExpense),
                "Landed Costs" => Some(LandedCosts),
                "Loss Notification" => Some(LossNotification),
                "Loan Repayment to Principal Only" => Some(LoanRepaymentToPrincipalOnly),
                "Loan Repayment" => Some(LoanRepayment),
                "Logistics Reassignment" => Some(LogisticsReassignment),
                "Levy" => Some(Levy),
                "Manufacturer Coupon Notification" => {
                    Some(ManufacturerCouponNotification)
                }
                "Measurement Events and Alarms" => Some(MeasurementEventsAndAlarms),
                "Mailing List" => Some(MailingList),
                "Maintenance to Business Taxpayer Registration" => {
                    Some(MaintenanceToBusinessTaxpayerRegistration)
                }
                "Material Credit Invoice" => Some(MaterialCreditInvoice),
                "Market Development Fund" => Some(MarketDevelopmentFund),
                "Memorandum" => Some(Memorandum),
                "Material, Engineering, Final Bill" => Some(CodeMF),
                "Material, Installation, Final Bill" => Some(CodeMI),
                "Membership List" => Some(MembershipList),
                "Multiple Shippers, Multiple Consignees" => Some(CodeMM),
                "Mise En Place (In Place)" => Some(CodeMP),
                "Material, Right to Use, Final Bill" => Some(CodeMR),
                "Material Final Bill" => Some(MaterialFinalBill),
                "Multifamily Program" => Some(MultifamilyProgram),
                "Bilateral" => Some(Bilateral),
                "Notice of Assessment" => Some(NoticeOfAssessment),
                "Notice of Warrant" => Some(NoticeOfWarrant),
                "Notice of Adjustment" => Some(NoticeOfAdjustment),
                "Notice of Determination" => Some(NoticeOfDetermination),
                "Notice of Settlement" => Some(NoticeOfSettlement),
                "Notice of Recorded Lien" => Some(NoticeOfRecordedLien),
                "Notice of Deficiency" => Some(NoticeOfDeficiency),
                "Material Release Order" => Some(MaterialReleaseOrder),
                "Material Release Inquiry" => Some(MaterialReleaseInquiry),
                "Material Release Order Forced Closure" => {
                    Some(MaterialReleaseOrderForcedClosure)
                }
                "Material Release Cancellation" => Some(MaterialReleaseCancellation),
                "Disposal Release Order" => Some(DisposalReleaseOrder),
                "Disposal Release Inquiry" => Some(DisposalReleaseInquiry),
                "Disposal Release Cancellation" => Some(DisposalReleaseCancellation),
                "Disposal Shipment Confirmation Inquiry" => {
                    Some(DisposalShipmentConfirmationInquiry)
                }
                "Redistribution Order" => Some(RedistributionOrder),
                "Material Release Confirmation" => Some(MaterialReleaseConfirmation),
                "Material Release Denial" => Some(MaterialReleaseDenial),
                "Material Release Advice" => Some(MaterialReleaseAdvice),
                "Disposal Release Confirmation" => Some(DisposalReleaseConfirmation),
                "Notice" => Some(Notice),
                "Disposal Release Denial" => Some(DisposalReleaseDenial),
                "Disposal Release Advice" => Some(DisposalReleaseAdvice),
                "Material Release Cancellation Advice" => {
                    Some(MaterialReleaseCancellationAdvice)
                }
                "In-Transit" => Some(InTransit),
                "Disposal Release Cancellation Advice" => {
                    Some(DisposalReleaseCancellationAdvice)
                }
                "Inventory Adjustment" => Some(InventoryAdjustment),
                "Operational Capacity" => Some(OperationalCapacity),
                "On Approval" => Some(OnApproval),
                "Offer" => Some(Offer),
                "Opinion" => Some(Opinion),
                "Order" => Some(Order),
                "Preliminary" => Some(Preliminary),
                "Employer Group Change" => Some(EmployerGroupChange),
                "Individual Change" => Some(IndividualChange),
                "Employer Open Enrollment" => Some(EmployerOpenEnrollment),
                "Predetermination - Medical" => Some(PredeterminationMedical),
                "Predetermination - Dental" => Some(PredeterminationDental),
                "Progress Payment Invoice" => Some(ProgressPaymentInvoice),
                "Partial Bill" => Some(PartialBill),
                "Invention Report" => Some(InventionReport),
                "Product Allocation" => Some(ProductAllocation),
                "Pleading" => Some(Pleading),
                "Partial Assignment Filing" => Some(PartialAssignmentFiling),
                "Premium Routing Guide" => Some(PremiumRoutingGuide),
                "Prospective Student Information" => Some(ProspectiveStudent),
                "Personal Injury Bill" => Some(PersonalInjuryBill),
                "Component Packing Confirmation" => Some(ComponentPackingConfirmation),
                "Plan Allocation" => Some(PlanAllocation),
                "Premium Payment" => Some(PremiumPayment),
                "Plan Takeover" => Some(PlanTakeover),
                "Prepaid Invoice" => Some(PrepaidInvoice),
                "Partial Enrollment File" => Some(PartialEnrollmentFile),
                "Product (or Service)" => Some(CodePR),
                "Postsecondary Student Academic Record" => {
                    Some(PostsecondaryStudentAcademicRecord)
                }
                "Plan-to-plan Transfer" => Some(PlanToPlanTransfer),
                "Notice of Claim" => Some(NoticeOfClaim),
                "Protest or Response to Claim" => Some(ProtestOrResponseToClaim),
                "Claim Decision" => Some(ClaimDecision),
                "Wage Verification Notice" => Some(WageVerificationNotice),
                "Purchase Report" => Some(PurchaseReport),
                "Scheduled Quantity" => Some(ScheduledQuantity),
                "Scheduled Quantity for Operator" => Some(ScheduledQuantityForOperator),
                "Coupon Regular Clearinghouse Invoice" => {
                    Some(CouponRegularClearinghouseInvoice)
                }
                "Coupon Direct Retailer Invoice" => Some(CouponDirectRetailerInvoice),
                "Coupon Clearinghouse Pay Direct Invoice" => {
                    Some(CouponClearinghousePayDirectInvoice)
                }
                "Product Quality Deficiency" => Some(ProductQualityDeficiency),
                "Coupon Scan Validate Invoice" => Some(CouponScanValidateInvoice),
                "Scan Validate Adjustment" => Some(ScanValidateAdjustment),
                "Quick Response Routing Guide" => Some(QuickResponseRoutingGuide),
                "Full Coupon Redemption (No Adjustments)" => Some(CodeQH),
                "Adjusted Coupon Redemption" => Some(AdjustedCouponRedemption),
                "Coupon Redemption Detail" => Some(CouponRedemptionDetail),
                "Adjustments to Previous Coupon Redemption" => {
                    Some(AdjustmentsToPreviousCouponRedemption)
                }
                "Coupon Quick Pay Invoice" => Some(CouponQuickPayInvoice),
                "Product Quality Deficiency Response" => {
                    Some(ProductQualityDeficiencyResponse)
                }
                "Request for Enrollment Verification" => {
                    Some(RequestForEnrollmentVerification)
                }
                "Response to Request for Enrollment Verification" => {
                    Some(ResponseToRequestForEnrollmentVerification)
                }
                "Response to Garnishment" => Some(ResponseToGarnishment),
                "Release of Garnishment" => Some(ReleaseOfGarnishment),
                "Response to Levy" => Some(ResponseToLevy),
                "Release of Levy" => Some(ReleaseOfLevy),
                "Response to Order to Withhold" => Some(ResponseToOrderToWithhold),
                "Release of Order to Withhold" => Some(ReleaseOfOrderToWithhold),
                "Request for Credit" => Some(RequestForCredit),
                "Right to Use" => Some(RightToUse),
                "Request for Quote" => Some(RequestForQuote),
                "Returns Detail" => Some(ReturnsDetail),
                "Rebill" => Some(Rebill),
                "Material, Engineering, Right to Use, Final Bill" => Some(CodeRF),
                "Revised Final Bill" => Some(RevisedFinalBill),
                "Request for Additional Funds" => Some(RequestForAdditionalFunds),
                "Routing Instructions" => Some(RoutingInstructions),
                "Response to Request for Routing Instructions" => {
                    Some(ResponseToRequestForRoutingInstructions)
                }
                "Registration" => Some(Registration),
                "Reminder to File" => Some(ReminderToFile),
                "Reporting" => Some(Reporting),
                "Request" => Some(Request),
                "Response - Additional Response(s) Available" => Some(CodeRS),
                "Spend Down" => Some(SpendDown),
                "Medical Services Reservation" => Some(MedicalServicesReservation),
                "Removed from Solicitation Mailing List" => {
                    Some(RemovedFromSolicitationMailingList)
                }
                "Special Routing Guide" => Some(SpecialRoutingGuide),
                "Standard Routing Guide" => Some(StandardRoutingGuide),
                "Supplemental Loan Repayment" => Some(SupplementalLoanRepayment),
                "Submission" => Some(Submission),
                "Stand-alone Lease Schedule" => Some(StandAloneLeaseSchedule),
                "Second Notice of Balance Due" => Some(SecondNoticeOfBalanceDue),
                "Deprescription" => Some(Deprescription),
                "Supply Process Deficiency" => Some(SupplyProcessDeficiency),
                "Special Bilateral" => Some(SpecialBilateral),
                "Single Family Program" => Some(SingleFamilyProgram),
                "Sample Goods Invoice" => Some(SampleGoodsInvoice),
                "Shipment Status Notification" => Some(ShipmentStatusNotification),
                "Sight Certification Request" => Some(SightCertificationRequest),
                "Summary Lease Schedule" => Some(SummaryLeaseSchedule),
                "Single Shipper, Multiple Consignees" => Some(CodeSM),
                "Spot Rate" => Some(SpotRate),
                "Supplier Rating" => Some(SupplierRating),
                "Schedule Query" => Some(ScheduleQuery),
                "Supply Process Deficiency Response" => {
                    Some(SupplyProcessDeficiencyResponse)
                }
                "Single Shipper, Single Consignee" => Some(CodeSS),
                "State Royalty" => Some(StateRoyalty),
                "Survey" => Some(Survey),
                "Supplemental Invoice" => Some(SupplementalInvoice),
                "Report sent by National Center for Education Statistics (NCES)" => {
                    Some(CodeT1)
                }
                "Report sent to National Center for Education Statistics (NCES)" => {
                    Some(CodeT2)
                }
                "Common Core of Data (CCD) Report from the National Center for Education Statistics (NCES)" => {
                    Some(CodeT3)
                }
                "Common Core of Data (CCD) Report to the National Center for Education Statistics (NCES)" => {
                    Some(CodeT4)
                }
                "Integrated Postsecondary Education Database System (IPEDS) Report from National Center for Education Statistics (NCES)" => {
                    Some(CodeT5)
                }
                "Integrated Postsecondary Education Database System (IPEDS) Report to National Center for Education Statistics (NCES)" => {
                    Some(CodeT6)
                }
                "Transportation Invoice" => Some(TransportationInvoice),
                "Sales Invoice" => Some(SalesInvoice),
                "Service Requester Level Invoice" => Some(ServiceRequesterLevelInvoice),
                "Buyer Managed Transportation" => Some(BuyerManagedTransportation),
                "Shipment or Movement Deficiency" => Some(ShipmentOrMovementDeficiency),
                "Tax or Fee Exemption Certification" => {
                    Some(TaxOrFeeExemptionCertification)
                }
                "Receipt Acknowledgment Inquiry" => Some(ReceiptAcknowledgmentInquiry),
                "Receipt Acknowledgment Advice" => Some(ReceiptAcknowledgmentAdvice),
                "Delinquent Due-in Advice" => Some(DelinquentDueInAdvice),
                "Delinquent Due-in Inquiry" => Some(DelinquentDueInInquiry),
                "Due-in Reconciliation Advice" => Some(DueInReconciliationAdvice),
                "Total Loss Evaluation" => Some(TotalLossEvaluation),
                "Trading Partner Information" => Some(TradingPartner),
                "Shipment or Movement Deficiency Response" => {
                    Some(ShipmentOrMovementDeficiencyResponse)
                }
                "Transfer Statement" => Some(TransferStatement),
                "Testing Service Report" => Some(TestingServiceReport),
                "Request for Testing Service Report" => {
                    Some(RequestForTestingServiceReport)
                }
                "Contract Abstract" => Some(ContractAbstract),
                "Shipment Performance Notice" => Some(ShipmentPerformanceNotice),
                "Acceptance Alert" => Some(AcceptanceAlert),
                "Update" => Some(Update),
                "Contract Payment Notice" => Some(ContractPaymentNotice),
                "Amendment Filing" => Some(AmendmentFiling),
                "Uniform Commercial Code Filing" => Some(UniformCommercialCodeFiling),
                "Unsalable Detail" => Some(UnsalableDetail),
                "Material, Engineering, Installation, Right to Use, Final Bill" => {
                    Some(CodeUF)
                }
                "Uniform Commercial Code Filing Inquiry" => {
                    Some(UniformCommercialCodeFilingInquiry)
                }
                "Termination Filing" => Some(TerminationFiling),
                "Original Filing" => Some(OriginalFiling),
                "Unsalable Product Invoice" => Some(UnsalableProductInvoice),
                "Uniform Commercial Code Filing Response to Inquiry" => {
                    Some(UniformCommercialCodeFilingResponseToInquiry)
                }
                "Unsubscribed Capacity" => Some(UnsubscribedCapacity),
                "Continuation Filing" => Some(ContinuationFiling),
                "Contract Completion Report" => Some(ContractCompletionReport),
                "Nomination Quick Response" => Some(NominationQuickResponse),
                "Confirmation Response Quick Response" => {
                    Some(ConfirmationResponseQuickResponse)
                }
                "Pre-determined Allocation Quick Response" => {
                    Some(PreDeterminedAllocationQuickResponse)
                }
                "Request for Confirmation Quick Response" => {
                    Some(RequestForConfirmationQuickResponse)
                }
                "Public Voucher" => Some(PublicVoucher),
                "Commercial Invoice" => Some(CommercialInvoice),
                "Violation Notice" => Some(ViolationNotice),
                "Voluntary MEDWATCH Report" => Some(VoluntaryMedwatchReport),
                "Mandatory MEDWATCH Report" => Some(MandatoryMedwatchReport),
                "Medical Device New Baseline Report" => {
                    Some(MedicalDeviceNewBaselineReport)
                }
                "Medical Device Annual Baseline Report" => {
                    Some(MedicalDeviceAnnualBaselineReport)
                }
                "User Facility Annual Medical Device Report" => {
                    Some(UserFacilityAnnualMedicalDeviceReport)
                }
                "Annual Certification of Medical Device Report" => {
                    Some(AnnualCertificationOfMedicalDeviceReport)
                }
                "Weapons Data Change" => Some(WeaponsDataChange),
                "Weapons Control Report" => Some(WeaponsControlReport),
                "Weapons Control Report Reconciliation" => {
                    Some(WeaponsControlReportReconciliation)
                }
                "Work Assignment" => Some(WorkAssignment),
                "Workers Compensation Proof of Coverage Notification" => {
                    Some(WorkersCompensationProofOfCoverageNotification)
                }
                "Withdrawal" => Some(Withdrawal),
                "Order to Withhold" => Some(OrderToWithhold),
                "Work Order" => Some(WorkOrder),
                "Waste" => Some(Waste),
                "Warrant" => Some(Warrant),
                "Consolidator's Invoice" => Some(ConsolidatorsInvoice),
                "Cancel Pending New Offer" => Some(CancelPendingNewOffer),
                "Bilateral Spot Rate" => Some(BilateralSpotRate),
                "Automatic Concurrence" => Some(AutomaticConcurrence),
                "Special Deprescription" => Some(SpecialDeprescription),
                "Firm Order Confirmation with Facility Information" => {
                    Some(FirmOrderConfirmationWithFacility)
                }
                "Firm Order Confirmation" => Some(FirmOrderConfirmation),
                "Facility Confirmation" => Some(FacilityConfirmation),
                "Funds Validation Inquiry" => Some(FundsValidationInquiry),
                "Funds Validation Response" => Some(FundsValidationResponse),
                "Military Interdepartmental Purchase Request (MIPR)" => Some(CodeZ1),
                "Project Directive" => Some(ProjectDirective),
                "Request for Contractual Procurement" => {
                    Some(RequestForContractualProcurement)
                }
                "Reimbursable Work Order" => Some(ReimbursableWorkOrder),
                "Request Initiation of Work Candidate" => {
                    Some(RequestInitiationOfWorkCandidate)
                }
                "Report of Work Candidate" => Some(ReportOfWorkCandidate),
                "Report of Assignment or Deletion of Work Candidate to Maintenance Period" => {
                    Some(ReportOfAssignmentOrDeletionOfWorkCandidateToMaintenancePeriod)
                }
                "Request Assignment of Work Candidate to Planning Maintenance Activity" => {
                    Some(RequestAssignmentOfWorkCandidateToPlanningMaintenanceActivity)
                }
                "Request for Full Work Candidate Detail" => {
                    Some(RequestForFullWorkCandidateDetail)
                }
                "Report of Full Work Candidate Detail" => {
                    Some(ReportOfFullWorkCandidateDetail)
                }
                "Report of Approved Work Candidate" => {
                    Some(ReportOfApprovedWorkCandidate)
                }
                "Request Work Candidate Cost/Duration Estimate" => {
                    Some(RequestWorkCandidateCostDurationEstimate)
                }
                "Report of Work Candidate Cost/Duration Estimate" => {
                    Some(ReportOfWorkCandidateCostDurationEstimate)
                }
                "Request Work Candidate Planning Services" => {
                    Some(RequestWorkCandidatePlanningServices)
                }
                "Report of Work Candidate Planning Services" => {
                    Some(ReportOfWorkCandidatePlanningServices)
                }
                "Report of Assignment or Deletion of Work Candidate to Planning/Maintenance Activity" => {
                    Some(
                        ReportOfAssignmentOrDeletionOfWorkCandidateToPlanningMaintenanceActivity,
                    )
                }
                "Request of Assignment or Deletion of Work Candidate to Maintenance Period" => {
                    Some(RequestOfAssignmentOrDeletionOfWorkCandidateToMaintenancePeriod)
                }
                "Stop Work Order" => Some(StopWorkOrder),
                "Authorization to Continue Work" => Some(AuthorizationToContinueWork),
                "Request for Departure From Specification" => {
                    Some(RequestForDepartureFromSpecification)
                }
                "Report of Authorized Departure From Specification" => {
                    Some(ReportOfAuthorizedDepartureFromSpecification)
                }
                "Request Work Progress Status" => Some(RequestWorkProgressStatus),
                "Report of Work Progress Status" => Some(ReportOfWorkProgressStatus),
                "Report of Rejection or Return of Work Candidate" => {
                    Some(ReportOfRejectionOrReturnOfWorkCandidate)
                }
                "Request Work Candidate Change" => Some(RequestWorkCandidateChange),
                "Lien Filing" => Some(LienFiling),
                "Sort and Segregate Detail" => Some(SortAndSegregateDetail),
                "Expungement of Prior Filing" => Some(ExpungementOfPriorFiling),
                "Cancellation of Filing" => Some(CancellationOfFiling),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for TransactionTypeCode {
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
    type Value = TransactionTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Transaction Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        TransactionTypeCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Transaction Type Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        TransactionTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Transaction Type Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for TransactionTypeCode {
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