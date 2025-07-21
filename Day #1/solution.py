# Day 1: Two-sum existence: one pass with a hash set of complements.
# Time: O(n), Space: O(n).
def two_sum(nums, k):
    seen = set()
    for x in nums:
        if k - x in seen:
            return True
        seen.add(x)
    return False


if __name__ == "__main__":
    print(str(two_sum([10, 15, 3, 7], 17)).lower())
