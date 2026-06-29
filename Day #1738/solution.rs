// Topological-order DP over DAG: dp[node][c]=max count of letter c on path from node.
// Kahn's algorithm; cycle => None. Time O((n+m)*26), Space O(n*26).
use std::collections::VecDeque;

fn largest_path_value(s: &str, edges: &[(usize, usize)]) -> Option<i32> {
    let bytes = s.as_bytes();
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
    let mut seen = 0usize;
    let mut ans = 0i32;
    while let Some(u) = q.pop_front() {
        seen += 1;
        let cu = (bytes[u] - b'A') as usize;
        dp[u][cu] += 1;
        ans = ans.max(dp[u][cu]);
        let row = dp[u];
        for &v in &adj[u] {
            for c in 0..26 {
                if row[c] > dp[v][c] {
                    dp[v][c] = row[c];
                }
            }
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if seen < n {
        None
    } else {
        Some(ans)
    }
}

fn main() {
    match largest_path_value("ABACA", &[(0, 1), (0, 2), (2, 3), (3, 4)]) {
        Some(v) => println!("{}", v),
        None => println!("null"),
    }
    match largest_path_value("A", &[(0, 0)]) {
        Some(v) => println!("{}", v),
        None => println!("null"),
    }
}
