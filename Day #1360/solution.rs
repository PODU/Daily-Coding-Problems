// Count Android unlock patterns of length N via DFS backtracking with a skip
// (midpoint) table; symmetry over corners/edges. Time O(N!) worst, Space O(N).
fn dfs(cur: usize, remaining: usize, skip: &[[usize; 10]; 10], visited: &mut [bool; 10]) -> i64 {
    if remaining == 0 {
        return 1;
    }
    visited[cur] = true;
    let mut count = 0i64;
    for next in 1..=9 {
        if !visited[next] && (skip[cur][next] == 0 || visited[skip[cur][next]]) {
            count += dfs(next, remaining - 1, skip, visited);
        }
    }
    visited[cur] = false;
    count
}

fn count_patterns(n: usize) -> i64 {
    let mut skip = [[0usize; 10]; 10];
    skip[1][3] = 2; skip[3][1] = 2;
    skip[1][7] = 4; skip[7][1] = 4;
    skip[3][9] = 6; skip[9][3] = 6;
    skip[7][9] = 8; skip[9][7] = 8;
    skip[1][9] = 5; skip[9][1] = 5;
    skip[2][8] = 5; skip[8][2] = 5;
    skip[3][7] = 5; skip[7][3] = 5;
    skip[4][6] = 5; skip[6][4] = 5;
    let mut visited = [false; 10];
    let corner = dfs(1, n - 1, &skip, &mut visited);
    let edge = dfs(2, n - 1, &skip, &mut visited);
    let center = dfs(5, n - 1, &skip, &mut visited);
    corner * 4 + edge * 4 + center
}

fn main() {
    println!("{}", count_patterns(4));
}
