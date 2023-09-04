use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Info {
    pub username: String,
    pub posted_by: PostedBy,
}

#[derive(Deserialize)]
pub struct PostedBy {
    pub staff_name: String,
    pub job_level: String,
}

#[derive(Deserialize)]
pub struct SalesBatchData {
    pub batch_no: Option<String>,
    pub sales_data: Vec<SalesData>,
}

#[derive(Deserialize)]
pub struct SalesData {
    pub customer_sales_data: CustomerSalesData,
    pub vehicle_sales_data: Option<VehicleSalesData>,
    pub carpet_sales_data: Option<CarpetSalesData>,
}

#[derive(Deserialize)]
pub struct CustomerSalesData {
    pub cust_name: String,
    pub mobile_no: String,
    pub sales_amount: String,
    pub paid_amount: String,
    pub payment_mode: String,
}

#[derive(Deserialize)]
pub struct VehicleSalesData {
    pub vehicle_make: String,
    pub vehicle_model: String,
    pub vehicle_regno: String,
    pub sales_amount: String,
    pub payment_mode: String,
    pub interior_cleaning: bool,
    pub exterior_cleaning: bool,
    pub engine_cleaning: bool,
    pub undercarriage_cleaning: bool,
    pub employee_id: i32,
    pub employee_full_names: String,
}

#[derive(Deserialize)]
pub struct CarpetSalesData {
    pub carpet_size: String,
    pub carpet_colour: String,
    pub sales_amount: String,
    pub payment_mode: String,
    pub employee_id: i32,
    pub employee_full_names: String,
}

#[derive(Deserialize)]
pub struct VehicleMakeData {
    pub mobile_no: Option<String>,
    pub device_registration_token: Option<String>,
}

#[derive(Deserialize)]
pub struct VehicleModelData {
    pub mobile_no: Option<String>,
    pub vehicle_make: Option<String>,
}

#[derive(Deserialize)]
pub struct CarpetTypeSizeData {
    pub mobile_no: Option<String>,
    pub device_registration_token: Option<String>,
}

#[derive(Deserialize)]
pub struct VehicleCleaningTypeCostData {
    pub mobile_no: Option<String>,
    pub device_registration_token: Option<String>,
}

#[derive(Deserialize)]
pub struct CarpetCleaningTypeCostData {
    pub mobile_no: Option<String>,
    pub device_registration_token: Option<String>,
}

#[derive(Deserialize)]
pub struct CarpetTypeColourData {
    pub mobile_no: Option<String>,
    pub device_registration_token: Option<String>,
}

