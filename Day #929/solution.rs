// Shortest uphill-then-downhill cycle from home (node 0).
// Dijkstra on two DAG subgraphs (uphill, reversed downhill), O(E log V) time, O(V+E) space.
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn dijkstra(adj: &Vec<Vec<(usize, i64)>>, src: usize, n: usize) -> Vec<i64> {
    let mut dist = vec![i64::MAX; n];
    dist[src] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0i64, src)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] { continue; }
        for &(v, w) in &adj[u] {
            if d + w < dist[v] {
                dist[v] = d + w;
                pq.push(Reverse((dist[v], v)));
            }
        }
    }
    dist
}

fn main() {
    let elevations = [(0, 5), (1, 25), (2, 15), (3, 20), (4, 10)];
    let elev = |i: usize| -> i64 { elevations.iter().find(|&&(k, _)| k == i).unwrap().1 };
    let paths: Vec<(usize, usize, i64)> = vec![
        (0, 1, 10), (0, 2, 8), (0, 3, 15), (1, 3, 12),
        (2, 4, 10), (3, 4, 5), (3, 0, 17), (4, 0, 10)];
    let n = elevations.iter().map(|&(k, _)| k + 1).max().unwrap();

    let mut up: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    let mut down_rev: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    for &(u, v, w) in &paths {
        if elev(v) > elev(u) { up[u].push((v, w)); }
        else if elev(v) < elev(u) { down_rev[v].push((u, w)); }
    }
    let up_d = dijkstra(&up, 0, n);
    let dn_d = dijkstra(&down_rev, 0, n);
    let mut best = i64::MAX;
    for p in 1..n {
        if up_d[p] != i64::MAX && dn_d[p] != i64::MAX {
            best = best.min(up_d[p] + dn_d[p]);
        }
    }
    println!("{}", best);
}
