# Day 1421: product of all elements except self, without division.
# Approach: prefix products pass then suffix products pass. O(n) time, O(1) extra (besides output).


def product_except_self(nums):
    n = len(nums)
    res = [1] * n
    prefix = 1
    for i in range(n):
        res[i] = prefix
        prefix *= nums[i]
    suffix = 1
    for i in range(n - 1, -1, -1):
        res[i] *= suffix
        suffix *= nums[i]
    return res


if __name__ == "__main__":
    print(product_except_self([1, 2, 3, 4, 5]))  # [120, 60, 40, 30, 24]
    print(product_except_self([3, 2, 1]))        # [2, 3, 6]
