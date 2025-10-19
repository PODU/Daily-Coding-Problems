# Day 459: Fewest perfect squares summing to N.
# Lagrange/Legendre theorem -> answer in {1,2,3,4}, O(sqrt N).
# Reconstruct one decomposition guided by the count.
from math import isqrt


def is_square(n):
    if n < 0:
        return False
    r = isqrt(n)
    return r * r == n


def min_squares(n):
    if is_square(n):
        return 1
    m = n
    while m % 4 == 0:
        m //= 4
    if m % 8 == 7:
        return 4
    i = 1
    while i * i <= n:
        if is_square(n - i * i):
            return 2
        i += 1
    return 3


def decompose(n):
    k = min_squares(n)
    res = []
    while k > 0:
        if k == 1:
            res.append(n)
            break
        for i in range(isqrt(n), 0, -1):
            if min_squares(n - i * i) == k - 1:
                res.append(i * i)
                n -= i * i
                k -= 1
                break
    return res


if __name__ == "__main__":
    n = 17
    sq = decompose(n)
    print(f"{min_squares(n)} ({' + '.join(map(str, sq))})")  # 2 (16 + 1)
