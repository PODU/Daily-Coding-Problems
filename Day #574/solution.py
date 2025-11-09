# Day 574: Space-efficient bit array backed by an integer bytearray (8 bits/byte).
# set/get are O(1); storage is ceil(size/8) bytes.


class BitArray:
    def __init__(self, size):
        self.n = size
        self.data = bytearray((size + 7) // 8)

    def set(self, i, val):
        if not 0 <= i < self.n:
            raise IndexError(i)
        if val:
            self.data[i >> 3] |= (1 << (i & 7))
        else:
            self.data[i >> 3] &= ~(1 << (i & 7))

    def get(self, i):
        if not 0 <= i < self.n:
            raise IndexError(i)
        return (self.data[i >> 3] >> (i & 7)) & 1


if __name__ == "__main__":
    b = BitArray(8)
    b.set(0, 1)
    b.set(3, 1)
    print("get(0) =", b.get(0))  # 1
    print("get(1) =", b.get(1))  # 0
    print("get(3) =", b.get(3))  # 1
    b.set(3, 0)
    print("get(3) =", b.get(3))  # 0
