// Day 1483: Shortest closed route from home (0) strictly ascending then
// descending. up[v]: shortest ascending 0->v; down[v]: shortest descending v->0
// (Dijkstra from 0 on reversed descending graph). Answer = min up[v]+down[v].
// Time O((V+E) log V), Space O(V+E).
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: i64 = i64::MAX;

fn dijkstra(n: usize, adj: &Vec<Vec<(usize, i64)>>, src: usize) -> (Vec<i64>, Vec<i32>) {
    let mut dist = vec![INF; n];
    let mut pred = vec![-1i32; n];
    dist[src] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0i64, src)));
    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            if d + w < dist[v] {
                dist[v] = d + w;
                pred[v] = u as i32;
                heap.push(Reverse((dist[v], v)));
            }
        }
    }
    (dist, pred)
}

fn main() {
    let n = 5;
    let elev = [5, 25, 15, 20, 10];
    let edges: [(usize, usize, i64); 8] = [
        (0, 1, 10), (0, 2, 8), (0, 3, 15), (1, 3, 12),
        (2, 4, 10), (3, 4, 5), (3, 0, 17), (4, 0, 10),
    ];

    let mut up_adj = vec![Vec::new(); n];
    let mut rev_desc = vec![Vec::new(); n];
    for &(a, b, w) in &edges {
        if elev[b] > elev[a] {
            up_adj[a].push((b, w));
        } else if elev[b] < elev[a] {
            rev_desc[b].push((a, w));
        }
    }

    let (up, up_pred) = dijkstra(n, &up_adj, 0);
    let (down, down_pred) = dijkstra(n, &rev_desc, 0);

    let mut best = INF;
    let mut peak: i32 = -1;
    for v in 1..n {
        if up[v] != INF && down[v] != INF && up[v] > 0 && down[v] > 0 && up[v] + down[v] < best {
            best = up[v] + down[v];
            peak = v as i32;
        }
    }

    let mut up_path = Vec::new();
    let mut c = peak;
    while c != -1 {
        up_path.push(c);
        c = up_pred[c as usize];
    }
    up_path.reverse();
    let mut route = up_path;
    c = down_pred[peak as usize];
    while c != -1 {
        route.push(c);
        c = down_pred[c as usize];
    }

    let parts: Vec<String> = route.iter().map(|x| x.to_string()).collect();
    println!(
        "The shortest valid path would be {}, with a distance of {}.",
        parts.join(" -> "),
        best
    );
    // The shortest valid path would be 0 -> 2 -> 4 -> 0, with a distance of 28.
}
