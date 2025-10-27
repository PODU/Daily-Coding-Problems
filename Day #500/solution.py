# Day 500: BFS shortest path on a boolean grid (False=walkable, True=wall).
# Time O(M*N), Space O(M*N).
from collections import deque


def min_steps(board, start, end):
    M, N = len(board), len(board[0])
    if board[start[0]][start[1]] or board[end[0]][end[1]]:
        return None
    visited = [[False] * N for _ in range(M)]
    q = deque([start])
    visited[start[0]][start[1]] = True
    steps = 0
    while q:
        for _ in range(len(q)):
            r, c = q.popleft()
            if (r, c) == end:
                return steps
            for nr, nc in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
                if 0 <= nr < M and 0 <= nc < N and not visited[nr][nc] and not board[nr][nc]:
                    visited[nr][nc] = True
                    q.append((nr, nc))
        steps += 1
    return None


if __name__ == "__main__":
    t, f = True, False
    board = [
        [f, f, f, f],
        [t, t, f, t],
        [f, f, f, f],
        [f, f, f, f],
    ]
    print(min_steps(board, (3, 0), (0, 0)))
