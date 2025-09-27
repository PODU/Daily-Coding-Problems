# Day 340: Closest pair of points via divide & conquer. O(n log n) time, O(n) space.
# Sort by x, recurse on halves, merge by checking a y-sorted strip near the split.
import math

def dist(a, b):
    return math.hypot(a[0]-b[0], a[1]-b[1])

def closest(px, py):
    n = len(px)
    if n <= 3:
        best = (float('inf'), None, None)
        for i in range(n):
            for j in range(i+1, n):
                d = dist(px[i], px[j])
                if d < best[0]:
                    best = (d, px[i], px[j])
        return best
    mid = n // 2
    midx = px[mid][0]
    lx, rx = px[:mid], px[mid:]
    # partition py preserving y-order
    left_set = {}
    for p in lx:
        left_set[p] = left_set.get(p, 0) + 1
    ly, ry = [], []
    for p in py:
        if left_set.get(p, 0) > 0:
            ly.append(p); left_set[p] -= 1
        else:
            ry.append(p)
    bl = closest(lx, ly)
    br = closest(rx, ry)
    best = bl if bl[0] < br[0] else br
    d = best[0]
    strip = [p for p in py if abs(p[0] - midx) < d]
    for i in range(len(strip)):
        j = i + 1
        while j < len(strip) and (strip[j][1] - strip[i][1]) < d:
            dd = dist(strip[i], strip[j])
            if dd < d:
                d = dd; best = (dd, strip[i], strip[j])
            j += 1
    return best

def main():
    pts = [(1,1),(-1,-1),(3,4),(6,1),(-1,-6),(-4,-3)]
    px = sorted(pts, key=lambda p:(p[0],p[1]))
    py = sorted(pts, key=lambda p:(p[1],p[0]))
    _, a, b = closest(px, py)
    a, b = sorted([a, b])
    print(f"[({a[0]}, {a[1]}), ({b[0]}, {b[1]})]")

if __name__ == "__main__":
    main()
