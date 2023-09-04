use crate::{
    models::{
        BeneficiaryDetails, CarpetCleaningTypeCostData, CarpetCleaningTypeCostDetails,
        CarpetCleaningTypeCostResponseData, CarpetTypeColourData, CarpetTypeColourResponseData,
        CarpetTypeSizeData, CarpetTypeSizeResponseData, EmployeesData, HistorySalesData, Info,
        PersonDetails, ProcessingStatus, ResponseData, ResponseData1, SalesBatchData,
        SalesCommissionData, SearchHistorySalesData, SearchSalesCommissionData,
        VehicleCleaningTypeCostData, VehicleCleaningTypeCostDetails,
        VehicleCleaningTypeCostResponseData, VehicleMakeData, VehicleMakeResponseData,
        VehicleModelData, VehicleModelResponseData,
    },
    persistence::{
        create_sales_batch_data, create_sales_commission_data, create_sales_data,
        get_employees_registered_data, get_history_sales_batch_data,
        get_history_search_sales_batch_data, get_location, get_sales_batch_data,
        get_sales_commission_data, get_sales_data, get_search_entry_sales_commission_data,
        validate_client_api,
    },
};
use actix_web::{get, post, web, HttpRequest, Responder};
use mysql::*;

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    format!("")
}

/// deserialize `Info` from request's body
#[post("/person")]
pub async fn get_person(info: web::Json<Info>) -> impl Responder {
    let user_name = &info.username;
    let my_staff_name = &info.posted_by.staff_name;
    let my_job_level = &info.posted_by.job_level;
    let location_name = get_location();
    let my_beneficiary = BeneficiaryDetails {
        full_name: String::from("Moses Weta"),
        relationship: String::from("Son"),
    };
    let my_beneficiary1 = BeneficiaryDetails {
        full_name: String::from("Benta Shiraku"),
        relationship: String::from("Daughter"),
    };
    let my_beneficiary2 = BeneficiaryDetails {
        full_name: String::from("Paul Owino"),
        relationship: String::from("Son"),
    };

    let mut x = Vec::new();
    let my_person = PersonDetails {
        username: user_name.to_string(),
        location: location_name,
        beneficiary: my_beneficiary,
        staff_name: my_staff_name.to_string(),
        job_level: my_job_level.to_string(),
    };
    let my_person1 = PersonDetails {
        username: String::from("walter"),
        location: String::from("westlands"),
        beneficiary: my_beneficiary1,
        staff_name: my_staff_name.to_string(),
        job_level: my_job_level.to_string(),
    };

    let my_person2 = PersonDetails {
        username: user_name.to_string(),
        location: String::from("ngong"),
        beneficiary: my_beneficiary2,
        staff_name: my_staff_name.to_string(),
        job_level: my_job_level.to_string(),
    };

    x.push(my_person);
    x.push(my_person1);
    x.push(my_person2);

    let my_response_data = ResponseData1 {
        status_code: ProcessingStatus::Zero as u32,
        status_description: String::from("Successful"),
        person_data: x,
    };
    web::Json(my_response_data)
}

