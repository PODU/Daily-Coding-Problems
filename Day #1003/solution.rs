// Day 1003: Transitive closure of a graph (adjacency list -> reachability matrix).
// DFS from each vertex marking everything reachable. O(V*(V+E)) time, O(V^2) space.
fn dfs(graph: &[Vec<usize>], m: &mut [Vec<i32>], start: usize, u: usize) {
    for &v in &graph[u] {
        if m[start][v] == 0 {
            m[start][v] = 1;
            dfs(graph, m, start, v);
        }
    }
}

fn main() {
    let graph: Vec<Vec<usize>> = vec![vec![0, 1, 3], vec![1, 2], vec![2], vec![3]];
    let n = graph.len();
    let mut m = vec![vec![0i32; n]; n];
    for s in 0..n {
        m[s][s] = 1;
        dfs(&graph, &mut m, s, s);
    }
    for row in &m {
        println!("{:?}", row);
    }
}
