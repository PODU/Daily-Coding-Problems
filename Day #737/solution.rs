// Topological sort of courses (Kahn's algorithm with cycle detection).
// Lexicographic tie-break via BinaryHeap (Reverse) for deterministic order.
// Time: O((V+E) log V), Space: O(V+E).
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn course_order(prereqs: &HashMap<&str, Vec<&str>>) -> Option<Vec<String>> {
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    let mut indeg: HashMap<String, i32> = HashMap::new();
    for c in prereqs.keys() {
        indeg.entry(c.to_string()).or_insert(0);
    }
    for (course, pres) in prereqs {
        for p in pres {
            adj.entry(p.to_string()).or_default().push(course.to_string());
            *indeg.entry(course.to_string()).or_insert(0) += 1;
            indeg.entry(p.to_string()).or_insert(0);
        }
    }
    let mut heap: BinaryHeap<Reverse<String>> = BinaryHeap::new();
    for (c, &d) in &indeg {
        if d == 0 {
            heap.push(Reverse(c.clone()));
        }
    }
    let mut order = Vec::new();
    while let Some(Reverse(c)) = heap.pop() {
        order.push(c.clone());
        if let Some(neighbors) = adj.get(&c) {
            for nx in neighbors {
                let e = indeg.get_mut(nx).unwrap();
                *e -= 1;
                if *e == 0 {
                    heap.push(Reverse(nx.clone()));
                }
            }
        }
    }
    if order.len() == indeg.len() {
        Some(order)
    } else {
        None
    }
}

fn main() {
    let mut prereqs: HashMap<&str, Vec<&str>> = HashMap::new();
    prereqs.insert("CSC300", vec!["CSC100", "CSC200"]);
    prereqs.insert("CSC200", vec!["CSC100"]);
    prereqs.insert("CSC100", vec![]);
    match course_order(&prereqs) {
        Some(order) => println!("['{}']", order.join("', '")), // ['CSC100', 'CSC200', 'CSC300']
        None => println!("null"),
    }
}
