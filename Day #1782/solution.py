# Day 1782: Min squared integers summing to n via Lagrange four-square + Legendre three-square theorem.
# O(sqrt(n)) per query (only the i^2+j^2 check loops), O(1) space.
import math


def is_perfect_square(n):
    r = int(math.isqrt(n))
    return r * r == n


def num_squares(n):
    if is_perfect_square(n):
        return 1
    m = n
    while m % 4 == 0:
        m //= 4
    if m % 8 == 7:
        return 4
    i = 1
    while i * i <= n:
        if is_perfect_square(n - i * i):
            return 2
        i += 1
    return 3


def main():
    print(num_squares(13))  # 2
    print(num_squares(27))  # 3


if __name__ == "__main__":
    main()