/// deserialize `VehicleMakeData` from request's body
#[post("/getvehiclemakedata")]
pub async fn get_vehicle_make_data(
    vehicle_make_data: web::Json<VehicleMakeData>,
    req: HttpRequest,
) -> impl Responder {
    let k = String::from(""); //Default value for string variables.
    let api_function = String::from("get_vehicle_make_data");

    let client_api_response = validate_client_api(req, api_function);
    let status_code = client_api_response.status_code;
    let status_description = client_api_response.status_description;

    let mobile_no = &vehicle_make_data.mobile_no.as_ref().unwrap_or(&k);
    let vehicle_make = String::from("ALFA ROMEO|ANY|ASHOK|AUDI|BACKHOE|BAJAJ|BEDFORD|BEIBEN|BEIFANG|BHACHU|BMW|BOBCAT|BOMAG|BULLDOZER|BUS|CADILLAC|CAM|CANTER|CASE|CAT|CHEVROLET|CHRYSLER|CITROEN|CMC|CRANE|DAEWOO|DAF|DAIHATSU|DODGE|DOLL|DOZER|DUMPER|EICHER|EXCAVATOR|FAW|FERRARI|FIAT|FORD|FOTON|GEELEY|GRADER|GREATWALL|HAMM|HANS KENYA|HINO|HITACHI|HONDA|HOWO|HUMMER|HYUNDAI|ISUZU|IVECO|JAC|JAGUAR|JCB|JEEP|JMC|JOHN-DEERE|KEHAR|KIA|KLUGER|KOMATSU|LANCER|LANDROVER|LEEBOY|LEXUS|LEYLAND|LEYLANDDAF|LIEBHERR|LOADER|LORRY|M/CYCLE|MACK|MAHINDRA|MAN|MARUTI|MASSEY|MAZDA|MERCEDES|MINI|MITSUBISHI|MIXER|MORRIS|NEWHOLLAND|NIS_DIE|NISSAN|OCEAN|OPEL|PACER|PEUGEOT|PORSCHE|PRIMEMOVER|PUCH|RANDON|RENAULT|ROLLER|ROLLS|ROVER|SAAB|SAILOR|SCANIA|SDLG|SHACMAN|SHOVEL|SINO|SKODA|SONALIKA|SSANG YONG|SUBARU|SUZUKI|TADANO|TANKER|TATA|TEREX|TIGER|TIGGO|TIPPER|TOYOTA|TRACTOR|TRAILER|TRUCK|TUKTUK|TVS|UD|VAUXHALL|VOLKSWAGEN|VOLVO|WUZHENG|XINKAI|YAMAHA|YARI|");
    let mut k = Vec::new();
    let interior_cleaning_name = String::from("interior");
    let exterior_cleaning_name = String::from("exterior");
    let engine_cleaning_name = String::from("engine");
    let under_carriage_cleaning_name = String::from("undercarriage");

    let interior_cleaning_cost = 200;
    let exterior_cleaning_cost = 300;
    let engine_cleaning_cost = 150;
    let under_carriage_cleaning_cost = 210;

    let interior_item = VehicleCleaningTypeCostDetails {
        cleaning_type_name: interior_cleaning_name,
        amount: interior_cleaning_cost,
    };
    k.push(interior_item);
    let exterior_item = VehicleCleaningTypeCostDetails {
        cleaning_type_name: exterior_cleaning_name,
        amount: exterior_cleaning_cost,
    };
    k.push(exterior_item);
    let engine_item = VehicleCleaningTypeCostDetails {
        cleaning_type_name: engine_cleaning_name,
        amount: engine_cleaning_cost,
    };
    k.push(engine_item);
    let under_carriage_item = VehicleCleaningTypeCostDetails {
        cleaning_type_name: under_carriage_cleaning_name,
        amount: under_carriage_cleaning_cost,
    };
    k.push(under_carriage_item);

    let response_data = VehicleMakeResponseData {
        message_data: vehicle_make.to_string(),
        status_code: ProcessingStatus::Zero as u32,
        status_description: String::from("Successful"),
        cost_data: k,
    };
    web::Json(response_data)
}

