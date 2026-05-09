# Day 1498: Conway's Game of Life on an infinite board using a set of live
# (row,col) cells; per step tally neighbor counts over live cells & neighbors.
# Time O(L) per step (L = live cells), Space O(L).
from collections import defaultdict


class GameOfLife:
    def __init__(self, cells):
        self.live = set(cells)

    def step(self):
        counts = defaultdict(int)
        for (r, c) in self.live:
            for dr in (-1, 0, 1):
                for dc in (-1, 0, 1):
                    if dr or dc:
                        counts[(r + dr, c + dc)] += 1
        next_live = set()
        for cell, n in counts.items():
            if n == 3 or (n == 2 and cell in self.live):
                next_live.add(cell)
        self.live = next_live

    def render(self):
        if not self.live:
            return "(empty)"
        rows = [r for r, _ in self.live]
        cols = [c for _, c in self.live]
        lines = []
        for r in range(min(rows), max(rows) + 1):
            line = "".join(
                "*" if (r, c) in self.live else "."
                for c in range(min(cols), max(cols) + 1)
            )
            lines.append(line)
        return "\n".join(lines)


if __name__ == "__main__":
    game = GameOfLife([(0, 1), (1, 1), (2, 1)])
    steps = 2
    print("Step 0:")
    print(game.render())
    for s in range(1, steps + 1):
        game.step()
        print(f"Step {s}:")
        print(game.render())
