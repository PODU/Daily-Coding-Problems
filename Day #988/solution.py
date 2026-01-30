# Day 988: Minimum number of perfect squares summing to n.
# Number theory (Lagrange + Legendre's three-square theorem). O(sqrt n) time, O(1) space.
import math


def is_square(x):
    r = int(math.isqrt(x))
    return r * r == x


def num_squares(n):
    if is_square(n):
        return 1
    m = n
    while m % 4 == 0:        # strip factors of 4
        m //= 4
    if m % 8 == 7:           # Legendre: 4^a(8b+7) needs 4
        return 4
    a = 1
    while a * a <= n:
        if is_square(n - a * a):
            return 2
        a += 1
    return 3


if __name__ == "__main__":
    print("13 ->", num_squares(13))  # expected 2
    print("27 ->", num_squares(27))  # expected 3
