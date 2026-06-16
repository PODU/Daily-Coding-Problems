# Day 1671: Min columns to remove so each column is non-decreasing top->bottom.
# Count columns containing any out-of-order adjacent pair. Time O(N*M), Space O(1).


def min_deletions(grid):
    if not grid:
        return 0
    rows, cols, del_count = len(grid), len(grid[0]), 0
    for j in range(cols):
        for i in range(rows - 1):
            if grid[i][j] > grid[i + 1][j]:
                del_count += 1
                break
    return del_count


if __name__ == "__main__":
    print(min_deletions(["cba", "daf", "ghi"]))  # 1
