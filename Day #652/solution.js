// Road-trip playlist: DP (LeetCode 920). dp[i][j]=playlists of length i with j distinct songs.
// dp[i][j]=dp[i-1][j-1]*(M-(j-1)) + dp[i-1][j]*max(j-B,0). Time O(N*M), space O(M).
const MOD = 1000000007n;

function countPlaylists(N, M, B) {
  let prev = new Array(M + 1).fill(0n);
  prev[0] = 1n;
  for (let i = 1; i <= N; i++) {
    const cur = new Array(M + 1).fill(0n);
    for (let j = 1; j <= M; j++) {
      cur[j] = (prev[j - 1] * BigInt(M - (j - 1))) % MOD;
      cur[j] = (cur[j] + prev[j] * BigInt(Math.max(j - B, 0))) % MOD;
    }
    prev = cur;
  }
  return prev[M];
}

const N = 3, M = 3, B = 1;
console.log(countPlaylists(N, M, B).toString());
