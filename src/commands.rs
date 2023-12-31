use argon2::{password_hash::{SaltString, rand_core::OsRng}, PasswordHasher};
use diesel::{PgConnection, Connection};

use crate::{models::NewUser, repositories::{UserRepository, RoleRepository}};

fn load_db_connection() ->PgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Cannot load database url from env");
    PgConnection::establish(&database_url)
        .expect("Cannot connect to postgres")
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection();

    let salt = SaltString::generate(OsRng);
    let argon = argon2::Argon2::default();
    let password_hash = argon.hash_password(password.as_bytes(), &salt).unwrap();

    let new_user = NewUser{ username, password: password_hash.to_string() };
    let user = UserRepository::create(&mut c, new_user, role_codes).unwrap();
    // models::User and models::Role must be implement derive Debug if we want to print them.
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).unwrap();
    println!("Role assigned {:?}", roles);
}

pub fn list_users() {
    let mut c = load_db_connection();

    let users = UserRepository::find_with_roles(&mut c).unwrap();
    for user in users {
        println!("{:?}", user);
    }
}

pub fn delete_user(id: i32) {
    let mut c = load_db_connection();
    UserRepository::delete(&mut c, id).unwrap();
}