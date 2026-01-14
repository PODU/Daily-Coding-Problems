// Day 901: Distributed Wikipedia crawler (concrete simulation).
// Approach: BFS over a page link graph with a shared visited set (dedup) and a
// frontier queue. Production: distributed frontier queue, sharded/bloom visited
// store, rotating rate-limited clients to avoid bans, RecentChanges-driven
// re-crawl. Time: O(V+E), Space: O(V).
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

struct CrawlerSystem {
    link_graph: HashMap<String, Vec<String>>,
    visited: HashSet<String>,     // central dedup store
    db: BTreeMap<String, String>, // page -> content
}

impl CrawlerSystem {
    fn new(g: HashMap<String, Vec<String>>) -> Self {
        CrawlerSystem { link_graph: g, visited: HashSet::new(), db: BTreeMap::new() }
    }

    fn crawl(&mut self, seeds: &[&str]) {
        let mut frontier: VecDeque<String> = VecDeque::new();
        for s in seeds {
            self.visited.insert(s.to_string());
            frontier.push_back(s.to_string());
        }
        while let Some(page) = frontier.pop_front() {
            self.db.insert(page.clone(), format!("content of {}", page));
            if let Some(links) = self.link_graph.get(&page) {
                for nxt in links.clone() {
                    if !self.visited.contains(&nxt) {
                        self.visited.insert(nxt.clone());
                        frontier.push_back(nxt);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert("Main_Page".into(), vec!["Python".into(), "Wikipedia".into()]);
    graph.insert("Python".into(), vec!["Programming".into(), "Wikipedia".into()]);
    graph.insert("Wikipedia".into(), vec!["Python".into()]);
    graph.insert("Programming".into(), vec![]);
    let mut sys = CrawlerSystem::new(graph);
    sys.crawl(&["Main_Page"]);
    println!("Pages crawled: {}", sys.db.len());
    let keys: Vec<&String> = sys.db.keys().collect();
    println!("Visited: {:?}", keys);
}
