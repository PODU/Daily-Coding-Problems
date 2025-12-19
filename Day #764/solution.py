# Day 764: Arrange numbers to form the largest integer.
# Sort by comparator: a+b vs b+a (descending). Time: O(n log n * L), Space: O(n).
from functools import cmp_to_key


def largest_number(nums):
    s = list(map(str, nums))
    s.sort(key=cmp_to_key(lambda a, b: (a + b < b + a) - (a + b > b + a)))
    if s[0] == "0":
        return "0"
    return "".join(s)


if __name__ == "__main__":
    print(largest_number([10, 7, 76, 415]))   # 77641510
