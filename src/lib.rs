#[macro_use]
extern crate redismodule;

use redismodule::NextArg;
use redismodule::{Context, RedisError, RedisResult};

use std::time::Duration;
use std::usize;

///
/// X.PREPEND <key> <value>
///
fn prepend(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let mut value = args.next_string()?;

    let redis_key = ctx.open_key_writable(&key);
    let val = redis_key.read()?.unwrap(); // read on writeable always returns Some
    value.push_str(&val);
    redis_key.write(&value)?;

    ctx.replicate_verbatim();

    Ok(value.len().into())
}

///
/// X.GETSETEX <key> <seconds> <value>
///
fn getsetex(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let seconds = args.next_u64()?;
    let value = args.next_string()?;

    let redis_key = ctx.open_key_writable(&key);
    let res = if redis_key.is_empty() {
        ().into()
    } else {
        redis_key.read()?.unwrap().into() // read on writeable always returns Some
    };

    redis_key.write(&value)?;
    redis_key.set_expire(Duration::from_secs(seconds))?;

    ctx.replicate_verbatim();

    Ok(res)
}

///
/// X.GETEX <key> <seconds>
///
fn getex(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let seconds = args.next_u64()?;

    let redis_key = ctx.open_key_writable(&key);

    let res = if redis_key.is_empty() {
        ().into()
    } else {
        redis_key.set_expire(Duration::from_secs(seconds))?;
        ctx.replicate_verbatim();
        redis_key.read()?.unwrap().into() // read on writeable always returns Some
    };

    Ok(res)
}

redis_module! {
    name: "redisx",
    version: 999999,
    data_types: [],
    commands: [
        ["x.prepend", prepend, "write deny-oom"],
        ["x.getsetex", getsetex, "write deny-oom"],
        ["x.getex", getex, "write"],
    ],
}
