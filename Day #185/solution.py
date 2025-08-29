# Day 185: Area of intersection of two axis-aligned rectangles (top-left + width/height, y up).
# Overlap = max(0, dx) * max(0, dy). Time O(1), Space O(1).
from typing import Tuple


def intersection_area(a: dict, b: dict) -> int:
    al, at = a["top_left"]; aw, ah = a["dimensions"]
    bl, bt = b["top_left"]; bw, bh = b["dimensions"]
    ox = min(al + aw, bl + bw) - max(al, bl)
    oy = min(at, bt) - max(at - ah, bt - bh)
    if ox <= 0 or oy <= 0:
        return 0
    return ox * oy


if __name__ == "__main__":
    a = {"top_left": (1, 4), "dimensions": (3, 3)}
    b = {"top_left": (0, 5), "dimensions": (4, 3)}
    print(intersection_area(a, b))
