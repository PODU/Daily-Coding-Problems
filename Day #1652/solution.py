# Day 1652: Shortest palindrome by insertions, lexicographically earliest: memoized DP on
# (i,j) building best palindrome for s[i..j]. Time O(n^2) states, Space O(n^2).
from functools import lru_cache


def shortest_palindrome(s):
    @lru_cache(maxsize=None)
    def solve(i, j):
        if i > j:
            return ""
        if i == j:
            return s[i]
        if s[i] == s[j]:
            return s[i] + solve(i + 1, j - 1) + s[i]
        opt1 = s[i] + solve(i + 1, j) + s[i]
        opt2 = s[j] + solve(i, j - 1) + s[j]
        if len(opt1) < len(opt2):
            return opt1
        if len(opt2) < len(opt1):
            return opt2
        return min(opt1, opt2)

    return solve(0, len(s) - 1)


if __name__ == "__main__":
    print(shortest_palindrome("race"))
    print(shortest_palindrome("google"))
