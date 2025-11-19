# Day 626: Largest product of three: track top-3 max and bottom-2 min in one pass.
# Answer = max(max1*max2*max3, min1*min2*max1). O(n) time, O(1) space.

def largest_product_of_three(a):
    max1 = max2 = max3 = float("-inf")
    min1 = min2 = float("inf")
    for x in a:
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
    return int(max(max1 * max2 * max3, min1 * min2 * max1))


if __name__ == "__main__":
    print(largest_product_of_three([-10, -10, 5, 2]))  # 500
