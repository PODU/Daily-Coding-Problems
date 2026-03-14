// Day 1206: Validate directional (N/S/E/W) rules for consistency.
// Split into vertical & horizontal strict-order graphs; a cycle = contradiction. Time O(V+E), Space O(V+E).
use std::collections::HashMap;

type Graph = HashMap<String, Vec<String>>;

fn add(g: &mut Graph, u: &str, v: &str) {
    g.entry(u.to_string()).or_default().push(v.to_string());
    g.entry(v.to_string()).or_default();
}

fn has_cycle(g: &Graph) -> bool {
    let mut color: HashMap<String, u8> = HashMap::new();
    fn dfs(u: &str, g: &Graph, color: &mut HashMap<String, u8>) -> bool {
        color.insert(u.to_string(), 1);
        if let Some(adj) = g.get(u) {
            for v in adj {
                match color.get(v).copied().unwrap_or(0) {
                    1 => return true,
                    0 => if dfs(v, g, color) { return true; },
                    _ => {}
                }
            }
        }
        color.insert(u.to_string(), 2);
        false
    }
    for u in g.keys() {
        if color.get(u).copied().unwrap_or(0) == 0 && dfs(u, g, &mut color) {
            return true;
        }
    }
    false
}

fn validate(rules: &[(&str, &str, &str)]) -> bool {
    let mut gy = Graph::new();
    let mut gx = Graph::new();
    for &(a, d, b) in rules {
        if d.contains('N') { add(&mut gy, a, b); }
        if d.contains('S') { add(&mut gy, b, a); }
        if d.contains('E') { add(&mut gx, a, b); }
        if d.contains('W') { add(&mut gx, b, a); }
    }
    !has_cycle(&gy) && !has_cycle(&gx)
}

fn main() {
    let rules = [("A", "N", "B"), ("B", "NE", "C"), ("C", "N", "A")];
    println!("{}", if validate(&rules) { "validates" } else { "does not validate" });
}
