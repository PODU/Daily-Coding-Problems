# Day 1261: Palindrome pairs.
# Hashmap of words + prefix/suffix palindrome splits. O(n*k^2) time, O(n*k) space.
from typing import List, Tuple


def palindrome_pairs(words: List[str]) -> List[Tuple[int, int]]:
    idx = {w: i for i, w in enumerate(words)}
    res = set()
    for i, w in enumerate(words):
        n = len(w)
        for j in range(n + 1):
            prefix, suffix = w[:j], w[j:]
            if prefix == prefix[::-1]:               # prefix palindrome -> rev(suffix)+w
                back = suffix[::-1]
                if back in idx and idx[back] != i:
                    res.add((idx[back], i))
            if j != n and suffix == suffix[::-1]:     # suffix palindrome -> w+rev(prefix)
                back = prefix[::-1]
                if back in idx and idx[back] != i:
                    res.add((i, idx[back]))
    return sorted(res)


if __name__ == "__main__":
    print(palindrome_pairs(["code", "edoc", "da", "d"]))
