// DFS subtree sizes; count non-root subtrees with even size = max edges removable.
// Time O(n), Space O(n).
fn dfs(u: usize, p: i32, g: &Vec<Vec<usize>>, ans: &mut i32) -> i32 {
    let mut sz = 1;
    for &v in &g[u] {
        if v as i32 != p {
            sz += dfs(v, u as i32, g, ans);
        }
    }
    if p != -1 && sz % 2 == 0 {
        *ans += 1;
    }
    sz
}

fn main() {
    let n = 8;
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    let edges = [(1, 2), (1, 3), (3, 4), (3, 5), (4, 6), (4, 7), (4, 8)];
    for &(a, b) in edges.iter() {
        g[a].push(b);
        g[b].push(a);
    }
    let mut ans = 0;
    dfs(1, -1, &g, &mut ans);
    println!("{}", ans);
}
