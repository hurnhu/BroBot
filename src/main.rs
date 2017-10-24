#[macro_use] extern crate serenity;
extern crate chrono;
extern crate rand;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
use serde_json::Error;

mod commands;

use serenity::prelude::*;
use serenity::framework::StandardFramework;

use std::env;
struct Handler; impl EventHandler for Handler {}
fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").unwrap(), Handler);

    client.with_framework(StandardFramework::new()
        .configure(|c| c.on_mention(true))
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
