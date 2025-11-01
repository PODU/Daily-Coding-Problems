# Day 529: Split a string into the fewest palindromic substrings.
# DP: is_pal[i][j] in O(n^2); dp[i] = min cuts for prefix i with parent pointers
# to reconstruct one optimal partition. Time O(n^2), space O(n^2).
from typing import List


def min_palindrome_partition(s: str) -> List[str]:
    n = len(s)
    pal = [[False] * n for _ in range(n)]
    for i in range(n - 1, -1, -1):
        for j in range(i, n):
            pal[i][j] = s[i] == s[j] and (j - i < 2 or pal[i + 1][j - 1])

    INF = float("inf")
    dp = [INF] * (n + 1)
    prev = [-1] * (n + 1)
    dp[0] = 0
    for j in range(1, n + 1):
        for i in range(j):
            if pal[i][j - 1] and dp[i] + 1 < dp[j]:
                dp[j] = dp[i] + 1
                prev[j] = i

    parts = []
    j = n
    while j > 0:
        parts.append(s[prev[j]:j])
        j = prev[j]
    parts.reverse()
    return parts


if __name__ == "__main__":
    s = "racecarannakayak"
    print(min_palindrome_partition(s))  # expected: ['racecar', 'anna', 'kayak']
