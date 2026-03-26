# Day 1268: Select x if b==1 else y using only arithmetic/bit ops (no branches).
# y ^ ((x ^ y) & -b): -b is all-ones when b==1, all-zeros when b==0. O(1).

def select(x: int, y: int, b: int) -> int:
    # Mask 32-bit to mirror fixed-width two's-complement behavior.
    mask = 0xFFFFFFFF
    res = (y ^ ((x ^ y) & (-b & mask))) & mask
    return res - (1 << 32) if res & (1 << 31) else res


if __name__ == "__main__":
    x, y = 5, 10
    print("b=1 ->", select(x, y, 1))  # 5
    print("b=0 ->", select(x, y, 0))  # 10
