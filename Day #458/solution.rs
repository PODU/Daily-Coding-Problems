// Day 458: Validate directional (NE/SW/...) rules for consistency.
// Decompose into x/y strict orders; a cycle in either graph = contradiction.
// Time O(R + V) via DFS cycle detection.
use std::collections::{HashMap, HashSet};

struct Graph {
    adj: HashMap<String, Vec<String>>,
    nodes: HashSet<String>,
}

impl Graph {
    fn new() -> Self {
        Graph { adj: HashMap::new(), nodes: HashSet::new() }
    }
    fn less(&mut self, small: &str, big: &str) {
        self.adj.entry(small.to_string()).or_default().push(big.to_string());
        self.nodes.insert(small.to_string());
        self.nodes.insert(big.to_string());
    }
    fn has_cycle(&self) -> bool {
        let mut color: HashMap<String, u8> = HashMap::new();
        for n in &self.nodes {
            if *color.get(n).unwrap_or(&0) == 0 && self.dfs(n, &mut color) {
                return true;
            }
        }
        false
    }
    fn dfs(&self, u: &str, color: &mut HashMap<String, u8>) -> bool {
        color.insert(u.to_string(), 1);
        if let Some(vs) = self.adj.get(u) {
            for v in vs {
                let cv = *color.get(v).unwrap_or(&0);
                if cv == 1 {
                    return true;
                }
                if cv == 0 && self.dfs(v, color) {
                    return true;
                }
            }
        }
        color.insert(u.to_string(), 2);
        false
    }
}

fn validate(rules: &[&str]) -> bool {
    let mut gx = Graph::new();
    let mut gy = Graph::new();
    for r in rules {
        let p: Vec<&str> = r.split_whitespace().collect();
        let (a, d, b) = (p[0], p[1], p[2]);
        for c in d.chars() {
            match c {
                'N' => gy.less(b, a),
                'S' => gy.less(a, b),
                'E' => gx.less(b, a),
                'W' => gx.less(a, b),
                _ => {}
            }
        }
    }
    !gx.has_cycle() && !gy.has_cycle()
}

fn label(ok: bool) -> &'static str {
    if ok { "Valid" } else { "Invalid" }
}

fn main() {
    println!("{}", label(validate(&["A N B", "B NE C", "C N A"]))); // Invalid
    println!("{}", label(validate(&["A NW B", "A N B"])));          // Valid
}
