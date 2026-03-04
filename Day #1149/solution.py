# Day 1149: Number of islands - 4-directional flood fill.
# Iterative DFS marks visited land. Time O(R*C), Space O(R*C).
def num_islands(grid):
    if not grid:
        return 0
    g = [row[:] for row in grid]
    rows, cols = len(g), len(g[0])
    count = 0
    for r in range(rows):
        for c in range(cols):
            if g[r][c] == 1:
                count += 1
                stack = [(r, c)]
                g[r][c] = 0
                while stack:
                    x, y = stack.pop()
                    for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                        nx, ny = x + dx, y + dy
                        if 0 <= nx < rows and 0 <= ny < cols and g[nx][ny] == 1:
                            g[nx][ny] = 0
                            stack.append((nx, ny))
    return count


if __name__ == "__main__":
    grid = [
        [1, 0, 0, 0, 0], [0, 0, 1, 1, 0], [0, 1, 1, 0, 0],
        [0, 0, 0, 0, 0], [1, 1, 0, 0, 1], [1, 1, 0, 0, 1]]
    print(num_islands(grid))  # 4
