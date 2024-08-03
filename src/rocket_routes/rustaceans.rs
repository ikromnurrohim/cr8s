use rocket::serde::json::{Json, serde_json::json, Value};
use rocket::response::status::{Custom, NoContent};
use rocket::http::Status;
use rocket_db_pools::Connection;
use crate::models::{NewRustacean, Rustacean};
use crate::rocket_routes::{DbConn, server_error};
use crate::repositories::RustaceanRepository;


#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_multiple(&mut db, 100).await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|e|server_error(e.into()))
}

#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(id: i32, mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find(&mut db, id).await
        .map(|rustacean| json!(rustacean))
        .map_err(|e| match e {
            // if match error not found this line will execute
            diesel::result::Error::NotFound => Custom(Status::NotFound, json!("Not Found")),
            // otherwise thi line will execute
            _ => server_error(e.into())
        })
}

#[rocket::post("/rustaceans", format="json", data="<new_rustacean>")]
pub async fn create_rustacean(new_rustacean: Json<NewRustacean>, mut db: Connection<DbConn>) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::create(&mut db, new_rustacean.into_inner()).await
        // since new_rustacean is wrap using json, that we need to unwrap new_rustacean using into.inner()
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(|e|server_error(e.into()))
}

#[rocket::put("/rustaceans/<id>", format="json", data="<rustacean>")]
pub async fn update_rustacean(id: i32, rustacean: Json<Rustacean>, mut db: Connection<DbConn>) -> Result<Value, Custom<Value>>{
    RustaceanRepository::update(&mut db, id, rustacean.into_inner()).await
        .map(|rustacean| json!(rustacean))
        .map_err(|e|server_error(e.into()))
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(id: i32, mut db: Connection<DbConn>) -> Result<NoContent, Custom<Value>> {
    RustaceanRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|e|server_error(e.into()))
}