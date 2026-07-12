# Day 1798: Grid shortest path via BFS over walkable cells. Time O(M*N), Space O(M*N).
# Walls are booleans (True=wall). Returns None if no path.
from collections import deque


def shortest_path(grid, start, end):
    m, n = len(grid), len(grid[0])
    if grid[start[0]][start[1]] or grid[end[0]][end[1]]:
        return None
    visited = [[False] * n for _ in range(m)]
    q = deque([start])
    visited[start[0]][start[1]] = True
    steps = 0
    dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    while q:
        for _ in range(len(q)):
            r, c = q.popleft()
            if (r, c) == end:
                return steps
            for dr, dc in dirs:
                nr, nc = r + dr, c + dc
                if 0 <= nr < m and 0 <= nc < n and not visited[nr][nc] and not grid[nr][nc]:
                    visited[nr][nc] = True
                    q.append((nr, nc))
        steps += 1
    return None


if __name__ == "__main__":
    F, T = False, True
    grid = [
        [F, F, F, F],
        [T, T, F, T],
        [F, F, F, F],
        [F, F, F, F],
    ]
    print(shortest_path(grid, (3, 0), (0, 0)))  # 7
