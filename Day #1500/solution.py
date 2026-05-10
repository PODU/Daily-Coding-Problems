# Day 1500: Number of islands via iterative DFS flood fill (4-directional).
# Time O(R*C), Space O(R*C).


def num_islands(grid):
    if not grid:
        return 0
    rows, cols = len(grid), len(grid[0])
    grid = [row[:] for row in grid]
    count = 0
    for sr in range(rows):
        for sc in range(cols):
            if grid[sr][sc] == 1:
                count += 1
                stack = [(sr, sc)]
                grid[sr][sc] = 0
                while stack:
                    r, c = stack.pop()
                    for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                        nr, nc = r + dr, c + dc
                        if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] == 1:
                            grid[nr][nc] = 0
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
    print(num_islands(grid))
