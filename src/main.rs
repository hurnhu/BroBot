#[macro_use] extern crate log;
#[macro_use] extern crate serenity;

extern crate env_logger;
extern crate kankyo;

mod commands;

use serenity::framework::StandardFramework;
use serenity::model::event::ResumedEvent;
use serenity::model::Ready;
use serenity::prelude::*;
use serenity::http;
use std::collections::HashSet;
use std::env;

extern crate chrono;
extern crate rand;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
use serde_json::Error;

struct Handler;

impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn on_resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
fn main() {
     // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    kankyo::load().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    env_logger::init().expect("Failed to initialize env_logger");

let mut client = Client::new(&env::var("DISCORD_TOKEN").unwrap(), Handler);
    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
        .configure(|c| c.on_mention(true) .owners(owners))
       .command("time", |c| c.exec(commands::meta::time))
        .command("ping", |c| c.exec(commands::meta::ping))
        .command("latency", |c| c.exec(commands::meta::latency))
        .command("multiply", |c| c.exec(commands::math::multiply))
        .command("lol", |c| c.exec(commands::meme::lol))
        .command("help", |c| c.exec(commands::meta::help))
        .command("usage", |c| c.exec(commands::meta::usage))
        .command("source", |c| c.exec(commands::meta::source))
        .command("author", |c| c.exec(commands::meta::author)));
        //.command("so", |c| c.exec(commands::utilities::so)));

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
