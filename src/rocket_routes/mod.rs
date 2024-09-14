pub mod rustaceans;
pub mod crates;
pub mod authorization;

use rocket::http::Status;
use rocket::request::{self, Request, FromRequest, Outcome};
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use crate::models::User;
use crate::repositories::UserRepository;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);


#[derive(rocket_db_pools::Database)]
#[database("redis")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);


pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}


#[rocket::async_trait]
impl<'r> FromRequest<'r>  for User {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    //     Authorization: Bearer SESSION_ID_128_CHARACTERS_LONG
        let header = req.headers().get_one("Authorization")
            .map(|h| h.split_whitespace().collect::<Vec<&str>>())
            .filter(|h| h.len() == 2 && h[0] == "Bearer");
        if let Some(header_value) = header {
            let mut cache = req.guard::<Connection<CacheConn>>().await
                .expect("couldn't connect to Redis");
            let mut db = req.guard::<Connection<DbConn>>().await
                .expect("couldn't connect to Postgres");
            let result = cache.get::<String, i32>(format!("sessions/{}", header_value[1])).await;
            if let Ok(user_id) = result {
                if let Ok(user) = UserRepository::find(&mut db, user_id).await {
                    return Outcome::Success(user)
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))

    }

}