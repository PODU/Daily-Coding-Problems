# Day 510: All index pairs (i,j) where words[i]+words[j] is a palindrome.
# Hash map of reversed words + prefix/suffix palindrome checks. Time O(N*L^2).
from typing import List, Tuple


def is_pal(s: str) -> bool:
    return s == s[::-1]


def palindrome_pairs(words: List[str]) -> List[Tuple[int, int]]:
    rev = {w[::-1]: i for i, w in enumerate(words)}
    result = set()
    for i, w in enumerate(words):
        n = len(w)
        for cut in range(n + 1):
            if is_pal(w[:cut]):                # left palindrome
                j = rev.get(w[cut:])
                if j is not None and j != i:
                    result.add((j, i))
            if cut != n and is_pal(w[cut:]):   # right palindrome
                j = rev.get(w[:cut])
                if j is not None and j != i:
                    result.add((i, j))
    return sorted(result)


if __name__ == "__main__":
    words = ["code", "edoc", "da", "d"]
    print(palindrome_pairs(words))
