use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Config {
    pub filename: String
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename")
        };

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<(Error)>> {
    let mut f = File::open(config.filename)?;

    let one_second = Duration::from_secs(1);

    loop {
        let mut content = String::new();

        f.read_to_string(&mut content)?;
        println!("{}", content);

        thread::sleep(one_second);
    }
}
