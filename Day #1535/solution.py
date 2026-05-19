# Day 1535: Two numbers summing to k via a single-pass hash set.
# Time O(n), Space O(n).
def has_pair_sum(nums, k):
    seen = set()
    for x in nums:
        if k - x in seen:
            return True
        seen.add(x)
    return False


if __name__ == "__main__":
    print(str(has_pair_sum([10, 15, 3, 7], 17)).lower())
