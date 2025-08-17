# Day 137: Space-efficient bit array: pack bits into a bytearray (8 bits/byte). set/get O(1), space O(size/8).


class BitArray:
    def init(self, size):
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
    b = BitArray()
    b.init(10)
    b.set(2, 1); b.set(7, 1); b.set(7, 0); b.set(9, 1)
    print(f"{b.get(2)}{b.get(7)}{b.get(9)}{b.get(0)}")  # 1010
