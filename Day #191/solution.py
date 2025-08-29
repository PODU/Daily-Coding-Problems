# Day 191: Min intervals to remove = n - max non-overlapping set (touching allowed).
# Greedy by earliest end. Time O(n log n), Space O(1).
from typing import List, Tuple


def min_removals(iv: List[Tuple[int, int]]) -> int:
    iv = sorted(iv, key=lambda p: p[1])
    kept, end = 0, float("-inf")
    for s, e in iv:
        if s >= end:
            kept += 1
            end = e
    return len(iv) - kept


if __name__ == "__main__":
    print(min_removals([(7, 9), (2, 4), (5, 8)]))
