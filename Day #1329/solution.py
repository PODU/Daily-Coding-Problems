# Day 1329: Closest pair of points via divide & conquer. O(n log n) time.
# Sort by x, recurse on halves, then check the middle strip ordered by y (each point vs next ~7).
import math


def dist(a, b):
    return math.hypot(a[0] - b[0], a[1] - b[1])


def closest(pts):
    px = sorted(pts)
    py = sorted(pts, key=lambda p: p[1])

    def rec(px, py):
        n = len(px)
        if n <= 3:
            best = (float("inf"), None, None)
            for i in range(n):
                for j in range(i + 1, n):
                    d = dist(px[i], px[j])
                    if d < best[0]:
                        best = (d, px[i], px[j])
            return best
        mid = n // 2
        pivot = px[mid]
        midx = pivot[0]
        lx, rx = px[:mid], px[mid:]
        ly = [p for p in py if p < pivot]
        ry = [p for p in py if not (p < pivot)]
        bl = rec(lx, ly)
        br = rec(rx, ry)
        best = bl if bl[0] <= br[0] else br
        strip = [p for p in py if abs(p[0] - midx) < best[0]]
        for i in range(len(strip)):
            j = i + 1
            while j < len(strip) and (strip[j][1] - strip[i][1]) < best[0]:
                d = dist(strip[i], strip[j])
                if d < best[0]:
                    best = (d, strip[i], strip[j])
                j += 1
        return best

    _, a, b = rec(px, py)
    return sorted([a, b])


if __name__ == "__main__":
    pts = [(1, 1), (-1, -1), (3, 4), (6, 1), (-1, -6), (-4, -3)]
    print(closest(pts))  # [(-1, -1), (1, 1)]
