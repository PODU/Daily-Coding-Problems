# Day 190: Maximum circular subarray sum, empty subarray (sum 0) allowed.
# ans = max(0, maxKadane, total - minKadane). Time O(n), Space O(1).
from typing import List


def max_circular_sum(a: List[int]) -> int:
    total = 0
    max_k = float("-inf"); cur_max = 0
    min_k = float("inf"); cur_min = 0
    for x in a:
        total += x
        cur_max = max(x, cur_max + x); max_k = max(max_k, cur_max)
        cur_min = min(x, cur_min + x); min_k = min(min_k, cur_min)
    return max(0, max_k, total - min_k)


if __name__ == "__main__":
    print(max_circular_sum([8, -1, 3, 4]))
    print(max_circular_sum([-4, 5, 1, 0]))
