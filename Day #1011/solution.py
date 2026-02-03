# Day 1011: Branchless max: two's-complement 64-bit diff; sign bit (via arithmetic
# right shift) selects the larger value. No if/compare. Time O(1), Space O(1).
def max_no_branch(a, b):
    d = (a - b) & ((1 << 64) - 1)  # a - b in 64-bit two's complement
    s = -(d >> 63)                 # 0 if a>=b, -1 if a<b
    return a - ((a - b) & s)       # a>=b -> a ; a<b -> b


def main():
    print(f"max(3, 7) = {max_no_branch(3, 7)}")
    print(f"max(10, -5) = {max_no_branch(10, -5)}")


if __name__ == "__main__":
    main()
