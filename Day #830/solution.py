# Day 830: Largest number formed by concatenating the given numbers.
# Sort strings by comparator a+b vs b+a (descending). O(N log N) compares of O(L) strings.
from functools import cmp_to_key


def largest_number(nums):
    strs = list(map(str, nums))

    def cmp(a, b):
        if a + b > b + a:
            return -1
        if a + b < b + a:
            return 1
        return 0

    strs.sort(key=cmp_to_key(cmp))
    result = "".join(strs)
    # Edge: all zeros -> "0"
    if result and result[0] == "0":
        return "0"
    return result


def main():
    print(largest_number([10, 7, 76, 415]))  # 77641510


if __name__ == "__main__":
    main()
