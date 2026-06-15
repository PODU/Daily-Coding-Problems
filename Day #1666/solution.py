# Day 1666: GCD of n numbers.
# Approach: fold Euclid's algorithm across the list. Time O(n*log(max)), Space O(1).
from math import gcd
from functools import reduce


def gcd_list(nums):
    return reduce(gcd, (abs(x) for x in nums), 0)


if __name__ == "__main__":
    print(gcd_list([42, 56, 14]))  # 14
