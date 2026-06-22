# Day 1697: Validate American crossword grid: each white cell in horiz & vert runs of len>=3, connected, 180-deg symmetric.
# Time O(N^2), Space O(N^2).
from collections import deque


def is_valid_crossword(g):
    n = len(g)
    if n == 0:
        return False

    # 180-degree rotational symmetry
    for i in range(n):
        for j in range(n):
            if g[i][j] != g[n - 1 - i][n - 1 - j]:
                return False

    # Run-length checks
    for i in range(n):
        for j in range(n):
            if g[i][j] != 0:
                continue
            l = r = j
            while l - 1 >= 0 and g[i][l - 1] == 0:
                l -= 1
            while r + 1 < n and g[i][r + 1] == 0:
                r += 1
            if r - l + 1 < 3:
                return False
            t = b = i
            while t - 1 >= 0 and g[t - 1][j] == 0:
                t -= 1
            while b + 1 < n and g[b + 1][j] == 0:
                b += 1
            if b - t + 1 < 3:
                return False

    # Connectivity
    cells = [(i, j) for i in range(n) for j in range(n) if g[i][j] == 0]
    if not cells:
        return False
    total = len(cells)
    vis = [[False] * n for _ in range(n)]
    sr, sc = cells[0]
    q = deque([(sr, sc)])
    vis[sr][sc] = True
    seen = 0
    while q:
        r, c = q.popleft()
        seen += 1
        for dr, dc in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < n and 0 <= nc < n and not vis[nr][nc] and g[nr][nc] == 0:
                vis[nr][nc] = True
                q.append((nr, nc))
    return seen == total


def main():
    valid = [[0] * 5 for _ in range(5)]  # all white
    print("true" if is_valid_crossword(valid) else "false")

    invalid = [[0] * 5 for _ in range(5)]
    invalid[0][0] = 1  # breaks symmetry
    print("true" if is_valid_crossword(invalid) else "false")


if __name__ == "__main__":
    main()
