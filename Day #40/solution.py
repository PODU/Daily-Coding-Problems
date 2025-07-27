# Day 40: Single number where others appear 3x: ones/twos bit-counting state machine.
# O(N) time, O(1) space. Mask to 32 bits and reinterpret to handle negatives.
def single_number(nums):
    ones = twos = 0
    for x in nums:
        ones = (ones ^ x) & ~twos
        twos = (twos ^ x) & ~ones
    ones &= 0xFFFFFFFF
    if ones >= 0x80000000:
        ones -= 0x100000000
    return ones


if __name__ == "__main__":
    print(single_number([6, 1, 3, 3, 3, 6, 6]))
