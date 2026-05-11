# Day 1508: Product of array except self without division: prefix-product then suffix-product.
# Two passes, output array only. Time O(n), Space O(1) extra (besides output).

def product_except_self(nums):
    n = len(nums)
    res = [1] * n
    for i in range(1, n):
        res[i] = res[i - 1] * nums[i - 1]
    suffix = 1
    for i in range(n - 1, -1, -1):
        res[i] *= suffix
        suffix *= nums[i]
    return res


if __name__ == "__main__":
    nums = [1, 2, 3, 4, 5]
    res = product_except_self(nums)
    print("[" + ", ".join(str(x) for x in res) + "]")
