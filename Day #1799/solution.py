# Day 1799: Longest contiguous subarray with at most 2 distinct values via sliding window + hashmap. O(n) time, O(1) space.
from collections import defaultdict


def longest_two_distinct(a):
    cnt = defaultdict(int)
    left = 0
    best = 0
    for right, val in enumerate(a):
        cnt[val] += 1
        while len(cnt) > 2:
            cnt[a[left]] -= 1
            if cnt[a[left]] == 0:
                del cnt[a[left]]
            left += 1
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    print(longest_two_distinct([2, 1, 2, 3, 3, 1, 3, 5]))  # 4
