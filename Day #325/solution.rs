// Unit conversion via weighted graph; addConversion adds a<->b edges, convert does BFS multiplying ratios.
// Time: O(V+E) per query, Space: O(V+E).
use std::collections::{HashMap, VecDeque};

struct UnitConverter {
    adj: HashMap<String, Vec<(String, f64)>>,
}

impl UnitConverter {
    fn new() -> Self {
        UnitConverter { adj: HashMap::new() }
    }
    fn add_conversion(&mut self, a: &str, b: &str, ratio: f64) {
        self.adj.entry(a.to_string()).or_default().push((b.to_string(), ratio));
        self.adj.entry(b.to_string()).or_default().push((a.to_string(), 1.0 / ratio));
    }
    fn convert(&self, value: f64, from: &str, to: &str) -> f64 {
        if from == to {
            return value;
        }
        let mut dist: HashMap<String, f64> = HashMap::new();
        dist.insert(from.to_string(), value);
        let mut q: VecDeque<String> = VecDeque::new();
        q.push_back(from.to_string());
        while let Some(u) = q.pop_front() {
            let du = dist[&u];
            if let Some(edges) = self.adj.get(&u) {
                for (v, w) in edges {
                    if !dist.contains_key(v) {
                        dist.insert(v.clone(), du * w);
                        if v == to {
                            return dist[v];
                        }
                        q.push_back(v.clone());
                    }
                }
            }
        }
        *dist.get(to).unwrap_or(&f64::NAN)
    }
}

fn main() {
    let mut uc = UnitConverter::new();
    uc.add_conversion("foot", "inch", 12.0);
    uc.add_conversion("yard", "foot", 3.0);
    uc.add_conversion("chain", "yard", 22.0);
    let r = uc.convert(1.0, "yard", "inch");
    println!("1 yard = {:.1} inch", r);
}
