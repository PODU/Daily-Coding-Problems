// Day 1434: Cheapest flight A->B with at most k connections; print itinerary.
// Approach: Bellman-Ford relaxed k+1 times (k connections = k+1 edges), track parent.
// Time: O((k+1) * E), Space: O(V).
use std::collections::HashMap;

fn cheapest(flights: &[(&str, &str, i32)], a: &str, b: &str, k: i32) -> String {
    let inf = i32::MAX;
    let mut dist: HashMap<&str, i32> = HashMap::new();
    for &(s, d, _) in flights {
        dist.entry(s).or_insert(inf);
        dist.entry(d).or_insert(inf);
    }
    dist.insert(a, 0);
    let mut parent: HashMap<&str, &str> = HashMap::new();
    for _ in 0..=k {
        let mut cur = dist.clone();
        let mut cur_parent = parent.clone();
        for &(s, d, price) in flights {
            let ds = dist[s];
            if ds != inf && ds + price < cur[d] {
                cur.insert(d, ds + price);
                cur_parent.insert(d, s);
            }
        }
        dist = cur;
        parent = cur_parent;
    }
    if dist[b] == inf {
        return "No route".to_string();
    }
    let mut path: Vec<&str> = Vec::new();
    let mut node = b;
    while node != a {
        path.push(node);
        node = parent[node];
    }
    path.push(a);
    path.reverse();
    format!("{}, costing ${}", path.join(" -> "), dist[b])
}

fn main() {
    let flights = [
        ("JFK", "ATL", 150), ("ATL", "SFO", 400), ("ORD", "LAX", 200),
        ("LAX", "DFW", 80), ("JFK", "HKG", 800), ("ATL", "ORD", 90), ("JFK", "LAX", 500),
    ];
    println!("{}", cheapest(&flights, "JFK", "LAX", 3));
}
