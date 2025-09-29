// Day 346: Cheapest itinerary with up to k connections.
// Bellman-Ford limited to k+1 edges, tracking the path. Time O(k*E), Space O(V).
use std::collections::HashMap;

#[derive(Clone)]
struct Entry { cost: i32, path: Vec<String> }

fn cheapest(flights: &[(&str, &str, i32)], src: &str, dst: &str, k: usize) -> Option<Entry> {
    let mut best: HashMap<String, Entry> = HashMap::new();
    best.insert(src.to_string(), Entry { cost: 0, path: vec![src.to_string()] });
    for _ in 0..=k {
        let mut nxt = best.clone();
        for &(u, v, w) in flights {
            if let Some(pu) = best.get(u) {
                let cost = pu.cost + w;
                let better = match nxt.get(v) { Some(c) => cost < c.cost, None => true };
                if better {
                    let mut p = pu.path.clone();
                    p.push(v.to_string());
                    nxt.insert(v.to_string(), Entry { cost, path: p });
                }
            }
        }
        best = nxt;
    }
    best.get(dst).cloned()
}

fn main() {
    let flights = [
        ("JFK", "ATL", 150), ("ATL", "SFO", 400), ("ORD", "LAX", 200),
        ("LAX", "DFW", 80), ("JFK", "HKG", 800), ("ATL", "ORD", 90), ("JFK", "LAX", 500),
    ];
    let res = cheapest(&flights, "JFK", "LAX", 3).unwrap();
    println!("{}, costing ${}.", res.path.join(" -> "), res.cost);
}
