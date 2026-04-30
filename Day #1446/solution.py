# Day 1446: Minimum set of points hitting every closed interval.
# Greedy: sort by right endpoint; whenever the current interval is unhit, pick
# its right endpoint. Time O(n log n), Space O(1). (Any minimal set is valid.)
from typing import List, Tuple


def min_stabbing_set(intervals: List[Tuple[int, int]]) -> List[int]:
    points = []
    last = float("-inf")
    for l, r in sorted(intervals, key=lambda iv: iv[1]):
        if l > last:
            points.append(r)
            last = r
    return points


if __name__ == "__main__":
    intervals = [(0, 3), (2, 6), (3, 4), (6, 9)]
    pts = min_stabbing_set(intervals)
    print("{" + ", ".join(map(str, pts)) + "}")  # e.g. {3, 9}; {3, 6} also valid
