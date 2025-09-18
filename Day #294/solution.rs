// Uphill-then-downhill closed route: Dijkstra on up-edges from 0, Dijkstra on reversed down-edges
// to 0; answer = min over peaks of distUp[m]+distDown[m]. Time O((V+E)logV), Space O(V+E).
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn dijkstra(n: usize, adj: &Vec<Vec<(usize, i64)>>, src: usize) -> Vec<i64> {
    let mut d = vec![i64::MAX; n];
    d[src] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0i64, src)));
    while let Some(Reverse((du, u))) = pq.pop() {
        if du > d[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            if d[u] + w < d[v] {
                d[v] = d[u] + w;
                pq.push(Reverse((d[v], v)));
            }
        }
    }
    d
}

fn main() {
    let n = 5;
    let elev = [5i64, 25, 15, 20, 10];
    let paths = [(0, 1, 10), (0, 2, 8), (0, 3, 15), (1, 3, 12),
                 (2, 4, 10), (3, 4, 5), (3, 0, 17), (4, 0, 10)];
    let mut up: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    let mut down_rev: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    for &(u, v, w) in paths.iter() {
        if elev[v] > elev[u] {
            up[u].push((v, w));
        }
        if elev[v] < elev[u] {
            down_rev[v].push((u, w)); // reversed for m->0 search
        }
    }
    let dist_up = dijkstra(n, &up, 0);
    let dist_down = dijkstra(n, &down_rev, 0);
    let mut best = i64::MAX;
    for m in 1..n {
        if dist_up[m] != i64::MAX && dist_down[m] != i64::MAX {
            best = best.min(dist_up[m] + dist_down[m]);
        }
    }
    println!("{}", best);
}
