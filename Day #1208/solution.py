# Day 1208: Fewest perfect squares summing to N.
# Lagrange four-square + Legendre's three-square theorem. Time O(sqrt N), Space O(1).
from math import isqrt


def is_square(n):
    r = isqrt(n)
    return r * r == n


def num_squares(n):
    if is_square(n):
        return 1
    a = 1
    while a * a <= n:
        if is_square(n - a * a):
            return 2
        a += 1
    m = n
    while m % 4 == 0:
        m //= 4
    if m % 8 == 7:
        return 4
    return 3


if __name__ == "__main__":
    print(num_squares(17))  # 2 (16 + 1)
