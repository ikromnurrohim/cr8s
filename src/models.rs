use diesel::{Insertable, Queryable, AsChangeset};
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Rustacean {
    #[serde(skip_deserializing)] // this will skip insert field
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime 
    // since we derive Deserialize this struct Rustacean with field created_at which implement using chrono, compiler will complain
    // chrono by default not enabling feature serde. for that we just need to enable serde feature in chrono in file Cargo.toml
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String
}

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Crate {
    #[serde(skip_deserializing)] // this will skip insert field
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>, //why using Option ? this indicate the field is not null(that mean can be empty)
    #[serde(skip_deserializing)] // this will skip insert field
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>
}