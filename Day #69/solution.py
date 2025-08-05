# Day 69: Largest product of three: max(max1*max2*max3, min1*min2*max1) tracking top-3 & bottom-2. Time O(n), Space O(1).
def largest_triple_product(nums):
    max1 = max2 = max3 = float("-inf")
    min1 = min2 = float("inf")
    for x in nums:
        if x > max1:
            max1, max2, max3 = x, max1, max2
        elif x > max2:
            max2, max3 = x, max2
        elif x > max3:
            max3 = x
        if x < min1:
            min1, min2 = x, min1
        elif x < min2:
            min2 = x
    return int(max(max1 * max2 * max3, min1 * min2 * max1))


if __name__ == "__main__":
    print(largest_triple_product([-10, -10, 5, 2]))
