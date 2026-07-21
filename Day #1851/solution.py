# Day 1851: Zigzag string across k lines.
# Place char i at row=zigzag(i), col=i in a grid; print rows. O(n*k) build. Space O(n*k).

def zigzag(s, k):
    n = len(s)
    grid = [[' '] * n for _ in range(k)]
    period = 1 if k <= 1 else 2 * (k - 1)
    for i, ch in enumerate(s):
        pos = 0 if k <= 1 else i % period
        row = pos if pos < k else period - pos
        grid[row][i] = ch
    return [''.join(r).rstrip() for r in grid]


if __name__ == "__main__":
    for line in zigzag("thisisazigzag", 4):
        print(line)
