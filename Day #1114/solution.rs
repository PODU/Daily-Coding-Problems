// Day 1114 - Voting stream: top 3 candidates + fraud detection
// Approach: hash-map vote counts + set of seen voters (O(1) dup detection);
// top-3 via sort. Time: O(R + M log M), Space: O(V+M).
use std::collections::{HashMap, HashSet};

fn main() {
    let stream: Vec<(i32, &str)> = vec![
        (1, "A"), (2, "B"), (3, "A"), (4, "C"), (5, "B"),
        (6, "A"), (2, "C"), (7, "D"), (8, "A"),
    ];
    let mut counts: HashMap<&str, i32> = HashMap::new();
    let mut seen: HashSet<i32> = HashSet::new();
    for (voter, cand) in &stream {
        if seen.contains(voter) {
            println!("Fraud detected: voter {} voted more than once", voter);
            continue;
        }
        seen.insert(*voter);
        *counts.entry(cand).or_insert(0) += 1;
    }
    let mut items: Vec<(&str, i32)> = counts.into_iter().collect();
    items.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let parts: Vec<String> = items
        .iter()
        .take(3)
        .map(|(c, n)| format!("('{}', {})", c, n))
        .collect();
    println!("Top 3 candidates: [{}]", parts.join(", "));
}
