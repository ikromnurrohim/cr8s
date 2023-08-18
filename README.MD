# ROCKET WEB FREMWORK ADVANCE

In this web application is using `postgresql` `redis` and of course `rocket` and all of application is build on docker, specific configuration can be look at file `docker-compose.yml` and this `rocket` application can change live code, then also sync that change on docker image, that mean we don't need to rebuild our images, and for that the configuration is store in `Dockerfile` in this line 
```
CMD ["cargo", "watch", "--why", "--", "echo"]
```

## Database
we have two table, crates and rustaceans which both have file type `date` and that we must add `chrono` in our dependencies for type implement `date` using `NaiveDateTime` from chrono in our models.

and if we have field in our table with null value is able, then we must set that field on our models as an `Option` type, \
sample
`pub description: Option<String>`

Db Connection
since I use docker compose, I cannot pass url for database connection in file Rocket.toml
but if I use monolith application, rocket will can read database url from file Rocket.toml
so how to pass database url when I use docker ?
the configuration can be place in `environment` in file `docker-compose.yml`
sample:
```
ROCKET_DATABASES={postgres={url=postgres://postgres:postgres@postgres/app_db}}
```

then we can use database connection to key `postgres` that I have been defined before in file `docker-compose.yml`

to use that we can code like this
```
#[rocket_sync_db_pools::databas("postgres")]
pub struct DbConn(PgConnection);
```

if I want to make sure the connection with database is OK before launch, if NOT OK then rocket will panic and not launch, I just need configure that on rocket build, before calling function `launch()` 
so sample code like this:
```
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            rocket_routes::rustaceans::get_rustaceans,
            rocket_routes::rustaceans::view_rustacean,
            rocket_routes::rustaceans::create_rustacean,
            rocket_routes::rustaceans::update_rustacean,
            rocket_routes::rustaceans::delete_rustacean,
        ])
        .attach(DbConn::fairing()) <-- this line
        .launch()
        .await;
}
```
