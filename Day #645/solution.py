# Day 645: Find a word in a grid going left-to-right or top-to-bottom.
# Approach: scan every row and every column for the target as a substring start.
# Time: O(R*C*L), Space: O(1).
def find_word(grid, word):
    R, C, L = len(grid), len(grid[0]), len(word)
    # horizontal
    for r in range(R):
        for c in range(C - L + 1):
            if all(grid[r][c + k] == word[k] for k in range(L)):
                return True
    # vertical
    for c in range(C):
        for r in range(R - L + 1):
            if all(grid[r + k][c] == word[k] for k in range(L)):
                return True
    return False


if __name__ == "__main__":
    grid = [['F', 'A', 'C', 'I'],
            ['O', 'B', 'Q', 'P'],
            ['A', 'N', 'O', 'B'],
            ['M', 'A', 'S', 'S']]
    print(str(find_word(grid, "FOAM")).lower())  # true
    print(str(find_word(grid, "MASS")).lower())  # true
