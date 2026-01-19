# Day 920: Bloom filter: fixed bit array (1000 bits) + k=3 hashes via double hashing.
# add/check are O(k); space O(m) bits. check has false positives, no false negatives.
import hashlib


class BloomFilter:
    SIZE = 1000
    K = 3

    def __init__(self):
        self.bits = [False] * self.SIZE

    def _base_hashes(self, v):
        h1 = int(hashlib.md5(v.encode()).hexdigest(), 16)
        h2 = int(hashlib.sha1(v.encode()).hexdigest(), 16)
        return h1, h2

    def add(self, v):
        h1, h2 = self._base_hashes(v)
        for i in range(self.K):
            self.bits[(h1 + i * h2) % self.SIZE] = True

    def check(self, v):
        h1, h2 = self._base_hashes(v)
        for i in range(self.K):
            if not self.bits[(h1 + i * h2) % self.SIZE]:
                return False
        return True


def main():
    bf = BloomFilter()
    added = ["apple", "banana", "cherry"]
    for s in added:
        bf.add(s)

    print("Added values (expect all true):")
    for s in added:
        print(f"  check({s}) = {bf.check(s)}")

    print("Non-added values (expect mostly false):")
    for s in ["date", "elderberry", "fig", "grape"]:
        print(f"  check({s}) = {bf.check(s)}")


if __name__ == "__main__":
    main()
