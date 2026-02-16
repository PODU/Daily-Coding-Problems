# Day 1079: XOR all -> a^b; isolate via lowest set bit; partition & XOR each group to recover a,b; O(n) time O(1) space

def find_two(nums):
    xor_all = 0
    for n in nums:
        xor_all ^= n
    bit = xor_all & (-xor_all)          # lowest set bit that differs between a and b
    a, b = 0, 0
    for n in nums:
        if n & bit:
            a ^= n
        else:
            b ^= n
    return min(a, b), max(a, b)

nums = [2, 4, 6, 8, 10, 2, 6, 10]
a, b = find_two(nums)
print(f"{a} and {b}")
