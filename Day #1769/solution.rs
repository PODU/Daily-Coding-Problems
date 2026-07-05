// Uphill-then-downhill shortest cyclic route from home (node 0): Dijkstra over
// states (node, phase). UP edges require rising elevation, DOWN edges require
// falling; a free phase switch (the peak) is allowed at non-home nodes.
// Time: O(E log V), Space: O(V+E).
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let elev = [5i64, 25, 15, 20, 10];
    let paths: [(usize, usize, i64); 8] = [
        (0, 1, 10), (0, 2, 8), (0, 3, 15), (1, 3, 12),
        (2, 4, 10), (3, 4, 5), (3, 0, 17), (4, 0, 10),
    ];
    let n = elev.len();
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    for &(u, v, w) in &paths {
        adj[u].push((v, w));
    }

    const INF: i64 = i64::MAX;
    let mut dist = vec![INF; 2 * n];
    let home = 0usize;
    dist[home * 2] = 0;
    let mut pq: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    pq.push(Reverse((0, home * 2)));
    while let Some(Reverse((d, s))) = pq.pop() {
        if d > dist[s] {
            continue;
        }
        let node = s / 2;
        let phase = s % 2;
        if phase == 0 && node != home {
            let ns = node * 2 + 1;
            if d < dist[ns] {
                dist[ns] = d;
                pq.push(Reverse((d, ns)));
            }
        }
        for &(v, w) in &adj[node] {
            if phase == 0 && elev[v] > elev[node] {
                let ns = v * 2;
                if d + w < dist[ns] {
                    dist[ns] = d + w;
                    pq.push(Reverse((d + w, ns)));
                }
            }
            if phase == 1 && elev[v] < elev[node] {
                let ns = v * 2 + 1;
                if d + w < dist[ns] {
                    dist[ns] = d + w;
                    pq.push(Reverse((d + w, ns)));
                }
            }
        }
    }
    println!("{}", dist[home * 2 + 1]);
}
