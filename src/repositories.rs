use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncPgConnection};
use crate::models::*;
use crate::schema::*;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        // why QueryResult using vec ? because this will return multiple rustaceans
        rustaceans::table.limit(limit).load(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(c).await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(c).await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c).await
    }
}



pub struct CrateRepository;

impl CrateRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        // why QueryResult using vec ? because this will return multiple crates
        crates::table.limit(limit).load(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(c).await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, a_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::rustacean_id.eq(a_crate.rustacean_id),
                crates::code.eq(a_crate.code),
                crates::name.eq(a_crate.name),
                crates::version.eq(a_crate.version),
                crates::description.eq(a_crate.description),
            ))
            .get_result(c).await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(c).await
    }
}

pub struct UserRepository;

impl UserRepository {
    pub async fn find_with_roles(c: &mut AsyncPgConnection) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load(c).await?;
        let result = users_roles::table.inner_join(roles::table)
            .load::<(UserRole, Role)>(c).await?
            .grouped_by(&users);
        Ok(users.into_iter().zip(result).collect())
    }

    pub async fn create(c: &mut AsyncPgConnection, new_user: NewUser, role_codes: Vec<String>) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(c).await?;
        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(c, role_code.to_owned()).await {
                    NewUserRole {user_id: user.id, role_id: role.id}
                } else {
                    let new_role = NewRole { name: role_code.clone(), code: role_code.to_owned().parse().unwrap() };
                    let role = RoleRepository::create(c, new_role).await?;
                    NewUserRole {user_id: user.id, role_id: role.id}
                }
            };
            diesel::insert_into(users_roles::table)
                    .values(new_user_role)
                    .get_result::<UserRole>(c).await?;
        }

        Ok(user)
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        // deleting user_roles first before deleting user, because we cannot delete if the forignkey it's still use on another table 
        diesel::delete(
            users_roles::table.filter(users_roles::user_id.eq(id))
        ).execute(c).await?;

        diesel::delete(users::table.find(id)).execute(c).await
    }
}

pub struct RoleRepository;

impl RoleRepository {
    pub async fn find_by_code(c: &mut AsyncPgConnection, code: String) -> QueryResult<Role> {
        // if the return query result is single, then use first()
        roles::table.filter(roles::code.eq(code)).first(c).await
    }

    pub async fn find_by_ids(c: &mut AsyncPgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        // if the return query result is many, then use get_results()
        // alse on the function use a vector if we want to return many.
        roles::table.filter(roles::id.eq_any(ids)).get_results(c).await
    }

    pub async fn find_by_user(c: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>> {
        // if we use table relationship, then we use belonging_to then in the models on both table 
        // must implemented derive Identifiable
        let user_roles = UserRole::belonging_to(&user).get_results(c).await?;

        let role_ids = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        Self::find_by_ids(c, role_ids).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result::<Role>(c).await
    }
}