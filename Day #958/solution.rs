// Day 958: reconstruct itinerary using every flight once, lexicographically smallest.
// Backtracking DFS over sorted adjacency. Worst O(E!) but fast in practice; Space O(E).
use std::collections::HashMap;

fn dfs(
    node: &str,
    adj: &HashMap<String, Vec<String>>,
    used: &mut HashMap<String, Vec<bool>>,
    total: usize,
    path: &mut Vec<String>,
) -> bool {
    if path.len() == total + 1 {
        return true;
    }
    if let Some(dests) = adj.get(node) {
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
    }
    false
}

fn itinerary(flights: &[(&str, &str)], start: &str) -> Option<Vec<String>> {
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for &(o, d) in flights {
        adj.entry(o.to_string()).or_default().push(d.to_string());
    }
    let mut used: HashMap<String, Vec<bool>> = HashMap::new();
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

fn show(v: &Option<Vec<String>>) -> String {
    match v {
        None => "null".to_string(),
        Some(list) => {
            let parts: Vec<String> = list.iter().map(|x| format!("'{}'", x)).collect();
            format!("[{}]", parts.join(", "))
        }
    }
}

fn main() {
    println!("{}", show(&itinerary(&[("SFO", "HKO"), ("YYZ", "SFO"), ("YUL", "YYZ"), ("HKO", "ORD")], "YUL")));
    println!("{}", show(&itinerary(&[("SFO", "COM"), ("COM", "YYZ")], "COM")));
    println!("{}", show(&itinerary(&[("A", "B"), ("A", "C"), ("B", "C"), ("C", "A")], "A")));
}
