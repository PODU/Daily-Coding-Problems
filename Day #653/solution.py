# Day 653: Brute-force all O(k^2) pairs; rectangles overlap iff their projections strictly overlap on both axes.
# Sweep-line O(k log k) is possible but k^2 is clear. Time O(k^2), space O(k).
from itertools import combinations


def make(x, y, w, h):
    # top_left (x,y), dims (w,h): x1=x, x2=x+w, y2=y(top), y1=y-h(bottom)
    return (x, y - h, x + w, y)


def overlap(a, b):
    return a[0] < b[2] and b[0] < a[2] and a[1] < b[3] and b[1] < a[3]


def main():
    rects = [
        make(1, 4, 3, 3),    # R1
        make(-1, 3, 2, 1),   # R2
        make(0, 5, 4, 3),    # R3
    ]
    any_overlap = any(overlap(a, b) for a, b in combinations(rects, 2))
    print("true" if any_overlap else "false")


if __name__ == "__main__":
    main()