#[derive(Deserialize)]
pub struct HistorySalesData {
    pub mobile_no: Option<String>,
    pub device_registration_token: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchSalesItems {
    pub mobile_no: Option<bool>,
    pub customer_name: Option<bool>,
    pub vehicle_regno: Option<bool>,
}

#[derive(Deserialize)]
pub struct SearchHistorySalesData {
    pub search_data: Option<String>,
    pub search_by: SearchSalesItems,
}

#[derive(Deserialize)]
pub struct EmployeesData {
    pub mobile_no: Option<String>,
    pub device_registration_token: Option<String>,
}

#[derive(Deserialize)]
pub struct SalesCommissionData {
    pub mobile_no: Option<String>,
    pub device_registration_token: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchSalesCommissionItems {
    pub employee_id: Option<bool>,
    pub employee_full_names: Option<bool>,
}

#[derive(Deserialize)]
pub struct SearchSalesCommissionData {
    pub search_data: Option<String>,
    pub search_by: SearchSalesCommissionItems,
}

pub enum ProcessingStatus {
    Zero,
    One,
    Two,
}

#[derive(Serialize)]
pub struct Measurement {
    pub temperature: f32,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub status_code: u32,
    pub status_description: String,
}

#[derive(Serialize)]
pub struct ResponseData1 {
    pub status_code: u32,
    pub status_description: String,
    pub person_data: Vec<PersonDetails>,
}

#[derive(Serialize)]
pub struct PersonDetails {
    pub username: String,
    pub location: String,
    pub beneficiary: BeneficiaryDetails,
    pub staff_name: String,
    pub job_level: String,
}

#[derive(Serialize)]
//#[derive(Debug)]
pub struct BeneficiaryDetails {
    pub full_name: String,
    pub relationship: String,
}

#[derive(Serialize)]
pub struct VehicleMakeResponseData {
    pub message_data: String,
    pub status_code: u32,
    pub status_description: String,
    pub cost_data: Vec<VehicleCleaningTypeCostDetails>,
}

#[derive(Serialize)]
pub struct VehicleModelResponseData {
    pub message_data: String,
    pub status_code: u32,
    pub status_description: String,
}

#[derive(Serialize)]
pub struct CarpetTypeSizeResponseData {
    pub message_data: String,
    pub status_code: u32,
    pub status_description: String,
    pub cost_data: Vec<CarpetCleaningTypeCostDetails>,
}

#[derive(Serialize)]
pub struct VehicleCleaningTypeCostResponseData {
    pub status_code: u32,
    pub status_description: String,
    pub cost_data: Vec<VehicleCleaningTypeCostDetails>,
}

#[derive(Serialize)]
pub struct VehicleCleaningTypeCostDetails {
    pub cleaning_type_name: String,
    pub amount: u32,
}

#[derive(Serialize)]
pub struct CarpetCleaningTypeCostResponseData {
    pub status_code: u32,
    pub status_description: String,
    pub cost_data: Vec<CarpetCleaningTypeCostDetails>,
}

#[derive(Serialize)]
pub struct CarpetCleaningTypeCostDetails {
    pub cleaning_size_name: String,
    pub amount: u32,
}

#[derive(Serialize)]
pub struct CarpetTypeColourResponseData {
    pub message_data: String,
    pub status_code: u32,
    pub status_description: String,
}

#[derive(Serialize)]
pub struct HistoryVehicleSalesData {
    pub vehicle_make: String,
    pub vehicle_model: String,
    pub vehicle_regno: String,
    pub sales_amount: u32,
    pub payment_mode: String,
    pub interior_cleaning: bool,
    pub exterior_cleaning: bool,
    pub engine_cleaning: bool,
    pub undercarriage_cleaning: bool,
    pub transaction_date: String,
}

#[derive(Serialize)]
pub struct HistoryCarpetSalesData {
    pub carpet_size: String,
    pub carpet_colour: String,
    pub sales_amount: u32,
    pub payment_mode: String,
    pub transaction_date: String,
}

#[derive(Serialize)]
pub struct HistoryCustomerSalesData {
    pub cust_name: String,
    pub mobile_no: String,
}

#[derive(Serialize)]
pub struct HistorySalesResponseData {
    pub customer_sales_data: HistoryCustomerSalesData,
    pub carpet_sales_data: Vec<HistoryCarpetSalesData>,
    pub vehicle_sales_data: Vec<HistoryVehicleSalesData>,
}

#[derive(Serialize)]
pub struct HistorySalesBatchData {
    pub batch_no: String,
    pub sales_data: HistorySalesResponseData,
}

#[derive(Serialize)]
pub struct HistorySalesBatchResponseData {
    pub status_code: u32,
    pub status_description: String,
    pub sales_batch_data: Vec<HistorySalesBatchData>,
}

#[derive(Serialize)]
pub struct EmployeeRegisteredDetails {
    pub full_names: String,
    pub id: u32,
}

#[derive(Serialize)]
pub struct EmployeesRegisteredResponseData {
    pub status_code: u32,
    pub status_description: String,
    pub employees_data: Vec<EmployeeRegisteredDetails>,
}

#[derive(Serialize)]
pub struct SalesCommissionDetails {
    pub batch_no: u32,
    pub cleaning_service: String,
    pub cleaning_service_type: String,
    pub cleaning_amount: i32,
    pub commission_percentage: i32,
    pub commission_amount: i32,
    pub employee_full_names: String,
    pub transaction_date: String,
}

#[derive(Serialize)]
pub struct SalesCommissionResponseData {
    pub status_code: u32,
    pub status_description: String,
    pub sales_commission_data: Vec<SalesCommissionDetails>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SalesBatchDataTable {
    pub batch_no: Option<i32>,
    pub cust_name: String,
    pub mobile_no: String,
    pub cleaning_service: String,
    pub sales_amount: i32,
    pub paid_amount: i32,
    pub payment_mode: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SalesDataTable {
    pub batch_no: i32,
    pub cleaning_service: String,
    pub carpet_size: String,
    pub carpet_colour: String,
    pub vehicle_make: String,
    pub vehicle_model: String,
    pub vehicle_regno: String,
    pub interior_cleaning: bool,
    pub exterior_cleaning: bool,
    pub engine_cleaning: bool,
    pub undercarriage_cleaning: bool,
    pub sales_amount: i32,
    pub employee_id: i32,
    pub employee_full_names: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ClientApiResponseDetails {
    pub status_code: u32,
    pub status_description: String,
}
