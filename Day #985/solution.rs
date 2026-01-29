// Count valid Android unlock patterns of length N via backtracking with a
// skip[a][b] over-jump table; use 8-fold symmetry of corners/edges/center.
// Time: O(N * 9!) worst-case bounded by symmetry; Space: O(9).

fn dfs(cur: usize, remaining: usize, skip: &[[usize; 10]; 10], used: &mut [bool; 10]) -> u64 {
    if remaining == 0 {
        return 1;
    }
    used[cur] = true;
    let mut count = 0u64;
    for next in 1..=9 {
        if used[next] {
            continue;
        }
        let mid = skip[cur][next];
        if mid == 0 || used[mid] {
            count += dfs(next, remaining - 1, skip, used);
        }
    }
    used[cur] = false;
    count
}

fn count_patterns(n: usize, skip: &[[usize; 10]; 10]) -> u64 {
    let mut used = [false; 10];
    4 * dfs(1, n - 1, skip, &mut used)
        + 4 * dfs(2, n - 1, skip, &mut used)
        + dfs(5, n - 1, skip, &mut used)
}

fn main() {
    let mut skip = [[0usize; 10]; 10];
    skip[1][3] = 2; skip[3][1] = 2;
    skip[1][7] = 4; skip[7][1] = 4;
    skip[3][9] = 6; skip[9][3] = 6;
    skip[7][9] = 8; skip[9][7] = 8;
    skip[1][9] = 5; skip[9][1] = 5;
    skip[3][7] = 5; skip[7][3] = 5;
    skip[2][8] = 5; skip[8][2] = 5;
    skip[4][6] = 5; skip[6][4] = 5;

    for n in 1..=9 {
        println!("N={}: {}", n, count_patterns(n, &skip));
    }
}
