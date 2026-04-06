# Day 1313: Bitwise AND of all integers in [M, N] = common binary prefix of M and N.
# Shift both right until equal, then shift back. Time O(log N), Space O(1).

def range_and(m, n):
    shift = 0
    while m < n:
        m >>= 1
        n >>= 1
        shift += 1
    return m << shift


if __name__ == "__main__":
    print(range_and(5, 7))  # 4  (5 & 6 & 7)
