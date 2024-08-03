pub mod rustaceans;
pub mod crates;
mod authorization;

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}