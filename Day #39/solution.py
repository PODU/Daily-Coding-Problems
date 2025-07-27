# Day 39: Game of Life on infinite board: track live cells in a set, count neighbors via a
# dict over live cells + neighbors each step. O(live*9) per step. Print cropped board.
from collections import defaultdict


def step(live):
    counts = defaultdict(int)
    for (x, y) in live:
        for dx in (-1, 0, 1):
            for dy in (-1, 0, 1):
                if dx or dy:
                    counts[(x + dx, y + dy)] += 1
    next_live = set()
    for cell, n in counts.items():
        if n == 3 or (cell in live and n == 2):
            next_live.add(cell)
    return next_live


def print_board(live):
    xs = [x for x, y in live]
    ys = [y for x, y in live]
    minx, maxx, miny, maxy = min(xs), max(xs), min(ys), max(ys)
    for y in range(miny, maxy + 1):
        row = "".join("*" if (x, y) in live else "." for x in range(minx, maxx + 1))
        print(row)


if __name__ == "__main__":
    live = {(0, 1), (1, 1), (2, 1)}
    steps = 2
    for s in range(steps + 1):
        print(f"Step {s}:")
        print_board(live)
        if s < steps:
            live = step(live)
