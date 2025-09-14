// Day 270: Broadcast time = max shortest-path distance from node 0 (Dijkstra).
// Min-heap Dijkstra over undirected weighted graph; answer = max finite dist. Time O(E log V), Space O(V+E).
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn network_delay(n: usize, edges: &[(usize, usize, i64)]) -> i64 {
    let mut adj = vec![Vec::new(); n + 1];
    for &(a, b, t) in edges {
        adj[a].push((b, t));
        adj[b].push((a, t));
    }
    let mut dist = vec![i64::MAX; n + 1];
    dist[0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0i64, 0usize)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            if d + w < dist[v] {
                dist[v] = d + w;
                pq.push(Reverse((dist[v], v)));
            }
        }
    }
    *dist.iter().max().unwrap()
}

fn main() {
    let edges = [
        (0, 1, 5), (0, 2, 3), (0, 5, 4),
        (1, 3, 8), (2, 3, 1), (3, 5, 10), (3, 4, 5),
    ];
    println!("{}", network_delay(5, &edges));
}
