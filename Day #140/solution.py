# Day 140: XOR all -> a^b; isolate a differing bit; partition into two groups and XOR each.
# Time O(n); Space O(1).


def two_singles(nums):
    x = 0
    for v in nums:
        x ^= v
    bit = x & (-x)  # lowest set bit where the two singles differ
    a = b = 0
    for v in nums:
        if v & bit:
            a ^= v
        else:
            b ^= v
    return tuple(sorted((a, b)))


if __name__ == "__main__":
    a, b = two_singles([2, 4, 6, 8, 10, 2, 6, 10])
    print("{} and {}".format(a, b))  # 4 and 8
