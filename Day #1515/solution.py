# Day 1515: Sort number-strings by comparator a+b > b+a (largest concat first), join; handle all-zeros.
# Time: O(n log n * L) comparisons, Space: O(n).
from functools import cmp_to_key


def largest_number(nums):
    s = [str(x) for x in nums]
    s.sort(key=cmp_to_key(lambda a, b: -1 if a + b > b + a else (1 if a + b < b + a else 0)))
    if s[0] == "0":
        return "0"
    return "".join(s)


if __name__ == "__main__":
    print(largest_number([10, 7, 76, 415]))
