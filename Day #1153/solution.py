# Day 1153: Min steps to visit points in order (8-directional moves).
# Sum of Chebyshev distances max(|dx|,|dy|) between consecutive points. O(n) time, O(1) space.
def min_steps(pts):
    return sum(
        max(abs(pts[i][0] - pts[i - 1][0]), abs(pts[i][1] - pts[i - 1][1]))
        for i in range(1, len(pts))
    )


if __name__ == "__main__":
    print(min_steps([(0, 0), (1, 1), (1, 2)]))  # 2
