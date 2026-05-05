# Day 1472: Largest product of any three integers.
# Track 3 largest and 2 smallest in one pass; answer is max of two candidates.
# Time O(N), Space O(1).

def max_product_three(nums):
    max1 = max2 = max3 = float("-inf")
    min1 = min2 = float("inf")
    for n in nums:
        if n > max1:
            max1, max2, max3 = n, max1, max2
        elif n > max2:
            max2, max3 = n, max2
        elif n > max3:
            max3 = n
        if n < min1:
            min1, min2 = n, min1
        elif n < min2:
            min2 = n
    return max(max1 * max2 * max3, max1 * min1 * min2)


if __name__ == "__main__":
    print(max_product_three([-10, -10, 5, 2]))  # 500
