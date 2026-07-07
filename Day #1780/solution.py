# Day 1780: Swap even/odd bits of 8-bit int: ((n&0xAA)>>1)|((n&0x55)<<1), masked to 8 bits.
# Time: O(1), Space: O(1).
def swap_bits(bin_str):
    n = int(bin_str, 2)
    r = (((n & 0xAA) >> 1) | ((n & 0x55) << 1)) & 0xFF
    return format(r, "08b")


if __name__ == "__main__":
    print(swap_bits("10101010"))
    print(swap_bits("11100010"))
