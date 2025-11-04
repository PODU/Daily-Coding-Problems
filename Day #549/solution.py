# Day 549: Single number among triples: sum each bit position mod 3 to rebuild the unique value. O(N) time, O(1) space.

def single_number(nums):
    result = 0
    for b in range(32):
        cnt = sum((x >> b) & 1 for x in nums)
        if cnt % 3:
            result |= (1 << b)
    return result


if __name__ == "__main__":
    print(single_number([6, 1, 3, 3, 3, 6, 6]))
    print(single_number([13, 19, 13, 13]))
