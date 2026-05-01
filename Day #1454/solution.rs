// Day 1454: A graph is minimally-connected iff it is a tree: connected AND has
// no cycle (exactly n-1 edges). DFS from node 0. Time O(V+E), Space O(V+E).
fn is_tree(n: usize, edges: &[(usize, usize)]) -> bool {
    if n == 0 {
        return true;
    }
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut seen = 1;
    let mut stack: Vec<(usize, isize)> = vec![(0, -1)];
    while let Some((u, parent)) = stack.pop() {
        for &w in &adj[u] {
            if !visited[w] {
                visited[w] = true;
                seen += 1;
                stack.push((w, u as isize));
            } else if w as isize != parent {
                return false; // back-edge -> cycle
            }
        }
    }
    seen == n
}

fn main() {
    let tree = [(0, 1), (1, 2), (1, 3)];
    let cyclic = [(0, 1), (1, 2), (2, 0), (2, 3)];
    println!("{}", if is_tree(4, &tree) { "True" } else { "False" });   // True
    println!("{}", if is_tree(4, &cyclic) { "True" } else { "False" }); // False
}
