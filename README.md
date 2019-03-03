Access Redis via docker

`docker run --rm --name some-redis -p 6379:6379 -d redis`

`docker exec -it some-redis redis-cli`

docker build -t redis-microsvc .

