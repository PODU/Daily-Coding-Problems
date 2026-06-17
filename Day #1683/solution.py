# Day 1683: Find word reading left-to-right (a row) or top-to-bottom (a column).
# Build row/column strings, substring search. Time O(N*M), Space O(N+M).


def find_word(grid, word):
    rows, cols = len(grid), len(grid[0])
    for row in grid:
        if word in "".join(row):
            return True
    for c in range(cols):
        col = "".join(grid[r][c] for r in range(rows))
        if word in col:
            return True
    return False


if __name__ == "__main__":
    grid = [['F', 'A', 'C', 'I'],
            ['O', 'B', 'Q', 'P'],
            ['A', 'N', 'O', 'B'],
            ['M', 'A', 'S', 'S']]
    print("'FOAM' ->", str(find_word(grid, "FOAM")).lower())  # true
    print("'MASS' ->", str(find_word(grid, "MASS")).lower())  # true
