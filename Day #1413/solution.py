# Day 1413: shortest substring of s containing all characters in a set.
# Approach: sliding window, expand right then shrink left while valid. O(n) time, O(k) space.
from collections import Counter


def shortest_substring(s, want):
    need = Counter(want)
    required = len(need)
    win = Counter()
    formed = 0
    best_len = float("inf")
    best_l = 0
    l = 0
    for r, c in enumerate(s):
        if c in need:
            win[c] += 1
            if win[c] == need[c]:
                formed += 1
        while formed == required:
            if r - l + 1 < best_len:
                best_len = r - l + 1
                best_l = l
            lc = s[l]
            if lc in need:
                win[lc] -= 1
                if win[lc] < need[lc]:
                    formed -= 1
            l += 1
    return None if best_len == float("inf") else s[best_l:best_l + best_len]


if __name__ == "__main__":
    print(shortest_substring("figehaeci", {"a", "e", "i"}))  # aeci
