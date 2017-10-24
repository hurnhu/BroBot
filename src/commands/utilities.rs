//extern crate serde;
//extern crate serde_json;

//#[macro_use]
//extern crate serde_derive;
use serde_json::Error;

//#[derive(Serialize, Deserialize)]
struct soBlob {
    title: String,
    answers: u8,
}


command!(so(_context, message, args) {
//    let p: soBlob = serde_json::from_str(one)?;    
    //let v: Value = serde_json::from_str(&one)?;
    let one = args.single::<str>().unwrap();
     println!("Please call {}", &one);
    
});
