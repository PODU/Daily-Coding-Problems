// Top-k website pairs by Jaccard similarity over user sets; sort desc, tie-break alpha.
// O(W^2 * U) to compare pairs. Output formatted as Python tuple list.
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let visits: Vec<(&str, i32)> = vec![
        ("a", 1), ("a", 3), ("a", 5), ("b", 2), ("b", 6),
        ("c", 1), ("c", 2), ("c", 3), ("c", 4), ("c", 5),
        ("d", 4), ("d", 5), ("d", 6), ("d", 7),
        ("e", 1), ("e", 3), ("e", 5), ("e", 6),
    ];
    let k = 1;

    let mut users: BTreeMap<&str, BTreeSet<i32>> = BTreeMap::new();
    for (site, user) in &visits {
        users.entry(site).or_default().insert(*user);
    }

    let sites: Vec<&str> = users.keys().cloned().collect();
    let mut results: Vec<(f64, &str, &str)> = Vec::new();
    for i in 0..sites.len() {
        for j in (i + 1)..sites.len() {
            let a = &users[sites[i]];
            let b = &users[sites[j]];
            let inter = a.intersection(b).count();
            let uni = a.union(b).count();
            let sim = if uni > 0 { inter as f64 / uni as f64 } else { 0.0 };
            results.push((sim, sites[i], sites[j]));
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
