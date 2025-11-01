# Day 527: Count distinct max-heaps from N distinct integers.
# Recurrence f(n) = C(n-1, L) * f(L) * f(R), L = left-subtree size of a complete
# binary tree with n nodes. Python big ints are exact. Time O(n^2), space O(n).
from math import comb


def left_subtree_size(n: int) -> int:
    """Number of nodes in the left subtree of a complete binary tree with n nodes."""
    if n <= 1:
        return 0
    h = 0
    while (1 << (h + 1)) - 1 <= n:
        h += 1  # h = height (root at height 0)
    last_level_cap = 1 << h
    nodes_above = (1 << h) - 1
    last_level_nodes = n - nodes_above
    left_base = (1 << (h - 1)) - 1
    left_last = min(last_level_nodes, last_level_cap // 2)
    return left_base + left_last


def count_heaps(n: int) -> int:
    if n <= 1:
        return 1
    L = left_subtree_size(n)
    R = n - 1 - L
    return comb(n - 1, L) * count_heaps(L) * count_heaps(R)


if __name__ == "__main__":
    N = 3
    integers = [1, 2, 3]
    print(count_heaps(N))  # expected: 2
