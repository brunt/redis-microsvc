# Redis Microservice REST API

This is an example of using [Actix-web](https://crates.io/crates/actix-web) and [Redis](http://redis.io) as a restful json datastore for a generic news feed.
Internally records are stored in Redis in a Hashmap with a GUID as the key and all record values as 🤔 emoji-separated values in a binary-safe string.


### Cool things
* Redis is [fast](https://redis.io/topics/benchmarks).
* Actix is [fast](https://www.techempower.com/benchmarks/#section=data-r17&hw=ph&test=plaintext).
* Actix uses an actor model pattern to create an asynchronous interface to Redis.
* There is no garbage collector.
* This API is using a Hashmap in Redis to store data but there are [many other ways](https://redis.io/documentation) Redis can be used.
* This API is using basic HTTP but there are [many other ways](https://github.com/actix/examples) Actix-web can be used

### Running locally
Generate localhost.key and localhost.crt in the openssl folder by running
`./makecert.sh`.

Starting Redis via docker:

`docker run --rm --name some-redis -p 6379:6379 -d redis`

(Optional) Examining/editing in Redis:
`docker exec -it some-redis redis-cli`

Then run the program with `cargo run`

### Deploying

Docker build & run:

`docker build -t redis-microsvc:v0.1.0 .`

`docker run --rm -p 8000:8000 -e REDIS_URL=host.docker.internal:6379 redis-microsvc:v0.1.0`

### HTTP endpoints:

If using certs generated with openssl the calls will require https.

POST **/feed** with some request like:
```json
{
  "title": "some title idk",
  "body": "some data to be stored"
}
```

This puts data in redis that can be seen with `hgetall`
```
 127.0.0.1:6379> hgetall feeditems
 1) "6bc07307-3d70-11d4-a3d8-cb2ce9ac0869"
 2) "some title idk\xf0\x9f\xa4\x94some data to be stored\xf0\x9f\xa4\x942019-08-03 23:37:58.433466200 +00:00"
 127.0.0.1:6379> 
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
