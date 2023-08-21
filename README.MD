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

## Test
> In here I make Testing for all endpoint and it's will called `Integration testing` 
> means checking if different modules are working fine when combined together as a group.
> So I just check those endpoint are work well with input and desire output.
so I want to make sure if any change in my code then it not effect to existing code, to make sure that I must be test each one endpoint, so it not a good idea to test manual each endpoint, so I use automate test to execute test automatically.
so i need to create folder called `tests` and file `rustaceans.rs` to write test to rustacean endpoint, after that i just need to run this command `cargo test` this command will find folder `tests` inside main folder and pick up any file inside that folder then execute them.


## Logging
This application using log, not from rocket. why it's so ?
because if in the future we want to change webframework with other like actix, we don't need much code refactoring.
this log is currently use if the communication with database is error then rocket will logging that.
```
pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}
```
this function using Box<> parameter for using as globaly, so if I using Box<> I can reuse this not only for error log database but also the other type error.
sample using log in the endpoint.
```
.map_err(|e|server_error(e.into()))
```
in that code I use `into()` to convert to whatever I want it, in this case I want to convert to `Box<>`