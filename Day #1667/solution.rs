// Day 1667: Course ordering via topological sort (Kahn's algorithm).
// Time O(V+E + V log V), Space O(V+E). Returns None if a cycle exists.
use std::collections::{BTreeMap, BinaryHeap};
use std::cmp::Reverse;

fn course_order(prereqs: &BTreeMap<String, Vec<String>>) -> Option<Vec<String>> {
    let mut indeg: BTreeMap<String, i32> = BTreeMap::new();
    let mut adj: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (course, deps) in prereqs {
        indeg.entry(course.clone()).or_insert(0);
        for d in deps {
            indeg.entry(d.clone()).or_insert(0);
        }
    }
    for (course, deps) in prereqs {
        for d in deps {
            adj.entry(d.clone()).or_default().push(course.clone());
            *indeg.get_mut(course).unwrap() += 1;
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
        if let Some(nexts) = adj.get(&c) {
            for nxt in nexts.clone() {
                let e = indeg.get_mut(&nxt).unwrap();
                *e -= 1;
                if *e == 0 {
                    heap.push(Reverse(nxt));
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
    let mut g: BTreeMap<String, Vec<String>> = BTreeMap::new();
    g.insert("CSC300".into(), vec!["CSC100".into(), "CSC200".into()]);
    g.insert("CSC200".into(), vec!["CSC100".into()]);
    g.insert("CSC100".into(), vec![]);
    println!("{:?}", course_order(&g)); // Some(["CSC100", "CSC200", "CSC300"])
}
