# Day 908: Count columns to delete so each remaining column is non-decreasing top->bottom.
# Scan each column for any adjacent out-of-order pair. Time O(N*M), Space O(1).

def min_deletions(grid):
    if not grid:
        return 0
    n, m = len(grid), len(grid[0])
    count = 0
    for c in range(m):
        for i in range(n - 1):
            if grid[i][c] > grid[i + 1][c]:
                count += 1
                break
    return count


if __name__ == "__main__":
    grid = ["cba", "daf", "ghi"]
    print(min_deletions(grid))
