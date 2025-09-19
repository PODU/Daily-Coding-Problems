// Stream voting: track seen voters + candidate counts; report top-3 (count desc, id asc) or FRAUD on repeat.
// Time: O(n * c log c) over stream, Space: O(voters + candidates).
use std::collections::{HashMap, HashSet};

fn main() {
    let stream: Vec<(i32, &str)> = vec![
        (1, "A"), (2, "B"), (3, "A"), (4, "C"), (5, "B"), (1, "A"), (6, "A"),
    ];

    let mut seen: HashSet<i32> = HashSet::new();
    let mut counts: HashMap<String, i32> = HashMap::new();

    for (voter, cand) in stream {
        if seen.contains(&voter) {
            println!("Fraud: voter {} voted more than once", voter);
            continue;
        }
        seen.insert(voter);
        *counts.entry(cand.to_string()).or_insert(0) += 1;

        let mut ranked: Vec<(&String, &i32)> = counts.iter().collect();
        ranked.sort_by(|a, b| {
            if a.1 != b.1 {
                b.1.cmp(a.1) // count desc
            } else {
                a.0.cmp(b.0) // id asc
            }
        });

        let limit = ranked.len().min(3);
        let parts: Vec<String> = ranked[..limit]
            .iter()
            .map(|(id, _)| format!("'{}'", id))
            .collect();
        println!("Top 3: [{}]", parts.join(", "));
    }
}
