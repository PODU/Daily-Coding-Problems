// Approach: Single-process simulation of a distributed BFS web crawler. Central frontier (queue) +
// visited set for dedup; N round-robin workers fetch outlinks from a mock graph, store to a mock DB,
// requeue work from a blacklisted worker. Time O(V+E), Space O(V+E).
//
// Real distributed design: a master holds the frontier and shards the "URL-seen" set across nodes via
// consistent hashing or a bloom filter; workers crawl politely (robots.txt, rate limits, rotating IPs)
// to avoid blacklisting; recrawl uses last-modified/ETag to detect and update changed pages.

use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let graph: HashMap<&str, Vec<&str>> = HashMap::from([
        ("Main", vec!["A", "B", "C"]),
        ("A", vec!["B", "D"]),
        ("B", vec!["C"]),
        ("C", vec!["A", "E"]),
        ("D", vec!["E"]),
        ("E", vec![]),
    ]);

    let num_workers = 3;
    let mut frontier: VecDeque<&str> = VecDeque::from(["Main"]);
    let mut visited: HashSet<&str> = HashSet::from(["Main"]);
    let mut db: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut blacklisted: HashSet<i32> = HashSet::new();
    let mut worker = 0;
    let mut processed_any = false;

    while let Some(url) = frontier.pop_front() {
        let w = worker % num_workers;
        worker += 1;

        // Blacklist worker #1 after at least one page processed; requeue its task.
        if w == 1 && processed_any && !blacklisted.contains(&1) {
            blacklisted.insert(1);
            frontier.push_back(url); // in-flight task requeued, no page lost
            continue;
        }

        let links = graph.get(url).cloned().unwrap_or_default();
        db.insert(url, links.clone()); // "fetch" + store to mock DB
        processed_any = true;

        for link in links {
            if !visited.contains(link) {
                visited.insert(link);
                frontier.push_back(link);
            }
        }
    }

    println!("Crawled {} pages", db.len());
    let mut titles: Vec<&str> = db.keys().copied().collect();
    titles.sort();
    println!("{}", titles.join(" "));
}
