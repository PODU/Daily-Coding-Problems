// Streaming vote tally: HashSet of seen voters (duplicate -> fraud, vote dropped),
// HashMap candidate->count, top 3 computed on demand. Time O(n + k log k), Space O(n+k).
use std::collections::{HashMap, HashSet};

fn main() {
    let stream = [
        ("v1", "A"), ("v2", "A"), ("v3", "B"), ("v4", "C"),
        ("v5", "B"), ("v6", "B"), ("v7", "C"), ("v2", "D"),
    ];

    let mut seen: HashSet<&str> = HashSet::new();
    let mut tally: HashMap<&str, i32> = HashMap::new();

    for &(voter, cand) in stream.iter() {
        if seen.contains(voter) {
            println!("Fraud detected: voter {} voted more than once", voter);
            continue; // do not count fraudulent vote
        }
        seen.insert(voter);
        *tally.entry(cand).or_insert(0) += 1;
    }

    let mut v: Vec<(&str, i32)> = tally.into_iter().collect();
    v.sort_by(|a, b| {
        if a.1 != b.1 {
            b.1.cmp(&a.1) // higher votes first
        } else {
            a.0.cmp(b.0) // tie: candidate id ascending
        }
    });

    let n = v.len().min(3);
    let parts: Vec<String> = v[..n].iter().map(|(c, k)| format!("{}({})", c, k)).collect();
    println!("Top 3: {}", parts.join(", "));
}
