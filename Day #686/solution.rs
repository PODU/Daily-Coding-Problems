// Remove max edges so each resulting subtree has an even node count.
// DFS subtree sizes; count non-root nodes whose subtree size is even (each = one removable edge above it). O(n) time/space.
fn dfs(u: usize, parent: i32, adj: &Vec<Vec<usize>>, removable: &mut i32) -> i32 {
    let mut size = 1;
    for &v in &adj[u] {
        if v as i32 != parent {
            size += dfs(v, u as i32, adj, removable);
        }
    }
    if parent != -1 && size % 2 == 0 {
        *removable += 1;
    }
    size
}

fn main() {
    let n = 8;
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    let edges = [(1, 2), (1, 3), (3, 4), (3, 5), (4, 6), (4, 7), (4, 8)];
    for &(a, b) in &edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut removable = 0;
    dfs(1, -1, &adj, &mut removable);
    println!("{}", removable);
}
