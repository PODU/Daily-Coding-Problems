# Day 76: Minimum columns to remove so every column is sorted top-to-bottom.
# Greedy scan: count columns that are not non-decreasing. Time O(N*M), Space O(1).


def min_columns_to_remove(grid):
    if not grid:
        return 0
    rows, cols = len(grid), len(grid[0])
    remove = 0
    for c in range(cols):
        for r in range(1, rows):
            if grid[r][c] < grid[r - 1][c]:
                remove += 1
                break
    return remove


if __name__ == "__main__":
    grid = ["cba", "daf", "ghi"]
    print(min_columns_to_remove(grid))  # 1
