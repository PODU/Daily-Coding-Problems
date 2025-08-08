# Day 84: Count islands (connected groups of 1s) via iterative DFS flood fill.
# Time O(rows*cols), Space O(rows*cols).
import sys


def num_islands(grid):
    g = [row[:] for row in grid]
    rows, cols = len(g), len(g[0])
    count = 0
    for sr in range(rows):
        for sc in range(cols):
            if g[sr][sc] == 1:
                count += 1
                stack = [(sr, sc)]
                g[sr][sc] = 0
                while stack:
                    r, c = stack.pop()
                    for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                        nr, nc = r + dr, c + dc
                        if 0 <= nr < rows and 0 <= nc < cols and g[nr][nc] == 1:
                            g[nr][nc] = 0
                            stack.append((nr, nc))
    return count


if __name__ == "__main__":
    grid = [
        [1, 0, 0, 0, 0],
        [0, 0, 1, 1, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [1, 1, 0, 0, 1],
        [1, 1, 0, 0, 1],
    ]
    print(num_islands(grid))  # 4
