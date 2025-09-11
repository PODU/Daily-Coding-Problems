// Transitive closure: DFS from each vertex marking reachable nodes (incl self).
// Time O(V*(V+E)), Space O(V^2) for the matrix.
fn dfs(u: usize, g: &Vec<Vec<usize>>, row: &mut Vec<i32>) {
    row[u] = 1;
    for &v in &g[u] {
        if row[v] == 0 {
            dfs(v, g, row);
        }
    }
}

fn main() {
    let graph: Vec<Vec<usize>> = vec![vec![0, 1, 3], vec![1, 2], vec![2], vec![3]];
    let n = graph.len();
    let mut m = vec![vec![0i32; n]; n];
    for i in 0..n {
        let mut row = vec![0i32; n];
        dfs(i, &graph, &mut row);
        m[i] = row;
    }
    for row in &m {
        let parts: Vec<String> = row.iter().map(|v| v.to_string()).collect();
        println!("[{}]", parts.join(", "));
    }
}
