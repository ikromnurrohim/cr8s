use argon2::{password_hash::{SaltString, rand_core::OsRng}, PasswordHasher};
use diesel_async::{AsyncPgConnection, AsyncConnection};
use crate::models::{NewUser};
use crate::repositories::{UserRepository, RoleRepository};


async fn load_db_connection() ->AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Cannot load database url from env");
    AsyncPgConnection::establish(&database_url).await
        .expect("Cannot connect to Postgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;

    let salt = SaltString::generate(OsRng);
    let argon = argon2::Argon2::default();
    let password_hash = argon.hash_password(password.as_bytes(), &salt).unwrap();

    let new_user = NewUser{ username, password: password_hash.to_string() };
    let user = UserRepository::create(&mut c, new_user, role_codes).await.unwrap();
    // models::User and models::Role must be implement derive Debug if we want to print them.
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).await.unwrap();
    println!("Role assigned {:?}", roles);
}

pub async fn list_users() {
    let mut c = load_db_connection().await;

    let users = UserRepository::find_with_roles(&mut c).await.unwrap();
    for user in users {
        println!("{:?}", user);
    }
}

pub async fn delete_user(id: i32) {
    let mut c = load_db_connection().await;
    UserRepository::delete(&mut c, id).await.unwrap();
}