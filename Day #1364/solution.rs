// Topological DP over DAG: dp[node][c] = max count of letter c on a path ending at node.
// Kahn's algorithm detects cycles (return None). Time O((V+E)*26), Space O(V*26).
use std::collections::VecDeque;

// Returns Some(value), or None if a cycle exists (null case).
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

    let mut processed = 0;
    let mut ans = 0;
    while let Some(u) = q.pop_front() {
        processed += 1;
        let cu = (bytes[u] - b'A') as usize;
        dp[u][cu] += 1;
        ans = ans.max(dp[u][cu]);
        let dpu = dp[u];
        for &v in &adj[u] {
            for c in 0..26 {
                if dpu[c] > dp[v][c] {
                    dp[v][c] = dpu[c];
                }
            }
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if processed < n {
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
