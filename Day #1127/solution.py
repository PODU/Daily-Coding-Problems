# Day 1127: Smallest window containing every distinct character of the string.
# Sliding window with frequency counts; expand then shrink. O(n) time, O(k) space.
from collections import defaultdict


def smallest_window(s):
    need = len(set(s))
    cnt = defaultdict(int)
    have = 0
    best = float("inf")
    left = 0
    for right, c in enumerate(s):
        cnt[c] += 1
        if cnt[c] == 1:
            have += 1
        while have == need:
            best = min(best, right - left + 1)
            cnt[s[left]] -= 1
            if cnt[s[left]] == 0:
                have -= 1
            left += 1
    return best


if __name__ == "__main__":
    print(smallest_window("jiujitsu"))
