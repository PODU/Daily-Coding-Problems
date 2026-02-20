# Day 1103: Count right/down paths through a grid avoiding walls (1).
# DP: dp[i][j] = dp[i-1][j] + dp[i][j-1], 0 at walls. Time: O(N*M). Space: O(M).
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
                dp[j] = (dp[j - 1] if j > 0 else 0) + dp[j]
    return dp[m - 1]


if __name__ == "__main__":
    g = [[0, 0, 1], [0, 0, 1], [1, 0, 0]]
    print(count_paths(g))  # 2
