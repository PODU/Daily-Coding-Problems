// Day 1724: Cheapest itinerary with at most k connections (k flights/edges).
// Bellman-Ford limited to k relaxation rounds; track parents to reconstruct path.
// Time: O(k * E), Space: O(V).
use std::collections::HashMap;

fn main() {
    let flights: Vec<(&str, &str, i64)> = vec![
        ("JFK", "ATL", 150), ("ATL", "SFO", 400), ("ORD", "LAX", 200),
        ("LAX", "DFW", 80),  ("JFK", "HKG", 800), ("ATL", "ORD", 90),
        ("JFK", "LAX", 500),
    ];
    let (src, dst, k) = ("JFK", "LAX", 3);

    let inf = i64::MAX;
    let mut dist: HashMap<&str, i64> = HashMap::new();
    let mut parent: HashMap<&str, &str> = HashMap::new();
    for &(u, v, _) in &flights {
        dist.entry(u).or_insert(inf);
        dist.entry(v).or_insert(inf);
    }
    dist.insert(src, 0);

    // Relax all edges k times over a snapshot to bound edge count.
    for _ in 0..k {
        let snap = dist.clone();
        for &(u, v, w) in &flights {
            let du = snap[u];
            if du != inf && du + w < dist[v] {
                dist.insert(v, du + w);
                parent.insert(v, u);
            }
        }
    }

    if dist[dst] == inf {
        println!("No route");
        return;
    }
    let mut path: Vec<&str> = Vec::new();
    let mut at = dst;
    loop {
        path.push(at);
        if at == src {
            break;
        }
        at = parent[at];
    }
    path.reverse();
    println!("{}, costing ${}", path.join(" -> "), dist[dst]);
}
