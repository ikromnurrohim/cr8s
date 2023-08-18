use diesel::{Insertable, Queryable, AsChangeset};
use chrono::NaiveDateTime;
use rocket::serde::Deserialize;
use crate::schema::*;

#[derive(Queryable, AsChangeset, Deserialize)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
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

#[derive(Queryable, AsChangeset)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>, //why using Option ? this indicate the field is not null(that mean can be empty)
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