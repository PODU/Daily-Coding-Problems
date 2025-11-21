# Day 637: Merge overlapping intervals.
# Approach: sort by start, sweep merging while next.start <= cur.end.
# Time: O(n log n), Space: O(n).
def merge(intervals):
    intervals = sorted(intervals)
    res = []
    for s, e in intervals:
        if res and s <= res[-1][1]:
            res[-1] = (res[-1][0], max(res[-1][1], e))
        else:
            res.append((s, e))
    return res


if __name__ == "__main__":
    print(merge([(1, 3), (5, 8), (4, 10), (20, 25)]))
