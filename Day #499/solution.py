# Day 499: Validate American-style crossword grid: every white cell in a horizontal AND vertical run
# of length >=3, single connected component of white cells, 180-degree rotational symmetry.
# Time: O(N^2), Space: O(N^2).
from collections import deque


def is_crossword(g):
    n = len(g)
    if n == 0:
        return False

    # Rule 4: rotational symmetry.
    for i in range(n):
        for j in range(n):
            if g[i][j] != g[n-1-i][n-1-j]:
                return False

    # Rules 1 & 2: runs of length >= 3 in both directions.
    for i in range(n):
        for j in range(n):
            if g[i][j] != '.':
                continue
            l = j
            while l > 0 and g[i][l-1] == '.':
                l -= 1
            r = j
            while r < n-1 and g[i][r+1] == '.':
                r += 1
            if r - l + 1 < 3:
                return False
            t = i
            while t > 0 and g[t-1][j] == '.':
                t -= 1
            b = i
            while b < n-1 and g[b+1][j] == '.':
                b += 1
            if b - t + 1 < 3:
                return False

    # Rule 3: connectivity via BFS.
    whites = [(i, j) for i in range(n) for j in range(n) if g[i][j] == '.']
    if not whites:
        return True
    seen = set()
    start = whites[0]
    q = deque([start])
    seen.add(start)
    while q:
        x, y = q.popleft()
        for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0)):
            nx, ny = x + dx, y + dy
            if 0 <= nx < n and 0 <= ny < n and g[nx][ny] == '.' and (nx, ny) not in seen:
                seen.add((nx, ny))
                q.append((nx, ny))
    return len(seen) == len(whites)


def main():
    valid = [".....", ".....", ".....", ".....", "....."]
    invalid = ["..#..", ".....", "#...#", ".....", "..#.."]
    print("true" if is_crossword(valid) else "false")
    print("true" if is_crossword(invalid) else "false")


if __name__ == "__main__":
    main()
