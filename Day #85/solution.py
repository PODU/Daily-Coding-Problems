# Day 85: Select x if b==1 else y using only bit ops. mask = -b (all 1s or all 0s).
# Time O(1), Space O(1).

MASK32 = 0xFFFFFFFF


def select(x, y, b):
    mask = (-b) & MASK32  # b=1 -> 0xFFFFFFFF, b=0 -> 0
    return (x & mask) | (y & ~mask & MASK32)


if __name__ == "__main__":
    print(select(5, 10, 1))  # 5
    print(select(5, 10, 0))  # 10
