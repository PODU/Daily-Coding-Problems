# Day 1254: Interval point stabbing: greedy, sort by right endpoint, pick endpoint when uncovered.
# Time O(n log n), Space O(n).
def stab(intervals):
    pts, last = [], float("-inf")
    for s, e in sorted(intervals, key=lambda x: x[1]):
        if s > last:
            last = e
            pts.append(e)
    return pts

if __name__ == "__main__":
    print(stab([(1, 4), (4, 5), (7, 9), (9, 12)]))
