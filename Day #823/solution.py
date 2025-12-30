# Day 823: Space-efficient bit array using 32-bit words; index>>5 picks word, 1<<(index&31) picks bit.
# Time: O(1) per op, Space: O(n/32 words).

class BitArray:
    def __init__(self, size):
        self.words = [0] * ((size + 31) >> 5)

    def set(self, i, val):
        if val:
            self.words[i >> 5] |= (1 << (i & 31))
        else:
            self.words[i >> 5] &= ~(1 << (i & 31))

    def get(self, i):
        return (self.words[i >> 5] >> (i & 31)) & 1


if __name__ == "__main__":
    ba = BitArray(16)
    ba.set(0, 1)
    ba.set(5, 1)
    ba.set(15, 1)
    print("get(0)=%d" % ba.get(0))
    print("get(1)=%d" % ba.get(1))
    print("get(5)=%d" % ba.get(5))
    print("get(15)=%d" % ba.get(15))
