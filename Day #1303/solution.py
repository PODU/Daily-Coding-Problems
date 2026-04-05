# Day 1303: Next larger integer with the same number of set bits (snoob).
# Bit-hack: add lowest set bit, then re-insert the moved bits at the bottom. O(1) time.


def next_same_bits(n: int) -> int:
    c = n & (-n)               # lowest set bit
    r = n + c                  # ripple carry
    ones = ((n ^ r) >> 2) // c # moved bits, shifted down
    return r | ones


if __name__ == "__main__":
    print(next_same_bits(6))  # 9
