# Day 969: Count islands (4-connected groups of 1s) in a binary matrix.
# Approach: DFS flood fill from each unvisited land cell. Time O(R*C), Space O(R*C).
import sys


def num_islands(grid):
    sys.setrecursionlimit(100000)
    g = [row[:] for row in grid]
    rows, cols = len(g), len(g[0])

    def dfs(r, c):
        if r < 0 or r >= rows or c < 0 or c >= cols or g[r][c] == 0:
            return
        g[r][c] = 0
        dfs(r + 1, c); dfs(r - 1, c); dfs(r, c + 1); dfs(r, c - 1)

    count = 0
    for i in range(rows):
        for j in range(cols):
            if g[i][j] == 1:
                count += 1
                dfs(i, j)
    return count


if __name__ == '__main__':
    grid = [
        [1, 0, 0, 0, 0],
        [0, 0, 1, 1, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [1, 1, 0, 0, 1],
        [1, 1, 0, 0, 1],
    ]
    print(num_islands(grid))  # 4
