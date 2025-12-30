# Day 822: Merge overlapping intervals: sort by start, merge when next.start <= current.end.
# Time: O(n log n), Space: O(n).

def merge(intervals):
    res = []
    for start, end in sorted(intervals):
        if res and start <= res[-1][1]:
            res[-1] = (res[-1][0], max(res[-1][1], end))
        else:
            res.append((start, end))
    return res


if __name__ == "__main__":
    merged = merge([(1, 3), (5, 8), (4, 10), (20, 25)])
    print("[" + ", ".join("(%d, %d)" % (a, b) for a, b in merged) + "]")
