// Day 700: Cheapest itinerary with at most k connections (k+1 flights).
// Approach: Bellman-Ford bounded to k+1 edges (relax with previous layer's dist),
// track predecessors to rebuild the route. Time O((k+1)*E), Space O(V).
use std::collections::HashMap;

fn cheapest(
    flights: &[(&str, &str, i32)],
    src: &str,
    dst: &str,
    k: usize,
) -> (i32, Vec<String>) {
    let mut dist: HashMap<String, i32> = HashMap::new();
    let mut par: HashMap<String, String> = HashMap::new();
    dist.insert(src.to_string(), 0);
    for _ in 0..=k {
        // up to k+1 edges
        let mut nd = dist.clone();
        let mut np = par.clone();
        for &(u, v, w) in flights {
            if let Some(&du) = dist.get(u) {
                let cand = du + w;
                if nd.get(v).map_or(true, |&c| cand < c) {
                    nd.insert(v.to_string(), cand);
                    np.insert(v.to_string(), u.to_string());
                }
            }
        }
        dist = nd;
        par = np;
    }
    if !dist.contains_key(dst) {
        return (-1, vec![]);
    }
    let mut path = Vec::new();
    let mut cur = dst.to_string();
    while cur != src {
        path.push(cur.clone());
        cur = par[&cur].clone();
    }
    path.push(src.to_string());
    path.reverse();
    (dist[dst], path)
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
