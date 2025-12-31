# Day 825: Sorted squares via two-pointer merge from both ends, filling result from the back.
# Time: O(n), Space: O(n).

def sorted_squares(nums):
    n = len(nums)
    res = [0] * n
    lo, hi = 0, n - 1
    for k in range(n - 1, -1, -1):
        l = nums[lo] * nums[lo]
        r = nums[hi] * nums[hi]
        if l > r:
            res[k] = l
            lo += 1
        else:
            res[k] = r
            hi -= 1
    return res


if __name__ == "__main__":
    print(sorted_squares([-9, -2, 0, 2, 3]))
