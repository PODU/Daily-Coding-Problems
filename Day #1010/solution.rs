// Unit Converter: graph where edge a->b stores factor (1 a = f b).
// convert() does BFS multiplying factors. Time O(V+E) per query, Space O(V+E).
use std::collections::{HashMap, VecDeque};

struct UnitConverter {
    g: HashMap<String, Vec<(String, f64)>>,
}

impl UnitConverter {
    fn new() -> Self {
        UnitConverter { g: HashMap::new() }
    }

    fn add_unit(&mut self, from: &str, to: &str, factor: f64) {
        self.g.entry(from.to_string()).or_default().push((to.to_string(), factor));
        self.g.entry(to.to_string()).or_default().push((from.to_string(), 1.0 / factor));
    }

    fn convert(&self, value: f64, from: &str, to: &str) -> f64 {
        if from == to {
            return value;
        }
        let mut dist: HashMap<String, f64> = HashMap::new();
        dist.insert(from.to_string(), 1.0);
        let mut q: VecDeque<String> = VecDeque::new();
        q.push_back(from.to_string());
        while let Some(u) = q.pop_front() {
            let du = dist[&u];
            if let Some(edges) = self.g.get(&u) {
                for (v, f) in edges {
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
    uc.add_unit("inch", "foot", 1.0 / 12.0);
    uc.add_unit("foot", "yard", 1.0 / 3.0);
    uc.add_unit("yard", "chain", 1.0 / 22.0);

    println!("1 chain = {} inches", uc.convert(1.0, "chain", "inch").round() as i64);
    println!("36 inches = {} yards", uc.convert(36.0, "inch", "yard").round() as i64);
}
