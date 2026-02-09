// Day 1053: Directional consistency. Decompose each rule into strict x/y
// inequalities, build a directed "greater-than" graph per axis, and detect a
// cycle (contradiction) via DFS. Time O(V+E), Space O(V+E).

use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, HashSet<String>>;

fn add(g: &mut Graph, u: &str, v: &str) {
    g.entry(u.to_string()).or_default().insert(v.to_string());
    g.entry(v.to_string()).or_default();
}

fn dfs(g: &Graph, u: &str, state: &mut HashMap<String, i32>) -> bool {
    state.insert(u.to_string(), 0);
    if let Some(adj) = g.get(u) {
        for w in adj {
            match state.get(w) {
                Some(&0) => return true,
                None => {
                    if dfs(g, w, state) {
                        return true;
                    }
                }
                _ => {}
            }
        }
    }
    state.insert(u.to_string(), 1);
    false
}

fn has_cycle(g: &Graph) -> bool {
    let mut state: HashMap<String, i32> = HashMap::new();
    for n in g.keys() {
        if !state.contains_key(n) && dfs(g, n, &mut state) {
            return true;
        }
    }
    false
}

fn validate(rules: &[&str]) -> bool {
    let mut gx = Graph::new();
    let mut gy = Graph::new();
    for rule in rules {
        let p: Vec<&str> = rule.split_whitespace().collect();
        let (a, d, b) = (p[0], p[1], p[2]);
        for ch in d.chars() {
            match ch {
                'N' => add(&mut gy, a, b),
                'S' => add(&mut gy, b, a),
                'E' => add(&mut gx, a, b),
                'W' => add(&mut gx, b, a),
                _ => {}
            }
        }
    }
    !(has_cycle(&gx) || has_cycle(&gy))
}

fn main() {
    let ex1 = ["A N B", "B NE C", "C N A"];
    let ex2 = ["A NW B", "A N B"];
    if !validate(&ex1) {
        println!("does not validate, since A cannot be both north and south of C.");
    } else {
        println!("is considered valid.");
    }
    if validate(&ex2) {
        println!("is considered valid.");
    } else {
        println!("does not validate.");
    }
}
