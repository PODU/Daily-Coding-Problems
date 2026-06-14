# Day 1658: Max weight path top->bottom in triangle, bottom-up DP collapsing rows. O(n) space.
def max_path(t):
    dp = list(t[-1])
    for i in range(len(t) - 2, -1, -1):
        for j in range(i + 1):
            dp[j] = t[i][j] + max(dp[j], dp[j + 1])
    return dp[0]

if __name__ == "__main__":
    print(max_path([[1], [2, 3], [1, 5, 1]]))
