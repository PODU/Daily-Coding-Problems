# Day 1124: Day 1124 - Minimum points to stab all intervals
# Approach: greedy; sort by right endpoint, place a point at the end of each
# not-yet-stabbed interval. Time: O(n log n), Space: O(n).

def stab(intervals):
    points = []
    last = float("-inf")
    for s, e in sorted(intervals, key=lambda iv: iv[1]):
        if s > last:
            last = e
            points.append(e)
    return points


if __name__ == "__main__":
    print(stab([(1, 4), (4, 5), (7, 9), (9, 12)]))  # [4, 9]
