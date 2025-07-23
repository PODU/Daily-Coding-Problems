# Day 22: Word Break reconstruction: DP over positions with memoization using a word set.
# Time: O(n^2) substring checks (n = string length), Space: O(n) for memo + recursion.
from functools import lru_cache


def word_break(s, words):
    dict_set = set(words)
    n = len(s)
    memo = {}

    def solve(i):
        if i == n:
            return True
        if i in memo:
            return memo[i] != -1
        for j in range(i + 1, n + 1):
            if s[i:j] in dict_set and solve(j):
                memo[i] = j - i
                return True
        memo[i] = -1
        return False

    if not solve(0):
        return None
    res = []
    i = 0
    while i < n:
        res.append(s[i:i + memo[i]])
        i += memo[i]
    return res


if __name__ == "__main__":
    words = {'quick', 'brown', 'the', 'fox'}
    string = "thequickbrownfox"
    print(word_break(string, words))
