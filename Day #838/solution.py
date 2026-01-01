# Day 838: Max coins moving only right/down through a grid.
# DP: dp[i][j] = grid[i][j] + max(dp[i-1][j], dp[i][j-1]).  Time O(R*C), Space O(R*C).


def max_coins(grid):
    if not grid or not grid[0]:
        return 0
    rows, cols = len(grid), len(grid[0])
    dp = [[0] * cols for _ in range(rows)]
    for i in range(rows):
        for j in range(cols):
            best = 0
            if i > 0:
                best = max(best, dp[i - 1][j])
            if j > 0:
                best = max(best, dp[i][j - 1])
            dp[i][j] = grid[i][j] + best
    return dp[rows - 1][cols - 1]


if __name__ == "__main__":
    matrix = [
        [0, 3, 1, 1],
        [2, 0, 0, 4],
        [1, 5, 3, 1],
    ]
    print(max_coins(matrix))
