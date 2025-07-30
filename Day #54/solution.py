# Day 54: Sudoku solver via backtracking with bitmask row/col/box constraints.
# Worst case exponential; bitmasks make pruning fast. Space: O(1).


def solve_sudoku(g):
    rows = [0] * 9
    cols = [0] * 9
    boxes = [0] * 9
    for r in range(9):
        for c in range(9):
            if g[r][c]:
                bit = 1 << g[r][c]
                rows[r] |= bit
                cols[c] |= bit
                boxes[(r // 3) * 3 + c // 3] |= bit

    def solve(cell):
        if cell == 81:
            return True
        r, c = divmod(cell, 9)
        b = (r // 3) * 3 + c // 3
        if g[r][c]:
            return solve(cell + 1)
        for d in range(1, 10):
            bit = 1 << d
            if (rows[r] | cols[c] | boxes[b]) & bit:
                continue
            g[r][c] = d
            rows[r] |= bit
            cols[c] |= bit
            boxes[b] |= bit
            if solve(cell + 1):
                return True
            g[r][c] = 0
            rows[r] &= ~bit
            cols[c] &= ~bit
            boxes[b] &= ~bit
        return False

    solve(0)
    return g


if __name__ == "__main__":
    grid = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ]
    for row in solve_sudoku(grid):
        print("".join(map(str, row)))