/// deserialize `VehicleModelData` from request's body
#[post("/getvehiclemodeldata")]
pub async fn get_vehicle_model_data(
    vehicle_model_data: web::Json<VehicleModelData>,
    req: HttpRequest,
) -> impl Responder {
    let k = String::from(""); //Default value for string variables.
    let api_function = String::from("get_vehicle_model_data");

    let client_api_response = validate_client_api(req, api_function);
    let status_code = client_api_response.status_code;
    let status_description = client_api_response.status_description;

    let mobile_no = &vehicle_model_data.mobile_no.as_ref().unwrap_or(&k);

    let vehicle_make = &vehicle_model_data.vehicle_make.as_ref().unwrap_or(&k);

    let mut vehicle_model = String::from("");

    let a_1 = String::from("audi");
    let a_2 = String::from("bajaj");
    let a_3 = String::from("bmw");

    vehicle_model = {
        if vehicle_make.to_lowercase().eq(&a_1) {
            String::from("AUDI|AUDI-A2|AUDI-A3|AUDI-A3 SE AUTO|AUDI-A4 AVANT S|AUDI-A4 AVANT W|AUDI-A4 FSI S-L|AUDI-A4 TURBO Q|AUDI-A6 SE AUTO|AUDI-A6 STDI|AUDI-A6 TURBO|AUDI-A6 TURBO S|AUDI-A8 SE TDI|AUDI-AUDI 500|AUDI-Q5|AUDI-Q5 3.2L AU|AUDI-Q7|")
        } else if vehicle_make.to_lowercase().eq(&a_2) {
            String::from("BAJAJ BM 150X|BAJAJ QUTE|BAJAJ TUKTUK|")
        } else if vehicle_make.to_lowercase().eq(&a_3) {
            String::from("BMW|BMW 316I|BMW 5|BMW 650GS|BMW ABA-VA20|BMW IGT|BMW-116D F20SH|BMW-116D N47 U|BMW-116I|BMW-116I E81 N|BMW-116I F20SH|BMW-118D N47 U|BMW-118D- N47|BMW-118I E88 C|BMW-118I N13 F|BMW-118I N46 U|BMW-120D CP|BMW-120D N47 U|BMW-120I E82 C|BMW-120I N46 U|BMW-125I N52 U|BMW-130I MANUA|BMW-130I N52 U|BMW-135I N54 U|BMW-135I N55 U|BMW-316I E90 L|BMW-318I I N46|BMW-318I N46 P|BMW-320D|BMW-320D N47 K|BMW-320ED|BMW-320I|BMW-320I E93 C|BMW-320I N46 P|BMW-325I AUTO|BMW-325I MANUA|BMW-325I N52 D|BMW-325I N52 K|BMW-325I N52 P|BMW-325I N53 C|BMW-330 CI CON|BMW-330D|BMW-330D N57 K|BMW-330I  N52|BMW-330I AUTO|BMW-330I MANUA|BMW-335I|BMW-335I N54 K|BMW-335I N54 P|BMW-335I N55 D|BMW-520D AUTO|BMW-520I|BMW-523I|BMW-525D|BMW-525I|BMW-525I AUTO|BMW-528I AUTO|BMW-530D N57 F|BMW-530I MANUA|BMW-535D N57 S|BMW-535I F07 G|BMW-535I N55 F|BMW-550I N63 F|BMW-550I N63 S|BMW-630I  E63|BMW-630I  E64|BMW-650I  N62|BMW-730D|BMW-730I F02 L|BMW-730IAUTO|BMW-730LD N57|BMW-735I|BMW-740I N54 K|BMW-740LI N54|BMW-745IA|BMW-750I N63 K|BMW-750I XDRIV|BMW-750LI N63|BMW-750LI XDRI|BMW-760I N74 K|BMW-760LI N74|BMW-BMW MINI C|BMW-BMW MOTOR CYCLE|BMW-BMW Z3 ROA|BMW-F650 GS 218|BMW-F800 GS 219|BMW-F800 R 217|BMW-F800 ST 234|BMW-G650 GS SERTA|BMW-G650GS 188|BMW-K1300 R 518|BMW-K1300 S 308|BMW-K1600 GT 601|BMW-K1600 GTL 602|BMW-M3 S65 DX9|BMW-M3 S65 KG9|BMW-M3 S65 PM9|BMW-M6 S85 EH9|BMW-M6 S85 EK9|BMW-R1200 GS 450|BMW-R1200 R 400|BMW-R1200 R GS ADV|BMW-R1200 RT 430|BMW-R1200RT 430|BMW-R900 RT 330|BMW-S1000 RR 524|BMW-X1|BMW-X1 SDRIVEN|BMW-X1 XDRIVEN|BMW-X3|BMW-X3 XDRIVE2|BMW-X3 XDRIVE3|BMW-X3 XDRIVEN|BMW-X3 XRIVE30|BMW-X5|BMW-X5 3.0D|BMW-X5 351|BMW-X5 M S63 G|BMW-X5 XDRIVE5|BMW-X5 XDRIVEN|BMW-X6|BMW-X6 M N63 G|BMW-X6 XDRIVE5|BMW-Z4 E89 ROA|")
        } else {
            String::from("")
        }
    };

    let response_data = VehicleModelResponseData {
        message_data: vehicle_model.to_string(),
        status_code: ProcessingStatus::Zero as u32,
        status_description: String::from("Successful"),
    };
    web::Json(response_data)
}

