# Day 1168: 24 game (fixed order): interval recursion combining left/right reachable
# values with + - * / using exact Fraction arithmetic (no epsilon needed).
# Time: O(n^3 * S^2), Space: O(n^2 * S). Here n=4.
from fractions import Fraction


def solve(a, l, r):
    if l == r:
        return {Fraction(a[l])}
    res = set()
    for m in range(l, r):
        left = solve(a, l, m)
        right = solve(a, m + 1, r)
        for x in left:
            for y in right:
                res.add(x + y)
                res.add(x - y)
                res.add(x * y)
                if y != 0:
                    res.add(x / y)
    return res


def can24(a):
    return Fraction(24) in solve(a, 0, len(a) - 1)


if __name__ == "__main__":
    print(can24([5, 2, 7, 8]))
