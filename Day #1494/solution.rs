// Day 1494: Reverse a directed graph by reversing every edge.
// Approach: build a reversed adjacency list (for u->v add v->u). Time O(V+E), Space O(V+E).
use std::collections::HashMap;

fn main() {
    // Input graph: A -> B -> C
    let edges = vec![("A", "B"), ("B", "C")];

    // Build reversed adjacency list.
    let mut rev: HashMap<&str, Vec<&str>> = HashMap::new();
    for &(u, v) in &edges {
        rev.entry(v).or_default().push(u); // v -> u
    }

    println!("Original: A -> B -> C");
    println!("Reversed: A <- B <- C");
    println!("Reversed edges:");
    for &(u, v) in &edges {
        println!("  {} -> {}", v, u);
    }
}
