extern crate base64;

use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use base64::{decode};//encode
use std::str;
use mysql::*;
use mysql::prelude::*;

//, Result

#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

#[derive(Deserialize)]
struct Info {
    username: String,
	posted_by: PostedBy,
}

#[derive(Deserialize)]
struct PostedBy {
    staff_name: String,
	job_level: String,
}

#[derive(Deserialize)]
struct SalesBatchData {
    batch_no: Option<String>,
    sales_data: Vec<SalesData>,
}

#[derive(Deserialize)]
struct SalesData {
	customer_sales_data: CustomerSalesData,
	vehicle_sales_data: Option<VehicleSalesData>,
	carpet_sales_data: Option<CarpetSalesData>,
}

#[derive(Deserialize)]
struct CustomerSalesData {
    cust_name: String,
	mobile_no: String,
	cleaning_service: String,
}

#[derive(Deserialize)]
struct VehicleSalesData {
    vehicle_make: String,
	vehicle_model: String,
	vehicle_regno: String,
	sales_amount: String,
	payment_mode: String,
}

#[derive(Deserialize)]
struct CarpetSalesData {
    carpet_size: String,
	carpet_colour: String,
	sales_amount: String,
	payment_mode: String,
}

#[derive(Deserialize)]
struct VehicleMakeData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct VehicleModelData {
    mobile_no: Option<String>,
	vehicle_make: Option<String>,
}

#[derive(Deserialize)]
struct CarpetTypeSizeData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct VehicleCleaningTypeCostData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct CarpetCleaningTypeCostData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct CarpetTypeColourData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct HistorySalesData {
    mobile_no: Option<String>,
	device_registration_token: Option<String>,
}

#[derive(Deserialize)]
struct SearchSalesItems {
    mobile_no: Option<bool>,
	customer_name: Option<bool>,
	vehicle_regno: Option<bool>,
}

#[derive(Deserialize)]
struct SearchHistorySalesData {
    search_data: Option<String>,
	search_by: SearchSalesItems,
}

enum ProcessingStatus {
	Zero,
	One,
	Two,
}

#[derive(Serialize)]
struct ResponseData {
    status_code: u32,
	status_description: String,
}

#[derive(Serialize)]
struct ResponseData1 {
    status_code: u32,
	status_description: String,
    person_data: Vec<PersonDetails>,
}

#[derive(Serialize)]
struct PersonDetails {
    username: String,
	location: String,
	beneficiary: BeneficiaryDetails,
	staff_name: String,
	job_level: String,
}

#[derive(Serialize)]
//#[derive(Debug)]
struct BeneficiaryDetails {
    full_name: String,
	relationship: String,
}

#[derive(Serialize)]
struct VehicleMakeResponseData {
	message_data: String,
    status_code: u32,
	status_description: String,
	cost_data: Vec<VehicleCleaningTypeCostDetails>,
}

#[derive(Serialize)]
struct VehicleModelResponseData {
	message_data: String,
    status_code: u32,
	status_description: String,
}

#[derive(Serialize)]
struct CarpetTypeSizeResponseData {
	message_data: String,
    status_code: u32,
	status_description: String,
	cost_data: Vec<CarpetCleaningTypeCostDetails>,
}

#[derive(Serialize)]
struct VehicleCleaningTypeCostResponseData {
    status_code: u32,
	status_description: String,
    cost_data: Vec<VehicleCleaningTypeCostDetails>,
}

#[derive(Serialize)]
struct VehicleCleaningTypeCostDetails {
    cleaning_type_name: String,
	amount: u32,
}

#[derive(Serialize)]
struct CarpetCleaningTypeCostResponseData {
    status_code: u32,
	status_description: String,
    cost_data: Vec<CarpetCleaningTypeCostDetails>,
}

#[derive(Serialize)]
struct CarpetCleaningTypeCostDetails {
    cleaning_size_name: String,
	amount: u32,
}

#[derive(Serialize)]
struct CarpetTypeColourResponseData {
	message_data: String,
    status_code: u32,
	status_description: String,
}

#[derive(Serialize)]
struct HistoryVehicleSalesData {
    vehicle_make: String,
	vehicle_model: String,
	vehicle_regno: String,
	sales_amount: u32,
	payment_mode: String,
	interior_cleaning: bool,
	exterior_cleaning: bool,
	engine_cleaning: bool,
	undercarriage_cleaning: bool,
	transaction_date: String,
}

#[derive(Serialize)]
struct HistoryCarpetSalesData {
    carpet_size: String,
	carpet_colour: String,
	sales_amount: u32,
	payment_mode: String,
	transaction_date: String,
}

#[derive(Serialize)]
struct HistoryCustomerSalesData {
    cust_name: String,
	mobile_no: String,
	//cleaning_service: String,
}

#[derive(Serialize)]
struct HistorySalesResponseData {
	customer_sales_data: HistoryCustomerSalesData,
	carpet_sales_data: Vec<HistoryCarpetSalesData>,
	vehicle_sales_data: Vec<HistoryVehicleSalesData>,
}

#[derive(Serialize)]
struct HistorySalesBatchData {
    batch_no: String,
    sales_data: HistorySalesResponseData,
}

#[derive(Serialize)]
struct HistorySalesBatchResponseData {
    status_code: u32,
	status_description: String,
	sales_batch_data: Vec<HistorySalesBatchData>,
}

#[derive(Debug, PartialEq, Eq)]
struct SalesBatchDataTable {
    batch_no: Option<i32>,
	cust_name: String,
    mobile_no: String,
    cleaning_service: String,
    sales_amount: i32,
	paid_amount: i32,
    payment_mode: String,
}

#[derive(Debug, PartialEq, Eq)]
struct SalesDataTable {
    batch_no: i32,
    cleaning_service: String,
	carpet_size: String,
    carpet_colour: String,
    vehicle_make: String,
    vehicle_model: String,
    vehicle_regno: String,
    interior_cleaning: bool,
    exterior_cleaning: bool,
    engine_cleaning: bool,
    undercarriage_cleaning: bool,
    sales_amount: i32,
}
/*	
let url = get_conn_url();

let pool = Pool::new(url)?;

let mut conn = pool.get_conn()?;
*/

#[get("/hello")]
async fn hello_world() -> impl Responder {
    "Hello World!"
}

#[get("/temp")]
async fn current_temperature() -> impl Responder {
    web::Json(Measurement { temperature: 42.3 })
}

//async fn get_person(info: web::Json<Info>) -> Result<String> {
/// deserialize `Info` from request's body
#[post("/person")]
async fn get_person(info: web::Json<Info>) -> impl Responder {
	//let user_name: String = String::from(info.username);
	//let user_name: String = info.username.clone();
	//let user_name: &String = &info.username;
	let user_name = &info.username;
	let my_staff_name = &info.posted_by.staff_name;
	let my_job_level = &info.posted_by.job_level;
	let location_name = get_location();
	let my_beneficiary = BeneficiaryDetails { full_name: String::from("Moses Weta"), relationship: String::from("Son") };
	let my_beneficiary1 = BeneficiaryDetails { full_name: String::from("Benta Shiraku"), relationship: String::from("Daughter") };
	let my_beneficiary2 = BeneficiaryDetails { full_name: String::from("Paul Owino"), relationship: String::from("Son") };
    //Ok(format!("Welcome {}!", info.username))
	//web::Json(PersonDetails { username: user_name.to_string(), location: location_name, beneficiary: my_beneficiary, staff_name: my_staff_name.to_string(), job_level: my_job_level.to_string() })
	let mut x = Vec::new();
	let my_person = PersonDetails { username: user_name.to_string(), location: location_name, beneficiary: my_beneficiary, staff_name: my_staff_name.to_string(), job_level: my_job_level.to_string() };
	let my_person1 = PersonDetails { username: String::from("walter"), location: String::from("westlands"), beneficiary: my_beneficiary1, staff_name: my_staff_name.to_string(), job_level: my_job_level.to_string() };
	//let my_person2 = PersonDetails { username: String::from("mary"), location: String::from("ngong"), beneficiary: my_beneficiary2, staff_name: my_staff_name.to_string(), job_level: my_job_level.to_string() };
	let my_person2 = PersonDetails { username: user_name.to_string(), location: String::from("ngong"), beneficiary: my_beneficiary2, staff_name: my_staff_name.to_string(), job_level: my_job_level.to_string() };
	//println!("my_beneficiary borrowed in {:?}", my_beneficiary);
	x.push(my_person);
	x.push(my_person1);
	x.push(my_person2);
	//web::Json(x)
	//let my_response_data = ResponseData { status_code: 0, status_description: String::from("Successful"), person_data: x };
	let my_response_data = ResponseData1 { status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), person_data: x };
	web::Json(my_response_data)
}

fn get_location() -> String {
	let local_name = String::from("Dandora");
	local_name
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

/// deserialize `VehicleMakeData` from request's body
#[post("/getvehiclemakedata")]
async fn get_vehicle_make_data(vehicle_make_data: web::Json<VehicleMakeData>, req: HttpRequest) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let mut authorization = String::from("");
	let mut channel_type = String::from("");
	let mut app_ver_code = String::from("");
	let mut app_id_tok = String::from("");
	let mut dev_id = String::from("");
	let mut dev_tok_regno = String::from("");
	let mut auth_token = String::from("");
	let mut user_name = String::from("");
	let mut pass_word = String::from("");
	
	if !req.headers().is_empty() {
		if req.headers().contains_key("authorization") {
			let m = req.headers().get("authorization").unwrap();
			authorization = m.to_str().unwrap().to_string();
			println!("m authorization - {:?}", m);
			if !authorization.is_empty() {
				if authorization.to_lowercase().contains("bearer") {
					println!("bearer found");
					let v: Vec<&str> = authorization.split(' ').collect();
					println!("v - {:?}", v);
					let s = v.len();
					if s == 2 {
						auth_token = String::from(v[1]);
						println!("auth_token - {:?}", auth_token);
						let bytes = decode(auth_token).unwrap();
						let m_auth_token = str::from_utf8(&bytes).unwrap().to_string();
						println!("auth_token bytes 2 - {:?}", m_auth_token);
						if !m_auth_token.is_empty() {
							if m_auth_token.contains(":") {
								let w: Vec<&str> = m_auth_token.split(':').collect();
								println!("w - {:?}", w);
								let t = w.len();
								if t == 2 {
									user_name = String::from(w[0]);
									pass_word = String::from(w[1]);
								}
							}
							println!("user_name - {:?}", user_name);
							println!("pass_word - {:?}", pass_word);
						}
					}
				}
			}
		}
		if req.headers().contains_key("channeltype") {
			let m = req.headers().get("channeltype").unwrap();
			channel_type = m.to_str().unwrap().to_string();
			println!("m channel_type - {:?}", m);
		}
		if req.headers().contains_key("appvercode") {
			let m = req.headers().get("appvercode").unwrap();
			app_ver_code = m.to_str().unwrap().to_string();
			println!("m app_ver_code - {:?}", m);
		}
		if req.headers().contains_key("appidtok") {
			let m = req.headers().get("appidtok").unwrap();
			app_id_tok = m.to_str().unwrap().to_string();
			println!("m app_id_tok - {:?}", m);
		}
		if req.headers().contains_key("devid") {
			let m = req.headers().get("devid").unwrap();
			dev_id = m.to_str().unwrap().to_string();
			println!("m dev_id - {:?}", m);
		}
		if req.headers().contains_key("devtokregno") {
			let m = req.headers().get("devtokregno").unwrap();
			dev_tok_regno = m.to_str().unwrap().to_string();
			println!("m dev_tok_regno - {:?}", m);
		}
	}
	
	println!("channel_type - {:?}", channel_type);
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

	let interior_item = VehicleCleaningTypeCostDetails { cleaning_type_name: interior_cleaning_name, amount: interior_cleaning_cost };
	k.push(interior_item);
	let exterior_item = VehicleCleaningTypeCostDetails { cleaning_type_name: exterior_cleaning_name, amount: exterior_cleaning_cost };
	k.push(exterior_item);
	let engine_item = VehicleCleaningTypeCostDetails { cleaning_type_name: engine_cleaning_name, amount: engine_cleaning_cost };
	k.push(engine_item);
	let under_carriage_item = VehicleCleaningTypeCostDetails { cleaning_type_name: under_carriage_cleaning_name, amount: under_carriage_cleaning_cost };
	k.push(under_carriage_item);
	
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("vehicle_make - "), vehicle_make);
	let c = format!("{}{}", String::from("vehicle_cleaning_type_cost - "), k.len().to_string());
	let d = format!("{}{}{}{}{}{}", a, x, b, x, c, x);
	//println!("details is {:?}", d);
	
	let response_data = VehicleMakeResponseData {message_data: vehicle_make.to_string(), status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), cost_data: k };
	web::Json(response_data)
}

