# Day 1145: Bloom filter - fixed-size bit array, k hashes.
# add/check O(k); check has false positives but never false negatives.
class BloomFilter:
    def __init__(self, m, k):
        self.m = m
        self.k = k
        self.bits = bytearray(m)

    def _hash(self, v, i):
        h = (1469598103934665603 ^ (i + 1)) & 0xFFFFFFFFFFFFFFFF
        for c in v.encode():
            h ^= c
            h = (h * 1099511628211) & 0xFFFFFFFFFFFFFFFF
        return h % self.m

    def add(self, v):
        for i in range(self.k):
            self.bits[self._hash(v, i)] = 1

    def check(self, v):
        return all(self.bits[self._hash(v, i)] for i in range(self.k))


if __name__ == "__main__":
    bf = BloomFilter(1000, 4)
    bf.add("apple")
    bf.add("banana")
    print(bf.check("apple"), bf.check("banana"), bf.check("cherry"))
    # True True False (likely)
