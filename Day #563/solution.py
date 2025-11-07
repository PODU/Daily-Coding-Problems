# Day 563: Max of two numbers without if-else/branch/comparison via bit manipulation.
# Emulate 64-bit two's complement; mask = sign-bit replicated; max = a - (d & mask). O(1)/O(1).
def to_signed(x, bits=64):
    x &= (1 << bits) - 1
    return x - (1 << bits) * (x >> (bits - 1))  # subtract 2^bits if sign bit set, no branch


def max_no_branch(a, b, bits=64):
    mask_all = (1 << bits) - 1
    d = (a - b) & mask_all                  # 64-bit two's complement of a-b
    sign = (d >> (bits - 1)) & 1            # 1 if a<b else 0
    mask = sign * mask_all                  # all ones when a<b else zero (no branch)
    return to_signed((a - (d & mask)) & mask_all, bits)


if __name__ == "__main__":
    print(max_no_branch(3, 7))
    print(max_no_branch(10, -4))
