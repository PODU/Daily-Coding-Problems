// Day 704: Count valid Android unlock patterns of length N on a 3x3 keypad.
// Approach: DFS with a "skip" table (a jump is legal only if the middle key was
// already used); symmetry cuts the work. Time O(N!) worst but tiny (<=9 keys).
fn dfs(cur: usize, remaining: usize, visited: &mut [bool; 10], skip: &[[usize; 10]; 10]) -> i32 {
    if remaining == 0 {
        return 1;
    }
    visited[cur] = true;
    let mut count = 0;
    for nx in 1..=9 {
        if !visited[nx] {
            let mid = skip[cur][nx];
            if mid == 0 || visited[mid] {
                count += dfs(nx, remaining - 1, visited, skip);
            }
        }
    }
    visited[cur] = false;
    count
}

fn number_of_patterns(n: usize) -> i32 {
    let mut skip = [[0usize; 10]; 10];
    skip[1][3] = 2; skip[3][1] = 2;
    skip[1][7] = 4; skip[7][1] = 4;
    skip[3][9] = 6; skip[9][3] = 6;
    skip[7][9] = 8; skip[9][7] = 8;
    skip[1][9] = 5; skip[9][1] = 5; skip[3][7] = 5; skip[7][3] = 5;
    skip[2][8] = 5; skip[8][2] = 5; skip[4][6] = 5; skip[6][4] = 5;
    let mut visited = [false; 10];
    4 * dfs(1, n - 1, &mut visited, &skip)
        + 4 * dfs(2, n - 1, &mut visited, &skip)
        + dfs(5, n - 1, &mut visited, &skip)
}

fn main() {
    println!("{}", number_of_patterns(4)); // 1624
}
