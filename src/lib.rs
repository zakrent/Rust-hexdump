use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Config{
    filename: String,
    columns: u32,
    size: u64,
    offset: u64,
}

impl Config{
    pub fn new(args: std::env::Args) -> Result<Config, Box<dyn Error>>{
        let mut args = args.peekable();
        args.next();
        let mut config = Config{
            filename: String::new(),
            columns: 16,
            size: 1024,
            offset: 0
        };
        while let Some(a) = args.next(){
            let is_next_some = args.peek().is_some();
            match &a[..] {
                "-c" if is_next_some => config.columns = args.next().unwrap().parse::<u32>()?,
                "-s" if is_next_some => config.size = args.next().unwrap().parse::<u64>()?,
                "-o" if is_next_some => config.offset = args.next().unwrap().parse::<u64>()?,
                _ => {
                    let f: String = args.map(|s| " ".to_string()+s.as_str()).collect(); 
                    config.filename=a+f.as_str(); 
                    break;
                },
            }
        }
        return Ok(config);
    }
}
