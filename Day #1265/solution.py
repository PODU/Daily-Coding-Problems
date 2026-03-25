# Day 1265: Reconstruct a sentence from a space-free string and a dictionary.
# DP over prefixes storing one valid breakpoint. O(n^2) time (avg), O(n) space.
from typing import List, Optional, Set


def word_break(s: str, words: Set[str]) -> Optional[List[str]]:
    n = len(s)
    back = [-2] * (n + 1)   # back[i] = start of word ending at i; -2 = unreachable
    back[0] = -1
    for i in range(1, n + 1):
        for j in range(i):
            if back[j] != -2 and s[j:i] in words:
                back[i] = j
                break
    if back[n] == -2:
        return None
    res = []
    i = n
    while i > 0:
        res.append(s[back[i]:i])
        i = back[i]
    return res[::-1]


if __name__ == "__main__":
    print(word_break("thequickbrownfox", {"quick", "brown", "the", "fox"}))
