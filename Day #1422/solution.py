# Day 1422: largest sum of non-adjacent numbers (values may be 0/negative).
# Approach: rolling include/exclude DP; empty subset allowed (floor 0). O(n) time, O(1) space.


def max_non_adjacent(nums):
    incl, excl = 0, 0
    for n in nums:
        incl, excl = excl + n, max(incl, excl)
    return max(incl, excl)


if __name__ == "__main__":
    print(max_non_adjacent([2, 4, 6, 2, 5]))  # 13
    print(max_non_adjacent([5, 1, 1, 5]))     # 10
