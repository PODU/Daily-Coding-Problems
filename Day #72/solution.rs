// Largest path value in directed graph: topo sort (Kahn) + DP dp[node][26]. Cycle -> null. Time O((n+m)*26), Space O(n*26).
use std::collections::VecDeque;

// Graph "A" with edge (0,0) returns None (self-loop is a cycle).
fn largest_path_value(colors: &str, edges: &[(usize, usize)]) -> Option<i32> {
    let bytes = colors.as_bytes();
    let n = bytes.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut indeg = vec![0usize; n];
    for &(a, b) in edges {
        adj[a].push(b);
        indeg[b] += 1;
    }
    let mut dp = vec![[0i32; 26]; n];
    let mut q: VecDeque<usize> = VecDeque::new();
    for i in 0..n {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }
    let mut seen = 0;
    let mut ans = 0;
    while let Some(u) = q.pop_front() {
        seen += 1;
        let cu = (bytes[u] - b'A') as usize;
        dp[u][cu] += 1;
        if dp[u][cu] > ans {
            ans = dp[u][cu];
        }
        let neighbors = adj[u].clone();
        let du = dp[u];
        for v in neighbors {
            for c in 0..26 {
                if du[c] > dp[v][c] {
                    dp[v][c] = du[c];
                }
            }
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if seen < n {
        return None; // cycle
    }
    Some(ans)
}

fn main() {
    let colors = "ABACA";
    let edges = [(0, 1), (0, 2), (2, 3), (3, 4)];
    match largest_path_value(colors, &edges) {
        Some(v) => println!("{}", v),
        None => println!("null"),
    }
}
