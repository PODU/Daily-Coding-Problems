# Day 1293: Count valid playlists of length N from M songs, each used >=1, repeats >=B apart.
# DP: dp[i][j]=playlists of len i with j distinct songs. O(N*M) time, O(N*M) space (mod 1e9+7).
MOD = 1000000007


def num_playlists(N: int, M: int, B: int) -> int:
    dp = [[0] * (M + 1) for _ in range(N + 1)]
    dp[0][0] = 1
    for i in range(1, N + 1):
        for j in range(1, M + 1):
            dp[i][j] = dp[i - 1][j - 1] * (M - (j - 1)) % MOD
            dp[i][j] = (dp[i][j] + dp[i - 1][j] * max(j - B, 0)) % MOD
    return dp[N][M]


if __name__ == "__main__":
    print(num_playlists(3, 3, 1))  # 6
