mod models;
mod persistence;
mod routes;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use mysql::*;
use std::env;

fn get_conn_builder(
    db_user: String,
    db_password: String,
    db_host: String,
    db_port: u16,
    db_name: String,
) -> OptsBuilder {
    let builder = OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password));
    builder
}

#[actix_web::main]
async fn main() {
    // get env vars
    dotenv().ok();
    let server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR is not set in .env file");
    let db_user = env::var("MYSQL_USER").expect("MYSQL_USER is not set in .env file");
    let db_password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set in .env file");
    let db_host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set in .env file");
    let my_db_port = env::var("MYSQL_PORT").expect("MYSQL_PORT is not set in .env file");
    let db_name = env::var("MYSQL_DBNAME").expect("MYSQL_DBNAME is not set in .env file");
    let mut http_server_status = String::from("[info] ActixWebHttpServer - Listening for HTTP on ");
    let db_port: u16 = match my_db_port.parse::<u16>() {
        Ok(a) => a,
        Err(e) => 3306, // default mysql server port
    };

    http_server_status.push_str(&server_addr);

    let builder: OptsBuilder = get_conn_builder(db_user, db_password, db_host, db_port, db_name);
    let pool = match Pool::new(builder) {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to open DB connection. {:?}", e);
            return;
        }
    };

    let shared_data = web::Data::new(pool);

    let server = match HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(routes::index)
            .service(routes::add_sales_data)
            .service(routes::get_vehicle_make_data)
            .service(routes::get_vehicle_model_data)
            .service(routes::get_carpet_type_size_data)
            .service(routes::get_carpet_type_colour_data)
            .service(routes::get_vehicle_cleaning_type_cost_data)
            .service(routes::get_carpet_cleaning_type_cost_data)
            .service(routes::get_all_sales_data)
            .service(routes::get_search_sales_data)
            .service(routes::get_all_employees_data)
            .service(routes::get_all_sales_commission_data)
            .service(routes::get_search_sales_commission_data)
    })
    .bind(server_addr)
    {
        Ok(s) => {
            println!("{:?}", http_server_status);
            s
        }
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
