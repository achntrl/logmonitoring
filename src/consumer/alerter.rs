use std::fmt;

use chrono::{DateTime, Local, TimeZone};

use consumer::Consumer;
use parser::HttpLog;

struct Alert {
    time: DateTime<Local>,
    hits: u32,
    recovered: bool,
    recovered_time: Option<DateTime<Local>>,
}

#[derive(Default)]
struct Alerter {
    events_time: Vec<DateTime<Local>>,
    current_alert: Option<Alert>,
    total_hits: u32,
    current_hits: u32,
    threshold: u32,
}

impl Alert {
    fn new(time: DateTime<Local>) -> Alert {
        Alert {
            time,
            hits: 0,
            recovered: false,
            recovered_time: None,
        }
    }

    fn recover(&mut self) {
        self.recovered_time = Some(Local::now());
        self.recovered = true;
    }
}

impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.recovered {
            true => write!(
                f,
                "  Alert raised at {} and recovered at {}",
                self.time.format("%d %b %Y %H:%M:%S"),
                self.recovered_time.unwrap().format("%d %b %Y %H:%M:%S")
            ),
            false => write!(
                f,
                "  High traffic generated an alert - hits = {}, triggered at {}",
                self.hits,
                self.time.format("%d %b %Y %H:%M:%S"),
            ),
        }
    }
}

impl Alerter {
    fn new(threshold: u32) -> Alerter {
        Alerter {
            events_time: Vec::with_capacity(10000),
            threshold: 5,
            ..Default::default()
        }
    }
}

impl Consumer for Alerter {
    fn ingest(&mut self, http_log: &HttpLog) {
        let event_time = DateTime::parse_from_str(&http_log.time, "%d/%b/%Y:%T %z").unwrap();
        let local_event_time = event_time.with_timezone(&Local);
        self.events_time.push(local_event_time);
        self.total_hits += 1;
        self.current_hits += 1;
    }

    fn report(&self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_ingest_dates() {
        let mut alerter = Alerter::new(5);
        let http_log = HttpLog {
            time: String::from("05/Jun/2018:20:55:45 +0200"),
            ..Default::default()
        };

        alerter.ingest(&http_log);

        let expected_time: DateTime<Local> = Local.ymd(2018, 6, 5).and_hms(20, 55, 45);
        assert_eq!(alerter.events_time, vec![expected_time]);
        assert_eq!(alerter.current_hits, 1);
        assert_eq!(alerter.total_hits, 1);
    }
}
