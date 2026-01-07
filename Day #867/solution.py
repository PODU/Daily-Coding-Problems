# Day 867: Maximum weight path from top to bottom of a triangle.
# Approach: bottom-up DP, fold each row into the one above using max of adjacent.
# Time: O(n^2), Space: O(n).


def max_path(triangle):
    dp = list(triangle[-1])
    for i in range(len(triangle) - 2, -1, -1):
        for j in range(i + 1):
            dp[j] = triangle[i][j] + max(dp[j], dp[j + 1])
    return dp[0]


if __name__ == "__main__":
    print(max_path([[1], [2, 3], [1, 5, 1]]))  # 9
