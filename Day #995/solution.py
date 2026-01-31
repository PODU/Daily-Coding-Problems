# Day 995: Product of array except self, without division.
# Pass 1 fills each slot with the product of everything to its left; pass 2
# multiplies by a running product of everything to its right. O(n) time, O(1) extra.

def products(nums):
    n = len(nums)
    res = [1] * n
    left = 1
    for i in range(n):
        res[i] = left
        left *= nums[i]
    right = 1
    for i in range(n - 1, -1, -1):
        res[i] *= right
        right *= nums[i]
    return res


if __name__ == "__main__":
    print(products([1, 2, 3, 4, 5]))  # [120, 60, 40, 30, 24]
    print(products([3, 2, 1]))        # [2, 3, 6]
