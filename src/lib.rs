#[macro_use]
extern crate redismodule;

use redismodule::NextArg;
use redismodule::{Context, RedisError, RedisResult};

use std::usize;

///
/// X.PREPEND <key> <value>
///
fn prepend(ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() > 3 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let mut value = args.next_string()?;

    let redis_key = ctx.open_key_writable(&key);
    let val = redis_key.read()?.unwrap(); // read on writeable always returns Some
    value.push_str(&val);
    redis_key.write(&value)?;
    Ok(value.len().into())
}

redis_module! {
    name: "redisx",
    version: 999999,
    data_types: [],
    commands: [
        ["x.prepend", prepend, "write deny-oom"],
    ],
}
