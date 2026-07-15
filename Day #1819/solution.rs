// Longest weighted path (diameter) in a tree via two DFS passes.
// DFS from any node finds one endpoint; DFS from it finds the other + path. Time: O(V+E). Space: O(V).
use std::collections::HashMap;

type Graph = HashMap<String, Vec<(String, i64)>>;

fn add(g: &mut Graph, a: &str, b: &str, w: i64) {
    g.entry(a.to_string()).or_default().push((b.to_string(), w));
    g.entry(b.to_string()).or_default().push((a.to_string(), w));
}

fn farthest(g: &Graph, src: &str) -> (String, i64, HashMap<String, String>) {
    let mut parent: HashMap<String, String> = HashMap::new();
    let mut visited: HashMap<String, bool> = HashMap::new();
    let mut best_node = src.to_string();
    let mut best_dist = 0i64;
    // iterative DFS
    let mut stack: Vec<(String, i64, Option<String>)> = vec![(src.to_string(), 0, None)];
    while let Some((u, d, par)) = stack.pop() {
        if visited.contains_key(&u) {
            continue;
        }
        visited.insert(u.clone(), true);
        if let Some(p) = par {
            parent.insert(u.clone(), p);
        }
        if d > best_dist {
            best_dist = d;
            best_node = u.clone();
        }
        if let Some(edges) = g.get(&u) {
            for (v, w) in edges {
                if !visited.contains_key(v) {
                    stack.push((v.clone(), d + w, Some(u.clone())));
                }
            }
        }
    }
    (best_node, best_dist, parent)
}

fn main() {
    let mut g: Graph = HashMap::new();
    add(&mut g, "a", "b", 3);
    add(&mut g, "a", "c", 5);
    add(&mut g, "a", "d", 8);
    add(&mut g, "d", "e", 2);
    add(&mut g, "d", "f", 4);
    add(&mut g, "e", "g", 1);
    add(&mut g, "e", "h", 1);

    let (u, _, _) = farthest(&g, "a");
    let (v, length, parent) = farthest(&g, &u);

    let mut path = Vec::new();
    let mut cur = v;
    loop {
        path.push(cur.clone());
        if cur == u {
            break;
        }
        cur = parent[&cur].clone();
    }
    println!("{}, with a length of {}", path.join(" -> "), length);
    // c -> a -> d -> f, with a length of 17
}
