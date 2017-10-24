extern crate rand;
use rand::distributions::{IndependentSample, Range};
command!(lol(_context, message, _args) {
  let a = Range::new(1, 50);
  let b = Range::new(1000, 6000);
  let mut rng = rand::thread_rng();
  let mut url = &format!("http://generated.inspirobot.me/{aa:>0width$}/aXm{bb}xjU.jpg", aa = a.ind_sample(&mut rng), bb = b.ind_sample(&mut rng), width = 3);
    if let Err(why) = message.channel_id.say(url){
        println!("error: {:?}", why);
    }
});
