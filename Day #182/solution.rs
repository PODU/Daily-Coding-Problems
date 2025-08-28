// Day 182: Graph is minimally-connected iff it is a tree (connected and |E| == |V|-1).
// BFS connectivity + edge count. Time O(V+E), Space O(V+E).
use std::collections::VecDeque;

fn is_minimally_connected(v: usize, edges: &[(usize, usize)]) -> bool {
    if edges.len() != v - 1 {
        return false;
    }
    let mut adj = vec![Vec::new(); v];
    for &(a, b) in edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut seen = vec![false; v];
    let mut q = VecDeque::new();
    q.push_back(0);
    seen[0] = true;
    let mut cnt = 1;
    while let Some(u) = q.pop_front() {
        for &w in &adj[u] {
            if !seen[w] {
                seen[w] = true;
                cnt += 1;
                q.push_back(w);
            }
        }
    }
    cnt == v
}

fn main() {
    let tree = [(0, 1), (0, 2), (1, 3), (1, 4)];
    let cyclic = [(0, 1), (0, 2), (1, 3), (1, 4), (3, 4)];
    println!("{}", is_minimally_connected(5, &tree));
    println!("{}", is_minimally_connected(5, &cyclic));
}
