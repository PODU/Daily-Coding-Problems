# Day 77: Merge overlapping intervals. Sort by start, then sweep merging.
# Time O(n log n), Space O(n).


def merge_intervals(intervals):
    intervals = sorted(intervals)
    res = []
    for start, end in intervals:
        if res and start <= res[-1][1]:
            res[-1] = (res[-1][0], max(res[-1][1], end))
        else:
            res.append((start, end))
    return res


if __name__ == "__main__":
    inp = [(1, 3), (5, 8), (4, 10), (20, 25)]
    print(merge_intervals(inp))  # [(1, 3), (4, 10), (20, 25)]
