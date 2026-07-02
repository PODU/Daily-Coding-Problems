# Day 1753: Egyptian fraction (sum of distinct unit fractions).
# Greedy Fibonacci-Sylvester: repeatedly subtract 1/ceil(b/a). Time O(terms), O(1) space.
from math import gcd


def egyptian(a, b):
    terms = []
    while a != 0:
        c = -(-b // a)  # ceil(b / a)
        terms.append("1 / {}".format(c))
        a = a * c - b
        b = b * c
        g = gcd(abs(a), b)
        if g > 1:
            a //= g
            b //= g
    return " + ".join(terms)


if __name__ == "__main__":
    # README example: 4 / 13 -> 1 / 4 + 1 / 18 + 1 / 468
    print(egyptian(4, 13))
