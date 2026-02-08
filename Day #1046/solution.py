# Day 1046: Shortest palindrome by inserting chars; lexicographically earliest on ties.
# DP dp[i][j]=min insertions; memoized build(i,j) reconstructs string. Time O(n^2), Space O(n^2).
import sys
from functools import lru_cache


def solve(s: str) -> str:
    n = len(s)
    if n == 0:
        return ""
    dp = [[0] * n for _ in range(n)]
    for length in range(2, n + 1):
        for i in range(0, n - length + 1):
            j = i + length - 1
            if s[i] == s[j]:
                dp[i][j] = dp[i + 1][j - 1] if i + 1 <= j - 1 else 0
            else:
                dp[i][j] = 1 + min(dp[i + 1][j], dp[i][j - 1])

    @lru_cache(maxsize=None)
    def build(i: int, j: int) -> str:
        if i > j:
            return ""
        if i == j:
            return s[i]
        if s[i] == s[j]:
            return s[i] + build(i + 1, j - 1) + s[i]
        a = s[i] + build(i + 1, j) + s[i]
        b = s[j] + build(i, j - 1) + s[j]
        if dp[i + 1][j] < dp[i][j - 1]:
            return a
        if dp[i][j - 1] < dp[i + 1][j]:
            return b
        return min(a, b)

    return build(0, n - 1)


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    print("race ->", solve("race"))
    print("google ->", solve("google"))
