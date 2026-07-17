// Count knight's tours on NxN via DFS backtracking from every start cell.
// Worst-case exponential; fine for small N (N=5 -> 1728).
const DR: [i32; 8] = [-2, -2, -1, -1, 1, 1, 2, 2];
const DC: [i32; 8] = [-1, 1, -2, 2, -2, 2, -1, 1];

fn dfs(n: i32, r: i32, c: i32, count: i32, vis: &mut Vec<Vec<bool>>) -> u64 {
    if count == n * n {
        return 1;
    }
    let mut total = 0u64;
    for k in 0..8 {
        let nr = r + DR[k];
        let nc = c + DC[k];
        if nr >= 0 && nr < n && nc >= 0 && nc < n && !vis[nr as usize][nc as usize] {
            vis[nr as usize][nc as usize] = true;
            total += dfs(n, nr, nc, count + 1, vis);
            vis[nr as usize][nc as usize] = false;
        }
    }
    total
}

fn count_tours(n: i32) -> u64 {
    let mut total = 0u64;
    for r in 0..n {
        for c in 0..n {
            let mut vis = vec![vec![false; n as usize]; n as usize];
            vis[r as usize][c as usize] = true;
            total += dfs(n, r, c, 1, &mut vis);
        }
    }
    total
}

fn main() {
    println!("{}", count_tours(5)); // 1728
}
