use std::error::Error;
use std::io::SeekFrom;
use std::fmt;
use std::io::prelude::*;
use std::fs::File;

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = read_file(&config)?;
    display(&config, file_content)?;
    return Ok(());
}

fn read_file(config: &Config) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut f = File::open(&config.filename)?;
    f.seek(SeekFrom::Start(config.offset))?;
    let mut f = f.take(config.size);
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

fn display(config: &Config, file_content: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let mut file_content = file_content.into_iter();
    let mut done = false;
    let mut address_counter = config.offset;
    while !done {
        print!("{:08X}: ", address_counter);
        let mut ascii_rep = Vec::new();
        for i in 0..config.columns {
            match file_content.next() {
                Some(v) => {
                    ascii_rep.push(byte_to_ascii(&v));
                    print!("{:02X}", v);
                },
                None    => {
                    print!("  ");
                    done = true;
                },
            }
            if i % 2 == 1 {
                print!{" "};
            }
        }
        let ascii_rep: String = ascii_rep.into_iter().collect();
        println!("{}", ascii_rep);
        address_counter += config.columns as u64;
    }
    Ok(())
}

fn byte_to_ascii(byte: &u8) -> char{
    match byte {
        0x20 ... 0x7e => *byte as char,
        _            => '.',
    }
}
