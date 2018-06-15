use parser::HttpLog;

pub mod errorwatcher;
pub mod ranker;
pub mod alerter;

pub trait Consumer {
    fn ingest(&mut self, http_log: &HttpLog);
    fn report(&self);
}
