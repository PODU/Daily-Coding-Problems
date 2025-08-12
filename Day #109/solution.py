# Day 109: Swap adjacent bit pairs: ((x&0xAA)>>1)|((x&0x55)<<1). O(1).
def swap_bits(x):
    return ((x & 0xAA) >> 1) | ((x & 0x55) << 1)


if __name__ == "__main__":
    print(format(swap_bits(0b10101010), "08b"))  # 01010101
    print(format(swap_bits(0b11100010), "08b"))  # 11010001
