# Day 1620: Sudoku solver: backtracking with row/col/box bitmasks; pick first empty cell.
# Time: exponential worst case, fast in practice. Space: O(1) extra (fixed 9x9).

rows = [0] * 9
cols = [0] * 9
boxes = [0] * 9
grid = [['.'] * 9 for _ in range(9)]


def box_index(r, c):
    return (r // 3) * 3 + c // 3


def solve(pos):
    if pos == 81:
        return True
    r, c = divmod(pos, 9)
    if grid[r][c] not in ('0', '.'):
        return solve(pos + 1)
    b = box_index(r, c)
    for d in range(1, 10):
        bit = 1 << d
        if rows[r] & bit or cols[c] & bit or boxes[b] & bit:
            continue
        rows[r] |= bit
        cols[c] |= bit
        boxes[b] |= bit
        grid[r][c] = chr(ord('0') + d)
        if solve(pos + 1):
            return True
        rows[r] &= ~bit
        cols[c] &= ~bit
        boxes[b] &= ~bit
        grid[r][c] = '.'
    return False


def main():
    puzzle = [
        "53..7....",
        "6..195...",
        ".98....6.",
        "8...6...3",
        "4..8.3..1",
        "7...2...6",
        ".6....28.",
        "...419..5",
        "....8..79",
    ]
    for r in range(9):
        for c in range(9):
            ch = puzzle[r][c]
            grid[r][c] = ch
            if ch not in ('.', '0'):
                bit = 1 << (ord(ch) - ord('0'))
                rows[r] |= bit
                cols[c] |= bit
                boxes[box_index(r, c)] |= bit
    solve(0)
    for r in range(9):
        print(''.join(grid[r]))


if __name__ == "__main__":
    main()
