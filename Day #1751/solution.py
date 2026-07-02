# Day 1751: Min steps to visit points in order on an 8-directional grid.
# Sum of Chebyshev distances max(|dx|,|dy|) between consecutive points. O(n) time, O(1) space.


def min_steps(pts):
    total = 0
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        total += max(abs(x1 - x0), abs(y1 - y0))
    return total


if __name__ == "__main__":
    points = [(0, 0), (1, 1), (1, 2)]
    print(min_steps(points))  # 2
