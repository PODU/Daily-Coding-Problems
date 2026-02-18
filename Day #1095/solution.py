# Day 1095: Detect if any pair of axis-aligned rectangles overlap (containment counts). Time O(n^2), Space O(n).
def from_top_left(x, y, w, h):
    # top_left corner; width grows right, height grows down
    return (x, x + w, y - h, y)  # (minx, maxx, miny, maxy)


def overlap(a, b):
    return a[0] < b[1] and b[0] < a[1] and a[2] < b[3] and b[2] < a[3]


def any_overlap(rects):
    for i in range(len(rects)):
        for j in range(i + 1, len(rects)):
            if overlap(rects[i], rects[j]):
                return True
    return False


if __name__ == "__main__":
    rects = [
        from_top_left(1, 4, 3, 3),
        from_top_left(-1, 3, 2, 1),
        from_top_left(0, 5, 4, 3)]
    print("true" if any_overlap(rects) else "false")
