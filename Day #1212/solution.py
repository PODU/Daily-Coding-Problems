# Day 1212: Space-efficient bit array backed by machine words (bytearray).
# Pack bits into bytes; set/get use byte index and bit offset. Time O(1) per op, Space O(size/8).


class BitArray:
    def __init__(self, size):
        self.n = size
        self.data = bytearray((size + 7) // 8)

    def set(self, i, val):
        if val:
            self.data[i >> 3] |= (1 << (i & 7))
        else:
            self.data[i >> 3] &= ~(1 << (i & 7))

    def get(self, i):
        return (self.data[i >> 3] >> (i & 7)) & 1


if __name__ == "__main__":
    b = BitArray(10)
    b.set(2, 1)
    b.set(7, 1)
    b.set(2, 0)
    print(b.get(2), b.get(7), b.get(0))  # 0 1 0
