# Day 656: Flood fill via BFS from start pixel; recolor only cells equal to original color.
# Guard against infinite loop when new color == original. Time O(rows*cols), space O(rows*cols).
from collections import deque


def flood_fill(img, sr, sc, color):
    rows, cols = len(img), len(img[0])
    orig = img[sr][sc]
    if orig == color:
        return img
    q = deque([(sr, sc)])
    img[sr][sc] = color
    while q:
        r, c = q.popleft()
        for nr, nc in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
            if 0 <= nr < rows and 0 <= nc < cols and img[nr][nc] == orig:
                img[nr][nc] = color
                q.append((nr, nc))
    return img


def main():
    img = [
        ['B', 'B', 'W'],
        ['W', 'W', 'W'],
        ['W', 'W', 'W'],
        ['B', 'B', 'B'],
    ]
    flood_fill(img, 2, 2, 'G')
    for row in img:
        print(' '.join(row))


if __name__ == '__main__':
    main()
