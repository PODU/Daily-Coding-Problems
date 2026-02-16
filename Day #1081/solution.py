# Day 1081: Flood fill via BFS from the seed pixel. Time O(R*C), Space O(R*C).
from collections import deque


def flood_fill(img, sr, sc, c):
    src = img[sr][sc]
    if src == c:
        return
    R, C = len(img), len(img[0])
    q = deque([(sr, sc)])
    img[sr][sc] = c
    while q:
        r, co = q.popleft()
        for nr, nc in ((r + 1, co), (r - 1, co), (r, co + 1), (r, co - 1)):
            if 0 <= nr < R and 0 <= nc < C and img[nr][nc] == src:
                img[nr][nc] = c
                q.append((nr, nc))


if __name__ == "__main__":
    img = [['B', 'B', 'W'], ['W', 'W', 'W'], ['W', 'W', 'W'], ['B', 'B', 'B']]
    flood_fill(img, 2, 2, 'G')
    for row in img:
        print(' '.join(row))
