# Day 1216: Min columns to delete so each column is non-decreasing top->bottom.
# Approach: scan each column once, count unsorted columns. Time O(N*M), Space O(1).
def min_deletions(grid):
    if not grid:
        return 0
    rows, cols = len(grid), len(grid[0])
    count = 0
    for c in range(cols):
        for r in range(1, rows):
            if grid[r][c] < grid[r-1][c]:
                count += 1
                break
    return count


if __name__ == "__main__":
    grid = ["cba", "daf", "ghi"]
    print(min_deletions(grid))  # 1
