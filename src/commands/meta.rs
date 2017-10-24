extern crate chrono;
use chrono::prelude::*;
use std::io::prelude::*;
use std::fs::File;

command!(latency(ctx, msg) {
    let latency = ctx.shard.lock()
        .latency()
        .map_or_else(|| "N/A".to_owned(), |s| {
            format!("{}.{}s", s.as_secs(), s.subsec_nanos())
        });

    let _ = msg.channel_id.say(&latency);
});

command!(ping(_ctx, msg) {
    let _ = msg.channel_id.say("Pong!");
});

command!(time(_context, message) {
    let utc: DateTime<Utc> = Utc::now();
    if let Err(why) = message.channel_id.say(&format!("Ding dong, ding dong! The current time is: {} (UTC)", &utc.format("%H:%M").to_string())){
        println!("error: {:?}", why);
    }
});

command!(help(_context, message) {
  let mut file = File::open("help.bro").unwrap();
  let mut s = String::new();
  file.read_to_string(&mut s).unwrap();
  if let Err(why) = message.channel_id.say(&s){
    println!("error: {:?}", why);
  }
});

command!(usage(_context, message) {
  let mut file = File::open("usage.bro").unwrap();
  let mut s = String::new();
  file.read_to_string(&mut s).unwrap();
  if let Err(why) = message.channel_id.say(&s){
    println!("error: {:?}", why);
  }
});

command!(source(_context, message) {
  let message_to_send = String::from("You need the sauce? I got the sauce right here -> https://github.com/hurnhu/BroBot");
  if let Err(why) = message.channel_id.say(&message_to_send) {
    println!("error: {:?}", why);
  }
});

command!(author(_context, message) {
  let message_to_send = String::from("here is my author: https://github.com/hurnhu , give him the lovin");
  if let Err(why) = message.channel_id.say(&message_to_send) {
    println!("error: {:?}", why);
  }
});

