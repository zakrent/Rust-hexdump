extern crate hexdump;

use std::env;
use hexdump::*;

fn main() {
    let config = Config::new(env::args()).unwrap();
    println!("{:?}",config); 
    run(config);
}
