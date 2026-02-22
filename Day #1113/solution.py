# Day 1113: Day 1113 - Validate an American-style crossword grid ('#' black, '.' white)
# Approach: check 180-degree symmetry, that every white cell lies in a
# horizontal AND vertical run of length >= 3, and white-square connectivity.
# Time: O(N^2), Space: O(N^2).

def is_crossword(grid):
    n = len(grid)
    if n == 0 or any(len(r) != n for r in grid):
        return False

    # Rule 4: rotational (180) symmetry
    for i in range(n):
        for j in range(n):
            if (grid[i][j] == '#') != (grid[n - 1 - i][n - 1 - j] == '#'):
                return False

    def run_len(cells):
        # cells: list of bools (True=white); ensure no maximal white run < 3
        best = []
        for i in range(len(cells)):
            if not cells[i]:
                continue
            length = 1
            l = i - 1
            while l >= 0 and cells[l]:
                length += 1; l -= 1
            r = i + 1
            while r < len(cells) and cells[r]:
                length += 1; r += 1
            best.append(length)
        return best

    # Rules 1 & 2: every white square in across (>=3) and down (>=3) word
    for i in range(n):
        row = [grid[i][j] == '.' for j in range(n)]
        for j in range(n):
            if grid[i][j] == '.':
                # horizontal run containing (i,j)
                length = 1
                k = j - 1
                while k >= 0 and grid[i][k] == '.':
                    length += 1; k -= 1
                k = j + 1
                while k < n and grid[i][k] == '.':
                    length += 1; k += 1
                if length < 3:
                    return False
                # vertical run containing (i,j)
                length = 1
                k = i - 1
                while k >= 0 and grid[k][j] == '.':
                    length += 1; k -= 1
                k = i + 1
                while k < n and grid[k][j] == '.':
                    length += 1; k += 1
                if length < 3:
                    return False

    # Rule 3: all white squares connected
    whites = [(i, j) for i in range(n) for j in range(n) if grid[i][j] == '.']
    if not whites:
        return True
    seen = set([whites[0]])
    stack = [whites[0]]
    while stack:
        i, j = stack.pop()
        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            ni, nj = i + di, j + dj
            if 0 <= ni < n and 0 <= nj < n and grid[ni][nj] == '.' and (ni, nj) not in seen:
                seen.add((ni, nj)); stack.append((ni, nj))
    return len(seen) == len(whites)


if __name__ == "__main__":
    valid = [
        "...##",
        ".....",
        ".....",
        ".....",
        "##...",
    ]
    invalid = [
        ".....",
        ".....",
        ".....",
        ".....",
        "....#",
    ]
    print("Grid 1 valid:", is_crossword(valid))    # True
    print("Grid 2 valid:", is_crossword(invalid))  # False
