// Day 1776: Count valid Android unlock patterns of length N.
// DFS with symmetry + jump-over rules. Time O(N!) bounded by 9!, Space O(9).

const skip = Array.from({ length: 10 }, () => new Array(10).fill(0));
[[1, 3, 2], [1, 7, 4], [3, 9, 6], [7, 9, 8],
 [1, 9, 5], [3, 7, 5], [2, 8, 5], [4, 6, 5]].forEach(([a, b, m]) => {
    skip[a][b] = m; skip[b][a] = m;
});
const visited = new Array(10).fill(false);

function dfs(cur, remaining) {
    if (remaining === 0) return 1;
    visited[cur] = true;
    let cnt = 0;
    for (let nxt = 1; nxt <= 9; nxt++) {
        if (visited[nxt]) continue;
        const mid = skip[cur][nxt];
        if (mid === 0 || visited[mid]) cnt += dfs(nxt, remaining - 1);
    }
    visited[cur] = false;
    return cnt;
}

function countPatterns(n) {
    return 4 * dfs(1, n - 1) + 4 * dfs(2, n - 1) + dfs(5, n - 1);
}

for (let n = 1; n <= 9; n++) console.log(`N=${n}: ${countPatterns(n)}`);
