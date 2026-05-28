# Day 1577: Smallest sparse number (no adjacent 1 bits) >= N.
# Repeatedly fix the lowest pair of adjacent ones by clearing low bits and carrying.
# Time: O((log N)^2); Space: O(1).


def smallest_sparse(n):
    while n & (n >> 1):
        i = 0
        while not ((n >> i) & 1 and (n >> (i + 1)) & 1):
            i += 1
        mask = (1 << (i + 2)) - 1
        n = (n & ~mask) + (1 << (i + 2))
    return n


if __name__ == "__main__":
    print(smallest_sparse(21))  # 21
