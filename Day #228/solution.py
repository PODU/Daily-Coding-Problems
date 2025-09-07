# Day 228: Largest number: sort by comparator a+b > b+a (concatenation), then join.
# Time: O(n log n * L), Space: O(n * L).
from functools import cmp_to_key


def largest_number(nums):
    s = list(map(str, nums))
    s.sort(key=cmp_to_key(lambda a, b: (a + b < b + a) - (a + b > b + a)))
    if s[0] == "0":
        return "0"
    return "".join(s)


if __name__ == "__main__":
    print(largest_number([10, 7, 76, 415]))  # 77641510
