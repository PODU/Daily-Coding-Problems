// Transitive closure via DFS from each vertex. Time O(V*(V+E)), Space O(V^2).
fn main() {
    let graph: Vec<Vec<usize>> = vec![vec![0, 1, 3], vec![1, 2], vec![2], vec![3]];
    let n = graph.len();
    let mut m = vec![vec![0u8; n]; n];
    for s in 0..n {
        let mut vis = vec![false; n];
        let mut stack = vec![s];
        while let Some(u) = stack.pop() {
            if vis[u] {
                continue;
            }
            vis[u] = true;
            m[s][u] = 1;
            for &v in &graph[u] {
                if !vis[v] {
                    stack.push(v);
                }
            }
        }
    }
    for i in 0..n {
        let parts: Vec<String> = (0..n).map(|j| m[i][j].to_string()).collect();
        println!("[{}]", parts.join(", "));
    }
}
