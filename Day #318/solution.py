# Day 318: Count playlists of length N from M songs, each used >=1, gap >=B between repeats.
# DP over length x distinct songs (LeetCode 920). Time O(N*M), Space O(N*M).

MOD = 1000000007


def num_playlists(N, M, B):
    dp = [[0] * (M + 1) for _ in range(N + 1)]
    dp[0][0] = 1
    for i in range(1, N + 1):
        for j in range(1, M + 1):
            dp[i][j] = dp[i - 1][j - 1] * (M - (j - 1)) % MOD
            dp[i][j] = (dp[i][j] + dp[i - 1][j] * max(j - B, 0)) % MOD
    return dp[N][M]


if __name__ == "__main__":
    N, M, B = 3, 3, 1
    print(f"Number of valid playlists (N={N}, M={M}, B={B}) = {num_playlists(N, M, B)}")
