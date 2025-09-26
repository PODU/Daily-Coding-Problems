# Day 336: Count distinct max-heaps from N distinct values: ways(n)=C(n-1,L)*ways(L)*ways(R).
# L = size of left subtree of a complete binary tree with n nodes. Time: O(n^2). Space: O(n^2).
import math


def left_subtree_size(n):
    h = int(math.floor(math.log2(n)))
    m = n - ((1 << h) - 1)
    last_cap = 1 << (h - 1)
    return ((1 << (h - 1)) - 1) + min(m, last_cap)


def count_heaps(N):
    C = [[0] * (N + 1) for _ in range(N + 1)]
    for i in range(N + 1):
        C[i][0] = 1
        for j in range(1, i + 1):
            C[i][j] = C[i-1][j-1] + C[i-1][j]
    ways = [0] * (N + 1)
    ways[0] = 1
    if N >= 1:
        ways[1] = 1
    for n in range(2, N + 1):
        L = left_subtree_size(n)
        R = n - 1 - L
        ways[n] = C[n-1][L] * ways[L] * ways[R]
    return ways[N]


if __name__ == "__main__":
    print(count_heaps(3))
