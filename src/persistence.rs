extern crate base64;

use actix_web::{web, HttpRequest};
use mysql::prelude::*;
use mysql::*;
use crate::{
    models::{
        CarpetSalesData,
        ClientApiResponseDetails, EmployeeRegisteredDetails,
        EmployeesRegisteredResponseData, HistoryCarpetSalesData, HistoryCustomerSalesData,
        HistorySalesBatchData, HistorySalesBatchResponseData,
        HistorySalesResponseData, HistoryVehicleSalesData, ProcessingStatus,
        SalesBatchDataTable, SalesCommissionDetails,
        SalesCommissionResponseData, SalesData, SalesDataTable,  VehicleSalesData,
    },
};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use std::str;

pub fn get_location() -> String {
    let local_name = String::from("Dandora");
    local_name
}

pub fn get_carpet_sales_data_1() -> HistoryCarpetSalesData {
	let carpet_size: String = String::from("6 by 9");
	let carpet_colour: String = String::from("PURPLE");
	let carpet_sales_amount = 120;
	let carpet_payment_mode: String = String::from("m-pesa");
	let carpet_transaction_date: String = String::from("10-03-2021, 07:29 pm");
	let carpet_sales_data = HistoryCarpetSalesData { carpet_size: carpet_size, carpet_colour: carpet_colour, sales_amount: carpet_sales_amount, payment_mode: carpet_payment_mode, transaction_date: carpet_transaction_date };
	carpet_sales_data
}
pub fn get_carpet_sales_data_2() -> HistoryCarpetSalesData {
	let carpet_size: String = String::from("5 by 8");
	let carpet_colour: String = String::from("BLUE");
	let carpet_sales_amount = 130;
	let carpet_payment_mode: String = String::from("cash");
	let carpet_transaction_date: String = String::from("12-03-2021, 02:15 pm");
	let carpet_sales_data = HistoryCarpetSalesData { carpet_size: carpet_size, carpet_colour: carpet_colour, sales_amount: carpet_sales_amount, payment_mode: carpet_payment_mode, transaction_date: carpet_transaction_date };
	carpet_sales_data
}
pub fn get_vehicle_sales_data_1() -> HistoryVehicleSalesData {
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
pub fn get_vehicle_sales_data_2() -> HistoryVehicleSalesData {
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
pub fn get_customer_sales_data_1() -> HistoryCustomerSalesData {
	let cust_name: String = String::from("nicole");
	let mobile_no: String = String::from("254723083761");
	let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
	customer_sales_data
}
pub fn get_customer_sales_data_2() -> HistoryCustomerSalesData {
	let cust_name: String = String::from("paul");
	let mobile_no: String = String::from("254723083760");
	let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
	customer_sales_data
}

pub fn create_sales_batch_data(data: &web::Data<Pool>, sales_batch_data: SalesBatchDataTable) -> i32  {
	let mut batch_no: i32 = 0;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_sales_batch_data(&mut conn, sales_batch_data))
    {
        Ok(sales_batch_no) => {
        
			batch_no = sales_batch_no as i32;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	batch_no
}

pub fn create_sales_data(data: &web::Data<Pool>, sales_data: Vec<SalesDataTable>) -> bool {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_sales_data(&mut conn, sales_data))
    {
        Ok(sales_no) => {

			successful = true;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	successful
}

fn insert_sales_batch_data(
    conn: &mut PooledConn, sales_batch_data: SalesBatchDataTable) -> std::result::Result<u64, mysql::error::Error> {
	

	
	// Now let's insert sales batch data to the database
	//let my_result =
	conn.exec_drop(
        "insert into incomingsalesbatchdatarequests (cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode) values (:cust_name, :mobile_no, :cleaning_service, :sales_amount, :paid_amount, :payment_mode);",
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

}

fn insert_sales_data(
    conn: &mut PooledConn, sales_data: Vec<SalesDataTable>) -> std::result::Result<u64, mysql::error::Error> {
	
	// Now let's insert sales data to the database
	conn.exec_batch(
		r"insert into incomingsalesdatarequests (batch_no, cleaning_service, carpet_size, carpet_colour, vehicle_make, vehicle_model, vehicle_regno, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, sales_amount, employee_id, employee_full_names)
		  values (:batch_no, :cleaning_service, :carpet_size, :carpet_colour, :vehicle_make, :vehicle_model, :vehicle_regno, :interior_cleaning, :exterior_cleaning, :engine_cleaning, :undercarriage_cleaning, :sales_amount, :employee_id, :employee_full_names);",
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
			"employee_id" => s.employee_id,
			"employee_full_names" => &s.employee_full_names,
		})
	)
	.and_then(|_| Ok(1))
	
}

fn insert_sales_commission_data(
    conn: &mut PooledConn, batch_no: i32) -> std::result::Result<u64, mysql::error::Error> {

	
	// Now let's insert sales commission data to the database
	//"call insertsalescommissiondetails (:mybatch_no, :myemployee_id, :myemployee_full_names);",
	conn.exec_drop(
        "call insertsalescommissiondetails (:mybatch_no);",
        params! {
            "mybatch_no" => batch_no,
            
        },
    )
	.and_then(|_| Ok(1))
}

pub fn create_sales_commission_data(data: web::Data<Pool>, batch_no: i32) -> bool {
	let mut successful: bool = false;
	
	match data
        .get_conn()
		.and_then(|mut conn| insert_sales_commission_data(&mut conn, batch_no))
    {
        Ok(sales_no) => {
			successful = true;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	successful
}

fn select_incoming_sales_batch_data_requests(
    conn: &mut PooledConn) -> std::result::Result<Vec<HistorySalesBatchData>, mysql::error::Error> {
	let mut sales_batch_data = Vec::new();
	
	
    conn.query_map(
        "select batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode from incomingsalesbatchdatarequests order by batch_no desc limit 10;",
        |(batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode)| {
            let a = SalesBatchDataTable { batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode };
			sales_batch_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
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

	
	//(*) is the dereferencing operator
	//We use it to get the actual value at the address of variable is_vehicle_regno
	let is_regno = *is_vehicle_regno;
	
	if !is_regno {
		conn.exec_map(
		
		"select batch_no, cust_name, mobile_no, cleaning_service, sales_amount, paid_amount, payment_mode from incomingsalesbatchdatarequests where (case when :is_mobile_no = 1 then mobile_no = :search_data else lower(replace(coalesce(cust_name,''), ' ', '')) = :search_data end) order by batch_no desc limit 10;",
		params! {
				"search_data" => search_data,
				"is_mobile_no" => is_mobile_no,
			
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
	}
	else{

		
		conn.exec_map(
		
		"select a.batch_no, a.cust_name, a.mobile_no, a.cleaning_service, a.sales_amount, a.paid_amount, a.payment_mode from incomingsalesbatchdatarequests a inner join incomingsalesdatarequests b on a.batch_no = b.batch_no where lower(replace(coalesce(b.vehicle_regno,''), ' ', '')) = :search_data order by a.batch_no asc limit 10;",
		params! {
				"search_data" => search_data,
				//"cleaning_service" => cleaning_service,
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
	}
	

	
	let mut vec_history_sales_batch_data = Vec::new();
	let k: i32 = 0;
	
	for sales_data in sales_batch_data.iter() {
		let cust_name = sales_data.cust_name.to_string();
		let mobile_no = sales_data.mobile_no.to_string();
		
		let batch_no = sales_data.batch_no.as_ref().unwrap_or(&k);

		let customer_sales_data = HistoryCustomerSalesData { cust_name: cust_name, mobile_no: mobile_no };
		
		let carpet_sales_data =
		if is_regno {
			//if client searched for vehicle, don't show carpet data
			Vec::new()
		}
		else {
			select_incoming_carpet_sales_data_requests(conn, batch_no)
		};
		
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
	

	
    conn.exec_map(
    "select carpet_size, carpet_colour, sales_amount, date_format(transaction_date, '%d-%m-%Y') transaction_date from incomingsalesdatarequests where batch_no = :batch_no and cleaning_service = :cleaning_service;",
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
	
	
    conn.exec_map(
    "select vehicle_make, vehicle_model, vehicle_regno, sales_amount, interior_cleaning, exterior_cleaning, engine_cleaning, undercarriage_cleaning, date_format(transaction_date, '%d-%m-%Y') transaction_date from incomingsalesdatarequests where batch_no = :batch_no and cleaning_service = :cleaning_service;",
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

pub fn get_sales_batch_data(sales_batch_data: &Vec<SalesData>) -> SalesBatchDataTable  {
	let mut sales_batch_data_table = SalesBatchDataTable { batch_no: None, cust_name: String::from(""), mobile_no: String::from(""), cleaning_service: String::from(""), sales_amount: 0, paid_amount: 0, payment_mode: String::from("") };
	
	let mut cust_name = String::from("");
	let mut mobile_no = String::from("");
	let mut sales_amount = 0;
	let mut paid_amount = 0;
	let mut sales_amount_s = String::from("");
	let mut paid_amount_s = String::from("");
	let mut payment_mode = String::from("");

	let vehicle_sales_data = VehicleSalesData { vehicle_make: String::from(""), vehicle_model: String::from(""), vehicle_regno: String::from(""), sales_amount: String::from(""), payment_mode: String::from(""), interior_cleaning: false, exterior_cleaning: false, engine_cleaning: false, undercarriage_cleaning: false, employee_id: 0, employee_full_names: String::from("") };
	let carpet_sales_data = CarpetSalesData { carpet_size: String::from(""), carpet_colour: String::from(""), sales_amount: String::from(""), payment_mode: String::from(""), employee_id: 0, employee_full_names: String::from("") };
	
	for sales_data in sales_batch_data.iter() {
		cust_name = sales_data.customer_sales_data.cust_name.to_string();
		mobile_no = sales_data.customer_sales_data.mobile_no.to_string();
		payment_mode = sales_data.customer_sales_data.payment_mode.to_string();
		sales_amount_s = sales_data.customer_sales_data.sales_amount.to_string();
		paid_amount_s = sales_data.customer_sales_data.paid_amount.to_string();

		let sales_amount = 
		match sales_amount_s.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
		
		let paid_amount = 
		match paid_amount_s.parse::<i32>() {
		  Ok(a) => a,
		  Err(e) => 0,
		};
		//Assign values to struct variable
		sales_batch_data_table = SalesBatchDataTable { batch_no: None, cust_name: cust_name, mobile_no: mobile_no, cleaning_service: String::from(""), sales_amount: sales_amount, paid_amount: paid_amount, payment_mode: payment_mode };

	}
	
	sales_batch_data_table
}

fn select_employees_registered_details_requests(
    conn: &mut PooledConn) -> std::result::Result<Vec<EmployeeRegisteredDetails>, mysql::error::Error> {
	let mut employees_registered_data = Vec::new();
	
    conn.query_map(
        "select id,full_names from employeesregistereddetails where employee_type_code = 1 and activated = 1 and duplicate_entry = 0 and deleted = 0 order by full_names asc;",
        |(id, full_names)| {
            let a = EmployeeRegisteredDetails { id, full_names };
			employees_registered_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	Ok(employees_registered_data)
	
}

fn select_sales_commission_details_requests(
    conn: &mut PooledConn) -> std::result::Result<Vec<SalesCommissionDetails>, mysql::error::Error> {
	let mut sales_commission_data = Vec::new();
	
    conn.query_map(
        "select batch_no, cleaning_service, cleaning_service_type, cleaning_amount, commission_percentage, commission_amount, employee_full_names, date_format(transaction_date, '%d-%m-%Y') transaction_date from salescommissiondata order by id asc;",
        |(batch_no, cleaning_service, cleaning_service_type, cleaning_amount, commission_percentage, commission_amount, employee_full_names, transaction_date)| {
            let a = SalesCommissionDetails { batch_no, cleaning_service, cleaning_service_type, cleaning_amount, commission_percentage, commission_amount, employee_full_names, transaction_date };
			sales_commission_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	Ok(sales_commission_data)
	
}

fn select_search_sales_commission_details_requests(search_data: &String,
    is_employee_id: &bool, is_employee_full_names: &bool, conn: &mut PooledConn) -> std::result::Result<Vec<SalesCommissionDetails>, mysql::error::Error> {
	let mut sales_commission_data = Vec::new();
	
    conn.exec_map(
        "select batch_no, cleaning_service, cleaning_service_type, cleaning_amount, commission_percentage, commission_amount, employee_full_names, date_format(transaction_date, '%d-%m-%Y') transaction_date from salescommissiondata where (case when :is_employee_id = 1 then employee_id = :search_data else employee_full_names = :search_data end) order by id asc;",
		params! {
			"search_data" => search_data,
			"is_employee_id" => is_employee_id,
		},
        |(batch_no, cleaning_service, cleaning_service_type, cleaning_amount, commission_percentage, commission_amount, employee_full_names, transaction_date)| {
            let a = SalesCommissionDetails { batch_no, cleaning_service, cleaning_service_type, cleaning_amount, commission_percentage, commission_amount, employee_full_names, transaction_date };
			sales_commission_data.push(a);
        },
    )
	.and_then(|_| Ok(1));
	
	Ok(sales_commission_data)
	
}

pub fn get_sales_data(sales_batch_data: &Vec<SalesData>, batch_no: i32) -> Vec<SalesDataTable>  {
	let mut sales_data_table = Vec::new();
	let mut vehicle_make = String::from("");
	let mut vehicle_model = String::from("");
	let mut vehicle_regno = String::from("");
	let mut sales_amount_v = String::from("");
	let mut sales_amount_c = String::from("");
	let mut carpet_size = String::from("");
	let mut carpet_colour = String::from("");
	let mut interior_cleaning: bool = false;
	let mut exterior_cleaning: bool = false;
	let mut engine_cleaning: bool = false;
	let mut undercarriage_cleaning: bool = false;
	let mut employee_id_vehicle: i32 = 0;
	let mut employee_full_names_vehicle: String = String::from("");
	let mut employee_id_carpet: i32 = 0;
	let mut employee_full_names_carpet: String = String::from("");
	let vehicle_sales_data = VehicleSalesData { vehicle_make: String::from(""), vehicle_model: String::from(""), vehicle_regno: String::from(""), sales_amount: String::from(""), payment_mode: String::from(""), interior_cleaning: false, exterior_cleaning: false, engine_cleaning: false, undercarriage_cleaning: false, employee_id: 0, employee_full_names: String::from("") };
	let carpet_sales_data = CarpetSalesData { carpet_size: String::from(""), carpet_colour: String::from(""), sales_amount: String::from(""), payment_mode: String::from(""), employee_id: 0, employee_full_names: String::from("") };
	let mut is_valid_vehicle_data: bool = false;
	let mut is_valid_carpet_data: bool = false;
	
	for sales_data in sales_batch_data.iter() {
		is_valid_vehicle_data = false;
		is_valid_carpet_data = false;
		
		interior_cleaning = false;
		exterior_cleaning = false;
		engine_cleaning = false;
		undercarriage_cleaning = false;
		
		employee_id_vehicle = 0;
		employee_full_names_vehicle = String::from("");
		employee_id_carpet = 0;
		employee_full_names_carpet = String::from("");
		
		vehicle_make = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).vehicle_make.to_string();
		vehicle_model = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).vehicle_model.to_string();
		vehicle_regno = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).vehicle_regno.to_string();
		sales_amount_v = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).sales_amount.to_string();
		carpet_size = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).carpet_size.to_string();
		carpet_colour = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).carpet_colour.to_string();
		sales_amount_c = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).sales_amount.to_string();
		employee_id_carpet = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).employee_id;
		employee_full_names_carpet = sales_data.carpet_sales_data.as_ref().unwrap_or(&carpet_sales_data).employee_full_names.to_string();
		
		interior_cleaning = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).interior_cleaning;
		exterior_cleaning = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).exterior_cleaning;
		engine_cleaning = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).engine_cleaning;
		undercarriage_cleaning = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).undercarriage_cleaning;
		employee_id_vehicle = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).employee_id;
		employee_full_names_vehicle = sales_data.vehicle_sales_data.as_ref().unwrap_or(&vehicle_sales_data).employee_full_names.to_string();
		
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
			  sales_amount: carpet_amount, employee_id: employee_id_carpet, employee_full_names: employee_full_names_carpet };
			  
			  sales_data_table.push(sales_data_1);
		}
		
		if is_valid_vehicle_data {
			//Assign values to struct variable
			let sales_data_2 = SalesDataTable { batch_no: batch_no, cleaning_service: String::from("vehicle"), carpet_size: String::from(""), carpet_colour: String::from(""), 
			  vehicle_make: vehicle_make, vehicle_model: vehicle_model, vehicle_regno: vehicle_regno, interior_cleaning: interior_cleaning, exterior_cleaning: exterior_cleaning, engine_cleaning: engine_cleaning, undercarriage_cleaning: undercarriage_cleaning,
			  sales_amount: vehicle_amount, employee_id: employee_id_vehicle, employee_full_names: employee_full_names_vehicle };
			  
			  sales_data_table.push(sales_data_2);
		}

	}
	
	sales_data_table
}

pub fn get_history_sales_batch_data(data: &web::Data<Pool>) -> HistorySalesBatchResponseData  {
	let mut vec_history_sales_batch_data = Vec::new();
	
	match data
        .get_conn()
		.and_then(|mut conn| select_incoming_sales_batch_data_requests(&mut conn))
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

pub fn get_history_search_sales_batch_data(search_data: &String,
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

pub fn get_employees_registered_data(data: &web::Data<Pool>) -> EmployeesRegisteredResponseData  {
	let mut vec_employees_registered_data = Vec::new();
	
	match data
        .get_conn()
		.and_then(|mut conn| select_employees_registered_details_requests(&mut conn))
    {
        Ok(s) => {
			vec_employees_registered_data = s;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	//Assign values to struct variable
	let output_data = EmployeesRegisteredResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), employees_data: vec_employees_registered_data };
	
	output_data
}

pub fn get_sales_commission_data(data: &web::Data<Pool>) -> SalesCommissionResponseData  {
	let mut vec_sales_commission_data = Vec::new();
	
	match data
        .get_conn()
		.and_then(|mut conn| select_sales_commission_details_requests(&mut conn))
    {
        Ok(s) => {
			vec_sales_commission_data = s;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	//Assign values to struct variable
	let output_data = SalesCommissionResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), sales_commission_data: vec_sales_commission_data };
	
	output_data
}

pub fn get_search_entry_sales_commission_data(search_data: &String,
    is_employee_id: &bool, is_employee_full_names: &bool, 
	data: &web::Data<Pool>) -> SalesCommissionResponseData  {
	let mut vec_sales_commission_data = Vec::new();
	
	match data
        .get_conn()
		.and_then(|mut conn| select_search_sales_commission_details_requests(search_data, is_employee_id, is_employee_full_names, &mut conn))
    {
        Ok(s) => {
			vec_sales_commission_data = s;
        },
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }
	
	//Assign values to struct variable
	let output_data = SalesCommissionResponseData {status_code: ProcessingStatus::Zero as u32, status_description: String::from("Successful"), sales_commission_data: vec_sales_commission_data };
	
	output_data
}

pub fn validate_client_api(req: HttpRequest, api_function: String) -> ClientApiResponseDetails  {
	
	let mut client_ip = String::from("");
	let mut authorization = String::from("");
	let mut channel_type = String::from("");
	let mut app_ver_code = String::from("");
	let mut app_id_tok = String::from("");
	let mut dev_id = String::from("");
	let mut dev_tok_regno = String::from("");
	let mut auth_token = String::from("");
	let mut user_name = String::from("");
	let mut pass_word = String::from("");
	let mut status_description = String::from("Error occured during processing, please try again.");
	let mut status_code = ProcessingStatus::One as u32;
	
	if !req.headers().is_empty() {
		if let Some(val) = req.peer_addr() {
			client_ip = val.ip().to_string()
		}
		if req.headers().contains_key("authorization") {
			let m = req.headers().get("authorization").unwrap();
			authorization = m.to_str().unwrap().to_string();
			
			if !authorization.is_empty() {
				if authorization.to_lowercase().contains("bearer") {
					
					let v: Vec<&str> = authorization.split(' ').collect();
					
					let s = v.len();
					if s == 2 {
						auth_token = String::from(v[1]);
					
						let bytes = general_purpose::STANDARD.decode(auth_token).unwrap();
						let m_auth_token = str::from_utf8(&bytes).unwrap().to_string();
						
						if !m_auth_token.is_empty() {
							if m_auth_token.contains(":") {
								let w: Vec<&str> = m_auth_token.split(':').collect();
							
								let t = w.len();
								if t == 2 {
									user_name = String::from(w[0]);
									pass_word = String::from(w[1]);
								}
							}

						}
					}
				}
			}
		}
		if req.headers().contains_key("channeltype") {
			let m = req.headers().get("channeltype").unwrap();
			channel_type = m.to_str().unwrap().to_string();

		}
		if req.headers().contains_key("appvercode") {
			let m = req.headers().get("appvercode").unwrap();
			app_ver_code = m.to_str().unwrap().to_string();

		}
		if req.headers().contains_key("appidtok") {
			let m = req.headers().get("appidtok").unwrap();
			app_id_tok = m.to_str().unwrap().to_string();
		}
		if req.headers().contains_key("devid") {
			let m = req.headers().get("devid").unwrap();
			dev_id = m.to_str().unwrap().to_string();
		}
		if req.headers().contains_key("devtokregno") {
			let m = req.headers().get("devtokregno").unwrap();
			dev_tok_regno = m.to_str().unwrap().to_string();
		}
	}
	
	if client_ip.len() > 0 && channel_type.len() > 0 && user_name.len() > 0 && pass_word.len() > 0 && api_function.len() > 0 {
		if channel_type.to_lowercase().eq(&String::from("mobileapp")) {
			status_code = ProcessingStatus::Zero as u32;
			status_description = String::from("Successful");
		}
	}
	
	//Assign values to struct variable
	let output_data = ClientApiResponseDetails {status_code: status_code, status_description: status_description };
	
	output_data
}