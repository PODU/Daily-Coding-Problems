# Day 1353: Count max-heaps: count(n)=C(n-1,L)*count(L)*count(R), L=left subtree size from heap shape.
# Time: O(N^2) (binomial table + recursion), Space: O(N^2).
import math


def left_size(n):
    if n == 1:
        return 0
    h = int(math.floor(math.log2(n)))
    lower = 1 << (h - 1)
    last = n - ((1 << h) - 1)
    left_last = min(last, lower)
    return ((1 << (h - 1)) - 1) + left_last


def count_heaps(n, C):
    if n <= 1:
        return 1
    L = left_size(n)
    R = n - 1 - L
    return C[n - 1][L] * count_heaps(L, C) * count_heaps(R, C)


if __name__ == "__main__":
    N = 3
    integers = [1, 2, 3]
    C = [[0] * (N + 1) for _ in range(N + 1)]
    for i in range(N + 1):
        C[i][0] = 1
        for j in range(1, i + 1):
            C[i][j] = C[i - 1][j - 1] + (C[i - 1][j] if j <= i - 1 else 0)
    print(count_heaps(N, C))
