# Day 1061: Shortest path on a boolean grid (True=wall) via BFS in 4 directions.
# Time: O(M*N), Space: O(M*N). Returns None if unreachable.
from collections import deque


def shortest_path(board, start, end):
    m, n = len(board), len(board[0])
    if board[start[0]][start[1]] or board[end[0]][end[1]]:
        return None
    seen = [[False] * n for _ in range(m)]
    q = deque([(start[0], start[1], 0)])
    seen[start[0]][start[1]] = True
    while q:
        r, c, steps = q.popleft()
        if (r, c) == end:
            return steps
        for dr, dc in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < m and 0 <= nc < n and not seen[nr][nc] and not board[nr][nc]:
                seen[nr][nc] = True
                q.append((nr, nc, steps + 1))
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
