# Day 711: Point strictly inside polygon. First reject boundary via on-segment
# test, then ray-casting parity test. Time O(N) per query.

def on_seg(a, b, p):
    cross = (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])
    if abs(cross) > 1e-9:
        return False
    return (min(a[0], b[0]) - 1e-9 <= p[0] <= max(a[0], b[0]) + 1e-9 and
            min(a[1], b[1]) - 1e-9 <= p[1] <= max(a[1], b[1]) + 1e-9)


def inside(poly, p):
    n = len(poly)
    for i in range(n):
        if on_seg(poly[i], poly[(i + 1) % n], p):
            return False
    inq = False
    j = n - 1
    for i in range(n):
        if (poly[i][1] > p[1]) != (poly[j][1] > p[1]):
            xint = (poly[j][0] - poly[i][0]) * (p[1] - poly[i][1]) / \
                   (poly[j][1] - poly[i][1]) + poly[i][0]
            if p[0] < xint:
                inq = not inq
        j = i
    return inq


if __name__ == "__main__":
    sq = [(0, 0), (4, 0), (4, 4), (0, 4)]
    print("True" if inside(sq, (2, 2)) else "False")
    print("True" if inside(sq, (4, 2)) else "False")
    print("True" if inside(sq, (5, 5)) else "False")
