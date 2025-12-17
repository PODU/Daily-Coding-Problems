// Day 755: Largest value path in a directed graph.
// Topological DP: dp[u][c] = max count of letter c on a path starting at u.
// Cycle -> value is infinite -> None. Time: O((n+m)*26), Space: O(n*26).
use std::collections::VecDeque;

fn largest_path_value(letters: &str, edges: &[(usize, usize)]) -> Option<i32> {
    let bytes = letters.as_bytes();
    let n = bytes.len();
    let mut adj = vec![Vec::new(); n];
    let mut indeg = vec![0usize; n];
    for &(i, j) in edges {
        adj[i].push(j);
        indeg[j] += 1;
    }

    let mut q: VecDeque<usize> = (0..n).filter(|&i| indeg[i] == 0).collect();
    let mut topo = Vec::new();
    while let Some(u) = q.pop_front() {
        topo.push(u);
        for &v in &adj[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if topo.len() < n {
        return None; // cycle
    }

    let mut dp = vec![[0i32; 26]; n];
    for i in 0..n {
        dp[i][(bytes[i] - b'A') as usize] = 1;
    }
    let mut best = 0;
    for &u in topo.iter().rev() {
        let uc = (bytes[u] - b'A') as usize;
        for k in 0..adj[u].len() {
            let v = adj[u][k];
            for c in 0..26 {
                let add = dp[v][c] + if c == uc { 1 } else { 0 };
                if add > dp[u][c] {
                    dp[u][c] = add;
                }
            }
        }
        best = best.max(*dp[u].iter().max().unwrap());
    }
    Some(best)
}

fn main() {
    let letters = "ABACA";
    let edges = [(0, 1), (0, 2), (2, 3), (3, 4)];
    match largest_path_value(letters, &edges) {
        Some(v) => println!("{}", v), // 3
        None => println!("null"),
    }
}
