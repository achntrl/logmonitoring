use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

mod consumer;
mod parser;

use consumer::{Consumer, ErrorWatcher};
use parser::Parser;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<(Error)>> {
    let mut f = File::open(config.filename)?;
    let parser = Parser::new();

    let one_second = Duration::from_secs(1);

    let mut error_watcher = ErrorWatcher::new();

    loop {
        let mut contents = String::new();

        f.read_to_string(&mut contents)?;

        if contents.len() > 0 {
            let lines = contents.split('\n');
            for line in lines {
                match parser.parse(line) {
                    Some(http_log) => error_watcher.ingest(http_log),
                    None => continue,
                }
            }
        }

        thread::sleep(one_second);
    }
}
