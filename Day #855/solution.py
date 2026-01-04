# Day 855: count N-Queens solutions via backtracking with bitmasks for column/diagonals.
# O(N!) worst case, O(N) space. Bitmask makes conflict checks O(1).

def count_n_queens(n):
    full = (1 << n) - 1
    count = 0

    def solve(cols, diag1, diag2):
        nonlocal count
        if cols == full:
            count += 1
            return
        avail = full & ~(cols | diag1 | diag2)
        while avail:
            p = avail & (-avail)
            avail -= p
            solve(cols | p, (diag1 | p) << 1, (diag2 | p) >> 1)

    solve(0, 0, 0)
    return count


if __name__ == "__main__":
    for n in range(1, 9):
        print("N={}: {}".format(n, count_n_queens(n)))  # N=8: 92
