#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate redismodule;

use redismodule::raw as rawmod;

use redismodule::{Context, RedisValue};
use rocket::response::content;
use rocket::response::status::BadRequest;
use std::path::PathBuf;
use std::thread;

fn to_json<'a>(value: &'a RedisValue) -> String {
    match value {
        RedisValue::SimpleStringStatic(v) => v.to_string(),
        RedisValue::SimpleString(v) => v.to_string(),
        RedisValue::BulkString(v) => v.to_string(),
        RedisValue::Integer(v) => v.to_string(),
        RedisValue::Float(v) => v.to_string(),
        RedisValue::Array(v) => {
            let mut s = String::from("[");
            s.push_str(
                v.iter()
                    .map(|i| to_json(i))
                    .collect::<Vec<String>>()
                    .join(",")
                    .as_str(),
            );
            s.push(']');
            s
        }
        RedisValue::None => "null".to_string(),
    }
}

#[get("/<args..>")]
fn path(args: PathBuf) -> Result<String, BadRequest<String>> {
    let args: Vec<&str> = args.iter().map(|arg| arg.to_str().unwrap()).collect();
    let ctx = Context::get_thread_safe_context();

    ctx.lock();
    let result = ctx.call(args[0], &args[1..]);
    ctx.unlock();

    match result {
        Ok(s) => Ok(to_json(&s)),
        Err(e) => Err(BadRequest(Some(format!("{}", e)))),
    }
}

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html(
        "<html>
            <head>
                <script type=\"text/javascript\">
                    function sendCmd(){
                        let cmd = document.getElementById('cli').value;
                        let xhr = new XMLHttpRequest();               
                        xhr.open('GET', window.location.href + cmd.match(/\\w+|\"[^\"]+\"/g).join('/')); 
                        xhr.onreadystatechange = function() {
                            if (xhr.readyState == XMLHttpRequest.DONE) {
                                let result = document.createElement(\"LI\")
                                let text = document.createTextNode(xhr.responseText);
                                result.appendChild(text); 
                                let results = document.getElementById('results');
                                results.appendChild(result)
                            }
                        } 
                        xhr.send();
                    }
                </script>
            </head>
            <body>
                <form onsubmit=\"sendCmd();return false;\">
                    Redis CLI: <input id=\"cli\" style=\"width: 60%;\"> <input type=\"submit\" value=\"Send\">
                </form>
                <ul id=\"results\">
                </ul>
            </body>
        </html>",
    )
}

pub extern "C" fn init(_raw_ctx: *mut rawmod::RedisModuleCtx) -> c_int {
    thread::spawn(|| rocket::ignite().mount("/", routes![index, path]).launch());
    0
}

redis_module! {
    name: "redisrest",
    version: 001000,
    data_types: [],
    init: init,
    commands: [],
}
