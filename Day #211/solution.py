# Day 211: All occurrences of pattern in string via KMP.
# Approach: Knuth-Morris-Pratt with LPS failure function. Time O(n+m), Space O(m).
from typing import List


def find_occurrences(s: str, p: str) -> List[int]:
    m, n = len(p), len(s)
    res: List[int] = []
    if m == 0 or m > n:
        return res
    lps = [0] * m
    length = 0
    i = 1
    while i < m:
        if p[i] == p[length]:
            length += 1
            lps[i] = length
            i += 1
        elif length:
            length = lps[length - 1]
        else:
            lps[i] = 0
            i += 1
    i = j = 0
    while i < n:
        if s[i] == p[j]:
            i += 1
            j += 1
            if j == m:
                res.append(i - m)
                j = lps[j - 1]
        elif j:
            j = lps[j - 1]
        else:
            i += 1
    return res


if __name__ == "__main__":
    print(find_occurrences("abracadabra", "abr"))
