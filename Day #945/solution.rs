// Day 945: Longest path (diameter) in a weighted tree.
// DFS: at each node combine its two deepest downward branches. Time O(V+E), Space O(V).
use std::collections::HashMap;

fn add_edge<'a>(adj: &mut HashMap<&'a str, Vec<(&'a str, i32)>>, u: &'a str, v: &'a str, w: i32) {
    adj.entry(u).or_default().push((v, w));
    adj.entry(v).or_default().push((u, w));
}

// Returns the longest downward path length; updates `best` with paths through each node.
fn dfs(adj: &HashMap<&str, Vec<(&str, i32)>>, node: &str, parent: &str, best: &mut i32) -> i32 {
    let mut max1 = 0;
    let mut max2 = 0;
    if let Some(neighbors) = adj.get(node) {
        for &(nb, w) in neighbors {
            if nb == parent {
                continue;
            }
            let d = dfs(adj, nb, node, best) + w;
            if d > max1 {
                max2 = max1;
                max1 = d;
            } else if d > max2 {
                max2 = d;
            }
        }
    }
    *best = (*best).max(max1 + max2);
    max1
}

fn main() {
    let mut adj: HashMap<&str, Vec<(&str, i32)>> = HashMap::new();
    add_edge(&mut adj, "a", "b", 3);
    add_edge(&mut adj, "a", "c", 5);
    add_edge(&mut adj, "a", "d", 8);
    add_edge(&mut adj, "d", "e", 2);
    add_edge(&mut adj, "d", "f", 4);
    add_edge(&mut adj, "e", "g", 1);
    add_edge(&mut adj, "e", "h", 1);
    let mut best = 0;
    dfs(&adj, "a", "", &mut best);
    println!("{}", best); // 17 (path c -> a -> d -> f)
}
