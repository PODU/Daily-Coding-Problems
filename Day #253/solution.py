# Day 253: Zigzag print: place each char at advancing column, row bounces 0..k-1..0.
# Build k row buffers, fill columns; rstrip each row. Time O(n*k), Space O(n*k).

def zigzag(s, k):
    n = len(s)
    grid = [[' '] * n for _ in range(k)]
    row, direction = 0, 1
    for col in range(n):
        grid[row][col] = s[col]
        if k > 1:
            if row == 0:
                direction = 1
            elif row == k - 1:
                direction = -1
            row += direction
    return [''.join(r).rstrip() for r in grid]


if __name__ == "__main__":
    for r in zigzag("thisisazigzag", 4):
        print(r)
