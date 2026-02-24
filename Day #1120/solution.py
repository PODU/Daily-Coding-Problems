# Day 1120: Day 1120 - Minimum steps to cover points in order (8-directional moves)
# Approach: with diagonal moves the cost between two points is the Chebyshev
# distance max(|dx|,|dy|); sum over consecutive points. Time: O(n), Space: O(1).

def min_steps(points):
    total = 0
    for (x1, y1), (x2, y2) in zip(points, points[1:]):
        total += max(abs(x2 - x1), abs(y2 - y1))
    return total


if __name__ == "__main__":
    points = [(0, 0), (1, 1), (1, 2)]
    print(min_steps(points))  # 2
