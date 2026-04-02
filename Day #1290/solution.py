# Day 1290: Strict point-in-polygon test (ray casting), boundary counts as outside.
# Check edges for on-boundary, then parity of rightward ray crossings. Time O(n), Space O(1).
def on_segment(a, b, p):
    cross = (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])
    if abs(cross) > 1e-9:
        return False
    return (min(a[0], b[0]) - 1e-9 <= p[0] <= max(a[0], b[0]) + 1e-9 and
            min(a[1], b[1]) - 1e-9 <= p[1] <= max(a[1], b[1]) + 1e-9)


def inside(poly, p):
    n = len(poly)
    for i in range(n):
        if on_segment(poly[i], poly[(i + 1) % n], p):
            return False  # boundary
    res = False
    j = n - 1
    for i in range(n):
        xi, yi = poly[i]
        xj, yj = poly[j]
        if (yi > p[1]) != (yj > p[1]):
            xint = (xj - xi) * (p[1] - yi) / (yj - yi) + xi
            if p[0] < xint:
                res = not res
        j = i
    return res


if __name__ == "__main__":
    square = [(0, 0), (4, 0), (4, 4), (0, 4)]
    print(inside(square, (2, 2)))  # True
    print(inside(square, (5, 5)))  # False
    print(inside(square, (4, 2)))  # False (boundary)
