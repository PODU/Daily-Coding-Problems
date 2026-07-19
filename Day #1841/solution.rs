// Day 1841: Top-k most similar website pairs by Jaccard similarity of their visitor sets.
// Time O(W^2 * U) over W websites; Space O(total pairs).
use std::collections::{BTreeMap, HashSet};

fn main() {
    let pairs: Vec<(&str, i32)> = vec![
        ("a", 1), ("a", 3), ("a", 5),
        ("b", 2), ("b", 6),
        ("c", 1), ("c", 2), ("c", 3), ("c", 4), ("c", 5),
        ("d", 4), ("d", 5), ("d", 6), ("d", 7),
        ("e", 1), ("e", 3), ("e", 5), ("e", 6),
    ];
    let k = 1;

    let mut sites: BTreeMap<&str, HashSet<i32>> = BTreeMap::new();
    for (w, u) in &pairs {
        sites.entry(w).or_default().insert(*u);
    }
    let names: Vec<&str> = sites.keys().cloned().collect();

    let mut scored: Vec<(f64, &str, &str)> = Vec::new();
    for i in 0..names.len() {
        for j in i + 1..names.len() {
            let a = &sites[names[i]];
            let b = &sites[names[j]];
            let inter = a.intersection(b).count();
            let union = a.union(b).count();
            let jac = if union > 0 { inter as f64 / union as f64 } else { 0.0 };
            scored.push((jac, names[i], names[j]));
        }
    }
    scored.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());

    let out: Vec<String> = scored
        .iter()
        .take(k)
        .map(|(_, a, b)| format!("('{}', '{}')", a, b))
        .collect();
    println!("[{}]", out.join(", "));
}
