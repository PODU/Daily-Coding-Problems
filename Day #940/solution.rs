// Day 940: Time for a message from node 0 to reach all = max shortest-path distance (Dijkstra).
// Time O(E log V), Space O(V + E). Returns -1 if some node is unreachable.
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn network_delay(n: usize, edges: &[(usize, usize, i64)], src: usize) -> i64 {
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n + 1];
    for &(a, b, t) in edges {
        adj[a].push((b, t));
    }
    let inf = i64::MAX;
    let mut dist = vec![inf; n + 1];
    dist[src] = 0;
    let mut pq: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    pq.push(Reverse((0, src)));
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
    let mut ans = 0;
    for &d in &dist {
        if d == inf {
            return -1;
        }
        ans = ans.max(d);
    }
    ans
}

fn main() {
    let edges = [
        (0, 1, 5), (0, 2, 3), (0, 5, 4),
        (1, 3, 8), (2, 3, 1), (3, 5, 10), (3, 4, 5),
    ];
    println!("{}", network_delay(5, &edges, 0)); // 9
}
