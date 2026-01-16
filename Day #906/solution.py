# Day 906: Closest pair of points via divide & conquer: sort by x, recurse, merge with strip check by y.
# O(n log n) time, O(n) space.
import math


def dist(a, b):
    return math.hypot(a[0] - b[0], a[1] - b[1])


def closest_pair(points):
    px = sorted(points)

    best = [float("inf"), None, None]

    def consider(a, b):
        d = dist(a, b)
        if d < best[0]:
            best[0], best[1], best[2] = d, a, b

    # returns list sorted by y; updates best via closure
    def rec(lo, hi):
        n = hi - lo
        if n <= 3:
            pts = px[lo:hi]
            for i in range(len(pts)):
                for j in range(i + 1, len(pts)):
                    consider(pts[i], pts[j])
            return sorted(pts, key=lambda p: p[1])
        mid = lo + n // 2
        midx = px[mid][0]
        left = rec(lo, mid)
        right = rec(mid, hi)
        # merge by y
        merged = []
        i = j = 0
        while i < len(left) and j < len(right):
            if left[i][1] <= right[j][1]:
                merged.append(left[i]); i += 1
            else:
                merged.append(right[j]); j += 1
        merged.extend(left[i:])
        merged.extend(right[j:])
        d = best[0]
        strip = [p for p in merged if abs(p[0] - midx) < d]
        for a in range(len(strip)):
            b = a + 1
            while b < len(strip) and (strip[b][1] - strip[a][1]) < best[0]:
                consider(strip[a], strip[b])
                b += 1
        return merged

    rec(0, len(px))
    a, b = sorted([best[1], best[2]])
    return [a, b]


if __name__ == "__main__":
    points = [(1, 1), (-1, -1), (3, 4), (6, 1), (-1, -6), (-4, -3)]
    a, b = closest_pair(points)
    print("[({}, {}), ({}, {})]".format(a[0], a[1], b[0], b[1]))
