# Day 783: Longest Increasing Subsequence (length), patience sorting.
# Maintain tails[]; bisect_left finds insertion point for each value. O(n log n) time, O(n) space.
from bisect import bisect_left


def length_of_lis(nums):
    tails = []
    for x in nums:
        i = bisect_left(tails, x)
        if i == len(tails):
            tails.append(x)
        else:
            tails[i] = x
    return len(tails)


if __name__ == "__main__":
    nums = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15]
    print(length_of_lis(nums))
