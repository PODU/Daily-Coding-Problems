# Day 592: Count islands in a binary matrix.
# Approach: iterative DFS flood-fill over each unvisited land cell (4-directional).
# Time: O(R*C), Space: O(R*C).


def num_islands(grid):
    if not grid:
        return 0
    g = [row[:] for row in grid]
    R, C = len(g), len(g[0])
    count = 0
    for sr in range(R):
        for sc in range(C):
            if g[sr][sc] == 1:
                count += 1
                stack = [(sr, sc)]
                g[sr][sc] = 0
                while stack:
                    r, c = stack.pop()
                    for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                        nr, nc = r + dr, c + dc
                        if 0 <= nr < R and 0 <= nc < C and g[nr][nc] == 1:
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
