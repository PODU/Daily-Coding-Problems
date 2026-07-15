# Day 1818: Closest pair of points via divide-and-conquer on x, strip-merge on y.
# Each point tagged with an id for an unambiguous left/right split. Time: O(n log n). Space: O(n).


def dist2(a, b):
    return (a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2


def _rec(px, py):
    n = len(px)
    if n <= 3:
        best, bp = float("inf"), (px[0], px[0])
        for i in range(n):
            for j in range(i + 1, n):
                if dist2(px[i], px[j]) < best:
                    best, bp = dist2(px[i], px[j]), (px[i], px[j])
        return bp
    mid = n // 2
    mid_x = px[mid][0]
    lx, rx = px[:mid], px[mid:]
    left_ids = {p[2] for p in lx}
    ly = [p for p in py if p[2] in left_ids]
    ry = [p for p in py if p[2] not in left_ids]

    bl = _rec(lx, ly)
    br = _rec(rx, ry)
    best = bl if dist2(*bl) < dist2(*br) else br
    d2 = dist2(*best)

    strip = [p for p in py if (p[0] - mid_x) ** 2 < d2]
    for i in range(len(strip)):
        j = i + 1
        while j < len(strip) and (strip[j][1] - strip[i][1]) ** 2 < d2:
            if dist2(strip[i], strip[j]) < d2:
                d2 = dist2(strip[i], strip[j])
                best = (strip[i], strip[j])
            j += 1
    return best


def closest_pair(points):
    pts = [(x, y, i) for i, (x, y) in enumerate(points)]
    px = sorted(pts, key=lambda p: (p[0], p[1]))
    py = sorted(pts, key=lambda p: (p[1], p[0]))
    a, b = _rec(px, py)
    a, b = (a[0], a[1]), (b[0], b[1])
    return sorted([a, b])


if __name__ == "__main__":
    pts = [(1, 1), (-1, -1), (3, 4), (6, 1), (-1, -6), (-4, -3)]
    res = closest_pair(pts)
    print([tuple(res[0]), tuple(res[1])])  # [(-1, -1), (1, 1)]
