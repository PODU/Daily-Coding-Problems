# Day 298: Longest contiguous subarray with at most 2 distinct values via sliding window + count map.
# Time: O(n), Space: O(1) (at most 3 keys in map).
from collections import defaultdict


def longest_at_most_2(a):
    cnt = defaultdict(int)
    left = best = 0
    for right, v in enumerate(a):
        cnt[v] += 1
        while len(cnt) > 2:
            cnt[a[left]] -= 1
            if cnt[a[left]] == 0:
                del cnt[a[left]]
            left += 1
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    print(longest_at_most_2([2, 1, 2, 3, 3, 1, 3, 5]))
