# Day 1455: Maximum top-to-bottom path sum in a triangle. Bottom-up DP folding
# each row into the one above. Time O(n^2), Space O(n).
from typing import List


def max_path_sum(triangle: List[List[int]]) -> int:
    if not triangle:
        return 0
    dp = list(triangle[-1])
    for r in range(len(triangle) - 2, -1, -1):
        for i in range(r + 1):
            dp[i] = triangle[r][i] + max(dp[i], dp[i + 1])
    return dp[0]


if __name__ == "__main__":
    triangle = [[1], [2, 3], [1, 5, 1]]
    print(max_path_sum(triangle))  # 9  (1 -> 3 -> 5)
