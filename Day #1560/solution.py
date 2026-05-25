# Day 1560: Greedy interval stabbing: sort by right endpoint; for the smallest uncovered right,
# pick point = max left among intervals containing it, cover them all. Time O(n^2), Space O(n).

def stab_points(intervals):
    iv = sorted(intervals, key=lambda x: x[1])
    n = len(iv)
    covered = [False] * n
    points = []
    for i in range(n):
        if covered[i]:
            continue
        r = iv[i][1]
        point = max(iv[j][0] for j in range(i, n) if not covered[j] and iv[j][0] <= r)
        points.append(point)
        for j in range(i, n):
            if not covered[j] and iv[j][0] <= point <= iv[j][1]:
                covered[j] = True
    return points


if __name__ == "__main__":
    pts = stab_points([[0, 3], [2, 6], [3, 4], [6, 9]])
    print("{" + ", ".join(str(p) for p in pts) + "}")
