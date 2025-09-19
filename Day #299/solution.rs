// Minimum spanning tree cost via Kruskal + union-find over undirected weighted graph.
// Time: O(E log E), Space: O(V + E).
use std::collections::{HashMap, HashSet};

fn find(parent: &mut HashMap<String, String>, x: &str) -> String {
    let mut cur = x.to_string();
    while parent[&cur] != cur {
        let next = parent[&cur].clone();
        let gp = parent[&next].clone();
        parent.insert(cur.clone(), gp);
        cur = next;
    }
    cur
}

fn unite(parent: &mut HashMap<String, String>, a: &str, b: &str) -> bool {
    let ra = find(parent, a);
    let rb = find(parent, b);
    if ra == rb {
        return false;
    }
    parent.insert(ra, rb);
    true
}

fn main() {
    let pipes: Vec<(&str, Vec<(&str, i32)>)> = vec![
        ("plant", vec![("A", 1), ("B", 5), ("C", 20)]),
        ("A", vec![("C", 15)]),
        ("B", vec![("C", 10)]),
        ("C", vec![]),
    ];

    let mut parent: HashMap<String, String> = HashMap::new();
    for (node, _) in &pipes {
        parent.insert(node.to_string(), node.to_string());
    }

    let mut seen: HashSet<(String, String, i32)> = HashSet::new();
    let mut edges: Vec<(i32, String, String)> = Vec::new();
    for (u, nbrs) in &pipes {
        for (v, w) in nbrs {
            let (a, b) = if u < v { (u.to_string(), v.to_string()) } else { (v.to_string(), u.to_string()) };
            let key = (a.clone(), b.clone(), *w);
            if seen.insert(key) {
                edges.push((*w, a, b));
            }
        }
    }
    edges.sort();

    let mut total = 0;
    for (w, a, b) in &edges {
        if unite(&mut parent, a, b) {
            total += w;
        }
    }
    println!("{}", total);
}
