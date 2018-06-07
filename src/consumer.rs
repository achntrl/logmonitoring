use std::collections::HashMap;
use parser::HttpLog;


pub trait Consumer {
    fn ingest(&mut self, http_log: HttpLog);
    fn report(&self);
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ErrorCode {
    Error4xx,
    Error5xx,
}


#[derive(Debug)]
pub struct ErrorWatcher {
    total_hits: u32,
    errors: HashMap<ErrorCode, u32>
}

impl ErrorWatcher {
    pub fn new() -> ErrorWatcher {
        let errors: HashMap<ErrorCode, u32> = HashMap::new();

        ErrorWatcher { total_hits: 0, errors }
    }
}


impl Consumer for ErrorWatcher {
    fn ingest(&mut self, http_log: HttpLog) {
        self.total_hits += 1;
        match http_log.status.chars().next() {
            Some('4') => *self.errors.entry(ErrorCode::Error4xx).or_insert(0) += 1,
            Some('5') => *self.errors.entry(ErrorCode::Error5xx).or_insert(0) += 1,
            _ => ()
        }
        println!("Hits: {}, {:?}",self.total_hits, self.errors);
    }

    fn report(&self) {
        unimplemented!();
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_ingest_errors() {
        let mut error_watcher = ErrorWatcher::new();
        let http_log1 = HttpLog { status: String::from("200"), ..Default::default() };
        let http_log2 = HttpLog { status: String::from("404"), ..Default::default() };
        let http_log3 = HttpLog { status: String::from("500"), ..Default::default() };

        let mut assert_hashmap = HashMap::new();
        error_watcher.ingest(http_log1);
        assert_eq!(error_watcher.total_hits, 1);
        assert_eq!(error_watcher.errors, assert_hashmap);

        error_watcher.ingest(http_log2);
        assert_eq!(error_watcher.total_hits, 2);
        assert_hashmap.insert(ErrorCode::Error4xx, 1);
        assert_eq!(error_watcher.errors, assert_hashmap);

        error_watcher.ingest(http_log3);
        assert_eq!(error_watcher.total_hits, 3);
        assert_hashmap.insert(ErrorCode::Error5xx, 1);
        assert_eq!(error_watcher.errors, assert_hashmap);
    }
}