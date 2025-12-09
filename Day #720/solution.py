# Day 720: Sudoku solver via backtracking with bitmasks for rows/cols/boxes,
# always filling the next empty cell. Time exponential worst-case but fast in practice.
import sys


def solve_sudoku(grid):
    rows = [0] * 9
    cols = [0] * 9
    boxes = [0] * 9

    def bidx(r, c):
        return (r // 3) * 3 + c // 3

    for r in range(9):
        for c in range(9):
            if grid[r][c]:
                bit = 1 << grid[r][c]
                rows[r] |= bit
                cols[c] |= bit
                boxes[bidx(r, c)] |= bit

    def solve(pos):
        if pos == 81:
            return True
        r, c = divmod(pos, 9)
        if grid[r][c]:
            return solve(pos + 1)
        b = bidx(r, c)
        for d in range(1, 10):
            bit = 1 << d
            if (rows[r] | cols[c] | boxes[b]) & bit:
                continue
            grid[r][c] = d
            rows[r] |= bit
            cols[c] |= bit
            boxes[b] |= bit
            if solve(pos + 1):
                return True
            grid[r][c] = 0
            rows[r] &= ~bit
            cols[c] &= ~bit
            boxes[b] &= ~bit
        return False

    solve(0)
    return grid


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    puzzle = [
        "530070000", "600195000", "098000060",
        "800060003", "400803001", "700020006",
        "060000280", "000419005", "000080079"]
    grid = [[int(ch) for ch in row] for row in puzzle]
    solve_sudoku(grid)
    for row in grid:
        print("".join(map(str, row)))
