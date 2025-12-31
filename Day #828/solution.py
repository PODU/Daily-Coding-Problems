# Day 828: Count distinct max heaps from N distinct integers.
# Root is the max; recurrence f(n) = C(n-1, L) * f(L) * f(R), where L = size of
# left subtree from the complete-tree shape. Time O(N^2) (Pascal), Space O(N^2).
def count_max_heaps(n):
    # Precompute binomial coefficients up to n.
    C = [[0] * (n + 1) for _ in range(n + 1)]
    for i in range(n + 1):
        C[i][0] = 1
        for j in range(1, i + 1):
            C[i][j] = C[i - 1][j - 1] + C[i - 1][j]

    def left_count(m):
        # number of nodes in left subtree of a complete tree with m nodes
        if m == 1:
            return 0
        h = (m).bit_length() - 1            # height (levels above 0-indexed)
        last = m - (2 ** h - 1)             # nodes in last level
        left_cap = 2 ** (h - 1)             # last-level capacity of left subtree
        return (2 ** (h - 1) - 1) + min(left_cap, last)

    def f(m):
        if m <= 1:
            return 1
        L = left_count(m)
        R = m - 1 - L
        return C[m - 1][L] * f(L) * f(R)

    return f(n)


if __name__ == "__main__":
    print(count_max_heaps(3))
