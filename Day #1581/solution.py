# Day 1581: Area of intersection of two axis-aligned rectangles.
# top_left=(x,y), dims=(w,h); x-range [x,x+w], y-range [y-h,y]. Overlap = clamped widths.
# Time: O(1); Space: O(1).


def intersection_area(r1, r2):
    (x1, y1), (w1, h1) = r1["top_left"], r1["dimensions"]
    (x2, y2), (w2, h2) = r2["top_left"], r2["dimensions"]
    ow = min(x1 + w1, x2 + w2) - max(x1, x2)
    oh = min(y1, y2) - max(y1 - h1, y2 - h2)
    if ow <= 0 or oh <= 0:
        return 0
    return ow * oh


if __name__ == "__main__":
    a = {"top_left": (1, 4), "dimensions": (3, 3)}
    b = {"top_left": (0, 5), "dimensions": (4, 3)}
    print(intersection_area(a, b))  # 6
