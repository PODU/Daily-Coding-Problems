// Minimally-connected = tree: edges == n-1 AND connected. Use BFS.
// Time O(n + e), Space O(n).
use std::collections::VecDeque;

fn is_tree(n: usize, edges: &[(usize, usize)]) -> bool {
    if edges.len() != n - 1 {
        return false;
    }
    let mut adj = vec![Vec::new(); n];
    for &(a, b) in edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut visited = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    visited[0] = true;
    let mut count = 1;
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if !visited[v] {
                visited[v] = true;
                count += 1;
                q.push_back(v);
            }
        }
    }
    count == n
}

fn main() {
    let n = 4;
    let edges = vec![(0, 1), (1, 2), (1, 3)];
    println!("{}", if is_tree(n, &edges) { "True" } else { "False" });
}
