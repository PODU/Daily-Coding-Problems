// Reconstruct Itinerary: backtracking DFS over lexicographically sorted edges.
// First complete itinerary using all edges (tried in lex order) is the answer.
// Time: exponential worst case; Space: O(E).
use std::collections::BTreeMap;

fn dfs(
    node: &str,
    adj: &BTreeMap<String, Vec<String>>,
    used: &mut BTreeMap<String, Vec<bool>>,
    total: usize,
    path: &mut Vec<String>,
) -> bool {
    if path.len() == total + 1 {
        return true;
    }
    let dests = match adj.get(node) {
        Some(d) => d.clone(),
        None => return false,
    };
    for i in 0..dests.len() {
        if used[node][i] {
            continue;
        }
        used.get_mut(node).unwrap()[i] = true;
        path.push(dests[i].clone());
        if dfs(&dests[i], adj, used, total, path) {
            return true;
        }
        path.pop();
        used.get_mut(node).unwrap()[i] = false;
    }
    false
}

fn reconstruct(flights: &[(&str, &str)], start: &str) -> Option<Vec<String>> {
    let mut adj: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (a, b) in flights {
        adj.entry(a.to_string()).or_default().push(b.to_string());
    }
    let mut used: BTreeMap<String, Vec<bool>> = BTreeMap::new();
    for (k, v) in adj.iter_mut() {
        v.sort();
        used.insert(k.clone(), vec![false; v.len()]);
    }
    let total = flights.len();
    let mut path = vec![start.to_string()];
    if dfs(start, &adj, &mut used, total, &mut path) {
        Some(path)
    } else {
        None
    }
}

fn fmt(v: &Option<Vec<String>>) -> String {
    match v {
        None => "null".to_string(),
        Some(list) => {
            let parts: Vec<String> = list.iter().map(|x| format!("'{}'", x)).collect();
            format!("[{}]", parts.join(", "))
        }
    }
}

fn main() {
    println!("{}", fmt(&reconstruct(&[("SFO", "HKO"), ("YYZ", "SFO"), ("YUL", "YYZ"), ("HKO", "ORD")], "YUL")));
    println!("{}", fmt(&reconstruct(&[("SFO", "COM"), ("COM", "YYZ")], "COM")));
    println!("{}", fmt(&reconstruct(&[("A", "B"), ("A", "C"), ("B", "C"), ("C", "A")], "A")));
}
