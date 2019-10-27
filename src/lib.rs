#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

#[macro_use]
extern crate redismodule;

use redismodule::{raw as rawmod};

use std::thread;
use std::path::PathBuf;
use redismodule::{Context};


#[get("/<args..>")]
fn index(args: PathBuf) -> String {
    
    let args: Vec<&str> = args.iter().map(|arg| arg.to_str().unwrap()).collect();
    let ctx = Context::get_thread_safe_context();
    
    ctx.lock();
    let result = ctx.call(args[0], &args[1..]);
    ctx.unlock();

    format!("{:?}", result)
}

pub extern "C" fn init(_raw_ctx: *mut rawmod::RedisModuleCtx) -> c_int {
    thread::spawn(|| rocket::ignite().mount("/", routes![index]).launch());
    0
}

redis_module! {
    name: "redisrest",
    version: 0.1.0,
    data_types: [],
    init: init,
    commands: [],
}
