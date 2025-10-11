// Day 407: Minimum Spanning Tree of water pipes (Prim's algorithm).
// Approach: Prim with a binary min-heap over an undirected weighted graph.
// Time: O(E log V), Space: O(V + E). Example MST total cost = 16.
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn minimum_cost(pipes: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let mut adj: HashMap<&str, Vec<(i32, &str)>> = HashMap::new();
    for &node in pipes.keys() {
        adj.entry(node).or_default();
    }
    for (&u, nbrs) in pipes {
        for (&v, &w) in nbrs {
            adj.entry(u).or_default().push((w, v));
            adj.entry(v).or_default().push((w, u));
        }
    }
    if adj.is_empty() {
        return 0;
    }
    let start = *adj.keys().next().unwrap();
    let mut visited: HashSet<&str> = HashSet::new();
    // Reverse for a min-heap on cost.
    let mut pq: BinaryHeap<Reverse<(i32, &str)>> = BinaryHeap::new();
    pq.push(Reverse((0, start)));
    let mut total = 0;
    while let Some(Reverse((cost, node))) = pq.pop() {
        if visited.contains(node) {
            continue;
        }
        visited.insert(node);
        total += cost;
        for &(w, nbr) in &adj[node] {
            if !visited.contains(nbr) {
                pq.push(Reverse((w, nbr)));
            }
        }
    }
    total
}

fn main() {
    let mut pipes: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    pipes.insert("plant", HashMap::from([("A", 1), ("B", 5), ("C", 20)]));
    pipes.insert("A", HashMap::from([("C", 15)]));
    pipes.insert("B", HashMap::from([("C", 10)]));
    pipes.insert("C", HashMap::new());
    println!("{}", minimum_cost(&pipes)); // 16
}
