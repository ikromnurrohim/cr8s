use rocket::serde::json::{Json, serde_json::json, Value};
use rocket::response::status::{Custom, NoContent};
use rocket::http::Status;

use crate::models::{NewRustacean, Rustacean};
use crate::rocket_routes::DbConn;
use crate::repositories::RustaceanRepository;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c|{
        RustaceanRepository::find_multiple(c, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    }).await
}

#[rocket::get("/rustacean/<id>")]
pub async fn view_rustacean(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c|{
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    }).await
}

#[rocket::post("/rustacean", format="json", data="<new_rustacean>")]
pub async fn create_rustacean(new_rustacean: Json<NewRustacean>, db: DbConn) -> Result<Custom<Value>, Custom<Value>> {
    db.run(|c|{
        RustaceanRepository::create(c, new_rustacean.into_inner())
            // since new_rustacean is wrap using json, that we need to unwrap new_rustacean using into.inner()
            .map(|rustacean| Custom(Status::Created, json!(rustacean)))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    }).await
}

#[rocket::put("/rustacean/<id>", format="json", data="<rustacean>")]
pub async fn update_rustacean(id: i32, rustacean: Json<Rustacean>, db: DbConn) -> Result<Value, Custom<Value>>{
    db.run(move |c|{
        RustaceanRepository::update(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    }).await
}

#[rocket::delete("/rustacean/<id>")]
pub async fn delete_rustacean(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
    db.run(move |c|{
        RustaceanRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    }).await
}