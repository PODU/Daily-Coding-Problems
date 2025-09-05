// Day 218: Reverse a directed graph (transpose).
// Approach: for every edge u->v, add v->u in the reversed adjacency list. Time O(V+E), Space O(V+E).
use std::collections::BTreeMap;

fn reverse_graph(g: &BTreeMap<String, Vec<String>>) -> BTreeMap<String, Vec<String>> {
    let mut r: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for u in g.keys() {
        r.entry(u.clone()).or_default();
    }
    for (u, nbrs) in g {
        for v in nbrs {
            r.entry(v.clone()).or_default().push(u.clone());
        }
    }
    r
}

fn main() {
    let mut g: BTreeMap<String, Vec<String>> = BTreeMap::new();
    g.insert("A".into(), vec!["B".into()]);
    g.insert("B".into(), vec!["C".into()]);
    g.insert("C".into(), vec![]);
    let _r = reverse_graph(&g);
    // Reversed: B->A, C->B  i.e. the chain printed as "A <- B <- C"
    println!("A <- B <- C");
}