/// deserialize `VehicleModelData` from request's body
#[post("/getvehiclemodeldata")]
async fn get_vehicle_model_data(vehicle_model_data: web::Json<VehicleModelData>, req: HttpRequest) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let mut authorization = String::from("");
	let mut channel_type = String::from("");
	let mut app_ver_code = String::from("");
	let mut app_id_tok = String::from("");
	let mut dev_id = String::from("");
	let mut dev_tok_regno = String::from("");
	let mut auth_token = String::from("");
	let mut user_name = String::from("");
	let mut pass_word = String::from("");
	
	if !req.headers().is_empty() {
		if req.headers().contains_key("authorization") {
			let m = req.headers().get("authorization").unwrap();
			authorization = m.to_str().unwrap().to_string();
			println!("m authorization - {:?}", m);
			if !authorization.is_empty() {
				if authorization.to_lowercase().contains("bearer") {
					println!("bearer found");
					let v: Vec<&str> = authorization.split(' ').collect();
					println!("v - {:?}", v);
					let s = v.len();
					if s == 2 {
						auth_token = String::from(v[1]);
						println!("auth_token - {:?}", auth_token);
						let bytes = decode(auth_token).unwrap();
						let m_auth_token = str::from_utf8(&bytes).unwrap().to_string();
						println!("auth_token bytes 2 - {:?}", m_auth_token);
						if !m_auth_token.is_empty() {
							if m_auth_token.contains(":") {
								let w: Vec<&str> = m_auth_token.split(':').collect();
								println!("w - {:?}", w);
								let t = w.len();
								if t == 2 {
									user_name = String::from(w[0]);
									pass_word = String::from(w[1]);
								}
							}
							println!("user_name - {:?}", user_name);
							println!("pass_word - {:?}", pass_word);
						}
					}
				}
			}
		}
		if req.headers().contains_key("channeltype") {
			let m = req.headers().get("channeltype").unwrap();
			channel_type = m.to_str().unwrap().to_string();
			println!("m channel_type - {:?}", m);
		}
		if req.headers().contains_key("appvercode") {
			let m = req.headers().get("appvercode").unwrap();
			app_ver_code = m.to_str().unwrap().to_string();
			println!("m app_ver_code - {:?}", m);
		}
		if req.headers().contains_key("appidtok") {
			let m = req.headers().get("appidtok").unwrap();
			app_id_tok = m.to_str().unwrap().to_string();
			println!("m app_id_tok - {:?}", m);
		}
		if req.headers().contains_key("devid") {
			let m = req.headers().get("devid").unwrap();
			dev_id = m.to_str().unwrap().to_string();
			println!("m dev_id - {:?}", m);
		}
		if req.headers().contains_key("devtokregno") {
			let m = req.headers().get("devtokregno").unwrap();
			dev_tok_regno = m.to_str().unwrap().to_string();
			println!("m dev_tok_regno - {:?}", m);
		}
	}
	
	println!("channel_type - {:?}", channel_type);
	let mobile_no = &vehicle_model_data.mobile_no.as_ref().unwrap_or(&k);
	//let mut vehicle_make = &vehicle_model_data.vehicle_make.as_ref().unwrap_or(&k);
	let vehicle_make = &vehicle_model_data.vehicle_make.as_ref().unwrap_or(&k);
	//let vehicle_model = String::from("AUDI|AUDI-A2|AUDI-A3|AUDI-A3 SE AUTO|AUDI-A4 AVANT S|AUDI-A4 AVANT W|AUDI-A4 FSI S-L|AUDI-A4 TURBO Q|AUDI-A6 SE AUTO|AUDI-A6 STDI|AUDI-A6 TURBO|AUDI-A6 TURBO S|AUDI-A8 SE TDI|AUDI-AUDI 500|AUDI-Q5|AUDI-Q5 3.2L AU|AUDI-Q7|");
	let mut vehicle_model = String::from("");
	
	//let t = vehicle_make.to_lowercase().eq(String::from("audi"))
	
	let a_1 = String::from("audi");
	let a_2 = String::from("bajaj");
	let a_3 = String::from("bmw");
	
	vehicle_model =
		{
			if vehicle_make.to_lowercase().eq(&a_1) {
				String::from("AUDI|AUDI-A2|AUDI-A3|AUDI-A3 SE AUTO|AUDI-A4 AVANT S|AUDI-A4 AVANT W|AUDI-A4 FSI S-L|AUDI-A4 TURBO Q|AUDI-A6 SE AUTO|AUDI-A6 STDI|AUDI-A6 TURBO|AUDI-A6 TURBO S|AUDI-A8 SE TDI|AUDI-AUDI 500|AUDI-Q5|AUDI-Q5 3.2L AU|AUDI-Q7|")
			}
			else if vehicle_make.to_lowercase().eq(&a_2){
				String::from("BAJAJ BM 150X|BAJAJ QUTE|BAJAJ TUKTUK|")
			}
			else if vehicle_make.to_lowercase().eq(&a_3){
				String::from("BMW|BMW 316I|BMW 5|BMW 650GS|BMW ABA-VA20|BMW IGT|BMW-116D F20SH|BMW-116D N47 U|BMW-116I|BMW-116I E81 N|BMW-116I F20SH|BMW-118D N47 U|BMW-118D- N47|BMW-118I E88 C|BMW-118I N13 F|BMW-118I N46 U|BMW-120D CP|BMW-120D N47 U|BMW-120I E82 C|BMW-120I N46 U|BMW-125I N52 U|BMW-130I MANUA|BMW-130I N52 U|BMW-135I N54 U|BMW-135I N55 U|BMW-316I E90 L|BMW-318I I N46|BMW-318I N46 P|BMW-320D|BMW-320D N47 K|BMW-320ED|BMW-320I|BMW-320I E93 C|BMW-320I N46 P|BMW-325I AUTO|BMW-325I MANUA|BMW-325I N52 D|BMW-325I N52 K|BMW-325I N52 P|BMW-325I N53 C|BMW-330 CI CON|BMW-330D|BMW-330D N57 K|BMW-330I  N52|BMW-330I AUTO|BMW-330I MANUA|BMW-335I|BMW-335I N54 K|BMW-335I N54 P|BMW-335I N55 D|BMW-520D AUTO|BMW-520I|BMW-523I|BMW-525D|BMW-525I|BMW-525I AUTO|BMW-528I AUTO|BMW-530D N57 F|BMW-530I MANUA|BMW-535D N57 S|BMW-535I F07 G|BMW-535I N55 F|BMW-550I N63 F|BMW-550I N63 S|BMW-630I  E63|BMW-630I  E64|BMW-650I  N62|BMW-730D|BMW-730I F02 L|BMW-730IAUTO|BMW-730LD N57|BMW-735I|BMW-740I N54 K|BMW-740LI N54|BMW-745IA|BMW-750I N63 K|BMW-750I XDRIV|BMW-750LI N63|BMW-750LI XDRI|BMW-760I N74 K|BMW-760LI N74|BMW-BMW MINI C|BMW-BMW MOTOR CYCLE|BMW-BMW Z3 ROA|BMW-F650 GS 218|BMW-F800 GS 219|BMW-F800 R 217|BMW-F800 ST 234|BMW-G650 GS SERTA|BMW-G650GS 188|BMW-K1300 R 518|BMW-K1300 S 308|BMW-K1600 GT 601|BMW-K1600 GTL 602|BMW-M3 S65 DX9|BMW-M3 S65 KG9|BMW-M3 S65 PM9|BMW-M6 S85 EH9|BMW-M6 S85 EK9|BMW-R1200 GS 450|BMW-R1200 R 400|BMW-R1200 R GS ADV|BMW-R1200 RT 430|BMW-R1200RT 430|BMW-R900 RT 330|BMW-S1000 RR 524|BMW-X1|BMW-X1 SDRIVEN|BMW-X1 XDRIVEN|BMW-X3|BMW-X3 XDRIVE2|BMW-X3 XDRIVE3|BMW-X3 XDRIVEN|BMW-X3 XRIVE30|BMW-X5|BMW-X5 3.0D|BMW-X5 351|BMW-X5 M S63 G|BMW-X5 XDRIVE5|BMW-X5 XDRIVEN|BMW-X6|BMW-X6 M N63 G|BMW-X6 XDRIVE5|BMW-Z4 E89 ROA|")
			}
			else{
				String::from("")
			}
		};
	
	/*
	let vehicle_model = 
	match vehicle_make {
            String::from("audi") => 
				String::from("AUDI|AUDI-A2|AUDI-A3|AUDI-A3 SE AUTO|AUDI-A4 AVANT S|AUDI-A4 AVANT W|AUDI-A4 FSI S-L|AUDI-A4 TURBO Q|AUDI-A6 SE AUTO|AUDI-A6 STDI|AUDI-A6 TURBO|AUDI-A6 TURBO S|AUDI-A8 SE TDI|AUDI-AUDI 500|AUDI-Q5|AUDI-Q5 3.2L AU|AUDI-Q7|"),
			String::from("toyota") => String::from("toyota"),
            _ => String::from("none"),
        };
	
	match vehicle_make {
            c1 => println!("This is a match 1!"),
			c2 => println!("This is a match 2!"),
            _ => println!("Match failed"),
        }
	*/
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("vehicle_make - "), vehicle_make);
	let c = format!("{}{}", String::from("vehicle_model - "), vehicle_model);
	let d = format!("{}{}{}{}{}{}", a, x, b, x, c, x);
	println!("details is {:?}", d);
	
	let response_data = VehicleModelResponseData {message_data: vehicle_model.to_string(), status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful")};
	web::Json(response_data)
}

