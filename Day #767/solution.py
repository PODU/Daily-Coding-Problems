# Day 767: Find all start indices in S that are anagrams of W.
# Sliding window of size |W| with a count match. O(|S|) time, O(1) space.
from collections import Counter


def find_anagrams(s, w):
    n, m = len(s), len(w)
    if m > n:
        return []
    need = Counter(w)
    win = Counter(s[:m])
    res = []
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
    print(", ".join(map(str, find_anagrams("abxaba", "ab"))))  # 0, 3, 4
