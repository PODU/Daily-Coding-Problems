# Day 111: Anagram substrings via fixed sliding window of char counts. O(|S|).
from collections import Counter


def find_anagrams(s, w):
    n, m = len(s), len(w)
    res = []
    if m > n:
        return res
    need = Counter(w)
    win = Counter(s[:m])
    if win == need:
        res.append(0)
    for i in range(m, n):
        win[s[i]] += 1
        win[s[i - m]] -= 1
        if win[s[i - m]] == 0:
            del win[s[i - m]]
        if win == need:
            res.append(i - m + 1)
    return res


if __name__ == "__main__":
    print(", ".join(map(str, find_anagrams("abxaba", "ab"))))
