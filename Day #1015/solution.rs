// Largest path value in a directed graph: DFS topological memo + color cycle detection.
// dp[u][c] = max count of letter c on a path ending at u. Cycle -> None (null). O((n+m)*26) time, O(n*26) space.
fn dfs(u: usize, adj: &Vec<Vec<usize>>, colors: &[u8], dp: &mut Vec<[i32; 26]>, state: &mut Vec<u8>) -> bool {
    state[u] = 1; // in-stack
    for vi in 0..adj[u].len() {
        let v = adj[u][vi];
        if state[v] == 1 {
            return false; // back edge -> cycle
        }
        if state[v] == 0 && !dfs(v, adj, colors, dp, state) {
            return false;
        }
    }
    for vi in 0..adj[u].len() {
        let v = adj[u][vi];
        for c in 0..26 {
            if dp[v][c] > dp[u][c] {
                dp[u][c] = dp[v][c];
            }
        }
    }
    dp[u][(colors[u] - b'A') as usize] += 1;
    state[u] = 2;
    true
}

fn largest_path_value(cols: &str, edges: &[(usize, usize)]) -> Option<i32> {
    let n = cols.len();
    let colors = cols.as_bytes();
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in edges {
        adj[u].push(v);
    }
    let mut dp = vec![[0i32; 26]; n];
    let mut state = vec![0u8; n];
    for i in 0..n {
        if state[i] == 0 && !dfs(i, &adj, colors, &mut dp, &mut state) {
            return None;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for c in 0..26 {
            if dp[i][c] > ans {
                ans = dp[i][c];
            }
        }
    }
    Some(ans)
}

fn main() {
    match largest_path_value("ABACA", &[(0, 1), (0, 2), (2, 3), (3, 4)]) {
        Some(x) => println!("{}", x),
        None => println!("null"),
    }
    match largest_path_value("A", &[(0, 0)]) {
        Some(x) => println!("{}", x),
        None => println!("null"),
    }
}
