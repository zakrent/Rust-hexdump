extern crate hexdump;

use std::error::Error;
use std::env;
use hexdump::*;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new(env::args())?;
    run(config)?;
    Ok(())
}
