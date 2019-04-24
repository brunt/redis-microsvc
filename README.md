# Redis Microservice REST API

This is an example of using [Actix-web](https://crates.io/crates/actix-web) and [Redis](http://redis.io) as a restful json datastore.
Internally records are stored in Redis in a Hashmap with a GUID as the key and all record values as emoji-separated values in a binary-safe string.

### Cool things
* Redis is [fast](https://redis.io/topics/benchmarks).
* Actix is [fast](https://www.techempower.com/benchmarks/#section=data-r17&hw=ph&test=plaintext).
* Actix uses an actor model pattern to create an asynchronous interface to Redis.
* There is no garbage collector.
* This API is using a Hashmap in Redis to store data but there are [many other ways](https://redis.io/documentation) Redis can be used.
* This API is using basic HTTP but there are [many other ways](https://github.com/actix/examples) Actix-web can be used

### Running locally
Starting Redis via docker:

`docker run --rm --name some-redis -p 6379:6379 -d redis`

(Optional) Examining/editing in Redis:
`docker exec -it some-redis redis-cli`

Then run the program with `cargo run`

### Deploying

Docker build & run:

`docker build -t redis-microsvc:v0.1.0 .`

`docker run --rm -p 8000:8000 -e REDIS_URL=... redis-microsvc:v0.1.0`

Kubernetes helmcharts WIP

### HTTP endpoints:

POST **/feed** with some request like:
```json
{
  "title": "some title idk",
  "body": "some data to be stored"
}
```

GET **/feed**

GET **/feed/{id}**

PUT **/feed/{id}** with some request like:
```json
{
  "title": "edited to be something else",
  "body": "whatever"
}
```

DELETE **/feed/{id}**
