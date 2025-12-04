# Day 691: Bitwise AND of all integers in [M, N].
# Approach: result is the common binary prefix of M and N. Shift both right until
# equal, then shift back. Time O(log N), Space O(1).


def range_bitwise_and(m: int, n: int) -> int:
    shift = 0
    while m < n:
        m >>= 1
        n >>= 1
        shift += 1
    return m << shift


if __name__ == "__main__":
    print(range_bitwise_and(5, 7))    # 4
    print(range_bitwise_and(12, 15))  # 12
