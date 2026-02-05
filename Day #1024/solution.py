# Day 1024: Reverse all 32 bits of a 32-bit integer.
# Approach: swap-mask reversal in O(log 32) = O(1) time, O(1) space.


def reverse_bits(n):
    n &= 0xFFFFFFFF
    n = ((n & 0xFFFF0000) >> 16) | ((n & 0x0000FFFF) << 16)
    n = ((n & 0xFF00FF00) >> 8) | ((n & 0x00FF00FF) << 8)
    n = ((n & 0xF0F0F0F0) >> 4) | ((n & 0x0F0F0F0F) << 4)
    n = ((n & 0xCCCCCCCC) >> 2) | ((n & 0x33333333) << 2)
    n = ((n & 0xAAAAAAAA) >> 1) | ((n & 0x55555555) << 1)
    return n & 0xFFFFFFFF


def grouped(n):
    bits = format(n, "032b")
    return " ".join(bits[i:i + 4] for i in range(0, 32, 4))


if __name__ == "__main__":
    x = 0xF0F0F0F0
    print(grouped(reverse_bits(x)))
