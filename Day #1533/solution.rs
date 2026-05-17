// Minimum spanning tree (Prim's algorithm) over an undirected pipe graph.
// Returns total cost to connect every house to the plant.
// Time O(E log V), Space O(V + E).
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

fn mst_cost(pipes: &[(&str, Vec<(&str, i32)>)]) -> i32 {
    let mut adj: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    for (u, nbrs) in pipes {
        adj.entry(u.to_string()).or_default();
        for (v, w) in nbrs {
            adj.entry(u.to_string()).or_default().push((*w, v.to_string()));
            adj.entry(v.to_string()).or_default().push((*w, u.to_string()));
        }
    }
    let start = pipes[0].0.to_string();
    let mut visited: HashSet<String> = HashSet::new();
    let mut heap: BinaryHeap<Reverse<(i32, String)>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));
    let mut total = 0;
    while let Some(Reverse((w, u))) = heap.pop() {
        if visited.contains(&u) {
            continue;
        }
        visited.insert(u.clone());
        total += w;
        if let Some(nbrs) = adj.get(&u) {
            for (cw, v) in nbrs {
                if !visited.contains(v) {
                    heap.push(Reverse((*cw, v.clone())));
                }
            }
        }
    }
    total
}

fn main() {
    let pipes = vec![
        ("plant", vec![("A", 1), ("B", 5), ("C", 20)]),
        ("A", vec![("C", 15)]),
        ("B", vec![("C", 10)]),
        ("C", vec![]),
    ];
    println!("{}", mst_cost(&pipes));
}
