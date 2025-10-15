# Day 433: Next larger integer with the same number of set bits (Gosper's hack).
# c = n & -n (lowest set bit); r = n + c; next = (((r ^ n) >> 2) // c) | r. O(1) time/space.

def next_same_bits(n):
    if n <= 0:
        return n
    c = n & -n
    r = n + c
    return (((r ^ n) >> 2) // c) | r


def main():
    n = 6
    m = next_same_bits(n)
    print("Input: %d (%s in binary)" % (n, format(n, 'b')))
    print("Next: %d (%s in binary)" % (m, format(m, 'b')))


if __name__ == "__main__":
    main()
