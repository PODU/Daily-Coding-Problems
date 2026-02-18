# Day 1096: LFU Cache with O(1) get/set.
# Approach: key->val/freq dicts + freq->OrderedDict (LRU) + min_freq pointer.
# Time: O(1) per op. Space: O(n).
from collections import defaultdict, OrderedDict


class LFUCache:
    def __init__(self, n):
        self.cap = n
        self.min_freq = 0
        self.vals = {}
        self.freqs = {}
        self.lists = defaultdict(OrderedDict)

    def _touch(self, key):
        f = self.freqs[key]
        del self.lists[f][key]
        if not self.lists[f]:
            del self.lists[f]
            if self.min_freq == f:
                self.min_freq += 1
        nf = f + 1
        self.freqs[key] = nf
        self.lists[nf][key] = True

    def get(self, key):
        if key not in self.vals:
            return None
        self._touch(key)
        return self.vals[key]

    def set(self, key, value):
        if self.cap <= 0:
            return
        if key in self.vals:
            self.vals[key] = value
            self._touch(key)
            return
        if len(self.vals) >= self.cap:
            evict, _ = self.lists[self.min_freq].popitem(last=False)  # LRU
            if not self.lists[self.min_freq]:
                del self.lists[self.min_freq]
            del self.vals[evict]
            del self.freqs[evict]
        self.vals[key] = value
        self.freqs[key] = 1
        self.lists[1][key] = True
        self.min_freq = 1


if __name__ == "__main__":
    c = LFUCache(2)
    c.set(1, 1)
    c.set(2, 2)
    print(c.get(1))                 # 1
    c.set(3, 3)                     # evicts key 2
    print(c.get(2))                 # None
    print(c.get(3))                 # 3
