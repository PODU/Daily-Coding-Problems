# Day 787: Next bigger integer with same popcount via bit-hack (Gosper's hack). O(1) time, O(1) space.

def next_same_popcount(n: int) -> int:
    if n == 0:
        return 0
    c = n & -n          # lowest set bit
    r = n + c           # ripple carry
    ones = ((n ^ r) >> 2) // c  # shifted-in ones
    return r | ones


if __name__ == "__main__":
    print(next_same_popcount(6))  # 0110 -> 1001 = 9
