use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**374

See docs at <https://www.stedi.com/edi/x12/element/374>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DateTimeQualifier {
    ///001 - Cancel After
    CancelAfter,
    ///002 - Delivery Requested
    DeliveryRequested,
    ///003 - Invoice
    Invoice,
    ///004 - Purchase Order
    PurchaseOrder,
    ///005 - Sailing
    Sailing,
    ///006 - Sold
    Sold,
    ///007 - Effective
    Effective,
    ///008 - Purchase Order Received
    PurchaseOrderReceived,
    ///009 - Process
    Process,
    ///010 - Requested Ship
    RequestedShip,
    ///011 - Shipped
    Shipped,
    ///012 - Terms Discount Due
    TermsDiscountDue,
    ///013 - Terms Net Due
    TermsNetDue,
    ///014 - Deferred Payment
    DeferredPayment,
    ///015 - Promotion Start
    PromotionStart,
    ///016 - Promotion End
    PromotionEnd,
    ///017 - Estimated Delivery
    EstimatedDelivery,
    ///018 - Available
    Available,
    ///019 - Unloaded
    Unloaded,
    ///020 - Check
    Check,
    ///021 - Charge Back
    ChargeBack,
    ///022 - Freight Bill
    FreightBill,
    ///023 - Promotion Order - Start
    PromotionOrderStart,
    ///024 - Promotion Order - End
    PromotionOrderEnd,
    ///025 - Promotion Ship - Start
    PromotionShipStart,
    ///026 - Promotion Ship - End
    PromotionShipEnd,
    ///027 - Promotion Requested Delivery - Start
    PromotionRequestedDeliveryStart,
    ///028 - Promotion Requested Delivery - End
    PromotionRequestedDeliveryEnd,
    ///029 - Promotion Performance - Start
    PromotionPerformanceStart,
    ///030 - Promotion Performance - End
    PromotionPerformanceEnd,
    ///031 - Promotion Invoice Performance - Start
    PromotionInvoicePerformanceStart,
    ///032 - Promotion Invoice Performance - End
    PromotionInvoicePerformanceEnd,
    ///033 - Promotion Floor Stock Protect - Start
    PromotionFloorStockProtectStart,
    ///034 - Promotion Floor Stock Protect - End
    PromotionFloorStockProtectEnd,
    ///035 - Delivered
    Delivered,
    ///036 - Expiration
    Expiration,
    ///037 - Ship Not Before
    ShipNotBefore,
    ///038 - Ship No Later
    ShipNoLater,
    ///039 - Ship Week of
    ShipWeekOf,
    ///040 - Status (After and Including)
    Code040,
    ///041 - Status (Prior and Including)
    Code041,
    ///042 - Superseded
    Superseded,
    ///043 - Publication
    Publication,
    ///044 - Settlement Date as Specified by the Originator
    SettlementDateAsSpecifiedByTheOriginator,
    ///045 - Endorsement Date
    EndorsementDate,
    ///046 - Field Failure
    FieldFailure,
    ///047 - Functional Test
    FunctionalTest,
    ///048 - System Test
    SystemTest,
    ///049 - Prototype Test
    PrototypeTest,
    ///050 - Received
    Received,
    ///051 - Cumulative Quantity Start
    CumulativeQuantityStart,
    ///052 - Cumulative Quantity End
    CumulativeQuantityEnd,
    ///053 - Buyers Local
    BuyersLocal,
    ///054 - Sellers Local
    SellersLocal,
    ///055 - Confirmed
    Confirmed,
    ///056 - Estimated Port of Entry
    EstimatedPortOfEntry,
    ///057 - Actual Port of Entry
    ActualPortOfEntry,
    ///058 - Customs Clearance
    CustomsClearance,
    ///059 - Inland Ship
    InlandShip,
    ///060 - Engineering Change Level
    EngineeringChangeLevel,
    ///061 - Cancel if Not Delivered by
    CancelIfNotDeliveredBy,
    ///062 - Blueprint
    Blueprint,
    ///063 - Do Not Deliver After
    DoNotDeliverAfter,
    ///064 - Do Not Deliver Before
    DoNotDeliverBefore,
    ///065 - 1st Schedule Delivery
    Code065,
    ///066 - 1st Schedule Ship
    Code066,
    ///067 - Current Schedule Delivery
    CurrentScheduleDelivery,
    ///068 - Current Schedule Ship
    CurrentScheduleShip,
    ///069 - Promised for Delivery
    PromisedForDelivery,
    ///070 - Scheduled for Delivery (After and Including)
    Code070,
    ///071 - Requested for Delivery (After and Including)
    Code071,
    ///072 - Promised for Delivery (After and Including)
    Code072,
    ///073 - Scheduled for Delivery (Prior to and Including)
    Code073,
    ///074 - Requested for Delivery (Prior to and Including)
    Code074,
    ///075 - Promised for Delivery (Prior to and Including)
    Code075,
    ///076 - Scheduled for Delivery (Week of)
    Code076,
    ///077 - Requested for Delivery (Week of)
    Code077,
    ///078 - Promised for Delivery (Week of)
    Code078,
    ///079 - Promised for Shipment
    PromisedForShipment,
    ///080 - Scheduled for Shipment (After and Including)
    Code080,
    ///081 - Requested for Shipment (After and Including)
    Code081,
    ///082 - Promised for Shipment (After and Including)
    Code082,
    ///083 - Scheduled for Shipment (Prior to and Including)
    Code083,
    ///084 - Requested for Shipment (Prior to and Including)
    Code084,
    ///085 - Promised for Shipment (Prior to and Including)
    Code085,
    ///086 - Scheduled for Shipment (Week of)
    Code086,
    ///087 - Requested for Shipment (Week of)
    Code087,
    ///088 - Promised for Shipment (Week of)
    Code088,
    ///089 - Inquiry
    Inquiry,
    ///090 - Report Start
    ReportStart,
    ///091 - Report End
    ReportEnd,
    ///092 - Contract Effective
    ContractEffective,
    ///093 - Contract Expiration
    ContractExpiration,
    ///094 - Manufacture
    Manufacture,
    ///095 - Bill of Lading
    BillOfLading,
    ///096 - Discharge
    Discharge,
    ///097 - Transaction Creation
    TransactionCreation,
    ///098 - Bid (Effective)
    Code098,
    ///099 - Bid Open (Date Bids Will Be Opened)
    Code099,
    ///100 - No Shipping Schedule Established as of
    NoShippingScheduleEstablishedAsOf,
    ///101 - No Production Schedule Established as of
    NoProductionScheduleEstablishedAsOf,
    ///102 - Issue
    Issue,
    ///103 - Award
    Award,
    ///104 - System Survey
    SystemSurvey,
    ///105 - Quality Rating
    QualityRating,
    ///106 - Required By
    RequiredBy,
    ///107 - Deposit
    Deposit,
    ///108 - Postmark
    Postmark,
    ///109 - Received at Lockbox
    ReceivedAtLockbox,
    ///110 - Originally Scheduled Ship
    OriginallyScheduledShip,
    ///111 - Manifest/Ship Notice
    ManifestShipNotice,
    ///112 - Buyers Dock
    BuyersDock,
    ///113 - Sample Required
    SampleRequired,
    ///114 - Tooling Required
    ToolingRequired,
    ///115 - Sample Available
    SampleAvailable,
    ///116 - Scheduled Interchange Delivery
    ScheduledInterchangeDelivery,
    ///118 - Requested Pickup
    RequestedPickup,
    ///119 - Test Performed
    TestPerformed,
    ///120 - Control Plan
    ControlPlan,
    ///121 - Feasibility Sign Off
    FeasibilitySignOff,
    ///122 - Failure Mode Effective
    FailureModeEffective,
    ///124 - Group Contract Effective
    GroupContractEffective,
    ///125 - Group Contract Expiration
    GroupContractExpiration,
    ///126 - Wholesale Contract Effective
    WholesaleContractEffective,
    ///127 - Wholesale Contract Expiration
    WholesaleContractExpiration,
    ///128 - Replacement Effective
    ReplacementEffective,
    ///129 - Customer Contract Effective
    CustomerContractEffective,
    ///130 - Customer Contract Expiration
    CustomerContractExpiration,
    ///131 - Item Contract Effective
    ItemContractEffective,
    ///132 - Item Contract Expiration
    ItemContractExpiration,
    ///133 - Accounts Receivable - Statement Date
    AccountsReceivableStatementDate,
    ///134 - Ready for Inspection
    ReadyForInspection,
    ///135 - Booking
    Booking,
    ///136 - Technical Rating
    TechnicalRating,
    ///137 - Delivery Rating
    DeliveryRating,
    ///138 - Commercial Rating
    CommercialRating,
    ///139 - Estimated
    Estimated,
    ///140 - Actual
    Actual,
    ///141 - Assigned
    Assigned,
    ///142 - Loss
    Loss,
    ///143 - Due Date of First Payment to Principal and Interest
    DueDateOfFirstPaymentToPrincipalAndInterest,
    ///144 - Estimated Acceptance
    EstimatedAcceptance,
    ///145 - Opening Date
    OpeningDate,
    ///146 - Closing Date
    ClosingDate,
    ///147 - Due Date Last Complete Installment Paid
    DueDateLastCompleteInstallmentPaid,
    ///148 - Date of Local Office Approval of Conveyance of Damaged Real Estate Property
    DateOfLocalOfficeApprovalOfConveyanceOfDamagedRealEstateProperty,
    ///149 - Date Deed Filed for Record
    DateDeedFiledForRecord,
    ///150 - Service Period Start
    ServicePeriodStart,
    ///151 - Service Period End
    ServicePeriodEnd,
    ///152 - Effective Date of Change
    EffectiveDateOfChange,
    ///153 - Service Interruption
    ServiceInterruption,
    ///154 - Adjustment Period Start
    AdjustmentPeriodStart,
    ///155 - Adjustment Period End
    AdjustmentPeriodEnd,
    ///156 - Allotment Period Start
    AllotmentPeriodStart,
    ///157 - Test Period Start
    TestPeriodStart,
    ///158 - Test Period Ending
    TestPeriodEnding,
    ///159 - Bid Price Exception
    BidPriceException,
    ///160 - Samples to be Returned By
    SamplesToBeReturnedBy,
    ///161 - Loaded on Vessel
    LoadedOnVessel,
    ///162 - Pending Archive
    PendingArchive,
    ///163 - Actual Archive
    ActualArchive,
    ///164 - First Issue
    FirstIssue,
    ///165 - Final Issue
    FinalIssue,
    ///166 - Message
    Message,
    ///167 - Most Recent Revision (or Initial Version)
    Code167,
    ///168 - Release
    Release,
    ///169 - Product Availability Date
    ProductAvailabilityDate,
    ///170 - Supplemental Issue
    SupplementalIssue,
    ///171 - Revision
    Revision,
    ///172 - Correction
    Correction,
    ///173 - Week Ending
    WeekEnding,
    ///174 - Month Ending
    MonthEnding,
    ///175 - Cancel if not shipped by
    CancelIfNotShippedBy,
    ///176 - Expedited on
    ExpeditedOn,
    ///177 - Cancellation
    Cancellation,
    ///178 - Hold (as of)
    Code178,
    ///179 - Hold as Stock (as of)
    Code179,
    ///180 - No Promise (as of)
    Code180,
    ///181 - Stop Work (as of)
    Code181,
    ///182 - Will Advise (as of)
    Code182,
    ///183 - Connection
    Connection,
    ///184 - Inventory
    Inventory,
    ///185 - Vessel Registry
    VesselRegistry,
    ///186 - Invoice Period Start
    InvoicePeriodStart,
    ///187 - Invoice Period End
    InvoicePeriodEnd,
    ///188 - Credit Advice
    CreditAdvice,
    ///189 - Debit Advice
    DebitAdvice,
    ///190 - Released to Vessel
    ReleasedToVessel,
    ///191 - Material Specification
    MaterialSpecification,
    ///192 - Delivery Ticket
    DeliveryTicket,
    ///193 - Period Start
    PeriodStart,
    ///194 - Period End
    PeriodEnd,
    ///195 - Contract Re-Open
    ContractReOpen,
    ///196 - Start
    Start,
    ///197 - End
    End,
    ///198 - Completion
    Completion,
    ///199 - Seal
    Seal,
    ///200 - Assembly Start
    AssemblyStart,
    ///201 - Acceptance
    Acceptance,
    ///202 - Master Lease Agreement
    MasterLeaseAgreement,
    ///203 - First Produced
    FirstProduced,
    ///204 - Official Rail Car Interchange (Either Actual or Agreed Upon)
    Code204,
    ///205 - Transmitted
    Transmitted,
    ///206 - Status (Outside Processor)
    Code206,
    ///207 - Status (Commercial)
    Code207,
    ///208 - Lot Number Expiration
    LotNumberExpiration,
    ///209 - Contract Performance Start
    ContractPerformanceStart,
    ///210 - Contract Performance Delivery
    ContractPerformanceDelivery,
    ///211 - Service Requested
    ServiceRequested,
    ///212 - Returned to Customer
    ReturnedToCustomer,
    ///213 - Adjustment to Bill Dated
    AdjustmentToBillDated,
    ///214 - Date of Repair/Service
    DateOfRepairService,
    ///215 - Interruption Start
    InterruptionStart,
    ///216 - Interruption End
    InterruptionEnd,
    ///217 - Spud
    Spud,
    ///218 - Initial Completion
    InitialCompletion,
    ///219 - Plugged and Abandoned
    PluggedAndAbandoned,
    ///220 - Penalty
    Penalty,
    ///221 - Penalty Begin
    PenaltyBegin,
    ///222 - Birth
    Birth,
    ///223 - Birth Certificate
    BirthCertificate,
    ///224 - Adoption
    Adoption,
    ///225 - Christening
    Christening,
    ///226 - Lease Commencement
    LeaseCommencement,
    ///227 - Lease Term Start
    LeaseTermStart,
    ///228 - Lease Term End
    LeaseTermEnd,
    ///229 - Rent Start
    RentStart,
    ///230 - Installation
    Installation,
    ///231 - Progress Payment
    ProgressPayment,
    ///232 - Claim Statement Period Start
    ClaimStatementPeriodStart,
    ///233 - Claim Statement Period End
    ClaimStatementPeriodEnd,
    ///234 - Settlement Date
    SettlementDate,
    ///235 - Delayed Billing (Not Delayed Payment)
    Code235,
    ///236 - Lender Credit Check
    LenderCreditCheck,
    ///237 - Student Signed
    StudentSigned,
    ///238 - Schedule Release
    ScheduleRelease,
    ///239 - Baseline
    Baseline,
    ///240 - Baseline Start
    BaselineStart,
    ///241 - Baseline Complete
    BaselineComplete,
    ///242 - Actual Start
    ActualStart,
    ///243 - Actual Complete
    ActualComplete,
    ///244 - Estimated Start
    EstimatedStart,
    ///245 - Estimated Completion
    EstimatedCompletion,
    ///246 - Start no earlier than
    StartNoEarlierThan,
    ///247 - Start no later than
    StartNoLaterThan,
    ///248 - Finish no later than
    FinishNoLaterThan,
    ///249 - Finish no earlier than
    FinishNoEarlierThan,
    ///250 - Mandatory (or Target) Start
    Code250,
    ///251 - Mandatory (or Target) Finish
    Code251,
    ///252 - Early Start
    EarlyStart,
    ///253 - Early Finish
    EarlyFinish,
    ///254 - Late Start
    LateStart,
    ///255 - Late Finish
    LateFinish,
    ///256 - Scheduled Start
    ScheduledStart,
    ///257 - Scheduled Finish
    ScheduledFinish,
    ///258 - Original Early Start
    OriginalEarlyStart,
    ///259 - Original Early Finish
    OriginalEarlyFinish,
    ///260 - Rest Day
    RestDay,
    ///261 - Rest Start
    RestStart,
    ///262 - Rest Finish
    RestFinish,
    ///263 - Holiday
    Holiday,
    ///264 - Holiday Start
    HolidayStart,
    ///265 - Holiday Finish
    HolidayFinish,
    ///266 - Base
    Base,
    ///267 - Timenow
    Timenow,
    ///268 - End Date of Support
    EndDateOfSupport,
    ///269 - Date Account Matures
    DateAccountMatures,
    ///270 - Date Filed
    DateFiled,
    ///271 - Penalty End
    PenaltyEnd,
    ///272 - Exit Plant Date
    ExitPlantDate,
    ///273 - Latest On Board Carrier Date
    LatestOnBoardCarrierDate,
    ///274 - Requested Departure Date
    RequestedDepartureDate,
    ///275 - Approved
    Approved,
    ///276 - Contract Start
    ContractStart,
    ///277 - Contract Definition
    ContractDefinition,
    ///278 - Last Item Delivery
    LastItemDelivery,
    ///279 - Contract Completion
    ContractCompletion,
    ///280 - Date Course of Orthodontics Treatment Began or is Expected to Begin
    DateCourseOfOrthodonticsTreatmentBeganOrIsExpectedToBegin,
    ///281 - Over Target Baseline Month
    OverTargetBaselineMonth,
    ///282 - Previous Report
    PreviousReport,
    ///283 - Funds Appropriation - Start
    FundsAppropriationStart,
    ///284 - Funds Appropriation - End
    FundsAppropriationEnd,
    ///285 - Employment or Hire
    EmploymentOrHire,
    ///286 - Retirement
    Retirement,
    ///287 - Medicare
    Medicare,
    ///288 - Consolidated Omnibus Budget Reconciliation Act (COBRA)
    Code288,
    ///289 - Premium Paid to Date
    PremiumPaidToDate,
    ///290 - Coordination of Benefits
    CoordinationOfBenefits,
    ///291 - Plan
    Plan,
    ///292 - Benefit
    Benefit,
    ///293 - Education
    Education,
    ///294 - Earnings Effective Date
    EarningsEffectiveDate,
    ///295 - Primary Care Provider
    PrimaryCareProvider,
    ///296 - Initial Disability Period Return To Work
    InitialDisabilityPeriodReturnToWork,
    ///297 - Initial Disability Period Last Day Worked
    InitialDisabilityPeriodLastDayWorked,
    ///298 - Latest Absence
    LatestAbsence,
    ///299 - Illness
    Illness,
    ///300 - Enrollment Signature Date
    EnrollmentSignatureDate,
    ///301 - Consolidated Omnibus Budget Reconciliation Act (COBRA) Qualifying Event
    Code301,
    ///302 - Maintenance
    Maintenance,
    ///303 - Maintenance Effective
    MaintenanceEffective,
    ///304 - Latest Visit or Consultation
    LatestVisitOrConsultation,
    ///305 - Net Credit Service Date
    NetCreditServiceDate,
    ///306 - Adjustment Effective Date
    AdjustmentEffectiveDate,
    ///307 - Eligibility
    Eligibility,
    ///308 - Pre-Award Survey
    PreAwardSurvey,
    ///309 - Plan Termination
    PlanTermination,
    ///310 - Date of Closing
    DateOfClosing,
    ///311 - Latest Receiving Date/Cutoff Date
    LatestReceivingDateCutoffDate,
    ///312 - Salary Deferral
    SalaryDeferral,
    ///313 - Cycle
    Cycle,
    ///314 - Disability
    Disability,
    ///315 - Offset
    Offset,
    ///316 - Prior Incorrect Date of Birth
    PriorIncorrectDateOfBirth,
    ///317 - Corrected Date of Birth
    CorrectedDateOfBirth,
    ///318 - Added
    Added,
    ///319 - Failed
    Failed,
    ///320 - Date Foreclosure Proceedings Instituted
    DateForeclosureProceedingsInstituted,
    ///321 - Purchased
    Purchased,
    ///322 - Put into Service
    PutIntoService,
    ///323 - Replaced
    Replaced,
    ///324 - Returned
    Returned,
    ///325 - Disbursement Date
    DisbursementDate,
    ///326 - Guarantee Date
    GuaranteeDate,
    ///327 - Quarter Ending
    QuarterEnding,
    ///328 - Changed
    Changed,
    ///329 - Terminated
    Terminated,
    ///330 - Referral Date
    ReferralDate,
    ///331 - Evaluation Date
    EvaluationDate,
    ///332 - Placement Date
    PlacementDate,
    ///333 - Individual Education Plan (IEP)
    Code333,
    ///334 - Re-evaluation Date
    ReEvaluationDate,
    ///335 - Dismissal Date
    DismissalDate,
    ///336 - Employment Begin
    EmploymentBegin,
    ///337 - Employment End
    EmploymentEnd,
    ///338 - Medicare Begin
    MedicareBegin,
    ///339 - Medicare End
    MedicareEnd,
    ///340 - Consolidated Omnibus Budget Reconciliation Act (COBRA) Begin
    Code340,
    ///341 - Consolidated Omnibus Budget Reconciliation Act (COBRA) End
    Code341,
    ///342 - Premium Paid to Date Begin
    PremiumPaidToDateBegin,
    ///343 - Premium Paid to Date End
    PremiumPaidToDateEnd,
    ///344 - Coordination of Benefits Begin
    CoordinationOfBenefitsBegin,
    ///345 - Coordination of Benefits End
    CoordinationOfBenefitsEnd,
    ///346 - Plan Begin
    PlanBegin,
    ///347 - Plan End
    PlanEnd,
    ///348 - Benefit Begin
    BenefitBegin,
    ///349 - Benefit End
    BenefitEnd,
    ///350 - Education Begin
    EducationBegin,
    ///351 - Education End
    EducationEnd,
    ///352 - Primary Care Provider Begin
    PrimaryCareProviderBegin,
    ///353 - Primary Care Provider End
    PrimaryCareProviderEnd,
    ///354 - Illness Begin
    IllnessBegin,
    ///355 - Illness End
    IllnessEnd,
    ///356 - Eligibility Begin
    EligibilityBegin,
    ///357 - Eligibility End
    EligibilityEnd,
    ///358 - Cycle Begin
    CycleBegin,
    ///359 - Cycle End
    CycleEnd,
    ///360 - Initial Disability Period Start
    InitialDisabilityPeriodStart,
    ///361 - Initial Disability Period End
    InitialDisabilityPeriodEnd,
    ///362 - Offset Begin
    OffsetBegin,
    ///363 - Offset End
    OffsetEnd,
    ///364 - Plan Period Election Begin
    PlanPeriodElectionBegin,
    ///365 - Plan Period Election End
    PlanPeriodElectionEnd,
    ///366 - Plan Period Election
    PlanPeriodElection,
    ///367 - Due to Customer
    DueToCustomer,
    ///368 - Submittal
    Submittal,
    ///369 - Estimated Departure Date
    EstimatedDepartureDate,
    ///370 - Actual Departure Date
    ActualDepartureDate,
    ///371 - Estimated Arrival Date
    EstimatedArrivalDate,
    ///372 - Actual Arrival Date
    ActualArrivalDate,
    ///373 - Order Start
    OrderStart,
    ///374 - Order End
    OrderEnd,
    ///375 - Delivery Start
    DeliveryStart,
    ///376 - Delivery End
    DeliveryEnd,
    ///377 - Contract Costs Through
    ContractCostsThrough,
    ///378 - Financial Information Submission
    FinancialInformationSubmission,
    ///379 - Business Termination
    BusinessTermination,
    ///380 - Applicant Signed
    ApplicantSigned,
    ///381 - Cosigner Signed
    CosignerSigned,
    ///382 - Enrollment
    Enrollment,
    ///383 - Adjusted Hire
    AdjustedHire,
    ///384 - Credited Service
    CreditedService,
    ///385 - Credited Service Begin
    CreditedServiceBegin,
    ///386 - Credited Service End
    CreditedServiceEnd,
    ///387 - Deferred Distribution
    DeferredDistribution,
    ///388 - Payment Commencement
    PaymentCommencement,
    ///389 - Payroll Period
    PayrollPeriod,
    ///390 - Payroll Period Begin
    PayrollPeriodBegin,
    ///391 - Payroll Period End
    PayrollPeriodEnd,
    ///392 - Plan Entry
    PlanEntry,
    ///393 - Plan Participation Suspension
    PlanParticipationSuspension,
    ///394 - Rehire
    Rehire,
    ///395 - Retermination
    Retermination,
    ///396 - Termination
    Termination,
    ///397 - Valuation
    Valuation,
    ///398 - Vesting Service
    VestingService,
    ///399 - Vesting Service Begin
    VestingServiceBegin,
    ///400 - Vesting Service End
    VestingServiceEnd,
    ///401 - Duplicate Bill
    DuplicateBill,
    ///402 - Adjustment Promised
    AdjustmentPromised,
    ///403 - Adjustment Processed
    AdjustmentProcessed,
    ///404 - Year Ending
    YearEnding,
    ///405 - Production
    Production,
    ///406 - Material Classification
    MaterialClassification,
    ///408 - Weighed
    Weighed,
    ///409 - Date of Deed in Lieu
    DateOfDeedInLieu,
    ///410 - Date of Firm Commitment
    DateOfFirmCommitment,
    ///411 - Expiration Date of Extension to Foreclose
    ExpirationDateOfExtensionToForeclose,
    ///412 - Date of Notice to Convey
    DateOfNoticeToConvey,
    ///413 - Date of Release of Bankruptcy
    DateOfReleaseOfBankruptcy,
    ///414 - Optimistic Early Start
    OptimisticEarlyStart,
    ///415 - Optimistic Early Finish
    OptimisticEarlyFinish,
    ///416 - Optimistic Late Start
    OptimisticLateStart,
    ///417 - Optimistic Late Finish
    OptimisticLateFinish,
    ///418 - Most Likely Early Start
    MostLikelyEarlyStart,
    ///419 - Most Likely Early Finish
    MostLikelyEarlyFinish,
    ///420 - Most Likely Late Start
    MostLikelyLateStart,
    ///421 - Most Likely Late Finish
    MostLikelyLateFinish,
    ///422 - Pessimistic Early Start
    PessimisticEarlyStart,
    ///423 - Pessimistic Early Finish
    PessimisticEarlyFinish,
    ///424 - Pessimistic Late Start
    PessimisticLateStart,
    ///425 - Pessimistic Late Finish
    PessimisticLateFinish,
    ///426 - First Payment Due
    FirstPaymentDue,
    ///427 - First Interest Payment Due
    FirstInterestPaymentDue,
    ///428 - Subsequent Interest Payment Due
    SubsequentInterestPaymentDue,
    ///429 - Irregular Interest Payment Due
    IrregularInterestPaymentDue,
    ///430 - Guarantor Received
    GuarantorReceived,
    ///431 - Onset of Current Symptoms or Illness
    OnsetOfCurrentSymptomsOrIllness,
    ///432 - Submission
    Submission,
    ///433 - Removed
    Removed,
    ///434 - Statement
    Statement,
    ///435 - Admission
    Admission,
    ///436 - Insurance Card
    InsuranceCard,
    ///437 - Spouse Retirement
    SpouseRetirement,
    ///438 - Onset of Similar Symptoms or Illness
    OnsetOfSimilarSymptomsOrIllness,
    ///439 - Accident
    Accident,
    ///440 - Release of Information
    ReleaseOf,
    ///441 - Prior Placement
    PriorPlacement,
    ///442 - Date of Death
    DateOfDeath,
    ///443 - Peer Review Organization (PRO) Approved Stay
    Code443,
    ///444 - First Visit or Consultation
    FirstVisitOrConsultation,
    ///445 - Initial Placement
    InitialPlacement,
    ///446 - Replacement
    Replacement,
    ///447 - Occurrence
    Occurrence,
    ///448 - Occurrence Span
    OccurrenceSpan,
    ///449 - Occurrence Span From
    OccurrenceSpanFrom,
    ///450 - Occurrence Span To
    OccurrenceSpanTo,
    ///451 - Initial Fee Due
    InitialFeeDue,
    ///452 - Appliance Placement
    AppliancePlacement,
    ///453 - Acute Manifestation of a Chronic Condition
    AcuteManifestationOfAChronicCondition,
    ///454 - Initial Treatment
    InitialTreatment,
    ///455 - Last X-Ray
    LastXRay,
    ///456 - Surgery
    Surgery,
    ///457 - Continuous Passive Motion (CPM)
    Code457,
    ///458 - Certification
    Certification,
    ///459 - Nursing Home From
    NursingHomeFrom,
    ///460 - Nursing Home To
    NursingHomeTo,
    ///461 - Last Certification
    LastCertification,
    ///462 - Date of Local Office Approval of Conveyance of Occupied Real Estate Property
    DateOfLocalOfficeApprovalOfConveyanceOfOccupiedRealEstateProperty,
    ///463 - Begin Therapy
    BeginTherapy,
    ///464 - Oxygen Therapy From
    OxygenTherapyFrom,
    ///465 - Oxygen Therapy To
    OxygenTherapyTo,
    ///466 - Oxygen Therapy
    OxygenTherapy,
    ///467 - Signature
    Signature,
    ///468 - Prescription Fill
    PrescriptionFill,
    ///469 - Provider Signature
    ProviderSignature,
    ///470 - Date of Local Office Certification of Conveyance of Damaged Real Estate Property
    DateOfLocalOfficeCertificationOfConveyanceOfDamagedRealEstateProperty,
    ///471 - Prescription
    Prescription,
    ///472 - Service
    Service,
    ///473 - Medicaid Begin
    MedicaidBegin,
    ///474 - Medicaid End
    MedicaidEnd,
    ///475 - Medicaid
    Medicaid,
    ///476 - Peer Review Organization (PRO) Approved Stay From
    Code476,
    ///477 - Peer Review Organization (PRO) Approved Stay To
    Code477,
    ///478 - Prescription From
    PrescriptionFrom,
    ///479 - Prescription To
    PrescriptionTo,
    ///480 - Arterial Blood Gas Test
    ArterialBloodGasTest,
    ///481 - Oxygen Saturation Test
    OxygenSaturationTest,
    ///482 - Pregnancy Begin
    PregnancyBegin,
    ///483 - Pregnancy End
    PregnancyEnd,
    ///484 - Last Menstrual Period
    LastMenstrualPeriod,
    ///485 - Injury Begin
    InjuryBegin,
    ///486 - Injury End
    InjuryEnd,
    ///487 - Nursing Home
    NursingHome,
    ///488 - Collateral Dependent
    CollateralDependent,
    ///489 - Collateral Dependent Begin
    CollateralDependentBegin,
    ///490 - Collateral Dependent End
    CollateralDependentEnd,
    ///491 - Sponsored Dependent
    SponsoredDependent,
    ///492 - Sponsored Dependent Begin
    SponsoredDependentBegin,
    ///493 - Sponsored Dependent End
    SponsoredDependentEnd,
    ///494 - Deductible
    Deductible,
    ///495 - Out-of-Pocket
    OutOfPocket,
    ///496 - Contract Audit Date
    ContractAuditDate,
    ///497 - Latest Delivery Date at Pier
    LatestDeliveryDateAtPier,
    ///498 - Mortgagee Reported Curtailment Date
    MortgageeReportedCurtailmentDate,
    ///499 - Mortgagee Official Signature Date
    MortgageeOfficialSignatureDate,
    ///500 - Resubmission
    Resubmission,
    ///501 - Expected Reply
    ExpectedReply,
    ///502 - Dropped to Less than Half Time
    DroppedToLessThanHalfTime,
    ///503 - Repayment Begin
    RepaymentBegin,
    ///504 - Loan Servicing Transfer
    LoanServicingTransfer,
    ///505 - Loan Purchase
    LoanPurchase,
    ///506 - Last Notification
    LastNotification,
    ///507 - Extract
    Extract,
    ///508 - Extended
    Extended,
    ///509 - Servicer Signature Date
    ServicerSignatureDate,
    ///510 - Date Packed
    DatePacked,
    ///511 - Shelf Life Expiration
    ShelfLifeExpiration,
    ///512 - Warranty Expiration
    WarrantyExpiration,
    ///513 - Overhauled
    Overhauled,
    ///514 - Transferred
    Transferred,
    ///515 - Notified
    Notified,
    ///516 - Discovered
    Discovered,
    ///517 - Inspected
    Inspected,
    ///518 - Voucher (Date of)
    Code518,
    ///519 - Date Bankruptcy Filed
    DateBankruptcyFiled,
    ///520 - Date of Damage
    DateOfDamage,
    ///521 - Date Hazard Insurance Policy Cancelled
    DateHazardInsurancePolicyCancelled,
    ///522 - Expiration Date to Submit Title Evidence
    ExpirationDateToSubmitTitleEvidence,
    ///523 - Date of Claim
    DateOfClaim,
    ///524 - Date of Notice of Referral for Assignment
    DateOfNoticeOfReferralForAssignment,
    ///525 - Date of Notice of Probable Ineligibility for Assignment
    DateOfNoticeOfProbableIneligibilityForAssignment,
    ///526 - Date of Foreclosure Notice
    DateOfForeclosureNotice,
    ///527 - Expiration of Foreclosure Timeframe
    ExpirationOfForeclosureTimeframe,
    ///528 - Date Possessory Action Initiated
    DatePossessoryActionInitiated,
    ///529 - Date of Possession
    DateOfPossession,
    ///530 - Date of Last Installment Received
    DateOfLastInstallmentReceived,
    ///531 - Date of Acquisition of Title
    DateOfAcquisitionOfTitle,
    ///532 - Expiration of Extension to Convey
    ExpirationOfExtensionToConvey,
    ///533 - Date of Assignment Approval
    DateOfAssignmentApproval,
    ///534 - Date of Assignment Rejection
    DateOfAssignmentRejection,
    ///535 - Curtailment Date from Advice of Payment
    CurtailmentDateFromAdviceOfPayment,
    ///536 - Expiration of Extension to Submit Fiscal Data
    ExpirationOfExtensionToSubmitFiscalData,
    ///537 - Date Documentation, or Paperwork, or Both Was Sent
    Code537,
    ///538 - Makegood Commercial Date
    MakegoodCommercialDate,
    ///539 - Policy Effective
    PolicyEffective,
    ///540 - Policy Expiration
    PolicyExpiration,
    ///541 - Employee Effective Date of Coverage
    EmployeeEffectiveDateOfCoverage,
    ///542 - Claim Administrator Notified of Employee Legal Representation
    ClaimAdministratorNotifiedOfEmployeeLegalRepresentation,
    ///543 - Last Premium Paid Date
    LastPremiumPaidDate,
    ///544 - Employer Knowledge of the Injury
    EmployerKnowledgeOfTheInjury,
    ///545 - Claim Administrator Knowledge of the Injury
    ClaimAdministratorKnowledgeOfTheInjury,
    ///546 - Date of Maximum Medical Improvement
    DateOfMaximumMedicalImprovement,
    ///547 - Date of Loan
    DateOfLoan,
    ///548 - Date of Advance
    DateOfAdvance,
    ///549 - Beginning Lay Date
    BeginningLayDate,
    ///550 - Certificate Effective
    CertificateEffective,
    ///551 - Benefit Application Date
    BenefitApplicationDate,
    ///552 - Actual Return to Work
    ActualReturnToWork,
    ///553 - Released Return to Work
    ReleasedReturnToWork,
    ///554 - Ending Lay Date
    EndingLayDate,
    ///555 - Employee Wages Ceased
    EmployeeWagesCeased,
    ///556 - Last Salary Increase
    LastSalaryIncrease,
    ///557 - Employee Laid Off
    EmployeeLaidOff,
    ///558 - Injury or Illness
    InjuryOrIllness,
    ///559 - Oldest Unpaid Installment
    OldestUnpaidInstallment,
    ///560 - Preforeclosure Acceptance Date
    PreforeclosureAcceptanceDate,
    ///561 - Preforeclosure Sale Closing Date
    PreforeclosureSaleClosingDate,
    ///562 - Date of First Uncured Default
    DateOfFirstUncuredDefault,
    ///563 - Date Default Was Cured
    DateDefaultWasCured,
    ///564 - Date of First Mortgage Payment
    DateOfFirstMortgagePayment,
    ///565 - Date of Property Inspection
    DateOfPropertyInspection,
    ///566 - Date Total Amount of Delinquency Reported
    DateTotalAmountOfDelinquencyReported,
    ///567 - Date Outstanding Loan Balance Reported
    DateOutstandingLoanBalanceReported,
    ///568 - Date Foreclosure Sale Scheduled
    DateForeclosureSaleScheduled,
    ///569 - Date Foreclosure Held
    DateForeclosureHeld,
    ///570 - Date Redemption Period Ends
    DateRedemptionPeriodEnds,
    ///571 - Date Voluntary Conveyance Accepted
    DateVoluntaryConveyanceAccepted,
    ///572 - Date Property Sold
    DatePropertySold,
    ///573 - Date Claim Paid
    DateClaimPaid,
    ///574 - Action Begin Date
    ActionBeginDate,
    ///575 - Projected Action End Date
    ProjectedActionEndDate,
    ///576 - Action End Date
    ActionEndDate,
    ///577 - Original Maturity Date
    OriginalMaturityDate,
    ///578 - Date Referred to Attorney for Foreclosure
    DateReferredToAttorneyForForeclosure,
    ///579 - Planned Release
    PlannedRelease,
    ///580 - Actual Release
    ActualRelease,
    ///581 - Contract Period
    ContractPeriod,
    ///582 - Report Period
    ReportPeriod,
    ///583 - Suspension
    Suspension,
    ///584 - Reinstatement
    Reinstatement,
    ///585 - Report
    Report,
    ///586 - First Contact
    FirstContact,
    ///587 - Projected Foreclosure Sale Date
    ProjectedForeclosureSaleDate,
    ///589 - Date Assignment Filed for Record
    DateAssignmentFiledForRecord,
    ///590 - Date of Appraisal
    DateOfAppraisal,
    ///591 - Expiration Date of Extension to Assign
    ExpirationDateOfExtensionToAssign,
    ///592 - Date of Extension to Convey
    DateOfExtensionToConvey,
    ///593 - Date Hazard Insurance Policy Refused
    DateHazardInsurancePolicyRefused,
    ///594 - High Fabrication Release Authorization
    HighFabricationReleaseAuthorization,
    ///595 - High Raw Material Authorization
    HighRawMaterialAuthorization,
    ///596 - Material Change Notice
    MaterialChangeNotice,
    ///597 - Latest Delivery Date at Rail Ramp
    LatestDeliveryDateAtRailRamp,
    ///598 - Rejected
    Rejected,
    ///599 - Repayment Schedule Sent
    RepaymentScheduleSent,
    ///600 - As Of
    AsOf,
    ///601 - First Submission
    FirstSubmission,
    ///602 - Subsequent Submission
    SubsequentSubmission,
    ///603 - Renewal
    Renewal,
    ///604 - Withdrawn
    Withdrawn,
    ///606 - Certification Period Start
    CertificationPeriodStart,
    ///607 - Certification Revision
    CertificationRevision,
    ///608 - Continuous Coverage Date(s)
    Code608,
    ///609 - Prearranged Deal Match
    PrearrangedDealMatch,
    ///610 - Contingency End
    ContingencyEnd,
    ///611 - Oxygen Therapy Evaluation
    OxygenTherapyEvaluation,
    ///612 - Shut In
    ShutIn,
    ///613 - Allowable Effective
    AllowableEffective,
    ///614 - First Sales
    FirstSales,
    ///615 - Date Acquired
    DateAcquired,
    ///616 - Interviewer Signed
    InterviewerSigned,
    ///617 - Application Logged Date
    ApplicationLoggedDate,
    ///618 - Review Date
    ReviewDate,
    ///619 - Decision Date
    DecisionDate,
    ///620 - Previously Resided
    PreviouslyResided,
    ///621 - Reported
    Reported,
    ///622 - Checked
    Checked,
    ///623 - Settled
    Settled,
    ///624 - Presently Residing
    PresentlyResiding,
    ///625 - Employed in this Position
    EmployedInThisPosition,
    ///626 - Verified
    Verified,
    ///627 - Second Admission Date
    SecondAdmissionDate,
    ///629 - Account Opened
    AccountOpened,
    ///630 - Account Closed
    AccountClosed,
    ///631 - Property Acquired
    PropertyAcquired,
    ///632 - Property Built
    PropertyBuilt,
    ///633 - Employed in this Profession
    EmployedInThisProfession,
    ///634 - Next Review Date
    NextReviewDate,
    ///635 - Initial Contact Date
    InitialContactDate,
    ///636 - Date of Last Update
    DateOfLastUpdate,
    ///637 - Second Discharge Date
    SecondDischargeDate,
    ///638 - Date of Last Draw
    DateOfLastDraw,
    ///640 - Complaint
    Complaint,
    ///641 - Option
    Option,
    ///642 - Solicitation
    Solicitation,
    ///643 - Clause
    Clause,
    ///644 - Meeting
    Meeting,
    ///646 - Rental Period
    RentalPeriod,
    ///647 - Next Pay Increase
    NextPayIncrease,
    ///648 - Period Covered by Source Documents
    PeriodCoveredBySourceDocuments,
    ///649 - Document Due
    DocumentDue,
    ///650 - Court Notice
    CourtNotice,
    ///651 - Expected Funding Date
    ExpectedFundingDate,
    ///652 - Assignment Recorded
    AssignmentRecorded,
    ///653 - Case Reopened
    CaseReopened,
    ///655 - Previous Court Event
    PreviousCourtEvent,
    ///656 - Last Date to Object
    LastDateToObject,
    ///657 - Court Event
    CourtEvent,
    ///658 - Last Date to File a Claim
    LastDateToFileAClaim,
    ///659 - Case Converted
    CaseConverted,
    ///660 - Debt Incurred
    DebtIncurred,
    ///661 - Judgment
    Judgment,
    ///662 - Wages Start
    WagesStart,
    ///663 - Wages End
    WagesEnd,
    ///664 - Date Through Which Property Taxes Have Been Paid
    DateThroughWhichPropertyTaxesHaveBeenPaid,
    ///665 - Paid Through Date
    PaidThroughDate,
    ///666 - Date Paid
    DatePaid,
    ///667 - Anesthesia Administration
    AnesthesiaAdministration,
    ///668 - Price Protection
    PriceProtection,
    ///669 - Claim Incurred
    ClaimIncurred,
    ///670 - Book Entry Delivery
    BookEntryDelivery,
    ///671 - Rate Adjustment
    RateAdjustment,
    ///672 - Next Installment Due Date
    NextInstallmentDueDate,
    ///673 - Daylight Overdraft Time
    DaylightOverdraftTime,
    ///674 - Presentment Date
    PresentmentDate,
    ///675 - Negotiated Extension Date
    NegotiatedExtensionDate,
    ///681 - Remittance
    Remittance,
    ///682 - Security Rate Adjustment
    SecurityRateAdjustment,
    ///683 - Filing Period
    FilingPeriod,
    ///684 - Review Period End
    ReviewPeriodEnd,
    ///685 - Requested Settlement
    RequestedSettlement,
    ///686 - Last Screening
    LastScreening,
    ///687 - Confinement
    Confinement,
    ///688 - Arrested
    Arrested,
    ///689 - Convicted
    Convicted,
    ///690 - Interviewed
    Interviewed,
    ///691 - Last Visit
    LastVisit,
    ///692 - Recovery
    Recovery,
    ///693 - Time in U.S.
    TimeInUS,
    ///694 - Future Period
    FuturePeriod,
    ///695 - Previous Period
    PreviousPeriod,
    ///696 - Interest Paid To
    InterestPaidTo,
    ///697 - Date of Seizure
    DateOfSeizure,
    ///699 - Setoff
    Setoff,
    ///700 - Override Date for Settlement
    OverrideDateForSettlement,
    ///701 - Settlement Date (From Interline Settlement System (ISS) only)
    Code701,
    ///702 - Sending Road Time Stamp
    SendingRoadTimeStamp,
    ///703 - Retransmission Time Stamp
    RetransmissionTimeStamp,
    ///704 - Delivery Appointment Date and Time
    DeliveryAppointmentDateAndTime,
    ///705 - Interest Paid Through
    InterestPaidThrough,
    ///706 - Date Material Usage Suspended
    DateMaterialUsageSuspended,
    ///707 - Last Payment Made
    LastPaymentMade,
    ///708 - Past Due
    PastDue,
    ///709 - Analysis Month Ending
    AnalysisMonthEnding,
    ///710 - Date of Specification
    DateOfSpecification,
    ///711 - Date of Standard
    DateOfStandard,
    ///712 - Return to Work Part Time
    ReturnToWorkPartTime,
    ///713 - Paid-through Date for Salary Continuation
    PaidThroughDateForSalaryContinuation,
    ///714 - Paid-through Date for Vacation Pay
    PaidThroughDateForVacationPay,
    ///715 - Paid-through Date for Accrued Sick Pay
    PaidThroughDateForAccruedSickPay,
    ///716 - Appraisal Ordered
    AppraisalOrdered,
    ///717 - Date of Operation
    DateOfOperation,
    ///718 - Best Time to Call
    BestTimeToCall,
    ///719 - Verbal Report Needed
    VerbalReportNeeded,
    ///720 - Estimated Escrow Closing
    EstimatedEscrowClosing,
    ///721 - Permit Year
    PermitYear,
    ///722 - Remodeling Completed
    RemodelingCompleted,
    ///723 - Current Month Ending
    CurrentMonthEnding,
    ///724 - Previous Month Ending
    PreviousMonthEnding,
    ///725 - Cycle to Date
    CycleToDate,
    ///726 - Year to Date
    YearToDate,
    ///727 - On Hold
    OnHold,
    ///728 - Off Hold
    OffHold,
    ///729 - Facsimile Due By
    FacsimileDueBy,
    ///730 - Reporting Cycle Date
    ReportingCycleDate,
    ///731 - Last Paid Installment Date
    LastPaidInstallmentDate,
    ///732 - Claims Made
    ClaimsMade,
    ///733 - Date of Last Payment Received
    DateOfLastPaymentReceived,
    ///734 - Curtailment Date
    CurtailmentDate,
    ///736 - Pool Settlement
    PoolSettlement,
    ///737 - Next Interest Change Date
    NextInterestChangeDate,
    ///738 - Most Recent Hemoglobin or Hematocrit or Both
    MostRecentHemoglobinOrHematocritOrBoth,
    ///739 - Most Recent Serum Creatine
    MostRecentSerumCreatine,
    ///740 - Closed
    Closed,
    ///741 - Therapy
    Therapy,
    ///742 - Implantation
    Implantation,
    ///743 - Explantation
    Explantation,
    ///744 - Date Became Aware
    DateBecameAware,
    ///745 - First Marketed
    FirstMarketed,
    ///746 - Last Marketed
    LastMarketed,
    ///747 - New Due Date of First Payment to Principal and Interest
    NewDueDateOfFirstPaymentToPrincipalAndInterest,
    ///748 - New Maturity Date
    NewMaturityDate,
    ///749 - Current
    Current,
    ///750 - Expected Problem Resolution
    ExpectedProblemResolution,
    ///751 - Alternate Problem Resolution
    AlternateProblemResolution,
    ///752 - Fee Capitalization
    FeeCapitalization,
    ///753 - Interest Capitalization
    InterestCapitalization,
    ///754 - Next Payment Due
    NextPaymentDue,
    ///755 - Conversion to Repayment
    ConversionToRepayment,
    ///756 - End of Grace Period
    EndOfGracePeriod,
    ///757 - School Refund
    SchoolRefund,
    ///758 - Simple Interest Due
    SimpleInterestDue,
    ///759 - Date Practice Ceased
    DatePracticeCeased,
    ///760 - Printed
    Printed,
    ///761 - Date Practice Established
    DatePracticeEstablished,
    ///762 - Drop Action Date
    DropActionDate,
    ///764 - Most Recent Renewal
    MostRecentRenewal,
    ///765 - Original
    Original,
    ///766 - Outside Auditor's Report
    OutsideAuditorsReport,
    ///769 - Pre-certification Date
    PreCertificationDate,
    ///770 - Back on Market
    BackOnMarket,
    ///771 - Status
    Status,
    ///772 - Benefit Adjustment Start
    BenefitAdjustmentStart,
    ///773 - Off-Market
    OffMarket,
    ///774 - Tour
    Tour,
    ///775 - Benefit Adjustment End
    BenefitAdjustmentEnd,
    ///776 - Listing Received
    ListingReceived,
    ///777 - Benefit Adjustment Period
    BenefitAdjustmentPeriod,
    ///778 - Anticipated Closing
    AnticipatedClosing,
    ///779 - Last Publication
    LastPublication,
    ///780 - Sold Book Publication
    SoldBookPublication,
    ///781 - Occupancy
    Occupancy,
    ///782 - Contingency
    Contingency,
    ///783 - Percolation Test
    PercolationTest,
    ///784 - Septic Approval
    SepticApproval,
    ///785 - Title Transfer
    TitleTransfer,
    ///786 - Open House
    OpenHouse,
    ///787 - Benefit Credit Period
    BenefitCreditPeriod,
    ///788 - Benefit Transfer Period
    BenefitTransferPeriod,
    ///789 - Homestead
    Homestead,
    ///790 - Sanction
    Sanction,
    ///791 - Tail Coverage Begin
    TailCoverageBegin,
    ///792 - Tail Coverage End
    TailCoverageEnd,
    ///793 - Training Begin
    TrainingBegin,
    ///794 - Training End
    TrainingEnd,
    ///795 - Verification Received
    VerificationReceived,
    ///796 - Verification Sent
    VerificationSent,
    ///797 - State Residency Date
    StateResidencyDate,
    ///798 - Effective Date of the Routing File
    EffectiveDateOfTheRoutingFile,
    ///799 - Test Data Analysis
    TestDataAnalysis,
    ///800 - Midpoint of Performance
    MidpointOfPerformance,
    ///801 - Acquisition Date
    AcquisitionDate,
    ///802 - Date of Action
    DateOfAction,
    ///803 - Paid in Full
    PaidInFull,
    ///804 - Refinance
    Refinance,
    ///805 - Voluntary Termination
    VoluntaryTermination,
    ///806 - Customer Order
    CustomerOrder,
    ///807 - Stored
    Stored,
    ///808 - Selected
    Selected,
    ///809 - Posted
    Posted,
    ///810 - Document Received
    DocumentReceived,
    ///811 - Rebuilt
    Rebuilt,
    ///812 - Marriage
    Marriage,
    ///813 - Customs Entry Date
    CustomsEntryDate,
    ///814 - Payment Due Date
    PaymentDueDate,
    ///815 - Maturity Date
    MaturityDate,
    ///816 - Trade Date
    TradeDate,
    ///817 - Gallons Per Minute (GPM) Test Performed
    Code817,
    ///818 - British Thermal Unit (BTU) Test Performed
    Code818,
    ///819 - Last Accounts Filed at Public Registration Agency
    LastAccountsFiledAtPublicRegistrationAgency,
    ///820 - Real Estate Tax Year
    RealEstateTaxYear,
    ///821 - Final Reconciliation Value Estimate as of
    FinalReconciliationValueEstimateAsOf,
    ///822 - Map
    Map,
    ///823 - Opinion
    Opinion,
    ///824 - Version
    Version,
    ///825 - Original Due Date
    OriginalDueDate,
    ///826 - Incumbency Period
    IncumbencyPeriod,
    ///827 - Audience Deficiency Period
    AudienceDeficiencyPeriod,
    ///828 - Aired Date
    AiredDate,
    ///830 - Schedule
    Schedule,
    ///831 - Paid Through Date for Minimum Payment
    PaidThroughDateForMinimumPayment,
    ///832 - Paid Through Date for Total Payment
    PaidThroughDateForTotalPayment,
    ///840 - Election
    Election,
    ///841 - Engineering Data List
    EngineeringDataList,
    ///842 - Last Production
    LastProduction,
    ///843 - Not Before
    NotBefore,
    ///844 - Not After
    NotAfter,
    ///845 - Initial Claim
    InitialClaim,
    ///846 - Benefits Paid
    BenefitsPaid,
    ///847 - Wages Earned
    WagesEarned,
    ///848 - Adjusted Start
    AdjustedStart,
    ///849 - Adjusted End
    AdjustedEnd,
    ///850 - Revised Adjusted Start
    RevisedAdjustedStart,
    ///851 - Revised Adjusted End
    RevisedAdjustedEnd,
    ///853 - Field Test
    FieldTest,
    ///854 - Mortgage Note Date
    MortgageNoteDate,
    ///855 - Alternative Due Date
    AlternativeDueDate,
    ///856 - First Payment Change
    FirstPaymentChange,
    ///857 - First Rate Adjustment
    FirstRateAdjustment,
    ///858 - Alternate Base Period
    AlternateBasePeriod,
    ///859 - Prior Notice
    PriorNotice,
    ///860 - Appointment Effective
    AppointmentEffective,
    ///861 - Appointment Expiration
    AppointmentExpiration,
    ///862 - Company Termination
    CompanyTermination,
    ///863 - Continuing Education Requirement
    ContinuingEducationRequirement,
    ///864 - Distributor Effective
    DistributorEffective,
    ///865 - Distributor Termination
    DistributorTermination,
    ///866 - Examination
    Examination,
    ///867 - Incorporation Dissolution
    IncorporationDissolution,
    ///868 - Last Follow-up
    LastFollowUp,
    ///869 - License Effective
    LicenseEffective,
    ///870 - License Expiration
    LicenseExpiration,
    ///871 - License Renewal
    LicenseRenewal,
    ///872 - License Requested
    LicenseRequested,
    ///873 - Mailed
    Mailed,
    ///874 - Paperwork Mailed
    PaperworkMailed,
    ///875 - Previous Employment
    PreviousEmployment,
    ///876 - Previous Employment End
    PreviousEmploymentEnd,
    ///877 - Previous Employment Start
    PreviousEmploymentStart,
    ///878 - Previous Residence
    PreviousResidence,
    ///879 - Previous Residence End
    PreviousResidenceEnd,
    ///880 - Previous Residence Start
    PreviousResidenceStart,
    ///881 - Request
    Request,
    ///882 - Resident License Effective
    ResidentLicenseEffective,
    ///883 - Resident License Expiration
    ResidentLicenseExpiration,
    ///884 - State Termination
    StateTermination,
    ///885 - Texas Line Termination
    TexasLineTermination,
    ///900 - Acceleration
    Acceleration,
    ///901 - Adjusted Contestability
    AdjustedContestability,
    ///903 - Application Entry
    ApplicationEntry,
    ///904 - Approval/Offer
    ApprovalOffer,
    ///905 - Automatic Premium Loan
    AutomaticPremiumLoan,
    ///906 - Collection
    Collection,
    ///907 - Confinement End
    ConfinementEnd,
    ///908 - Confinement Start
    ConfinementStart,
    ///909 - Contestability
    Contestability,
    ///910 - Flat Extra End
    FlatExtraEnd,
    ///911 - Last Activity
    LastActivity,
    ///912 - Last Change
    LastChange,
    ///913 - Last Episode
    LastEpisode,
    ///914 - Last Meal
    LastMeal,
    ///915 - Loan
    Loan,
    ///916 - Application Status
    ApplicationStatus,
    ///917 - Maturity
    Maturity,
    ///918 - Medical Information Signature
    MedicalInformationSignature,
    ///919 - Medical Information System
    MedicalInformationSystem,
    ///920 - Note
    Note,
    ///921 - Offer Expiration
    OfferExpiration,
    ///922 - Original Receipt
    OriginalReceipt,
    ///923 - Placement
    Placement,
    ///924 - Placement Period Expiration
    PlacementPeriodExpiration,
    ///925 - Processing
    Processing,
    ///926 - Recapture
    Recapture,
    ///927 - Re-entry
    ReEntry,
    ///928 - Reissue
    Reissue,
    ///930 - Requalification
    Requalification,
    ///931 - Reinsurance Effective
    ReinsuranceEffective,
    ///932 - Reservation of Facility
    ReservationOfFacility,
    ///933 - Settlement Status
    SettlementStatus,
    ///934 - Table Rating End
    TableRatingEnd,
    ///935 - Termination of Facility
    TerminationOfFacility,
    ///936 - Treatment
    Treatment,
    ///937 - Department of Labor Wage Determination Date
    DepartmentOfLaborWageDeterminationDate,
    ///938 - Order
    Order,
    ///939 - Resolved
    Resolved,
    ///940 - Execution Date
    ExecutionDate,
    ///941 - Capitation Period Start
    CapitationPeriodStart,
    ///942 - Capitation Period End
    CapitationPeriodEnd,
    ///943 - Last Date for a Government Agency to File a Claim
    LastDateForAGovernmentAgencyToFileAClaim,
    ///944 - Adjustment Period
    AdjustmentPeriod,
    ///945 - Activity
    Activity,
    ///946 - Mail By
    MailBy,
    ///947 - Preparation
    Preparation,
    ///948 - Payment Initiated
    PaymentInitiated,
    ///949 - Payment Effective
    PaymentEffective,
    ///950 - Application
    Application,
    ///951 - Reclassification
    Reclassification,
    ///952 - Reclassification (Exit Date)
    Code952,
    ///953 - Post-Reclassification
    PostReclassification,
    ///954 - Post-Reclassification (First Report Card)
    Code954,
    ///955 - Post-Reclassification (First Semi-annual)
    Code955,
    ///956 - Post-Reclassification (Second Semi-annual)
    Code956,
    ///957 - Post-Reclassification (End of Second Year)
    Code957,
    ///960 - Adjusted Death Benefit
    AdjustedDeathBenefit,
    ///961 - Anniversary
    Anniversary,
    ///962 - Annuitization
    Annuitization,
    ///963 - Annuity Commencement Date
    AnnuityCommencementDate,
    ///964 - Bill
    Bill,
    ///965 - Calendar Anniversary
    CalendarAnniversary,
    ///966 - Contract Mailed
    ContractMailed,
    ///967 - Early Withdrawal
    EarlyWithdrawal,
    ///968 - Fiscal Anniversary
    FiscalAnniversary,
    ///969 - Income
    Income,
    ///970 - Initial Premium
    InitialPremium,
    ///971 - Initial Premium Effective
    InitialPremiumEffective,
    ///972 - Last Premium Effective
    LastPremiumEffective,
    ///973 - Minimum Required Distribution
    MinimumRequiredDistribution,
    ///974 - Next Anniversary
    NextAnniversary,
    ///975 - Notice
    Notice,
    ///976 - Notification of Death
    NotificationOfDeath,
    ///977 - Partial Annuitization
    PartialAnnuitization,
    ///978 - Plan Anniversary
    PlanAnniversary,
    ///979 - Policy Surrender
    PolicySurrender,
    ///980 - Prior Contract Anniversary
    PriorContractAnniversary,
    ///981 - Prior Contract Issue
    PriorContractIssue,
    ///982 - Signature Received
    SignatureReceived,
    ///983 - Tax
    Tax,
    ///984 - Benefit Period
    BenefitPeriod,
    ///985 - Month to Date
    MonthToDate,
    ///986 - Semiannual Ending
    SemiannualEnding,
    ///987 - Surrender
    Surrender,
    ///988 - Plan of Treatment Period
    PlanOfTreatmentPeriod,
    ///989 - Prior Hospitalization Date(s) Related to Current Service(s)
    Code989,
    ///990 - Original Name Change
    OriginalNameChange,
    ///992 - Date Requested
    DateRequested,
    ///993 - Request for Quotation
    RequestForQuotation,
    ///994 - Quote
    Quote,
    ///995 - Recorded Date
    RecordedDate,
    ///996 - Required Delivery
    RequiredDelivery,
    ///997 - Quote to be Received By
    QuoteToBeReceivedBy,
    ///998 - Continuation of Pay Start Date
    ContinuationOfPayStartDate,
    ///999 - Document Date
    DocumentDate,
    ///AA1 - Estimated Point of Arrival
    EstimatedPointOfArrival,
    ///AA2 - Estimated Point of Discharge
    EstimatedPointOfDischarge,
    ///AA3 - Cancel After, Ex Country
    CodeAA3,
    ///AA4 - Cancel After, Ex Factory
    CodeAA4,
    ///AA5 - Do Not Ship Before, Ex Country
    CodeAA5,
    ///AA6 - Do Not Ship Before, Ex Factory
    CodeAA6,
    ///AA7 - Final Scheduled Payment
    FinalScheduledPayment,
    ///AA8 - Actual Discharge
    ActualDischarge,
    ///AA9 - Address Period
    AddressPeriod,
    ///AAA - Arrival in Country
    ArrivalInCountry,
    ///AAB - Citation
    Citation,
    ///AAC - Suspension Effective
    SuspensionEffective,
    ///AAD - Crime
    Crime,
    ///AAE - Discharge - Planned
    DischargePlanned,
    ///AAF - Draft
    Draft,
    ///AAG - Due Date
    DueDate,
    ///AAH - Event
    Event,
    ///AAI - First Involvement
    FirstInvolvement,
    ///AAJ - Guarantee Period
    GuaranteePeriod,
    ///AAK - Income Increase Period
    IncomeIncreasePeriod,
    ///AAL - Installment Date
    InstallmentDate,
    ///AAM - Last Civilian Flight
    LastCivilianFlight,
    ///AAN - Last Flight
    LastFlight,
    ///AAO - Last Insurance Medical
    LastInsuranceMedical,
    ///AAP - Last Military Flight
    LastMilitaryFlight,
    ///AAQ - Last Physical
    LastPhysical,
    ///AAR - License
    License,
    ///AAS - Medical Certificate
    MedicalCertificate,
    ///AAT - Medication
    Medication,
    ///AAU - Net Worth Date
    NetWorthDate,
    ///AAV - Next Activity
    NextActivity,
    ///AAW - Ownership Change
    OwnershipChange,
    ///AAX - Ownership Period
    OwnershipPeriod,
    ///AAY - Rate Date
    RateDate,
    ///AAZ - Requested Contract
    RequestedContract,
    ///AB1 - Requested Offer
    RequestedOffer,
    ///AB2 - Sales Period
    SalesPeriod,
    ///AB3 - Tax Year
    TaxYear,
    ///AB4 - Time Period
    TimePeriod,
    ///AB5 - Travel
    Travel,
    ///AB6 - Treatment End
    TreatmentEnd,
    ///AB7 - Treatment Start
    TreatmentStart,
    ///AB8 - Trust
    Trust,
    ///AB9 - Worst Time to Call
    WorstTimeToCall,
    ///ABA - Registration
    Registration,
    ///ABB - Revoked
    Revoked,
    ///ABC - Estimated Date of Birth
    EstimatedDateOfBirth,
    ///ABD - Last Annual Report
    LastAnnualReport,
    ///ABE - Legal Action Started
    LegalActionStarted,
    ///ABF - Lien
    Lien,
    ///ABG - Payment Period
    PaymentPeriod,
    ///ABH - Profit Period
    ProfitPeriod,
    ///ABI - Registered
    Registered,
    ///ABK - Consolidated
    Consolidated,
    ///ABL - Board of Directors Not Authorized As Of
    BoardOfDirectorsNotAuthorizedAsOf,
    ///ABM - Board of Directors Incomplete As Of
    BoardOfDirectorsIncompleteAsOf,
    ///ABN - Manager Not Registered As Of
    ManagerNotRegisteredAsOf,
    ///ABO - Citizenship Change
    CitizenshipChange,
    ///ABP - Participation
    Participation,
    ///ABQ - Capitalization
    Capitalization,
    ///ABR - Registration of Board of Directors
    RegistrationOfBoardOfDirectors,
    ///ABS - Ceased Operations
    CeasedOperations,
    ///ABT - Satisfied
    Satisfied,
    ///ABU - Terms Met
    TermsMet,
    ///ABV - Asset Documentation Expiration
    AssetDocumentationExpiration,
    ///ABW - Credit Documentation Expiration
    CreditDocumentationExpiration,
    ///ABX - Income Documentation Expiration
    IncomeDocumentationExpiration,
    ///ABY - Product Held Until
    ProductHeldUntil,
    ///ACA - Immigration Date
    ImmigrationDate,
    ///ACB - Estimated Immigration Date
    EstimatedImmigrationDate,
    ///ACC - Current Disability Period Start
    CurrentDisabilityPeriodStart,
    ///ACD - Current Disability Period End
    CurrentDisabilityPeriodEnd,
    ///ACE - Current Disability Period Last Day Worked
    CurrentDisabilityPeriodLastDayWorked,
    ///ACF - Benefit Type Gross Weekly Amount Effective
    BenefitTypeGrossWeeklyAmountEffective,
    ///ACG - Benefit Type Net Weekly Amount Effective
    BenefitTypeNetWeeklyAmountEffective,
    ///ACH - Benefit Type Period Start
    BenefitTypePeriodStart,
    ///ACI - Benefit Type Period End
    BenefitTypePeriodEnd,
    ///ACJ - Benefit Debit Start
    BenefitDebitStart,
    ///ACK - Acknowledgment
    Acknowledgment,
    ///ACL - Benefit Debit End
    BenefitDebitEnd,
    ///ACM - Benefit Credit Start
    BenefitCreditStart,
    ///ACN - Benefit Credit End
    BenefitCreditEnd,
    ///ACO - Benefit Transfer Start
    BenefitTransferStart,
    ///ACP - Benefit Transfer End
    BenefitTransferEnd,
    ///ACQ - Wage Effective
    WageEffective,
    ///ACR - Full Denial Effective
    FullDenialEffective,
    ///ACS - Full Denial Rescission
    FullDenialRescission,
    ///ACT - Payment Issue
    PaymentIssue,
    ///ACU - Payment Period Start
    PaymentPeriodStart,
    ///ACV - Payment Period End
    PaymentPeriodEnd,
    ///ACW - Employer Reported Injury To Claim Administrator
    EmployerReportedInjuryToClaimAdministrator,
    ///ACX - Survey Year
    SurveyYear,
    ///ACZ - Controvert Date
    ControvertDate,
    ///ADA - Billed Through
    BilledThrough,
    ///ADB - Business Control Change
    BusinessControlChange,
    ///ADC - Court Registration
    CourtRegistration,
    ///ADD - Annual Report Due
    AnnualReportDue,
    ///ADE - Claim Notification Received
    ClaimNotificationReceived,
    ///ADF - Conversion Privilege End
    ConversionPrivilegeEnd,
    ///ADG - Dividend Applied
    DividendApplied,
    ///ADH - In-force
    InForce,
    ///ADI - Paid-up
    PaidUp,
    ///ADJ - Premium Change
    PremiumChange,
    ///ADK - Policy Effective on or before
    PolicyEffectiveOnOrBefore,
    ///ADL - Asset and Liability Schedule
    AssetAndLiabilitySchedule,
    ///ADM - Annual Report Mailed
    AnnualReportMailed,
    ///ADN - Policy Effective on or after
    PolicyEffectiveOnOrAfter,
    ///ADR - Annual Report Filed
    AnnualReportFiled,
    ///ADS - Audit Period After Split Date
    AuditPeriodAfterSplitDate,
    ///ADT - Audit Period Prior to Split Date
    AuditPeriodPriorToSplitDate,
    ///ADU - Exposure Source Period
    ExposureSourcePeriod,
    ///ADV - Subcontractor Period of Hire
    SubcontractorPeriodOfHire,
    ///ADW - Divorce
    Divorce,
    ///ADX - Power of Attorney
    PowerOfAttorney,
    ///ADY - Uniform Gifts to Minors Account Established
    UniformGiftsToMinorsAccountEstablished,
    ///AEA - Medicare Part A Eligibility Begin Date
    MedicarePartAEligibilityBeginDate,
    ///AEB - Medicare Part A Eligibility End Date
    MedicarePartAEligibilityEndDate,
    ///AEC - Medicare Part A Coverage Effective Date
    MedicarePartACoverageEffectiveDate,
    ///AED - Medicare Part A Termination Date
    MedicarePartATerminationDate,
    ///AEE - Medicare Part B Eligibility Begin Date
    MedicarePartBEligibilityBeginDate,
    ///AEF - Medicare Part B Eligibility End Date
    MedicarePartBEligibilityEndDate,
    ///AEG - Medicare Part B Coverage Effective Date
    MedicarePartBCoverageEffectiveDate,
    ///AEH - Medicare Part B Termination Date
    MedicarePartBTerminationDate,
    ///AEI - Loading Period
    LoadingPeriod,
    ///AEK - Date on which Assets Judged Insufficient to Pay Creditors
    DateOnWhichAssetsJudgedInsufficientToPayCreditors,
    ///AEL - Employees Temporarily Laid Off Begin Period
    EmployeesTemporarilyLaidOffBeginPeriod,
    ///AEM - Employees Temporarily Laid Off End Period
    EmployeesTemporarilyLaidOffEndPeriod,
    ///AEN - First Published
    FirstPublished,
    ///AEO - Forecast Period Start
    ForecastPeriodStart,
    ///AEP - Forecast Period End
    ForecastPeriodEnd,
    ///AEQ - Investigation Start
    InvestigationStart,
    ///AER - Investigation End
    InvestigationEnd,
    ///AES - Last Published
    LastPublished,
    ///AET - Latest Balance Sheet
    LatestBalanceSheet,
    ///AEU - Share Price
    SharePrice,
    ///AEV - Stop Distribution
    StopDistribution,
    ///AEW - Maximum Credit Date
    MaximumCreditDate,
    ///AEX - Founding Date
    FoundingDate,
    ///AEY - Repayment Plan Start Date
    RepaymentPlanStartDate,
    ///AFA - Medicare Part D Eligibility Begin Date
    MedicarePartDEligibilityBeginDate,
    ///AFB - Medicare Part D Eligibility End Date
    MedicarePartDEligibilityEndDate,
    ///AFC - Medicare Part D Coverage Effective Date
    MedicarePartDCoverageEffectiveDate,
    ///AFD - Medicare Part D Termination Date
    MedicarePartDTerminationDate,
    ///ARD - Annual Report Delinquency
    AnnualReportDelinquency,
    ///AWH - Withheld Date
    WithheldDate,
    ///BAA - Compliance Audit
    ComplianceAudit,
    ///BAB - Contractor Safety Performance Evaluation
    ContractorSafetyPerformanceEvaluation,
    ///BAC - Contractor Safety Procedures Review
    ContractorSafetyProceduresReview,
    ///BAD - Date of Equipment Inspection
    DateOfEquipmentInspection,
    ///BAE - Date of Safety Inspection
    DateOfSafetyInspection,
    ///BAF - Employees Participation Plan Review
    EmployeesParticipationPlanReview,
    /**BAG - Expected Completion of Changes Resulting from
Compliance Audit*/
    CodeBAG,
    /**BAH - Expected Completion of Changes Resulting from
Process Hazard Analysis*/
    CodeBAH,
    ///BAI - Expected Completion of Changes Resulting from Hazard Review
    ExpectedCompletionOfChangesResultingFromHazardReview,
    ///BAJ - Hazard Review Completion
    HazardReviewCompletion,
    ///BAK - Hot Work Permit Procedures Review
    HotWorkPermitProceduresReview,
    ///BAL - Investigation
    Investigation,
    ///BAM - Maintenance Procedures Review
    MaintenanceProceduresReview,
    ///BAN - Management of Change Procedures Review
    ManagementOfChangeProceduresReview,
    ///BAO - Operating Procedures Review
    OperatingProceduresReview,
    ///BAP - Safety Information Review
    SafetyInformationReview,
    ///BAQ - Training
    Training,
    ///BAR - Training Program Review
    TrainingProgramReview,
    ///BED - Billback End Date
    BillbackEndDate,
    ///BEE - Program Performance End Date
    ProgramPerformanceEndDate,
    ///BES - Program Performance Start Date
    ProgramPerformanceStartDate,
    ///BGP - Beginning of Grace Period
    BeginningOfGracePeriod,
    ///BIA - Billing Activities
    BillingActivities,
    ///BIP - Beginning of Interest Paid After Claim
    BeginningOfInterestPaidAfterClaim,
    ///BSD - Billback Start Date
    BillbackStartDate,
    ///CAD - Changed Accounting Date
    ChangedAccountingDate,
    ///CCR - Customs Cargo Release
    CustomsCargoRelease,
    ///CDD - Contract Definitization Date
    ContractDefinitizationDate,
    ///CDE, CSE - Campaign End Date
    CampaignEndDate,
    ///CDS, CSD - Campaign Start Date
    CampaignStartDate,
    ///CDT - Maintenance Comment
    MaintenanceComment,
    ///CEA - Formation
    Formation,
    ///CEB - Continuance
    Continuance,
    ///CEC - Merger
    Merger,
    ///CED - Year Due
    YearDue,
    ///CEE - Next Annual Meeting
    NextAnnualMeeting,
    ///CEF - End of Last Fiscal Year
    EndOfLastFiscalYear,
    ///CEH - Year Beginning
    YearBeginning,
    ///CEJ - Started Doing Business
    StartedDoingBusiness,
    ///CEK - Sworn and Subscribed
    SwornAndSubscribed,
    ///CEL - Calendar Year
    CalendarYear,
    ///CEM - Asset
    Asset,
    ///CEN - Inactivity
    Inactivity,
    ///CEO - High Capital Year
    HighCapitalYear,
    ///CLO - Closing Date of First Balance Sheet
    ClosingDateOfFirstBalanceSheet,
    ///CLU - Closed Until
    ClosedUntil,
    ///COM - Compliance
    Compliance,
    ///CON - Converted into Holding Company
    ConvertedIntoHoldingCompany,
    ///COS - Care of Supplies in Storage Inspection Date
    CareOfSuppliesInStorageInspectionDate,
    ///CPA - Consumer Product Availability Date
    ConsumerProductAvailabilityDate,
    ///CPD - Consumer Product Information Publication Date
    ConsumerProductInformationPublicationDate,
    ///CPE - Consumer Product Variant End Effective Date
    ConsumerProductVariantEndEffectiveDate,
    ///CPF - Consumer Product Variant Discontinued Date
    ConsumerProductVariantDiscontinuedDate,
    ///CPG - Consumer Product Variant Cancelled Date
    ConsumerProductVariantCancelledDate,
    ///CPS - Consumer Product Variant Start Effective Date
    ConsumerProductVariantStartEffectiveDate,
    ///CRV - Claim Revised
    ClaimRevised,
    ///CUR - Current List
    CurrentList,
    ///CVD - Community Visibility
    CommunityVisibility,
    ///DAF - Account Frozen
    AccountFrozen,
    ///DDO - Declaration
    Declaration,
    ///DEE - Deed Not Available
    DeedNotAvailable,
    ///DEL - Delete
    Delete,
    ///DET - Detrimental Information Received
    DetrimentalInformationReceived,
    ///DFF - Deferral
    Deferral,
    ///DFS - Departure From Specification
    DepartureFromSpecification,
    ///DIL - Deed In Lieu (DIL) Approved
    CodeDIL,
    ///DIP - Delayed Interest Paid Through
    DelayedInterestPaidThrough,
    ///DIS - Disposition
    Disposition,
    ///DLC - Date of Last Contact
    DateOfLastContact,
    ///DOA - Date of Abandonment
    DateOfAbandonment,
    ///DOD - Date of Delinquency
    DateOfDelinquency,
    ///DOI - Delivery Order Issued
    DeliveryOrderIssued,
    ///DOR - Repossession
    Repossession,
    ///DSP - Disposal
    Disposal,
    ///DTC - Deed and Title Received
    DeedAndTitleReceived,
    ///DTD - Technical Data Supply By
    TechnicalDataSupplyBy,
    ///DTQ - Deed and Title Requested
    DeedAndTitleRequested,
    ///E01 - Tenure Decision
    TenureDecision,
    ///E02 - Most Recent Position Change
    MostRecentPositionChange,
    ///E03 - Fee Payment
    FeePayment,
    ///E04 - Start Date for Continuous Employment
    StartDateForContinuousEmployment,
    ///E05 - Start Date for Current Position
    StartDateForCurrentPosition,
    ///E06 - Start Date for Original Position
    StartDateForOriginalPosition,
    ///E07 - Fiscal Year
    FiscalYear,
    ///EAD - End Availability Date
    EndAvailabilityDate,
    ///ECD - Estimated Construction Date
    EstimatedConstructionDate,
    ///ECF - Estimated Completion - First Prior Month
    EstimatedCompletionFirstPriorMonth,
    ///ECS - Estimated Completion - Second Prior Month
    EstimatedCompletionSecondPriorMonth,
    ///ECT - Estimated Completion - Third Prior Month
    EstimatedCompletionThirdPriorMonth,
    ///EDA - Affirmed
    Affirmed,
    ///EDB - Auction
    Auction,
    ///EDC - Authorized
    Authorized,
    ///EDD - Contribution
    Contribution,
    ///EDE - Executed
    Executed,
    ///EDF - Forgiven
    Forgiven,
    ///EDG - Presented
    Presented,
    ///EDH - Legislative Session
    LegislativeSession,
    ///EDI - Organized
    Organized,
    ///EDJ - Pledged
    Pledged,
    ///EDK - Primary Election
    PrimaryElection,
    ///EDL - Qualified
    Qualified,
    ///EDM - Refunded
    Refunded,
    ///EDN - Rescinded
    Rescinded,
    ///EDO - Restructured From
    RestructuredFrom,
    ///EDP - Vote
    Vote,
    ///EKD - Employer Knowledge of the Disability
    EmployerKnowledgeOfTheDisability,
    ///EMM - End Date Maximum Buying Quantity
    EndDateMaximumBuyingQuantity,
    ///END - End Date Minimum Buying Quantity
    EndDateMinimumBuyingQuantity,
    ///EPP - Estimate Preparation
    EstimatePreparation,
    ///ESC - Estimate Comment
    EstimateComment,
    ///ESD - Effective Start Date
    EffectiveStartDate,
    ///ESF - Estimated Start - First Prior Month
    EstimatedStartFirstPriorMonth,
    ///ESS - Estimated Start - Second Prior Month
    EstimatedStartSecondPriorMonth,
    ///EST - Estimated Start - Third Prior Month
    EstimatedStartThirdPriorMonth,
    ///ETP - Earliest Filing Period
    EarliestFilingPeriod,
    ///EXO - Exposure
    Exposure,
    ///EXP - Export
    Export,
    ///FAC - Facility Latest Billing Action
    FacilityLatestBillingAction,
    ///FEB - Facility Earliest Billing Action
    FacilityEarliestBillingAction,
    ///FFI - Financial Information
    Financial,
    ///FFO - First Order
    FirstOrder,
    ///FIA - Final Interest Accrual
    FinalInterestAccrual,
    ///FPE - Funding Period - End
    FundingPeriodEnd,
    ///FPS - Funding Period - Start
    FundingPeriodStart,
    ///FSD - First Available Ship Date
    FirstAvailableShipDate,
    ///FSE - Free Service Call End Date
    FreeServiceCallEndDate,
    ///FSS - Free Service Call Start Date
    FreeServiceCallStartDate,
    ///GRD - Graduated
    Graduated,
    ///HHD - Home Health Date of Earliest Billable Action
    HomeHealthDateOfEarliestBillableAction,
    ///HHE - Home Health Episode
    HomeHealthEpisode,
    ///HHL - Home Health Date of Latest Billable Action
    HomeHealthDateOfLatestBillableAction,
    ///HTD - Host Train Date
    HostTrainDate,
    ///ICF - Converted to Electronic Date
    ConvertedToElectronicDate,
    ///IDG - Insolvency Discharge Granted
    InsolvencyDischargeGranted,
    ///IFH - Initial Federal Housing Authority Claim Payment
    InitialFederalHousingAuthorityClaimPayment,
    ///III - Incorporation
    Incorporation,
    ///ILU - Image Last Update Date
    ImageLastUpdateDate,
    ///IMB - Imbalance Period
    ImbalancePeriod,
    ///IMP - Import
    Import,
    ///INC - Incident
    Incident,
    ///INT - Inactive Until
    InactiveUntil,
    ///IPS - Interest on Presale Start
    InterestOnPresaleStart,
    ///IVA - Initial Veterans Administration Claim Payment
    InitialVeteransAdministrationClaimPayment,
    ///KEV - Key Event Fiscal Year
    KeyEventFiscalYear,
    ///KEW - Key Event Calendar Year
    KeyEventCalendarYear,
    ///LAM - Last Annual Meeting
    LastAnnualMeeting,
    ///LAS - Last Check for Balance Sheet Update
    LastCheckForBalanceSheetUpdate,
    ///LCC - Last Capital Change
    LastCapitalChange,
    ///LEA - Letter of Agreement
    LetterOfAgreement,
    ///LEL - Letter of Liability
    LetterOfLiability,
    ///LIQ - Liquidation
    Liquidation,
    ///LLP - Low Period
    LowPeriod,
    ///LOG - Equipment Log Entry
    EquipmentLogEntry,
    ///LPC - List Price Change
    ListPriceChange,
    ///LSC - Legal Structure Change
    LegalStructureChange,
    ///LSD - Last Submission Date
    LastSubmissionDate,
    ///LTP - Latest Filing Period
    LatestFilingPeriod,
    ///MRR - Meter Reading
    MeterReading,
    ///MSD - Latest Material Safety Data Sheet Date
    LatestMaterialSafetyDataSheetDate,
    ///NAM - Present Name
    PresentName,
    ///NFD - Negotiated Finish
    NegotiatedFinish,
    ///NOD - Notice of Delinquency (NOD) Received
    CodeNOD,
    ///NRG - Not Registered
    NotRegistered,
    ///NSD - Negotiated Start
    NegotiatedStart,
    ///OCD - Organic Certification Date
    OrganicCertificationDate,
    ///ORG - Original List
    OriginalList,
    ///PBC - Present Control
    PresentControl,
    ///PCP - Primary Coverage Claim Paid
    PrimaryCoverageClaimPaid,
    ///PCS - Primary Coverage Claim Submission
    PrimaryCoverageClaimSubmission,
    ///PCT - Price Changes Allowed From Date
    PriceChangesAllowedFromDate,
    ///PCU - Price Changes Allowed To Date
    PriceChangesAllowedToDate,
    ///PDA - Product Discontinued but Still Available
    ProductDiscontinuedButStillAvailable,
    ///PDE - Partial Denial Effective
    PartialDenialEffective,
    ///PDR - Partial Denial Rescission
    PartialDenialRescission,
    ///PDS - Correct Program Start Date
    CorrectProgramStartDate,
    ///PDT - Correct Program End Date
    CorrectProgramEndDate,
    ///PDU - Correct Contract Start Date
    CorrectContractStartDate,
    ///PDV - Privilege Details Verification
    PrivilegeDetailsVerification,
    ///PDW - Correct Contract End Date
    CorrectContractEndDate,
    ///PED - Program End Date
    ProgramEndDate,
    ///PIS - Product Image Start Date
    ProductImageStartDate,
    ///PLS - Present Legal Structure
    PresentLegalStructure,
    ///PME - Packaging Material Effective Date
    PackagingMaterialEffectiveDate,
    ///PPC - Pool Policy Claim Submission
    PoolPolicyClaimSubmission,
    ///PPD - Post Paid Date
    PostPaidDate,
    ///PPP - Peak Period
    PeakPeriod,
    ///PRD - Previously Reported Date of Birth
    PreviouslyReportedDateOfBirth,
    ///PRR - Presented to Receivers
    PresentedToReceivers,
    ///PSA - Property Sale Approved
    PropertySaleApproved,
    ///PSC - Property Sale Closed
    PropertySaleClosed,
    ///PSD - Program Start Date
    ProgramStartDate,
    ///PSF - Property Sale Confirmation
    PropertySaleConfirmation,
    ///PTD - Paid To Date
    PaidToDate,
    ///PTO - Planned Obsolescence Item
    PlannedObsolescenceItem,
    ///PUD - Pick-up Date
    PickUpDate,
    ///RAP - Receiver Appointed
    ReceiverAppointed,
    ///REM - Remanufacture Date
    RemanufactureDate,
    ///RES - Resigned
    Resigned,
    ///RFD - Requested Finish
    RequestedFinish,
    ///RFF - Recovery Finish
    RecoveryFinish,
    ///RFO - Referred From
    ReferredFrom,
    ///RNT - Rent Survey
    RentSurvey,
    ///RRM - Received in the Mail
    ReceivedInTheMail,
    ///RRT - Revocation
    Revocation,
    ///RSD - Requested Start
    RequestedStart,
    ///RSS - Recovery Start
    RecoveryStart,
    ///RTO - Referred To
    ReferredTo,
    ///SCV - Social Security Claims Verification
    SocialSecurityClaimsVerification,
    ///SDD - Sole Directorship Date
    SoleDirectorshipDate,
    ///SDM - Start Date Maximum Buying Quantity
    StartDateMaximumBuyingQuantity,
    ///SFH - Subsequent Federal Housing Authority Claim Payment
    SubsequentFederalHousingAuthorityClaimPayment,
    ///SMB - Start Date Minimum Buying Quantity
    StartDateMinimumBuyingQuantity,
    ///SPD - Initial Support Date
    InitialSupportDate,
    ///SPE - Suggested Retail Price Effective Date
    SuggestedRetailPriceEffectiveDate,
    ///STN - Transition
    Transition,
    ///SVA - Subsequent Veterans Administration Claim Payment
    SubsequentVeteransAdministrationClaimPayment,
    ///TSR - Trade Style Registered
    TradeStyleRegistered,
    ///TSS - Trial Started
    TrialStarted,
    ///TST - Trial Set
    TrialSet,
    ///TTD - Tenant Train Date
    TenantTrainDate,
    ///VAT - Value Added Tax (VAT) Claims Verification
    CodeVAT,
    ///VLU - Valid Until
    ValidUntil,
    ///W01 - Sample Collected
    SampleCollected,
    ///W02 - Status Change
    StatusChange,
    ///W03 - Construction Start
    ConstructionStart,
    ///W05 - Recompletion
    Recompletion,
    ///W06 - Last Logged
    LastLogged,
    ///W07 - Well Log Run
    WellLogRun,
    ///W08 - Surface Casing Authority Approval
    SurfaceCasingAuthorityApproval,
    ///W09 - Reached Total Depth
    ReachedTotalDepth,
    ///W10 - Spacing Order Unit Assigned
    SpacingOrderUnitAssigned,
    ///W11 - Rig Arrival
    RigArrival,
    ///W12 - Location Exception Order Number Assigned
    LocationExceptionOrderNumberAssigned,
    ///W13 - Sidetracked Wellbore
    SidetrackedWellbore,
    ///WAA - Time Employee Began Work
    TimeEmployeeBeganWork,
    ///WAY - Waybill
    Waybill,
    ///XX1 - Order Day
    OrderDay,
    ///XX2 - Delivery Day
    DeliveryDay,
    ///XX3 - Order Cut-Off Time
    OrderCutOffTime,
    ///XX5 - Active Item
    ActiveItem,
    ///XX6 - Mature Item
    MatureItem,
    ///XX7 - Introductory Item
    IntroductoryItem,
    ///XX8 - Obsolete Item
    ObsoleteItem,
    ///Y11 - Best Before Date
    BestBeforeDate,
    ///Y12 - Harvest Date
    HarvestDate,
    ///YXX - Programmed Fiscal Year
    ProgrammedFiscalYear,
    ///YXY - Programmed Calendar Year
    ProgrammedCalendarYear,
    ///ZZZ - Mutually Defined
    MutuallyDefined,
}
impl DateTimeQualifier {
    pub fn code(&self) -> &str {
        {
            use DateTimeQualifier::*;
            match self {
                CancelAfter => "001",
                DeliveryRequested => "002",
                Invoice => "003",
                PurchaseOrder => "004",
                Sailing => "005",
                Sold => "006",
                Effective => "007",
                PurchaseOrderReceived => "008",
                Process => "009",
                RequestedShip => "010",
                Shipped => "011",
                TermsDiscountDue => "012",
                TermsNetDue => "013",
                DeferredPayment => "014",
                PromotionStart => "015",
                PromotionEnd => "016",
                EstimatedDelivery => "017",
                Available => "018",
                Unloaded => "019",
                Check => "020",
                ChargeBack => "021",
                FreightBill => "022",
                PromotionOrderStart => "023",
                PromotionOrderEnd => "024",
                PromotionShipStart => "025",
                PromotionShipEnd => "026",
                PromotionRequestedDeliveryStart => "027",
                PromotionRequestedDeliveryEnd => "028",
                PromotionPerformanceStart => "029",
                PromotionPerformanceEnd => "030",
                PromotionInvoicePerformanceStart => "031",
                PromotionInvoicePerformanceEnd => "032",
                PromotionFloorStockProtectStart => "033",
                PromotionFloorStockProtectEnd => "034",
                Delivered => "035",
                Expiration => "036",
                ShipNotBefore => "037",
                ShipNoLater => "038",
                ShipWeekOf => "039",
                Code040 => "040",
                Code041 => "041",
                Superseded => "042",
                Publication => "043",
                SettlementDateAsSpecifiedByTheOriginator => "044",
                EndorsementDate => "045",
                FieldFailure => "046",
                FunctionalTest => "047",
                SystemTest => "048",
                PrototypeTest => "049",
                Received => "050",
                CumulativeQuantityStart => "051",
                CumulativeQuantityEnd => "052",
                BuyersLocal => "053",
                SellersLocal => "054",
                Confirmed => "055",
                EstimatedPortOfEntry => "056",
                ActualPortOfEntry => "057",
                CustomsClearance => "058",
                InlandShip => "059",
                EngineeringChangeLevel => "060",
                CancelIfNotDeliveredBy => "061",
                Blueprint => "062",
                DoNotDeliverAfter => "063",
                DoNotDeliverBefore => "064",
                Code065 => "065",
                Code066 => "066",
                CurrentScheduleDelivery => "067",
                CurrentScheduleShip => "068",
                PromisedForDelivery => "069",
                Code070 => "070",
                Code071 => "071",
                Code072 => "072",
                Code073 => "073",
                Code074 => "074",
                Code075 => "075",
                Code076 => "076",
                Code077 => "077",
                Code078 => "078",
                PromisedForShipment => "079",
                Code080 => "080",
                Code081 => "081",
                Code082 => "082",
                Code083 => "083",
                Code084 => "084",
                Code085 => "085",
                Code086 => "086",
                Code087 => "087",
                Code088 => "088",
                Inquiry => "089",
                ReportStart => "090",
                ReportEnd => "091",
                ContractEffective => "092",
                ContractExpiration => "093",
                Manufacture => "094",
                BillOfLading => "095",
                Discharge => "096",
                TransactionCreation => "097",
                Code098 => "098",
                Code099 => "099",
                NoShippingScheduleEstablishedAsOf => "100",
                NoProductionScheduleEstablishedAsOf => "101",
                Issue => "102",
                Award => "103",
                SystemSurvey => "104",
                QualityRating => "105",
                RequiredBy => "106",
                Deposit => "107",
                Postmark => "108",
                ReceivedAtLockbox => "109",
                OriginallyScheduledShip => "110",
                ManifestShipNotice => "111",
                BuyersDock => "112",
                SampleRequired => "113",
                ToolingRequired => "114",
                SampleAvailable => "115",
                ScheduledInterchangeDelivery => "116",
                RequestedPickup => "118",
                TestPerformed => "119",
                ControlPlan => "120",
                FeasibilitySignOff => "121",
                FailureModeEffective => "122",
                GroupContractEffective => "124",
                GroupContractExpiration => "125",
                WholesaleContractEffective => "126",
                WholesaleContractExpiration => "127",
                ReplacementEffective => "128",
                CustomerContractEffective => "129",
                CustomerContractExpiration => "130",
                ItemContractEffective => "131",
                ItemContractExpiration => "132",
                AccountsReceivableStatementDate => "133",
                ReadyForInspection => "134",
                Booking => "135",
                TechnicalRating => "136",
                DeliveryRating => "137",
                CommercialRating => "138",
                Estimated => "139",
                Actual => "140",
                Assigned => "141",
                Loss => "142",
                DueDateOfFirstPaymentToPrincipalAndInterest => "143",
                EstimatedAcceptance => "144",
                OpeningDate => "145",
                ClosingDate => "146",
                DueDateLastCompleteInstallmentPaid => "147",
                DateOfLocalOfficeApprovalOfConveyanceOfDamagedRealEstateProperty => "148",
                DateDeedFiledForRecord => "149",
                ServicePeriodStart => "150",
                ServicePeriodEnd => "151",
                EffectiveDateOfChange => "152",
                ServiceInterruption => "153",
                AdjustmentPeriodStart => "154",
                AdjustmentPeriodEnd => "155",
                AllotmentPeriodStart => "156",
                TestPeriodStart => "157",
                TestPeriodEnding => "158",
                BidPriceException => "159",
                SamplesToBeReturnedBy => "160",
                LoadedOnVessel => "161",
                PendingArchive => "162",
                ActualArchive => "163",
                FirstIssue => "164",
                FinalIssue => "165",
                Message => "166",
                Code167 => "167",
                Release => "168",
                ProductAvailabilityDate => "169",
                SupplementalIssue => "170",
                Revision => "171",
                Correction => "172",
                WeekEnding => "173",
                MonthEnding => "174",
                CancelIfNotShippedBy => "175",
                ExpeditedOn => "176",
                Cancellation => "177",
                Code178 => "178",
                Code179 => "179",
                Code180 => "180",
                Code181 => "181",
                Code182 => "182",
                Connection => "183",
                Inventory => "184",
                VesselRegistry => "185",
                InvoicePeriodStart => "186",
                InvoicePeriodEnd => "187",
                CreditAdvice => "188",
                DebitAdvice => "189",
                ReleasedToVessel => "190",
                MaterialSpecification => "191",
                DeliveryTicket => "192",
                PeriodStart => "193",
                PeriodEnd => "194",
                ContractReOpen => "195",
                Start => "196",
                End => "197",
                Completion => "198",
                Seal => "199",
                AssemblyStart => "200",
                Acceptance => "201",
                MasterLeaseAgreement => "202",
                FirstProduced => "203",
                Code204 => "204",
                Transmitted => "205",
                Code206 => "206",
                Code207 => "207",
                LotNumberExpiration => "208",
                ContractPerformanceStart => "209",
                ContractPerformanceDelivery => "210",
                ServiceRequested => "211",
                ReturnedToCustomer => "212",
                AdjustmentToBillDated => "213",
                DateOfRepairService => "214",
                InterruptionStart => "215",
                InterruptionEnd => "216",
                Spud => "217",
                InitialCompletion => "218",
                PluggedAndAbandoned => "219",
                Penalty => "220",
                PenaltyBegin => "221",
                Birth => "222",
                BirthCertificate => "223",
                Adoption => "224",
                Christening => "225",
                LeaseCommencement => "226",
                LeaseTermStart => "227",
                LeaseTermEnd => "228",
                RentStart => "229",
                Installation => "230",
                ProgressPayment => "231",
                ClaimStatementPeriodStart => "232",
                ClaimStatementPeriodEnd => "233",
                SettlementDate => "234",
                Code235 => "235",
                LenderCreditCheck => "236",
                StudentSigned => "237",
                ScheduleRelease => "238",
                Baseline => "239",
                BaselineStart => "240",
                BaselineComplete => "241",
                ActualStart => "242",
                ActualComplete => "243",
                EstimatedStart => "244",
                EstimatedCompletion => "245",
                StartNoEarlierThan => "246",
                StartNoLaterThan => "247",
                FinishNoLaterThan => "248",
                FinishNoEarlierThan => "249",
                Code250 => "250",
                Code251 => "251",
                EarlyStart => "252",
                EarlyFinish => "253",
                LateStart => "254",
                LateFinish => "255",
                ScheduledStart => "256",
                ScheduledFinish => "257",
                OriginalEarlyStart => "258",
                OriginalEarlyFinish => "259",
                RestDay => "260",
                RestStart => "261",
                RestFinish => "262",
                Holiday => "263",
                HolidayStart => "264",
                HolidayFinish => "265",
                Base => "266",
                Timenow => "267",
                EndDateOfSupport => "268",
                DateAccountMatures => "269",
                DateFiled => "270",
                PenaltyEnd => "271",
                ExitPlantDate => "272",
                LatestOnBoardCarrierDate => "273",
                RequestedDepartureDate => "274",
                Approved => "275",
                ContractStart => "276",
                ContractDefinition => "277",
                LastItemDelivery => "278",
                ContractCompletion => "279",
                DateCourseOfOrthodonticsTreatmentBeganOrIsExpectedToBegin => "280",
                OverTargetBaselineMonth => "281",
                PreviousReport => "282",
                FundsAppropriationStart => "283",
                FundsAppropriationEnd => "284",
                EmploymentOrHire => "285",
                Retirement => "286",
                Medicare => "287",
                Code288 => "288",
                PremiumPaidToDate => "289",
                CoordinationOfBenefits => "290",
                Plan => "291",
                Benefit => "292",
                Education => "293",
                EarningsEffectiveDate => "294",
                PrimaryCareProvider => "295",
                InitialDisabilityPeriodReturnToWork => "296",
                InitialDisabilityPeriodLastDayWorked => "297",
                LatestAbsence => "298",
                Illness => "299",
                EnrollmentSignatureDate => "300",
                Code301 => "301",
                Maintenance => "302",
                MaintenanceEffective => "303",
                LatestVisitOrConsultation => "304",
                NetCreditServiceDate => "305",
                AdjustmentEffectiveDate => "306",
                Eligibility => "307",
                PreAwardSurvey => "308",
                PlanTermination => "309",
                DateOfClosing => "310",
                LatestReceivingDateCutoffDate => "311",
                SalaryDeferral => "312",
                Cycle => "313",
                Disability => "314",
                Offset => "315",
                PriorIncorrectDateOfBirth => "316",
                CorrectedDateOfBirth => "317",
                Added => "318",
                Failed => "319",
                DateForeclosureProceedingsInstituted => "320",
                Purchased => "321",
                PutIntoService => "322",
                Replaced => "323",
                Returned => "324",
                DisbursementDate => "325",
                GuaranteeDate => "326",
                QuarterEnding => "327",
                Changed => "328",
                Terminated => "329",
                ReferralDate => "330",
                EvaluationDate => "331",
                PlacementDate => "332",
                Code333 => "333",
                ReEvaluationDate => "334",
                DismissalDate => "335",
                EmploymentBegin => "336",
                EmploymentEnd => "337",
                MedicareBegin => "338",
                MedicareEnd => "339",
                Code340 => "340",
                Code341 => "341",
                PremiumPaidToDateBegin => "342",
                PremiumPaidToDateEnd => "343",
                CoordinationOfBenefitsBegin => "344",
                CoordinationOfBenefitsEnd => "345",
                PlanBegin => "346",
                PlanEnd => "347",
                BenefitBegin => "348",
                BenefitEnd => "349",
                EducationBegin => "350",
                EducationEnd => "351",
                PrimaryCareProviderBegin => "352",
                PrimaryCareProviderEnd => "353",
                IllnessBegin => "354",
                IllnessEnd => "355",
                EligibilityBegin => "356",
                EligibilityEnd => "357",
                CycleBegin => "358",
                CycleEnd => "359",
                InitialDisabilityPeriodStart => "360",
                InitialDisabilityPeriodEnd => "361",
                OffsetBegin => "362",
                OffsetEnd => "363",
                PlanPeriodElectionBegin => "364",
                PlanPeriodElectionEnd => "365",
                PlanPeriodElection => "366",
                DueToCustomer => "367",
                Submittal => "368",
                EstimatedDepartureDate => "369",
                ActualDepartureDate => "370",
                EstimatedArrivalDate => "371",
                ActualArrivalDate => "372",
                OrderStart => "373",
                OrderEnd => "374",
                DeliveryStart => "375",
                DeliveryEnd => "376",
                ContractCostsThrough => "377",
                FinancialInformationSubmission => "378",
                BusinessTermination => "379",
                ApplicantSigned => "380",
                CosignerSigned => "381",
                Enrollment => "382",
                AdjustedHire => "383",
                CreditedService => "384",
                CreditedServiceBegin => "385",
                CreditedServiceEnd => "386",
                DeferredDistribution => "387",
                PaymentCommencement => "388",
                PayrollPeriod => "389",
                PayrollPeriodBegin => "390",
                PayrollPeriodEnd => "391",
                PlanEntry => "392",
                PlanParticipationSuspension => "393",
                Rehire => "394",
                Retermination => "395",
                Termination => "396",
                Valuation => "397",
                VestingService => "398",
                VestingServiceBegin => "399",
                VestingServiceEnd => "400",
                DuplicateBill => "401",
                AdjustmentPromised => "402",
                AdjustmentProcessed => "403",
                YearEnding => "404",
                Production => "405",
                MaterialClassification => "406",
                Weighed => "408",
                DateOfDeedInLieu => "409",
                DateOfFirmCommitment => "410",
                ExpirationDateOfExtensionToForeclose => "411",
                DateOfNoticeToConvey => "412",
                DateOfReleaseOfBankruptcy => "413",
                OptimisticEarlyStart => "414",
                OptimisticEarlyFinish => "415",
                OptimisticLateStart => "416",
                OptimisticLateFinish => "417",
                MostLikelyEarlyStart => "418",
                MostLikelyEarlyFinish => "419",
                MostLikelyLateStart => "420",
                MostLikelyLateFinish => "421",
                PessimisticEarlyStart => "422",
                PessimisticEarlyFinish => "423",
                PessimisticLateStart => "424",
                PessimisticLateFinish => "425",
                FirstPaymentDue => "426",
                FirstInterestPaymentDue => "427",
                SubsequentInterestPaymentDue => "428",
                IrregularInterestPaymentDue => "429",
                GuarantorReceived => "430",
                OnsetOfCurrentSymptomsOrIllness => "431",
                Submission => "432",
                Removed => "433",
                Statement => "434",
                Admission => "435",
                InsuranceCard => "436",
                SpouseRetirement => "437",
                OnsetOfSimilarSymptomsOrIllness => "438",
                Accident => "439",
                ReleaseOf => "440",
                PriorPlacement => "441",
                DateOfDeath => "442",
                Code443 => "443",
                FirstVisitOrConsultation => "444",
                InitialPlacement => "445",
                Replacement => "446",
                Occurrence => "447",
                OccurrenceSpan => "448",
                OccurrenceSpanFrom => "449",
                OccurrenceSpanTo => "450",
                InitialFeeDue => "451",
                AppliancePlacement => "452",
                AcuteManifestationOfAChronicCondition => "453",
                InitialTreatment => "454",
                LastXRay => "455",
                Surgery => "456",
                Code457 => "457",
                Certification => "458",
                NursingHomeFrom => "459",
                NursingHomeTo => "460",
                LastCertification => "461",
                DateOfLocalOfficeApprovalOfConveyanceOfOccupiedRealEstateProperty => {
                    "462"
                }
                BeginTherapy => "463",
                OxygenTherapyFrom => "464",
                OxygenTherapyTo => "465",
                OxygenTherapy => "466",
                Signature => "467",
                PrescriptionFill => "468",
                ProviderSignature => "469",
                DateOfLocalOfficeCertificationOfConveyanceOfDamagedRealEstateProperty => {
                    "470"
                }
                Prescription => "471",
                Service => "472",
                MedicaidBegin => "473",
                MedicaidEnd => "474",
                Medicaid => "475",
                Code476 => "476",
                Code477 => "477",
                PrescriptionFrom => "478",
                PrescriptionTo => "479",
                ArterialBloodGasTest => "480",
                OxygenSaturationTest => "481",
                PregnancyBegin => "482",
                PregnancyEnd => "483",
                LastMenstrualPeriod => "484",
                InjuryBegin => "485",
                InjuryEnd => "486",
                NursingHome => "487",
                CollateralDependent => "488",
                CollateralDependentBegin => "489",
                CollateralDependentEnd => "490",
                SponsoredDependent => "491",
                SponsoredDependentBegin => "492",
                SponsoredDependentEnd => "493",
                Deductible => "494",
                OutOfPocket => "495",
                ContractAuditDate => "496",
                LatestDeliveryDateAtPier => "497",
                MortgageeReportedCurtailmentDate => "498",
                MortgageeOfficialSignatureDate => "499",
                Resubmission => "500",
                ExpectedReply => "501",
                DroppedToLessThanHalfTime => "502",
                RepaymentBegin => "503",
                LoanServicingTransfer => "504",
                LoanPurchase => "505",
                LastNotification => "506",
                Extract => "507",
                Extended => "508",
                ServicerSignatureDate => "509",
                DatePacked => "510",
                ShelfLifeExpiration => "511",
                WarrantyExpiration => "512",
                Overhauled => "513",
                Transferred => "514",
                Notified => "515",
                Discovered => "516",
                Inspected => "517",
                Code518 => "518",
                DateBankruptcyFiled => "519",
                DateOfDamage => "520",
                DateHazardInsurancePolicyCancelled => "521",
                ExpirationDateToSubmitTitleEvidence => "522",
                DateOfClaim => "523",
                DateOfNoticeOfReferralForAssignment => "524",
                DateOfNoticeOfProbableIneligibilityForAssignment => "525",
                DateOfForeclosureNotice => "526",
                ExpirationOfForeclosureTimeframe => "527",
                DatePossessoryActionInitiated => "528",
                DateOfPossession => "529",
                DateOfLastInstallmentReceived => "530",
                DateOfAcquisitionOfTitle => "531",
                ExpirationOfExtensionToConvey => "532",
                DateOfAssignmentApproval => "533",
                DateOfAssignmentRejection => "534",
                CurtailmentDateFromAdviceOfPayment => "535",
                ExpirationOfExtensionToSubmitFiscalData => "536",
                Code537 => "537",
                MakegoodCommercialDate => "538",
                PolicyEffective => "539",
                PolicyExpiration => "540",
                EmployeeEffectiveDateOfCoverage => "541",
                ClaimAdministratorNotifiedOfEmployeeLegalRepresentation => "542",
                LastPremiumPaidDate => "543",
                EmployerKnowledgeOfTheInjury => "544",
                ClaimAdministratorKnowledgeOfTheInjury => "545",
                DateOfMaximumMedicalImprovement => "546",
                DateOfLoan => "547",
                DateOfAdvance => "548",
                BeginningLayDate => "549",
                CertificateEffective => "550",
                BenefitApplicationDate => "551",
                ActualReturnToWork => "552",
                ReleasedReturnToWork => "553",
                EndingLayDate => "554",
                EmployeeWagesCeased => "555",
                LastSalaryIncrease => "556",
                EmployeeLaidOff => "557",
                InjuryOrIllness => "558",
                OldestUnpaidInstallment => "559",
                PreforeclosureAcceptanceDate => "560",
                PreforeclosureSaleClosingDate => "561",
                DateOfFirstUncuredDefault => "562",
                DateDefaultWasCured => "563",
                DateOfFirstMortgagePayment => "564",
                DateOfPropertyInspection => "565",
                DateTotalAmountOfDelinquencyReported => "566",
                DateOutstandingLoanBalanceReported => "567",
                DateForeclosureSaleScheduled => "568",
                DateForeclosureHeld => "569",
                DateRedemptionPeriodEnds => "570",
                DateVoluntaryConveyanceAccepted => "571",
                DatePropertySold => "572",
                DateClaimPaid => "573",
                ActionBeginDate => "574",
                ProjectedActionEndDate => "575",
                ActionEndDate => "576",
                OriginalMaturityDate => "577",
                DateReferredToAttorneyForForeclosure => "578",
                PlannedRelease => "579",
                ActualRelease => "580",
                ContractPeriod => "581",
                ReportPeriod => "582",
                Suspension => "583",
                Reinstatement => "584",
                Report => "585",
                FirstContact => "586",
                ProjectedForeclosureSaleDate => "587",
                DateAssignmentFiledForRecord => "589",
                DateOfAppraisal => "590",
                ExpirationDateOfExtensionToAssign => "591",
                DateOfExtensionToConvey => "592",
                DateHazardInsurancePolicyRefused => "593",
                HighFabricationReleaseAuthorization => "594",
                HighRawMaterialAuthorization => "595",
                MaterialChangeNotice => "596",
                LatestDeliveryDateAtRailRamp => "597",
                Rejected => "598",
                RepaymentScheduleSent => "599",
                AsOf => "600",
                FirstSubmission => "601",
                SubsequentSubmission => "602",
                Renewal => "603",
                Withdrawn => "604",
                CertificationPeriodStart => "606",
                CertificationRevision => "607",
                Code608 => "608",
                PrearrangedDealMatch => "609",
                ContingencyEnd => "610",
                OxygenTherapyEvaluation => "611",
                ShutIn => "612",
                AllowableEffective => "613",
                FirstSales => "614",
                DateAcquired => "615",
                InterviewerSigned => "616",
                ApplicationLoggedDate => "617",
                ReviewDate => "618",
                DecisionDate => "619",
                PreviouslyResided => "620",
                Reported => "621",
                Checked => "622",
                Settled => "623",
                PresentlyResiding => "624",
                EmployedInThisPosition => "625",
                Verified => "626",
                SecondAdmissionDate => "627",
                AccountOpened => "629",
                AccountClosed => "630",
                PropertyAcquired => "631",
                PropertyBuilt => "632",
                EmployedInThisProfession => "633",
                NextReviewDate => "634",
                InitialContactDate => "635",
                DateOfLastUpdate => "636",
                SecondDischargeDate => "637",
                DateOfLastDraw => "638",
                Complaint => "640",
                Option => "641",
                Solicitation => "642",
                Clause => "643",
                Meeting => "644",
                RentalPeriod => "646",
                NextPayIncrease => "647",
                PeriodCoveredBySourceDocuments => "648",
                DocumentDue => "649",
                CourtNotice => "650",
                ExpectedFundingDate => "651",
                AssignmentRecorded => "652",
                CaseReopened => "653",
                PreviousCourtEvent => "655",
                LastDateToObject => "656",
                CourtEvent => "657",
                LastDateToFileAClaim => "658",
                CaseConverted => "659",
                DebtIncurred => "660",
                Judgment => "661",
                WagesStart => "662",
                WagesEnd => "663",
                DateThroughWhichPropertyTaxesHaveBeenPaid => "664",
                PaidThroughDate => "665",
                DatePaid => "666",
                AnesthesiaAdministration => "667",
                PriceProtection => "668",
                ClaimIncurred => "669",
                BookEntryDelivery => "670",
                RateAdjustment => "671",
                NextInstallmentDueDate => "672",
                DaylightOverdraftTime => "673",
                PresentmentDate => "674",
                NegotiatedExtensionDate => "675",
                Remittance => "681",
                SecurityRateAdjustment => "682",
                FilingPeriod => "683",
                ReviewPeriodEnd => "684",
                RequestedSettlement => "685",
                LastScreening => "686",
                Confinement => "687",
                Arrested => "688",
                Convicted => "689",
                Interviewed => "690",
                LastVisit => "691",
                Recovery => "692",
                TimeInUS => "693",
                FuturePeriod => "694",
                PreviousPeriod => "695",
                InterestPaidTo => "696",
                DateOfSeizure => "697",
                Setoff => "699",
                OverrideDateForSettlement => "700",
                Code701 => "701",
                SendingRoadTimeStamp => "702",
                RetransmissionTimeStamp => "703",
                DeliveryAppointmentDateAndTime => "704",
                InterestPaidThrough => "705",
                DateMaterialUsageSuspended => "706",
                LastPaymentMade => "707",
                PastDue => "708",
                AnalysisMonthEnding => "709",
                DateOfSpecification => "710",
                DateOfStandard => "711",
                ReturnToWorkPartTime => "712",
                PaidThroughDateForSalaryContinuation => "713",
                PaidThroughDateForVacationPay => "714",
                PaidThroughDateForAccruedSickPay => "715",
                AppraisalOrdered => "716",
                DateOfOperation => "717",
                BestTimeToCall => "718",
                VerbalReportNeeded => "719",
                EstimatedEscrowClosing => "720",
                PermitYear => "721",
                RemodelingCompleted => "722",
                CurrentMonthEnding => "723",
                PreviousMonthEnding => "724",
                CycleToDate => "725",
                YearToDate => "726",
                OnHold => "727",
                OffHold => "728",
                FacsimileDueBy => "729",
                ReportingCycleDate => "730",
                LastPaidInstallmentDate => "731",
                ClaimsMade => "732",
                DateOfLastPaymentReceived => "733",
                CurtailmentDate => "734",
                PoolSettlement => "736",
                NextInterestChangeDate => "737",
                MostRecentHemoglobinOrHematocritOrBoth => "738",
                MostRecentSerumCreatine => "739",
                Closed => "740",
                Therapy => "741",
                Implantation => "742",
                Explantation => "743",
                DateBecameAware => "744",
                FirstMarketed => "745",
                LastMarketed => "746",
                NewDueDateOfFirstPaymentToPrincipalAndInterest => "747",
                NewMaturityDate => "748",
                Current => "749",
                ExpectedProblemResolution => "750",
                AlternateProblemResolution => "751",
                FeeCapitalization => "752",
                InterestCapitalization => "753",
                NextPaymentDue => "754",
                ConversionToRepayment => "755",
                EndOfGracePeriod => "756",
                SchoolRefund => "757",
                SimpleInterestDue => "758",
                DatePracticeCeased => "759",
                Printed => "760",
                DatePracticeEstablished => "761",
                DropActionDate => "762",
                MostRecentRenewal => "764",
                Original => "765",
                OutsideAuditorsReport => "766",
                PreCertificationDate => "769",
                BackOnMarket => "770",
                Status => "771",
                BenefitAdjustmentStart => "772",
                OffMarket => "773",
                Tour => "774",
                BenefitAdjustmentEnd => "775",
                ListingReceived => "776",
                BenefitAdjustmentPeriod => "777",
                AnticipatedClosing => "778",
                LastPublication => "779",
                SoldBookPublication => "780",
                Occupancy => "781",
                Contingency => "782",
                PercolationTest => "783",
                SepticApproval => "784",
                TitleTransfer => "785",
                OpenHouse => "786",
                BenefitCreditPeriod => "787",
                BenefitTransferPeriod => "788",
                Homestead => "789",
                Sanction => "790",
                TailCoverageBegin => "791",
                TailCoverageEnd => "792",
                TrainingBegin => "793",
                TrainingEnd => "794",
                VerificationReceived => "795",
                VerificationSent => "796",
                StateResidencyDate => "797",
                EffectiveDateOfTheRoutingFile => "798",
                TestDataAnalysis => "799",
                MidpointOfPerformance => "800",
                AcquisitionDate => "801",
                DateOfAction => "802",
                PaidInFull => "803",
                Refinance => "804",
                VoluntaryTermination => "805",
                CustomerOrder => "806",
                Stored => "807",
                Selected => "808",
                Posted => "809",
                DocumentReceived => "810",
                Rebuilt => "811",
                Marriage => "812",
                CustomsEntryDate => "813",
                PaymentDueDate => "814",
                MaturityDate => "815",
                TradeDate => "816",
                Code817 => "817",
                Code818 => "818",
                LastAccountsFiledAtPublicRegistrationAgency => "819",
                RealEstateTaxYear => "820",
                FinalReconciliationValueEstimateAsOf => "821",
                Map => "822",
                Opinion => "823",
                Version => "824",
                OriginalDueDate => "825",
                IncumbencyPeriod => "826",
                AudienceDeficiencyPeriod => "827",
                AiredDate => "828",
                Schedule => "830",
                PaidThroughDateForMinimumPayment => "831",
                PaidThroughDateForTotalPayment => "832",
                Election => "840",
                EngineeringDataList => "841",
                LastProduction => "842",
                NotBefore => "843",
                NotAfter => "844",
                InitialClaim => "845",
                BenefitsPaid => "846",
                WagesEarned => "847",
                AdjustedStart => "848",
                AdjustedEnd => "849",
                RevisedAdjustedStart => "850",
                RevisedAdjustedEnd => "851",
                FieldTest => "853",
                MortgageNoteDate => "854",
                AlternativeDueDate => "855",
                FirstPaymentChange => "856",
                FirstRateAdjustment => "857",
                AlternateBasePeriod => "858",
                PriorNotice => "859",
                AppointmentEffective => "860",
                AppointmentExpiration => "861",
                CompanyTermination => "862",
                ContinuingEducationRequirement => "863",
                DistributorEffective => "864",
                DistributorTermination => "865",
                Examination => "866",
                IncorporationDissolution => "867",
                LastFollowUp => "868",
                LicenseEffective => "869",
                LicenseExpiration => "870",
                LicenseRenewal => "871",
                LicenseRequested => "872",
                Mailed => "873",
                PaperworkMailed => "874",
                PreviousEmployment => "875",
                PreviousEmploymentEnd => "876",
                PreviousEmploymentStart => "877",
                PreviousResidence => "878",
                PreviousResidenceEnd => "879",
                PreviousResidenceStart => "880",
                Request => "881",
                ResidentLicenseEffective => "882",
                ResidentLicenseExpiration => "883",
                StateTermination => "884",
                TexasLineTermination => "885",
                Acceleration => "900",
                AdjustedContestability => "901",
                ApplicationEntry => "903",
                ApprovalOffer => "904",
                AutomaticPremiumLoan => "905",
                Collection => "906",
                ConfinementEnd => "907",
                ConfinementStart => "908",
                Contestability => "909",
                FlatExtraEnd => "910",
                LastActivity => "911",
                LastChange => "912",
                LastEpisode => "913",
                LastMeal => "914",
                Loan => "915",
                ApplicationStatus => "916",
                Maturity => "917",
                MedicalInformationSignature => "918",
                MedicalInformationSystem => "919",
                Note => "920",
                OfferExpiration => "921",
                OriginalReceipt => "922",
                Placement => "923",
                PlacementPeriodExpiration => "924",
                Processing => "925",
                Recapture => "926",
                ReEntry => "927",
                Reissue => "928",
                Requalification => "930",
                ReinsuranceEffective => "931",
                ReservationOfFacility => "932",
                SettlementStatus => "933",
                TableRatingEnd => "934",
                TerminationOfFacility => "935",
                Treatment => "936",
                DepartmentOfLaborWageDeterminationDate => "937",
                Order => "938",
                Resolved => "939",
                ExecutionDate => "940",
                CapitationPeriodStart => "941",
                CapitationPeriodEnd => "942",
                LastDateForAGovernmentAgencyToFileAClaim => "943",
                AdjustmentPeriod => "944",
                Activity => "945",
                MailBy => "946",
                Preparation => "947",
                PaymentInitiated => "948",
                PaymentEffective => "949",
                Application => "950",
                Reclassification => "951",
                Code952 => "952",
                PostReclassification => "953",
                Code954 => "954",
                Code955 => "955",
                Code956 => "956",
                Code957 => "957",
                AdjustedDeathBenefit => "960",
                Anniversary => "961",
                Annuitization => "962",
                AnnuityCommencementDate => "963",
                Bill => "964",
                CalendarAnniversary => "965",
                ContractMailed => "966",
                EarlyWithdrawal => "967",
                FiscalAnniversary => "968",
                Income => "969",
                InitialPremium => "970",
                InitialPremiumEffective => "971",
                LastPremiumEffective => "972",
                MinimumRequiredDistribution => "973",
                NextAnniversary => "974",
                Notice => "975",
                NotificationOfDeath => "976",
                PartialAnnuitization => "977",
                PlanAnniversary => "978",
                PolicySurrender => "979",
                PriorContractAnniversary => "980",
                PriorContractIssue => "981",
                SignatureReceived => "982",
                Tax => "983",
                BenefitPeriod => "984",
                MonthToDate => "985",
                SemiannualEnding => "986",
                Surrender => "987",
                PlanOfTreatmentPeriod => "988",
                Code989 => "989",
                OriginalNameChange => "990",
                DateRequested => "992",
                RequestForQuotation => "993",
                Quote => "994",
                RecordedDate => "995",
                RequiredDelivery => "996",
                QuoteToBeReceivedBy => "997",
                ContinuationOfPayStartDate => "998",
                DocumentDate => "999",
                EstimatedPointOfArrival => "AA1",
                EstimatedPointOfDischarge => "AA2",
                CodeAA3 => "AA3",
                CodeAA4 => "AA4",
                CodeAA5 => "AA5",
                CodeAA6 => "AA6",
                FinalScheduledPayment => "AA7",
                ActualDischarge => "AA8",
                AddressPeriod => "AA9",
                ArrivalInCountry => "AAA",
                Citation => "AAB",
                SuspensionEffective => "AAC",
                Crime => "AAD",
                DischargePlanned => "AAE",
                Draft => "AAF",
                DueDate => "AAG",
                Event => "AAH",
                FirstInvolvement => "AAI",
                GuaranteePeriod => "AAJ",
                IncomeIncreasePeriod => "AAK",
                InstallmentDate => "AAL",
                LastCivilianFlight => "AAM",
                LastFlight => "AAN",
                LastInsuranceMedical => "AAO",
                LastMilitaryFlight => "AAP",
                LastPhysical => "AAQ",
                License => "AAR",
                MedicalCertificate => "AAS",
                Medication => "AAT",
                NetWorthDate => "AAU",
                NextActivity => "AAV",
                OwnershipChange => "AAW",
                OwnershipPeriod => "AAX",
                RateDate => "AAY",
                RequestedContract => "AAZ",
                RequestedOffer => "AB1",
                SalesPeriod => "AB2",
                TaxYear => "AB3",
                TimePeriod => "AB4",
                Travel => "AB5",
                TreatmentEnd => "AB6",
                TreatmentStart => "AB7",
                Trust => "AB8",
                WorstTimeToCall => "AB9",
                Registration => "ABA",
                Revoked => "ABB",
                EstimatedDateOfBirth => "ABC",
                LastAnnualReport => "ABD",
                LegalActionStarted => "ABE",
                Lien => "ABF",
                PaymentPeriod => "ABG",
                ProfitPeriod => "ABH",
                Registered => "ABI",
                Consolidated => "ABK",
                BoardOfDirectorsNotAuthorizedAsOf => "ABL",
                BoardOfDirectorsIncompleteAsOf => "ABM",
                ManagerNotRegisteredAsOf => "ABN",
                CitizenshipChange => "ABO",
                Participation => "ABP",
                Capitalization => "ABQ",
                RegistrationOfBoardOfDirectors => "ABR",
                CeasedOperations => "ABS",
                Satisfied => "ABT",
                TermsMet => "ABU",
                AssetDocumentationExpiration => "ABV",
                CreditDocumentationExpiration => "ABW",
                IncomeDocumentationExpiration => "ABX",
                ProductHeldUntil => "ABY",
                ImmigrationDate => "ACA",
                EstimatedImmigrationDate => "ACB",
                CurrentDisabilityPeriodStart => "ACC",
                CurrentDisabilityPeriodEnd => "ACD",
                CurrentDisabilityPeriodLastDayWorked => "ACE",
                BenefitTypeGrossWeeklyAmountEffective => "ACF",
                BenefitTypeNetWeeklyAmountEffective => "ACG",
                BenefitTypePeriodStart => "ACH",
                BenefitTypePeriodEnd => "ACI",
                BenefitDebitStart => "ACJ",
                Acknowledgment => "ACK",
                BenefitDebitEnd => "ACL",
                BenefitCreditStart => "ACM",
                BenefitCreditEnd => "ACN",
                BenefitTransferStart => "ACO",
                BenefitTransferEnd => "ACP",
                WageEffective => "ACQ",
                FullDenialEffective => "ACR",
                FullDenialRescission => "ACS",
                PaymentIssue => "ACT",
                PaymentPeriodStart => "ACU",
                PaymentPeriodEnd => "ACV",
                EmployerReportedInjuryToClaimAdministrator => "ACW",
                SurveyYear => "ACX",
                ControvertDate => "ACZ",
                BilledThrough => "ADA",
                BusinessControlChange => "ADB",
                CourtRegistration => "ADC",
                AnnualReportDue => "ADD",
                ClaimNotificationReceived => "ADE",
                ConversionPrivilegeEnd => "ADF",
                DividendApplied => "ADG",
                InForce => "ADH",
                PaidUp => "ADI",
                PremiumChange => "ADJ",
                PolicyEffectiveOnOrBefore => "ADK",
                AssetAndLiabilitySchedule => "ADL",
                AnnualReportMailed => "ADM",
                PolicyEffectiveOnOrAfter => "ADN",
                AnnualReportFiled => "ADR",
                AuditPeriodAfterSplitDate => "ADS",
                AuditPeriodPriorToSplitDate => "ADT",
                ExposureSourcePeriod => "ADU",
                SubcontractorPeriodOfHire => "ADV",
                Divorce => "ADW",
                PowerOfAttorney => "ADX",
                UniformGiftsToMinorsAccountEstablished => "ADY",
                MedicarePartAEligibilityBeginDate => "AEA",
                MedicarePartAEligibilityEndDate => "AEB",
                MedicarePartACoverageEffectiveDate => "AEC",
                MedicarePartATerminationDate => "AED",
                MedicarePartBEligibilityBeginDate => "AEE",
                MedicarePartBEligibilityEndDate => "AEF",
                MedicarePartBCoverageEffectiveDate => "AEG",
                MedicarePartBTerminationDate => "AEH",
                LoadingPeriod => "AEI",
                DateOnWhichAssetsJudgedInsufficientToPayCreditors => "AEK",
                EmployeesTemporarilyLaidOffBeginPeriod => "AEL",
                EmployeesTemporarilyLaidOffEndPeriod => "AEM",
                FirstPublished => "AEN",
                ForecastPeriodStart => "AEO",
                ForecastPeriodEnd => "AEP",
                InvestigationStart => "AEQ",
                InvestigationEnd => "AER",
                LastPublished => "AES",
                LatestBalanceSheet => "AET",
                SharePrice => "AEU",
                StopDistribution => "AEV",
                MaximumCreditDate => "AEW",
                FoundingDate => "AEX",
                RepaymentPlanStartDate => "AEY",
                MedicarePartDEligibilityBeginDate => "AFA",
                MedicarePartDEligibilityEndDate => "AFB",
                MedicarePartDCoverageEffectiveDate => "AFC",
                MedicarePartDTerminationDate => "AFD",
                AnnualReportDelinquency => "ARD",
                WithheldDate => "AWH",
                ComplianceAudit => "BAA",
                ContractorSafetyPerformanceEvaluation => "BAB",
                ContractorSafetyProceduresReview => "BAC",
                DateOfEquipmentInspection => "BAD",
                DateOfSafetyInspection => "BAE",
                EmployeesParticipationPlanReview => "BAF",
                CodeBAG => "BAG",
                CodeBAH => "BAH",
                ExpectedCompletionOfChangesResultingFromHazardReview => "BAI",
                HazardReviewCompletion => "BAJ",
                HotWorkPermitProceduresReview => "BAK",
                Investigation => "BAL",
                MaintenanceProceduresReview => "BAM",
                ManagementOfChangeProceduresReview => "BAN",
                OperatingProceduresReview => "BAO",
                SafetyInformationReview => "BAP",
                Training => "BAQ",
                TrainingProgramReview => "BAR",
                BillbackEndDate => "BED",
                ProgramPerformanceEndDate => "BEE",
                ProgramPerformanceStartDate => "BES",
                BeginningOfGracePeriod => "BGP",
                BillingActivities => "BIA",
                BeginningOfInterestPaidAfterClaim => "BIP",
                BillbackStartDate => "BSD",
                ChangedAccountingDate => "CAD",
                CustomsCargoRelease => "CCR",
                ContractDefinitizationDate => "CDD",
                CampaignEndDate => "CDE",
                CampaignStartDate => "CDS",
                MaintenanceComment => "CDT",
                Formation => "CEA",
                Continuance => "CEB",
                Merger => "CEC",
                YearDue => "CED",
                NextAnnualMeeting => "CEE",
                EndOfLastFiscalYear => "CEF",
                YearBeginning => "CEH",
                StartedDoingBusiness => "CEJ",
                SwornAndSubscribed => "CEK",
                CalendarYear => "CEL",
                Asset => "CEM",
                Inactivity => "CEN",
                HighCapitalYear => "CEO",
                ClosingDateOfFirstBalanceSheet => "CLO",
                ClosedUntil => "CLU",
                Compliance => "COM",
                ConvertedIntoHoldingCompany => "CON",
                CareOfSuppliesInStorageInspectionDate => "COS",
                ConsumerProductAvailabilityDate => "CPA",
                ConsumerProductInformationPublicationDate => "CPD",
                ConsumerProductVariantEndEffectiveDate => "CPE",
                ConsumerProductVariantDiscontinuedDate => "CPF",
                ConsumerProductVariantCancelledDate => "CPG",
                ConsumerProductVariantStartEffectiveDate => "CPS",
                ClaimRevised => "CRV",
                CurrentList => "CUR",
                CommunityVisibility => "CVD",
                AccountFrozen => "DAF",
                Declaration => "DDO",
                DeedNotAvailable => "DEE",
                Delete => "DEL",
                DetrimentalInformationReceived => "DET",
                Deferral => "DFF",
                DepartureFromSpecification => "DFS",
                CodeDIL => "DIL",
                DelayedInterestPaidThrough => "DIP",
                Disposition => "DIS",
                DateOfLastContact => "DLC",
                DateOfAbandonment => "DOA",
                DateOfDelinquency => "DOD",
                DeliveryOrderIssued => "DOI",
                Repossession => "DOR",
                Disposal => "DSP",
                DeedAndTitleReceived => "DTC",
                TechnicalDataSupplyBy => "DTD",
                DeedAndTitleRequested => "DTQ",
                TenureDecision => "E01",
                MostRecentPositionChange => "E02",
                FeePayment => "E03",
                StartDateForContinuousEmployment => "E04",
                StartDateForCurrentPosition => "E05",
                StartDateForOriginalPosition => "E06",
                FiscalYear => "E07",
                EndAvailabilityDate => "EAD",
                EstimatedConstructionDate => "ECD",
                EstimatedCompletionFirstPriorMonth => "ECF",
                EstimatedCompletionSecondPriorMonth => "ECS",
                EstimatedCompletionThirdPriorMonth => "ECT",
                Affirmed => "EDA",
                Auction => "EDB",
                Authorized => "EDC",
                Contribution => "EDD",
                Executed => "EDE",
                Forgiven => "EDF",
                Presented => "EDG",
                LegislativeSession => "EDH",
                Organized => "EDI",
                Pledged => "EDJ",
                PrimaryElection => "EDK",
                Qualified => "EDL",
                Refunded => "EDM",
                Rescinded => "EDN",
                RestructuredFrom => "EDO",
                Vote => "EDP",
                EmployerKnowledgeOfTheDisability => "EKD",
                EndDateMaximumBuyingQuantity => "EMM",
                EndDateMinimumBuyingQuantity => "END",
                EstimatePreparation => "EPP",
                EstimateComment => "ESC",
                EffectiveStartDate => "ESD",
                EstimatedStartFirstPriorMonth => "ESF",
                EstimatedStartSecondPriorMonth => "ESS",
                EstimatedStartThirdPriorMonth => "EST",
                EarliestFilingPeriod => "ETP",
                Exposure => "EXO",
                Export => "EXP",
                FacilityLatestBillingAction => "FAC",
                FacilityEarliestBillingAction => "FEB",
                Financial => "FFI",
                FirstOrder => "FFO",
                FinalInterestAccrual => "FIA",
                FundingPeriodEnd => "FPE",
                FundingPeriodStart => "FPS",
                FirstAvailableShipDate => "FSD",
                FreeServiceCallEndDate => "FSE",
                FreeServiceCallStartDate => "FSS",
                Graduated => "GRD",
                HomeHealthDateOfEarliestBillableAction => "HHD",
                HomeHealthEpisode => "HHE",
                HomeHealthDateOfLatestBillableAction => "HHL",
                HostTrainDate => "HTD",
                ConvertedToElectronicDate => "ICF",
                InsolvencyDischargeGranted => "IDG",
                InitialFederalHousingAuthorityClaimPayment => "IFH",
                Incorporation => "III",
                ImageLastUpdateDate => "ILU",
                ImbalancePeriod => "IMB",
                Import => "IMP",
                Incident => "INC",
                InactiveUntil => "INT",
                InterestOnPresaleStart => "IPS",
                InitialVeteransAdministrationClaimPayment => "IVA",
                KeyEventFiscalYear => "KEV",
                KeyEventCalendarYear => "KEW",
                LastAnnualMeeting => "LAM",
                LastCheckForBalanceSheetUpdate => "LAS",
                LastCapitalChange => "LCC",
                LetterOfAgreement => "LEA",
                LetterOfLiability => "LEL",
                Liquidation => "LIQ",
                LowPeriod => "LLP",
                EquipmentLogEntry => "LOG",
                ListPriceChange => "LPC",
                LegalStructureChange => "LSC",
                LastSubmissionDate => "LSD",
                LatestFilingPeriod => "LTP",
                MeterReading => "MRR",
                LatestMaterialSafetyDataSheetDate => "MSD",
                PresentName => "NAM",
                NegotiatedFinish => "NFD",
                CodeNOD => "NOD",
                NotRegistered => "NRG",
                NegotiatedStart => "NSD",
                OrganicCertificationDate => "OCD",
                OriginalList => "ORG",
                PresentControl => "PBC",
                PrimaryCoverageClaimPaid => "PCP",
                PrimaryCoverageClaimSubmission => "PCS",
                PriceChangesAllowedFromDate => "PCT",
                PriceChangesAllowedToDate => "PCU",
                ProductDiscontinuedButStillAvailable => "PDA",
                PartialDenialEffective => "PDE",
                PartialDenialRescission => "PDR",
                CorrectProgramStartDate => "PDS",
                CorrectProgramEndDate => "PDT",
                CorrectContractStartDate => "PDU",
                PrivilegeDetailsVerification => "PDV",
                CorrectContractEndDate => "PDW",
                ProgramEndDate => "PED",
                ProductImageStartDate => "PIS",
                PresentLegalStructure => "PLS",
                PackagingMaterialEffectiveDate => "PME",
                PoolPolicyClaimSubmission => "PPC",
                PostPaidDate => "PPD",
                PeakPeriod => "PPP",
                PreviouslyReportedDateOfBirth => "PRD",
                PresentedToReceivers => "PRR",
                PropertySaleApproved => "PSA",
                PropertySaleClosed => "PSC",
                ProgramStartDate => "PSD",
                PropertySaleConfirmation => "PSF",
                PaidToDate => "PTD",
                PlannedObsolescenceItem => "PTO",
                PickUpDate => "PUD",
                ReceiverAppointed => "RAP",
                RemanufactureDate => "REM",
                Resigned => "RES",
                RequestedFinish => "RFD",
                RecoveryFinish => "RFF",
                ReferredFrom => "RFO",
                RentSurvey => "RNT",
                ReceivedInTheMail => "RRM",
                Revocation => "RRT",
                RequestedStart => "RSD",
                RecoveryStart => "RSS",
                ReferredTo => "RTO",
                SocialSecurityClaimsVerification => "SCV",
                SoleDirectorshipDate => "SDD",
                StartDateMaximumBuyingQuantity => "SDM",
                SubsequentFederalHousingAuthorityClaimPayment => "SFH",
                StartDateMinimumBuyingQuantity => "SMB",
                InitialSupportDate => "SPD",
                SuggestedRetailPriceEffectiveDate => "SPE",
                Transition => "STN",
                SubsequentVeteransAdministrationClaimPayment => "SVA",
                TradeStyleRegistered => "TSR",
                TrialStarted => "TSS",
                TrialSet => "TST",
                TenantTrainDate => "TTD",
                CodeVAT => "VAT",
                ValidUntil => "VLU",
                SampleCollected => "W01",
                StatusChange => "W02",
                ConstructionStart => "W03",
                Recompletion => "W05",
                LastLogged => "W06",
                WellLogRun => "W07",
                SurfaceCasingAuthorityApproval => "W08",
                ReachedTotalDepth => "W09",
                SpacingOrderUnitAssigned => "W10",
                RigArrival => "W11",
                LocationExceptionOrderNumberAssigned => "W12",
                SidetrackedWellbore => "W13",
                TimeEmployeeBeganWork => "WAA",
                Waybill => "WAY",
                OrderDay => "XX1",
                DeliveryDay => "XX2",
                OrderCutOffTime => "XX3",
                ActiveItem => "XX5",
                MatureItem => "XX6",
                IntroductoryItem => "XX7",
                ObsoleteItem => "XX8",
                BestBeforeDate => "Y11",
                HarvestDate => "Y12",
                ProgrammedFiscalYear => "YXX",
                ProgrammedCalendarYear => "YXY",
                MutuallyDefined => "ZZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<DateTimeQualifier> {
        use DateTimeQualifier::*;
        match code {
            b"001" => Some(CancelAfter),
            b"002" => Some(DeliveryRequested),
            b"003" => Some(Invoice),
            b"004" => Some(PurchaseOrder),
            b"005" => Some(Sailing),
            b"006" => Some(Sold),
            b"007" => Some(Effective),
            b"008" => Some(PurchaseOrderReceived),
            b"009" => Some(Process),
            b"010" => Some(RequestedShip),
            b"011" => Some(Shipped),
            b"012" => Some(TermsDiscountDue),
            b"013" => Some(TermsNetDue),
            b"014" => Some(DeferredPayment),
            b"015" => Some(PromotionStart),
            b"016" => Some(PromotionEnd),
            b"017" => Some(EstimatedDelivery),
            b"018" => Some(Available),
            b"019" => Some(Unloaded),
            b"020" => Some(Check),
            b"021" => Some(ChargeBack),
            b"022" => Some(FreightBill),
            b"023" => Some(PromotionOrderStart),
            b"024" => Some(PromotionOrderEnd),
            b"025" => Some(PromotionShipStart),
            b"026" => Some(PromotionShipEnd),
            b"027" => Some(PromotionRequestedDeliveryStart),
            b"028" => Some(PromotionRequestedDeliveryEnd),
            b"029" => Some(PromotionPerformanceStart),
            b"030" => Some(PromotionPerformanceEnd),
            b"031" => Some(PromotionInvoicePerformanceStart),
            b"032" => Some(PromotionInvoicePerformanceEnd),
            b"033" => Some(PromotionFloorStockProtectStart),
            b"034" => Some(PromotionFloorStockProtectEnd),
            b"035" => Some(Delivered),
            b"036" => Some(Expiration),
            b"037" => Some(ShipNotBefore),
            b"038" => Some(ShipNoLater),
            b"039" => Some(ShipWeekOf),
            b"040" => Some(Code040),
            b"041" => Some(Code041),
            b"042" => Some(Superseded),
            b"043" => Some(Publication),
            b"044" => Some(SettlementDateAsSpecifiedByTheOriginator),
            b"045" => Some(EndorsementDate),
            b"046" => Some(FieldFailure),
            b"047" => Some(FunctionalTest),
            b"048" => Some(SystemTest),
            b"049" => Some(PrototypeTest),
            b"050" => Some(Received),
            b"051" => Some(CumulativeQuantityStart),
            b"052" => Some(CumulativeQuantityEnd),
            b"053" => Some(BuyersLocal),
            b"054" => Some(SellersLocal),
            b"055" => Some(Confirmed),
            b"056" => Some(EstimatedPortOfEntry),
            b"057" => Some(ActualPortOfEntry),
            b"058" => Some(CustomsClearance),
            b"059" => Some(InlandShip),
            b"060" => Some(EngineeringChangeLevel),
            b"061" => Some(CancelIfNotDeliveredBy),
            b"062" => Some(Blueprint),
            b"063" => Some(DoNotDeliverAfter),
            b"064" => Some(DoNotDeliverBefore),
            b"065" => Some(Code065),
            b"066" => Some(Code066),
            b"067" => Some(CurrentScheduleDelivery),
            b"068" => Some(CurrentScheduleShip),
            b"069" => Some(PromisedForDelivery),
            b"070" => Some(Code070),
            b"071" => Some(Code071),
            b"072" => Some(Code072),
            b"073" => Some(Code073),
            b"074" => Some(Code074),
            b"075" => Some(Code075),
            b"076" => Some(Code076),
            b"077" => Some(Code077),
            b"078" => Some(Code078),
            b"079" => Some(PromisedForShipment),
            b"080" => Some(Code080),
            b"081" => Some(Code081),
            b"082" => Some(Code082),
            b"083" => Some(Code083),
            b"084" => Some(Code084),
            b"085" => Some(Code085),
            b"086" => Some(Code086),
            b"087" => Some(Code087),
            b"088" => Some(Code088),
            b"089" => Some(Inquiry),
            b"090" => Some(ReportStart),
            b"091" => Some(ReportEnd),
            b"092" => Some(ContractEffective),
            b"093" => Some(ContractExpiration),
            b"094" => Some(Manufacture),
            b"095" => Some(BillOfLading),
            b"096" => Some(Discharge),
            b"097" => Some(TransactionCreation),
            b"098" => Some(Code098),
            b"099" => Some(Code099),
            b"100" => Some(NoShippingScheduleEstablishedAsOf),
            b"101" => Some(NoProductionScheduleEstablishedAsOf),
            b"102" => Some(Issue),
            b"103" => Some(Award),
            b"104" => Some(SystemSurvey),
            b"105" => Some(QualityRating),
            b"106" => Some(RequiredBy),
            b"107" => Some(Deposit),
            b"108" => Some(Postmark),
            b"109" => Some(ReceivedAtLockbox),
            b"110" => Some(OriginallyScheduledShip),
            b"111" => Some(ManifestShipNotice),
            b"112" => Some(BuyersDock),
            b"113" => Some(SampleRequired),
            b"114" => Some(ToolingRequired),
            b"115" => Some(SampleAvailable),
            b"116" => Some(ScheduledInterchangeDelivery),
            b"118" => Some(RequestedPickup),
            b"119" => Some(TestPerformed),
            b"120" => Some(ControlPlan),
            b"121" => Some(FeasibilitySignOff),
            b"122" => Some(FailureModeEffective),
            b"124" => Some(GroupContractEffective),
            b"125" => Some(GroupContractExpiration),
            b"126" => Some(WholesaleContractEffective),
            b"127" => Some(WholesaleContractExpiration),
            b"128" => Some(ReplacementEffective),
            b"129" => Some(CustomerContractEffective),
            b"130" => Some(CustomerContractExpiration),
            b"131" => Some(ItemContractEffective),
            b"132" => Some(ItemContractExpiration),
            b"133" => Some(AccountsReceivableStatementDate),
            b"134" => Some(ReadyForInspection),
            b"135" => Some(Booking),
            b"136" => Some(TechnicalRating),
            b"137" => Some(DeliveryRating),
            b"138" => Some(CommercialRating),
            b"139" => Some(Estimated),
            b"140" => Some(Actual),
            b"141" => Some(Assigned),
            b"142" => Some(Loss),
            b"143" => Some(DueDateOfFirstPaymentToPrincipalAndInterest),
            b"144" => Some(EstimatedAcceptance),
            b"145" => Some(OpeningDate),
            b"146" => Some(ClosingDate),
            b"147" => Some(DueDateLastCompleteInstallmentPaid),
            b"148" => {
                Some(DateOfLocalOfficeApprovalOfConveyanceOfDamagedRealEstateProperty)
            }
            b"149" => Some(DateDeedFiledForRecord),
            b"150" => Some(ServicePeriodStart),
            b"151" => Some(ServicePeriodEnd),
            b"152" => Some(EffectiveDateOfChange),
            b"153" => Some(ServiceInterruption),
            b"154" => Some(AdjustmentPeriodStart),
            b"155" => Some(AdjustmentPeriodEnd),
            b"156" => Some(AllotmentPeriodStart),
            b"157" => Some(TestPeriodStart),
            b"158" => Some(TestPeriodEnding),
            b"159" => Some(BidPriceException),
            b"160" => Some(SamplesToBeReturnedBy),
            b"161" => Some(LoadedOnVessel),
            b"162" => Some(PendingArchive),
            b"163" => Some(ActualArchive),
            b"164" => Some(FirstIssue),
            b"165" => Some(FinalIssue),
            b"166" => Some(Message),
            b"167" => Some(Code167),
            b"168" => Some(Release),
            b"169" => Some(ProductAvailabilityDate),
            b"170" => Some(SupplementalIssue),
            b"171" => Some(Revision),
            b"172" => Some(Correction),
            b"173" => Some(WeekEnding),
            b"174" => Some(MonthEnding),
            b"175" => Some(CancelIfNotShippedBy),
            b"176" => Some(ExpeditedOn),
            b"177" => Some(Cancellation),
            b"178" => Some(Code178),
            b"179" => Some(Code179),
            b"180" => Some(Code180),
            b"181" => Some(Code181),
            b"182" => Some(Code182),
            b"183" => Some(Connection),
            b"184" => Some(Inventory),
            b"185" => Some(VesselRegistry),
            b"186" => Some(InvoicePeriodStart),
            b"187" => Some(InvoicePeriodEnd),
            b"188" => Some(CreditAdvice),
            b"189" => Some(DebitAdvice),
            b"190" => Some(ReleasedToVessel),
            b"191" => Some(MaterialSpecification),
            b"192" => Some(DeliveryTicket),
            b"193" => Some(PeriodStart),
            b"194" => Some(PeriodEnd),
            b"195" => Some(ContractReOpen),
            b"196" => Some(Start),
            b"197" => Some(End),
            b"198" => Some(Completion),
            b"199" => Some(Seal),
            b"200" => Some(AssemblyStart),
            b"201" => Some(Acceptance),
            b"202" => Some(MasterLeaseAgreement),
            b"203" => Some(FirstProduced),
            b"204" => Some(Code204),
            b"205" => Some(Transmitted),
            b"206" => Some(Code206),
            b"207" => Some(Code207),
            b"208" => Some(LotNumberExpiration),
            b"209" => Some(ContractPerformanceStart),
            b"210" => Some(ContractPerformanceDelivery),
            b"211" => Some(ServiceRequested),
            b"212" => Some(ReturnedToCustomer),
            b"213" => Some(AdjustmentToBillDated),
            b"214" => Some(DateOfRepairService),
            b"215" => Some(InterruptionStart),
            b"216" => Some(InterruptionEnd),
            b"217" => Some(Spud),
            b"218" => Some(InitialCompletion),
            b"219" => Some(PluggedAndAbandoned),
            b"220" => Some(Penalty),
            b"221" => Some(PenaltyBegin),
            b"222" => Some(Birth),
            b"223" => Some(BirthCertificate),
            b"224" => Some(Adoption),
            b"225" => Some(Christening),
            b"226" => Some(LeaseCommencement),
            b"227" => Some(LeaseTermStart),
            b"228" => Some(LeaseTermEnd),
            b"229" => Some(RentStart),
            b"230" => Some(Installation),
            b"231" => Some(ProgressPayment),
            b"232" => Some(ClaimStatementPeriodStart),
            b"233" => Some(ClaimStatementPeriodEnd),
            b"234" => Some(SettlementDate),
            b"235" => Some(Code235),
            b"236" => Some(LenderCreditCheck),
            b"237" => Some(StudentSigned),
            b"238" => Some(ScheduleRelease),
            b"239" => Some(Baseline),
            b"240" => Some(BaselineStart),
            b"241" => Some(BaselineComplete),
            b"242" => Some(ActualStart),
            b"243" => Some(ActualComplete),
            b"244" => Some(EstimatedStart),
            b"245" => Some(EstimatedCompletion),
            b"246" => Some(StartNoEarlierThan),
            b"247" => Some(StartNoLaterThan),
            b"248" => Some(FinishNoLaterThan),
            b"249" => Some(FinishNoEarlierThan),
            b"250" => Some(Code250),
            b"251" => Some(Code251),
            b"252" => Some(EarlyStart),
            b"253" => Some(EarlyFinish),
            b"254" => Some(LateStart),
            b"255" => Some(LateFinish),
            b"256" => Some(ScheduledStart),
            b"257" => Some(ScheduledFinish),
            b"258" => Some(OriginalEarlyStart),
            b"259" => Some(OriginalEarlyFinish),
            b"260" => Some(RestDay),
            b"261" => Some(RestStart),
            b"262" => Some(RestFinish),
            b"263" => Some(Holiday),
            b"264" => Some(HolidayStart),
            b"265" => Some(HolidayFinish),
            b"266" => Some(Base),
            b"267" => Some(Timenow),
            b"268" => Some(EndDateOfSupport),
            b"269" => Some(DateAccountMatures),
            b"270" => Some(DateFiled),
            b"271" => Some(PenaltyEnd),
            b"272" => Some(ExitPlantDate),
            b"273" => Some(LatestOnBoardCarrierDate),
            b"274" => Some(RequestedDepartureDate),
            b"275" => Some(Approved),
            b"276" => Some(ContractStart),
            b"277" => Some(ContractDefinition),
            b"278" => Some(LastItemDelivery),
            b"279" => Some(ContractCompletion),
            b"280" => Some(DateCourseOfOrthodonticsTreatmentBeganOrIsExpectedToBegin),
            b"281" => Some(OverTargetBaselineMonth),
            b"282" => Some(PreviousReport),
            b"283" => Some(FundsAppropriationStart),
            b"284" => Some(FundsAppropriationEnd),
            b"285" => Some(EmploymentOrHire),
            b"286" => Some(Retirement),
            b"287" => Some(Medicare),
            b"288" => Some(Code288),
            b"289" => Some(PremiumPaidToDate),
            b"290" => Some(CoordinationOfBenefits),
            b"291" => Some(Plan),
            b"292" => Some(Benefit),
            b"293" => Some(Education),
            b"294" => Some(EarningsEffectiveDate),
            b"295" => Some(PrimaryCareProvider),
            b"296" => Some(InitialDisabilityPeriodReturnToWork),
            b"297" => Some(InitialDisabilityPeriodLastDayWorked),
            b"298" => Some(LatestAbsence),
            b"299" => Some(Illness),
            b"300" => Some(EnrollmentSignatureDate),
            b"301" => Some(Code301),
            b"302" => Some(Maintenance),
            b"303" => Some(MaintenanceEffective),
            b"304" => Some(LatestVisitOrConsultation),
            b"305" => Some(NetCreditServiceDate),
            b"306" => Some(AdjustmentEffectiveDate),
            b"307" => Some(Eligibility),
            b"308" => Some(PreAwardSurvey),
            b"309" => Some(PlanTermination),
            b"310" => Some(DateOfClosing),
            b"311" => Some(LatestReceivingDateCutoffDate),
            b"312" => Some(SalaryDeferral),
            b"313" => Some(Cycle),
            b"314" => Some(Disability),
            b"315" => Some(Offset),
            b"316" => Some(PriorIncorrectDateOfBirth),
            b"317" => Some(CorrectedDateOfBirth),
            b"318" => Some(Added),
            b"319" => Some(Failed),
            b"320" => Some(DateForeclosureProceedingsInstituted),
            b"321" => Some(Purchased),
            b"322" => Some(PutIntoService),
            b"323" => Some(Replaced),
            b"324" => Some(Returned),
            b"325" => Some(DisbursementDate),
            b"326" => Some(GuaranteeDate),
            b"327" => Some(QuarterEnding),
            b"328" => Some(Changed),
            b"329" => Some(Terminated),
            b"330" => Some(ReferralDate),
            b"331" => Some(EvaluationDate),
            b"332" => Some(PlacementDate),
            b"333" => Some(Code333),
            b"334" => Some(ReEvaluationDate),
            b"335" => Some(DismissalDate),
            b"336" => Some(EmploymentBegin),
            b"337" => Some(EmploymentEnd),
            b"338" => Some(MedicareBegin),
            b"339" => Some(MedicareEnd),
            b"340" => Some(Code340),
            b"341" => Some(Code341),
            b"342" => Some(PremiumPaidToDateBegin),
            b"343" => Some(PremiumPaidToDateEnd),
            b"344" => Some(CoordinationOfBenefitsBegin),
            b"345" => Some(CoordinationOfBenefitsEnd),
            b"346" => Some(PlanBegin),
            b"347" => Some(PlanEnd),
            b"348" => Some(BenefitBegin),
            b"349" => Some(BenefitEnd),
            b"350" => Some(EducationBegin),
            b"351" => Some(EducationEnd),
            b"352" => Some(PrimaryCareProviderBegin),
            b"353" => Some(PrimaryCareProviderEnd),
            b"354" => Some(IllnessBegin),
            b"355" => Some(IllnessEnd),
            b"356" => Some(EligibilityBegin),
            b"357" => Some(EligibilityEnd),
            b"358" => Some(CycleBegin),
            b"359" => Some(CycleEnd),
            b"360" => Some(InitialDisabilityPeriodStart),
            b"361" => Some(InitialDisabilityPeriodEnd),
            b"362" => Some(OffsetBegin),
            b"363" => Some(OffsetEnd),
            b"364" => Some(PlanPeriodElectionBegin),
            b"365" => Some(PlanPeriodElectionEnd),
            b"366" => Some(PlanPeriodElection),
            b"367" => Some(DueToCustomer),
            b"368" => Some(Submittal),
            b"369" => Some(EstimatedDepartureDate),
            b"370" => Some(ActualDepartureDate),
            b"371" => Some(EstimatedArrivalDate),
            b"372" => Some(ActualArrivalDate),
            b"373" => Some(OrderStart),
            b"374" => Some(OrderEnd),
            b"375" => Some(DeliveryStart),
            b"376" => Some(DeliveryEnd),
            b"377" => Some(ContractCostsThrough),
            b"378" => Some(FinancialInformationSubmission),
            b"379" => Some(BusinessTermination),
            b"380" => Some(ApplicantSigned),
            b"381" => Some(CosignerSigned),
            b"382" => Some(Enrollment),
            b"383" => Some(AdjustedHire),
            b"384" => Some(CreditedService),
            b"385" => Some(CreditedServiceBegin),
            b"386" => Some(CreditedServiceEnd),
            b"387" => Some(DeferredDistribution),
            b"388" => Some(PaymentCommencement),
            b"389" => Some(PayrollPeriod),
            b"390" => Some(PayrollPeriodBegin),
            b"391" => Some(PayrollPeriodEnd),
            b"392" => Some(PlanEntry),
            b"393" => Some(PlanParticipationSuspension),
            b"394" => Some(Rehire),
            b"395" => Some(Retermination),
            b"396" => Some(Termination),
            b"397" => Some(Valuation),
            b"398" => Some(VestingService),
            b"399" => Some(VestingServiceBegin),
            b"400" => Some(VestingServiceEnd),
            b"401" => Some(DuplicateBill),
            b"402" => Some(AdjustmentPromised),
            b"403" => Some(AdjustmentProcessed),
            b"404" => Some(YearEnding),
            b"405" => Some(Production),
            b"406" => Some(MaterialClassification),
            b"408" => Some(Weighed),
            b"409" => Some(DateOfDeedInLieu),
            b"410" => Some(DateOfFirmCommitment),
            b"411" => Some(ExpirationDateOfExtensionToForeclose),
            b"412" => Some(DateOfNoticeToConvey),
            b"413" => Some(DateOfReleaseOfBankruptcy),
            b"414" => Some(OptimisticEarlyStart),
            b"415" => Some(OptimisticEarlyFinish),
            b"416" => Some(OptimisticLateStart),
            b"417" => Some(OptimisticLateFinish),
            b"418" => Some(MostLikelyEarlyStart),
            b"419" => Some(MostLikelyEarlyFinish),
            b"420" => Some(MostLikelyLateStart),
            b"421" => Some(MostLikelyLateFinish),
            b"422" => Some(PessimisticEarlyStart),
            b"423" => Some(PessimisticEarlyFinish),
            b"424" => Some(PessimisticLateStart),
            b"425" => Some(PessimisticLateFinish),
            b"426" => Some(FirstPaymentDue),
            b"427" => Some(FirstInterestPaymentDue),
            b"428" => Some(SubsequentInterestPaymentDue),
            b"429" => Some(IrregularInterestPaymentDue),
            b"430" => Some(GuarantorReceived),
            b"431" => Some(OnsetOfCurrentSymptomsOrIllness),
            b"432" => Some(Submission),
            b"433" => Some(Removed),
            b"434" => Some(Statement),
            b"435" => Some(Admission),
            b"436" => Some(InsuranceCard),
            b"437" => Some(SpouseRetirement),
            b"438" => Some(OnsetOfSimilarSymptomsOrIllness),
            b"439" => Some(Accident),
            b"440" => Some(ReleaseOf),
            b"441" => Some(PriorPlacement),
            b"442" => Some(DateOfDeath),
            b"443" => Some(Code443),
            b"444" => Some(FirstVisitOrConsultation),
            b"445" => Some(InitialPlacement),
            b"446" => Some(Replacement),
            b"447" => Some(Occurrence),
            b"448" => Some(OccurrenceSpan),
            b"449" => Some(OccurrenceSpanFrom),
            b"450" => Some(OccurrenceSpanTo),
            b"451" => Some(InitialFeeDue),
            b"452" => Some(AppliancePlacement),
            b"453" => Some(AcuteManifestationOfAChronicCondition),
            b"454" => Some(InitialTreatment),
            b"455" => Some(LastXRay),
            b"456" => Some(Surgery),
            b"457" => Some(Code457),
            b"458" => Some(Certification),
            b"459" => Some(NursingHomeFrom),
            b"460" => Some(NursingHomeTo),
            b"461" => Some(LastCertification),
            b"462" => {
                Some(DateOfLocalOfficeApprovalOfConveyanceOfOccupiedRealEstateProperty)
            }
            b"463" => Some(BeginTherapy),
            b"464" => Some(OxygenTherapyFrom),
            b"465" => Some(OxygenTherapyTo),
            b"466" => Some(OxygenTherapy),
            b"467" => Some(Signature),
            b"468" => Some(PrescriptionFill),
            b"469" => Some(ProviderSignature),
            b"470" => {
                Some(
                    DateOfLocalOfficeCertificationOfConveyanceOfDamagedRealEstateProperty,
                )
            }
            b"471" => Some(Prescription),
            b"472" => Some(Service),
            b"473" => Some(MedicaidBegin),
            b"474" => Some(MedicaidEnd),
            b"475" => Some(Medicaid),
            b"476" => Some(Code476),
            b"477" => Some(Code477),
            b"478" => Some(PrescriptionFrom),
            b"479" => Some(PrescriptionTo),
            b"480" => Some(ArterialBloodGasTest),
            b"481" => Some(OxygenSaturationTest),
            b"482" => Some(PregnancyBegin),
            b"483" => Some(PregnancyEnd),
            b"484" => Some(LastMenstrualPeriod),
            b"485" => Some(InjuryBegin),
            b"486" => Some(InjuryEnd),
            b"487" => Some(NursingHome),
            b"488" => Some(CollateralDependent),
            b"489" => Some(CollateralDependentBegin),
            b"490" => Some(CollateralDependentEnd),
            b"491" => Some(SponsoredDependent),
            b"492" => Some(SponsoredDependentBegin),
            b"493" => Some(SponsoredDependentEnd),
            b"494" => Some(Deductible),
            b"495" => Some(OutOfPocket),
            b"496" => Some(ContractAuditDate),
            b"497" => Some(LatestDeliveryDateAtPier),
            b"498" => Some(MortgageeReportedCurtailmentDate),
            b"499" => Some(MortgageeOfficialSignatureDate),
            b"500" => Some(Resubmission),
            b"501" => Some(ExpectedReply),
            b"502" => Some(DroppedToLessThanHalfTime),
            b"503" => Some(RepaymentBegin),
            b"504" => Some(LoanServicingTransfer),
            b"505" => Some(LoanPurchase),
            b"506" => Some(LastNotification),
            b"507" => Some(Extract),
            b"508" => Some(Extended),
            b"509" => Some(ServicerSignatureDate),
            b"510" => Some(DatePacked),
            b"511" => Some(ShelfLifeExpiration),
            b"512" => Some(WarrantyExpiration),
            b"513" => Some(Overhauled),
            b"514" => Some(Transferred),
            b"515" => Some(Notified),
            b"516" => Some(Discovered),
            b"517" => Some(Inspected),
            b"518" => Some(Code518),
            b"519" => Some(DateBankruptcyFiled),
            b"520" => Some(DateOfDamage),
            b"521" => Some(DateHazardInsurancePolicyCancelled),
            b"522" => Some(ExpirationDateToSubmitTitleEvidence),
            b"523" => Some(DateOfClaim),
            b"524" => Some(DateOfNoticeOfReferralForAssignment),
            b"525" => Some(DateOfNoticeOfProbableIneligibilityForAssignment),
            b"526" => Some(DateOfForeclosureNotice),
            b"527" => Some(ExpirationOfForeclosureTimeframe),
            b"528" => Some(DatePossessoryActionInitiated),
            b"529" => Some(DateOfPossession),
            b"530" => Some(DateOfLastInstallmentReceived),
            b"531" => Some(DateOfAcquisitionOfTitle),
            b"532" => Some(ExpirationOfExtensionToConvey),
            b"533" => Some(DateOfAssignmentApproval),
            b"534" => Some(DateOfAssignmentRejection),
            b"535" => Some(CurtailmentDateFromAdviceOfPayment),
            b"536" => Some(ExpirationOfExtensionToSubmitFiscalData),
            b"537" => Some(Code537),
            b"538" => Some(MakegoodCommercialDate),
            b"539" => Some(PolicyEffective),
            b"540" => Some(PolicyExpiration),
            b"541" => Some(EmployeeEffectiveDateOfCoverage),
            b"542" => Some(ClaimAdministratorNotifiedOfEmployeeLegalRepresentation),
            b"543" => Some(LastPremiumPaidDate),
            b"544" => Some(EmployerKnowledgeOfTheInjury),
            b"545" => Some(ClaimAdministratorKnowledgeOfTheInjury),
            b"546" => Some(DateOfMaximumMedicalImprovement),
            b"547" => Some(DateOfLoan),
            b"548" => Some(DateOfAdvance),
            b"549" => Some(BeginningLayDate),
            b"550" => Some(CertificateEffective),
            b"551" => Some(BenefitApplicationDate),
            b"552" => Some(ActualReturnToWork),
            b"553" => Some(ReleasedReturnToWork),
            b"554" => Some(EndingLayDate),
            b"555" => Some(EmployeeWagesCeased),
            b"556" => Some(LastSalaryIncrease),
            b"557" => Some(EmployeeLaidOff),
            b"558" => Some(InjuryOrIllness),
            b"559" => Some(OldestUnpaidInstallment),
            b"560" => Some(PreforeclosureAcceptanceDate),
            b"561" => Some(PreforeclosureSaleClosingDate),
            b"562" => Some(DateOfFirstUncuredDefault),
            b"563" => Some(DateDefaultWasCured),
            b"564" => Some(DateOfFirstMortgagePayment),
            b"565" => Some(DateOfPropertyInspection),
            b"566" => Some(DateTotalAmountOfDelinquencyReported),
            b"567" => Some(DateOutstandingLoanBalanceReported),
            b"568" => Some(DateForeclosureSaleScheduled),
            b"569" => Some(DateForeclosureHeld),
            b"570" => Some(DateRedemptionPeriodEnds),
            b"571" => Some(DateVoluntaryConveyanceAccepted),
            b"572" => Some(DatePropertySold),
            b"573" => Some(DateClaimPaid),
            b"574" => Some(ActionBeginDate),
            b"575" => Some(ProjectedActionEndDate),
            b"576" => Some(ActionEndDate),
            b"577" => Some(OriginalMaturityDate),
            b"578" => Some(DateReferredToAttorneyForForeclosure),
            b"579" => Some(PlannedRelease),
            b"580" => Some(ActualRelease),
            b"581" => Some(ContractPeriod),
            b"582" => Some(ReportPeriod),
            b"583" => Some(Suspension),
            b"584" => Some(Reinstatement),
            b"585" => Some(Report),
            b"586" => Some(FirstContact),
            b"587" => Some(ProjectedForeclosureSaleDate),
            b"589" => Some(DateAssignmentFiledForRecord),
            b"590" => Some(DateOfAppraisal),
            b"591" => Some(ExpirationDateOfExtensionToAssign),
            b"592" => Some(DateOfExtensionToConvey),
            b"593" => Some(DateHazardInsurancePolicyRefused),
            b"594" => Some(HighFabricationReleaseAuthorization),
            b"595" => Some(HighRawMaterialAuthorization),
            b"596" => Some(MaterialChangeNotice),
            b"597" => Some(LatestDeliveryDateAtRailRamp),
            b"598" => Some(Rejected),
            b"599" => Some(RepaymentScheduleSent),
            b"600" => Some(AsOf),
            b"601" => Some(FirstSubmission),
            b"602" => Some(SubsequentSubmission),
            b"603" => Some(Renewal),
            b"604" => Some(Withdrawn),
            b"606" => Some(CertificationPeriodStart),
            b"607" => Some(CertificationRevision),
            b"608" => Some(Code608),
            b"609" => Some(PrearrangedDealMatch),
            b"610" => Some(ContingencyEnd),
            b"611" => Some(OxygenTherapyEvaluation),
            b"612" => Some(ShutIn),
            b"613" => Some(AllowableEffective),
            b"614" => Some(FirstSales),
            b"615" => Some(DateAcquired),
            b"616" => Some(InterviewerSigned),
            b"617" => Some(ApplicationLoggedDate),
            b"618" => Some(ReviewDate),
            b"619" => Some(DecisionDate),
            b"620" => Some(PreviouslyResided),
            b"621" => Some(Reported),
            b"622" => Some(Checked),
            b"623" => Some(Settled),
            b"624" => Some(PresentlyResiding),
            b"625" => Some(EmployedInThisPosition),
            b"626" => Some(Verified),
            b"627" => Some(SecondAdmissionDate),
            b"629" => Some(AccountOpened),
            b"630" => Some(AccountClosed),
            b"631" => Some(PropertyAcquired),
            b"632" => Some(PropertyBuilt),
            b"633" => Some(EmployedInThisProfession),
            b"634" => Some(NextReviewDate),
            b"635" => Some(InitialContactDate),
            b"636" => Some(DateOfLastUpdate),
            b"637" => Some(SecondDischargeDate),
            b"638" => Some(DateOfLastDraw),
            b"640" => Some(Complaint),
            b"641" => Some(Option),
            b"642" => Some(Solicitation),
            b"643" => Some(Clause),
            b"644" => Some(Meeting),
            b"646" => Some(RentalPeriod),
            b"647" => Some(NextPayIncrease),
            b"648" => Some(PeriodCoveredBySourceDocuments),
            b"649" => Some(DocumentDue),
            b"650" => Some(CourtNotice),
            b"651" => Some(ExpectedFundingDate),
            b"652" => Some(AssignmentRecorded),
            b"653" => Some(CaseReopened),
            b"655" => Some(PreviousCourtEvent),
            b"656" => Some(LastDateToObject),
            b"657" => Some(CourtEvent),
            b"658" => Some(LastDateToFileAClaim),
            b"659" => Some(CaseConverted),
            b"660" => Some(DebtIncurred),
            b"661" => Some(Judgment),
            b"662" => Some(WagesStart),
            b"663" => Some(WagesEnd),
            b"664" => Some(DateThroughWhichPropertyTaxesHaveBeenPaid),
            b"665" => Some(PaidThroughDate),
            b"666" => Some(DatePaid),
            b"667" => Some(AnesthesiaAdministration),
            b"668" => Some(PriceProtection),
            b"669" => Some(ClaimIncurred),
            b"670" => Some(BookEntryDelivery),
            b"671" => Some(RateAdjustment),
            b"672" => Some(NextInstallmentDueDate),
            b"673" => Some(DaylightOverdraftTime),
            b"674" => Some(PresentmentDate),
            b"675" => Some(NegotiatedExtensionDate),
            b"681" => Some(Remittance),
            b"682" => Some(SecurityRateAdjustment),
            b"683" => Some(FilingPeriod),
            b"684" => Some(ReviewPeriodEnd),
            b"685" => Some(RequestedSettlement),
            b"686" => Some(LastScreening),
            b"687" => Some(Confinement),
            b"688" => Some(Arrested),
            b"689" => Some(Convicted),
            b"690" => Some(Interviewed),
            b"691" => Some(LastVisit),
            b"692" => Some(Recovery),
            b"693" => Some(TimeInUS),
            b"694" => Some(FuturePeriod),
            b"695" => Some(PreviousPeriod),
            b"696" => Some(InterestPaidTo),
            b"697" => Some(DateOfSeizure),
            b"699" => Some(Setoff),
            b"700" => Some(OverrideDateForSettlement),
            b"701" => Some(Code701),
            b"702" => Some(SendingRoadTimeStamp),
            b"703" => Some(RetransmissionTimeStamp),
            b"704" => Some(DeliveryAppointmentDateAndTime),
            b"705" => Some(InterestPaidThrough),
            b"706" => Some(DateMaterialUsageSuspended),
            b"707" => Some(LastPaymentMade),
            b"708" => Some(PastDue),
            b"709" => Some(AnalysisMonthEnding),
            b"710" => Some(DateOfSpecification),
            b"711" => Some(DateOfStandard),
            b"712" => Some(ReturnToWorkPartTime),
            b"713" => Some(PaidThroughDateForSalaryContinuation),
            b"714" => Some(PaidThroughDateForVacationPay),
            b"715" => Some(PaidThroughDateForAccruedSickPay),
            b"716" => Some(AppraisalOrdered),
            b"717" => Some(DateOfOperation),
            b"718" => Some(BestTimeToCall),
            b"719" => Some(VerbalReportNeeded),
            b"720" => Some(EstimatedEscrowClosing),
            b"721" => Some(PermitYear),
            b"722" => Some(RemodelingCompleted),
            b"723" => Some(CurrentMonthEnding),
            b"724" => Some(PreviousMonthEnding),
            b"725" => Some(CycleToDate),
            b"726" => Some(YearToDate),
            b"727" => Some(OnHold),
            b"728" => Some(OffHold),
            b"729" => Some(FacsimileDueBy),
            b"730" => Some(ReportingCycleDate),
            b"731" => Some(LastPaidInstallmentDate),
            b"732" => Some(ClaimsMade),
            b"733" => Some(DateOfLastPaymentReceived),
            b"734" => Some(CurtailmentDate),
            b"736" => Some(PoolSettlement),
            b"737" => Some(NextInterestChangeDate),
            b"738" => Some(MostRecentHemoglobinOrHematocritOrBoth),
            b"739" => Some(MostRecentSerumCreatine),
            b"740" => Some(Closed),
            b"741" => Some(Therapy),
            b"742" => Some(Implantation),
            b"743" => Some(Explantation),
            b"744" => Some(DateBecameAware),
            b"745" => Some(FirstMarketed),
            b"746" => Some(LastMarketed),
            b"747" => Some(NewDueDateOfFirstPaymentToPrincipalAndInterest),
            b"748" => Some(NewMaturityDate),
            b"749" => Some(Current),
            b"750" => Some(ExpectedProblemResolution),
            b"751" => Some(AlternateProblemResolution),
            b"752" => Some(FeeCapitalization),
            b"753" => Some(InterestCapitalization),
            b"754" => Some(NextPaymentDue),
            b"755" => Some(ConversionToRepayment),
            b"756" => Some(EndOfGracePeriod),
            b"757" => Some(SchoolRefund),
            b"758" => Some(SimpleInterestDue),
            b"759" => Some(DatePracticeCeased),
            b"760" => Some(Printed),
            b"761" => Some(DatePracticeEstablished),
            b"762" => Some(DropActionDate),
            b"764" => Some(MostRecentRenewal),
            b"765" => Some(Original),
            b"766" => Some(OutsideAuditorsReport),
            b"769" => Some(PreCertificationDate),
            b"770" => Some(BackOnMarket),
            b"771" => Some(Status),
            b"772" => Some(BenefitAdjustmentStart),
            b"773" => Some(OffMarket),
            b"774" => Some(Tour),
            b"775" => Some(BenefitAdjustmentEnd),
            b"776" => Some(ListingReceived),
            b"777" => Some(BenefitAdjustmentPeriod),
            b"778" => Some(AnticipatedClosing),
            b"779" => Some(LastPublication),
            b"780" => Some(SoldBookPublication),
            b"781" => Some(Occupancy),
            b"782" => Some(Contingency),
            b"783" => Some(PercolationTest),
            b"784" => Some(SepticApproval),
            b"785" => Some(TitleTransfer),
            b"786" => Some(OpenHouse),
            b"787" => Some(BenefitCreditPeriod),
            b"788" => Some(BenefitTransferPeriod),
            b"789" => Some(Homestead),
            b"790" => Some(Sanction),
            b"791" => Some(TailCoverageBegin),
            b"792" => Some(TailCoverageEnd),
            b"793" => Some(TrainingBegin),
            b"794" => Some(TrainingEnd),
            b"795" => Some(VerificationReceived),
            b"796" => Some(VerificationSent),
            b"797" => Some(StateResidencyDate),
            b"798" => Some(EffectiveDateOfTheRoutingFile),
            b"799" => Some(TestDataAnalysis),
            b"800" => Some(MidpointOfPerformance),
            b"801" => Some(AcquisitionDate),
            b"802" => Some(DateOfAction),
            b"803" => Some(PaidInFull),
            b"804" => Some(Refinance),
            b"805" => Some(VoluntaryTermination),
            b"806" => Some(CustomerOrder),
            b"807" => Some(Stored),
            b"808" => Some(Selected),
            b"809" => Some(Posted),
            b"810" => Some(DocumentReceived),
            b"811" => Some(Rebuilt),
            b"812" => Some(Marriage),
            b"813" => Some(CustomsEntryDate),
            b"814" => Some(PaymentDueDate),
            b"815" => Some(MaturityDate),
            b"816" => Some(TradeDate),
            b"817" => Some(Code817),
            b"818" => Some(Code818),
            b"819" => Some(LastAccountsFiledAtPublicRegistrationAgency),
            b"820" => Some(RealEstateTaxYear),
            b"821" => Some(FinalReconciliationValueEstimateAsOf),
            b"822" => Some(Map),
            b"823" => Some(Opinion),
            b"824" => Some(Version),
            b"825" => Some(OriginalDueDate),
            b"826" => Some(IncumbencyPeriod),
            b"827" => Some(AudienceDeficiencyPeriod),
            b"828" => Some(AiredDate),
            b"830" => Some(Schedule),
            b"831" => Some(PaidThroughDateForMinimumPayment),
            b"832" => Some(PaidThroughDateForTotalPayment),
            b"840" => Some(Election),
            b"841" => Some(EngineeringDataList),
            b"842" => Some(LastProduction),
            b"843" => Some(NotBefore),
            b"844" => Some(NotAfter),
            b"845" => Some(InitialClaim),
            b"846" => Some(BenefitsPaid),
            b"847" => Some(WagesEarned),
            b"848" => Some(AdjustedStart),
            b"849" => Some(AdjustedEnd),
            b"850" => Some(RevisedAdjustedStart),
            b"851" => Some(RevisedAdjustedEnd),
            b"853" => Some(FieldTest),
            b"854" => Some(MortgageNoteDate),
            b"855" => Some(AlternativeDueDate),
            b"856" => Some(FirstPaymentChange),
            b"857" => Some(FirstRateAdjustment),
            b"858" => Some(AlternateBasePeriod),
            b"859" => Some(PriorNotice),
            b"860" => Some(AppointmentEffective),
            b"861" => Some(AppointmentExpiration),
            b"862" => Some(CompanyTermination),
            b"863" => Some(ContinuingEducationRequirement),
            b"864" => Some(DistributorEffective),
            b"865" => Some(DistributorTermination),
            b"866" => Some(Examination),
            b"867" => Some(IncorporationDissolution),
            b"868" => Some(LastFollowUp),
            b"869" => Some(LicenseEffective),
            b"870" => Some(LicenseExpiration),
            b"871" => Some(LicenseRenewal),
            b"872" => Some(LicenseRequested),
            b"873" => Some(Mailed),
            b"874" => Some(PaperworkMailed),
            b"875" => Some(PreviousEmployment),
            b"876" => Some(PreviousEmploymentEnd),
            b"877" => Some(PreviousEmploymentStart),
            b"878" => Some(PreviousResidence),
            b"879" => Some(PreviousResidenceEnd),
            b"880" => Some(PreviousResidenceStart),
            b"881" => Some(Request),
            b"882" => Some(ResidentLicenseEffective),
            b"883" => Some(ResidentLicenseExpiration),
            b"884" => Some(StateTermination),
            b"885" => Some(TexasLineTermination),
            b"900" => Some(Acceleration),
            b"901" => Some(AdjustedContestability),
            b"903" => Some(ApplicationEntry),
            b"904" => Some(ApprovalOffer),
            b"905" => Some(AutomaticPremiumLoan),
            b"906" => Some(Collection),
            b"907" => Some(ConfinementEnd),
            b"908" => Some(ConfinementStart),
            b"909" => Some(Contestability),
            b"910" => Some(FlatExtraEnd),
            b"911" => Some(LastActivity),
            b"912" => Some(LastChange),
            b"913" => Some(LastEpisode),
            b"914" => Some(LastMeal),
            b"915" => Some(Loan),
            b"916" => Some(ApplicationStatus),
            b"917" => Some(Maturity),
            b"918" => Some(MedicalInformationSignature),
            b"919" => Some(MedicalInformationSystem),
            b"920" => Some(Note),
            b"921" => Some(OfferExpiration),
            b"922" => Some(OriginalReceipt),
            b"923" => Some(Placement),
            b"924" => Some(PlacementPeriodExpiration),
            b"925" => Some(Processing),
            b"926" => Some(Recapture),
            b"927" => Some(ReEntry),
            b"928" => Some(Reissue),
            b"930" => Some(Requalification),
            b"931" => Some(ReinsuranceEffective),
            b"932" => Some(ReservationOfFacility),
            b"933" => Some(SettlementStatus),
            b"934" => Some(TableRatingEnd),
            b"935" => Some(TerminationOfFacility),
            b"936" => Some(Treatment),
            b"937" => Some(DepartmentOfLaborWageDeterminationDate),
            b"938" => Some(Order),
            b"939" => Some(Resolved),
            b"940" => Some(ExecutionDate),
            b"941" => Some(CapitationPeriodStart),
            b"942" => Some(CapitationPeriodEnd),
            b"943" => Some(LastDateForAGovernmentAgencyToFileAClaim),
            b"944" => Some(AdjustmentPeriod),
            b"945" => Some(Activity),
            b"946" => Some(MailBy),
            b"947" => Some(Preparation),
            b"948" => Some(PaymentInitiated),
            b"949" => Some(PaymentEffective),
            b"950" => Some(Application),
            b"951" => Some(Reclassification),
            b"952" => Some(Code952),
            b"953" => Some(PostReclassification),
            b"954" => Some(Code954),
            b"955" => Some(Code955),
            b"956" => Some(Code956),
            b"957" => Some(Code957),
            b"960" => Some(AdjustedDeathBenefit),
            b"961" => Some(Anniversary),
            b"962" => Some(Annuitization),
            b"963" => Some(AnnuityCommencementDate),
            b"964" => Some(Bill),
            b"965" => Some(CalendarAnniversary),
            b"966" => Some(ContractMailed),
            b"967" => Some(EarlyWithdrawal),
            b"968" => Some(FiscalAnniversary),
            b"969" => Some(Income),
            b"970" => Some(InitialPremium),
            b"971" => Some(InitialPremiumEffective),
            b"972" => Some(LastPremiumEffective),
            b"973" => Some(MinimumRequiredDistribution),
            b"974" => Some(NextAnniversary),
            b"975" => Some(Notice),
            b"976" => Some(NotificationOfDeath),
            b"977" => Some(PartialAnnuitization),
            b"978" => Some(PlanAnniversary),
            b"979" => Some(PolicySurrender),
            b"980" => Some(PriorContractAnniversary),
            b"981" => Some(PriorContractIssue),
            b"982" => Some(SignatureReceived),
            b"983" => Some(Tax),
            b"984" => Some(BenefitPeriod),
            b"985" => Some(MonthToDate),
            b"986" => Some(SemiannualEnding),
            b"987" => Some(Surrender),
            b"988" => Some(PlanOfTreatmentPeriod),
            b"989" => Some(Code989),
            b"990" => Some(OriginalNameChange),
            b"992" => Some(DateRequested),
            b"993" => Some(RequestForQuotation),
            b"994" => Some(Quote),
            b"995" => Some(RecordedDate),
            b"996" => Some(RequiredDelivery),
            b"997" => Some(QuoteToBeReceivedBy),
            b"998" => Some(ContinuationOfPayStartDate),
            b"999" => Some(DocumentDate),
            b"AA1" => Some(EstimatedPointOfArrival),
            b"AA2" => Some(EstimatedPointOfDischarge),
            b"AA3" => Some(CodeAA3),
            b"AA4" => Some(CodeAA4),
            b"AA5" => Some(CodeAA5),
            b"AA6" => Some(CodeAA6),
            b"AA7" => Some(FinalScheduledPayment),
            b"AA8" => Some(ActualDischarge),
            b"AA9" => Some(AddressPeriod),
            b"AAA" => Some(ArrivalInCountry),
            b"AAB" => Some(Citation),
            b"AAC" => Some(SuspensionEffective),
            b"AAD" => Some(Crime),
            b"AAE" => Some(DischargePlanned),
            b"AAF" => Some(Draft),
            b"AAG" => Some(DueDate),
            b"AAH" => Some(Event),
            b"AAI" => Some(FirstInvolvement),
            b"AAJ" => Some(GuaranteePeriod),
            b"AAK" => Some(IncomeIncreasePeriod),
            b"AAL" => Some(InstallmentDate),
            b"AAM" => Some(LastCivilianFlight),
            b"AAN" => Some(LastFlight),
            b"AAO" => Some(LastInsuranceMedical),
            b"AAP" => Some(LastMilitaryFlight),
            b"AAQ" => Some(LastPhysical),
            b"AAR" => Some(License),
            b"AAS" => Some(MedicalCertificate),
            b"AAT" => Some(Medication),
            b"AAU" => Some(NetWorthDate),
            b"AAV" => Some(NextActivity),
            b"AAW" => Some(OwnershipChange),
            b"AAX" => Some(OwnershipPeriod),
            b"AAY" => Some(RateDate),
            b"AAZ" => Some(RequestedContract),
            b"AB1" => Some(RequestedOffer),
            b"AB2" => Some(SalesPeriod),
            b"AB3" => Some(TaxYear),
            b"AB4" => Some(TimePeriod),
            b"AB5" => Some(Travel),
            b"AB6" => Some(TreatmentEnd),
            b"AB7" => Some(TreatmentStart),
            b"AB8" => Some(Trust),
            b"AB9" => Some(WorstTimeToCall),
            b"ABA" => Some(Registration),
            b"ABB" => Some(Revoked),
            b"ABC" => Some(EstimatedDateOfBirth),
            b"ABD" => Some(LastAnnualReport),
            b"ABE" => Some(LegalActionStarted),
            b"ABF" => Some(Lien),
            b"ABG" => Some(PaymentPeriod),
            b"ABH" => Some(ProfitPeriod),
            b"ABI" => Some(Registered),
            b"ABK" => Some(Consolidated),
            b"ABL" => Some(BoardOfDirectorsNotAuthorizedAsOf),
            b"ABM" => Some(BoardOfDirectorsIncompleteAsOf),
            b"ABN" => Some(ManagerNotRegisteredAsOf),
            b"ABO" => Some(CitizenshipChange),
            b"ABP" => Some(Participation),
            b"ABQ" => Some(Capitalization),
            b"ABR" => Some(RegistrationOfBoardOfDirectors),
            b"ABS" => Some(CeasedOperations),
            b"ABT" => Some(Satisfied),
            b"ABU" => Some(TermsMet),
            b"ABV" => Some(AssetDocumentationExpiration),
            b"ABW" => Some(CreditDocumentationExpiration),
            b"ABX" => Some(IncomeDocumentationExpiration),
            b"ABY" => Some(ProductHeldUntil),
            b"ACA" => Some(ImmigrationDate),
            b"ACB" => Some(EstimatedImmigrationDate),
            b"ACC" => Some(CurrentDisabilityPeriodStart),
            b"ACD" => Some(CurrentDisabilityPeriodEnd),
            b"ACE" => Some(CurrentDisabilityPeriodLastDayWorked),
            b"ACF" => Some(BenefitTypeGrossWeeklyAmountEffective),
            b"ACG" => Some(BenefitTypeNetWeeklyAmountEffective),
            b"ACH" => Some(BenefitTypePeriodStart),
            b"ACI" => Some(BenefitTypePeriodEnd),
            b"ACJ" => Some(BenefitDebitStart),
            b"ACK" => Some(Acknowledgment),
            b"ACL" => Some(BenefitDebitEnd),
            b"ACM" => Some(BenefitCreditStart),
            b"ACN" => Some(BenefitCreditEnd),
            b"ACO" => Some(BenefitTransferStart),
            b"ACP" => Some(BenefitTransferEnd),
            b"ACQ" => Some(WageEffective),
            b"ACR" => Some(FullDenialEffective),
            b"ACS" => Some(FullDenialRescission),
            b"ACT" => Some(PaymentIssue),
            b"ACU" => Some(PaymentPeriodStart),
            b"ACV" => Some(PaymentPeriodEnd),
            b"ACW" => Some(EmployerReportedInjuryToClaimAdministrator),
            b"ACX" => Some(SurveyYear),
            b"ACZ" => Some(ControvertDate),
            b"ADA" => Some(BilledThrough),
            b"ADB" => Some(BusinessControlChange),
            b"ADC" => Some(CourtRegistration),
            b"ADD" => Some(AnnualReportDue),
            b"ADE" => Some(ClaimNotificationReceived),
            b"ADF" => Some(ConversionPrivilegeEnd),
            b"ADG" => Some(DividendApplied),
            b"ADH" => Some(InForce),
            b"ADI" => Some(PaidUp),
            b"ADJ" => Some(PremiumChange),
            b"ADK" => Some(PolicyEffectiveOnOrBefore),
            b"ADL" => Some(AssetAndLiabilitySchedule),
            b"ADM" => Some(AnnualReportMailed),
            b"ADN" => Some(PolicyEffectiveOnOrAfter),
            b"ADR" => Some(AnnualReportFiled),
            b"ADS" => Some(AuditPeriodAfterSplitDate),
            b"ADT" => Some(AuditPeriodPriorToSplitDate),
            b"ADU" => Some(ExposureSourcePeriod),
            b"ADV" => Some(SubcontractorPeriodOfHire),
            b"ADW" => Some(Divorce),
            b"ADX" => Some(PowerOfAttorney),
            b"ADY" => Some(UniformGiftsToMinorsAccountEstablished),
            b"AEA" => Some(MedicarePartAEligibilityBeginDate),
            b"AEB" => Some(MedicarePartAEligibilityEndDate),
            b"AEC" => Some(MedicarePartACoverageEffectiveDate),
            b"AED" => Some(MedicarePartATerminationDate),
            b"AEE" => Some(MedicarePartBEligibilityBeginDate),
            b"AEF" => Some(MedicarePartBEligibilityEndDate),
            b"AEG" => Some(MedicarePartBCoverageEffectiveDate),
            b"AEH" => Some(MedicarePartBTerminationDate),
            b"AEI" => Some(LoadingPeriod),
            b"AEK" => Some(DateOnWhichAssetsJudgedInsufficientToPayCreditors),
            b"AEL" => Some(EmployeesTemporarilyLaidOffBeginPeriod),
            b"AEM" => Some(EmployeesTemporarilyLaidOffEndPeriod),
            b"AEN" => Some(FirstPublished),
            b"AEO" => Some(ForecastPeriodStart),
            b"AEP" => Some(ForecastPeriodEnd),
            b"AEQ" => Some(InvestigationStart),
            b"AER" => Some(InvestigationEnd),
            b"AES" => Some(LastPublished),
            b"AET" => Some(LatestBalanceSheet),
            b"AEU" => Some(SharePrice),
            b"AEV" => Some(StopDistribution),
            b"AEW" => Some(MaximumCreditDate),
            b"AEX" => Some(FoundingDate),
            b"AEY" => Some(RepaymentPlanStartDate),
            b"AFA" => Some(MedicarePartDEligibilityBeginDate),
            b"AFB" => Some(MedicarePartDEligibilityEndDate),
            b"AFC" => Some(MedicarePartDCoverageEffectiveDate),
            b"AFD" => Some(MedicarePartDTerminationDate),
            b"ARD" => Some(AnnualReportDelinquency),
            b"AWH" => Some(WithheldDate),
            b"BAA" => Some(ComplianceAudit),
            b"BAB" => Some(ContractorSafetyPerformanceEvaluation),
            b"BAC" => Some(ContractorSafetyProceduresReview),
            b"BAD" => Some(DateOfEquipmentInspection),
            b"BAE" => Some(DateOfSafetyInspection),
            b"BAF" => Some(EmployeesParticipationPlanReview),
            b"BAG" => Some(CodeBAG),
            b"BAH" => Some(CodeBAH),
            b"BAI" => Some(ExpectedCompletionOfChangesResultingFromHazardReview),
            b"BAJ" => Some(HazardReviewCompletion),
            b"BAK" => Some(HotWorkPermitProceduresReview),
            b"BAL" => Some(Investigation),
            b"BAM" => Some(MaintenanceProceduresReview),
            b"BAN" => Some(ManagementOfChangeProceduresReview),
            b"BAO" => Some(OperatingProceduresReview),
            b"BAP" => Some(SafetyInformationReview),
            b"BAQ" => Some(Training),
            b"BAR" => Some(TrainingProgramReview),
            b"BED" => Some(BillbackEndDate),
            b"BEE" => Some(ProgramPerformanceEndDate),
            b"BES" => Some(ProgramPerformanceStartDate),
            b"BGP" => Some(BeginningOfGracePeriod),
            b"BIA" => Some(BillingActivities),
            b"BIP" => Some(BeginningOfInterestPaidAfterClaim),
            b"BSD" => Some(BillbackStartDate),
            b"CAD" => Some(ChangedAccountingDate),
            b"CCR" => Some(CustomsCargoRelease),
            b"CDD" => Some(ContractDefinitizationDate),
            b"CDE" => Some(CampaignEndDate),
            b"CSE" => Some(CampaignEndDate),
            b"CDS" => Some(CampaignStartDate),
            b"CSD" => Some(CampaignStartDate),
            b"CDT" => Some(MaintenanceComment),
            b"CEA" => Some(Formation),
            b"CEB" => Some(Continuance),
            b"CEC" => Some(Merger),
            b"CED" => Some(YearDue),
            b"CEE" => Some(NextAnnualMeeting),
            b"CEF" => Some(EndOfLastFiscalYear),
            b"CEH" => Some(YearBeginning),
            b"CEJ" => Some(StartedDoingBusiness),
            b"CEK" => Some(SwornAndSubscribed),
            b"CEL" => Some(CalendarYear),
            b"CEM" => Some(Asset),
            b"CEN" => Some(Inactivity),
            b"CEO" => Some(HighCapitalYear),
            b"CLO" => Some(ClosingDateOfFirstBalanceSheet),
            b"CLU" => Some(ClosedUntil),
            b"COM" => Some(Compliance),
            b"CON" => Some(ConvertedIntoHoldingCompany),
            b"COS" => Some(CareOfSuppliesInStorageInspectionDate),
            b"CPA" => Some(ConsumerProductAvailabilityDate),
            b"CPD" => Some(ConsumerProductInformationPublicationDate),
            b"CPE" => Some(ConsumerProductVariantEndEffectiveDate),
            b"CPF" => Some(ConsumerProductVariantDiscontinuedDate),
            b"CPG" => Some(ConsumerProductVariantCancelledDate),
            b"CPS" => Some(ConsumerProductVariantStartEffectiveDate),
            b"CRV" => Some(ClaimRevised),
            b"CUR" => Some(CurrentList),
            b"CVD" => Some(CommunityVisibility),
            b"DAF" => Some(AccountFrozen),
            b"DDO" => Some(Declaration),
            b"DEE" => Some(DeedNotAvailable),
            b"DEL" => Some(Delete),
            b"DET" => Some(DetrimentalInformationReceived),
            b"DFF" => Some(Deferral),
            b"DFS" => Some(DepartureFromSpecification),
            b"DIL" => Some(CodeDIL),
            b"DIP" => Some(DelayedInterestPaidThrough),
            b"DIS" => Some(Disposition),
            b"DLC" => Some(DateOfLastContact),
            b"DOA" => Some(DateOfAbandonment),
            b"DOD" => Some(DateOfDelinquency),
            b"DOI" => Some(DeliveryOrderIssued),
            b"DOR" => Some(Repossession),
            b"DSP" => Some(Disposal),
            b"DTC" => Some(DeedAndTitleReceived),
            b"DTD" => Some(TechnicalDataSupplyBy),
            b"DTQ" => Some(DeedAndTitleRequested),
            b"E01" => Some(TenureDecision),
            b"E02" => Some(MostRecentPositionChange),
            b"E03" => Some(FeePayment),
            b"E04" => Some(StartDateForContinuousEmployment),
            b"E05" => Some(StartDateForCurrentPosition),
            b"E06" => Some(StartDateForOriginalPosition),
            b"E07" => Some(FiscalYear),
            b"EAD" => Some(EndAvailabilityDate),
            b"ECD" => Some(EstimatedConstructionDate),
            b"ECF" => Some(EstimatedCompletionFirstPriorMonth),
            b"ECS" => Some(EstimatedCompletionSecondPriorMonth),
            b"ECT" => Some(EstimatedCompletionThirdPriorMonth),
            b"EDA" => Some(Affirmed),
            b"EDB" => Some(Auction),
            b"EDC" => Some(Authorized),
            b"EDD" => Some(Contribution),
            b"EDE" => Some(Executed),
            b"EDF" => Some(Forgiven),
            b"EDG" => Some(Presented),
            b"EDH" => Some(LegislativeSession),
            b"EDI" => Some(Organized),
            b"EDJ" => Some(Pledged),
            b"EDK" => Some(PrimaryElection),
            b"EDL" => Some(Qualified),
            b"EDM" => Some(Refunded),
            b"EDN" => Some(Rescinded),
            b"EDO" => Some(RestructuredFrom),
            b"EDP" => Some(Vote),
            b"EKD" => Some(EmployerKnowledgeOfTheDisability),
            b"EMM" => Some(EndDateMaximumBuyingQuantity),
            b"END" => Some(EndDateMinimumBuyingQuantity),
            b"EPP" => Some(EstimatePreparation),
            b"ESC" => Some(EstimateComment),
            b"ESD" => Some(EffectiveStartDate),
            b"ESF" => Some(EstimatedStartFirstPriorMonth),
            b"ESS" => Some(EstimatedStartSecondPriorMonth),
            b"EST" => Some(EstimatedStartThirdPriorMonth),
            b"ETP" => Some(EarliestFilingPeriod),
            b"EXO" => Some(Exposure),
            b"EXP" => Some(Export),
            b"FAC" => Some(FacilityLatestBillingAction),
            b"FEB" => Some(FacilityEarliestBillingAction),
            b"FFI" => Some(Financial),
            b"FFO" => Some(FirstOrder),
            b"FIA" => Some(FinalInterestAccrual),
            b"FPE" => Some(FundingPeriodEnd),
            b"FPS" => Some(FundingPeriodStart),
            b"FSD" => Some(FirstAvailableShipDate),
            b"FSE" => Some(FreeServiceCallEndDate),
            b"FSS" => Some(FreeServiceCallStartDate),
            b"GRD" => Some(Graduated),
            b"HHD" => Some(HomeHealthDateOfEarliestBillableAction),
            b"HHE" => Some(HomeHealthEpisode),
            b"HHL" => Some(HomeHealthDateOfLatestBillableAction),
            b"HTD" => Some(HostTrainDate),
            b"ICF" => Some(ConvertedToElectronicDate),
            b"IDG" => Some(InsolvencyDischargeGranted),
            b"IFH" => Some(InitialFederalHousingAuthorityClaimPayment),
            b"III" => Some(Incorporation),
            b"ILU" => Some(ImageLastUpdateDate),
            b"IMB" => Some(ImbalancePeriod),
            b"IMP" => Some(Import),
            b"INC" => Some(Incident),
            b"INT" => Some(InactiveUntil),
            b"IPS" => Some(InterestOnPresaleStart),
            b"IVA" => Some(InitialVeteransAdministrationClaimPayment),
            b"KEV" => Some(KeyEventFiscalYear),
            b"KEW" => Some(KeyEventCalendarYear),
            b"LAM" => Some(LastAnnualMeeting),
            b"LAS" => Some(LastCheckForBalanceSheetUpdate),
            b"LCC" => Some(LastCapitalChange),
            b"LEA" => Some(LetterOfAgreement),
            b"LEL" => Some(LetterOfLiability),
            b"LIQ" => Some(Liquidation),
            b"LLP" => Some(LowPeriod),
            b"LOG" => Some(EquipmentLogEntry),
            b"LPC" => Some(ListPriceChange),
            b"LSC" => Some(LegalStructureChange),
            b"LSD" => Some(LastSubmissionDate),
            b"LTP" => Some(LatestFilingPeriod),
            b"MRR" => Some(MeterReading),
            b"MSD" => Some(LatestMaterialSafetyDataSheetDate),
            b"NAM" => Some(PresentName),
            b"NFD" => Some(NegotiatedFinish),
            b"NOD" => Some(CodeNOD),
            b"NRG" => Some(NotRegistered),
            b"NSD" => Some(NegotiatedStart),
            b"OCD" => Some(OrganicCertificationDate),
            b"ORG" => Some(OriginalList),
            b"PBC" => Some(PresentControl),
            b"PCP" => Some(PrimaryCoverageClaimPaid),
            b"PCS" => Some(PrimaryCoverageClaimSubmission),
            b"PCT" => Some(PriceChangesAllowedFromDate),
            b"PCU" => Some(PriceChangesAllowedToDate),
            b"PDA" => Some(ProductDiscontinuedButStillAvailable),
            b"PDE" => Some(PartialDenialEffective),
            b"PDR" => Some(PartialDenialRescission),
            b"PDS" => Some(CorrectProgramStartDate),
            b"PDT" => Some(CorrectProgramEndDate),
            b"PDU" => Some(CorrectContractStartDate),
            b"PDV" => Some(PrivilegeDetailsVerification),
            b"PDW" => Some(CorrectContractEndDate),
            b"PED" => Some(ProgramEndDate),
            b"PIS" => Some(ProductImageStartDate),
            b"PLS" => Some(PresentLegalStructure),
            b"PME" => Some(PackagingMaterialEffectiveDate),
            b"PPC" => Some(PoolPolicyClaimSubmission),
            b"PPD" => Some(PostPaidDate),
            b"PPP" => Some(PeakPeriod),
            b"PRD" => Some(PreviouslyReportedDateOfBirth),
            b"PRR" => Some(PresentedToReceivers),
            b"PSA" => Some(PropertySaleApproved),
            b"PSC" => Some(PropertySaleClosed),
            b"PSD" => Some(ProgramStartDate),
            b"PSF" => Some(PropertySaleConfirmation),
            b"PTD" => Some(PaidToDate),
            b"PTO" => Some(PlannedObsolescenceItem),
            b"PUD" => Some(PickUpDate),
            b"RAP" => Some(ReceiverAppointed),
            b"REM" => Some(RemanufactureDate),
            b"RES" => Some(Resigned),
            b"RFD" => Some(RequestedFinish),
            b"RFF" => Some(RecoveryFinish),
            b"RFO" => Some(ReferredFrom),
            b"RNT" => Some(RentSurvey),
            b"RRM" => Some(ReceivedInTheMail),
            b"RRT" => Some(Revocation),
            b"RSD" => Some(RequestedStart),
            b"RSS" => Some(RecoveryStart),
            b"RTO" => Some(ReferredTo),
            b"SCV" => Some(SocialSecurityClaimsVerification),
            b"SDD" => Some(SoleDirectorshipDate),
            b"SDM" => Some(StartDateMaximumBuyingQuantity),
            b"SFH" => Some(SubsequentFederalHousingAuthorityClaimPayment),
            b"SMB" => Some(StartDateMinimumBuyingQuantity),
            b"SPD" => Some(InitialSupportDate),
            b"SPE" => Some(SuggestedRetailPriceEffectiveDate),
            b"STN" => Some(Transition),
            b"SVA" => Some(SubsequentVeteransAdministrationClaimPayment),
            b"TSR" => Some(TradeStyleRegistered),
            b"TSS" => Some(TrialStarted),
            b"TST" => Some(TrialSet),
            b"TTD" => Some(TenantTrainDate),
            b"VAT" => Some(CodeVAT),
            b"VLU" => Some(ValidUntil),
            b"W01" => Some(SampleCollected),
            b"W02" => Some(StatusChange),
            b"W03" => Some(ConstructionStart),
            b"W05" => Some(Recompletion),
            b"W06" => Some(LastLogged),
            b"W07" => Some(WellLogRun),
            b"W08" => Some(SurfaceCasingAuthorityApproval),
            b"W09" => Some(ReachedTotalDepth),
            b"W10" => Some(SpacingOrderUnitAssigned),
            b"W11" => Some(RigArrival),
            b"W12" => Some(LocationExceptionOrderNumberAssigned),
            b"W13" => Some(SidetrackedWellbore),
            b"WAA" => Some(TimeEmployeeBeganWork),
            b"WAY" => Some(Waybill),
            b"XX1" => Some(OrderDay),
            b"XX2" => Some(DeliveryDay),
            b"XX3" => Some(OrderCutOffTime),
            b"XX5" => Some(ActiveItem),
            b"XX6" => Some(MatureItem),
            b"XX7" => Some(IntroductoryItem),
            b"XX8" => Some(ObsoleteItem),
            b"Y11" => Some(BestBeforeDate),
            b"Y12" => Some(HarvestDate),
            b"YXX" => Some(ProgrammedFiscalYear),
            b"YXY" => Some(ProgrammedCalendarYear),
            b"ZZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use DateTimeQualifier::*;
        match self {
            CancelAfter => "Cancel After",
            DeliveryRequested => "Delivery Requested",
            Invoice => "Invoice",
            PurchaseOrder => "Purchase Order",
            Sailing => "Sailing",
            Sold => "Sold",
            Effective => "Effective",
            PurchaseOrderReceived => "Purchase Order Received",
            Process => "Process",
            RequestedShip => "Requested Ship",
            Shipped => "Shipped",
            TermsDiscountDue => "Terms Discount Due",
            TermsNetDue => "Terms Net Due",
            DeferredPayment => "Deferred Payment",
            PromotionStart => "Promotion Start",
            PromotionEnd => "Promotion End",
            EstimatedDelivery => "Estimated Delivery",
            Available => "Available",
            Unloaded => "Unloaded",
            Check => "Check",
            ChargeBack => "Charge Back",
            FreightBill => "Freight Bill",
            PromotionOrderStart => "Promotion Order - Start",
            PromotionOrderEnd => "Promotion Order - End",
            PromotionShipStart => "Promotion Ship - Start",
            PromotionShipEnd => "Promotion Ship - End",
            PromotionRequestedDeliveryStart => "Promotion Requested Delivery - Start",
            PromotionRequestedDeliveryEnd => "Promotion Requested Delivery - End",
            PromotionPerformanceStart => "Promotion Performance - Start",
            PromotionPerformanceEnd => "Promotion Performance - End",
            PromotionInvoicePerformanceStart => "Promotion Invoice Performance - Start",
            PromotionInvoicePerformanceEnd => "Promotion Invoice Performance - End",
            PromotionFloorStockProtectStart => "Promotion Floor Stock Protect - Start",
            PromotionFloorStockProtectEnd => "Promotion Floor Stock Protect - End",
            Delivered => "Delivered",
            Expiration => "Expiration",
            ShipNotBefore => "Ship Not Before",
            ShipNoLater => "Ship No Later",
            ShipWeekOf => "Ship Week of",
            Code040 => "Status (After and Including)",
            Code041 => "Status (Prior and Including)",
            Superseded => "Superseded",
            Publication => "Publication",
            SettlementDateAsSpecifiedByTheOriginator => {
                "Settlement Date as Specified by the Originator"
            }
            EndorsementDate => "Endorsement Date",
            FieldFailure => "Field Failure",
            FunctionalTest => "Functional Test",
            SystemTest => "System Test",
            PrototypeTest => "Prototype Test",
            Received => "Received",
            CumulativeQuantityStart => "Cumulative Quantity Start",
            CumulativeQuantityEnd => "Cumulative Quantity End",
            BuyersLocal => "Buyers Local",
            SellersLocal => "Sellers Local",
            Confirmed => "Confirmed",
            EstimatedPortOfEntry => "Estimated Port of Entry",
            ActualPortOfEntry => "Actual Port of Entry",
            CustomsClearance => "Customs Clearance",
            InlandShip => "Inland Ship",
            EngineeringChangeLevel => "Engineering Change Level",
            CancelIfNotDeliveredBy => "Cancel if Not Delivered by",
            Blueprint => "Blueprint",
            DoNotDeliverAfter => "Do Not Deliver After",
            DoNotDeliverBefore => "Do Not Deliver Before",
            Code065 => "1st Schedule Delivery",
            Code066 => "1st Schedule Ship",
            CurrentScheduleDelivery => "Current Schedule Delivery",
            CurrentScheduleShip => "Current Schedule Ship",
            PromisedForDelivery => "Promised for Delivery",
            Code070 => "Scheduled for Delivery (After and Including)",
            Code071 => "Requested for Delivery (After and Including)",
            Code072 => "Promised for Delivery (After and Including)",
            Code073 => "Scheduled for Delivery (Prior to and Including)",
            Code074 => "Requested for Delivery (Prior to and Including)",
            Code075 => "Promised for Delivery (Prior to and Including)",
            Code076 => "Scheduled for Delivery (Week of)",
            Code077 => "Requested for Delivery (Week of)",
            Code078 => "Promised for Delivery (Week of)",
            PromisedForShipment => "Promised for Shipment",
            Code080 => "Scheduled for Shipment (After and Including)",
            Code081 => "Requested for Shipment (After and Including)",
            Code082 => "Promised for Shipment (After and Including)",
            Code083 => "Scheduled for Shipment (Prior to and Including)",
            Code084 => "Requested for Shipment (Prior to and Including)",
            Code085 => "Promised for Shipment (Prior to and Including)",
            Code086 => "Scheduled for Shipment (Week of)",
            Code087 => "Requested for Shipment (Week of)",
            Code088 => "Promised for Shipment (Week of)",
            Inquiry => "Inquiry",
            ReportStart => "Report Start",
            ReportEnd => "Report End",
            ContractEffective => "Contract Effective",
            ContractExpiration => "Contract Expiration",
            Manufacture => "Manufacture",
            BillOfLading => "Bill of Lading",
            Discharge => "Discharge",
            TransactionCreation => "Transaction Creation",
            Code098 => "Bid (Effective)",
            Code099 => "Bid Open (Date Bids Will Be Opened)",
            NoShippingScheduleEstablishedAsOf => "No Shipping Schedule Established as of",
            NoProductionScheduleEstablishedAsOf => {
                "No Production Schedule Established as of"
            }
            Issue => "Issue",
            Award => "Award",
            SystemSurvey => "System Survey",
            QualityRating => "Quality Rating",
            RequiredBy => "Required By",
            Deposit => "Deposit",
            Postmark => "Postmark",
            ReceivedAtLockbox => "Received at Lockbox",
            OriginallyScheduledShip => "Originally Scheduled Ship",
            ManifestShipNotice => "Manifest/Ship Notice",
            BuyersDock => "Buyers Dock",
            SampleRequired => "Sample Required",
            ToolingRequired => "Tooling Required",
            SampleAvailable => "Sample Available",
            ScheduledInterchangeDelivery => "Scheduled Interchange Delivery",
            RequestedPickup => "Requested Pickup",
            TestPerformed => "Test Performed",
            ControlPlan => "Control Plan",
            FeasibilitySignOff => "Feasibility Sign Off",
            FailureModeEffective => "Failure Mode Effective",
            GroupContractEffective => "Group Contract Effective",
            GroupContractExpiration => "Group Contract Expiration",
            WholesaleContractEffective => "Wholesale Contract Effective",
            WholesaleContractExpiration => "Wholesale Contract Expiration",
            ReplacementEffective => "Replacement Effective",
            CustomerContractEffective => "Customer Contract Effective",
            CustomerContractExpiration => "Customer Contract Expiration",
            ItemContractEffective => "Item Contract Effective",
            ItemContractExpiration => "Item Contract Expiration",
            AccountsReceivableStatementDate => "Accounts Receivable - Statement Date",
            ReadyForInspection => "Ready for Inspection",
            Booking => "Booking",
            TechnicalRating => "Technical Rating",
            DeliveryRating => "Delivery Rating",
            CommercialRating => "Commercial Rating",
            Estimated => "Estimated",
            Actual => "Actual",
            Assigned => "Assigned",
            Loss => "Loss",
            DueDateOfFirstPaymentToPrincipalAndInterest => {
                "Due Date of First Payment to Principal and Interest"
            }
            EstimatedAcceptance => "Estimated Acceptance",
            OpeningDate => "Opening Date",
            ClosingDate => "Closing Date",
            DueDateLastCompleteInstallmentPaid => {
                "Due Date Last Complete Installment Paid"
            }
            DateOfLocalOfficeApprovalOfConveyanceOfDamagedRealEstateProperty => {
                "Date of Local Office Approval of Conveyance of Damaged Real Estate Property"
            }
            DateDeedFiledForRecord => "Date Deed Filed for Record",
            ServicePeriodStart => "Service Period Start",
            ServicePeriodEnd => "Service Period End",
            EffectiveDateOfChange => "Effective Date of Change",
            ServiceInterruption => "Service Interruption",
            AdjustmentPeriodStart => "Adjustment Period Start",
            AdjustmentPeriodEnd => "Adjustment Period End",
            AllotmentPeriodStart => "Allotment Period Start",
            TestPeriodStart => "Test Period Start",
            TestPeriodEnding => "Test Period Ending",
            BidPriceException => "Bid Price Exception",
            SamplesToBeReturnedBy => "Samples to be Returned By",
            LoadedOnVessel => "Loaded on Vessel",
            PendingArchive => "Pending Archive",
            ActualArchive => "Actual Archive",
            FirstIssue => "First Issue",
            FinalIssue => "Final Issue",
            Message => "Message",
            Code167 => "Most Recent Revision (or Initial Version)",
            Release => "Release",
            ProductAvailabilityDate => "Product Availability Date",
            SupplementalIssue => "Supplemental Issue",
            Revision => "Revision",
            Correction => "Correction",
            WeekEnding => "Week Ending",
            MonthEnding => "Month Ending",
            CancelIfNotShippedBy => "Cancel if not shipped by",
            ExpeditedOn => "Expedited on",
            Cancellation => "Cancellation",
            Code178 => "Hold (as of)",
            Code179 => "Hold as Stock (as of)",
            Code180 => "No Promise (as of)",
            Code181 => "Stop Work (as of)",
            Code182 => "Will Advise (as of)",
            Connection => "Connection",
            Inventory => "Inventory",
            VesselRegistry => "Vessel Registry",
            InvoicePeriodStart => "Invoice Period Start",
            InvoicePeriodEnd => "Invoice Period End",
            CreditAdvice => "Credit Advice",
            DebitAdvice => "Debit Advice",
            ReleasedToVessel => "Released to Vessel",
            MaterialSpecification => "Material Specification",
            DeliveryTicket => "Delivery Ticket",
            PeriodStart => "Period Start",
            PeriodEnd => "Period End",
            ContractReOpen => "Contract Re-Open",
            Start => "Start",
            End => "End",
            Completion => "Completion",
            Seal => "Seal",
            AssemblyStart => "Assembly Start",
            Acceptance => "Acceptance",
            MasterLeaseAgreement => "Master Lease Agreement",
            FirstProduced => "First Produced",
            Code204 => "Official Rail Car Interchange (Either Actual or Agreed Upon)",
            Transmitted => "Transmitted",
            Code206 => "Status (Outside Processor)",
            Code207 => "Status (Commercial)",
            LotNumberExpiration => "Lot Number Expiration",
            ContractPerformanceStart => "Contract Performance Start",
            ContractPerformanceDelivery => "Contract Performance Delivery",
            ServiceRequested => "Service Requested",
            ReturnedToCustomer => "Returned to Customer",
            AdjustmentToBillDated => "Adjustment to Bill Dated",
            DateOfRepairService => "Date of Repair/Service",
            InterruptionStart => "Interruption Start",
            InterruptionEnd => "Interruption End",
            Spud => "Spud",
            InitialCompletion => "Initial Completion",
            PluggedAndAbandoned => "Plugged and Abandoned",
            Penalty => "Penalty",
            PenaltyBegin => "Penalty Begin",
            Birth => "Birth",
            BirthCertificate => "Birth Certificate",
            Adoption => "Adoption",
            Christening => "Christening",
            LeaseCommencement => "Lease Commencement",
            LeaseTermStart => "Lease Term Start",
            LeaseTermEnd => "Lease Term End",
            RentStart => "Rent Start",
            Installation => "Installation",
            ProgressPayment => "Progress Payment",
            ClaimStatementPeriodStart => "Claim Statement Period Start",
            ClaimStatementPeriodEnd => "Claim Statement Period End",
            SettlementDate => "Settlement Date",
            Code235 => "Delayed Billing (Not Delayed Payment)",
            LenderCreditCheck => "Lender Credit Check",
            StudentSigned => "Student Signed",
            ScheduleRelease => "Schedule Release",
            Baseline => "Baseline",
            BaselineStart => "Baseline Start",
            BaselineComplete => "Baseline Complete",
            ActualStart => "Actual Start",
            ActualComplete => "Actual Complete",
            EstimatedStart => "Estimated Start",
            EstimatedCompletion => "Estimated Completion",
            StartNoEarlierThan => "Start no earlier than",
            StartNoLaterThan => "Start no later than",
            FinishNoLaterThan => "Finish no later than",
            FinishNoEarlierThan => "Finish no earlier than",
            Code250 => "Mandatory (or Target) Start",
            Code251 => "Mandatory (or Target) Finish",
            EarlyStart => "Early Start",
            EarlyFinish => "Early Finish",
            LateStart => "Late Start",
            LateFinish => "Late Finish",
            ScheduledStart => "Scheduled Start",
            ScheduledFinish => "Scheduled Finish",
            OriginalEarlyStart => "Original Early Start",
            OriginalEarlyFinish => "Original Early Finish",
            RestDay => "Rest Day",
            RestStart => "Rest Start",
            RestFinish => "Rest Finish",
            Holiday => "Holiday",
            HolidayStart => "Holiday Start",
            HolidayFinish => "Holiday Finish",
            Base => "Base",
            Timenow => "Timenow",
            EndDateOfSupport => "End Date of Support",
            DateAccountMatures => "Date Account Matures",
            DateFiled => "Date Filed",
            PenaltyEnd => "Penalty End",
            ExitPlantDate => "Exit Plant Date",
            LatestOnBoardCarrierDate => "Latest On Board Carrier Date",
            RequestedDepartureDate => "Requested Departure Date",
            Approved => "Approved",
            ContractStart => "Contract Start",
            ContractDefinition => "Contract Definition",
            LastItemDelivery => "Last Item Delivery",
            ContractCompletion => "Contract Completion",
            DateCourseOfOrthodonticsTreatmentBeganOrIsExpectedToBegin => {
                "Date Course of Orthodontics Treatment Began or is Expected to Begin"
            }
            OverTargetBaselineMonth => "Over Target Baseline Month",
            PreviousReport => "Previous Report",
            FundsAppropriationStart => "Funds Appropriation - Start",
            FundsAppropriationEnd => "Funds Appropriation - End",
            EmploymentOrHire => "Employment or Hire",
            Retirement => "Retirement",
            Medicare => "Medicare",
            Code288 => "Consolidated Omnibus Budget Reconciliation Act (COBRA)",
            PremiumPaidToDate => "Premium Paid to Date",
            CoordinationOfBenefits => "Coordination of Benefits",
            Plan => "Plan",
            Benefit => "Benefit",
            Education => "Education",
            EarningsEffectiveDate => "Earnings Effective Date",
            PrimaryCareProvider => "Primary Care Provider",
            InitialDisabilityPeriodReturnToWork => {
                "Initial Disability Period Return To Work"
            }
            InitialDisabilityPeriodLastDayWorked => {
                "Initial Disability Period Last Day Worked"
            }
            LatestAbsence => "Latest Absence",
            Illness => "Illness",
            EnrollmentSignatureDate => "Enrollment Signature Date",
            Code301 => {
                "Consolidated Omnibus Budget Reconciliation Act (COBRA) Qualifying Event"
            }
            Maintenance => "Maintenance",
            MaintenanceEffective => "Maintenance Effective",
            LatestVisitOrConsultation => "Latest Visit or Consultation",
            NetCreditServiceDate => "Net Credit Service Date",
            AdjustmentEffectiveDate => "Adjustment Effective Date",
            Eligibility => "Eligibility",
            PreAwardSurvey => "Pre-Award Survey",
            PlanTermination => "Plan Termination",
            DateOfClosing => "Date of Closing",
            LatestReceivingDateCutoffDate => "Latest Receiving Date/Cutoff Date",
            SalaryDeferral => "Salary Deferral",
            Cycle => "Cycle",
            Disability => "Disability",
            Offset => "Offset",
            PriorIncorrectDateOfBirth => "Prior Incorrect Date of Birth",
            CorrectedDateOfBirth => "Corrected Date of Birth",
            Added => "Added",
            Failed => "Failed",
            DateForeclosureProceedingsInstituted => {
                "Date Foreclosure Proceedings Instituted"
            }
            Purchased => "Purchased",
            PutIntoService => "Put into Service",
            Replaced => "Replaced",
            Returned => "Returned",
            DisbursementDate => "Disbursement Date",
            GuaranteeDate => "Guarantee Date",
            QuarterEnding => "Quarter Ending",
            Changed => "Changed",
            Terminated => "Terminated",
            ReferralDate => "Referral Date",
            EvaluationDate => "Evaluation Date",
            PlacementDate => "Placement Date",
            Code333 => "Individual Education Plan (IEP)",
            ReEvaluationDate => "Re-evaluation Date",
            DismissalDate => "Dismissal Date",
            EmploymentBegin => "Employment Begin",
            EmploymentEnd => "Employment End",
            MedicareBegin => "Medicare Begin",
            MedicareEnd => "Medicare End",
            Code340 => "Consolidated Omnibus Budget Reconciliation Act (COBRA) Begin",
            Code341 => "Consolidated Omnibus Budget Reconciliation Act (COBRA) End",
            PremiumPaidToDateBegin => "Premium Paid to Date Begin",
            PremiumPaidToDateEnd => "Premium Paid to Date End",
            CoordinationOfBenefitsBegin => "Coordination of Benefits Begin",
            CoordinationOfBenefitsEnd => "Coordination of Benefits End",
            PlanBegin => "Plan Begin",
            PlanEnd => "Plan End",
            BenefitBegin => "Benefit Begin",
            BenefitEnd => "Benefit End",
            EducationBegin => "Education Begin",
            EducationEnd => "Education End",
            PrimaryCareProviderBegin => "Primary Care Provider Begin",
            PrimaryCareProviderEnd => "Primary Care Provider End",
            IllnessBegin => "Illness Begin",
            IllnessEnd => "Illness End",
            EligibilityBegin => "Eligibility Begin",
            EligibilityEnd => "Eligibility End",
            CycleBegin => "Cycle Begin",
            CycleEnd => "Cycle End",
            InitialDisabilityPeriodStart => "Initial Disability Period Start",
            InitialDisabilityPeriodEnd => "Initial Disability Period End",
            OffsetBegin => "Offset Begin",
            OffsetEnd => "Offset End",
            PlanPeriodElectionBegin => "Plan Period Election Begin",
            PlanPeriodElectionEnd => "Plan Period Election End",
            PlanPeriodElection => "Plan Period Election",
            DueToCustomer => "Due to Customer",
            Submittal => "Submittal",
            EstimatedDepartureDate => "Estimated Departure Date",
            ActualDepartureDate => "Actual Departure Date",
            EstimatedArrivalDate => "Estimated Arrival Date",
            ActualArrivalDate => "Actual Arrival Date",
            OrderStart => "Order Start",
            OrderEnd => "Order End",
            DeliveryStart => "Delivery Start",
            DeliveryEnd => "Delivery End",
            ContractCostsThrough => "Contract Costs Through",
            FinancialInformationSubmission => "Financial Information Submission",
            BusinessTermination => "Business Termination",
            ApplicantSigned => "Applicant Signed",
            CosignerSigned => "Cosigner Signed",
            Enrollment => "Enrollment",
            AdjustedHire => "Adjusted Hire",
            CreditedService => "Credited Service",
            CreditedServiceBegin => "Credited Service Begin",
            CreditedServiceEnd => "Credited Service End",
            DeferredDistribution => "Deferred Distribution",
            PaymentCommencement => "Payment Commencement",
            PayrollPeriod => "Payroll Period",
            PayrollPeriodBegin => "Payroll Period Begin",
            PayrollPeriodEnd => "Payroll Period End",
            PlanEntry => "Plan Entry",
            PlanParticipationSuspension => "Plan Participation Suspension",
            Rehire => "Rehire",
            Retermination => "Retermination",
            Termination => "Termination",
            Valuation => "Valuation",
            VestingService => "Vesting Service",
            VestingServiceBegin => "Vesting Service Begin",
            VestingServiceEnd => "Vesting Service End",
            DuplicateBill => "Duplicate Bill",
            AdjustmentPromised => "Adjustment Promised",
            AdjustmentProcessed => "Adjustment Processed",
            YearEnding => "Year Ending",
            Production => "Production",
            MaterialClassification => "Material Classification",
            Weighed => "Weighed",
            DateOfDeedInLieu => "Date of Deed in Lieu",
            DateOfFirmCommitment => "Date of Firm Commitment",
            ExpirationDateOfExtensionToForeclose => {
                "Expiration Date of Extension to Foreclose"
            }
            DateOfNoticeToConvey => "Date of Notice to Convey",
            DateOfReleaseOfBankruptcy => "Date of Release of Bankruptcy",
            OptimisticEarlyStart => "Optimistic Early Start",
            OptimisticEarlyFinish => "Optimistic Early Finish",
            OptimisticLateStart => "Optimistic Late Start",
            OptimisticLateFinish => "Optimistic Late Finish",
            MostLikelyEarlyStart => "Most Likely Early Start",
            MostLikelyEarlyFinish => "Most Likely Early Finish",
            MostLikelyLateStart => "Most Likely Late Start",
            MostLikelyLateFinish => "Most Likely Late Finish",
            PessimisticEarlyStart => "Pessimistic Early Start",
            PessimisticEarlyFinish => "Pessimistic Early Finish",
            PessimisticLateStart => "Pessimistic Late Start",
            PessimisticLateFinish => "Pessimistic Late Finish",
            FirstPaymentDue => "First Payment Due",
            FirstInterestPaymentDue => "First Interest Payment Due",
            SubsequentInterestPaymentDue => "Subsequent Interest Payment Due",
            IrregularInterestPaymentDue => "Irregular Interest Payment Due",
            GuarantorReceived => "Guarantor Received",
            OnsetOfCurrentSymptomsOrIllness => "Onset of Current Symptoms or Illness",
            Submission => "Submission",
            Removed => "Removed",
            Statement => "Statement",
            Admission => "Admission",
            InsuranceCard => "Insurance Card",
            SpouseRetirement => "Spouse Retirement",
            OnsetOfSimilarSymptomsOrIllness => "Onset of Similar Symptoms or Illness",
            Accident => "Accident",
            ReleaseOf => "Release of Information",
            PriorPlacement => "Prior Placement",
            DateOfDeath => "Date of Death",
            Code443 => "Peer Review Organization (PRO) Approved Stay",
            FirstVisitOrConsultation => "First Visit or Consultation",
            InitialPlacement => "Initial Placement",
            Replacement => "Replacement",
            Occurrence => "Occurrence",
            OccurrenceSpan => "Occurrence Span",
            OccurrenceSpanFrom => "Occurrence Span From",
            OccurrenceSpanTo => "Occurrence Span To",
            InitialFeeDue => "Initial Fee Due",
            AppliancePlacement => "Appliance Placement",
            AcuteManifestationOfAChronicCondition => {
                "Acute Manifestation of a Chronic Condition"
            }
            InitialTreatment => "Initial Treatment",
            LastXRay => "Last X-Ray",
            Surgery => "Surgery",
            Code457 => "Continuous Passive Motion (CPM)",
            Certification => "Certification",
            NursingHomeFrom => "Nursing Home From",
            NursingHomeTo => "Nursing Home To",
            LastCertification => "Last Certification",
            DateOfLocalOfficeApprovalOfConveyanceOfOccupiedRealEstateProperty => {
                "Date of Local Office Approval of Conveyance of Occupied Real Estate Property"
            }
            BeginTherapy => "Begin Therapy",
            OxygenTherapyFrom => "Oxygen Therapy From",
            OxygenTherapyTo => "Oxygen Therapy To",
            OxygenTherapy => "Oxygen Therapy",
            Signature => "Signature",
            PrescriptionFill => "Prescription Fill",
            ProviderSignature => "Provider Signature",
            DateOfLocalOfficeCertificationOfConveyanceOfDamagedRealEstateProperty => {
                "Date of Local Office Certification of Conveyance of Damaged Real Estate Property"
            }
            Prescription => "Prescription",
            Service => "Service",
            MedicaidBegin => "Medicaid Begin",
            MedicaidEnd => "Medicaid End",
            Medicaid => "Medicaid",
            Code476 => "Peer Review Organization (PRO) Approved Stay From",
            Code477 => "Peer Review Organization (PRO) Approved Stay To",
            PrescriptionFrom => "Prescription From",
            PrescriptionTo => "Prescription To",
            ArterialBloodGasTest => "Arterial Blood Gas Test",
            OxygenSaturationTest => "Oxygen Saturation Test",
            PregnancyBegin => "Pregnancy Begin",
            PregnancyEnd => "Pregnancy End",
            LastMenstrualPeriod => "Last Menstrual Period",
            InjuryBegin => "Injury Begin",
            InjuryEnd => "Injury End",
            NursingHome => "Nursing Home",
            CollateralDependent => "Collateral Dependent",
            CollateralDependentBegin => "Collateral Dependent Begin",
            CollateralDependentEnd => "Collateral Dependent End",
            SponsoredDependent => "Sponsored Dependent",
            SponsoredDependentBegin => "Sponsored Dependent Begin",
            SponsoredDependentEnd => "Sponsored Dependent End",
            Deductible => "Deductible",
            OutOfPocket => "Out-of-Pocket",
            ContractAuditDate => "Contract Audit Date",
            LatestDeliveryDateAtPier => "Latest Delivery Date at Pier",
            MortgageeReportedCurtailmentDate => "Mortgagee Reported Curtailment Date",
            MortgageeOfficialSignatureDate => "Mortgagee Official Signature Date",
            Resubmission => "Resubmission",
            ExpectedReply => "Expected Reply",
            DroppedToLessThanHalfTime => "Dropped to Less than Half Time",
            RepaymentBegin => "Repayment Begin",
            LoanServicingTransfer => "Loan Servicing Transfer",
            LoanPurchase => "Loan Purchase",
            LastNotification => "Last Notification",
            Extract => "Extract",
            Extended => "Extended",
            ServicerSignatureDate => "Servicer Signature Date",
            DatePacked => "Date Packed",
            ShelfLifeExpiration => "Shelf Life Expiration",
            WarrantyExpiration => "Warranty Expiration",
            Overhauled => "Overhauled",
            Transferred => "Transferred",
            Notified => "Notified",
            Discovered => "Discovered",
            Inspected => "Inspected",
            Code518 => "Voucher (Date of)",
            DateBankruptcyFiled => "Date Bankruptcy Filed",
            DateOfDamage => "Date of Damage",
            DateHazardInsurancePolicyCancelled => {
                "Date Hazard Insurance Policy Cancelled"
            }
            ExpirationDateToSubmitTitleEvidence => {
                "Expiration Date to Submit Title Evidence"
            }
            DateOfClaim => "Date of Claim",
            DateOfNoticeOfReferralForAssignment => {
                "Date of Notice of Referral for Assignment"
            }
            DateOfNoticeOfProbableIneligibilityForAssignment => {
                "Date of Notice of Probable Ineligibility for Assignment"
            }
            DateOfForeclosureNotice => "Date of Foreclosure Notice",
            ExpirationOfForeclosureTimeframe => "Expiration of Foreclosure Timeframe",
            DatePossessoryActionInitiated => "Date Possessory Action Initiated",
            DateOfPossession => "Date of Possession",
            DateOfLastInstallmentReceived => "Date of Last Installment Received",
            DateOfAcquisitionOfTitle => "Date of Acquisition of Title",
            ExpirationOfExtensionToConvey => "Expiration of Extension to Convey",
            DateOfAssignmentApproval => "Date of Assignment Approval",
            DateOfAssignmentRejection => "Date of Assignment Rejection",
            CurtailmentDateFromAdviceOfPayment => {
                "Curtailment Date from Advice of Payment"
            }
            ExpirationOfExtensionToSubmitFiscalData => {
                "Expiration of Extension to Submit Fiscal Data"
            }
            Code537 => "Date Documentation, or Paperwork, or Both Was Sent",
            MakegoodCommercialDate => "Makegood Commercial Date",
            PolicyEffective => "Policy Effective",
            PolicyExpiration => "Policy Expiration",
            EmployeeEffectiveDateOfCoverage => "Employee Effective Date of Coverage",
            ClaimAdministratorNotifiedOfEmployeeLegalRepresentation => {
                "Claim Administrator Notified of Employee Legal Representation"
            }
            LastPremiumPaidDate => "Last Premium Paid Date",
            EmployerKnowledgeOfTheInjury => "Employer Knowledge of the Injury",
            ClaimAdministratorKnowledgeOfTheInjury => {
                "Claim Administrator Knowledge of the Injury"
            }
            DateOfMaximumMedicalImprovement => "Date of Maximum Medical Improvement",
            DateOfLoan => "Date of Loan",
            DateOfAdvance => "Date of Advance",
            BeginningLayDate => "Beginning Lay Date",
            CertificateEffective => "Certificate Effective",
            BenefitApplicationDate => "Benefit Application Date",
            ActualReturnToWork => "Actual Return to Work",
            ReleasedReturnToWork => "Released Return to Work",
            EndingLayDate => "Ending Lay Date",
            EmployeeWagesCeased => "Employee Wages Ceased",
            LastSalaryIncrease => "Last Salary Increase",
            EmployeeLaidOff => "Employee Laid Off",
            InjuryOrIllness => "Injury or Illness",
            OldestUnpaidInstallment => "Oldest Unpaid Installment",
            PreforeclosureAcceptanceDate => "Preforeclosure Acceptance Date",
            PreforeclosureSaleClosingDate => "Preforeclosure Sale Closing Date",
            DateOfFirstUncuredDefault => "Date of First Uncured Default",
            DateDefaultWasCured => "Date Default Was Cured",
            DateOfFirstMortgagePayment => "Date of First Mortgage Payment",
            DateOfPropertyInspection => "Date of Property Inspection",
            DateTotalAmountOfDelinquencyReported => {
                "Date Total Amount of Delinquency Reported"
            }
            DateOutstandingLoanBalanceReported => {
                "Date Outstanding Loan Balance Reported"
            }
            DateForeclosureSaleScheduled => "Date Foreclosure Sale Scheduled",
            DateForeclosureHeld => "Date Foreclosure Held",
            DateRedemptionPeriodEnds => "Date Redemption Period Ends",
            DateVoluntaryConveyanceAccepted => "Date Voluntary Conveyance Accepted",
            DatePropertySold => "Date Property Sold",
            DateClaimPaid => "Date Claim Paid",
            ActionBeginDate => "Action Begin Date",
            ProjectedActionEndDate => "Projected Action End Date",
            ActionEndDate => "Action End Date",
            OriginalMaturityDate => "Original Maturity Date",
            DateReferredToAttorneyForForeclosure => {
                "Date Referred to Attorney for Foreclosure"
            }
            PlannedRelease => "Planned Release",
            ActualRelease => "Actual Release",
            ContractPeriod => "Contract Period",
            ReportPeriod => "Report Period",
            Suspension => "Suspension",
            Reinstatement => "Reinstatement",
            Report => "Report",
            FirstContact => "First Contact",
            ProjectedForeclosureSaleDate => "Projected Foreclosure Sale Date",
            DateAssignmentFiledForRecord => "Date Assignment Filed for Record",
            DateOfAppraisal => "Date of Appraisal",
            ExpirationDateOfExtensionToAssign => "Expiration Date of Extension to Assign",
            DateOfExtensionToConvey => "Date of Extension to Convey",
            DateHazardInsurancePolicyRefused => "Date Hazard Insurance Policy Refused",
            HighFabricationReleaseAuthorization => {
                "High Fabrication Release Authorization"
            }
            HighRawMaterialAuthorization => "High Raw Material Authorization",
            MaterialChangeNotice => "Material Change Notice",
            LatestDeliveryDateAtRailRamp => "Latest Delivery Date at Rail Ramp",
            Rejected => "Rejected",
            RepaymentScheduleSent => "Repayment Schedule Sent",
            AsOf => "As Of",
            FirstSubmission => "First Submission",
            SubsequentSubmission => "Subsequent Submission",
            Renewal => "Renewal",
            Withdrawn => "Withdrawn",
            CertificationPeriodStart => "Certification Period Start",
            CertificationRevision => "Certification Revision",
            Code608 => "Continuous Coverage Date(s)",
            PrearrangedDealMatch => "Prearranged Deal Match",
            ContingencyEnd => "Contingency End",
            OxygenTherapyEvaluation => "Oxygen Therapy Evaluation",
            ShutIn => "Shut In",
            AllowableEffective => "Allowable Effective",
            FirstSales => "First Sales",
            DateAcquired => "Date Acquired",
            InterviewerSigned => "Interviewer Signed",
            ApplicationLoggedDate => "Application Logged Date",
            ReviewDate => "Review Date",
            DecisionDate => "Decision Date",
            PreviouslyResided => "Previously Resided",
            Reported => "Reported",
            Checked => "Checked",
            Settled => "Settled",
            PresentlyResiding => "Presently Residing",
            EmployedInThisPosition => "Employed in this Position",
            Verified => "Verified",
            SecondAdmissionDate => "Second Admission Date",
            AccountOpened => "Account Opened",
            AccountClosed => "Account Closed",
            PropertyAcquired => "Property Acquired",
            PropertyBuilt => "Property Built",
            EmployedInThisProfession => "Employed in this Profession",
            NextReviewDate => "Next Review Date",
            InitialContactDate => "Initial Contact Date",
            DateOfLastUpdate => "Date of Last Update",
            SecondDischargeDate => "Second Discharge Date",
            DateOfLastDraw => "Date of Last Draw",
            Complaint => "Complaint",
            Option => "Option",
            Solicitation => "Solicitation",
            Clause => "Clause",
            Meeting => "Meeting",
            RentalPeriod => "Rental Period",
            NextPayIncrease => "Next Pay Increase",
            PeriodCoveredBySourceDocuments => "Period Covered by Source Documents",
            DocumentDue => "Document Due",
            CourtNotice => "Court Notice",
            ExpectedFundingDate => "Expected Funding Date",
            AssignmentRecorded => "Assignment Recorded",
            CaseReopened => "Case Reopened",
            PreviousCourtEvent => "Previous Court Event",
            LastDateToObject => "Last Date to Object",
            CourtEvent => "Court Event",
            LastDateToFileAClaim => "Last Date to File a Claim",
            CaseConverted => "Case Converted",
            DebtIncurred => "Debt Incurred",
            Judgment => "Judgment",
            WagesStart => "Wages Start",
            WagesEnd => "Wages End",
            DateThroughWhichPropertyTaxesHaveBeenPaid => {
                "Date Through Which Property Taxes Have Been Paid"
            }
            PaidThroughDate => "Paid Through Date",
            DatePaid => "Date Paid",
            AnesthesiaAdministration => "Anesthesia Administration",
            PriceProtection => "Price Protection",
            ClaimIncurred => "Claim Incurred",
            BookEntryDelivery => "Book Entry Delivery",
            RateAdjustment => "Rate Adjustment",
            NextInstallmentDueDate => "Next Installment Due Date",
            DaylightOverdraftTime => "Daylight Overdraft Time",
            PresentmentDate => "Presentment Date",
            NegotiatedExtensionDate => "Negotiated Extension Date",
            Remittance => "Remittance",
            SecurityRateAdjustment => "Security Rate Adjustment",
            FilingPeriod => "Filing Period",
            ReviewPeriodEnd => "Review Period End",
            RequestedSettlement => "Requested Settlement",
            LastScreening => "Last Screening",
            Confinement => "Confinement",
            Arrested => "Arrested",
            Convicted => "Convicted",
            Interviewed => "Interviewed",
            LastVisit => "Last Visit",
            Recovery => "Recovery",
            TimeInUS => "Time in U.S.",
            FuturePeriod => "Future Period",
            PreviousPeriod => "Previous Period",
            InterestPaidTo => "Interest Paid To",
            DateOfSeizure => "Date of Seizure",
            Setoff => "Setoff",
            OverrideDateForSettlement => "Override Date for Settlement",
            Code701 => "Settlement Date (From Interline Settlement System (ISS) only)",
            SendingRoadTimeStamp => "Sending Road Time Stamp",
            RetransmissionTimeStamp => "Retransmission Time Stamp",
            DeliveryAppointmentDateAndTime => "Delivery Appointment Date and Time",
            InterestPaidThrough => "Interest Paid Through",
            DateMaterialUsageSuspended => "Date Material Usage Suspended",
            LastPaymentMade => "Last Payment Made",
            PastDue => "Past Due",
            AnalysisMonthEnding => "Analysis Month Ending",
            DateOfSpecification => "Date of Specification",
            DateOfStandard => "Date of Standard",
            ReturnToWorkPartTime => "Return to Work Part Time",
            PaidThroughDateForSalaryContinuation => {
                "Paid-through Date for Salary Continuation"
            }
            PaidThroughDateForVacationPay => "Paid-through Date for Vacation Pay",
            PaidThroughDateForAccruedSickPay => "Paid-through Date for Accrued Sick Pay",
            AppraisalOrdered => "Appraisal Ordered",
            DateOfOperation => "Date of Operation",
            BestTimeToCall => "Best Time to Call",
            VerbalReportNeeded => "Verbal Report Needed",
            EstimatedEscrowClosing => "Estimated Escrow Closing",
            PermitYear => "Permit Year",
            RemodelingCompleted => "Remodeling Completed",
            CurrentMonthEnding => "Current Month Ending",
            PreviousMonthEnding => "Previous Month Ending",
            CycleToDate => "Cycle to Date",
            YearToDate => "Year to Date",
            OnHold => "On Hold",
            OffHold => "Off Hold",
            FacsimileDueBy => "Facsimile Due By",
            ReportingCycleDate => "Reporting Cycle Date",
            LastPaidInstallmentDate => "Last Paid Installment Date",
            ClaimsMade => "Claims Made",
            DateOfLastPaymentReceived => "Date of Last Payment Received",
            CurtailmentDate => "Curtailment Date",
            PoolSettlement => "Pool Settlement",
            NextInterestChangeDate => "Next Interest Change Date",
            MostRecentHemoglobinOrHematocritOrBoth => {
                "Most Recent Hemoglobin or Hematocrit or Both"
            }
            MostRecentSerumCreatine => "Most Recent Serum Creatine",
            Closed => "Closed",
            Therapy => "Therapy",
            Implantation => "Implantation",
            Explantation => "Explantation",
            DateBecameAware => "Date Became Aware",
            FirstMarketed => "First Marketed",
            LastMarketed => "Last Marketed",
            NewDueDateOfFirstPaymentToPrincipalAndInterest => {
                "New Due Date of First Payment to Principal and Interest"
            }
            NewMaturityDate => "New Maturity Date",
            Current => "Current",
            ExpectedProblemResolution => "Expected Problem Resolution",
            AlternateProblemResolution => "Alternate Problem Resolution",
            FeeCapitalization => "Fee Capitalization",
            InterestCapitalization => "Interest Capitalization",
            NextPaymentDue => "Next Payment Due",
            ConversionToRepayment => "Conversion to Repayment",
            EndOfGracePeriod => "End of Grace Period",
            SchoolRefund => "School Refund",
            SimpleInterestDue => "Simple Interest Due",
            DatePracticeCeased => "Date Practice Ceased",
            Printed => "Printed",
            DatePracticeEstablished => "Date Practice Established",
            DropActionDate => "Drop Action Date",
            MostRecentRenewal => "Most Recent Renewal",
            Original => "Original",
            OutsideAuditorsReport => "Outside Auditor's Report",
            PreCertificationDate => "Pre-certification Date",
            BackOnMarket => "Back on Market",
            Status => "Status",
            BenefitAdjustmentStart => "Benefit Adjustment Start",
            OffMarket => "Off-Market",
            Tour => "Tour",
            BenefitAdjustmentEnd => "Benefit Adjustment End",
            ListingReceived => "Listing Received",
            BenefitAdjustmentPeriod => "Benefit Adjustment Period",
            AnticipatedClosing => "Anticipated Closing",
            LastPublication => "Last Publication",
            SoldBookPublication => "Sold Book Publication",
            Occupancy => "Occupancy",
            Contingency => "Contingency",
            PercolationTest => "Percolation Test",
            SepticApproval => "Septic Approval",
            TitleTransfer => "Title Transfer",
            OpenHouse => "Open House",
            BenefitCreditPeriod => "Benefit Credit Period",
            BenefitTransferPeriod => "Benefit Transfer Period",
            Homestead => "Homestead",
            Sanction => "Sanction",
            TailCoverageBegin => "Tail Coverage Begin",
            TailCoverageEnd => "Tail Coverage End",
            TrainingBegin => "Training Begin",
            TrainingEnd => "Training End",
            VerificationReceived => "Verification Received",
            VerificationSent => "Verification Sent",
            StateResidencyDate => "State Residency Date",
            EffectiveDateOfTheRoutingFile => "Effective Date of the Routing File",
            TestDataAnalysis => "Test Data Analysis",
            MidpointOfPerformance => "Midpoint of Performance",
            AcquisitionDate => "Acquisition Date",
            DateOfAction => "Date of Action",
            PaidInFull => "Paid in Full",
            Refinance => "Refinance",
            VoluntaryTermination => "Voluntary Termination",
            CustomerOrder => "Customer Order",
            Stored => "Stored",
            Selected => "Selected",
            Posted => "Posted",
            DocumentReceived => "Document Received",
            Rebuilt => "Rebuilt",
            Marriage => "Marriage",
            CustomsEntryDate => "Customs Entry Date",
            PaymentDueDate => "Payment Due Date",
            MaturityDate => "Maturity Date",
            TradeDate => "Trade Date",
            Code817 => "Gallons Per Minute (GPM) Test Performed",
            Code818 => "British Thermal Unit (BTU) Test Performed",
            LastAccountsFiledAtPublicRegistrationAgency => {
                "Last Accounts Filed at Public Registration Agency"
            }
            RealEstateTaxYear => "Real Estate Tax Year",
            FinalReconciliationValueEstimateAsOf => {
                "Final Reconciliation Value Estimate as of"
            }
            Map => "Map",
            Opinion => "Opinion",
            Version => "Version",
            OriginalDueDate => "Original Due Date",
            IncumbencyPeriod => "Incumbency Period",
            AudienceDeficiencyPeriod => "Audience Deficiency Period",
            AiredDate => "Aired Date",
            Schedule => "Schedule",
            PaidThroughDateForMinimumPayment => "Paid Through Date for Minimum Payment",
            PaidThroughDateForTotalPayment => "Paid Through Date for Total Payment",
            Election => "Election",
            EngineeringDataList => "Engineering Data List",
            LastProduction => "Last Production",
            NotBefore => "Not Before",
            NotAfter => "Not After",
            InitialClaim => "Initial Claim",
            BenefitsPaid => "Benefits Paid",
            WagesEarned => "Wages Earned",
            AdjustedStart => "Adjusted Start",
            AdjustedEnd => "Adjusted End",
            RevisedAdjustedStart => "Revised Adjusted Start",
            RevisedAdjustedEnd => "Revised Adjusted End",
            FieldTest => "Field Test",
            MortgageNoteDate => "Mortgage Note Date",
            AlternativeDueDate => "Alternative Due Date",
            FirstPaymentChange => "First Payment Change",
            FirstRateAdjustment => "First Rate Adjustment",
            AlternateBasePeriod => "Alternate Base Period",
            PriorNotice => "Prior Notice",
            AppointmentEffective => "Appointment Effective",
            AppointmentExpiration => "Appointment Expiration",
            CompanyTermination => "Company Termination",
            ContinuingEducationRequirement => "Continuing Education Requirement",
            DistributorEffective => "Distributor Effective",
            DistributorTermination => "Distributor Termination",
            Examination => "Examination",
            IncorporationDissolution => "Incorporation Dissolution",
            LastFollowUp => "Last Follow-up",
            LicenseEffective => "License Effective",
            LicenseExpiration => "License Expiration",
            LicenseRenewal => "License Renewal",
            LicenseRequested => "License Requested",
            Mailed => "Mailed",
            PaperworkMailed => "Paperwork Mailed",
            PreviousEmployment => "Previous Employment",
            PreviousEmploymentEnd => "Previous Employment End",
            PreviousEmploymentStart => "Previous Employment Start",
            PreviousResidence => "Previous Residence",
            PreviousResidenceEnd => "Previous Residence End",
            PreviousResidenceStart => "Previous Residence Start",
            Request => "Request",
            ResidentLicenseEffective => "Resident License Effective",
            ResidentLicenseExpiration => "Resident License Expiration",
            StateTermination => "State Termination",
            TexasLineTermination => "Texas Line Termination",
            Acceleration => "Acceleration",
            AdjustedContestability => "Adjusted Contestability",
            ApplicationEntry => "Application Entry",
            ApprovalOffer => "Approval/Offer",
            AutomaticPremiumLoan => "Automatic Premium Loan",
            Collection => "Collection",
            ConfinementEnd => "Confinement End",
            ConfinementStart => "Confinement Start",
            Contestability => "Contestability",
            FlatExtraEnd => "Flat Extra End",
            LastActivity => "Last Activity",
            LastChange => "Last Change",
            LastEpisode => "Last Episode",
            LastMeal => "Last Meal",
            Loan => "Loan",
            ApplicationStatus => "Application Status",
            Maturity => "Maturity",
            MedicalInformationSignature => "Medical Information Signature",
            MedicalInformationSystem => "Medical Information System",
            Note => "Note",
            OfferExpiration => "Offer Expiration",
            OriginalReceipt => "Original Receipt",
            Placement => "Placement",
            PlacementPeriodExpiration => "Placement Period Expiration",
            Processing => "Processing",
            Recapture => "Recapture",
            ReEntry => "Re-entry",
            Reissue => "Reissue",
            Requalification => "Requalification",
            ReinsuranceEffective => "Reinsurance Effective",
            ReservationOfFacility => "Reservation of Facility",
            SettlementStatus => "Settlement Status",
            TableRatingEnd => "Table Rating End",
            TerminationOfFacility => "Termination of Facility",
            Treatment => "Treatment",
            DepartmentOfLaborWageDeterminationDate => {
                "Department of Labor Wage Determination Date"
            }
            Order => "Order",
            Resolved => "Resolved",
            ExecutionDate => "Execution Date",
            CapitationPeriodStart => "Capitation Period Start",
            CapitationPeriodEnd => "Capitation Period End",
            LastDateForAGovernmentAgencyToFileAClaim => {
                "Last Date for a Government Agency to File a Claim"
            }
            AdjustmentPeriod => "Adjustment Period",
            Activity => "Activity",
            MailBy => "Mail By",
            Preparation => "Preparation",
            PaymentInitiated => "Payment Initiated",
            PaymentEffective => "Payment Effective",
            Application => "Application",
            Reclassification => "Reclassification",
            Code952 => "Reclassification (Exit Date)",
            PostReclassification => "Post-Reclassification",
            Code954 => "Post-Reclassification (First Report Card)",
            Code955 => "Post-Reclassification (First Semi-annual)",
            Code956 => "Post-Reclassification (Second Semi-annual)",
            Code957 => "Post-Reclassification (End of Second Year)",
            AdjustedDeathBenefit => "Adjusted Death Benefit",
            Anniversary => "Anniversary",
            Annuitization => "Annuitization",
            AnnuityCommencementDate => "Annuity Commencement Date",
            Bill => "Bill",
            CalendarAnniversary => "Calendar Anniversary",
            ContractMailed => "Contract Mailed",
            EarlyWithdrawal => "Early Withdrawal",
            FiscalAnniversary => "Fiscal Anniversary",
            Income => "Income",
            InitialPremium => "Initial Premium",
            InitialPremiumEffective => "Initial Premium Effective",
            LastPremiumEffective => "Last Premium Effective",
            MinimumRequiredDistribution => "Minimum Required Distribution",
            NextAnniversary => "Next Anniversary",
            Notice => "Notice",
            NotificationOfDeath => "Notification of Death",
            PartialAnnuitization => "Partial Annuitization",
            PlanAnniversary => "Plan Anniversary",
            PolicySurrender => "Policy Surrender",
            PriorContractAnniversary => "Prior Contract Anniversary",
            PriorContractIssue => "Prior Contract Issue",
            SignatureReceived => "Signature Received",
            Tax => "Tax",
            BenefitPeriod => "Benefit Period",
            MonthToDate => "Month to Date",
            SemiannualEnding => "Semiannual Ending",
            Surrender => "Surrender",
            PlanOfTreatmentPeriod => "Plan of Treatment Period",
            Code989 => "Prior Hospitalization Date(s) Related to Current Service(s)",
            OriginalNameChange => "Original Name Change",
            DateRequested => "Date Requested",
            RequestForQuotation => "Request for Quotation",
            Quote => "Quote",
            RecordedDate => "Recorded Date",
            RequiredDelivery => "Required Delivery",
            QuoteToBeReceivedBy => "Quote to be Received By",
            ContinuationOfPayStartDate => "Continuation of Pay Start Date",
            DocumentDate => "Document Date",
            EstimatedPointOfArrival => "Estimated Point of Arrival",
            EstimatedPointOfDischarge => "Estimated Point of Discharge",
            CodeAA3 => "Cancel After, Ex Country",
            CodeAA4 => "Cancel After, Ex Factory",
            CodeAA5 => "Do Not Ship Before, Ex Country",
            CodeAA6 => "Do Not Ship Before, Ex Factory",
            FinalScheduledPayment => "Final Scheduled Payment",
            ActualDischarge => "Actual Discharge",
            AddressPeriod => "Address Period",
            ArrivalInCountry => "Arrival in Country",
            Citation => "Citation",
            SuspensionEffective => "Suspension Effective",
            Crime => "Crime",
            DischargePlanned => "Discharge - Planned",
            Draft => "Draft",
            DueDate => "Due Date",
            Event => "Event",
            FirstInvolvement => "First Involvement",
            GuaranteePeriod => "Guarantee Period",
            IncomeIncreasePeriod => "Income Increase Period",
            InstallmentDate => "Installment Date",
            LastCivilianFlight => "Last Civilian Flight",
            LastFlight => "Last Flight",
            LastInsuranceMedical => "Last Insurance Medical",
            LastMilitaryFlight => "Last Military Flight",
            LastPhysical => "Last Physical",
            License => "License",
            MedicalCertificate => "Medical Certificate",
            Medication => "Medication",
            NetWorthDate => "Net Worth Date",
            NextActivity => "Next Activity",
            OwnershipChange => "Ownership Change",
            OwnershipPeriod => "Ownership Period",
            RateDate => "Rate Date",
            RequestedContract => "Requested Contract",
            RequestedOffer => "Requested Offer",
            SalesPeriod => "Sales Period",
            TaxYear => "Tax Year",
            TimePeriod => "Time Period",
            Travel => "Travel",
            TreatmentEnd => "Treatment End",
            TreatmentStart => "Treatment Start",
            Trust => "Trust",
            WorstTimeToCall => "Worst Time to Call",
            Registration => "Registration",
            Revoked => "Revoked",
            EstimatedDateOfBirth => "Estimated Date of Birth",
            LastAnnualReport => "Last Annual Report",
            LegalActionStarted => "Legal Action Started",
            Lien => "Lien",
            PaymentPeriod => "Payment Period",
            ProfitPeriod => "Profit Period",
            Registered => "Registered",
            Consolidated => "Consolidated",
            BoardOfDirectorsNotAuthorizedAsOf => {
                "Board of Directors Not Authorized As Of"
            }
            BoardOfDirectorsIncompleteAsOf => "Board of Directors Incomplete As Of",
            ManagerNotRegisteredAsOf => "Manager Not Registered As Of",
            CitizenshipChange => "Citizenship Change",
            Participation => "Participation",
            Capitalization => "Capitalization",
            RegistrationOfBoardOfDirectors => "Registration of Board of Directors",
            CeasedOperations => "Ceased Operations",
            Satisfied => "Satisfied",
            TermsMet => "Terms Met",
            AssetDocumentationExpiration => "Asset Documentation Expiration",
            CreditDocumentationExpiration => "Credit Documentation Expiration",
            IncomeDocumentationExpiration => "Income Documentation Expiration",
            ProductHeldUntil => "Product Held Until",
            ImmigrationDate => "Immigration Date",
            EstimatedImmigrationDate => "Estimated Immigration Date",
            CurrentDisabilityPeriodStart => "Current Disability Period Start",
            CurrentDisabilityPeriodEnd => "Current Disability Period End",
            CurrentDisabilityPeriodLastDayWorked => {
                "Current Disability Period Last Day Worked"
            }
            BenefitTypeGrossWeeklyAmountEffective => {
                "Benefit Type Gross Weekly Amount Effective"
            }
            BenefitTypeNetWeeklyAmountEffective => {
                "Benefit Type Net Weekly Amount Effective"
            }
            BenefitTypePeriodStart => "Benefit Type Period Start",
            BenefitTypePeriodEnd => "Benefit Type Period End",
            BenefitDebitStart => "Benefit Debit Start",
            Acknowledgment => "Acknowledgment",
            BenefitDebitEnd => "Benefit Debit End",
            BenefitCreditStart => "Benefit Credit Start",
            BenefitCreditEnd => "Benefit Credit End",
            BenefitTransferStart => "Benefit Transfer Start",
            BenefitTransferEnd => "Benefit Transfer End",
            WageEffective => "Wage Effective",
            FullDenialEffective => "Full Denial Effective",
            FullDenialRescission => "Full Denial Rescission",
            PaymentIssue => "Payment Issue",
            PaymentPeriodStart => "Payment Period Start",
            PaymentPeriodEnd => "Payment Period End",
            EmployerReportedInjuryToClaimAdministrator => {
                "Employer Reported Injury To Claim Administrator"
            }
            SurveyYear => "Survey Year",
            ControvertDate => "Controvert Date",
            BilledThrough => "Billed Through",
            BusinessControlChange => "Business Control Change",
            CourtRegistration => "Court Registration",
            AnnualReportDue => "Annual Report Due",
            ClaimNotificationReceived => "Claim Notification Received",
            ConversionPrivilegeEnd => "Conversion Privilege End",
            DividendApplied => "Dividend Applied",
            InForce => "In-force",
            PaidUp => "Paid-up",
            PremiumChange => "Premium Change",
            PolicyEffectiveOnOrBefore => "Policy Effective on or before",
            AssetAndLiabilitySchedule => "Asset and Liability Schedule",
            AnnualReportMailed => "Annual Report Mailed",
            PolicyEffectiveOnOrAfter => "Policy Effective on or after",
            AnnualReportFiled => "Annual Report Filed",
            AuditPeriodAfterSplitDate => "Audit Period After Split Date",
            AuditPeriodPriorToSplitDate => "Audit Period Prior to Split Date",
            ExposureSourcePeriod => "Exposure Source Period",
            SubcontractorPeriodOfHire => "Subcontractor Period of Hire",
            Divorce => "Divorce",
            PowerOfAttorney => "Power of Attorney",
            UniformGiftsToMinorsAccountEstablished => {
                "Uniform Gifts to Minors Account Established"
            }
            MedicarePartAEligibilityBeginDate => "Medicare Part A Eligibility Begin Date",
            MedicarePartAEligibilityEndDate => "Medicare Part A Eligibility End Date",
            MedicarePartACoverageEffectiveDate => {
                "Medicare Part A Coverage Effective Date"
            }
            MedicarePartATerminationDate => "Medicare Part A Termination Date",
            MedicarePartBEligibilityBeginDate => "Medicare Part B Eligibility Begin Date",
            MedicarePartBEligibilityEndDate => "Medicare Part B Eligibility End Date",
            MedicarePartBCoverageEffectiveDate => {
                "Medicare Part B Coverage Effective Date"
            }
            MedicarePartBTerminationDate => "Medicare Part B Termination Date",
            LoadingPeriod => "Loading Period",
            DateOnWhichAssetsJudgedInsufficientToPayCreditors => {
                "Date on which Assets Judged Insufficient to Pay Creditors"
            }
            EmployeesTemporarilyLaidOffBeginPeriod => {
                "Employees Temporarily Laid Off Begin Period"
            }
            EmployeesTemporarilyLaidOffEndPeriod => {
                "Employees Temporarily Laid Off End Period"
            }
            FirstPublished => "First Published",
            ForecastPeriodStart => "Forecast Period Start",
            ForecastPeriodEnd => "Forecast Period End",
            InvestigationStart => "Investigation Start",
            InvestigationEnd => "Investigation End",
            LastPublished => "Last Published",
            LatestBalanceSheet => "Latest Balance Sheet",
            SharePrice => "Share Price",
            StopDistribution => "Stop Distribution",
            MaximumCreditDate => "Maximum Credit Date",
            FoundingDate => "Founding Date",
            RepaymentPlanStartDate => "Repayment Plan Start Date",
            MedicarePartDEligibilityBeginDate => "Medicare Part D Eligibility Begin Date",
            MedicarePartDEligibilityEndDate => "Medicare Part D Eligibility End Date",
            MedicarePartDCoverageEffectiveDate => {
                "Medicare Part D Coverage Effective Date"
            }
            MedicarePartDTerminationDate => "Medicare Part D Termination Date",
            AnnualReportDelinquency => "Annual Report Delinquency",
            WithheldDate => "Withheld Date",
            ComplianceAudit => "Compliance Audit",
            ContractorSafetyPerformanceEvaluation => {
                "Contractor Safety Performance Evaluation"
            }
            ContractorSafetyProceduresReview => "Contractor Safety Procedures Review",
            DateOfEquipmentInspection => "Date of Equipment Inspection",
            DateOfSafetyInspection => "Date of Safety Inspection",
            EmployeesParticipationPlanReview => "Employees Participation Plan Review",
            CodeBAG => "Expected Completion of Changes Resulting from\nCompliance Audit",
            CodeBAH => {
                "Expected Completion of Changes Resulting from\nProcess Hazard Analysis"
            }
            ExpectedCompletionOfChangesResultingFromHazardReview => {
                "Expected Completion of Changes Resulting from Hazard Review"
            }
            HazardReviewCompletion => "Hazard Review Completion",
            HotWorkPermitProceduresReview => "Hot Work Permit Procedures Review",
            Investigation => "Investigation",
            MaintenanceProceduresReview => "Maintenance Procedures Review",
            ManagementOfChangeProceduresReview => {
                "Management of Change Procedures Review"
            }
            OperatingProceduresReview => "Operating Procedures Review",
            SafetyInformationReview => "Safety Information Review",
            Training => "Training",
            TrainingProgramReview => "Training Program Review",
            BillbackEndDate => "Billback End Date",
            ProgramPerformanceEndDate => "Program Performance End Date",
            ProgramPerformanceStartDate => "Program Performance Start Date",
            BeginningOfGracePeriod => "Beginning of Grace Period",
            BillingActivities => "Billing Activities",
            BeginningOfInterestPaidAfterClaim => "Beginning of Interest Paid After Claim",
            BillbackStartDate => "Billback Start Date",
            ChangedAccountingDate => "Changed Accounting Date",
            CustomsCargoRelease => "Customs Cargo Release",
            ContractDefinitizationDate => "Contract Definitization Date",
            CampaignEndDate => "Campaign End Date",
            CampaignStartDate => "Campaign Start Date",
            MaintenanceComment => "Maintenance Comment",
            Formation => "Formation",
            Continuance => "Continuance",
            Merger => "Merger",
            YearDue => "Year Due",
            NextAnnualMeeting => "Next Annual Meeting",
            EndOfLastFiscalYear => "End of Last Fiscal Year",
            YearBeginning => "Year Beginning",
            StartedDoingBusiness => "Started Doing Business",
            SwornAndSubscribed => "Sworn and Subscribed",
            CalendarYear => "Calendar Year",
            Asset => "Asset",
            Inactivity => "Inactivity",
            HighCapitalYear => "High Capital Year",
            ClosingDateOfFirstBalanceSheet => "Closing Date of First Balance Sheet",
            ClosedUntil => "Closed Until",
            Compliance => "Compliance",
            ConvertedIntoHoldingCompany => "Converted into Holding Company",
            CareOfSuppliesInStorageInspectionDate => {
                "Care of Supplies in Storage Inspection Date"
            }
            ConsumerProductAvailabilityDate => "Consumer Product Availability Date",
            ConsumerProductInformationPublicationDate => {
                "Consumer Product Information Publication Date"
            }
            ConsumerProductVariantEndEffectiveDate => {
                "Consumer Product Variant End Effective Date"
            }
            ConsumerProductVariantDiscontinuedDate => {
                "Consumer Product Variant Discontinued Date"
            }
            ConsumerProductVariantCancelledDate => {
                "Consumer Product Variant Cancelled Date"
            }
            ConsumerProductVariantStartEffectiveDate => {
                "Consumer Product Variant Start Effective Date"
            }
            ClaimRevised => "Claim Revised",
            CurrentList => "Current List",
            CommunityVisibility => "Community Visibility",
            AccountFrozen => "Account Frozen",
            Declaration => "Declaration",
            DeedNotAvailable => "Deed Not Available",
            Delete => "Delete",
            DetrimentalInformationReceived => "Detrimental Information Received",
            Deferral => "Deferral",
            DepartureFromSpecification => "Departure From Specification",
            CodeDIL => "Deed In Lieu (DIL) Approved",
            DelayedInterestPaidThrough => "Delayed Interest Paid Through",
            Disposition => "Disposition",
            DateOfLastContact => "Date of Last Contact",
            DateOfAbandonment => "Date of Abandonment",
            DateOfDelinquency => "Date of Delinquency",
            DeliveryOrderIssued => "Delivery Order Issued",
            Repossession => "Repossession",
            Disposal => "Disposal",
            DeedAndTitleReceived => "Deed and Title Received",
            TechnicalDataSupplyBy => "Technical Data Supply By",
            DeedAndTitleRequested => "Deed and Title Requested",
            TenureDecision => "Tenure Decision",
            MostRecentPositionChange => "Most Recent Position Change",
            FeePayment => "Fee Payment",
            StartDateForContinuousEmployment => "Start Date for Continuous Employment",
            StartDateForCurrentPosition => "Start Date for Current Position",
            StartDateForOriginalPosition => "Start Date for Original Position",
            FiscalYear => "Fiscal Year",
            EndAvailabilityDate => "End Availability Date",
            EstimatedConstructionDate => "Estimated Construction Date",
            EstimatedCompletionFirstPriorMonth => {
                "Estimated Completion - First Prior Month"
            }
            EstimatedCompletionSecondPriorMonth => {
                "Estimated Completion - Second Prior Month"
            }
            EstimatedCompletionThirdPriorMonth => {
                "Estimated Completion - Third Prior Month"
            }
            Affirmed => "Affirmed",
            Auction => "Auction",
            Authorized => "Authorized",
            Contribution => "Contribution",
            Executed => "Executed",
            Forgiven => "Forgiven",
            Presented => "Presented",
            LegislativeSession => "Legislative Session",
            Organized => "Organized",
            Pledged => "Pledged",
            PrimaryElection => "Primary Election",
            Qualified => "Qualified",
            Refunded => "Refunded",
            Rescinded => "Rescinded",
            RestructuredFrom => "Restructured From",
            Vote => "Vote",
            EmployerKnowledgeOfTheDisability => "Employer Knowledge of the Disability",
            EndDateMaximumBuyingQuantity => "End Date Maximum Buying Quantity",
            EndDateMinimumBuyingQuantity => "End Date Minimum Buying Quantity",
            EstimatePreparation => "Estimate Preparation",
            EstimateComment => "Estimate Comment",
            EffectiveStartDate => "Effective Start Date",
            EstimatedStartFirstPriorMonth => "Estimated Start - First Prior Month",
            EstimatedStartSecondPriorMonth => "Estimated Start - Second Prior Month",
            EstimatedStartThirdPriorMonth => "Estimated Start - Third Prior Month",
            EarliestFilingPeriod => "Earliest Filing Period",
            Exposure => "Exposure",
            Export => "Export",
            FacilityLatestBillingAction => "Facility Latest Billing Action",
            FacilityEarliestBillingAction => "Facility Earliest Billing Action",
            Financial => "Financial Information",
            FirstOrder => "First Order",
            FinalInterestAccrual => "Final Interest Accrual",
            FundingPeriodEnd => "Funding Period - End",
            FundingPeriodStart => "Funding Period - Start",
            FirstAvailableShipDate => "First Available Ship Date",
            FreeServiceCallEndDate => "Free Service Call End Date",
            FreeServiceCallStartDate => "Free Service Call Start Date",
            Graduated => "Graduated",
            HomeHealthDateOfEarliestBillableAction => {
                "Home Health Date of Earliest Billable Action"
            }
            HomeHealthEpisode => "Home Health Episode",
            HomeHealthDateOfLatestBillableAction => {
                "Home Health Date of Latest Billable Action"
            }
            HostTrainDate => "Host Train Date",
            ConvertedToElectronicDate => "Converted to Electronic Date",
            InsolvencyDischargeGranted => "Insolvency Discharge Granted",
            InitialFederalHousingAuthorityClaimPayment => {
                "Initial Federal Housing Authority Claim Payment"
            }
            Incorporation => "Incorporation",
            ImageLastUpdateDate => "Image Last Update Date",
            ImbalancePeriod => "Imbalance Period",
            Import => "Import",
            Incident => "Incident",
            InactiveUntil => "Inactive Until",
            InterestOnPresaleStart => "Interest on Presale Start",
            InitialVeteransAdministrationClaimPayment => {
                "Initial Veterans Administration Claim Payment"
            }
            KeyEventFiscalYear => "Key Event Fiscal Year",
            KeyEventCalendarYear => "Key Event Calendar Year",
            LastAnnualMeeting => "Last Annual Meeting",
            LastCheckForBalanceSheetUpdate => "Last Check for Balance Sheet Update",
            LastCapitalChange => "Last Capital Change",
            LetterOfAgreement => "Letter of Agreement",
            LetterOfLiability => "Letter of Liability",
            Liquidation => "Liquidation",
            LowPeriod => "Low Period",
            EquipmentLogEntry => "Equipment Log Entry",
            ListPriceChange => "List Price Change",
            LegalStructureChange => "Legal Structure Change",
            LastSubmissionDate => "Last Submission Date",
            LatestFilingPeriod => "Latest Filing Period",
            MeterReading => "Meter Reading",
            LatestMaterialSafetyDataSheetDate => "Latest Material Safety Data Sheet Date",
            PresentName => "Present Name",
            NegotiatedFinish => "Negotiated Finish",
            CodeNOD => "Notice of Delinquency (NOD) Received",
            NotRegistered => "Not Registered",
            NegotiatedStart => "Negotiated Start",
            OrganicCertificationDate => "Organic Certification Date",
            OriginalList => "Original List",
            PresentControl => "Present Control",
            PrimaryCoverageClaimPaid => "Primary Coverage Claim Paid",
            PrimaryCoverageClaimSubmission => "Primary Coverage Claim Submission",
            PriceChangesAllowedFromDate => "Price Changes Allowed From Date",
            PriceChangesAllowedToDate => "Price Changes Allowed To Date",
            ProductDiscontinuedButStillAvailable => {
                "Product Discontinued but Still Available"
            }
            PartialDenialEffective => "Partial Denial Effective",
            PartialDenialRescission => "Partial Denial Rescission",
            CorrectProgramStartDate => "Correct Program Start Date",
            CorrectProgramEndDate => "Correct Program End Date",
            CorrectContractStartDate => "Correct Contract Start Date",
            PrivilegeDetailsVerification => "Privilege Details Verification",
            CorrectContractEndDate => "Correct Contract End Date",
            ProgramEndDate => "Program End Date",
            ProductImageStartDate => "Product Image Start Date",
            PresentLegalStructure => "Present Legal Structure",
            PackagingMaterialEffectiveDate => "Packaging Material Effective Date",
            PoolPolicyClaimSubmission => "Pool Policy Claim Submission",
            PostPaidDate => "Post Paid Date",
            PeakPeriod => "Peak Period",
            PreviouslyReportedDateOfBirth => "Previously Reported Date of Birth",
            PresentedToReceivers => "Presented to Receivers",
            PropertySaleApproved => "Property Sale Approved",
            PropertySaleClosed => "Property Sale Closed",
            ProgramStartDate => "Program Start Date",
            PropertySaleConfirmation => "Property Sale Confirmation",
            PaidToDate => "Paid To Date",
            PlannedObsolescenceItem => "Planned Obsolescence Item",
            PickUpDate => "Pick-up Date",
            ReceiverAppointed => "Receiver Appointed",
            RemanufactureDate => "Remanufacture Date",
            Resigned => "Resigned",
            RequestedFinish => "Requested Finish",
            RecoveryFinish => "Recovery Finish",
            ReferredFrom => "Referred From",
            RentSurvey => "Rent Survey",
            ReceivedInTheMail => "Received in the Mail",
            Revocation => "Revocation",
            RequestedStart => "Requested Start",
            RecoveryStart => "Recovery Start",
            ReferredTo => "Referred To",
            SocialSecurityClaimsVerification => "Social Security Claims Verification",
            SoleDirectorshipDate => "Sole Directorship Date",
            StartDateMaximumBuyingQuantity => "Start Date Maximum Buying Quantity",
            SubsequentFederalHousingAuthorityClaimPayment => {
                "Subsequent Federal Housing Authority Claim Payment"
            }
            StartDateMinimumBuyingQuantity => "Start Date Minimum Buying Quantity",
            InitialSupportDate => "Initial Support Date",
            SuggestedRetailPriceEffectiveDate => "Suggested Retail Price Effective Date",
            Transition => "Transition",
            SubsequentVeteransAdministrationClaimPayment => {
                "Subsequent Veterans Administration Claim Payment"
            }
            TradeStyleRegistered => "Trade Style Registered",
            TrialStarted => "Trial Started",
            TrialSet => "Trial Set",
            TenantTrainDate => "Tenant Train Date",
            CodeVAT => "Value Added Tax (VAT) Claims Verification",
            ValidUntil => "Valid Until",
            SampleCollected => "Sample Collected",
            StatusChange => "Status Change",
            ConstructionStart => "Construction Start",
            Recompletion => "Recompletion",
            LastLogged => "Last Logged",
            WellLogRun => "Well Log Run",
            SurfaceCasingAuthorityApproval => "Surface Casing Authority Approval",
            ReachedTotalDepth => "Reached Total Depth",
            SpacingOrderUnitAssigned => "Spacing Order Unit Assigned",
            RigArrival => "Rig Arrival",
            LocationExceptionOrderNumberAssigned => {
                "Location Exception Order Number Assigned"
            }
            SidetrackedWellbore => "Sidetracked Wellbore",
            TimeEmployeeBeganWork => "Time Employee Began Work",
            Waybill => "Waybill",
            OrderDay => "Order Day",
            DeliveryDay => "Delivery Day",
            OrderCutOffTime => "Order Cut-Off Time",
            ActiveItem => "Active Item",
            MatureItem => "Mature Item",
            IntroductoryItem => "Introductory Item",
            ObsoleteItem => "Obsolete Item",
            BestBeforeDate => "Best Before Date",
            HarvestDate => "Harvest Date",
            ProgrammedFiscalYear => "Programmed Fiscal Year",
            ProgrammedCalendarYear => "Programmed Calendar Year",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<DateTimeQualifier> {
        {
            use DateTimeQualifier::*;
            match description {
                "Cancel After" => Some(CancelAfter),
                "Delivery Requested" => Some(DeliveryRequested),
                "Invoice" => Some(Invoice),
                "Purchase Order" => Some(PurchaseOrder),
                "Sailing" => Some(Sailing),
                "Sold" => Some(Sold),
                "Effective" => Some(Effective),
                "Purchase Order Received" => Some(PurchaseOrderReceived),
                "Process" => Some(Process),
                "Requested Ship" => Some(RequestedShip),
                "Shipped" => Some(Shipped),
                "Terms Discount Due" => Some(TermsDiscountDue),
                "Terms Net Due" => Some(TermsNetDue),
                "Deferred Payment" => Some(DeferredPayment),
                "Promotion Start" => Some(PromotionStart),
                "Promotion End" => Some(PromotionEnd),
                "Estimated Delivery" => Some(EstimatedDelivery),
                "Available" => Some(Available),
                "Unloaded" => Some(Unloaded),
                "Check" => Some(Check),
                "Charge Back" => Some(ChargeBack),
                "Freight Bill" => Some(FreightBill),
                "Promotion Order - Start" => Some(PromotionOrderStart),
                "Promotion Order - End" => Some(PromotionOrderEnd),
                "Promotion Ship - Start" => Some(PromotionShipStart),
                "Promotion Ship - End" => Some(PromotionShipEnd),
                "Promotion Requested Delivery - Start" => {
                    Some(PromotionRequestedDeliveryStart)
                }
                "Promotion Requested Delivery - End" => {
                    Some(PromotionRequestedDeliveryEnd)
                }
                "Promotion Performance - Start" => Some(PromotionPerformanceStart),
                "Promotion Performance - End" => Some(PromotionPerformanceEnd),
                "Promotion Invoice Performance - Start" => {
                    Some(PromotionInvoicePerformanceStart)
                }
                "Promotion Invoice Performance - End" => {
                    Some(PromotionInvoicePerformanceEnd)
                }
                "Promotion Floor Stock Protect - Start" => {
                    Some(PromotionFloorStockProtectStart)
                }
                "Promotion Floor Stock Protect - End" => {
                    Some(PromotionFloorStockProtectEnd)
                }
                "Delivered" => Some(Delivered),
                "Expiration" => Some(Expiration),
                "Ship Not Before" => Some(ShipNotBefore),
                "Ship No Later" => Some(ShipNoLater),
                "Ship Week of" => Some(ShipWeekOf),
                "Status (After and Including)" => Some(Code040),
                "Status (Prior and Including)" => Some(Code041),
                "Superseded" => Some(Superseded),
                "Publication" => Some(Publication),
                "Settlement Date as Specified by the Originator" => {
                    Some(SettlementDateAsSpecifiedByTheOriginator)
                }
                "Endorsement Date" => Some(EndorsementDate),
                "Field Failure" => Some(FieldFailure),
                "Functional Test" => Some(FunctionalTest),
                "System Test" => Some(SystemTest),
                "Prototype Test" => Some(PrototypeTest),
                "Received" => Some(Received),
                "Cumulative Quantity Start" => Some(CumulativeQuantityStart),
                "Cumulative Quantity End" => Some(CumulativeQuantityEnd),
                "Buyers Local" => Some(BuyersLocal),
                "Sellers Local" => Some(SellersLocal),
                "Confirmed" => Some(Confirmed),
                "Estimated Port of Entry" => Some(EstimatedPortOfEntry),
                "Actual Port of Entry" => Some(ActualPortOfEntry),
                "Customs Clearance" => Some(CustomsClearance),
                "Inland Ship" => Some(InlandShip),
                "Engineering Change Level" => Some(EngineeringChangeLevel),
                "Cancel if Not Delivered by" => Some(CancelIfNotDeliveredBy),
                "Blueprint" => Some(Blueprint),
                "Do Not Deliver After" => Some(DoNotDeliverAfter),
                "Do Not Deliver Before" => Some(DoNotDeliverBefore),
                "1st Schedule Delivery" => Some(Code065),
                "1st Schedule Ship" => Some(Code066),
                "Current Schedule Delivery" => Some(CurrentScheduleDelivery),
                "Current Schedule Ship" => Some(CurrentScheduleShip),
                "Promised for Delivery" => Some(PromisedForDelivery),
                "Scheduled for Delivery (After and Including)" => Some(Code070),
                "Requested for Delivery (After and Including)" => Some(Code071),
                "Promised for Delivery (After and Including)" => Some(Code072),
                "Scheduled for Delivery (Prior to and Including)" => Some(Code073),
                "Requested for Delivery (Prior to and Including)" => Some(Code074),
                "Promised for Delivery (Prior to and Including)" => Some(Code075),
                "Scheduled for Delivery (Week of)" => Some(Code076),
                "Requested for Delivery (Week of)" => Some(Code077),
                "Promised for Delivery (Week of)" => Some(Code078),
                "Promised for Shipment" => Some(PromisedForShipment),
                "Scheduled for Shipment (After and Including)" => Some(Code080),
                "Requested for Shipment (After and Including)" => Some(Code081),
                "Promised for Shipment (After and Including)" => Some(Code082),
                "Scheduled for Shipment (Prior to and Including)" => Some(Code083),
                "Requested for Shipment (Prior to and Including)" => Some(Code084),
                "Promised for Shipment (Prior to and Including)" => Some(Code085),
                "Scheduled for Shipment (Week of)" => Some(Code086),
                "Requested for Shipment (Week of)" => Some(Code087),
                "Promised for Shipment (Week of)" => Some(Code088),
                "Inquiry" => Some(Inquiry),
                "Report Start" => Some(ReportStart),
                "Report End" => Some(ReportEnd),
                "Contract Effective" => Some(ContractEffective),
                "Contract Expiration" => Some(ContractExpiration),
                "Manufacture" => Some(Manufacture),
                "Bill of Lading" => Some(BillOfLading),
                "Discharge" => Some(Discharge),
                "Transaction Creation" => Some(TransactionCreation),
                "Bid (Effective)" => Some(Code098),
                "Bid Open (Date Bids Will Be Opened)" => Some(Code099),
                "No Shipping Schedule Established as of" => {
                    Some(NoShippingScheduleEstablishedAsOf)
                }
                "No Production Schedule Established as of" => {
                    Some(NoProductionScheduleEstablishedAsOf)
                }
                "Issue" => Some(Issue),
                "Award" => Some(Award),
                "System Survey" => Some(SystemSurvey),
                "Quality Rating" => Some(QualityRating),
                "Required By" => Some(RequiredBy),
                "Deposit" => Some(Deposit),
                "Postmark" => Some(Postmark),
                "Received at Lockbox" => Some(ReceivedAtLockbox),
                "Originally Scheduled Ship" => Some(OriginallyScheduledShip),
                "Manifest/Ship Notice" => Some(ManifestShipNotice),
                "Buyers Dock" => Some(BuyersDock),
                "Sample Required" => Some(SampleRequired),
                "Tooling Required" => Some(ToolingRequired),
                "Sample Available" => Some(SampleAvailable),
                "Scheduled Interchange Delivery" => Some(ScheduledInterchangeDelivery),
                "Requested Pickup" => Some(RequestedPickup),
                "Test Performed" => Some(TestPerformed),
                "Control Plan" => Some(ControlPlan),
                "Feasibility Sign Off" => Some(FeasibilitySignOff),
                "Failure Mode Effective" => Some(FailureModeEffective),
                "Group Contract Effective" => Some(GroupContractEffective),
                "Group Contract Expiration" => Some(GroupContractExpiration),
                "Wholesale Contract Effective" => Some(WholesaleContractEffective),
                "Wholesale Contract Expiration" => Some(WholesaleContractExpiration),
                "Replacement Effective" => Some(ReplacementEffective),
                "Customer Contract Effective" => Some(CustomerContractEffective),
                "Customer Contract Expiration" => Some(CustomerContractExpiration),
                "Item Contract Effective" => Some(ItemContractEffective),
                "Item Contract Expiration" => Some(ItemContractExpiration),
                "Accounts Receivable - Statement Date" => {
                    Some(AccountsReceivableStatementDate)
                }
                "Ready for Inspection" => Some(ReadyForInspection),
                "Booking" => Some(Booking),
                "Technical Rating" => Some(TechnicalRating),
                "Delivery Rating" => Some(DeliveryRating),
                "Commercial Rating" => Some(CommercialRating),
                "Estimated" => Some(Estimated),
                "Actual" => Some(Actual),
                "Assigned" => Some(Assigned),
                "Loss" => Some(Loss),
                "Due Date of First Payment to Principal and Interest" => {
                    Some(DueDateOfFirstPaymentToPrincipalAndInterest)
                }
                "Estimated Acceptance" => Some(EstimatedAcceptance),
                "Opening Date" => Some(OpeningDate),
                "Closing Date" => Some(ClosingDate),
                "Due Date Last Complete Installment Paid" => {
                    Some(DueDateLastCompleteInstallmentPaid)
                }
                "Date of Local Office Approval of Conveyance of Damaged Real Estate Property" => {
                    Some(
                        DateOfLocalOfficeApprovalOfConveyanceOfDamagedRealEstateProperty,
                    )
                }
                "Date Deed Filed for Record" => Some(DateDeedFiledForRecord),
                "Service Period Start" => Some(ServicePeriodStart),
                "Service Period End" => Some(ServicePeriodEnd),
                "Effective Date of Change" => Some(EffectiveDateOfChange),
                "Service Interruption" => Some(ServiceInterruption),
                "Adjustment Period Start" => Some(AdjustmentPeriodStart),
                "Adjustment Period End" => Some(AdjustmentPeriodEnd),
                "Allotment Period Start" => Some(AllotmentPeriodStart),
                "Test Period Start" => Some(TestPeriodStart),
                "Test Period Ending" => Some(TestPeriodEnding),
                "Bid Price Exception" => Some(BidPriceException),
                "Samples to be Returned By" => Some(SamplesToBeReturnedBy),
                "Loaded on Vessel" => Some(LoadedOnVessel),
                "Pending Archive" => Some(PendingArchive),
                "Actual Archive" => Some(ActualArchive),
                "First Issue" => Some(FirstIssue),
                "Final Issue" => Some(FinalIssue),
                "Message" => Some(Message),
                "Most Recent Revision (or Initial Version)" => Some(Code167),
                "Release" => Some(Release),
                "Product Availability Date" => Some(ProductAvailabilityDate),
                "Supplemental Issue" => Some(SupplementalIssue),
                "Revision" => Some(Revision),
                "Correction" => Some(Correction),
                "Week Ending" => Some(WeekEnding),
                "Month Ending" => Some(MonthEnding),
                "Cancel if not shipped by" => Some(CancelIfNotShippedBy),
                "Expedited on" => Some(ExpeditedOn),
                "Cancellation" => Some(Cancellation),
                "Hold (as of)" => Some(Code178),
                "Hold as Stock (as of)" => Some(Code179),
                "No Promise (as of)" => Some(Code180),
                "Stop Work (as of)" => Some(Code181),
                "Will Advise (as of)" => Some(Code182),
                "Connection" => Some(Connection),
                "Inventory" => Some(Inventory),
                "Vessel Registry" => Some(VesselRegistry),
                "Invoice Period Start" => Some(InvoicePeriodStart),
                "Invoice Period End" => Some(InvoicePeriodEnd),
                "Credit Advice" => Some(CreditAdvice),
                "Debit Advice" => Some(DebitAdvice),
                "Released to Vessel" => Some(ReleasedToVessel),
                "Material Specification" => Some(MaterialSpecification),
                "Delivery Ticket" => Some(DeliveryTicket),
                "Period Start" => Some(PeriodStart),
                "Period End" => Some(PeriodEnd),
                "Contract Re-Open" => Some(ContractReOpen),
                "Start" => Some(Start),
                "End" => Some(End),
                "Completion" => Some(Completion),
                "Seal" => Some(Seal),
                "Assembly Start" => Some(AssemblyStart),
                "Acceptance" => Some(Acceptance),
                "Master Lease Agreement" => Some(MasterLeaseAgreement),
                "First Produced" => Some(FirstProduced),
                "Official Rail Car Interchange (Either Actual or Agreed Upon)" => {
                    Some(Code204)
                }
                "Transmitted" => Some(Transmitted),
                "Status (Outside Processor)" => Some(Code206),
                "Status (Commercial)" => Some(Code207),
                "Lot Number Expiration" => Some(LotNumberExpiration),
                "Contract Performance Start" => Some(ContractPerformanceStart),
                "Contract Performance Delivery" => Some(ContractPerformanceDelivery),
                "Service Requested" => Some(ServiceRequested),
                "Returned to Customer" => Some(ReturnedToCustomer),
                "Adjustment to Bill Dated" => Some(AdjustmentToBillDated),
                "Date of Repair/Service" => Some(DateOfRepairService),
                "Interruption Start" => Some(InterruptionStart),
                "Interruption End" => Some(InterruptionEnd),
                "Spud" => Some(Spud),
                "Initial Completion" => Some(InitialCompletion),
                "Plugged and Abandoned" => Some(PluggedAndAbandoned),
                "Penalty" => Some(Penalty),
                "Penalty Begin" => Some(PenaltyBegin),
                "Birth" => Some(Birth),
                "Birth Certificate" => Some(BirthCertificate),
                "Adoption" => Some(Adoption),
                "Christening" => Some(Christening),
                "Lease Commencement" => Some(LeaseCommencement),
                "Lease Term Start" => Some(LeaseTermStart),
                "Lease Term End" => Some(LeaseTermEnd),
                "Rent Start" => Some(RentStart),
                "Installation" => Some(Installation),
                "Progress Payment" => Some(ProgressPayment),
                "Claim Statement Period Start" => Some(ClaimStatementPeriodStart),
                "Claim Statement Period End" => Some(ClaimStatementPeriodEnd),
                "Settlement Date" => Some(SettlementDate),
                "Delayed Billing (Not Delayed Payment)" => Some(Code235),
                "Lender Credit Check" => Some(LenderCreditCheck),
                "Student Signed" => Some(StudentSigned),
                "Schedule Release" => Some(ScheduleRelease),
                "Baseline" => Some(Baseline),
                "Baseline Start" => Some(BaselineStart),
                "Baseline Complete" => Some(BaselineComplete),
                "Actual Start" => Some(ActualStart),
                "Actual Complete" => Some(ActualComplete),
                "Estimated Start" => Some(EstimatedStart),
                "Estimated Completion" => Some(EstimatedCompletion),
                "Start no earlier than" => Some(StartNoEarlierThan),
                "Start no later than" => Some(StartNoLaterThan),
                "Finish no later than" => Some(FinishNoLaterThan),
                "Finish no earlier than" => Some(FinishNoEarlierThan),
                "Mandatory (or Target) Start" => Some(Code250),
                "Mandatory (or Target) Finish" => Some(Code251),
                "Early Start" => Some(EarlyStart),
                "Early Finish" => Some(EarlyFinish),
                "Late Start" => Some(LateStart),
                "Late Finish" => Some(LateFinish),
                "Scheduled Start" => Some(ScheduledStart),
                "Scheduled Finish" => Some(ScheduledFinish),
                "Original Early Start" => Some(OriginalEarlyStart),
                "Original Early Finish" => Some(OriginalEarlyFinish),
                "Rest Day" => Some(RestDay),
                "Rest Start" => Some(RestStart),
                "Rest Finish" => Some(RestFinish),
                "Holiday" => Some(Holiday),
                "Holiday Start" => Some(HolidayStart),
                "Holiday Finish" => Some(HolidayFinish),
                "Base" => Some(Base),
                "Timenow" => Some(Timenow),
                "End Date of Support" => Some(EndDateOfSupport),
                "Date Account Matures" => Some(DateAccountMatures),
                "Date Filed" => Some(DateFiled),
                "Penalty End" => Some(PenaltyEnd),
                "Exit Plant Date" => Some(ExitPlantDate),
                "Latest On Board Carrier Date" => Some(LatestOnBoardCarrierDate),
                "Requested Departure Date" => Some(RequestedDepartureDate),
                "Approved" => Some(Approved),
                "Contract Start" => Some(ContractStart),
                "Contract Definition" => Some(ContractDefinition),
                "Last Item Delivery" => Some(LastItemDelivery),
                "Contract Completion" => Some(ContractCompletion),
                "Date Course of Orthodontics Treatment Began or is Expected to Begin" => {
                    Some(DateCourseOfOrthodonticsTreatmentBeganOrIsExpectedToBegin)
                }
                "Over Target Baseline Month" => Some(OverTargetBaselineMonth),
                "Previous Report" => Some(PreviousReport),
                "Funds Appropriation - Start" => Some(FundsAppropriationStart),
                "Funds Appropriation - End" => Some(FundsAppropriationEnd),
                "Employment or Hire" => Some(EmploymentOrHire),
                "Retirement" => Some(Retirement),
                "Medicare" => Some(Medicare),
                "Consolidated Omnibus Budget Reconciliation Act (COBRA)" => Some(Code288),
                "Premium Paid to Date" => Some(PremiumPaidToDate),
                "Coordination of Benefits" => Some(CoordinationOfBenefits),
                "Plan" => Some(Plan),
                "Benefit" => Some(Benefit),
                "Education" => Some(Education),
                "Earnings Effective Date" => Some(EarningsEffectiveDate),
                "Primary Care Provider" => Some(PrimaryCareProvider),
                "Initial Disability Period Return To Work" => {
                    Some(InitialDisabilityPeriodReturnToWork)
                }
                "Initial Disability Period Last Day Worked" => {
                    Some(InitialDisabilityPeriodLastDayWorked)
                }
                "Latest Absence" => Some(LatestAbsence),
                "Illness" => Some(Illness),
                "Enrollment Signature Date" => Some(EnrollmentSignatureDate),
                "Consolidated Omnibus Budget Reconciliation Act (COBRA) Qualifying Event" => {
                    Some(Code301)
                }
                "Maintenance" => Some(Maintenance),
                "Maintenance Effective" => Some(MaintenanceEffective),
                "Latest Visit or Consultation" => Some(LatestVisitOrConsultation),
                "Net Credit Service Date" => Some(NetCreditServiceDate),
                "Adjustment Effective Date" => Some(AdjustmentEffectiveDate),
                "Eligibility" => Some(Eligibility),
                "Pre-Award Survey" => Some(PreAwardSurvey),
                "Plan Termination" => Some(PlanTermination),
                "Date of Closing" => Some(DateOfClosing),
                "Latest Receiving Date/Cutoff Date" => {
                    Some(LatestReceivingDateCutoffDate)
                }
                "Salary Deferral" => Some(SalaryDeferral),
                "Cycle" => Some(Cycle),
                "Disability" => Some(Disability),
                "Offset" => Some(Offset),
                "Prior Incorrect Date of Birth" => Some(PriorIncorrectDateOfBirth),
                "Corrected Date of Birth" => Some(CorrectedDateOfBirth),
                "Added" => Some(Added),
                "Failed" => Some(Failed),
                "Date Foreclosure Proceedings Instituted" => {
                    Some(DateForeclosureProceedingsInstituted)
                }
                "Purchased" => Some(Purchased),
                "Put into Service" => Some(PutIntoService),
                "Replaced" => Some(Replaced),
                "Returned" => Some(Returned),
                "Disbursement Date" => Some(DisbursementDate),
                "Guarantee Date" => Some(GuaranteeDate),
                "Quarter Ending" => Some(QuarterEnding),
                "Changed" => Some(Changed),
                "Terminated" => Some(Terminated),
                "Referral Date" => Some(ReferralDate),
                "Evaluation Date" => Some(EvaluationDate),
                "Placement Date" => Some(PlacementDate),
                "Individual Education Plan (IEP)" => Some(Code333),
                "Re-evaluation Date" => Some(ReEvaluationDate),
                "Dismissal Date" => Some(DismissalDate),
                "Employment Begin" => Some(EmploymentBegin),
                "Employment End" => Some(EmploymentEnd),
                "Medicare Begin" => Some(MedicareBegin),
                "Medicare End" => Some(MedicareEnd),
                "Consolidated Omnibus Budget Reconciliation Act (COBRA) Begin" => {
                    Some(Code340)
                }
                "Consolidated Omnibus Budget Reconciliation Act (COBRA) End" => {
                    Some(Code341)
                }
                "Premium Paid to Date Begin" => Some(PremiumPaidToDateBegin),
                "Premium Paid to Date End" => Some(PremiumPaidToDateEnd),
                "Coordination of Benefits Begin" => Some(CoordinationOfBenefitsBegin),
                "Coordination of Benefits End" => Some(CoordinationOfBenefitsEnd),
                "Plan Begin" => Some(PlanBegin),
                "Plan End" => Some(PlanEnd),
                "Benefit Begin" => Some(BenefitBegin),
                "Benefit End" => Some(BenefitEnd),
                "Education Begin" => Some(EducationBegin),
                "Education End" => Some(EducationEnd),
                "Primary Care Provider Begin" => Some(PrimaryCareProviderBegin),
                "Primary Care Provider End" => Some(PrimaryCareProviderEnd),
                "Illness Begin" => Some(IllnessBegin),
                "Illness End" => Some(IllnessEnd),
                "Eligibility Begin" => Some(EligibilityBegin),
                "Eligibility End" => Some(EligibilityEnd),
                "Cycle Begin" => Some(CycleBegin),
                "Cycle End" => Some(CycleEnd),
                "Initial Disability Period Start" => Some(InitialDisabilityPeriodStart),
                "Initial Disability Period End" => Some(InitialDisabilityPeriodEnd),
                "Offset Begin" => Some(OffsetBegin),
                "Offset End" => Some(OffsetEnd),
                "Plan Period Election Begin" => Some(PlanPeriodElectionBegin),
                "Plan Period Election End" => Some(PlanPeriodElectionEnd),
                "Plan Period Election" => Some(PlanPeriodElection),
                "Due to Customer" => Some(DueToCustomer),
                "Submittal" => Some(Submittal),
                "Estimated Departure Date" => Some(EstimatedDepartureDate),
                "Actual Departure Date" => Some(ActualDepartureDate),
                "Estimated Arrival Date" => Some(EstimatedArrivalDate),
                "Actual Arrival Date" => Some(ActualArrivalDate),
                "Order Start" => Some(OrderStart),
                "Order End" => Some(OrderEnd),
                "Delivery Start" => Some(DeliveryStart),
                "Delivery End" => Some(DeliveryEnd),
                "Contract Costs Through" => Some(ContractCostsThrough),
                "Financial Information Submission" => {
                    Some(FinancialInformationSubmission)
                }
                "Business Termination" => Some(BusinessTermination),
                "Applicant Signed" => Some(ApplicantSigned),
                "Cosigner Signed" => Some(CosignerSigned),
                "Enrollment" => Some(Enrollment),
                "Adjusted Hire" => Some(AdjustedHire),
                "Credited Service" => Some(CreditedService),
                "Credited Service Begin" => Some(CreditedServiceBegin),
                "Credited Service End" => Some(CreditedServiceEnd),
                "Deferred Distribution" => Some(DeferredDistribution),
                "Payment Commencement" => Some(PaymentCommencement),
                "Payroll Period" => Some(PayrollPeriod),
                "Payroll Period Begin" => Some(PayrollPeriodBegin),
                "Payroll Period End" => Some(PayrollPeriodEnd),
                "Plan Entry" => Some(PlanEntry),
                "Plan Participation Suspension" => Some(PlanParticipationSuspension),
                "Rehire" => Some(Rehire),
                "Retermination" => Some(Retermination),
                "Termination" => Some(Termination),
                "Valuation" => Some(Valuation),
                "Vesting Service" => Some(VestingService),
                "Vesting Service Begin" => Some(VestingServiceBegin),
                "Vesting Service End" => Some(VestingServiceEnd),
                "Duplicate Bill" => Some(DuplicateBill),
                "Adjustment Promised" => Some(AdjustmentPromised),
                "Adjustment Processed" => Some(AdjustmentProcessed),
                "Year Ending" => Some(YearEnding),
                "Production" => Some(Production),
                "Material Classification" => Some(MaterialClassification),
                "Weighed" => Some(Weighed),
                "Date of Deed in Lieu" => Some(DateOfDeedInLieu),
                "Date of Firm Commitment" => Some(DateOfFirmCommitment),
                "Expiration Date of Extension to Foreclose" => {
                    Some(ExpirationDateOfExtensionToForeclose)
                }
                "Date of Notice to Convey" => Some(DateOfNoticeToConvey),
                "Date of Release of Bankruptcy" => Some(DateOfReleaseOfBankruptcy),
                "Optimistic Early Start" => Some(OptimisticEarlyStart),
                "Optimistic Early Finish" => Some(OptimisticEarlyFinish),
                "Optimistic Late Start" => Some(OptimisticLateStart),
                "Optimistic Late Finish" => Some(OptimisticLateFinish),
                "Most Likely Early Start" => Some(MostLikelyEarlyStart),
                "Most Likely Early Finish" => Some(MostLikelyEarlyFinish),
                "Most Likely Late Start" => Some(MostLikelyLateStart),
                "Most Likely Late Finish" => Some(MostLikelyLateFinish),
                "Pessimistic Early Start" => Some(PessimisticEarlyStart),
                "Pessimistic Early Finish" => Some(PessimisticEarlyFinish),
                "Pessimistic Late Start" => Some(PessimisticLateStart),
                "Pessimistic Late Finish" => Some(PessimisticLateFinish),
                "First Payment Due" => Some(FirstPaymentDue),
                "First Interest Payment Due" => Some(FirstInterestPaymentDue),
                "Subsequent Interest Payment Due" => Some(SubsequentInterestPaymentDue),
                "Irregular Interest Payment Due" => Some(IrregularInterestPaymentDue),
                "Guarantor Received" => Some(GuarantorReceived),
                "Onset of Current Symptoms or Illness" => {
                    Some(OnsetOfCurrentSymptomsOrIllness)
                }
                "Submission" => Some(Submission),
                "Removed" => Some(Removed),
                "Statement" => Some(Statement),
                "Admission" => Some(Admission),
                "Insurance Card" => Some(InsuranceCard),
                "Spouse Retirement" => Some(SpouseRetirement),
                "Onset of Similar Symptoms or Illness" => {
                    Some(OnsetOfSimilarSymptomsOrIllness)
                }
                "Accident" => Some(Accident),
                "Release of Information" => Some(ReleaseOf),
                "Prior Placement" => Some(PriorPlacement),
                "Date of Death" => Some(DateOfDeath),
                "Peer Review Organization (PRO) Approved Stay" => Some(Code443),
                "First Visit or Consultation" => Some(FirstVisitOrConsultation),
                "Initial Placement" => Some(InitialPlacement),
                "Replacement" => Some(Replacement),
                "Occurrence" => Some(Occurrence),
                "Occurrence Span" => Some(OccurrenceSpan),
                "Occurrence Span From" => Some(OccurrenceSpanFrom),
                "Occurrence Span To" => Some(OccurrenceSpanTo),
                "Initial Fee Due" => Some(InitialFeeDue),
                "Appliance Placement" => Some(AppliancePlacement),
                "Acute Manifestation of a Chronic Condition" => {
                    Some(AcuteManifestationOfAChronicCondition)
                }
                "Initial Treatment" => Some(InitialTreatment),
                "Last X-Ray" => Some(LastXRay),
                "Surgery" => Some(Surgery),
                "Continuous Passive Motion (CPM)" => Some(Code457),
                "Certification" => Some(Certification),
                "Nursing Home From" => Some(NursingHomeFrom),
                "Nursing Home To" => Some(NursingHomeTo),
                "Last Certification" => Some(LastCertification),
                "Date of Local Office Approval of Conveyance of Occupied Real Estate Property" => {
                    Some(
                        DateOfLocalOfficeApprovalOfConveyanceOfOccupiedRealEstateProperty,
                    )
                }
                "Begin Therapy" => Some(BeginTherapy),
                "Oxygen Therapy From" => Some(OxygenTherapyFrom),
                "Oxygen Therapy To" => Some(OxygenTherapyTo),
                "Oxygen Therapy" => Some(OxygenTherapy),
                "Signature" => Some(Signature),
                "Prescription Fill" => Some(PrescriptionFill),
                "Provider Signature" => Some(ProviderSignature),
                "Date of Local Office Certification of Conveyance of Damaged Real Estate Property" => {
                    Some(
                        DateOfLocalOfficeCertificationOfConveyanceOfDamagedRealEstateProperty,
                    )
                }
                "Prescription" => Some(Prescription),
                "Service" => Some(Service),
                "Medicaid Begin" => Some(MedicaidBegin),
                "Medicaid End" => Some(MedicaidEnd),
                "Medicaid" => Some(Medicaid),
                "Peer Review Organization (PRO) Approved Stay From" => Some(Code476),
                "Peer Review Organization (PRO) Approved Stay To" => Some(Code477),
                "Prescription From" => Some(PrescriptionFrom),
                "Prescription To" => Some(PrescriptionTo),
                "Arterial Blood Gas Test" => Some(ArterialBloodGasTest),
                "Oxygen Saturation Test" => Some(OxygenSaturationTest),
                "Pregnancy Begin" => Some(PregnancyBegin),
                "Pregnancy End" => Some(PregnancyEnd),
                "Last Menstrual Period" => Some(LastMenstrualPeriod),
                "Injury Begin" => Some(InjuryBegin),
                "Injury End" => Some(InjuryEnd),
                "Nursing Home" => Some(NursingHome),
                "Collateral Dependent" => Some(CollateralDependent),
                "Collateral Dependent Begin" => Some(CollateralDependentBegin),
                "Collateral Dependent End" => Some(CollateralDependentEnd),
                "Sponsored Dependent" => Some(SponsoredDependent),
                "Sponsored Dependent Begin" => Some(SponsoredDependentBegin),
                "Sponsored Dependent End" => Some(SponsoredDependentEnd),
                "Deductible" => Some(Deductible),
                "Out-of-Pocket" => Some(OutOfPocket),
                "Contract Audit Date" => Some(ContractAuditDate),
                "Latest Delivery Date at Pier" => Some(LatestDeliveryDateAtPier),
                "Mortgagee Reported Curtailment Date" => {
                    Some(MortgageeReportedCurtailmentDate)
                }
                "Mortgagee Official Signature Date" => {
                    Some(MortgageeOfficialSignatureDate)
                }
                "Resubmission" => Some(Resubmission),
                "Expected Reply" => Some(ExpectedReply),
                "Dropped to Less than Half Time" => Some(DroppedToLessThanHalfTime),
                "Repayment Begin" => Some(RepaymentBegin),
                "Loan Servicing Transfer" => Some(LoanServicingTransfer),
                "Loan Purchase" => Some(LoanPurchase),
                "Last Notification" => Some(LastNotification),
                "Extract" => Some(Extract),
                "Extended" => Some(Extended),
                "Servicer Signature Date" => Some(ServicerSignatureDate),
                "Date Packed" => Some(DatePacked),
                "Shelf Life Expiration" => Some(ShelfLifeExpiration),
                "Warranty Expiration" => Some(WarrantyExpiration),
                "Overhauled" => Some(Overhauled),
                "Transferred" => Some(Transferred),
                "Notified" => Some(Notified),
                "Discovered" => Some(Discovered),
                "Inspected" => Some(Inspected),
                "Voucher (Date of)" => Some(Code518),
                "Date Bankruptcy Filed" => Some(DateBankruptcyFiled),
                "Date of Damage" => Some(DateOfDamage),
                "Date Hazard Insurance Policy Cancelled" => {
                    Some(DateHazardInsurancePolicyCancelled)
                }
                "Expiration Date to Submit Title Evidence" => {
                    Some(ExpirationDateToSubmitTitleEvidence)
                }
                "Date of Claim" => Some(DateOfClaim),
                "Date of Notice of Referral for Assignment" => {
                    Some(DateOfNoticeOfReferralForAssignment)
                }
                "Date of Notice of Probable Ineligibility for Assignment" => {
                    Some(DateOfNoticeOfProbableIneligibilityForAssignment)
                }
                "Date of Foreclosure Notice" => Some(DateOfForeclosureNotice),
                "Expiration of Foreclosure Timeframe" => {
                    Some(ExpirationOfForeclosureTimeframe)
                }
                "Date Possessory Action Initiated" => Some(DatePossessoryActionInitiated),
                "Date of Possession" => Some(DateOfPossession),
                "Date of Last Installment Received" => {
                    Some(DateOfLastInstallmentReceived)
                }
                "Date of Acquisition of Title" => Some(DateOfAcquisitionOfTitle),
                "Expiration of Extension to Convey" => {
                    Some(ExpirationOfExtensionToConvey)
                }
                "Date of Assignment Approval" => Some(DateOfAssignmentApproval),
                "Date of Assignment Rejection" => Some(DateOfAssignmentRejection),
                "Curtailment Date from Advice of Payment" => {
                    Some(CurtailmentDateFromAdviceOfPayment)
                }
                "Expiration of Extension to Submit Fiscal Data" => {
                    Some(ExpirationOfExtensionToSubmitFiscalData)
                }
                "Date Documentation, or Paperwork, or Both Was Sent" => Some(Code537),
                "Makegood Commercial Date" => Some(MakegoodCommercialDate),
                "Policy Effective" => Some(PolicyEffective),
                "Policy Expiration" => Some(PolicyExpiration),
                "Employee Effective Date of Coverage" => {
                    Some(EmployeeEffectiveDateOfCoverage)
                }
                "Claim Administrator Notified of Employee Legal Representation" => {
                    Some(ClaimAdministratorNotifiedOfEmployeeLegalRepresentation)
                }
                "Last Premium Paid Date" => Some(LastPremiumPaidDate),
                "Employer Knowledge of the Injury" => Some(EmployerKnowledgeOfTheInjury),
                "Claim Administrator Knowledge of the Injury" => {
                    Some(ClaimAdministratorKnowledgeOfTheInjury)
                }
                "Date of Maximum Medical Improvement" => {
                    Some(DateOfMaximumMedicalImprovement)
                }
                "Date of Loan" => Some(DateOfLoan),
                "Date of Advance" => Some(DateOfAdvance),
                "Beginning Lay Date" => Some(BeginningLayDate),
                "Certificate Effective" => Some(CertificateEffective),
                "Benefit Application Date" => Some(BenefitApplicationDate),
                "Actual Return to Work" => Some(ActualReturnToWork),
                "Released Return to Work" => Some(ReleasedReturnToWork),
                "Ending Lay Date" => Some(EndingLayDate),
                "Employee Wages Ceased" => Some(EmployeeWagesCeased),
                "Last Salary Increase" => Some(LastSalaryIncrease),
                "Employee Laid Off" => Some(EmployeeLaidOff),
                "Injury or Illness" => Some(InjuryOrIllness),
                "Oldest Unpaid Installment" => Some(OldestUnpaidInstallment),
                "Preforeclosure Acceptance Date" => Some(PreforeclosureAcceptanceDate),
                "Preforeclosure Sale Closing Date" => Some(PreforeclosureSaleClosingDate),
                "Date of First Uncured Default" => Some(DateOfFirstUncuredDefault),
                "Date Default Was Cured" => Some(DateDefaultWasCured),
                "Date of First Mortgage Payment" => Some(DateOfFirstMortgagePayment),
                "Date of Property Inspection" => Some(DateOfPropertyInspection),
                "Date Total Amount of Delinquency Reported" => {
                    Some(DateTotalAmountOfDelinquencyReported)
                }
                "Date Outstanding Loan Balance Reported" => {
                    Some(DateOutstandingLoanBalanceReported)
                }
                "Date Foreclosure Sale Scheduled" => Some(DateForeclosureSaleScheduled),
                "Date Foreclosure Held" => Some(DateForeclosureHeld),
                "Date Redemption Period Ends" => Some(DateRedemptionPeriodEnds),
                "Date Voluntary Conveyance Accepted" => {
                    Some(DateVoluntaryConveyanceAccepted)
                }
                "Date Property Sold" => Some(DatePropertySold),
                "Date Claim Paid" => Some(DateClaimPaid),
                "Action Begin Date" => Some(ActionBeginDate),
                "Projected Action End Date" => Some(ProjectedActionEndDate),
                "Action End Date" => Some(ActionEndDate),
                "Original Maturity Date" => Some(OriginalMaturityDate),
                "Date Referred to Attorney for Foreclosure" => {
                    Some(DateReferredToAttorneyForForeclosure)
                }
                "Planned Release" => Some(PlannedRelease),
                "Actual Release" => Some(ActualRelease),
                "Contract Period" => Some(ContractPeriod),
                "Report Period" => Some(ReportPeriod),
                "Suspension" => Some(Suspension),
                "Reinstatement" => Some(Reinstatement),
                "Report" => Some(Report),
                "First Contact" => Some(FirstContact),
                "Projected Foreclosure Sale Date" => Some(ProjectedForeclosureSaleDate),
                "Date Assignment Filed for Record" => Some(DateAssignmentFiledForRecord),
                "Date of Appraisal" => Some(DateOfAppraisal),
                "Expiration Date of Extension to Assign" => {
                    Some(ExpirationDateOfExtensionToAssign)
                }
                "Date of Extension to Convey" => Some(DateOfExtensionToConvey),
                "Date Hazard Insurance Policy Refused" => {
                    Some(DateHazardInsurancePolicyRefused)
                }
                "High Fabrication Release Authorization" => {
                    Some(HighFabricationReleaseAuthorization)
                }
                "High Raw Material Authorization" => Some(HighRawMaterialAuthorization),
                "Material Change Notice" => Some(MaterialChangeNotice),
                "Latest Delivery Date at Rail Ramp" => Some(LatestDeliveryDateAtRailRamp),
                "Rejected" => Some(Rejected),
                "Repayment Schedule Sent" => Some(RepaymentScheduleSent),
                "As Of" => Some(AsOf),
                "First Submission" => Some(FirstSubmission),
                "Subsequent Submission" => Some(SubsequentSubmission),
                "Renewal" => Some(Renewal),
                "Withdrawn" => Some(Withdrawn),
                "Certification Period Start" => Some(CertificationPeriodStart),
                "Certification Revision" => Some(CertificationRevision),
                "Continuous Coverage Date(s)" => Some(Code608),
                "Prearranged Deal Match" => Some(PrearrangedDealMatch),
                "Contingency End" => Some(ContingencyEnd),
                "Oxygen Therapy Evaluation" => Some(OxygenTherapyEvaluation),
                "Shut In" => Some(ShutIn),
                "Allowable Effective" => Some(AllowableEffective),
                "First Sales" => Some(FirstSales),
                "Date Acquired" => Some(DateAcquired),
                "Interviewer Signed" => Some(InterviewerSigned),
                "Application Logged Date" => Some(ApplicationLoggedDate),
                "Review Date" => Some(ReviewDate),
                "Decision Date" => Some(DecisionDate),
                "Previously Resided" => Some(PreviouslyResided),
                "Reported" => Some(Reported),
                "Checked" => Some(Checked),
                "Settled" => Some(Settled),
                "Presently Residing" => Some(PresentlyResiding),
                "Employed in this Position" => Some(EmployedInThisPosition),
                "Verified" => Some(Verified),
                "Second Admission Date" => Some(SecondAdmissionDate),
                "Account Opened" => Some(AccountOpened),
                "Account Closed" => Some(AccountClosed),
                "Property Acquired" => Some(PropertyAcquired),
                "Property Built" => Some(PropertyBuilt),
                "Employed in this Profession" => Some(EmployedInThisProfession),
                "Next Review Date" => Some(NextReviewDate),
                "Initial Contact Date" => Some(InitialContactDate),
                "Date of Last Update" => Some(DateOfLastUpdate),
                "Second Discharge Date" => Some(SecondDischargeDate),
                "Date of Last Draw" => Some(DateOfLastDraw),
                "Complaint" => Some(Complaint),
                "Option" => Some(Option),
                "Solicitation" => Some(Solicitation),
                "Clause" => Some(Clause),
                "Meeting" => Some(Meeting),
                "Rental Period" => Some(RentalPeriod),
                "Next Pay Increase" => Some(NextPayIncrease),
                "Period Covered by Source Documents" => {
                    Some(PeriodCoveredBySourceDocuments)
                }
                "Document Due" => Some(DocumentDue),
                "Court Notice" => Some(CourtNotice),
                "Expected Funding Date" => Some(ExpectedFundingDate),
                "Assignment Recorded" => Some(AssignmentRecorded),
                "Case Reopened" => Some(CaseReopened),
                "Previous Court Event" => Some(PreviousCourtEvent),
                "Last Date to Object" => Some(LastDateToObject),
                "Court Event" => Some(CourtEvent),
                "Last Date to File a Claim" => Some(LastDateToFileAClaim),
                "Case Converted" => Some(CaseConverted),
                "Debt Incurred" => Some(DebtIncurred),
                "Judgment" => Some(Judgment),
                "Wages Start" => Some(WagesStart),
                "Wages End" => Some(WagesEnd),
                "Date Through Which Property Taxes Have Been Paid" => {
                    Some(DateThroughWhichPropertyTaxesHaveBeenPaid)
                }
                "Paid Through Date" => Some(PaidThroughDate),
                "Date Paid" => Some(DatePaid),
                "Anesthesia Administration" => Some(AnesthesiaAdministration),
                "Price Protection" => Some(PriceProtection),
                "Claim Incurred" => Some(ClaimIncurred),
                "Book Entry Delivery" => Some(BookEntryDelivery),
                "Rate Adjustment" => Some(RateAdjustment),
                "Next Installment Due Date" => Some(NextInstallmentDueDate),
                "Daylight Overdraft Time" => Some(DaylightOverdraftTime),
                "Presentment Date" => Some(PresentmentDate),
                "Negotiated Extension Date" => Some(NegotiatedExtensionDate),
                "Remittance" => Some(Remittance),
                "Security Rate Adjustment" => Some(SecurityRateAdjustment),
                "Filing Period" => Some(FilingPeriod),
                "Review Period End" => Some(ReviewPeriodEnd),
                "Requested Settlement" => Some(RequestedSettlement),
                "Last Screening" => Some(LastScreening),
                "Confinement" => Some(Confinement),
                "Arrested" => Some(Arrested),
                "Convicted" => Some(Convicted),
                "Interviewed" => Some(Interviewed),
                "Last Visit" => Some(LastVisit),
                "Recovery" => Some(Recovery),
                "Time in U.S." => Some(TimeInUS),
                "Future Period" => Some(FuturePeriod),
                "Previous Period" => Some(PreviousPeriod),
                "Interest Paid To" => Some(InterestPaidTo),
                "Date of Seizure" => Some(DateOfSeizure),
                "Setoff" => Some(Setoff),
                "Override Date for Settlement" => Some(OverrideDateForSettlement),
                "Settlement Date (From Interline Settlement System (ISS) only)" => {
                    Some(Code701)
                }
                "Sending Road Time Stamp" => Some(SendingRoadTimeStamp),
                "Retransmission Time Stamp" => Some(RetransmissionTimeStamp),
                "Delivery Appointment Date and Time" => {
                    Some(DeliveryAppointmentDateAndTime)
                }
                "Interest Paid Through" => Some(InterestPaidThrough),
                "Date Material Usage Suspended" => Some(DateMaterialUsageSuspended),
                "Last Payment Made" => Some(LastPaymentMade),
                "Past Due" => Some(PastDue),
                "Analysis Month Ending" => Some(AnalysisMonthEnding),
                "Date of Specification" => Some(DateOfSpecification),
                "Date of Standard" => Some(DateOfStandard),
                "Return to Work Part Time" => Some(ReturnToWorkPartTime),
                "Paid-through Date for Salary Continuation" => {
                    Some(PaidThroughDateForSalaryContinuation)
                }
                "Paid-through Date for Vacation Pay" => {
                    Some(PaidThroughDateForVacationPay)
                }
                "Paid-through Date for Accrued Sick Pay" => {
                    Some(PaidThroughDateForAccruedSickPay)
                }
                "Appraisal Ordered" => Some(AppraisalOrdered),
                "Date of Operation" => Some(DateOfOperation),
                "Best Time to Call" => Some(BestTimeToCall),
                "Verbal Report Needed" => Some(VerbalReportNeeded),
                "Estimated Escrow Closing" => Some(EstimatedEscrowClosing),
                "Permit Year" => Some(PermitYear),
                "Remodeling Completed" => Some(RemodelingCompleted),
                "Current Month Ending" => Some(CurrentMonthEnding),
                "Previous Month Ending" => Some(PreviousMonthEnding),
                "Cycle to Date" => Some(CycleToDate),
                "Year to Date" => Some(YearToDate),
                "On Hold" => Some(OnHold),
                "Off Hold" => Some(OffHold),
                "Facsimile Due By" => Some(FacsimileDueBy),
                "Reporting Cycle Date" => Some(ReportingCycleDate),
                "Last Paid Installment Date" => Some(LastPaidInstallmentDate),
                "Claims Made" => Some(ClaimsMade),
                "Date of Last Payment Received" => Some(DateOfLastPaymentReceived),
                "Curtailment Date" => Some(CurtailmentDate),
                "Pool Settlement" => Some(PoolSettlement),
                "Next Interest Change Date" => Some(NextInterestChangeDate),
                "Most Recent Hemoglobin or Hematocrit or Both" => {
                    Some(MostRecentHemoglobinOrHematocritOrBoth)
                }
                "Most Recent Serum Creatine" => Some(MostRecentSerumCreatine),
                "Closed" => Some(Closed),
                "Therapy" => Some(Therapy),
                "Implantation" => Some(Implantation),
                "Explantation" => Some(Explantation),
                "Date Became Aware" => Some(DateBecameAware),
                "First Marketed" => Some(FirstMarketed),
                "Last Marketed" => Some(LastMarketed),
                "New Due Date of First Payment to Principal and Interest" => {
                    Some(NewDueDateOfFirstPaymentToPrincipalAndInterest)
                }
                "New Maturity Date" => Some(NewMaturityDate),
                "Current" => Some(Current),
                "Expected Problem Resolution" => Some(ExpectedProblemResolution),
                "Alternate Problem Resolution" => Some(AlternateProblemResolution),
                "Fee Capitalization" => Some(FeeCapitalization),
                "Interest Capitalization" => Some(InterestCapitalization),
                "Next Payment Due" => Some(NextPaymentDue),
                "Conversion to Repayment" => Some(ConversionToRepayment),
                "End of Grace Period" => Some(EndOfGracePeriod),
                "School Refund" => Some(SchoolRefund),
                "Simple Interest Due" => Some(SimpleInterestDue),
                "Date Practice Ceased" => Some(DatePracticeCeased),
                "Printed" => Some(Printed),
                "Date Practice Established" => Some(DatePracticeEstablished),
                "Drop Action Date" => Some(DropActionDate),
                "Most Recent Renewal" => Some(MostRecentRenewal),
                "Original" => Some(Original),
                "Outside Auditor's Report" => Some(OutsideAuditorsReport),
                "Pre-certification Date" => Some(PreCertificationDate),
                "Back on Market" => Some(BackOnMarket),
                "Status" => Some(Status),
                "Benefit Adjustment Start" => Some(BenefitAdjustmentStart),
                "Off-Market" => Some(OffMarket),
                "Tour" => Some(Tour),
                "Benefit Adjustment End" => Some(BenefitAdjustmentEnd),
                "Listing Received" => Some(ListingReceived),
                "Benefit Adjustment Period" => Some(BenefitAdjustmentPeriod),
                "Anticipated Closing" => Some(AnticipatedClosing),
                "Last Publication" => Some(LastPublication),
                "Sold Book Publication" => Some(SoldBookPublication),
                "Occupancy" => Some(Occupancy),
                "Contingency" => Some(Contingency),
                "Percolation Test" => Some(PercolationTest),
                "Septic Approval" => Some(SepticApproval),
                "Title Transfer" => Some(TitleTransfer),
                "Open House" => Some(OpenHouse),
                "Benefit Credit Period" => Some(BenefitCreditPeriod),
                "Benefit Transfer Period" => Some(BenefitTransferPeriod),
                "Homestead" => Some(Homestead),
                "Sanction" => Some(Sanction),
                "Tail Coverage Begin" => Some(TailCoverageBegin),
                "Tail Coverage End" => Some(TailCoverageEnd),
                "Training Begin" => Some(TrainingBegin),
                "Training End" => Some(TrainingEnd),
                "Verification Received" => Some(VerificationReceived),
                "Verification Sent" => Some(VerificationSent),
                "State Residency Date" => Some(StateResidencyDate),
                "Effective Date of the Routing File" => {
                    Some(EffectiveDateOfTheRoutingFile)
                }
                "Test Data Analysis" => Some(TestDataAnalysis),
                "Midpoint of Performance" => Some(MidpointOfPerformance),
                "Acquisition Date" => Some(AcquisitionDate),
                "Date of Action" => Some(DateOfAction),
                "Paid in Full" => Some(PaidInFull),
                "Refinance" => Some(Refinance),
                "Voluntary Termination" => Some(VoluntaryTermination),
                "Customer Order" => Some(CustomerOrder),
                "Stored" => Some(Stored),
                "Selected" => Some(Selected),
                "Posted" => Some(Posted),
                "Document Received" => Some(DocumentReceived),
                "Rebuilt" => Some(Rebuilt),
                "Marriage" => Some(Marriage),
                "Customs Entry Date" => Some(CustomsEntryDate),
                "Payment Due Date" => Some(PaymentDueDate),
                "Maturity Date" => Some(MaturityDate),
                "Trade Date" => Some(TradeDate),
                "Gallons Per Minute (GPM) Test Performed" => Some(Code817),
                "British Thermal Unit (BTU) Test Performed" => Some(Code818),
                "Last Accounts Filed at Public Registration Agency" => {
                    Some(LastAccountsFiledAtPublicRegistrationAgency)
                }
                "Real Estate Tax Year" => Some(RealEstateTaxYear),
                "Final Reconciliation Value Estimate as of" => {
                    Some(FinalReconciliationValueEstimateAsOf)
                }
                "Map" => Some(Map),
                "Opinion" => Some(Opinion),
                "Version" => Some(Version),
                "Original Due Date" => Some(OriginalDueDate),
                "Incumbency Period" => Some(IncumbencyPeriod),
                "Audience Deficiency Period" => Some(AudienceDeficiencyPeriod),
                "Aired Date" => Some(AiredDate),
                "Schedule" => Some(Schedule),
                "Paid Through Date for Minimum Payment" => {
                    Some(PaidThroughDateForMinimumPayment)
                }
                "Paid Through Date for Total Payment" => {
                    Some(PaidThroughDateForTotalPayment)
                }
                "Election" => Some(Election),
                "Engineering Data List" => Some(EngineeringDataList),
                "Last Production" => Some(LastProduction),
                "Not Before" => Some(NotBefore),
                "Not After" => Some(NotAfter),
                "Initial Claim" => Some(InitialClaim),
                "Benefits Paid" => Some(BenefitsPaid),
                "Wages Earned" => Some(WagesEarned),
                "Adjusted Start" => Some(AdjustedStart),
                "Adjusted End" => Some(AdjustedEnd),
                "Revised Adjusted Start" => Some(RevisedAdjustedStart),
                "Revised Adjusted End" => Some(RevisedAdjustedEnd),
                "Field Test" => Some(FieldTest),
                "Mortgage Note Date" => Some(MortgageNoteDate),
                "Alternative Due Date" => Some(AlternativeDueDate),
                "First Payment Change" => Some(FirstPaymentChange),
                "First Rate Adjustment" => Some(FirstRateAdjustment),
                "Alternate Base Period" => Some(AlternateBasePeriod),
                "Prior Notice" => Some(PriorNotice),
                "Appointment Effective" => Some(AppointmentEffective),
                "Appointment Expiration" => Some(AppointmentExpiration),
                "Company Termination" => Some(CompanyTermination),
                "Continuing Education Requirement" => {
                    Some(ContinuingEducationRequirement)
                }
                "Distributor Effective" => Some(DistributorEffective),
                "Distributor Termination" => Some(DistributorTermination),
                "Examination" => Some(Examination),
                "Incorporation Dissolution" => Some(IncorporationDissolution),
                "Last Follow-up" => Some(LastFollowUp),
                "License Effective" => Some(LicenseEffective),
                "License Expiration" => Some(LicenseExpiration),
                "License Renewal" => Some(LicenseRenewal),
                "License Requested" => Some(LicenseRequested),
                "Mailed" => Some(Mailed),
                "Paperwork Mailed" => Some(PaperworkMailed),
                "Previous Employment" => Some(PreviousEmployment),
                "Previous Employment End" => Some(PreviousEmploymentEnd),
                "Previous Employment Start" => Some(PreviousEmploymentStart),
                "Previous Residence" => Some(PreviousResidence),
                "Previous Residence End" => Some(PreviousResidenceEnd),
                "Previous Residence Start" => Some(PreviousResidenceStart),
                "Request" => Some(Request),
                "Resident License Effective" => Some(ResidentLicenseEffective),
                "Resident License Expiration" => Some(ResidentLicenseExpiration),
                "State Termination" => Some(StateTermination),
                "Texas Line Termination" => Some(TexasLineTermination),
                "Acceleration" => Some(Acceleration),
                "Adjusted Contestability" => Some(AdjustedContestability),
                "Application Entry" => Some(ApplicationEntry),
                "Approval/Offer" => Some(ApprovalOffer),
                "Automatic Premium Loan" => Some(AutomaticPremiumLoan),
                "Collection" => Some(Collection),
                "Confinement End" => Some(ConfinementEnd),
                "Confinement Start" => Some(ConfinementStart),
                "Contestability" => Some(Contestability),
                "Flat Extra End" => Some(FlatExtraEnd),
                "Last Activity" => Some(LastActivity),
                "Last Change" => Some(LastChange),
                "Last Episode" => Some(LastEpisode),
                "Last Meal" => Some(LastMeal),
                "Loan" => Some(Loan),
                "Application Status" => Some(ApplicationStatus),
                "Maturity" => Some(Maturity),
                "Medical Information Signature" => Some(MedicalInformationSignature),
                "Medical Information System" => Some(MedicalInformationSystem),
                "Note" => Some(Note),
                "Offer Expiration" => Some(OfferExpiration),
                "Original Receipt" => Some(OriginalReceipt),
                "Placement" => Some(Placement),
                "Placement Period Expiration" => Some(PlacementPeriodExpiration),
                "Processing" => Some(Processing),
                "Recapture" => Some(Recapture),
                "Re-entry" => Some(ReEntry),
                "Reissue" => Some(Reissue),
                "Requalification" => Some(Requalification),
                "Reinsurance Effective" => Some(ReinsuranceEffective),
                "Reservation of Facility" => Some(ReservationOfFacility),
                "Settlement Status" => Some(SettlementStatus),
                "Table Rating End" => Some(TableRatingEnd),
                "Termination of Facility" => Some(TerminationOfFacility),
                "Treatment" => Some(Treatment),
                "Department of Labor Wage Determination Date" => {
                    Some(DepartmentOfLaborWageDeterminationDate)
                }
                "Order" => Some(Order),
                "Resolved" => Some(Resolved),
                "Execution Date" => Some(ExecutionDate),
                "Capitation Period Start" => Some(CapitationPeriodStart),
                "Capitation Period End" => Some(CapitationPeriodEnd),
                "Last Date for a Government Agency to File a Claim" => {
                    Some(LastDateForAGovernmentAgencyToFileAClaim)
                }
                "Adjustment Period" => Some(AdjustmentPeriod),
                "Activity" => Some(Activity),
                "Mail By" => Some(MailBy),
                "Preparation" => Some(Preparation),
                "Payment Initiated" => Some(PaymentInitiated),
                "Payment Effective" => Some(PaymentEffective),
                "Application" => Some(Application),
                "Reclassification" => Some(Reclassification),
                "Reclassification (Exit Date)" => Some(Code952),
                "Post-Reclassification" => Some(PostReclassification),
                "Post-Reclassification (First Report Card)" => Some(Code954),
                "Post-Reclassification (First Semi-annual)" => Some(Code955),
                "Post-Reclassification (Second Semi-annual)" => Some(Code956),
                "Post-Reclassification (End of Second Year)" => Some(Code957),
                "Adjusted Death Benefit" => Some(AdjustedDeathBenefit),
                "Anniversary" => Some(Anniversary),
                "Annuitization" => Some(Annuitization),
                "Annuity Commencement Date" => Some(AnnuityCommencementDate),
                "Bill" => Some(Bill),
                "Calendar Anniversary" => Some(CalendarAnniversary),
                "Contract Mailed" => Some(ContractMailed),
                "Early Withdrawal" => Some(EarlyWithdrawal),
                "Fiscal Anniversary" => Some(FiscalAnniversary),
                "Income" => Some(Income),
                "Initial Premium" => Some(InitialPremium),
                "Initial Premium Effective" => Some(InitialPremiumEffective),
                "Last Premium Effective" => Some(LastPremiumEffective),
                "Minimum Required Distribution" => Some(MinimumRequiredDistribution),
                "Next Anniversary" => Some(NextAnniversary),
                "Notice" => Some(Notice),
                "Notification of Death" => Some(NotificationOfDeath),
                "Partial Annuitization" => Some(PartialAnnuitization),
                "Plan Anniversary" => Some(PlanAnniversary),
                "Policy Surrender" => Some(PolicySurrender),
                "Prior Contract Anniversary" => Some(PriorContractAnniversary),
                "Prior Contract Issue" => Some(PriorContractIssue),
                "Signature Received" => Some(SignatureReceived),
                "Tax" => Some(Tax),
                "Benefit Period" => Some(BenefitPeriod),
                "Month to Date" => Some(MonthToDate),
                "Semiannual Ending" => Some(SemiannualEnding),
                "Surrender" => Some(Surrender),
                "Plan of Treatment Period" => Some(PlanOfTreatmentPeriod),
                "Prior Hospitalization Date(s) Related to Current Service(s)" => {
                    Some(Code989)
                }
                "Original Name Change" => Some(OriginalNameChange),
                "Date Requested" => Some(DateRequested),
                "Request for Quotation" => Some(RequestForQuotation),
                "Quote" => Some(Quote),
                "Recorded Date" => Some(RecordedDate),
                "Required Delivery" => Some(RequiredDelivery),
                "Quote to be Received By" => Some(QuoteToBeReceivedBy),
                "Continuation of Pay Start Date" => Some(ContinuationOfPayStartDate),
                "Document Date" => Some(DocumentDate),
                "Estimated Point of Arrival" => Some(EstimatedPointOfArrival),
                "Estimated Point of Discharge" => Some(EstimatedPointOfDischarge),
                "Cancel After, Ex Country" => Some(CodeAA3),
                "Cancel After, Ex Factory" => Some(CodeAA4),
                "Do Not Ship Before, Ex Country" => Some(CodeAA5),
                "Do Not Ship Before, Ex Factory" => Some(CodeAA6),
                "Final Scheduled Payment" => Some(FinalScheduledPayment),
                "Actual Discharge" => Some(ActualDischarge),
                "Address Period" => Some(AddressPeriod),
                "Arrival in Country" => Some(ArrivalInCountry),
                "Citation" => Some(Citation),
                "Suspension Effective" => Some(SuspensionEffective),
                "Crime" => Some(Crime),
                "Discharge - Planned" => Some(DischargePlanned),
                "Draft" => Some(Draft),
                "Due Date" => Some(DueDate),
                "Event" => Some(Event),
                "First Involvement" => Some(FirstInvolvement),
                "Guarantee Period" => Some(GuaranteePeriod),
                "Income Increase Period" => Some(IncomeIncreasePeriod),
                "Installment Date" => Some(InstallmentDate),
                "Last Civilian Flight" => Some(LastCivilianFlight),
                "Last Flight" => Some(LastFlight),
                "Last Insurance Medical" => Some(LastInsuranceMedical),
                "Last Military Flight" => Some(LastMilitaryFlight),
                "Last Physical" => Some(LastPhysical),
                "License" => Some(License),
                "Medical Certificate" => Some(MedicalCertificate),
                "Medication" => Some(Medication),
                "Net Worth Date" => Some(NetWorthDate),
                "Next Activity" => Some(NextActivity),
                "Ownership Change" => Some(OwnershipChange),
                "Ownership Period" => Some(OwnershipPeriod),
                "Rate Date" => Some(RateDate),
                "Requested Contract" => Some(RequestedContract),
                "Requested Offer" => Some(RequestedOffer),
                "Sales Period" => Some(SalesPeriod),
                "Tax Year" => Some(TaxYear),
                "Time Period" => Some(TimePeriod),
                "Travel" => Some(Travel),
                "Treatment End" => Some(TreatmentEnd),
                "Treatment Start" => Some(TreatmentStart),
                "Trust" => Some(Trust),
                "Worst Time to Call" => Some(WorstTimeToCall),
                "Registration" => Some(Registration),
                "Revoked" => Some(Revoked),
                "Estimated Date of Birth" => Some(EstimatedDateOfBirth),
                "Last Annual Report" => Some(LastAnnualReport),
                "Legal Action Started" => Some(LegalActionStarted),
                "Lien" => Some(Lien),
                "Payment Period" => Some(PaymentPeriod),
                "Profit Period" => Some(ProfitPeriod),
                "Registered" => Some(Registered),
                "Consolidated" => Some(Consolidated),
                "Board of Directors Not Authorized As Of" => {
                    Some(BoardOfDirectorsNotAuthorizedAsOf)
                }
                "Board of Directors Incomplete As Of" => {
                    Some(BoardOfDirectorsIncompleteAsOf)
                }
                "Manager Not Registered As Of" => Some(ManagerNotRegisteredAsOf),
                "Citizenship Change" => Some(CitizenshipChange),
                "Participation" => Some(Participation),
                "Capitalization" => Some(Capitalization),
                "Registration of Board of Directors" => {
                    Some(RegistrationOfBoardOfDirectors)
                }
                "Ceased Operations" => Some(CeasedOperations),
                "Satisfied" => Some(Satisfied),
                "Terms Met" => Some(TermsMet),
                "Asset Documentation Expiration" => Some(AssetDocumentationExpiration),
                "Credit Documentation Expiration" => Some(CreditDocumentationExpiration),
                "Income Documentation Expiration" => Some(IncomeDocumentationExpiration),
                "Product Held Until" => Some(ProductHeldUntil),
                "Immigration Date" => Some(ImmigrationDate),
                "Estimated Immigration Date" => Some(EstimatedImmigrationDate),
                "Current Disability Period Start" => Some(CurrentDisabilityPeriodStart),
                "Current Disability Period End" => Some(CurrentDisabilityPeriodEnd),
                "Current Disability Period Last Day Worked" => {
                    Some(CurrentDisabilityPeriodLastDayWorked)
                }
                "Benefit Type Gross Weekly Amount Effective" => {
                    Some(BenefitTypeGrossWeeklyAmountEffective)
                }
                "Benefit Type Net Weekly Amount Effective" => {
                    Some(BenefitTypeNetWeeklyAmountEffective)
                }
                "Benefit Type Period Start" => Some(BenefitTypePeriodStart),
                "Benefit Type Period End" => Some(BenefitTypePeriodEnd),
                "Benefit Debit Start" => Some(BenefitDebitStart),
                "Acknowledgment" => Some(Acknowledgment),
                "Benefit Debit End" => Some(BenefitDebitEnd),
                "Benefit Credit Start" => Some(BenefitCreditStart),
                "Benefit Credit End" => Some(BenefitCreditEnd),
                "Benefit Transfer Start" => Some(BenefitTransferStart),
                "Benefit Transfer End" => Some(BenefitTransferEnd),
                "Wage Effective" => Some(WageEffective),
                "Full Denial Effective" => Some(FullDenialEffective),
                "Full Denial Rescission" => Some(FullDenialRescission),
                "Payment Issue" => Some(PaymentIssue),
                "Payment Period Start" => Some(PaymentPeriodStart),
                "Payment Period End" => Some(PaymentPeriodEnd),
                "Employer Reported Injury To Claim Administrator" => {
                    Some(EmployerReportedInjuryToClaimAdministrator)
                }
                "Survey Year" => Some(SurveyYear),
                "Controvert Date" => Some(ControvertDate),
                "Billed Through" => Some(BilledThrough),
                "Business Control Change" => Some(BusinessControlChange),
                "Court Registration" => Some(CourtRegistration),
                "Annual Report Due" => Some(AnnualReportDue),
                "Claim Notification Received" => Some(ClaimNotificationReceived),
                "Conversion Privilege End" => Some(ConversionPrivilegeEnd),
                "Dividend Applied" => Some(DividendApplied),
                "In-force" => Some(InForce),
                "Paid-up" => Some(PaidUp),
                "Premium Change" => Some(PremiumChange),
                "Policy Effective on or before" => Some(PolicyEffectiveOnOrBefore),
                "Asset and Liability Schedule" => Some(AssetAndLiabilitySchedule),
                "Annual Report Mailed" => Some(AnnualReportMailed),
                "Policy Effective on or after" => Some(PolicyEffectiveOnOrAfter),
                "Annual Report Filed" => Some(AnnualReportFiled),
                "Audit Period After Split Date" => Some(AuditPeriodAfterSplitDate),
                "Audit Period Prior to Split Date" => Some(AuditPeriodPriorToSplitDate),
                "Exposure Source Period" => Some(ExposureSourcePeriod),
                "Subcontractor Period of Hire" => Some(SubcontractorPeriodOfHire),
                "Divorce" => Some(Divorce),
                "Power of Attorney" => Some(PowerOfAttorney),
                "Uniform Gifts to Minors Account Established" => {
                    Some(UniformGiftsToMinorsAccountEstablished)
                }
                "Medicare Part A Eligibility Begin Date" => {
                    Some(MedicarePartAEligibilityBeginDate)
                }
                "Medicare Part A Eligibility End Date" => {
                    Some(MedicarePartAEligibilityEndDate)
                }
                "Medicare Part A Coverage Effective Date" => {
                    Some(MedicarePartACoverageEffectiveDate)
                }
                "Medicare Part A Termination Date" => Some(MedicarePartATerminationDate),
                "Medicare Part B Eligibility Begin Date" => {
                    Some(MedicarePartBEligibilityBeginDate)
                }
                "Medicare Part B Eligibility End Date" => {
                    Some(MedicarePartBEligibilityEndDate)
                }
                "Medicare Part B Coverage Effective Date" => {
                    Some(MedicarePartBCoverageEffectiveDate)
                }
                "Medicare Part B Termination Date" => Some(MedicarePartBTerminationDate),
                "Loading Period" => Some(LoadingPeriod),
                "Date on which Assets Judged Insufficient to Pay Creditors" => {
                    Some(DateOnWhichAssetsJudgedInsufficientToPayCreditors)
                }
                "Employees Temporarily Laid Off Begin Period" => {
                    Some(EmployeesTemporarilyLaidOffBeginPeriod)
                }
                "Employees Temporarily Laid Off End Period" => {
                    Some(EmployeesTemporarilyLaidOffEndPeriod)
                }
                "First Published" => Some(FirstPublished),
                "Forecast Period Start" => Some(ForecastPeriodStart),
                "Forecast Period End" => Some(ForecastPeriodEnd),
                "Investigation Start" => Some(InvestigationStart),
                "Investigation End" => Some(InvestigationEnd),
                "Last Published" => Some(LastPublished),
                "Latest Balance Sheet" => Some(LatestBalanceSheet),
                "Share Price" => Some(SharePrice),
                "Stop Distribution" => Some(StopDistribution),
                "Maximum Credit Date" => Some(MaximumCreditDate),
                "Founding Date" => Some(FoundingDate),
                "Repayment Plan Start Date" => Some(RepaymentPlanStartDate),
                "Medicare Part D Eligibility Begin Date" => {
                    Some(MedicarePartDEligibilityBeginDate)
                }
                "Medicare Part D Eligibility End Date" => {
                    Some(MedicarePartDEligibilityEndDate)
                }
                "Medicare Part D Coverage Effective Date" => {
                    Some(MedicarePartDCoverageEffectiveDate)
                }
                "Medicare Part D Termination Date" => Some(MedicarePartDTerminationDate),
                "Annual Report Delinquency" => Some(AnnualReportDelinquency),
                "Withheld Date" => Some(WithheldDate),
                "Compliance Audit" => Some(ComplianceAudit),
                "Contractor Safety Performance Evaluation" => {
                    Some(ContractorSafetyPerformanceEvaluation)
                }
                "Contractor Safety Procedures Review" => {
                    Some(ContractorSafetyProceduresReview)
                }
                "Date of Equipment Inspection" => Some(DateOfEquipmentInspection),
                "Date of Safety Inspection" => Some(DateOfSafetyInspection),
                "Employees Participation Plan Review" => {
                    Some(EmployeesParticipationPlanReview)
                }
                "Expected Completion of Changes Resulting from\nCompliance Audit" => {
                    Some(CodeBAG)
                }
                "Expected Completion of Changes Resulting from\nProcess Hazard Analysis" => {
                    Some(CodeBAH)
                }
                "Expected Completion of Changes Resulting from Hazard Review" => {
                    Some(ExpectedCompletionOfChangesResultingFromHazardReview)
                }
                "Hazard Review Completion" => Some(HazardReviewCompletion),
                "Hot Work Permit Procedures Review" => {
                    Some(HotWorkPermitProceduresReview)
                }
                "Investigation" => Some(Investigation),
                "Maintenance Procedures Review" => Some(MaintenanceProceduresReview),
                "Management of Change Procedures Review" => {
                    Some(ManagementOfChangeProceduresReview)
                }
                "Operating Procedures Review" => Some(OperatingProceduresReview),
                "Safety Information Review" => Some(SafetyInformationReview),
                "Training" => Some(Training),
                "Training Program Review" => Some(TrainingProgramReview),
                "Billback End Date" => Some(BillbackEndDate),
                "Program Performance End Date" => Some(ProgramPerformanceEndDate),
                "Program Performance Start Date" => Some(ProgramPerformanceStartDate),
                "Beginning of Grace Period" => Some(BeginningOfGracePeriod),
                "Billing Activities" => Some(BillingActivities),
                "Beginning of Interest Paid After Claim" => {
                    Some(BeginningOfInterestPaidAfterClaim)
                }
                "Billback Start Date" => Some(BillbackStartDate),
                "Changed Accounting Date" => Some(ChangedAccountingDate),
                "Customs Cargo Release" => Some(CustomsCargoRelease),
                "Contract Definitization Date" => Some(ContractDefinitizationDate),
                "Campaign End Date" => Some(CampaignEndDate),
                "Campaign Start Date" => Some(CampaignStartDate),
                "Maintenance Comment" => Some(MaintenanceComment),
                "Formation" => Some(Formation),
                "Continuance" => Some(Continuance),
                "Merger" => Some(Merger),
                "Year Due" => Some(YearDue),
                "Next Annual Meeting" => Some(NextAnnualMeeting),
                "End of Last Fiscal Year" => Some(EndOfLastFiscalYear),
                "Year Beginning" => Some(YearBeginning),
                "Started Doing Business" => Some(StartedDoingBusiness),
                "Sworn and Subscribed" => Some(SwornAndSubscribed),
                "Calendar Year" => Some(CalendarYear),
                "Asset" => Some(Asset),
                "Inactivity" => Some(Inactivity),
                "High Capital Year" => Some(HighCapitalYear),
                "Closing Date of First Balance Sheet" => {
                    Some(ClosingDateOfFirstBalanceSheet)
                }
                "Closed Until" => Some(ClosedUntil),
                "Compliance" => Some(Compliance),
                "Converted into Holding Company" => Some(ConvertedIntoHoldingCompany),
                "Care of Supplies in Storage Inspection Date" => {
                    Some(CareOfSuppliesInStorageInspectionDate)
                }
                "Consumer Product Availability Date" => {
                    Some(ConsumerProductAvailabilityDate)
                }
                "Consumer Product Information Publication Date" => {
                    Some(ConsumerProductInformationPublicationDate)
                }
                "Consumer Product Variant End Effective Date" => {
                    Some(ConsumerProductVariantEndEffectiveDate)
                }
                "Consumer Product Variant Discontinued Date" => {
                    Some(ConsumerProductVariantDiscontinuedDate)
                }
                "Consumer Product Variant Cancelled Date" => {
                    Some(ConsumerProductVariantCancelledDate)
                }
                "Consumer Product Variant Start Effective Date" => {
                    Some(ConsumerProductVariantStartEffectiveDate)
                }
                "Claim Revised" => Some(ClaimRevised),
                "Current List" => Some(CurrentList),
                "Community Visibility" => Some(CommunityVisibility),
                "Account Frozen" => Some(AccountFrozen),
                "Declaration" => Some(Declaration),
                "Deed Not Available" => Some(DeedNotAvailable),
                "Delete" => Some(Delete),
                "Detrimental Information Received" => {
                    Some(DetrimentalInformationReceived)
                }
                "Deferral" => Some(Deferral),
                "Departure From Specification" => Some(DepartureFromSpecification),
                "Deed In Lieu (DIL) Approved" => Some(CodeDIL),
                "Delayed Interest Paid Through" => Some(DelayedInterestPaidThrough),
                "Disposition" => Some(Disposition),
                "Date of Last Contact" => Some(DateOfLastContact),
                "Date of Abandonment" => Some(DateOfAbandonment),
                "Date of Delinquency" => Some(DateOfDelinquency),
                "Delivery Order Issued" => Some(DeliveryOrderIssued),
                "Repossession" => Some(Repossession),
                "Disposal" => Some(Disposal),
                "Deed and Title Received" => Some(DeedAndTitleReceived),
                "Technical Data Supply By" => Some(TechnicalDataSupplyBy),
                "Deed and Title Requested" => Some(DeedAndTitleRequested),
                "Tenure Decision" => Some(TenureDecision),
                "Most Recent Position Change" => Some(MostRecentPositionChange),
                "Fee Payment" => Some(FeePayment),
                "Start Date for Continuous Employment" => {
                    Some(StartDateForContinuousEmployment)
                }
                "Start Date for Current Position" => Some(StartDateForCurrentPosition),
                "Start Date for Original Position" => Some(StartDateForOriginalPosition),
                "Fiscal Year" => Some(FiscalYear),
                "End Availability Date" => Some(EndAvailabilityDate),
                "Estimated Construction Date" => Some(EstimatedConstructionDate),
                "Estimated Completion - First Prior Month" => {
                    Some(EstimatedCompletionFirstPriorMonth)
                }
                "Estimated Completion - Second Prior Month" => {
                    Some(EstimatedCompletionSecondPriorMonth)
                }
                "Estimated Completion - Third Prior Month" => {
                    Some(EstimatedCompletionThirdPriorMonth)
                }
                "Affirmed" => Some(Affirmed),
                "Auction" => Some(Auction),
                "Authorized" => Some(Authorized),
                "Contribution" => Some(Contribution),
                "Executed" => Some(Executed),
                "Forgiven" => Some(Forgiven),
                "Presented" => Some(Presented),
                "Legislative Session" => Some(LegislativeSession),
                "Organized" => Some(Organized),
                "Pledged" => Some(Pledged),
                "Primary Election" => Some(PrimaryElection),
                "Qualified" => Some(Qualified),
                "Refunded" => Some(Refunded),
                "Rescinded" => Some(Rescinded),
                "Restructured From" => Some(RestructuredFrom),
                "Vote" => Some(Vote),
                "Employer Knowledge of the Disability" => {
                    Some(EmployerKnowledgeOfTheDisability)
                }
                "End Date Maximum Buying Quantity" => Some(EndDateMaximumBuyingQuantity),
                "End Date Minimum Buying Quantity" => Some(EndDateMinimumBuyingQuantity),
                "Estimate Preparation" => Some(EstimatePreparation),
                "Estimate Comment" => Some(EstimateComment),
                "Effective Start Date" => Some(EffectiveStartDate),
                "Estimated Start - First Prior Month" => {
                    Some(EstimatedStartFirstPriorMonth)
                }
                "Estimated Start - Second Prior Month" => {
                    Some(EstimatedStartSecondPriorMonth)
                }
                "Estimated Start - Third Prior Month" => {
                    Some(EstimatedStartThirdPriorMonth)
                }
                "Earliest Filing Period" => Some(EarliestFilingPeriod),
                "Exposure" => Some(Exposure),
                "Export" => Some(Export),
                "Facility Latest Billing Action" => Some(FacilityLatestBillingAction),
                "Facility Earliest Billing Action" => Some(FacilityEarliestBillingAction),
                "Financial Information" => Some(Financial),
                "First Order" => Some(FirstOrder),
                "Final Interest Accrual" => Some(FinalInterestAccrual),
                "Funding Period - End" => Some(FundingPeriodEnd),
                "Funding Period - Start" => Some(FundingPeriodStart),
                "First Available Ship Date" => Some(FirstAvailableShipDate),
                "Free Service Call End Date" => Some(FreeServiceCallEndDate),
                "Free Service Call Start Date" => Some(FreeServiceCallStartDate),
                "Graduated" => Some(Graduated),
                "Home Health Date of Earliest Billable Action" => {
                    Some(HomeHealthDateOfEarliestBillableAction)
                }
                "Home Health Episode" => Some(HomeHealthEpisode),
                "Home Health Date of Latest Billable Action" => {
                    Some(HomeHealthDateOfLatestBillableAction)
                }
                "Host Train Date" => Some(HostTrainDate),
                "Converted to Electronic Date" => Some(ConvertedToElectronicDate),
                "Insolvency Discharge Granted" => Some(InsolvencyDischargeGranted),
                "Initial Federal Housing Authority Claim Payment" => {
                    Some(InitialFederalHousingAuthorityClaimPayment)
                }
                "Incorporation" => Some(Incorporation),
                "Image Last Update Date" => Some(ImageLastUpdateDate),
                "Imbalance Period" => Some(ImbalancePeriod),
                "Import" => Some(Import),
                "Incident" => Some(Incident),
                "Inactive Until" => Some(InactiveUntil),
                "Interest on Presale Start" => Some(InterestOnPresaleStart),
                "Initial Veterans Administration Claim Payment" => {
                    Some(InitialVeteransAdministrationClaimPayment)
                }
                "Key Event Fiscal Year" => Some(KeyEventFiscalYear),
                "Key Event Calendar Year" => Some(KeyEventCalendarYear),
                "Last Annual Meeting" => Some(LastAnnualMeeting),
                "Last Check for Balance Sheet Update" => {
                    Some(LastCheckForBalanceSheetUpdate)
                }
                "Last Capital Change" => Some(LastCapitalChange),
                "Letter of Agreement" => Some(LetterOfAgreement),
                "Letter of Liability" => Some(LetterOfLiability),
                "Liquidation" => Some(Liquidation),
                "Low Period" => Some(LowPeriod),
                "Equipment Log Entry" => Some(EquipmentLogEntry),
                "List Price Change" => Some(ListPriceChange),
                "Legal Structure Change" => Some(LegalStructureChange),
                "Last Submission Date" => Some(LastSubmissionDate),
                "Latest Filing Period" => Some(LatestFilingPeriod),
                "Meter Reading" => Some(MeterReading),
                "Latest Material Safety Data Sheet Date" => {
                    Some(LatestMaterialSafetyDataSheetDate)
                }
                "Present Name" => Some(PresentName),
                "Negotiated Finish" => Some(NegotiatedFinish),
                "Notice of Delinquency (NOD) Received" => Some(CodeNOD),
                "Not Registered" => Some(NotRegistered),
                "Negotiated Start" => Some(NegotiatedStart),
                "Organic Certification Date" => Some(OrganicCertificationDate),
                "Original List" => Some(OriginalList),
                "Present Control" => Some(PresentControl),
                "Primary Coverage Claim Paid" => Some(PrimaryCoverageClaimPaid),
                "Primary Coverage Claim Submission" => {
                    Some(PrimaryCoverageClaimSubmission)
                }
                "Price Changes Allowed From Date" => Some(PriceChangesAllowedFromDate),
                "Price Changes Allowed To Date" => Some(PriceChangesAllowedToDate),
                "Product Discontinued but Still Available" => {
                    Some(ProductDiscontinuedButStillAvailable)
                }
                "Partial Denial Effective" => Some(PartialDenialEffective),
                "Partial Denial Rescission" => Some(PartialDenialRescission),
                "Correct Program Start Date" => Some(CorrectProgramStartDate),
                "Correct Program End Date" => Some(CorrectProgramEndDate),
                "Correct Contract Start Date" => Some(CorrectContractStartDate),
                "Privilege Details Verification" => Some(PrivilegeDetailsVerification),
                "Correct Contract End Date" => Some(CorrectContractEndDate),
                "Program End Date" => Some(ProgramEndDate),
                "Product Image Start Date" => Some(ProductImageStartDate),
                "Present Legal Structure" => Some(PresentLegalStructure),
                "Packaging Material Effective Date" => {
                    Some(PackagingMaterialEffectiveDate)
                }
                "Pool Policy Claim Submission" => Some(PoolPolicyClaimSubmission),
                "Post Paid Date" => Some(PostPaidDate),
                "Peak Period" => Some(PeakPeriod),
                "Previously Reported Date of Birth" => {
                    Some(PreviouslyReportedDateOfBirth)
                }
                "Presented to Receivers" => Some(PresentedToReceivers),
                "Property Sale Approved" => Some(PropertySaleApproved),
                "Property Sale Closed" => Some(PropertySaleClosed),
                "Program Start Date" => Some(ProgramStartDate),
                "Property Sale Confirmation" => Some(PropertySaleConfirmation),
                "Paid To Date" => Some(PaidToDate),
                "Planned Obsolescence Item" => Some(PlannedObsolescenceItem),
                "Pick-up Date" => Some(PickUpDate),
                "Receiver Appointed" => Some(ReceiverAppointed),
                "Remanufacture Date" => Some(RemanufactureDate),
                "Resigned" => Some(Resigned),
                "Requested Finish" => Some(RequestedFinish),
                "Recovery Finish" => Some(RecoveryFinish),
                "Referred From" => Some(ReferredFrom),
                "Rent Survey" => Some(RentSurvey),
                "Received in the Mail" => Some(ReceivedInTheMail),
                "Revocation" => Some(Revocation),
                "Requested Start" => Some(RequestedStart),
                "Recovery Start" => Some(RecoveryStart),
                "Referred To" => Some(ReferredTo),
                "Social Security Claims Verification" => {
                    Some(SocialSecurityClaimsVerification)
                }
                "Sole Directorship Date" => Some(SoleDirectorshipDate),
                "Start Date Maximum Buying Quantity" => {
                    Some(StartDateMaximumBuyingQuantity)
                }
                "Subsequent Federal Housing Authority Claim Payment" => {
                    Some(SubsequentFederalHousingAuthorityClaimPayment)
                }
                "Start Date Minimum Buying Quantity" => {
                    Some(StartDateMinimumBuyingQuantity)
                }
                "Initial Support Date" => Some(InitialSupportDate),
                "Suggested Retail Price Effective Date" => {
                    Some(SuggestedRetailPriceEffectiveDate)
                }
                "Transition" => Some(Transition),
                "Subsequent Veterans Administration Claim Payment" => {
                    Some(SubsequentVeteransAdministrationClaimPayment)
                }
                "Trade Style Registered" => Some(TradeStyleRegistered),
                "Trial Started" => Some(TrialStarted),
                "Trial Set" => Some(TrialSet),
                "Tenant Train Date" => Some(TenantTrainDate),
                "Value Added Tax (VAT) Claims Verification" => Some(CodeVAT),
                "Valid Until" => Some(ValidUntil),
                "Sample Collected" => Some(SampleCollected),
                "Status Change" => Some(StatusChange),
                "Construction Start" => Some(ConstructionStart),
                "Recompletion" => Some(Recompletion),
                "Last Logged" => Some(LastLogged),
                "Well Log Run" => Some(WellLogRun),
                "Surface Casing Authority Approval" => {
                    Some(SurfaceCasingAuthorityApproval)
                }
                "Reached Total Depth" => Some(ReachedTotalDepth),
                "Spacing Order Unit Assigned" => Some(SpacingOrderUnitAssigned),
                "Rig Arrival" => Some(RigArrival),
                "Location Exception Order Number Assigned" => {
                    Some(LocationExceptionOrderNumberAssigned)
                }
                "Sidetracked Wellbore" => Some(SidetrackedWellbore),
                "Time Employee Began Work" => Some(TimeEmployeeBeganWork),
                "Waybill" => Some(Waybill),
                "Order Day" => Some(OrderDay),
                "Delivery Day" => Some(DeliveryDay),
                "Order Cut-Off Time" => Some(OrderCutOffTime),
                "Active Item" => Some(ActiveItem),
                "Mature Item" => Some(MatureItem),
                "Introductory Item" => Some(IntroductoryItem),
                "Obsolete Item" => Some(ObsoleteItem),
                "Best Before Date" => Some(BestBeforeDate),
                "Harvest Date" => Some(HarvestDate),
                "Programmed Fiscal Year" => Some(ProgrammedFiscalYear),
                "Programmed Calendar Year" => Some(ProgrammedCalendarYear),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for DateTimeQualifier {
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
    type Value = DateTimeQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Date/Time Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DateTimeQualifier::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Date/Time Qualifier: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DateTimeQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Date/Time Qualifier: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for DateTimeQualifier {
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