# Day 1439: Find a word in a char grid reading left-to-right or top-to-bottom.
# Approach: build each row and column string, check if target is a substring.
# Time: O(R*C*L) substring scan, Space: O(R+C).


def find_word(grid, target):
    if not grid:
        return False
    rows, cols = len(grid), len(grid[0])
    # rows left-to-right
    for row in grid:
        if target in "".join(row):
            return True
    # columns top-to-bottom
    for c in range(cols):
        col = "".join(grid[r][c] for r in range(rows))
        if target in col:
            return True
    return False


if __name__ == "__main__":
    grid = [
        ['F', 'A', 'C', 'I'],
        ['O', 'B', 'Q', 'P'],
        ['A', 'N', 'O', 'B'],
        ['M', 'A', 'S', 'S'],
    ]
    print(str(find_word(grid, "FOAM")).lower())  # true
    print(str(find_word(grid, "MASS")).lower())  # true
