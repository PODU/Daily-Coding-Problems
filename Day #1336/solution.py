# Day 1336: Count distinct N-Queens arrangements.
# Backtracking with column/diagonal bitmasks. Time: O(N!) worst, Space: O(N).


def total_n_queens(n: int) -> int:
    def count(row: int, cols: int, d1: int, d2: int) -> int:
        if row == n:
            return 1
        total = 0
        avail = ((1 << n) - 1) & ~(cols | d1 | d2)
        while avail:
            bit = avail & (-avail)
            avail -= bit
            total += count(row + 1, cols | bit, (d1 | bit) << 1, (d2 | bit) >> 1)
        return total

    return count(0, 0, 0, 0)


if __name__ == "__main__":
    print("N=1 ->", total_n_queens(1))
    print("N=4 ->", total_n_queens(4))
    print("N=8 ->", total_n_queens(8))
