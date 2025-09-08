// Android unlock patterns of length N: DFS over the 1..9 keypad, tracking visited keys and a
// "skip" matrix (key jumped over must already be visited). Symmetry reduces work to 3 starts.
// Time: O(9!) worst case, Space: O(9).
const skip = Array.from({length: 10}, () => new Array(10).fill(0));
skip[1][3] = skip[3][1] = 2;
skip[1][7] = skip[7][1] = 4;
skip[3][9] = skip[9][3] = 6;
skip[7][9] = skip[9][7] = 8;
skip[1][9] = skip[9][1] = skip[3][7] = skip[7][3] = 5;
skip[2][8] = skip[8][2] = 5;
skip[4][6] = skip[6][4] = 5;

function dfs(cur, remaining, visited) {
  if (remaining === 0) return 1;
  visited[cur] = true;
  let count = 0;
  for (let next = 1; next <= 9; next++) {
    const mid = skip[cur][next];
    if (!visited[next] && (mid === 0 || visited[mid]))
      count += dfs(next, remaining - 1, visited);
  }
  visited[cur] = false;
  return count;
}

function patterns(n) {
  const v = new Array(10).fill(false);
  return dfs(1, n - 1, v) * 4 + dfs(2, n - 1, v) * 4 + dfs(5, n - 1, v);
}

for (let n = 1; n <= 9; n++) console.log(`N=${n}: ${patterns(n)}`);
// e.g. N=1 -> 9, N=4 -> 1624 (standard canonical counts)
