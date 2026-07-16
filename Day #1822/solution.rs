// Bipartite check via BFS 2-coloring. O(V+E) time, O(V) space.
use std::collections::VecDeque;

fn is_bipartite(n: usize, adj: &Vec<Vec<usize>>) -> bool {
    let mut color = vec![-1i32; n];
    for s in 0..n {
        if color[s] != -1 {
            continue;
        }
        color[s] = 0;
        let mut q = VecDeque::new();
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                if color[v] == -1 {
                    color[v] = color[u] ^ 1;
                    q.push_back(v);
                } else if color[v] == color[u] {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let n = 4;
    let mut adj = vec![Vec::new(); n];
    let edges = [(0, 1), (1, 2), (2, 3), (3, 0)]; // even cycle -> bipartite
    for &(a, b) in edges.iter() {
        adj[a].push(b);
        adj[b].push(a);
    }
    println!("{}", if is_bipartite(n, &adj) { "true" } else { "false" });
}
