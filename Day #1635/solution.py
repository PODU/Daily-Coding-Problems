# Day 1635: Minimum stabbing points: greedy sort intervals by right endpoint; add right
# endpoint when current interval is unstabbed. Time O(n log n), Space O(n).

def min_stabbing_points(intervals):
    points = []
    last = float("-inf")
    for a, b in sorted(intervals, key=lambda iv: iv[1]):
        if a > last:
            points.append(b)
            last = b
    return points


if __name__ == "__main__":
    intervals = [(1, 4), (4, 5), (7, 9), (9, 12)]
    print(min_stabbing_points(intervals))
