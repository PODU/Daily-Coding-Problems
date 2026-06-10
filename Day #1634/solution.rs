// Word ladder: BFS over dictionary words (one-letter changes), tracking parents to rebuild shortest path. O(N*L*N) time.
use std::collections::{HashMap, HashSet, VecDeque};

fn differs_by_one(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0;
    for (ca, cb) in a.bytes().zip(b.bytes()) {
        if ca != cb {
            diff += 1;
            if diff > 1 {
                return false;
            }
        }
    }
    diff == 1
}

fn ladder(start: &str, end: &str, dict: &[&str]) -> Option<Vec<String>> {
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.to_string());
    let mut q: VecDeque<String> = VecDeque::new();
    q.push_back(start.to_string());
    let mut parent: HashMap<String, String> = HashMap::new();
    while let Some(cur) = q.pop_front() {
        if cur == end {
            let mut path: Vec<String> = Vec::new();
            let mut at = cur.clone();
            loop {
                path.push(at.clone());
                if at == start {
                    break;
                }
                at = parent[&at].clone();
            }
            path.reverse();
            return Some(path);
        }
        for &w in dict {
            if !visited.contains(w) && differs_by_one(&cur, w) {
                visited.insert(w.to_string());
                parent.insert(w.to_string(), cur.clone());
                q.push_back(w.to_string());
            }
        }
    }
    None
}

fn format(path: Option<Vec<String>>) -> String {
    match path {
        None => "null".to_string(),
        Some(p) => {
            let quoted: Vec<String> = p.iter().map(|w| format!("\"{}\"", w)).collect();
            format!("[{}]", quoted.join(", "))
        }
    }
}

fn main() {
    println!("{}", format(ladder("dog", "cat", &["dot", "dop", "dat", "cat"])));
    println!("{}", format(ladder("dog", "cat", &["tod", "dat", "dar", "dot"])));
}
