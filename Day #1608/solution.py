# Day 1608: Count distinct max-heaps from N distinct integers. ways(n)=C(n-1,L)*ways(L)*ways(R),
# L = left-subtree node count of a complete binary tree of n nodes. Time O(n^2), Space O(1) (math.comb).
import math


def left_count(n: int) -> int:
    if n == 1:
        return 0
    h = n.bit_length() - 1               # height (root at level 0)
    last = n - ((1 << h) - 1)            # nodes in the bottom level
    max_last = 1 << (h - 1)              # max bottom-level nodes for left subtree
    return ((1 << (h - 1)) - 1) + min(last, max_last)


def ways(n: int) -> int:
    if n <= 1:
        return 1
    L = left_count(n)
    R = n - 1 - L
    return math.comb(n - 1, L) * ways(L) * ways(R)


if __name__ == "__main__":
    arr = [1, 2, 3]   # N distinct integers
    print(ways(len(arr)))
