// Dijkstra from node 0; answer is the max shortest-path distance (broadcast time).
// O((V+E) log V).
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let edges = [
        (0, 1, 5), (0, 2, 3), (0, 5, 4), (1, 3, 8), (2, 3, 1), (3, 5, 10), (3, 4, 5),
    ];
    let max_node = edges.iter().map(|&(a, b, _)| a.max(b)).max().unwrap();
    let v = max_node + 1;
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); v];
    for &(a, b, t) in edges.iter() {
        adj[a].push((b, t));
        adj[b].push((a, t));
    }

    let mut dist = vec![i64::MAX; v];
    dist[0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0i64, 0usize)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] {
            continue;
        }
        for &(w, c) in &adj[u] {
            if d + c < dist[w] {
                dist[w] = d + c;
                pq.push(Reverse((dist[w], w)));
            }
        }
    }
    let ans = dist.iter().cloned().max().unwrap();
    println!("{}", ans); // 9
}
