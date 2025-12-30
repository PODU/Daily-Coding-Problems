# Day 820: First missing positive via cyclic sort: place nums[i] at index nums[i]-1, then scan. O(n) time, O(1) space.

def first_missing_positive(nums):
    nums = nums[:]  # work on a copy; placement is still O(1) extra beyond this
    n = len(nums)
    for i in range(n):
        while 0 < nums[i] <= n and nums[nums[i] - 1] != nums[i]:
            j = nums[i] - 1
            nums[i], nums[j] = nums[j], nums[i]
    for i in range(n):
        if nums[i] != i + 1:
            return i + 1
    return n + 1


if __name__ == "__main__":
    print(first_missing_positive([3, 4, -1, 1]))
    print(first_missing_positive([1, 2, 0]))