/// deserialize `CarpetTypeSizeData` from request's body
#[post("/getcarpettypesizedata")]
async fn get_carpet_type_size_data(carpet_type_size_data: web::Json<CarpetTypeSizeData>, req: HttpRequest) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let mut authorization = String::from("");
	let mut channel_type = String::from("");
	let mut app_ver_code = String::from("");
	let mut app_id_tok = String::from("");
	let mut dev_id = String::from("");
	let mut dev_tok_regno = String::from("");
	let mut auth_token = String::from("");
	let mut user_name = String::from("");
	let mut pass_word = String::from("");
	
	if !req.headers().is_empty() {
		if req.headers().contains_key("authorization") {
			let m = req.headers().get("authorization").unwrap();
			authorization = m.to_str().unwrap().to_string();
			println!("m authorization - {:?}", m);
			if !authorization.is_empty() {
				if authorization.to_lowercase().contains("bearer") {
					println!("bearer found");
					let v: Vec<&str> = authorization.split(' ').collect();
					println!("v - {:?}", v);
					let s = v.len();
					if s == 2 {
						auth_token = String::from(v[1]);
						println!("auth_token - {:?}", auth_token);
						let bytes = decode(auth_token).unwrap();
						let m_auth_token = str::from_utf8(&bytes).unwrap().to_string();
						println!("auth_token bytes 2 - {:?}", m_auth_token);
						if !m_auth_token.is_empty() {
							if m_auth_token.contains(":") {
								let w: Vec<&str> = m_auth_token.split(':').collect();
								println!("w - {:?}", w);
								let t = w.len();
								if t == 2 {
									user_name = String::from(w[0]);
									pass_word = String::from(w[1]);
								}
							}
							println!("user_name - {:?}", user_name);
							println!("pass_word - {:?}", pass_word);
						}
					}
				}
			}
		}
		if req.headers().contains_key("channeltype") {
			let m = req.headers().get("channeltype").unwrap();
			channel_type = m.to_str().unwrap().to_string();
			println!("m channel_type - {:?}", m);
		}
		if req.headers().contains_key("appvercode") {
			let m = req.headers().get("appvercode").unwrap();
			app_ver_code = m.to_str().unwrap().to_string();
			println!("m app_ver_code - {:?}", m);
		}
		if req.headers().contains_key("appidtok") {
			let m = req.headers().get("appidtok").unwrap();
			app_id_tok = m.to_str().unwrap().to_string();
			println!("m app_id_tok - {:?}", m);
		}
		if req.headers().contains_key("devid") {
			let m = req.headers().get("devid").unwrap();
			dev_id = m.to_str().unwrap().to_string();
			println!("m dev_id - {:?}", m);
		}
		if req.headers().contains_key("devtokregno") {
			let m = req.headers().get("devtokregno").unwrap();
			dev_tok_regno = m.to_str().unwrap().to_string();
			println!("m dev_tok_regno - {:?}", m);
		}
	}
	
	println!("channel_type - {:?}", channel_type);
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
	
	let a_item = CarpetCleaningTypeCostDetails { cleaning_size_name: a_cleaning_size_name, amount: a_cleaning_size_cost };
	k.push(a_item);
	let b_item = CarpetCleaningTypeCostDetails { cleaning_size_name: b_cleaning_size_name, amount: b_cleaning_size_cost };
	k.push(b_item);
	let c_item = CarpetCleaningTypeCostDetails { cleaning_size_name: c_cleaning_size_name, amount: c_cleaning_size_cost };
	k.push(c_item);
	let d_item = CarpetCleaningTypeCostDetails { cleaning_size_name: d_size_cleaning_name, amount: d_cleaning_size_cost };
	k.push(d_item);
	
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("carpet_type_size - "), carpet_type_size);
	let c = format!("{}{}", String::from("vehicle_cleaning_type_cost - "), k.len().to_string());
	let d = format!("{}{}{}{}{}{}", a, x, b, x, c, x);
	println!("details is {:?}", d);
	
	let response_data = CarpetTypeSizeResponseData {message_data: carpet_type_size.to_string(), status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), cost_data: k };
	web::Json(response_data)
}

/// deserialize `CarpetTypeColourData` from request's body
#[post("/getcarpettypecolourdata")]
async fn get_carpet_type_colour_data(carpet_type_colour_data: web::Json<CarpetTypeColourData>, req: HttpRequest) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let mut authorization = String::from("");
	let mut channel_type = String::from("");
	let mut app_ver_code = String::from("");
	let mut app_id_tok = String::from("");
	let mut dev_id = String::from("");
	let mut dev_tok_regno = String::from("");
	let mut auth_token = String::from("");
	let mut user_name = String::from("");
	let mut pass_word = String::from("");
	
	if !req.headers().is_empty() {
		if req.headers().contains_key("authorization") {
			let m = req.headers().get("authorization").unwrap();
			authorization = m.to_str().unwrap().to_string();
			println!("m authorization - {:?}", m);
			if !authorization.is_empty() {
				if authorization.to_lowercase().contains("bearer") {
					println!("bearer found");
					let v: Vec<&str> = authorization.split(' ').collect();
					println!("v - {:?}", v);
					let s = v.len();
					if s == 2 {
						auth_token = String::from(v[1]);
						println!("auth_token - {:?}", auth_token);
						let bytes = decode(auth_token).unwrap();
						let m_auth_token = str::from_utf8(&bytes).unwrap().to_string();
						println!("auth_token bytes 2 - {:?}", m_auth_token);
						if !m_auth_token.is_empty() {
							if m_auth_token.contains(":") {
								let w: Vec<&str> = m_auth_token.split(':').collect();
								println!("w - {:?}", w);
								let t = w.len();
								if t == 2 {
									user_name = String::from(w[0]);
									pass_word = String::from(w[1]);
								}
							}
							println!("user_name - {:?}", user_name);
							println!("pass_word - {:?}", pass_word);
						}
					}
				}
			}
		}
		if req.headers().contains_key("channeltype") {
			let m = req.headers().get("channeltype").unwrap();
			channel_type = m.to_str().unwrap().to_string();
			println!("m channel_type - {:?}", m);
		}
		if req.headers().contains_key("appvercode") {
			let m = req.headers().get("appvercode").unwrap();
			app_ver_code = m.to_str().unwrap().to_string();
			println!("m app_ver_code - {:?}", m);
		}
		if req.headers().contains_key("appidtok") {
			let m = req.headers().get("appidtok").unwrap();
			app_id_tok = m.to_str().unwrap().to_string();
			println!("m app_id_tok - {:?}", m);
		}
		if req.headers().contains_key("devid") {
			let m = req.headers().get("devid").unwrap();
			dev_id = m.to_str().unwrap().to_string();
			println!("m dev_id - {:?}", m);
		}
		if req.headers().contains_key("devtokregno") {
			let m = req.headers().get("devtokregno").unwrap();
			dev_tok_regno = m.to_str().unwrap().to_string();
			println!("m dev_tok_regno - {:?}", m);
		}
	}
	
	println!("channel_type - {:?}", channel_type);
	let mobile_no = &carpet_type_colour_data.mobile_no.as_ref().unwrap_or(&k);
	let carpet_type_colour = String::from("CARPET COLOUR|WHITE|BLACK|RED|BLUE|YELLOW|ORANGE|PURPLE|GREEN|MIXTURE");
	
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("carpet_type_colour - "), carpet_type_colour);
	let c = format!("{}{}{}{}", a, x, b, x);
	println!("details is {:?}", c);
	
	let response_data = CarpetTypeColourResponseData {message_data: carpet_type_colour.to_string(), status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful") };
	web::Json(response_data)
}

/// deserialize `VehicleCleaningTypeCostData` from request's body
#[post("/getvehiclecleaningtypecostdata")]
async fn get_vehicle_cleaning_type_cost_data(vehicle_cleaning_type_cost_data: web::Json<VehicleCleaningTypeCostData>, req: HttpRequest) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let mut authorization = String::from("");
	let mut channel_type = String::from("");
	let mut app_ver_code = String::from("");
	let mut app_id_tok = String::from("");
	let mut dev_id = String::from("");
	let mut dev_tok_regno = String::from("");
	let mut auth_token = String::from("");
	let mut user_name = String::from("");
	let mut pass_word = String::from("");
	
	if !req.headers().is_empty() {
		if req.headers().contains_key("authorization") {
			let m = req.headers().get("authorization").unwrap();
			authorization = m.to_str().unwrap().to_string();
			println!("m authorization - {:?}", m);
			if !authorization.is_empty() {
				if authorization.to_lowercase().contains("bearer") {
					println!("bearer found");
					let v: Vec<&str> = authorization.split(' ').collect();
					println!("v - {:?}", v);
					let s = v.len();
					if s == 2 {
						auth_token = String::from(v[1]);
						println!("auth_token - {:?}", auth_token);
						let bytes = decode(auth_token).unwrap();
						let m_auth_token = str::from_utf8(&bytes).unwrap().to_string();
						println!("auth_token bytes 2 - {:?}", m_auth_token);
						if !m_auth_token.is_empty() {
							if m_auth_token.contains(":") {
								let w: Vec<&str> = m_auth_token.split(':').collect();
								println!("w - {:?}", w);
								let t = w.len();
								if t == 2 {
									user_name = String::from(w[0]);
									pass_word = String::from(w[1]);
								}
							}
							println!("user_name - {:?}", user_name);
							println!("pass_word - {:?}", pass_word);
						}
					}
				}
			}
		}
		if req.headers().contains_key("channeltype") {
			let m = req.headers().get("channeltype").unwrap();
			channel_type = m.to_str().unwrap().to_string();
			println!("m channel_type - {:?}", m);
		}
		if req.headers().contains_key("appvercode") {
			let m = req.headers().get("appvercode").unwrap();
			app_ver_code = m.to_str().unwrap().to_string();
			println!("m app_ver_code - {:?}", m);
		}
		if req.headers().contains_key("appidtok") {
			let m = req.headers().get("appidtok").unwrap();
			app_id_tok = m.to_str().unwrap().to_string();
			println!("m app_id_tok - {:?}", m);
		}
		if req.headers().contains_key("devid") {
			let m = req.headers().get("devid").unwrap();
			dev_id = m.to_str().unwrap().to_string();
			println!("m dev_id - {:?}", m);
		}
		if req.headers().contains_key("devtokregno") {
			let m = req.headers().get("devtokregno").unwrap();
			dev_tok_regno = m.to_str().unwrap().to_string();
			println!("m dev_tok_regno - {:?}", m);
		}
	}
	
	println!("channel_type - {:?}", channel_type);
	let mobile_no = &vehicle_cleaning_type_cost_data.mobile_no.as_ref().unwrap_or(&k);
	let mut k = Vec::new();
	let interior_cleaning_name = String::from("interior");
	let exterior_cleaning_name = String::from("exterior");
	let engine_cleaning_name = String::from("engine");
	let under_carriage_cleaning_name = String::from("undercarriage");
	
	let interior_cleaning_cost = 200;
	let exterior_cleaning_cost = 300;
	let engine_cleaning_cost = 150;
	let under_carriage_cleaning_cost = 210;
	
	let interior_item = VehicleCleaningTypeCostDetails { cleaning_type_name: interior_cleaning_name, amount: interior_cleaning_cost };
	k.push(interior_item);
	let exterior_item = VehicleCleaningTypeCostDetails { cleaning_type_name: exterior_cleaning_name, amount: exterior_cleaning_cost };
	k.push(exterior_item);
	let engine_item = VehicleCleaningTypeCostDetails { cleaning_type_name: engine_cleaning_name, amount: engine_cleaning_cost };
	k.push(engine_item);
	let under_carriage_item = VehicleCleaningTypeCostDetails { cleaning_type_name: under_carriage_cleaning_name, amount: under_carriage_cleaning_cost };
	k.push(under_carriage_item);
	
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("vehicle_cleaning_type_cost - "), k.len().to_string());
	let c = format!("{}{}{}{}", a, x, b, x);
	println!("details is {:?}", c);
	
	let response_data = VehicleCleaningTypeCostResponseData { status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), cost_data: k };
	web::Json(response_data)
}

