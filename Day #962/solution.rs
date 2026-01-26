// Day 962: Count knight's tours on an N x N board (visit every square once).
// Approach: DFS backtracking from every start square. Time O(8^(N^2)) worst, Space O(N^2).

const DX: [i32; 8] = [1, 1, -1, -1, 2, 2, -2, -2];
const DY: [i32; 8] = [2, -2, 2, -2, 1, -1, 1, -1];

fn dfs(vis: &mut Vec<Vec<bool>>, n: i32, x: i32, y: i32, count: i32) -> u64 {
    if count == n * n {
        return 1;
    }
    let mut total = 0u64;
    for d in 0..8 {
        let nx = x + DX[d];
        let ny = y + DY[d];
        if nx >= 0 && nx < n && ny >= 0 && ny < n && !vis[nx as usize][ny as usize] {
            vis[nx as usize][ny as usize] = true;
            total += dfs(vis, n, nx, ny, count + 1);
            vis[nx as usize][ny as usize] = false;
        }
    }
    total
}

fn count_tours(n: i32) -> u64 {
    let mut total = 0u64;
    for i in 0..n {
        for j in 0..n {
            let mut vis = vec![vec![false; n as usize]; n as usize];
            vis[i as usize][j as usize] = true;
            total += dfs(&mut vis, n, i, j, 1);
        }
    }
    total
}

fn main() {
    println!("{}", count_tours(5)); // 1728
}
