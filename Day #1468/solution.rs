// Knight's tour counting via plain DFS backtracking from every start cell.
// Time: exponential (bounded by tour search); Space: O(N^2) visited grid + recursion.
const DX: [i32; 8] = [1, 1, -1, -1, 2, 2, -2, -2];
const DY: [i32; 8] = [2, -2, 2, -2, 1, -1, 1, -1];

fn dfs(x: i32, y: i32, count: usize, n: i32, visited: &mut Vec<Vec<bool>>, total: &mut u64) {
    if count == (n * n) as usize {
        *total += 1;
        return;
    }
    for d in 0..8 {
        let nx = x + DX[d];
        let ny = y + DY[d];
        if nx >= 0 && nx < n && ny >= 0 && ny < n && !visited[nx as usize][ny as usize] {
            visited[nx as usize][ny as usize] = true;
            dfs(nx, ny, count + 1, n, visited, total);
            visited[nx as usize][ny as usize] = false;
        }
    }
}

fn count_tours(n: i32) -> u64 {
    let mut visited = vec![vec![false; n as usize]; n as usize];
    let mut total: u64 = 0;
    for i in 0..n {
        for j in 0..n {
            visited[i as usize][j as usize] = true;
            dfs(i, j, 1, n, &mut visited, &mut total);
            visited[i as usize][j as usize] = false;
        }
    }
    total
}

fn main() {
    println!("{}", count_tours(1));
    println!("{}", count_tours(5));
}
