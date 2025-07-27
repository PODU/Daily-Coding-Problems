# Day 38: N-Queens count via bitmask backtracking (columns + two diagonals). O(N!) worst, O(N) space.
def count_n_queens(n):
    full = (1 << n) - 1

    def solve(cols, diag1, diag2):
        if cols == full:
            return 1
        count = 0
        avail = ~(cols | diag1 | diag2) & full
        while avail:
            bit = avail & (-avail)
            avail -= bit
            count += solve(cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1)
        return count

    return solve(0, 0, 0)


if __name__ == "__main__":
    print(count_n_queens(8))
