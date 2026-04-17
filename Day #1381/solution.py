# Day 1381: Conway's Game of Life on an infinite board using a set of live coordinates.
# Each step only inspects live cells and their neighbours.
# Time: O(L) per step (L = live cells), Space: O(L).

def neighbours(x, y):
    for dx in (-1, 0, 1):
        for dy in (-1, 0, 1):
            if dx or dy:
                yield x + dx, y + dy


def step(live):
    from collections import Counter
    counts = Counter()
    for (x, y) in live:
        for n in neighbours(x, y):
            counts[n] += 1
    new = set()
    for cell, c in counts.items():
        if c == 3 or (c == 2 and cell in live):
            new.add(cell)
    return new


def render(live):
    if not live:
        return "(empty)"
    xs = [c[0] for c in live]
    ys = [c[1] for c in live]
    minx, maxx = min(xs), max(xs)
    miny, maxy = min(ys), max(ys)
    rows = []
    for x in range(minx, maxx + 1):
        rows.append("".join("*" if (x, y) in live else "." for y in range(miny, maxy + 1)))
    return "\n".join(rows)


def run(live_cells, steps):
    live = set(live_cells)
    for i in range(steps + 1):
        print(f"Step {i}:")
        print(render(live))
        print()
        live = step(live)


if __name__ == "__main__":
    # Blinker oscillator: horizontal row -> vertical column -> horizontal row.
    run([(1, 0), (1, 1), (1, 2)], 2)
