# Day 521: Zigzag: place char i at (zigzag-row, column i) in a k x n grid; print rows. O(n*k).
def zigzag(s, k):
    n = len(s)
    grid = [[' '] * n for _ in range(k)]
    row, direction = 0, (0 if k == 1 else 1)
    for i, ch in enumerate(s):
        grid[row][i] = ch
        if row == 0:
            direction = 1
        elif row == k - 1:
            direction = -1
        row += direction
    for r in grid:
        print(''.join(r).rstrip())


if __name__ == "__main__":
    zigzag("thisisazigzag", 4)
