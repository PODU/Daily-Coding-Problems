# Day 200: Minimum points stabbing all intervals.
# Greedy: sort by right endpoint; pick the right end whenever current interval is unstabbed.
# Time: O(n log n), Space: O(1).


def stab(intervals):
    pts = []
    last = float("-inf")
    for lo, hi in sorted(intervals, key=lambda p: p[1]):
        if lo > last:
            last = hi
            pts.append(last)
    return pts


if __name__ == "__main__":
    print(stab([(1, 4), (4, 5), (7, 9), (9, 12)]))  # [4, 9]
