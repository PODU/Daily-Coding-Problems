# Day 909: Minimum points covering all closed intervals: greedy, sort by END ascending; open a new
# group when start>anchor-end, pick each group's MAX start as its point. Time O(n log n), Space O(n).

def min_points(intervals):
    intervals = sorted(intervals, key=lambda iv: iv[1])
    points = []
    have = False
    anchor_end = group_max_start = None
    for start, end in intervals:
        if not have or start > anchor_end:
            if have:
                points.append(group_max_start)
            have = True
            anchor_end = end
            group_max_start = start
        elif start > group_max_start:
            group_max_start = start
    if have:
        points.append(group_max_start)
    return points


if __name__ == "__main__":
    intervals = [[0, 3], [2, 6], [3, 4], [6, 9]]
    pts = min_points(intervals)
    print("{" + ", ".join(str(p) for p in pts) + "}")
