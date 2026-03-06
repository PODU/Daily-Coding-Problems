// Root tree, DFS subtree sizes; count non-root nodes with even subtree size (cuttable parent edges). O(n) time, O(n) space.
fn dfs(u: usize, parent: i32, adj: &Vec<Vec<usize>>, answer: &mut i32) -> i32 {
    let mut size = 1;
    for &v in &adj[u] {
        if v as i32 != parent {
            size += dfs(v, u as i32, adj, answer);
        }
    }
    if parent != -1 && size % 2 == 0 {
        *answer += 1;
    }
    size
}

fn main() {
    let n = 8;
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    let edges = [(1, 2), (1, 3), (3, 4), (3, 5), (4, 6), (4, 7), (4, 8)];
    for &(a, b) in edges.iter() {
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut answer = 0;
    dfs(1, -1, &adj, &mut answer);
    println!("{}", answer);
}
