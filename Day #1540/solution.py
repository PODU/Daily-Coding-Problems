# Day 1540: Largest product of any three integers.
# Sort once; answer is max of top-3 product and (two smallest * largest).
# Time O(n log n) (or O(n) with selection), Space O(1).
def largest_product_of_three(a):
    a = sorted(a)
    return max(a[-1] * a[-2] * a[-3], a[0] * a[1] * a[-1])


if __name__ == "__main__":
    print(largest_product_of_three([-10, -10, 5, 2]))
