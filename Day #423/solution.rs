// Day 423: Transitive closure via DFS from each vertex. O(V*(V+E)) time, O(V^2) space.
// M[i][j] = 1 iff j is reachable from i (each vertex reaches itself).
fn dfs(src: usize, u: usize, g: &Vec<Vec<usize>>, m: &mut Vec<Vec<i32>>) {
    m[src][u] = 1;
    for &v in &g[u] {
        if m[src][v] == 0 {
            dfs(src, v, g, m);
        }
    }
}

fn main() {
    let g: Vec<Vec<usize>> = vec![vec![0, 1, 3], vec![1, 2], vec![2], vec![3]];
    let n = g.len();
    let mut m = vec![vec![0i32; n]; n];
    for i in 0..n {
        dfs(i, i, &g, &mut m);
    }
    for row in &m {
        let parts: Vec<String> = row.iter().map(|x| x.to_string()).collect();
        println!("[{}]", parts.join(", "));
    }
}
