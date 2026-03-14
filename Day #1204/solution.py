# Day 1204: GCD of n numbers.
# Fold Euclidean gcd across the list. Time O(n log max), Space O(1).
from functools import reduce
from math import gcd


def gcd_all(nums):
    return reduce(gcd, nums, 0)


if __name__ == "__main__":
    print(gcd_all([42, 56, 14]))  # 14
