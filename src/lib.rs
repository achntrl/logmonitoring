extern crate chrono;

use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use chrono::now;

mod consumer;
mod parser;

use consumer::errorwatcher::ErrorWatcher;
use consumer::ranker::Ranker;
use consumer::Consumer;
use parser::{HttpLog, Parser};

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
    let mut queue: VecDeque<HttpLog> = VecDeque::with_capacity(10000);

    let ten_seconds = Duration::from_secs(10);

    let mut error_watcher = ErrorWatcher::new();
    let mut ranker = Ranker::new();

    let mut consumers: Vec<&mut Consumer> = Vec::new();

    consumers.push(&mut error_watcher);
    consumers.push(&mut ranker);

    loop {
        let mut contents = String::new();

        f.read_to_string(&mut contents)?;

        if contents.len() > 0 {
            let lines = contents.split('\n');
            for line in lines {
                match parser.parse(line) {
                    Some(http_log) => queue.push_back(http_log),
                    None => continue,
                }
            }
        }

        while queue.len() > 0 {
            let http_log = queue.pop_front().unwrap();
            for consumer in &mut consumers {
                consumer.ingest(&http_log);
            }
        }

        println!("{}");
        for consumer in &mut consumers {
            println!("{}", "= ".repeat(40));
            consumer.report();
        }

        println!("");
        thread::sleep(ten_seconds);
    }
}
