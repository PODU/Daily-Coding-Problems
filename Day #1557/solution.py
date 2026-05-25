# Day 1557: Grid DP: max coins from top-left to bottom-right moving right/down only.
# dp[j] = grid + max(top, left). Time O(m*n), Space O(n).


def main():
    grid = [
        [0, 3, 1, 1],
        [2, 0, 0, 4],
        [1, 5, 3, 1],
    ]
    m, n = len(grid), len(grid[0])
    dp = [0] * n
    for i in range(m):
        for j in range(n):
            if i == 0 and j == 0:
                best = 0
            elif i == 0:
                best = dp[j - 1]
            elif j == 0:
                best = dp[j]
            else:
                best = max(dp[j], dp[j - 1])
            dp[j] = grid[i][j] + best
    print(dp[n - 1])


if __name__ == "__main__":
    main()
