# Day 564: Largest sum of non-adjacent numbers (empty subset allowed -> answer >= 0).
# DP tracking incl/excl running maxes. Time O(n), Space O(1).


def largest_non_adjacent(nums):
    incl, excl = 0, 0  # best sums including / excluding previous element
    for x in nums:
        incl, excl = excl + x, max(incl, excl)
    return max(incl, excl, 0)


if __name__ == "__main__":
    print(largest_non_adjacent([2, 4, 6, 2, 5]))
    print(largest_non_adjacent([5, 1, 1, 5]))
