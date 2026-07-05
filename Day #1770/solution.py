# Day 1770: Flood fill via BFS, 4-directional. Replace connected same-colored region.
# Time: O(rows*cols), Space: O(rows*cols) for the queue in worst case.
from collections import deque


def flood_fill(grid, sr, sc, color):
    R, C = len(grid), len(grid[0])
    target = grid[sr][sc]
    if target == color:
        return grid
    q = deque([(sr, sc)])
    grid[sr][sc] = color
    while q:
        r, c = q.popleft()
        for nr, nc in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
            if 0 <= nr < R and 0 <= nc < C and grid[nr][nc] == target:
                grid[nr][nc] = color
                q.append((nr, nc))
    return grid


if __name__ == "__main__":
    grid = [
        ['B', 'B', 'W'],
        ['W', 'W', 'W'],
        ['W', 'W', 'W'],
        ['B', 'B', 'B'],
    ]
    flood_fill(grid, 2, 2, 'G')
    for row in grid:
        print(' '.join(row))
