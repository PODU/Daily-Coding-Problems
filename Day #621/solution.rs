// Tree diameter with edge weights: DFS returning max downward path; global best
// = sum of two largest child paths. Time O(N), Space O(N).
fn dfs(u: usize, parent: i32, adj: &Vec<Vec<(usize, i64)>>, best: &mut i64) -> i64 {
    let mut max1 = 0i64;
    let mut max2 = 0i64;
    for &(v, w) in &adj[u] {
        if v as i32 == parent {
            continue;
        }
        let path = dfs(v, u as i32, adj, best) + w;
        if path > max1 {
            max2 = max1;
            max1 = path;
        } else if path > max2 {
            max2 = path;
        }
    }
    if max1 + max2 > *best {
        *best = max1 + max2;
    }
    max1
}

fn main() {
    // a0 b1 c2 d3 e4 f5 g6 h7
    let n = 8;
    let edges = [(0, 1, 3), (0, 2, 5), (0, 3, 8),
                 (3, 4, 2), (3, 5, 4), (4, 6, 1), (4, 7, 1)];
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    for &(u, v, w) in &edges {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }
    let mut best = 0i64;
    dfs(0, -1, &adj, &mut best);
    println!("{}", best);
}
