# Day 416: Min king-moves to visit points in order = sum of Chebyshev distances max(|dx|,|dy|).
# Time O(n), Space O(1).


def min_steps(pts):
    total = 0
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        total += max(abs(x1 - x0), abs(y1 - y0))
    return total


if __name__ == "__main__":
    pts = [(0, 0), (1, 1), (1, 2)]
    print(min_steps(pts))
