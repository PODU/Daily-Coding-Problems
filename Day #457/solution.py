# Day 457: All start indices in S that are anagrams of W.
# Fixed-size sliding window of char counts. Time O(|S|), Space O(1).
from collections import Counter


def anagram_indices(w, s):
    m, n = len(w), len(s)
    res = []
    if m == 0 or m > n:
        return res
    need = Counter(w)
    win = Counter(s[:m])
    if win == need:
        res.append(0)
    for i in range(m, n):
        win[s[i]] += 1
        left = s[i - m]
        win[left] -= 1
        if win[left] == 0:
            del win[left]
        if win == need:
            res.append(i - m + 1)
    return res


if __name__ == "__main__":
    print(", ".join(map(str, anagram_indices("ab", "abxaba"))))  # 0, 3, 4
