// Day 1107: Max edges removable so every component has an even node count.
// DFS subtree sizes; every non-root node with an even-sized subtree => one cuttable edge.
// Time: O(V+E). Space: O(V).
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

fn max_edges_removed(n: usize, edges: &[(usize, usize)], root: usize) -> i32 {
    let mut adj = vec![Vec::new(); n + 1];
    for &(a, b) in edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut answer = 0;
    dfs(root, -1, &adj, &mut answer);
    answer
}

fn main() {
    let edges = [(1, 2), (1, 3), (3, 4), (3, 5), (4, 6), (4, 7), (4, 8)];
    println!("{}", max_edges_removed(8, &edges, 1)); // 2
}
