# Day 236: Point in polygon: ray-casting (even-odd rule). Boundary points are detected separately
# and return False. Time: O(N), Space: O(1).


def on_segment(px, py, ax, ay, bx, by):
    cross = (bx - ax) * (py - ay) - (by - ay) * (px - ax)
    if abs(cross) > 1e-9:
        return False
    return (min(ax, bx) - 1e-9 <= px <= max(ax, bx) + 1e-9 and
            min(ay, by) - 1e-9 <= py <= max(ay, by) + 1e-9)


def inside(poly, px, py):
    n = len(poly)
    res = False
    j = n - 1
    for i in range(n):
        xi, yi = poly[i]
        xj, yj = poly[j]
        if on_segment(px, py, xi, yi, xj, yj):
            return False  # boundary
        if (yi > py) != (yj > py) and px < (xj - xi) * (py - yi) / (yj - yi) + xi:
            res = not res
        j = i
    return res


if __name__ == "__main__":
    poly = [(0, 0), (4, 0), (4, 4), (0, 4)]
    print(inside(poly, 2, 2))  # True
    print(inside(poly, 4, 2))  # False (boundary)
    print(inside(poly, 5, 5))  # False (outside)
