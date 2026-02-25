# Day 1126: Smallest number of perfect squares summing to N.
# Legendre/Lagrange: 1 if perfect square; 4 if N=4^a(8b+7); 2 if sum of two squares; else 3. O(sqrt(N)) time, O(1) space.
import math


def is_square(n):
    r = int(math.isqrt(n))
    return r * r == n


def num_squares(n):
    if is_square(n):
        return 1
    m = n
    while m % 4 == 0:
        m //= 4
    if m % 8 == 7:
        return 4
    a = 1
    while a * a <= n:
        if is_square(n - a * a):
            return 2
        a += 1
    return 3


if __name__ == "__main__":
    N = 17
    print(num_squares(N))