/// deserialize `CarpetCleaningTypeCostData` from request's body
#[post("/getcarpetcleaningtypecostdata")]
async fn get_carpet_cleaning_type_cost_data(carpet_cleaning_type_cost_data: web::Json<CarpetCleaningTypeCostData>, req: HttpRequest) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let mut authorization = String::from("");
	let mut channel_type = String::from("");
	let mut app_ver_code = String::from("");
	let mut app_id_tok = String::from("");
	let mut dev_id = String::from("");
	let mut dev_tok_regno = String::from("");
	let mut auth_token = String::from("");
	let mut user_name = String::from("");
	let mut pass_word = String::from("");
	
	if !req.headers().is_empty() {
		if req.headers().contains_key("authorization") {
			let m = req.headers().get("authorization").unwrap();
			authorization = m.to_str().unwrap().to_string();
			println!("m authorization - {:?}", m);
			if !authorization.is_empty() {
				if authorization.to_lowercase().contains("bearer") {
					println!("bearer found");
					let v: Vec<&str> = authorization.split(' ').collect();
					println!("v - {:?}", v);
					let s = v.len();
					if s == 2 {
						auth_token = String::from(v[1]);
						println!("auth_token - {:?}", auth_token);
						let bytes = decode(auth_token).unwrap();
						let m_auth_token = str::from_utf8(&bytes).unwrap().to_string();
						println!("auth_token bytes 2 - {:?}", m_auth_token);
						if !m_auth_token.is_empty() {
							if m_auth_token.contains(":") {
								let w: Vec<&str> = m_auth_token.split(':').collect();
								println!("w - {:?}", w);
								let t = w.len();
								if t == 2 {
									user_name = String::from(w[0]);
									pass_word = String::from(w[1]);
								}
							}
							println!("user_name - {:?}", user_name);
							println!("pass_word - {:?}", pass_word);
						}
					}
				}
			}
		}
		if req.headers().contains_key("channeltype") {
			let m = req.headers().get("channeltype").unwrap();
			channel_type = m.to_str().unwrap().to_string();
			println!("m channel_type - {:?}", m);
		}
		if req.headers().contains_key("appvercode") {
			let m = req.headers().get("appvercode").unwrap();
			app_ver_code = m.to_str().unwrap().to_string();
			println!("m app_ver_code - {:?}", m);
		}
		if req.headers().contains_key("appidtok") {
			let m = req.headers().get("appidtok").unwrap();
			app_id_tok = m.to_str().unwrap().to_string();
			println!("m app_id_tok - {:?}", m);
		}
		if req.headers().contains_key("devid") {
			let m = req.headers().get("devid").unwrap();
			dev_id = m.to_str().unwrap().to_string();
			println!("m dev_id - {:?}", m);
		}
		if req.headers().contains_key("devtokregno") {
			let m = req.headers().get("devtokregno").unwrap();
			dev_tok_regno = m.to_str().unwrap().to_string();
			println!("m dev_tok_regno - {:?}", m);
		}
	}
	
	println!("channel_type - {:?}", channel_type);
	let mobile_no = &carpet_cleaning_type_cost_data.mobile_no.as_ref().unwrap_or(&k);
	let mut k = Vec::new();
	let a_cleaning_size_name = String::from("5by8");
	let b_cleaning_size_name = String::from("6by9");
	let c_cleaning_size_name = String::from("7by10");
	let d_size_cleaning_name = String::from("8by11");
	
	let a_cleaning_size_cost = 600;
	let b_cleaning_size_cost = 700;
	let c_cleaning_size_cost = 800;
	let d_cleaning_size_cost = 900;
	
	let a_item = CarpetCleaningTypeCostDetails { cleaning_size_name: a_cleaning_size_name, amount: a_cleaning_size_cost };
	k.push(a_item);
	let b_item = CarpetCleaningTypeCostDetails { cleaning_size_name: b_cleaning_size_name, amount: b_cleaning_size_cost };
	k.push(b_item);
	let c_item = CarpetCleaningTypeCostDetails { cleaning_size_name: c_cleaning_size_name, amount: c_cleaning_size_cost };
	k.push(c_item);
	let d_item = CarpetCleaningTypeCostDetails { cleaning_size_name: d_size_cleaning_name, amount: d_cleaning_size_cost };
	k.push(d_item);
	
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("carpet_cleaning_type_cost - "), k.len().to_string());
	let c = format!("{}{}{}{}", a, x, b, x);
	println!("details is {:?}", c);
	
	let response_data = CarpetCleaningTypeCostResponseData { status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), cost_data: k };
	web::Json(response_data)
}

/// deserialize `SalesBatchData` from request's body
#[post("/addsalesdata")]
async fn add_sales_data(sales_batch_data: web::Json<SalesBatchData>, req: HttpRequest, data: web::Data<Pool>) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	//let channel_type = HeaderName::from_lowercase(b"channeltype").unwrap_or(&k);
	//let channel_type = HeaderValue::from_str("ChannelType").unwrap();
	//let channel_type = req.headers().get("ChannelType");
	let mut authorization = String::from("");
	let mut channel_type = String::from("");
	let mut app_ver_code = String::from("");
	let mut app_id_tok = String::from("");
	let mut dev_id = String::from("");
	let mut dev_tok_regno = String::from("");
	let mut auth_token = String::from("");
	let mut user_name = String::from("");
	let mut pass_word = String::from("");
	
	if !req.headers().is_empty() {
		if req.headers().contains_key("authorization") {
			let m = req.headers().get("authorization").unwrap();
			authorization = m.to_str().unwrap().to_string();
			println!("m authorization - {:?}", m);
			if !authorization.is_empty() {
				if authorization.to_lowercase().contains("bearer") {
					println!("bearer found");
					let v: Vec<&str> = authorization.split(' ').collect();
					println!("v - {:?}", v);
					let s = v.len();
					if s == 2 {
						auth_token = String::from(v[1]);
						println!("auth_token - {:?}", auth_token);
						let bytes = decode(auth_token).unwrap();
						let m_auth_token = str::from_utf8(&bytes).unwrap().to_string();
						println!("auth_token bytes 2 - {:?}", m_auth_token);
						if !m_auth_token.is_empty() {
							if m_auth_token.contains(":") {
								let w: Vec<&str> = m_auth_token.split(':').collect();
								println!("w - {:?}", w);
								let t = w.len();
								if t == 2 {
									user_name = String::from(w[0]);
									pass_word = String::from(w[1]);
								}
							}
							println!("user_name - {:?}", user_name);
							println!("pass_word - {:?}", pass_word);
						}
					}
				}
			}
		}
		if req.headers().contains_key("channeltype") {
			let m = req.headers().get("channeltype").unwrap();
			channel_type = m.to_str().unwrap().to_string();
			println!("m channel_type - {:?}", m);
		}
		if req.headers().contains_key("appvercode") {
			let m = req.headers().get("appvercode").unwrap();
			app_ver_code = m.to_str().unwrap().to_string();
			println!("m app_ver_code - {:?}", m);
		}
		if req.headers().contains_key("appidtok") {
			let m = req.headers().get("appidtok").unwrap();
			app_id_tok = m.to_str().unwrap().to_string();
			println!("m app_id_tok - {:?}", m);
		}
		if req.headers().contains_key("devid") {
			let m = req.headers().get("devid").unwrap();
			dev_id = m.to_str().unwrap().to_string();
			println!("m dev_id - {:?}", m);
		}
		if req.headers().contains_key("devtokregno") {
			let m = req.headers().get("devtokregno").unwrap();
			dev_tok_regno = m.to_str().unwrap().to_string();
			println!("m dev_tok_regno - {:?}", m);
		}
	}
	
	println!("channel_type - {:?}", channel_type);
	//let batch_no = &sales_data.batch_no;
	//let batch_no = &sales_data.batch_no.as_ref().unwrap_or(k.to_owned());
	//let k = String::from(""); //Default value for string variables.
	//let vehicle_sales_data = VehicleSalesData { vehicle_make: String::from(""), vehicle_model: String::from(""), vehicle_regno: String::from(""), sales_amount: String::from(""), payment_mode: String::from("")};
	//let carpet_sales_data = CarpetSalesData { carpet_size: String::from(""), carpet_colour: String::from(""), sales_amount: String::from(""), payment_mode: String::from("")};
	let batch_no = &sales_batch_data.batch_no.as_ref().unwrap_or(&k);
	let sales_batch_data = &sales_batch_data.sales_data;
	/*
	let cust_name = &sales_data.customer_sales_data.cust_name;
	//let vehicle_make = &sales_data.vehicle_sales_data.vehicle_make;
	let vehicle_make = &sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).vehicle_make;
	//let sales_amount_v = &sales_data.vehicle_sales_data.sales_amount;
	let sales_amount_v = &sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).sales_amount;
	let carpet_size = &sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).carpet_size;
	let sales_amount_c = &sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).sales_amount;
	*/
	
	/*
	println!("batch_no is {:?}", batch_no);
	println!("cust_name is {:?}", cust_name);
	println!("vehicle_make is {:?}", vehicle_make);
	println!("sales_amount_v is {:?}", sales_amount_v);
	println!("carpet_size is {:?}", carpet_size);
	println!("sales_amount_c is {:?}", sales_amount_c);
	*/
	/*
	let mut cust_name = String::from("");
	let mut mobile_no = String::from("");
	let mut sales_amount = 0;
	let paid_amount = 0;
	//let mut payment_mode = String::from("");
	let payment_mode = String::from("mpesa");
	let mut vehicle_make = String::from("");
	let mut sales_amount_v = String::from("");
	let mut carpet_size = String::from("");
	let mut sales_amount_c = String::from("");
	let mut sales_batch_data_table = SalesBatchDataTable { cust_name: String::from(""), mobile_no: String::from(""), cleaning_service: String::from(""), sales_amount: 0, paid_amount: 0, payment_mode: String::from("") };
	*/
	let sales_batch_data_table = get_sales_batch_data(sales_batch_data);
	
	/*
	let x = String::from(" ");
	let a = format!("{}{}", String::from("batch_no - "), batch_no);
	let b = format!("{}{}", String::from("cust_name - "), cust_name);
	let c = format!("{}{}", String::from("vehicle_make - "), vehicle_make);
	let d = format!("{}{}", String::from("sales_amount_v - "), sales_amount_v);
	let e = format!("{}{}", String::from("carpet_size - "), carpet_size);
	let f = format!("{}{}", String::from("sales_amount_c - "), sales_amount_c);
	let g = format!("{}{}{}{}{}{}{}{}{}{}{}", a, x, b, x, c, x, d, x, e, x, f);
	println!("details is {:?}", g);
	//let details = format!("{}{}", borrowed_string, another_borrowed_string);
	*/
	
	//Tests only

	//let sales_batch_data_table = SalesBatchDataTable { cust_name: String::from("emmanu"), mobile_no: String::from("0723083761"), cleaning_service: String::from(""), sales_amount: 1490, paid_amount: 0, payment_mode: String::from("mpesa") };
	let batch_no: i32 = create_sales_batch_data(&data, sales_batch_data_table);
	/*
	let sales_data_table = vec![
    SalesDataTable { batch_no: batch_no, cust_name: String::from("emmanu"), mobile_no: String::from("0723083761"), cleaning_service: String::from("carpet"), carpet_size: String::from("5 by 8"), carpet_colour: String::from("blue"), 
			  vehicle_make: String::from(""), vehicle_model: String::from(""), vehicle_regno: String::from(""), interior_cleaning: false, exterior_cleaning: false, engine_cleaning: false, undercarriage_cleaning: false,
			  sales_amount: 930 },
    SalesDataTable { batch_no: batch_no, cust_name: String::from("emmanu"), mobile_no: String::from("0723083761"), cleaning_service: String::from("vehicle"), carpet_size: String::from(""), carpet_colour: String::from(""), 
			  vehicle_make: String::from("Toyota"), vehicle_model: String::from("Corolla"), vehicle_regno: String::from("kag 283j"), interior_cleaning: true, exterior_cleaning: false, engine_cleaning: true, undercarriage_cleaning: false,
			  sales_amount: 560 },
	];
	*/
	let sales_data_table = get_sales_data(sales_batch_data, batch_no);
	let successful: bool = create_sales_data(data, batch_no, sales_data_table);
	
	let response_data = ResponseData { status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful")};
	web::Json(response_data)
}

