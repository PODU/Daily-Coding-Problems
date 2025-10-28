# Day 509: Fewest-insertion palindrome with lexicographically earliest result.
# Memoized interval DP building the actual string. Time O(n^3), Space O(n^2).
from functools import lru_cache


def solve(s: str) -> str:
    n = len(s)
    if n == 0:
        return ""

    @lru_cache(maxsize=None)
    def build(i: int, j: int) -> str:
        if i > j:
            return ""
        if i == j:
            return s[i]
        if s[i] == s[j]:
            return s[i] + build(i + 1, j - 1) + s[j]
        a = s[i] + build(i + 1, j) + s[i]   # insert s[i] at end
        b = s[j] + build(i, j - 1) + s[j]   # insert s[j] at front
        if len(a) < len(b):
            return a
        if len(b) < len(a):
            return b
        return a if a <= b else b

    return build(0, n - 1)


if __name__ == "__main__":
    print(solve("race"))
    print(solve("google"))
