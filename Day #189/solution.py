# Day 189: Longest contiguous subarray with all distinct elements.
# Sliding window with last-seen map. Time O(n), Space O(n).
from typing import List


def longest_distinct(a: List[int]) -> int:
    last = {}
    best = start = 0
    for i, x in enumerate(a):
        if x in last and last[x] >= start:
            start = last[x] + 1
        last[x] = i
        best = max(best, i - start + 1)
    return best


if __name__ == "__main__":
    print(longest_distinct([5, 1, 3, 5, 2, 3, 4, 1]))
