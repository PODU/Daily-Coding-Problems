# Day 224: Smallest positive integer not expressible as a subset sum (sorted array).
# Approach: greedy. Keep reachable range [1, ans-1]; if next a <= ans, extend by a, else ans is the gap.
# Time O(N), Space O(1).
from typing import List


def smallest_non_subset_sum(a: List[int]) -> int:
    ans = 1  # smallest unreachable so far
    for x in a:
        if x > ans:
            break
        ans += x
    return ans


if __name__ == "__main__":
    print(smallest_non_subset_sum([1, 2, 3, 10]))  # 7
