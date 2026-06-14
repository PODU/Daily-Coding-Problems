# Day 1661: Bloom filter: fixed 1000-bit array, 3 hashes via double hashing. add/check.
# add O(k), check O(k); Space O(m bits). check may false-positive, never false-negative.
class Bloom:
    M = 1000
    K = 3
    def __init__(self):
        self.bits = [0] * self.M
    def _h1(self, s):
        h = 5381
        for c in s:
            h = (h * 33 + ord(c)) & 0xFFFFFFFFFFFFFFFF
        return h
    def _h2(self, s):
        h = 0
        for c in s:
            h = (h * 131 + ord(c)) & 0xFFFFFFFFFFFFFFFF
        return h
    def add(self, s):
        a, b = self._h1(s), self._h2(s)
        for i in range(self.K):
            self.bits[(a + i * b) % self.M] = 1
    def check(self, s):
        a, b = self._h1(s), self._h2(s)
        return all(self.bits[(a + i * b) % self.M] for i in range(self.K))

def main():
    bf = Bloom()
    for w in ("apple", "banana", "cat"):
        bf.add(w)
    for w in ("apple", "banana", "cat", "dog"):
        print(f"check {w}: {str(bf.check(w)).lower()}")

main()
