// Day 1651: Count valid playlists of length N from M songs, each used >=1 time,
// with a buffer of B songs between repeats.
// DP: dp[i][j] = #playlists of length i using exactly j distinct songs.
// Time O(N*M), Space O(N*M).
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

// N=3, M=2, B=0 -> 6
console.log(numPlaylists(3, 2, 0).toString());
