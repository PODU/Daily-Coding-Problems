# Day 879: Two numbers summing to k via one-pass hash set. Time O(n), Space O(n).

def two_sum(nums, k):
    seen = set()
    for x in nums:
        if k - x in seen:
            return True
        seen.add(x)
    return False


if __name__ == "__main__":
    nums = [10, 15, 3, 7]
    print(str(two_sum(nums, 17)).lower())
