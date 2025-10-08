# Day 393: Largest consecutive range via hash set: from each start (n-1 absent) extend up. O(n) time, O(n) space.

def largest_range(nums):
    s = set(nums)
    best_lo, best_hi, best_len = nums[0], nums[0], 0
    for n in s:
        if n - 1 in s:
            continue
        hi = n
        while hi + 1 in s:
            hi += 1
        if hi - n + 1 > best_len:
            best_len, best_lo, best_hi = hi - n + 1, n, hi
    return (best_lo, best_hi)


if __name__ == "__main__":
    nums = [9, 6, 1, 3, 8, 10, 12, 11]
    print(largest_range(nums))
