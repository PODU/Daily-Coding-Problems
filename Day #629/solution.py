# Day 629: Split array into k contiguous parts minimizing the max partition sum.
# Binary search on answer in [max, total]; greedy feasibility check. O(n log(sum)).


def feasible(nums, k, cap):
    parts, cur = 1, 0
    for x in nums:
        if cur + x > cap:
            parts += 1
            cur = x
        else:
            cur += x
    return parts <= k


def split_array(nums, k):
    lo, hi = max(nums), sum(nums)
    while lo < hi:
        mid = (lo + hi) // 2
        if feasible(nums, k, mid):
            hi = mid
        else:
            lo = mid + 1
    return lo


if __name__ == "__main__":
    print(split_array([5, 1, 2, 7, 3, 4], 3))  # expected 8
