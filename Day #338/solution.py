# Day 338: Next higher integer with same number of set bits (snoob bit-twiddle).
# O(1) time, O(1) space.


def next_same_bits(n):
    smallest = n & (-n)
    ripple = n + smallest
    ones = n ^ ripple
    ones = (ones >> 2) // smallest
    return ripple | ones


def main():
    print(next_same_bits(6))  # 9


if __name__ == "__main__":
    main()
