# Day 1000: Minimum of a rotated sorted array (no duplicates).
# Binary search: compare mid with the right end to decide which half holds the
# minimum. O(log N) time, O(1) space.

def find_min(nums):
    lo, hi = 0, len(nums) - 1
    while lo < hi:
        mid = (lo + hi) // 2
        if nums[mid] > nums[hi]:   # min is to the right of mid
            lo = mid + 1
        else:                      # min is at mid or to the left
            hi = mid
    return nums[lo]


if __name__ == "__main__":
    print(find_min([5, 7, 10, 3, 4]))  # 3
