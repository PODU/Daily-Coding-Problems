# Day 560: Two-sum existence: one pass with a hash set of seen numbers.
# Time: O(N), Space: O(N).


def has_pair_with_sum(nums, k):
    seen = set()
    for x in nums:
        if (k - x) in seen:
            return True
        seen.add(x)
    return False


def main():
    nums = [10, 15, 3, 7]
    k = 17
    print("true" if has_pair_with_sum(nums, k) else "false")


if __name__ == "__main__":
    main()
