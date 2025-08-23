# Day 158: Count paths (right/down only) avoiding walls. DP: dp[j] = ways into a
# free cell from top+left; walls contribute 0. Time O(N*M), Space O(M).


def count_paths(grid):
    n, m = len(grid), len(grid[0])
    dp = [0] * m
    dp[0] = 1 if grid[0][0] == 0 else 0
    for i in range(n):
        for j in range(m):
            if grid[i][j] == 1:
                dp[j] = 0
            elif j > 0:
                dp[j] += dp[j - 1]
    return dp[m - 1]


if __name__ == "__main__":
    grid = [
        [0, 0, 1],
        [0, 0, 1],
        [1, 0, 0],
    ]
    print(count_paths(grid))  # 2
