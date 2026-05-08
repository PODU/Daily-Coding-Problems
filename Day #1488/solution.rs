// Day 1488: Topological sort of courses via Kahn's algorithm (BFS on in-degrees).
// Returns Some(ordering), or None if a cycle exists. Time O(V+E), Space O(V+E).
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

fn topo_sort(prereqs: &HashMap<&str, Vec<&str>>) -> Option<Vec<String>> {
    let mut indeg: HashMap<String, i32> = HashMap::new();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();

    for (course, ps) in prereqs {
        indeg.entry((*course).to_string()).or_insert(0);
        for p in ps {
            indeg.entry((*p).to_string()).or_insert(0);
        }
    }
    for (course, ps) in prereqs {
        for p in ps {
            adj.entry((*p).to_string()).or_default().push((*course).to_string());
            *indeg.get_mut(*course).unwrap() += 1;
        }
    }

    // Min-heap via Reverse for deterministic lexicographic output.
    let mut heap: BinaryHeap<Reverse<String>> = BinaryHeap::new();
    for (c, d) in &indeg {
        if *d == 0 {
            heap.push(Reverse(c.clone()));
        }
    }

    let mut order = Vec::new();
    while let Some(Reverse(c)) = heap.pop() {
        if let Some(nbrs) = adj.get(&c) {
            for nxt in nbrs.clone() {
                let e = indeg.get_mut(&nxt).unwrap();
                *e -= 1;
                if *e == 0 {
                    heap.push(Reverse(nxt));
                }
            }
        }
        order.push(c);
    }

    if order.len() == indeg.len() { Some(order) } else { None }
}

fn main() {
    let mut prereqs: HashMap<&str, Vec<&str>> = HashMap::new();
    prereqs.insert("CSC300", vec!["CSC100", "CSC200"]);
    prereqs.insert("CSC200", vec!["CSC100"]);
    prereqs.insert("CSC100", vec![]);

    match topo_sort(&prereqs) {
        None => println!("null"),
        Some(order) => {
            let parts: Vec<String> = order.iter().map(|c| format!("'{}'", c)).collect();
            println!("[{}]", parts.join(", "));
        }
    }
}
