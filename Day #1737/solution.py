# Day 1737: XOR all -> a^b; partition by a set bit, XOR each group to recover the two singletons. O(n) time, O(1) space.

def two_unique(nums):
    xor_all = 0
    for x in nums:
        xor_all ^= x
    bit = xor_all & (-xor_all)  # lowest set bit
    a = b = 0
    for x in nums:
        if x & bit:
            a ^= x
        else:
            b ^= x
    return (a, b) if a < b else (b, a)


def main():
    nums = [2, 4, 6, 8, 10, 2, 6, 10]
    a, b = two_unique(nums)
    print(f"{a} and {b}")


if __name__ == "__main__":
    main()
