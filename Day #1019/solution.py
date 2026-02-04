# Day 1019: Rectangle intersection area: O(1) time, O(1) space.
# top_left is top y, height extends downward. x_overlap*y_overlap clamped at 0.
def intersect_area(a, b):
    # rect = (left, top, width, height)
    a_right, b_right = a[0] + a[2], b[0] + b[2]
    a_bottom, b_bottom = a[1] - a[3], b[1] - b[3]
    xo = max(0, min(a_right, b_right) - max(a[0], b[0]))
    yo = max(0, min(a[1], b[1]) - max(a_bottom, b_bottom))
    return xo * yo


if __name__ == "__main__":
    r1 = (1, 4, 3, 3)
    r2 = (0, 5, 4, 3)
    area = intersect_area(r1, r2)
    print(int(area) if area == int(area) else area)
