// Day 427: Shortest uphill-then-downhill route from/to home (location 0).
// State Dijkstra: each node split into up/down phases; switch at the peak.
// Up edges need strictly higher elevation, down edges strictly lower. Time O((V+E)logV).
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let mut elev: HashMap<usize, i64> = HashMap::new();
    for (k, v) in [(0, 5), (1, 25), (2, 15), (3, 20), (4, 10)] {
        elev.insert(k, v);
    }
    let edges = [
        (0usize, 1usize, 10i64), (0, 2, 8), (0, 3, 15), (1, 3, 12),
        (2, 4, 10), (3, 4, 5), (3, 0, 17), (4, 0, 10),
    ];
    let n = elev.len();
    let home = 0usize;
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    for &(u, v, w) in edges.iter() {
        adj[u].push((v, w));
    }
    let s_count = n * 2; // state = node*2 + phase (0 up, 1 down)
    let mut dist = vec![i64::MAX; s_count];
    let mut prev = vec![-1i64; s_count];
    dist[home * 2] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0i64, home * 2)));
    while let Some(Reverse((d, s))) = pq.pop() {
        if d > dist[s] {
            continue;
        }
        let u = s / 2;
        let ph = s % 2;
        if ph == 0 && u != home {
            let ns = u * 2 + 1;
            if d < dist[ns] {
                dist[ns] = d;
                prev[ns] = s as i64;
                pq.push(Reverse((d, ns)));
            }
        }
        for &(v, w) in adj[u].iter() {
            let ns;
            if ph == 0 && elev[&v] > elev[&u] {
                ns = v * 2;
            } else if ph == 1 && elev[&v] < elev[&u] {
                ns = v * 2 + 1;
            } else {
                continue;
            }
            if d + w < dist[ns] {
                dist[ns] = d + w;
                prev[ns] = s as i64;
                pq.push(Reverse((d + w, ns)));
            }
        }
    }
    let goal = home * 2 + 1;
    let mut nodes = Vec::new();
    let mut cur = goal as i64;
    while cur != -1 {
        nodes.push((cur as usize) / 2);
        cur = prev[cur as usize];
    }
    nodes.reverse();
    let mut path: Vec<usize> = Vec::new();
    for x in nodes {
        if path.last() != Some(&x) {
            path.push(x);
        }
    }
    let strs: Vec<String> = path.iter().map(|x| x.to_string()).collect();
    println!("{}, distance {}", strs.join(" -> "), dist[goal]);
}
