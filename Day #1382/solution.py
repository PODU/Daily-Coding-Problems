# Day 1382: LIS length via patience sorting: maintain tails[], bisect_left for strict increase. O(n log n) time, O(n) space.
import bisect


def length_of_lis(nums):
    tails = []
    for x in nums:
        i = bisect.bisect_left(tails, x)
        if i == len(tails):
            tails.append(x)
        else:
            tails[i] = x
    return len(tails)


if __name__ == "__main__":
    nums = [10, 9, 2, 5, 3, 7, 101, 18]
    print(length_of_lis(nums))
