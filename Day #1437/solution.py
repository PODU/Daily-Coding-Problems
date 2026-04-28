# Day 1437: Length of longest strictly increasing subsequence.
# Approach: Patience sorting; maintain tails array, binary search via bisect_left.
# Time: O(n log n), Space: O(n).
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
    print(length_of_lis(nums))  # 6
