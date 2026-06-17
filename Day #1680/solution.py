# Day 1680: Strict point-in-polygon. Reject boundary via on-segment test, else
# ray-casting parity. Time O(N), Space O(1).


def on_seg(x1, y1, x2, y2, px, py):
    cross = (x2 - x1) * (py - y1) - (y2 - y1) * (px - x1)
    if abs(cross) > 1e-9:
        return False
    return (min(x1, x2) - 1e-9 <= px <= max(x1, x2) + 1e-9 and
            min(y1, y2) - 1e-9 <= py <= max(y1, y2) + 1e-9)


def inside(poly, px, py):
    n = len(poly)
    for i in range(n):
        x1, y1 = poly[i]
        x2, y2 = poly[(i + 1) % n]
        if on_seg(x1, y1, x2, y2, px, py):
            return False  # boundary counts as outside
    res = False
    j = n - 1
    for i in range(n):
        xi, yi = poly[i]
        xj, yj = poly[j]
        if (yi > py) != (yj > py) and px < (xj - xi) * (py - yi) / (yj - yi) + xi:
            res = not res
        j = i
    return res


if __name__ == "__main__":
    sq = [(0, 0), (4, 0), (4, 4), (0, 4)]
    print(inside(sq, 2, 2))  # True
    print(inside(sq, 4, 2))  # False (boundary)
    print(inside(sq, 5, 5))  # False (outside)
