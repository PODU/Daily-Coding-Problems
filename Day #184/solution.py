# Day 184: GCD of n numbers via iterated Euclidean algorithm.
# Time O(n * log(max)), Space O(1).
from functools import reduce
from math import gcd
from typing import List


def gcd_all(nums: List[int]) -> int:
    return reduce(gcd, nums, 0)


if __name__ == "__main__":
    print(gcd_all([42, 56, 14]))
