# Day 1691: Next bigger integer with the same number of set bits (Gosper's hack). O(1).

def next_same_popcount(n):
    c = n & (-n)               # lowest set bit
    r = n + c                  # ripple the carry
    return r | (((n ^ r) >> 2) // c)  # restore trailing ones

if __name__ == "__main__":
    print(next_same_popcount(6))  # 6 (0110) -> 9 (1001)
