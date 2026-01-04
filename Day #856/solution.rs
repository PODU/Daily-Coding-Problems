// Day 856: Top k similar website pairs.
// Approach: Jaccard similarity (|A∩B| / |A∪B|) over user sets, take top k pairs.
// Time: O(W^2 * U), Space: O(W * U).
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let visits: Vec<(&str, i32)> = vec![
        ("a", 1), ("a", 3), ("a", 5),
        ("b", 2), ("b", 6),
        ("c", 1), ("c", 2), ("c", 3), ("c", 4), ("c", 5),
        ("d", 4), ("d", 5), ("d", 6), ("d", 7),
        ("e", 1), ("e", 3), ("e", 5), ("e", 6),
    ];
    let k = 1;

    let mut sites: BTreeMap<&str, BTreeSet<i32>> = BTreeMap::new();
    for (s, u) in &visits {
        sites.entry(s).or_default().insert(*u);
    }
    let names: Vec<&str> = sites.keys().cloned().collect();

    let mut scored: Vec<(f64, (&str, &str))> = Vec::new();
    for i in 0..names.len() {
        for j in i + 1..names.len() {
            let a = &sites[names[i]];
            let b = &sites[names[j]];
            let inter = a.intersection(b).count();
            let uni = a.union(b).count();
            let sim = if uni == 0 { 0.0 } else { inter as f64 / uni as f64 };
            scored.push((sim, (names[i], names[j])));
        }
    }
    scored.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());

    let parts: Vec<String> = scored
        .iter()
        .take(k)
        .map(|(_, (a, b))| format!("('{}', '{}')", a, b))
        .collect();
    println!("[{}]", parts.join(", "));
}
