// Group users into a set per site; for every site pair compute Jaccard = |A&B|/|A|B|.
// Sort by similarity DESC, tie-break by pair lexicographically; take top k. Time O(P^2 * U).
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let visits: Vec<(&str, i32)> = vec![
        ("a", 1), ("a", 3), ("a", 5),
        ("b", 2), ("b", 6),
        ("c", 1), ("c", 2), ("c", 3), ("c", 4), ("c", 5),
        ("d", 4), ("d", 5), ("d", 6), ("d", 7),
        ("e", 1), ("e", 3), ("e", 5), ("e", 6),
    ];
    let k = 1usize;

    let mut sites: BTreeMap<String, BTreeSet<i32>> = BTreeMap::new();
    for (site, user) in &visits {
        sites.entry(site.to_string()).or_default().insert(*user);
    }

    let names: Vec<String> = sites.keys().cloned().collect();
    let mut cands: Vec<(f64, String, String)> = Vec::new();
    for i in 0..names.len() {
        for j in (i + 1)..names.len() {
            let a = &sites[&names[i]];
            let b = &sites[&names[j]];
            let inter = a.intersection(b).count();
            let uni = a.union(b).count();
            let sim = if uni == 0 { 0.0 } else { inter as f64 / uni as f64 };
            cands.push((sim, names[i].clone(), names[j].clone()));
        }
    }
    cands.sort_by(|p, q| {
        q.0.partial_cmp(&p.0)
            .unwrap()
            .then(p.1.cmp(&q.1))
            .then(p.2.cmp(&q.2))
    });

    let parts: Vec<String> = cands
        .iter()
        .take(k)
        .map(|(_, x, y)| format!("('{}', '{}')", x, y))
        .collect();
    println!("[{}]", parts.join(", "));
}
