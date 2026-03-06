# Day 1166: Flood fill via BFS from start pixel, recoloring 4-connected same-color region.
# Time: O(rows*cols), Space: O(rows*cols).
from collections import deque


def flood_fill(g, sr, sc, color):
    rows, cols = len(g), len(g[0])
    start = g[sr][sc]
    if start == color:
        return
    q = deque([(sr, sc)])
    g[sr][sc] = color
    while q:
        r, c = q.popleft()
        for dr, dc in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < rows and 0 <= nc < cols and g[nr][nc] == start:
                g[nr][nc] = color
                q.append((nr, nc))


if __name__ == "__main__":
    g = [
        ['B', 'B', 'W'],
        ['W', 'W', 'W'],
        ['W', 'W', 'W'],
        ['B', 'B', 'B'],
    ]
    flood_fill(g, 2, 2, 'G')
    for row in g:
        print(' '.join(row))
