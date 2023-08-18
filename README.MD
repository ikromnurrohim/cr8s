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