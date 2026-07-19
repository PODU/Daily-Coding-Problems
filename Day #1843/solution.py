# Day 1843: Max of two numbers with no branching/comparison; uses sign bit of the difference.
# Python ints are unbounded, so derive the sign via an arithmetic shift on 64-bit. O(1).

MASK = (1 << 64) - 1


def max_no_branch(a, b):
    d = (a - b) & MASK            # work in 64-bit two's complement
    sign = (d >> 63) & 1          # 1 if a < b, else 0
    # if sign==0 -> a, if sign==1 -> b, chosen arithmetically
    return a * (1 - sign) + b * sign


def main():
    print(max_no_branch(5, 9))    # 9
    print(max_no_branch(12, 4))   # 12
    print(max_no_branch(-3, -7))  # -3


if __name__ == "__main__":
    main()
