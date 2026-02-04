# Day 1021: Swap even and odd bits of an 8-bit integer.
# Approach: ((n & 0xAA) >> 1) | ((n & 0x55) << 1).  Time O(1), Space O(1).


def swap_bits(n):
    return (((n & 0xAA) >> 1) | ((n & 0x55) << 1)) & 0xFF


if __name__ == "__main__":
    for b in ["10101010", "11100010"]:
        print(format(swap_bits(int(b, 2)), "08b"))
