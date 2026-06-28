# Day 1732: Bitwise AND of range [M,N] = common binary prefix; shift both right until equal, then back. O(log N) time, O(1) space.
def range_bitwise_and(m: int, n: int) -> int:
    shift = 0
    while m < n:
        m >>= 1
        n >>= 1
        shift += 1
    return m << shift


if __name__ == "__main__":
    print(range_bitwise_and(5, 7))
