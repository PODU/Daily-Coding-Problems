# Day 437: Shortest substring containing all chars of a set via sliding window.
# O(N) time, O(set) space.
from collections import defaultdict


def shortest_substring(s, chars):
    if not chars:
        return ""
    need = {c: 0 for c in chars}
    required = len(chars)
    formed = 0
    best_len = float("inf")
    best_l = 0
    l = 0
    for r, c in enumerate(s):
        if c in need:
            if need[c] == 0:
                formed += 1
            need[c] += 1
        while formed == required:
            if r - l + 1 < best_len:
                best_len = r - l + 1
                best_l = l
            lc = s[l]
            if lc in need:
                need[lc] -= 1
                if need[lc] == 0:
                    formed -= 1
            l += 1
    return None if best_len == float("inf") else s[best_l:best_l + best_len]


if __name__ == "__main__":
    res = shortest_substring("figehaeci", {"a", "e", "i"})
    print("null" if res is None else f'"{res}"')  # "aeci"
