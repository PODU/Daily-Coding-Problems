# Day 1125: Day 1125 - Contiguous sublist summing to K
# Approach: prefix sums + hash map (handles negatives) to find a range with
# sum == K in one pass. Time: O(n), Space: O(n).

def subarray_sum(nums, k):
    seen = {0: -1}  # prefix sum -> earliest index
    total = 0
    for j, x in enumerate(nums):
        total += x
        if total - k in seen:
            i = seen[total - k]
            return nums[i + 1:j + 1]
        seen.setdefault(total, j)
    return None


if __name__ == "__main__":
    print(subarray_sum([1, 2, 3, 4, 5], 9))  # [2, 3, 4]
