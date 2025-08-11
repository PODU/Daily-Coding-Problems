# Day 102: Contiguous subarray summing to K via prefix sums + hashmap. For each
# prefix p, look for p-K seen earlier. Returns earliest-ending match. O(n) time.
def subarray_sum(nums, k):
    first = {0: -1}  # prefix sum -> earliest index
    prefix = 0
    for j, x in enumerate(nums):
        prefix += x
        if prefix - k in first:
            i = first[prefix - k]
            return nums[i + 1:j + 1]
        first.setdefault(prefix, j)
    return None


if __name__ == "__main__":
    print(subarray_sum([1, 2, 3, 4, 5], 9))  # [2, 3, 4]
