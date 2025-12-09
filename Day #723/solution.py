# Day 723: Smallest set of points stabbing all closed intervals.
# Approach: Sort by right endpoint; greedily pick the end of each uncovered interval.
# Time: O(n log n), Space: O(n). (Any minimum-size set is valid; {3,9} here.)

def min_stabbing_points(intervals):
    intervals = sorted(intervals, key=lambda iv: iv[1])
    points = []
    last = float("-inf")
    for s, e in intervals:
        if s > last:
            last = e
            points.append(e)
    return points


if __name__ == "__main__":
    intervals = [(0, 3), (2, 6), (3, 4), (6, 9)]
    pts = min_stabbing_points(intervals)
    print("{" + ", ".join(map(str, pts)) + "}")
