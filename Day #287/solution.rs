// Top-k similar website pairs by Jaccard similarity of user sets.
// Build per-site user sets, compute Jaccard for all pairs, sort desc (ties alpha), take k.
// Time: O(S^2 * U), Space: O(S*U).
use std::collections::{BTreeMap, HashSet};

fn main() {
    let visits: Vec<(&str, i32)> = vec![
        ("a", 1), ("a", 3), ("a", 5), ("b", 2), ("b", 6),
        ("c", 1), ("c", 2), ("c", 3), ("c", 4), ("c", 5),
        ("d", 4), ("d", 5), ("d", 6), ("d", 7),
        ("e", 1), ("e", 3), ("e", 5), ("e", 6),
    ];
    let k = 1;

    let mut sites: BTreeMap<&str, HashSet<i32>> = BTreeMap::new();
    for (site, user) in &visits {
        sites.entry(site).or_default().insert(*user);
    }
    let names: Vec<&str> = sites.keys().cloned().collect();

    let mut results: Vec<(f64, &str, &str)> = Vec::new();
    for i in 0..names.len() {
        for j in (i + 1)..names.len() {
            let a = &sites[names[i]];
            let b = &sites[names[j]];
            let inter = a.intersection(b).count();
            let uni = a.len() + b.len() - inter;
            let sim = if uni == 0 { 0.0 } else { inter as f64 / uni as f64 };
            results.push((sim, names[i], names[j]));
        }
    }
    results.sort_by(|x, y| {
        y.0.partial_cmp(&x.0).unwrap()
            .then(x.1.cmp(y.1))
            .then(x.2.cmp(y.2))
    });

    let parts: Vec<String> = results.iter().take(k)
        .map(|r| format!("('{}', '{}')", r.1, r.2))
        .collect();
    println!("[{}]", parts.join(", "));
}
