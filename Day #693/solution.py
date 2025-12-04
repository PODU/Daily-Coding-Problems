# Day 693: Swap adjacent (even/odd) bits of an unsigned 8-bit integer.
# Approach: one-liner masks. Odd bits 0x55 shift left, even bits 0xAA shift right.
# Time O(1), Space O(1).


def swap_bits(n: int) -> int:
    return ((n & 0xAA) >> 1) | ((n & 0x55) << 1)


if __name__ == "__main__":
    print(format(swap_bits(0b10101010), "08b"))  # 01010101
    print(format(swap_bits(0b11100010), "08b"))  # 11010001
