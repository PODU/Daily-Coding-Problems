# Day 414: Count N-Queens arrangements via bitmask backtracking.
# Track used columns/diagonals as bitmasks. Time O(N!)-ish, Space O(N).
def count_n_queens(n):
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


if __name__ == "__main__":
    for n in range(1, 9):
        print(f"N={n}: {count_n_queens(n)}")
    print(count_n_queens(8))  # 92