/// deserialize `CarpetTypeSizeData` from request's body
#[post("/getcarpettypesizedata")]
pub async fn get_carpet_type_size_data(
    carpet_type_size_data: web::Json<CarpetTypeSizeData>,
    req: HttpRequest,
) -> impl Responder {
    let k = String::from(""); //Default value for string variables.
    let api_function = String::from("get_carpet_type_size_data");

    let client_api_response = validate_client_api(req, api_function);
    let status_code = client_api_response.status_code;
    let status_description = client_api_response.status_description;

    //println!("channel_type - {:?}", channel_type);
    let mobile_no = &carpet_type_size_data.mobile_no.as_ref().unwrap_or(&k);
    let carpet_type_size = String::from("CARPET SIZE|5 by 8|6 by 9|7 by 10|8 by 11|");
    let mut k = Vec::new();
    let a_cleaning_size_name = String::from("5by8");
    let b_cleaning_size_name = String::from("6by9");
    let c_cleaning_size_name = String::from("7by10");
    let d_size_cleaning_name = String::from("8by11");

    let a_cleaning_size_cost = 600;
    let b_cleaning_size_cost = 700;
    let c_cleaning_size_cost = 800;
    let d_cleaning_size_cost = 900;

    let a_item = CarpetCleaningTypeCostDetails {
        cleaning_size_name: a_cleaning_size_name,
        amount: a_cleaning_size_cost,
    };
    k.push(a_item);
    let b_item = CarpetCleaningTypeCostDetails {
        cleaning_size_name: b_cleaning_size_name,
        amount: b_cleaning_size_cost,
    };
    k.push(b_item);
    let c_item = CarpetCleaningTypeCostDetails {
        cleaning_size_name: c_cleaning_size_name,
        amount: c_cleaning_size_cost,
    };
    k.push(c_item);
    let d_item = CarpetCleaningTypeCostDetails {
        cleaning_size_name: d_size_cleaning_name,
        amount: d_cleaning_size_cost,
    };
    k.push(d_item);

    let response_data = CarpetTypeSizeResponseData {
        message_data: carpet_type_size.to_string(),
        status_code: ProcessingStatus::Zero as u32,
        status_description: String::from("Successful"),
        cost_data: k,
    };
    web::Json(response_data)
}

/// deserialize `CarpetTypeColourData` from request's body
#[post("/getcarpettypecolourdata")]
pub async fn get_carpet_type_colour_data(
    carpet_type_colour_data: web::Json<CarpetTypeColourData>,
    req: HttpRequest,
) -> impl Responder {
    let k = String::from(""); //Default value for string variables.
    let api_function = String::from("get_carpet_type_colour_data");

    let client_api_response = validate_client_api(req, api_function);
    let status_code = client_api_response.status_code;
    let status_description = client_api_response.status_description;

    let mobile_no = &carpet_type_colour_data.mobile_no.as_ref().unwrap_or(&k);
    let carpet_type_colour =
        String::from("CARPET COLOUR|WHITE|BLACK|RED|BLUE|YELLOW|ORANGE|PURPLE|GREEN|MIXTURE");

    let response_data = CarpetTypeColourResponseData {
        message_data: carpet_type_colour.to_string(),
        status_code: ProcessingStatus::Zero as u32,
        status_description: String::from("Successful"),
    };
    web::Json(response_data)
}

/// deserialize `VehicleCleaningTypeCostData` from request's body
#[post("/getvehiclecleaningtypecostdata")]
pub async fn get_vehicle_cleaning_type_cost_data(
    vehicle_cleaning_type_cost_data: web::Json<VehicleCleaningTypeCostData>,
    req: HttpRequest,
) -> impl Responder {
    let k = String::from(""); //Default value for string variables.
    let api_function = String::from("get_vehicle_cleaning_type_cost_data");

    let client_api_response = validate_client_api(req, api_function);
    let status_code = client_api_response.status_code;
    let status_description = client_api_response.status_description;

    let mobile_no = &vehicle_cleaning_type_cost_data
        .mobile_no
        .as_ref()
        .unwrap_or(&k);
    let mut k = Vec::new();
    let interior_cleaning_name = String::from("interior");
    let exterior_cleaning_name = String::from("exterior");
    let engine_cleaning_name = String::from("engine");
    let under_carriage_cleaning_name = String::from("undercarriage");

    let interior_cleaning_cost = 200;
    let exterior_cleaning_cost = 300;
    let engine_cleaning_cost = 150;
    let under_carriage_cleaning_cost = 210;

    let interior_item = VehicleCleaningTypeCostDetails {
        cleaning_type_name: interior_cleaning_name,
        amount: interior_cleaning_cost,
    };
    k.push(interior_item);
    let exterior_item = VehicleCleaningTypeCostDetails {
        cleaning_type_name: exterior_cleaning_name,
        amount: exterior_cleaning_cost,
    };
    k.push(exterior_item);
    let engine_item = VehicleCleaningTypeCostDetails {
        cleaning_type_name: engine_cleaning_name,
        amount: engine_cleaning_cost,
    };
    k.push(engine_item);
    let under_carriage_item = VehicleCleaningTypeCostDetails {
        cleaning_type_name: under_carriage_cleaning_name,
        amount: under_carriage_cleaning_cost,
    };
    k.push(under_carriage_item);

    let response_data = VehicleCleaningTypeCostResponseData {
        status_code: ProcessingStatus::Zero as u32,
        status_description: String::from("Successful"),
        cost_data: k,
    };
    web::Json(response_data)
}

