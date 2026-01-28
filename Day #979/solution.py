# Day 979: Conway's Game of Life on an infinite board: store live cells in a set; each step
# tally neighbor counts only around live cells, then apply the 4 rules.
# Time: O(L) per step (L live cells); Space: O(L). Printing bounds to the live region.
from collections import defaultdict


class GameOfLife:
    def __init__(self, cells):
        self.live = set(map(tuple, cells))

    def step(self):
        counts = defaultdict(int)
        for r, c in self.live:
            for dr in (-1, 0, 1):
                for dc in (-1, 0, 1):
                    if dr or dc:
                        counts[(r + dr, c + dc)] += 1
        nxt = set()
        for cell, cnt in counts.items():
            if cnt == 3 or (cnt == 2 and cell in self.live):
                nxt.add(cell)
        self.live = nxt

    def print_board(self, step_no):
        print(f"Step {step_no}:")
        if not self.live:
            print("(empty)\n")
            return
        rows = [r for r, _ in self.live]
        cols = [c for _, c in self.live]
        min_r, max_r, min_c, max_c = min(rows), max(rows), min(cols), max(cols)
        for r in range(min_r, max_r + 1):
            print("".join("*" if (r, c) in self.live else "." for c in range(min_c, max_c + 1)))
        print()


if __name__ == "__main__":
    g = GameOfLife([(0, 0), (0, 1), (0, 2)])  # horizontal blinker
    steps = 2
    g.print_board(0)
    for s in range(1, steps + 1):
        g.step()
        g.print_board(s)
