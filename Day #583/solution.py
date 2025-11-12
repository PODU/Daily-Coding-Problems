# Day 583: Grid DP: dp[i][j] = grid[i][j] + max(dp[i-1][j], dp[i][j-1]). Time O(R*C), Space O(R*C).
def max_coins(grid):
    R, C = len(grid), len(grid[0])
    dp = [[0] * C for _ in range(R)]
    for i in range(R):
        for j in range(C):
            best = 0
            if i > 0:
                best = max(best, dp[i-1][j])
            if j > 0:
                best = max(best, dp[i][j-1])
            dp[i][j] = grid[i][j] + (0 if (i == 0 and j == 0) else best)
    return dp[R-1][C-1]


if __name__ == "__main__":
    grid = [[0, 3, 1, 1], [2, 0, 0, 4], [1, 5, 3, 1]]
    result = max_coins(grid)
    assert result == 12
    print("The most we can collect is 0 + 2 + 1 + 5 + 3 + 1 = " + str(result) + " coins.")
