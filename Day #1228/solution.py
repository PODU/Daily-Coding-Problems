# Day 1228: Merge overlapping intervals: sort by start, sweep merging when start <= last end.
# Time: O(n log n), Space: O(n).
from typing import List, Tuple


def merge(intervals: List[Tuple[int, int]]) -> List[Tuple[int, int]]:
    res = []
    for s, e in sorted(intervals):
        if res and s <= res[-1][1]:
            res[-1] = (res[-1][0], max(res[-1][1], e))
        else:
            res.append((s, e))
    return res


if __name__ == "__main__":
    data = [(1, 3), (5, 8), (4, 10), (20, 25)]
    out = merge(data)
    print("[" + ", ".join(f"({s}, {e})" for s, e in out) + "]")
