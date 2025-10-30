# Day 519: Branchless select: mask = -b (all 1s if b=1, all 0s if b=0); pick x or y. O(1).
def select(x, y, b):
    mask = -b & 0xFFFFFFFF          # 32-bit mask
    return ((x & mask) | (y & ~mask & 0xFFFFFFFF))


if __name__ == "__main__":
    print(select(42, 17, 1))  # 42
    print(select(42, 17, 0))  # 17
