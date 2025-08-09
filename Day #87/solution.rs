// Day 87: Validate directional rules. N/S -> vertical graph, E/W -> horizontal graph,
// edge u->v means u strictly greater on that axis. A contradiction is a cycle. Time O(V+E).
use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, HashSet<String>>;

fn edge(g: &mut Graph, a: &str, b: &str) {
    g.entry(a.to_string()).or_default().insert(b.to_string());
    g.entry(b.to_string()).or_default();
}

fn dfs(u: &str, g: &Graph, state: &mut HashMap<String, u8>) -> bool {
    state.insert(u.to_string(), 1);
    if let Some(neighbors) = g.get(u) {
        for v in neighbors {
            match state.get(v).copied().unwrap_or(0) {
                1 => return true,
                0 if dfs(v, g, state) => return true,
                _ => {}
            }
        }
    }
    state.insert(u.to_string(), 2);
    false
}

fn has_cycle(g: &Graph) -> bool {
    let mut state: HashMap<String, u8> = HashMap::new();
    for u in g.keys() {
        if state.get(u).copied().unwrap_or(0) == 0 && dfs(u, g, &mut state) {
            return true;
        }
    }
    false
}

fn validate(rules: &[(&str, &str, &str)]) -> bool {
    let mut vert: Graph = HashMap::new();
    let mut horiz: Graph = HashMap::new();
    for &(a, dir, b) in rules {
        for c in dir.chars() {
            match c {
                'N' => edge(&mut vert, a, b),
                'S' => edge(&mut vert, b, a),
                'E' => edge(&mut horiz, a, b),
                'W' => edge(&mut horiz, b, a),
                _ => {}
            }
        }
    }
    !has_cycle(&vert) && !has_cycle(&horiz)
}

fn main() {
    let rules = [("A", "N", "B"), ("B", "NE", "C"), ("C", "N", "A")];
    println!("{}", if validate(&rules) { "validates" } else { "does not validate" });
    // does not validate
}
