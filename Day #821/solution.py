# Day 821: Fixed point in sorted distinct array via binary search (nums[i]-i non-decreasing).
# Time: O(log n), Space: O(1).

def fixed_point(nums):
    lo, hi = 0, len(nums) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if nums[mid] == mid:
            return mid
        elif nums[mid] < mid:
            lo = mid + 1
        else:
            hi = mid - 1
    return False


if __name__ == "__main__":
    print(fixed_point([-6, 0, 2, 40]))
    print(fixed_point([1, 5, 7, 8]))
