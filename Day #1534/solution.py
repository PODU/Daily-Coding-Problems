# Day 1534: Space-efficient bit array packing 32 bits per word.
# init(size), set(i,val), get(i): each O(1); space O(size/32) words.
class BitArray:
    def init(self, size):
        self.n = size
        self.words = [0] * ((size + 31) // 32)

    def set(self, i, val):
        if val:
            self.words[i >> 5] |= (1 << (i & 31))
        else:
            self.words[i >> 5] &= ~(1 << (i & 31))

    def get(self, i):
        return (self.words[i >> 5] >> (i & 31)) & 1


if __name__ == "__main__":
    b = BitArray()
    b.init(10)
    b.set(1, 1)
    b.set(4, 1)
    b.set(4, 0)
    b.set(9, 1)
    print(b.get(1), b.get(4), b.get(9), b.get(0))
    # expected: 1 0 1 0
