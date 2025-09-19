# Day 301: Bloom filter - fixed-size probabilistic set. No false negatives.
# add/check O(k); space O(m) bits.
import hashlib


class BloomFilter:
    def __init__(self, m, k):
        self.bits = [False] * m
        self.k = k

    def _h(self, v, i):
        d = hashlib.md5((str(i) + ":" + v).encode()).hexdigest()
        return int(d, 16) % len(self.bits)

    def add(self, v):
        for i in range(self.k):
            self.bits[self._h(v, i)] = True

    def check(self, v):
        return all(self.bits[self._h(v, i)] for i in range(self.k))


if __name__ == "__main__":
    bf = BloomFilter(1000, 4)
    bf.add("apple"); bf.add("banana")
    print(bf.check("apple"))    # True
    print(bf.check("banana"))   # True
    print(bf.check("cherry"))   # False
