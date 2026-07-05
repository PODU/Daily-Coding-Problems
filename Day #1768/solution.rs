// Direction-rule consistency: decompose each rule into strict x/y "greater-than"
// edges; a contradiction is a cycle in either axis graph (DFS cycle detection).
// Time: O(V+E) per axis, Space: O(V+E).
use std::collections::{BTreeSet, HashMap};

struct Checker {
    gx: HashMap<String, Vec<String>>,
    gy: HashMap<String, Vec<String>>,
    nodes: BTreeSet<String>,
}

impl Checker {
    fn new() -> Self {
        Checker { gx: HashMap::new(), gy: HashMap::new(), nodes: BTreeSet::new() }
    }
    fn edge(g: &mut HashMap<String, Vec<String>>, nodes: &mut BTreeSet<String>, a: &str, b: &str) {
        g.entry(a.to_string()).or_default().push(b.to_string());
        nodes.insert(a.to_string());
        nodes.insert(b.to_string());
    }
    fn add_rule(&mut self, a: &str, dir: &str, b: &str) {
        for c in dir.chars() {
            match c {
                'N' => Self::edge(&mut self.gy, &mut self.nodes, a, b),
                'S' => Self::edge(&mut self.gy, &mut self.nodes, b, a),
                'E' => Self::edge(&mut self.gx, &mut self.nodes, a, b),
                'W' => Self::edge(&mut self.gx, &mut self.nodes, b, a),
                _ => {}
            }
        }
    }
    fn dfs(g: &HashMap<String, Vec<String>>, u: &str, state: &mut HashMap<String, u8>) -> bool {
        state.insert(u.to_string(), 1);
        if let Some(adj) = g.get(u) {
            for v in adj {
                match state.get(v).copied().unwrap_or(0) {
                    1 => return true,
                    0 => {
                        if Self::dfs(g, v, state) {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        }
        state.insert(u.to_string(), 2);
        false
    }
    fn has_cycle(&self, g: &HashMap<String, Vec<String>>) -> bool {
        let mut state: HashMap<String, u8> = HashMap::new();
        for n in &self.nodes {
            if state.get(n).copied().unwrap_or(0) == 0 && Self::dfs(g, n, &mut state) {
                return true;
            }
        }
        false
    }
    fn validates(&self) -> bool {
        !self.has_cycle(&self.gx) && !self.has_cycle(&self.gy)
    }
}

fn main() {
    let mut c1 = Checker::new();
    c1.add_rule("A", "N", "B");
    c1.add_rule("B", "NE", "C");
    c1.add_rule("C", "N", "A");
    if !c1.validates() {
        println!("does not validate, since A cannot be both north and south of C.");
    }

    let mut c2 = Checker::new();
    c2.add_rule("A", "NW", "B");
    c2.add_rule("A", "N", "B");
    if c2.validates() {
        println!("A NW B / A N B is considered valid.");
    }
}
