[![GitHub issues](https://img.shields.io/github/release/RedisLabsModules/RedisREST.svg)](https://github.com/RedisLabsModules/RedisREST/releases/latest)
[![CircleCI](https://circleci.com/gh/RedisLabsModules/RedisREST/tree/master.svg?style=svg)](https://circleci.com/gh/RedisLabsModules/RedisREST/tree/master)

# RedisREST
Extension modules to Redis' native data types and commands


## Getting started

```bash
cargo +nightly build --release
ROCKET_ENV=production redis-server --loadmodule ./target/release/libredisrest.so
```

*Note: Redis REST Module uses [Rocket](https://rocket.rs/) web frameworks that must be build using nightly version of Rust.*

### Using the Redis REST module

Let's get, set and delete key in Redis:

```bash

curl -X GET http://localhost:8000/set/foo/bar

curl -X GET http://localhost:8000/get/foo

curl -X GET http://localhost:8000/del/foo
```

Increment a value

``` bash
curl -X GET http://localhost:8000/set/num/1

curl -X GET http://localhost:8000/get/num

curl -X GET http://localhost:8000/incr/num

curl -X GET http://localhost:8000/get/num
```

Set and get a hash

``` bash
curl -X GET http://localhost:8000/hset/hfoo/key1/val1/key2/val2

curl -X GET http://localhost:8000/hgetall/hfoo

curl -X GET http://localhost:8000/hmset/hfoo/key1/new_value1

curl -X GET http://localhost:8000/hgetall/hfoo
```

You have the pattern...

Enjoy!
