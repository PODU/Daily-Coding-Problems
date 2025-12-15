# Day 745: Bloom filter: fixed bit array, k hash functions via double hashing.
# check() may report false positives but never false negatives.
# Time: O(k) per add/check, Space: O(m bits).

class BloomFilter:
    def __init__(self, size=1000, num_hashes=4):
        self.m = size
        self.k = num_hashes
        self.bits = [False] * size

    def _hashes(self, s):
        h1 = 1469598103934665603  # FNV-1a
        for ch in s:
            h1 = ((h1 ^ ord(ch)) * 1099511628211) & 0xFFFFFFFFFFFFFFFF
        h2 = 5381  # djb2
        for ch in s:
            h2 = ((h2 << 5) + h2 + ord(ch)) & 0xFFFFFFFFFFFFFFFF
        return h1, h2

    def add(self, s):
        h1, h2 = self._hashes(s)
        for i in range(self.k):
            self.bits[(h1 + i * h2) % self.m] = True

    def check(self, s):
        h1, h2 = self._hashes(s)
        return all(self.bits[(h1 + i * h2) % self.m] for i in range(self.k))


if __name__ == "__main__":
    bf = BloomFilter()
    bf.add("apple")
    bf.add("banana")
    print("apple:", bf.check("apple"))    # True
    print("banana:", bf.check("banana"))  # True
    print("cherry:", bf.check("cherry"))  # False
