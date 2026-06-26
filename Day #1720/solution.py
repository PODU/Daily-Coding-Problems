# Day 1720: Min perfect squares: Legendre/Lagrange three-square theorem gives the count in
# O(sqrt N); decomposition found greedily largest-square-first. Time O(sqrt N), Space O(1).
import math


def is_square(n):
    r = int(math.isqrt(n))
    return r * r == n


def min_squares_count(n):
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


def find(n, count, out):
    if count == 1:
        if is_square(n):
            out.append(n)
            return True
        return False
    for a in range(int(math.isqrt(n)), 0, -1):
        if a * a > n:
            continue
        if find(n - a * a, count - 1, out):
            out.append(a * a)
            return True
    return False


def demo(n):
    count = min_squares_count(n)
    parts = []
    find(n, count, parts)
    parts.sort(reverse=True)
    print(f"{count} ({' + '.join(str(p) for p in parts)})")


if __name__ == "__main__":
    demo(4)
    demo(17)
    demo(18)
