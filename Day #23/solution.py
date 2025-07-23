# Day 23: Shortest path on a grid with walls using BFS (4-directional).
# Time: O(M*N), Space: O(M*N).
from collections import deque


def shortest_path(board, start, end):
    m, n = len(board), len(board[0])
    visited = [[False] * n for _ in range(m)]
    q = deque([(start[0], start[1], 0)])
    visited[start[0]][start[1]] = True
    while q:
        r, c, d = q.popleft()
        if (r, c) == end:
            return d
        for dr, dc in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < m and 0 <= nc < n and not visited[nr][nc] and not board[nr][nc]:
                visited[nr][nc] = True
                q.append((nr, nc, d + 1))
    return None


if __name__ == "__main__":
    f, t = False, True
    board = [
        [f, f, f, f],
        [t, t, f, t],
        [f, f, f, f],
        [f, f, f, f],
    ]
    print(shortest_path(board, (3, 0), (0, 0)))
