// Distributed crawler design (simulation): Coordinator holds a FIFO URL frontier + central visited set.
// Workers pull a URL, "download" (map lookup), record it, push unvisited links. BFS over a mock graph.
// Architecture: reach pages = BFS from seeds; visited = central set/bloom filter on coordinator;
// blacklisting = rotate IPs/user-agents, rate-limit + backoff; updates = recrawl by last-modified queue.
// Time O(V+E), Space O(V).
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut wiki: HashMap<&str, Vec<&str>> = HashMap::new();
    wiki.insert("/Main", vec!["/Apple", "/Banana"]);
    wiki.insert("/Apple", vec!["/Banana", "/Fruit"]);
    wiki.insert("/Banana", vec!["/Fruit"]);
    wiki.insert("/Fruit", vec!["/Main"]);

    let seeds = vec!["/Main"];
    let mut frontier: VecDeque<&str> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();
    let mut order: Vec<&str> = Vec::new();

    for s in seeds {
        if visited.insert(s) {
            frontier.push_back(s);
        }
    }

    while let Some(url) = frontier.pop_front() {
        order.push(url); // "download" + record into results DB
        if let Some(links) = wiki.get(url) {
            for &link in links {
                if visited.insert(link) {
                    frontier.push_back(link);
                }
            }
        }
    }

    println!("Crawled {} pages:", order.len());
    for p in &order {
        println!("{}", p);
    }
}
