# Day 181: Minimum palindrome partitioning.
# DP: palindrome table + min-cut DP with reconstruction. Time O(n^2), Space O(n^2).
from typing import List


def min_palindrome_partition(s: str) -> List[str]:
    n = len(s)
    if n == 0:
        return []
    pal = [[False] * n for _ in range(n)]
    for i in range(n):
        pal[i][i] = True
    for L in range(2, n + 1):
        for i in range(0, n - L + 1):
            j = i + L - 1
            if s[i] == s[j] and (L == 2 or pal[i + 1][j - 1]):
                pal[i][j] = True
    INF = float("inf")
    cut = [INF] * (n + 1)
    prev = [-1] * (n + 1)
    cut[0] = 0
    for i in range(1, n + 1):
        for j in range(0, i):
            if pal[j][i - 1] and cut[j] + 1 < cut[i]:
                cut[i] = cut[j] + 1
                prev[i] = j
    res = []
    i = n
    while i > 0:
        j = prev[i]
        res.append(s[j:i])
        i = j
    res.reverse()
    return res


def fmt(v: List[str]) -> str:
    return "[" + ", ".join('"' + x + '"' for x in v) + "]"


if __name__ == "__main__":
    print(fmt(min_palindrome_partition("racecarannakayak")))
    print(fmt(min_palindrome_partition("abc")))
