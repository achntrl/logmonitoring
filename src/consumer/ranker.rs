use std::collections::HashMap;

use consumer::Consumer;
use parser::HttpLog;

#[derive(Debug)]
pub struct Ranker {
    top_sections: HashMap<String, u32>,
    top_hosts: HashMap<String, u32>,
    max_ranking: usize,
}

impl Ranker {
    pub fn new() -> Ranker {
        let top_sections: HashMap<String, u32> = HashMap::new();
        let top_hosts: HashMap<String, u32> = HashMap::new();

        Ranker {
            top_sections,
            top_hosts,
            max_ranking: 5,
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

    fn process_hosts(&mut self, http_log: &HttpLog) {
        let host = &http_log.host;
        if host != "-" {
            *self.top_hosts.entry(host.to_string()).or_insert(0) += 1;
        }
    }

    fn rank<'a>(&self, container: &'a HashMap<String, u32>) -> Vec<(&'a String, &'a u32)> {
        let mut ranking: Vec<(&String, &u32)> = container.iter().collect();
        ranking.sort_by(|a, b| b.1.cmp(a.1));

        ranking.truncate(self.max_ranking);

        ranking
    }
}

impl Consumer for Ranker {
    fn ingest(&mut self, http_log: &HttpLog) {
        self.process_sections(&http_log);
        self.process_hosts(&http_log);
    }

    fn report(&self) {
        let host_ranking = self.rank(&self.top_hosts);
        let sections_ranking = self.rank(&self.top_sections);

        println!("  Most requested sections:");
        for r in &sections_ranking {
            println!("    {} {:padding$}", r.0, r.1, padding = 50 - r.0.len());
        }

        println!("  {}  ", "- ".repeat(38));

        println!("  Most active hosts:");
        for r in &host_ranking {
            println!("    {} {:padding$}", r.0, r.1, padding = 50 - r.0.len());
        }
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

    #[test]
    fn should_ingest_host() {
        let mut ranker = Ranker::new();
        let http_log = HttpLog {
            host: String::from("204.249.225.59"),
            ..Default::default()
        };

        ranker.ingest(&http_log);

        let mut assert_hashmap = HashMap::new();
        assert_hashmap.insert(String::from("204.249.225.59"), 1);

        assert_eq!(ranker.top_hosts, assert_hashmap);
    }

    #[test]
    fn should_not_ingest_empty_host() {
        let mut ranker = Ranker::new();
        let http_log = HttpLog {
            host: String::from("-"),
            ..Default::default()
        };

        ranker.ingest(&http_log);

        let assert_hashmap = HashMap::new();

        assert_eq!(ranker.top_hosts, assert_hashmap);
    }

    #[test]
    fn should_rank() {
        let mut ranker = Ranker::new();

        ranker.top_sections.insert(String::from("/"), 12);
        ranker.top_sections.insert(String::from("/a"), 10);
        ranker.top_sections.insert(String::from("/b"), 120);
        ranker.top_sections.insert(String::from("/c"), 1);
        ranker.top_sections.insert(String::from("/d"), 5);
        ranker.top_sections.insert(String::from("/e"), 6);

        let sections_ranking = ranker.rank(&ranker.top_sections);

        assert_eq!(
            sections_ranking,
            vec![
                (&String::from("/b"), &120),
                (&String::from("/"), &12),
                (&String::from("/a"), &10),
                (&String::from("/e"), &6),
                (&String::from("/d"), &5),
            ]
        );
    }
}
