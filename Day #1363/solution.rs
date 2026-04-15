// Stream voting: HashMap candidate->count, HashSet of voters to detect fraud; top-3 via sort.
// Time: O(records) processing + O(C log C) reporting. Space: O(C + V).
use std::collections::{HashMap, HashSet};

fn main() {
    let stream = [
        ("v1", "A"), ("v2", "B"), ("v3", "A"), ("v4", "C"),
        ("v5", "B"), ("v6", "A"), ("v7", "C"), ("v1", "B"),
    ];

    let mut counts: HashMap<&str, i32> = HashMap::new();
    let mut seen: HashSet<&str> = HashSet::new();

    for (voter, cand) in stream.iter() {
        if seen.contains(voter) {
            println!("Fraud detected: voter {}", voter);
            continue;
        }
        seen.insert(voter);
        *counts.entry(cand).or_insert(0) += 1;
    }

    let mut v: Vec<(&str, i32)> = counts.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));

    let top: Vec<&str> = v.iter().take(3).map(|e| e.0).collect();
    println!("Top 3 candidates: {}", top.join(", "));
}
