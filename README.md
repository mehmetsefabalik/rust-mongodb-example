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

┌─────────┬──────┬──────┬───────┬──────┬─────────┬─────────┬───────────┐
│ Stat    │ 2.5% │ 50%  │ 97.5% │ 99%  │ Avg     │ Stdev   │ Max       │
├─────────┼──────┼──────┼───────┼──────┼─────────┼─────────┼───────────┤
│ Latency │ 3 ms │ 4 ms │ 6 ms  │ 7 ms │ 3.74 ms │ 2.02 ms │ 102.96 ms │
└─────────┴──────┴──────┴───────┴──────┴─────────┴─────────┴───────────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 16639   │ 16639   │ 17711   │ 18975   │ 17731.2 │ 652.45  │ 16628   │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 1.33 MB │ 1.33 MB │ 1.42 MB │ 1.52 MB │ 1.42 MB │ 52.3 kB │ 1.33 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.

177k requests in 10.14s, 14.2 MB read
315 errors (293 timeouts)

## NodeJS (clustered using pm2 -- pm2 start main.js -i max)


autocannon -c 500  http://localhost:3000/hello?name=joe
Running 10s test @ http://localhost:3000/hello?name=joe
500 connections

┌─────────┬───────┬───────┬────────┬────────┬──────────┬──────────┬───────────┐
│ Stat    │ 2.5%  │ 50%   │ 97.5%  │ 99%    │ Avg      │ Stdev    │ Max       │
├─────────┼───────┼───────┼────────┼────────┼──────────┼──────────┼───────────┤
│ Latency │ 28 ms │ 70 ms │ 157 ms │ 202 ms │ 75.06 ms │ 37.27 ms │ 513.63 ms │
└─────────┴───────┴───────┴────────┴────────┴──────────┴──────────┴───────────┘
┌───────────┬─────────┬─────────┬─────────┬────────┬─────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%  │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 4239    │ 4239    │ 7055    │ 8199   │ 6637.4  │ 1214.75 │ 4238    │
├───────────┼─────────┼─────────┼─────────┼────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 1.14 MB │ 1.14 MB │ 1.89 MB │ 2.2 MB │ 1.78 MB │ 325 kB  │ 1.14 MB │
└───────────┴─────────┴─────────┴─────────┴────────┴─────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.

66k requests in 10.22s, 17.8 MB read