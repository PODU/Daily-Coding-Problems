# Day 122: Max coins from top-left to bottom-right moving right/down.
# DP O(R*C) time and space, with path reconstruction (prefer left on ties).


def solve(g):
    R, C = len(g), len(g[0])
    dp = [[0] * C for _ in range(R)]
    for i in range(R):
        for j in range(C):
            if i == 0 and j == 0:
                best = 0
            elif i == 0:
                best = dp[i][j - 1]
            elif j == 0:
                best = dp[i - 1][j]
            else:
                best = max(dp[i - 1][j], dp[i][j - 1])
            dp[i][j] = g[i][j] + best
    path = []
    i, j = R - 1, C - 1
    while i > 0 or j > 0:
        path.append(g[i][j])
        if i == 0:
            j -= 1
        elif j == 0:
            i -= 1
        elif dp[i - 1][j] > dp[i][j - 1]:
            i -= 1
        else:
            j -= 1
    path.append(g[0][0])
    path.reverse()
    return dp[R - 1][C - 1], path


if __name__ == "__main__":
    g = [[0, 3, 1, 1], [2, 0, 0, 4], [1, 5, 3, 1]]
    total, path = solve(g)
    print("The most we can collect is " + " + ".join(map(str, path)) + " = " + str(total) + " coins.")
