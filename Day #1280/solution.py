# Day 1280: Swap adjacent bit pairs of an 8-bit unsigned integer.
# One-liner: shift odd bits up, even bits down. Time O(1), Space O(1).
def swap_bits(n):
    return ((n & 0xAA) >> 1) | ((n & 0x55) << 1)


if __name__ == "__main__":
    for s in ["10101010", "11100010"]:
        n = int(s, 2)
        print(format(swap_bits(n), "08b"))
