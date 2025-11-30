# Day 674: Longest contiguous subarray with at most 2 distinct values. Sliding window.
# Time O(n), Space O(1) (at most 3 keys in the map).
from collections import defaultdict


def longest_two_types(a):
    cnt = defaultdict(int)
    best = l = 0
    for r, x in enumerate(a):
        cnt[x] += 1
        while len(cnt) > 2:
            cnt[a[l]] -= 1
            if cnt[a[l]] == 0:
                del cnt[a[l]]
            l += 1
        best = max(best, r - l + 1)
    return best


if __name__ == "__main__":
    print(longest_two_types([2, 1, 2, 3, 3, 1, 3, 5]))  # 4
