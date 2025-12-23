# Day 785: Largest product of three: one pass tracking 3 largest + 2 smallest.
# answer = max(max1*max2*max3, max1*min1*min2). Time O(n), Space O(1).

def largest_product(nums):
    max1 = max2 = max3 = float("-inf")
    min1 = min2 = float("inf")
    for x in nums:
        if x > max1:
            max3, max2, max1 = max2, max1, x
        elif x > max2:
            max3, max2 = max2, x
        elif x > max3:
            max3 = x
        if x < min1:
            min2, min1 = min1, x
        elif x < min2:
            min2 = x
    return int(max(max1 * max2 * max3, max1 * min1 * min2))


if __name__ == "__main__":
    print(largest_product([-10, -10, 5, 2]))
