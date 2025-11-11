# Day 582: Greedy interval stabbing: sort by right endpoint, pick right end when uncovered. Time O(n log n).
def stab(intervals):
    points = []
    last = float("-inf")
    for start, end in sorted(intervals, key=lambda iv: iv[1]):
        if start > last:
            last = end
            points.append(end)
    return points


if __name__ == "__main__":
    intervals = [(1, 4), (4, 5), (7, 9), (9, 12)]
    print(stab(intervals))
