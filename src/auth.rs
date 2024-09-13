use argon2::password_hash::Error;
use crate::models::User;
use argon2::{PasswordVerifier, PasswordHash};
use rand::distributions::Alphanumeric;
use rand::Rng;
use argon2::{password_hash::{SaltString, rand_core::OsRng}, PasswordHasher};

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn authorize_user(user: &User, credentials: Credentials) -> Result<String, Error> {
    let argon2 = argon2::Argon2::default();
    let db_hash = PasswordHash::new(&user.password)?;
    argon2.verify_password(credentials.password.as_bytes(), &db_hash)?;
    let session_id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    Ok(session_id)
}

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);
    let argon = argon2::Argon2::default();
    let hashed_password = argon.hash_password(password.as_bytes(), &salt)?;
    Ok(hashed_password.to_string())
}