# Day 1854: Shortest substring containing all chars in a set.
# Sliding window with a need-counter; expand right, contract left while valid. O(n) time, O(|set|) space.
from collections import Counter


def shortest_substring(s, need):
    want = Counter(need)
    required = len(want)
    win = Counter()
    formed = 0
    best_len = float("inf")
    best_l = 0
    l = 0
    for r, c in enumerate(s):
        if c in want:
            win[c] += 1
            if win[c] == want[c]:
                formed += 1
        while formed == required:
            if r - l + 1 < best_len:
                best_len = r - l + 1
                best_l = l
            lc = s[l]
            l += 1
            if lc in want:
                win[lc] -= 1
                if win[lc] < want[lc]:
                    formed -= 1
    return None if best_len == float("inf") else s[best_l:best_l + best_len]


if __name__ == "__main__":
    res = shortest_substring("figehaeci", {"a", "e", "i"})
    print(res if res is not None else "null")  # aeci
