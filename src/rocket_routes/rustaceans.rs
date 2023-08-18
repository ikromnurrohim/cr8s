use rocket::serde::json::{Json, serde_json::json, Value};

use crate::models::{NewRustacean, Rustacean};

#[rocket::get("/rustaceans")]
pub fn get_rustaceans() -> Value {
    json!({"message": "hello duni"})
}

#[rocket::get("/rustaceans/<id>")]
pub fn view_rustacean(id: i32) {

}

#[rocket::post("/rustaceans", format="json", data="<new_rustacean>")]
pub fn create_rustacean(new_rustacean: Json<NewRustacean>) {

}

#[rocket::put("/rustacean/<id>", format="json", data="<rustacean>")]
pub fn update_rustacean(id: i32, rustacean: Json<Rustacean>) {

}

#[rocket::delete("/rustacean/<id>")]
pub fn delete_rustacean(id: i32) {

}