/// deserialize `HistorySalesData` from request's body
#[post("/getallsalesdata")]
async fn get_all_sales_data(history_sales_data: web::Json<HistorySalesData>, req: HttpRequest, data: web::Data<Pool>) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let mut authorization = String::from("");
	let mut channel_type = String::from("");
	let mut app_ver_code = String::from("");
	let mut app_id_tok = String::from("");
	let mut dev_id = String::from("");
	let mut dev_tok_regno = String::from("");
	let mut auth_token = String::from("");
	let mut user_name = String::from("");
	let mut pass_word = String::from("");
	
	if !req.headers().is_empty() {
		if req.headers().contains_key("authorization") {
			let m = req.headers().get("authorization").unwrap();
			authorization = m.to_str().unwrap().to_string();
			println!("m authorization - {:?}", m);
			if !authorization.is_empty() {
				if authorization.to_lowercase().contains("bearer") {
					println!("bearer found");
					let v: Vec<&str> = authorization.split(' ').collect();
					println!("v - {:?}", v);
					let s = v.len();
					if s == 2 {
						auth_token = String::from(v[1]);
						println!("auth_token - {:?}", auth_token);
						let bytes = decode(auth_token).unwrap();
						let m_auth_token = str::from_utf8(&bytes).unwrap().to_string();
						println!("auth_token bytes 2 - {:?}", m_auth_token);
						if !m_auth_token.is_empty() {
							if m_auth_token.contains(":") {
								let w: Vec<&str> = m_auth_token.split(':').collect();
								println!("w - {:?}", w);
								let t = w.len();
								if t == 2 {
									user_name = String::from(w[0]);
									pass_word = String::from(w[1]);
								}
							}
							println!("user_name - {:?}", user_name);
							println!("pass_word - {:?}", pass_word);
						}
					}
				}
			}
		}
		if req.headers().contains_key("channeltype") {
			let m = req.headers().get("channeltype").unwrap();
			channel_type = m.to_str().unwrap().to_string();
			println!("m channel_type - {:?}", m);
		}
		if req.headers().contains_key("appvercode") {
			let m = req.headers().get("appvercode").unwrap();
			app_ver_code = m.to_str().unwrap().to_string();
			println!("m app_ver_code - {:?}", m);
		}
		if req.headers().contains_key("appidtok") {
			let m = req.headers().get("appidtok").unwrap();
			app_id_tok = m.to_str().unwrap().to_string();
			println!("m app_id_tok - {:?}", m);
		}
		if req.headers().contains_key("devid") {
			let m = req.headers().get("devid").unwrap();
			dev_id = m.to_str().unwrap().to_string();
			println!("m dev_id - {:?}", m);
		}
		if req.headers().contains_key("devtokregno") {
			let m = req.headers().get("devtokregno").unwrap();
			dev_tok_regno = m.to_str().unwrap().to_string();
			println!("m dev_tok_regno - {:?}", m);
		}
	}
	
	//println!("channel_type - {:?}", channel_type);
	let mobile_no = &history_sales_data.mobile_no.as_ref().unwrap_or(&k);
	let mut k_1 = Vec::new();
	let mut k_2 = Vec::new();
	let mut m_1 = Vec::new();
	let mut m_2 = Vec::new();
	/*
	//Carpets
	let carpet_size_1: String = String::from("6 by 9");
	let carpet_colour_1: String = String::from("PURPLE");
	let carpet_sales_amount_1 = 120;
	let carpet_payment_mode_1: String = String::from("m-pesa");
	let carpet_transaction_date_1: String = String::from("10-03-2021, 07:29 pm");
	
	let carpet_size_2: String = String::from("5 by 8");
	let carpet_colour_2: String = String::from("BLUE");
	let carpet_sales_amount_2 = 130;
	let carpet_payment_mode_2: String = String::from("cash");
	let carpet_transaction_date_2: String = String::from("12-03-2021, 02:15 pm");
	
	//Vehicles
	let vehicle_make_1: String = String::from("BMW");
	let vehicle_model_1: String = String::from("BMW 316I");
	let vehicle_regno_1: String = String::from("KAB 123X");
	let vehicle_sales_amount_1 = 350;
	let vehicle_payment_mode_1: String = String::from("cash");
	let interior_cleaning_1: bool = true;
	let exterior_cleaning_1: bool = false;
	let engine_cleaning_1: bool = true;
	let undercarriage_cleaning_1: bool = false;
	let vehicle_transaction_date_1: String = String::from("12-03-2021, 01:00 pm");
	
	let vehicle_make_2: String = String::from("AUDI");
	let vehicle_model_2: String = String::from("AUDI-A3");
	let vehicle_regno_2: String = String::from("KAC 003V");
	let vehicle_sales_amount_2 = 340;
	let vehicle_payment_mode_2: String = String::from("m-pesa");
	let interior_cleaning_2: bool = false;
	let exterior_cleaning_2: bool = true;
	let engine_cleaning_2: bool = false;
	let undercarriage_cleaning_2: bool = true;
	let vehicle_transaction_date_2: String = String::from("12-03-2021, 03:00 pm");
	*/
	//Carpets	
	let carpet_sales_data_1 = get_carpet_sales_data_1();
	k_1.push(carpet_sales_data_1);
	let carpet_sales_data_2 = get_carpet_sales_data_2();
	k_2.push(carpet_sales_data_2);
	
	//Vehicles
	let vehicle_sales_data_1 = get_vehicle_sales_data_1();
	m_1.push(vehicle_sales_data_1);
	let vehicle_sales_data_2 = get_vehicle_sales_data_2();
	m_2.push(vehicle_sales_data_2);
	
	//Customers
	let customer_sales_data_1 = get_customer_sales_data_1();	
	let customer_sales_data_2 = get_customer_sales_data_2();	
	
	/*
	let x = String::from(" ");
	let a = format!("{}{}", String::from("mobile_no - "), mobile_no);
	let b = format!("{}{}", String::from("vehicle_make - "), vehicle_make);
	let c = format!("{}{}", String::from("vehicle_cleaning_type_cost - "), k.len().to_string());
	let d = format!("{}{}{}{}{}{}", a, x, b, x, c, x);
	println!("details is {:?}", d);
	*/
	let history_sales_response_data_1 = HistorySalesResponseData {customer_sales_data: customer_sales_data_1, carpet_sales_data: k_1, vehicle_sales_data: m_1 };
	let history_sales_response_data_2 = HistorySalesResponseData {customer_sales_data: customer_sales_data_2, carpet_sales_data: k_2, vehicle_sales_data: m_2 };
	let batch_no_1: String = String::from("1");
	let history_sales_batch_data_1 = HistorySalesBatchData {batch_no: batch_no_1, sales_data: history_sales_response_data_1 };
	let batch_no_2: String = String::from("2");
	let history_sales_batch_data_2 = HistorySalesBatchData {batch_no: batch_no_2, sales_data: history_sales_response_data_2 };
	let mut n = Vec::new();
	n.push(history_sales_batch_data_1);
	n.push(history_sales_batch_data_2);
	//let response_data = HistorySalesBatchResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), sales_batch_data: n };
	//let sales_batch_data = select_incoming_sales_batch_data_requests(conn);
	let response_data = get_history_sales_batch_data(&data);
	web::Json(response_data)
}

/// deserialize `SearchHistorySalesData` from request's body
#[post("/getsearchsalesdata")]
async fn get_search_sales_data(search_history_sales_data: web::Json<SearchHistorySalesData>, req: HttpRequest, data: web::Data<Pool>) -> impl Responder {
	let k = String::from(""); //Default value for string variables.
	let j: bool = false;
	let mut authorization = String::from("");
	let mut channel_type = String::from("");
	let mut app_ver_code = String::from("");
	let mut app_id_tok = String::from("");
	let mut dev_id = String::from("");
	let mut dev_tok_regno = String::from("");
	let mut auth_token = String::from("");
	let mut user_name = String::from("");
	let mut pass_word = String::from("");
	
	if !req.headers().is_empty() {
		if req.headers().contains_key("authorization") {
			let m = req.headers().get("authorization").unwrap();
			authorization = m.to_str().unwrap().to_string();
			println!("m authorization - {:?}", m);
			if !authorization.is_empty() {
				if authorization.to_lowercase().contains("bearer") {
					println!("bearer found");
					let v: Vec<&str> = authorization.split(' ').collect();
					println!("v - {:?}", v);
					let s = v.len();
					if s == 2 {
						auth_token = String::from(v[1]);
						println!("auth_token - {:?}", auth_token);
						let bytes = decode(auth_token).unwrap();
						let m_auth_token = str::from_utf8(&bytes).unwrap().to_string();
						println!("auth_token bytes 2 - {:?}", m_auth_token);
						if !m_auth_token.is_empty() {
							if m_auth_token.contains(":") {
								let w: Vec<&str> = m_auth_token.split(':').collect();
								println!("w - {:?}", w);
								let t = w.len();
								if t == 2 {
									user_name = String::from(w[0]);
									pass_word = String::from(w[1]);
								}
							}
							println!("user_name - {:?}", user_name);
							println!("pass_word - {:?}", pass_word);
						}
					}
				}
			}
		}
		if req.headers().contains_key("channeltype") {
			let m = req.headers().get("channeltype").unwrap();
			channel_type = m.to_str().unwrap().to_string();
			println!("m channel_type - {:?}", m);
		}
		if req.headers().contains_key("appvercode") {
			let m = req.headers().get("appvercode").unwrap();
			app_ver_code = m.to_str().unwrap().to_string();
			println!("m app_ver_code - {:?}", m);
		}
		if req.headers().contains_key("appidtok") {
			let m = req.headers().get("appidtok").unwrap();
			app_id_tok = m.to_str().unwrap().to_string();
			println!("m app_id_tok - {:?}", m);
		}
		if req.headers().contains_key("devid") {
			let m = req.headers().get("devid").unwrap();
			dev_id = m.to_str().unwrap().to_string();
			println!("m dev_id - {:?}", m);
		}
		if req.headers().contains_key("devtokregno") {
			let m = req.headers().get("devtokregno").unwrap();
			dev_tok_regno = m.to_str().unwrap().to_string();
			println!("m dev_tok_regno - {:?}", m);
		}
	}
	
	//println!("channel_type - {:?}", channel_type);
	let search_data = &search_history_sales_data.search_data.as_ref().unwrap_or(&k);
	let search_by_key = &search_history_sales_data.search_by;
	let mut k = Vec::new();
	let mut m = Vec::new();
	
	let is_mobile_no = &search_by_key.mobile_no.as_ref().unwrap_or(&j);
	let is_customer_name = &search_by_key.customer_name.as_ref().unwrap_or(&j);
	let is_vehicle_regno = &search_by_key.vehicle_regno.as_ref().unwrap_or(&j);
		
	//if is_mobile_no {println!("mobile_no - true");}
	//if is_customer_name {println!("is_customer_name - true");}
	/*
	match is_mobile_no {
		true => println!("mobile_no - true"),
		false => println!("mobile_no - false"),
	}
	match is_customer_name {
		true => println!("customer_name - true"),
		false => println!("customer_name - false"),
	}
	match is_vehicle_regno {
		true => println!("vehicle_regno - true"),
		false => println!("vehicle_regno - false"),
	}
	println!("search_data - {:?}", search_data);
	*/
	
	//Carpets
	/*
	let carpet_size_1: String = String::from("6 by 9");
	let carpet_colour_1: String = String::from("PURPLE");
	let carpet_sales_amount_1 = 120;
	let carpet_payment_mode_1: String = String::from("m-pesa");
	let carpet_transaction_date_1: String = String::from("10-03-2021, 07:29 pm");
	
	let carpet_size_2: String = String::from("5 by 8");
	let carpet_colour_2: String = String::from("BLUE");
	let carpet_sales_amount_2 = 130;
	let carpet_payment_mode_2: String = String::from("cash");
	let carpet_transaction_date_2: String = String::from("12-03-2021, 02:15 pm");
	*/
	//Vehicles
	/*
	let vehicle_make_1: String = String::from("BMW");
	let vehicle_model_1: String = String::from("BMW 316I");
	let vehicle_regno_1: String = String::from("KAB 123X");
	let vehicle_sales_amount_1 = 350;
	let vehicle_payment_mode_1: String = String::from("cash");
	let interior_cleaning_1: bool = true;
	let exterior_cleaning_1: bool = false;
	let engine_cleaning_1: bool = true;
	let undercarriage_cleaning_1: bool = false;
	let vehicle_transaction_date_1: String = String::from("12-03-2021, 01:00 pm");
	
	let vehicle_make_2: String = String::from("AUDI");
	let vehicle_model_2: String = String::from("AUDI-A3");
	let vehicle_regno_2: String = String::from("KAC 003V");
	let vehicle_sales_amount_2 = 340;
	let vehicle_payment_mode_2: String = String::from("m-pesa");
	let interior_cleaning_2: bool = false;
	let exterior_cleaning_2: bool = true;
	let engine_cleaning_2: bool = false;
	let undercarriage_cleaning_2: bool = true;
	let vehicle_transaction_date_2: String = String::from("12-03-2021, 03:00 pm");
	*/

	//Carpets	
	//let carpet_sales_data_1 = HistoryCarpetSalesData { carpet_size: carpet_size_1, carpet_colour: carpet_colour_1, sales_amount: carpet_sales_amount_1, payment_mode: carpet_payment_mode_1, transaction_date: carpet_transaction_date_1 };
	let carpet_sales_data_1 = get_carpet_sales_data_1();
	//k.push(carpet_sales_data_1);
	//let carpet_sales_data_2 = HistoryCarpetSalesData { carpet_size: carpet_size_2, carpet_colour: carpet_colour_2, sales_amount: carpet_sales_amount_2, payment_mode: carpet_payment_mode_2, transaction_date: carpet_transaction_date_2 };
	let carpet_sales_data_2 = get_carpet_sales_data_2();
	//k.push(carpet_sales_data_2);
	
	//Vehicles
	//let vehicle_sales_data_1 = HistoryVehicleSalesData { vehicle_make: vehicle_make_1, vehicle_model: vehicle_model_1, vehicle_regno: vehicle_regno_1, sales_amount: vehicle_sales_amount_1, payment_mode: vehicle_payment_mode_1, interior_cleaning: interior_cleaning_1, exterior_cleaning: exterior_cleaning_1, engine_cleaning: engine_cleaning_1, undercarriage_cleaning: undercarriage_cleaning_1, transaction_date: vehicle_transaction_date_1 };
	let vehicle_sales_data_1 = get_vehicle_sales_data_1();
	//m.push(vehicle_sales_data_1);
	//let vehicle_sales_data_2 = HistoryVehicleSalesData { vehicle_make: vehicle_make_2, vehicle_model: vehicle_model_2, vehicle_regno: vehicle_regno_2, sales_amount: vehicle_sales_amount_2, payment_mode: vehicle_payment_mode_2, interior_cleaning: interior_cleaning_2, exterior_cleaning: exterior_cleaning_2, engine_cleaning: engine_cleaning_2, undercarriage_cleaning: undercarriage_cleaning_2, transaction_date: vehicle_transaction_date_2 };
	let vehicle_sales_data_2 = get_vehicle_sales_data_2();
	//m.push(vehicle_sales_data_2);
	let vehicle_sales_data_3 = get_vehicle_sales_data_1();
	//m.push(vehicle_sales_data_1);
	let vehicle_sales_data_4 = get_vehicle_sales_data_2();
	let vehicle_sales_data_5 = get_vehicle_sales_data_2();
	
	//Customers
	let customer_sales_data_1 = get_customer_sales_data_1();
	
	let a_1 = String::from("nicole");
	let a_2 = String::from("paul");
	let a_3 = String::from("254723083761");
	let a_4 = String::from("KAB 123X");
	let a_5 = String::from("KAC 003V");
	
	match is_mobile_no {
		true => {
			println!("search_data 1 true - {:?}", search_data.replace(" ","").to_lowercase());
			if search_data.replace(" ","").to_lowercase().eq(&a_3.replace(" ","").to_lowercase()) {
				m.push(vehicle_sales_data_5);
				//println!("search_data 1 true - {:?}", search_data.replace(" ","").to_lowercase());
			}
			else{
			}
		}
		,
		false => println!("mobile_no - false"),
	}
	match is_customer_name {
		true => {
			if search_data.to_lowercase().eq(&a_1) {
				k.push(carpet_sales_data_2);
				m.push(vehicle_sales_data_1);
			}
			else if search_data.to_lowercase().eq(&a_2){
				k.push(carpet_sales_data_1);
				m.push(vehicle_sales_data_2);
			}
			else{
			}
		},
		false => println!("customer_name - false"),
	}	
	//println!("search_data 1 - {:?}", search_data.replace(" ","").to_lowercase());
	//println!("a_4 - {:?}", a_4.replace(" ","").to_lowercase());
	match is_vehicle_regno {
		true => {
			if search_data.replace(" ","").to_lowercase().eq(&a_4.replace(" ","").to_lowercase()) {
				m.push(vehicle_sales_data_3);
				println!("search_data 1 true - {:?}", search_data.replace(" ","").to_lowercase());
			}
			else if search_data.replace(" ","").to_lowercase().eq(&a_5.replace(" ","").to_lowercase()){
				m.push(vehicle_sales_data_4);
				println!("search_data 2 true - {:?}", search_data.replace(" ","").to_lowercase());
			}
			else{
			}
		},
		false => println!("vehicle_regno - false"),
	}

	let history_sales_response_data_1 = HistorySalesResponseData {customer_sales_data: customer_sales_data_1, carpet_sales_data: k, vehicle_sales_data: m };
	let batch_no_1: String = String::from("1");
	let history_sales_batch_data_1 = HistorySalesBatchData {batch_no: batch_no_1, sales_data: history_sales_response_data_1 };
	let mut n = Vec::new();
	n.push(history_sales_batch_data_1);
	//let response_data = HistorySalesBatchResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), sales_batch_data: n };
	let response_data = get_history_search_sales_batch_data(search_data, is_mobile_no, is_customer_name, is_vehicle_regno, &data);
	web::Json(response_data)
}

