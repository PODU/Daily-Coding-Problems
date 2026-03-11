// Dijkstra from node 0 over nodes 0..N (undirected); answer = max finite shortest-path distance.
// Time: O(E log V), Space: O(V + E).
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn network_delay(n: usize, edges: &[(usize, usize, i64)]) -> i64 {
    let v = n + 1;
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); v];
    for &(a, b, t) in edges {
        adj[a].push((b, t));
        adj[b].push((a, t));
    }
    let inf = i64::MAX;
    let mut dist = vec![inf; v];
    dist[0] = 0;
    let mut pq: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    pq.push(Reverse((0, 0)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] {
            continue;
        }
        for &(vv, w) in &adj[u] {
            if dist[u] + w < dist[vv] {
                dist[vv] = dist[u] + w;
                pq.push(Reverse((dist[vv], vv)));
            }
        }
    }
    dist.iter().filter(|&&d| d != inf).cloned().max().unwrap_or(0)
}

fn main() {
    let n = 5;
    let edges = [(0, 1, 5), (0, 2, 3), (0, 5, 4), (1, 3, 8), (2, 3, 1), (3, 5, 10), (3, 4, 5)];
    println!("{}", network_delay(n, &edges));
}
