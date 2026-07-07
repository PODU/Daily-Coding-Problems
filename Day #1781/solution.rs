// Unit converter as a graph: add_relation stores 1 a = factor b (edge a->b, b->a=1/factor).
// convert does BFS multiplying factors along the path. Time O(V+E) per query, Space O(V+E).
use std::collections::{HashMap, VecDeque};

struct UnitConverter {
    adj: HashMap<String, HashMap<String, f64>>,
}

impl UnitConverter {
    fn new() -> Self {
        UnitConverter { adj: HashMap::new() }
    }
    fn add_relation(&mut self, a: &str, b: &str, factor: f64) {
        self.adj.entry(a.to_string()).or_default().insert(b.to_string(), factor);
        self.adj.entry(b.to_string()).or_default().insert(a.to_string(), 1.0 / factor);
    }
    fn convert(&self, qty: f64, from: &str, to: &str) -> Option<f64> {
        if from == to {
            return Some(qty);
        }
        if !self.adj.contains_key(from) || !self.adj.contains_key(to) {
            return None;
        }
        let mut dist: HashMap<String, f64> = HashMap::new();
        dist.insert(from.to_string(), qty);
        let mut q: VecDeque<String> = VecDeque::new();
        q.push_back(from.to_string());
        while let Some(u) = q.pop_front() {
            let cur = dist[&u];
            if let Some(nbrs) = self.adj.get(&u) {
                for (nxt, f) in nbrs {
                    if !dist.contains_key(nxt) {
                        let v = cur * f;
                        dist.insert(nxt.clone(), v);
                        if nxt == to {
                            return Some(v);
                        }
                        q.push_back(nxt.clone());
                    }
                }
            }
        }
        None
    }
}

fn main() {
    let mut uc = UnitConverter::new();
    uc.add_relation("inches", "foot", 1.0 / 12.0);
    uc.add_relation("feet", "yard", 1.0 / 3.0);
    uc.add_relation("yards", "chain", 1.0 / 22.0);
    uc.add_relation("foot", "feet", 1.0);
    uc.add_relation("yard", "yards", 1.0);

    println!("1 yard = {:.0} inches", uc.convert(1.0, "yard", "inches").unwrap());
    println!("1 chain = {:.0} inches", uc.convert(1.0, "chain", "inches").unwrap());
    println!("1 chain = {:.0} feet", uc.convert(1.0, "chain", "feet").unwrap());
}
