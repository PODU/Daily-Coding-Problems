# Day 553: Count columns that are NOT non-decreasing top-to-bottom; that's the min to remove.
# O(N*M) time, O(1) extra space.


def min_deletions(grid):
    if not grid:
        return 0
    rows, cols, count = len(grid), len(grid[0]), 0
    for c in range(cols):
        for r in range(1, rows):
            if grid[r][c] < grid[r - 1][c]:
                count += 1
                break
    return count


def main():
    print(min_deletions(["cba", "daf", "ghi"]))  # 1
    print(min_deletions(["abcdef"]))             # 0
    print(min_deletions(["zyx", "wvu", "tsr"]))  # 3


if __name__ == "__main__":
    main()
