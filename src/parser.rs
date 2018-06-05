extern crate regex;

use self::regex::Regex;

pub struct Parser {
    pub regex: Regex,
}

#[derive(Debug)]
pub struct HttpLog {
    pub host: String,
    pub identity: String,
    pub user: String,
    pub time: String,
    pub request: String,
    pub status: String,
    pub size: String,
}

impl Parser {
    pub fn new() -> Parser {
        let mut regex_parts: Vec<&str> = Vec::with_capacity(7);
        regex_parts.push(r"(?P<host>\S+)");
        regex_parts.push(r"(?P<identity>\S+)");
        regex_parts.push(r"(?P<user>\S+)");
        regex_parts.push(r"\[(?P<time>.+)\]");
        regex_parts.push(r#""(?P<request>.+)""#);
        regex_parts.push(r"(?P<status>[0-9]+)");
        regex_parts.push(r"(?P<size>\S+)");

        let string: String = regex_parts.join(r"\s");
        let regex = Regex::new(&string).unwrap();

        Parser { regex }
    }

    pub fn parse(&self, line: &str) -> Option<HttpLog> {
        match self.regex.captures(&line) {
            Some(cap) => {
                let http_log = HttpLog {
                    host: cap["host"].to_owned(),
                    identity: cap["identity"].to_owned(),
                    user: cap["user"].to_owned(),
                    time: cap["time"].to_owned(),
                    request: cap["request"].to_owned(),
                    status: cap["status"].to_owned(),
                    size: cap["size"].to_owned(),
                };
                Some(http_log)
            }
            None => None,
        }
    }
}

