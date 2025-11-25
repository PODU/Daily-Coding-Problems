# Day 654: Smallest window containing every distinct char: sliding window with two pointers.
# Expand right until all distinct chars present, shrink left to minimize. Time O(n), space O(k).
from collections import defaultdict


def smallest_window(s: str) -> int:
    need = len(set(s))
    cnt = defaultdict(int)
    have = 0
    best = float("inf")
    left = 0
    for right, ch in enumerate(s):
        cnt[ch] += 1
        if cnt[ch] == 1:
            have += 1
        while have == need:
            best = min(best, right - left + 1)
            cnt[s[left]] -= 1
            if cnt[s[left]] == 0:
                have -= 1
            left += 1
    return 0 if best == float("inf") else best


if __name__ == "__main__":
    print(smallest_window("jiujitsu"))
