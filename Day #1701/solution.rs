// Stream voting: map vote counts + set of seen voters; duplicate voter = fraud.
// Top 3 by count (ties by candidate id). Time O(n + k log k), Space O(k+v).
use std::collections::{HashMap, HashSet};

fn main() {
    let records = vec![(1, "A"), (2, "B"), (3, "A"), (4, "C"), (2, "A"), (5, "B"), (6, "A")];
    let mut counts: HashMap<&str, i32> = HashMap::new();
    let mut seen: HashSet<i32> = HashSet::new();
    for (voter, cand) in &records {
        if seen.contains(voter) {
            println!("Fraud detected: voter {} voted more than once", voter);
            continue;
        }
        seen.insert(*voter);
        *counts.entry(cand).or_insert(0) += 1;
    }
    let mut v: Vec<(&str, i32)> = counts.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let top: Vec<String> = v.iter().take(3).map(|(c, n)| format!("{}({})", c, n)).collect();
    println!("Top 3 candidates: {}", top.join(", "));
}
