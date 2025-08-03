// Count open knight's tours on N x N board via backtracking DFS from every start.
// Time exponential, Space O(N^2).
const MOVES: [(i32, i32); 8] = [(-2,-1),(-2,1),(-1,-2),(-1,2),(1,-2),(1,2),(2,-1),(2,1)];

fn dfs(vis: &mut Vec<Vec<bool>>, n: i32, r: i32, c: i32, count: i32) -> u64 {
    if count == n * n {
        return 1;
    }
    let mut res = 0u64;
    for (dr, dc) in MOVES.iter() {
        let nr = r + dr;
        let nc = c + dc;
        if nr >= 0 && nr < n && nc >= 0 && nc < n && !vis[nr as usize][nc as usize] {
            vis[nr as usize][nc as usize] = true;
            res += dfs(vis, n, nr, nc, count + 1);
            vis[nr as usize][nc as usize] = false;
        }
    }
    res
}

fn count_tours(n: i32) -> u64 {
    let mut total = 0u64;
    let mut vis = vec![vec![false; n as usize]; n as usize];
    for r in 0..n {
        for c in 0..n {
            vis[r as usize][c as usize] = true;
            total += dfs(&mut vis, n, r, c, 1);
            vis[r as usize][c as usize] = false;
        }
    }
    total
}

fn main() {
    println!("{}", count_tours(5));
}
