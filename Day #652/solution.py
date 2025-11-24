# Day 652: Road-trip playlist: DP (LeetCode 920). dp[i][j]=playlists of length i with j distinct songs.
# dp[i][j]=dp[i-1][j-1]*(M-(j-1)) + dp[i-1][j]*max(j-B,0). Time O(N*M), space O(M).
MOD = 1_000_000_007


def count_playlists(N: int, M: int, B: int) -> int:
    prev = [0] * (M + 1)
    prev[0] = 1
    for _ in range(1, N + 1):
        cur = [0] * (M + 1)
        for j in range(1, M + 1):
            cur[j] = prev[j - 1] * (M - (j - 1)) % MOD
            cur[j] = (cur[j] + prev[j] * max(j - B, 0)) % MOD
        prev = cur
    return prev[M]


if __name__ == "__main__":
    N, M, B = 3, 3, 1
    print(count_playlists(N, M, B))
