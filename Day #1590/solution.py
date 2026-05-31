# Day 1590: Smallest window containing every distinct char: sliding window with counts.
# Time O(n), Space O(k).
from collections import defaultdict


def smallest_window(s):
    need = len(set(s))
    cnt = defaultdict(int)
    formed = 0
    left = 0
    best = len(s)
    for right, ch in enumerate(s):
        cnt[ch] += 1
        if cnt[ch] == 1:
            formed += 1
        while formed == need:
            best = min(best, right - left + 1)
            cnt[s[left]] -= 1
            if cnt[s[left]] == 0:
                formed -= 1
            left += 1
    return best


if __name__ == "__main__":
    print(smallest_window("jiujitsu"))
