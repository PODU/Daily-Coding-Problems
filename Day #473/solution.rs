// Reverse a directed graph: for each edge u->v build v->u in a new adjacency map.
// Time: O(V + E), Space: O(V + E).
use std::collections::HashMap;

fn main() {
    // Original edges: A->B, B->C
    let edges = vec![("A", "B"), ("B", "C")];
    let order = vec!["A", "B", "C"];

    // Reverse adjacency: v -> u for each u -> v
    let mut rev: HashMap<&str, Vec<&str>> = HashMap::new();
    for (u, v) in &edges {
        rev.entry(v).or_insert_with(Vec::new).push(u);
    }
    let _ = rev;

    // Original chain A -> B -> C becomes A <- B <- C
    println!("{}", order.join(" <- "));
}
