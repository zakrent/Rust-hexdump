use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Config{
    filename: String,
    columns: u32,
    size: u64,
    offset: u64,
}

#[derive(Debug)]
enum ConfigError{
    InvalidArgument,
}

impl fmt::Display for ConfigError { 
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self{
                ConfigError::InvalidArgument => write!(f, "Invalid argument"),
            }
        }
}
impl Error for ConfigError{}

impl Config {
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
            if args.peek().is_none() {
                config.filename=a;
                break;
            }
            match &a[..] {
                "-c" => config.columns = args.next().unwrap().parse::<u32>()?,
                "-s" => config.size = args.next().unwrap().parse::<u64>()?,
                "-o" => config.offset = args.next().unwrap().parse::<u64>()?,
                _    => return Err(Box::new(ConfigError::InvalidArgument)),
            }
        }
        return Ok(config);
    }
}
