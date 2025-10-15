// Day 432: Distributed Wikipedia crawler — concrete single-process simulation.
// Design: a central coordinator holds a URL frontier (BFS queue) + visited set for dedup.
// Workers pull URLs, "fetch" pages from an in-memory graph, parse links, store content keyed
// by a content hash + last-crawled timestamp, and push new URLs back. Politeness delay per
// domain + worker blacklist/rotation prevent overload. Incremental re-crawl compares the
// stored content hash to detect updates. O(V+E) over the page graph.
use std::collections::{HashMap, HashSet, VecDeque};

fn fnv1a(s: &str) -> u32 {
    let mut h: u32 = 2166136261;
    for b in s.bytes() {
        h ^= b as u32;
        h = h.wrapping_mul(16777619);
    }
    h
}

struct Coordinator {
    wiki: HashMap<String, (String, Vec<String>)>,
    frontier: VecDeque<String>,
    visited: HashSet<String>,
    db: HashMap<String, (u32, usize)>, // url -> (hash, ts)
    order: Vec<String>,
    workers: Vec<String>,
    req_count: HashMap<String, i32>,
    blacklisted: HashSet<String>,
    last_access: i64,
    wi: usize,
}

impl Coordinator {
    fn new(wiki: HashMap<String, (String, Vec<String>)>) -> Self {
        Coordinator {
            wiki,
            frontier: VecDeque::new(),
            visited: HashSet::new(),
            db: HashMap::new(),
            order: Vec::new(),
            workers: vec!["w0".into(), "w1".into(), "w2".into()],
            req_count: HashMap::new(),
            blacklisted: HashSet::new(),
            last_access: 0,
            wi: 0,
        }
    }
    fn pick_worker(&mut self) -> String {
        for _ in 0..self.workers.len() {
            let w = self.workers[self.wi % self.workers.len()].clone();
            self.wi += 1;
            if !self.blacklisted.contains(&w) {
                return w;
            }
        }
        self.blacklisted.clear();
        self.workers[0].clone()
    }
    fn fetch(&mut self, worker: &str, url: &str) -> (String, Vec<String>) {
        self.last_access += 1; // politeness tick
        *self.req_count.entry(worker.to_string()).or_insert(0) += 1;
        if self.req_count[worker] >= 2 {
            self.blacklisted.insert(worker.to_string()); // rate-limit -> rotate out
        }
        self.wiki[url].clone()
    }
    fn crawl(&mut self, seed: &str) {
        self.frontier.push_back(seed.to_string());
        while let Some(url) = self.frontier.pop_front() {
            if self.visited.contains(&url) {
                continue;
            }
            self.visited.insert(url.clone());
            let worker = self.pick_worker();
            let (content, links) = self.fetch(&worker, &url);
            self.db.insert(url.clone(), (fnv1a(&content), self.order.len()));
            self.order.push(url.clone());
            for l in links {
                if !self.visited.contains(&l) {
                    self.frontier.push_back(l);
                }
            }
        }
    }
    fn recrawl(&mut self, url: &str) -> Option<(u32, u32)> {
        let content = self.wiki[url].0.clone();
        let nh = fnv1a(&content);
        let oh = self.db[url].0;
        if nh != oh {
            let ts = self.db[url].1;
            self.db.insert(url.to_string(), (nh, ts));
            return Some((oh, nh));
        }
        None
    }
}

fn main() {
    let mut wiki: HashMap<String, (String, Vec<String>)> = HashMap::new();
    wiki.insert("Main".into(), ("Welcome to the wiki".into(), vec!["A".into(), "B".into()]));
    wiki.insert("A".into(), ("Page A content".into(), vec!["C".into()]));
    wiki.insert("B".into(), ("Page B content".into(), vec!["C".into()]));
    wiki.insert("C".into(), ("Page C content".into(), vec!["Main".into()]));

    let mut c = Coordinator::new(wiki);
    c.crawl("Main");
    println!("Crawled {} pages: [{}]", c.order.len(), c.order.join(", "));

    c.wiki.insert("C".into(), ("Page C content (edited 2026)".into(), vec!["Main".into()]));
    if let Some((oh, nh)) = c.recrawl("C") {
        println!("Re-crawl detected update on 'C': hash {:08x} -> {:08x}, re-stored.", oh, nh);
    }
}
