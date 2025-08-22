# Day 151: Flood fill via BFS. Replace target pixel's connected same-colored
# region with new color. Time O(R*C), Space O(R*C).
from collections import deque


def flood_fill(m, r, c, color):
    R, C = len(m), len(m[0])
    target = m[r][c]
    if target == color:
        return m
    q = deque([(r, c)])
    m[r][c] = color
    while q:
        x, y = q.popleft()
        for nx, ny in ((x-1, y), (x+1, y), (x, y-1), (x, y+1)):
            if 0 <= nx < R and 0 <= ny < C and m[nx][ny] == target:
                m[nx][ny] = color
                q.append((nx, ny))
    return m


if __name__ == "__main__":
    matrix = [
        ['B', 'B', 'W'],
        ['W', 'W', 'W'],
        ['W', 'W', 'W'],
        ['B', 'B', 'B'],
    ]
    flood_fill(matrix, 2, 2, 'G')
    for row in matrix:
        print(' '.join(row))
