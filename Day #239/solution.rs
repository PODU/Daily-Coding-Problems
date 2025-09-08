// Android unlock patterns of length N: DFS over the 1..9 keypad, tracking visited keys and a
// "skip" matrix (key jumped over must already be visited). Symmetry reduces work to 3 starts.
// Time: O(9!) worst case, Space: O(9).
fn dfs(cur: usize, remaining: i32, visited: &mut [bool; 10], skip: &[[usize; 10]; 10]) -> i32 {
    if remaining == 0 {
        return 1;
    }
    visited[cur] = true;
    let mut count = 0;
    for next in 1..=9 {
        let mid = skip[cur][next];
        if !visited[next] && (mid == 0 || visited[mid]) {
            count += dfs(next, remaining - 1, visited, skip);
        }
    }
    visited[cur] = false;
    count
}

fn patterns(n: i32) -> i32 {
    let mut skip = [[0usize; 10]; 10];
    skip[1][3] = 2; skip[3][1] = 2;
    skip[1][7] = 4; skip[7][1] = 4;
    skip[3][9] = 6; skip[9][3] = 6;
    skip[7][9] = 8; skip[9][7] = 8;
    skip[1][9] = 5; skip[9][1] = 5; skip[3][7] = 5; skip[7][3] = 5;
    skip[2][8] = 5; skip[8][2] = 5;
    skip[4][6] = 5; skip[6][4] = 5;
    let mut v = [false; 10];
    dfs(1, n - 1, &mut v, &skip) * 4 + dfs(2, n - 1, &mut v, &skip) * 4 + dfs(5, n - 1, &mut v, &skip)
}

fn main() {
    for n in 1..=9 {
        println!("N={}: {}", n, patterns(n));
    }
    // e.g. N=1 -> 9, N=4 -> 1624 (standard canonical counts)
}
