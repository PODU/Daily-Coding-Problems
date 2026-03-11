# Day 1195: Smallest window containing all distinct chars of the string. Sliding window:
# expand right, shrink left while all distinct kinds present. O(N) time, O(K) space.
from collections import defaultdict


def smallest_window(s):
    distinct = len(set(s))
    cnt = defaultdict(int)
    have = 0
    left = 0
    best = float("inf")
    for right, ch in enumerate(s):
        cnt[ch] += 1
        if cnt[ch] == 1:
            have += 1
        while have == distinct:
            best = min(best, right - left + 1)
            lc = s[left]
            cnt[lc] -= 1
            if cnt[lc] == 0:
                have -= 1
            left += 1
    return 0 if best == float("inf") else best


def main():
    print(smallest_window("jiujitsu"))


if __name__ == "__main__":
    main()
