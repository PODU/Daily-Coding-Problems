# Day 1086: Sudoku solver via backtracking with bitmask constraints (rows/cols/boxes).
# Worst-case exponential, fast in practice; Space O(1).
def solve_sudoku(grid):
    rows = [0] * 9
    cols = [0] * 9
    boxes = [0] * 9

    def box_id(r, c):
        return (r // 3) * 3 + c // 3

    for r in range(9):
        for c in range(9):
            d = grid[r][c]
            if d:
                bit = 1 << d
                rows[r] |= bit
                cols[c] |= bit
                boxes[box_id(r, c)] |= bit

    def solve(pos):
        if pos == 81:
            return True
        r, c = divmod(pos, 9)
        if grid[r][c] != 0:
            return solve(pos + 1)
        b = box_id(r, c)
        for d in range(1, 10):
            bit = 1 << d
            if not ((rows[r] | cols[c] | boxes[b]) & bit):
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
    puzzle = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9]]
    for row in solve_sudoku(puzzle):
        print(''.join(map(str, row)))
