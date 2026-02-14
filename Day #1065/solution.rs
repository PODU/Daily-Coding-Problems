// Day 1065: Reverse a directed graph (reverse every edge).
// Approach: iterate all edges u->v, add v->u to reversed adjacency. Time O(V+E), Space O(V+E).
use std::collections::BTreeMap;

fn reverse_graph(g: &BTreeMap<String, Vec<String>>) -> BTreeMap<String, Vec<String>> {
    let mut r: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for u in g.keys() {
        r.entry(u.clone()).or_default(); // keep isolated nodes
    }
    for (u, neighbors) in g {
        for v in neighbors {
            r.entry(v.clone()).or_default().push(u.clone());
        }
    }
    r
}

fn main() {
    // A -> B -> C
    let mut g: BTreeMap<String, Vec<String>> = BTreeMap::new();
    g.insert("A".to_string(), vec!["B".to_string()]);
    g.insert("B".to_string(), vec!["C".to_string()]);
    g.insert("C".to_string(), vec![]);

    let r = reverse_graph(&g);
    // Reversed: B -> A, C -> B  ("A <- B <- C")
    println!("A <- B <- C");
    for (u, neighbors) in &r {
        for v in neighbors {
            println!("{} -> {}", u, v);
        }
    }
}
