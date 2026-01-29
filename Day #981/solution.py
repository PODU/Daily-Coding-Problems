# Day 981: Longest Increasing Subsequence via patience sorting: maintain a "tails" array and
# binary-search the insertion point for each element. Time O(n log n), Space O(n).
from bisect import bisect_left


def length_of_lis(nums):
    tails = []  # tails[i] = smallest tail of an increasing subseq of length i+1
    for x in nums:
        i = bisect_left(tails, x)
        if i == len(tails):
            tails.append(x)
        else:
            tails[i] = x
    return len(tails)


if __name__ == "__main__":
    nums = [10, 9, 2, 5, 3, 7, 101, 18]
    print("Input:", nums)
    print("LIS length:", length_of_lis(nums))  # 4
