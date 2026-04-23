// Transitive closure: each input row is [node, neighbors...]. DFS from every node
// marks all reachable vertices (incl. itself). Time O(V*(V+E)), Space O(V^2).

fn dfs(u: usize, adj: &Vec<Vec<usize>>, row: &mut Vec<i32>) {
    row[u] = 1;
    for &v in &adj[u] {
        if row[v] == 0 {
            dfs(v, adj, row);
        }
    }
}

fn transitive_closure(graph: &[Vec<usize>]) -> Vec<Vec<i32>> {
    let n = graph.len();
    let mut adj = vec![Vec::new(); n];
    for r in graph {
        let node = r[0];
        adj[node].extend_from_slice(&r[1..]);
    }
    let mut m = vec![vec![0i32; n]; n];
    for i in 0..n {
        dfs(i, &adj, &mut m[i]);
    }
    m
}

fn main() {
    let graph = vec![vec![0, 1, 3], vec![1, 2], vec![2], vec![3]];
    for row in transitive_closure(&graph) {
        println!("{:?}", row);
    }
}
