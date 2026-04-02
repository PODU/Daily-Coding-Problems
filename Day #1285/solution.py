# Day 1285: Print a string in zigzag form across k lines.
# Place char i at column i, row = triangle-wave of i. Time O(n*k) to render, Space O(n*k).
def zigzag(s, k):
    n = len(s)
    if k <= 1:
        print(s)
        return
    period = 2 * (k - 1)
    grid = [[" "] * n for _ in range(k)]
    for i in range(n):
        pos = i % period
        row = pos if pos < k else period - pos
        grid[row][i] = s[i]
    for row in grid:
        print("".join(row).rstrip())


if __name__ == "__main__":
    zigzag("thisisazigzag", 4)
