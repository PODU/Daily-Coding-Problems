# Day 248: Max of two ints without if/else/branch/ternary/comparison.
# Use sign bit of (a-b) on a 64-bit-masked diff. O(1) time, O(1) space.

def max_of(a, b):
    d = a - b
    sign = (d >> 63) & 1  # 1 if a<b (negative diff), else 0; Python ints are arbitrary-precision
    return a - sign * d


def main():
    print("max(3, 7) =", max_of(3, 7))
    print("max(10, 2) =", max_of(10, 2))


if __name__ == "__main__":
    main()
