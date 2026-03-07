# Day 1171: Validate an American-style crossword grid.
# Checks rotational symmetry, white-square connectivity (BFS), and that every
# maximal horizontal/vertical white run has length >= 3.
# Time O(N^2), Space O(N^2).  '#' = black, '.' = white.
from collections import deque


def is_valid_crossword(g):
    n = len(g)
    if n == 0 or any(len(r) != n for r in g):
        return False

    # 1. Rotational symmetry.
    for i in range(n):
        for j in range(n):
            if (g[i][j] == '.') != (g[n-1-i][n-1-j] == '.'):
                return False

    # 2 & 3. Horizontal and vertical white runs must be >= 3.
    def runs_ok(get):
        for a in range(n):
            run = 0
            for b in range(n + 1):
                if b < n and get(a, b) == '.':
                    run += 1
                else:
                    if 0 < run < 3:
                        return False
                    run = 0
        return True

    if not runs_ok(lambda i, j: g[i][j]):      # rows
        return False
    if not runs_ok(lambda j, i: g[i][j]):      # columns
        return False

    # 4. Connectivity.
    whites = [(i, j) for i in range(n) for j in range(n) if g[i][j] == '.']
    if not whites:
        return True
    start = whites[0]
    seen = {start}
    q = deque([start])
    while q:
        x, y = q.popleft()
        for nx, ny in ((x+1, y), (x-1, y), (x, y+1), (x, y-1)):
            if 0 <= nx < n and 0 <= ny < n and g[nx][ny] == '.' and (nx, ny) not in seen:
                seen.add((nx, ny))
                q.append((nx, ny))
    return len(seen) == len(whites)


if __name__ == "__main__":
    g1 = [".....", ".....", ".....", ".....", "....."]
    g2 = [".#...", ".....", ".....", ".....", "...#."]
    print("true" if is_valid_crossword(g1) else "false")
    print("true" if is_valid_crossword(g2) else "false")