/// deserialize `CarpetCleaningTypeCostData` from request's body
#[post("/getcarpetcleaningtypecostdata")]
pub async fn get_carpet_cleaning_type_cost_data(
    carpet_cleaning_type_cost_data: web::Json<CarpetCleaningTypeCostData>,
    req: HttpRequest,
) -> impl Responder {
    let k = String::from(""); //Default value for string variables.
    let api_function = String::from("get_carpet_cleaning_type_cost_data");

    let client_api_response = validate_client_api(req, api_function);
    let status_code = client_api_response.status_code;
    let status_description = client_api_response.status_description;

    let mobile_no = &carpet_cleaning_type_cost_data
        .mobile_no
        .as_ref()
        .unwrap_or(&k);
    let mut k = Vec::new();
    let a_cleaning_size_name = String::from("5by8");
    let b_cleaning_size_name = String::from("6by9");
    let c_cleaning_size_name = String::from("7by10");
    let d_size_cleaning_name = String::from("8by11");

    let a_cleaning_size_cost = 600;
    let b_cleaning_size_cost = 700;
    let c_cleaning_size_cost = 800;
    let d_cleaning_size_cost = 900;

    let a_item = CarpetCleaningTypeCostDetails {
        cleaning_size_name: a_cleaning_size_name,
        amount: a_cleaning_size_cost,
    };
    k.push(a_item);
    let b_item = CarpetCleaningTypeCostDetails {
        cleaning_size_name: b_cleaning_size_name,
        amount: b_cleaning_size_cost,
    };
    k.push(b_item);
    let c_item = CarpetCleaningTypeCostDetails {
        cleaning_size_name: c_cleaning_size_name,
        amount: c_cleaning_size_cost,
    };
    k.push(c_item);
    let d_item = CarpetCleaningTypeCostDetails {
        cleaning_size_name: d_size_cleaning_name,
        amount: d_cleaning_size_cost,
    };
    k.push(d_item);

    let response_data = CarpetCleaningTypeCostResponseData {
        status_code: ProcessingStatus::Zero as u32,
        status_description: String::from("Successful"),
        cost_data: k,
    };
    web::Json(response_data)
}

/// deserialize `SalesBatchData` from request's body
#[post("/addsalesdata")]
pub async fn add_sales_data(
    sales_batch_data: web::Json<SalesBatchData>,
    req: HttpRequest,
    data: web::Data<Pool>,
) -> impl Responder {
    let k = String::from(""); //Default value for string variables.
    let api_function = String::from("add_sales_data");

    let client_api_response = validate_client_api(req, api_function);
    let status_code = client_api_response.status_code;
    let status_description = client_api_response.status_description;

    let batch_no = &sales_batch_data.batch_no.as_ref().unwrap_or(&k);
    let sales_batch_data = &sales_batch_data.sales_data;

    let sales_batch_data_table = get_sales_batch_data(sales_batch_data);

    let batch_no: i32 = create_sales_batch_data(&data, sales_batch_data_table);

    let sales_data_table = get_sales_data(sales_batch_data, batch_no);
    let successful: bool = create_sales_data(&data, sales_data_table);

    let successful_1: bool = create_sales_commission_data(data, batch_no);

    let response_data = ResponseData {
        status_code: ProcessingStatus::Zero as u32,
        status_description: String::from("Successful"),
    };
    web::Json(response_data)
}

