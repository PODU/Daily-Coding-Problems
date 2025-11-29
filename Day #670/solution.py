# Day 670: Fewest perfect squares summing to n. Lagrange (answer in {1,2,3,4}) +
# Legendre three-square theorem. Time O(sqrt n), Space O(1).
import math


def is_square(n):
    r = math.isqrt(n)
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
    print(num_squares(13))  # 2
    print(num_squares(27))  # 3
