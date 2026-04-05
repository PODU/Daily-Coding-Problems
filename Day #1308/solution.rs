// Minimum cost to connect all houses to plant = Minimum Spanning Tree weight.
// Prim's algorithm with a min-heap. Time O(E log V), Space O(V + E).
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

fn min_cost(g: &HashMap<String, HashMap<String, i64>>) -> i64 {
    if g.is_empty() {
        return 0;
    }
    let start = g.keys().next().unwrap().clone();
    let mut visited: HashSet<String> = HashSet::new();
    let mut heap: BinaryHeap<Reverse<(i64, String)>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));
    let mut total = 0;
    while let Some(Reverse((w, u))) = heap.pop() {
        if visited.contains(&u) {
            continue;
        }
        visited.insert(u.clone());
        total += w;
        if let Some(nbrs) = g.get(&u) {
            for (v, &c) in nbrs {
                if !visited.contains(v) {
                    heap.push(Reverse((c, v.clone())));
                }
            }
        }
    }
    total
}

fn add_edge(g: &mut HashMap<String, HashMap<String, i64>>, a: &str, b: &str, c: i64) {
    g.entry(a.to_string()).or_default().insert(b.to_string(), c);
    g.entry(b.to_string()).or_default().insert(a.to_string(), c);
}

fn main() {
    let mut g: HashMap<String, HashMap<String, i64>> = HashMap::new();
    add_edge(&mut g, "plant", "A", 1);
    add_edge(&mut g, "plant", "B", 5);
    add_edge(&mut g, "plant", "C", 20);
    add_edge(&mut g, "A", "C", 15);
    add_edge(&mut g, "B", "C", 10);
    println!("{}", min_cost(&g)); // 16
}
