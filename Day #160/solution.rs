// Day 160: Weighted tree diameter. One DFS; each node returns its longest
// downward branch, while we combine the top two branches. Time O(V+E).
use std::collections::HashMap;

fn dfs(node: &str, parent: &str, adj: &HashMap<String, Vec<(String, i64)>>, best: &mut i64) -> i64 {
    let mut top1 = 0;
    let mut top2 = 0;
    if let Some(neighbors) = adj.get(node) {
        for (nb, w) in neighbors {
            if nb == parent {
                continue;
            }
            let d = w + dfs(nb, node, adj, best);
            if d > top1 {
                top2 = top1;
                top1 = d;
            } else if d > top2 {
                top2 = d;
            }
        }
    }
    *best = (*best).max(top1 + top2);
    top1
}

fn main() {
    let edges = [
        ("a", "b", 3), ("a", "c", 5), ("a", "d", 8),
        ("d", "e", 2), ("d", "f", 4),
        ("e", "g", 1), ("e", "h", 1),
    ];
    let mut adj: HashMap<String, Vec<(String, i64)>> = HashMap::new();
    for (u, v, w) in edges.iter() {
        adj.entry(u.to_string()).or_default().push((v.to_string(), *w));
        adj.entry(v.to_string()).or_default().push((u.to_string(), *w));
    }
    let mut best = 0;
    dfs("a", "", &adj, &mut best);
    println!("{}", best); // 17
}
