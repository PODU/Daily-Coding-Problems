# Day 931: GCD of n numbers by folding pairwise gcd.
# Time: O(n * log(maxVal)), Space: O(1).
from math import gcd
from functools import reduce


def gcd_list(nums):
    return reduce(gcd, nums, 0)


if __name__ == "__main__":
    print(gcd_list([42, 56, 14]))  # 14
