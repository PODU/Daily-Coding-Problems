# Day 455: Conway's Game of Life on an infinite board.
# Set of live cells; tally neighbours each tick. Time O(live) per step.
from collections import Counter


def step(live):
    cnt = Counter()
    for (r, c) in live:
        for dr in (-1, 0, 1):
            for dc in (-1, 0, 1):
                if dr or dc:
                    cnt[(r + dr, c + dc)] += 1
    nxt = set()
    for cell, n in cnt.items():
        if n == 3 or (n == 2 and cell in live):
            nxt.add(cell)
    return nxt


def print_board(live):
    if not live:
        print("(empty)")
        return
    rs = [r for r, _ in live]
    cs = [c for _, c in live]
    for r in range(min(rs), max(rs) + 1):
        print("".join('*' if (r, c) in live else '.'
                       for c in range(min(cs), max(cs) + 1)))


if __name__ == "__main__":
    live = {(1, 0), (1, 1), (1, 2)}  # horizontal blinker
    steps = 2
    print("Step 0:")
    print_board(live)
    for s in range(1, steps + 1):
        live = step(live)
        print(f"Step {s}:")
        print_board(live)
