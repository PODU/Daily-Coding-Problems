// Distributed Wikipedia crawler simulation: master holds a FIFO URL frontier +
// visited set (dedup); workers "fetch" mock pages, parse links, report back; BFS.
// Real design: shard frontier by URL hash (load-balance), distributed visited set
// (bloom filter / consistent hashing), rotating IPs + politeness/backoff to avoid
// blacklisting, recrawl via Last-Modified timestamps. Time/Space: O(V + E).
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    // Mock in-memory "Wikipedia" link graph (adjacency, ordered).
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    graph.insert("Wikipedia", vec!["Computer_Science", "Mathematics"]);
    graph.insert("Computer_Science", vec!["Algorithms", "Mathematics"]);
    graph.insert("Mathematics", vec!["Algorithms"]);
    graph.insert("Algorithms", vec![]);

    let seed = "Wikipedia";
    let mut frontier: VecDeque<&str> = VecDeque::new(); // master FIFO frontier
    let mut seen: HashSet<&str> = HashSet::new();       // dedup set
    let mut db: HashMap<&str, Vec<&str>> = HashMap::new(); // crawled database

    frontier.push_back(seed);
    seen.insert(seed);
    while let Some(url) = frontier.pop_front() {
        let links = graph.get(url).cloned().unwrap_or_default(); // worker fetch+parse
        db.insert(url, links.clone());
        println!("Crawled: {}", url);
        for nxt in links {
            if !seen.contains(nxt) {
                seen.insert(nxt);
                frontier.push_back(nxt);
            }
        }
    }
}
