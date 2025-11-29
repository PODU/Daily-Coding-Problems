# Day 672: Maximum-weight top-to-bottom triangle path. Bottom-up DP folding rows.
# Time O(n^2) cells, Space O(n).


def max_path(t):
    dp = list(t[-1])
    for i in range(len(t) - 2, -1, -1):
        for j in range(i + 1):
            dp[j] = t[i][j] + max(dp[j], dp[j + 1])
    return dp[0]


if __name__ == "__main__":
    print(max_path([[1], [2, 3], [1, 5, 1]]))  # 9
