# Day 1651: Count valid playlists of length N from M songs, each used >=1 time,
# with a buffer of B songs between repeats.
# DP: dp[i][j] = #playlists of length i using exactly j distinct songs.
# Time O(N*M), Space O(N*M).
MOD = 10**9 + 7


def num_playlists(N, M, B):
    dp = [[0] * (M + 1) for _ in range(N + 1)]
    dp[0][0] = 1
    for i in range(1, N + 1):
        for j in range(1, M + 1):
            dp[i][j] = dp[i - 1][j - 1] * (M - (j - 1)) % MOD
            dp[i][j] = (dp[i][j] + dp[i - 1][j] * max(j - B, 0)) % MOD
    return dp[N][M]


if __name__ == "__main__":
    # N=3, M=2, B=0 -> 6
    print(num_playlists(3, 2, 0))
