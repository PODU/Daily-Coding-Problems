// Unit conversion as a weighted graph; convert via BFS multiplying edge ratios.
// add_conversion(a, b, r): 1 a = r b. Time O(V + E) per query, Space O(V + E).
use std::collections::{HashMap, HashSet, VecDeque};

struct UnitConverter {
    graph: HashMap<String, HashMap<String, f64>>,
}

impl UnitConverter {
    fn new() -> Self {
        UnitConverter { graph: HashMap::new() }
    }

    fn add_conversion(&mut self, frm: &str, to: &str, factor: f64) {
        // 1 frm = factor to
        self.graph.entry(frm.to_string()).or_default().insert(to.to_string(), factor);
        self.graph.entry(to.to_string()).or_default().insert(frm.to_string(), 1.0 / factor);
    }

    fn convert(&self, qty: f64, frm: &str, to: &str) -> Option<f64> {
        if frm == to {
            return Some(qty);
        }
        if !self.graph.contains_key(frm) || !self.graph.contains_key(to) {
            return None;
        }
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(frm.to_string());
        let mut queue: VecDeque<(String, f64)> = VecDeque::new();
        queue.push_back((frm.to_string(), 1.0));
        while let Some((unit, ratio)) = queue.pop_front() {
            if unit == to {
                return Some(qty * ratio);
            }
            if let Some(neighbors) = self.graph.get(&unit) {
                for (nxt, f) in neighbors {
                    if !visited.contains(nxt) {
                        visited.insert(nxt.clone());
                        queue.push_back((nxt.clone(), ratio * f));
                    }
                }
            }
        }
        None
    }
}

fn main() {
    let mut uc = UnitConverter::new();
    uc.add_conversion("foot", "inch", 12.0);   // 12 inches = 1 foot
    uc.add_conversion("yard", "foot", 3.0);    // 3 feet = 1 yard
    uc.add_conversion("chain", "yard", 22.0);  // 22 yards = 1 chain

    let r1 = uc.convert(1.0, "chain", "inch").unwrap();
    let r2 = uc.convert(1.0, "yard", "inch").unwrap();
    println!("1 chain = {} inches", r1.round() as i64);
    println!("1 yard = {} inches", r2.round() as i64);
}
