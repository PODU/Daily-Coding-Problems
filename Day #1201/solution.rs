// Day 1201: Reverse a directed graph.
// Build reversed adjacency (for each edge u->v add v->u). Time O(V+E), Space O(V+E).
use std::collections::HashMap;

fn reverse_graph(g: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut r: HashMap<String, Vec<String>> = HashMap::new();
    for u in g.keys() {
        r.entry(u.clone()).or_default();
    }
    for (u, vs) in g {
        for v in vs {
            r.entry(v.clone()).or_default().push(u.clone());
        }
    }
    r
}

fn main() {
    let nodes = vec!["A", "B", "C"];
    let mut g: HashMap<String, Vec<String>> = HashMap::new();
    for i in 0..nodes.len() - 1 {
        g.entry(nodes[i].to_string()).or_default().push(nodes[i + 1].to_string());
    }
    reverse_graph(&g);
    println!("{}", nodes.join(" <- "));
}
