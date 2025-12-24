# Day 796: Point strictly inside a polygon.
# Ray-casting (even-odd rule) + on-boundary check. Time O(N), Space O(1).


def on_segment(a, b, p):
    cross = (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])
    if abs(cross) > 1e-9:
        return False
    return (min(a[0], b[0]) - 1e-9 <= p[0] <= max(a[0], b[0]) + 1e-9 and
            min(a[1], b[1]) - 1e-9 <= p[1] <= max(a[1], b[1]) + 1e-9)


def inside_polygon(poly, p):
    n = len(poly)
    for i in range(n):
        if on_segment(poly[i], poly[(i + 1) % n], p):
            return False  # boundary counts as outside
    inside = False
    j = n - 1
    for i in range(n):
        if ((poly[i][1] > p[1]) != (poly[j][1] > p[1]) and
            p[0] < (poly[j][0] - poly[i][0]) * (p[1] - poly[i][1]) /
                   (poly[j][1] - poly[i][1]) + poly[i][0]):
            inside = not inside
        j = i
    return inside


if __name__ == "__main__":
    square = [(0, 0), (4, 0), (4, 4), (0, 4)]
    print(inside_polygon(square, (2, 2)))  # True
    print(inside_polygon(square, (4, 2)))  # False (boundary)
    print(inside_polygon(square, (5, 5)))  # False
