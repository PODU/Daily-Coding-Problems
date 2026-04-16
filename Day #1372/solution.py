# Day 1372: Shortest path on grid with walls via BFS. Time O(M*N), Space O(M*N).
# Returns None if unreachable.
from collections import deque


def shortest_path(grid, start, end):
    m, n = len(grid), len(grid[0])
    if grid[start[0]][start[1]] or grid[end[0]][end[1]]:
        return None
    dist = [[-1] * n for _ in range(m)]
    dist[start[0]][start[1]] = 0
    q = deque([start])
    while q:
        r, c = q.popleft()
        if (r, c) == end:
            return dist[r][c]
        for nr, nc in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
            if 0 <= nr < m and 0 <= nc < n and not grid[nr][nc] and dist[nr][nc] == -1:
                dist[nr][nc] = dist[r][c] + 1
                q.append((nr, nc))
    return None


if __name__ == "__main__":
    f, t = False, True
    board = [
        [f, f, f, f],
        [t, t, f, t],
        [f, f, f, f],
        [f, f, f, f],
    ]
    res = shortest_path(board, (3, 0), (0, 0))
    print(res if res is not None else "null")