/// deserialize `HistorySalesData` from request's body
#[post("/getallsalesdata")]
pub async fn get_all_sales_data(
    history_sales_data: web::Json<HistorySalesData>,
    req: HttpRequest,
    data: web::Data<Pool>,
) -> impl Responder {
    let k = String::from(""); //Default value for string variables.
    let api_function = String::from("get_all_sales_data");

    let client_api_response = validate_client_api(req, api_function);
    let status_code = client_api_response.status_code;
    let status_description = client_api_response.status_description;

    let response_data = get_history_sales_batch_data(&data);

    web::Json(response_data)
}

/// deserialize `SearchHistorySalesData` from request's body
#[post("/getsearchsalesdata")]
pub async fn get_search_sales_data(
    search_history_sales_data: web::Json<SearchHistorySalesData>,
    req: HttpRequest,
    data: web::Data<Pool>,
) -> impl Responder {
    let k = String::from(""); //Default value for string variables.
    let j: bool = false;
    let api_function = String::from("get_search_sales_data");

    let client_api_response = validate_client_api(req, api_function);
    let status_code = client_api_response.status_code;
    let status_description = client_api_response.status_description;

    let search_data = &search_history_sales_data.search_data.as_ref().unwrap_or(&k);
    let search_by_key = &search_history_sales_data.search_by;

    let is_mobile_no = &search_by_key.mobile_no.as_ref().unwrap_or(&j);
    let is_customer_name = &search_by_key.customer_name.as_ref().unwrap_or(&j);
    let is_vehicle_regno = &search_by_key.vehicle_regno.as_ref().unwrap_or(&j);

    let search_data = search_data.replace(" ", "");
    let search_data = search_data.to_lowercase();

    let response_data = get_history_search_sales_batch_data(
        &search_data,
        is_mobile_no,
        is_customer_name,
        is_vehicle_regno,
        &data,
    );

    web::Json(response_data)
}

/// deserialize `EmployeesData` from request's body
#[post("/getallemployeesdata")]
pub async fn get_all_employees_data(
    employees_data: web::Json<EmployeesData>,
    req: HttpRequest,
    data: web::Data<Pool>,
) -> impl Responder {
    let api_function = String::from("get_all_employees_data");

    let client_api_response = validate_client_api(req, api_function);
    let status_code = client_api_response.status_code;
    let status_description = client_api_response.status_description;

    let response_data = get_employees_registered_data(&data);
    web::Json(response_data)
}

/// deserialize `SalesCommissionData` from request's body
#[post("/getallsalescommissiondata")]
pub async fn get_all_sales_commission_data(
    sales_commission_data: web::Json<SalesCommissionData>,
    req: HttpRequest,
    data: web::Data<Pool>,
) -> impl Responder {
    let api_function = String::from("get_all_sales_commission_data");

    let client_api_response = validate_client_api(req, api_function);
    let status_code = client_api_response.status_code;
    let status_description = client_api_response.status_description;

    let response_data = get_sales_commission_data(&data);

    web::Json(response_data)
}

/// deserialize `SearchSalesCommissionData` from request's body
#[post("/getsearchsalescommissiondata")]
pub async fn get_search_sales_commission_data(
    search_sales_commission_data: web::Json<SearchSalesCommissionData>,
    req: HttpRequest,
    data: web::Data<Pool>,
) -> impl Responder {
    let k = String::from(""); //Default value for string variables.
    let j: bool = false;
    let api_function = String::from("get_search_sales_commission_data");

    let client_api_response = validate_client_api(req, api_function);
    let status_code = client_api_response.status_code;
    let status_description = client_api_response.status_description;

    let search_data = &search_sales_commission_data
        .search_data
        .as_ref()
        .unwrap_or(&k);
    let search_by_key = &search_sales_commission_data.search_by;

    let is_employee_id = &search_by_key.employee_id.as_ref().unwrap_or(&j);
    let is_employee_full_names = &search_by_key.employee_full_names.as_ref().unwrap_or(&j);

    let response_data = get_search_entry_sales_commission_data(
        search_data,
        is_employee_id,
        is_employee_full_names,
        &data,
    );

    web::Json(response_data)
}
