# Day 201: Maximum weight path in a triangle.
# Bottom-up DP: each cell becomes its value + max of the two children below.
# Time: O(n^2), Space: O(n).


def max_path(t):
    dp = list(t[-1])
    for r in range(len(t) - 2, -1, -1):
        for c in range(r + 1):
            dp[c] = t[r][c] + max(dp[c], dp[c + 1])
    return dp[0]


if __name__ == "__main__":
    print(max_path([[1], [2, 3], [1, 5, 1]]))  # 9
