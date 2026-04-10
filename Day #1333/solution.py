# Day 1333: Count right/down paths from top-left to bottom-right avoiding walls (1).
# DP: dp[i][j] = dp[i-1][j] + dp[i][j-1], 0 at walls. O(N*M) time, O(M) space.

def count_paths(g):
    n, m = len(g), len(g[0])
    dp = [0] * m
    for i in range(n):
        for j in range(m):
            if g[i][j] == 1:
                dp[j] = 0
            elif i == 0 and j == 0:
                dp[j] = 1
            else:
                dp[j] = (dp[j] if i else 0) + (dp[j - 1] if j else 0)
    return dp[m - 1]


if __name__ == "__main__":
    g = [[0, 0, 1], [0, 0, 1], [1, 0, 0]]
    print(count_paths(g))  # 2
