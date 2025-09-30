# Day 352: Validate crossword: rotational symmetry, all white runs (H & V) length>=3, white cells connected. O(N^2).
from collections import deque


def valid(grid):
    n = len(grid)
    # 1. rotational symmetry
    for i in range(n):
        for j in range(n):
            if (grid[i][j] == '#') != (grid[n-1-i][n-1-j] == '#'):
                return False
    # 2a. horizontal runs >= 3
    for i in range(n):
        run = 0
        for j in range(n + 1):
            if j < n and grid[i][j] == '.':
                run += 1
            else:
                if 0 < run < 3:
                    return False
                run = 0
    # 2b. vertical runs >= 3
    for j in range(n):
        run = 0
        for i in range(n + 1):
            if i < n and grid[i][j] == '.':
                run += 1
            else:
                if 0 < run < 3:
                    return False
                run = 0
    # 3. connectivity of white cells
    whites = [(i, j) for i in range(n) for j in range(n) if grid[i][j] == '.']
    if not whites:
        return True
    seen = {whites[0]}
    q = deque([whites[0]])
    while q:
        i, j = q.popleft()
        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            ni, nj = i + di, j + dj
            if 0 <= ni < n and 0 <= nj < n and grid[ni][nj] == '.' and (ni, nj) not in seen:
                seen.add((ni, nj))
                q.append((ni, nj))
    return len(seen) == len(whites)


def main():
    gridA = ["....." for _ in range(5)]
    gridB = ["#...."] + ["....." for _ in range(4)]  # break symmetry: corner black only
    print(str(valid(gridA)).lower())
    print(str(valid(gridB)).lower())


if __name__ == "__main__":
    main()
