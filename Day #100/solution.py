# Day 100: With 8-directional moves, the steps between two points equals the
# Chebyshev distance max(|dx|,|dy|). Sum over consecutive points. O(n) time.
def min_steps(points):
    total = 0
    for (x1, y1), (x2, y2) in zip(points, points[1:]):
        total += max(abs(x2 - x1), abs(y2 - y1))
    return total


if __name__ == "__main__":
    print(min_steps([(0, 0), (1, 1), (1, 2)]))  # 2
