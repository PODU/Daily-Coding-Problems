// Day 1776: Count valid Android unlock patterns of length N.
// DFS with symmetry + jump-over rules. Time O(N!) bounded by 9!, Space O(9).

fn dfs(cur: usize, remaining: usize, skip: &[[usize; 10]; 10], visited: &mut [bool; 10]) -> usize {
    if remaining == 0 {
        return 1;
    }
    visited[cur] = true;
    let mut cnt = 0;
    for nxt in 1..=9 {
        if visited[nxt] {
            continue;
        }
        let mid = skip[cur][nxt];
        if mid == 0 || visited[mid] {
            cnt += dfs(nxt, remaining - 1, skip, visited);
        }
    }
    visited[cur] = false;
    cnt
}

fn count_patterns(n: usize, skip: &[[usize; 10]; 10]) -> usize {
    let mut v = [false; 10];
    4 * dfs(1, n - 1, skip, &mut v) + 4 * dfs(2, n - 1, skip, &mut v) + dfs(5, n - 1, skip, &mut v)
}

fn main() {
    let mut skip = [[0usize; 10]; 10];
    for &(a, b, m) in &[(1, 3, 2), (1, 7, 4), (3, 9, 6), (7, 9, 8),
                        (1, 9, 5), (3, 7, 5), (2, 8, 5), (4, 6, 5)] {
        skip[a][b] = m;
        skip[b][a] = m;
    }
    for n in 1..=9 {
        println!("N={}: {}", n, count_patterns(n, &skip));
    }
}
