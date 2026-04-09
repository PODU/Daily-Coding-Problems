# Day 1331: Does any pair of rectangles overlap (full containment counts; edge-touching does not)?
# Convert top_left+dimensions to [xmin,xmax,ymin,ymax]; pairwise strict-interval overlap test. O(n^2).
from itertools import combinations


def make(tlx, tly, w, h):
    return (tlx, tlx + w, tly - h, tly)  # xmin, xmax, ymin, ymax


def overlap(a, b):
    return a[0] < b[1] and b[0] < a[1] and a[2] < b[3] and b[2] < a[3]


def any_overlap(rects):
    return any(overlap(a, b) for a, b in combinations(rects, 2))


if __name__ == "__main__":
    rects = [
        make(1, 4, 3, 3),
        make(-1, 3, 2, 1),
        make(0, 5, 4, 3),
    ]
    print(any_overlap(rects))  # True
