# Day 1766: Merge overlapping intervals: sort by start, then sweep merging when the next
# start <= current end. Time: O(n log n), Space: O(n).

def merge(intervals):
    res = []
    for s, e in sorted(intervals):
        if res and s <= res[-1][1]:
            res[-1] = (res[-1][0], max(res[-1][1], e))
        else:
            res.append((s, e))
    return res


if __name__ == "__main__":
    iv = [(1, 3), (5, 8), (4, 10), (20, 25)]
    r = merge(iv)
    print("[" + ", ".join("(%d, %d)" % (a, b) for a, b in r) + "]")
