// Day 1293: Count valid playlists of length N from M songs, each used >=1, repeats >=B apart.
// DP: dp[i][j]=playlists of len i with j distinct songs. O(N*M) time, O(N*M) space (mod 1e9+7).
const MOD = 1000000007n;

function numPlaylists(N, M, B) {
  const dp = Array.from({ length: N + 1 }, () => new Array(M + 1).fill(0n));
  dp[0][0] = 1n;
  for (let i = 1; i <= N; i++)
    for (let j = 1; j <= M; j++) {
      dp[i][j] = (dp[i - 1][j - 1] * BigInt(M - (j - 1))) % MOD;
      dp[i][j] = (dp[i][j] + dp[i - 1][j] * BigInt(Math.max(j - B, 0))) % MOD;
    }
  return dp[N][M];
}

console.log(numPlaylists(3, 3, 1).toString()); // 6
