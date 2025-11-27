# Day 662: GCD of n numbers via repeated Euclidean algorithm.
# Time O(n * log(max)), Space O(1).
from functools import reduce
from math import gcd


def gcd_all(v):
    return reduce(gcd, v, 0)


if __name__ == "__main__":
    print(gcd_all([42, 56, 14]))  # 14
