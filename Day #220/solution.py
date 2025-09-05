# Day 220: Optimal coin-picking game (first player guaranteed max).
# Approach: interval DP. dp[i][j] = max you can collect from coins i..j vs optimal opponent.
# Time O(n^2), Space O(n^2).
from typing import List


def max_coins(v: List[int]) -> int:
    n = len(v)
    if n == 0:
        return 0
    dp = [[0] * n for _ in range(n)]
    for i in range(n):
        dp[i][i] = v[i]
    for length in range(2, n + 1):
        for i in range(0, n - length + 1):
            j = i + length - 1
            inner_left = dp[i + 2][j] if i + 2 <= j else 0
            inner_mid = dp[i + 1][j - 1] if i + 1 <= j - 1 else 0
            inner_right = dp[i][j - 2] if i <= j - 2 else 0
            take_i = v[i] + min(inner_left, inner_mid)
            take_j = v[j] + min(inner_mid, inner_right)
            dp[i][j] = max(take_i, take_j)
    return dp[0][n - 1]


if __name__ == "__main__":
    print(max_coins([8, 15, 3, 7]))  # 22
