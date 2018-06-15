use std::fmt;

use chrono::{DateTime, Local};

struct Alert {
    time: DateTime<Local>,
    hits: u32,
    recovered: bool,
    recovered_time: Option<DateTime<Local>>,
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
