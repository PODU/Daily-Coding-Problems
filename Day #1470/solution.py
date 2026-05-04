# Day 1470: Least squares summing to n via Legendre/Lagrange: 1 if square, 4 if 4^a(8b+7),
# 2 if i^2+j^2, else 3. Time: O(sqrt(n)); Space: O(1).
import math


def is_square(x):
    r = int(math.isqrt(x))
    return r * r == x


def num_squares(n):
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


if __name__ == "__main__":
    print(num_squares(13))
    print(num_squares(27))
