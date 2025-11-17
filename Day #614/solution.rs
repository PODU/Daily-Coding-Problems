// Day 614: Time for a message from node 0 to reach all nodes = max shortest-path distance.
// Approach: Dijkstra from node 0, return the largest distance. Time O(E log V), Space O(V+E).
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn broadcast_time(n: usize, edges: &[(usize, usize, i64)]) -> i64 {
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n + 1];
    for &(a, b, t) in edges {
        adj[a].push((b, t));
    }
    let mut dist = vec![i64::MAX; n + 1];
    dist[0] = 0;
    let mut pq: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    pq.push(Reverse((0, 0)));
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
    let n = 5;
    let edges = [
        (0, 1, 5), (0, 2, 3), (0, 5, 4), (1, 3, 8),
        (2, 3, 1), (3, 5, 10), (3, 4, 5),
    ];
    println!("{}", broadcast_time(n, &edges)); // 9
}