fn get_carpet_sales_data_1() -> HistoryCarpetSalesData {
	let carpet_size: String = String::from("6 by 9");
	let carpet_colour: String = String::from("PURPLE");
	let carpet_sales_amount = 120;
	let carpet_payment_mode: String = String::from("m-pesa");
	let carpet_transaction_date: String = String::from("10-03-2021, 07:29 pm");
	let carpet_sales_data = HistoryCarpetSalesData { carpet_size: carpet_size, carpet_colour: carpet_colour, sales_amount: carpet_sales_amount, payment_mode: carpet_payment_mode, transaction_date: carpet_transaction_date };
	carpet_sales_data
}
fn get_carpet_sales_data_2() -> HistoryCarpetSalesData {
	let carpet_size: String = String::from("5 by 8");
	let carpet_colour: String = String::from("BLUE");
	let carpet_sales_amount = 130;
	let carpet_payment_mode: String = String::from("cash");
	let carpet_transaction_date: String = String::from("12-03-2021, 02:15 pm");
	let carpet_sales_data = HistoryCarpetSalesData { carpet_size: carpet_size, carpet_colour: carpet_colour, sales_amount: carpet_sales_amount, payment_mode: carpet_payment_mode, transaction_date: carpet_transaction_date };
	carpet_sales_data
}
fn get_vehicle_sales_data_1() -> HistoryVehicleSalesData {
	let vehicle_make: String = String::from("BMW");
	let vehicle_model: String = String::from("BMW 316I");
	let vehicle_regno: String = String::from("KAB 123X");
	let vehicle_sales_amount = 350;
	let vehicle_payment_mode: String = String::from("cash");
	let interior_cleaning: bool = true;
	let exterior_cleaning: bool = false;
	let engine_cleaning: bool = true;
	let undercarriage_cleaning: bool = false;
	let vehicle_transaction_date: String = String::from("12-03-2021, 01:00 pm");
	let vehicle_sales_data = HistoryVehicleSalesData { vehicle_make: vehicle_make, vehicle_model: vehicle_model, vehicle_regno: vehicle_regno, sales_amount: vehicle_sales_amount, payment_mode: vehicle_payment_mode, interior_cleaning: interior_cleaning, exterior_cleaning: exterior_cleaning, engine_cleaning: engine_cleaning, undercarriage_cleaning: undercarriage_cleaning, transaction_date: vehicle_transaction_date };
	vehicle_sales_data
}
fn get_vehicle_sales_data_2() -> HistoryVehicleSalesData {
	let vehicle_make: String = String::from("AUDI");
	let vehicle_model: String = String::from("AUDI-A3");
	let vehicle_regno: String = String::from("KAC 003V");
	let vehicle_sales_amount = 340;
	let vehicle_payment_mode: String = String::from("m-pesa");
	let interior_cleaning: bool = false;
	let exterior_cleaning: bool = true;
	let engine_cleaning: bool = false;
	let undercarriage_cleaning: bool = true;
	let vehicle_transaction_date: String = String::from("12-03-2021, 03:00 pm");
	let vehicle_sales_data = HistoryVehicleSalesData { vehicle_make: vehicle_make, vehicle_model: vehicle_model, vehicle_regno: vehicle_regno, sales_amount: vehicle_sales_amount, payment_mode: vehicle_payment_mode, interior_cleaning: interior_cleaning, exterior_cleaning: exterior_cleaning, engine_cleaning: engine_cleaning, undercarriage_cleaning: undercarriage_cleaning, transaction_date: vehicle_transaction_date };
	vehicle_sales_data
}
fn get_customer_sales_data_1() -> HistoryCustomerSalesData {
	let cust_name: String = String::from("nicole");
	let mobile_no: String = String::from("254723083761");
	let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
	customer_sales_data
}
fn get_customer_sales_data_2() -> HistoryCustomerSalesData {
	let cust_name: String = String::from("paul");
	let mobile_no: String = String::from("254723083760");
	let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
	customer_sales_data
}

fn get_conn_url() -> String {
	let url = "mysql://ussd:arunga@2030!@localhost:3306/carwash_n_spa";
	String::from(url)
}

