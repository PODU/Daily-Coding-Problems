# Day 9: Max sum of non-adjacent numbers: track best-including vs best-excluding current.
# Time: O(n), Space: O(1). (Empty pick allowed, so negatives can be skipped.)
def max_non_adjacent(nums):
    incl, excl = 0, 0
    for n in nums:
        incl, excl = excl + n, max(incl, excl)
    return max(incl, excl)


if __name__ == "__main__":
    print(max_non_adjacent([2, 4, 6, 2, 5]))
