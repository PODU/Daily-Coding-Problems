# Day 187: Do any two rectangles overlap (containment counts; edge-touching does not).
# Pairwise interior-overlap test. Time O(n^2), Space O(1). (Sweep line gives O(n log n).)
from itertools import combinations
from typing import List


def overlap(a: dict, b: dict) -> bool:
    al, at = a["top_left"]; aw, ah = a["dimensions"]
    bl, bt = b["top_left"]; bw, bh = b["dimensions"]
    ox = min(al + aw, bl + bw) - max(al, bl)
    oy = min(at, bt) - max(at - ah, bt - bh)
    return ox > 0 and oy > 0


def any_overlap(rs: List[dict]) -> bool:
    return any(overlap(a, b) for a, b in combinations(rs, 2))


if __name__ == "__main__":
    rs = [
        {"top_left": (1, 4), "dimensions": (3, 3)},
        {"top_left": (-1, 3), "dimensions": (2, 1)},
        {"top_left": (0, 5), "dimensions": (4, 3)},
    ]
    print(str(any_overlap(rs)).lower())
