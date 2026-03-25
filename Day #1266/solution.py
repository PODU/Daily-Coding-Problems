# Day 1266: Arrange numbers to form the largest integer.
# Sort by custom comparator a+b vs b+a (descending). O(n log n) comparisons.
from functools import cmp_to_key
from typing import List


def largest_number(nums: List[int]) -> int:
    s = [str(x) for x in nums]
    s.sort(key=cmp_to_key(lambda a, b: (a + b < b + a) - (a + b > b + a)))
    if not s or s[0] == "0":
        return 0
    return int("".join(s))


if __name__ == "__main__":
    print(largest_number([10, 7, 76, 415]))
