# Day 1696: N-Queens count via backtracking with bitmasks (columns + two diagonals).
# Time O(N!) worst case (heavily pruned), Space O(N) recursion.

def total_n_queens(n):
    def solve(row, cols, diag1, diag2):
        if row == n:
            return 1
        count = 0
        avail = ((1 << n) - 1) & ~(cols | diag1 | diag2)
        while avail:
            bit = avail & (-avail)
            avail -= bit
            count += solve(row + 1, cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1)
        return count

    return solve(0, 0, 0, 0)


def main():
    print(total_n_queens(8))  # 92


if __name__ == "__main__":
    main()
