// Day 1445: Unit converter. Model units as a weighted graph (edge = conversion
// factor) and BFS for a path on query. add_unit O(1); convert O(V+E).
use std::collections::{HashMap, VecDeque};

struct UnitConverter {
    graph: HashMap<String, HashMap<String, f64>>, // graph[a][b] = factor; 1 a = factor b
}

impl UnitConverter {
    fn new() -> Self {
        UnitConverter { graph: HashMap::new() }
    }

    fn add_unit(&mut self, from: &str, to: &str, factor: f64) {
        self.graph.entry(from.to_string()).or_default().insert(to.to_string(), factor);
        self.graph.entry(to.to_string()).or_default().insert(from.to_string(), 1.0 / factor);
    }

    fn convert(&self, value: f64, from: &str, to: &str) -> f64 {
        if from == to {
            return value;
        }
        let mut dist: HashMap<String, f64> = HashMap::new();
        dist.insert(from.to_string(), 1.0);
        let mut q = VecDeque::new();
        q.push_back(from.to_string());
        while let Some(u) = q.pop_front() {
            if let Some(neighbors) = self.graph.get(&u) {
                let du = dist[&u];
                for (v, f) in neighbors {
                    if !dist.contains_key(v) {
                        dist.insert(v.clone(), du * f);
                        if v == to {
                            return value * dist[v];
                        }
                        q.push_back(v.clone());
                    }
                }
            }
        }
        f64::NAN
    }
}

fn main() {
    let mut uc = UnitConverter::new();
    uc.add_unit("foot", "inch", 12.0);
    uc.add_unit("yard", "foot", 3.0);
    uc.add_unit("chain", "yard", 22.0);
    println!("{}", uc.convert(1.0, "yard", "inch") as i64); // 36
}
