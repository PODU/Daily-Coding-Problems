# Day 391: Longest common contiguous subsequence (substring) of two histories.
# DP on suffix-run lengths. Time O(n*m), Space O(n*m).
from typing import List


def longest_common(a: List[str], b: List[str]) -> List[str]:
    n, m = len(a), len(b)
    dp = [[0] * (m + 1) for _ in range(n + 1)]
    best, end_i = 0, 0
    for i in range(1, n + 1):
        for j in range(1, m + 1):
            if a[i - 1] == b[j - 1]:
                dp[i][j] = dp[i - 1][j - 1] + 1
                if dp[i][j] > best:
                    best, end_i = dp[i][j], i
    return a[end_i - best:end_i]


if __name__ == "__main__":
    user1 = ['/home', '/register', '/login', '/user', '/one', '/two']
    user2 = ['/home', '/red', '/login', '/user', '/one', '/pink']
    print(longest_common(user1, user2))
