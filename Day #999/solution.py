# Day 999: Count occurrences of X in an N x N multiplication table.
# X appears at (i, j) iff i divides X and X/i <= N, for i in 1..N. O(N) time, O(1) space.

def count_x(n, x):
    return sum(1 for i in range(1, n + 1) if x % i == 0 and x // i <= n)


if __name__ == "__main__":
    print(count_x(6, 12))  # 4
