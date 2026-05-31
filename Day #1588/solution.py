# Day 1588: Count right/down paths in a grid with walls. DP: dp[i][j] = paths to cell (0 if wall).
# Time: O(N*M); Space: O(N*M).


def count_paths(grid):
    n, m = len(grid), len(grid[0])
    if grid[0][0] == 1 or grid[n - 1][m - 1] == 1:
        return 0
    dp = [[0] * m for _ in range(n)]
    for i in range(n):
        for j in range(m):
            if grid[i][j] == 1:
                dp[i][j] = 0
            elif i == 0 and j == 0:
                dp[i][j] = 1
            else:
                up = dp[i - 1][j] if i > 0 else 0
                left = dp[i][j - 1] if j > 0 else 0
                dp[i][j] = up + left
    return dp[n - 1][m - 1]


def main():
    grid = [[0, 0, 1], [0, 0, 1], [1, 0, 0]]
    print(count_paths(grid))


if __name__ == "__main__":
    main()
