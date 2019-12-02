# ACTIX WEB - MONGODB EXAMPLE

A web server, written in Rust, using actix-web framework. Web server exposes `GET /hello` endpoint, and writes `name` query string value to mongodb as a document(`{name: $name-query-string`).

# HOW TO RUN

- start a `mongod` deamon locally (`mongodb://localhost:27017`)
- run `cargo build --release`
- run `./target/release/rust-mongo-example`

`GET /hello` endpoint will be exposed.

# HOW TO TEST

`curl http://localhost:3000/hello?name=joe`

`Success` should be returned and `{name: "joe"}` document should have been written in `mydb` database, `users` collection.


# PERFORMANCE

In order to see the performance of Rust as a web server with I/O bound tasks, I benchmarked this project against a NodeJS web server(using NestJS) which does the same task. Here are results:

## Rust

autocannon -c 500  http://localhost:3000/hello?name=joe
Running 10s test @ http://localhost:3000/hello?name=joe
500 connections

`
Avg Latency: 3.74 ms
`

`
Avg Req/Sec: 17731.2
`

Req/Bytes counts sampled once per second.

177k requests in 10.14s, 14.2 MB read
315 errors (293 timeouts)

## NodeJS (clustered using pm2 -- pm2 start main.js -i max)


autocannon -c 500  http://localhost:3000/hello?name=joe
Running 10s test @ http://localhost:3000/hello?name=joe
500 connections

`
Avg Latency: 75.06 ms
`

`
Avg Req/Sec: 6637.4
`


Req/Bytes counts sampled once per second.

66k requests in 10.22s, 17.8 MB read


# CONTRIBUTE

Feel free to contribute, you may want to add extra endpoints, benchmark against different stacks, etc.
