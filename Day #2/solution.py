# Day 2: Product of array except self via prefix and suffix passes (no division).
# Time: O(n), Space: O(1) extra (excluding output).
def product_except_self(nums):
    n = len(nums)
    res = [1] * n
    pre = 1
    for i in range(n):
        res[i] = pre
        pre *= nums[i]
    suf = 1
    for i in range(n - 1, -1, -1):
        res[i] *= suf
        suf *= nums[i]
    return res


if __name__ == "__main__":
    print(product_except_self([1, 2, 3, 4, 5]))
