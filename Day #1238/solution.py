# Day 1238: Fewest insertions to make a palindrome; among minima return lexicographically
# smallest. DP on intervals with memoized reconstruction.
# Time O(n^2) states, Space O(n^2).
from functools import lru_cache


def make_palindrome(s):
    @lru_cache(maxsize=None)
    def build(i, j):
        if i > j:
            return ""
        if i == j:
            return s[i]
        if s[i] == s[j]:
            return s[i] + build(i + 1, j - 1) + s[i]
        left = s[i] + build(i + 1, j) + s[i]   # insert s[i] at right
        right = s[j] + build(i, j - 1) + s[j]  # insert s[j] at left
        if len(left) != len(right):
            return left if len(left) < len(right) else right
        return min(left, right)

    return build(0, len(s) - 1)


if __name__ == "__main__":
    print(make_palindrome("race"))
    print(make_palindrome("google"))
