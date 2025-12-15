# Day 744: O(1) LFU cache. Maps key->(value,freq); each freq holds an OrderedDict of keys
# (ordered by recency) so eviction picks least-frequent, then least-recent. Track min_freq.
# Time: O(1) per get/set, Space: O(capacity).
from collections import defaultdict, OrderedDict


class LFUCache:
    def __init__(self, capacity):
        self.cap = capacity
        self.val = {}                       # key -> value
        self.freq = {}                      # key -> frequency
        self.buckets = defaultdict(OrderedDict)  # freq -> {key: None} ordered by recency
        self.min_freq = 0

    def _bump(self, key):
        f = self.freq[key]
        del self.buckets[f][key]
        if not self.buckets[f]:
            del self.buckets[f]
            if self.min_freq == f:
                self.min_freq += 1
        self.freq[key] = f + 1
        self.buckets[f + 1][key] = None

    def get(self, key):
        if key not in self.val:
            return None
        self._bump(key)
        return self.val[key]

    def set(self, key, value):
        if self.cap <= 0:
            return
        if key in self.val:
            self.val[key] = value
            self._bump(key)
            return
        if len(self.val) >= self.cap:
            evict_key, _ = self.buckets[self.min_freq].popitem(last=False)
            del self.val[evict_key]
            del self.freq[evict_key]
        self.val[key] = value
        self.freq[key] = 1
        self.buckets[1][key] = None
        self.min_freq = 1


if __name__ == "__main__":
    c = LFUCache(2)
    c.set(1, 1)
    c.set(2, 2)
    print(c.get(1))   # 1
    c.set(3, 3)       # evicts key 2
    print(c.get(2))   # None
    print(c.get(3))   # 3
    c.set(4, 4)       # evicts key 1
    print(c.get(1))   # None
    print(c.get(3))   # 3
    print(c.get(4))   # 4
