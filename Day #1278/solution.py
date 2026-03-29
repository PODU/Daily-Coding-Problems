# Day 1278: Sudoku solver via backtracking with row/col/box bitmasks.
# Time: exponential worst-case but fast with constraint pruning. Space O(1).
def solve_sudoku(grid):
    rowm = [0] * 9
    colm = [0] * 9
    boxm = [0] * 9

    def bidx(r, c):
        return (r // 3) * 3 + c // 3

    for r in range(9):
        for c in range(9):
            v = grid[r][c]
            if v:
                bit = 1 << v
                rowm[r] |= bit
                colm[c] |= bit
                boxm[bidx(r, c)] |= bit

    def solve(pos):
        if pos == 81:
            return True
        r, c = divmod(pos, 9)
        if grid[r][c]:
            return solve(pos + 1)
        b = bidx(r, c)
        used = rowm[r] | colm[c] | boxm[b]
        for d in range(1, 10):
            bit = 1 << d
            if used & bit:
                continue
            grid[r][c] = d
            rowm[r] |= bit; colm[c] |= bit; boxm[b] |= bit
            if solve(pos + 1):
                return True
            grid[r][c] = 0
            rowm[r] &= ~bit; colm[c] &= ~bit; boxm[b] &= ~bit
        return False

    solve(0)
    return grid


if __name__ == "__main__":
    puzzle = [
        "53..7....", "6..195...", ".98....6.",
        "8...6...3", "4..8.3..1", "7...2...6",
        ".6....28.", "...419..5", "....8..79"]
    grid = [[0 if ch == '.' else int(ch) for ch in row] for row in puzzle]
    for row in solve_sudoku(grid):
        print("".join(map(str, row)))
