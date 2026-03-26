# Day 1270: Find all start indices in S that are anagrams of W.
# Fixed-size sliding window with a Counter. O(|S|) time.
from collections import Counter
from typing import List


def find_anagrams(w: str, s: str) -> List[int]:
    m, n = len(w), len(s)
    if m > n:
        return []
    need = Counter(w)
    win = Counter(s[:m])
    res = [0] if win == need else []
    for i in range(m, n):
        win[s[i]] += 1
        win[s[i - m]] -= 1
        if win[s[i - m]] == 0:
            del win[s[i - m]]
        if win == need:
            res.append(i - m + 1)
    return res


if __name__ == "__main__":
    print(", ".join(map(str, find_anagrams("ab", "abxaba"))))