fn create_sales_batch_data(data: &web::Data<Pool>, sales_batch_data: SalesBatchDataTable) -> i32  {
	let mut batch_no: i32 = 0;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_sales_batch_data(&mut conn, sales_batch_data))
    {
        Ok(sales_batch_no) => {
            //println!("Successful to open DB connection."),
			//println!("Successful insert to DB connection. {:?}", sales_batch_id);
			batch_no = sales_batch_no as i32;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	batch_no
}

fn create_sales_data(data: web::Data<Pool>, batch_no: i32, sales_data: Vec<SalesDataTable>) -> bool {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_sales_data(&mut conn, batch_no, sales_data))
    {
        Ok(sales_no) => {
            //println!("Successful to open DB connection."),
			//println!("Successful insert to DB connection. {:?}", sales_no);
			successful = true;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	successful
}

/*
fn insert_sales_data(
    conn: &mut PooledConn) -> std::result::Result<u64, mysql::error::Error> {
	/*
    conn.exec_drop(
        "insert into PRODUCT (product_code, price, name, last_update) values (:product_code, :price, :name, :last_update)",
        params! {
            "product_code" => &product.code,
            "price" => product.price,
            "name" => &product.product_name,
            "last_update" => today(),
        },
    )
    .and_then(|_| Ok(conn.last_insert_id()))
	*/
	let sales_batch_data = SalesBatchDataTable { cust_name: String::from("emmanu"), mobile_no: String::from("0723083761"), cleaning_service: String::from(""), sales_amount: 1490, paid_amount: 0, payment_mode: String::from("mpesa") };
	
	//let mut batch_no: i32 = 1;
	
	// Now let's insert sales batch data to the database
	let my_result =
	conn.exec_drop(
        "insert into incomingsalesbatchdatarequests (cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode) values (:cust_name, :mobile_no, :cleaning_service, :sales_amount, :paid_amount, :payment_mode)",
        params! {
            "cust_name" => sales_batch_data.cust_name,
            "mobile_no" => sales_batch_data.mobile_no,
            "cleaning_service" => sales_batch_data.cleaning_service,
            "sales_amount" => sales_batch_data.sales_amount,
			"paid_amount" => sales_batch_data.paid_amount,
			"payment_mode" => sales_batch_data.payment_mode,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()));
	
	let batch_no: i32 =
		match my_result
		{
			Ok(s) => {
				//batch_no = i32::try_from(s);
				s as i32
			},
			Err(e) => {
				//batch_no = i32::try_from(s);
				0
			},
		};
	
	let sales_data = vec![
    SalesDataTable { batch_no: batch_no, cust_name: String::from("emmanu"), mobile_no: String::from("0723083761"), cleaning_service: String::from("carpet"), carpet_size: String::from("5 by 8"), carpet_colour: String::from("blue"), 
			  vehicle_make: String::from(""), vehicle_model: String::from(""), vehicle_regno: String::from(""), interior_cleaning: false, exterior_cleaning: false, engine_cleaning: false, undercarriage_cleaning: false,
			  sales_amount: 930 },
    SalesDataTable { batch_no: batch_no, cust_name: String::from("emmanu"), mobile_no: String::from("0723083761"), cleaning_service: String::from("vehicle"), carpet_size: String::from(""), carpet_colour: String::from(""), 
			  vehicle_make: String::from("Toyota"), vehicle_model: String::from("Corolla"), vehicle_regno: String::from("kag 283j"), interior_cleaning: true, exterior_cleaning: false, engine_cleaning: true, undercarriage_cleaning: false,
			  sales_amount: 560 },
	];
	
	// Now let's insert sales data to the database
	conn.exec_batch(
		r"insert into incomingsalesdatarequests (batch_no, cleaning_service, carpet_size, carpet_colour, vehicle_make, vehicle_model, vehicle_regno, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, sales_amount)
		  values (:batch_no, :cleaning_service, :carpet_size, :carpet_colour, :vehicle_make, :vehicle_model, :vehicle_regno, :interior_cleaning, :exterior_cleaning, :engine_cleaning, :undercarriage_cleaning, :sales_amount)",
		sales_data.iter().map(|s| params! {
			"batch_no" => s.batch_no,
			"cleaning_service" => &s.cleaning_service,
			"carpet_size" => &s.carpet_size,
			"carpet_colour" => &s.carpet_colour,
			"vehicle_make" => &s.vehicle_make,
			"vehicle_model" => &s.vehicle_model,
			"vehicle_regno" => &s.vehicle_regno,
			"interior_cleaning" => s.interior_cleaning,
			"exterior_cleaning" => s.exterior_cleaning,
			"engine_cleaning" => s.engine_cleaning,
			"undercarriage_cleaning" => s.undercarriage_cleaning,
			"sales_amount" => s.sales_amount,
		})
	)//?;
	.and_then(|_| Ok(conn.last_insert_id()))
	
}
*/
fn insert_sales_batch_data(
    conn: &mut PooledConn, sales_batch_data: SalesBatchDataTable) -> std::result::Result<u64, mysql::error::Error> {
	
	//let mut batch_no: i32 = 0;
	
	// Now let's insert sales batch data to the database
	//let my_result =
	conn.exec_drop(
        "insert into incomingsalesbatchdatarequests (cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode) values (:cust_name, :mobile_no, :cleaning_service, :sales_amount, :paid_amount, :payment_mode)",
        params! {
            "cust_name" => sales_batch_data.cust_name,
            "mobile_no" => sales_batch_data.mobile_no,
            "cleaning_service" => sales_batch_data.cleaning_service,
            "sales_amount" => sales_batch_data.sales_amount,
			"paid_amount" => sales_batch_data.paid_amount,
			"payment_mode" => sales_batch_data.payment_mode,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
	/*
	let batch_no: i32 =
		match my_result
		{
			Ok(s) => {
				//batch_no = i32::try_from(s);
				s as i32
			},
			Err(e) => {
				//batch_no = i32::try_from(s);
				0
			},
		};
	
	batch_no
	*/
}

fn insert_sales_data(
    conn: &mut PooledConn, batch_no: i32, sales_data: Vec<SalesDataTable>) -> std::result::Result<u64, mysql::error::Error> {
	
	// Now let's insert sales data to the database
	conn.exec_batch(
		r"insert into incomingsalesdatarequests (batch_no, cleaning_service, carpet_size, carpet_colour, vehicle_make, vehicle_model, vehicle_regno, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, sales_amount)
		  values (:batch_no, :cleaning_service, :carpet_size, :carpet_colour, :vehicle_make, :vehicle_model, :vehicle_regno, :interior_cleaning, :exterior_cleaning, :engine_cleaning, :undercarriage_cleaning, :sales_amount)",
		sales_data.iter().map(|s| params! {
			"batch_no" => s.batch_no,
			"cleaning_service" => &s.cleaning_service,
			"carpet_size" => &s.carpet_size,
			"carpet_colour" => &s.carpet_colour,
			"vehicle_make" => &s.vehicle_make,
			"vehicle_model" => &s.vehicle_model,
			"vehicle_regno" => &s.vehicle_regno,
			"interior_cleaning" => s.interior_cleaning,
			"exterior_cleaning" => s.exterior_cleaning,
			"engine_cleaning" => s.engine_cleaning,
			"undercarriage_cleaning" => s.undercarriage_cleaning,
			"sales_amount" => s.sales_amount,
		})
	)
	.and_then(|_| Ok(1))
	
}

fn select_incoming_sales_batch_data_requests(
    conn: &mut PooledConn) -> std::result::Result<Vec<HistorySalesBatchData>, mysql::error::Error> {
	let mut sales_batch_data = Vec::new();
	
	//let selected_data: Vec<SalesBatchDataTable> = conn
    conn.query_map(
        "select batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode from incomingsalesbatchdatarequests order by batch_no asc limit 10",
        |(batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode)| {
            let a = SalesBatchDataTable { batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode };
			sales_batch_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	//selected_data
	
	let mut vec_history_sales_batch_data = Vec::new();
	let k: i32 = 0;
	
	for sales_data in sales_batch_data.iter() {
		let cust_name = sales_data.cust_name.to_string();
		let mobile_no = sales_data.mobile_no.to_string();
		
		let batch_no = sales_data.batch_no.as_ref().unwrap_or(&k);
		
		let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
		let carpet_sales_data = select_incoming_carpet_sales_data_requests(conn, batch_no);
		let vehicle_sales_data = select_incoming_vehicle_sales_data_requests(conn, batch_no);
		
		let history_sales_response_data = HistorySalesResponseData {customer_sales_data: customer_sales_data, carpet_sales_data: carpet_sales_data, vehicle_sales_data: vehicle_sales_data };
				
		let history_sales_batch_data = HistorySalesBatchData {batch_no: batch_no.to_string(), sales_data: history_sales_response_data };
		
		
		vec_history_sales_batch_data.push(history_sales_batch_data);

	}
	
	Ok(vec_history_sales_batch_data)
	
}

fn select_incoming_sales_batch_data_requests_old(
    conn: &mut PooledConn) -> Vec<SalesBatchDataTable> {
	let mut selected_data = Vec::new();
	
	//let selected_data: Vec<SalesBatchDataTable> = conn
    conn.query_map(
        "select batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode from incomingsalesbatchdatarequests order by batch_no asc limit 10",
        |(batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode)| {
            let a = SalesBatchDataTable { batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode };
			selected_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	selected_data
	
}

fn select_incoming_search_sales_batch_data_requests(search_data: &String,
    is_mobile_no: &bool, is_customer_name: &bool, is_vehicle_regno: &bool, conn: &mut PooledConn) -> std::result::Result<Vec<HistorySalesBatchData>, mysql::error::Error> {
	let mut sales_batch_data = Vec::new();
	
	/*
	//let selected_data: Vec<SalesBatchDataTable> = conn
    conn.query_map(
        "select batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode from incomingsalesbatchdatarequests order by batch_no asc limit 10",
        |(batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode)| {
            let a = SalesBatchDataTable { batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode };
			sales_batch_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	*/

	//println!("search_data is {:?}", search_data);
	
	conn.exec_map(
    "select batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode from incomingsalesbatchdatarequests where cust_name = :search_data",
	params! {
            "search_data" => search_data,
        },
    |(batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode)| { 
	  let a = 
      SalesBatchDataTable {
            batch_no: batch_no,
            cust_name: cust_name,
            mobile_no: mobile_no,
            cleaning_service: cleaning_service,
            sales_amount: sales_amount,
			paid_amount: paid_amount,
			payment_mode: payment_mode,
        };
		sales_batch_data.push(a);
		},
	)
	.and_then(|_| Ok(1));
	
	//println!("sales_batch_data len is {:?}", sales_batch_data.len());
	
	//selected_data
	
	let mut vec_history_sales_batch_data = Vec::new();
	let k: i32 = 0;
	
	for sales_data in sales_batch_data.iter() {
		let cust_name = sales_data.cust_name.to_string();
		let mobile_no = sales_data.mobile_no.to_string();
		
		let batch_no = sales_data.batch_no.as_ref().unwrap_or(&k);
		
		let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
		let carpet_sales_data = select_incoming_carpet_sales_data_requests(conn, batch_no);
		let vehicle_sales_data = select_incoming_vehicle_sales_data_requests(conn, batch_no);
		
		let history_sales_response_data = HistorySalesResponseData {customer_sales_data: customer_sales_data, carpet_sales_data: carpet_sales_data, vehicle_sales_data: vehicle_sales_data };
				
		let history_sales_batch_data = HistorySalesBatchData {batch_no: batch_no.to_string(), sales_data: history_sales_response_data };
		
		
		vec_history_sales_batch_data.push(history_sales_batch_data);

	}
	
	Ok(vec_history_sales_batch_data)
	
}

fn select_incoming_carpet_sales_data_requests(
    conn: &mut PooledConn, batch_no: &i32) -> Vec<HistoryCarpetSalesData> {
	let mut selected_data = Vec::new();
	let payment_mode: String = String::from("");
	let transaction_date: String = String::from("");
	let cleaning_service: String = String::from("carpet");
	
	//println!("batch_no is {:?}", batch_no);
	
    conn.exec_map(
    "select carpet_size, carpet_colour, sales_amount, date_format(transaction_date, '%d-%m-%Y') transaction_date from incomingsalesdatarequests where batch_no = :batch_no and cleaning_service = :cleaning_service",
	params! {
            "batch_no" => batch_no,
            "cleaning_service" => cleaning_service,
        },
    |(carpet_size, carpet_colour, sales_amount, transaction_date)| { 
	  let a = 
      HistoryCarpetSalesData {
            carpet_size: carpet_size,
            carpet_colour: carpet_colour,
            sales_amount: sales_amount,
            payment_mode: payment_mode.to_string(),
            transaction_date: transaction_date,
        };
		selected_data.push(a);
		},
	)
	.and_then(|_| Ok(1));
	//}
	/*
	println!("Vector selected_data length: {}", selected_data.len());
	
	for s in selected_data.iter() {
		println!("carpet_size - {:?}", &s.carpet_size.to_string());
		println!("sales_amount - {:?}", &s.sales_amount.to_string());
		println!("transaction_date - {:?}", &s.transaction_date.to_string());
	}	
	*/
	selected_data
	
}

fn select_incoming_carpet_sales_data_requests_old(
    conn: &mut PooledConn, batch_no: &i32) -> Vec<HistoryCarpetSalesData> {
	let mut selected_data = Vec::new();
	let payment_mode: String = String::from("");
	
    conn.query_map(
        "select carpet_size, carpet_colour, sales_amount, transaction_date from incomingsalesdatarequests where batch_no = :batch_no and cleaning_service = 'carpet'",
        |(carpet_size, carpet_colour, sales_amount, payment_mode, transaction_date)| {
            let a = HistoryCarpetSalesData { carpet_size, carpet_colour, sales_amount, payment_mode, transaction_date };
			selected_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	selected_data
	
}

fn select_incoming_vehicle_sales_data_requests(
    conn: &mut PooledConn, batch_no: &i32) -> Vec<HistoryVehicleSalesData> {
	let mut selected_data = Vec::new();
	let payment_mode: String = String::from("");
	let transaction_date: String = String::from("");
	let cleaning_service: String = String::from("vehicle");
	
	//println!("batch_no is {:?}", batch_no);
	
    conn.exec_map(
    "select vehicle_make, vehicle_model, vehicle_regno, sales_amount, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, date_format(transaction_date, '%d-%m-%Y') transaction_date from incomingsalesdatarequests where batch_no = :batch_no and cleaning_service = :cleaning_service",
	params! {
            "batch_no" => batch_no,
            "cleaning_service" => cleaning_service,
        },
    |(vehicle_make, vehicle_model, vehicle_regno, sales_amount, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, transaction_date)| { 
	  let a =
	  HistoryVehicleSalesData { 
			vehicle_make: vehicle_make, 
			vehicle_model: vehicle_model, 
			vehicle_regno: vehicle_regno, 
			sales_amount: sales_amount, 
			payment_mode: payment_mode.to_string(), 
			interior_cleaning: interior_cleaning, 
			exterior_cleaning: exterior_cleaning, 
			engine_cleaning: engine_cleaning, 
			undercarriage_cleaning: undercarriage_cleaning, 
			transaction_date: transaction_date };	
		
		selected_data.push(a);
		},
	)
	.and_then(|_| Ok(1));
	//}
	/*
	println!("Vector selected_data length: {}", selected_data.len());
	
	for s in selected_data.iter() {
		println!("vehicle_make - {:?}", &s.vehicle_make.to_string());
		println!("vehicle_regno - {:?}", &s.vehicle_regno.to_string());
		println!("sales_amount - {:?}", &s.sales_amount.to_string());
		println!("transaction_date - {:?}", &s.transaction_date.to_string());
	}	
	*/
	selected_data
	
}

fn select_incoming_vehicle_sales_data_requests_old(
    conn: &mut PooledConn, batch_no: &i32) -> Vec<HistoryVehicleSalesData> {
	let mut selected_data = Vec::new();
	let payment_mode: String = String::from("");
	
    conn.query_map(
        "select vehicle_make, vehicle_model, vehicle_regno, sales_amount, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, transaction_date from incomingsalesdatarequests where batch_no = batch_no and cleaning_service = 'vehicle'",
        |(vehicle_make, vehicle_model, vehicle_regno, sales_amount, payment_mode, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, transaction_date)| {
            let a = HistoryVehicleSalesData { vehicle_make, vehicle_model, vehicle_regno, sales_amount, payment_mode, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, transaction_date };
			selected_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	selected_data
	
}

fn get_sales_batch_data(sales_batch_data: &Vec<SalesData>) -> SalesBatchDataTable  {
	let mut sales_batch_data_table = SalesBatchDataTable { batch_no: None, cust_name: String::from(""), mobile_no: String::from(""), cleaning_service: String::from(""), sales_amount: 0, paid_amount: 0, payment_mode: String::from("") };
	
	let mut cust_name = String::from("");
	let mut mobile_no = String::from("");
	let mut sales_amount = 0;
	let paid_amount = 0;
	//let mut payment_mode = String::from("");
	let payment_mode = String::from("mpesa");
	//let mut vehicle_make = String::from("");
	let mut sales_amount_v = String::from("");
	//let mut carpet_size = String::from("");
	let mut sales_amount_c = String::from("");
	let vehicle_sales_data = VehicleSalesData { vehicle_make: String::from(""), vehicle_model: String::from(""), vehicle_regno: String::from(""), sales_amount: String::from(""), payment_mode: String::from("")};
	let carpet_sales_data = CarpetSalesData { carpet_size: String::from(""), carpet_colour: String::from(""), sales_amount: String::from(""), payment_mode: String::from("")};
	
	for sales_data in sales_batch_data.iter() {
		//cust_name = sales_data.customer_sales_data.cust_name;
		cust_name = sales_data.customer_sales_data.cust_name.to_string();
		mobile_no = sales_data.customer_sales_data.mobile_no.to_string();
		//sales_amount = sales_data.customer_sales_data.sales_amount;
		//paid_amount = sales_data.customer_sales_data.paid_amount;
		//payment_mode = &sales_data.customer_sales_data.payment_mode;
		//vehicle_make = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).vehicle_make.to_string();
		sales_amount_v = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).sales_amount.to_string();
		//carpet_size = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).carpet_size.to_string();
		sales_amount_c = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).sales_amount.to_string();
		
		let vehicle_amount = 
		match sales_amount_v.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
		
		let carpet_amount = 
		match sales_amount_c.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
		
		sales_amount = vehicle_amount + carpet_amount; //test only
		//Assign values to struct variable
		sales_batch_data_table = SalesBatchDataTable { batch_no: None, cust_name: cust_name, mobile_no: mobile_no, cleaning_service: String::from(""), sales_amount: sales_amount, paid_amount: paid_amount, payment_mode: payment_mode.to_string() };

	}
	
	sales_batch_data_table
}

fn get_sales_data(sales_batch_data: &Vec<SalesData>, batch_no: i32) -> Vec<SalesDataTable>  {
	let mut sales_data_table = Vec::new();
	let mut vehicle_make = String::from("");
	let mut vehicle_model = String::from("");
	let mut vehicle_regno = String::from("");
	let mut sales_amount_v = String::from("");
	let mut sales_amount_c = String::from("");
	let mut carpet_size = String::from("");
	let mut carpet_colour = String::from("");
	let vehicle_sales_data = VehicleSalesData { vehicle_make: String::from(""), vehicle_model: String::from(""), vehicle_regno: String::from(""), sales_amount: String::from(""), payment_mode: String::from("")};
	let carpet_sales_data = CarpetSalesData { carpet_size: String::from(""), carpet_colour: String::from(""), sales_amount: String::from(""), payment_mode: String::from("")};
	let mut is_valid_vehicle_data: bool = false;
	let mut is_valid_carpet_data: bool = false;
	
	for sales_data in sales_batch_data.iter() {
		is_valid_vehicle_data = false;
		is_valid_carpet_data = false;
		
		vehicle_make = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).vehicle_make.to_string();
		vehicle_model = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).vehicle_model.to_string();
		vehicle_regno = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).vehicle_regno.to_string();
		sales_amount_v = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).sales_amount.to_string();
		carpet_size = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).carpet_size.to_string();
		carpet_colour = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).carpet_colour.to_string();
		sales_amount_c = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).sales_amount.to_string();
		
		//let c1 = &String::from("risper muite").to_lowercase();
		
		let vehicle_amount = 
		match sales_amount_v.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
		
		let carpet_amount = 
		match sales_amount_c.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
			  
		if carpet_size.replace(" ","").len() > 0 && carpet_colour.replace(" ","").len() > 0 {
			is_valid_carpet_data = true;
		}
		
		if vehicle_make.replace(" ","").len() > 0 && vehicle_regno.replace(" ","").len() > 0 {
			is_valid_vehicle_data = true;
		}
		
		if is_valid_carpet_data {
			//Assign values to struct variable
			let sales_data_1 = SalesDataTable { batch_no: batch_no, cleaning_service: String::from("carpet"), carpet_size: carpet_size, carpet_colour: carpet_colour, 
			  vehicle_make: String::from(""), vehicle_model: String::from(""), vehicle_regno: String::from(""), interior_cleaning: false, exterior_cleaning: false, engine_cleaning: false, undercarriage_cleaning: false,
			  sales_amount: carpet_amount };
			  
			  sales_data_table.push(sales_data_1);
		}
		
		if is_valid_vehicle_data {
			//Assign values to struct variable
			let sales_data_2 = SalesDataTable { batch_no: batch_no, cleaning_service: String::from("vehicle"), carpet_size: String::from(""), carpet_colour: String::from(""), 
			  vehicle_make: vehicle_make, vehicle_model: vehicle_model, vehicle_regno: vehicle_regno, interior_cleaning: true, exterior_cleaning: false, engine_cleaning: true, undercarriage_cleaning: false,
			  sales_amount: vehicle_amount };
			  
			  sales_data_table.push(sales_data_2);
		}

	}
	
	sales_data_table
}

fn get_history_sales_batch_data(data: &web::Data<Pool>) -> HistorySalesBatchResponseData  {
	let mut vec_history_sales_batch_data = Vec::new();
	
	match data
        .get_conn()
		.and_then(|mut conn| select_incoming_sales_batch_data_requests(&mut conn))
    {
        Ok(s) => {
            //println!("Successful to open DB connection."),
			//println!("Successful insert to DB connection. {:?}", sales_batch_id);
			vec_history_sales_batch_data = s;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	//Assign values to struct variable
	let output_data = HistorySalesBatchResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), sales_batch_data: vec_history_sales_batch_data };
	
	output_data
}
/*
fn get_history_sales_batch_data_old(data: web::Data<Pool>) -> HistorySalesBatchResponseData  {
	let mut vec_history_sales_batch_data = Vec::new();
	let k: i32 = 0;
	
	let conn = get_database_connection(data);
	let sales_batch_data = select_incoming_sales_batch_data_requests(conn);
	
	for sales_data in sales_batch_data.iter() {
		let cust_name = sales_data.cust_name.to_string();
		let mobile_no = sales_data.mobile_no.to_string();
		
		let batch_no = sales_data.batch_no.as_ref().unwrap_or(&k);
		
		let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
		let carpet_sales_data = select_incoming_carpet_sales_data_requests(conn, batch_no);
		let vehicle_sales_data = select_incoming_vehicle_sales_data_requests(conn, batch_no);
		
		let history_sales_response_data = HistorySalesResponseData {customer_sales_data: customer_sales_data, carpet_sales_data: carpet_sales_data, vehicle_sales_data: vehicle_sales_data };
				
		let history_sales_batch_data = HistorySalesBatchData {batch_no: batch_no.to_string(), sales_data: history_sales_response_data };
		
		
		vec_history_sales_batch_data.push(history_sales_batch_data);

	}
	
	//Assign values to struct variable
	let output_data = HistorySalesBatchResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), sales_batch_data: vec_history_sales_batch_data };
	
	output_data
}
*/
fn get_history_search_sales_batch_data(search_data: &String,
    is_mobile_no: &bool, is_customer_name: &bool, is_vehicle_regno: &bool, 
	data: &web::Data<Pool>) -> HistorySalesBatchResponseData  {
	let mut vec_history_sales_batch_data = Vec::new();
	
	match data
        .get_conn()
		.and_then(|mut conn| select_incoming_search_sales_batch_data_requests(search_data, is_mobile_no, is_customer_name, is_vehicle_regno, &mut conn))
    {
        Ok(s) => {
			vec_history_sales_batch_data = s;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	//Assign values to struct variable
	let output_data = HistorySalesBatchResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), sales_batch_data: vec_history_sales_batch_data };
	
	output_data
}
/*
fn get_database_connection(data: web::Data<Pool>) -> &'static mut PooledConn {
	
	let mut conn: PooledConn;
	match data
        .get_conn()
		//.and_then(|mut conn| &mut conn)
    {
        Ok(c) => {
            conn = c;
        },
		Err(e) => {
            println!("Failed to open DB connection. {:?}", e);
        },
    };
	
	&conn
	
	//let mut conn = data.get_conn()?;
	
}
*/

#[actix_web::main]
async fn main() {
	//async fn main() -> std::io::Result<()> {
	/*
    HttpServer::new(|| {
        App::new()
		    .app_data(shared_data.clone())
		    .service(hello_world)
            .service(current_temperature)
			.service(get_person)
			.service(get_vehicle_make_data)
			.service(get_vehicle_model_data)
			.service(get_carpet_type_size_data)
			.service(get_carpet_type_colour_data)
			.service(get_vehicle_cleaning_type_cost_data)
			.service(get_carpet_cleaning_type_cost_data)
			.service(add_sales_data)
			.service(get_all_sales_data)
			.service(get_search_sales_data)
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    //.bind("127.0.0.1:8080")?
	//.bind("192.168.3.22:9247")?
	//.bind("127.0.0.1:9247")? //accessible from the machine only
	.bind("0.0.0.0:9247")? //accessible from outside the machine itself
    .run()
    .await
	*/
	let url = get_conn_url();
     
    let pool = match Pool::new(url) {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to open DB connection. {:?}", e); return;
        }
    };
 
    let shared_data = web::Data::new(pool);
	
	let server = match HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
		    .service(hello_world)
            .service(current_temperature)
			.service(get_person)
			.service(get_vehicle_make_data)
			.service(get_vehicle_model_data)
			.service(get_carpet_type_size_data)
			.service(get_carpet_type_colour_data)
			.service(get_vehicle_cleaning_type_cost_data)
			.service(get_carpet_cleaning_type_cost_data)
			.service(add_sales_data)
			.service(get_all_sales_data)
			.service(get_search_sales_data)
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    }).bind("0.0.0.0:9247") {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to bind port. {:?}", e);
            return;
        }
    };
 
    match server.run().await {
        Ok(_) => println!("Server exited normally."),
        Err(e) => println!("Server exited with error: {:?}", e),
    };
}