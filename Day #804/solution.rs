// Day 804: Cheapest fare from A to B with <= k connections (<= k+1 flights).
// Bellman-Ford limited to k+1 relaxation rounds, tracking parents for itinerary.
// Time O((k+1) * E), Space O(V + E).
use std::collections::HashMap;

fn cheapest(flights: &[(&str, &str, i32)], a: &str, b: &str, k: i32)
    -> (i32, Vec<String>) {
    const INF: i32 = i32::MAX;
    let mut dist: HashMap<&str, i32> = HashMap::new();
    for &(s, d, _) in flights {
        dist.insert(s, INF);
        dist.insert(d, INF);
    }
    dist.insert(a, 0);
    let mut parent: HashMap<&str, &str> = HashMap::new();
    for _ in 0..=k {
        let snap = dist.clone();
        for &(s, d, p) in flights {
            if snap[s] != INF && snap[s] + p < dist[d] {
                dist.insert(d, snap[s] + p);
                parent.insert(d, s);
            }
        }
    }
    if dist[b] == INF {
        return (-1, vec![]);
    }
    let mut path = vec![];
    let mut c = b;
    loop {
        path.push(c.to_string());
        if c == a {
            break;
        }
        c = parent[c];
    }
    path.reverse();
    (dist[b], path)
}

fn main() {
    let flights = [
        ("JFK", "ATL", 150), ("ATL", "SFO", 400), ("ORD", "LAX", 200),
        ("LAX", "DFW", 80), ("JFK", "HKG", 800), ("ATL", "ORD", 90),
        ("JFK", "LAX", 500),
    ];
    let (cost, path) = cheapest(&flights, "JFK", "LAX", 3);
    println!("{}, costing ${}", path.join(" -> "), cost);
    // JFK -> ATL -> ORD -> LAX, costing $440
}
