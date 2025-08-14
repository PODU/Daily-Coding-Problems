# Day 119: Min points to stab all intervals. Greedy: sort by start desc, pick start
# of each not-yet-stabbed interval (mirror of the classic sort-by-end greedy). O(n log n).
def min_cover(intervals):
    pts = []
    last = None
    for s, e in sorted(intervals, key=lambda x: -x[0]):
        if last is None or last > e:
            last = s
            pts.append(s)
    return sorted(pts)


if __name__ == "__main__":
    r = min_cover([(0, 3), (2, 6), (3, 4), (6, 9)])
    print("{" + ", ".join(map(str, r)) + "}")  # {3, 6}
