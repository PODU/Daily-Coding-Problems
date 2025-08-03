# Day 63: Search word in matrix rows (L->R) and columns (top->bottom) via substring check.
# Time O(N*M*L), Space O(max(N,M)).
def find_word(grid, word):
    rows = ["".join(r) for r in grid]
    cols = ["".join(col) for col in zip(*grid)]
    return any(word in s for s in rows + cols)


if __name__ == "__main__":
    grid = [
        ['F', 'A', 'C', 'I'],
        ['O', 'B', 'Q', 'P'],
        ['A', 'N', 'O', 'B'],
        ['M', 'A', 'S', 'S'],
    ]
    print(f"'FOAM' -> {str(find_word(grid, 'FOAM')).lower()}")
    print(f"'MASS' -> {str(find_word(grid, 'MASS')).lower()}")
