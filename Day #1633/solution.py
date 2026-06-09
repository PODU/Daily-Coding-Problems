# Day 1633: Branchless select: mask = -b (all-ones if b=1 else 0); result = (x & mask) | (y & ~mask). O(1) time/space.

MASK32 = 0xFFFFFFFF


def select(b, x, y):
    mask = (-b) & MASK32  # 0xFFFFFFFF if b=1, 0 if b=0
    res = (x & mask) | (y & ~mask & MASK32)
    # interpret as signed 32-bit
    return res - (1 << 32) if res >= (1 << 31) else res


def main():
    print(select(1, 42, 99))
    print(select(0, 42, 99))


if __name__ == "__main__":
    main()
