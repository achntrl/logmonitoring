use std::collections::HashMap;

use consumer::Consumer;
use parser::HttpLog;

#[derive(Debug)]
pub struct Ranker {
    top_sections: HashMap<String, u32>,
    top_hosts: HashMap<String, u32>,
}

impl Ranker {
    pub fn new() -> Ranker {
        let top_sections: HashMap<String, u32> = HashMap::new();
        let top_hosts: HashMap<String, u32> = HashMap::new();

        Ranker {
            top_sections,
            top_hosts,
        }
    }

    fn process_sections(&mut self, http_log: &HttpLog) {
        let mut request = http_log.request.split(' ');
        request.next();
        let mut path = request.next().unwrap_or("").split('/');
        path.next();

        let mut section = String::from("/");
        section.push_str(path.next().unwrap_or(""));

        *self.top_sections.entry(section).or_insert(0) += 1;
    }
}

impl Consumer for Ranker {
    fn ingest(&mut self, http_log: &HttpLog) {
        self.process_sections(&http_log);
        self.report();
    }

    fn report(&self) {
        println!("{:?}", self);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_ingest_sections_from_root_url() {
        let mut ranker = Ranker::new();
        let http_log = HttpLog {
            request: String::from("GET / HTTP/1.0"),
            ..Default::default()
        };

        ranker.ingest(&http_log);

        let mut assert_hashmap = HashMap::new();
        assert_hashmap.insert(String::from("/"), 1);

        assert_eq!(ranker.top_sections, assert_hashmap);
    }

    #[test]
    fn should_ingest_sections_from_simple_url() {
        let mut ranker = Ranker::new();
        let http_log = HttpLog {
            request: String::from("GET /domain HTTP/1.0"),
            ..Default::default()
        };

        ranker.ingest(&http_log);

        let mut assert_hashmap = HashMap::new();
        assert_hashmap.insert(String::from("/domain"), 1);

        assert_eq!(ranker.top_sections, assert_hashmap);
    }

    #[test]
    fn should_ingest_sections_from_long_url() {
        let mut ranker = Ranker::new();
        let http_log = HttpLog {
            request: String::from("GET /pub/job/vk/view17.jpg HTTP/1.0"),
            ..Default::default()
        };

        ranker.ingest(&http_log);

        let mut assert_hashmap = HashMap::new();
        assert_hashmap.insert(String::from("/pub"), 1);

        assert_eq!(ranker.top_sections, assert_hashmap);
    }
}
