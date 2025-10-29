# Day 511: Minimum jumps to reach end of array (each value = max jump length).
# Greedy level-by-level reachability. Time O(n), Space O(1).
from typing import List


def min_jumps(a: List[int]) -> int:
    n = len(a)
    if n <= 1:
        return 0
    jumps = cur_end = farthest = 0
    for i in range(n - 1):
        farthest = max(farthest, i + a[i])
        if i == cur_end:
            jumps += 1
            cur_end = farthest
            if cur_end >= n - 1:
                break
    return jumps


if __name__ == "__main__":
    a = [6, 2, 4, 0, 5, 1, 1, 4, 2, 9]
    print(min_jumps(a))
