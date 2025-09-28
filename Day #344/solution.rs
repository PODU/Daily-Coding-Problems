// Max edges removed so every component has even node count. DFS subtree sizes;
// answer = count of non-root nodes with even subtree size. Time O(n), Space O(n).
// Note: README narrates one valid single cut (3,4), but the MAXIMUM is 2:
// cut (1,3) and (3,4) -> {1,2},{3,5},{4,6,7,8}, all even.
fn dfs(u: usize, parent: usize, root: usize, adj: &Vec<Vec<usize>>, answer: &mut i32) -> i32 {
    let mut s = 1;
    for &v in &adj[u] {
        if v != parent {
            s += dfs(v, u, root, adj, answer);
        }
    }
    if u != root && s % 2 == 0 {
        *answer += 1;
    }
    s
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
    dfs(1, 0, 1, &adj, &mut answer);
    println!("{}", answer);
}
