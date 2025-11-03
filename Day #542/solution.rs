// Bipartite check via BFS two-coloring. O(V+E) time, O(V) space.
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

fn build(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); n];
    for &(a, b) in edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    adj
}

fn main() {
    let g1 = build(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
    println!("{}", is_bipartite(4, &g1));
    let g2 = build(3, &[(0, 1), (1, 2), (2, 0)]);
    println!("{}", is_bipartite(3, &g2));
}
